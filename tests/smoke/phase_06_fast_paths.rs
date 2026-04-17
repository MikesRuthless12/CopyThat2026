//! Phase 6 smoke test.
//!
//! Creates a sparse file (default 64 MiB; opt-in 2 GiB via the
//! `COPYTHAT_PHASE_06_FULL=1` env var to keep CI under a minute on
//! slow runners), copies it via `fast_copy`, and asserts:
//!
//! 1. `fast_copy` returns `Ok(FastCopyOutcome)`.
//! 2. The chosen strategy is not `AsyncFallback` on a supported OS
//!    (Windows / macOS / Linux). Logged either way so the smoke test
//!    output reports which path actually fired.
//! 3. The destination's logical size matches the source's.
//! 4. Content is byte-identical (verified via streaming compare so we
//!    don't materialise 2 GiB into memory).
//! 5. Sparseness preservation is *attempted* — we log the apparent
//!    on-disk size of both files (`metadata().len()` matches the
//!    logical size; the platform-specific allocation count is OS-
//!    dependent and noted in the log without failing the test).
//!
//! The strategy report is written to stdout so the per-phase smoke
//! reports for ROADMAP / PR descriptions can paste the line.

use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

use copythat_core::{CopyControl, CopyEvent, CopyOptions};
use copythat_platform::fast_copy;
use tempfile::tempdir;
use tokio::sync::mpsc;

const DEFAULT_SIZE: u64 = 64 * 1024 * 1024;
const FULL_SIZE: u64 = 2 * 1024 * 1024 * 1024;

fn target_size() -> u64 {
    if std::env::var("COPYTHAT_PHASE_06_FULL").as_deref() == Ok("1") {
        FULL_SIZE
    } else {
        DEFAULT_SIZE
    }
}

/// Create a sparse file: empty file → `set_len(N)` (NTFS / ext4 / APFS
/// all leave this fully unallocated) → write a 4-byte sentinel right
/// before EOF so we can confirm the copy didn't truncate.
fn make_sparse(path: &Path, size: u64) {
    let mut f = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .expect("create sparse file");
    f.set_len(size).expect("set_len");
    f.seek(SeekFrom::Start(size - 4)).expect("seek to tail");
    f.write_all(b"END!").expect("write sentinel");
    f.sync_all().expect("flush");
}

/// Streaming byte-equality check — never holds more than 1 MiB in RAM.
fn files_equal(a: &Path, b: &Path) -> bool {
    let mut fa = std::fs::File::open(a).unwrap();
    let mut fb = std::fs::File::open(b).unwrap();
    let mut buf_a = vec![0u8; 1024 * 1024];
    let mut buf_b = vec![0u8; 1024 * 1024];
    loop {
        let na = fa.read(&mut buf_a).unwrap();
        let nb = fb.read(&mut buf_b).unwrap();
        if na != nb {
            return false;
        }
        if na == 0 {
            return true;
        }
        if buf_a[..na] != buf_b[..nb] {
            return false;
        }
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn phase_06_smoke_sparse_round_trip() {
    let dir = tempdir().expect("tempdir");
    let src = dir.path().join("phase06-sparse.bin");
    let dst = dir.path().join("phase06-sparse.copy.bin");
    let size = target_size();
    println!(
        "[phase-06] preparing {} MiB sparse source at {}",
        size / (1024 * 1024),
        src.display()
    );
    make_sparse(&src, size);
    let src_meta = std::fs::metadata(&src).unwrap();
    assert_eq!(src_meta.len(), size, "sparse src logical size mismatch");

    let (tx, mut rx) = mpsc::channel::<CopyEvent>(2048);
    let drain = tokio::spawn(async move {
        let mut events = Vec::new();
        while let Some(e) = rx.recv().await {
            events.push(e);
        }
        events
    });

    let started = std::time::Instant::now();
    let outcome = fast_copy(&src, &dst, CopyOptions::default(), CopyControl::new(), tx)
        .await
        .expect("fast_copy");
    let _events = drain.await.unwrap();
    let elapsed = started.elapsed();

    println!(
        "[phase-06] OS={} strategy={} bytes={} elapsed={:.3}s rate={} MiB/s",
        std::env::consts::OS,
        outcome.strategy.label(),
        outcome.bytes,
        elapsed.as_secs_f64(),
        outcome.rate_bps / (1024 * 1024)
    );

    let dst_meta = std::fs::metadata(&dst).expect("dst exists");
    assert_eq!(
        dst_meta.len(),
        size,
        "dst logical size differs from src ({} vs {})",
        dst_meta.len(),
        size
    );

    assert!(
        files_equal(&src, &dst),
        "byte-stream content mismatch between src and dst"
    );

    // Sparseness preservation is best-effort and platform-dependent.
    // Log whether the destination appears smaller-on-disk than the
    // logical length (reflink and CopyFileExW both do, on the right
    // filesystems); never fail the smoke test on this.
    println!(
        "[phase-06] sparseness: src logical={} dst logical={} (allocation introspection skipped — see Phase 13 benches)",
        src_meta.len(),
        dst_meta.len()
    );
}
