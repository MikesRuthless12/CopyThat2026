//! Per-OS snapshot backends.
//!
//! The module tree is flat by design — each backend is a sibling
//! behind a `cfg` gate so the compiled artifact for a given platform
//! only carries the code it can actually run.

use std::path::Path;

use crate::SnapshotHandle;
use crate::error::SnapshotError;
use crate::kind::SnapshotKind;

#[cfg(target_os = "macos")]
mod apfs;
#[cfg(target_os = "linux")]
mod btrfs;
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
mod unix_mount;
#[cfg(windows)]
mod vss;
#[cfg(windows)]
mod win_pipe_security;
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
mod zfs;

/// Backend-specific cleanup state. Lives inside `SnapshotHandle` and
/// gets moved out on `Drop` / `release` so the backend can unmount /
/// destroy its snapshot.
#[derive(Debug)]
#[allow(dead_code)] // Variants are cfg-gated; fields read per-backend.
pub(crate) enum Cleanup {
    #[cfg(target_os = "linux")]
    Btrfs(btrfs::Cleanup),
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    Zfs(zfs::Cleanup),
    #[cfg(target_os = "macos")]
    Apfs(apfs::Cleanup),
    #[cfg(windows)]
    Vss(vss::Cleanup),
}

pub(crate) fn capabilities(path: &Path) -> Vec<SnapshotKind> {
    #[allow(unused_mut)]
    let mut caps: Vec<SnapshotKind> = Vec::new();

    #[cfg(windows)]
    if vss::applies_to(path) {
        caps.push(SnapshotKind::Vss);
    }

    #[cfg(target_os = "linux")]
    if btrfs::applies_to(path) {
        caps.push(SnapshotKind::Btrfs);
    }

    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    if zfs::applies_to(path) {
        caps.push(SnapshotKind::Zfs);
    }

    #[cfg(target_os = "macos")]
    if apfs::applies_to(path) {
        caps.push(SnapshotKind::Apfs);
    }

    let _ = path; // Silence unused on platforms with zero backends.
    caps
}

pub(crate) async fn create(src_path: &Path) -> Result<SnapshotHandle, SnapshotError> {
    let caps = capabilities(src_path);
    let Some(&first) = caps.first() else {
        return Err(SnapshotError::Unsupported {
            path: src_path.to_path_buf(),
        });
    };
    dispatch_create(first, src_path).await
}

async fn dispatch_create(
    kind: SnapshotKind,
    src_path: &Path,
) -> Result<SnapshotHandle, SnapshotError> {
    match kind {
        #[cfg(windows)]
        SnapshotKind::Vss => vss::create(src_path).await,
        #[cfg(target_os = "linux")]
        SnapshotKind::Btrfs => btrfs::create(src_path).await,
        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
        SnapshotKind::Zfs => zfs::create(src_path).await,
        #[cfg(target_os = "macos")]
        SnapshotKind::Apfs => apfs::create(src_path).await,
        // Catch-all for kinds not compiled on this platform — should be
        // unreachable because `capabilities()` is also `cfg`-gated.
        #[allow(unreachable_patterns)]
        other => Err(SnapshotError::Unsupported {
            path: {
                let _ = other;
                src_path.to_path_buf()
            },
        }),
    }
}

/// Async release path — surfaced by `SnapshotHandle::release`.
pub(crate) async fn release(cleanup: Cleanup) -> Result<(), SnapshotError> {
    match cleanup {
        #[cfg(target_os = "linux")]
        Cleanup::Btrfs(c) => btrfs::release(c).await,
        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
        Cleanup::Zfs(c) => zfs::release(c).await,
        #[cfg(target_os = "macos")]
        Cleanup::Apfs(c) => apfs::release(c).await,
        #[cfg(windows)]
        Cleanup::Vss(c) => vss::release(c).await,
    }
}

/// Synchronous best-effort release for the `Drop` path. Each backend
/// implements a short blocking teardown that doesn't depend on a
/// tokio runtime — so dropping a `SnapshotHandle` outside an async
/// context (e.g. on process exit) still cleans up.
pub(crate) fn release_blocking(cleanup: Cleanup) -> Result<(), SnapshotError> {
    match cleanup {
        #[cfg(target_os = "linux")]
        Cleanup::Btrfs(c) => btrfs::release_blocking(c),
        #[cfg(any(target_os = "linux", target_os = "freebsd"))]
        Cleanup::Zfs(c) => zfs::release_blocking(c),
        #[cfg(target_os = "macos")]
        Cleanup::Apfs(c) => apfs::release_blocking(c),
        #[cfg(windows)]
        Cleanup::Vss(c) => vss::release_blocking(c),
    }
}
