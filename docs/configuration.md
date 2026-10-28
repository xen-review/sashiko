# Configuration Reference

Sashiko is configured through two files:

- **Settings.toml** -- application settings (AI, server, git, review)
- **projects/\<project\>/mailing_lists.toml** -- per-mailing-list tracking, embargo, email delivery, and Patchwork policy

Both can be bootstrapped from the examples in [docs/examples/](examples/).
All settings can also be overridden via environment variables using the
`SASHIKO` prefix with `__` (double underscore) as the separator (e.g.
`SASHIKO__AI__PROVIDER=gemini`).

Every command reads one file, the first of `--settings`, `$SASHIKO_CONFIG`,
`Settings.toml` in the working directory, and `~/.config/sashiko.toml` (or
`$XDG_CONFIG_HOME/sashiko.toml`).

The daemon and `sashiko review` read the same shape, so one file serves both.
`[database]`, `[server]`, and `[git]` are optional, since a local review has
none of them; the daemon refuses to start without `[database] url`,
`[git] repository_path`, and `[review] worktree_dir`.

For LLM provider-specific setup (API keys, auth, provider features), see
the [LLM Provider Configuration Guide](llm-providers.md).

## Settings.toml sections

### Top-level keys

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `log_level` | string | `"info"` | Default log filter for the daemon (e.g. `"warn"`, `"info"`, `"debug"`). `RUST_LOG` overrides it when set, `--debug` forces `"info"`, and `sashiko review` defaults to `"warn"` regardless of this value. |

### `[project]`

Optional. Describes the project this configuration is for.

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `kind` | string | -- | Project this file is for: `"linux"`, `"sashiko"`, or `"xen"`. When set, it is checked against the project selected by `--project` / `SASHIKO_PROJECT`, and a mismatch is an error. When absent, the file is accepted for any project. |
| `name` | string | `""` | Display name shown in the web UI. |
| `description` | string | `""` | Short description shown in the web UI. |
| `domain` | string | `""` | Public hostname used to build links to the web UI in forge comments (`https://<domain>/#/patchset/...`). Falls back to `sashiko.sashiko.dev` when empty. |
| `attribution` | string | -- | Actor name recorded on bug discoveries. Defaults to `domain`, or `"sashiko"` when neither is set. |

### `[forge]`

Optional. Controls forge (GitHub/GitLab) webhook integration.

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `enabled` | bool | `false` | Enable forge webhook endpoint. |
| `disable_nntp` | bool | `true` | Disable NNTP ingestion when forge is enabled. |
| `provider` | string | -- | Forge provider: `"github"` or `"gitlab"`. |
| `webhook_secret` | string | -- | Webhook signing token or shared secret for authenticating incoming requests. When configured, requests are authenticated via signature verification, which is the only way a forge can authenticate. See the [Webhook Security Guide](WEBHOOK_SECURITY.md). |
| `api_token` | string | -- | Forge API token (for future API-based features). |

> **Security:** When `Settings.toml` contains secrets, restrict file
> permissions: `chmod 600 Settings.toml`.

### `[database]`

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `url` | string | `"sashiko.db"` | Path to the SQLite database file. |
| `token` | string | `""` | Database token (unused for SQLite). |

### `[mailing_lists]`

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `track` | string or list | `[]` | Optional override for mailing lists to monitor via NNTP. When empty (default), tracked lists are loaded from `projects/<project>/mailing_lists.toml` (all entries with `track = true`). Accepts a TOML array or a comma-separated string. |

### `[nntp]`

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `server` | string | `"nntp.lore.kernel.org"` | NNTP server hostname. |
| `port` | integer | `119` | NNTP server port. |
| `tls` | bool | `false` | Wrap the session in TLS (implicit NNTPS). |

Setting `tls` does not change `port`; set it to `563` as well when
enabling NNTPS. The certificate is verified against the host trust
store, so an internal CA must be installed there. `lore.kernel.org`
offers no TLS-protected NNTP, so this is for internal mirrors.

### `[smtp]`

Optional. If omitted, Sashiko sends no email: no review emails, sign-in
links or bug assignment notices. Even when present, `dry_run` defaults to
`true` as a safety measure.

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `transport` | string | `smtp` | Delivery method: `smtp` or `sendmail`. |
| `server` | string | -- | SMTP server hostname. Required for `smtp`. |
| `port` | integer | -- | SMTP server port. Required for `smtp`. |
| `username` | string | -- | SMTP username (optional). |
| `password` | string | -- | SMTP password (optional). |
| `sendmail_path` | string | `/usr/sbin/sendmail` | Sendmail binary to invoke. |
| `sender_address` | string | -- | From address for review emails. |
| `reply_to` | string | -- | Reply-To address for review emails (optional). Email addressed to a person, such as a sign-in link, carries none. |
| `dry_run` | bool | `true` | When true, emails are logged but not sent. |

