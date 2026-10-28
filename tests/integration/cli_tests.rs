use std::env;
use std::path::PathBuf;
use std::process::Command;

fn get_bin_path() -> PathBuf {
    if let Ok(path) = env::var("CARGO_BIN_EXE_sashiko") {
        return PathBuf::from(path);
    }

    // Fallback for local run
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("target");
    // We can't easily know if the binary was built with release or debug from here
    // if we are running under cargo test. But we can check which one exists.
    let release_path = path.join("release").join("sashiko");
    let debug_path = path.join("debug").join("sashiko");

    // If both exist, choose the most recently modified binary, preferring debug.
    if debug_path.exists() && release_path.exists() {
        let debug_mtime = std::fs::metadata(&debug_path)
            .and_then(|m| m.modified())
            .ok();
        let release_mtime = std::fs::metadata(&release_path)
            .and_then(|m| m.modified())
            .ok();
        if release_mtime > debug_mtime {
            release_path
        } else {
            debug_path
        }
    } else if debug_path.exists() {
        debug_path
    } else if release_path.exists() {
        release_path
    } else {
        panic!(
            "Could not find sashiko binary in target/debug or target/release. Path: {:?}",
            path
        );
    }
}

#[test]
fn test_review_subcommand_hides_info_logs() {
    let bin_path = get_bin_path();

    let output = Command::new(&bin_path)
        .args(["review", "HEAD", "--no-ai"])
        .output()
        .expect("Failed to execute sashiko binary");

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        !stderr.contains("INFO"),
        "stderr contains INFO logs: {}",
        stderr
    );
    assert!(
        !stderr.contains("Skipping AI review"),
        "stderr contains info log message: {}",
        stderr
    );
    assert!(
        stderr.contains("Reviewing: HEAD"),
        "stderr missing 'Reviewing: HEAD': {}",
        stderr
    );
}

#[test]
fn test_review_subcommand_shows_info_logs_with_debug() {
    let bin_path = get_bin_path();

    let output = Command::new(&bin_path)
        .args(["--debug", "review", "HEAD", "--no-ai"])
        .output()
        .expect("Failed to execute sashiko binary");

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        stderr.contains("INFO"),
        "stderr missing INFO logs in debug mode: {}",
        stderr
    );
    assert!(
        stderr.contains("Skipping AI review"),
        "stderr missing info log message in debug mode: {}",
        stderr
    );
}

/// A review reads the file SASHIKO_CONFIG names, ahead of ./Settings.toml.
/// The variable is not itself taken for a setting: those are spelled
/// SASHIKO__SECTION__KEY, so a file that refuses unknown keys still loads.
#[test]
fn test_review_reads_the_file_sashiko_config_names() {
    let bin_path = get_bin_path();
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("named.toml");
    let settings = "[ai]\nprovider = \"gemini\"\nmodel = \"gemini-3-pro\"\n\n\
                    [review]\nconcurrency = 1\n";

    std::fs::write(&path, settings).unwrap();
    let output = Command::new(&bin_path)
        .args(["review", "HEAD", "--no-ai"])
        .env("SASHIKO_CONFIG", &path)
        .output()
        .expect("Failed to execute sashiko binary");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "review failed: {}", stderr);

    std::fs::write(&path, format!("{settings}mistyped = 1\n")).unwrap();
    let output = Command::new(&bin_path)
        .args(["review", "HEAD", "--no-ai"])
        .env("SASHIKO_CONFIG", &path)
        .output()
        .expect("Failed to execute sashiko binary");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !output.status.success(),
        "review ignored {}",
        path.display()
    );
    assert!(
        stderr.contains("mistyped"),
        "stderr does not name the key mistyped in that file: {}",
        stderr
    );
}

