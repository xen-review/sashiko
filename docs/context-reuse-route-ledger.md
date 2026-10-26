# Context Reuse Route Ledger

Updated: 2026-10-26. This is the execution ledger for the selected route;
the general improvement ledger retains other independent tasks.

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

The user reports 67% of the five-hour allowance remaining and approximately
3h40m until the 14:00 reset. This is a user-reported sample, not independently
read telemetry or a cost forecast. Do not equate percentage quota with tokens
or promise all milestones within this window. Preserve a debugging reserve.
Development review calls must use subscription-backed gpt-6-luna, low effort
by default. This policy is separate from the coding assistant's model setting.

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
3. Next: wire local review dispatch and explicit output labeling. Preserve no-AI mode,
   exact applied commits, selected patch scope, and settings-relative snapshots.
   Completion is not independent verification. Keep unsupported daemon mode
   rejected until persistence and cancellation are implemented.
4. Add daemon whole-series persistence and cancellation with one series usage
   record, covered patch identities, atomic status updates, and no automatic
   public delivery. Validate before enabling this mode in runtime settings.
5. Establish one bounded Luna evaluation with retained artifacts and measured
   cached input, uncached input, output, provider calls, tools, and review quality.
   No full/small benchmark or automatic queue refill is authorized by this plan.
6. Design sequential checkpoints on this engine before implementation. Retain
   instruction/tool declarations and complete series context as a stable prefix;
   append patch requests, prior replies, and source evidence in order. Bind each
   checkpoint to exact baseline, series revision, and selected patch identity.
   Do not blindly rerun prior completed patches or promote old conclusions to
   operator instructions. Ask before changing architecture beyond this route.
7. Compare quality and total usage on the same small series, then expose a typed
   configurable strategy. Keep phase-by-phase reuse as a later experiment rather
   than adding hidden planning/verification phases to custom-agent execution.

## Cache and Correctness Gates

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

Work in the clean detached implementation worktree
/tmp/sashiko-improvements-20261026. The main checkout contains unrelated user
CI and agent edits; never stage or revert them. Fast-forward main and push only
origin/add-xen-support after signed commits and required checks.

The installed daemon remains on the older restricted Codex completion runtime;
new harness source is not yet enabled or deployed. Preserve mute_all and SMTP
policy. Do not stop an active review for routine checks. Before tests, stop at
a safe boundary, verify workers exited, and resume the service promptly.

Use four Cargo jobs and the shared target directory. Full checks require the
local Git 2.56.0, GIT_TEMPLATE_DIR=/usr/share/git-core/templates, and the shared
CARGO_BIN_EXE_sashiko override for linked-worktree integration tests. Self-review
remains deferred under the explicit spending and final-only restrictions.