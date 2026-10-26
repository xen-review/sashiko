# Sashiko Improvement Ledger

Updated: 2026-10-26. Work in priority order, updating evidence and status as
changes are validated. Do not create work merely to keep the daemon busy.

## Operating Constraints

- Prefer `gpt-6-luna`, low effort by default; the runtime currently uses medium
  after the malformed-output recovery. Bounded premium Codex subscription trials
  are now authorized only under the agent's fresh strictly-above-50% gate.
  Start Terra/Sol at low or medium, never above high. A reset alone does not
  authorize broad Sol use, metered billing, or paid overages.
- Do not run Sashiko self-review through paid Copilot CLI. Do not spend Sol
  quota to investigate or validate these items without fresh authorization.
- Preserve muted outbound delivery, completed results, review artifacts, and
  user edits. Back up the database before maintenance; use supported retry APIs.
- Keep healthy reviews running. Stop the daemon and workers before tests or
  deployment, resume promptly, and verify actual forward progress.
- Validate with deterministic tests and fake providers before paid experiments.
  Follow AGENTS.md for sign-offs, checks, focused commits, and authorized pushes.

## Prioritized Work

### P1: Explain Review Results, Effort, and Confidence

Status: operator feedback recorded on 2026-10-26; first UI slice implemented.
See [the concrete report and acceptance checks](../designs/DESIGN_REVIEW_TRANSPARENCY.md).
The runstate-NULL v2 patch page hides a one-line inline result, while review map
48 retains one Low pre-existing compat concern and one dismissed native-path
item. Do not describe these as two final regressions or erase the original data.
The supplied GPT-6.1 Sol re-review identifies a likely false positive: the
constant-zero compat macro fallback is inactive where the x86 compat handler
is built. Confirm exact revision, CONFIG_COMPAT, and the object build rule.

Show concise final results immediately; explain map candidates, verification,
pre-existing status, dismissal and final-reply omission. Display actual thinking
effort as well as provider/model, with Unknown for missing historical evidence.
Separate missing token usage from zero. Add confidence rationale and optional
provenanced score/level without inventing calibrated probabilities, backfilling
historical guesses, or hiding low-confidence possible bugs. Preserve the
operator's recall preference. First inspect stored output and invocation evidence;
do not assume Luna means low effort or that current runtime settings applied.
No new premium review or source modification is authorized by this report.

The initial slice shows concise inline results directly and displays the same
stored outcome breakdown on cards/maps. Responsive map tables no longer widen
the page on narrow screens. Syntax, escaping, real review-48 rendering, mixed
dispositions, legacy cases, desktop/mobile overflow, and disclosure-handler
checks pass; make check-pr passes. No historical artifact was relabeled.
The saved baseline independently confirms the inactive compat fallback, and the
submitted patch leaves the header/build rule unchanged. Shared Xen guidance now
requires active-branch/object-build evidence and retains real non-default-build
findings; its prompt contract and full make check-pr gates pass. This is not
proof of universal false-positive elimination. The first attribution slice saves
provider/model/configured effort in workflow output at the real parent backend,
not its stdio worker proxy, and shows configured versus effective effort in
cards/maps. Historical missing effort and unconfirmed effective effort remain
Unknown. Snapshot/proxy/provider/escaping tests, full make check-pr, and narrow
desktop/mobile label checks pass. This configured-effort slice is deployed.
Optional model confidence with score/rationale, application-owned provenance,
uncalibrated labels, and historical-unassessed fallback is deployed at 17:50 CET
on 2026-10-26 after the context switch. Contract, projection, fake
workflow, escaping, full make check-pr, and final desktop/mobile fixture checks
pass; even a 1% assessment remains a finding, not a filter. The fixture is stopped
and no synthetic score entered the live database. A checked idle database backup
and rollback binary are retained; the installed hash matches the release build.
Fresh live-page/API checks preserve review 48's original results and show two
unassessed historical threads without overflow at desktop/mobile widths.
The first missing-usage presentation slice is deployed with e813b5f2:
cards/maps show zero-only unconfirmed legacy totals as Unavailable, preserve
confirmed checkpoint zeros, and retain known positive/partial counters without
defaulting missing values to zero. Stored data is unchanged and positive legacy
totals are not claimed complete. Focused rendering, syntax, editor diagnostics,
the diff-aware make check-pr gate, and read-only card/map browser checks at
desktop/mobile widths pass. The fixture is stopped and the daemon resumed.
The live patch/map checks preserve review 48's original data and no longer
present its unconfirmed counters as a measured zero. Backend-confirmed per-stage
effort and complete native-call/live telemetry remain open; see the report for
browser limits and the response-coverage scope below.