The `smtp` transport connects over implicit TLS, so it cannot reach a
submission service that offers only cleartext. The `sendmail`
transport pipes the message to the local MTA instead, which leaves
transport security and queueing to the host's mail configuration. It
ignores `server` and `port`. Supplying credentials alongside it is
rejected when the configuration loads. A `sendmail_path` that is not
an executable file is rejected at startup, unless `dry_run` is on,
since a dry run never invokes it.

A message the local MTA accepts is queued, not delivered. Sashiko
records it as sent once sendmail exits zero, so a later rejection
arrives as a bounce that only the host mail configuration sees. A
sendmail that has not exited after 60 seconds is killed and the
message marked failed.

### `[ai]`

Core AI settings that apply to all providers.

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `provider` | string | -- | LLM provider: `gemini`, `claude`, `claude-cli`, `codex-cli`, `copilot-cli`, `bedrock`, `vertex`, `kiro-cli`, `goose`, `openai`, `openai-compatible`. |
| `model` | string | -- | Model identifier (provider-specific). |
| `max_input_tokens` | integer | `150000` | Maximum input tokens per request. |
| `max_interactions` | integer | `100` | Maximum tool-call rounds per review turn. |
| `temperature` | float | `1.0` | Sampling temperature. |
| `api_timeout_secs` | integer | `300` | Timeout for individual API calls (seconds). |
| `log_turns` | bool | `false` | Log each AI request/response turn at info level. Verbose but useful for debugging. |
| `response_cache` | bool | `false` | Cache AI responses to disk. The daemon keeps the cache beside its database; a local review, which has none, keeps it under `$XDG_DATA_HOME/sashiko/`. Entries are keyed on the provider's own settings as well as the request, so changing `model`, an endpoint, a reasoning level, or an output cap misses the entries recorded under the old value rather than replaying them. |
| `response_cache_ttl_days` | integer | `7` | TTL for cached responses (days). Entries stranded by a settings change age out on this schedule. |

#### `[ai.claude]`

Settings specific to the Claude API provider (`provider = "claude"`).

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `prompt_caching` | bool | `true` | Enable Anthropic prompt caching (5-minute TTL). |
| `max_tokens` | integer | `32768` | Max output tokens per response, thinking included. |
| `base_url` | string | -- | Override the API base URL (optional, for proxies like Portkey). |
| `thinking` | string | -- | Thinking mode, sent as `thinking.type`: `"adaptive"` (Opus 4.6, Sonnet 4.6, and later), or `"disabled"` where the model allows it. |
| `effort` | string | -- | Effort level, sent as `output_config.effort`: `"low"`, `"medium"`, `"high"`, `"xhigh"`, `"max"`. Which levels a model accepts, and its default, vary by model. |

#### `[ai.claude_cli]`

Settings for the Claude Code CLI provider (`provider = "claude-cli"`).

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `effort` | string | -- | Thinking effort: `"low"`, `"medium"`, `"high"`, `"xhigh"`, `"max"`. |

#### `[ai.codex_cli]`

Settings for the Codex CLI provider (`provider = "codex-cli"`).

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `effort` | string | -- | Reasoning effort: `"none"`, `"minimal"`, `"low"`, `"medium"`, `"high"`, `"xhigh"`, `"max"`. Passed as `-c model_reasoning_effort=<effort>`, which outranks `~/.codex/config.toml` but not an enterprise-managed requirements layer. A run whose effort that layer substitutes fails. |

Optional exact model/effort timeout profiles override the Codex call deadline
and the local/daemon review deadline. Without a match, the call deadline remains
600 seconds and existing review/harness limits remain unchanged. Profiles do not
select a model or change effort, and unset effort never guesses a model default.

```toml
[[ai.codex_cli.timeout_profiles]]
model = "gpt-6-luna"
effort = "xhigh"
call_timeout_seconds = 4800
minimum_review_timeout_seconds = 3600
review_timeout_seconds_per_patch = 600

[[ai.codex_cli.timeout_profiles]]
model = "gpt-6-luna"
effort = "max"
call_timeout_seconds = 4800
minimum_review_timeout_seconds = 3600
review_timeout_seconds_per_patch = 600
```

The overall allowance is the larger of the minimum and selected patch count
times seconds per patch. Eight selected patches therefore receive 4,800 seconds
(80 minutes), shared across the full-series harness, not renewed at tool turns
or checkpoints. Independent workflow jobs use the one-patch allowance. A matched
profile replaces both legacy review and harness caps; cancellation and token,
tool, and provider-call limits remain in force. The call deadline includes prompt
transfer and output collection. Configure each model/effort pair separately;
these Codex-specific profiles do not affect other providers.

All profile limits must be positive and representable; duplicate keys, unknown
effort names, and scaled-duration overflow fail. MAX profiles require at least
3,600 seconds for both the call and minimum overall deadline. Longer timeouts do
not extend the authorized operating window or enable higher-effort calls.

#### `[ai.codex_cli.account_quota]`

