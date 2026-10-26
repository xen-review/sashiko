# Attributable Human-Review Evidence

Status: proposed incremental, view-only architecture for the outstanding human
review filter. Approval is required for the evidence store and positive-match
policy before implementation. Existing Sashiko and staging states are unchanged.

## Existing Inputs

Messages retain Message-ID, author, direct In-Reply-To, body or archive-blob
provenance, and thread references. Patch records name their original message and
stable diff identity. The mail parser supplies decoded metadata and body text.
There is no current typed human-review attribution store. HTTP requests must not
walk Git archives or scan all message bodies to implement a paginated filter.

## Recommended First Scope

Recognize a recorded review statement only in a non-patch direct reply to one
exact saved patch version. Require an unquoted Reviewed-by trailer whose single
mailbox matches the reply's single parsed From mailbox. Use the existing RFC
mail parser for addresses, not substring matching or display names. A generic
reply, a matching subject, or thread membership alone is not review evidence.

Ignore quoted lines, forwarded original-message sections, fenced code, diffs,
signature blocks, malformed/multiple addresses, and unsupported target ambiguity.
Do not transfer a v1 review to v2, or a cover-letter reply to every patch. Do not
interpret Tested-by, Acked-by, Suggested-by, or Reported-by as Reviewed-by.
An author-carried trailer in a submission is a different, weaker claim and is
not a positive match in this first scope. It may be supported separately later.

Mail attribution is not cryptographic authentication. Preserve the claimed
reviewer, From mailbox, source Message-ID, exact target patch/message, timestamp,
review text location, and extraction rules. The UI must say recorded human-review
evidence, not guarantee a human identity or independent verification. A tag does
not prove the review remains applicable after target content changes or that no
later message withdrew it. Preserve the source link and limitations for inspection.

## Storage and Flow

Add a typed, versioned evidence table independently of reviews and patch status.
Retain source and target content identity, extraction version, parsed mailbox,
source/target Message-IDs and database references, and check time. Deduplicate
repeated ingestion of the same evidence atomically. A changed source/target or
unavailable identity makes the record non-current, not a default negative claim.
Keep algorithm/profile identity so changed extraction rules cannot reuse old
results silently. Do not overwrite successful AI reviews or human mail.

Use an opt-in setting, disabled by default. Extract from decoded input during the
existing ingestion path; add no independent network-fetch or review worker.
For historical local data, provide a separately bounded explicit backfill over
saved messages/local archive bodies with a resumable cursor and reported unknown
or unsupported records. Missing bodies never trigger a network request from the
filter or imply absence of human review. Bound records, bytes, and wall time.

Batch detail/list enrichment and use the same positive-evidence SQL predicate
for rows and counts before pagination. Add a separate human-review-evidence
query dimension, leaving ReviewStatus and staging_state independent. Filtered
requests bypass default caches. Positive series selection means at least one
saved part has current evidence; expose part counts and do not claim the entire
series is reviewed. No negative absence-of-human-review filter is justified by
partial ingestion. Keep Messages/Bugs parameters separate and preserve list,
search, page-reset, clear, URL restoration, and current embargo/access redaction.

No scheduling, retry, email, public-posting, model, or effort change is included.
Back up before the database migration and deploy only at an idle review boundary.
Do not perform an automatic historical backfill until its bounds are approved.

## Verification and Decision

Use existing mail-parser, ingestion/database, API, and frontend test homes.
Deterministic fixtures must distinguish genuine direct self-attributed trailers
from quoted examples, code/diffs/signatures, forged display names, malformed
addresses, reply chains, wrong revisions, cover letters, author-carried tags,
source changes, unavailable bodies, duplicates, and withdrawal ambiguity. Verify
bounded/idempotent backfill, restart/cancellation, row/count/pagination agreement,
cache isolation, unchanged permissions, and desktop/mobile provenance rendering.
No new model call, live mail submission, or public post is needed.

Recommended approval: integrated opt-in provenance store and positive-only filter,
initially limited to the direct reviewer-authored replies above. Alternative:
include author-carried Reviewed-by claims as separately labeled lower-assurance
evidence, only if the operator explicitly wants them eligible for the filter.
Design-only is also valid; do not make a claimed identity an authenticated one.