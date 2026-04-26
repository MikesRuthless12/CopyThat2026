//! `copythat-snapshot` — filesystem-snapshot source for locked files.
//!
//! When a copy engine hits a shared-violation / busy error reading the
//! source — Windows' `ERROR_SHARING_VIOLATION`, Linux's `EBUSY` on
//! certain FUSE mounts, macOS' `NSFileLockingError` — it can ask this
//! crate for a snapshot of the source volume, translate the original
//! path into a path inside the snapshot mount, and retry the open
//! against the snapshot. The snapshot is released via RAII when the
//! returned [`SnapshotHandle`] is dropped.
//!
//! # Per-OS backends
//!
//! | Backend                | How it works                                                                                                                                                         |
//! | ---------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------|
//! | [`SnapshotKind::Vss`]  | Windows Volume Shadow Copy Service. The privileged work runs in a separate `copythat-helper-vss` binary that the main process spawns via `ShellExecute("runas", ...)`. The helper speaks JSON-RPC over stdin/stdout so the main app never needs a UAC prompt of its own. |
//! | [`SnapshotKind::Zfs`]  | `zfs snapshot <dataset>@copythat-<uuid>` + `zfs mount`. Cleanup unmounts and destroys.                                                                               |
//! | [`SnapshotKind::Btrfs`]| `btrfs subvolume snapshot -r <subvol> <snap-path>`. Cleanup via `btrfs subvolume delete`.                                                                            |
//! | [`SnapshotKind::Apfs`] | `tmutil localsnapshot` + `mount_apfs -o nobrowse`. Cleanup via `tmutil deletelocalsnapshots`.                                                                        |
//!
//! [`capabilities`] returns the set of backends applicable to a given
//! path — typically exactly one (the filesystem the path lives on) or
//! zero (the filesystem has no snapshot primitive this crate knows
//! about). The engine dispatches on the first element.
//!
//! # Example
//!
//! ```no_run
//! use copythat_snapshot::{create_snapshot, translate_path};
//! use std::path::Path;
//!
//! # async fn demo() -> Result<(), copythat_snapshot::SnapshotError> {
//! // A source file the OS just told us is locked by another process.
//! let original = Path::new("/home/alice/vm.qcow2");
//! let snap = create_snapshot(original).await?;
//! let snapshot_path = translate_path(&snap, original)
//!     .expect("path is under the snapshot root");
//! // open `snapshot_path` for read; when `snap` drops, the underlying
//! // snapshot is released.
//! # drop(snap);
//! # Ok(())
//! # }
//! ```

mod backends;
mod error;
mod hook;
mod kind;
#[cfg(windows)]
pub mod rpc;

/// Direct `IVssBackupComponents` COM primitives, re-exported so the
/// sibling `copythat-helper-vss` binary can call the same shape the
/// in-process snapshot path uses. Only available on Windows builds
/// with `--features vss-com` (the default). For typical consumers the
/// snapshot orchestration in [`create_snapshot`] is the right entry
/// point — these re-exports are deliberately low-level.
#[cfg(all(windows, feature = "vss-com"))]
pub mod vss_com {
    pub use crate::backends::vss_com::{create_shadow_via_com, release_shadow_via_com};
}

pub use error::SnapshotError;
pub use hook::CopyThatSnapshotHook;
pub use kind::SnapshotKind;

use std::path::{Path, PathBuf};

/// Live snapshot plus enough metadata to translate paths into it.
///
/// Dropping the handle triggers per-backend cleanup in a best-effort
/// `Drop` — failures are logged via `tracing::warn!` and swallowed so a
/// panic in release never cascades. Callers that want to *observe*
/// cleanup errors should prefer the async [`SnapshotHandle::release`]
/// method, which returns the cleanup `Result`.
#[derive(Debug)]
pub struct SnapshotHandle {
    /// Which backend minted this snapshot. Lets callers render a
    /// per-kind UI badge ("📷 Reading from VSS snapshot of C:") and
    /// decide whether to fall through to a slower retry on failure.
    pub kind: SnapshotKind,
    /// Root mount point / device path of the snapshot.
    ///
    /// - VSS: `\\?\GLOBALROOT\Device\HarddiskVolumeShadowCopyN`
    /// - ZFS: `/path/to/mount/of/<dataset>/.zfs/snapshot/<name>`
    /// - Btrfs: absolute path to the read-only snapshot subvolume
    /// - APFS: absolute path where `mount_apfs` attached the snapshot
    pub mount_path: PathBuf,
    /// The original volume / subvolume / dataset root the snapshot was
    /// taken of. [`translate_path`] replaces this prefix with
    /// `mount_path` when resolving a live-source path into a
    /// snapshot-side path.
    pub original_root: PathBuf,
    /// Backend-specific cleanup state, moved out of the handle inside
    /// `Drop` / `release`. `None` after release or after a failed
    /// create that still returned a partial handle.
    cleanup: Option<backends::Cleanup>,
}

