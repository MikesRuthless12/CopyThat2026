//! Phase 13 — Criterion benchmarks for the `copythat-core` async engine.
//!
//! Four bench groups, each configurable through environment
//! variables so `xtask bench-ci` can shrink the workload for a
//! CI-friendly run:
//!
//! - `single_huge_file` — one big file, default 1 GiB. Scales down
//!   to 100 MiB when `COPYTHAT_BENCH_CI=1` to keep the CI suite
//!   under 90 s on the slowest runner.
//! - `many_small_files`  — 10 000 × 16 KiB on the full run,
//!   1 000 × 16 KiB in CI mode.
//! - `mixed_tree`        — 500 files across 5 subdirs with sizes
//!   distributed between 1 KiB and 256 KiB, simulating a typical
//!   source-tree corpus without the Linux kernel download.
//! - `buffer_size_sweep` — Phase 13b tuning hook: same workload as
//!   `single_huge_file` but sweeps `CopyOptions::buffer_size`
//!   across 64 KiB / 256 KiB / 1 MiB / 4 MiB / 16 MiB so the CI
//!   bench output reveals the breakeven point on the current host.
//!
//! Every bench sets up its workload inside a `tempdir()` so runs
//! never touch the user's data. The destination is `dst/<file>`
//! under the same tempdir; callers who want to measure cross-device
//! copies can point `COPYTHAT_BENCH_DST=/path/on/other/volume`.

use std::env;
use std::path::PathBuf;

use copythat_core::{CopyOptions, copy_file, copy_tree};
use copythat_core::{CopyControl, TreeOptions};
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use tempfile::tempdir;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;

// --- CI scaling -------------------------------------------------------

/// Returns `true` when the `COPYTHAT_BENCH_CI=1` env var is set.
/// `xtask bench-ci` exports this before invoking the bench harness;
/// every workload constant checks this and picks the scaled value.
fn ci_mode() -> bool {
    env::var("COPYTHAT_BENCH_CI")
        .ok()
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

// --- Shared helpers ---------------------------------------------------

fn runtime() -> Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("tokio runtime")
}

fn write_random_file(path: &std::path::Path, size: usize) {
    // Deterministic pattern — reproducible bench runs without a
    // PRNG seed flag. A rolling 256-byte block has enough entropy
    // to defeat any plausible OS-level compression / dedup hint.
    use std::io::Write;
    let pattern: [u8; 256] = std::array::from_fn(|i| i as u8);
    let mut remaining = size;
    let mut file = std::fs::File::create(path).expect("create");
    while remaining > 0 {
        let n = remaining.min(pattern.len());
        file.write_all(&pattern[..n]).expect("write");
        remaining -= n;
    }
    file.sync_all().ok();
}

fn dst_override_or(path: PathBuf) -> PathBuf {
    env::var("COPYTHAT_BENCH_DST")
        .map(|v| PathBuf::from(v).join("copythat-bench"))
        .unwrap_or(path)
}

// --- Single huge file -------------------------------------------------

fn single_huge_file(c: &mut Criterion) {
    let size: usize = if ci_mode() {
        100 * 1024 * 1024 // 100 MiB
    } else {
        1024 * 1024 * 1024 // 1 GiB
    };

    let tmp = tempdir().expect("tempdir");
    let src = tmp.path().join("huge.bin");
    write_random_file(&src, size);
    let dst_root = dst_override_or(tmp.path().join("dst"));
    std::fs::create_dir_all(&dst_root).ok();

    let mut group = c.benchmark_group("single_huge_file");
    group.sample_size(10);
    group.throughput(Throughput::Bytes(size as u64));

    group.bench_function("default-options", |b| {
        let rt = runtime();
        b.iter(|| {
            let dst = dst_root.join("huge.out");
            // Clean destination on every iteration so we measure
            // the write cost, not truncate-in-place.
            let _ = std::fs::remove_file(&dst);
            rt.block_on(async {
                let ctrl = CopyControl::new();
                let (tx, mut rx) = mpsc::channel(32);
                let drain = tokio::spawn(async move { while rx.recv().await.is_some() {} });
                copy_file(&src, &dst, CopyOptions::default(), ctrl, tx)
                    .await
                    .expect("copy");
                drain.abort();
            });
        });
    });

    group.finish();
}

// --- Buffer-size sweep (Phase 13b tuning) -----------------------------

fn buffer_size_sweep(c: &mut Criterion) {
    // Smaller sample per point — the sweep has 5 data points so
    // the total runtime stays reasonable even with a big workload.
    let size: usize = if ci_mode() {
        50 * 1024 * 1024 // 50 MiB per data point, 5 points → 250 MiB total
    } else {
        200 * 1024 * 1024 // 200 MiB per data point, 5 points → 1 GiB total
    };

    let tmp = tempdir().expect("tempdir");
    let src = tmp.path().join("sweep.bin");
    write_random_file(&src, size);
    let dst_root = dst_override_or(tmp.path().join("dst"));
    std::fs::create_dir_all(&dst_root).ok();

    let mut group = c.benchmark_group("buffer_size_sweep");
    group.sample_size(10);
    group.throughput(Throughput::Bytes(size as u64));

    for &buf_kb in &[64usize, 256, 1024, 4096, 16 * 1024] {
        let buffer_size = buf_kb * 1024;
        group.bench_with_input(
            BenchmarkId::new("buffer-kb", buf_kb),
            &buffer_size,
            |b, &buffer_size| {
                let rt = runtime();
                b.iter(|| {
                    let dst = dst_root.join(format!("sweep-{buf_kb}kb.out"));
                    let _ = std::fs::remove_file(&dst);
                    rt.block_on(async {
                        let opts = CopyOptions {
                            buffer_size,
                            ..Default::default()
                        };
                        let ctrl = CopyControl::new();
                        let (tx, mut rx) = mpsc::channel(32);
                        let drain =
                            tokio::spawn(async move { while rx.recv().await.is_some() {} });
                        copy_file(&src, &dst, opts, ctrl, tx).await.expect("copy");
                        drain.abort();
                    });
                });
            },
        );
    }

    group.finish();
}

