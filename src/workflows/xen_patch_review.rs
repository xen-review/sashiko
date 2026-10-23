use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::workflow::{
    OutputFormat, ParallelPolicy, PromptTemplate, Stage, StagePolicy, ToolScope, Workflow,
};
use crate::workflows::linux_patch_review::{
    AnalysisStage, LinuxPatchReviewState, StageConcernsOutput, VerificationOutput,
    append_stage_dismissed_concerns_with_prompts, append_stage_items_with_prompts,
    apply_verification_stage_output, has_valid_proof_location, validate_locations_side,
};

pub static ANALYSIS_STAGES: &[AnalysisStage] = &[
    AnalysisStage {
        name: "implementation",
        short: "Implementation",
        instruction: "Trace every changed function against its stated intent and callers. Check bounds, arithmetic, API contracts, error handling, architecture/configuration differences and partial failure cleanup.",
        guides: &["review-core.md"],
        uses_commit_log: true,
        optional: false,
        wants_series_context: true,
    },
    AnalysisStage {
        name: "concurrency",
        short: "Concurrency",
        instruction: "Audit domain/vCPU and object lifetime, references, teardown, RCU, locks, interrupt context and memory ordering. Prove the interleaving and show the relevant acquisition and release paths.",
        guides: &["review-core.md"],
        uses_commit_log: true,
        optional: false,
        wants_series_context: true,
    },
    AnalysisStage {
        name: "security",
        short: "Trust Boundaries",
        instruction: "Trace guest and device-controlled data through hypercalls, mappings, grant tables, event channels and toolstack interfaces. Verify validation and privilege checks in actual callers. State the attacker's privileges and reachable trigger; do not assume Linux contracts.",
        guides: &["review-core.md"],
        uses_commit_log: true,
        optional: false,
        wants_series_context: true,
    },
];

pub fn analysis_stage_by_name(name: &str) -> Option<&'static AnalysisStage> {
    ANALYSIS_STAGES.iter().find(|stage| stage.name == name)
}

pub(crate) fn validate_stage_selection(stages: Option<&[String]>) -> Result<(), String> {
    if let Some(stages) = stages {
        if stages.is_empty() {
            return Err("Xen reviews require at least one analysis stage".to_string());
        }
        for name in stages {
            if analysis_stage_by_name(name).is_none() {
                return Err(format!(
                    "Unknown Xen analysis stage {name:?}; expected implementation, concurrency or security"
                ));
            }
        }
    }
    Ok(())
}

pub fn stage_short_label(name: &str) -> Option<&'static str> {
    analysis_stage_by_name(name)
        .map(|stage| stage.short)
        .or(match name {
            "verification" => Some("Verification"),
            "report" => Some("Report"),
            _ => None,
        })
}

pub fn xen_system_prompt(_use_log: bool) -> PromptTemplate<LinuxPatchReviewState> {
    PromptTemplate::new(
        "You are an expert Xen hypervisor and toolstack maintainer performing a source-grounded patch review.\n@includes\nTarget: {{target}}\nParent: {{parent}}\nPatch:\n{{diff}}\nSource context:\n{{context}}\nSeries context:\n{{series}}\nOperator guidance:\n{{custom}}",
    )
    .include_file("review-core.md")
    .include_file("severity.md")
    .with_var("target", |state: &LinuxPatchReviewState| state.target_commit_sha.clone())
    .with_var("parent", |state: &LinuxPatchReviewState| state.baseline_sha.clone())
    .with_var("diff", |state: &LinuxPatchReviewState| state.target_commit_diff.clone())
    .with_var("context", |state: &LinuxPatchReviewState| state.prefetched_context.clone())
    .with_var("series", |state: &LinuxPatchReviewState| state.follow_up_series_context.clone().unwrap_or_default())
    .with_var("custom", |state: &LinuxPatchReviewState| state.custom_prompt.clone().unwrap_or_default())
}