The adjacent availability-persistence slice now carries typed provider-response
coverage through success/failure sessions, stage/workflow aggregation, worker
results, maps, and full/concise local JSON, including the nested result used for
daemon persistence. It needs no migration or historical backfill. Unknown stages
and unaccounted worker restarts stay unknown, and truncation retains reported
usage. The UI and human output consume this metadata without calling response
counts native generations or claiming billing completeness. Malformed partial
coverage objects stay unknown. Focused contract, aggregation, serialization,
display, and desktop/mobile card/map/stage checks pass. The full make check-pr
gate passed 1,130 library tests (two ignored), 49 main, seven CLI, 25 integration,
formatting, and all Clippy profiles. The fixture is stopped and the daemon
resumed. This slice is deployed at 18:28 CET after an idle checked database
backup, matching release/installed hashes, and fresh desktop/mobile live-page
checks. Review 48's saved output and zero counters remain untouched; its UI says
Unavailable with unknown coverage. No model call was needed. Actual backend
effort and complete native-call/live telemetry remain separate open work.

A subsequent local parser check found that Codex CLI defaulted missing
input/output counters to zero and discarded explicit cached zero. The focused
fix requires measured counters, keeps cached unknown distinct from zero, and
rejects inconsistent/overflowed counts as unavailable. Response-cache identity
now separates the corrected parser profile from pre-fix synthesized totals.
Backend/fake-CLI tests pass; the full gate passed 1,132 library tests (two ignored),
49 main, seven CLI, 26 integration, formatting, and all Clippy profiles. The
fix is deployed as 96ab1ae0 at 19:11 CET after a checked idle database backup,
four-job release build, matching installed/release hashes, and a healthy API
check with unchanged review 48 data. No active review was interrupted. No model
call or historical backfill was used. The completion stream does not supply a
trustworthy effective-effort field, so the UI must retain Unknown rather than
promoting configured effort into backend confirmation.

### P1: Budget-Gated Premium Model Comparisons

Status: agent policy updated in 02453527 with the operator's authorization.
The separate Codex subscription account, not this coding chat's Copilot Business
plan, controls eligibility. Each premium call requires fresh authenticated quota
strictly above 50%, enough weekly allowance and operating time, verified model
availability, subscription authentication, and effective effort no higher than
high. Start at low/medium. Self-review and paid-billing restrictions are unchanged.

The bounded replay guard passed offline tests for 0%, 49%, exactly 50%, 50.1%,
51%, unknown/NaN readings, API-key authentication, and absent weekly allowance.
At 13:59 CET its live check denied an 8% account without a model call. After
14:00, a fresh account reading confirmed 100% remaining and subscription auth.
An initial trial was also denied because the daemon had restarted, preserving
its isolation rule. The idle daemon was then stopped for two bounded calls,
each independently rechecking quota at 100%; native turn metadata confirmed the
requested model and low effort. No active review was cancelled.

Exact same failed-stage prompt, verified by identical SHA-256:

| Model/Effort | Input, All Uncached | Output | Seconds | Dismissed Concerns |
| --- | ---: | ---: | ---: | ---: |
| gpt-6-luna / medium | 10,634 | 562 | 8.829 | 1 |
| gpt-5.6-terra / low | 10,082 | 105 | 9.821 | 0 |
| gpt-5.6-sol / low | 10,082 | 58 | 7.896 | 0 |

Each returned parseable envelope and stage JSON in one completed turn. The
Terra/Sol replies were only empty arrays; lower output volume does not establish
equivalent evidence or improved full-review quality. These are format replays,
not complete reviews or independent correctness checks. Do not infer prices,
general efficiency, or permission for unrestricted Sol use from these samples.
Artifacts remain outside the repository under the existing effort-trial folder.
The daemon resumed on its unchanged Luna/medium default. GPT-6 Sol, GPT-6.1 Sol,
and high-effort premium calls were not tested; no reason to spend on them was
established by this successful format check.

### P1: Configure Model/Effort and Patch-Scaled Timeouts

Status: implemented following the operator's 2026-10-26 request. Optional exact
Codex model/effort profiles configure call, minimum overall, and per-selected-
patch allowances. Local and daemon paths share resolution; a matching profile
replaces legacy review/harness caps. Eight patches at 600 seconds receive
4,800 seconds overall. Defaults and other providers remain unchanged. The
provider deadline also bounds stdin transfer. Full gates passed 1,116 library,
48 main, seven CLI, and 25 integration tests, formatting, and all Clippy profiles.
See the configuration reference and DESIGN_MODEL_EFFORT_TIMEOUTS.md. Longer
limits do not select higher effort or authorize additional model spending.
Installed runtime validation completed at 13:42 CET: all six Luna/Terra
xhigh/MAX profiles passed explicit-config no-AI preparation at an idle queue
boundary, and the daemon restarted. No hour-long model trial was run.

