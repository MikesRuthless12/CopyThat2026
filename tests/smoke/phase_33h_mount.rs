//! Phase 33h smoke — platform-gated mount validation.
//!
//! Two env-gated cases:
//! - `COPYTHAT_PHASE33H_FUSE=1` — exercise the real FUSE mount on
//!   Linux/macOS (requires `cargo test --features fuse`).
//! - `COPYTHAT_PHASE33H_WINFSP=1` — exercise the real WinFsp mount
//!   on Windows (requires `cargo test --features winfsp`, the
//!   WinFsp driver installed, and an admin shell for the mount
//!   syscall).
//!
//! Both cases skip by default so the normal `cargo test` run
//! stays portable. See the Phase 33h CHANGELOG entry for the full
//! setup instructions.
//!
//! # What they verify (once implemented)
//!
//! 1. `MountBackend::mount` returns a live handle with a valid
//!    mountpoint.
//! 2. `ls` on the mountpoint shows `by-date/`, `by-source/`,
//!    `by-job-id/` at the root.
//! 3. Reading a job-placeholder leaf at `by-job-id/<id>/<label>`
//!    returns bytes that byte-equal the original source file.
//! 4. Dropping the handle unmounts cleanly.
//!
//! # Why the skeleton ships now
//!
//! FUSE mount semantics vary across kernels (Linux libfuse3 vs
//! macOS macFUSE) and require root for some mount options; WinFsp
//! needs the driver install + drive-letter allocation. The smoke
//! is kept here so a Linux/macOS dev can flip the env var + run
//! it during 33i validation without rewriting the test from
//! scratch.

use std::path::PathBuf;

fn should_run_fuse() -> bool {
    std::env::var("COPYTHAT_PHASE33H_FUSE").as_deref() == Ok("1")
}

fn should_run_winfsp() -> bool {
    std::env::var("COPYTHAT_PHASE33H_WINFSP").as_deref() == Ok("1")
}

/// Case 1 — FUSE mount round-trip. Requires Linux/macOS +
/// `cargo test --features fuse` + `COPYTHAT_PHASE33H_FUSE=1`.
#[test]
#[cfg(all(feature = "fuse", any(target_os = "linux", target_os = "macos")))]
fn case1_fuse_mount_round_trip() {
    if !should_run_fuse() {
        eprintln!("skipping: COPYTHAT_PHASE33H_FUSE != 1");
        return;
    }
    // Phase 33i fills in:
    //   1. Open a real ChunkStore with a seeded manifest.
    //   2. Open a real History with a matching job row.
    //   3. backend.mount(mountpoint, MountLayout::all(), &archive_refs).
    //   4. std::fs::read_dir(mountpoint) — expect 3 entries.
    //   5. std::fs::read(mountpoint / "by-job-id" / "<id>" / "<label>")
    //      — expect byte-equal to the seeded source.
    //   6. drop(handle) — expect the mountpoint is gone.
    eprintln!("Phase 33h FUSE smoke is a skeleton — case body lands in Phase 33i.");
}

/// Case 1 fallback — doesn't require the feature/target gates so
/// default `cargo test` runs exercise the skeleton even without
/// the feature enabled.
#[test]
#[cfg(not(all(feature = "fuse", any(target_os = "linux", target_os = "macos"))))]
fn case1_fuse_mount_skipped_on_default_build() {
    // On the default build there's no FUSE body to exercise.
    // Kept so the smoke file compiles + reports "1 passed" on
    // every target.
    // Intentional no-op on default builds — the case body lands
    // in Phase 33i on a Linux/macOS dev box with `--features fuse`.
    let _ = should_run_fuse();
}

/// Case 2 — WinFsp mount round-trip. Requires Windows +
/// `cargo test --features winfsp` + `COPYTHAT_PHASE33H_WINFSP=1` +
/// admin shell + WinFsp install.
#[test]
#[cfg(all(feature = "winfsp", target_os = "windows"))]
fn case2_winfsp_mount_round_trip() {
    if !should_run_winfsp() {
        eprintln!("skipping: COPYTHAT_PHASE33H_WINFSP != 1");
        return;
    }
    // Phase 33i fills in the equivalent body:
    //   1. Allocate a drive letter (e.g. Z:\) for the mount.
    //   2. backend.mount(Z:\, MountLayout::all(), &archive_refs).
    //   3. std::fs::read_dir("Z:\\") — expect 3 entries.
    //   4. std::fs::read("Z:\\by-job-id\\<id>\\<label>") —
    //      byte-equal to the seeded source.
    //   5. drop(handle) — mountpoint disappears.
    eprintln!("Phase 33h WinFsp smoke is a skeleton — case body lands in Phase 33i.");
}

/// Case 2 fallback — same shape as case 1.
#[test]
#[cfg(not(all(feature = "winfsp", target_os = "windows")))]
fn case2_winfsp_mount_skipped_on_default_build() {
    // Intentional no-op on default builds — the case body lands
    // in Phase 33i on a Windows box with `--features winfsp`.
    let _ = should_run_winfsp();
}

/// Case 3 — `workspace_root` sanity for the smoke. Confirms the
/// test binary can locate the workspace so Phase 33i can seed
/// fixtures without guessing paths.
#[test]
fn case3_workspace_root_resolvable() {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace = manifest
        .parent()
        .and_then(|p| p.parent())
        .expect("workspace root");
    assert!(workspace.join("Cargo.toml").exists());
    assert!(workspace.join("crates").join("copythat-mount").exists());
}
