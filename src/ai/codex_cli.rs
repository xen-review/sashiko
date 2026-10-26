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

use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use std::process::Stdio;
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use tokio::time::timeout;
use tracing::{debug, warn};

use super::claude_cli::{build_prompt, log_and_redact_stderr, parse_inner_response};
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
        "tools.view_image=false".to_string(),
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
        let mut child = Command::new(binary)
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

        let execution = async move {
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(prompt.as_bytes()).await?;
                stdin.flush().await?;
            }
            child.wait_with_output().await
        };
        let output = timeout(Duration::from_secs(self.timeout_seconds), execution)
            .await
            .map_err(|_| {
                anyhow::anyhow!("codex CLI timed out after {} seconds", self.timeout_seconds)
            })?
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

        for line in raw.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            if let Ok(event) = serde_json::from_str::<Value>(trimmed) {
                if let Some(message) = error_message(&event) {
                    self.check_error_message(message)?;
                }
                match event["type"].as_str() {
                    Some("item.completed") => {
                        if let Some(text) = event["item"]["text"].as_str() {
                            text_parts.push(text.to_string());
                        }
                    }
                    Some("turn.completed") => {
                        let u = &event["usage"];
                        if !u.is_null() {
                            let input = u["input_tokens"].as_u64().unwrap_or(0) as usize;
                            let output_tokens = u["output_tokens"].as_u64().unwrap_or(0) as usize;
                            let cached = u["cached_input_tokens"].as_u64().unwrap_or(0) as usize;
                            usage = Some(AiUsage {
                                prompt_tokens: input,
                                completion_tokens: output_tokens,
                                total_tokens: input + output_tokens,
                                cached_tokens: if cached > 0 { Some(cached) } else { None },
                            });
                        }
                    }
                    _ => {}
                }
            }
        }

        let response_text = text_parts.join("\n");
        if response_text.is_empty() {
            // Fall back to raw output if no events parsed
            warn!("codex-cli: no item.completed events found, using raw output");
            return parse_inner_response(&raw, usage);
        }

        parse_inner_response(&response_text, usage)
    }
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
                ("completion_profile", Some("restricted-v1")),
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

/// Returns the explanation an error event carries. A rejected invocation
/// surfaces either as a bare `{"type":"error"}` line or as an
/// `item.completed` wrapping an error item; both carry it in `message`.
fn error_message(event: &Value) -> Option<&str> {
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
        assert_eq!(response.usage.unwrap().total_tokens, 15);
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
            "tools.view_image=false",
            "model_reasoning_effort=low",
        ] {
            assert!(arguments.windows(2).any(|pair| pair == ["-c", setting]));
        }
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
    fn error_messages_come_from_both_event_shapes() {
        let raw = concat!(
            r#"{"type":"session.created"}"#,
            "\n",
            r#"{"type":"error","message":"[reasoning.effort] Invalid value: 'higest'"}"#,
            "\n",
            r#"{"type":"item.completed","item":{"type":"error","message":"disallowed by requirements"}}"#,
            "\n",
            r#"not json"#,
            "\n",
        );
        assert_eq!(
            collect_error_messages(raw),
            vec![
                "[reasoning.effort] Invalid value: 'higest'".to_string(),
                "disallowed by requirements".to_string(),
            ]
        );
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
    }
}
