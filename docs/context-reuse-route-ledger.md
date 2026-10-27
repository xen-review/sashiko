# Context Reuse Route Ledger

Updated: 2026-10-27. This is the execution ledger for the selected route;
the general improvement ledger retains other independent tasks.

## Current Continuation

### Retained-Diagnostics Rollout Checkpoint

At 16:50 CET on 2026-10-27, signed/pushed source 9f419024 is deployed with schema
18. Each workflow worker retains its own completed-stage diagnostics by exact
review ID as stderr records arrive. Migration 018 leaves historical snapshots
null. Bounded snapshots survive normal/failed/cancelled completion without
overwriting final interaction totals, and the timeout regression proves a
reported stage survives an error while the fake child is reaped promptly.
Parallel workers do not borrow each other's patchset display-registry state.
The post-reap stderr drain is bounded at one second and warns on interruption;
retained records are partial diagnostics, not guaranteed complete final usage.

The existing shared review query exposes snapshots on both detail routes after
the same permission/embargo boundaries, without per-review queries. Cancelled
cards render retained records, and their summary stays Cancelled rather than
In progress. Historical rows remain unknown. Storage bounds, replacement,
legacy null, cancellation retention, timeout failure/reaping, both projections
and embargo exclusion pass focused checks. The full gate passes 1,161 library
tests (two ignored), 49 main, seven CLI, 28 integration, formatting and all
Clippy profiles. Synthetic terminal-card checks prove hostile names stay text
and 390/1440-pixel layouts do not overflow. A component screenshot timed out;
that capture is not claimed as passed. The private fixture is stopped.

Private before-retained-stage-usage-20261027-155045.db passed quick_check;
matching rollback binary and Settings snapshot are preserved outside Git.
Installed/running/release SHA-256:
051280368d05e235c9b9ff488d49ef4f9c834866c49f311618cabe44edac7e12.
Service active PID 1195489, schema 18 and both APIs verified. Review counts
remain 110 Reviewed, 90 Cancelled, 27 Failed, 195 Failed To Apply and 46
Incomplete, with no Pending or In Review. Raw messages numbered 4,285 before
deployment. Retained snapshots are still zero in live data: no diagnostic
backfill, feature-specific review or cancellation was started. The seven
positive human-evidence series and terminal detail provenance remain intact.

Configuration bytes are unchanged: codex-cli / gpt-6-luna / configured medium,
1,800-second review timeout, backend-effective effort Unknown, existing auth
and muted delivery. The mirror timer is restored unchanged. Fresh metadata at
16:51:29 CET reports 45% five-hour and 43% weekly remaining, reset 19:27:29 CET;
premium calls are ineligible. The observed shared decline from 78% at 15:35
includes other consumers and is not an isolated daemon burn or exact token
forecast. No controller, self-review, model turn, benchmark, paid overage,
public post, PR, queue refill or watcher was started. First live staging
publication, opaque quota-account binding, complete interrupted/native/harness
usage and backend-effective effort remain open; do not infer or fabricate them.

### Daytime Active-Usage Checkpoint

At 16:07 CET on 2026-10-27, source/runtime a535d045 is signed, pushed to
origin/add-xen-support and deployed. It follows the bounded metadata-only
Codex reader 6d5cf3fd, which remains unwired: no admission controller or watcher
is enabled. Login/plan comparison is not opaque account/workspace binding;
weekly reserve, durable state, cancellation and wakeup integration remain gates.

The active-usage increment preserves measured stage coverage through a separate
workflow/worker/local accounting event and typed subprocess record. The daemon
retains latest patch/stage reports per worker run, replaces duplicates, rejects
stale output and caps reports at 256 with explicit truncation. Both detail
routes attach snapshots only after existing permission/redaction checks.
Pre-screen/planning accounting cannot advance counted-stage progress. The UI
keeps unknown usage distinct from measured zero and labels latest reports, not
workflow totals or in-flight counters. Native harness attribution and terminal
retention across failures, timeouts and cancellation are still incomplete.

Focused fake-provider, JSON protocol, bounded-registry and API tests pass.
Synthetic browser checks cover missing/zero, escaped hostile names, polling,
reload/reconnect, truncation, terminal removal and 390/1440-pixel containment.
One positive update arrived after its 10-second wait timed out; matching fresh
API/rendered counters were then verified, not a latency guarantee. The full
gate passes 1,160 library tests (two ignored), 49 main, seven CLI, 28 integration,
formatting and all Clippy profiles. A disk-full all-feature link was repaired
with package-scoped debug cleanup and CARGO_INCREMENTAL=0. The user mirror
timer was paused only during test/deploy isolation and restored unchanged.
Review data, private backups, artifacts and user-owned files were preserved.

Private before-live-stage-usage-20261027-150754.db passed quick_check; matching
rollback binary and Settings snapshot remain outside Git. Installed/release
SHA-256: 838c44c4a290f26d82ec5a0b247f39094c583a6b62fd8ed0625bd2a701bd1cc3.
Service active PID 1188587, schema 17 and API health verified; raw message count
was 4,284 at deployment. Review counts remained 110 Reviewed, 90 Cancelled,
27 Failed, 195 Failed To Apply and 46 Incomplete; none Pending or In Review.
No live review was started solely to test telemetry. Normal new-mail ingestion
now yields seven positive human-evidence series, including 468; both detail
routes retain provenance and omit active snapshots for terminal reviews.

Configuration bytes are unchanged. Runtime remains codex-cli / gpt-6-luna /
configured medium, 1,800-second review timeout, effective effort Unknown, muted
delivery and existing web-admin authentication. At 16:09:38 CET authenticated
metadata reports 63% five-hour and 46% weekly remaining; five-hour reset
19:27:29 CET. Earlier samples were 78%/48% at 15:35 and 69%/47% at 15:49;
these shared-account percentages are not exact tokens or isolated daemon burn.
No open matching PR exists in the fork or parent; no Stage 2 findings are pending
on this branch. No self-review, premium trial, benchmark, public post, paid
overage, historical backfill, queue refill or watcher was started. First live
staging publication remains unresolved; do not manufacture fetch evidence.

### Daytime Maintenance Checkpoint

At 15:15 CET on 2026-10-27, source/runtime 2fbfa794 is signed, pushed and
deployed. A repeating RepackWorker failure was reproduced in a temporary
blob:none clone: Git rejects geometric stdin packs with promisor exclusion.
Full clones retain geometric packing; partial clones now use supported
incremental packing without pruning or changing remote configuration. Both
Git 2.39.5 and 2.56.0 pass object retention, reachable loose packing and real
filtered-clone missing-blob tests. The full gate passes 1,153 library tests
(two ignored), 49 main, seven CLI and 28 integration, formatting and all Clippy
profiles. A disk-full link was repaired by deleting only regenerable Cargo
incremental cache; operational backups, review artifacts and user files remain.

Private before-promisor-repack-20261027-151506.db passed quick_check; matching
rollback binary and Settings snapshot are retained. Installed/release SHA-256:
2e26c91a3607447c4737d226faf9407725ea2c3accea46dac9f8809490d40db6.
At the checked idle boundary, live incremental maintenance moved 204 loose
objects into a pack, retained all 23 promisor markers, and left refs and
repository configuration unchanged. API healthy, schema 17, service active
PID 1168273. All 110 Reviewed series are preserved; no Pending or In Review.
No active review was interrupted, no model turn was started for the repair,
and provider/model/effort, auth and muted delivery settings are unchanged.