This optional table is reserved for a future account-quota controller. Its
settings are validated but have no runtime effect. `enabled = true` is rejected
until account binding, durable state, and scheduler integration are complete.

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `enabled` | bool | `false` | Must remain false. |
| `pause_below_percent` | float | `20.0` | Reserved five-hour pause threshold. |
| `resume_above_percent` | float | `30.0` | Reserved five-hour resume threshold. |
| `max_sample_age_seconds` | integer | `300` | Reserved maximum observation age. |
| `unavailable_retry_seconds` | integer | `60` | Reserved retry delay for unavailable observations. |

Thresholds must be finite percentages in `0..=100`, with pause strictly below
resume. Durations must be between 1 and 3,600 seconds, inclusive. These values
do not authorize provider calls or affect existing review scheduling.

#### `[ai.gemini]`

Settings for the Gemini provider (`provider = "gemini"`).

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `explicit_prompt_caching` | bool | `false` | Use explicit caching hints in requests. |
| `max_tokens` | integer | model-derived | Max output tokens per response, thinking included (`generationConfig.maxOutputTokens`). Defaults to `131072` on models with Long Decodes v2 (`CONTINUATION` support, such as Argon) and is omitted otherwise (or when set to `0`) so the backend uses the model default. |
| `thinking_level` | string | -- | Default thinking level sent as `thinkingConfig.thinkingLevel` (`"LOW"`, `"MEDIUM"`, `"HIGH"`) when a stage does not specify a per-request reasoning effort. |

#### `[ai.openai_compat]`

Settings for the OpenAI providers (`provider = "openai"` or `provider = "openai-compatible"`).

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `base_url` | string | model-derived | API endpoint URL. Derived from the model name, `https://api.openai.com/v1/chat/completions` for anything unrecognized. |
| `context_window_size` | integer | model-derived | Context window size. `128000` for most models. |
| `max_tokens` | integer | `4096` | Max output tokens per response. With `provider = "openai"` it is sent as `max_completion_tokens`, which bounds reasoning tokens as well as the reply. |
| `effort` | string | -- | Reasoning effort, sent as the `reasoning_effort` request field and omitted entirely when unset. The valid set belongs to the model rather than to the API: `"none"` is legal, and is what a gpt-5.6 chat completion carrying function tools requires, while gpt-5.4-pro takes only `"medium"` and `"high"`. A level the model rejects comes back as a 400, which is fatal. Applies to both `provider = "openai"` and `provider = "openai-compatible"`; a compatible endpoint that does not implement the field may reject the request, so leave it unset there. |

#### `[ai.kiro_cli]`

Settings for the Kiro CLI provider (`provider = "kiro-cli"`).

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `binary` | string | `"kiro-cli"` | Path to the kiro-cli binary. |
| `agent` | string | -- | Custom agent name (optional). |
| `context_window_size` | integer | `200000` | Context window size. |

#### `[ai.goose_cli]`

Settings for the goose provider (`provider = "goose"`).

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `binary` | string | `"goose"` | Path to the goose binary. |
| `goose_provider` | string | `"openai"` | Backend goose itself talks to, passed as GOOSE_PROVIDER. |
| `env` | table | `{}` | Environment for the goose child process, e.g. `OPENAI_HOST`. goose inherits Sashiko's environment and these entries win over it, but they cannot override the variables Sashiko pins to keep goose a completion backend (`GOOSE_MODE`, `GOOSE_MODEL`, `GOOSE_PROVIDER`, `GOOSE_CONTEXT_LIMIT`, `XDG_CONFIG_HOME`, `XDG_DATA_HOME`, `XDG_STATE_HOME`). |
| `context_window_size` | integer | `128000` | Context window size. goose adds roughly 5k tokens of its own prompt, so keep `max_input_tokens` well below this. |

### `[server]`

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `host` | string | `"::"` | Listen address. `"::"` binds to all interfaces (IPv4 and IPv6). |
| `port` | integer | `8080` | Listen port for the web UI and API. |
| `read_only` | bool | `false` | When true, disables write API endpoints. Set automatically by `--no-api`. |
| `public_base_url` | string | -- | The URL the service is reachable at from outside, with no trailing slash. Required whenever `[smtp]` is configured: sign-in links and bug links are mailed, and the bind address names no host a recipient can open. The server refuses to start without it. |
| `jwt_secret` | string | -- | Signs sign-in links and session tokens. Without it, sign-in returns `501` and no identity can be established. Keep it stable: replacing it invalidates every session and every unopened link. Prefer `SASHIKO__SERVER__JWT_SECRET` over writing it to disk. |
| `log_sign_in_links` | bool | `false` | Writes sign-in links to the log. For a developer machine with no real users; a link in a log is a credential anyone reading the log can spend. |
| `human_review_evidence` | bool | `false` | Records bounded, directly replied Reviewed-by evidence during normal raw-mail ingestion. The single trailer mailbox must match the actual single From mailbox. Exact target and source revisions must remain unchanged. This is recorded mail attribution, not authenticated identity or proof that no later reply withdrew the review. No historical scan, network fetch, scheduling change, or outbound delivery is enabled. |

