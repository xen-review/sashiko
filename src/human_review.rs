use anyhow::{Context, Result, ensure};
use sha2::{Digest, Sha256};

use crate::db::Database;
use crate::patch::{DirectReviewClaim, PatchsetMetadata};

pub const EXTRACTION_VERSION: i64 = 1;

#[derive(Debug)]
pub struct HumanReviewInput {
    source_hash: Option<String>,
    claims: Vec<DirectReviewClaim>,
}

pub fn parse_ingested_mail(
    raw: &[u8],
    enabled: bool,
) -> Result<(
    PatchsetMetadata,
    Option<crate::patch::Patch>,
    Option<HumanReviewInput>,
)> {
    let (metadata, patch) = crate::patch::parse_email(raw)?;
    let source_hash = (enabled && raw.len() <= 1024 * 1024).then(|| {
        Sha256::digest(raw)
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect()
    });
    let claims = if enabled && !metadata.is_patch_or_cover {
        match crate::patch::extract_direct_review_claims(raw) {
            Ok(claims) => claims,
            Err(error) => {
                tracing::warn!(message_id = %metadata.message_id, %error, "Human-review evidence unavailable");
                Vec::new()
            }
        }
    } else {
        Vec::new()
    };
    Ok((
        metadata,
        patch,
        enabled.then_some(HumanReviewInput {
            source_hash,
            claims,
        }),
    ))
}

pub const CURRENT_EVIDENCE_SQL: &str = "SELECT h.* FROM human_review_evidence h
    JOIN messages source ON source.id = h.source_id
    JOIN patches target ON target.id = h.target_patch_id
    JOIN messages target_message ON target_message.id = h.target_message_id
    WHERE h.extraction_version = 1
      AND source.human_review_revision = h.source_revision
      AND target.human_review_revision = h.target_revision
      AND target_message.human_review_revision = h.target_message_revision
      AND source.in_reply_to = target_message.message_id
      AND target.message_id = target_message.message_id AND target.part_index > 0";

impl Database {
    pub async fn record_direct_review_claims(
        &self,
        metadata: &PatchsetMetadata,
        input: &HumanReviewInput,
        recorded_at: i64,
    ) -> Result<u64> {
        let claims = &input.claims;
        ensure!(
            claims.len() <= 1,
            "At most one direct-review claim per message"
        );
        let tx = self.begin_immediate_transaction().await?;
        let mut rows = tx
            .query(
                "SELECT id, human_review_revision, body FROM messages
            WHERE message_id = ? AND author = ? AND subject = ? AND date = ? AND in_reply_to = ?",
                libsql::params![
                    metadata.message_id.as_str(),
                    metadata.author.as_str(),
                    metadata.subject.as_str(),
                    metadata.date,
                    metadata.in_reply_to.as_deref()
                ],
            )
            .await?;
        let source = rows
            .next()
            .await?
            .context("Saved human-review source metadata changed before persistence")?;
        ensure!(
            crate::compression::get_compressed_string(&source, 2)? == metadata.body,
            "Saved human-review source body changed before persistence"
        );
        let source_id: i64 = source.get(0)?;
        tx.execute(
            "UPDATE messages SET human_review_source_hash = ? WHERE id = ?",
            libsql::params![input.source_hash.as_deref(), source_id],
        )
        .await?;
        if claims.is_empty() || metadata.is_patch_or_cover || metadata.body.is_empty() {
            tx.commit().await?;
            return Ok(0);
        }
        let source_revision: i64 = tx
            .query(
                "SELECT human_review_revision FROM messages WHERE id = ?",
                [source_id],
            )
            .await?
            .next()
            .await?
            .context("Saved source revision missing")?
            .get(0)?;
        let claim = &claims[0];
        ensure!(
            claim.source_message_id == metadata.message_id
                && Some(claim.target_message_id.as_str()) == metadata.in_reply_to.as_deref()
                && Some(claim.source_hash.as_str()) == input.source_hash.as_deref(),
            "Direct-review claim does not describe the saved source message"
        );
        let inserted =
            Self::insert_direct_review_claim(&tx, source_id, source_revision, claim, recorded_at)
                .await?;
        tx.commit().await?;
        Ok(inserted)
    }

