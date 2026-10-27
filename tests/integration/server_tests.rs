//! Integration tests that spin up an in-memory HTTP server on an ephemeral port
//! and exercise the REST API.

use std::net::SocketAddr;
use std::sync::Arc;

use sashiko::db::Database;
use sashiko::events::Event;
use sashiko::fetcher::FetchRequest;
use sashiko::server::{ServerOptions, build_router};
use sashiko::settings::{DatabaseSettings, Settings};
use tokio::net::TcpListener;
use tokio::sync::mpsc;

/// Build a minimal [`Settings`] for integration tests.
///
/// The `read_only` flag on `server` is set from the parameter; all other
/// fields use harmless defaults that don't require external resources.
fn test_settings(read_only: bool) -> Settings {
    let toml = format!(
        r#"
[database]
url = ":memory:"
token = ""

[nntp]
server = "localhost"
port = 119

[mailing_lists]
track = []

[ai]
provider = "gemini"
model = "test"

[server]
host = "127.0.0.1"
port = 0
read_only = {read_only}

[git]
repository_path = "."

[review]
concurrency = 1
worktree_dir = "/tmp/sashiko-test-trees"
timeout_seconds = 60
"#
    );
    let cfg = config::Config::builder()
        .add_source(config::File::from_str(&toml, config::FileFormat::Toml))
        .build()
        .expect("test settings parse");
    cfg.try_deserialize::<Settings>()
        .expect("test settings deserialize")
}

/// A running test server instance with its base URL and background handles.
struct TestServer {
    /// Base URL including the OS-assigned port, e.g. `http://127.0.0.1:12345`.
    base_url: String,
    /// Shared database handle — tests can insert fixture data directly.
    db: Arc<Database>,
    /// Event receiver — tests can drain submitted events from the channel.
    event_rx: mpsc::Receiver<Event>,
}

/// Spawn a real axum server on a random port with an in-memory database.
///
/// The server runs in a background tokio task and is dropped when the
/// [`TestServer`] goes out of scope (the task is detached, so cleanup is
/// automatic when the tokio runtime shuts down).
async fn spawn_test_server(read_only: bool) -> TestServer {
    spawn_test_server_with_settings(test_settings(read_only)).await
}

async fn spawn_test_server_with_settings(settings: Settings) -> TestServer {
    let db_settings = DatabaseSettings {
        url: ":memory:".to_string(),
        token: String::new(),
    };
    let db = Arc::new(Database::new(&db_settings).await.unwrap());
    db.migrate().await.unwrap();

    let (event_tx, event_rx) = mpsc::channel::<Event>(100);
    let (fetch_tx, _fetch_rx) = mpsc::channel::<FetchRequest>(100);

    let settings = Arc::new(settings);
    let app = build_router(
        settings,
        Arc::clone(&db),
        event_tx,
        fetch_tx,
        ServerOptions {
            allow_all_submit: true,
            dry_run: true,
            ..Default::default()
        },
    );

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr: SocketAddr = listener.local_addr().unwrap();
    let base_url = format!("http://{addr}");

    tokio::spawn(async move {
        axum::serve(
            listener,
            app.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await
        .unwrap();
    });

    TestServer {
        base_url,
        db,
        event_rx,
    }
}

// ── Smoke Tests ─────────────────────────────────────────────────────────

#[tokio::test]
async fn test_empty_db_smoke_endpoints() {
    let server = spawn_test_server(false).await;
    let client = reqwest::Client::new();

    let resp = client
        .get(format!("{}/api/stats", server.base_url))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["status"], "ok");
    assert!(body["version"].is_string());

    for endpoint in ["patchsets", "messages"] {
        let resp = client
            .get(format!("{}/api/{endpoint}", server.base_url))
            .send()
            .await
            .unwrap();
        assert_eq!(resp.status(), 200);
        let body: serde_json::Value = resp.json().await.unwrap();
        assert_eq!(body["total"], 0);
        assert!(body["items"].as_array().unwrap().is_empty());
    }

    let resp = client
        .get(format!("{}/api/lists", server.base_url))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(body.as_array().unwrap().is_empty());
}

// ── Submit / Inject Tests ───────────────────────────────────────────────