When enabled, `GET /api/patchsets?human_review=recorded` selects series with
current evidence on at least one saved part. It composes with `review_state`,
`staging_state`, list/search, and pagination. There is no negative human-review
filter: unavailable or unprocessed replies cannot prove absence. Disabled
configurations reject this query. List responses expose distinct recorded-part
counts; both detail routes include exact provenance for their requested page,
bounded to 100 patch IDs and 200 records with an explicit truncation flag.
The web state selector labels this as Recorded Human-Review Evidence, separately
from Sashiko's AI status and staging history.

### `[server.acl]`

Capability lists, matched against the address a caller signed in with. Every
list is empty by default, which grants nothing (fail-closed). The local
operator token covers `ingest`, `cancel` and `review` for tooling that can read
the server's token file, so these lists are only about remote, identified
callers.

| Key | Type | Description |
|-----|------|-------------|
| `admins` | list | All capabilities, including root-level maintenance operations. |
| `security` | list | Reads and comments on every bug and reads raw AI transcripts. Grants no ingest, cancel or review. |
| `bug_reporters` | list | May file new bugs over HTTP. Empty means only admins can. |
| `ingest` | list | May submit patches (`/api/submit`). |
| `cancel` | list | May halt running workloads. |
| `review` | list | May trigger AI analyses of existing patches. |
| `blocklist` | list | Denies everything, overriding every grant above. |

Each list accepts either a TOML array or a comma separated string, so a
deployment can name its operators through the environment instead of baking
them into the image:

```bash
SASHIKO__SERVER__ACL__ADMINS="first@example.org,second@example.org"
```

### `[git]`

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `repository_path` | string | -- | Path to the kernel git repository used for patch application and context. |

#### `[git.staging]`

The optional Xen-only `[git.staging]` section is an opt-in for read-only staging
membership evidence. Bounded background publication, per-patch provenance,
full-series aggregation and view-only API/UI filters are implemented. Back up the
database before enabling schema-16 deployments. Omission preserves existing
behavior and does not guess a target from local HEAD or origin. Staging history
matches prove diff equivalence at a checked tip, not that a change remains
effective. Missing, stale or unsupported evidence remains Unknown; an unmatched
bounded scan is only No match in checked range, never an Unmerged assertion.

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `remote` | string | required | Explicit configured authoritative remote, such as `upstream`. |
| `target_ref` | string | required | Full `refs/remotes/<remote>/<branch>` name. HEAD, revision expressions, and mismatched remotes are rejected. |
| `freshness_seconds` | integer | `86400` | Maximum age of successful refresh evidence; positive. Commit dates do not establish freshness. |
| `max_commits` | integer | `1000` | Maximum reachable commits per indexing pass; `1..=10000`. An incomplete range cannot establish absence. |
| `max_patches` | integer | `1000` | Maximum most-recent mailed patches checked per publication; `1..=10000`. Older unchecked patches remain Unknown. |
| `max_diff_bytes` | integer | `16777216` | Maximum new diff bytes per pass; `1..=67108864`. |
| `timeout_seconds` | integer | `120` | Whole-pass deadline, including Git reads and patch-ID calculation; `1..=600`. |

This does not change review scheduling or outbound delivery. A matching commit
in history does not establish that the change remains effective after reverts,
that it entered master, or that a human reviewed it. See
[the approved staging-evidence design](../designs/DESIGN_XEN_STAGING_EVIDENCE.md).

#### `[[git.custom_remotes]]`

Optional array of additional git remotes to track.

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `name` | string | -- | Remote name. Required. |
| `url` | string | -- | Remote URL. Required. |
| `check_all_branches` | bool | -- | Try all branches as baselines. Required -- omitting it is a parse error, not a default of `false`. |
| `only_branches` | list | -- | Additional specific branches to try (optional). Additive: when `check_all_branches` is also true, these are appended to the full branch list rather than replacing it. |

Baselines are tried in order and the first one the series applies to wins.
A tree taken from a MAINTAINERS `T:` entry without a branch resolves to the
remote's HEAD. When that HEAD is a strict ancestor of the local mainline ref,
it is tried after every other candidate instead of in its usual place: it has
no commit that mainline lacks, and on some trees HEAD points to a commit dated
years ago.
Custom remotes are tried after any `base-commit:` trailer and the MAINTAINERS
subsystem heuristic, but **before** linux-next and mainline. A subsystem topic
branch is where a series was actually developed, whereas linux-next carries a
snapshot of that branch which lags by at least one daily build -- and shares no
SHAs with it once the maintainer rebases.

Use this to reach topic branches that the MAINTAINERS `T:` entry doesn't name.
For example, the NFSD entry lists a tree but no branch, so the only candidate it
yields is `cel/HEAD` (a symref to `cel/master`):

```toml
[[git.custom_remotes]]
name = "cel"
url = "git://git.kernel.org/pub/scm/linux/kernel/git/cel/linux.git"
check_all_branches = false
only_branches = ["nfsd-next", "nfsd-testing"]
```

Match `name` and `url` to a remote already in the repository when one exists.
`ensure_remote` rewrites the URL whenever the configured one differs, so an
`https://` URL here against a `git://` remote flips it back and forth on every
pass.