    async fn insert_direct_review_claim(
        tx: &libsql::Transaction,
        source_id: i64,
        source_revision: i64,
        claim: &DirectReviewClaim,
        recorded_at: i64,
    ) -> Result<u64> {
        let mut targets = tx
            .query(
                "SELECT p.id, p.human_review_revision, m.id, m.human_review_revision, p.diff
            FROM patches p JOIN messages m ON m.message_id = p.message_id
            WHERE p.message_id = ? AND p.part_index > 0 LIMIT 2",
                [claim.target_message_id.as_str()],
            )
            .await?;
        let Some(target) = targets.next().await? else {
            return Ok(0);
        };
        let target_id: i64 = target.get(0)?;
        let target_revision: i64 = target.get(1)?;
        let target_message_id: i64 = target.get(2)?;
        let target_message_revision: i64 = target.get(3)?;
        let diff = crate::compression::get_compressed_string(&target, 4)?;
        if targets.next().await?.is_some() {
            return Ok(0);
        }
        if diff.is_empty() || diff.len() > 16 * 1024 * 1024 {
            return Ok(0);
        }
        let prior_target_versions: i64 = tx
            .query(
                "SELECT count(*) FROM human_review_evidence
                        WHERE source_id = ? AND target_patch_id = ?
                            AND (target_revision != ? OR target_message_revision != ?)",
                libsql::params![
                    source_id,
                    target_id,
                    target_revision,
                    target_message_revision
                ],
            )
            .await?
            .next()
            .await?
            .context("Target version count")?
            .get(0)?;
        if prior_target_versions > 0 {
            return Ok(0);
        }
        let target_hash: String = Sha256::digest(diff.as_bytes())
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect();
        let inserted = tx
            .execute(
                "INSERT INTO human_review_evidence
            (source_id, target_patch_id, target_message_id, source_revision, target_revision,
             target_message_revision, extraction_version, source_hash, target_diff_hash,
             reviewer_mailbox, trailer, body_line, recorded_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) ON CONFLICT DO NOTHING",
                libsql::params![
                    source_id,
                    target_id,
                    target_message_id,
                    source_revision,
                    target_revision,
                    target_message_revision,
                    EXTRACTION_VERSION,
                    claim.source_hash.as_str(),
                    target_hash,
                    claim.reviewer_mailbox.as_str(),
                    claim.trailer.as_str(),
                    i64::try_from(claim.body_line)?,
                    recorded_at
                ],
            )
            .await?;
        Ok(inserted)
    }