### P0: Track Provider JSON Failures Without Inventing Results

Update at 13:22 CET: the operator authorized raising Luna effort progressively
and checking available GPT-5.6 Luna/Terra alternatives if needed. Authenticated
model/list confirms gpt-6-luna, gpt-5.6-luna, and gpt-5.6-terra with medium,
high, xhigh, and max effort. No model names were guessed and no Sol substitution
was made. The original 1,800-second review and 600-second provider limits were
subsequently replaced for explicitly matching xhigh/MAX timeout profiles. Both
call and overall deadlines must allow at least 3,600 seconds before a MAX
experiment; configuring them does not itself start or authorize that experiment.

One exact failed-request replay on gpt-6-luna/medium returned valid envelope and
inner stage JSON with concerns/dismissed_concerns arrays in 8.829 seconds. Usage:
10,634 input, zero cached input, and 562 output tokens (179 reasoning tokens
reported within output). Artifacts and the 18,628-character original prompt are
retained outside the repository under /tmp/sashiko-effort-trial-20261026. This
is a one-request format check, not proof of causation, semantic correctness, or
elimination of future failures. No higher effort or alternate model was called.

After a completed-patch boundary, runtime Luna effort was changed from low to
medium. Explicit-config no-AI preparation passed, the daemon resumed, and its
actual invocation confirmed model_reasoning_effort=medium. Cache identity already
includes effort. Database and configuration were backed up before one supported
patch-only retry of the failed alternatives patch; API accepted it. The original
failure and successful patches remain preserved. The patch-only retry completed
as Reviewed at 13:30:01 CET. A read-only check at 13:41 CET confirmed the new
Reviewed result and retained original Failed record. This closes that targeted
recovery, not the separate GIC malformed-output failure or future reliability.
Latest fresh quota before the test was 10% five-hour and 86% weekly remaining,
reset at 14:00:02 CET. Keep the existing remaining-budget authorization; do not
increase effort again unless medium actually fails, and do not change active calls.
Fresh 13:41:52 CET quota: 8% five-hour, 86% weekly, reset 14:00:02 CET. The
loaded queue was exhausted with no pending or active series; no refill was made.

Status: the two identified malformed-output examples have recovered; future
format reliability remains observational, not guaranteed. A 13:43 CET read-only
check found the GICv3 v4 series fully Reviewed, including its previously failed
first patch, with the earlier Failed review retained. Native artifacts also
contain a malformed 3,048-character verification response; its JSON was already
invalid before Sashiko parsed it. No additional retry or model call was needed.

Original provider-output failure observed on 2026-10-26 at 12:07 UTC:
The final patch of x86/alternatives: Adjust all insn-relative fields exhausted
three schema-validation attempts. Native Codex assistant artifacts at 12:07:00,
12:07:08, and 12:07:13 contain malformed JSON themselves (1,519, 1,453, and
1,453 characters), with missing container closures. This is not evidence that
Sashiko dropped a complete valid response. The prior GICv3 EOF failure has now
also been compared against native output and its later recovery confirmed.

Preserve the failed review, native artifacts, and completed patch results. Do
not manufacture a successful result by guessing missing JSON or repeatedly
requeue the series. Normal review throughput continues on other work. Investigate
schema-specific corrective feedback or a supported structured-output backend
only with a focused reproduction and bounded validation; no parser weakening or
extra retry budget is authorized by this observation. Check the failed stage's
full request and native response before attributing the error to a prompt bug.

### P1: Classify Xen Staging Merges and Filter Review State

Status: view-only recorded review-state filter deployed as 44094e87 at 18:58 CET
on 2026-10-26. See
[the incremental filter design](../designs/DESIGN_PATCHSET_REVIEW_STATE_FILTER.md).
The dropdown sits directly right of All Lists, before search, with typed
server-side predicates shared by list/count queries before pagination. It
combines with search and list selection, isolates default caches, restores URL
state, resets pages, clears the state without clearing the list, and is omitted
from Messages/Bugs. Not Reviewed Yet means current series state is not Reviewed,
not absence of earlier partial or human reviews. Existing embargo redaction and
scheduling are unchanged. Focused database/API/permissions/URL checks and
desktop/mobile workflows pass. The full gate passed 1,131 library tests (two
ignored), 49 main, seven CLI, 26 integration, formatting, and all Clippy profiles.
The fixture is stopped and the daemon resumed. No model call was needed.
The release build passed with no new OOM events. An idle checked database backup
and rollback binary are retained. Live API totals match database counts and the
live desktop/mobile dropdown correctly shows 98 Reviewed across two pages with
no page/control overflow. Schema 15 and muted delivery remain unchanged.

