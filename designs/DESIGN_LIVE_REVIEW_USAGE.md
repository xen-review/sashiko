# Live Review Usage Attribution

Status: active workflow stage snapshots implemented and validated on 2026-10-27.
No model test, historical backfill, billing change or public posting is included.

## Existing Boundary

Workflow StageFinished carries token totals but omits the SessionUsageCoverage
already present in the stage result. WorkerProgressEvent retains only stage
names, local progress becomes a display string, and the daemon registry exposes
only that string. Parsing text to reconstruct usage would invent provenance and
confuse missing counters with measured zero. Native backend generations and
hidden CLI retries are not observable from these events.

## Incremental Contract

1. Attach measured coverage to StageFinished and test known-zero versus missing
   provider usage in the existing fake-provider test home. The fields describe
   one completed stage, not cumulative workflow totals or in-flight tokens.
2. Propagate typed stage-completion usage through worker/local progress and a
   separate structured subprocess record. Retain the human progress string for
   compatibility; never parse it as an accounting protocol.
3. Store per-run, per-patch and per-stage snapshots in the existing progress
   registry. Duplicate/stale records replace or are ignored rather than added
   again. Whole-series scope stays a single attribution, not duplicated across
   patch rows. Preserve missing/partial coverage and actual-effort Unknown.
4. Expose bounded active snapshots only after existing authorization and embargo
   checks, then render completed-stage measured totals and coverage. An active
   generation has unavailable usage until the provider reports it. Keep daemon
   progress independent of browser polling. Reconnects read the same snapshot.
5. Separately prove terminal persistence for failed, timed-out and cancelled
   reviews, retaining completed work and avoiding duplicated retry usage. Extend
   the same contract to native generation telemetry only if the backend actually
   reports it; do not rename Sashiko turns into backend generations.

## First Verification

Use a fake provider returning an explicit zero AiUsage and the same provider
returning no usage. Their numeric totals may both be zero, but the lifecycle
event must preserve complete measured coverage only in the former case. Match
the event's fields to the saved StageOutcome exactly. Custom stages and skipped
stages remain unknown unless their existing contract establishes no requests.
No UI or completed-result schema change is needed for this first event increment.

## Remaining Gates

The first four increments are implemented for workflow worker subprocesses.
Coverage is a separate accounting event, so preliminary stages do not advance
the counted-stage progress bar. The registry replaces reports by patch/stage
within a run, rejects stale-run output and caps retained reports at 256 with
explicit truncation. Both detail routes attach the snapshot after existing
authorization/redaction. The UI labels these latest reports rather than totals.
The native harness path does not yet emit these events; no scope is fabricated.

Focused fake-provider, JSON round-trip, registry and API projection tests pass.
Synthetic browser checks prove missing versus zero, escaped hostile stage names,
polling updates, reload/reconnect, truncation, terminal removal and contained
390/1440-pixel layouts. One positive-update wait expired at 10 seconds before the
update arrived; a fresh API/readback confirmed matching rendered counters, and
foreground terminal polling passed. Do not treat that as a latency guarantee.
The full make check-pr gate passes 1,160 library tests (two ignored), 49 main,
seven CLI and 28 integration tests, formatting and all Clippy profiles. An
initial all-feature link exhausted disk; package-scoped debug cleanup and
CARGO_INCREMENTAL=0 resolved it without touching operational data.

Full deterministic checks and signed delivery remain required for each code
increment. Later tests must cover parallel stages, errors, duplicate and stale
events, reruns, whole-series scope, cancellation, reconnects, terminal retention,
embargo/access redaction and desktop/mobile containment. The active-stage
snapshot is not the completed live/final usage feature. It does not authorize
model calls, premium trials, self-review or an automatic budget controller.