# Sashiko Improvement Ledger

Updated: 2026-10-27. Work in priority order, updating evidence and status as
changes are validated. Do not create work merely to keep the daemon busy.

## Operating Constraints

- Prefer verified `gpt-6.1-terra` for Xen reviews, with high effort for review
  tests and low or medium when appropriate. Bounded premium Codex subscription
  trials require the agent's fresh strictly-above-50% gate before every call,
  sufficient weekly and time reserve, and no paid overages. Other Terra/Sol
  trials start at low or medium, never above high. A reset alone does not
  authorize broad Sol use or metered billing.
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
The daemon resumed on its unchanged Luna/medium default. At that point GPT-6
Sol, GPT-6.1 Sol, and high-effort premium calls had not been tested; no reason
to spend on them was established by this successful format check.

A later targeted gpt-6.1-terra / HIGH replay returned valid stage JSON in one
6.761-second call using 10,639 uncached input and 322 output tokens. Native
metadata confirmed the requested model and effort. This was one saved-stage
format replay, not a complete sequential review or a quality comparison.

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

An offline fake-CLI regression found that an explicit turn.failed event after a
valid message could be returned as success when the process exited zero. The
provider now treats that event as failure for either exit status, retains a
redacted nested diagnostic or an explicit missing-detail message, and separates
old response-cache entries from the corrected parser. The installed CLI contains
the event name, but no live failure was induced or replayed. This does not
classify subscription exhaustion or enable retries. Focused provider tests and
the full `make check-pr` gate passed.

An offline fake CLI also reproduced a prompt-transfer stall when startup output
filled a pipe before the CLI read a large prompt. Codex now uses the existing
concurrent stdin/output collector, preserving nonzero-exit diagnostics and
rejecting incomplete prompt delivery on a zero exit. Fake-process checks cover
stdout and stderr backpressure, timeout, redaction, and cancellation cleanup;
the full `make check-pr` gate passes. No equivalent live stall was established.

A separate fake Copilot CLI showed that its 600-second deadline began only
after prompt delivery, allowing a blocked stdin write to wait indefinitely.
The shared collector now bounds delivery and output collection together.
Fake-process checks cover both output streams, incomplete delivery, redacted
early failure and timeout child cleanup; the full `make check-pr` gate passes.
No authenticated Copilot call or live stall was used for this fix.

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
positive live ancestry is claimed before the first snapshot. At that checkpoint,
attributable human-review evidence was still pending; the rollout below supersedes
that status. No live staging
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
keeps partial/unknown/withdrawal limitations explicit. The operator approved the
integrated opt-in store and direct-reply policy on 2026-10-27. The first parser
increment passes fake-mail checks for exact single reply targets, single matching
mailboxes, MIME decoding, spoofed display names, duplicate headers, and excluded
quoted, forwarded, fenced, signature, and patch-submission content. Input and
decoded body sizes are bounded. The next storage increment adds schema 17 and
disabled-by-default normal-ingestion recording. Source fingerprints, mail/patch
revision guards, exact targets, and idempotent inserts retain provenance without
rebinding old replies to changed targets. Rejected raw-only source changes also
invalidate old positives. The focused store, daemon compile, historical schema
upgrade fixtures, and full make check-pr gate pass. API/UI enrichment now includes
the positive-only human_review=recorded filter, independent current-state
predicates before row/count pagination, distinct recorded-part counts, and
bounded native provenance on both detail routes. Real-router checks cover
partial series, disabled mode, cache freshness, invalid/duplicate values, and
stale exclusion. Synthetic desktop/mobile disclosures escape hostile strings,
fit 1280/390 pixels, restore/Clear URLs, and isolate Messages requests. The full
API/UI make check-pr gate passes: 1,145 library tests (two ignored), 49 main,
seven CLI, and 28 integration tests, plus formatting and all Clippy profiles.
API/UI source 47cdd5c1 is deployed at 01:52 CET on 2026-10-27 with a checked
idle database backup and matching binary hashes. Schema 17 and both detail
routes are verified. The opt-in is enabled for normal new-mail ingestion; no
historical backfill was run. The live positive filter correctly returns zero
recorded evidence. All 103 Reviewed series remain preserved, with no Pending or
In Review. Provider/model/configured medium effort, timeout, muted delivery,
SMTP-off, and web-admin authentication remain unchanged. Current quota at
01:53 CET is 43% five-hour and 54% weekly remaining, reset 05:03 CET; no premium
call, self-review, or feature-specific model call was started.
No live classifier or automatic historical backfill is enabled. Historical
backfill remains bounded and is not run automatically.

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

