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

pub async fn fetch_covers_target(repo: &Path, settings: &StagingSettings) -> Result<bool> {
    settings.validate().map_err(anyhow::Error::msg)?;
    let key = format!("remote.{}.fetch", settings.remote);
    let deadline = Instant::now() + std::time::Duration::from_secs(5);
    let output = git_output(repo, &["config", "--get-all", &key], 16384, deadline).await?;
    let specs = std::str::from_utf8(&output)?;
    if specs.lines().any(|line| line.starts_with('^')) {
        return Ok(false);
    }
    let branch = settings
        .target_ref
        .strip_prefix(&format!("refs/remotes/{}/", settings.remote))
        .context("staging target remote mismatch")?;
    let exact = format!("refs/heads/{branch}:{}", settings.target_ref);
    let wildcard = format!("refs/heads/*:refs/remotes/{}/*", settings.remote);
    Ok(specs.lines().any(|line| {
        let line = line.strip_prefix('+').unwrap_or(line);
        line == exact || line == wildcard
    }))
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MembershipState {
    MatchedHistory,
    NoMatchInCheckedRange,
    Unknown,
    Stale,
}

#[derive(Debug, Clone, Serialize)]
pub struct PatchEvidence {
    pub patch_id: i64,
    pub state: MembershipState,
    pub target_ref: String,
    pub remote: String,
    pub algorithm_version: u32,
    pub source_identity: Option<String>,
    pub match_kind: Option<MatchKind>,
    pub tip: Option<String>,
    pub refreshed_at: Option<i64>,
    pub checked_at: Option<i64>,
    pub scope_complete: bool,
    pub matched_commits: Vec<String>,
    pub reason: Option<String>,
}

impl PatchEvidence {
    pub fn unknown(patch_id: i64, settings: &StagingSettings, reason: &str) -> Self {
        Self {
            patch_id,
            state: MembershipState::Unknown,
            target_ref: settings.target_ref.clone(),
            remote: settings.remote.clone(),
            algorithm_version: INDEX_VERSION,
            source_identity: None,
            match_kind: None,
            tip: None,
            refreshed_at: None,
            checked_at: None,
            scope_complete: false,
            matched_commits: Vec::new(),
            reason: Some(reason.to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MatchKind {
    StablePatchId,
}

pub use crate::api::{SeriesEvidence, SeriesState};

pub struct StagingQuery {
    pub snapshot: Option<CurrentSnapshot>,
    pub state: Option<SeriesState>,
}

#[derive(Debug, Clone)]
pub struct CurrentSnapshot {
    pub id: i64,
    pub target_ref: String,
    pub tip: String,
    pub refreshed_at: i64,
    pub checked_at: i64,
    pub scope_complete: bool,
}

impl CurrentSnapshot {
    pub fn series_evidence_sql(&self) -> String {
        format!("(SELECT json_object('state', CASE WHEN total_parts > 0 AND present = total_parts AND valid = total_parts AND matched = total_parts THEN 'all_matched' WHEN matched > 0 THEN 'partially_matched' WHEN total_parts > 0 AND present = total_parts AND valid = total_parts AND checked = total_parts THEN 'no_match_in_checked_range' ELSE 'unknown' END, 'present_parts', present, 'expected_parts', total_parts, 'checked_parts', checked, 'matched_parts', matched, 'complete', json(CASE WHEN total_parts > 0 AND present = total_parts AND valid = total_parts THEN 'true' ELSE 'false' END))
            FROM (SELECT p.total_parts AS total_parts, COUNT(*) AS present,
                COUNT(DISTINCT CASE WHEN q.part_index BETWEEN 1 AND p.total_parts THEN q.part_index END) AS valid,
                COALESCE(SUM(CASE WHEN e.source_identity IS NOT NULL AND e.source_identity = q.git_patch_id THEN 1 ELSE 0 END), 0) AS checked,
                COALESCE(SUM(CASE WHEN e.source_identity IS NOT NULL AND e.source_identity = q.git_patch_id AND EXISTS (SELECT 1 FROM staging_snapshot_commits c WHERE c.snapshot_id = e.snapshot_id AND c.patch_identity = e.source_identity) THEN 1 ELSE 0 END), 0) AS matched
                FROM patches q LEFT JOIN staging_patch_evidence e ON e.patch_id = q.id AND e.snapshot_id = {} WHERE q.patchset_id = p.id))", self.id)
    }
}

impl crate::db::Database {
    pub async fn current_staging_snapshot(
        &self,
        repository: &str,
        settings: &StagingSettings,
        now: i64,
    ) -> Result<Option<CurrentSnapshot>> {
        let oldest = now.saturating_sub(i64::try_from(settings.freshness_seconds)?);
        let mut rows = self.conn.query("SELECT s.id, s.target_ref, s.tip, s.refreshed_at, s.checked_at, s.scope_complete FROM staging_targets t JOIN staging_snapshots s ON s.id = t.snapshot_id
            WHERE t.repository = ? AND t.remote = ? AND t.target_ref = ? AND t.last_failure IS NULL AND s.algorithm_version = ? AND s.refreshed_at BETWEEN ? AND ?",
            libsql::params![repository, settings.remote.as_str(), settings.target_ref.as_str(), i64::from(INDEX_VERSION), oldest, now]).await?;
        let Some(row) = rows.next().await? else {
            return Ok(None);
        };
        Ok(Some(CurrentSnapshot {
            id: row.get(0)?,
            target_ref: row.get(1)?,
            tip: row.get(2)?,
            refreshed_at: row.get(3)?,
            checked_at: row.get(4)?,
            scope_complete: row.get::<i64>(5)? != 0,
        }))
    }

    pub async fn staging_series_evidence(
        &self,
        snapshot: &CurrentSnapshot,
        patchset_ids: &[i64],
    ) -> Result<BTreeMap<i64, SeriesEvidence>> {
        ensure!(
            patchset_ids.len() <= 100,
            "staging series pages are limited to 100 items"
        );
        let sql = format!(
            "SELECT p.id, {} FROM patchsets p JOIN json_each(?) requested ON requested.value = p.id",
            snapshot.series_evidence_sql()
        );
        let mut rows = self
            .conn
            .query(&sql, libsql::params![serde_json::to_string(patchset_ids)?])
            .await?;
        let mut results = BTreeMap::new();
        while let Some(row) = rows.next().await? {
            results.insert(row.get(0)?, serde_json::from_str(&row.get::<String>(1)?)?);
        }
        Ok(results)
    }

    pub async fn load_staging_index(
        &self,
        repository: &str,
        limit: usize,
    ) -> Result<BTreeMap<String, CommitIdentity>> {
        ensure!((1..=10000).contains(&limit), "invalid staging cache limit");
        let mut rows = self.conn.query(
            "SELECT commit_id, identity_json FROM staging_commit_index WHERE repository = ? AND algorithm_version = ? ORDER BY computed_at DESC LIMIT ?",
            libsql::params![repository, i64::from(INDEX_VERSION), i64::try_from(limit)?],
        ).await?;
        let mut entries = BTreeMap::new();
        while let Some(row) = rows.next().await? {
            entries.insert(
                row.get::<String>(0)?,
                serde_json::from_str(&row.get::<String>(1)?)?,
            );
        }
        Ok(entries)
    }

    pub async fn publish_staging_snapshot(
        &self,
        repository: &str,
        settings: &StagingSettings,
        snapshot: &StagingSnapshot,
        refreshed_at: i64,
        checked_at: i64,
    ) -> Result<i64> {
        settings.validate().map_err(anyhow::Error::msg)?;
        ensure!(
            snapshot.algorithm_version == INDEX_VERSION
                && snapshot.target_ref == settings.target_ref,
            "staging snapshot configuration/version mismatch"
        );
        ensure!(
            snapshot.entries.len() <= settings.max_commits
                && snapshot.diff_bytes <= settings.max_diff_bytes,
            "staging snapshot exceeds configured bounds"
        );
        ensure!(
            refreshed_at > 0
                && refreshed_at <= checked_at
                && checked_at - refreshed_at <= i64::try_from(settings.freshness_seconds)?,
            "staging snapshot requires fresh successful refresh evidence"
        );
        let tx = self.begin_immediate_transaction().await?;
        tx.execute("INSERT INTO staging_snapshots (repository, remote, target_ref, tip, refreshed_at, checked_at, algorithm_version, scope_complete, diff_bytes) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            libsql::params![repository, settings.remote.as_str(), snapshot.target_ref.as_str(), snapshot.tip.as_str(), refreshed_at, checked_at, i64::from(INDEX_VERSION), i64::from(snapshot.scope_complete), i64::try_from(snapshot.diff_bytes)?]).await?;
        let id = tx.last_insert_rowid();
        for entry in &snapshot.entries {
            let identity = serde_json::to_string(&entry.identity)?;
            let patch_identity = match &entry.identity {
                CommitIdentity::Text { patch_id } => Some(patch_id.as_str()),
                _ => None,
            };
            tx.execute("INSERT INTO staging_commit_index (repository, algorithm_version, commit_id, identity_json, patch_identity, computed_at) VALUES (?, ?, ?, ?, ?, ?) ON CONFLICT DO NOTHING",
                libsql::params![repository, i64::from(INDEX_VERSION), entry.commit.as_str(), identity.as_str(), patch_identity, checked_at]).await?;
            let mut existing = tx.query("SELECT identity_json FROM staging_commit_index WHERE repository = ? AND algorithm_version = ? AND commit_id = ?",
                libsql::params![repository, i64::from(INDEX_VERSION), entry.commit.as_str()]).await?;
            let stored = existing
                .next()
                .await?
                .context("staging cache entry missing")?
                .get::<String>(0)?;
            ensure!(
                stored == identity,
                "immutable staging commit identity changed"
            );
            tx.execute("INSERT INTO staging_snapshot_commits (snapshot_id, commit_id, patch_identity) VALUES (?, ?, ?)",
                libsql::params![id, entry.commit.as_str(), patch_identity]).await?;
        }
        tx.execute("INSERT INTO staging_patch_evidence (snapshot_id, patch_id, source_identity) SELECT ?, id, git_patch_id FROM patches ORDER BY id DESC LIMIT ?", libsql::params![id, i64::try_from(settings.max_patches)?]).await?;
        tx.execute("INSERT INTO staging_targets (repository, remote, target_ref, snapshot_id) VALUES (?, ?, ?, ?) ON CONFLICT(repository, remote, target_ref) DO UPDATE SET snapshot_id = excluded.snapshot_id, last_failure = NULL, failed_at = NULL",
            libsql::params![repository, settings.remote.as_str(), settings.target_ref.as_str(), id]).await?;
        tx.commit().await?;
        Ok(id)
    }

    pub async fn invalidate_staging_target(
        &self,
        repository: &str,
        settings: &StagingSettings,
        reason: &str,
        checked_at: i64,
    ) -> Result<()> {
        self.execute("INSERT INTO staging_targets (repository, remote, target_ref, last_failure, failed_at) VALUES (?, ?, ?, ?, ?) ON CONFLICT(repository, remote, target_ref) DO UPDATE SET last_failure = excluded.last_failure, failed_at = excluded.failed_at",
            libsql::params![repository, settings.remote.as_str(), settings.target_ref.as_str(), reason, checked_at]).await?;
        Ok(())
    }

    pub async fn staging_patch_evidence(
        &self,
        repository: &str,
        settings: &StagingSettings,
        patch_id: i64,
        now: i64,
    ) -> Result<PatchEvidence> {
        self.staging_patch_evidence_batch(repository, settings, &[patch_id], now)
            .await?
            .pop()
            .context("staging evidence batch omitted a requested patch")
    }

    pub async fn staging_patch_evidence_batch(
        &self,
        repository: &str,
        settings: &StagingSettings,
        patch_ids: &[i64],
        now: i64,
    ) -> Result<Vec<PatchEvidence>> {
        ensure!(
            patch_ids.len() <= 100,
            "staging evidence pages are limited to 100 patches"
        );
        if patch_ids.is_empty() {
            return Ok(Vec::new());
        }
        let requested = serde_json::to_string(patch_ids)?;
        let mut rows = self.conn.query("SELECT requested.value, s.id, s.tip, s.refreshed_at, s.checked_at, s.scope_complete, t.last_failure, e.source_identity, p.git_patch_id, s.algorithm_version,
            (SELECT json_group_array(commit_id) FROM staging_snapshot_commits c WHERE c.snapshot_id = s.id AND c.patch_identity = e.source_identity)
            FROM json_each(?) requested
            LEFT JOIN staging_targets t ON t.repository = ? AND t.remote = ? AND t.target_ref = ?
            LEFT JOIN staging_snapshots s ON s.id = t.snapshot_id
            LEFT JOIN staging_patch_evidence e ON e.snapshot_id = s.id AND e.patch_id = requested.value
            LEFT JOIN patches p ON p.id = requested.value ORDER BY requested.key",
            libsql::params![requested, repository, settings.remote.as_str(), settings.target_ref.as_str()]).await?;
        let mut results = Vec::new();
        while let Some(row) = rows.next().await? {
            results.push(project_evidence(&row, settings, now)?);
        }
        Ok(results)
    }
}

fn project_evidence(
    row: &libsql::Row,
    settings: &StagingSettings,
    now: i64,
) -> Result<PatchEvidence> {
    let mut result = PatchEvidence::unknown(row.get(0)?, settings, "No current evidence");
    result.tip = row.get(2)?;
    result.refreshed_at = row.get(3)?;
    result.checked_at = row.get(4)?;
    result.scope_complete = row.get::<Option<i64>>(5)?.is_some_and(|value| value != 0);
    if let Some(reason) = row.get::<Option<String>>(6)? {
        result.reason = Some(reason);
        return Ok(result);
    }
    if row.get::<Option<i64>>(9)? != Some(i64::from(INDEX_VERSION)) {
        return Ok(result);
    }
    let Some(refreshed) = result.refreshed_at else {
        return Ok(result);
    };
    if now < refreshed || now.saturating_sub(refreshed) > i64::try_from(settings.freshness_seconds)?
    {
        result.state = MembershipState::Stale;
        result.reason =
            Some("Successful refresh is stale or the clock moved backwards".to_string());
        return Ok(result);
    }
    let source: Option<String> = row.get(7)?;
    result.source_identity = source.clone();
    if source.is_none() || source != row.get::<Option<String>>(8)? {
        result.reason =
            Some("Patch identity is unavailable or changed since this check".to_string());
        return Ok(result);
    }
    result.matched_commits = serde_json::from_str(&row.get::<String>(10)?)?;
    if result.matched_commits.is_empty() {
        result.state = MembershipState::NoMatchInCheckedRange;
        result.reason = Some("No textual patch-ID match in the checked range; not proof of absence or an effective change".to_string());
    } else {
        result.state = MembershipState::MatchedHistory;
        result.match_kind = Some(MatchKind::StablePatchId);
        result.reason = Some(
            "Diff-equivalent commit reachable at the checked tip; may have been reverted"
                .to_string(),
        );
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn series_evidence_uses_all_parts_and_a_fresh_pinned_snapshot() -> Result<()> {
        let db = crate::db::Database::new(&crate::settings::DatabaseSettings {
            url: ":memory:".into(),
            token: String::new(),
        })
        .await?;
        db.migrate().await?;
        db.execute("INSERT INTO patchsets(id, total_parts, received_parts, status) VALUES (1,2,2,'Reviewed'), (2,2,1,'Failed'), (3,1,1,'Reviewed'), (4,1,1,'Reviewed'), (5,1,0,'Incomplete')", ()).await?;
        db.execute(
            "UPDATE patchsets SET subject = 'saved series', date = id",
            (),
        )
        .await?;
        for (id, series, part, identity) in [
            (1, 1, 1, Some("a")),
            (2, 1, 2, Some("a")),
            (3, 2, 1, Some("a")),
            (4, 3, 1, Some("d")),
            (5, 4, 1, None),
        ] {
            let message = format!("aggregate-{id}@example.invalid");
            db.execute(
                "INSERT INTO messages(message_id) VALUES (?)",
                libsql::params![message.as_str()],
            )
            .await?;
            db.execute("INSERT INTO patches(id,patchset_id,message_id,part_index,git_patch_id) VALUES (?,?,?,?,?)",libsql::params![id,series,message,part,identity.map(|s|s.repeat(40))]).await?;
        }
        let (_, settings) = fixture();
        let snapshot = StagingSnapshot {
            target_ref: settings.target_ref.clone(),
            tip: "b".repeat(40),
            algorithm_version: INDEX_VERSION,
            scope_complete: false,
            diff_bytes: 1,
            entries: vec![IndexEntry {
                commit: "c".repeat(40),
                identity: CommitIdentity::Text {
                    patch_id: "a".repeat(40),
                },
            }],
        };
        db.publish_staging_snapshot("repo", &settings, &snapshot, 90, 100)
            .await?;
        let pinned = db
            .current_staging_snapshot("repo", &settings, 100)
            .await?
            .unwrap();
        let result = db
            .staging_series_evidence(&pinned, &[1, 2, 3, 4, 5])
            .await?;
        assert_eq!(result[&1].state, SeriesState::AllMatched);
        assert!(result[&1].complete);
        assert_eq!(result[&2].state, SeriesState::PartiallyMatched);
        assert!(!result[&2].complete);
        assert_eq!(result[&3].state, SeriesState::NoMatchInCheckedRange);
        assert_eq!(result[&4].state, SeriesState::Unknown);
        assert_eq!(result[&5].state, SeriesState::Unknown);
        for (state, expected) in [
            (SeriesState::AllMatched, vec![1]),
            (SeriesState::PartiallyMatched, vec![2]),
            (SeriesState::NoMatchInCheckedRange, vec![3]),
            (SeriesState::Unknown, vec![4, 5]),
        ] {
            let query = StagingQuery {
                snapshot: Some(pinned.clone()),
                state: Some(state),
            };
            let mut actual = db
                .get_patchsets_classified(100, 0, None, None, None, Some(&query))
                .await?
                .iter()
                .map(|row| row.id)
                .collect::<Vec<_>>();
            actual.sort_unstable();
            assert_eq!(actual, expected);
            assert_eq!(
                db.count_patchsets_classified(None, None, None, Some(&query))
                    .await?,
                expected.len()
            );
            if expected.len() == 2 {
                assert_eq!(
                    db.get_patchsets_classified(1, 1, None, None, None, Some(&query))
                        .await?
                        .len(),
                    1
                );
            }
        }
        let query = StagingQuery {
            snapshot: None,
            state: Some(SeriesState::AllMatched),
        };
        assert_eq!(
            db.count_patchsets_classified(None, None, None, Some(&query))
                .await?,
            0
        );
        let query = StagingQuery {
            snapshot: None,
            state: Some(SeriesState::Unknown),
        };
        assert_eq!(
            db.count_patchsets_classified(None, None, None, Some(&query))
                .await?,
            5
        );
        let query = StagingQuery {
            snapshot: Some(pinned.clone()),
            state: Some(SeriesState::PartiallyMatched),
        };
        let reviewed = Some(crate::api::PatchsetReviewFilter::State(
            crate::ReviewStatus::Reviewed,
        ));
        assert_eq!(
            db.count_patchsets_classified(None, None, reviewed, Some(&query))
                .await?,
            0
        );
        db.execute("UPDATE patches SET part_index = 1 WHERE id = 2", ())
            .await?;
        let duplicate = db.staging_series_evidence(&pinned, &[1]).await?;
        assert!(!duplicate[&1].complete);
        assert_eq!(duplicate[&1].state, SeriesState::PartiallyMatched);
        db.execute("UPDATE patches SET part_index = 2 WHERE id = 2", ())
            .await?;
        db.execute("UPDATE patches SET git_patch_id = NULL WHERE id = 2", ())
            .await?;
        assert_eq!(
            db.staging_series_evidence(&pinned, &[1]).await?[&1].state,
            SeriesState::PartiallyMatched
        );
        db.invalidate_staging_target("repo", &settings, "Refresh failed", 101)
            .await?;
        assert!(
            db.current_staging_snapshot("repo", &settings, 102)
                .await?
                .is_none()
        );
        assert_eq!(
            db.staging_series_evidence(&pinned, &[1]).await?[&1].matched_parts,
            1
        );
        db.publish_staging_snapshot("repo", &settings, &snapshot, 90, 100)
            .await?;
        assert!(
            db.current_staging_snapshot(
                "repo",
                &settings,
                90 + i64::try_from(settings.freshness_seconds)? + 1
            )
            .await?
            .is_none()
        );
        assert!(
            db.current_staging_snapshot("repo", &settings, 89)
                .await?
                .is_none()
        );
        Ok(())
    }

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
    async fn evidence_publication_is_atomic_fresh_and_independent_of_review_state() -> Result<()> {
        let db = crate::db::Database::new(&crate::settings::DatabaseSettings {
            url: ":memory:".to_string(),
            token: String::new(),
        })
        .await?;
        db.migrate().await?;
        db.migrate().await?;
        let identity = "a".repeat(40);
        db.execute(
            "INSERT INTO patchsets (id, subject, status) VALUES (1, 'saved series', 'Reviewed')",
            (),
        )
        .await?;
        db.execute("INSERT INTO messages (message_id, subject) VALUES ('saved@example.invalid', 'saved patch')", ()).await?;
        db.execute("INSERT INTO patches (id, patchset_id, message_id, part_index, git_patch_id) VALUES (1, 1, 'saved@example.invalid', 1, ?)", libsql::params![identity.clone()]).await?;
        let (_, settings) = fixture();
        let mut snapshot = StagingSnapshot {
            target_ref: settings.target_ref.clone(),
            tip: "b".repeat(40),
            algorithm_version: INDEX_VERSION,
            scope_complete: false,
            diff_bytes: 10,
            entries: vec![IndexEntry {
                commit: "c".repeat(40),
                identity: CommitIdentity::Text {
                    patch_id: identity.clone(),
                },
            }],
        };
        assert_eq!(
            db.staging_patch_evidence("repo", &settings, 1, 100)
                .await?
                .state,
            MembershipState::Unknown
        );
        let first = db
            .publish_staging_snapshot("repo", &settings, &snapshot, 90, 100)
            .await?;
        let batch = db
            .staging_patch_evidence_batch("repo", &settings, &[1, 999], 100)
            .await?;
        assert_eq!(batch.len(), 2);
        assert_eq!(batch[0].patch_id, 1);
        assert_eq!(batch[0].state, MembershipState::MatchedHistory);
        assert_eq!(batch[1].patch_id, 999);
        assert_eq!(batch[1].state, MembershipState::Unknown);
        assert_eq!(
            db.staging_patch_evidence("repo", &settings, 1, 100)
                .await?
                .state,
            MembershipState::MatchedHistory
        );
        assert_eq!(db.load_staging_index("repo", 10).await?.len(), 1);
        assert_eq!(
            db.staging_patch_evidence("other-repo", &settings, 1, 100)
                .await?
                .state,
            MembershipState::Unknown
        );
        snapshot.entries[0].identity = CommitIdentity::Empty;
        assert!(
            db.publish_staging_snapshot("repo", &settings, &snapshot, 90, 101)
                .await
                .is_err()
        );
        let mut pointer = db
            .conn
            .query(
                "SELECT snapshot_id FROM staging_targets WHERE repository='repo'",
                (),
            )
            .await?;
        assert_eq!(pointer.next().await?.unwrap().get::<i64>(0)?, first);
        assert_eq!(
            db.staging_patch_evidence("repo", &settings, 1, 100)
                .await?
                .state,
            MembershipState::MatchedHistory
        );
        assert_eq!(
            db.staging_patch_evidence("repo", &settings, 1, 90000)
                .await?
                .state,
            MembershipState::Stale
        );
        assert_eq!(
            db.staging_patch_evidence("repo", &settings, 1, 80)
                .await?
                .state,
            MembershipState::Stale
        );
        db.invalidate_staging_target("repo", &settings, "Refresh failed", 101)
            .await?;
        assert_eq!(
            db.staging_patch_evidence("repo", &settings, 1, 102)
                .await?
                .state,
            MembershipState::Unknown
        );
        snapshot.entries.clear();
        db.publish_staging_snapshot("repo", &settings, &snapshot, 100, 102)
            .await?;
        assert_eq!(
            db.staging_patch_evidence("repo", &settings, 1, 102)
                .await?
                .state,
            MembershipState::NoMatchInCheckedRange
        );
        db.execute("UPDATE patches SET git_patch_id = NULL WHERE id=1", ())
            .await?;
        assert_eq!(
            db.staging_patch_evidence("repo", &settings, 1, 102)
                .await?
                .state,
            MembershipState::Unknown
        );
        let mut status = db
            .conn
            .query("SELECT status FROM patchsets WHERE id=1", ())
            .await?;
        assert_eq!(status.next().await?.unwrap().get::<String>(0)?, "Reviewed");
        db.execute("INSERT INTO messages (message_id, subject) VALUES ('second@example.invalid', 'second patch')", ()).await?;
        db.execute("INSERT INTO patches (id, patchset_id, message_id, part_index) VALUES (2, 1, 'second@example.invalid', 2)", ()).await?;
        let mut bounded = settings.clone();
        bounded.max_patches = 1;
        db.publish_staging_snapshot("repo", &bounded, &snapshot, 101, 102)
            .await?;
        let mut rows = db.conn.query("SELECT COUNT(*) FROM staging_patch_evidence WHERE snapshot_id = (SELECT MAX(id) FROM staging_snapshots)", ()).await?;
        assert_eq!(rows.next().await?.unwrap().get::<i64>(0)?, 1);
        assert_eq!(
            db.staging_patch_evidence("repo", &settings, 1, 102)
                .await?
                .state,
            MembershipState::Unknown
        );
        Ok(())
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
