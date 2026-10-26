ALTER TABLE reviews ADD COLUMN review_mode TEXT NOT NULL DEFAULT 'workflow'
    CHECK (review_mode IN ('workflow', 'agent_harness'));
ALTER TABLE reviews ADD COLUMN independently_verified INTEGER
    CHECK (independently_verified IN (0, 1));

CREATE TABLE review_patch_coverage (
    review_id INTEGER NOT NULL REFERENCES reviews(id) ON DELETE CASCADE,
    patch_id INTEGER NOT NULL REFERENCES patches(id) ON DELETE CASCADE,
    commit_id TEXT NOT NULL CHECK (length(commit_id) = 40),
    PRIMARY KEY (review_id, patch_id)
);
CREATE INDEX idx_review_patch_coverage_patch ON review_patch_coverage(patch_id);