fn validate_analysis(
    output: &StageConcernsOutput,
    _state: &LinuxPatchReviewState,
) -> Result<(), String> {
    for (collection, items) in [
        ("concerns", &output.concerns),
        ("dismissed_concerns", &output.dismissed_concerns),
    ] {
        for (index, item) in items.iter().enumerate() {
            for field in ["description", "reasoning"] {
                if item
                    .get(field)
                    .and_then(Value::as_str)
                    .is_none_or(|text| text.trim().is_empty())
                {
                    return Err(format!(
                        "{collection}[{index}].{field} must be a non-empty string"
                    ));
                }
            }
            if collection == "concerns" && !item.get("preexisting").is_some_and(Value::is_boolean) {
                return Err(format!(
                    "{collection}[{index}].preexisting must be a boolean"
                ));
            }
            let locations = item
                .get("locations")
                .and_then(Value::as_array)
                .ok_or_else(|| format!("{collection}[{index}].locations must be an array"))?;
            validate_locations_side(locations, collection, index)?;
            if !has_valid_proof_location(item) {
                return Err(format!(
                    "{collection}[{index}] needs a file, symbol, side and verbatim code_snippet proving the claim"
                ));
            }
        }
    }
    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
struct VerifiedOutput {
    findings: Vec<Value>,
    dismissed_concerns: Vec<Value>,
}

fn validate_verified(output: &VerifiedOutput, state: &LinuxPatchReviewState) -> Result<(), String> {
    let inputs: Vec<&Value> = state
        .all_concerns
        .iter()
        .chain(&state.all_dismissed_concerns)
        .collect();
    let mut covered = std::collections::BTreeSet::new();
    for (collection, items) in [
        ("findings", &output.findings),
        ("dismissed_concerns", &output.dismissed_concerns),
    ] {
        for (index, item) in items.iter().enumerate() {
            if !has_valid_proof_location(item) {
                return Err(format!(
                    "{collection}[{index}] must include concrete proving locations"
                ));
            }
            let locations = item["locations"]
                .as_array()
                .ok_or("locations must be an array")?;
            validate_locations_side(locations, collection, index)?;
            let sources = item
                .get("source_ids")
                .and_then(Value::as_array)
                .filter(|ids| !ids.is_empty())
                .ok_or_else(|| format!("{collection}[{index}].source_ids must name input IDs"))?;
            for source in sources {
                let source = source
                    .as_str()
                    .ok_or("source_ids entries must be strings")?;
                if !inputs
                    .iter()
                    .any(|input| input["id"].as_str() == Some(source))
                {
                    return Err(format!("unknown source ID {source}"));
                }
                covered.insert(source.to_string());
            }
            let fields: &[&str] = if collection == "findings" {
                &["problem", "severity_explanation"]
            } else {
                &["description", "reasoning"]
            };
            for field in fields {
                if item
                    .get(*field)
                    .and_then(Value::as_str)
                    .is_none_or(|text| text.trim().is_empty())
                {
                    return Err(format!("{collection}[{index}].{field} must be non-empty"));
                }
            }
            if collection == "findings" {
                if !item.get("preexisting").is_some_and(Value::is_boolean) {
                    return Err("findings require a boolean preexisting".to_string());
                }
                if !matches!(
                    item["severity"].as_str(),
                    Some("Low" | "Medium" | "High" | "Critical" | "Unknown")
                ) {
                    return Err(
                        "severity must be Low, Medium, High, Critical or Unknown".to_string()
                    );
                }
            }
        }
    }
    for input in inputs {
        let id = input["id"].as_str().ok_or("input missing ID")?;
        if !covered.contains(id) {
            return Err(format!(
                "input {id} must be verified or explicitly dismissed"
            ));
        }
    }
    Ok(())
}

fn record_analysis_output(
    state: &mut LinuxPatchReviewState,
    mut output: StageConcernsOutput,
    stage: &str,
) {
    for item in output
        .concerns
        .iter_mut()
        .chain(&mut output.dismissed_concerns)
    {
        if let Some(fields) = item.as_object_mut() {
            fields.remove("id");
        }
    }
    let prompts = ["review-core.md".to_string()];
    append_stage_items_with_prompts(
        &mut state.all_concerns,
        &output.concerns,
        stage,
        "General",
        &prompts,
    );
    append_stage_dismissed_concerns_with_prompts(
        &mut state.all_dismissed_concerns,
        &output.dismissed_concerns,
        stage,
        &prompts,
    );
}

fn validate_report(text: &str, state: &LinuxPatchReviewState) -> Result<(), String> {
    if !state.findings.is_empty() && text.trim().is_empty() {
        return Err(
            "A review with verified findings requires a non-empty report describing those findings"
                .to_string(),
        );
    }
    Ok(())
}

pub fn build_xen_patch_review_workflow_with_effort(
    max_turns: usize,
    temperature: f32,
    effort: Option<crate::ai::EffortMode>,
) -> Workflow<LinuxPatchReviewState> {
    let stages = ANALYSIS_STAGES.iter().map(|definition| {
        let stage = Stage::builder(definition.name)
            .system_prompt(xen_system_prompt(true))
            .user_prompt(PromptTemplate::new(format!("{}\nReturn only JSON with concerns and dismissed_concerns arrays. Each item needs type, description, reasoning and locations. Concerns also require a boolean preexisting. Locations require file, function_or_symbol, line, code_snippet, side (PARENT or REVISION), and why_this_location_matters. Use empty arrays when no issues exist.", definition.instruction)))
            .output_format(OutputFormat::json().with_validator(validate_analysis))
            .policy(StagePolicy { tools: ToolScope::All, max_turns, temperature, reasoning_effort: effort.map(crate::ai::EffortMode::verification_stage_effort), ..Default::default() })
            .skip_if(move |state: &LinuxPatchReviewState| state.manual_stages.as_ref().is_some_and(|names| !names.iter().any(|name| name == definition.name)))
            .reduce(move |state: &mut LinuxPatchReviewState, output: StageConcernsOutput| {
                record_analysis_output(state, output, definition.name);
            }).build();
        Box::new(stage) as Box<dyn crate::workflow::ExecutableStage<LinuxPatchReviewState>>
    }).collect();
    let verification = Stage::builder("verification")
        .system_prompt(xen_system_prompt(true))
        .user_prompt(PromptTemplate::new("Independently verify every input using Git tools at explicit revisions. Trace callers and triggers. Deduplicate overlapping issues. Revisit dismissals: never assume an absent check exists. Return only JSON with findings and dismissed_concerns arrays. Every input ID must appear in a non-empty source_ids array. Findings require problem, severity (Low/Medium/High/Critical/Unknown), severity_explanation with source proof and reachable trigger, preexisting (boolean), and locations. Dismissals require description, reasoning citing disproving code, and locations. Every location requires file, function_or_symbol, line, code_snippet, side (PARENT/REVISION), and why_this_location_matters. Do not silently discard unresolved candidates; use source tools to resolve them.\nConcerns: {{concerns}}\nDismissals: {{dismissals}}")
            .with_var("concerns", |state: &LinuxPatchReviewState| json!(state.all_concerns).to_string())
            .with_var("dismissals", |state: &LinuxPatchReviewState| json!(state.all_dismissed_concerns).to_string()))
        .output_format(OutputFormat::json().with_validator(validate_verified))
        .policy(StagePolicy { tools: ToolScope::All, max_turns, temperature, reasoning_effort: effort.map(crate::ai::EffortMode::verification_stage_effort), ..Default::default() })
        .skip_if(|state: &LinuxPatchReviewState| state.all_concerns.is_empty() && state.all_dismissed_concerns.is_empty())
        .reduce_with_outcome(|state: &mut LinuxPatchReviewState, output: VerifiedOutput, outcome| {
            apply_verification_stage_output(state, VerificationOutput {
                findings: output.findings,
                hard_cases: Vec::new(),
                dismissed_concerns: output.dismissed_concerns,
            }, outcome, analysis_stage_by_name);
        }).build();
    let report = Stage::builder("report")
        .system_prompt(xen_system_prompt(true))
        .user_prompt(PromptTemplate::new("Write a concise plain-text review for xen-devel using ONLY the verified findings below. Include each finding's severity, file and line, reachable trigger, impact and source proof. Do not add new claims.\n{{findings}}")
            .with_var("findings", |state: &LinuxPatchReviewState| json!(state.findings).to_string()))
        .output_format(OutputFormat::text_with_validator(validate_report, |error| format!("The report was rejected: {error}. Return a non-empty plain-text report of the verified findings.")))
        .policy(StagePolicy { tools: ToolScope::None, max_turns: 1, temperature, ..Default::default() })
        .skip_if(|state: &LinuxPatchReviewState| state.skip_report || state.findings.is_empty())
        .reduce(|state: &mut LinuxPatchReviewState, output: String| state.review_inline = output)
        .build();
    Workflow::builder("xen_patch_review")
        .parallel(stages, ParallelPolicy::FailFast)
        .stage(verification)
        .stage(report)
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xen_workflow_has_own_stages_and_prompts() {
        assert_eq!(
            build_xen_patch_review_workflow_with_effort(10, 0.0, None).name,
            "xen_patch_review"
        );
        assert!(stage_short_label("security").is_some());
        let rendered = xen_system_prompt(true).render_for_log(&LinuxPatchReviewState::default());
        assert!(rendered.contains("Xen hypervisor"));
        assert!(!rendered.contains("principal Rust"));
    }

    #[test]
    fn verification_cannot_drop_candidates() {
        let state = LinuxPatchReviewState {
            all_concerns: vec![json!({"id": "C1"})],
            ..Default::default()
        };
        let output = VerifiedOutput {
            findings: vec![],
            dismissed_concerns: vec![],
        };
        assert!(
            validate_verified(&output, &state)
                .unwrap_err()
                .contains("C1")
        );
    }

    #[test]
    fn model_supplied_ids_cannot_collide_across_candidates() {
        let mut state = LinuxPatchReviewState::default();
        for stage in ["implementation", "security"] {
            record_analysis_output(
                &mut state,
                StageConcernsOutput {
                    concerns: vec![json!({"id": "C1", "description": stage})],
                    dismissed_concerns: vec![json!({"id": "C1", "description": stage})],
                },
                stage,
            );
        }
        assert_eq!(
            state
                .all_concerns
                .iter()
                .map(|item| item["id"].as_str().unwrap())
                .collect::<Vec<_>>(),
            ["C1", "C2"]
        );
        assert_eq!(
            state
                .all_dismissed_concerns
                .iter()
                .map(|item| item["id"].as_str().unwrap())
                .collect::<Vec<_>>(),
            ["D1", "D2"]
        );
    }

    #[test]
    fn verified_findings_cannot_produce_an_empty_report() {
        let state = LinuxPatchReviewState {
            findings: vec![json!({"problem": "example"})],
            ..Default::default()
        };
        assert!(validate_report(" \n", &state).is_err());
        assert!(validate_report("common: example", &state).is_ok());
    }

    #[test]
    fn xen_manual_stage_selection_rejects_empty_and_unknown_names() {
        assert!(validate_stage_selection(None).is_ok());
        assert!(validate_stage_selection(Some(&[])).is_err());
        assert!(validate_stage_selection(Some(&["implementaton".to_string()])).is_err());
        assert!(
            validate_stage_selection(Some(&["implementation".to_string(), "unknown".to_string()]))
                .is_err()
        );
        assert!(validate_stage_selection(Some(&["implementation".to_string()])).is_ok());
    }

    #[test]
    fn analysis_rejects_unproven_claims() {
        let output = StageConcernsOutput {
            concerns: vec![
                json!({"description": "test", "reasoning": "test", "preexisting": false, "locations": []}),
            ],
            ..Default::default()
        };
        assert!(validate_analysis(&output, &LinuxPatchReviewState::default()).is_err());
    }

    #[tokio::test]
    async fn xen_workflow_verifies_candidates_before_recording_findings() {
        struct Provider;

        #[async_trait::async_trait]
        impl crate::ai::AiProvider for Provider {
            async fn generate_content(
                &self,
                request: crate::ai::AiRequest,
            ) -> anyhow::Result<crate::ai::AiResponse> {
                assert!(
                    request
                        .system
                        .as_deref()
                        .unwrap_or_default()
                        .contains("Xen hypervisor")
                );
                let location = json!({"file": "xen/common/example.c", "function_or_symbol": "example", "line": 1, "code_snippet": "return value;", "side": "REVISION", "why_this_location_matters": "unchecked guest value"});
                let verifying = request.messages.iter().any(|message| {
                    message
                        .content
                        .as_deref()
                        .is_some_and(|text| text.contains("Independently verify every input"))
                });
                let content = if verifying {
                    json!({"findings": [{"source_ids": ["C1"], "problem": "common: unchecked value", "severity": "Medium", "severity_explanation": "Guest input reaches the unchecked operation.", "preexisting": false, "locations": [location]}], "dismissed_concerns": []})
                } else {
                    json!({"concerns": [{"type": "bounds", "description": "Unchecked guest value", "reasoning": "The guest controls value.", "preexisting": false, "locations": [location]}], "dismissed_concerns": []})
                };
                Ok(crate::ai::AiResponse {
                    content: Some(content.to_string()),
                    thought: None,
                    thought_signature: None,
                    tool_calls: None,
                    usage: None,
                    truncated: false,
                })
            }

            fn get_capabilities(&self) -> crate::ai::ProviderCapabilities {
                crate::ai::ProviderCapabilities {
                    model_name: "mock".to_string(),
                    context_window_size: 10000,
                }
            }
        }

        let temp = tempfile::tempdir().unwrap();
        let environment = crate::workflow::stage::WorkflowEnv {
            provider: std::sync::Arc::new(Provider),
            tools: std::sync::Arc::new(crate::toolbox::ToolBox::new(
                temp.path().to_path_buf(),
                None,
            )),
            base_dir: std::path::Path::new("projects/xen/prompts"),
            context_tag: None,
        };
        let mut state = LinuxPatchReviewState {
            project: "xen".to_string(),
            manual_stages: Some(vec!["implementation".to_string()]),
            skip_report: true,
            ..Default::default()
        };
        crate::workflow::WorkflowEngine::execute(
            &build_xen_patch_review_workflow_with_effort(2, 0.0, None),
            &environment,
            &mut state,
            None,
        )
        .await
        .unwrap();
        assert_eq!(state.all_concerns.len(), 1);
        assert_eq!(state.verification_findings.len(), 1);
        assert_eq!(state.findings.len(), 1);
        assert_eq!(state.findings[0]["stage_item_id"], "VF1");
        assert_eq!(state.project, "xen");
        assert_eq!(state.findings[0]["source_ids"], json!(["C1"]));
    }
}