Normal ingestion now has six human-review evidence records on six series.
The live positive filter and both detail routes agree on exact targets and
provenance. A Failed To Apply series remains in that independent AI state.
The real mobile disclosure renders its source link and caveat without overflow
after DOM activation. Pointer activation timed out and keyboard automation was
inconclusive; neither is claimed as a passed live interaction test.
No historical backfill was run. Staging publication is still unobserved. A
user-owned mirror-refresh timer runs every 30 minutes and the upstream fetch
timestamp is recent; do not change that timer or force-fetch to manufacture
snapshot evidence. Latest fresh quota at 14:56 CET: 99% five-hour, 51% weekly,
five-hour reset 19:27:29 CET. No reset credit, premium experiment, benchmark,
self-review, public post or watcher was started. The quota foundations remain
unwired; bounded metadata RPC is the next independently testable increment.

### Deployed Human-Evidence Checkpoint

Latest deployed source/runtime at 01:53 CET on 2026-10-27: 47cdd5c1 is signed, pushed to
origin/add-xen-support, and deployed. Parser foundation d1e13b6e and opt-in
storage d4549623 precede the API/UI increment. Installed/release SHA-256:
bfa5ef8d2baad4dca76b58d466ae761eabaa45bf20984385bdb2ebb7c4fc0f08.
Private before-human-review-20261027-015113.db passed quick_check; matching
rollback binary and Settings snapshot remain outside Git. Schema 17, active
service PID 1132519, and API health are verified. There are 4,076 messages,
456 series, 103 Reviewed, and no Pending or In Review. The four additional
completed reviews were normal daemon work, not feature-specific model tests.

The approved direct-reply store and positive-only filter are enabled for normal
new-mail ingestion through server.human_review_evidence. Raw-only ambiguity and
source/target changes invalidate current evidence without deleting provenance;
old replies cannot be rebound to changed targets. The exact reply target and
matching actual From/Reviewed-by mailbox are required. Both detail routes return
bounded provenance. Row/count filtering, independent review/staging parameters,
cache isolation, partial series, idempotency, opt-out, and the 200-record cap
pass deterministic checks. The full gate passes 1,145 library tests (two
ignored), 49 main, seven CLI, 28 integration, formatting, and all Clippy profiles.
Native disclosure/escaping, Clear/URL restoration, Messages isolation, and
1280/390-pixel containment pass. The private fixture is stopped. The live
browser is reloaded on the enabled positive filter and correctly shows zero
records; no historical backfill or synthetic mail was inserted into live data.

Structured config comparison verifies that only the opt-in changed. Runtime
remains codex-cli / gpt-6-luna / configured medium, 1,800-second review timeout.
Unconfirmed backend-effective effort stays Unknown. SMTP, testing bypass, and
sign-in-link logging remain off; web-admin authentication and muted delivery
are preserved. No active review was interrupted. Fresh authenticated quota at
01:53:32 CET: 43% five-hour, 54% weekly remaining; reset 05:03:09 CET. Shared
allowance declined from 67% at 00:57 and 51% at 01:38; this is not an exact
token count or isolated Sashiko burn. Premium calls are ineligible. No self-review,
benchmark, PR, public post, paid overage, or watcher was started.

The first staging snapshot remains unobserved. Do not force a fetch, mutate its
timestamps, weaken covering-fetch checks, or restart merely for reassurance.
Quota planning/deferral and complete native/live usage remain open. The quota
design is committed as ed14a3cf and the first disabled pure policy now passes
five fake-clock tests and the full gate (1,150 library, two ignored; 49 main,
seven CLI, 28 integration). Source carries this unwired foundation; the deployed
human-evidence release stays unchanged. Authenticated metadata adapter, weekly
reserve checks, settings, persisted state, cancellation, wakeup scheduling and
paused-state UI remain future gates. No automatic controller is enabled.

Quota policy source is committed as 88243c76. The subsequent pure Codex metadata
decoder passes fake-response and full gates (1,151 library, two ignored; 49 main,
seven CLI, 28 integration). The metadata-only schema read at 02:10 CET reports
35% five-hour and 53% weekly, with the same 05:03 CET five-hour reset. Premium
calls are ineligible. Runtime binding is unresolved because account/read exposes
login/plan but no opaque workspace ID. A derived hash is not a substitute for
that missing binding. No account controller, paid self-review, benchmark, public
post, backfill, or watcher is enabled. Deployment remains 47cdd5c1/schema 17.

Decoder source is committed as f6411372 and all source commits are signed and
pushed to origin/add-xen-support. Final read-only checkpoint at 02:20:38 CET:
30% five-hour and 52% weekly remaining, reset 05:03:09 CET; service active,
PID 1145716, 103 Reviewed series, no Pending or In Review. The positive human
filter and staging snapshot store remain empty, without a historical backfill
or forced fetch. There is no open matching PR in xen-review/sashiko or parent
sashiko-dev/sashiko, so no Stage 2 bot findings are pending on this branch.
Stage 1 self-review remains prohibited/deferred, not passed. The metadata-only
decoder and policy do not install a watcher or authorize runtime admission;
stable account/workspace binding is a concrete integration gate. Preserve the
healthy daemon while those external/protocol and remaining budget gates hold.

### Earlier Exact-Ancestry Checkpoint

Latest source/runtime at 00:38 CET: exact-ancestry increment 113b83a5 is signed,
pushed, and deployed. Installed/release SHA-256:
9617248a4903bf54a06ba2765f8fe103c2a39caf273eeee408ac18750ed71bac.
Private before-exact-ancestry-20261027-003736.db passed quick_check; matching
sashiko-before-exact-ancestry-20261027-003736 rollback binary retained. Service
active, PID 1096952; API healthy, schema 16, 4,072 messages, 452 series, 99
Reviewed, no Pending or In Review. No active review was interrupted or model
turn started. Normal provider/model/effort, scheduling, and muted delivery stay
unchanged. Review 48's original findings and counters remain intact.

Exact provenance reuses saved applied coverage and requires the same immutable
commit and current patch ID in the pinned reachable snapshot. It adds no schema,
history scan, or series-filter changes. Focused storage and both-route API tests,
renderer escaping, and native browser disclosure checks at 1,280/390 pixels pass.
The full gate passes 1,141 library tests (two ignored), 49 main, seven CLI, 27
integration, formatting, and all Clippy profiles. Release build passed with no
new OOM events. The synthetic 8097 fixture is stopped and the shared browser is
back on the live Unknown list; the pre-existing 8096 listener was left alone.

The first normal snapshot remains unobserved (zero saved snapshots). The saved
upstream successful-fetch timestamp is 00:00 CET; normal refresh skips until its
one-hour interval elapses. Do not force-fetch, alter timestamps, or weaken the
successful-covering-fetch requirement merely to finish validation. Attributable
human-review evidence is the next open staging/filter dimension. Quota policy,
live/native usage, and self-review restrictions remain separate.

### Earlier Rollout Evidence

Authoritative rollout state at 00:20 CET on 2026-10-27: bd966c85 is signed,
pushed to origin/add-xen-support, and deployed on schema 16. The implementation
worktree is clean before this documentation update. Main's unrelated untracked
user files remain preserved. Installed/release SHA-256:
ada9eabd9c9154ecb60e3f08a298b541950f6b1a240df7f7e89297dd93e140ff.
Private backup before-staging-evidence-20261027-0010.db and its matching rollback
binary and Settings snapshot are retained outside Git. The stopped backup passed
quick_check before migration, and all 99 Reviewed series remain preserved.

