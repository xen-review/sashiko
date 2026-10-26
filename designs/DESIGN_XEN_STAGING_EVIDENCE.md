# Xen Staging Membership Evidence

Status: integrated index/evidence architecture and the explicit
refs/remotes/upstream/staging target approved by the operator on 2026-10-26
after the interrupted approval question was recovered. Implement incrementally
with deterministic tests and backed-up migration, without scheduling changes.
The existing recorded review-state filter is deployed and remains view-only.

## Local Evidence and Decision

The repository already stores a stable Git patch ID on mailed patch records.
GitSyncWorker refreshes remotes through refresh_remote, but neither an existing
patch ID nor a successful fetch is a saved per-patch membership classification.
At the read-only inspection on 2026-10-26, origin/staging resolves to
4e3c0eeac5, while upstream/staging resolves to 46678752be. Selecting a remote by
name, local HEAD, or commit timestamp would therefore change the answer without
establishing authoritative upstream or ref freshness. No ref was fetched or
changed for this inspection. Commit dates are not fetch-success timestamps.
The configured endpoints, inspected without credentials, are origin at the
xen-review/xen GitHub fork and upstream at xenbits.xen.org/git-http/xen.git.
The recommended explicit target is refs/remotes/upstream/staging, subject to
operator confirmation rather than silently changing project configuration.

Recommended decision: an explicitly configured authoritative remote-tracking
ref, a bounded background Git patch-ID index, and separately persisted per-patch
evidence. Do not run a history walk or patch-ID calculation in HTTP handlers.
Do not reuse ReviewStatus or overwrite patch application/review state.

Alternative: an explicit operator-run, read-only classification command with
retained JSON artifacts, no database schema or automatic UI classification.
It reduces runtime integration but cannot provide reliable paginated merge
filters without additional persisted state. Choose this only if the operator
prefers an investigative tool over the requested integrated view.

## Data Contract

Use a typed optional project-specific configuration with an exact remote/ref,
freshness limit, and bounded indexing limits. Disabled is unchanged behavior.
An absent, ambiguous, missing, stale, or failed-to-refresh target is Unknown,
never proof that a patch is absent. Check ref arguments as data, not revisions
supplied by submitted mail; reject revision expressions and option injection.
Pin the resolved tip before each batch and discard results if the target moves
before committing evidence. Preserve the last check as diagnostic history.

Store evidence independently of the review and human-review dimensions:

- Exact target ref, repository identity, resolved tip, successful refresh/check
  time, and evidence/index algorithm version.
- Original patch identity and stable patch ID; matching upstream commit and
  match kind (exact ancestry or patch-ID equivalence) when available.
- Coverage limits and completeness, error/unknown reason, and current check
  result. Missing evidence is Unknown, not a default Unmerged value.
- Index entries keyed by immutable upstream commit and stable patch-ID version,
  with reachability attached to the pinned ref snapshot rather than assumed
  forever after the first match.

Keep original successful reviews, comments, failed artifacts, and muted delivery.
Back up before migrating. Use existing safe git_cmd process constructors,
blocking-work boundaries, cancellation, fetch/backoff helpers, and database
transaction patterns. Do not add an independent remote-fetch loop.

## Matching and UI Semantics

Prove exact commit ancestry when a trustworthy patch commit is available.
Otherwise compare stable patch IDs of the actual mailed diff and candidate
upstream commits, not subjects or apply outcomes. A patch-ID match proves a
diff-equivalent commit is reachable at that snapshot; it does not prove the
change remains effective, was not reverted, or entered master. Label that
evidence precisely rather than claiming semantic acceptance.

Bound first-pass history work. If the index does not cover the relevant history,
an unmatched patch remains Unknown or No match in checked range, not Unmerged.
Do not guess through squash, modified application, reverts, binary/empty diffs,
incomplete series, unavailable patch IDs, or force-pushed refs. Refresh pinned
reachability after target rewrites; an old positive must not stay current by
accident. Indexing failure must not block ordinary review throughput.

