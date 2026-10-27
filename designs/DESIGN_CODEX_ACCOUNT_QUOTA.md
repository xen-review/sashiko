# Codex Subscription Quota Admission

Status: selected for incremental implementation under the operator's 2026-10-27
no-ask directive. The first increment is a disabled, pure five-hour admission
policy with deterministic tests, followed by a bounded metadata-only reader.
Neither is wired into a controller or scheduler, model-call authorization, or
overnight watcher.

## First Implementation Checkpoint

The pure policy lives beside the unchanged QuotaManager in src/ai/quota.rs.
It exposes account-scoped typed admission state and a separate FiveHourReady
decision, not general spending authorization. Time and samples are supplied
explicitly; it creates no task, network request, sleep or process. Disabled
evaluation bypasses the gate while preserving the caller's retained pause.
All five fake-clock tests and make check-pr pass, including 1,150 library tests
(two ignored), 49 main, seven CLI and 28 integration tests. This is caller-carried
state testing, not proof of database restart recovery. No runtime caller or
settings toggle is installed, and the deployed daemon remains unchanged.

## Metadata Decoder Checkpoint

The next pure decoder is in src/ai/codex_cli.rs. A bounded metadata-only schema
read verified type=chatgpt, 300-minute primary and 10,080-minute secondary windows,
usedPercent/resetsAt, and the codex limit identity. The observed endpoint exposes
email and planType, not an opaque account/workspace ID. At 02:10 CET the readings
were 35% five-hour and 53% weekly remaining; no model turn was started.

The decoder requires matching account/limit plans, complete known windows,
finite measured percentages, future resets and conservative false paid/spend
flags. Missing, malformed, metered, expired and unsupported results are errors.
An explicit weekly zero stays zero rather than becoming unknown or spendable.
The derived SHA-256 login/plan key is a local comparison key, not proof of an
allowance-owning workspace. Do not persist or enable cross-process admission on
that key without resolving this identity limitation and authentication changes
between account/rate-limit reads. No raw account identity or credentials are
logged. The decoder takes supplied JSON only; it launches no process and makes
no model call. Its fake-response checks and full make check-pr gate pass:
1,151 library tests (two ignored), 49 main, seven CLI and 28 integration tests.
Process/RPC bounds, verified runtime binding, weekly reserve and durable state
remain unimplemented integration gates.

The decoder is committed as f6411372, following policy 88243c76. Final fresh
metadata at 02:20 CET reports 30% five-hour and 52% weekly remaining, with the
05:03 CET reset unchanged. Neither foundation is wired into production admission.
Current endpoint identity limitations must be resolved without reading raw auth
tokens or assuming a login/plan hash uniquely identifies an allowance workspace.

## Bounded RPC Reader Checkpoint

The next increment adds read_codex_subscription_quota with a required expected
local account key and a fixed 20-second deadline. It launches app-server --stdio
in a temporary isolated directory, sends only initialize/initialized,
account/read, account/rateLimits/read, then account/read again. The before/after
login and plan must remain bound to the expected key; a login/plan match still
does not establish opaque workspace identity. No model-turn method is available
through this reader and no runtime admission caller is added.

The subprocess has a 1 MiB cumulative stdout limit, 64 messages and 64 KiB per
JSON line. It rejects unexpected IDs, missing results, unsolicited server
requests, RPC errors, malformed output and account switches with payload-free
errors. Stderr is discarded, so credentials or server diagnostic payloads do
not enter logs. Timeout/error/success paths kill and reap the child; cancellation
uses kill-on-drop, verified with a Linux process fixture. Four fake-executable
tests pass for the exact method allowlist, isolated directory, measured windows,
notification flooding, oversized/malformed output, RPC errors, early exit,
timeouts, identity changes, payload privacy and cancellation cleanup. No live
model call or scheduler activation is needed. The full make check-pr gate passes
1,157 library tests (two ignored), 49 main, seven CLI and 28 integration tests,
formatting and all Clippy profiles. Disk pressure required package-scoped
debug-artifact cleanup; dependencies, release artifacts, operational backups and
review data were preserved. No production deployment is needed for this unwired
observation API and the installed maintenance repair stays unchanged.

Stable workspace binding, weekly reserve policy, validated runtime settings,
persisted state, wakeup scheduling and paused-state API/UI remain later gates.
This reader neither redeems credits nor enables metered billing or automatic
effort adaptation. The installed daemon continues its existing admission policy.

## Premium Quota Predicate

The separate has_fresh_premium_quota predicate accepts only the private-field
workspace snapshot produced by the workspace decoder or reader, not the older
login-only snapshot. It compares a caller-supplied account key, observation start
and current time. It requires an observation from that query no more than 20
seconds old, unexpired five-hour and weekly windows, strictly more than 50%
five-hour remaining and positive weekly remaining. Invalid times, mismatched
identities, expired windows and exhausted allowances return false. This is
separate from the lower-threshold scheduler policy and does not change its
decisions.