Service active, PID 1086081; API healthy version 0.6.3, 4,072 messages and 452
series, no Pending or In Review. Runtime remains workflow / codex-cli /
gpt-6-luna / medium with muted outbound delivery and no SMTP. The approved
upstream target is enabled with 128 commits, 1,000 mailed patches, 16 MiB of new
diffs, a 120-second indexing bound, and 86,400-second evidence freshness.

No live snapshot has been published yet: zero snapshots, index entries, or patch
evidence. The startup sync cycle at 00:10 CET skipped both fetches as fresh and
correctly stored No successful staging remote fetch in this sync cycle. The
existing sync worker checks hourly; only an actual covering successful fetch
can publish under the approved conservative contract. Do not call this a live
matching success, infer freshness from commit dates, bypass the guard, or restart
the healthy daemon just to repeat it. Recheck after normal forward sync progress.
The source gates and synthetic browser evidence below remain valid, but their
older pending-rollout/schema-15 statements are historical.

Fresh authenticated quota at 00:20:09 CET: 95% five-hour and 62% weekly remaining,
next five-hour reset 05:03:09 CET. These are shared-account percentages, not exact
tokens or permission for self-review, extra experiments, or paid overages. No
model turn, queue refill, force push, self-review, or benchmark was started for
rollout verification. Exact ancestry and attributable human-review evidence
remain open; quota allocation/deferral and complete native/live usage remain
separate ledger tasks. No watcher or temporary fixture process is running.

### Earlier Implementation Evidence

Current task boundary: staging design drafted in
designs/DESIGN_XEN_STAGING_EVIDENCE.md. After interruption, the operator explicitly
approved the integrated bounded index/persisted evidence architecture and
refs/remotes/upstream/staging target on 2026-10-26. Configuration and bounded
matching foundations are now implemented in source, with focused temporary-Git
tests and full gates passing: 1,137 library tests (two ignored), 49 main, seven
CLI, 26 integration, formatting, and all Clippy profiles. The idle daemon was
resumed after validation. Atomic schema-16 evidence storage is also implemented
in source; its gate passed 1,138 library tests (two ignored), 49 main, seven CLI,
26 integration, formatting, and all Clippy profiles. Bounded publication,
immutable cache/rollback, stale/failure/changed-identity invalidation, and no
review-status mutation are tested. Optional refresh integration is implemented
in source using the existing GitSyncWorker only. It requires an actual successful
fetch and a covering refspec, records the fetch time, bounds indexing/publication,
and invalidates failed/skipped refreshes or moved tips. Its gate passed 1,139
library tests (two ignored), 49 main, seven CLI, 26 integration, formatting, and
all Clippy profiles. Batched API retrieval and per-patch provenance UI are also
implemented in source. Both summary/full routes use one bounded saved-evidence
query, leave disabled responses unchanged, preserve Unknown on optional failures,
and expose no canonical repository path. The real-router regression covers
pagination, stale/changed/failed evidence and preserved review state. The full
API gate passed 1,139 library tests (two ignored), 49 main, seven CLI and 27
integration tests, formatting and all Clippy profiles. Inline script parsing,
state labels and hostile-value escaping pass. The provenance commit is 3fb5cef4;
its code tree equals superseded 03ccb0fd. The operator explicitly approved a
one-time exact-lease message-only rewrite to repair literal newline escapes.
No broader force-push permission exists.
Series aggregation and paginated staging_state filters are implemented in source.
They use every saved part, distinct expected part numbers and current identities,
pin one fresh snapshot, and share the row/count predicate before pagination.
Typed API metadata preserves the independent review dimension. The full gate
passes 1,140 library tests (two ignored), 49 main, seven CLI and 27 integration
tests, formatting and all Clippy profiles; duplicate-part coverage also passes.
Synthetic browser checks at 1,280/390 pixels verify native disclosures, filters,
list combination, Messages parameter isolation, UTC times and no page/control
overflow. Screenshots are captured, not independently visually inspected here.
Exact ancestry and human-review evidence remain open. Schema-16 rollout remains
pending. The normal v2 bzimage review completed without new issues or email;
there are 99 Reviewed series. At 00:02 CET the unchanged schema-15 daemon is
stopped at an idle validation boundary. Resume it promptly after the final gate.
No live staging migration, feature-specific fetch/history scan, model call, or
scheduling change has been made. Remote inspection
confirms origin is https://github.com/xen-review/xen.git and upstream is the
Xenbits repository; their staging tips differ, so silently choosing origin
would be wrong for the proposed target. Do not infer freshness from commit dates.

Latest state at 19:11 CET: parser/cache fix 96ab1ae0 is signed, pushed, and
installed; implementation clean before this documentation update. Release hash:
ecaab14961996b13fa9db35ad7804aa6aee7cd2430b1b8c79e4caa4df645f00a.
Backup before-codex-usage-20261026-191039.db passed quick_check; matching
sashiko-before-codex-usage-20261026-191039 rollback binary retained privately.
The four-job build added no OOM events. Service active PID 1031277, API healthy,
schema 15, 4,070 messages/451 series/98 Reviewed, no Pending or In Review.
No active review was interrupted and no model call was needed for validation.

Codex usage now requires measured input/output counters, preserves explicit
cached zero versus unknown, and rejects invalid/overflowed counts. A cache parser
profile excludes pre-fix synthesized usage. Ten focused backend tests and the
full gate passed: 1,132 library (two ignored), 49 main, seven CLI, 26 integration,
formatting, all Clippy profiles. Historical review 48 data is unchanged. Actual
effort remains Unknown: this completion stream supplies text and usage, not a
trustworthy backend-effective effort field. Do not promote configured effort.

Fresh authenticated quota at 19:03:02 CET confirms the reset: 99% five-hour and
71% weekly, next five-hour reset 00:01:44 CET on 2026-10-27. Coding-chat and
daemon-account quotas remain distinct. No provider/model/effort switch, premium
experiment, sleep, benchmark, self-review, public post, or queue refill occurred.
Continue useful authorized ledger work without spending just because reset.

Interruption-resume check at 22:24 CET: daemon healthy, 4,071 messages, same
451 series and 98 Reviewed, no Pending/In Review. Fresh authenticated quota
91% five-hour / 69% weekly, reset 00:01:44 CET on 2026-10-27. That does not
authorize overnight model experiments or self-review. No model call is needed
for the staging implementation. Preserve the existing muted Luna/medium runtime.

### Status-Filter Rollout Evidence

Latest state at 18:58 CET: view-only filter source 44094e87 is signed, pushed,
and installed. Implementation checkout clean before this documentation update;
main's unrelated user-owned files remain untouched. Installed/release SHA-256:
0f97890d61fbe2e24bae2f6fff61e2b103b3e976f267db4a94dc431411106970.
Private backup before-state-filter-20261026-185746.db passed quick_check, with
sashiko-before-state-filter-20261026-185746 rollback binary. Four-job release
build passed without new OOM events. No active review was interrupted.

Service active, PID 1024593; API healthy version 0.6.3, schema 15, 4,070 messages,
451 series, 98 Reviewed, no Pending/In Review. Runtime remains workflow / Codex
gpt-6-luna / medium, mute_all/no SMTP. The live filter matches database totals:
Reviewed 98, not currently Reviewed 353, Failed To Apply 191, In Review zero.
Unknown filters return 400; both live widths show the expected two Reviewed
pages with no page/control overflow. Synthetic fixture 8094 is stopped and the
shared browser now shows the live Reviewed list.