Staging membership and attributable human-review classification remain open;
neither option is fabricated from existing status. A targeted code
check did not establish working Xen staging-merge classification or the requested
human-review evidence. Existing stored Git patch IDs and baseline resolution
are possible foundations, not evidence this feature is already implemented.
The [staging evidence proposal](../designs/DESIGN_XEN_STAGING_EVIDENCE.md) now
defines an explicit authoritative ref, bounded background patch-ID index, separate
persisted evidence, unknown/stale and revert semantics, and unchanged scheduling.
Read-only inspection confirms origin is the xen-review fork and upstream is the
Xenbits repository, with different staging tips. Recommend explicit
refs/remotes/upstream/staging. Approval is required for that new index/evidence
architecture and target choice was explicitly granted by the operator after
the interrupted question was recovered on 2026-10-26. Typed opt-in settings and
the bounded pinned-tip matcher are implemented in source. Four temporary-Git
tests cover rewritten identities, ref rewrites, history/byte limits, cache reuse,
duplicates, merges, binary/empty diffs, and reverts. Full make check-pr passed
1,137 library tests (two ignored), 49 main, seven CLI, 26 integration, formatting,
and all Clippy profiles. Schema-16 atomic evidence storage is now implemented
in source, with a subsequent full gate passing 1,138 library tests (two ignored)
and the same main/CLI/integration counts. It bounds mailed-patch publication,
retains immutable cache identities and historical snapshots, invalidates stale,
failed, or changed-source evidence, and never changes review state. Optional
integration now uses the existing GitSyncWorker only, requiring a real successful
covering fetch and invalidating failed/skipped refreshes or moved tips. Batched
saved-evidence retrieval and a per-patch provenance disclosure are implemented
for both detail routes. The current full gate passes 1,139 library tests (two
ignored), 49 main, seven CLI and 27 integration tests plus formatting and all
Clippy profiles. Typed series aggregation and separate staging_state filters are
also implemented in source: one fresh pinned snapshot, full saved parts, distinct
expected part numbers, unchanged review states, and one row/count predicate
before pagination. The latest full gate passes 1,140 library tests (two ignored),
49 main, seven CLI and 27 integration tests, formatting and all Clippy profiles.
Synthetic desktop/mobile interaction and DOM bounds checks pass at 1,280/390
pixels, with screenshots captured and no independent visual-inspection claim.
Schema 16 and source bd966c85 are deployed at 00:20 CET on 2026-10-27 after an
idle checked backup, matching release/installed hashes, and API health checks.
The approved upstream/staging target is enabled with bounded initial limits.
Its first normal fetch was skipped as fresh, so zero snapshots are published
and current evidence correctly remains Unknown. Observe a later normal successful
covering refresh before claiming live matching or filter validation. Exact
ancestry is implemented in source as the next bounded increment: saved applied
coverage must name the same reachable commit and current patch identity. Exact
commit evidence is separate from diff-equivalent matches and cannot survive
stale, changed-source, or rewritten-snapshot checks. No new schema or series
filter semantics are required. The focused storage/API/escaping checks and
desktop/mobile native disclosure checks pass. The full gate passes 1,141 library
tests (two ignored), 49 main, seven CLI, 27 integration, formatting, and all
Clippy profiles. The 8097 fixture is stopped. Ancestry source 113b83a5 is deployed
at 00:38 CET on 2026-10-27 with a checked idle database backup, matching hashes,
healthy API, unchanged schema 16, and all 99 successful series preserved. No
positive live ancestry is claimed before the first snapshot. Attributable
human-review evidence remains pending. No live staging
history scan or feature-specific model call has run. Ordinary
scheduling and muted delivery are unchanged; the normal bzimage review completed
without new issues or email, bringing Reviewed series to 99.

Goal: detect and display when an individual patch is already incorporated into
the configured upstream reference. For Xen, prefer the upstream `staging` branch
as the classification reference; `master` is the main branch and may lag staging.
Resolve the actual upstream remote/ref explicitly rather than assuming local
HEAD or an arbitrary remote's staging branch is authoritative. Record the checked
ref, resolved tip SHA, check time, matched commit, and matching evidence. Missing
or stale refs mean unknown, not unmerged; staging membership does not prove master
membership or permanent acceptance if staging is rewritten.