An offline formatter regression also found that model-supplied tool IDs and
names could break quoted prompt attributes. The shared CLI formatter now encodes
attribute delimiters and line breaks while retaining ordinary IDs unchanged.
All six CLI adapters use a new prompt-format cache identity, so their earlier
responses are not replayed under the corrected framing. Focused formatter and
cache-profile checks and the full `make check-pr` gate pass. This is not a claim
that prompt injection is eliminated, and no live model call was made.

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

The sequential runner now reuses its initial validation of the immutable series
instead of serializing the complete series again before every checkpoint. The
public checkpoint builder still validates independently, and focused tests
confirm identical checkpoint requests, retained initial validation, stable
prefixes, cancellation, and budget behavior; make check-pr passes. This reduces
local serialization work only; it does not reduce prompt bytes or establish token
savings.

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

The unwired policy also retains a later reset reported by a fresh high-allowance
sample observed before the existing pause barrier. A fake-clock regression
reproduced the previous early wakeup and now covers earlier and later resets,
late evaluation and the post-reset observation; make check-pr also passes. This
does not enable scheduling or authorize a provider call.

The optional Codex account-quota settings table now parses the same disabled
policy defaults and validates thresholds, durations, and unknown fields. The
loader rejects enabled = true while account binding, durable state and scheduler
integration remain unfinished. Loader tests cover omitted and custom settings,
invalid and non-finite values, and rejected activation; make check-pr passes.
There are no runtime behavior or provider authorization changes.

The next offline foundation stores a bounded workspace-scoped account state in
schema 20. Revision-checked writes preserve reset barriers and the last credible
sample, reject stale competing updates, and require a fresh matching workspace
snapshot when storing a new sample. File-backed tests cover database reopen,
concurrent connections, unavailable or corrupt telemetry, account isolation,
reset barriers, and revision overflow; make check-pr passes. This storage does
not evaluate quota thresholds or authorize model calls, and has no scheduler
or runtime caller.

An offline transition now combines the pure five-hour policy with the stored
state using one revision-checked write. Disabled evaluation bypasses storage;
unknown, stale, expired, future, or wrong-workspace observations pause and retain
the last credible sample. Fake-clock tests cover reset shifts and reopening a
database, and make check-pr passes. This transition has no production caller
and its FiveHourReady result is not authorization for a model call.

Status: requested on 2026-10-26; the staged
[Codex account quota design](../designs/DESIGN_CODEX_ACCOUNT_QUOTA.md) now selects
a disabled pure five-hour admission policy as the first increment. Its five
fake-clock tests now pass: strict threshold edges, reset barriers, freshness,
wrong accounts, shifted/late resets, unknown telemetry, disabled state retention,
invalid inputs and timestamp overflow. The full make check-pr gate passes 1,150
library tests (two ignored), 49 main, seven CLI and 28 integration tests, plus
formatting and all Clippy profiles. The existing transient QuotaManager remains
unchanged. No account adapter, persisted controller, automatic pause/resume or
watcher is implemented or enabled; the pure policy has no runtime caller.

The next Codex metadata decoder also passes fake-response and full deterministic
gates (1,151 library, two ignored; 49 main; seven CLI; 28 integration). It keeps
measured weekly zero distinct from unknown, requires known windows and matching
subscription metadata, and rejects malformed, expired or paid/unknown spend
controls. A metadata-only schema read at 02:10 CET reported 35% five-hour and
53% weekly remaining. That initial inspection missed workspace fields outside
the account object; the login/plan key alone cannot establish workspace identity.
The deployed daemon remains unchanged; no controller or model call was started.

