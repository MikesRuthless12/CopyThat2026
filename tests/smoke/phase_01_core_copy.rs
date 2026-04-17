//! Phase 1 smoke test.
//!
//! Creates a 100 MiB random source file, copies it through the Phase 1
//! engine, and proves:
//!
//! 1. Byte-for-byte equality source → destination.
//! 2. Mtime preservation.
//! 3. Throughput > 50 MB/s — warn-only. The smoke exits non-zero on
//!    (1) or (2) but only emits a line on stderr for (3) so a
//!    genuinely slow CI runner doesn't mark a correct engine as
//!    failed.

use std::time::Instant;

use copythat_core::{CopyControl, CopyEvent, CopyOptions, copy_file};
use filetime::{FileTime, set_file_mtime};
use rand::{RngCore, SeedableRng};
use tempfile::tempdir;
use tokio::sync::mpsc;

const SIZE: usize = 100 * 1024 * 1024; // 100 MiB
const TARGET_BYTES_PER_SEC: f64 = 50.0 * 1024.0 * 1024.0;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn phase_01_smoke_100mib_roundtrip() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("phase01.bin");
    let dst = dir.path().join("phase01.out");

    // Generate 100 MiB of deterministic pseudo-random data so failures
    // are reproducible.
    let mut rng = rand::rngs::StdRng::seed_from_u64(0xC09_1AB_u64);
    let mut buf = vec![0u8; SIZE];
    rng.fill_bytes(&mut buf);
    std::fs::write(&src, &buf).unwrap();

    // Stamp a recognisable mtime on the source.
    let stamp = FileTime::from_unix_time(1_712_000_000, 123_456_000);
    set_file_mtime(&src, stamp).unwrap();

    let (tx, mut rx) = mpsc::channel::<CopyEvent>(1024);
    let ctrl = CopyControl::new();
    let src_path = src.clone();
    let dst_path = dst.clone();

    let started = Instant::now();
    let task = tokio::spawn(async move {
        copy_file(&src_path, &dst_path, CopyOptions::default(), ctrl, tx).await
    });

    // Drain the event stream so the channel doesn't back-pressure.
    let drain = tokio::spawn(async move {
        let mut n = 0u64;
        while let Some(_evt) = rx.recv().await {
            n += 1;
        }
        n
    });

    let report = task.await.unwrap().expect("copy failed");
    let events = drain.await.unwrap();
    let wall = started.elapsed();

    // 1. Size + byte equality.
    assert_eq!(
        report.bytes as usize, SIZE,
        "engine reported {} bytes, expected {}",
        report.bytes, SIZE
    );
    let src_bytes = std::fs::read(&src).unwrap();
    let dst_bytes = std::fs::read(&dst).unwrap();
    assert_eq!(
        src_bytes, dst_bytes,
        "destination bytes do not match source"
    );

    // 2. Mtime preservation.
    let src_mtime = FileTime::from_last_modification_time(&std::fs::metadata(&src).unwrap());
    let dst_mtime = FileTime::from_last_modification_time(&std::fs::metadata(&dst).unwrap());
    assert_eq!(
        src_mtime, dst_mtime,
        "mtime on destination ({dst_mtime:?}) does not match source ({src_mtime:?})"
    );

    // 3. Rate check — warn only.
    let secs = wall.as_secs_f64().max(1e-9);
    let observed = SIZE as f64 / secs;
    if observed < TARGET_BYTES_PER_SEC {
        eprintln!(
            "PHASE01 SMOKE: rate {:.2} MiB/s below 50 MiB/s target (events={events}, secs={secs:.3})",
            observed / (1024.0 * 1024.0)
        );
    } else {
        eprintln!(
            "PHASE01 SMOKE: rate {:.2} MiB/s (events={events}, secs={secs:.3})",
            observed / (1024.0 * 1024.0)
        );
    }
}
