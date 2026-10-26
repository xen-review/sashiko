# Review Transparency and Configuration-Sensitive Evidence

Status: operator-reported improvement and false-positive regression case,
2026-10-26. Recording this report does not relabel historical results, authorize
another model review, or replace recall-oriented review with score filtering.

## Reproduction

Submission: [PATCH v2] xen/vcpu: ensure no write when runstate handler is NULL.
Original message ID: 20261023161742.67329-1-roger@xenproject.org.

- Patch page:
  http://127.0.0.1:8080/#/patchset/20261023161742.67329-1-roger%40xenproject.org
- Review map: http://127.0.0.1:8080/#/map/48

The operator reports that a one-line "No issues found" result requires opening
the inline-review disclosure, while the map appears to contain additional issues.
Inspection confirms the patch card says "No regressions" and collapses its inline
review. The map contains two threads: one Low pre-existing compat concern and
one dismissed statement that the native path skips the initial write. They must
not be described as two outstanding verified regressions.

Stored attribution and both pages identify copilot-cli/gpt-6-luna. Historical
thinking effort has not yet been established from invocation evidence. Do not
infer low effort from the model name or copy the current daemon's medium setting
onto this older review. Both pages display zero tokens; verify whether the
provider reported real zeros or usage was unavailable before interpreting them.

## False-Positive Evidence

The operator supplied a GPT-6.1 Sol re-review transcript and the Xen Patch
Reviewer and Improver agent text. Both attachments are readable. This is supplied
external review evidence, not a new model call or independently repeated build.

The reported compat finding relies on the inactive constant-zero definition of
compat_handle_is_null(). In xen/include/xen/compat.h, the CONFIG_COMPAT branch
uses ((hnd).c == 0); xen/arch/x86/x86_64/Makefile builds domain.o only under
CONFIG_COMPAT. The supplied re-review reports actual preprocessing of the handler
as `if ( ((area.addr.h).c == 0) ) break;`, plus focused x86 compilation. If those
conditions match the reviewed tree, the constant-zero claim is a false positive.
Verify the exact submitted revision and surrounding conditional/build rules
before treating a macro definition as reachable evidence.

Independent read-only inspection of Sashiko's saved baseline confirms the
CONFIG_COMPAT definition and the conditional domain.o build rule above. The
original submitted diff does not change either file. This establishes that the
compat claim selected an inactive fallback in the reviewed code. No Xen source
was modified, and the supplied compile/preprocessing checks were not rerun here.

Keep a separate issue separate: the supplied mailing-list review found an agreed
native deregistration adjustment that bypasses guest_handle_okay() for NULL.
That portability hardening and the commit-message corrections do not validate
the compat claim. The transcript reports no all-architecture or guest-runtime
test, and this report does not claim those gates passed or apply its Xen edits.

## Recommended Route

1. Show a meaningful one-line final result without opening a disclosure. Keep
   genuinely long reports collapsible and preserve escaped output and mobile
   layout. "No regressions" must not imply that no pre-existing concerns exist.
2. Explain map dispositions with counts and links: candidate, verified regression,
   verified pre-existing concern, dismissed, and unresolved/unknown. Explain why
   each item is absent from final patch replies, using stored pipeline evidence
   rather than a guessed explanation. Preserve raw artifacts and lineage.
3. Persist and expose provider, exact model, and actual thinking effort for each
   review/stage, distinguishing requested and effective effort if they differ.
   Historical missing effort is "Unknown", never reconstructed from live config.
   Likewise distinguish unavailable token usage from measured zero.
4. Add confidence evidence in the map, separate from severity and pre-existing
   status. Prefer an evidence rationale and optional explicitly model-reported
   score/level with Unknown support. An uncalibrated percentage is not a measured
   probability; do not fabricate scores for historical findings or hide possible
   bugs using a confidence threshold. The operator prefers a false positive to
   a missed finding, while still wanting this avoidable mistake reduced.
5. Strengthen configuration-sensitive review guidance and a deterministic
   regression fixture: inspecting a fallback macro alone cannot prove a reachable
   defect. Establish active #if conditions, architecture, object build selection,
   and relevant caller before reporting. Unknown configuration remains explicit;
   do not suppress a real bug affecting a supported non-default configuration.

These are conservative implementation recommendations, not evidence that the
features already exist. Inspect the current API, final-result selection, map
lineage, and effort propagation locally before choosing a schema change. Do not
start a high-level confidence-calibration or review-policy redesign implicitly.

Incremental attribution decision: use the existing serialized review result for
a provider/model/configured-effort snapshot; no database migration or new store
is needed. A configured CLI effort is not backend-confirmed effective effort.
Keep the latter null unless the provider supplies trustworthy confirmation.
Per-stage request budgets can differ from a CLI's configured effort, so neither
is silently promoted to actual backend attribution. Unknown old records remain
unknown after restart or a settings edit. This first step does not implement
per-stage effective-effort telemetry or confidence calibration.

