// Copyright 2026 The Sashiko Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! AI provider that shells out to the `codex` CLI (OpenAI Codex/GPT).
//! Uses the local Codex CLI installation with subscription auth.

use anyhow::{Context, Result, ensure};
use async_trait::async_trait;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::process::Stdio;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;
use tokio::time::timeout;
use tracing::{debug, warn};

use super::claude_cli::{
    CLI_PROMPT_FORMAT_PROFILE, build_prompt, collect_cli_output, log_and_redact_stderr,
    parse_inner_response,
};
use crate::ai::{
    AiProvider, AiRequest, AiResponse, AiUsage, ProviderCapabilities, cache_identity_with,
};
use crate::utils::redact_secret;

pub struct CodexCliProvider {
    pub model: String,
    pub effort: Option<String>,
    pub timeout_seconds: u64,
}

fn build_codex_args(
    model: &str,
    effort: Option<&str>,
    instructions: &std::path::Path,
) -> Vec<String> {
    let mut args: Vec<String> = [
        "exec",
        "--json",
        "--sandbox",
        "read-only",
        "--skip-git-repo-check",
        "--ignore-user-config",
        "--ignore-rules",
        "--model",
        model,
    ]
    .into_iter()
    .map(str::to_string)
    .collect();
    for feature in [
        "shell_tool",
        "code_mode_host",
        "view_image",
        "apps",
        "hooks",
        "multi_agent",
        "skill_search",
        "skill_mcp_dependency_install",
        "remote_plugin",
        "plugins",
        "tool_suggest",
        "browser_use",
        "browser_use_external",
        "computer_use",
        "memories",
        "goals",
        "sleep_tool",
    ] {
        args.extend(["--disable".to_string(), feature.to_string()]);
    }
    for setting in [
        "project_doc_max_bytes=0".to_string(),
        "developer_instructions=\"\"".to_string(),
        "web_search=\"disabled\"".to_string(),
        format!(
            "model_instructions_file={}",
            toml::Value::String(instructions.to_string_lossy().into_owned())
        ),
    ] {
        args.extend(["-c".to_string(), setting]);
    }
    if let Some(effort) = effort {
        args.extend(["-c".to_string(), format!("model_reasoning_effort={effort}")]);
    }
    args
}

impl CodexCliProvider {
    async fn generate_content_with_binary(
        &self,
        request: AiRequest,
        binary: &std::ffi::OsStr,
    ) -> Result<AiResponse> {
        let prompt = build_prompt(&request);

        debug!("codex-cli prompt length: {} chars", prompt.len());

        let working_directory = tempfile::tempdir()?;
        let instructions = working_directory.path().join("completion.md");
        tokio::fs::write(
            &instructions,
            "Act only as a text completion backend. Follow the supplied review prompt and its response format. Tool names inside that prompt describe JSON requests for the caller, not native tools to discover or execute. Do not inspect the host, load local instructions, or delegate work. Return exactly one response.\n",
        )
        .await?;
        let child = Command::new(binary)
            .args(build_codex_args(
                &self.model,
                self.effort.as_deref(),
                &instructions,
            ))
            .current_dir(working_directory.path())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .map_err(|e| anyhow::anyhow!("Failed to spawn codex CLI: {}. Is it installed?", e))?;

        let output = timeout(
            Duration::from_secs(self.timeout_seconds),
            collect_cli_output(child, &prompt),
        )
        .await
        .map_err(|_| anyhow::anyhow!("codex CLI timed out after {} seconds", self.timeout_seconds))?
        .map_err(|e| anyhow::anyhow!("codex CLI wait error: {}", e))?;

        let stderr = log_and_redact_stderr("codex-cli", &output.stderr);

        if !output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // codex reports why it rejected an invocation -- an unsupported
            // model_reasoning_effort value, say -- as a JSON error event on
            // stdout, while stderr carries only progress chatter. Either one
            // can hold the cause the other is missing: a per-turn error event
            // on stdout does not explain a login failure printed on stderr, so
            // report both.
            let mut detail = collect_error_messages(&stdout);
            if !stderr.is_empty() {
                detail.push(stderr);
            }
            anyhow::bail!(
                "codex CLI exited with {}: {}",
                output.status,
                detail.join("; ")
            );
        }

        let raw = String::from_utf8_lossy(&output.stdout);

        // Codex outputs line-delimited JSON events:
        //   {"type": "item.completed", "item": {"text": "..."}}
        //   {"type": "turn.completed", "usage": {"input_tokens": N, "output_tokens": N}}
        //   {"type": "error", "message": "..."}
        let mut text_parts = Vec::new();
        let mut usage: Option<AiUsage> = None;
        let mut saw_event = false;

        for line in raw.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            if let Ok(event) = serde_json::from_str::<Value>(trimmed) {
                if matches!(
                    event["type"].as_str(),
                    Some(
                        "thread.started"
                            | "turn.started"
                            | "turn.completed"
                            | "item.started"
                            | "item.updated"
                            | "item.completed"
                            | "error"
                    )
                ) {
                    saw_event = true;
                }
                if event["type"] == "turn.failed" {
                    let detail = error_message(&event).unwrap_or("No failure detail reported");
                    anyhow::bail!(
                        "codex CLI reported a failed turn: {}",
                        redact_secret(detail)
                    );
                }
                if let Some(message) = error_message(&event) {
                    self.check_error_message(message)?;
                }
                match event["type"].as_str() {
                    Some("item.completed") if event["item"]["type"] == "agent_message" => {
                        if let Some(text) = event["item"]["text"].as_str() {
                            text_parts.push(text.to_string());
                        }
                    }
                    Some("turn.completed") => {
                        usage = parse_usage(&event["usage"]);
                    }
                    _ => {}
                }
            }
        }

        let response_text = text_parts.join("\n");
        if response_text.trim().is_empty() {
            if saw_event || raw.trim().is_empty() {
                anyhow::bail!("codex CLI produced no agent message");
            }
            // Fall back to raw output if no events parsed
            warn!("codex-cli: no item.completed events found, using raw output");
            return parse_inner_response(&raw, usage);
        }

        parse_inner_response(&response_text, usage)
    }
}

#[derive(Debug, Clone)]
pub struct CodexSubscriptionQuotaSnapshot {
    pub five_hour: super::quota::FiveHourQuotaSample,
    pub weekly_remaining: super::quota::RemainingQuotaPercent,
    pub weekly_resets_at: i64,
}

#[derive(Debug, Clone)]
pub struct CodexWorkspaceQuotaSnapshot(CodexSubscriptionQuotaSnapshot);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodexWorkspaceQuotaKey(String);

impl CodexWorkspaceQuotaKey {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

const PREMIUM_QUOTA_MAX_AGE_SECONDS: i64 = 20;

fn valid_quota_account_key(key: &str) -> bool {
    key.len() == 64 && key.bytes().all(|byte| byte.is_ascii_hexdigit())
}

impl CodexWorkspaceQuotaSnapshot {
    pub fn quota(&self) -> &CodexSubscriptionQuotaSnapshot {
        &self.0
    }

    pub fn workspace_key(&self) -> CodexWorkspaceQuotaKey {
        CodexWorkspaceQuotaKey(self.0.five_hour.account_key.clone())
    }