Investigate exact commit ancestry and stable Git patch-ID matching for mailed
patches whose upstream commit SHA differs. Reuse existing patch-ID facilities
where suitable, but do not infer a merge from subject similarity, an apply
failure, or an empty diff alone. Define limitations for modified/squashed patches,
reverts, and rewritten staging history. Test refreshed tips and changed evidence.

Keep classification per patch and aggregate series as fully merged, partially
merged, unmerged, or unknown. Preserve prior review results and human comments.
As a later explicit scheduling policy, consider deferring already-merged patches
until eligible non-merged patches are reviewed; do not silently mark merged work
as reviewed, cancel an active review, or skip the unmerged part of a mixed series.
Expose the policy and classification so operators can understand queue order.

Add a view-only status dropdown beside the existing mailing-list selector,
directly to its right and before the search box. Keep the All Lists selector,
including Xen's xen-devel list. Proposed options: All States, Not Reviewed Yet,
In Review, Reviewed by Sashiko, Reviewed by a Human, Merged to Staging, and relevant
existing failure/skipped/cancelled states. Combine the filter with list selection,
search, pagination, and clear/reset behavior; preserve usable responsive layout.

Review state, human review evidence, and merge state overlap rather than forming
one mutually exclusive status enum. Define Not Reviewed Yet precisely and show
badges/details without losing these independent dimensions. Human-reviewed must
have attributable evidence, such as parsed review replies or Reviewed-by trailers
with provenance; a generic mailing-list reply is not proof of review. Unknown
human-review evidence must not be presented as confirmed absence of human review.
The [human-review evidence proposal](../designs/DESIGN_HUMAN_REVIEW_EVIDENCE.md)
grounds an opt-in, separately versioned provenance store in existing parsed mail
and exact reply links. Recommended first positive filter: direct replies with
unquoted Reviewed-by matching the parsed From mailbox, not quoted or author-carried
claims. It distinguishes recorded attribution from authenticated identity and
keeps partial/unknown/withdrawal limitations explicit. Store/policy approval and
bounded historical-backfill approval are pending; no human classifier is enabled.

Done when: deterministic fixtures cover exact/upstream-rewritten patch identities,
partial series, stale/missing refs, staging versus master, and human-review evidence;
API and UI filters agree on counts and pagination; scheduling is separately tested
and explicitly enabled without losing successful results or diagnostic artifacts.

### P0: Resolve Verification Failures That Lose Completed Analysis

Status: resolved. Prompt/retry feedback fix deployed in `3c3db2d4`; supported
retry completed as Reviewed on 2026-10-26 at 02:53:27 UTC using Luna. A read-only
database check at 12:54 CET confirmed both the later Reviewed record and the
original Failed record remain stored. This is evidence of pipeline completion,
not an independent correctness claim or a guarantee of future model compliance.

Evidence: a Luna review of the x86 microcode timeout-removal patch exhausted
three validation attempts on 2026-10-26 at 02:23 UTC with
`dismissed_concerns[0].source_ids must name input IDs`. Analysis had completed,
but the patch did not receive a completed verified result.

Diagnosis: the request supplied D1, D2, and D3, but the model copied `id: "D1"`
instead of returning `source_ids: ["D1"]`. The validator correctly rejected it.
The prompt now gives this exact distinction, and retry errors list valid IDs.
A focused regression checks the rejected shape, accepted lineage, and rejection
of unknown IDs. Evidence and provenance checks remain strict.

Outcome verified without rerunning or editing the queue. Do not claim prompt
changes guarantee model compliance. Checks passed with 1,088 library,
47 main, seven CLI, and 24 integration tests, plus all formatting/Clippy profiles.

Done when: an exact offline reproduction has a focused regression, the cause is
fixed or explicitly classified as a provider failure, and supported retries
preserve successful results and diagnostic artifacts.

### P1: Disable Unnecessary Native CLI Tools and Ambient Instructions

Status: restricted invocation implemented; fake-CLI tests and full gates passed.
No paid Codex experiment was run and the daemon remains locked to Luna.

Evidence: three sequential Sol CLI sessions at 02:55:48, 02:55:58, and 02:56:10
CET loaded repository instructions and native tool definitions. Native `exec`
calls searched `ALL_TOOLS` for Sashiko's prompt-defined Git tools, causing extra
model generations. These were not parallel patchset reviews. The current
`review.concurrency = 1` limits model calls to one in flight.

Implemented: isolate the Codex completion backend using installed CLI controls
and a fresh working directory, with a short replacement instruction instead of
the coding-agent prompt. Fake-subprocess tests check arguments, prompt delivery,
cleanup, model/effort, usage, and cache-profile separation. Managed substitution
of the restricted profile fails rather than silently accepting ambient tools.

