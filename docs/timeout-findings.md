# Timeout Findings

Updated: 2026-10-26.

## Short Answer

The ten-minute limit we found was imposed by Sashiko, not an established hard
limit of Codex. Sashiko wrapped a Codex CLI invocation in a fixed 600-second
timeout. I replaced that constant with configurable model/effort profiles.

Longer timeouts are implemented, tested, committed, and installed. We have not
run an uninterrupted 80-minute model call, so this does not establish that the
remote provider will always permit or successfully complete such a call.

## What Each Deadline Covers

| Deadline | Scope |
| --- | --- |
| Codex call timeout | One Sashiko provider invocation: a single codex exec process, including prompt transfer and output collection. Any internal backend turns share this deadline. |
| Whole-series harness timeout | The entire series invocation, including its multiple provider calls, source tools, and checkpoint transitions. It is not renewed after each call or patch. |
| Independent workflow timeout | One patch's workflow job, rather than one shared whole-series session. Existing rate-limit backoff accounting remains unchanged. |

A native Codex turn, a Sashiko provider call, and a whole-series harness session
are different units. The old 600-second wrapper was not a separate ten-minute
allowance for every internal backend turn.

There were also overall Sashiko review/harness limits. Raising only those would
not have removed the shorter Codex call limit. A matching timeout profile now
configures the call limit and replaces the legacy overall review/harness caps.

## Configuration

For example, an eight-patch MAX review can use:

```toml
[ai]
provider = "codex-cli"
model = "gpt-6-luna"

[ai.codex_cli]
effort = "max"

[[ai.codex_cli.timeout_profiles]]
model = "gpt-6-luna"
effort = "max"
call_timeout_seconds = 4800
minimum_review_timeout_seconds = 3600
review_timeout_seconds_per_patch = 600
```

This is a configuration example, not authorization to start a MAX review.

The overall allowance is the larger of the configured minimum and the number
of selected patches multiplied by seconds per patch. Eight selected patches at
600 seconds receive 4,800 seconds, or 80 minutes. The 4,800-second call allowance
also permits one uninterrupted call to use that time, subject to the overall
deadline and all other budgets. A 3,600-second call allowance would still stop
a single call after an hour even if the whole series had 80 minutes available.

Profiles match exact model and explicitly configured effort. Separate entries
can give Luna, Terra, or other Codex models different allowances. They do not
select a model, escalate effort automatically, or change active requests.
The current implementation applies to the Codex CLI backend, not every backend.

Without a matching profile, the legacy 600-second Codex call timeout and existing
overall limits remain. MAX profiles require at least 3,600 seconds for both the
call and minimum overall timeout. Cancellation and token, tool, invocation,
quota, and authorized operating-window limits remain applicable.

See [configuration.md](configuration.md) for the complete configuration notes.

## Evidence and Remaining Limits

- Tests passed for exact model/effort matching, unchanged defaults, invalid and
  duplicate profiles, overflow rejection, eight-patch scaling, and replacement
  of both legacy overall caps.
- A fake Codex executable that did not read stdin was stopped by its configured
  one-second timeout. Prompt transfer no longer sits outside the call deadline.
- The full gate passed 1,116 library tests, 48 main tests, seven CLI tests,
  25 integration tests, formatting, and all Clippy profiles.
- The implementation was pushed and installed from commit 82905979. Runtime
  configuration contains xhigh/MAX profiles for the verified gpt-6-luna,
  gpt-5.6-luna, and gpt-5.6-terra IDs. Medium remains the selected effort.
- Runtime profile reload/preparation was interrupted before a result was
  recorded. The service is active, but that interrupted command is not claimed
  to have validated or activated the new profiles. No long-running MAX test has
  been performed.
- The malformed-JSON failures do not prove truncation by a timeout or low
  effort. Their native assistant responses were malformed too. One exact
  Luna/medium replay returned valid JSON in 8.829 seconds; that is encouraging
  evidence, not proof that higher effort eliminates the problem.

Provider-side duration limits, network failures, account quota, and process
termination behavior can still affect a long review. Configuring 80 minutes
removes the known shorter Sashiko limits; it does not guarantee completion.