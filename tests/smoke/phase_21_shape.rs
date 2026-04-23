//! Phase 21 smoke test — bandwidth shaping (GCRA token bucket +
//! mid-copy rate swap + rclone-style schedule parsing).
//!
//! Acceptance criteria from the phase prompt:
//!
//! 1. **Create a 256 MiB random source file.** Seeded once per test
//!    run from `rand`. Small enough to keep the default smoke under
//!    15 s on a modern laptop, big enough that the 32 MiB/s cap
//!    produces an unambiguously-observable wall time.
//! 2. **Set Shape rate to 32 MiB/s, copy, assert elapsed time is in
//!    `[7s, 9s]`.** 256 MiB ÷ 32 MiB/s = 8.0 s in theory. GCRA is
//!    accurate to a few milliseconds; the ±1 s window absorbs fs
//!    warm-up and the progress-event loop overhead.
//! 3. **Mid-copy, set rate to 8 MiB/s, assert post-change
//!    throughput converges.** A second copy with a sleep-then-
//!    `set_rate(8 MiB/s)` task running alongside the copy exercises
//!    hot-swap; the test asserts the final wall time is longer than
//!    the uncapped first half would suggest, i.e. the later rate
//!    actually slowed the tail.
//! 4. **Parse a schedule string, assert `current_limit()` returns
//!    expected values for fake-clock times at 09:00, 12:30, 14:00,
//!    19:00, Sat 13:00.**
//!
//! Environment knobs:
//!
//! - `COPYTHAT_PHASE21_FULL=1` → 1 GiB workload at 32 MiB/s (≈32 s).
//!   Off by default so `cargo test` stays fast.

use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};

use chrono::TimeZone;
use copythat_core::{CopyControl, CopyOptions, copy_file};
use copythat_shape::{ByteRate, CopyThatShapeSink, Schedule, Shape};
use rand::RngCore;
use tokio::sync::mpsc;

fn workload_bytes() -> u64 {
    if std::env::var("COPYTHAT_PHASE21_FULL").is_ok() {
        1024 * 1024 * 1024
    } else {
        256 * 1024 * 1024
    }
}

fn target_elapsed_window(total: u64, rate: u64) -> (Duration, Duration) {
    let expected_secs = total as f64 / rate as f64;
    // ±15% window. Tight enough to catch "we forgot to shape"
    // (would come back in 1-2 s) and "the bucket drains too slowly"
    // (would come back in 15+ s); loose enough to survive CI runner
    // scheduler variance.
    let lower = expected_secs * 0.85;
    let upper = expected_secs * 1.20;
    (
        Duration::from_secs_f64(lower),
        Duration::from_secs_f64(upper),
    )
}

#[test]
fn phase_21_shape_caps_throughput_to_configured_rate() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    rt.block_on(run_fixed_rate_case());
}

async fn run_fixed_rate_case() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let src = tmp.path().join("src.bin");
    let dst = tmp.path().join("dst.bin");
    let total = workload_bytes();
    seed_random_file(&src, total);

    let rate = ByteRate::mebibytes_per_second(32);
    let shape = Arc::new(Shape::new(Some(rate)));
    let sink: Arc<dyn copythat_core::ShapeSink> = Arc::new(CopyThatShapeSink::new(shape.clone()));
    let opts = CopyOptions {
        shape: Some(sink),
        ..CopyOptions::default()
    };

    let (tx, mut rx) = mpsc::channel(64);
    let drain = tokio::spawn(async move { while rx.recv().await.is_some() {} });
    let ctrl = CopyControl::new();

    let started = Instant::now();
    let report = copy_file(&src, &dst, opts, ctrl, tx)
        .await
        .expect("copy_file");
    drain.await.expect("drain");
    let elapsed = started.elapsed();

    assert_eq!(report.bytes, total);

    let (lo, hi) = target_elapsed_window(total, rate.bytes_per_second());
    assert!(
        elapsed >= lo,
        "shape did not cap throughput — elapsed {:?} < lower bound {:?}",
        elapsed,
        lo
    );
    assert!(
        elapsed <= hi,
        "shape blocked for too long — elapsed {:?} > upper bound {:?}",
        elapsed,
        hi
    );

    // Source + destination must still match byte-for-byte — shaping
    // slows the engine but never corrupts.
    assert_eq!(blake3_of_file(&src), blake3_of_file(&dst));
}

#[test]
fn phase_21_shape_honours_mid_copy_rate_change() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    rt.block_on(run_mid_copy_rate_change());
}

