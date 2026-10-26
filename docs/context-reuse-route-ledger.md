# Context Reuse Route Ledger

Updated: 2026-10-26. This is the execution ledger for the selected route;
the general improvement ledger retains other independent tasks.

## Current Handoff

Authoritative resume summary at 15:36 CET on 2026-10-26. Older milestone and
resume entries below are historical evidence, not current quota readings or
deployment claims. The user switched the coding chat provider; do not
infer daemon authentication, billing, quota, or model from that switch.

- Last delivered UI commit: 4cbed0e1, signed and pushed to
  origin/add-xen-support. Work in the clean detached implementation worktree
  /tmp/sashiko-improvements-20261026. Main contains unrelated untracked user
  agents/skills/CI files; preserve them and never stage or revert them.
- Installed binary remains 82905979, with timeout profiles. Live database schema
  is 14; checkpoint migration 15 and subsequent source changes are not deployed.
  Runtime uses Codex gpt-6-luna/medium with muted outbound delivery. The service
  was stopped at an idle boundary for required checks and must be resumed after
  delivery. Latest check: 95 Reviewed, no Pending or In Review series.
  Do not cancel a later active review just to repeat checks.
- Completed source: migration 15; atomic invocation snapshots, ordered checkpoint
  artifacts and disjoint call interactions; terminal success/failure/cancellation;
  no-replay startup recovery; bounded internal sequential runner; batched review
  detail/list API enrichment and partial-result UI. Daemon sequential dispatch
  now persists before advancing and uses a fresh revision-bound toolbox per
  checkpoint, preserving shared conversation, limits, and cancellation.
  Full latest gate: 1,122 library tests, 48 main,
  seven CLI, 25 integration, formatting, and all three Clippy profiles passed.
- Sequential configuration still fails closed. The installed daemon remains
  workflow mode. Existing independent workflows and whole-series harness remain
  supported; no sequential trial, new default, or transparent crash resume is
  claimed. See the approved checkpoint design for the partial-completion policy.
- UI rendering passed syntax, escaped injection, active/Failed/Cancelled/Reviewed
  fixtures, desktop/mobile overflow and header checks, raw-log access, and the
  existing disclosure toggle. The synthetic fixture server has been stopped.
- API checkpoint_session contains state, completed_count, provider_calls_recorded,
  nullable provider_calls_attempted, usage_complete, ordered checkpoints,
  selected_patch_indexes, and remaining_patch_indexes. Top-level token totals
  are sums of disjoint recorded interactions. Full invocation input is omitted
  from list payloads. No terminal usage is invented for interrupted calls.
- Daemon fake-provider tests now cover one selected/context-only patch and two
  real commits, source reads through virtual HEAD, persisted state before the
  next call, reversed selection order, malformed second output, supported force
  cancellation, retained diagnostics on explicit rerun, and no delivery.
  Fixed exposed integration gaps: stale toolbox HEAD cache across checkpoints;
  unfinished selected patches not marked active; rerun cleanup discarding
  checkpoint diagnostics; nondeterministic ordering of same-second reviews.
- Local sequential dispatch now retains partial JSON/human/agent artifacts and
  graceful cancellation. Unit tests cover two ordered checkpoints, malformed
  second output, cancellation, shared totals, and unknown terminal call usage.
  The latest full gate passes with 1,123 library, 48 main, seven CLI, and 25
  integration tests plus formatting and all three Clippy profiles.
  Next inspect harness scheduling coverage and enable configuration with real
  CLI integration fixtures; ordinary per-patch review counts do not yet
  recognize series checkpoint coverage.
  Enable the typed strategy only when these paths and partial-result UI pass.
- Before installing schema 15, stop at a safe boundary, verify workers exited,
  back up the database, and run required checks. Scheduled/external service
  starts have occurred during previous maintenance; recheck actual processes.
  Do not deploy an unfinished sequential feature or claim source is installed.
- Workspace search tools search main by default. For uncommitted implementation
  code, read its absolute path or run targeted rg in the implementation worktree;
  otherwise line numbers and content can misleadingly reflect the older checkout.

Validation environment: use four Cargo jobs and local Git 2.56.0 with
GIT_TEMPLATE_DIR=/usr/share/git-core/templates,
CARGO_TARGET_DIR=/home/review/gh/sashiko/target, and
CARGO_BIN_EXE_sashiko=/home/review/gh/sashiko/target/debug/sashiko. Use
make check-pr before every code commit, git commit -s with wrapped messages,
fast-forward main while preserving user files, and push only the authorized
branch without force. No PR or Sashiko self-review is authorized.