    pub async fn current_human_review_count(&self, patch_id: i64) -> Result<i64> {
        let sql =
            format!("SELECT count(*) FROM ({CURRENT_EVIDENCE_SQL}) WHERE target_patch_id = ?");
        self.conn
            .query(&sql, [patch_id])
            .await?
            .next()
            .await?
            .context("Missing human evidence count")?
            .get(0)
            .map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn direct_review_storage_is_idempotent_and_revision_guarded() -> Result<()> {
        let db = Database::new(&crate::settings::DatabaseSettings {
            url: ":memory:".into(),
            token: String::new(),
        })
        .await?;
        db.migrate().await?;
        db.conn.execute_batch("
            INSERT INTO messages (id, message_id, thread_id, author, subject, date, body)
                VALUES (1, 'patch@example.invalid', NULL, 'Author <author@example.invalid>', '[PATCH] change', 1, 'diff --git a/file b/file');
            INSERT INTO patchsets (id, subject, total_parts, status)
                VALUES (1, '[PATCH] change', 1, 'Reviewed');
            INSERT INTO patches (id, patchset_id, message_id, part_index, diff)
                VALUES (1, 1, 'patch@example.invalid', 1, 'diff --git a/file b/file');").await?;
        let raw = b"From: Reviewer <reviewer@example.invalid>\r\nMessage-ID: <review@example.invalid>\r\nIn-Reply-To: <patch@example.invalid>\r\nSubject: Re: [PATCH] change\r\nDate: Thu, 01 Jan 1970 00:00:02 +0000\r\n\r\nReviewed-by: Reviewer <reviewer@example.invalid>\r\n";
        assert!(!crate::settings::ServerSettings::default().human_review_evidence);
        assert!(parse_ingested_mail(raw, false)?.2.is_none());
        let (metadata, _, input) = parse_ingested_mail(raw, true)?;
        let input = input.context("Enabled human-review input")?;
        let thread_id = db
            .ensure_thread_for_message("patch@example.invalid", 1)
            .await?;
        db.create_message_with_references(
            &metadata.message_id,
            thread_id,
            metadata.in_reply_to.as_deref(),
            &metadata.author,
            &metadata.subject,
            metadata.date,
            &metadata.body,
            "",
            "",
            None,
            None,
            None,
        )
        .await?;
        assert_eq!(input.claims.len(), 1);
        assert!(!metadata.is_patch_or_cover);
        let target_count: i64 = db.conn.query("SELECT count(*) FROM patches p JOIN messages m ON m.message_id = p.message_id WHERE p.message_id = ? AND p.part_index > 0", [input.claims[0].target_message_id.as_str()]).await?.next().await?.context("Target count")?.get(0)?;
        assert_eq!(target_count, 1);
        let recorded = db.record_direct_review_claims(&metadata, &input, 3).await?;
        let stored: i64 = db
            .conn
            .query("SELECT count(*) FROM human_review_evidence", ())
            .await?
            .next()
            .await?
            .context("Stored evidence count")?
            .get(0)?;
        assert_eq!((recorded, stored), (1, 1));
        assert_eq!(
            db.record_direct_review_claims(&metadata, &input, 4).await?,
            0
        );
        assert_eq!(db.current_human_review_count(1).await?, 1);
        db.execute(
            "UPDATE messages SET body = 'Withdrawn' WHERE message_id = 'review@example.invalid'",
            (),
        )
        .await?;
        assert_eq!(db.current_human_review_count(1).await?, 0);
        assert!(
            db.record_direct_review_claims(&metadata, &input, 5)
                .await
                .is_err()
        );
        db.execute(
            "UPDATE messages SET body = ? WHERE message_id = ?",
            libsql::params![metadata.body.as_str(), metadata.message_id.as_str()],
        )
        .await?;
        assert_eq!(
            db.record_direct_review_claims(&metadata, &input, 6).await?,
            1
        );
        assert_eq!(db.current_human_review_count(1).await?, 1);
        let ambiguous = String::from_utf8(raw.to_vec())?.replace(
            "Subject: Re:",
            "Subject: Re: [PATCH] change\r\nSubject: Re:",
        );
        let (_, _, ambiguous_input) = parse_ingested_mail(ambiguous.as_bytes(), true)?;
        let ambiguous_input = ambiguous_input.context("Enabled ambiguous source")?;
        assert!(ambiguous_input.claims.is_empty());
        db.record_direct_review_claims(&metadata, &ambiguous_input, 7)
            .await?;
        assert_eq!(db.current_human_review_count(1).await?, 0);
        assert_eq!(
            db.record_direct_review_claims(&metadata, &input, 8).await?,
            1
        );
        assert_eq!(db.current_human_review_count(1).await?, 1);
        db.execute(
            "UPDATE patches SET diff = 'different patch' WHERE id = 1",
            (),
        )
        .await?;
        assert_eq!(db.current_human_review_count(1).await?, 0);
        assert_eq!(
            db.record_direct_review_claims(&metadata, &input, 9).await?,
            0
        );
        assert_eq!(db.current_human_review_count(1).await?, 0);
        let count: i64 = db
            .conn
            .query("SELECT count(*) FROM human_review_evidence", ())
            .await?
            .next()
            .await?
            .context("Evidence count")?
            .get(0)?;
        assert_eq!(count, 3);
        let wrong_version = String::from_utf8(raw.to_vec())?
            .replace("<patch@example.invalid>", "<patch-v2@example.invalid>");
        let (wrong_metadata, _, wrong_input) = parse_ingested_mail(wrong_version.as_bytes(), true)?;
        db.create_message_with_references(
            &wrong_metadata.message_id,
            thread_id,
            wrong_metadata.in_reply_to.as_deref(),
            &wrong_metadata.author,
            &wrong_metadata.subject,
            wrong_metadata.date,
            &wrong_metadata.body,
            "",
            "",
            None,
            None,
            None,
        )
        .await?;
        assert_eq!(
            db.record_direct_review_claims(
                &wrong_metadata,
                &wrong_input.context("Enabled input")?,
                7
            )
            .await?,
            0
        );
        assert_eq!(db.current_human_review_count(1).await?, 0);
        Ok(())
    }
}
