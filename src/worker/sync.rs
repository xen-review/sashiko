use crate::git_ops::{BackoffError, FetchOutcome, refresh_remote};
use anyhow::Result;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info, warn};

pub struct GitSyncWorker {
    repo_path: PathBuf,
    staging: Option<(crate::settings::StagingSettings, Arc<crate::db::Database>)>,
}

impl GitSyncWorker {
    pub fn new(repo_path: PathBuf) -> Self {
        Self {
            repo_path,
            staging: None,
        }
    }

    pub fn with_staging(
        mut self,
        settings: Option<crate::settings::StagingSettings>,
        db: Arc<crate::db::Database>,
    ) -> Self {
        self.staging = settings.map(|settings| (settings, db));
        self
    }

    async fn update_staging(&self, fetched_at: Option<i64>) -> Result<()> {
        let Some((settings, db)) = &self.staging else {
            return Ok(());
        };
        let repository = tokio::fs::canonicalize(&self.repo_path)
            .await?
            .to_string_lossy()
            .into_owned();
        let result = tokio::time::timeout(Duration::from_secs(settings.timeout_seconds), async {
            let refreshed_at = fetched_at.ok_or_else(|| {
                anyhow::anyhow!("No successful staging remote fetch in this sync cycle")
            })?;
            anyhow::ensure!(
                crate::staging::fetch_covers_target(&self.repo_path, settings).await?,
                "Remote fetch does not establish freshness of the configured staging branch"
            );
            let cache = db
                .load_staging_index(&repository, settings.max_commits)
                .await?;
            let snapshot =
                crate::staging::build_snapshot(&self.repo_path, settings, &cache).await?;
            anyhow::ensure!(
                crate::staging::target_still_matches(&self.repo_path, &snapshot).await?,
                "Staging target changed before publication"
            );
            let id = db
                .publish_staging_snapshot(
                    &repository,
                    settings,
                    &snapshot,
                    refreshed_at,
                    chrono::Utc::now().timestamp(),
                )
                .await?;
            anyhow::ensure!(
                crate::staging::target_still_matches(&self.repo_path, &snapshot).await?,
                "Staging target changed while publishing"
            );
            info!(
                snapshot_id = id,
                commits = snapshot.entries.len(),
                scope_complete = snapshot.scope_complete,
                "Published staging history evidence without changing review state"
            );
            Ok::<_, anyhow::Error>(())
        })
        .await
        .map_err(|_| anyhow::anyhow!("Staging indexing deadline exceeded"))
        .and_then(|result| result);
        if let Err(error) = result {
            let reason = crate::utils::redact_secret(&error.to_string());
            db.invalidate_staging_target(
                &repository,
                settings,
                &reason,
                chrono::Utc::now().timestamp(),
            )
            .await?;
            warn!("Staging evidence unavailable: {}", reason);
        }
        Ok(())
    }

    pub async fn run(&self) {
        info!("GitSyncWorker started. Will sync remotes periodically.");
        loop {
            if let Err(e) = self.sync_all_remotes().await {
                error!(
                    "GitSyncWorker failed during sync cycle: {}",
                    crate::utils::redact_secret(&e.to_string())
                );
                if let Err(error) = self.update_staging(None).await {
                    warn!(
                        "Failed to invalidate staging evidence after sync error: {}",
                        crate::utils::redact_secret(&error.to_string())
                    );
                }
            }
            // Sleep for 1 hour before checking again.
            // refresh_remote applies the per-remote fetch interval, the
            // hour of backoff after a failure, and the day-long fallback
            // window on the local ref.
            sleep(Duration::from_secs(3600)).await;
        }
    }

    async fn sync_all_remotes(&self) -> Result<()> {
        info!("GitSyncWorker: Starting sync cycle.");

        // Enumerate all configured remotes
        let output = crate::git_cmd::in_dir_async(&self.repo_path)
            .args(["remote"])
            .output()
            .await?;

        if !output.status.success() {
            let stderr = crate::utils::redact_secret(&String::from_utf8_lossy(&output.stderr));
            error!("GitSyncWorker: Failed to list git remotes: {}", stderr);
            return Err(anyhow::anyhow!("Failed to list remotes"));
        }

        let remotes_str = String::from_utf8_lossy(&output.stdout);
        let remotes: Vec<&str> = remotes_str
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty() && !l.starts_with("fetcher-"))
            .collect();

        info!("GitSyncWorker: Found {} remotes to check.", remotes.len());

        let mut fetched = 0usize;
        let mut skipped = 0usize;
        let mut backed_off = 0usize;
        let mut stale = 0usize;
        let mut failed = 0usize;
        let mut staging_fetched = None;
        let mut first_failure = String::new();
        // Git's stderr can carry the remote URL, credentials and
        // all, and runs to several lines.
        let mut record_failure = |message: &str| {
            if first_failure.is_empty() {
                first_failure = crate::utils::redact_secret(message)
                    .lines()
                    .map(str::trim)
                    .filter(|line| !line.is_empty())
                    .collect::<Vec<_>>()
                    .join("; ");
            }
        };