#[cfg(unix)]
#[test]
fn test_agent_harness_reviews_series_without_workflow_fallback() {
    let temp = tempfile::tempdir().unwrap();
    let repo = temp.path().join("repo");
    let bin = temp.path().join("bin");
    std::fs::create_dir_all(&repo).unwrap();
    std::fs::create_dir_all(&bin).unwrap();
    let git = |args: &[&str]| {
        let output = sashiko::git_cmd::in_dir(&repo)
            .args([
                "-c",
                "user.name=Test",
                "-c",
                "user.email=test@example.org",
                "-c",
                "commit.gpgsign=false",
            ])
            .args(args)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        output
    };
    git(&["init"]);
    for index in 0..3 {
        std::fs::write(repo.join("source.txt"), format!("revision {index}\n")).unwrap();
        git(&["add", "source.txt"]);
        git(&["commit", "-m", &format!("Revision {index}")]);
    }
    let settings = temp.path().join("Settings.toml");
    std::fs::write(
        temp.path().join("agent.md"),
        "Review source and return complete reply drafts.",
    )
    .unwrap();
    std::fs::write(&settings, "[ai]\nprovider = 'codex-cli'\nmodel = 'gpt-6-luna'\n[ai.codex_cli]\neffort = 'low'\n[review]\nconcurrency = 1\nmode = 'agent_harness'\n[review.agent_harness]\ninstruction_files = ['agent.md']\nmax_generations = 3\nmax_tool_calls = 5\nmax_input_tokens = 10000\nmax_output_tokens = 2000\ntimeout_seconds = 10\n").unwrap();
    let source = bin.join("codex.source");
    std::fs::write(&source, "#!/bin/sh\nfixture=${0%/*}\nprintf 'call\\n' >> \"$fixture/calls\"\ncat > \"$fixture/input\"\ncat \"$fixture/response\"\n").unwrap();
    assert!(
        Command::new("install")
            .args(["-m", "755"])
            .arg(&source)
            .arg(bin.join("codex"))
            .status()
            .unwrap()
            .success()
    );
    let report = serde_json::json!({"reviewer_notes":"Series investigation.", "reply_drafts":[
        {"patch_index":1, "message_id":null, "body":"Reply one."},
        {"patch_index":2, "message_id":null, "body":"Reply two."}
    ]});
    let response = |content: &str| {
        format!(
            "{}\n{}\n{}\n",
            serde_json::json!({"type":"item.completed", "item":{"type":"reasoning", "text":"internal reasoning"}}),
            serde_json::json!({"type":"item.completed", "item":{"type":"agent_message", "text":serde_json::json!({"content":content}).to_string()}}),
            serde_json::json!({"type":"turn.completed", "usage":{"input_tokens":30, "output_tokens":10, "cached_input_tokens":20}})
        )
    };
    std::fs::write(bin.join("response"), response(&report.to_string())).unwrap();
    let path = env::join_paths(
        std::iter::once(bin.clone()).chain(env::split_paths(&env::var_os("PATH").unwrap())),
    )
    .unwrap();
    let run = |extra: &[&str]| {
        Command::new(get_bin_path())
            .current_dir(&repo)
            .env("SASHIKO_CONFIG", &settings)
            .env("PATH", &path)
            .args(["--project", "xen", "review", "HEAD~2..HEAD"])
            .args(extra)
            .output()
            .unwrap()
    };
    let no_ai = run(&["--no-ai", "--agent"]);
    assert!(
        no_ai.status.success(),
        "{}",
        String::from_utf8_lossy(&no_ai.stderr)
    );
    assert!(!bin.join("calls").exists());
    let output = run(&["--agent"]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let result: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        result["status"],
        "completed_without_independent_verification"
    );
    assert_eq!(
        result["artifacts"]["reply_drafts"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    assert_eq!(result["provider_calls"], 1);
    assert_eq!(result["tokens_cached"], 20);
    assert_eq!(result["model"], "gpt-6-luna");
    assert_eq!(result["effort"], "low");
    assert_eq!(
        std::fs::read_to_string(bin.join("calls")).unwrap(),
        "call\n"
    );
    let captured = std::fs::read_to_string(bin.join("input")).unwrap();
    let baseline = git(&["rev-parse", "HEAD~2"]);
    assert!(captured.contains(String::from_utf8_lossy(&baseline.stdout).trim()));
    assert!(captured.contains("whole-series-review-v1"));
    std::fs::write(bin.join("response"), response("Malformed report")).unwrap();
    let failed = run(&["--format", "json"]);
    assert_eq!(
        failed.status.code(),
        Some(3),
        "{}",
        String::from_utf8_lossy(&failed.stderr)
    );
    let result: serde_json::Value = serde_json::from_slice(&failed.stdout).unwrap();
    assert_eq!(result["harness"]["status"]["status"], "failed");
    assert_eq!(result["harness"]["trace"]["provider_calls"], 1);
    assert_eq!(result["review_attribution"]["response_cache"], false);
    assert_eq!(
        result["review_attribution"]["selected_patch_indexes"],
        serde_json::json!([1, 2])
    );
    assert_eq!(
        result["review_attribution"]["instruction_snapshot"]["files"][0]["body"],
        "Review source and return complete reply drafts."
    );
    assert_eq!(
        result["harness"]["trace"]["history"][1]["content"],
        "Malformed report"
    );
    assert_eq!(
        std::fs::read_to_string(bin.join("calls")).unwrap(),
        "call\ncall\n"
    );
    let configuration = std::fs::read_to_string(&settings).unwrap();
    std::fs::write(
        &settings,
        format!("{configuration}strategy = 'sequential_patches'\n"),
    )
    .unwrap();
    std::fs::remove_file(bin.join("calls")).unwrap();
    let no_ai = run(&["--no-ai", "--agent"]);
    assert!(
        no_ai.status.success(),
        "{}",
        String::from_utf8_lossy(&no_ai.stderr)
    );
    assert!(!bin.join("calls").exists());
    std::fs::write(&source, "#!/bin/sh\nfixture=${0%/*}\nprintf 'call\\n' >> \"$fixture/calls\"\nnumber=$(wc -l < \"$fixture/calls\")\ncat > \"$fixture/input.$number\"\ncat \"$fixture/response.$number\"\n").unwrap();
    assert!(
        Command::new("install")
            .args(["-m", "755"])
            .arg(&source)
            .arg(bin.join("codex"))
            .status()
            .unwrap()
            .success()
    );
    let first_report = serde_json::json!({"reviewer_notes":"First checkpoint investigation.", "reply_drafts":[{"patch_index":1,"message_id":null,"body":"Reply one."}]}).to_string();
    let second_report = serde_json::json!({"reviewer_notes":"Second checkpoint investigation.", "reply_drafts":[{"patch_index":2,"message_id":null,"body":"Reply two."}]}).to_string();
    std::fs::write(bin.join("response.1"), response(&first_report)).unwrap();
    for (report, complete) in [
        (&second_report, true),
        (&"Malformed checkpoint".to_string(), false),
    ] {
        std::fs::write(bin.join("response.2"), response(report)).unwrap();
        for extra in [vec!["--format", "json"], vec!["--agent"], vec![]] {
            std::fs::write(bin.join("calls"), "").unwrap();
            let output = run(&extra);
            assert_eq!(
                output.status.code(),
                Some(if complete { 0 } else { 3 }),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
            if extra.is_empty() {
                let human = String::from_utf8(output.stdout).unwrap();
                assert!(human.contains("not independently verified"));
                assert!(human.contains("Reply one."));
                assert_eq!(human.contains("Reply two."), complete);
            } else {
                let result: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
                assert_eq!(result["review_strategy"], "sequential_patches");
                assert_eq!(result["partial"], !complete);
                assert_eq!(
                    result["checkpoint_session"]["completed_count"],
                    if complete { 2 } else { 1 }
                );
                assert_eq!(
                    result["checkpoint_session"]["remaining_patch_indexes"],
                    if complete {
                        serde_json::json!([])
                    } else {
                        serde_json::json!([2])
                    }
                );
                assert_eq!(result["checkpoint_session"]["provider_calls_recorded"], 2);
                assert_eq!(result["checkpoint_session"]["provider_calls_attempted"], 2);
                assert_eq!(result["checkpoint_session"]["usage_complete"], true);
                assert_eq!(result["artifacts"]["reply_drafts"][0]["body"], "Reply one.");
                assert_eq!(result["tokens_in"], 60);
                assert_eq!(result["tokens_cached"], 40);
                assert_eq!(result["independently_verified"], false);
            }
            assert_eq!(
                std::fs::read_to_string(bin.join("calls")).unwrap(),
                "call\ncall\n"
            );
            let first_prompt = std::fs::read_to_string(bin.join("input.1")).unwrap();
            let second_prompt = std::fs::read_to_string(bin.join("input.2")).unwrap();
            assert!(second_prompt.starts_with(&first_prompt));
            assert!(second_prompt.contains("First checkpoint investigation."));
            assert!(second_prompt.contains("sequential-series-context-v1"));
        }
    }
}
