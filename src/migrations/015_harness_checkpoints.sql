CREATE TABLE harness_checkpoint_sessions (
    review_id INTEGER PRIMARY KEY REFERENCES reviews(id) ON DELETE CASCADE,
    input_context BLOB NOT NULL,
    series_fingerprint TEXT NOT NULL CHECK (length(series_fingerprint) = 64),
    state TEXT NOT NULL DEFAULT 'active'
        CHECK (state IN ('active', 'completed', 'failed', 'cancelled', 'interrupted')),
    checkpoint_count INTEGER NOT NULL DEFAULT 0 CHECK (checkpoint_count >= 0),
    provider_calls_end INTEGER NOT NULL DEFAULT 0 CHECK (provider_calls_end >= 0),
    history_end INTEGER NOT NULL DEFAULT 0 CHECK (history_end >= 0),
    created_at INTEGER NOT NULL
);

CREATE TABLE harness_patch_checkpoints (
    review_id INTEGER NOT NULL REFERENCES harness_checkpoint_sessions(review_id) ON DELETE CASCADE,
    checkpoint_index INTEGER NOT NULL CHECK (checkpoint_index > 0),
    patch_id INTEGER NOT NULL,
    message_id TEXT NOT NULL,
    commit_id TEXT NOT NULL CHECK (length(commit_id) = 40),
    reviewer_notes BLOB NOT NULL,
    reply_body BLOB NOT NULL,
    history_end INTEGER NOT NULL CHECK (history_end > 0),
    provider_calls_end INTEGER NOT NULL CHECK (provider_calls_end > 0),
    created_at INTEGER NOT NULL,
    PRIMARY KEY (review_id, patch_id),
    UNIQUE (review_id, checkpoint_index),
    FOREIGN KEY (review_id, patch_id) REFERENCES review_patch_coverage(review_id, patch_id) ON DELETE CASCADE
);

CREATE TABLE harness_provider_calls (
    review_id INTEGER NOT NULL REFERENCES harness_checkpoint_sessions(review_id) ON DELETE CASCADE,
    provider_call INTEGER NOT NULL CHECK (provider_call > 0),
    response_history_end INTEGER NOT NULL CHECK (response_history_end > 0),
    interaction_id TEXT NOT NULL UNIQUE REFERENCES ai_interactions(id),
    PRIMARY KEY (review_id, provider_call),
    UNIQUE (review_id, response_history_end)
);