### `[review]`

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `concurrency` | integer | -- | Number of concurrent reviews. |
| `worktree_dir` | string | -- | Directory for git worktrees used during daemon reviews. The daemon empties it on startup, so it must hold nothing else. |
| `timeout_seconds` | integer | `3600` | Review timeout in seconds; see daemon workflow details below. |
| `max_retries` | integer | `3` | Retry count on transient failures. |
| `max_lines_changed` | integer | `10000` | Skip patches with more changed lines than this. |
| `max_files_touched` | integer | `200` | Skip patches touching more files than this. |
| `ignore_files` | list | `[]` | File patterns to skip during review (e.g. `MAINTAINERS`). |
| `email_policy_path` | string | `"projects/linux/mailing_lists.toml"` | Path to the per-project mailing list and email delivery policy file (`projects/<project>/mailing_lists.toml`). |
| `max_total_tokens` | integer | `10000000` | Maximum cumulative uncached tokens (input + output, including thinking tokens) per review. Cached tokens are excluded. Set to 0 to disable. |
| `max_total_output_tokens` | integer | `1000000` | Maximum cumulative output tokens (including thinking tokens) per review. Set to 0 to disable. |
| `effort` | string | `"mid"` (Argon 4) | Reasoning effort mode for models that support configurable effort levels (currently Argon 4; ignored for other models). Options: `"low"` (uniform `Low` effort across all stages), `"mid"` (uniform `Medium` effort across all stages, default), `"high"` (`High` effort across screening, planning, analysis, and verification stages; `Medium` for report formatting), or `"dynamic"` (`Low` for screening/planning/summary, planned `Medium` or `High` per analysis stage, `High` for verification stages, `Medium` for report formatting). Can also be set via `--effort`. |

For daemon workflow jobs, the resolved review timeout limits active worker time;
shared quota backoff is excluded. A separate wall-clock limit of twice that
timeout stops the worker and prevents new provider calls even during backoff.
Both limits are rechecked after a request acquires a concurrency permit. Process
cleanup can take additional time. A matching Codex timeout profile sets the
active timeout used for both limits. Local workflow calls also recheck their
retry budget after acquiring a permit when a timeout is configured. Local and
agent-harness deadline policies are otherwise unchanged.

For daemon workflow reviews, either enabled token budget requires provider
responses to report usage. If usage is unavailable, or the worker exits while
calls remain unaccounted, the review fails without an automatic retry or another
provider call. Calls already in progress may finish within the existing review
deadline; available responses and worker history are retained in the failed
review's diagnostic log. Nonterminal provider errors without usage, including
rate-limit errors, also stop a budgeted review; the first redacted provider
error and any observed failed-call usage are kept in its diagnostic log. These
counters do not establish complete usage for the failed call. A reported rate
limit still starts shared backoff for other reviews. Cancellation and the
deadline can interrupt in-progress calls. Setting both limits to zero allows
workflow reviews to continue without usage and retry transient and rate-limit
errors, but disables both token budgets. A terminal workflow error carrying
reported usage can retain those observations in diagnostic history. Locally
reported failures from each workflow attempt are kept even when a later attempt
succeeds or provider setup fails. Earlier failed attempts' measured successful
responses are listed separately from the successful or final-attempt totals;
internal retries and other parallel calls may still be missing. These budgets
are checked after responses arrive and can be overshot by calls already in
progress; they are not hard per-generation limits. After session retry exhaustion,
reported counters on the terminal provider error can be retained; earlier internal
attempts are not recovered by that observation.

#### Agent Harness

`[review] mode = "workflow"` remains the default. Local and daemon reviews can
instead use `mode = "agent_harness"` with `provider = "codex-cli"`. This mode
replaces the entire planning/analysis/verification/report workflow with one
bounded whole-series session using operator-supplied instructions and read-only
Git tools. It may make multiple provider calls. Unsupported providers, missing
instructions, and conflicting workflow stages fail rather than falling back.

Example local configuration, using subscription-backed Codex authentication:

```toml
[ai]
provider = "codex-cli"
model = "gpt-6-luna"

[ai.codex_cli]
effort = "low"

[review]
concurrency = 1
mode = "agent_harness"

[review.agent_harness]
strategy = "whole_series"
instruction_files = [
  "agents/friendly-series-review-with-suggestions.agent.md",
  "skills/friendly-series-review-with-suggestions/SKILL.md",
  "skills/xen-coding-style/SKILL.md",
  "../xen/CODING_STYLE",
]
max_generations = 12
max_tool_calls = 30
max_input_tokens = 400000
max_output_tokens = 20000
timeout_seconds = 3600
```

These sample paths assume the settings file is in the Sashiko checkout and a
sibling Xen checkout supplies CODING_STYLE. Adjust paths for your installation;
the daemon's private settings file normally needs absolute in-tree paths. The
friendly-series-review-with-suggestions skill governs code checking and reply
content. Its agent selects that procedure, while the style reference governs
Xen formatting and Sashiko's artifact contract governs transport/storage.