Budget policy is in the updated Sashiko-auto-improver agent. Premium Codex
subscription trials require a fresh authenticated strictly-above-50% five-hour
reading before every new call, verified exact models, sufficient weekly/time
reserve, low/medium starts, and a high effort cap. No metered billing or paid
overages. Last trial readings at 14:01 CET were 100%/86% with reset near 19:01
CET; they are stale now. Re-read rather than assuming a reset or allowance.
Terra/low and GPT-5.6 Sol/low exact-prompt format replays passed, but returned
empty arrays versus Luna/medium's evidence-backed dismissal. Lower output is
not equivalent review quality or justification for broad Sol use. Artifacts
remain under /tmp/sashiko-effort-trial-20261026; daemon default was not changed.

Other ledger work remains: staging merge/status filtering, live/final usage,
configurable quota hysteresis, and durable subscription-exhaustion deferral.
The overnight pause/resume trial is lower priority, test-first, near a reported
00:00-06:00 local reset only with fresh authorization; no watcher is installed.

### Continuation Update

At 15:10 CET the user confirmed the coding-chat plan switch and requested no
clarifying questions until another message/new session. Record conservative
recommended assumptions instead of treating predictions as prior approval;
existing budget, architecture, posting, and self-review restrictions still apply.
The next UI edit described above is now implemented: retained checkpoint notes
and drafts render for active, Failed, Cancelled, and Reviewed sessions alongside
completed/selected counts, remaining scope, and explicit incomplete usage.
Implementation assumption: reuse existing review cards, styling, and disclosure
controls rather than add another page or redesign the application.

Inline JavaScript syntax and rendering checks passed, including legacy harness
compatibility and escaped injection text. Isolated browser fixtures at 1280px
and 390px showed no page/card overflow or header overlap, no injected elements,
all raw-log links, no false review-map links, and a working notes toggle. Visual
snapshots were inspected. The synthetic fixture never touched the live database.
The immediate next task is daemon/local dispatch; sequential startup stays gated
until its persistence and cancellation integration tests pass. No sequential
mode or schema-15 deployment is claimed by this frontend change.

## Decision

Finish the bounded whole-series harness as the nearest usable milestone, then
use that foundation for sequential per-patch checkpoints with retained context.
Keep the independent workflow as the compatibility default. Do not implement a
second session engine or rewrite the existing multiphase workflow first.

Whole-series execution is nearest because instruction snapshots, bounded source
tool execution, token accounting, and per-patch reply contracts already exist.
Sequential per-patch execution is the preferred follow-on for operator control,
patch-level progress, and carrying source evidence across review boundaries.
Neither alternative has demonstrated better review quality on Luna yet.

Earlier quota samples below are historical and cannot authorize new calls.
Follow the current handoff and updated agent budget policy. Do not equate quota
percentages with tokens, assume a reset, or promise completion within a window.

## Ordered Milestones

1. Done: fail-closed typed settings, ordered immutable instruction snapshots,
   a bounded read-only session engine, and exact series/reply identity contracts.
   Delivered in 76439f00, ad34e99f, 02b8345b, and 62d00f14.
2. Done: compose series input, session execution, and artifact validation behind
  one tested entry point in agent_harness::series::review. A fake-provider test
  proves invalid input makes zero provider calls, a two-patch series completes
  in one call, and invalid final artifacts preserve raw output and usage without
  a repair call. Required gates passed: 1,108 library, 47 main, seven CLI, and
  24 integration tests, plus formatting and all Clippy profiles.
3. Done: local dispatch branches before per-patch workflows, with exact baseline
  and patch commits, selected scope, settings-relative instruction snapshots,
  explicit unverified artifacts, raw failure traces, and model attribution.
  A fake Codex executable proves a two-patch range uses one provider call,
  no-AI uses zero, and malformed output is retained without retry or fallback.
  Required gates passed: 1,109 library, 48 main, seven CLI, 25 integration tests,
  formatting, and all Clippy profiles. Daemon harness mode remains rejected.
