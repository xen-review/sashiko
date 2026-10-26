# Model and Effort Timeouts

Status: implemented and deterministically validated, 2026-10-26. Preserve existing timeouts when
no profile matches; do not change effort, model, or active requests automatically.

## Policy

Add optional Codex CLI timeout profiles keyed by exact model and typed reasoning
effort. A profile sets the provider call timeout, minimum review timeout, and
seconds per selected patch. The effective review deadline is the larger of the
minimum and selected patch count multiplied by seconds per patch. Reject zero
limits, duplicate model/effort keys, unsupported effort names, and arithmetic
overflow. Do not guess model aliases or use substring matching.

Matching profiles explicitly replace the old fixed provider timeout and overall
review/harness caps. Otherwise preserve the existing 600-second Codex call limit
and existing review/harness behavior. An eight-patch profile with 600 seconds
per patch permits an 80-minute series; it does not create eight independent
80-minute budgets. Source calls and checkpoint transitions share that deadline.

Initially apply profiles to the configured Codex CLI backend, where actual
effort and model are explicit and cache identity already includes effort. This
supports Luna, Terra, and Sol identifiers without granting permission to select
any model. Other backend effort plumbing remains unchanged and is not claimed
to honor these Codex-specific profiles.

For MAX experiments, provider and whole-review timeouts must each permit at
least 3,600 seconds. Xhigh/MAX profiles may use longer provider limits plus
600 seconds per patch. Longer limits do not authorize spending beyond the
operator window or remove cumulative token/tool/invocation limits.

## Implementation and Checks

Resolve the provider-call limit at provider construction. Resolve review limits
after eligibility is known for a whole-series harness, and use a single-patch
limit for independent workflow jobs. Local and daemon paths must agree; local
outer deadlines must not retain a shorter legacy cap. Keep cancellation sticky.

Use focused settings tests for exact matching, defaults, duplicate/zero limits,
unknown effort, eight-patch scaling, and overflow. A fake Codex executable must
verify configurable per-call timeout behavior without paid calls. Existing
whole-series and independent workflow tests must retain legacy behavior when
profiles are absent. Run required deterministic gates before code commits.

Validation passed: exact matching and legacy fallback, duplicate/zero/unknown
effort rejection, overflow checks, eight-patch scaling to 4,800 seconds, profile
override of both harness/review caps, and a fake Codex process whose blocked
stdin is terminated by its configured one-second deadline. Full gates passed
1,116 library tests, 48 main tests, seven CLI tests, 25 integration tests,
formatting, and all three Clippy feature profiles. No MAX model call was used
to validate timeout plumbing.