The filter is view-only and uses typed states plus shared row/count predicates
before pagination, with default-cache isolation and unchanged embargo/access
redaction. URL restoration, mode isolation, Clear, page reset, and mobile
container scrolling passed. The full gate passed 1,131 library tests (two
ignored), 49 main, seven CLI, 26 integration, formatting, and all Clippy profiles.
Human-review evidence and staging membership are still separate unimplemented
dimensions, not guessed options or new scheduler behavior. See the incremental
filter design and general ledger for remaining work.

The operator switched coding chat back to Codex and asked to conserve its
remaining allowance until reset. Fresh daemon-account telemetry at 18:57:31 CET:
9% five-hour / 71% weekly, reset 19:01:38 CET. Do not infer chat quota from daemon
account telemetry or assume reset without a fresh reading. No sleeps, model
experiments, premium calls, self-review, benchmark, or public post were used.
Continue only useful authorized work; do not repeat checks just to consume time.

### Usage Rollout Evidence

Latest state at 18:28 CET: source e813b5f2 is signed, pushed, and installed,
including the conservative legacy-usage UI fix 921be4e2 and typed provider-response
coverage. Installed/release SHA-256:
a360fec0d37583ac1ca9df047fe3613445dfc12680903d364e58445c093fc353.
Private backup before-usage-coverage-20261026-182731.db passed quick_check;
sashiko-before-usage-coverage-20261026-182731 retains the prior binary. No active
review was interrupted. Four-job release build passed with no new OOM events.

Service active, PID 1012834; API healthy, schema 15, 4,070 messages, 451 series,
98 Reviewed and no Pending or In Review. Runtime workflow / codex-cli /
gpt-6-luna / medium and muted/no-SMTP policy are unchanged. The queue is empty;
no new review throughput or model call is claimed for deterministic validation.

Final gate: 1,130 library tests passed, two ignored; 49 main, seven CLI, 25
integration, formatting, and all Clippy profiles pass. Sessions retain typed
request/response usage coverage through failures, stages, workflow branches,
worker/local JSON, and saved review output. Legacy/custom records and unaccounted
worker restarts stay unknown. Truncated responses retain their reported tokens.
Coverage is not native generation count, hidden-retry accounting, or complete
billing evidence. Cached-unknown and partial totals stay explicit. No migration,
historical backfill, model re-review, self-review, benchmark, or public post.

Synthetic card/map/stage fixtures and fresh live review-48 patch/map checks pass
at 1,280 and 390 pixels without overflow or injected elements. The original
findings, counters, unknown effort, and unassessed confidence remain unchanged.
Synthetic fixture stopped; browser now shows the live patch, not a fixture.
Latest quota reading at 18:10:16 CET remains 14% five-hour / 72% weekly,
reset 19:01:38 CET. Premium gate is false. Fresh telemetry is still required
before any later experiment. Next scoped ledger work: view-only review-state
filter; staging/human evidence and any new quota controller need their own
reviewed design, not inferred status or scheduling changes.

### Earlier Resume Evidence

The operator resumed the improvement loop on the Copilot Business coding-chat
plan. At 17:50 CET on 2026-10-26 the confidence source eed4fbdd is deployed
from source HEAD 71181008. Release/installed SHA-256:
5f50f8a71435e81961ac67d4f9a7e7085164ae9e73517fe96d2199a57e74c57f.
The four-job release build passed with unchanged OOM counters. The idle database
backup before-confidence-20261026-174938.db passed quick_check; the matching
sashiko-before-confidence-20261026-174938 rollback binary is retained privately
under the Xen data backups directory. No active review was interrupted.

Service active, PID 991092; API healthy, schema 15, 4,066 messages, 451 series,
no Pending or In Review. Ingestion advanced after restart. No new model call was
needed to verify rollout. Freshly served HTML includes the confidence renderer;
live review 48 at 1,280 and 390 pixels has no overflow, two Not assessed labels,
Unknown effective effort, no synthetic score, and unchanged saved findings.
Runtime remains workflow / codex-cli / gpt-6-luna / medium, muted with no SMTP.

Latest authenticated quota at 17:48:55 CET: 14% five-hour and 72% weekly,
reset 19:01:38 CET. The premium gate is false; no model experiment is planned.
The chat plan change does not alter daemon billing or self-review restrictions.
The next task at that point was to distinguish unavailable historical token usage from
measured zero without relabeling successful reviews or inventing token counts.
Other open work and validation constraints remain in the ledger and historical
handoff below; its pause and undeployed-confidence statements are superseded.

## Switch Handoff (Historical)

Authoritative state at 17:39 CET on 2026-10-26. The operator requested saving
state before switching the coding chat back to the Copilot harness/plan.
Pause improvement work for that switch; do not start another ledger task or
model experiment as part of the handoff. The claimed chat context size is not
verified here and says nothing about the daemon account, quota, or billing.
All older resume sections below are historical, not current instructions.

### Source and Deployment

- Last code commit: eed4fbdd, optional model confidence with explicit provenance.
  Signed and pushed to origin/add-xen-support; main fast-forwarded to it.
  Implementation worktree /tmp/sashiko-improvements-20261026 is clean before
  this handoff documentation edit. The next documentation commit saves this
  handoff. Remote: https://github.com/xen-review/sashiko.git. No force push,
  new branch, PR, or delegated agent was used.
- Main checkout /home/review/gh/sashiko has unrelated user-owned untracked
  agents, skills, CI/action/hook files, and copilot-instructions.md. Preserve
  them; never stage, delete, or revert them to make main appear clean. Main
  has no tracked diff. Use the clean implementation worktree for new tasks.
- Installed binary remains 8e0ea8be, configured-effort snapshot and UI.
  SHA-256: 4bac44289b2c9ed6ead14dd42a9b1de62d648ddf30e025aa675f41889da669d9.
  Confidence-capable source eed4fbdd has NOT been built or installed. Tracked
  prompt files in main have advanced with source; do not assume their loaded
  daemon snapshot or displayed capability from the binary version alone.
- Service sashiko-xen.service was resumed after checks and is active, PID
  990066 at 17:39 CET. API http://127.0.0.1:8080 reports healthy version 0.6.3,
  4,064 messages, 451 series, zero Pending and zero In Review. Database schema
  15; counts: Reviewed 98, Cancelled 90, Failed 27, Failed To Apply 191,
  Incomplete 45. Historical failures are not authorization for mass retries.
- Runtime remains workflow mode, codex-cli / gpt-6-luna / medium. Delivery
  remains mute_all=true with no SMTP. Do not switch mode/provider/model/effort
  or post results merely because the coding chat changes plan.
- No build, test, foreground review, fixture server, or watcher remains. The
  shared browser is a stale read-only fixture, not a live review. Synthetic
  confidence shown there was never written to the database. No new OOM events:
  cgroup counters still oom=3, oom_kill=2.

### Validated Work

- Sequential harness is implemented and explicitly opt-in: fixed instruction
  snapshot/full-series prefix, one shared bounded conversation, submission-order
  selected checkpoints, patch-scoped source tools, atomic persist-before-next-call,
  disjoint usage, partial failure/cancellation, no-replay restart recovery,
  local partial JSON/agent/human output, and checkpoint API/UI. Workflow remains
  the runtime default; no independent-verification claim or outbound delivery.
