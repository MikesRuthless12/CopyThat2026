//! Phase 4 smoke test.
//!
//! For every non-Purge `ShredMethod`, shred a freshly-seeded 10 MiB
//! file and assert:
//!
//! 1. `shred_file` returns `Ok(ShredReport)`.
//! 2. The file no longer exists at its original path.
//! 3. The parent directory's listing no longer contains the original
//!    filename (covers the "rename to scrub dirent" contract).
//! 4. The event count matches the method's nominal pass count.
//!
//! Separately asserts that `Nist80088Purge` refuses on this platform
//! (Phase 4 has no hardware path; Phase 17 will wire in the privileged
//! helper).
//!
//! Runtime: Gutmann's 35 passes × 10 MiB dominates; everything else is
//! sub-second. Total well under a minute on any modern dev machine.

use std::path::Path;

use copythat_core::CopyControl;
use copythat_secure_delete::{ShredEvent, ShredMethod, shred_file};
use tempfile::tempdir;
use tokio::sync::mpsc;

const SIZE: usize = 10 * 1024 * 1024; // 10 MiB

fn seed(path: &Path) {
    // Deterministic non-zero payload so a silent "wrote nothing" bug
    // can't masquerade as a successful zero-pass shred.
    let mut buf = Vec::with_capacity(SIZE);
    for i in 0..SIZE {
        buf.push((i as u8).wrapping_add(0x37));
    }
    std::fs::write(path, &buf).expect("seed 10 MiB");
}

fn parent_listing_contains(dir: &Path, name: &str) -> bool {
    std::fs::read_dir(dir)
        .map(|it| {
            it.flatten()
                .any(|entry| entry.file_name().to_string_lossy() == name)
        })
        .unwrap_or(false)
}

async fn drain(mut rx: mpsc::Receiver<ShredEvent>) -> Vec<ShredEvent> {
    let mut out = Vec::new();
    while let Some(evt) = rx.recv().await {
        out.push(evt);
    }
    out
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn phase_04_smoke_shred_10mib_each_method() {
    let dir = tempdir().unwrap();
    let methods = [
        ShredMethod::Zero,
        ShredMethod::Random,
        ShredMethod::Nist80088Clear,
        ShredMethod::DoD3Pass,
        ShredMethod::DoD7Pass,
        ShredMethod::Schneier7,
        ShredMethod::Vsitr7,
        ShredMethod::Gutmann35,
    ];

    for method in methods {
        let filename = format!("phase04-{}.bin", method.name());
        let path = dir.path().join(&filename);
        seed(&path);
        assert!(path.exists(), "{}: failed to seed source", method.name());

        let (tx, rx) = mpsc::channel::<ShredEvent>(2048);
        let ctrl = CopyControl::new();
        let task_path = path.clone();
        let t0 = std::time::Instant::now();
        let handle = tokio::spawn(async move { shred_file(&task_path, method, ctrl, tx).await });
        let events = drain(rx).await;
        let report = handle
            .await
            .unwrap()
            .unwrap_or_else(|e| panic!("{}: shred failed: {e}", method.name()));
        let wall = t0.elapsed();

        assert_eq!(
            report.passes,
            method.pass_count(),
            "{}: report.passes != method.pass_count()",
            method.name()
        );
        assert_eq!(
            report.bytes_per_pass,
            SIZE as u64,
            "{}: wrong bytes_per_pass",
            method.name()
        );
        assert!(
            !path.exists(),
            "{}: file still exists at {path:?}",
            method.name()
        );
        assert!(
            !parent_listing_contains(dir.path(), &filename),
            "{}: original filename `{}` lingers in parent dirent",
            method.name(),
            filename
        );

        let pass_starts = events
            .iter()
            .filter(|e| matches!(e, ShredEvent::PassStarted { .. }))
            .count();
        let pass_completed = events
            .iter()
            .filter(|e| matches!(e, ShredEvent::PassCompleted { .. }))
            .count();
        assert_eq!(
            pass_starts,
            method.pass_count(),
            "{}: PassStarted count",
            method.name()
        );
        assert_eq!(
            pass_completed,
            method.pass_count(),
            "{}: PassCompleted count",
            method.name()
        );

        let mib_total = (SIZE as f64 * method.pass_count() as f64) / (1024.0 * 1024.0);
        let secs = wall.as_secs_f64().max(1e-9);
        eprintln!(
            "PHASE04 SMOKE: {} shredded 10 MiB × {} passes ({mib_total:.1} MiB written) in {:.2} s ({:.1} MiB/s)",
            method.name(),
            method.pass_count(),
            secs,
            mib_total / secs
        );
    }
}

#[tokio::test]
async fn phase_04_smoke_purge_refuses_and_preserves_file() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("purge-target.bin");
    seed(&path);
    let before = std::fs::read(&path).unwrap();

    let (tx, _) = mpsc::channel::<ShredEvent>(16);
    let err = shred_file(&path, ShredMethod::Nist80088Purge, CopyControl::new(), tx)
        .await
        .expect_err("purge should refuse without hardware path");
    assert!(err.is_purge_not_supported(), "wrong kind: {err:?}");

    // The file must survive untouched.
    assert!(path.exists());
    let after = std::fs::read(&path).unwrap();
    assert_eq!(before, after, "refused Purge must not touch file contents");
}
