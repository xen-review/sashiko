# View-Only Patchset Review-State Filter

Status: incremental implementation of the operator's requested list filter.
This slice does not implement staging classification, human-review inference,
queue policy, or an additional persistence store.

## Contract

- Place a native state dropdown immediately right of All Lists, before search.
  Default to All States. Hide it for Messages and Bugs without applying it to
  those APIs. Preserve existing list selection, search, and mode controls.
- Use existing typed ReviewStatus values. Reviewed by Sashiko selects the
  recorded current Reviewed series state, not independently verified findings
  or an assertion about human review. Other options select their exact current
  recorded state: Pending, In Review, Incomplete, Failed, Failed To Apply,
  Skipped, and Cancelled.
- Not Reviewed Yet means current series state is not Reviewed, including a
  missing state. It can include partially reviewed, failed, skipped, cancelled,
  or active series. It does not mean that no patch was reviewed earlier, that
  no successful historical result exists, or that no human reviewed the series.
- Filter rows on the server before LIMIT/OFFSET and use the identical predicate
  for total counts. Combine it with existing search and mailing-list predicates.
  Reject unknown filter strings instead of silently returning all records.
- Filtered requests bypass the unfiltered homepage and count caches; they must
  neither consume an unrelated cached result nor pollute the default view.
- Keep existing embargo/access-control redaction unchanged. This filter targets
  recorded review state, so a Reviewed row can still be displayed as Embargoed
  to an unauthorized reader. It does not release findings, logs, or content.
- Changing a filter resets the page to one. Clear resets the new filter with
  existing search/list controls. Preserve/restorable URL state and responsive
  layout using existing conventions; no client-only pagination filtering.

## Scope and Compatibility

Keep existing database list/count callers compatible with unfiltered wrappers.
Add typed optional filtering to their shared query construction and the API
pagination request. Use bound parameters for exact states; no interpolated
user-supplied SQL. No schema migration, historical rewrite, status mutation,
review retry, delivery action, or model call is required.

Human-reviewed and merged-to-staging options are deferred until separate,
attributable per-patch evidence exists. Do not label a generic reply as human
review, or infer staging membership from an apply failure or matching subject.
Those dimensions can overlap review state and are not new ReviewStatus values.

## Verification

Use existing API/database test homes and fake data. Check mixed-state row/count
agreement, missing states, pagination, search/list combinations, exact failed
states, invalid filters, unfiltered cache isolation, and unchanged redaction.
Check frontend URL propagation, page reset, clear, mode changes, and desktop/
mobile control placement/overflow. Keep the daemon running for inspection, stop
only at an idle boundary for executable checks, then resume it promptly.

The first discriminating check is a mixed-state API fixture: prime the default
cache, filter a one-row page of Reviewed data, and require its total to count
only matching rows. A subsequent default request must still return all rows.

## Implementation Evidence

The typed filter and a shared bound-parameter predicate are implemented in source.
Existing unfiltered database callers are unchanged. Database fixtures verify
matching rows/counts, missing states, exact failed states, search, and offset
pages. A real-router regression verifies list/search composition, pagination,
unknown/duplicate filter rejection, and default-cache isolation. The existing
restricted-principal embargo test now checks a filtered anonymous request and
retains Embargoed status with zero finding counts. No access policy was changed.

The native dropdown is immediately after All Lists and before search. It resets
pagination, round-trips spaced states in both supported URL forms, preserves the
list selector on Clear, and is hidden/omitted from Messages requests. Structured
query decoding also retains encoded values and hash precedence. The narrow list
table now scrolls inside its container rather than widening the page.

Focused syntax, URL/page-reset, database, API, and embargo checks pass. A read-only
synthetic browser fixture at 1,280 and 390 pixels verifies control placement,
zero page/control overflow, page-three reset, filtered counts, reload, Messages
isolation, and Clear. No page errors occurred. A screenshot was captured; no
separate workspace-image inspection is claimed. The fixture is stopped and the
daemon resumed after testing; no model call or live record was changed.

The full make check-pr gate passed 1,131 library tests (two ignored), 49 main,
seven CLI, 26 integration, formatting, and all Clippy profiles. Source 44094e87
was deployed at 18:58 CET on 2026-10-26 after a four-job release build, an idle
database backup that passed quick_check, and matching installed/release hashes.
Live API totals match read-only database counts: Reviewed 98, not currently
Reviewed 353, Failed To Apply 191, In Review zero; unknown filters return 400.
The live dropdown at both widths shows 98 Reviewed across two pages without
page/control overflow. The daemon is healthy, schema 15 unchanged, and no active
review was interrupted. Human-review and staging evidence remain independent follow-up work;
this filter does not establish either classification or alter queue scheduling.