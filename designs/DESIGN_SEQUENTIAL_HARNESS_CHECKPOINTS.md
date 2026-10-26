# Sequential Harness Checkpoints

Status: proposed for operator approval, 2026-10-26. This extends the approved
whole-series harness route, not the existing multiphase workflow. No runtime
default change, automatic retry, public posting, or effort adaptation is proposed.

## Motivation and Evidence

The whole-series harness already has bounded source tools, immutable instruction
snapshots, exact baseline and patch identities, retained traces, cancellation,
and one series usage record. Its serializer has a tested append-only prefix.
Two real Luna/low trials on one Xen commit demonstrated valid artifacts and,
after clarifying caller-executed JSON tools, three source requests across two
provider calls. They did not establish sequential reuse, general review quality,
or savings. The clarified trial used more uncached input than the diff-only run.

The operator prefers patch-level control while retaining prior investigation.
Add checkpoints to the existing engine rather than another tool loop. Compare
against whole-series and independent workflow execution before choosing defaults.

## Proposed Contract

Keep review.mode = workflow as the compatibility default. Within explicitly
selected agent_harness mode, introduce a typed strategy with whole_series as
the default and sequential_patches as an opt-in. Do not overload model, effort,
or prompt text to select execution semantics. Unsupported strategies fail closed.

Prepare the complete series and exact applied commits once. Preserve existing
eligibility filtering. Visit selected patch indexes in submission order, while
retaining unselected patches as context. Each checkpoint produces reviewer notes
and exactly one reply draft for the current selected patch. Validate its exact
index and original message ID locally, with no repair call. The cover remains
context, not a separate review checkpoint.

The system instruction, tool catalog, immutable full-series context, and artifact
contract remain byte-stable. The contract describes a one-patch checkpoint from
the start; do not rewrite the system schema for each patch. Append a separate
user message identifying the current patch and its exact parent and commit.
Retain prior assistant replies, tool requests, and tool results in order. Append
the next checkpoint request only after the prior result is validated and stored.

Earlier findings are attributed review evidence, never operator instructions or
proof of safety. Inspect each change against its own parent. A later patch fixing
an earlier defect does not erase that earlier finding; a later change invalidating
an earlier assumption must be called out in the later notes. Do not silently
rewrite earlier reply drafts or promote earlier assertions to verified facts.

## Shared Engine and Limits

Refactor the existing session state into an owned, serializable trace plus a
continuation operation. Whole-series execution remains a single use of that
operation. Sequential execution supplies the next checkpoint message and asks
for another terminal artifact using the same state. There is one source-tool
allowlist, parser, cancellation scope, and accounting implementation.

Generation, tool, input, output, and timeout limits apply to the entire series
invocation, not separately to each patch. Counters and the original deadline
never reset at checkpoints. Cached input still counts toward input thresholds;
one-call overshoot remains explicit. Missing usage is an error. Do not claim a
backend generation count when only provider calls are observable.

Reject continuation before starting a provider call if any cumulative limit is
reached. Preserve completed checkpoints and the failing checkpoint's raw trace.
Do not compact, summarize, drop tools, or silently restart history to fit limits.
An explicit bounded context-restart policy would require another design decision.

## Persistence and Recovery

Use one series invocation record plus ordered checkpoint records. Each record
binds invocation identity, patch ID/index, applied commit, original message ID,
artifact contract version, and the trace position ending that checkpoint. Store
instruction snapshot, full input identity, provider/model/effort, and strategy
once on the invocation. A later configuration edit cannot mutate that identity.

Commit each validated checkpoint and its covered patch status atomically before
starting the next. Do not mark the entire series Reviewed until all selected
checkpoints complete. If later work fails or is cancelled, previously completed
patches retain their artifacts and Reviewed state; the invocation/series exposes
partial completion and the remaining scope explicitly. Never report a partial
series as clean or independently verified. Preserve sticky external cancellation.

Persist provider interaction usage once per completed call, not as cumulative
copies on every checkpoint. Checkpoint totals are derived from disjoint trace
ranges; series totals sum those interactions once. Failed calls without terminal
usage remain explicitly unknown, not zero. Match immutable trace positions to
interaction identities so restart cannot double-count an already stored call.

For the first implementation, do not automatically resume a persisted provider
conversation after process loss. Mark its remaining scope interrupted and retain
all checkpoints and traces. An explicit rerun creates a new invocation and never
overwrites prior output. Resume across restarts is deferred until backend and
toolbox reconstruction identities can be verified. Normal queue recovery must
not blindly replay completed checkpoints from a partial sequential invocation.

## API and UI

Expose strategy, current selected patch, completed/remaining scope, terminal
partial state, and checkpoint artifacts. Keep independent verification false.
Render earlier completed checkpoints even when the invocation later fails or is
cancelled. Existing raw-log access and access controls remain applicable.

Display cumulative usage once, with optional per-checkpoint deltas labeled as
such. Distinguish provider calls from source calls and backend generations.
No email, forge comment, embargo delivery, or suggested source edit is enabled.

## Incremental Validation

1. Add the typed strategy and shared continuation state with fake providers.
   Existing whole-series and workflow fixtures must remain compatible. Assert
   complete serialized prompts retain the preceding prompt as an exact prefix
   across source rounds and checkpoint transitions.
2. Validate two selected patches plus an unselected context patch. Prove ordered
   exact-revision requests, prior evidence attribution, one draft per checkpoint,
   rejection of another patch's response, and no hidden repair or workflow calls.
3. Exercise cumulative limits across checkpoint boundaries, cancellation while
   awaiting a provider or source result, malformed later output, unknown usage,
   and instruction/series identity changes. No limit resets or silent truncation.
4. Add atomic persistence and migration fixtures. Inject failure between usage,
   checkpoint, patch, and series writes. Verify rollback, retained earlier output,
   idempotency, no duplicated usage, and explicit interrupted-state recovery.
5. Wire local/daemon dispatch and partial-state API/UI. Test existing eligibility,
   shared limiter release, reruns, access controls, muted delivery, and mobile
   presentation. Use required deterministic gates before each code commit.
6. Only with sufficient fresh quota, compare a bounded representative multi-patch
   series on Luna/low using whole_series and sequential_patches. Retain exact
   input identities, full traces, per-call cached/uncached input, output, elapsed
   time, tool requests, correctness, and missed concerns. No response cache hits
   count as provider-prefix reuse. Do not choose a default from cache ratio alone.

## Approval Boundary

Confirm the partial-completion policy before implementation: completed patch
checkpoints remain completed when a later checkpoint fails, while the series
reports partial failure; no automatic replay or cross-process session resume.
The alternative is all-or-nothing patch status with retained diagnostic drafts,
which is simpler but offers less useful patch-level progress. This design
recommends the explicit partial-completion policy above.