This predicate is unwired and does not authorize a model call. A caller must
actually initiate a fresh metadata read for each call, verify subscription
authentication and billing, establish an account binding, check the permitted
model and effort, and establish enough weekly and time reserve for the bounded
review. The older login/plan key cannot prove opaque workspace identity, and
positive weekly allowance alone does not prove sufficient reserve. Tests use
decoded fake metadata and do not invoke a model or enable the daemon.

## Workspace-Bound Reader Checkpoint

The earlier metadata inspection missed fields outside the account object. At
20:03 CET on 2026-10-27, authenticated account/read workspaceRouting reported
chatgptAccountId and account/rateLimits/read reported accountId. Two successive
reads had stable matching identifiers and ordinaryUsageAllowed was true. The
identifiers were compared without logging their values; no model turn was made.

The new opt-in workspace reader derives a separate key from the validated login,
plan and opaque workspace ID. It requires the workspace key to match before and
after the quota read, the quota account ID to match that workspace, and explicit
ordinaryUsageAllowed=true. It also retains the existing subscription, measured
window and no-paid-credit checks. Missing or malformed identity, a workspace
switch, mismatched quota identity, denied or unknown included-usage permission,
and paid credits fail closed. Existing login-bound callers remain unchanged.
Fake-process tests cover successful reads and changed or denied metadata.

This does not establish that a later model invocation uses the same workspace,
or wire the reader into scheduler or provider admission. Account changes after
the metadata read, weekly and time reserves, retries, runtime configuration,
and durable pause behavior remain separate gates. No raw IDs are persisted or
logged by this reader and no automatic review is enabled by this checkpoint.

The saved targeted Terra replay's session creator_account_id matched the current
workspace and quota IDs in a metadata-only comparison. Codex documents
forced_chatgpt_workspace_id and forced_login_method as login restrictions, but
also says a mismatch logs the user out and exits. That makes a negative test or
blind production use against shared credentials unsafe. The session ID comparison
occurs after session creation and cannot alone prevent a wrong-account call.

At 20:28-20:29 CET on 2026-10-27, metadata-only app-server probes started one
ephemeral and one persistent thread with native tools disabled. Both selected
gpt-6.1-terra / HIGH, reported zero turns and emitted no turn events; neither
received a turn/start request. The ephemeral response exposed no account ID.
The persistent response named a session path, but no session file or creator ID
existed before the first turn. The tested pre-turn responses therefore do not
provide a model-execution account binding. The persistent artifact is retained
if Codex later materializes it; no forced-workspace mismatch was attempted.

## Evidence and Ownership

The authenticated Codex app-server metadata reader already reports the shared
five-hour and weekly windows without a model turn. At 00:57, 01:38, and 01:53
CET on 2026-10-27, shared five-hour remaining allowance was 67%, 51%, and 43%.
The last reading reported 54% weekly and a five-hour reset at 05:03:09 CET.
Other consumers contribute to these percentages. They are not exact token
counts, an isolated Sashiko burn rate, or a reason to drain the allowance.

The existing ai::quota::QuotaManager handles transient request backoff, capped
at five minutes. BackoffProvider owns rate-limit retries. Neither is a durable,
account-specific subscription admission policy. Do not lengthen transient
backoff into a reset scheduler or turn subscription exhaustion into a retry
storm. Keep per-review token/tool/call limits independent of account resets.

## Selected Stages

1. Add a pure policy beside the existing quota abstraction, disabled by default.
   Supply time and samples explicitly; never sleep or read the network inside
   it. Preserve the existing QuotaManager unchanged. Test decisions before any
   scheduler integration or settings promise.
2. Add an authenticated metadata-only account adapter with bounded RPC, process,
   time and output limits. It must establish subscription-backed authentication,
   account identity, five-hour and weekly readings, freshness and reset times.
   Reject API-key/metered/unknown billing for this subscription-only policy.
   Do not expose raw credentials or start a model turn to read quota.
3. Add validated Codex-specific settings and persisted per-account admission
   state before enabling scheduler integration. Keep disabled behavior exactly
   as it is today. Gate new automatic reviews only at admission boundaries;
   never stop an in-flight request or automatically replay completed work.
4. Integrate one bounded wakeup path and paused-state API/UI. Record the account
   key, reason, reset barrier, next refresh and last credible sample. Resume only
   after fresh authenticated evidence. Runtime activation is a separate explicit
   deployment decision, not a consequence of adding the pure policy.
5. Extend the same state with typed subscription-exhaustion deferral. Preserve
   stages/checkpoints, cancellation and diagnostics; do not mark failed analysis
   successful or silently restart terminal harness sessions. Test durable resume
   and usage attribution before any separately authorized live exhaustion trial.

## Disabled Settings Checkpoint

The optional Codex account-quota table uses the existing pure policy defaults
and validation, including strict threshold ordering and bounded durations.
Absent configuration preserves legacy behavior. A present disabled table is
validated but has no runtime effect. Until account binding, durable state and
scheduler integration are implemented, loading enabled = true must fail with an
explicit unsupported-activation error; silently accepting it would falsely
promise quota protection. Unknown fields and invalid values are rejected.