4. Done: atomic persistence is implemented with one series usage record,
  covered patch identities, explicit mode/verification metadata, and preserved
  cancellation. Tests cover rollback on the final write, invalid scope,
  instruction identity, duplicate completion, failed artifacts, and cleanup.
  Required gates passed: 1,110 library, 48 main, seven CLI, 25 integration tests,
  formatting, and all Clippy profiles. Cancellation now retains partial session
  history and invocation counts. Daemon execution branches once after existing
  eligibility filters, uses the shared model limiter, and persists without
  email/forge delivery or workflow retries. Fake-provider tests cover a selected
  subset, malformed output, cancellation, and one usage record per invocation.
  Dispatch gates passed 1,112 library, 48 main, seven CLI, 25 integration tests,
  formatting, and all Clippy profiles. Explicit reruns retain prior artifacts;
  daemon validation accepts only supported providers, snapshots, and limits.
  The UI displays escaped notes/drafts, unverified status, and one usage summary
  without a review-map claim. Browser checks at 1280px and 390px covered raw-log
  access, text injection, long source lines, overflow, and header wrapping.
  Final deterministic gates passed the same complete test and lint profiles.
5. Initial bounded Luna evaluation completed on 2026-10-26; broader quality
  comparison remains pending. Two runs on exact Xen commit
  9a6cc49b44b458c2ce4cac36ea6c411d06850ac5 retained full JSON and stderr under
  /tmp/sashiko-harness-luna-trial-20261026, outside the repository. Both used
  Luna/low, nine explicit instruction files, no response cache, and no delivery.
  The first completed with one provider call, zero tools, 70,223 input tokens
  (38,656 cached; 31,567 uncached), and 474 output tokens. Its honest diff-only
  caveat exposed confusion between disabled native tools and caller-executed
  source-tool JSON. A trial-only clarification produced two provider calls,
  three source calls, 62,986 input tokens (25,600 cached; 37,386 uncached), and
  631 output tokens. It inspected the exact changed function, parent, nearby
  caller, and ARM page declarations and reported no remaining concern. This is
  limited inspection, not independent verification or a quality benchmark.
  The second run used more uncached input despite less total input; neither
  run establishes savings or sequential cache reuse. The harness system prefix
  now explicitly explains caller-executed JSON source access, with the existing
  tool-continuation and serialized-prefix regression extended accordingly.
  Full deterministic gates passed: 1,112 library, 48 main, seven CLI, and
  25 integration tests, formatting, and all three Clippy profiles.
  No full/small benchmark or automatic queue refill is authorized by this plan.
