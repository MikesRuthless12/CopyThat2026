//! Phase 42 — `compio` migration scaffolding (experimental).
//!
//! `compio` is a Rust async runtime built directly on IOCP (Windows)
//! and io_uring (Linux). For our overlapped-pipeline path
//! (`windows_overlapped.rs`) it would replace the hand-rolled IOCP
//! loop + `OpCtx` boxes + `OVERLAPPED` plumbing with a maintained
//! runtime — the same code shape that `compio-fs::File::read_at` /
//! `write_at` give for free.
//!
//! ## Why this is gated, not on by default
//!
//! 1. **Shipping default works.** Phase 41 measured the hand-rolled
//!    overlapped path beating Robocopy on the canonical
//!    cross-volume USB-SSD scenario. compio would simplify the
//!    code, not speed it up — `CopyFileExW` is the perf default
//!    anyway, and the overlapped path is only engaged for
//!    cross-volume large copies.
//!
//! 2. **Migration is non-trivial.** compio has its own runtime
//!    bootstrap; it can't be casually added to a Tokio-driven
//!    process without thinking through how the two runtimes share
//!    threads. The shipping engine is a Tokio app; introducing
//!    compio means either:
//!      - running compio on a dedicated thread and bridging via
//!        channels (what we'd do — minimal coupling), or
//!      - adopting `compio-tokio` for combined runtime hosting
//!        (more invasive, more upside).
//!
//! 3. **A/B benchmarks needed first.** The Phase 42 swarm's async
//!    research agent put compio at 2-4× faster than `tokio::fs` on
//!    small-file workloads but ~equal on large sequential. Our
//!    overlapped path runs only on large sequential — so the
//!    expected delta is ~0 %. Worth confirming before flipping
//!    defaults.
//!
//! ## Migration plan (Phase 43)
//!
//! - [ ] Build a `compio_overlapped::try_overlapped_copy_compio`
//!       function with the same signature as
//!       `windows_overlapped::try_overlapped_copy`.
//! - [ ] Use `compio::fs::File::open` + `read_at` / `write_at` to
//!       drive `N_SLOTS` × `BUFFER_BYTES` in flight, mirroring the
//!       hand-rolled loop's pipelining.
//! - [ ] Add an env-var toggle (`COPYTHAT_COMPIO_OVERLAPPED=1`) so
//!       users can A/B without rebuilding.
//! - [ ] Add a smoke test that copies a 256 MiB file via the compio
//!       path and verifies byte-equal output.
//! - [ ] Run `xtask bench-vs` head-to-head; if compio is within
//!       ±5 % of the hand-rolled path on every measured topology,
//!       flip the default.
//! - [ ] Once the default is flipped, delete `windows_overlapped.rs`
//!       (the OpCtx / OVERLAPPED / IOCP machinery becomes dead code).
//!
//! Until then this module is scaffolding only — it documents the
//! plan and reserves the `compio` dependency slot under the
//! `compio-experimental` feature flag.

#![cfg(feature = "compio-experimental")]

// Intentional: no implementation yet. The `compio` crate is pulled
// in via `Cargo.toml` so a future `cargo build --features
// compio-experimental` succeeds; the actual `try_overlapped_copy_compio`
// implementation lands in Phase 43 per the plan above.

#[cfg(test)]
mod tests {
    #[test]
    fn module_compiles_under_feature_flag() {
        // Smoke test that the module is visible when the feature is on.
    }
}