Expose per-patch evidence first. Aggregate a complete received series as all
matched, partially matched, no matches in fully checked scope, or Unknown;
retain missing-part and bounded-coverage uncertainty. The visible wording must
distinguish diff-equivalent history from a currently effective change. Staging
and master are separate targets and never inferred from one another.

Only enable a paginated filter when its predicate and count use the same saved
current evidence snapshot. Show target tip/check time, match kind, and stale or
unknown reasons. Do not add the deferred human-reviewed filter without separate
attributable evidence such as parsed review trailers/replies. A generic reply
does not prove human review, and lack of recorded evidence is not proof of none.

No scheduling change is included. A later policy for prioritizing unmatched work
requires separate approval and tests; it must preserve active and completed
reviews and retain unmerged patches in mixed series.

## Incremental Checks

1. Fake Git repositories cover exact ancestry, rewritten commit IDs with stable
   diff identity, staging/master divergence, duplicate patch IDs, unavailable
   refs, explicit freshness, moved tips, force pushes, bounded scans, empty and
   binary diffs, and reverted changes without a semantic-acceptance claim.
2. Database tests cover immutable evidence versions, atomic snapshot publication,
   stale/unknown transitions, partial series, restart/cancellation, idempotent
   incremental indexing, and no review/status mutation.
3. API/UI tests cover row/count agreement, overlapping review/merge dimensions,
   provenance and stale reasons, access control, list/search/pagination, and
   desktop/mobile layout. Fake clocks/providers precede any operational trial.

Do not start a model review, public post, benchmark, broad history scan, schema
migration, new scheduled worker, or upstream fetch solely to draft this design.

## Implementation Progress

The opt-in Xen configuration and read-only bounded matcher are implemented in
source. Target refs must name the configured remote explicitly. The matcher
pins an immutable tip, bounds reachable commits, total new diff bytes, and time,
then rechecks that tip before returning. Its versioned immutable-commit identities
can be reused, but reachability is enumerated afresh for every snapshot. It uses
the existing stable Git patch-ID helper and safe Git command constructors.

Temporary-repository tests cover rewritten commit IDs, master/staging divergence,
missing refs, byte/history limits, cache reuse, removed reachability after ref
rewrites, binary/empty diffs, and historical matches after reverts. Merge commits
are explicitly unsupported rather than treated as textual single-parent diffs;
duplicate matching commits are retained, not arbitrarily collapsed.
The full make check-pr gate passed 1,137 library tests (two ignored), 49 main,
seven CLI, and 26 integration tests, formatting, and all Clippy profiles. The
installed daemon was resumed after the idle validation boundary.

Schema 16 and atomic evidence storage are now implemented in source. Immutable
commit identities are cached by repository and algorithm version. Reachability
belongs to each snapshot, and a separate publication pointer changes only after
all bounded mailed-patch evidence is saved. Conflicting cached identity rejects
the transaction and preserves the previous pointer. Reads return typed matched
history, no match in checked range, Unknown, or Stale with patch-ID provenance,
tip, refresh/check timestamps, and explicit limitations. Source identity changes,
missing evidence, failed refresh/indexing, stale timestamps, and backwards clock
movement cannot become current positive evidence. No review status is mutated.
The five matcher/storage tests and opt-in settings checks pass. The full storage
gate passed 1,138 library tests (two ignored), 49 main, seven CLI, and 26
integration tests, formatting, and all Clippy profiles. Historical-migration
fixtures explicitly remove schema-16 tables before rewinding and now verify the
new final version. The installed daemon is resumed on schema 15; no live
database migration has occurred.

Refresh integration, exact-ancestry matching, batched API retrieval, and UI/filter
classification remain pending. The runtime is unchanged; there has been no live
repository scan, fetch, migration, or model call for this feature. Do not infer
feature rollout from the configuration or storage types alone.