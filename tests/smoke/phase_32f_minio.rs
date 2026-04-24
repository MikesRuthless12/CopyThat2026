//! Phase 32f smoke — minio-backed S3 round-trip.
//!
//! # How to run
//!
//! The smoke is gated on the `COPYTHAT_PHASE32F_MINIO` env var — set
//! to `1` to activate. Skipped by default so CI + local
//! `cargo test` don't need docker.
//!
//! ```bash
//! # Start a minio sidecar (runs on localhost:9000, RootUser / RootPass).
//! docker run -d --rm --name copythat-minio \
//!   -p 9000:9000 -p 9001:9001 \
//!   -e "MINIO_ROOT_USER=minioadmin" \
//!   -e "MINIO_ROOT_PASSWORD=minioadmin" \
//!   quay.io/minio/minio server /data --console-address ":9001"
//!
//! # Create the target bucket (via the mc client or the web UI at :9001).
//! docker run --rm --network host quay.io/minio/mc \
//!   alias set local http://localhost:9000 minioadmin minioadmin
//! docker run --rm --network host quay.io/minio/mc mb local/copythat-test
//!
//! # Run the smoke.
//! COPYTHAT_PHASE32F_MINIO=1 \
//! COPYTHAT_MINIO_ENDPOINT=http://localhost:9000 \
//! COPYTHAT_MINIO_BUCKET=copythat-test \
//! COPYTHAT_MINIO_AK=minioadmin \
//! COPYTHAT_MINIO_SK=minioadmin \
//!   cargo test -p copythat-cloud --test phase_32f_minio -- --nocapture
//!
//! # Cleanup.
//! docker stop copythat-minio
//! ```
//!
//! # What it verifies
//!
//! 1. `make_operator` with the configured S3Config + a `<ak>\n<sk>`
//!    secret blob builds a live operator against minio.
//! 2. `copy_to_target` uploads a 1 MiB payload to the bucket.
//! 3. `copy_from_target` pulls it back to a local tempfile.
//! 4. Byte-for-byte equality on the round-trip.
//! 5. `stat` returns the right content length after the upload.
//!
//! Phase 32f's streaming writer path exercises the same operator
//! via `CopyThatCloudSink::put_stream_blocking` when `cloud_sink`
//! is wired through `copy_file` — that happy-path lands in a
//! follow-up smoke once a "streaming end-to-end through minio"
//! case is worth the extra docker orchestration.

use std::path::PathBuf;
use std::sync::Arc;

use copythat_cloud::{
    Backend, BackendConfig, BackendKind, CopyTarget, OperatorTarget, S3Config, copy_from_target,
    copy_to_target, make_operator,
};

fn should_run() -> bool {
    std::env::var("COPYTHAT_PHASE32F_MINIO").as_deref() == Ok("1")
}

fn minio_backend() -> Option<(Backend, String)> {
    if !should_run() {
        return None;
    }
    let endpoint =
        std::env::var("COPYTHAT_MINIO_ENDPOINT").unwrap_or_else(|_| "http://localhost:9000".into());
    let bucket = std::env::var("COPYTHAT_MINIO_BUCKET").unwrap_or_else(|_| "copythat-test".into());
    let ak = std::env::var("COPYTHAT_MINIO_AK").unwrap_or_else(|_| "minioadmin".into());
    let sk = std::env::var("COPYTHAT_MINIO_SK").unwrap_or_else(|_| "minioadmin".into());

    let backend = Backend {
        name: "minio-smoke".into(),
        kind: BackendKind::S3,
        config: BackendConfig::S3(S3Config {
            bucket,
            region: "us-east-1".into(),
            endpoint,
            root: String::new(),
        }),
    };
    let secret = format!("{ak}\n{sk}");
    Some((backend, secret))
}

#[tokio::test(flavor = "current_thread")]
async fn case_minio_round_trip_1mib() {
    let Some((backend, secret)) = minio_backend() else {
        eprintln!("skipping: COPYTHAT_PHASE32F_MINIO != 1");
        return;
    };

    let op = make_operator(&backend, Some(&secret)).expect("minio operator");
    let target: Arc<dyn CopyTarget> = Arc::new(OperatorTarget::new(backend.name.clone(), op));

    let src_dir = tempfile::tempdir().expect("src dir");
    let dst_dir = tempfile::tempdir().expect("dst dir");

    let src = src_dir.path().join("payload.bin");
    let payload: Vec<u8> = (0..1024 * 1024).map(|i| (i % 251) as u8).collect();
    tokio::fs::write(&src, &payload).await.expect("seed");

    // Push to minio.
    let written = copy_to_target(&src, &target, "smoke/payload.bin")
        .await
        .expect("upload");
    assert_eq!(written as usize, payload.len());

    // stat the object.
    let meta = target
        .stat("smoke/payload.bin")
        .await
        .expect("stat ok")
        .expect("object present");
    assert_eq!(meta.size, Some(payload.len() as u64));

    // Pull back.
    let dst: PathBuf = dst_dir.path().join("payload.bin");
    let read = copy_from_target(&target, "smoke/payload.bin", &dst)
        .await
        .expect("download");
    assert_eq!(read as usize, payload.len());

    // Byte-equal round-trip.
    let pulled = tokio::fs::read(&dst).await.expect("disk read");
    assert_eq!(pulled, payload);

    // Cleanup so the bucket doesn't grow over repeated smoke runs.
    target.delete("smoke/payload.bin").await.expect("cleanup");
}