impl SnapshotHandle {
    /// Release the snapshot and return any backend cleanup error.
    ///
    /// Equivalent to `Drop` but surfaces the `Result` instead of
    /// swallowing it. After this returns, [`SnapshotHandle::mount_path`]
    /// is no longer valid.
    pub async fn release(mut self) -> Result<(), SnapshotError> {
        if let Some(c) = self.cleanup.take() {
            backends::release(c).await
        } else {
            Ok(())
        }
    }
}

impl Drop for SnapshotHandle {
    fn drop(&mut self) {
        if let Some(c) = self.cleanup.take() {
            // We can't `.await` in Drop. Delegate to a synchronous
            // best-effort cleanup that shells out or calls the helper.
            if let Err(e) = backends::release_blocking(c) {
                tracing::warn!("copythat-snapshot: cleanup failed in Drop: {e}");
            }
        }
    }
}

/// Return every snapshot backend that can operate on `path`.
///
/// The first element (when non-empty) is the caller's best bet: it's
/// the backend matching the filesystem `path` lives on. Extra elements
/// are *possible* fallbacks — e.g. a Btrfs subvolume nested inside a
/// ZFS dataset would list `[Btrfs, Zfs]`. Typical output is
/// zero or one element.
///
/// This probe is cheap and side-effect-free: it inspects mount
/// metadata and file attributes, never creates a snapshot. Callers can
/// safely call it in hot paths (e.g. every `EBUSY` retry) to decide
/// whether the [`LockedFilePolicy::Snapshot`][engine-policy] path is
/// reachable at all.
///
/// [engine-policy]: https://github.com/MikesRuthless12/CopyThat2026 "CopyOptions::on_locked"
pub fn capabilities(path: &Path) -> Vec<SnapshotKind> {
    backends::capabilities(path)
}

/// Create a read-only snapshot covering `src_path`.
///
/// Returns an error — not a panic — when:
/// - The filesystem has no snapshot primitive this crate knows about
///   ([`SnapshotError::Unsupported`]).
/// - The current process lacks the privilege the backend needs
///   ([`SnapshotError::NeedsElevation`] — VSS-only). On Windows, the
///   main process spawns `copythat-helper-vss.exe` with the `runas`
///   verb, triggering UAC. The user's choice in the UAC dialog flows
///   back as either a successful snapshot or [`SnapshotError::UacDenied`].
/// - The backend tool is not installed on the host
///   ([`SnapshotError::BackendMissing`] — e.g. `zfs` binary not on
///   `$PATH`).
/// - The backend refused the request (disk full, writer timeout, etc.)
///   ([`SnapshotError::BackendFailure`]).
pub async fn create_snapshot(src_path: &Path) -> Result<SnapshotHandle, SnapshotError> {
    backends::create(src_path).await
}

/// Translate a path on the *live* source into its snapshot-side
/// counterpart.
///
/// Returns `None` when `original` is not under
/// [`SnapshotHandle::original_root`]. Callers should treat this as
/// "the snapshot can't help with this file" and fall back to their
/// retry / skip policy.
///
/// The translation is purely lexical — no filesystem probe. Trailing
/// separators, Unicode normalisation forms, and relative `..` segments
/// in `original` are preserved verbatim on the returned path.
pub fn translate_path(snap: &SnapshotHandle, original: &Path) -> Option<PathBuf> {
    let rel = original.strip_prefix(&snap.original_root).ok()?;
    Some(snap.mount_path.join(rel))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translate_path_strips_original_root() {
        let handle = SnapshotHandle {
            kind: SnapshotKind::Btrfs,
            mount_path: PathBuf::from("/snap/copythat-abc"),
            original_root: PathBuf::from("/home/alice"),
            cleanup: None,
        };
        assert_eq!(
            translate_path(&handle, Path::new("/home/alice/docs/report.pdf")),
            Some(PathBuf::from("/snap/copythat-abc/docs/report.pdf"))
        );
        assert_eq!(
            translate_path(&handle, Path::new("/etc/passwd")),
            None,
            "paths outside the original root must not translate"
        );
    }

    #[test]
    fn translate_path_identity_at_root() {
        let handle = SnapshotHandle {
            kind: SnapshotKind::Btrfs,
            mount_path: PathBuf::from("/snap/copythat-abc"),
            original_root: PathBuf::from("/home/alice"),
            cleanup: None,
        };
        // Snapshotting the root itself is a legitimate case.
        assert_eq!(
            translate_path(&handle, Path::new("/home/alice")),
            Some(PathBuf::from("/snap/copythat-abc"))
        );
    }
}
