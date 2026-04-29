//! Phase 39 smoke — browser-accessible recovery UI.
//!
//! Three cases mirroring the prompt's contract:
//!
//! 1. `GET /` with a valid token → 200 + the brand string
//!    "CopyThat Recovery" appears in the rendered HTML.
//! 2. `GET /` without a token → 401 (loopback gate alone is not
//!    enough — every request must present the bearer token).
//! 3. `POST /restore` with `{job_id, path, timestamp_ms}` → 202 +
//!    JSON body whose `job_id` is the freshly-minted restore-job
//!    rowid.
//!
//! Plus a fourth case asserting all eight Phase 39 Fluent keys exist
//! in every one of the 18 locales (Standing Per-Phase Rule #3).

use std::fs;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use copythat_chunk::ChunkStore;
use copythat_history::{History, JobSummary};
use copythat_recovery::{generate_token, serve};
use secrecy::SecretString;

const PHASE_39_KEYS: &[&str] = &[
    "settings-recovery-heading",
    "settings-recovery-enable",
    "settings-recovery-bind-address",
    "settings-recovery-port",
    "settings-recovery-show-url",
    "settings-recovery-rotate-token",
    "settings-recovery-allow-non-loopback",
    "settings-recovery-non-loopback-warning",
];

const LOCALES: &[&str] = &[
    "en", "es", "zh-CN", "hi", "ar", "pt-BR", "ru", "ja", "de", "fr", "ko", "it", "tr", "vi", "pl",
    "nl", "id", "uk",
];

struct Harness {
    handle: copythat_recovery::JoinHandle,
    token: String,
    seed_job_id: i64,
    _tmp: tempfile::TempDir,
}

async fn boot() -> Harness {
    let tmp = tempfile::tempdir().expect("tmpdir");
    let history = History::open_in_memory().await.expect("history");
    // Seed one job so `GET /` has something to render and
    // `POST /restore` has a valid `job_id` to reference.
    let id = history
        .record_start(&JobSummary {
            row_id: 0,
            kind: "copy".into(),
            status: "succeeded".into(),
            started_at_ms: 1_777_473_000_000,
            finished_at_ms: Some(1_777_473_010_000),
            src_root: PathBuf::from("/src"),
            dst_root: PathBuf::from("/dst"),
            total_bytes: 1024,
            files_ok: 1,
            files_failed: 0,
            verify_algo: None,
            options_json: None,
        })
        .await
        .expect("seed job");
    let chunk = ChunkStore::open(tmp.path()).expect("chunk store");

    let token = generate_token();
    let secret: SecretString = token.clone().into();
    let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
    let handle = serve(addr, Arc::new(history), Arc::new(chunk), secret).expect("serve");

    Harness {
        handle,
        token,
        seed_job_id: id.0,
        _tmp: tmp,
    }
}

fn client() -> reqwest::Client {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .unwrap()
}

#[tokio::test]
async fn case01_landing_returns_200_with_valid_token() {
    let h = boot().await;
    let url = format!("http://{}/", h.handle.local_addr());
    let resp = client()
        .get(&url)
        .header("Authorization", format!("Bearer {}", h.token))
        .send()
        .await
        .expect("send");
    assert_eq!(resp.status(), reqwest::StatusCode::OK, "GET / with token");
    let body = resp.text().await.unwrap();
    assert!(
        body.contains("CopyThat Recovery"),
        "landing must contain the brand string; got: {body}"
    );
    h.handle.shutdown().await;
}

#[tokio::test]
async fn case02_landing_returns_401_without_token() {
    let h = boot().await;
    let url = format!("http://{}/", h.handle.local_addr());
    let resp = client().get(&url).send().await.expect("send");
    assert_eq!(
        resp.status(),
        reqwest::StatusCode::UNAUTHORIZED,
        "GET / without token"
    );
    let www_authenticate = resp
        .headers()
        .get("www-authenticate")
        .map(|v| v.to_str().unwrap_or("").to_string())
        .unwrap_or_default();
    assert!(
        www_authenticate.contains("Bearer"),
        "401 must include WWW-Authenticate: Bearer challenge, got `{www_authenticate}`"
    );
    h.handle.shutdown().await;
}

#[tokio::test]
async fn case03_post_restore_returns_202_and_job_id() {
    let h = boot().await;
    let url = format!("http://{}/restore", h.handle.local_addr());
    let resp = client()
        .post(&url)
        .header("Authorization", format!("Bearer {}", h.token))
        .json(&serde_json::json!({
            "job_id": h.seed_job_id,
            "path": "docs/quarterly-report.pdf",
            "timestamp_ms": 1_777_400_000_000_i64,
        }))
        .send()
        .await
        .expect("send");
    assert_eq!(
        resp.status(),
        reqwest::StatusCode::ACCEPTED,
        "POST /restore valid body"
    );
    let body: serde_json::Value = resp.json().await.expect("json body");
    let new_id = body
        .get("job_id")
        .and_then(|v| v.as_i64())
        .expect("`job_id` field in 202 response");
    assert!(
        new_id > h.seed_job_id,
        "new restore-job rowid must be greater than the seed job's rowid (got {new_id}, seed {})",
        h.seed_job_id
    );
    h.handle.shutdown().await;
}

#[tokio::test]
async fn case04_token_in_query_param_also_authenticates() {
    let h = boot().await;
    let url = format!("http://{}/?t={}", h.handle.local_addr(), h.token);
    let resp = client().get(&url).send().await.expect("send");
    assert_eq!(
        resp.status(),
        reqwest::StatusCode::OK,
        "?t= query param must authenticate"
    );
    h.handle.shutdown().await;
}

#[test]
fn case05_phase_39_fluent_keys_present_in_every_locale() {
    let root = locate_locales_dir().expect("locate locales/");
    for code in LOCALES {
        let path = root.join(code).join("copythat.ftl");
        let content =
            fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        for key in PHASE_39_KEYS {
            let starts = content.starts_with(&format!("{key} ="));
            let inline = content.contains(&format!("\n{key} ="));
            assert!(
                starts || inline,
                "locale `{code}` missing key `{key}` at {}",
                path.display()
            );
        }
    }
}

fn locate_locales_dir() -> Option<PathBuf> {
    let mut cur = std::env::current_dir().ok()?;
    for _ in 0..6 {
        let candidate = cur.join("locales");
        if candidate.join("en").join("copythat.ftl").exists() {
            return Some(candidate);
        }
        if !cur.pop() {
            break;
        }
    }
    None
}