Incremental confidence contract: verification items may carry confidence with
score in [0, 1] or null, plus a non-empty evidence/uncertainty rationale. Omission
remains valid for old records and uncertain assessments. The application, not
the model, labels provenance as model-reported and uncalibrated. No score changes
severity, reporting eligibility, verification lineage, or scheduling. Do not
inherit another candidate's confidence or synthesize it from model identity.

## Acceptance Checks

- An isolated fixture reproduces this page/map combination and the final
  one-line result is immediately visible on desktop and mobile.
- The map clearly separates its one pre-existing concern from the dismissed
  item and explains omission from the regression-only final response.
- Exact stored/native invocation evidence establishes effort or the UI says
  Unknown. Mixed effort stages retain their own attribution after restart.
- Missing usage is unavailable, not an apparent zero-token completed review.
- Confidence has explicit provenance and uncertainty; severity and recall
  behavior are unchanged. Historical records do not receive invented scores.
- Configuration fixtures distinguish active CONFIG_COMPAT from an unreachable
  fallback and retain a genuine alternative-configuration counterexample.
- Successful results, original false-positive artifacts, human replies, access
  controls, and muted outbound delivery remain intact. No automatic rerun or
  public posting is part of this report.

## Implementation Progress

The first UI slice now renders short single-line inline results directly and
retains disclosures for substantive reports. Cards and maps share a breakdown
of stored regression findings, pre-existing concerns, dismissed items, and
unresolved states. The runstate fixture shows zero, one, one, and zero respectively.
Both map tables scroll within narrow screens instead of widening the page.
No historical disposition or final artifact is changed.

JavaScript syntax, short/long/multiline/escaped content, unknown and legacy map
states, and read-only review-48 rendering checks pass. Browser fixtures at 1,280
and 390 pixels show no page or summary overflow, no injected image element, and
working disclosure handlers. Pointer automation timed out in the shared browser;
direct DOM activation verified the unchanged handlers. Browser screenshots were
captured but not available to the workspace image viewer, so no separate visual
inspection is claimed. The diff-aware make check-pr gate passed.

The shared Xen review prompt now requires surrounding #if/#else and object build
selection evidence, includes this exact CONFIG_COMPAT counterexample, and retains
findings affecting supported non-default configurations. A deterministic prompt
contract check and the full make check-pr gate pass. This checks the supplied
guidance, not a guarantee that future model reviews cannot repeat the mistake.

The first attribution slice stores provider/model/configured effort in existing
workflow result JSON, including concise local output. The daemon stamps its
parent-side snapshot before persistence, replacing a worker's stdio proxy label.
Cards/maps distinguish configured from effective effort; old review 48 displays
Unknown for both. Tests cover snapshot immutability, supported provider routing,
proxy replacement, malformed result shape, explicit none, missing values, and
escaped text. Full make check-pr and desktop/mobile effort-label layout checks
pass. This attribution slice was installed on 2026-10-26 at 17:21 CET with
matching release/installed hashes and a private stopped-database backup.

Optional confidence validation, map projection, and presentation are implemented
and deployed after the context switch at 17:50 CET on 2026-10-26.
Verification accepts absent or null confidence; a supplied assessment requires
a bounded numeric or unknown score and an evidence rationale. The map supplies
model/stage provenance and labels scores uncalibrated. Old or invalid assessments
remain unassessed, and raw candidate scores are not inherited. A fake workflow
test retains a 1% assessment as a final finding with unchanged severity/lineage.
Focused contract, projection, workflow, and escaping checks pass, as does the
full make check-pr gate. The final synthetic-confidence browser check passed at
1,280 and 390 pixels: no page overflow or injected image elements, visible scored
and historical-unassessed cases, and visible escaped rationale. The temporary
fixture is stopped; its synthetic score was never written to the live database.
The four-job release build passed, an idle database backup passed quick_check,
and the release/installed hashes match. Fresh live review-48 checks at both
widths show two Not assessed labels, no synthetic score or overflow, and unchanged
saved findings. The daemon is healthy and ingestion advanced after restart.

The first missing-usage presentation slice shares card/map rendering. Zero-only
legacy aggregates without availability evidence say Unavailable, not measured
zero. A complete checkpoint session with recorded calls can still display real
zeros. Positive counters are labeled reported tokens, missing fields stay
Unknown, and an invalid cached-prefix subtraction does not invent zero. Existing
stored aggregates are unchanged; their lost availability cannot be reconstructed
from zeros alone. This does not establish completeness of positive legacy totals.
Syntax and focused legacy/confirmed-zero/positive/partial/invalid-input checks
pass, along with the diff-aware make check-pr gate and editor diagnostics.
The read-only review-48 fixture passed card/map checks at 1,280 and 390 pixels:
Unavailable is visible, the false zero triple is absent, and there is no page
overflow or injected image. The fixture is stopped and the daemon resumed;
deployment of this presentation slice is pending.

Backend-confirmed per-stage effort, future usage-availability persistence, and
live/final usage completeness remain open.
Review 48's public API and saved prompt do not contain effort; its stored zero
token totals do not establish that its Copilot invocation used no tokens. No
historical effort or confidence was invented, and no new model review was run
for the transparency/confidence changes.