        for remote in remotes {
            // Get URL for the remote
            let url_output = crate::git_cmd::in_dir_async(&self.repo_path)
                .args(["remote", "get-url", remote])
                .output()
                .await?;

            if !url_output.status.success() {
                let stderr =
                    crate::utils::redact_secret(&String::from_utf8_lossy(&url_output.stderr));
                let message = format!("Failed to get URL for remote {}: {}", remote, stderr.trim());
                warn!("GitSyncWorker: {}", message);
                failed += 1;
                record_failure(&message);
                continue;
            }

            let url = String::from_utf8_lossy(&url_output.stdout)
                .trim()
                .to_string();

            // Check if it's time to fetch and fetch if necessary
            // force_fetch=false so the fetch interval in refresh_remote holds
            match refresh_remote(&self.repo_path, remote, &url, false).await {
                Ok(FetchOutcome::Fetched) => {
                    fetched += 1;
                    if self
                        .staging
                        .as_ref()
                        .is_some_and(|(settings, _)| settings.remote == remote)
                    {
                        staging_fetched = Some(chrono::Utc::now().timestamp());
                    }
                }
                Ok(FetchOutcome::Skipped) => skipped += 1,
                Ok(FetchOutcome::BackedOff) => backed_off += 1,
                Ok(FetchOutcome::StaleLocalRef(message)) => {
                    stale += 1;
                    record_failure(&message);
                }
                Err(e) if e.downcast_ref::<BackoffError>().is_some() => {
                    info!("GitSyncWorker: {}", e);
                    backed_off += 1;
                }
                Err(e) => {
                    let message = crate::utils::redact_secret(&e.to_string());
                    error!(
                        "GitSyncWorker: Failed to sync remote {}: {}",
                        remote, message
                    );
                    failed += 1;
                    record_failure(&message);
                }
            }
        }