// --- Many small files -------------------------------------------------

fn many_small_files(c: &mut Criterion) {
    // Phase 13b-2 — the name stays "small files" but the size
    // spread mirrors what users actually throw at the app:
    // a mix of tiny (10 KiB) dotfiles, medium (100 KiB, 1 MiB)
    // photos/documents, and a few large (10 MiB) media clips.
    // CI mode keeps the spread but trims the count.
    const SIZES: [usize; 4] = [10 * 1024, 100 * 1024, 1024 * 1024, 10 * 1024 * 1024];
    let count: usize = if ci_mode() { 200 } else { 2_000 };
    let sizes: &[usize] = &SIZES;

    let tmp = tempdir().expect("tempdir");
    let src_dir = tmp.path().join("src");
    std::fs::create_dir_all(&src_dir).ok();
    let mut total_bytes: u64 = 0;
    for i in 0..count {
        let size = sizes[i % sizes.len()];
        write_random_file(&src_dir.join(format!("f{i:06}.bin")), size);
        total_bytes += size as u64;
    }
    let dst_root = dst_override_or(tmp.path().join("dst"));

    let mut group = c.benchmark_group("many_small_files");
    group.sample_size(10);
    group.throughput(Throughput::Bytes(total_bytes));

    group.bench_function("default-options", |b| {
        let rt = runtime();
        b.iter(|| {
            // Fresh destination each run so we're timing the tree
            // copy, not a no-op.
            let _ = std::fs::remove_dir_all(&dst_root);
            std::fs::create_dir_all(&dst_root).ok();
            rt.block_on(async {
                let ctrl = CopyControl::new();
                let (tx, mut rx) = mpsc::channel(1024);
                // Drain events on a background task so the channel
                // doesn't block the engine.
                let drain = tokio::spawn(async move { while rx.recv().await.is_some() {} });
                copy_tree(&src_dir, &dst_root, TreeOptions::default(), ctrl, tx)
                    .await
                    .expect("copy_tree");
                drain.abort();
            });
        });
    });

    group.finish();
}

// --- Mixed tree -------------------------------------------------------

fn mixed_tree(c: &mut Criterion) {
    // Phase 13b-2 — wide-range size distribution so the bench
    // exercises every bucket in the engine's `buffer_size_for_file`
    // dynamic-sizing ladder. CI mode shrinks counts but keeps the
    // same size spread so regressions in any bucket (tiny, small,
    // medium, large) still surface.
    //
    // Sizes: 10 KiB, 100 KiB, 1 MiB, 10 MiB, 50 MiB, 250 MiB.
    // Subdirs × per_subdir chosen to keep the bench under ~2 min
    // on a fast SSD host (the 250 MiB bucket dominates; the
    // smaller buckets cycle in to keep metadata churn realistic).
    let subdirs = if ci_mode() { 2 } else { 4 };
    let per_subdir = if ci_mode() { 6 } else { 12 };
    let sizes: [usize; 6] = [
        10 * 1024,          // 10 KiB  — tiny
        100 * 1024,         // 100 KiB — small
        1024 * 1024,        // 1 MiB   — medium
        10 * 1024 * 1024,   // 10 MiB  — medium-large
        50 * 1024 * 1024,   // 50 MiB  — large
        250 * 1024 * 1024,  // 250 MiB — huge
    ];

    let tmp = tempdir().expect("tempdir");
    let src_root = tmp.path().join("src");
    let mut total_bytes: u64 = 0;
    for s in 0..subdirs {
        let sub = src_root.join(format!("sub{s:02}"));
        std::fs::create_dir_all(&sub).ok();
        for i in 0..per_subdir {
            let size = sizes[(s * per_subdir + i) % sizes.len()];
            write_random_file(&sub.join(format!("f{i:04}.bin")), size);
            total_bytes += size as u64;
        }
    }
    let dst_root = dst_override_or(tmp.path().join("dst"));

    let mut group = c.benchmark_group("mixed_tree");
    group.sample_size(10);
    group.throughput(Throughput::Bytes(total_bytes));

    group.bench_function("default-options", |b| {
        let rt = runtime();
        b.iter(|| {
            let _ = std::fs::remove_dir_all(&dst_root);
            std::fs::create_dir_all(&dst_root).ok();
            rt.block_on(async {
                let ctrl = CopyControl::new();
                let (tx, mut rx) = mpsc::channel(1024);
                let drain = tokio::spawn(async move { while rx.recv().await.is_some() {} });
                copy_tree(&src_root, &dst_root, TreeOptions::default(), ctrl, tx)
                    .await
                    .expect("copy_tree");
                drain.abort();
            });
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    single_huge_file,
    buffer_size_sweep,
    many_small_files,
    mixed_tree,
);
criterion_main!(benches);