6. Sequential checkpoint design drafted in
  [DESIGN_SEQUENTIAL_HARNESS_CHECKPOINTS.md](../designs/DESIGN_SEQUENTIAL_HARNESS_CHECKPOINTS.md).
  The operator approved preserving completed checkpoints on later failure,
  explicit partial-series state, and no automatic replay/session resume.
  Typed strategy selection is implemented with whole_series as the default;
  sequential_patches fails closed until continuation and persistence are wired.
  Focused settings regression and all full deterministic gates passed.
  The shared HarnessSession now owns the fixed prefix, read-only declarations,
  cumulative trace/limits, and original deadline. Whole-series execution uses
  the same checkpoint operation. Fake-provider tests cover two append-only
  checkpoints, shared invocation/input limits, unchanged deadline, terminal
  cancellation, and no calls after expiry. Full gates passed 1,113 library,
  48 main, seven CLI, and 25 integration tests, formatting, and all Clippy
  profiles. Per-patch artifact validation and atomic partial persistence are
  still required before enabling sequential strategy.
  Checkpoint context/request builders now preserve complete series context in
  a fixed contract and append exact selected patch, original message ID,
  applied commit, and parent identity. Tests cover parent attribution across
  unselected context patches, stable context, scope/identity substitutions,
  first-patch baseline, and oversize rejection. The existing artifact parser
  validates one-patch scope without another schema. Full gates passed 1,114
  library, 48 main, seven CLI, and 25 integration tests plus all lint profiles.
  Runtime sequential execution remains disabled pending atomic persistence.
  Completed provider calls now retain their individual usage and exact response
  history boundary alongside aggregate totals. Checkpoint tests verify disjoint
  boundaries, summed input totals, and older trace deserialization without
  invented call usage. The same full deterministic gates passed. This is a
  persistence prerequisite, not a completed database checkpoint implementation.
  Additive migration 15 now defines sequential invocation records, ordered
  patch checkpoints constrained to covered patches, and unique per-call
  interaction links. Schema tests reject out-of-scope and duplicate checkpoints
  and verify review-deletion cleanup. Historical rewind fixtures were updated
  to remove these objects before simulating older schemas. Full gates passed
  1,117 library, 48 main, seven CLI, and 25 integration tests, formatting, and
  all Clippy profiles. This schema foundation is not installed in the live
  database; atomic checkpoint writers, interruption recovery, and runtime
  dispatch remain pending. Sequential selection still fails closed.
  Startup recovery now terminalizes active sequential invocation markers in
  one transaction rather than requeueing them. Completed checkpoint artifacts
  and Reviewed patches are preserved; cancellation remains sticky. Tests cover
  rollback on the final status write, repeated startup, ordinary workflow
  recovery, and an explicit later rerun unaffected by the older interrupted
  marker. Full gates passed 1,118 library, 48 main, seven CLI, and 25 integration
  tests, formatting, and all Clippy profiles. Atomic checkpoint writers and
  dispatch remain disabled/pending, and this recovery change is not deployed.
  Invocation initialization now snapshots serialized context once, derives its
  SHA-256 identity, and requires active covered review/series ownership. Atomic
  checkpoint writes enforce the expected cursor, next selected patch, exact
  commit/message identity, unchanged instruction/context fingerprints, complete
  per-call accounting, and preserved cancellation. They store only new call
  interactions, artifact text, patch completion, and the cursor in one
  transaction. Tests cover duplicate/out-of-order writes, changed snapshots,
  substituted identities, impossible/missing usage, patch/series cancellation,
  final-cursor-write rollback, and two checkpoints with two calls counted once.
  Full gates passed 1,120 library, 48 main, seven CLI, and 25 integration tests,
  formatting, and all Clippy profiles. Terminal invocation completion and
  daemon/local sequential dispatch remain pending; runtime selection is still
  fail-closed and these storage changes have not been installed in the live DB.
  Terminal invocation completion now requires all selected checkpoints before
  success, retains earlier Reviewed artifacts on later failure/cancellation,
  stores only newly reported call interactions, and keeps raw failed or
  interrupted traces without inventing in-flight usage. Shared accounting
  helpers serve checkpoint and terminal writes. Tests inject final-state failure
  and verify rollback, repeated completion rejection, full-scope success, partial
  failure, external cancellation, and exact known-call totals. Server-only
  helpers are feature-gated; the minimal build and all full gates passed:
  1,121 library, 48 main, seven CLI, 25 integration tests, formatting, and all
  Clippy profiles. Dispatch and partial-result API/UI remain pending, so runtime
  sequential selection is still disabled and no live DB deployment was made.
  The internal SequentialReviewSession now seeds complete context once and
  advances ordered selected patch requests on the shared bounded engine. Fake
  providers prove selector order cannot reorder submission review, the entire
  serialized prompt remains append-only between checkpoints, prior replies
  remain available, wrong-patch output stops without a repair call, cumulative
  token overshoot retains trace/usage, and pre-cancellation makes zero calls.
  Runtime startup remains gated separately from internal budget validation.
  Full gates passed 1,122 library, 48 main, seven CLI, and 25 integration tests,
  formatting, and all Clippy profiles. API/UI and daemon/local dispatch are
  still pending; the runtime continues to reject sequential_patches selection.
  Review details and patchset review lists now batch-load checkpoint sessions,
  ordered notes/drafts, selected and remaining scope, and disjoint interaction
  token totals. Usage completeness and recorded versus attempted call counts
  remain explicit, including cancelled calls without terminal usage. List
  payloads omit full invocation input. Regression fixtures prove list/detail
  agreement across completion, later failure, and cancellation without changing
  ordinary whole-series output. All full gates passed with 1,122 library,
  48 main, seven CLI, 25 integration tests, formatting, and all Clippy profiles.
  UI rendering and daemon/local dispatch remain pending and configuration
  continues to reject runtime sequential selection.
  Retain
   instruction/tool declarations and complete series context as a stable prefix;
   append patch requests, prior replies, and source evidence in order. Bind each
   checkpoint to exact baseline, series revision, and selected patch identity.
   Do not blindly rerun prior completed patches or promote old conclusions to
   operator instructions. Ask before changing architecture beyond this route.
