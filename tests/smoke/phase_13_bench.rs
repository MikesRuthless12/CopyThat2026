//! Phase 13 smoke test — engine throughput floor.
//!
//! CI guardrail for the Criterion bench suite. This test does a
//! tiny 32 MiB synthetic copy through `copy_file` with the
//! platform fast-path hook attached (the shipped Tauri config)
//! and asserts the observed throughput is ≥ 20 MiB/s. That floor
//! is deliberately loose:
//!
//! - Real desktops sit at 200–800 MiB/s on this workload. 20 MiB/s
//!   is ~10× slower than the slowest runner we have; anything that
//!   drops the engine below this is almost certainly a regression,
//!   not normal variance.
//! - CI shared runners are occasionally congested; a 2× sample
//!   floor would trip on noisy shards and erode trust in the gate.
//!
//! The point of this test is to catch the "I accidentally added a
//! `sleep(Duration::from_secs(1))` in the write loop" category of
//! bug — not to grade performance. The full Criterion matrix lives
//! in `crates/copythat-core/benches/copy_bench.rs` and runs via
//! `cargo bench` / `xtask bench-ci`; CI wires the bench into the
//! `clippy + test` matrix via `cargo test --workspace`, which
//! picks up this file automatically.
//!
//! The workload is deterministic (256-byte rolling pattern) and
//! lives entirely inside a `tempdir()`. Cross-platform: `copy_file`
//! is the same engine everywhere. On Windows the platform hook
//! routes through `CopyFileExW`; on Linux/macOS it routes through
//! `copy_file_range`/`copyfile` respectively.

use std::io::Write;
use std::sync::Arc;
use std::time::Instant;

use copythat_core::{CopyControl, CopyOptions, FastCopyHook, copy_file};
use copythat_platform::PlatformFastCopyHook;
use tempfile::tempdir;
use tokio::sync::mpsc;

const WORKLOAD_BYTES: usize = 32 * 1024 * 1024;
/// Conservative: real desktops deliver 10×–40× this. Anything below
/// here is a ~10× regression, not environmental noise.
const MIN_THROUGHPUT_MIB_S: f64 = 20.0;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn phase_13_engine_meets_throughput_floor() {
    let tmp = tempdir().expect("tempdir");
    let src = tmp.path().join("bench-src.bin");
    let dst = tmp.path().join("bench-dst.bin");

    write_synthetic(&src, WORKLOAD_BYTES);

    // Warm the page cache by doing one untimed copy. Matches the
    // Criterion bench's "1 warmup + n measured" pattern; a cold
    // cache on a slow CI shard can easily drop a 32 MiB copy
    // below the floor.
    do_copy(&src, &dst).await;
    let _ = std::fs::remove_file(&dst);

    let t0 = Instant::now();
    do_copy(&src, &dst).await;
    let elapsed = t0.elapsed();

    let mib_per_s = (WORKLOAD_BYTES as f64) / (1024.0 * 1024.0) / elapsed.as_secs_f64().max(1e-9);
    assert!(
        mib_per_s >= MIN_THROUGHPUT_MIB_S,
        "engine throughput floor: {mib_per_s:.1} MiB/s < {MIN_THROUGHPUT_MIB_S:.1} MiB/s ({}-byte workload, elapsed {:?})",
        WORKLOAD_BYTES,
        elapsed,
    );

    // Sanity: dst actually matches src in size.
    let got = std::fs::metadata(&dst).expect("dst metadata").len();
    assert_eq!(
        got as usize, WORKLOAD_BYTES,
        "copy produced wrong byte count"
    );
}

async fn do_copy(src: &std::path::Path, dst: &std::path::Path) {
    let hook: Arc<dyn FastCopyHook> = Arc::new(PlatformFastCopyHook);
    let opts = CopyOptions {
        fast_copy_hook: Some(hook),
        ..Default::default()
    };
    let ctrl = CopyControl::new();
    let (tx, mut rx) = mpsc::channel(32);
    // Drain on a background task so backpressure never stalls the
    // engine.
    let drain = tokio::spawn(async move { while rx.recv().await.is_some() {} });
    copy_file(src, dst, opts, ctrl, tx)
        .await
        .expect("copy_file");
    drain.abort();
}

fn write_synthetic(path: &std::path::Path, size: usize) {
    // Deterministic 256-byte rolling pattern — same shape as the
    // Criterion bench's payload writer. Defeats transparent
    // compression on filesystems that have it (ZFS, APFS under
    // some configs) so the floor measures real write work.
    let pattern: [u8; 256] = std::array::from_fn(|i| i as u8);
    let mut f = std::fs::File::create(path).expect("create");
    let mut remaining = size;
    while remaining > 0 {
        let n = remaining.min(pattern.len());
        f.write_all(&pattern[..n]).expect("write");
        remaining -= n;
    }
    f.sync_all().ok();
}