All limits must be positive. The harness timeout defaults to 3600 seconds; the
instruction manifest and the generation, tool, input token, and output token
limits are required. The strategy defaults to `whole_series` when omitted. Set
`strategy = "sequential_patches"` to review selected patches in submission order
in one shared conversation. Each checkpoint returns notes and exactly one
validated reply for its original patch identity. Full series context and prior
investigation remain available, but source tools bind to that patch's exact
applied commit. Limits and the original deadline apply cumulatively to the
entire invocation, not per patch. Unknown strategies fail rather than falling
back. The instruction manifest is ordered; list all required agents, skills,
style guides, and reference examples explicitly. Relative paths resolve against
the settings file directory, not the repository or CLI working directory.
Sashiko snapshots original contents and hashes them at configuration load;
frontmatter does not enable tools. Missing files fail configuration loading.
Instructions are capped at 1 MiB total and serialized series input at 8 MiB;
oversized inputs are rejected, not truncated.

Run `sashiko --settings local.toml --project xen review BASE..HEAD --format json`
for the full result, including instructions, raw conversation, usage, and exact
baseline/patch attribution. `--agent` returns concise artifacts labeled
`completed_without_independent_verification`, never a verified-clean result.
Reviewer notes and per-patch plain-text reply drafts remain separate. Drafts are
not sent and proposed source fixes are not applied. `--no-ai` still makes zero
provider calls. Custom prompts, selected workflow stages, and pre-existing bug
reports are rejected rather than silently ignored in harness execution.
A failed Codex turn may include observed usage in the full harness trace. These
counters stay separate from successful-call totals and do not establish complete
usage for the failed call.

For a daemon, add the usual database, repository, worktree, and delivery-policy
settings. The complete series is context, with replies limited to patches that
pass the existing subject and size filters. One series review record stores
covered patch commits and one usage total; results and instruction snapshots
are available through review details. The UI separates reviewer notes and reply
drafts and explicitly labels them not independently verified. No email, forge
comment, or deferred embargo notification is generated from harness output.
Existing delivery policy remains in force for other workflow results.

The model returns JSON artifacts, not a chat-only report: reviewer_notes plus
reply_drafts carrying each exact patch index, message ID and plain-text body.
Whole-series returns all selected replies together. Sequential mode returns
one patch's notes and reply per checkpoint. Neither strategy writes .review;
the daemon owns database persistence and UI presentation, while local CLI
output is returned to its caller. The samples retain .review file conventions
only for direct agents authorized to write review artifacts. Sashiko currently
has no automatic .review import/export or previous-database-review input. Prior
checkpoints in the same conversation are available; unrelated external reviews
must be supplied explicitly and are not assumed to have been consulted.

Sequential daemon invocations atomically store each checkpoint before the next
patch starts. Earlier completed patches and replies remain available if a later
checkpoint fails or is cancelled. Review details and the UI expose completed and
remaining scope, disjoint recorded-call totals, and incomplete usage when a call
has not reported its terminal accounting. Interrupted daemon invocations retain
completed results without automatically replaying them after restart. A local
CLI run includes partial artifacts in its final JSON, agent, or human output,
including graceful Ctrl-C cancellation; local output is not durable across
SIGKILL or host loss. Neither strategy is independently verified or guaranteed
to improve quality, context-cache use, or cost.

An explicit rerun queues one new series invocation and retains earlier artifacts;
the normal per-patch target count does not trigger multiple harness invocations.
Cancellation stops session continuation and retains completed responses and known
usage. Tokens from an interrupted in-flight provider call may not be reported;
persisted usage covers completed responses, not an invented terminal total.
Daemon `--no-ai` is not supported with harness mode; local `--no-ai` remains a
zero-call patch check. An empty eligible scope is an error, not a clean review.

`max_generations` bounds Sashiko provider calls, not hidden backend generations.
`max_tool_calls` bounds local read-only source-tool executions; their results may
require later provider calls and add to input token use. Native CLI tools remain
disabled. Token thresholds count all reported input (including cached tokens)
and output between calls, allowing a single call to overshoot; that overshoot
fails the review and is recorded. Missing usage also fails the review. No repair
call or whole-session retry runs automatically. Without a matching Codex timeout
profile, the smaller positive review or harness timeout applies and the per-call
timeout is 600 seconds. A matching profile overrides both deadlines as described
above. MAX effort requires separate timeout support and is not authorized by
this example. Completed-response caching is disabled for this mode; provider-side
prefix caching may still apply but is not guaranteed.
Harness tool instructions precede conversation history so completed tool turns
append to a stable serialized prompt prefix rather than moving the tool catalog.
This improves cache eligibility, not a guarantee of provider cache hits or cost.

### `[linux_bug]`