        // A cycle that reaches no remote it tried is a condition of
        // the repository or the network rather than of any one
        // remote, and the per-remote errors above do not say so
        // anywhere.  A cycle that tried nothing because every remote
        // is backing off is the same outage, still unresolved.  The
        // fetches that failed were not this worker's own.  Those are
        // an hour old by its next cycle, past the backoff window, so
        // a review or a baseline lookup failed them within the hour.
        // That cycle has no failure to quote and the network may have
        // recovered since, so it logs at warn.
        if let Err(error) = self.update_staging(staging_fetched).await {
            warn!(
                "Failed to update staging evidence: {}",
                crate::utils::redact_secret(&error.to_string())
            );
        }
        let tried = fetched + stale + failed;
        if fetched == 0 && tried > 0 {
            error!(
                "GitSyncWorker: Sync cycle fetched nothing: {} skipped, {} backing off, {} stale, {} failed. First failure: {}",
                skipped, backed_off, stale, failed, first_failure
            );
        } else if fetched == 0 && backed_off > 0 {
            warn!(
                "GitSyncWorker: Sync cycle fetched nothing: {} skipped, {} backing off.",
                skipped, backed_off
            );
        } else {
            info!(
                "GitSyncWorker: Sync cycle complete: {} fetched, {} skipped, {} backing off, {} stale, {} failed.",
                fetched, skipped, backed_off, stale, failed
            );
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::sync::{Arc, Mutex};
    use tracing::instrument::WithSubscriber;

    #[tokio::test]
    async fn staging_hook_requires_successful_covering_refresh_and_preserves_reviews() -> Result<()>
    {
        let repo = tempfile::tempdir()?;
        for args in [
            vec!["init"],
            vec!["config", "user.name", "Staging Test"],
            vec!["config", "user.email", "staging@example.invalid"],
            vec!["commit", "--allow-empty", "-m", "base"],
            vec!["remote", "add", "upstream", "https://example.invalid/xen"],
            vec!["update-ref", "refs/remotes/upstream/staging", "HEAD"],
        ] {
            let status = crate::git_cmd::in_dir_async(repo.path())
                .args(args)
                .status()
                .await?;
            anyhow::ensure!(status.success(), "fixture Git setup failed");
        }
        let db = Arc::new(
            crate::db::Database::new(&crate::settings::DatabaseSettings {
                url: ":memory:".to_string(),
                token: String::new(),
            })
            .await?,
        );
        db.migrate().await?;
        db.execute(
            "INSERT INTO patchsets (id, status, subject) VALUES (1, 'Reviewed', 'preserved')",
            (),
        )
        .await?;
        db.execute(
            "INSERT INTO messages (message_id, subject) VALUES ('hook@example.invalid', 'patch')",
            (),
        )
        .await?;
        db.execute("INSERT INTO patches (id, patchset_id, message_id, part_index, git_patch_id) VALUES (1, 1, 'hook@example.invalid', 1, ?)", libsql::params!["a".repeat(40)]).await?;
        let settings: crate::settings::StagingSettings =
            toml::from_str("remote='upstream'\ntarget_ref='refs/remotes/upstream/staging'")?;
        let repository = tokio::fs::canonicalize(repo.path())
            .await?
            .to_string_lossy()
            .into_owned();
        let worker = GitSyncWorker::new(repo.path().to_path_buf())
            .with_staging(Some(settings.clone()), db.clone());
        worker
            .update_staging(Some(chrono::Utc::now().timestamp()))
            .await?;
        assert_eq!(
            db.staging_patch_evidence(&repository, &settings, 1, chrono::Utc::now().timestamp())
                .await?
                .state,
            crate::staging::MembershipState::NoMatchInCheckedRange
        );
        worker.update_staging(None).await?;
        assert_eq!(
            db.staging_patch_evidence(&repository, &settings, 1, chrono::Utc::now().timestamp())
                .await?
                .state,
            crate::staging::MembershipState::Unknown
        );
        let status = crate::git_cmd::in_dir_async(repo.path())
            .args([
                "config",
                "remote.upstream.fetch",
                "+refs/heads/master:refs/remotes/upstream/master",
            ])
            .status()
            .await?;
        assert!(status.success());
        worker
            .update_staging(Some(chrono::Utc::now().timestamp()))
            .await?;
        assert_eq!(
            db.staging_patch_evidence(&repository, &settings, 1, chrono::Utc::now().timestamp())
                .await?
                .state,
            crate::staging::MembershipState::Unknown
        );
        let mut rows = db
            .conn
            .query("SELECT status FROM patchsets WHERE id=1", ())
            .await?;
        assert_eq!(rows.next().await?.unwrap().get::<String>(0)?, "Reviewed");
        Ok(())
    }

    #[derive(Clone, Default)]
    struct LogBuffer(Arc<Mutex<Vec<u8>>>);

    struct BufferWriter(Arc<Mutex<Vec<u8>>>);

    impl Write for BufferWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0.lock().unwrap().extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    impl<'a> tracing_subscriber::fmt::MakeWriter<'a> for LogBuffer {
        type Writer = BufferWriter;

        fn make_writer(&'a self) -> Self::Writer {
            BufferWriter(Arc::clone(&self.0))
        }
    }

    #[tokio::test]
    async fn test_sync_all_remotes_redacts_secrets_in_logs() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let repo_path = dir.path().to_path_buf();

        let init = crate::git_cmd::in_dir_async(&repo_path)
            .args(["init"])
            .output()
            .await?;
        assert!(init.status.success(), "git init failed");

        let secret_url =
            "https://myuser:secret_token_123@127.0.0.1:0/repo.git?token=api_secret_456";
        let add = crate::git_cmd::in_dir_async(&repo_path)
            .args(["remote", "add", "secret-remote", secret_url])
            .output()
            .await?;
        assert!(add.status.success(), "git remote add failed");

        let buffer = LogBuffer::default();
        let subscriber = tracing_subscriber::fmt()
            .with_writer(buffer.clone())
            .with_ansi(false)
            .finish();

        let worker = GitSyncWorker::new(repo_path);
        worker
            .sync_all_remotes()
            .with_subscriber(subscriber)
            .await?;

        let logs = String::from_utf8_lossy(&buffer.0.lock().unwrap()).into_owned();
        assert!(
            logs.contains("GitSyncWorker: Failed to sync remote secret-remote:"),
            "expected failure log in: {logs}"
        );
        assert!(
            logs.contains("[REDACTED]"),
            "expected redacted marker in: {logs}"
        );
        assert!(
            !logs.contains("secret_token_123"),
            "secret password leaked in logs: {logs}"
        );
        assert!(
            !logs.contains("myuser"),
            "secret username leaked in logs: {logs}"
        );
        assert!(
            !logs.contains("api_secret_456"),
            "secret token parameter leaked in logs: {logs}"
        );

        // Also verify redaction when `git remote` itself fails with a secret in stderr.
        // Writing a non-`gitdir:` content into `.git` triggers `invalid gitfile format: <path>`,
        // which echoes the `.git` file path across all Git versions (including Git 2.55+, where
        // a nonexistent `gitdir:` target prints `not a git repository: (null)`).
        let bad_dir = tempfile::tempdir()?;
        let sub_dir = bad_dir.path().join("token=api_secret_456");
        std::fs::create_dir(&sub_dir)?;
        std::fs::write(sub_dir.join(".git"), "invalid-gitfile\n")?;
        let bad_buffer = LogBuffer::default();
        let bad_subscriber = tracing_subscriber::fmt()
            .with_writer(bad_buffer.clone())
            .with_ansi(false)
            .finish();
        let bad_worker = GitSyncWorker::new(sub_dir);
        let res = bad_worker
            .sync_all_remotes()
            .with_subscriber(bad_subscriber)
            .await;
        assert!(res.is_err());

        let bad_logs = String::from_utf8_lossy(&bad_buffer.0.lock().unwrap()).into_owned();
        assert!(
            bad_logs.contains("GitSyncWorker: Failed to list git remotes:"),
            "expected list remotes failure log in: {bad_logs}"
        );
        assert!(
            bad_logs.contains("[REDACTED]"),
            "expected redacted marker in: {bad_logs}"
        );
        assert!(
            !bad_logs.contains("api_secret_456"),
            "secret token parameter leaked in list remotes logs: {bad_logs}"
        );

        Ok(())
    }
}