- A real three-patch libxenstat comparison used matching instructions, applied
  commits, original message IDs, and native Luna/medium. Whole-series completed
  three replies: 3 calls/9 tools, 146,011 input, 38,400 cached, 107,611 uncached,
  1,758 output, 37.887 seconds. Sequential failed its first artifact: 4 calls/
  3 tools, 200,052 input, 108,160 cached, 91,892 uncached, 1,075 output, 33.815
  seconds. Native malformed content-wrapper bytes match retained failure. This
  is not an equivalent-work cost saving. No automatic repair/retry occurred.
- Response-envelope fix f26e416b separates the catalog from the generic content
  wrapper and requires direct harness artifact JSON. It is deployed. No real
  post-fix comparison has been run; original failed artifacts remain intact.
- User feedback on runstate review 48 is recorded in
  [the transparency report](../designs/DESIGN_REVIEW_TRANSPARENCY.md) and
  [the improvement ledger](improvement-ledger.md). Both supplied attachments
  were readable; no need to ask for them again for the saved facts.
- Review 48 actually has zero regression findings, one Low pre-existing compat
  concern, and one dismissed native-path item. Main page formerly hid the short
  inline result. Deployed UI now shows it directly and matches map disposition
  counts; responsive map tables no longer widen the page. Saved review untouched.
- Independent read-only inspection of its exact saved baseline confirms the
  real compat_handle_is_null definition under CONFIG_COMPAT and the compat
  handler object's matching build selection. The submitted patch changes neither
  file: the constant-zero fallback claim is a false positive. The supplied
  re-review's native NULL portability adjustment is separate. No Xen source
  edits, repeated builds, new Sol call, or public response was made here.
- Deployed Xen guidance 869f4902 requires active conditional/build evidence
  and retains real supported non-default-configuration bugs. It does not
  guarantee model correctness or suppress uncertain candidates.
- Deployed 8e0ea8be records configured effort in existing workflow result JSON.
  The daemon parent replaces worker stdio-proxy attribution with its real backend
  settings snapshot. Cards/maps distinguish configured from effective effort;
  the latter remains Unknown without trustworthy backend confirmation. Historical
  review 48 has no saved effort; never infer low from Luna or use live medium.
- New source eed4fbdd accepts optional confidence score in [0,1] or null with
  evidence rationale, projects only that verification item's assessment, and
  labels it model-reported/uncalibrated. Old/invalid values stay unassessed.
  No severity, lineage, eligibility, or scheduling filter changes. Tests keep a
  1% assessment as a final finding; raw candidate scores are not inherited.
- Latest full make check-pr passed for confidence source: formatting, all three
  Clippy profiles, library tests, 48 main, seven CLI, 25 integration tests.
  Focused confidence contract, projection, fake workflow, JavaScript syntax,
  provenance/bounds/escaping tests passed. Final isolated browser check at 1,280
  and 390 pixels passed: visible scored/unassessed cases and escaped rationale,
  zero injected images, zero page overflow/errors. Pointer automation previously
  timed out; handlers were checked through DOM activation. Browser screenshots
  are on the browser host, not workspace /tmp; do not claim inaccessible images
  were visually inspected. No further source edits followed these checks.

### Resume Order and Constraints

1. Re-read this section, AGENTS.md, applicable skills, the transparency report,
   and the improvement ledger. Recheck live queue/service/worktree rather than
   assume the saved PID, quota, or idle state persists after the chat switch.
2. Confidence deployment is the immediate pending delivery step, not another
   implementation rewrite: build with four jobs, check memory/OOM counters,
   preserve active reviews, stop only at a safe idle boundary, verify children
   exited, back up the database, install atomically, restart, and verify the
   actual served UI/API. Do not relabel or rerun review 48. Existing deterministic
   gates passed; rerun required gates if source changes before delivery.
3. Continue remaining transparency work: backend-confirmed per-stage effort,
   missing versus zero token usage, and real provider confidence observations
   when naturally available. Optional uncalibrated confidence is not correctness
   proof. Do not invent historical metadata or launch a premium re-review.
4. Other open ledger work: Xen staging ancestry/patch-ID classification and the
   status filter beside All Lists; live/final usage attribution; configurable
   Codex quota allocation/hysteresis; durable quota-exhaustion deferral. The
   latter is P3/test-first with a possible later bounded 00:00-06:00 reset trial,
   not an installed watcher or overnight authorization. Intermittent NNTP errors
   need current concrete evidence, not speculative refactors. A representative
   successful post-envelope-fix sequential comparison is still unverified.

Fresh account/rateLimits/read at 17:37:09 CET: authenticated ChatGPT subscription,
20% shared five-hour allowance, 73% weekly; five-hour reset 19:01:38 CET.
These are percentages, not token counts or isolated daemon consumption. The
premium gate requires strictly more than 50% before EACH call, so it is false.
No premium calls, quota redemption, paid overages, or model turns were started
for this handoff. Earlier 29%, 43%, 72%, and 100% readings are historical. The
requested below-20%/above-30% automatic hysteresis is NOT implemented; an earlier
fixed testing reserve was superseded by the operator, not silently reinstated.
Reassess conservatively with fresh telemetry before any further experiments.

Self-review remains deferred/prohibited through paid Copilot CLI. Changing the
coding-chat plan does not authorize daemon self-review on Codex, another model,
or another billing route. No Sashiko self-review passed, no benchmarks were run,
and no PR/public posting was authorized. Do not claim unattended monitoring.

Runtime settings: /home/review/.config/sashiko/Settings.toml. Database:
/home/review/.local/share/sashiko/xen/sashiko.db. Xen repo: /home/review/gh/xen.
Private stopped backups and rollback binaries are outside git under the Xen
data directory's backups folder. Latest deployed backup:
before-effort-attribution-20261026-1720.db (quick_check passed), with matching
sashiko-before-effort-attribution-20261026-1720 rollback binary. Older schema-15
and transparency rollout backups are retained. Never commit these or raw logs.

Use this validation environment from the implementation worktree, only after
checking that no daemon/worker is running; never run suites from Git hooks:

```sh
PATH=/home/review/git-build/git-2.56.0:$PATH \
GIT_TEMPLATE_DIR=/usr/share/git-core/templates \
CARGO_TARGET_DIR=/home/review/gh/sashiko/target \
CARGO_BIN_EXE_sashiko=/home/review/gh/sashiko/target/debug/sashiko \
CARGO_BUILD_JOBS=4 make check-pr
```

Use git commit -s with the configured user identity, wrapped messages, no tool
metadata; push only origin/add-xen-support without force. Workspace search tools
default to main, not the implementation worktree: use absolute paths for pending
implementation reads. External service starts around :00/:30 have happened;
verify actual executable processes, not argument matches on a log-viewer tail.

Preserved trial artifacts: /tmp/sashiko-checkpoint-comparison-20261026,
/tmp/sashiko-effort-trial-20261026, and /tmp/sashiko-harness-luna-trial-20261026.
Temporary transparency fixture source: /tmp/sashiko-review-transparency-20261026.mjs;
server stopped. Starting it with --confidence creates a synthetic score in memory
only. Shared browser page ID: 11b782fd-1ff1-429a-bfca-ca92e71b3e27. Refresh onto
the live app instead of mistaking the fixture for production. fetch_webpage may
return stale SPA content after rollout; verify nonce/no-cache HTML or browser
reload. The transcript path supplied in the conversation remains the fallback
for older tool details; repository memory has a concise current pointer.

## Previous Handoff (Historical)

Authoritative resume summary at 16:05 CET on 2026-10-26. Older milestone and
resume entries below are historical evidence, not current quota readings or
deployment claims. The user switched the coding chat provider; do not
infer daemon authentication, billing, quota, or model from that switch.

