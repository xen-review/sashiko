ALTER TABLE messages ADD COLUMN human_review_revision INTEGER NOT NULL DEFAULT 0;
ALTER TABLE messages ADD COLUMN human_review_source_hash TEXT;
ALTER TABLE patches ADD COLUMN human_review_revision INTEGER NOT NULL DEFAULT 0;

CREATE TRIGGER human_review_message_changed
AFTER UPDATE OF message_id, author, subject, in_reply_to, body, date, git_blob_hash, human_review_source_hash ON messages
WHEN OLD.message_id IS NOT NEW.message_id OR OLD.author IS NOT NEW.author
  OR OLD.subject IS NOT NEW.subject OR OLD.in_reply_to IS NOT NEW.in_reply_to
  OR OLD.body IS NOT NEW.body OR OLD.date IS NOT NEW.date
  OR OLD.git_blob_hash IS NOT NEW.git_blob_hash
  OR OLD.human_review_source_hash IS NOT NEW.human_review_source_hash
BEGIN
    UPDATE messages SET human_review_revision = OLD.human_review_revision + 1 WHERE id = NEW.id;
END;

CREATE TRIGGER human_review_patch_changed
AFTER UPDATE OF message_id, diff, part_index ON patches
WHEN OLD.message_id IS NOT NEW.message_id OR OLD.diff IS NOT NEW.diff
  OR OLD.part_index IS NOT NEW.part_index
BEGIN
    UPDATE patches SET human_review_revision = OLD.human_review_revision + 1 WHERE id = NEW.id;
END;

CREATE TABLE human_review_evidence (
    id INTEGER PRIMARY KEY,
    source_id INTEGER NOT NULL REFERENCES messages(id) ON DELETE CASCADE,
    target_patch_id INTEGER NOT NULL REFERENCES patches(id) ON DELETE CASCADE,
    target_message_id INTEGER NOT NULL REFERENCES messages(id) ON DELETE CASCADE,
    source_revision INTEGER NOT NULL,
    target_revision INTEGER NOT NULL,
    target_message_revision INTEGER NOT NULL,
    extraction_version INTEGER NOT NULL,
    source_hash TEXT NOT NULL,
    target_diff_hash TEXT NOT NULL,
    reviewer_mailbox TEXT NOT NULL,
    trailer TEXT NOT NULL,
    body_line INTEGER NOT NULL CHECK (body_line > 0),
    recorded_at INTEGER NOT NULL,
    UNIQUE (source_id, target_patch_id, source_revision, target_revision,
            target_message_revision, extraction_version, source_hash)
);
CREATE INDEX human_review_by_patch ON human_review_evidence(target_patch_id, extraction_version);