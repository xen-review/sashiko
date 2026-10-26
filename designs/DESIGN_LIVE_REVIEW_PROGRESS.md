# Design: Live Review Stage Progress

## Status
Implemented and validated.

## Context
The local review worker already emits concise stage and turn progress lines to
stderr. The daemon currently logs those lines but does not retain them. The
patchset detail page fetches its summary once, so it cannot display the current
stage beneath its `In Review` badge.

## Design
1. Add a cloneable, in-memory progress registry keyed by patchset ID. The
   reviewer updates the entry when it reads the worker's existing progress
   prefix and removes it when the review subprocess operation finishes,
   including cancellation and timeout. Run IDs reject late updates from an
   earlier worker for the same patchset.
2. Share the registry between the reviewer and API server through the existing
   startup options. Add the current value to the existing patchset summary
   response after its normal access checks; do not persist transient progress
   in the database.
3. Render the value as escaped text directly below the patchset status badge.
   Poll the same summary endpoint while the page is open and the patchset is
   `In Review`. Stop polling on navigation or when the review reaches a
   terminal status, refreshing the page once to show its final result.

The registry holds the most recently received progress line for each patchset,
including the patch index, stage, and turn when present. Progress is
best-effort and is cleared on worker exit or daemon restart.

## Verification
- `cargo check --all-targets` passes.
- `cargo fmt --check` and inline JavaScript parsing pass.
- Registry lifecycle and summary scoping tests pass.
- `make check-pr` passes with Git 2.56.0 built with PCRE2 and standard Git
  templates available. Run tests only while the daemon and workers are stopped.