- Last code commit: 63649823 (supported sequential opt-in), following 738d5be3
  local dispatch, 5ddb8d3f daemon dispatch, and 4cbed0e1 UI, signed and pushed to
  origin/add-xen-support. Work in the clean detached implementation worktree
  /tmp/sashiko-improvements-20261026. Main contains unrelated untracked user
  agents/skills/CI files; preserve them and never stage or revert them.
- Installed release now matches 63649823; SHA-256
  8c6f53b96a39c67e0f8a2a75ab950ea6f8b4293b7d3cd0784db9b3de47e6c9c0.
  Live schema 15 migrated successfully, quick_check passed, API healthy, and
  service active (PID 952894). No checkpoint session was inserted by rollout.
  Runtime remains workflow, Codex gpt-6-luna/medium, muted delivery/no SMTP.
  Latest check: 95 Reviewed, no Pending or In Review series. Message ingestion
  advanced to 4,045; no new post-deployment review throughput is claimed while
  the queue is empty.
  Do not cancel a later active review just to repeat checks.
- Completed source: migration 15; atomic invocation snapshots, ordered checkpoint
  artifacts and disjoint call interactions; terminal success/failure/cancellation;
  no-replay startup recovery; bounded internal sequential runner; batched review
  detail/list API enrichment and partial-result UI. Daemon sequential dispatch
  now persists before advancing and uses a fresh revision-bound toolbox per
  checkpoint, preserving shared conversation, limits, and cancellation.
  Full latest gate: 1,122 library tests, 48 main,
  seven CLI, 25 integration, formatting, and all three Clippy profiles passed.
- Sequential configuration is an explicit supported opt-in in source, with
  complete/partial JSON, agent, human, no-AI, usage, and exact prompt-prefix CLI
  integration checks passing. The installed daemon remains
  workflow mode. Existing independent workflows and whole-series harness remain
  supported; no sequential trial, new default, or transparent crash resume is
  claimed. See the approved checkpoint design for the partial-completion policy.
- UI rendering passed syntax, escaped injection, active/Failed/Cancelled/Reviewed
  fixtures, desktop/mobile overflow and header checks, raw-log access, and the
  existing disclosure toggle. The synthetic fixture server has been stopped.
- API checkpoint_session contains state, completed_count, provider_calls_recorded,
  nullable provider_calls_attempted, usage_complete, ordered checkpoints,
  selected_patch_indexes, and remaining_patch_indexes. Top-level token totals
  are sums of disjoint recorded interactions. Full invocation input is omitted
  from list payloads. No terminal usage is invented for interrupted calls.
- Daemon fake-provider tests now cover one selected/context-only patch and two
  real commits, source reads through virtual HEAD, persisted state before the
  next call, reversed selection order, malformed second output, supported force
  cancellation, retained diagnostics on explicit rerun, and no delivery.
  Fixed exposed integration gaps: stale toolbox HEAD cache across checkpoints;
  unfinished selected patches not marked active; rerun cleanup discarding
  checkpoint diagnostics; nondeterministic ordering of same-second reviews.
- Local sequential dispatch now retains partial JSON/human/agent artifacts and
  graceful cancellation. Unit tests cover two ordered checkpoints, malformed
  second output, cancellation, shared totals, and unknown terminal call usage.
  The latest full gate passes with 1,123 library, 48 main, seven CLI, and 25
  integration tests plus formatting and all three Clippy profiles.
  Queue inspection confirmed Pending status controls harness scheduling; it
  bypasses independent per-patch worker counts. Leave those counts unchanged
  rather than conflate unverified harness artifacts with independently verified
  workflow completion or embargo-clean classification. Next finish enablement
  gates and deployment are complete. Next run a bounded real comparison only
  after fresh authenticated quota and no-AI input preparation.
- Stopped private backups are outside git under the Xen data directory's backups:
  before-checkpoints-20261026-1604.db (quick_check passed) and rollback binary
  sashiko-before-checkpoints-20261026-1557. A previous 15:57 database snapshot is
  also retained. Scheduled/external starts occurred during build preparation;
  stop only at an idle boundary and verify actual processes before foreground
  tests/reviews. No active model call was cancelled for this rollout.
- Workspace search tools search main by default. For uncommitted implementation
  code, read its absolute path or run targeted rg in the implementation worktree;
  otherwise line numbers and content can misleadingly reflect the older checkout.

Validation environment: use four Cargo jobs and local Git 2.56.0 with
GIT_TEMPLATE_DIR=/usr/share/git-core/templates,
CARGO_TARGET_DIR=/home/review/gh/sashiko/target, and
CARGO_BIN_EXE_sashiko=/home/review/gh/sashiko/target/debug/sashiko. Use
make check-pr before every code commit, git commit -s with wrapped messages,
fast-forward main while preserving user files, and push only the authorized
branch without force. No PR or Sashiko self-review is authorized.

Budget policy is in the updated Sashiko-auto-improver agent. Premium Codex
subscription trials require a fresh authenticated strictly-above-50% five-hour
reading before every new call, verified exact models, sufficient weekly/time
reserve, low/medium starts, and a high effort cap. No metered billing or paid
overages. Last trial readings at 14:01 CET were 100%/86% with reset near 19:01
CET; they are historical. A fresh subscription-backed account reading at
16:05:34 CET reports 72% five-hour and 81% weekly remaining, five-hour reset
19:01:38 CET. These are shared-account percentages, not exact remaining token
counts or isolated Sashiko consumption. Re-read before later calls as needed.
Terra/low and GPT-5.6 Sol/low exact-prompt format replays passed, but returned
empty arrays versus Luna/medium's evidence-backed dismissal. Lower output is
not equivalent review quality or justification for broad Sol use. Artifacts
remain under /tmp/sashiko-effort-trial-20261026; daemon default was not changed.

Recommended comparison route: reuse an already-reviewed three-patch libxenstat
series and its saved baseline, original message identities, identical instruction
snapshot, Luna/medium model, and explicit cumulative limits. Prepare with no AI
first; do not requeue the live series, ingest more work, post drafts, or modify
the daemon's workflow default. The first comparison is exploratory evidence,
not proof of equal quality, cost reduction, or justification for switching modes.

Other ledger work remains: staging merge/status filtering, live/final usage,
configurable quota hysteresis, and durable subscription-exhaustion deferral.
The overnight pause/resume trial is lower priority, test-first, near a reported
00:00-06:00 local reset only with fresh authorization; no watcher is installed.

### Continuation Update

At 15:10 CET the user confirmed the coding-chat plan switch and requested no
clarifying questions until another message/new session. Record conservative
recommended assumptions instead of treating predictions as prior approval;
existing budget, architecture, posting, and self-review restrictions still apply.
The next UI edit described above is now implemented: retained checkpoint notes
and drafts render for active, Failed, Cancelled, and Reviewed sessions alongside
completed/selected counts, remaining scope, and explicit incomplete usage.
Implementation assumption: reuse existing review cards, styling, and disclosure
controls rather than add another page or redesign the application.

Inline JavaScript syntax and rendering checks passed, including legacy harness
compatibility and escaped injection text. Isolated browser fixtures at 1280px
and 390px showed no page/card overflow or header overlap, no injected elements,
all raw-log links, no false review-map links, and a working notes toggle. Visual
snapshots were inspected. The synthetic fixture never touched the live database.
The immediate next task is daemon/local dispatch; sequential startup stays gated
until its persistence and cancellation integration tests pass. No sequential
mode or schema-15 deployment is claimed by this frontend change.