The bounded metadata-only RPC increment now passes four fake-process tests.
It reads account identity before and after the quota windows, uses an isolated
working directory and a fixed 20-second deadline, and bounds stdout bytes,
line size and notification count. Only metadata methods are emitted. Unexpected
requests/IDs, malformed responses, RPC errors, changed accounts, timeout and
early exit fail closed without exposing payloads. Child reaping and cancellation
cleanup are tested. It remains unwired from admission; weekly reserves,
persistence, wakeup scheduling and paused-state UI are not claimed complete.
The full gate passes 1,157 library tests (two ignored), 49 main, seven CLI and
28 integration tests, formatting and all Clippy profiles. No live model call
was needed and the reader is not deployed or wired into scheduling.

A later workspace-bound reader compares the account/read workspace ID with the
quota response account ID, checks included-usage permission and account stability,
and keeps raw identifiers out of logs and persistence. Its separate premium
predicate requires a fresh matching snapshot and strictly more than 50% five-hour
remaining. Positive weekly allowance alone is not a sufficient reserve, and
neither reader nor predicate is wired into model-call admission. The latest full
gate passes 1,168 library tests (two ignored), 49 main, seven CLI and 28
integration tests, formatting and all Clippy profiles.

Metadata-only thread probes exposed no execution account identity before a turn.
The saved Terra replay's creator ID matched the observed workspace only after
model work began. Safe pre-call binding, sufficient weekly and time reserves,
and preservation of partial work on denial remain required before automatic
premium reviews can resume. See the quota design for the probe and shared-login
restrictions; do not interpret a post-call identity match as a pre-call safeguard.

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

Status: first bounded active-stage increment implemented and validated on
2026-10-27; full live/final feature remains incomplete. See
[live usage design](../designs/DESIGN_LIVE_REVIEW_USAGE.md).

Completed workflow stages retain measured SessionUsageCoverage with token
totals through a separate worker/local accounting event and structured
subprocess record. The daemon registry keeps latest patch/stage reports per
worker run, replacing duplicates and ignoring stale runs; 256 entries maximum
with explicit truncation. Both detail routes enrich only matching active
patchsets after existing permission/redaction checks. The UI reuses coverage
rendering, keeps missing usage distinct from measured zero, and labels latest
reports rather than a workflow total. Preliminary-stage accounting cannot
advance the counted-stage progress bar. The native harness path emits no such
snapshot yet, and no whole-series scope is inferred.

Fake-provider, protocol round-trip, registry and API projection checks pass.
Synthetic browser tests cover updates, reload/reconnect, hostile stage escaping,
truncation, terminal removal and 390/1440-pixel containment. A 10-second positive
update wait timed out before matching rendered counters arrived; no fixed
polling-latency guarantee is claimed. Full make check-pr passes 1,160 library
tests (two ignored), 49 main, seven CLI, 28 integration, formatting and all
Clippy profiles. Package-scoped debug cleanup plus CARGO_INCREMENTAL=0 repaired
an initial disk-full link. Service and mirror timer were restored after checks.

Still required: complete final accounting across failed, timed-out and cancelled
reviews, including failed-stage/in-flight usage; active/completed turn and tool
counters; elapsed/budget data; actual
backend effort and generation visibility; and native harness/whole-series
attribution. No model call or automatic quota controller was started.

Signed commit a535d045 is pushed and deployed at a verified idle boundary.
Private database/binary/config backups are retained; quick_check passes,
schema 17 and all review-state counts are unchanged. Both live detail routes
preserve human provenance and omit active snapshots on terminal records. The
synthetic fixture is stopped. Existing authentication, muted delivery and
codex-cli / gpt-6-luna / configured medium settings are unchanged; effective
backend effort remains Unknown. No telemetry-specific live review was queued.

