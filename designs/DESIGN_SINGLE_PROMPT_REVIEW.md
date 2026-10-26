# Optional Whole-Series Single-Prompt Review

Status: architecture choice approved on 2026-10-26: one bounded harness
invocation with read-only source tools. Implementation and deterministic
validation are next; a paid trial requires separate authorization. No runtime
default changes are approved by this document. The daemon now uses the
separately authorized subscription-backed Codex gpt-6-luna at low effort.

## Goal

Keep Sashiko's daemon management, ingestion, queue, baseline application,
timeouts, cancellation, access controls, persistence, and delivery policy, but
allow the entire review procedure to be replaced with one whole-series prompt
using operator-selected agent and skill instructions. Do not wrap the selected
instructions in the existing analysis/verification/report workflow.

The motivating reference is the user's friendly-series-review-with-suggestions
agent and skill. Preserve functional-first Xen review, applicable CODING_STYLE,
question-led complete replies, source-grounded suggestions, separate human notes,
and the prohibition on source edits or automatic public posting.

## Evidence and Expected Savings

The short Sol interval on 2026-10-26 used three sequential CLI invocations and
five recorded model generations, not parallel patchset reviews. The saved
requests each contained approximately:

| Section | Characters |
| --- | ---: |
| Shared Sashiko system context | 52,124 |
| Stage-specific user instruction | 515-595 |
| Sashiko tool catalog | 4,684 |
| Ambient CLI developer instructions | 30,671-30,892 |
| Ambient CLI user/repository instructions | 14,894 |

Recorded totals were 171,604 input tokens, 118,528 cached input tokens, and 785
output tokens. These are observed usage records, not a conversion from quota
percentages. Character counts are not token counts. CLI isolation addresses the
ambient instructions and native-tool detours; replacing the multiphase workflow
addresses repeated Sashiko review context. Do not remove needed source evidence
merely to make a prompt smaller.

## Approved Architecture

The operator selected option 2 below: one bounded harness invocation with
read-only source tools. It matches the reference agent's source-inspection
workflow without Sashiko's separate review phases. Option 1 is retained only
as an explanation of the tradeoff, not the selected implementation.

Two meanings of single call must not be conflated:

1. **Exactly one completion request:** Sashiko resolves all instructions and
   supplies the series, baseline, and bounded source context. No tool loop,
   automatic schema repair, planning, verification, or report calls run. Native
   CLI tools remain disabled. The reviewer must disclose insufficient supplied
   context rather than claim it inspected arbitrary callers. This gives the
   strongest Sashiko-side cost bound but cannot reproduce an interactive
   harness's unrestricted source inspection.
2. **One bounded harness invocation:** invoke the chosen agent/skill once, with
   restricted read-only source tools and no Sashiko review stages. The harness
   can inspect callers at exact revisions, but it can make multiple internal
   model generations. Generation, token, wall-time, and tool-call limits must
   be observable and enforceable; one subprocess is not a one-call cost bound.

Implement option 2 as a separately named `agent_harness` mode. Do not silently
enable native tools in existing completion providers or advertise this mode as
exactly one model generation. Read-only source inspection is authorized by the
architecture choice; arbitrary commands, writes, public posting, and changes to
the current daemon model are not. Approving the architecture does not renew the
earlier operating window or authorize a paid trial.

## Configuration Proposal

Use a typed enum, not arbitrary strings in control flow. Existing configurations
deserialize to `workflow`, preserving all current behavior. The following is a
proposal, not a working configuration yet:

```toml
[review]
mode = "agent_harness"

[review.agent_harness]
instruction_files = [
  "/operator/agents/friendly-series-review-with-suggestions.agent.md",
  "/operator/skills/friendly-series-review-with-suggestions/SKILL.md",
  "/operator/skills/xen-coding-style/SKILL.md",
  "/review/xen/CODING_STYLE",
]
```

An explicit ordered instruction manifest makes dependencies auditable. Arbitrary
natural-language directions to read files are not a dependency resolver. Include
required reference examples explicitly too; do not pretend a completion backend
will load the user's home-directory paths. Do not modify the original agents or
skills. Resolve/canonicalize local files at startup, reject absent/unreadable
inputs, parse customization frontmatter as data, and include its body without
interpreting a frontmatter tools list as permission to enable tools.

Resolve relative paths against the settings file directory before passing a
snapshot to a worker. Do not depend on the worker's current directory or let
untrusted patch contents add instruction files. Snapshot instruction bytes and
hash them once per job so edits during a review cannot change its identity.

## Execution Boundaries