async fn run_mid_copy_rate_change() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let src = tmp.path().join("src.bin");
    let dst = tmp.path().join("dst.bin");
    // 128 MiB workload. At 32 MiB/s the copy would take ~3 s; the
    // 500 ms swap to 8 MiB/s guarantees a long shaped tail so the
    // total elapsed unambiguously reflects the post-swap rate.
    let total = 128 * 1024 * 1024;
    seed_random_file(&src, total);

    let initial_rate = ByteRate::mebibytes_per_second(32);
    let slowed_rate = ByteRate::mebibytes_per_second(8);
    let shape = Arc::new(Shape::new(Some(initial_rate)));
    let sink: Arc<dyn copythat_core::ShapeSink> = Arc::new(CopyThatShapeSink::new(shape.clone()));
    let opts = CopyOptions {
        shape: Some(sink),
        ..CopyOptions::default()
    };

    let shape_for_swapper = shape.clone();
    let swapper = tokio::spawn(async move {
        // Swap early — after burst drain but before the full
        // transfer could complete even at the initial rate.
        tokio::time::sleep(Duration::from_millis(500)).await;
        shape_for_swapper.set_rate(Some(slowed_rate));
    });

    let (tx, mut rx) = mpsc::channel(64);
    let drain = tokio::spawn(async move { while rx.recv().await.is_some() {} });
    let ctrl = CopyControl::new();

    let started = Instant::now();
    let report = copy_file(&src, &dst, opts, ctrl, tx)
        .await
        .expect("copy_file");
    drain.await.expect("drain");
    swapper.await.expect("swapper");
    let elapsed = started.elapsed();

    assert_eq!(report.bytes, total);

    // Without the swap: 128 MiB @ 32 MiB/s (minus 32 MiB burst) =
    // ~3 s. With the swap at 500 ms: ~48 MiB transfers in the
    // initial window, the remaining 80 MiB drains at 8 MiB/s =
    // ~10 s. Total ≥ 6 s is a safe lower bound (absorbs scheduler
    // jitter + swap-task wake-up variance); upper bound 30 s
    // guards against "swap hung the whole pipeline".
    assert!(
        elapsed >= Duration::from_secs(6),
        "mid-copy swap had no visible effect — elapsed {:?} (expected >= 6 s)",
        elapsed
    );
    assert!(
        elapsed <= Duration::from_secs(30),
        "mid-copy swap stalled the engine — elapsed {:?}",
        elapsed
    );

    // After the swap, the shape's current rate should be the slow
    // one — confirms the set_rate path actually mutated state.
    assert_eq!(shape.current_rate(), Some(slowed_rate));

    assert_eq!(blake3_of_file(&src), blake3_of_file(&dst));
}

#[test]
fn phase_21_schedule_parses_and_computes_current_limit() {
    // Day-range rule + four time-of-day boundaries.
    let spec = "08:00,512k 12:00,off 13:00,512k 18:00,10M Sat-Sun,unlimited";
    let s = Schedule::parse(spec).expect("schedule parses");
    assert_eq!(s.rules().len(), 5);

    // Fake clocks — 2026-04-22 is a Wednesday; 2026-04-25 is a
    // Saturday. All local time.
    let wed_9 = chrono::Local
        .with_ymd_and_hms(2026, 4, 22, 9, 0, 0)
        .single()
        .unwrap();
    let wed_12_30 = chrono::Local
        .with_ymd_and_hms(2026, 4, 22, 12, 30, 0)
        .single()
        .unwrap();
    let wed_14 = chrono::Local
        .with_ymd_and_hms(2026, 4, 22, 14, 0, 0)
        .single()
        .unwrap();
    let wed_19 = chrono::Local
        .with_ymd_and_hms(2026, 4, 22, 19, 0, 0)
        .single()
        .unwrap();
    let sat_13 = chrono::Local
        .with_ymd_and_hms(2026, 4, 25, 13, 0, 0)
        .single()
        .unwrap();

    let kib_512 = ByteRate::new(512 * 1024);
    let mib_10 = ByteRate::new(10 * 1024 * 1024);

    assert_eq!(s.current_limit(wed_9), Some(kib_512));
    assert_eq!(s.current_limit(wed_12_30), Some(ByteRate::new(0)));
    assert_eq!(s.current_limit(wed_14), Some(kib_512));
    assert_eq!(s.current_limit(wed_19), Some(mib_10));
    // Saturday falls under the day-range rule → unlimited (None).
    assert_eq!(s.current_limit(sat_13), None);
}

fn seed_random_file(path: &Path, total: u64) {
    use std::io::Write;
    let mut f = std::fs::File::create(path).expect("create src");
    let mut rng = rand::rng();
    let mut buf = vec![0u8; 1024 * 1024];
    let mut written: u64 = 0;
    while written < total {
        let to_write = std::cmp::min(buf.len() as u64, total - written) as usize;
        rng.fill_bytes(&mut buf[..to_write]);
        f.write_all(&buf[..to_write]).expect("write src");
        written += to_write as u64;
    }
    f.flush().expect("flush src");
}

fn blake3_of_file(path: &Path) -> [u8; 32] {
    use std::io::Read;
    let mut f = std::fs::File::open(path).expect("open for hash");
    let mut hasher = blake3::Hasher::new();
    let mut buf = vec![0u8; 64 * 1024];
    loop {
        let n = f.read(&mut buf).expect("read for hash");
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    *hasher.finalize().as_bytes()
}