Controls pre-existing Linux kernel bug tracking and background analysis.
By default (`enabled = false`), pre-existing issues discovered during patch
reviews are ignored and the background bug worker is not started.

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `enabled` | bool | `false` | Enable pre-existing bug tracking, the `/api/bug/analyze` endpoint, and the background `BugWorker`. |
| `concurrency` | integer | `4` | Maximum number of concurrent bug analyses run by the background `BugWorker`. |
| `fix_check_enabled` | bool | `false` | Enable periodic verification of whether open bugs have been fixed in the upstream mainline branch (`master` for Linux, `main` for Sashiko). |
| `lease_ttl_seconds` | integer | `300` | How long a worker's claim on a bug stays valid without renewal. If the worker dies, the bug becomes claimable again once this elapses. |
| `max_attempts` | integer | `3` | How many analysis attempts a bug gets before it is abandoned. Abandoned bugs are never retried automatically. |
| `fix_check_interval_seconds` | integer | `21600` | Interval in seconds between periodic upstream fix checks against the mainline tree. Set to `0` to disable periodic checks. |
| `fix_check_batch_size` | integer | `50` | Maximum number of open bugs evaluated per upstream fix check cycle. |

### `[subsystems]`

Controls how patches and emails are categorized into subsystems for targeted reviews and specific email policies. By default, this section is empty, meaning the system relies on fallback heuristics (like identifying `@vger.kernel.org` addresses) to determine subsystems.

This feature is **globally active** and applies to both mailing list (NNTP) and Forge (Webhook) ingestion.

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `mapping` | list of objects | `[]` | A list of rules mapping a regular expression pattern to a subsystem name. |

Each mapping object in the list requires two fields:
* `pattern` (string): A regular expression used for matching.
* `name` (string): The resulting subsystem name if the pattern matches.

**How it works:**
- **For Git Forges (Webhooks):** The system applies the `pattern` against the **file paths** modified by a pull request (e.g., matching `^drivers/net/.*`).
- **For Mailing Lists (NNTP):** The system applies the `pattern` against the **To and Cc email addresses** of the incoming patch email.

When a patch is tagged with a subsystem, it can trigger subsystem-specific AI review rules (context loading) and specific email/embargo policies defined in `projects/<project>/mailing_lists.toml`.

```toml
[subsystems]
mapping = [
    { pattern = ".*drivers/.*", name = "Drivers" },
    { pattern = ".*net/.*", name = "Networking" },
    { pattern = ".*fs/.*", name = "Filesystems" },
    { pattern = ".*mm/.*", name = "Memory Management" },
]
```

## `projects/<project>/mailing_lists.toml`

Controls which mailing lists Sashiko tracks via NNTP, embargo delays, how Sashiko routes or suppresses review emails, and Patchwork check delivery. See [projects/linux/mailing_lists.toml](../projects/linux/mailing_lists.toml) and [docs/examples/email_policy.toml](examples/email_policy.toml).

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `defaults.track` | bool | `true` | Whether listed mailing lists are tracked via NNTP by default. |
| `defaults.embargo_hours` | integer | `0` | Hours to wait before publishing a review with findings. Clean reviews are released immediately after the complete patchset review succeeds. |
| `defaults.reply_to` | list | `[]` | Recipient targets when findings are present: `"author"`, `"list"`, `"recipients"`. |
| `defaults.positive_review` | string | `"none"` | Who receives a review email when 0 issues are found: `"none"`, `"author"`, or `"all"`. |
| `defaults.mute_all` | bool | `false` | Suppress all email sending for this scope. |
| `defaults.cc` | list | `[]` | Static CC addresses always included on review emails. |
| `defaults.ignored_emails` | list | `[]` | Author addresses to mute entirely. |

Each mailing list is configured as a `["<list-email>"]` table (e.g. `["bpf@vger.kernel.org"]`). Omitted fields inherit from `[defaults]`. Each mailing list section accepts:

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `track` | bool | `defaults.track` | Whether Sashiko ingests patches from this mailing list via NNTP. |
| `nntp_group` | string | derived from email | Explicit Lore NNTP group override when the group name cannot be derived by reversing the email domain and appending the list's local part. |
| `embargo_hours` | integer | `defaults.embargo_hours` | Hours to wait before publishing a review with findings. |
| `subject_prefixes` | list | `[]` | Subject prefix tags (e.g. `["net", "net-next"]`) used to disambiguate `embargo_hours` on cross-posted series. |
| `reply_to` | list | `defaults.reply_to` | Recipient targets for this list (`"author"`, `"list"`, `"recipients"`). |
| `positive_review` | string | `defaults.positive_review` | Positive review delivery mode (`"none"`, `"author"`, `"all"`). |
| `mute_all` | bool | `defaults.mute_all` | Suppress all email sending for patches matching this list. |
| `cc` | list | `[]` | Additional static CC addresses for this list (merged with `defaults.cc`). |
| `ignored_emails` | list | `[]` | Additional author addresses to mute for this list. |
| `patchwork.enabled` | bool | `false` | Enable Patchwork integration for this mailing list. |
| `patchwork.api_url` | string | -- | Patchwork REST API URL (e.g. `https://patchwork.kernel.org/api/1.3`). Trailing slashes are stripped automatically. Invalid schemes are rejected with a warning. |
| `patchwork.token` | string | -- | Patchwork API token. Can also be set via `SASHIKO_PATCHWORK_TOKEN` env var (fills in where token is omitted in TOML). |
| `patchwork.email` | string | -- | Email address for email-based Patchwork notifications. |
| `patchwork.min_severity` | string | -- | Minimum finding severity to include in patchwork checks. Findings below this threshold are excluded. Accepts: `Low`, `Medium`, `High`, `Critical` (case-insensitive). Default: all findings included. |
| `patchwork.fail_severity` | string | `High` | Minimum severity of NEW findings that triggers the `fail` check state instead of `warning`. New findings at or above this threshold produce `fail`; below it produce `warning`. Pre-existing findings never affect the check state. |

