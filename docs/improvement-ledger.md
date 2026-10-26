# Sashiko Improvement Ledger

Updated: 2026-10-26. Work in priority order, updating evidence and status as
changes are validated. Do not create work merely to keep the daemon busy.

## Operating Constraints

- Keep the daemon on Copilot CLI `gpt-6-luna`, low effort, default context until
  the user explicitly authorizes another provider or model.
- Do not run Sashiko self-review through paid Copilot CLI. Do not spend Sol
  quota to investigate or validate these items without fresh authorization.
- Preserve muted outbound delivery, completed results, review artifacts, and
  user edits. Back up the database before maintenance; use supported retry APIs.
- Keep healthy reviews running. Stop the daemon and workers before tests or
  deployment, resume promptly, and verify actual forward progress.
- Validate with deterministic tests and fake providers before paid experiments.
  Follow AGENTS.md for sign-offs, checks, focused commits, and authorized pushes.

## Prioritized Work

### P0: Resolve Verification Failures That Lose Completed Analysis

Status: contract violation reproduced; prompt and retry feedback improved;
all deterministic gates passed. Deployment and supported retry are next.

Evidence: a Luna review of the x86 microcode timeout-removal patch exhausted
three validation attempts on 2026-10-26 at 02:23 UTC with
`dismissed_concerns[0].source_ids must name input IDs`. Analysis had completed,
but the patch did not receive a completed verified result.

Diagnosis: the request supplied D1, D2, and D3, but the model copied `id: "D1"`
instead of returning `source_ids: ["D1"]`. The validator correctly rejected it.
The prompt now gives this exact distinction, and retry errors list valid IDs.
A focused regression checks the rejected shape, accepted lineage, and rejection
of unknown IDs. Evidence and provenance checks remain strict.

Next: deploy and use the supported retry API for the affected review. Do not
claim prompt changes guarantee model compliance. Checks passed with 1,088 library,
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

### P2: Add an Optional Skill/Agent-Driven Single-Prompt Review Mode

Status: requested and authorized for planning and implementation after the most
severe functional issues are resolved. Write and review a design first.

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
- Test with a fake provider that a series uses exactly one review invocation,
  with no hidden workflow stages or automatic repair calls. Test default-mode
  compatibility, errors, cancellation, persistence, and absent dependencies.
- Implement and deploy incrementally after the design is reviewed. Any paid
  trial must remain on the authorized Luna configuration with bounded usage.

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