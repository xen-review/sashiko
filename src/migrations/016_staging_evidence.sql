CREATE TABLE staging_snapshots (
    id INTEGER PRIMARY KEY,
    repository TEXT NOT NULL,
    remote TEXT NOT NULL,
    target_ref TEXT NOT NULL,
    tip TEXT NOT NULL CHECK (length(tip) IN (40, 64)),
    refreshed_at INTEGER NOT NULL,
    checked_at INTEGER NOT NULL CHECK (checked_at >= refreshed_at AND refreshed_at > 0),
    algorithm_version INTEGER NOT NULL,
    scope_complete INTEGER NOT NULL CHECK (scope_complete IN (0, 1)),
    diff_bytes INTEGER NOT NULL CHECK (diff_bytes >= 0)
);

CREATE TABLE staging_commit_index (
    repository TEXT NOT NULL,
    algorithm_version INTEGER NOT NULL,
    commit_id TEXT NOT NULL CHECK (length(commit_id) IN (40, 64)),
    identity_json TEXT NOT NULL,
    patch_identity TEXT,
    computed_at INTEGER NOT NULL,
    PRIMARY KEY (repository, algorithm_version, commit_id)
);
CREATE INDEX idx_staging_commit_cache ON staging_commit_index(repository, algorithm_version, computed_at DESC);

CREATE TABLE staging_snapshot_commits (
    snapshot_id INTEGER NOT NULL REFERENCES staging_snapshots(id) ON DELETE CASCADE,
    commit_id TEXT NOT NULL,
    patch_identity TEXT,
    PRIMARY KEY (snapshot_id, commit_id)
);
CREATE INDEX idx_staging_snapshot_identity ON staging_snapshot_commits(snapshot_id, patch_identity);

CREATE TABLE staging_patch_evidence (
    snapshot_id INTEGER NOT NULL REFERENCES staging_snapshots(id) ON DELETE CASCADE,
    patch_id INTEGER NOT NULL REFERENCES patches(id) ON DELETE CASCADE,
    source_identity TEXT,
    PRIMARY KEY (snapshot_id, patch_id)
);

CREATE TABLE staging_targets (
    repository TEXT NOT NULL,
    remote TEXT NOT NULL,
    target_ref TEXT NOT NULL,
    snapshot_id INTEGER REFERENCES staging_snapshots(id),
    last_failure TEXT,
    failed_at INTEGER,
    PRIMARY KEY (repository, remote, target_ref)
);