### Author-only delivery

A mailing list can be **tracked** (its patches are ingested and reviewed) while **not pinging** that list with the review email — sending only to the patch author instead:

```toml
["intel-xe@lists.freedesktop.org"]
reply_to = ["author"]
```

### Patchwork integration

Sashiko can report review results as
[checks](https://patchwork.readthedocs.io/en/latest/usage/overview/#checks)
on a Patchwork instance. Two delivery modes are available and can be
enabled simultaneously for the same mailing list.

**API mode** posts checks directly to the Patchwork REST API with
retry-queuing (3 attempts, exponential backoff). Requires a maintainer
API token. Note: Patchwork tokens grant full project-maintainer
permissions (state changes, delegation, etc.), not just check access.

```toml
["netdev@vger.kernel.org".patchwork]
enabled = true
api_url = "https://patchwork.kernel.org/api/1.3"
token = "your-api-token"   # or set SASHIKO_PATCHWORK_TOKEN env var
```

**Email mode** sends a structured notification email to a bot address.
A local script (such as
[pw_tools](https://github.com/mchehab/pw_tools)) parses the email and
posts the check. This avoids giving Sashiko a write token.

```toml
["linux-media@vger.kernel.org".patchwork]
enabled = true
email = "pw-bot@lists.example.org"
```

#### Severity filtering and check state mapping

By default, all findings are included in the patchwork check count.
Set `min_severity` to exclude findings below a threshold. When all
findings fall below the threshold, the check is posted as `success`.

The check state depends only on **new** findings (not pre-existing):

- `fail` -- new findings at or above `fail_severity` (default: `High`)
- `warning` -- new findings below `fail_severity`
- `success` -- no new findings (pre-existing findings are still
  shown in the description but do not affect the state)

The check description shows a per-severity breakdown with
pre-existing counts in parentheses, dropping zero-count severities.
For example: `Critical: 1 · High: 2 (1 pre-existing)`.

```toml
["netdev@vger.kernel.org".patchwork]
enabled = true
api_url = "https://patchwork.kernel.org/api/1.3"
min_severity = "Medium"    # exclude Low findings entirely
fail_severity = "High"     # High+ new findings = fail (default)
```

Edge case behaviors:

- Missing or null `preexisting` flag on a finding is treated as new
- When `min_severity` filters out all findings, the check is `success`
  with "Sashiko AI review found no regressions"
- When only pre-existing findings remain after filtering, the check
  is `success` but the description shows the pre-existing breakdown

#### Email notification format

When email mode is enabled, Sashiko sends a plain-text email with:

- **To**: the configured `patchwork.email` address
- **Subject**: `[sashiko-check] {status} - {patch_subject}`
- **Body** (one key-value pair per line):

```
msgid: <message-id>
status: success|warning
description: Sashiko AI review found N potential issue(s)
target_url: https://sashiko.dev/#/patchset/...
context: sashiko
```

Downstream tools can parse this format with simple line splitting.

## Environment variables

| Variable | Description |
|----------|-------------|
| `LLM_API_KEY` | API key for the configured LLM provider (universal fallback). |
| `GEMINI_API_KEY` | API key for Gemini (takes precedence over `LLM_API_KEY`). |
| `ANTHROPIC_API_KEY` | API key for Claude (takes precedence over `LLM_API_KEY`). |
| `OPENAI_API_KEY` | API key for OpenAI-compatible providers (takes precedence over `LLM_API_KEY`). |
| `ANTHROPIC_BASE_URL` | Override the Claude API base URL (for proxies). |
| `ANTHROPIC_VERTEX_PROJECT_ID` | GCP project ID for Vertex AI provider. |
| `CLOUD_ML_REGION` | GCP region for Vertex AI provider. |
| `SASHIKO_SERVER` | Override daemon URL for CLI commands. |
| `SASHIKO__*` | Override any Settings.toml value (e.g. `SASHIKO__AI__PROVIDER`). |
| `SASHIKO__FORGE__WEBHOOK_SECRET` | Override webhook secret from Settings.toml. Avoids storing the secret on disk. |
| `SASHIKO_PATCHWORK_TOKEN` | Patchwork API token. Fills in `patchwork.token` for enabled subsystems that have `api_url` set but no explicit token in TOML. |
| `NO_COLOR` | Disable ANSI color output. |
| `SASHIKO_LOG_PLAIN` | Use plain log format (no level/target/timestamp). |
