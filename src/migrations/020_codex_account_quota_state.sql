CREATE TABLE codex_account_quota_state (
    account_key TEXT NOT NULL PRIMARY KEY
        CHECK (length(account_key) = 64 AND account_key NOT GLOB '*[^0-9a-f]*'),
    revision INTEGER NOT NULL CHECK (typeof(revision) = 'integer' AND revision > 0),
    admission TEXT NOT NULL CHECK (admission IN ('ready', 'paused')),
    pause_reason TEXT CHECK (pause_reason IN ('low_allowance', 'awaiting_reset_evidence', 'telemetry_unavailable')),
    resume_not_before INTEGER CHECK (resume_not_before IS NULL OR (typeof(resume_not_before) = 'integer' AND resume_not_before >= 0)),
    next_check_at INTEGER CHECK (next_check_at IS NULL OR (typeof(next_check_at) = 'integer' AND next_check_at >= 0)),
    sample_observed_at INTEGER CHECK (sample_observed_at IS NULL OR (typeof(sample_observed_at) = 'integer' AND sample_observed_at >= 0)),
    sample_remaining_percent REAL CHECK (sample_remaining_percent IS NULL OR (sample_remaining_percent >= 0 AND sample_remaining_percent <= 100)),
    sample_resets_at INTEGER CHECK (sample_resets_at IS NULL OR (typeof(sample_resets_at) = 'integer' AND sample_resets_at >= 0)),
    CHECK (
        (admission = 'ready' AND pause_reason IS NULL AND resume_not_before IS NULL AND next_check_at IS NULL)
        OR
        (admission = 'paused' AND pause_reason IS NOT NULL AND next_check_at IS NOT NULL
         AND (resume_not_before IS NULL OR next_check_at >= resume_not_before)
         AND (pause_reason = 'telemetry_unavailable' OR resume_not_before IS NOT NULL))
    ),
    CHECK (
        (sample_observed_at IS NULL AND sample_remaining_percent IS NULL AND sample_resets_at IS NULL)
        OR
        (sample_observed_at IS NOT NULL AND sample_remaining_percent IS NOT NULL
         AND sample_resets_at IS NOT NULL AND sample_resets_at > sample_observed_at)
    ),
    CHECK (admission != 'ready' OR sample_observed_at IS NOT NULL)
);