The implementation replaces the ambient coding-agent instructions, disables
native tool features, and ignores user configuration. Existing authentication
is retained, but custom routing from user config is intentionally not loaded;
this compatibility change is documented. The active daemon remains on Luna.

Done when: offline subprocess tests verify effective arguments, working
directory, cleanup, cache-profile separation, and failure handling. Do not claim
that read-only sandboxing disables native tools. Any later paid confirmation
requires explicit model authorization and recorded generation counts.

### P1: Reduce Repeated Large Prompts and Improve Cache Reuse

Status: ambient Codex instruction overhead removed by the restricted profile;
repeated Sashiko stage context still needs measurement and a focused reduction.

Saved-request measurement: each of the three prompts carried 52,124 characters
of shared review context, 4,684 of tool definitions, 515-595 of stage instruction,
and approximately 45,500 of ambient CLI instructions. Do not translate character
counts into tokens or promise a quota percentage saving. Keep needed source
context; the optional whole-series mode below addresses repeated review stages.

Evidence: the same brief Sol interval recorded 171,604 input tokens, including
118,528 cached and 53,076 uncached tokens, with only 785 output tokens. The three
CLI sessions accounted for five recorded model generations. Fresh generations
were approximately 34,000 input tokens; extra within-session generations were
over 99% cached. Cross-session caching was partial, not absent.

Next: separate required patch/source context from CLI instruction/tool overhead,
repeated stage context, and growing tool history. Measure serialized prompt
sections and actual cached/uncached usage. Prefer stable shared prefixes and
removing demonstrably redundant material without dropping review evidence.

Done when: representative offline fixtures demonstrate reduced prompt volume
without changing required evidence, request ordering, provenance, or cache
identity. Report actual generation counts and cache hits rather than converting
shared account percentage changes into an invented token price.

### P2: Compare Series Context Reuse Strategies

Status: route evaluated on 2026-10-26. Finish the approved whole-series harness
first, then design sequential per-patch checkpoints on the shared engine. See
[the dedicated route ledger](context-reuse-route-ledger.md) for ordered steps,
budget caveats, cache measurements, and resume notes. Do not start a competing
implementation or change the runtime default before evaluation.

Current behavior: local per-patch review constructs a new Worker for each patch,
with an initially empty global history. Each workflow session starts fresh.
Series/source context is supplied, but earlier patches' review conversations,
tool results, and conclusions are not carried forward as shared model history.
Provider-side cached input prefixes may still be reused; that is distinct from
persisting a conversation or reusing Sashiko's completed-response cache.

Compare these explicit, ideally configurable alternatives:

- Whole-series harness review in one session, the current implementation path:
  share source investigation and reasoning across the series, then validate and
  split per-patch reply artifacts while retaining series-level usage attribution.
- Sequential per-patch reviews with earlier review context available: carry
  forward relevant conversation/tool evidence or a carefully attributed summary,
  preserving exact patch revisions and per-patch output boundaries.
- Existing independent per-patch workflow, retained for compatibility and
  workloads where isolation or parallelism is preferable.

Optimize for measured review quality and total cost, not just fewer subprocesses.
Keep stable instructions and shared series context at cache-friendly prefixes;
append patch-specific content and tool results without needlessly changing that
prefix. Do not promise provider cache hits or assume retained conversation means
zero additional input cost. Compare actual cached/uncached input, output tokens,
backend generations, tool calls, latency, and correctness on representative series.

Test that earlier review conclusions remain attributed to their original patch,
are rechecked when later changes invalidate them, and cannot inject operator
instructions into subsequent reviews. Bound growing history, preserve diagnostic
artifacts across cancellation/retry, and prevent duplicated per-patch token
accounting. Choose a default only after evidence; keep this as a future alternative
alongside, not a prerequisite for, the approved whole-series mode.

### P2: Add an Optional Skill/Agent-Driven Single-Prompt Review Mode

Status: design proposed in
[DESIGN_SINGLE_PROMPT_REVIEW.md](../designs/DESIGN_SINGLE_PROMPT_REVIEW.md).
Operator approved one bounded harness invocation with read-only source tools on
2026-10-26. Implement a separate `agent_harness` mode, not a tool-free
single-generation mode. Internal model turns must be observable and bounded.
Two bounded local subscription-backed Luna trials now have retained artifacts
and measured usage in the context-reuse route ledger. Runtime mode remains
workflow. The earlier operating window
was not extended by the architecture decision alone; the operator subsequently
authorized continued implementation and subscription-backed daemon operation.

Implemented foundations:

- `76439f00`: typed mode and positive generation/tool/token/time settings;
  existing configurations still select workflow. Local harness dispatch is now
  implemented, and daemon mode now has atomic persistence, cancellation, explicit
  reruns, and unverified artifact presentation. Runtime settings remain workflow.
- `ad34e99f`: ordered, settings-relative instruction snapshots with original
  bytes, validated frontmatter, prompt bodies, total byte limits, and SHA-256
  identity covering contents and order. No metadata enables tools.
- Latest full gate passed 1,112 library, 48 main, seven CLI, and 25 integration
  tests, plus formatting and all Clippy profiles. Two library tests are ignored.
- Local CLI regression: a two-patch range uses one fake Codex call, no-AI uses
  zero calls, and malformed final artifacts fail with raw history and usage
  retained. Full JSON includes exact baseline, selected indexes, model/effort,
  and immutable instruction contents. Human and agent output explicitly avoid
  claiming independent verification. No paid trial or runtime mode change.
- Daemon fake-provider regression covers one selected scope, successful and
  malformed output, cancellation, preserved artifacts across supported reruns,
  shared limiter release, one interaction per invocation, and empty outboxes.
  Deferred delivery explicitly excludes harness results. Desktop/mobile browser
  checks confirm escaped drafts, unverified labeling, raw-log access, and no
  horizontal overflow or status-header overlap after narrow-width wrapping.

Execution budget decision approved: for subscription-backed Codex, enforce
token thresholds between model turns, permitting one-generation overshoot.
Keep hard time and tool-call limits, report actual usage and overshoot, and
never start another generation after reaching a threshold. Missing usage is an
error, not zero. Do not claim a hard per-generation output-token ceiling or
extend this approval to metered billing. The installed normal-turn schema does
not expose such a verified ceiling.

Goal: allow configuration to bypass Sashiko's entire multiphase review procedure
and replace it with one explicitly selected agent/skill prompt for a whole Xen
series. Preserve daemon ingestion, queue management, cancellation, timeouts,
budgets, persistence, access controls, and muted delivery. Keep the current
workflow as the default; do not silently change existing installations.

Reference inputs supplied by the user:

- `.github/agents/friendly-series-review-with-suggestions.agent.md`
- `.claude/skills/friendly-series-review-with-suggestions/SKILL.md`
- Their base friendly-review agent/skill dependencies and the Xen coding-style
  skill, including applicable Xen CODING_STYLE files and reviewer conventions.

Design decisions and implementation gates:

- Define a typed mode selector and explicit agent/skill paths, resolve their
  dependencies predictably, and reject missing files rather than falling back.
  Do not modify the user's original agents or skills to fit the daemon.
- Supply the complete series, baseline, and sufficient relevant source context
  without hidden planning, per-patch analysis, verification, or report calls.
- Distinguish one Sashiko provider invocation from one backend model generation.
  A harness can make native tool calls internally; expose and bound that cost.
  Decide explicitly whether source inspection uses supplied context or a
  restricted harness. Do not enable arbitrary native tools as a shortcut.
- Preserve friendly plain-text replies, independently justified functional-fix
  suggestions, and separate reviewer notes. Define how these artifacts map to
  existing findings/status APIs without inventing structured findings or
  claiming that unverified output passed the old verification pipeline.
- Keep sending mail, posting publicly, editing source, and applying suggestions
  disabled unless separately authorized. Treat patch content as untrusted.
- Include resolved instruction contents, model, effort, mode, and output
  contract in cache identity and review attribution.
- Test with a fake harness that a series uses exactly one harness invocation,
  with no hidden Sashiko workflow stages or automatic repair calls. Test
  enforceable internal-generation/tool budgets, unsupported capabilities,
  default-mode compatibility, errors, cancellation, and persistence.
- Implement and deploy incrementally after the design is reviewed. Any paid
  trial must remain on the authorized Luna configuration with bounded usage.

### P2: Plan Codex Account Quota and Pause/Resume Hysteresis

Status: future task requested on 2026-10-26; not implemented or enabled.

Goal: use fresh authenticated Codex five-hour remaining quota and reset time to
plan review allocations over the remaining window, retaining a debugging reserve.
Start with Codex subscription-backed accounts only, not generic assumptions
about every provider's billing or quota model. Account percentages are not exact
token counts; estimates need observed history and confidence bounds.

Make thresholds configurable without recompilation, initially in a simple
Codex-specific settings section. Defaults requested by the operator: pause new
automatic reviews below 20% remaining; resume only above 30%. Respect strict
inequalities. Wait until the reported reset time, then re-read quota before
resuming; never assume a reset happened or sufficient quota returned. If delayed,
still require the upper threshold. Use a scheduled wakeup with bounded refreshes,
not a busy poll or unconditional restart. Preserve active reviews and completed
results, and expose paused state, reason, and next check time.