This checkpoint adds no quota read, timer, pause, provider admission or review
retry. Settings-loader tests cover omitted and disabled tables, defaults,
custom values, invalid thresholds and durations, unknown fields, and rejected
activation. Activation remains a later explicitly verified integration step.

## Durable State Checkpoint

Store one bounded state row per workspace-derived account key. Only a key obtained
from a validated workspace quota snapshot may be used by the storage API; neither
the raw workspace identifier nor login or credentials are stored. Persist the
typed ready/paused state, reason, reset barrier, next check, and latest credible
five-hour sample when available. Absence of a sample is unknown, not zero.

Every row carries a monotonically increasing revision. Creation and replacement
use compare-and-swap writes so a decision based on an older row cannot overwrite
a newer pause. Validate account consistency, state and timestamp invariants on
both writes and reads; malformed persisted state fails closed rather than becoming
ready. Writes accept samples only from a matching validated workspace snapshot
whose observation is fresh and unexpired. Preserve the previous sample when
telemetry is unavailable, and never replace it with an older observation.
Exhausted revisions fail rather than wrap.

This is storage only; callers supply the state, and storage does not evaluate the
policy thresholds. A retained ready row never authorizes a model call, and a
fresh authenticated observation and the separate premium, execution-account,
reserve, scheduler and recovery gates remain mandatory. No controller, timer,
provider call, queue mutation or historical reconstruction is enabled. Tests must
cover migration, process reopen, account isolation, stale competing writes,
sample preservation, malformed rows, reset barriers and revision overflow.

## Pure Decision Contract

Account-scoped state is either Ready or Paused, with a typed reason, earliest
resume time and next check time. A sample names the same opaque account key and
contains its observation time, measured five-hour percentage and reported reset.
Account identifiers are not tokens. A sample from another account cannot resume
this one. Policy decisions do not authorize a premium model or paid overage.

Defaults are pause below 20%, resume above 30%, sample maximum age 300 seconds,
and unavailable-telemetry retry 60 seconds. Require finite percentages in 0..100,
ordered thresholds in that range, and positive bounded refresh/age durations.
Reject invalid enabled configuration rather than clamp it silently.

When Ready, a fresh valid sample at exactly 20% does not pause; below 20% does.
When Paused, exactly 30% does not resume; above 30% may resume only after the
stored reset barrier and a sample observed at or after that barrier. Reaching a
wall-clock reset is never proof that quota reset or that another consumer left
enough. A late wakeup still needs a fresh sample above the upper threshold.

Low quota records the reported reset as an earliest resume barrier. New credible
samples observed before that barrier may move it later, even when their remaining
allowance is already above the resume threshold. They never move it earlier
within the same pause.
A shifted/late reset therefore cannot accidentally unlock the account. Unknown,
stale, future-dated, malformed, wrong-account or already-expired-window samples
pause admission and schedule a bounded retry without busy polling. Preserve an
existing later barrier. Do not reuse a pre-reset high sample after reset.

Once the barrier passes, a fresh reading at or below 30% keeps the pause and
adopts its next credible reset. A fresh reading above 30% opens admission.
While waiting, the next check is the future barrier; unavailable telemetry after
the barrier uses a bounded retry. All timestamp arithmetic must be checked or
saturating. An injected clock makes thresholds and late wakeups deterministic.

When disabled, report Bypassed and leave the saved state intact. No quota read,
timer, scheduling, active-call, model/effort, retry, delivery or posting change is
implied. Re-enabling must not discard a retained pause barrier.

## Runtime Boundaries Still Required

- The metadata adapter must apply separate weekly allowance/debugging reserve
  gates; the first pure five-hour policy is not a complete budget allocator.
- Validate configuration without recompilation, persist state across restarts,
  and bind it to provider/account before admitting real work.
- Use one scheduled wakeup with bounded metadata refreshes, not polling sleeps
  or service restarts. Refresh after reset and before resuming.
- Cancelled work stays cancelled. A reset cannot recreate a cancelled task or
  weaken per-review limits. Expose an honest paused reason and next check time.
- Preserve muted delivery, access controls, successful results and original
  failure artifacts. Do not refill an exhausted queue just to consume budget.
- No premium trial at 50% or below; no self-review, live exhaustion trial,
  credits, metered billing, paid overages or overnight job follows from this
  design. Separate spending and operating-window gates remain mandatory.

## Design Checks and Verification

The design separates observation, pure admission, scheduling and continuation,
so missing data cannot require a model call and a pause cannot cancel active
analysis. A persisted earliest-resume barrier prevents restarts, stale samples
or clock/reset assumptions from bypassing hysteresis. Keeping the first module
unwired makes its disabled behavior falsifiable without installing a watcher.

Use the existing quota test home. Cover 19.9/20/20.1 and 29.9/30/30.1 percent,
fresh/stale/future observations, exact reset and late wakeups, changed resets,
another account, another consumer, missing/invalid readings, invalid settings,
disabled state retention and timestamp overflow. Subsequent runtime work must
add fake-provider/RPC tests, restart recovery, cancellation, no duplicate calls
or usage, API/UI pause visibility and all existing deterministic gates.