## Three-Patch Comparison and Envelope Follow-Up

On 2026-10-26 at 16:14-16:17 CET, the existing three-patch libxenstat series was
exported read-only with original message identities and its saved exact baseline.
Both zero-AI preparations succeeded. The real runs had identical applied patch
contexts and instruction fingerprints, verified from retained request histories.
Native turn metadata confirmed gpt-6-luna/medium for all seven comparison calls.
Response caching was disabled. Each strategy received the same cumulative caps:
12 provider calls, 30 source calls, 300,000 input tokens, 16,000 output tokens,
and 1,800 seconds. No live series was requeued or modified.

| Strategy | Outcome | Calls | Tools | Input | Cached | Uncached | Output | Seconds |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| whole_series | Three validated replies | 3 | 9 | 146,011 | 38,400 | 107,611 | 1,758 | 37.887 |
| sequential_patches | Failed at first artifact | 4 | 3 | 200,052 | 108,160 | 91,892 | 1,075 | 33.815 |

The sequential native reply begins with the malformed syntax
`{"content":"reviewer_notes":...}`. Its bytes exactly match the retained
failure, so this is not evidence of parser corruption. All four calls reported
usage; no checkpoint validated. Lower uncached input and elapsed time are not
savings on equivalent completed work. No automatic repair or retry was attempted.
The worker's process exit was zero; the structured result correctly says Failed,
so consumers must inspect the result rather than infer success from exit alone.
Artifacts remain outside git in /tmp/sashiko-checkpoint-comparison-20261026.

Local hypothesis: the shared source catalog also supplied a generic final-answer
content wrapper, while the harness requested top-level reviewer_notes and
reply_drafts. Extract only the catalog for harness use and give one explicit
direct-artifact rule; retain the generic provider's envelope unchanged. Focused
tests check both contracts, tool continuation, prefix stability, limits, missing
usage, and cancellation. Full make check-pr passes. This removes demonstrated
instruction ambiguity, not a guarantee that the model will always emit valid
JSON. Any further real trial requires a new bounded decision and fresh quota.

After deployment, the ordinary daemon ingested and completed all five patches
of the RISC-V v8 boot/MMU series. It was left running through that work and
stopped only after completion for the deterministic checks above. This is
post-deployment review progress, not independent confirmation of its findings.

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

Earlier quota samples below are historical and cannot authorize new calls.
Follow the current handoff and updated agent budget policy. Do not equate quota
percentages with tokens, assume a reset, or promise completion within a window.

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
3. Done: local dispatch branches before per-patch workflows, with exact baseline
  and patch commits, selected scope, settings-relative instruction snapshots,
  explicit unverified artifacts, raw failure traces, and model attribution.
  A fake Codex executable proves a two-patch range uses one provider call,
  no-AI uses zero, and malformed output is retained without retry or fallback.
  Required gates passed: 1,109 library, 48 main, seven CLI, 25 integration tests,
  formatting, and all Clippy profiles. Daemon harness mode remains rejected.
4. Done: atomic persistence is implemented with one series usage record,
  covered patch identities, explicit mode/verification metadata, and preserved
  cancellation. Tests cover rollback on the final write, invalid scope,
  instruction identity, duplicate completion, failed artifacts, and cleanup.
  Required gates passed: 1,110 library, 48 main, seven CLI, 25 integration tests,
  formatting, and all Clippy profiles. Cancellation now retains partial session
  history and invocation counts. Daemon execution branches once after existing
  eligibility filters, uses the shared model limiter, and persists without
  email/forge delivery or workflow retries. Fake-provider tests cover a selected
  subset, malformed output, cancellation, and one usage record per invocation.
  Dispatch gates passed 1,112 library, 48 main, seven CLI, 25 integration tests,
  formatting, and all Clippy profiles. Explicit reruns retain prior artifacts;
  daemon validation accepts only supported providers, snapshots, and limits.
  The UI displays escaped notes/drafts, unverified status, and one usage summary
  without a review-map claim. Browser checks at 1280px and 390px covered raw-log
  access, text injection, long source lines, overflow, and header wrapping.
  Final deterministic gates passed the same complete test and lint profiles.
5. Initial bounded Luna evaluation completed on 2026-10-26; broader quality
  comparison remains pending. Two runs on exact Xen commit
  9a6cc49b44b458c2ce4cac36ea6c411d06850ac5 retained full JSON and stderr under
  /tmp/sashiko-harness-luna-trial-20261026, outside the repository. Both used
  Luna/low, nine explicit instruction files, no response cache, and no delivery.
  The first completed with one provider call, zero tools, 70,223 input tokens
  (38,656 cached; 31,567 uncached), and 474 output tokens. Its honest diff-only
  caveat exposed confusion between disabled native tools and caller-executed
  source-tool JSON. A trial-only clarification produced two provider calls,
  three source calls, 62,986 input tokens (25,600 cached; 37,386 uncached), and
  631 output tokens. It inspected the exact changed function, parent, nearby
  caller, and ARM page declarations and reported no remaining concern. This is
  limited inspection, not independent verification or a quality benchmark.
  The second run used more uncached input despite less total input; neither
  run establishes savings or sequential cache reuse. The harness system prefix
  now explicitly explains caller-executed JSON source access, with the existing
  tool-continuation and serialized-prefix regression extended accordingly.
  Full deterministic gates passed: 1,112 library, 48 main, seven CLI, and
  25 integration tests, formatting, and all three Clippy profiles.
  No full/small benchmark or automatic queue refill is authorized by this plan.