Later scope: key state and policy by provider account, since several processes
can share an allowance. Plan fair estimated per-review budgets from remaining
capacity and time to reset without spending quota simply to exhaust it. Unknown
or stale telemetry must not authorize more expensive models, credits, metered
billing, or automatic effort adaptation.

Done when: fake-clock and fake-quota tests cover threshold edges, hysteresis,
reset changes, late wakeups, telemetry errors, other account consumers, restart
recovery, cancellation, and unchanged behavior when disabled. Review the design
before implementation; this ledger entry is not an installed quota controller.

### P2: Expose Live and Final Review Usage

Status: future task requested on 2026-10-26; not implemented.

Show model/provider/actual effort, completed and active turns, tool calls,
input/output/cached tokens, elapsed time, and budget consumption in the web UI's
status/progress surface. Preserve final totals on completed, failed, timed-out,
and cancelled reviews. Differentiate Sashiko turns, harness invocations, and
backend generations; do not imply those counts are interchangeable.

Use provider-reported counters, state whether usage is cumulative or incremental,
and label unavailable or in-flight usage honestly. A currently running generation
may not report tokens until completion. Keep access controls, avoid double
counting cached tokens or retried events, and do not duplicate whole-series
usage across per-patch rows. Keep daemon progress independent of UI polling.

Done when: API and UI tests cover live updates, terminal persistence, absent
telemetry, reconnects, cancelled reviews, and whole-series attribution.

### P3: Durably Defer Reviews on Subscription Quota Exhaustion

Status: lower-priority follow-up requested on 2026-10-26, after the daytime
reset. Not implemented; no overnight watcher or live exhaustion trial is running.
Extend the P2 account-quota/hysteresis design rather than adding another quota
controller. Keep account subscription exhaustion distinct from configured
per-review token/tool/call limits; a reset must not bypass those safety limits.

Current evidence: QuotaManager provides in-memory shared backoff capped at five
minutes. BackoffProvider retries typed rate-limit errors, with an attempt ceiling
on paths without a deadline budget. Untyped Codex CLI errors currently fall back
to Fatal classification. This is not durable reset-aware queue deferral, and
subscription exhaustion is not guaranteed to avoid a Failed review today.

Goal: classify authenticated subscription exhaustion explicitly, retain completed
stages/checkpoints and diagnostic artifacts, and park affected unfinished review
work until the account's reported reset instead of repeatedly calling the model
or declaring the patch failed. Persist the account, pause reason, reset/next-check
time, and resumable progress. Let unrelated eligible accounts continue. Keep
cancellation, access controls, muted delivery, and explicit interruption semantics;
do not silently resume a terminal checkpoint invocation or rerun completed work.
After restart or scheduled wakeup, refresh authenticated quota and apply the
configured resume threshold before continuing. Unknown/stale telemetry is not
permission to spend. Never buy credits, enable metered billing, or substitute a
premium model merely to get past exhaustion.

Test first with fake clocks, quota responses, and providers: short versus long
reset windows, 429 versus subscription exhaustion versus fatal auth failures,
late/shifted resets, absent telemetry, another consumer spending the allowance,
process restart, cancellation while paused, retained output, and no duplicate
usage or completed work. The paused API/UI state must remain distinguishable
from Failed, with an honest next-check time. Avoid busy polling and retry storms.

Only after deterministic checks and design approval, prefer a small bounded live
trial near an authenticated reset between 00:00 and 06:00 in the configured local
timezone, checking timezone/DST explicitly. Do not drain quota just to force the
test, start a multi-hour daytime wait, or install an unrequested overnight job.
Record real pause/resume events, provider calls, quota refreshes, preserved
progress, and limits. Fresh budget/window authorization is still required.

### P3: Keep Remaining Operational Problems Evidence-Based

Status: intermittent NNTP TLS/read timeouts continue while queued reviews make
progress. Historical patches also have ordinary baseline/applicability failures.

Next: distinguish transport failures, wrong-project patches, stale baselines,
and actual daemon defects. Do not mass-requeue, repeatedly restart a healthy
daemon, or introduce a new ingestion architecture without evidence and approval.

## Completed Repairs

- `924b1a4c`: restore MIME decoding for selectively parsed email headers;
  reparse the three affected archived patch diffs without changing identities.
- `6835e48d`: retain Linux worker spawning after replacement of the running
  binary, with an isolated executable-removal regression.
- `58b02fee`: retain complete JSON tool calls before trailing invalid material;
  do not execute incomplete objects. Replay of the exact saved failure passed.