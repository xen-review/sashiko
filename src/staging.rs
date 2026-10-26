use std::collections::BTreeMap;
use std::path::Path;
use std::process::Stdio;

use anyhow::{Context, Result, bail, ensure};
use serde::{Deserialize, Serialize};
use tokio::io::AsyncReadExt;
use tokio::time::{Instant, timeout_at};

use crate::settings::StagingSettings;

pub const INDEX_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CommitIdentity {
    Text { patch_id: String },
    Empty,
    Binary,
    Merge,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexEntry {
    pub commit: String,
    pub identity: CommitIdentity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StagingSnapshot {
    pub target_ref: String,
    pub tip: String,
    pub algorithm_version: u32,
    pub scope_complete: bool,
    pub diff_bytes: usize,
    pub entries: Vec<IndexEntry>,
}

impl StagingSnapshot {
    pub fn matching_commits(&self, patch_id: &str) -> Vec<&str> {
        self.entries
            .iter()
            .filter_map(|entry| match &entry.identity {
                CommitIdentity::Text { patch_id: indexed } if indexed == patch_id => {
                    Some(entry.commit.as_str())
                }
                _ => None,
            })
            .collect()
    }
}

async fn git_output(
    repo: &Path,
    args: &[&str],
    limit: usize,
    deadline: Instant,
) -> Result<Vec<u8>> {
    timeout_at(deadline, async {
        let mut child = crate::git_cmd::in_dir_async(repo)
            .arg("--no-replace-objects")
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .context("start bounded staging Git read")?;
        let mut stdout = child
            .stdout
            .take()
            .context("staging Git stdout unavailable")?;
        let mut bytes = Vec::new();
        (&mut stdout)
            .take(u64::try_from(limit)?.saturating_add(1))
            .read_to_end(&mut bytes)
            .await?;
        if bytes.len() > limit {
            child.kill().await?;
            bail!("staging Git read exceeded its byte limit");
        }
        ensure!(child.wait().await?.success(), "staging Git read failed");
        Ok(bytes)
    })
    .await
    .context("staging indexing time limit exceeded")?
}

async fn resolve_tip(repo: &Path, target: &str, deadline: Instant) -> Result<String> {
    let bytes = git_output(
        repo,
        &["rev-parse", "--verify", &format!("{target}^{{commit}}")],
        128,
        deadline,
    )
    .await?;
    let tip = std::str::from_utf8(&bytes)?.trim();
    ensure!(
        [40, 64].contains(&tip.len()) && tip.bytes().all(|byte| byte.is_ascii_hexdigit()),
        "invalid staging target commit"
    );
    Ok(tip.to_string())
}

pub async fn target_still_matches(repo: &Path, snapshot: &StagingSnapshot) -> Result<bool> {
    let deadline = Instant::now() + std::time::Duration::from_secs(5);
    Ok(resolve_tip(repo, &snapshot.target_ref, deadline).await? == snapshot.tip)
}

pub async fn build_snapshot(
    repo: &Path,
    settings: &StagingSettings,
    cached: &BTreeMap<String, CommitIdentity>,
) -> Result<StagingSnapshot> {
    settings.validate().map_err(anyhow::Error::msg)?;
    let deadline = Instant::now() + std::time::Duration::from_secs(settings.timeout_seconds);
    let tip = resolve_tip(repo, &settings.target_ref, deadline).await?;
    let max_count = format!("--max-count={}", settings.max_commits + 1);
    let history = git_output(
        repo,
        &["rev-list", "--parents", &max_count, &tip, "--"],
        (settings.max_commits + 1) * 1024,
        deadline,
    )
    .await?;
    let history = std::str::from_utf8(&history)?;
    let mut snapshot = StagingSnapshot {
        target_ref: settings.target_ref.clone(),
        tip,
        algorithm_version: INDEX_VERSION,
        scope_complete: history.lines().count() <= settings.max_commits,
        diff_bytes: 0,
        entries: Vec::new(),
    };
    for line in history.lines().take(settings.max_commits) {
        let mut revisions = line.split_whitespace();
        let commit = revisions.next().context("missing staging history commit")?;
        let identity = if let Some(identity) = cached.get(commit) {
            identity.clone()
        } else if revisions.count() > 1 {
            CommitIdentity::Merge
        } else {
            let diff = git_output(
                repo,
                &[
                    "show",
                    "--format=",
                    "--no-ext-diff",
                    "--no-textconv",
                    "--no-renames",
                    "--no-color",
                    "--full-index",
                    "--binary",
                    "--diff-algorithm=myers",
                    commit,
                    "--",
                ],
                settings.max_diff_bytes.saturating_sub(snapshot.diff_bytes),
                deadline,
            )
            .await?;
            snapshot.diff_bytes += diff.len();
            let diff = std::str::from_utf8(&diff).context("staging diff is not UTF-8")?;
            if diff
                .lines()
                .any(|line| line == "GIT binary patch" || line.starts_with("Binary files "))
            {
                CommitIdentity::Binary
            } else {
                match timeout_at(deadline, crate::prerequisites::calculate_git_patch_id(diff))
                    .await
                    .context("staging patch-ID deadline exceeded")??
                {
                    Some(patch_id) => CommitIdentity::Text { patch_id },
                    None => CommitIdentity::Empty,
                }
            }
        };
        snapshot.entries.push(IndexEntry {
            commit: commit.to_string(),
            identity,
        });
    }
    ensure!(
        resolve_tip(repo, &settings.target_ref, deadline).await? == snapshot.tip,
        "staging target moved during indexing"
    );
    Ok(snapshot)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn git(repo: &Path, args: &[&str]) -> String {
        let output = crate::git_cmd::in_dir(repo).args(args).output().unwrap();
        assert!(
            output.status.success(),
            "{args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout).unwrap().trim().to_string()
    }

    fn fixture() -> (tempfile::TempDir, StagingSettings) {
        let repo = tempfile::tempdir().unwrap();
        git(repo.path(), &["init", "-b", "master"]);
        git(repo.path(), &["config", "user.name", "Staging Test"]);
        git(
            repo.path(),
            &["config", "user.email", "staging@example.invalid"],
        );
        git(repo.path(), &["commit", "--allow-empty", "-m", "base"]);
        let settings: StagingSettings =
            toml::from_str("remote='upstream'\ntarget_ref='refs/remotes/upstream/staging'")
                .unwrap();
        (repo, settings)
    }

    fn commit_file(repo: &Path, name: &str, content: &[u8]) -> String {
        std::fs::write(repo.join(name), content).unwrap();
        git(repo, &["add", name]);
        git(repo, &["commit", "-m", name]);
        git(repo, &["rev-parse", "HEAD"])
    }

    #[tokio::test]
    async fn snapshot_matches_rewritten_commits_and_pins_reachability() -> Result<()> {
        let (repo, settings) = fixture();
        let original = commit_file(repo.path(), "file", b"change\n");
        git(
            repo.path(),
            &["commit", "--amend", "-m", "rewritten subject"],
        );
        let rewritten = git(repo.path(), &["rev-parse", "HEAD"]);
        assert_ne!(original, rewritten);
        git(
            repo.path(),
            &["update-ref", &settings.target_ref, &rewritten],
        );
        git(repo.path(), &["reset", "--hard", "HEAD^"]);
        let snapshot = build_snapshot(repo.path(), &settings, &BTreeMap::new()).await?;
        assert!(snapshot.scope_complete);
        let diff = git(repo.path(), &["show", "--format=", &original]);
        let patch_id = crate::prerequisites::calculate_git_patch_id(&diff)
            .await?
            .unwrap();
        assert_eq!(
            snapshot.matching_commits(&patch_id),
            vec![rewritten.as_str()]
        );
        assert_ne!(git(repo.path(), &["rev-parse", "master"]), snapshot.tip);
        assert!(target_still_matches(repo.path(), &snapshot).await?);
        git(repo.path(), &["update-ref", &settings.target_ref, "HEAD"]);
        assert!(!target_still_matches(repo.path(), &snapshot).await?);
        let replaced = build_snapshot(repo.path(), &settings, &BTreeMap::new()).await?;
        assert!(replaced.matching_commits(&patch_id).is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn snapshot_bounds_and_reuses_only_reachable_commit_entries() -> Result<()> {
        let (repo, mut settings) = fixture();
        assert!(
            build_snapshot(repo.path(), &settings, &BTreeMap::new())
                .await
                .is_err()
        );
        let original = commit_file(repo.path(), "file", b"change\n");
        git(
            repo.path(),
            &["update-ref", &settings.target_ref, &original],
        );
        let snapshot = build_snapshot(repo.path(), &settings, &BTreeMap::new()).await?;
        let cache = snapshot
            .entries
            .iter()
            .map(|entry| (entry.commit.clone(), entry.identity.clone()))
            .collect();
        settings.max_diff_bytes = 1;
        assert!(
            build_snapshot(repo.path(), &settings, &BTreeMap::new())
                .await
                .is_err()
        );
        assert_eq!(
            build_snapshot(repo.path(), &settings, &cache)
                .await?
                .diff_bytes,
            0
        );
        settings.max_commits = 1;
        assert!(
            !build_snapshot(repo.path(), &settings, &cache)
                .await?
                .scope_complete
        );
        git(repo.path(), &["update-ref", &settings.target_ref, "HEAD^"]);
        assert!(
            build_snapshot(repo.path(), &settings, &cache)
                .await?
                .entries
                .iter()
                .all(|entry| entry.commit != original)
        );
        Ok(())
    }

    #[tokio::test]
    async fn snapshot_retains_history_after_revert_and_skips_binary_and_empty() -> Result<()> {
        let (repo, settings) = fixture();
        let original = commit_file(repo.path(), "file", b"change\n");
        git(repo.path(), &["revert", "--no-edit", &original]);
        commit_file(repo.path(), "binary", b"\0\x01\x02");
        git(repo.path(), &["update-ref", &settings.target_ref, "HEAD"]);
        let snapshot = build_snapshot(repo.path(), &settings, &BTreeMap::new()).await?;
        assert!(
            snapshot
                .entries
                .iter()
                .any(|entry| entry.commit == original)
        );
        assert!(
            snapshot
                .entries
                .iter()
                .any(|entry| entry.identity == CommitIdentity::Binary)
        );
        assert!(
            snapshot
                .entries
                .iter()
                .any(|entry| entry.identity == CommitIdentity::Empty)
        );
        Ok(())
    }

    #[tokio::test]
    async fn snapshot_preserves_duplicate_patch_ids_and_marks_merges() -> Result<()> {
        let (repo, settings) = fixture();
        let original = commit_file(repo.path(), "file", b"change\n");
        git(repo.path(), &["revert", "--no-edit", &original]);
        git(repo.path(), &["cherry-pick", &original]);
        let repeated = git(repo.path(), &["rev-parse", "HEAD"]);
        git(repo.path(), &["checkout", "-b", "side"]);
        commit_file(repo.path(), "side", b"side\n");
        git(repo.path(), &["checkout", "master"]);
        commit_file(repo.path(), "main", b"main\n");
        git(
            repo.path(),
            &["merge", "--no-ff", "side", "-m", "merge side"],
        );
        git(repo.path(), &["update-ref", &settings.target_ref, "HEAD"]);
        let snapshot = build_snapshot(repo.path(), &settings, &BTreeMap::new()).await?;
        assert_eq!(snapshot.entries[0].identity, CommitIdentity::Merge);
        let diff = git(repo.path(), &["show", "--format=", &original]);
        let patch_id = crate::prerequisites::calculate_git_patch_id(&diff)
            .await?
            .unwrap();
        let matches = snapshot.matching_commits(&patch_id);
        assert_eq!(matches.len(), 2);
        assert!(matches.contains(&original.as_str()));
        assert!(matches.contains(&repeated.as_str()));
        Ok(())
    }
}