6. Sequential checkpoint design drafted in
  [DESIGN_SEQUENTIAL_HARNESS_CHECKPOINTS.md](../designs/DESIGN_SEQUENTIAL_HARNESS_CHECKPOINTS.md).
  The operator approved preserving completed checkpoints on later failure,
  explicit partial-series state, and no automatic replay/session resume.
  Typed strategy selection is implemented with whole_series as the default;
  sequential_patches fails closed until continuation and persistence are wired.
  Focused settings regression and all full deterministic gates passed.
  The shared HarnessSession now owns the fixed prefix, read-only declarations,
  cumulative trace/limits, and original deadline. Whole-series execution uses
  the same checkpoint operation. Fake-provider tests cover two append-only
  checkpoints, shared invocation/input limits, unchanged deadline, terminal
  cancellation, and no calls after expiry. Full gates passed 1,113 library,
  48 main, seven CLI, and 25 integration tests, formatting, and all Clippy
  profiles. Per-patch artifact validation and atomic partial persistence are
  still required before enabling sequential strategy.
  Checkpoint context/request builders now preserve complete series context in
  a fixed contract and append exact selected patch, original message ID,
  applied commit, and parent identity. Tests cover parent attribution across
  unselected context patches, stable context, scope/identity substitutions,
  first-patch baseline, and oversize rejection. The existing artifact parser
  validates one-patch scope without another schema. Full gates passed 1,114
  library, 48 main, seven CLI, and 25 integration tests plus all lint profiles.
  Runtime sequential execution remains disabled pending atomic persistence.
  Completed provider calls now retain their individual usage and exact response
  history boundary alongside aggregate totals. Checkpoint tests verify disjoint
  boundaries, summed input totals, and older trace deserialization without
  invented call usage. The same full deterministic gates passed. This is a
  persistence prerequisite, not a completed database checkpoint implementation.
  Additive migration 15 now defines sequential invocation records, ordered
  patch checkpoints constrained to covered patches, and unique per-call
  interaction links. Schema tests reject out-of-scope and duplicate checkpoints
  and verify review-deletion cleanup. Historical rewind fixtures were updated
  to remove these objects before simulating older schemas. Full gates passed
  1,117 library, 48 main, seven CLI, and 25 integration tests, formatting, and
  all Clippy profiles. This schema foundation is not installed in the live
  database; atomic checkpoint writers, interruption recovery, and runtime
  dispatch remain pending. Sequential selection still fails closed.
  Startup recovery now terminalizes active sequential invocation markers in
  one transaction rather than requeueing them. Completed checkpoint artifacts
  and Reviewed patches are preserved; cancellation remains sticky. Tests cover
  rollback on the final status write, repeated startup, ordinary workflow
  recovery, and an explicit later rerun unaffected by the older interrupted
  marker. Full gates passed 1,118 library, 48 main, seven CLI, and 25 integration
  tests, formatting, and all Clippy profiles. Atomic checkpoint writers and
  dispatch remain disabled/pending, and this recovery change is not deployed.
  Invocation initialization now snapshots serialized context once, derives its
  SHA-256 identity, and requires active covered review/series ownership. Atomic
  checkpoint writes enforce the expected cursor, next selected patch, exact
  commit/message identity, unchanged instruction/context fingerprints, complete
  per-call accounting, and preserved cancellation. They store only new call
  interactions, artifact text, patch completion, and the cursor in one
  transaction. Tests cover duplicate/out-of-order writes, changed snapshots,
  substituted identities, impossible/missing usage, patch/series cancellation,
  final-cursor-write rollback, and two checkpoints with two calls counted once.
  Full gates passed 1,120 library, 48 main, seven CLI, and 25 integration tests,
  formatting, and all Clippy profiles. Terminal invocation completion and
  daemon/local sequential dispatch remain pending; runtime selection is still
  fail-closed and these storage changes have not been installed in the live DB.
  Terminal invocation completion now requires all selected checkpoints before
  success, retains earlier Reviewed artifacts on later failure/cancellation,
  stores only newly reported call interactions, and keeps raw failed or
  interrupted traces without inventing in-flight usage. Shared accounting
  helpers serve checkpoint and terminal writes. Tests inject final-state failure
  and verify rollback, repeated completion rejection, full-scope success, partial
  failure, external cancellation, and exact known-call totals. Server-only
  helpers are feature-gated; the minimal build and all full gates passed:
  1,121 library, 48 main, seven CLI, 25 integration tests, formatting, and all
  Clippy profiles. Dispatch and partial-result API/UI remain pending, so runtime
  sequential selection is still disabled and no live DB deployment was made.
  The internal SequentialReviewSession now seeds complete context once and
  advances ordered selected patch requests on the shared bounded engine. Fake
  providers prove selector order cannot reorder submission review, the entire
  serialized prompt remains append-only between checkpoints, prior replies
  remain available, wrong-patch output stops without a repair call, cumulative
  token overshoot retains trace/usage, and pre-cancellation makes zero calls.
  Runtime startup remains gated separately from internal budget validation.
  Full gates passed 1,122 library, 48 main, seven CLI, and 25 integration tests,
  formatting, and all Clippy profiles. API/UI and daemon/local dispatch are
  still pending; the runtime continues to reject sequential_patches selection.
  Review details and patchset review lists now batch-load checkpoint sessions,
  ordered notes/drafts, selected and remaining scope, and disjoint interaction
  token totals. Usage completeness and recorded versus attempted call counts
  remain explicit, including cancelled calls without terminal usage. List
  payloads omit full invocation input. Regression fixtures prove list/detail
  agreement across completion, later failure, and cancellation without changing
  ordinary whole-series output. All full gates passed with 1,122 library,
  48 main, seven CLI, 25 integration tests, formatting, and all Clippy profiles.
  UI rendering and daemon/local dispatch remain pending and configuration
  continues to reject runtime sequential selection.
  Retain
   instruction/tool declarations and complete series context as a stable prefix;
   append patch requests, prior replies, and source evidence in order. Bind each
   checkpoint to exact baseline, series revision, and selected patch identity.
   Do not blindly rerun prior completed patches or promote old conclusions to
   operator instructions. Ask before changing architecture beyond this route.
7. Compare quality and total usage on the same small series, then expose a typed
   configurable strategy. Keep phase-by-phase reuse as a later experiment rather
   than adding hidden planning/verification phases to custom-agent execution.

## Cache and Correctness Gates

- Implemented: harness tool definitions and response instructions now live in
  the fixed system prefix, before append-only conversation history. A fake
  provider checks that the complete serialized prompt from one tool turn is an
  exact prefix of the next, with one copy of the tool catalog. Existing workflow
  CLI prompts retain their ordering. This establishes input stability only;
  actual provider cache hits and review quality still require measurement.
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

Deployment: f6213600 installed with four build jobs after deterministic checks.
The stopped database was backed up outside the repository before migration to
schema 14. API health returned 200 and a normal Luna/low workflow completed
patch 2 of the nospec always_inline series. Patch 3 began preparation before
the stop completed, so it was interrupted rather than untouched. A separate
12:00 service start resumed normal workflow; it was not an automatic crash
restart. The later test stop used another completed-patch boundary. Runtime
harness mode has not been enabled and no real harness trial has run.
The stable-prefix follow-up eb6e81ee was also installed after the same full gate
passed. No additional schema change or model request was needed for that install.

Quota observations (CET): 63% remaining at 10:32, 54% at 10:52, 44% at 11:22,
29% at 11:48, and 25% at 11:57; reset reported for 14:00. Coding and daemon
reviews share the account, so these samples do not isolate either consumer's
cost. Pause new review work to retain a debugging reserve, re-read quota before
resuming, and do not assume a reset or promise an unattended resume watcher.
The fresh 12:10 reading was 15% five-hour remaining and 87% weekly. The daemon
was already stopped at a completed-patch boundary; leave it stopped to preserve
the reserve. This pauses the API/UI too, since there is no separate reviewer-only
pause control yet. No watcher remains and no automatic resume has been scheduled.
These reserve-stop instructions were superseded by the user's subsequent
authorization to use the remaining Codex allowance for Sashiko testing. A fresh
12:35 CET account reading showed 12% five-hour and 86% weekly remaining, with
reset at 14:00:02 CET. The daemon was already active with Luna at low effort;
leave its active review intact. Temporary Copilot CLI fallback is authorized
only if Codex reaches zero before reset, subject to verified Luna availability
and billing. This does not authorize Sashiko self-review or a Sol substitution.
The configurable 20%/30% hysteresis remains a future ledger task,
not an implemented automatic feature. Next work is a separately bounded Luna
harness evaluation when authorized and budget permits, then design of sequential
patch checkpoints; neither was claimed complete by these deterministic checks.

Work in the clean detached implementation worktree
/tmp/sashiko-improvements-20261026. The main checkout contains unrelated user
CI and agent edits; never stage or revert them. Fast-forward main and push only
origin/add-xen-support after signed commits and required checks.

The checked implementation supports opt-in harness mode; runtime settings still
select workflow and no real harness trial has run. Preserve mute_all and SMTP
policy. Do not stop an active review for routine checks. Before tests, stop at
a safe boundary, verify workers exited, and resume the service promptly.

Use four Cargo jobs and the shared target directory. Full checks require the
local Git 2.56.0, GIT_TEMPLATE_DIR=/usr/share/git-core/templates, and the shared
CARGO_BIN_EXE_sashiko override for linked-worktree integration tests. Self-review
remains deferred under the explicit spending and final-only restrictions.