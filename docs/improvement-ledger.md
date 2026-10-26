# Sashiko Improvement Ledger

Updated: 2026-10-26. Work in priority order, updating evidence and status as
changes are validated. Do not create work merely to keep the daemon busy.

## Operating Constraints

- Keep development Xen reviews on `gpt-6-luna`, low effort by default. The
  authorized runtime now uses subscription-backed Codex CLI; do not select Sol
  merely because quota has reset or enable metered billing without permission.
- Do not run Sashiko self-review through paid Copilot CLI. Do not spend Sol
  quota to investigate or validate these items without fresh authorization.
- Preserve muted outbound delivery, completed results, review artifacts, and
  user edits. Back up the database before maintenance; use supported retry APIs.
- Keep healthy reviews running. Stop the daemon and workers before tests or
  deployment, resume promptly, and verify actual forward progress.
- Validate with deterministic tests and fake providers before paid experiments.
  Follow AGENTS.md for sign-offs, checks, focused commits, and authorized pushes.

## Prioritized Work

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

### P0: Track Provider JSON Failures Without Inventing Results

Update at 13:22 CET: the operator authorized raising Luna effort progressively
and checking available GPT-5.6 Luna/Terra alternatives if needed. Authenticated
model/list confirms gpt-6-luna, gpt-5.6-luna, and gpt-5.6-terra with medium,
high, xhigh, and max effort. No model names were guessed and no Sol substitution
was made. MAX is excluded under current 1,800-second review and 600-second
provider limits; both must allow at least 3,600 seconds before a MAX experiment.

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
failure and successful patches remain preserved. Full retry outcome is pending.
Latest fresh quota before the test was 10% five-hour and 86% weekly remaining,
reset at 14:00:02 CET. Keep the existing remaining-budget authorization; do not
increase effort again unless medium actually fails, and do not change active calls.

Status: unresolved provider-output failure observed on 2026-10-26 at 12:07 UTC.
The final patch of x86/alternatives: Adjust all insn-relative fields exhausted
three schema-validation attempts. Native Codex assistant artifacts at 12:07:00,
12:07:08, and 12:07:13 contain malformed JSON themselves (1,519, 1,453, and
1,453 characters), with missing container closures. This is not evidence that
Sashiko dropped a complete valid response. A prior GICv3 review also failed on
EOF JSON parsing; its native artifacts have not been compared yet.

Preserve the failed review, native artifacts, and completed patch results. Do
not manufacture a successful result by guessing missing JSON or repeatedly
requeue the series. Normal review throughput continues on other work. Investigate
schema-specific corrective feedback or a supported structured-output backend
only with a focused reproduction and bounded validation; no parser weakening or
extra retry budget is authorized by this observation. Check the failed stage's
full request and native response before attributing the error to a prompt bug.

### P1: Classify Xen Staging Merges and Filter Review State

Status: operator-requested follow-up on 2026-10-26; ledger only, not a change to
the current harness implementation or active review scheduling. A targeted code
check did not establish working Xen staging-merge classification or the requested
human-review/status filter. Existing stored Git patch IDs and baseline resolution
are possible foundations, not evidence this feature is already implemented.

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