`Reviewer::review_patchset_task` currently prepares the complete series and then
creates per-patch jobs. Branch after baseline preparation and existing subject/
size/permission checks, before spawning per-patch review jobs. Harness mode
must schedule exactly one eligible series job, not one single prompt per patch.
Preserve explicit skipped-patch scope in the input; reject an empty eligible set.

The worker currently receives the complete input plus a target patch index, and
local review constructs a Worker per target patch. Add a whole-series dispatch
before that per-patch workflow loop. Both daemon and local review must reach the
same implementation. The whole-series runner invokes the selected harness once,
under the existing shared limiter and cancellation/timeout scope. Its only
source tools are explicitly allowed, read-only operations against the prepared
repository at named revisions. Reject unsupported backends or capabilities
rather than falling back to unrestricted shell access. Existing completion-only
providers retain their restrictions.

Require explicit positive generation, token, wall-time, and tool-call budgets
before enabling the mode. The operator approved between-turn token thresholds
for subscription-backed Codex on 2026-10-26: account for cached and uncached
input and all reported output, and do not start another generation once either
cumulative token threshold is reached. One generation can exceed a threshold;
report that overshoot explicitly rather than advertising a hard token ceiling.
Keep time, invocation, and tool-call bounds hard. Check backend support for
reporting usage; reject absent accounting rather than treating it as zero.
Record internal model generations
and usage, terminate the complete process group on cancellation or limit breach,
and retain partial output as diagnostics, not a successful review. Do not use a
session helper that silently retries invalid output or starts another harness.

The deterministic input builder supplies original subjects/message IDs/order,
cover context, exact parent/commit IDs, all selected patches, and bounded source
context labeled by revision and file. Reuse existing baseline/prefetch helpers
where appropriate. Check the final serialized request against configured input
limits before calling the provider. Fail rather than silently truncate a series
or drop instruction files. The harness can inspect additional callers through
the allowed source tools; its output must disclose unresolved context gaps.

## Results and Persistence

Preserve the raw response and usage once for the series. A successful call means
the requested review ran, not that the old independent verification phase passed.
Store explicit review-mode attribution; do not infer it from provider/model text
or pretend absent structured findings mean a verified clean review.

The friendly reference produces reviewer notes and per-patch complete reply
drafts. Define a small response envelope for those artifacts, keyed by original
patch index/message ID, without requiring the old concern/lineage schema. Reject
empty output or unknown patch references locally with no automatic model repair.
Keep raw malformed output as diagnostics. Do not manufacture severity/findings
from prose just to populate existing counters. Additive API/UI fields must make
the mode and absence of a separate verification pass visible.

Use a series-level review record (`patch_id = NULL`) and explicit covered-patch
references rather than duplicating one response and its usage across every
patch. Define rerun behavior so a mode change does not reuse old workflow results
or overwrite successful artifacts. Update statuses atomically only after storing
the response, covered scope, and artifacts. Cancellation must remain terminal
even if a late provider result arrives. Public posting stays separately gated.

Include mode, resolved instruction bytes/order, input scope, output-contract
version, provider, model, actual effort, and completion profile in cache/review
identity. Relative paths or a binary hash alone do not identify custom prompts.
Do not reuse the earlier paid Copilot cache while its effort identity is absent.

## Incremental Delivery and Tests

Implemented: typed settings and fail-closed selection (`76439f00`), plus bounded
immutable instruction snapshots (`ad34e99f`). Enabled harness execution is still
rejected; no existing configuration changes its review procedure.

Backend limitation: the installed Codex app-server normal-turn schema does not
offer a verified per-generation output-token ceiling. The operator explicitly
approved between-turn accounting with one-generation overshoot, as described
above. Its goal-budget API and experimental rollout tracking are not relied on
for hard-stop guarantees. Future metered-backend support must not silently inherit
subscription-specific spending authorization.

1. Add typed config and startup validation for the approved `agent_harness`
   mode, keeping defaults unchanged and rejecting invalid combinations or
   absent enforceable limits and required usage reporting.
2. Add deterministic instruction/input loading and a fake-harness runner test
   proving exactly one harness invocation for a multi-patch series. Test
   internal-turn and tool limits, zero calls on unsupported capabilities,
   missing instructions, oversized input, empty scope, or no-AI. Do not claim
   this proves one backend model generation.
3. Wire shared daemon/local whole-series dispatch and series-level persistence.
   Test cancellation, timeouts, failure artifacts, rerun/mode changes, access
   control, muted delivery, and no hidden workflow/repair calls.
4. Add visible mode/artifact attribution and documentation; verify existing
   default-mode behavior. Run required deterministic gates before each commit.
5. Only after explicit bounded trial authorization, deploy on Luna-low/default
   and measure actual CLI generations and usage. Do not change model or run
   paid Copilot self-review as part of this work.