    pub fn has_fresh_premium_quota(
        &self,
        expected_account_key: &str,
        observation_started_at: i64,
        now: i64,
    ) -> bool {
        valid_quota_account_key(expected_account_key)
            && self.0.five_hour.account_key == expected_account_key
            && observation_started_at >= 0
            && self.0.five_hour.observed_at >= observation_started_at
            && self.0.five_hour.observed_at <= now
            && now - self.0.five_hour.observed_at <= PREMIUM_QUOTA_MAX_AGE_SECONDS
            && self.0.five_hour.resets_at > now
            && self.0.weekly_resets_at > now
            && self.0.five_hour.remaining.exceeds(50.0)
            && self.0.weekly_remaining.exceeds(0.0)
    }
}

const QUOTA_METADATA_BYTES: u64 = 1024 * 1024;

#[derive(Clone, Copy)]
enum QuotaMetadataMethod {
    Initialize,
    Account,
    RateLimits,
}

impl QuotaMetadataMethod {
    fn request(self, id: u64) -> Value {
        let (method, params) = match self {
            Self::Initialize => (
                "initialize",
                serde_json::json!({
                    "clientInfo": {"name": "sashiko-quota-observer", "version": "1"},
                    "capabilities": {"experimentalApi": true}
                }),
            ),
            Self::Account => ("account/read", serde_json::json!({"refreshToken": false})),
            Self::RateLimits => ("account/rateLimits/read", serde_json::json!({})),
        };
        serde_json::json!({"id": id, "method": method, "params": params})
    }
}

struct QuotaMetadataRpc {
    stdout: BufReader<tokio::io::Take<tokio::process::ChildStdout>>,
    stdin: tokio::process::ChildStdin,
    bytes_read: u64,
    messages_read: usize,
}

impl QuotaMetadataRpc {
    async fn request(&mut self, id: u64, method: QuotaMetadataMethod) -> Result<Value> {
        let mut request = serde_json::to_vec(&method.request(id))?;
        request.push(b'\n');
        self.stdin.write_all(&request).await?;
        self.stdin.flush().await?;
        loop {
            let mut line = Vec::new();
            let size = self.stdout.read_until(b'\n', &mut line).await?;
            ensure!(
                size > 0,
                "Codex quota metadata stream ended before its response"
            );
            self.bytes_read += u64::try_from(size)?;
            self.messages_read += 1;
            ensure!(
                self.bytes_read <= QUOTA_METADATA_BYTES
                    && self.messages_read <= 64
                    && line.len() <= 65536,
                "Codex quota metadata output limit exceeded"
            );
            let message: Value = serde_json::from_slice(&line)
                .map_err(|_| anyhow::anyhow!("Codex quota metadata response is not valid JSON"))?;
            if message.get("method").is_some() && message.get("id").is_none() {
                continue;
            }
            ensure!(
                message.get("method").is_none() && message["id"].as_u64() == Some(id),
                "Unexpected Codex quota metadata response or server request"
            );
            ensure!(
                message.get("error").is_none(),
                "Codex quota metadata RPC failed"
            );
            return message
                .get("result")
                .cloned()
                .context("Codex quota metadata result unavailable");
        }
    }
}

pub async fn read_codex_subscription_quota(
    expected_account_key: &str,
) -> Result<CodexSubscriptionQuotaSnapshot> {
    read_codex_subscription_quota_with_binary(
        std::ffi::OsStr::new("codex"),
        expected_account_key,
        Duration::from_secs(20),
    )
    .await
}

pub async fn read_codex_workspace_quota(
    expected_workspace_key: &str,
) -> Result<CodexWorkspaceQuotaSnapshot> {
    read_codex_workspace_quota_with_binary(
        std::ffi::OsStr::new("codex"),
        expected_workspace_key,
        Duration::from_secs(20),
    )
    .await
}

async fn read_codex_workspace_quota_with_binary(
    binary: &std::ffi::OsStr,
    expected_workspace_key: &str,
    deadline: Duration,
) -> Result<CodexWorkspaceQuotaSnapshot> {
    read_codex_quota_with_binary(
        binary,
        expected_workspace_key,
        deadline,
        codex_workspace_quota_key,
        decode_codex_workspace_quota,
    )
    .await
}

async fn read_codex_subscription_quota_with_binary(
    binary: &std::ffi::OsStr,
    expected_account_key: &str,
    deadline: Duration,
) -> Result<CodexSubscriptionQuotaSnapshot> {
    read_codex_quota_with_binary(
        binary,
        expected_account_key,
        deadline,
        codex_quota_account_key,
        decode_codex_subscription_quota,
    )
    .await
}

async fn read_codex_quota_with_binary<T>(
    binary: &std::ffi::OsStr,
    expected_account_key: &str,
    deadline: Duration,
    account_key: fn(&Value) -> Result<String>,
    decode: fn(&Value, &Value, &str, i64) -> Result<T>,
) -> Result<T> {
    ensure!(
        valid_quota_account_key(expected_account_key),
        "Invalid expected Codex quota account key"
    );
    ensure!(
        !deadline.is_zero() && deadline <= Duration::from_secs(20),
        "Invalid Codex metadata deadline"
    );
    let working_directory = tempfile::tempdir()?;
    let mut child = Command::new(binary)
        .args(["app-server", "--stdio"])
        .current_dir(working_directory.path())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .kill_on_drop(true)
        .spawn()
        .context("Failed to start Codex quota metadata process")?;
    let stdout = child
        .stdout
        .take()
        .context("Codex metadata stdout unavailable")?;
    let stdin = child
        .stdin
        .take()
        .context("Codex metadata stdin unavailable")?;
    let mut rpc = QuotaMetadataRpc {
        stdout: BufReader::new(stdout.take(QUOTA_METADATA_BYTES + 1)),
        stdin,
        bytes_read: 0,
        messages_read: 0,
    };
    let result = timeout(deadline, async {
        rpc.request(1, QuotaMetadataMethod::Initialize).await?;
        rpc.stdin
            .write_all(b"{\"method\":\"initialized\",\"params\":{}}\n")
            .await?;
        let before = rpc.request(2, QuotaMetadataMethod::Account).await?;
        ensure!(
            account_key(&before)? == expected_account_key,
            "Codex quota account binding changed before observation"
        );
        let observed_at = chrono::Utc::now().timestamp();
        let limits = rpc.request(3, QuotaMetadataMethod::RateLimits).await?;
        let after = rpc.request(4, QuotaMetadataMethod::Account).await?;
        ensure!(
            account_key(&after)? == expected_account_key
                && before["account"]["email"] == after["account"]["email"],
            "Codex quota account binding changed during observation"
        );
        decode(&after, &limits, expected_account_key, observed_at)
    })
    .await
    .map_err(|_| anyhow::anyhow!("Codex quota metadata deadline exceeded"));
    drop(rpc);
    let _ = child.kill().await;
    child
        .wait()
        .await
        .context("Failed to reap Codex quota metadata process")?;
    result?
}

pub fn codex_quota_account_key(account_result: &Value) -> Result<String> {
    let account = &account_result["account"];
    ensure!(
        account["type"].as_str() == Some("chatgpt"),
        "Codex subscription account metadata required"
    );
    let email = account["email"]
        .as_str()
        .filter(|value| {
            value.contains('@')
                && value.len() <= 1024
                && !value
                    .chars()
                    .any(|character| character.is_control() || character.is_whitespace())
        })
        .context("Codex login identity unavailable")?;
    let plan = account["planType"]
        .as_str()
        .filter(|value| {
            !value.is_empty()
                && value.len() <= 128
                && value
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
        })
        .context("Codex subscription plan unavailable")?;
    let identity = format!(
        "codex-chatgpt-v1\n{}\n{plan}",
        email.trim().to_ascii_lowercase()
    );
    Ok(Sha256::digest(identity.as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect())
}

pub fn codex_workspace_quota_key(account_result: &Value) -> Result<String> {
    let login_key = codex_quota_account_key(account_result)?;
    let workspace_id = account_result["workspaceRouting"]["chatgptAccountId"]
        .as_str()
        .filter(|value| {
            !value.is_empty()
                && value.len() <= 1024
                && !value
                    .chars()
                    .any(|character| character.is_control() || character.is_whitespace())
        })
        .context("Codex workspace identity unavailable")?;
    let identity = format!("codex-chatgpt-workspace-v1\n{login_key}\n{workspace_id}");
    Ok(Sha256::digest(identity.as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect())
}

fn decode_quota_window(
    value: &Value,
    minutes: i64,
    observed_at: i64,
) -> Result<(super::quota::RemainingQuotaPercent, i64)> {
    ensure!(
        value["windowDurationMins"].as_i64() == Some(minutes),
        "Unsupported Codex quota window duration"
    );
    let used = value["usedPercent"]
        .as_f64()
        .context("Measured Codex quota percentage unavailable")?;
    ensure!(
        used.is_finite() && (0.0..=100.0).contains(&used),
        "Invalid Codex used quota percentage"
    );
    let remaining = super::quota::RemainingQuotaPercent::new(100.0 - used)?;
    let resets_at = value["resetsAt"]
        .as_i64()
        .context("Codex quota reset unavailable")?;
    ensure!(
        resets_at > observed_at,
        "Codex quota window already expired"
    );
    Ok((remaining, resets_at))
}

pub fn decode_codex_subscription_quota(
    account_result: &Value,
    limits_result: &Value,
    expected_account_key: &str,
    observed_at: i64,
) -> Result<CodexSubscriptionQuotaSnapshot> {
    ensure!(observed_at >= 0, "Invalid Codex quota observation time");
    let account_key = codex_quota_account_key(account_result)?;
    ensure!(
        account_key == expected_account_key,
        "Codex quota account binding changed"
    );
    let limits = &limits_result["rateLimits"];
    ensure!(
        limits["limitId"].as_str() == Some("codex"),
        "Codex account quota limit identity unavailable"
    );
    ensure!(
        limits["planType"].as_str() == account_result["account"]["planType"].as_str(),
        "Codex account and quota plans differ"
    );
    ensure!(
        limits["spendControlReached"].as_bool() == Some(false)
            && limits["credits"]["hasCredits"].as_bool() == Some(false)
            && limits["credits"]["unlimited"].as_bool() == Some(false),
        "Paid, exhausted or unknown Codex spend controls are unsupported"
    );
    let (remaining, resets_at) = decode_quota_window(&limits["primary"], 300, observed_at)?;
    let (weekly_remaining, weekly_resets_at) =
        decode_quota_window(&limits["secondary"], 10080, observed_at)?;
    Ok(CodexSubscriptionQuotaSnapshot {
        five_hour: super::quota::FiveHourQuotaSample {
            account_key,
            observed_at,
            remaining,
            resets_at,
        },
        weekly_remaining,
        weekly_resets_at,
    })
}

pub fn decode_codex_workspace_quota(
    account_result: &Value,
    limits_result: &Value,
    expected_workspace_key: &str,
    observed_at: i64,
) -> Result<CodexWorkspaceQuotaSnapshot> {
    let workspace_key = codex_workspace_quota_key(account_result)?;
    ensure!(
        workspace_key == expected_workspace_key,
        "Codex workspace binding changed"
    );
    ensure!(
        limits_result["accountId"].as_str()
            == account_result["workspaceRouting"]["chatgptAccountId"].as_str(),
        "Codex workspace and quota account differ"
    );
    ensure!(
        limits_result["ordinaryUsageAllowed"].as_bool() == Some(true),
        "Codex included usage permission unavailable or denied"
    );
    let login_key = codex_quota_account_key(account_result)?;
    let mut snapshot =
        decode_codex_subscription_quota(account_result, limits_result, &login_key, observed_at)?;
    snapshot.five_hour.account_key = workspace_key;
    Ok(CodexWorkspaceQuotaSnapshot(snapshot))
}

fn parse_usage(value: &Value) -> Option<AiUsage> {
    let prompt_tokens = usize::try_from(value["input_tokens"].as_u64()?).ok()?;
    let completion_tokens = usize::try_from(value["output_tokens"].as_u64()?).ok()?;
    let cached_tokens = match value
        .get("cached_input_tokens")
        .filter(|value| !value.is_null())
    {
        Some(value) => Some(usize::try_from(value.as_u64()?).ok()?),
        None => None,
    };
    if cached_tokens.is_some_and(|cached| cached > prompt_tokens) {
        return None;
    }
    Some(AiUsage {
        prompt_tokens,
        completion_tokens,
        total_tokens: prompt_tokens.checked_add(completion_tokens)?,
        cached_tokens,
    })
}

#[async_trait]
impl AiProvider for CodexCliProvider {
    async fn generate_content(&self, request: AiRequest) -> Result<AiResponse> {
        self.generate_content_with_binary(request, std::ffi::OsStr::new("codex"))
            .await
    }

    fn get_capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            model_name: self.model.clone(),
            context_window_size: 200_000,
        }
    }

    fn cache_identity(&self) -> String {
        cache_identity_with(
            &self.model,
            &[
                ("effort", self.effort.as_deref()),
                ("completion_profile", Some("restricted-v2")),
                ("usage_profile", Some("measured-v1")),
                ("terminal_profile", Some("failed-turn-v1")),
                ("item_profile", Some("agent-message-v1")),
                ("prompt_format", Some(CLI_PROMPT_FORMAT_PROFILE)),
            ],
        )
    }
}

impl CodexCliProvider {
    /// Handles an error event from a run that exited 0. A requirements layer
    /// that substitutes its own value for a setting reports the substitution
    /// this way and lets the run proceed.
    fn check_error_message(&self, message: &str) -> Result<()> {
        let message = redact_secret(message);
        if is_approval_policy_substitution(&message) {
            debug!("codex-cli: {}", message);
        } else if message.contains("disallowed by requirements")
            && !is_effort_substitution(&message)
        {
            anyhow::bail!("codex CLI could not apply the completion profile: {message}");
        } else if self.effort.is_some() && is_effort_substitution(&message) {
            // The response cache files the reply under the configured effort,
            // so a reply produced at the substituted one must not be returned.
            anyhow::bail!("codex CLI did not run at the configured effort: {message}");
        } else {
            warn!("codex-cli: {}", message);
        }
        Ok(())
    }
}

/// Returns the explanation from a CLI error or failed-turn event.
fn error_message(event: &Value) -> Option<&str> {
    if event["type"] == "turn.failed" {
        return Some(
            event["error"]["message"]
                .as_str()
                .unwrap_or("No failure detail reported"),
        );
    }
    [event, &event["item"]]
        .into_iter()
        .find(|candidate| candidate["type"] == "error")
        .and_then(|candidate| candidate["message"].as_str())
}

/// Pulls the distinct error messages out of a run's stdout.
fn collect_error_messages(raw: &str) -> Vec<String> {
    let mut messages = Vec::new();
    for line in raw.lines() {
        let Ok(event) = serde_json::from_str::<Value>(line.trim()) else {
            continue;
        };
        if let Some(message) = error_message(&event) {
            let redacted = redact_secret(message);
            if !messages.iter().any(|m| m == &redacted) {
                messages.push(redacted);
            }
        }
    }
    messages
}

/// Recognizes the one substitution that carries nothing to act on. `codex
/// exec` resolves `approval_policy` to `Never` itself, a non-interactive run
/// having nobody to prompt, so a requirements layer that disallows `Never`
/// substitutes its own value on every invocation. Nothing here configures the
/// policy and a `-c` override does not reach the check, so the message names
/// no setting the operator can change. The sandbox stays read-only whichever
/// policy wins, so an escalation request is refused under the substituted
/// value exactly as it was under `Never`.
fn is_approval_policy_substitution(message: &str) -> bool {
    message.contains("`approval_policy`")
}

/// Recognizes a substitution of the effort this provider passed on the
/// command line.
fn is_effort_substitution(message: &str) -> bool {
    message.contains("`model_reasoning_effort`")
}

#[cfg(test)]
mod tests {
    #[cfg(unix)]
    #[tokio::test]
    async fn quota_reader_only_uses_bounded_metadata_methods() -> Result<()> {
        let fixture = tempfile::tempdir()?;
        let executable = fixture.path().join("codex");
        crate::test_support::write_executable_script(
            &executable,
            r#"#!/bin/sh
fixture_dir=${0%/*}
printf '%s\n' "$PWD" "$@" > "$fixture_dir/metadata-args"
while IFS= read -r request; do
    printf '%s\n' "$request" >> "$fixture_dir/metadata-requests"
    case "$request" in
        *'"id":1,'*) printf '%s\n' '{"id":1,"result":{}}' ;;
        *'"id":2,'*) printf '%s\n' '{"id":2,"result":{"account":{"type":"chatgpt","email":"reviewer@example.invalid","planType":"pro"}}}' ;;
        *'"id":3,'*) printf '%s\n' '{"id":3,"result":{"rateLimits":{"limitId":"codex","planType":"pro","spendControlReached":false,"credits":{"hasCredits":false,"unlimited":false},"primary":{"usedPercent":65,"windowDurationMins":300,"resetsAt":4102444800},"secondary":{"usedPercent":47,"windowDurationMins":10080,"resetsAt":4103049600}}}}' ;;
        *'"id":4,'*) printf '%s\n' '{"id":4,"result":{"account":{"type":"chatgpt","email":"reviewer@example.invalid","planType":"pro"}}}' ;;
    esac
done
"#,
        );
        let account = serde_json::json!({"account":{"type":"chatgpt","email":"reviewer@example.invalid","planType":"pro"}});
        let key = codex_quota_account_key(&account)?;
        let result = read_codex_subscription_quota_with_binary(
            executable.as_os_str(),
            &key,
            Duration::from_secs(2),
        )
        .await?;
        assert_eq!(
            result.five_hour.remaining,
            super::super::quota::RemainingQuotaPercent::new(35.0)?
        );
        assert_eq!(
            result.weekly_remaining,
            super::super::quota::RemainingQuotaPercent::new(53.0)?
        );
        let arguments = std::fs::read_to_string(fixture.path().join("metadata-args"))?;
        let mut arguments = arguments.lines();
        let directory = std::path::PathBuf::from(
            arguments
                .next()
                .context("Metadata working directory missing")?,
        );
        assert_ne!(directory, std::env::current_dir()?);
        assert!(!directory.exists());
        assert_eq!(arguments.collect::<Vec<_>>(), ["app-server", "--stdio"]);
        let requests = std::fs::read_to_string(fixture.path().join("metadata-requests"))?;
        let requests: Vec<Value> = requests
            .lines()
            .map(serde_json::from_str)
            .collect::<std::result::Result<_, _>>()?;
        assert_eq!(
            requests
                .iter()
                .map(|request| request["method"].as_str().unwrap_or(""))
                .collect::<Vec<_>>(),
            [
                "initialize",
                "initialized",
                "account/read",
                "account/rateLimits/read",
                "account/read"
            ]
        );
        Ok(())
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn quota_reader_fails_closed_on_untrusted_or_unbounded_output() -> Result<()> {
        let account = serde_json::json!({"account":{"type":"chatgpt","email":"reviewer@example.invalid","planType":"pro"}});
        let key = codex_quota_account_key(&account)?;
        let responses = [
            "not JSON".to_string(),
            r#"{"id":3,"error":{"message":"fixture-sensitive-payload"}}"#.to_string(),
            r#"{"id":3}"#.to_string(),
            r#"{"id":4,"result":{}}"#.to_string(),
            r#"{"id":99,"method":"tool/call","params":{"private":"fixture-sensitive-payload"}}"#
                .to_string(),
            "x".repeat(65537),
            "x".repeat(usize::try_from(QUOTA_METADATA_BYTES)? + 1),
            "{\"method\":\"progress\"}\n".repeat(65),
        ];
        for response in responses {
            let fixture = tempfile::tempdir()?;
            let executable = fixture.path().join("codex");
            let script = r#"#!/bin/sh
fixture_dir=${0%/*}
printf '%s\n' "$$" > "$fixture_dir/pid"
while IFS= read -r request; do
    case "$request" in
        *'"id":1,'*) printf '%s\n' '{"id":1,"result":{}}' ;;
        *'"id":2,'*) printf '%s\n' '{"id":2,"result":{"account":{"type":"chatgpt","email":"reviewer@example.invalid","planType":"pro"}}}' ;;
        *'"id":3,'*) printf '%s\n' 'QUOTA_RESPONSE' ;;
    esac
done
"#.replace("QUOTA_RESPONSE", &response);
            crate::test_support::write_executable_script(&executable, &script);
            let error = read_codex_subscription_quota_with_binary(
                executable.as_os_str(),
                &key,
                Duration::from_secs(2),
            )
            .await
            .expect_err("invalid metadata must not yield quota");
            assert!(!error.to_string().contains("fixture-sensitive-payload"));
            #[cfg(target_os = "linux")]
            {
                let pid = std::fs::read_to_string(fixture.path().join("pid"))?;
                assert!(
                    !std::path::Path::new(&format!("/proc/{}", pid.trim())).exists(),
                    "metadata child was not reaped"
                );
            }
        }
        for script in [
            "#!/bin/sh\nexit 0\n",
            "#!/bin/sh\nwhile IFS= read -r request; do :; done\n",
        ] {
            let fixture = tempfile::tempdir()?;
            let executable = fixture.path().join("codex");
            crate::test_support::write_executable_script(&executable, script);
            assert!(
                read_codex_subscription_quota_with_binary(
                    executable.as_os_str(),
                    &key,
                    Duration::from_millis(250)
                )
                .await
                .is_err()
            );
        }
        Ok(())
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn quota_reader_rejects_account_switch_between_metadata_calls() -> Result<()> {
        let fixture = tempfile::tempdir()?;
        let executable = fixture.path().join("codex");
        crate::test_support::write_executable_script(
            &executable,
            r#"#!/bin/sh
while IFS= read -r request; do
    case "$request" in
        *'"id":1,'*) printf '%s\n' '{"id":1,"result":{}}' ;;
        *'"id":2,'*) printf '%s\n' '{"id":2,"result":{"account":{"type":"chatgpt","email":"reviewer@example.invalid","planType":"pro"}}}' ;;
        *'"id":3,'*) printf '%s\n' '{"id":3,"result":{}}' ;;
        *'"id":4,'*) printf '%s\n' '{"id":4,"result":{"account":{"type":"chatgpt","email":"other@example.invalid","planType":"pro"}}}' ;;
    esac
done
"#,
        );
        let account = serde_json::json!({"account":{"type":"chatgpt","email":"reviewer@example.invalid","planType":"pro"}});
        let key = codex_quota_account_key(&account)?;
        let error = read_codex_subscription_quota_with_binary(
            executable.as_os_str(),
            &key,
            Duration::from_secs(2),
        )
        .await
        .expect_err("changed account must not yield quota");
        assert!(error.to_string().contains("changed during observation"));
        assert!(!error.to_string().contains("other@example.invalid"));
        Ok(())
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn workspace_quota_reader_checks_binding_across_metadata_calls() -> Result<()> {
        let account = serde_json::json!({
            "account": {"type": "chatgpt", "email": "reviewer@example.invalid", "planType": "pro"},
            "workspaceRouting": {"chatgptAccountId": "workspace-a"}
        });
        let key = codex_workspace_quota_key(&account)?;
        for (quota_id, after_id, allowed, expected) in [
            ("workspace-a", "workspace-a", "true", true),
            ("workspace-a", "workspace-b", "true", false),
            ("workspace-b", "workspace-a", "true", false),
            ("workspace-a", "workspace-a", "false", false),
        ] {
            let fixture = tempfile::tempdir()?;
            let executable = fixture.path().join("codex");
            let script = r#"#!/bin/sh
while IFS= read -r request; do
    case "$request" in
        *'"id":1,'*) printf '%s\n' '{"id":1,"result":{}}' ;;
        *'"id":2,'*) printf '%s\n' '{"id":2,"result":{"account":{"type":"chatgpt","email":"reviewer@example.invalid","planType":"pro"},"workspaceRouting":{"chatgptAccountId":"workspace-a"}}}' ;;
        *'"id":3,'*) printf '%s\n' '{"id":3,"result":{"accountId":"QUOTA_ID","ordinaryUsageAllowed":ALLOWED,"rateLimits":{"limitId":"codex","planType":"pro","spendControlReached":false,"credits":{"hasCredits":false,"unlimited":false},"primary":{"usedPercent":49,"windowDurationMins":300,"resetsAt":4102444800},"secondary":{"usedPercent":47,"windowDurationMins":10080,"resetsAt":4103049600}}}}' ;;
        *'"id":4,'*) printf '%s\n' '{"id":4,"result":{"account":{"type":"chatgpt","email":"reviewer@example.invalid","planType":"pro"},"workspaceRouting":{"chatgptAccountId":"AFTER_ID"}}}' ;;
    esac
done
"#
            .replace("QUOTA_ID", quota_id)
            .replace("AFTER_ID", after_id)
            .replace("ALLOWED", allowed);
            crate::test_support::write_executable_script(&executable, &script);
            let result = read_codex_workspace_quota_with_binary(
                executable.as_os_str(),
                &key,
                Duration::from_secs(2),
            )
            .await;
            assert_eq!(result.is_ok(), expected);
            if let Ok(snapshot) = result {
                assert_eq!(snapshot.quota().five_hour.account_key, key);
            }
        }
        Ok(())
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn quota_reader_cancellation_terminates_its_metadata_child() -> Result<()> {
        let fixture = tempfile::tempdir()?;
        let executable = fixture.path().join("codex");
        let pid_file = fixture.path().join("pid");
        crate::test_support::write_executable_script(
            &executable,
            r#"#!/bin/sh
fixture_dir=${0%/*}
printf '%s\n' "$$" > "$fixture_dir/pid"
while IFS= read -r request; do :; done
"#,
        );
        let account = serde_json::json!({"account":{"type":"chatgpt","email":"reviewer@example.invalid","planType":"pro"}});
        let key = codex_quota_account_key(&account)?;
        let task = tokio::spawn(async move {
            read_codex_subscription_quota_with_binary(
                executable.as_os_str(),
                &key,
                Duration::from_secs(20),
            )
            .await
        });
        timeout(Duration::from_secs(2), async {
            while !pid_file.exists() {
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
        .await?;
        let pid = std::fs::read_to_string(pid_file)?;
        let process = std::path::PathBuf::from(format!("/proc/{}", pid.trim()));
        assert!(process.exists());
        task.abort();
        assert!(task.await.unwrap_err().is_cancelled());
        timeout(Duration::from_secs(2), async {
            while process.exists() {
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
        .await?;
        Ok(())
    }

    #[test]
    fn subscription_quota_decoder_binds_known_windows_without_authorizing_spend() -> Result<()> {
        let account = serde_json::json!({"account":{"type":"chatgpt","email":"reviewer@example.invalid","planType":"pro"}});
        let limits = serde_json::json!({"rateLimits":{"limitId":"codex","planType":"pro","spendControlReached":false,
            "credits":{"hasCredits":false,"unlimited":false},
            "primary":{"usedPercent":65,"windowDurationMins":300,"resetsAt":200},
            "secondary":{"usedPercent":47,"windowDurationMins":10080,"resetsAt":1000}}});
        let key = codex_quota_account_key(&account)?;
        assert_eq!(key.len(), 64);
        assert!(!key.contains("reviewer"));
        let decoded = decode_codex_subscription_quota(&account, &limits, &key, 100)?;
        assert_eq!(decoded.five_hour.account_key, key);
        assert_eq!(
            decoded.five_hour.remaining,
            super::super::quota::RemainingQuotaPercent::new(35.0)?
        );
        assert_eq!(
            decoded.weekly_remaining,
            super::super::quota::RemainingQuotaPercent::new(53.0)?
        );
        assert_eq!(decoded.weekly_resets_at, 1000);
        let policy = super::super::quota::CodexAccountQuotaPolicy {
            enabled: true,
            ..Default::default()
        };
        let state = super::super::quota::AccountQuotaState {
            account_key: key.clone(),
            admission: super::super::quota::AccountQuotaAdmission::Ready,
        };
        assert_eq!(
            policy.evaluate(100, &state, Some(&decoded.five_hour))?.gate,
            super::super::quota::QuotaAdmissionGate::FiveHourReady
        );
        assert!(decode_codex_subscription_quota(&account, &limits, "other-account", 100).is_err());
        let mut metered = account.clone();
        metered["account"]["type"] = serde_json::json!("apiKey");
        assert!(codex_quota_account_key(&metered).is_err());
        for invalid in [
            serde_json::Value::Null,
            serde_json::json!({"account":{"type":"chatgpt","email":""}}),
            serde_json::json!({"account":{"type":"chatgpt","email":"reviewer@example.invalid\npro","planType":"pro"}}),
        ] {
            assert!(codex_quota_account_key(&invalid).is_err());
        }
        let mut changed = account.clone();
        changed["account"]["email"] = serde_json::json!("other@example.invalid");
        assert!(decode_codex_subscription_quota(&changed, &limits, &key, 100).is_err());
        for pointer in [
            "/rateLimits/primary/usedPercent",
            "/rateLimits/primary/windowDurationMins",
            "/rateLimits/secondary",
            "/rateLimits/spendControlReached",
            "/rateLimits/credits/hasCredits",
        ] {
            let mut malformed = limits.clone();
            *malformed.pointer_mut(pointer).unwrap() = serde_json::Value::Null;
            assert!(
                decode_codex_subscription_quota(&account, &malformed, &key, 100).is_err(),
                "{pointer}"
            );
        }
        for (pointer, value) in [
            ("/rateLimits/primary/usedPercent", serde_json::json!(-0.1)),
            ("/rateLimits/primary/usedPercent", serde_json::json!(-1e-30)),
            ("/rateLimits/primary/usedPercent", serde_json::json!(100.1)),
            (
                "/rateLimits/secondary/usedPercent",
                serde_json::json!(-1e-30),
            ),
            ("/rateLimits/secondary/usedPercent", serde_json::json!("47")),
            (
                "/rateLimits/primary/windowDurationMins",
                serde_json::json!(60),
            ),
            (
                "/rateLimits/secondary/windowDurationMins",
                serde_json::json!(300),
            ),
            ("/rateLimits/primary/resetsAt", serde_json::json!(100)),
            ("/rateLimits/primary/resetsAt", serde_json::json!(200.5)),
            ("/rateLimits/secondary/resetsAt", serde_json::json!(99)),
            ("/rateLimits/planType", serde_json::json!("team")),
            ("/rateLimits/limitId", serde_json::json!("other-model")),
            ("/rateLimits/spendControlReached", serde_json::json!(true)),
            ("/rateLimits/credits/hasCredits", serde_json::json!(true)),
            ("/rateLimits/credits/unlimited", serde_json::json!(true)),
        ] {
            let mut malformed = limits.clone();
            *malformed.pointer_mut(pointer).unwrap() = value;
            assert!(
                decode_codex_subscription_quota(&account, &malformed, &key, 100).is_err(),
                "{pointer}"
            );
        }
        let mut exhausted = limits.clone();
        exhausted["rateLimits"]["secondary"]["usedPercent"] = serde_json::json!(100);
        assert_eq!(
            decode_codex_subscription_quota(&account, &exhausted, &key, 100)?.weekly_remaining,
            super::super::quota::RemainingQuotaPercent::new(0.0)?
        );
        for (used, remaining) in [(0.0, 100.0), (100.0, 0.0)] {
            let mut boundary = limits.clone();
            boundary["rateLimits"]["primary"]["usedPercent"] = serde_json::json!(used);
            boundary["rateLimits"]["secondary"]["usedPercent"] = serde_json::json!(used);
            let decoded = decode_codex_subscription_quota(&account, &boundary, &key, 100)?;
            let expected = super::super::quota::RemainingQuotaPercent::new(remaining)?;
            assert_eq!(decoded.five_hour.remaining, expected);
            assert_eq!(decoded.weekly_remaining, expected);
        }
        assert!(decode_codex_subscription_quota(&account, &limits, &key, -1).is_err());
        Ok(())
    }

    #[test]
    fn premium_call_quota_requires_fresh_matching_positive_allowances() -> Result<()> {
        use super::super::quota::RemainingQuotaPercent;

        let account = serde_json::json!({"account":{"type":"chatgpt","email":"reviewer@example.invalid","planType":"pro"},
            "workspaceRouting":{"chatgptAccountId":"workspace-a"}});
        let limits = serde_json::json!({"accountId":"workspace-a","ordinaryUsageAllowed":true,
            "rateLimits":{"limitId":"codex","planType":"pro","spendControlReached":false,
            "credits":{"hasCredits":false,"unlimited":false},
            "primary":{"usedPercent":49.9,"windowDurationMins":300,"resetsAt":200},
            "secondary":{"usedPercent":99.9,"windowDurationMins":10080,"resetsAt":1000}}});
        let key = codex_workspace_quota_key(&account)?;
        let snapshot = decode_codex_workspace_quota(&account, &limits, &key, 100)?;
        assert!(snapshot.has_fresh_premium_quota(&key, 100, 100));
        assert!(snapshot.has_fresh_premium_quota(&key, 99, 120));
        assert!(!snapshot.has_fresh_premium_quota(&key, 101, 101));
        assert!(!snapshot.has_fresh_premium_quota(&key, 100, 99));
        assert!(!snapshot.has_fresh_premium_quota(&key, 100, 121));
        assert!(!snapshot.has_fresh_premium_quota(&key, -1, 100));
        assert!(!snapshot.has_fresh_premium_quota("", 100, 100));
        assert!(!snapshot.has_fresh_premium_quota("other-account", 100, 100));
        let mut malformed_key = snapshot.clone();
        malformed_key.0.five_hour.account_key = "not-a-valid-key".into();
        assert!(!malformed_key.has_fresh_premium_quota("not-a-valid-key", 100, 100));

        for remaining in [0.0, 49.9, 50.0] {
            let mut blocked = snapshot.clone();
            blocked.0.five_hour.remaining = RemainingQuotaPercent::new(remaining)?;
            assert!(!blocked.has_fresh_premium_quota(&key, 100, 100));
        }
        let mut blocked = snapshot.clone();
        blocked.0.weekly_remaining = RemainingQuotaPercent::new(0.0)?;
        assert!(!blocked.has_fresh_premium_quota(&key, 100, 100));
        blocked = snapshot.clone();
        blocked.0.five_hour.resets_at = 100;
        assert!(!blocked.has_fresh_premium_quota(&key, 100, 100));
        blocked = snapshot.clone();
        blocked.0.weekly_resets_at = 100;
        assert!(!blocked.has_fresh_premium_quota(&key, 100, 100));
        Ok(())
    }

    #[test]
    fn workspace_quota_requires_matching_backend_identity_and_permission() -> Result<()> {
        let account = serde_json::json!({
            "account": {"type": "chatgpt", "email": "reviewer@example.invalid", "planType": "pro"},
            "workspaceRouting": {"chatgptAccountId": "workspace-a"}
        });
        let limits = serde_json::json!({
            "accountId": "workspace-a", "ordinaryUsageAllowed": true,
            "rateLimits": {"limitId": "codex", "planType": "pro", "spendControlReached": false,
                "credits": {"hasCredits": false, "unlimited": false},
                "primary": {"usedPercent": 49.9, "windowDurationMins": 300, "resetsAt": 200},
                "secondary": {"usedPercent": 47, "windowDurationMins": 10080, "resetsAt": 1000}}
        });
        let key = codex_workspace_quota_key(&account)?;
        assert_eq!(key.len(), 64);
        assert!(!key.contains("workspace-a"));
        let snapshot = decode_codex_workspace_quota(&account, &limits, &key, 100)?;
        assert_eq!(snapshot.quota().five_hour.account_key, key);
        let typed_key = snapshot.workspace_key();
        assert_eq!(typed_key.as_str(), key);
        assert_ne!(typed_key.as_str(), codex_quota_account_key(&account)?);
        assert!(snapshot.has_fresh_premium_quota(&key, 100, 100));

        let mut changed_workspace = account.clone();
        changed_workspace["workspaceRouting"]["chatgptAccountId"] =
            serde_json::json!("workspace-b");
        let changed_key = codex_workspace_quota_key(&changed_workspace)?;
        assert_ne!(changed_key, key);
        assert!(decode_codex_workspace_quota(&changed_workspace, &limits, &key, 100).is_err());
        let mut changed_limits = limits.clone();
        changed_limits["accountId"] = serde_json::json!("workspace-b");
        let changed_snapshot =
            decode_codex_workspace_quota(&changed_workspace, &changed_limits, &changed_key, 100)?;
        assert_ne!(typed_key, changed_snapshot.workspace_key());
        for value in [
            serde_json::Value::Null,
            serde_json::json!(""),
            serde_json::json!("bad\nidentity"),
        ] {
            let mut invalid = account.clone();
            invalid["workspaceRouting"]["chatgptAccountId"] = value;
            assert!(codex_workspace_quota_key(&invalid).is_err());
        }
        for (field, value) in [
            ("accountId", serde_json::Value::Null),
            ("accountId", serde_json::json!("workspace-b")),
            ("ordinaryUsageAllowed", serde_json::Value::Null),
            ("ordinaryUsageAllowed", serde_json::json!(false)),
        ] {
            let mut invalid = limits.clone();
            invalid[field] = value;
            assert!(decode_codex_workspace_quota(&account, &invalid, &key, 100).is_err());
        }
        let mut paid = limits.clone();
        paid["rateLimits"]["credits"]["hasCredits"] = serde_json::json!(true);
        assert!(decode_codex_workspace_quota(&account, &paid, &key, 100).is_err());
        Ok(())
    }

    use super::*;

    #[cfg(unix)]
    #[tokio::test]
    async fn completion_profile_isolated_from_repository_and_native_tools() {
        let fixture = tempfile::tempdir().unwrap();
        let executable = fixture.path().join("codex");
        crate::test_support::write_executable_script(
            &executable,
            r#"#!/bin/sh
fixture_dir=${0%/*}
printf '%s\n' "$PWD" "$@" > "$fixture_dir/args"
cat completion.md > "$fixture_dir/instructions"
cat > "$fixture_dir/stdin"
printf '%s\n' '{"type":"item.completed","item":{"type":"reasoning","text":"internal reasoning"}}'
printf '%s\n' '{"type":"item.completed","item":{"type":"agent_message","text":"{\"content\":\"OK\"}"}}'
printf '%s\n' '{"type":"turn.completed","usage":{"input_tokens":12,"output_tokens":3}}'
"#,
        );
        let provider = CodexCliProvider {
            model: "test-model".to_string(),
            effort: Some("low".to_string()),
            timeout_seconds: 600,
        };
        let request = AiRequest {
            system: None,
            messages: vec![],
            tools: None,
            temperature: None,
            response_format: None,
            context_tag: None,
            reasoning_effort: None,
        };
        let prompt = build_prompt(&request);
        let response = provider
            .generate_content_with_binary(request, executable.as_os_str())
            .await
            .unwrap();
        assert_eq!(response.content.as_deref(), Some("OK"));
        let usage = response.usage.unwrap();
        assert_eq!(usage.total_tokens, 15);
        assert_eq!(usage.cached_tokens, None);
        let captured = std::fs::read_to_string(fixture.path().join("args")).unwrap();
        let mut lines = captured.lines();
        let cwd = std::path::PathBuf::from(lines.next().unwrap());
        assert_ne!(cwd, std::env::current_dir().unwrap());
        assert!(!cwd.exists());
        let arguments: Vec<&str> = lines.collect();
        for flag in [
            "--ignore-user-config",
            "--ignore-rules",
            "--skip-git-repo-check",
        ] {
            assert!(arguments.contains(&flag), "{flag}");
        }
        for feature in [
            "shell_tool",
            "code_mode_host",
            "view_image",
            "hooks",
            "apps",
            "multi_agent",
            "plugins",
            "browser_use",
            "computer_use",
        ] {
            assert!(
                arguments
                    .windows(2)
                    .any(|pair| pair == ["--disable", feature])
            );
        }
        for setting in [
            "project_doc_max_bytes=0",
            "developer_instructions=\"\"",
            "web_search=\"disabled\"",
            "model_reasoning_effort=low",
        ] {
            assert!(arguments.windows(2).any(|pair| pair == ["-c", setting]));
        }
        assert!(
            !arguments
                .iter()
                .any(|argument| argument.starts_with("tools.view_image="))
        );
        assert!(
            arguments
                .windows(2)
                .any(|pair| pair == ["--model", "test-model"])
        );
        assert!(
            arguments
                .iter()
                .any(|argument| argument.starts_with("model_instructions_file="))
        );
        let instructions = std::fs::read_to_string(fixture.path().join("instructions")).unwrap();
        assert!(instructions.contains("not native tools to discover or execute"));
        assert_eq!(
            std::fs::read_to_string(fixture.path().join("stdin")).unwrap(),
            prompt
        );
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn completion_distinguishes_missing_message_from_plain_output() {
        let fixture = tempfile::tempdir().unwrap();
        let executable = fixture.path().join("codex");
        crate::test_support::write_executable_script(
            &executable,
            r#"#!/bin/sh
cat >/dev/null
printf '%s\n' '{"type":"item.completed","item":{"type":"reasoning","text":"internal reasoning"}}'
printf '%s\n' '{"type":"turn.completed","usage":{"input_tokens":12,"output_tokens":3}}'
"#,
        );
        let provider = CodexCliProvider {
            model: "test-model".to_string(),
            effort: None,
            timeout_seconds: 600,
        };
        let request = AiRequest {
            system: None,
            messages: vec![],
            tools: None,
            temperature: None,
            response_format: None,
            context_tag: None,
            reasoning_effort: None,
        };
        let result = provider
            .generate_content_with_binary(request.clone(), executable.as_os_str())
            .await;
        assert!(result.is_err());

        for script in [
            "#!/bin/sh\ncat >/dev/null\n",
            "#!/bin/sh\ncat >/dev/null\nprintf '%s\\n' '{\"type\":\"item.completed\",\"item\":{\"type\":\"agent_message\",\"text\":\"   \"}}'\n",
        ] {
            crate::test_support::write_executable_script(&executable, script);
            let result = provider
                .generate_content_with_binary(request.clone(), executable.as_os_str())
                .await;
            assert!(result.is_err());
        }

        crate::test_support::write_executable_script(
            &executable,
            "#!/bin/sh\ncat >/dev/null\nprintf '%s\\n' '{\"content\":\"plain reply\"}'\n",
        );
        let response = provider
            .generate_content_with_binary(request, executable.as_os_str())
            .await
            .unwrap();
        assert_eq!(response.content.as_deref(), Some("plain reply"));
    }

    #[test]
    fn usage_requires_measured_counters_and_preserves_explicit_cached_zero() {
        for value in [
            serde_json::json!(null),
            serde_json::json!({}),
            serde_json::json!({"input_tokens": 12}),
            serde_json::json!({"output_tokens": 3}),
            serde_json::json!({"input_tokens": "12", "output_tokens": 3}),
            serde_json::json!({"input_tokens": -1, "output_tokens": 3}),
            serde_json::json!({"input_tokens": 12, "output_tokens": 3, "cached_input_tokens": 13}),
            serde_json::json!({"input_tokens": 12, "output_tokens": 3, "cached_input_tokens": "unknown"}),
            serde_json::json!({"input_tokens": u64::MAX, "output_tokens": 1}),
        ] {
            assert!(parse_usage(&value).is_none(), "{value}");
        }
        for cached in [None, Some(0), Some(10)] {
            let mut value = serde_json::json!({"input_tokens": 12, "output_tokens": 3});
            if let Some(cached) = cached {
                value["cached_input_tokens"] = serde_json::json!(cached);
            }
            let usage = parse_usage(&value).unwrap();
            assert_eq!(usage.total_tokens, 15);
            assert_eq!(usage.cached_tokens, cached);
        }
        let measured_zero = parse_usage(&serde_json::json!({
            "input_tokens": 0, "output_tokens": 0, "cached_input_tokens": 0
        }))
        .unwrap();
        assert_eq!(measured_zero.total_tokens, 0);
        assert_eq!(measured_zero.cached_tokens, Some(0));
    }

    #[test]
    fn restricted_cache_profile_excludes_ambient_agent_responses() {
        let provider = CodexCliProvider {
            model: "test-model".to_string(),
            effort: Some("low".to_string()),
            timeout_seconds: 600,
        };
        assert_ne!(
            provider.cache_identity(),
            cache_identity_with("test-model", &[("effort", Some("low"))])
        );
        assert_ne!(
            provider.cache_identity(),
            cache_identity_with(
                "test-model",
                &[
                    ("effort", Some("low")),
                    ("completion_profile", Some("restricted-v1")),
                ]
            )
        );
        assert_ne!(
            provider.cache_identity(),
            cache_identity_with(
                "test-model",
                &[
                    ("effort", Some("low")),
                    ("completion_profile", Some("restricted-v2")),
                    ("usage_profile", Some("measured-v1")),
                ]
            )
        );
        assert_ne!(
            provider.cache_identity(),
            cache_identity_with(
                "test-model",
                &[
                    ("effort", Some("low")),
                    ("completion_profile", Some("restricted-v2")),
                    ("usage_profile", Some("measured-v1")),
                    ("terminal_profile", Some("failed-turn-v1")),
                    ("prompt_format", Some(CLI_PROMPT_FORMAT_PROFILE)),
                ]
            )
        );
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn completion_drains_output_while_sending_prompt() {
        for redirect in ["", ">&2"] {
            let fixture = tempfile::tempdir().unwrap();
            let executable = fixture.path().join("codex");
            let script = r#"#!/bin/sh
fixture_dir=${0%/*}
head -c 262144 /dev/zero | tr '\000' ' ' REDIRECT
printf '\n'
cat > "$fixture_dir/stdin"
printf '%s\n' '{"type":"item.completed","item":{"type":"agent_message","text":"{\"content\":\"OK\"}"}}'
printf '%s\n' '{"type":"turn.completed","usage":{"input_tokens":12,"output_tokens":3}}'
"#
            .replace("REDIRECT", redirect);
            crate::test_support::write_executable_script(&executable, &script);
            let provider = CodexCliProvider {
                model: "test-model".to_string(),
                effort: None,
                timeout_seconds: 5,
            };
            let request = AiRequest {
                system: Some("x".repeat(1024 * 1024)),
                ..Default::default()
            };
            let expected_prompt = build_prompt(&request);
            let response = provider
                .generate_content_with_binary(request, executable.as_os_str())
                .await
                .unwrap();
            assert_eq!(response.content.as_deref(), Some("OK"));
            assert_eq!(
                std::fs::read_to_string(fixture.path().join("stdin")).unwrap(),
                expected_prompt
            );
        }
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn early_exit_preserves_failure_and_rejects_incomplete_prompt() {
        for (exit_code, expected) in [
            (0, "failed to write prompt"),
            (1, "provider rejected token=[REDACTED]"),
        ] {
            let fixture = tempfile::tempdir().unwrap();
            let executable = fixture.path().join("codex");
            crate::test_support::write_executable_script(
                &executable,
                &format!(
                    "#!/bin/sh\nprintf '%s\\n' '{{\"type\":\"turn.failed\",\"error\":{{\"message\":\"provider rejected token=tok-123\"}}}}'\nexit {exit_code}\n"
                ),
            );
            let provider = CodexCliProvider {
                model: "test-model".to_string(),
                effort: None,
                timeout_seconds: 5,
            };
            let error = provider
                .generate_content_with_binary(
                    AiRequest {
                        system: Some("x".repeat(1024 * 1024)),
                        ..Default::default()
                    },
                    executable.as_os_str(),
                )
                .await
                .unwrap_err()
                .to_string();
            assert!(error.contains(expected), "{error}");
            assert!(!error.contains("tok-123"), "{error}");
        }
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn completion_cancellation_terminates_child_during_prompt_transfer() -> Result<()> {
        let fixture = tempfile::tempdir()?;
        let executable = fixture.path().join("codex");
        let pid_file = fixture.path().join("pid");
        crate::test_support::write_executable_script(
            &executable,
            r#"#!/bin/sh
fixture_dir=${0%/*}
printf '%s\n' "$$" > "$fixture_dir/pid"
while :; do :; done
"#,
        );
        let task = tokio::spawn(async move {
            CodexCliProvider {
                model: "test-model".to_string(),
                effort: None,
                timeout_seconds: 20,
            }
            .generate_content_with_binary(
                AiRequest {
                    system: Some("x".repeat(1024 * 1024)),
                    ..Default::default()
                },
                executable.as_os_str(),
            )
            .await
        });
        let pid = timeout(Duration::from_secs(2), async {
            loop {
                if let Ok(pid) = std::fs::read_to_string(&pid_file)
                    && pid.trim().parse::<u32>().is_ok()
                {
                    break pid;
                }
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
        .await?;
        let process = std::path::PathBuf::from(format!("/proc/{}", pid.trim()));
        assert!(process.exists());
        task.abort();
        assert!(task.await.unwrap_err().is_cancelled());
        timeout(Duration::from_secs(2), async {
            while process.exists() {
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
        .await?;
        Ok(())
    }

    #[tokio::test]
    async fn configured_timeout_includes_blocked_prompt_transfer() {
        let fixture = tempfile::tempdir().unwrap();
        let executable = fixture.path().join("codex");
        crate::test_support::write_executable_script(
            &executable,
            "#!/bin/sh\nwhile :; do :; done\n",
        );
        let provider = CodexCliProvider {
            model: "test-model".to_string(),
            effort: None,
            timeout_seconds: 1,
        };
        let error = provider
            .generate_content_with_binary(
                AiRequest {
                    system: Some("x".repeat(1024 * 1024)),
                    ..Default::default()
                },
                executable.as_os_str(),
            )
            .await
            .unwrap_err();
        assert!(error.to_string().contains("timed out after 1 seconds"));
    }

    #[test]
    fn only_the_approval_policy_substitution_is_demoted() {
        assert!(is_approval_policy_substitution(
            "Configured value for `approval_policy` is disallowed by \
             requirements; falling back to required value OnRequest."
        ));
        assert!(!is_approval_policy_substitution(
            "Configured value for `model_reasoning_effort` is disallowed by \
             requirements; falling back to required value medium."
        ));
    }

    #[test]
    fn a_substituted_effort_fails_the_run() {
        let message = "Configured value for `model_reasoning_effort` is disallowed by \
                       requirements; falling back to required value medium.";
        let raised = CodexCliProvider {
            model: "gpt-5-codex".to_string(),
            effort: Some("xhigh".to_string()),
            timeout_seconds: 600,
        };
        assert!(raised.check_error_message(message).is_err());

        // With no effort configured the identity carries none, so the
        // substituted run is recorded under the identity it ran at.
        let plain = CodexCliProvider {
            model: "gpt-5-codex".to_string(),
            effort: None,
            timeout_seconds: 600,
        };
        assert!(plain.check_error_message(message).is_ok());
        assert!(
            raised
                .check_error_message(
                    "Configured value for `approval_policy` is disallowed by \
                     requirements; falling back to required value OnRequest."
                )
                .is_ok()
        );
    }

    #[test]
    fn error_messages_include_failed_turns() {
        let raw = concat!(
            r#"{"type":"session.created"}"#,
            "\n",
            r#"{"type":"error","message":"[reasoning.effort] Invalid value: 'higest'"}"#,
            "\n",
            r#"{"type":"item.completed","item":{"type":"error","message":"disallowed by requirements"}}"#,
            "\n",
            r#"{"type":"turn.failed","error":{"message":"provider rejected the turn"}}"#,
            "\n",
            r#"not json"#,
            "\n",
        );
        assert_eq!(
            collect_error_messages(raw),
            vec![
                "[reasoning.effort] Invalid value: 'higest'".to_string(),
                "disallowed by requirements".to_string(),
                "provider rejected the turn".to_string(),
            ]
        );
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn failed_turn_is_never_returned_as_success() {
        for (exit_code, failure, expected) in [
            (
                0,
                r#"{"type":"turn.failed","error":{"message":"provider rejected token=tok-123"}}"#,
                "provider rejected token=[REDACTED]",
            ),
            (
                1,
                r#"{"type":"turn.failed","error":{"message":"provider rejected token=tok-123"}}"#,
                "provider rejected token=[REDACTED]",
            ),
            (0, r#"{"type":"turn.failed"}"#, "No failure detail reported"),
            (1, r#"{"type":"turn.failed"}"#, "No failure detail reported"),
        ] {
            let fixture = tempfile::tempdir().unwrap();
            let executable = fixture.path().join("codex");
            crate::test_support::write_executable_script(
                &executable,
                &format!(
                    "#!/bin/sh\ncat >/dev/null\nprintf '%s\\n' '{{\"type\":\"item.completed\",\"item\":{{\"text\":\"{{\\\"content\\\":\\\"OK\\\"}}\"}}}}'\nprintf '%s\\n' '{failure}'\nexit {exit_code}\n"
                ),
            );
            let provider = CodexCliProvider {
                model: "test-model".to_string(),
                effort: None,
                timeout_seconds: 10,
            };
            let error = provider
                .generate_content_with_binary(AiRequest::default(), executable.as_os_str())
                .await
                .unwrap_err()
                .to_string();
            assert!(error.contains(expected), "{error}");
            assert!(!error.contains("tok-123"), "{error}");
        }
    }

    #[test]
    fn error_messages_redact_secrets_in_json_and_check_error() {
        let raw = concat!(
            r#"{"type":"error","message":"auth failed for https://user:pass@host?key=sk-secret token=tok-123"}"#,
            "\n",
            r#"{"type":"item.completed","item":{"type":"error","message":"auth failed for https://user:pass@host?key=sk-secret token=tok-999"}}"#,
            "\n",
        );
        let messages = collect_error_messages(raw);
        assert_eq!(
            messages,
            vec![
                "auth failed for https://[REDACTED]:[REDACTED]@host?key=[REDACTED] token=[REDACTED]"
                    .to_string(),
            ]
        );

        let raised = CodexCliProvider {
            model: "gpt-5-codex".to_string(),
            effort: Some("xhigh".to_string()),
            timeout_seconds: 600,
        };
        let err = raised
            .check_error_message(
                "Configured value for `model_reasoning_effort` rejected at https://user:pass@host key=sk-secret token=tok-123",
            )
            .unwrap_err()
            .to_string();
        assert!(err.contains("https://[REDACTED]:[REDACTED]@host"));
        assert!(err.contains("key=[REDACTED]"));
        assert!(err.contains("token=[REDACTED]"));
        assert!(!err.contains("sk-secret"));
        assert!(!err.contains("tok-123"));
        assert!(!err.contains("user:pass"));
    }

    #[test]
    fn managed_substitution_cannot_silently_relax_completion_profile() {
        let provider = CodexCliProvider {
            model: "test-model".to_string(),
            effort: None,
            timeout_seconds: 600,
        };
        assert!(provider.check_error_message(
            "Configured value for `features.shell_tool` is disallowed by requirements; falling back to required value true."
        ).is_err());
    }

    #[test]
    fn cache_identity_tracks_effort() {
        let plain = CodexCliProvider {
            model: "gpt-5-codex".to_string(),
            effort: None,
            timeout_seconds: 600,
        };
        let raised = CodexCliProvider {
            model: "gpt-5-codex".to_string(),
            effort: Some("xhigh".to_string()),
            timeout_seconds: 600,
        };
        assert_ne!(plain.cache_identity(), raised.cache_identity());
        assert_ne!(
            plain.cache_identity(),
            cache_identity_with(
                &plain.model,
                &[
                    ("effort", None),
                    ("completion_profile", Some("restricted-v1")),
                    ("usage_profile", Some("measured-v1")),
                ],
            )
        );
    }
}