The next retained-diagnostic increment adds migration 018: nullable compressed
stage snapshots on the exact review row. A worker-owned accumulator prevents
parallel patch workers from borrowing each other's display-registry snapshot;
duplicate reports replace prior values. Writes are capped at 256 reports and
256 KiB. Stderr drains under a one-second post-reap deadline; warnings preserve
honest partial state if persistence or drain fails. Terminal completion does
not clear the snapshot, and no ai_interaction or final total is fabricated.
Both existing detail projections include bounded retained reports through the
same query and embargo boundary. Cancelled cards show them too; cancelled
summaries no longer fall through to In progress.

The fake timeout remains an error, reaps promptly and retains the completed
stage's reported usage. Round-trip, legacy unknown, cancellation completion,
oversize rejection, both detail projections and embargo exclusion pass. Full
make check-pr passes 1,161 library tests (two ignored), 49 main, seven CLI,
28 integration, formatting and all Clippy profiles. Synthetic terminal-card
and escaping checks pass at 390/1440 pixels without overflow. A component
screenshot timed out and is not counted as a passed capture. No live review,
model call or cancellation was started; complete final/native usage remains
open. Deployment requires the usual fresh backup and idle boundary.

Signed commit 9f419024 is now pushed and deployed with schema 18 at an idle
boundary, with a checked private schema-17 backup and matching rollback binary
and config. The running binary hash, migration and both detail APIs agree;
review-state counts and configuration bytes are unchanged. Historical snapshots
remain null, and no telemetry-specific review or live cancellation was queued.

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

The next increment closes one measured failure-path gap: SessionFailure already
contains usage, but previously returned before emitting an accounting event.
StageFailed now forwards those reported counters/coverage and returns the
original error; failed stages never emit successful counted-stage completion.
The existing record adds a typed completed/failed/unknown outcome with a safe
Unknown default for historical or future values. Both live/retained tables show
the outcome without describing failed reports as completed-stage work. No schema
change or inferred usage is needed.

A fake truncated response retains measured tokens while still failing; a
provider error without counters stays incomplete, not a measured zero. Worker,
protocol/storage and renderer checks pass. Full make check-pr passes 1,163
library tests (two ignored), 49 main, seven CLI, 28 integration, formatting and
all Clippy profiles. Synthetic browser checks cover all three outcome labels,
hostile names and 390/1440-pixel containment. No live failure, model turn,
replay or cancellation was induced. Complete unreported/in-flight/native and
harness accounting remain open.

An additional active-progress fix follows the explicit concurrent patch loop:
workflow subprocesses now own entries scoped by patchset and patch index rather
than overwriting one patchset entry. Legitimate siblings retain active reports;
same-patch replacements still reject stale updates/cleanup. The latest updated
nonempty line remains the display string. Merged active reports stay bounded
at 256 with deterministic ordering and truncation, without summing duplicates.
Per-review persisted diagnostics and review scheduling remain unchanged.
Five focused registry tests and full make check-pr pass: 1,165 library tests
(two ignored), 49 main, seven CLI, 28 integration and all lint profiles. No
live parallel series or model call was queued to test this correction.

Native harness aggregation previously converted missing cached-token counters
to measured zero, even though the Codex parser distinguishes them. The
accumulator now retains an unknown aggregate if any recorded call omits its
cache count, and both whole-series and checkpoint persistence use nullable
storage. Checkpoint API totals are unknown if any recorded cache count is
missing; explicit zero remains measured zero. Existing rows are unchanged and
previously synthesized zeros cannot be distinguished retrospectively. Focused
accumulator, storage and renderer checks pass, as does the full make check-pr
gate: 1,169 library tests (two ignored), 49 main, seven CLI, 28 integration,
formatting and all Clippy profiles. Native live progress and complete accounting
for unreported in-flight calls remain separate open work.