/// A minimal mbox-formatted kernel patch for testing ingestion.
const SAMPLE_MBOX: &str = "\
From dummy@example.com Thu May 14 12:00:00 2026
From: Test Author <test@example.com>
Date: Thu, 14 May 2026 12:00:00 +0000
Subject: [PATCH] mm/slub: fix object count in partial slab
Message-Id: <test-integration-1@example.com>

Fix an off-by-one in the partial slab object count that could lead
to an incorrect freelist walk under memory pressure.

---
 mm/slub.c | 2 +-
 1 file changed, 1 insertion(+), 1 deletion(-)

diff --git a/mm/slub.c b/mm/slub.c
index 1a2b3c4d5e6f..7a8b9c0d1e2f 100644
--- a/mm/slub.c
+++ b/mm/slub.c
@@ -100,7 +100,7 @@ static int count_partial_objects(struct kmem_cache_node *n)
 \tstruct slab *slab;
 \tint count = 0;
 
-\tlist_for_each_entry(slab, &n->partial, slab_list)
+\tlist_for_each_entry(slab, &n->partial, slab_list) {
 \t\tcount += slab->objects - slab->inuse;
 \t}
 
-- 
2.40.0
";

#[tokio::test]
async fn test_submit_inject_validation_and_acceptance() {
    let mut server = spawn_test_server(false).await;
    let client = reqwest::Client::new();

    // 1. Reject payloads without a valid mbox header.
    let resp = client
        .post(format!("{}/api/submit", server.base_url))
        .json(&serde_json::json!({
            "type": "inject",
            "raw": "this is not an mbox",
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 400);

    // 2. Reject baselines that are not 40-hex-digit commit IDs.
    for baseline in [
        "v6.7-rc2",
        "origin/master",
        "1234567",
        "--upload-pack=sh",
        "1234567890ABCDEF1234567890ABCDEF12345678",
        "1234567890abcdef1234567890abcdef12345678\nbase-commit: x",
    ] {
        let resp = client
            .post(format!("{}/api/submit", server.base_url))
            .json(&serde_json::json!({
                "type": "inject",
                "raw": SAMPLE_MBOX,
                "base_commit": baseline,
            }))
            .send()
            .await
            .unwrap();
        assert_eq!(resp.status(), 400, "baseline {baseline:?} was accepted");
    }

    // 3. Accept a valid inject request and enqueue Event::RawMboxSubmitted.
    let resp = client
        .post(format!("{}/api/submit", server.base_url))
        .json(&serde_json::json!({
            "type": "inject",
            "raw": SAMPLE_MBOX,
            "base_commit": "1234567890abcdef1234567890abcdef12345678",
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["status"], "accepted");

    let event = server
        .event_rx
        .try_recv()
        .expect("expected an event on the channel");
    match event {
        Event::RawMboxSubmitted { raw, baseline, .. } => {
            assert!(raw.contains("[PATCH] mm/slub"));
            assert_eq!(
                baseline.as_deref(),
                Some("1234567890abcdef1234567890abcdef12345678")
            );
        }
        other => panic!("expected RawMboxSubmitted, got {other:?}"),
    }
}

#[tokio::test]
async fn test_submit_rejected_in_read_only_mode() {
    let server = spawn_test_server(/* read_only */ true).await;

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/api/submit", server.base_url))
        .json(&serde_json::json!({
            "type": "inject",
            "raw": SAMPLE_MBOX,
        }))
        .send()
        .await
        .unwrap();

    // read_only mode should reject POST requests.
    assert_eq!(resp.status(), 403);
}

// ── Database-Backed Query Tests ─────────────────────────────────────────

#[tokio::test]
async fn test_patchset_message_and_review_query_endpoints() {
    let server = spawn_test_server(false).await;
    let client = reqwest::Client::new();

    server
        .db
        .conn
        .execute(
            "INSERT INTO patchsets (id, status, subject, author, date) \
             VALUES (1, 'Pending', '[PATCH] test patch', 'Author <a@b.com>', 1234567890)",
            (),
        )
        .await
        .unwrap();

    server
        .db
        .conn
        .execute(
            "INSERT INTO messages (message_id, subject, author, date, body) \
             VALUES ('<integ-1@example.com>', 'Test Subject', 'Author <a@b.com>', 1234567890, 'Test body')",
            (),
        )
        .await
        .unwrap();

    server
        .db
        .conn
        .execute(
            "INSERT INTO patches (id, patchset_id, message_id, part_index) \
             VALUES (1, 1, '<integ-1@example.com>', 1)",
            (),
        )
        .await
        .unwrap();

    let logs_json = "[{\"role\":\"user\",\"content\":\"test prompt\"}]";
    server
        .db
        .conn
        .execute(
            &format!(
                "INSERT INTO reviews (id, patchset_id, status, logs, created_at) \
                 VALUES (1, 1, 'Reviewed', '{logs_json}', 1234567890)"
            ),
            (),
        )
        .await
        .unwrap();

    let resp = client
        .get(format!("{}/api/patchsets", server.base_url))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["total"], 1);
    assert_eq!(body["items"].as_array().unwrap().len(), 1);

    let resp = client
        .get(format!(
            "{}/api/message?id=<integ-1@example.com>",
            server.base_url
        ))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["subject"], "Test Subject");
    assert_eq!(body["body"], "Test body");

    let resp = client
        .get(format!("{}/api/review?id=1", server.base_url))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["logs"].as_str().unwrap(), logs_json);
}

/// The patchset page asks for its page size as limit=. Through the real
/// router, the size reaches the query on both patchset routes, per_page wins
/// when both are named, the size is bounded at 100, a value that is not a
/// number falls back to the default, and a repeated key is refused as a
/// repeated per_page always was.
#[tokio::test]
async fn test_patchset_page_size_reaches_the_query() {
    let server = spawn_test_server(false).await;
    let client = reqwest::Client::new();

    let tx = server.db.begin_immediate_transaction().await.unwrap();
    tx.execute(
        "INSERT INTO patchsets (id, status, subject, author, date, total_parts, received_parts) \
         VALUES (1, 'Pending', '[PATCH 0/120] big series', 'Author <a@b.com>', 1234567890, 120, 120)",
        (),
    )
    .await
    .unwrap();
    tx.execute(
        "WITH RECURSIVE cnt(x) AS (SELECT 1 UNION ALL SELECT x + 1 FROM cnt WHERE x < 120)
         INSERT INTO messages (message_id, subject, author, date, body)
         SELECT '<page-' || x || '@example.com>', 'part ' || x, 'Author <a@b.com>', 1234567890, ''
         FROM cnt",
        (),
    )
    .await
    .unwrap();
    tx.execute(
        "WITH RECURSIVE cnt(x) AS (SELECT 1 UNION ALL SELECT x + 1 FROM cnt WHERE x < 120)
         INSERT INTO patches (id, patchset_id, message_id, part_index)
         SELECT x, 1, '<page-' || x || '@example.com>', x FROM cnt",
        (),
    )
    .await
    .unwrap();
    tx.commit().await.unwrap();

    let get = |route: &str, query: &str| {
        let url = format!("{}/api/{route}?id=1&{query}", server.base_url);
        let client = client.clone();
        async move { client.get(url).send().await.unwrap() }
    };
    for route in ["patch", "patchset"] {
        for (query, expected) in [
            ("limit=7", 7),
            ("per_page=7", 7),
            ("per_page=9&limit=7", 9),
            ("limit=4294967295", 100),
            ("limit=abc", 50),
            ("page=1", 50),
        ] {
            let resp = get(route, query).await;
            assert_eq!(resp.status(), 200, "{route}?{query}");
            let body: serde_json::Value = resp.json().await.unwrap();
            assert_eq!(body["limit"], expected, "{route}?{query}");
            // The rows, not only the echo: the size is what bounds the query.
            let patches = body["patches"].as_array().map(Vec::len);
            assert_eq!(patches, Some(expected), "{route}?{query} patches");
        }
        let resp = get(route, "limit=7&limit=9").await;
        assert_eq!(resp.status(), 400, "{route}: a repeated limit");
    }
}

#[tokio::test]
async fn test_patchset_review_state_filter_counts_pages_and_cache_isolation() {
    use serde_json::Value;
    let server = spawn_test_server(false).await;
    server.db.conn.execute_batch("INSERT INTO messages (id, message_id, subject, date, body)
        VALUES (1, '<state-cover@example.com>', 'needle xen', 60, '');
        INSERT INTO mailing_lists (id, name, nntp_group) VALUES (1, 'Xen', 'xen-devel');
        INSERT INTO messages_mailing_lists (message_id, mailing_list_id) VALUES (1, 1);
        INSERT INTO patchsets (id, status, subject, date, cover_letter_message_id, embargo_until) VALUES
        (1, 'Reviewed', 'needle xen', 60, '<state-cover@example.com>', NULL),
        (2, 'Pending', 'needle pending', 50, NULL, NULL),
        (3, 'Reviewed', 'other reviewed', 40, NULL, 9999999999),
        (4, NULL, 'other missing', 30, NULL, NULL);
        INSERT INTO reviews (id, patchset_id, status) VALUES (1, 3, 'Reviewed');
        INSERT INTO findings (id, review_id, severity, problem, preexisting)
        VALUES (1, 1, 3, 'embargoed finding', 0);").await.unwrap();
    let client = reqwest::Client::new();
    let get = |query: &str| {
        let client = client.clone();
        let url = format!("{}/api/patchsets?{query}", server.base_url);
        async move { client.get(url).send().await.unwrap() }
    };
    let default: Value = get("").await.json().await.unwrap();
    assert_eq!(default["total"], 4);
    let filtered: Value = get("review_state=Reviewed").await.json().await.unwrap();
    assert_eq!(filtered["total"], 2);
    assert_eq!(filtered["items"].as_array().unwrap().len(), 2);
    assert_eq!(filtered["items"][1]["status"], "Embargoed");
    let original_embargoed = default["items"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["id"] == 3)
        .unwrap();
    assert_eq!(
        filtered["items"][1]["findings_high"],
        original_embargoed["findings_high"]
    );
    let page: Value = get("review_state=Reviewed&per_page=1&page=2")
        .await
        .json()
        .await
        .unwrap();
    assert_eq!(page["total"], 2);
    assert_eq!(page["items"][0]["id"], 3);
    let remaining: Value = get("review_state=not_reviewed").await.json().await.unwrap();
    assert_eq!(remaining["total"], 2);
    let combined: Value = get("review_state=Reviewed&mailing_list=xen-devel&q=needle")
        .await
        .json()
        .await
        .unwrap();
    assert_eq!(combined["total"], 1);
    assert_eq!(combined["items"][0]["id"], 1);
    assert_eq!(get("review_state=unknown").await.status(), 400);
    assert_eq!(
        get("review_state=Reviewed&review_state=Pending")
            .await
            .status(),
        400
    );
    let restored: Value = get("").await.json().await.unwrap();
    assert_eq!(restored["total"], 4);
    assert_eq!(restored["items"].as_array().unwrap().len(), 4);
}

#[tokio::test]
async fn test_human_review_evidence_is_opt_in_positive_partial_and_current() {
    let mut settings = test_settings(false);
    settings.server.human_review_evidence = true;
    let server = spawn_test_server_with_settings(settings).await;
    server.db.conn.execute_batch("INSERT INTO patchsets (id,status,subject,date,total_parts,received_parts)
        VALUES (1,'Reviewed','human series',100,2,2), (2,'Failed','other series',200,1,1);
        INSERT INTO messages (message_id,subject) VALUES ('human-part-1@example.invalid','first'), ('human-part-2@example.invalid','second'), ('human-part-3@example.invalid','other');
        INSERT INTO patches (id,patchset_id,message_id,part_index,diff) VALUES
        (1,1,'human-part-1@example.invalid',1,'first diff'), (2,1,'human-part-2@example.invalid',2,'second diff'), (3,2,'human-part-3@example.invalid',1,'other diff');").await.unwrap();
    let thread = server
        .db
        .create_thread("human-part-1@example.invalid", "human series", 1)
        .await
        .unwrap();
    for raw in [
        "From: Author <author@example.invalid>\r\nMessage-ID: <human-part-1@example.invalid>\r\nSubject: [PATCH 1/2] first\r\n\r\nReviewed-by: Author <author@example.invalid>\r\n",
        "From: Reviewer <reviewer@example.invalid>\r\nMessage-ID: <human-reply@example.invalid>\r\nIn-Reply-To: <human-part-1@example.invalid>\r\nSubject: Re: [PATCH 1/2] first\r\n\r\nReviewed-by: Reviewer <reviewer@example.invalid>\r\n",
    ] {
        let (metadata, _, input) =
            sashiko::human_review::parse_ingested_mail(raw.as_bytes(), true).unwrap();
        server
            .db
            .create_message_with_references(
                &metadata.message_id,
                thread,
                metadata.in_reply_to.as_deref(),
                &metadata.author,
                &metadata.subject,
                metadata.date,
                &metadata.body,
                "",
                "",
                None,
                None,
                None,
            )
            .await
            .unwrap();
        server
            .db
            .record_direct_review_claims(&metadata, &input.unwrap(), 300)
            .await
            .unwrap();
    }
    let client = reqwest::Client::new();
    let get = |query: &str| {
        let client = client.clone();
        let url = format!("{}/api/{query}", server.base_url);
        async move {
            client
                .get(url)
                .send()
                .await
                .unwrap()
                .error_for_status()
                .unwrap()
                .json::<serde_json::Value>()
                .await
                .unwrap()
        }
    };
    let default = get("patchsets").await;
    assert_eq!(default["total"], 2);
    assert_eq!(default["human_review"]["series"]["1"]["recorded_parts"], 1);
    assert_eq!(default["human_review"]["series"]["1"]["present_parts"], 2);
    let first = get("patchsets?human_review=recorded&per_page=1").await;
    assert_eq!(first["total"], 1);
    assert_eq!(first["items"][0]["id"], 1);
    assert_eq!(first["items"][0]["status"], "Reviewed");
    let second = get("patchsets?human_review=recorded&per_page=1&page=2").await;
    assert_eq!(second["total"], 1);
    assert!(second["items"].as_array().unwrap().is_empty());
    assert_eq!(
        get("patchsets?human_review=recorded&review_state=Failed").await["total"],
        0
    );
    assert_eq!(
        get("patchsets?human_review=recorded&review_state=Reviewed&q=human").await["total"],
        1
    );
    for route in ["patchset", "patch"] {
        let details = get(&format!("{route}?id=1&per_page=1")).await;
        assert_eq!(details["human_review"]["page_only"], true);
        assert_eq!(details["human_review"]["available"], true);
        let records = details["patches"][0]["human_review_evidence"]["records"]
            .as_array()
            .unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(
            records[0]["source_message_id"],
            "human-reply@example.invalid"
        );
        assert_eq!(
            records[0]["target_message_id"],
            "human-part-1@example.invalid"
        );
        assert_eq!(records[0]["reviewer_mailbox"], "reviewer@example.invalid");
        assert_eq!(records[0]["extraction_version"], 1);
        assert_eq!(records[0]["source_hash"].as_str().unwrap().len(), 64);
        let next = get(&format!("{route}?id=1&per_page=1&page=2")).await;
        assert!(
            next["patches"][0]["human_review_evidence"]["records"]
                .as_array()
                .unwrap()
                .is_empty()
        );
    }
    for query in [
        "human_review=absent",
        "human_review=none",
        "human_review=recorded&human_review=recorded",
    ] {
        assert_eq!(
            client
                .get(format!("{}/api/patchsets?{query}", server.base_url))
                .send()
                .await
                .unwrap()
                .status(),
            400
        );
    }
    server.db.conn.execute_batch("WITH RECURSIVE extra(n) AS (VALUES(1) UNION ALL SELECT n+1 FROM extra WHERE n<205)
        INSERT INTO messages (message_id,author,subject,date,body,in_reply_to,human_review_source_hash)
        SELECT 'bounded-'||n||'@example.invalid',author,subject,date,body,in_reply_to,printf('%064d',n)
        FROM extra CROSS JOIN messages WHERE message_id='human-reply@example.invalid';
        INSERT INTO human_review_evidence (source_id,target_patch_id,target_message_id,source_revision,target_revision,
            target_message_revision,extraction_version,source_hash,target_diff_hash,reviewer_mailbox,trailer,body_line,recorded_at)
        SELECT source.id,h.target_patch_id,h.target_message_id,source.human_review_revision,h.target_revision,
            h.target_message_revision,h.extraction_version,source.human_review_source_hash,h.target_diff_hash,h.reviewer_mailbox,h.trailer,h.body_line,h.recorded_at
        FROM messages source CROSS JOIN human_review_evidence h WHERE source.message_id LIKE 'bounded-%' AND h.id=1;").await.unwrap();
    let bounded = get("patchset?id=1&per_page=1").await;
    assert_eq!(
        bounded["patches"][0]["human_review_evidence"]["records"]
            .as_array()
            .unwrap()
            .len(),
        200
    );
    assert_eq!(
        bounded["patches"][0]["human_review_evidence"]["truncated"],
        true
    );
    assert_eq!(
        get("patchsets?human_review=recorded").await["human_review"]["series"]["1"]["recorded_parts"],
        1
    );
    assert!(server.db.human_review_patch_batch(&[1; 101]).await.is_err());
    assert!(
        server
            .db
            .human_review_series_batch(&[1; 101])
            .await
            .is_err()
    );
    server.db.execute("UPDATE messages SET body = 'withdrawn' WHERE message_id = 'human-reply@example.invalid' OR message_id LIKE 'bounded-%'", ()).await.unwrap();
    assert_eq!(get("patchsets?human_review=recorded").await["total"], 0);
    let after = get("patchsets").await;
    assert_eq!(after["total"], 2);
    assert_eq!(after["human_review"]["series"]["1"]["recorded_parts"], 0);
    let disabled = spawn_test_server(false).await;
    assert_eq!(
        client
            .get(format!(
                "{}/api/patchsets?human_review=recorded",
                disabled.base_url
            ))
            .send()
            .await
            .unwrap()
            .status(),
        400
    );
    let default_disabled: serde_json::Value = client
        .get(format!("{}/api/patchsets", disabled.base_url))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert!(default_disabled.get("human_review").is_none());
}

#[tokio::test]
async fn test_patchset_staging_evidence_is_batched_paginated_and_read_only() {
    use sashiko::staging::{CommitIdentity, INDEX_VERSION, IndexEntry, StagingSnapshot};
    let repo = tempfile::tempdir().unwrap();
    let mut settings = test_settings(false);
    settings.git.repository_path = repo.path().to_string_lossy().into_owned();
    let staging: sashiko::settings::StagingSettings =
        toml::from_str("remote='upstream'\ntarget_ref='refs/remotes/upstream/staging'").unwrap();
    settings.git.staging = Some(staging.clone());
    let server = spawn_test_server_with_settings(settings).await;
    server.db.conn.execute_batch("INSERT INTO patchsets (id, status, subject, date, total_parts, received_parts) VALUES (1, 'Reviewed', 'saved series', 100, 2, 2);
        INSERT INTO messages (message_id, subject) VALUES ('evidence-1@example.invalid', 'first'), ('evidence-2@example.invalid', 'second');
        INSERT INTO patches (id, patchset_id, message_id, part_index, git_patch_id) VALUES (1,1,'evidence-1@example.invalid',1,'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'), (2,1,'evidence-2@example.invalid',2,NULL);").await.unwrap();
    let repository = std::fs::canonicalize(repo.path())
        .unwrap()
        .to_string_lossy()
        .into_owned();
    let now = chrono::Utc::now().timestamp();
    let snapshot = StagingSnapshot {
        target_ref: staging.target_ref.clone(),
        tip: "b".repeat(40),
        algorithm_version: INDEX_VERSION,
        scope_complete: false,
        diff_bytes: 0,
        entries: vec![IndexEntry {
            commit: "c".repeat(40),
            identity: CommitIdentity::Text {
                patch_id: "a".repeat(40),
            },
        }],
    };
    server
        .db
        .publish_staging_snapshot(&repository, &staging, &snapshot, now, now)
        .await
        .unwrap();
    let client = reqwest::Client::new();
    let get = |page: u32| {
        let client = client.clone();
        let url = format!(
            "{}/api/patchset?id=1&per_page=1&page={page}",
            server.base_url
        );
        async move {
            client
                .get(url)
                .send()
                .await
                .unwrap()
                .json::<serde_json::Value>()
                .await
                .unwrap()
        }
    };
    let first = get(1).await;
    assert_eq!(first["status"], "Reviewed");
    assert_eq!(first["patches"].as_array().unwrap().len(), 1);
    assert_eq!(
        first["patches"][0]["staging_evidence"]["state"],
        "matched_history"
    );
    assert_eq!(
        first["patches"][0]["staging_evidence"]["match_kind"],
        "stable_patch_id"
    );
    assert_eq!(
        first["patches"][0]["staging_evidence"]["tip"],
        "b".repeat(40)
    );
    assert_eq!(first["staging"]["page_only"], true);
    assert_eq!(
        first["patches"][0]["staging_evidence"]["exact_commits"],
        serde_json::json!([])
    );
    server
        .db
        .conn
        .execute(
            "INSERT INTO reviews (id,patchset_id,patch_id,status) VALUES (1,1,1,'Reviewed')",
            (),
        )
        .await
        .unwrap();
    server
        .db
        .conn
        .execute(
            "INSERT INTO review_patch_coverage (review_id,patch_id,commit_id) VALUES (1,1,?)",
            libsql::params!["c".repeat(40)],
        )
        .await
        .unwrap();
    for route in ["patchset", "patch"] {
        let exact: serde_json::Value = client
            .get(format!("{}/api/{route}?id=1&per_page=1", server.base_url))
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        assert_eq!(exact["status"], "Reviewed");
        assert_eq!(
            exact["patches"][0]["staging_evidence"]["match_kind"],
            "exact_ancestry"
        );
        assert_eq!(
            exact["patches"][0]["staging_evidence"]["exact_commits"],
            serde_json::json!(["c".repeat(40)])
        );
    }
    let listing: serde_json::Value = client
        .get(format!(
            "{}/api/patchsets?staging_state=partially_matched&per_page=1",
            server.base_url
        ))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(listing["total"], 1);
    assert_eq!(listing["items"].as_array().unwrap().len(), 1);
    assert_eq!(
        listing["staging"]["series"]["1"]["state"],
        "partially_matched"
    );
    assert_eq!(listing["staging"]["series"]["1"]["present_parts"], 2);
    assert_eq!(listing["staging"]["series"]["1"]["matched_parts"], 1);
    let no_match: serde_json::Value = client
        .get(format!(
            "{}/api/patchsets?staging_state=all_matched",
            server.base_url
        ))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(no_match["total"], 0);
    let invalid = client
        .get(format!(
            "{}/api/patchsets?staging_state=merged",
            server.base_url
        ))
        .send()
        .await
        .unwrap();
    assert_eq!(invalid.status(), 400);
    let full: serde_json::Value = client
        .get(format!("{}/api/patch?id=1&per_page=1", server.base_url))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(
        full["patches"][0]["staging_evidence"]["state"],
        "matched_history"
    );
    let second = get(2).await;
    assert_eq!(second["patches"][0]["id"], 2);
    assert_eq!(second["patches"][0]["staging_evidence"]["state"], "unknown");
    let old = now - i64::try_from(staging.freshness_seconds).unwrap() - 10;
    server
        .db
        .publish_staging_snapshot(&repository, &staging, &snapshot, old, old)
        .await
        .unwrap();
    assert_eq!(
        get(1).await["patches"][0]["staging_evidence"]["state"],
        "stale"
    );
    server
        .db
        .publish_staging_snapshot(&repository, &staging, &snapshot, now, now)
        .await
        .unwrap();
    server
        .db
        .conn
        .execute(
            "UPDATE patches SET git_patch_id = ? WHERE id = 1",
            libsql::params!["d".repeat(40)],
        )
        .await
        .unwrap();
    assert_eq!(
        get(1).await["patches"][0]["staging_evidence"]["state"],
        "unknown"
    );
    server
        .db
        .invalidate_staging_target(&repository, &staging, "Refresh failed", now)
        .await
        .unwrap();
    let invalidated = get(1).await;
    assert_eq!(
        invalidated["patches"][0]["staging_evidence"]["state"],
        "unknown"
    );
    assert_eq!(invalidated["status"], "Reviewed");
    assert!(!first.to_string().contains(&repository));
    assert!(!repo.path().join(".git").exists());
    std::fs::remove_dir(repo.path()).unwrap();
    let unavailable = get(1).await;
    assert_eq!(unavailable["status"], "Reviewed");
    assert_eq!(
        unavailable["patches"][0]["staging_evidence"]["state"],
        "unknown"
    );
    assert_eq!(
        unavailable["patches"][0]["staging_evidence"]["reason"],
        "Saved staging evidence unavailable"
    );
    let unknown: serde_json::Value = client
        .get(format!(
            "{}/api/patchsets?staging_state=unknown",
            server.base_url
        ))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(unknown["total"], 1);
    assert_eq!(unknown["staging"]["tip"], serde_json::Value::Null);
    let disabled = spawn_test_server(false).await;
    assert_eq!(
        client
            .get(format!(
                "{}/api/patchsets?staging_state=unknown",
                disabled.base_url
            ))
            .send()
            .await
            .unwrap()
            .status(),
        400
    );
    let unchanged: serde_json::Value = client
        .get(format!("{}/api/patchsets", disabled.base_url))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert!(unchanged.get("staging").is_none());
}

#[tokio::test]
async fn test_stats_reviews_and_tools_endpoints() {
    let server = spawn_test_server(false).await;
    let client = reqwest::Client::new();

    server
        .db
        .conn
        .execute(
            "INSERT INTO patchsets (id, status, subject, author, date) \
             VALUES (1, 'Pending', '[PATCH] test patch', 'Author <a@b.com>', 1234567890)",
            (),
        )
        .await
        .unwrap();

    // Insert 1005 reviews (5 Failed first, then 1000 Reviewed) and tool usages in bulk.
    let tx = server.db.begin_immediate_transaction().await.unwrap();
    tx.execute(
        "WITH RECURSIVE cnt(x) AS (SELECT 1 UNION ALL SELECT x + 1 FROM cnt WHERE x < 5)
         INSERT INTO reviews (id, patchset_id, status, created_at)
         SELECT x, 1, 'Failed', 1234567890 + x FROM cnt",
        (),
    )
    .await
    .unwrap();
    tx.execute(
        "WITH RECURSIVE cnt(x) AS (SELECT 6 UNION ALL SELECT x + 1 FROM cnt WHERE x < 1005)
         INSERT INTO ai_interactions (id, tokens_in, tokens_out, tokens_cached)
         SELECT 'int-' || x, 10, 20, 5 FROM cnt",
        (),
    )
    .await
    .unwrap();
    tx.execute(
        "WITH RECURSIVE cnt(x) AS (SELECT 6 UNION ALL SELECT x + 1 FROM cnt WHERE x < 1005)
         INSERT INTO reviews (id, patchset_id, status, interaction_id, created_at)
         SELECT x, 1, 'Reviewed', 'int-' || x, 1234567890 + x FROM cnt",
        (),
    )
    .await
    .unwrap();
    tx.execute(
        "INSERT INTO tool_usages (review_id, tool_name, output_length)
         SELECT id, CASE WHEN id <= 5 THEN 'old_tool' ELSE 'new_tool' END,
                CASE WHEN id <= 5 THEN 100 ELSE 200 END
         FROM reviews WHERE id <= 10",
        (),
    )
    .await
    .unwrap();
    tx.commit().await.unwrap();

    let resp = client
        .get(format!("{}/api/stats/reviews", server.base_url))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["total_reviews"], 1005);
    assert_eq!(body["total_failures"], 5);
    let reviews = body["reviews"].as_array().unwrap();
    assert_eq!(reviews.len(), 1);
    let group = &reviews[0];
    assert_eq!(group["status"], "Reviewed");
    assert_eq!(group["count"], 1000);
    assert_eq!(group["tokens_in"], 10000);
    assert_eq!(group["tokens_out"], 20000);
    assert_eq!(group["tokens_cached"], 5000);

    let resp = client
        .get(format!("{}/api/stats/tools", server.base_url))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    let tools = body.as_array().unwrap();
    assert_eq!(tools.len(), 1);
    assert_eq!(tools[0]["tool"], "new_tool");
    assert_eq!(tools[0]["count"], 5);
    assert_eq!(tools[0]["avg_output_length"], 200.0);
}

// ── Redirect Tests ───────────────────────────────────────────────────────

#[tokio::test]
async fn test_redirect_www_to_non_www() {
    let server = spawn_test_server(false).await;

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    for (path, expected_location) in [
        ("", "https://sashiko.dev/"),
        (
            "/api/stats?foo=bar",
            "https://sashiko.dev/api/stats?foo=bar",
        ),
    ] {
        let resp = client
            .get(format!("{}{path}", server.base_url))
            .header("Host", "www.sashiko.dev")
            .send()
            .await
            .unwrap();

        assert_eq!(resp.status(), 308);
        assert_eq!(
            resp.headers().get("Location").unwrap().to_str().unwrap(),
            expected_location
        );
    }
}
