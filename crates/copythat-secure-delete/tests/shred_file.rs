//! Integration tests for `shred_file`.
//!
//! These use tiny temp files so the whole suite stays under a few
//! seconds even on slow CI runners. The 10 MiB per-method smoke test
//! lives at `tests/smoke/phase_04_shred.rs`.

use std::path::Path;

use copythat_core::CopyControl;
use copythat_secure_delete::{ShredEvent, ShredMethod, shred_file};
use tempfile::tempdir;
use tokio::sync::mpsc;

fn write_file(path: &Path, bytes: &[u8]) {
    std::fs::write(path, bytes).expect("seed file");
}

async fn collect_events(mut rx: mpsc::Receiver<ShredEvent>) -> Vec<ShredEvent> {
    let mut out = Vec::new();
    while let Some(evt) = rx.recv().await {
        out.push(evt);
    }
    out
}

#[tokio::test]
async fn shred_file_removes_the_target() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("secret.bin");
    write_file(&path, &vec![0x5Au8; 4096]);
    assert!(path.exists());

    let (tx, rx) = mpsc::channel::<ShredEvent>(64);
    let ctrl = CopyControl::new();
    let task_path = path.clone();
    let handle =
        tokio::spawn(async move { shred_file(&task_path, ShredMethod::DoD3Pass, ctrl, tx).await });
    let events = collect_events(rx).await;
    let report = handle.await.unwrap().expect("shred succeeded");

    assert!(!path.exists(), "shredded file still exists at {path:?}");
    assert_eq!(report.passes, 3);
    assert_eq!(report.bytes_per_pass, 4096);
    assert!(
        events
            .iter()
            .any(|e| matches!(e, ShredEvent::Completed { .. })),
        "no Completed event"
    );
}

#[tokio::test]
async fn per_method_pass_event_counts_match_spec() {
    // One Started + N PassStarted + N PassCompleted (+ maybe
    // SsdAdvisory) + 1 Completed. Assert the N on the pass events.
    let expected = [
        (ShredMethod::Zero, 1),
        (ShredMethod::Random, 1),
        (ShredMethod::Nist80088Clear, 1),
        (ShredMethod::DoD3Pass, 3),
        (ShredMethod::DoD7Pass, 7),
        (ShredMethod::Schneier7, 7),
        (ShredMethod::Vsitr7, 7),
        (ShredMethod::Gutmann35, 35),
    ];
    for (method, want) in expected {
        let dir = tempdir().unwrap();
        let path = dir.path().join(format!("{}.bin", method.name()));
        // Small file — we're testing the event shape, not throughput.
        write_file(&path, &vec![0xA5u8; 4096]);

        let (tx, rx) = mpsc::channel::<ShredEvent>(512);
        let ctrl = CopyControl::new();
        let task_path = path.clone();
        let handle = tokio::spawn(async move { shred_file(&task_path, method, ctrl, tx).await });
        let events = collect_events(rx).await;
        let report = handle.await.unwrap().expect("shred succeeded");
        assert_eq!(report.passes, want, "{}: report.passes", method.name());

        let starts = events
            .iter()
            .filter(|e| matches!(e, ShredEvent::PassStarted { .. }))
            .count();
        let completed_passes = events
            .iter()
            .filter(|e| matches!(e, ShredEvent::PassCompleted { .. }))
            .count();
        assert_eq!(
            starts,
            want,
            "{}: PassStarted count (got {starts}, want {want})",
            method.name()
        );
        assert_eq!(
            completed_passes,
            want,
            "{}: PassCompleted count (got {completed_passes}, want {want})",
            method.name()
        );
        assert!(!path.exists(), "{}: file still exists", method.name());
    }
}

#[tokio::test]
async fn nist_purge_refuses_without_hardware_path() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("purge.bin");
    write_file(&path, b"will not be touched");

    let (tx, _) = mpsc::channel::<ShredEvent>(64);
    let ctrl = CopyControl::new();
    let err = shred_file(&path, ShredMethod::Nist80088Purge, ctrl, tx)
        .await
        .expect_err("purge should be rejected");

    assert!(err.is_purge_not_supported(), "got {err:?}");
    // File must be untouched — refusing to shred is a promise.
    assert!(path.exists(), "Purge refusal must not delete the file");
    let contents = std::fs::read(&path).unwrap();
    assert_eq!(contents, b"will not be touched");
}

#[tokio::test]
async fn shred_rejects_directories() {
    let dir = tempdir().unwrap();
    let (tx, _) = mpsc::channel::<ShredEvent>(8);
    let ctrl = CopyControl::new();
    let err = shred_file(dir.path(), ShredMethod::Zero, ctrl, tx)
        .await
        .expect_err("should reject a directory");
    assert_eq!(err.kind, copythat_secure_delete::ShredErrorKind::BadTarget);
}

#[tokio::test]
async fn cancellation_between_passes_is_clean() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("cancelled.bin");
    write_file(&path, &vec![0u8; 256 * 1024]);

    let ctrl = CopyControl::new();
    let ctrl_for_cancel = ctrl.clone();
    ctrl_for_cancel.cancel(); // cancel before the shred starts

    let (tx, _) = mpsc::channel::<ShredEvent>(64);
    let err = shred_file(&path, ShredMethod::Gutmann35, ctrl, tx)
        .await
        .expect_err("cancellation should error");
    assert!(err.is_cancelled(), "expected cancelled, got {err:?}");
}

#[tokio::test]
async fn empty_file_is_handled() {
    // Zero-byte file: no bytes to overwrite, but the rename + unlink
    // still has to happen.
    let dir = tempdir().unwrap();
    let path = dir.path().join("empty.bin");
    std::fs::File::create(&path).unwrap();

    let (tx, _) = mpsc::channel::<ShredEvent>(64);
    let report = shred_file(&path, ShredMethod::DoD3Pass, CopyControl::new(), tx)
        .await
        .expect("empty-file shred");
    assert_eq!(report.bytes_per_pass, 0);
    assert!(!path.exists());
}