7. Compare quality and total usage on the same small series, then expose a typed
   configurable strategy. Keep phase-by-phase reuse as a later experiment rather
   than adding hidden planning/verification phases to custom-agent execution.

## Cache and Correctness Gates

- Implemented: harness tool definitions and response instructions now live in
  the fixed system prefix, before append-only conversation history. A fake
  provider checks that the complete serialized prompt from one tool turn is an
  exact prefix of the next, with one copy of the tool catalog. Existing workflow
  CLI prompts retain their ordering. This establishes input stability only;
  actual provider cache hits and review quality still require measurement.
- Retained history is not free input. Cached tokens still count in reported
  input thresholds; provider cache availability and retention are not guaranteed.
- Keep system instructions, tool declarations, and prior messages byte-stable
  between turns. Add new evidence at the end instead of rebuilding summaries
  into the prefix. Test this invariant with a fake provider before paid trials.
- Distinguish model prefix caching from completed-response caching. A response
  cache hit cannot demonstrate a new model invocation's prefix hit ratio.
- Compare cached/input ratios together with total uncached input, output,
  latency, findings correctness, missed issues, and evidence reuse. A larger
  repeated prompt can improve the ratio while increasing total cost.
- Bound history growth and plan restart/checkpoint semantics before sequential
  execution. Summarization changes the prefix and can lose evidence; it requires
  explicit attribution and measured benefit, not an assumed free optimization.
- Preserve failed/cancelled traces and actual usage. Do not duplicate a shared
  investigation's usage across patch records or label provider calls as backend
  model generations when the backend may perform internal turns.

## Resume Notes

Deployment: f6213600 installed with four build jobs after deterministic checks.
The stopped database was backed up outside the repository before migration to
schema 14. API health returned 200 and a normal Luna/low workflow completed
patch 2 of the nospec always_inline series. Patch 3 began preparation before
the stop completed, so it was interrupted rather than untouched. A separate
12:00 service start resumed normal workflow; it was not an automatic crash
restart. The later test stop used another completed-patch boundary. Runtime
harness mode has not been enabled and no real harness trial has run.
The stable-prefix follow-up eb6e81ee was also installed after the same full gate
passed. No additional schema change or model request was needed for that install.

Quota observations (CET): 63% remaining at 10:32, 54% at 10:52, 44% at 11:22,
29% at 11:48, and 25% at 11:57; reset reported for 14:00. Coding and daemon
reviews share the account, so these samples do not isolate either consumer's
cost. Pause new review work to retain a debugging reserve, re-read quota before
resuming, and do not assume a reset or promise an unattended resume watcher.
The fresh 12:10 reading was 15% five-hour remaining and 87% weekly. The daemon
was already stopped at a completed-patch boundary; leave it stopped to preserve
the reserve. This pauses the API/UI too, since there is no separate reviewer-only
pause control yet. No watcher remains and no automatic resume has been scheduled.
These reserve-stop instructions were superseded by the user's subsequent
authorization to use the remaining Codex allowance for Sashiko testing. A fresh
12:35 CET account reading showed 12% five-hour and 86% weekly remaining, with
reset at 14:00:02 CET. The daemon was already active with Luna at low effort;
leave its active review intact. Temporary Copilot CLI fallback is authorized
only if Codex reaches zero before reset, subject to verified Luna availability
and billing. This does not authorize Sashiko self-review or a Sol substitution.
The configurable 20%/30% hysteresis remains a future ledger task,
not an implemented automatic feature. Next work is a separately bounded Luna
harness evaluation when authorized and budget permits, then design of sequential
patch checkpoints; neither was claimed complete by these deterministic checks.

Work in the clean detached implementation worktree
/tmp/sashiko-improvements-20261026. The main checkout contains unrelated user
CI and agent edits; never stage or revert them. Fast-forward main and push only
origin/add-xen-support after signed commits and required checks.

The checked implementation supports opt-in harness mode; runtime settings still
select workflow and no real harness trial has run. Preserve mute_all and SMTP
policy. Do not stop an active review for routine checks. Before tests, stop at
a safe boundary, verify workers exited, and resume the service promptly.

Use four Cargo jobs and the shared target directory. Full checks require the
local Git 2.56.0, GIT_TEMPLATE_DIR=/usr/share/git-core/templates, and the shared
CARGO_BIN_EXE_sashiko override for linked-worktree integration tests. Self-review
remains deferred under the explicit spending and final-only restrictions.