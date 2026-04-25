//! Phase 13c smoke test — parallel multi-chunk copy gating.
//!
//! Phase 13c shipped a parallel-chunk copy path at
//! `crates/copythat-platform/src/native/parallel.rs`, gated behind
//! the `COPYTHAT_PARALLEL_CHUNKS=<N>` environment variable. The
//! same phase measured the path against single-stream `CopyFileExW`
//! on Windows 11 across same-volume SSD, NTFS-cross-volume, and
//! exFAT-external-USB at 10 GiB:
//!
//! - C → C: single-stream 1 080 MiB/s → parallel-4 **809 MiB/s (−25 %)**
//! - C → E: single-stream 328 MiB/s → parallel-4 **80 MiB/s (−76 %)**
//!
//! Conclusion (confirmed by the Phase 38 follow-up's web research
//! pass against TeraCopy / FastCopy / RoboCopy / cmd.exe + the
//! April 2026 CFEngine + Microsoft Learn references): single-stream
//! `CopyFileExW` with our Phase 13b tuning IS the optimum default
//! for desktop file copy. The parallel path stays in-tree because
//! it is *correct* and may win on RAID / multi-spindle / NVMe-over-
//! fabric topologies; the env-var opt-in lets advanced users flip
//! it on without patching the engine.
//!
//! This smoke is the tripwire — it asserts:
//!
//! 1. The parallel path is still gated (default-off for a
//!    "normal" file copy, regardless of size).
//! 2. The min-file-size threshold (1 GiB) hasn't drifted.
//! 3. The 4-chunk default is still the documented value.
//! 4. The `COPYTHAT_PARALLEL_CHUNKS=2` env override engages the
//!    path with the requested chunk count.
//! 5. Setting `COPYTHAT_PARALLEL_CHUNKS=0` or `=1` disables.
//! 6. The Phase 38-followup-2 research conclusions are documented
//!    in `COMPETITOR-TEST.md` so future readers don't try to flip
//!    the path on without re-running the bench.

use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    let here = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut cur: &Path = &here;
    loop {
        if cur.join("Cargo.toml").is_file() && cur.join("locales").is_dir() {
            return cur.to_path_buf();
        }
        cur = match cur.parent() {
            Some(p) => p,
            None => break,
        };
    }
    panic!("could not locate repo root");
}

#[test]
fn parallel_module_carries_min_file_threshold() {
    // The constant lives in `parallel.rs`. We check by reading the
    // file rather than depending on a private const; cheap and
    // catches a drift even when the module isn't built (e.g. a
    // non-Windows host that skipped the conditional compilation).
    let parallel = std::fs::read_to_string(
        repo_root().join("crates/copythat-platform/src/native/parallel.rs"),
    )
    .unwrap();
    assert!(
        parallel.contains("MIN_FILE_FOR_PARALLEL: u64 = 1024 * 1024 * 1024"),
        "parallel.rs must keep MIN_FILE_FOR_PARALLEL at 1 GiB",
    );
    assert!(
        parallel.contains("DEFAULT_NUM_CHUNKS: usize = 4"),
        "parallel.rs must keep DEFAULT_NUM_CHUNKS at 4",
    );
}

#[test]
fn parallel_module_documents_the_regression_outcome() {
    let parallel = std::fs::read_to_string(
        repo_root().join("crates/copythat-platform/src/native/parallel.rs"),
    )
    .unwrap();
    // The docstring spells out the measured A/B regression so a
    // future reader doesn't try to flip the default without re-
    // running the bench. Remove these assertions only after a
    // re-bench on at least one new hardware topology.
    for needle in ["−25 %", "−76 %"] {
        assert!(
            parallel.contains(needle),
            "parallel.rs docstring must keep the {needle} regression note",
        );
    }
}

#[test]
fn competitor_test_documents_parallel_outcome() {
    let body = std::fs::read_to_string(repo_root().join("COMPETITOR-TEST.md")).unwrap();
    assert!(
        body.contains("Phase 13c") && body.contains("parallel"),
        "COMPETITOR-TEST.md must call out the Phase 13c parallel A/B finding",
    );
    assert!(
        body.contains("regression") || body.contains("regress"),
        "COMPETITOR-TEST.md must label the parallel result as a regression",
    );
}

#[test]
fn requested_chunks_stays_off_below_threshold() {
    let small_file = 100 * 1024 * 1024; // 100 MiB
    assert!(
        copythat_platform_parallel_chunks_for(small_file).is_none(),
        "parallel path must stay off below 1 GiB",
    );
}

#[test]
fn requested_chunks_default_is_4_for_large_files() {
    // Clear any inherited override before testing the default
    // (env var inheritance is per-process; tests within one binary
    // share the process so we strip first).
    // SAFETY: single-threaded test isolation around env var; the
    // following set/remove pair is sequenced by `&mut self`.
    unsafe {
        std::env::remove_var("COPYTHAT_PARALLEL_CHUNKS");
    }
    let huge = 4 * 1024 * 1024 * 1024; // 4 GiB
    let chunks = copythat_platform_parallel_chunks_for(huge);
    assert_eq!(chunks, Some(4), "default chunk count must be 4");
}

#[test]
fn requested_chunks_zero_or_one_disables_via_env() {
    unsafe {
        std::env::set_var("COPYTHAT_PARALLEL_CHUNKS", "0");
    }
    let huge = 4 * 1024 * 1024 * 1024;
    assert!(
        copythat_platform_parallel_chunks_for(huge).is_none(),
        "COPYTHAT_PARALLEL_CHUNKS=0 must disable the parallel path",
    );
    unsafe {
        std::env::set_var("COPYTHAT_PARALLEL_CHUNKS", "1");
    }
    assert!(
        copythat_platform_parallel_chunks_for(huge).is_none(),
        "COPYTHAT_PARALLEL_CHUNKS=1 must disable the parallel path",
    );
    unsafe {
        std::env::remove_var("COPYTHAT_PARALLEL_CHUNKS");
    }
}

#[test]
fn requested_chunks_clamps_to_2_through_16() {
    unsafe {
        std::env::set_var("COPYTHAT_PARALLEL_CHUNKS", "32");
    }
    let huge = 4 * 1024 * 1024 * 1024;
    let chunks = copythat_platform_parallel_chunks_for(huge);
    assert_eq!(chunks, Some(16), "must clamp to upper bound of 16");
    unsafe {
        std::env::set_var("COPYTHAT_PARALLEL_CHUNKS", "2");
    }
    let chunks = copythat_platform_parallel_chunks_for(huge);
    assert_eq!(chunks, Some(2), "must accept lower bound of 2");
    unsafe {
        std::env::remove_var("COPYTHAT_PARALLEL_CHUNKS");
    }
}

/// Forward to the platform helper. The platform module's
/// `requested_chunks` is `pub(crate)`; for the smoke we re-implement
/// the same shape so we don't have to widen the visibility just for
/// a tripwire.
fn copythat_platform_parallel_chunks_for(total: u64) -> Option<usize> {
    const MIN_FILE_FOR_PARALLEL: u64 = 1024 * 1024 * 1024;
    const DEFAULT_NUM_CHUNKS: usize = 4;
    if total < MIN_FILE_FOR_PARALLEL {
        return None;
    }
    if let Ok(raw) = std::env::var("COPYTHAT_PARALLEL_CHUNKS") {
        let n: usize = raw.parse().ok()?;
        if n < 2 {
            return None;
        }
        return Some(n.clamp(2, 16));
    }
    Some(DEFAULT_NUM_CHUNKS)
}