The next native progress foundation adds an optional session callback with
series-scoped attempted-call and source-tool counts, in-flight state, and usage
from reported calls only. Fake-provider checks cover missing usage, missing
versus zero cache counts, tool attempts and cancellation. Whole-series and
sequential daemon paths now publish the latest snapshot into the existing
run-scoped registry; both protected detail APIs attach it only for an active
matching patchset. Fake-daemon and projection checks cover call progression,
stale-run rejection, cleanup, terminal states and embargo. The patchset header
now shows that series snapshot once and refreshes it during active review;
synthetic desktop/mobile checks cover missing versus zero cache values, polling,
terminal cleanup and layout. Resolved call/tool limits, input/output thresholds
and the review timeout are also displayed exactly. Token thresholds are checked
between calls and can be overshot by one response, not hard generation caps.
Approximate elapsed time advances from the harness's monotonic deadline clock
even while a provider call is pending; fake-clock and responsive fixture checks
cover updates and layout. The configured Codex per-call timeout is resolved by
the same helper as the provider and shown separately from the series deadline;
it is not a measurement of a completed call. The snapshot does not expose
backend-native generations or unreported in-flight tokens.

Active progress now respects the embargo timestamp and resolved access even
while the patchset status is In Review. For terminal harness reviews, schema 19
adds a nullable, bounded series snapshot written atomically with the existing
whole-series or sequential completion transaction. It retains only matching
run-owned counters, reported usage and configured limits; invalid or absent
diagnostics remain unknown and do not block completion. Legacy and interrupted
records are not reconstructed. Both protected review projections expose the
snapshot without loading the sequential trace, and the series card labels
unreported calls as incomplete. Fake-daemon, migration, rollback, timeout,
cancellation, cache-availability and embargo checks pass, as do synthetic
desktop/mobile renderer checks. This work has not been deployed or tested with
a live model; backend-native generations and unreported usage remain unknown.

Sequential checkpoints now retain a validated, run-owned progress snapshot in
the same transaction as their artifacts and cursor. A checkpoint marker prevents
an interrupted review from presenting those counters as terminal totals; a
missing later snapshot can leave an earlier one intact. Recovery projections now
require a persisted terminal trace before reporting total attempted calls or
complete usage, including when a cancelled session is recovered. Focused fake
daemon, rollback, recovery, cancellation, embargo and renderer checks pass, as
does the full make check-pr gate. The integrated browser was unavailable for
this increment, so its responsive layout was not rechecked. This change is not
yet deployed or live-tested.

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

The 2026-10-27 daytime check found a concrete repeating maintenance failure:
geometric repack combines --stdin-packs with --exclude-promisor-objects in Xen's
blob:none clone. A temporary promisor fixture reproduces the exact failure on
Git 2.39.5. Use supported incremental repacking when partial-clone configuration
is present; full repositories retain geometric packing. No persistent remote
configuration is changed, no object is pruned, and lazy fetching is disabled.
Partial clones pack reachable loose objects but may leave unreachable objects
loose rather than rolling every object up. Both focused regressions pass on
Git 2.39.5 and 2.56.0: reachable loose packing, retained unreferenced/promisor
objects and markers, unchanged remote configuration, and a real blob-filtered
clone whose missing blob remains unfetched with an unavailable remote. The full
make check-pr gate passes 1,153 library tests (two ignored), 49 main, seven CLI,
28 integration, formatting and all Clippy profiles. An initial disk-full link
was repaired by removing only regenerable incremental build cache, preserving
review artifacts and operational backups. Source 2fbfa794 is signed, pushed and
deployed at 15:15 CET after an idle checked database backup and matching binary
hashes. Live incremental maintenance packs 204 loose objects, retains all 23
promisor markers, and leaves refs/configuration unchanged. API is healthy,
schema 17 and all 110 Reviewed series remain intact; no active review was
interrupted. Six normal-ingestion human-evidence records now appear in the live
positive filter independently of AI state. First staging publication remains
unobserved. The user-owned mirror refresh timer is preserved.
This maintenance repair does not change
review scheduling or erase previous failures.

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