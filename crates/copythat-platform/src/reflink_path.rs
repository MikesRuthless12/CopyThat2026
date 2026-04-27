//! Reflink (block-clone) attempt via the `reflink-copy` crate.
//!
//! The crate handles the per-OS syscall: `ioctl(FICLONE)` on Linux,
//! `clonefile()` on macOS, `FSCTL_DUPLICATE_EXTENTS_TO_FILE` on
//! Windows ReFS / Dev Drives. Errors split into "not supported on this
//! filesystem" (we fall through) versus real I/O errors (we propagate).

use std::io;
use std::path::{Path, PathBuf};

/// Result of a reflink attempt.
#[derive(Debug)]
pub(crate) enum ReflinkOutcome {
    /// Block-clone succeeded; the destination is byte-identical and
    /// shares extents with the source until one of them is modified.
    Cloned,
    /// The filesystem does not support cross-extent cloning. Caller
    /// should try the next strategy.
    NotSupported,
    /// A real I/O error (permission, ENOSPC, …) — propagate.
    Io(io::Error),
}

/// Attempt to reflink `src` into `dst`.
///
/// Runs the syscall on a blocking thread so the async runtime stays
/// responsive on slow / contended filesystems. The reflink-copy crate
/// presents `ErrorKind::Unsupported` and `ErrorKind::InvalidInput` for
/// the "not supported" case across all three OSes; everything else is
/// treated as a real failure.
pub(crate) async fn try_reflink(src: PathBuf, dst: PathBuf) -> ReflinkOutcome {
    // Phase 42 — emit a one-shot warning on Linux hosts running
    // OpenZFS 2.2.0-2.2.6 with `zfs_bclone_enabled=1`, where
    // block-cloning has a known data-corruption bug
    // (openzfs/zfs#15526). The reflink path is otherwise our
    // fastest fallback; we don't disable it, but users deserve a
    // heads-up on the affected versions.
    #[cfg(target_os = "linux")]
    zfs_version_warning::check_once();

    // The reflink call is synchronous; spawn_blocking keeps the runtime
    // free if the kernel decides to actually copy bytes (some
    // filesystems implement reflink as a sub-second large clone, not a
    // pure metadata flip).
    let join = tokio::task::spawn_blocking(move || reflink_inner(&src, &dst)).await;

    match join {
        Ok(Ok(())) => ReflinkOutcome::Cloned,
        Ok(Err(e)) if is_unsupported(&e) => ReflinkOutcome::NotSupported,
        Ok(Err(e)) => ReflinkOutcome::Io(e),
        Err(join_err) => ReflinkOutcome::Io(io::Error::other(format!(
            "reflink spawn_blocking panicked: {join_err}"
        ))),
    }
}

fn reflink_inner(src: &Path, dst: &Path) -> io::Result<()> {
    // reflink-copy returns `()` on success. A missing destination is
    // created by the underlying syscall; an existing destination is
    // truncated and overwritten on Linux/macOS, while Windows refuses —
    // unlink first to keep the cross-platform contract identical to
    // `copy_file`.
    if dst.exists() {
        // best-effort; ignore failure (race between exists() and unlink)
        let _ = std::fs::remove_file(dst);
    }
    reflink_copy::reflink(src, dst)
}

fn is_unsupported(err: &io::Error) -> bool {
    // Reflink failures break into two camps:
    //
    // - "this filesystem can't COW" — the syscall surfaces ENOTSUP /
    //   EXDEV / EOPNOTSUPP on Linux/macOS, and Windows returns
    //   ERROR_INVALID_FUNCTION (1, often wrapped as HRESULT
    //   0x80070001) / ERROR_NOT_SUPPORTED (50) on plain NTFS. We
    //   want the dispatcher to silently move on to the next
    //   strategy — that's the entire point of a fast-path fall-through.
    //
    // - real I/O errors (NotFound, PermissionDenied, StorageFull,
    //   AlreadyExists) — propagate so the user sees the actual
    //   problem instead of a confusing "fell back to async" message
    //   that buries the same error.

    // Phase 42 — explicit raw-os-error handling for the
    // "wrong-volume" / "wrong-FS" sentinels. These previously fell
    // into the catch-all bucket; naming them outright makes the
    // dispatch path easier to audit.
    if let Some(code) = err.raw_os_error() {
        // Linux/macOS: EXDEV (cross-device link) = 18; ENOTSUP/EOPNOTSUPP
        // vary by platform (95 on Linux, 102 on macOS).
        #[cfg(unix)]
        if matches!(code, 18 | 95 | 102) {
            return true;
        }
        // Windows: ERROR_INVALID_FUNCTION (1) and ERROR_NOT_SUPPORTED (50)
        // are both "this FS doesn't COW".
        #[cfg(windows)]
        if matches!(code, 1 | 50) {
            return true;
        }
    }

    use io::ErrorKind::*;
    if matches!(
        err.kind(),
        NotFound | PermissionDenied | StorageFull | AlreadyExists | OutOfMemory
    ) {
        return false;
    }
    // Some Windows reflink failures arrive as InvalidInput / Unsupported;
    // many arrive as Other with raw_os_error = 1 / 50. Anything that's
    // not one of the propagatable kinds above is fall-through-worthy.
    true
}

#[cfg(target_os = "linux")]
mod zfs_version_warning {
    //! Phase 42 — one-shot OpenZFS 2.2.0-2.2.6 warning.
    //!
    //! The block-cloning code path that landed in 2.2.0 has a known
    //! data-corruption bug (openzfs/zfs#15526). It's disabled by
    //! default on those versions; only fires when the user has
    //! enabled `zfs_bclone_enabled=1` in module params. We emit a
    //! single stderr warning per process if we detect the affected
    //! window; we do NOT block the reflink path — Linux ZFS users
    //! who haven't toggled the param are perfectly safe and the
    //! same code paths benefit other COW filesystems (Btrfs, XFS,
    //! bcachefs).
    use std::sync::OnceLock;

    pub fn check_once() {
        static CHECKED: OnceLock<()> = OnceLock::new();
        CHECKED.get_or_init(|| {
            if let Some((maj, min, patch)) = read_zfs_version() {
                let bclone = read_zfs_bclone_enabled().unwrap_or(false);
                if maj == 2 && min == 2 && patch <= 6 && bclone {
                    eprintln!(
                        "copythat-platform: WARNING — OpenZFS {}.{}.{} with \
                         zfs_bclone_enabled=1 has a known data-corruption \
                         bug (openzfs/zfs#15526). Consider upgrading to \
                         2.3+ or setting zfs_bclone_enabled=0. Reflink \
                         path remains active.",
                        maj, min, patch
                    );
                }
            }
        });
    }

    fn read_zfs_version() -> Option<(u32, u32, u32)> {
        let raw = std::fs::read_to_string("/sys/module/zfs/version").ok()?;
        // Format examples: "2.2.4-1\n", "2.3.0-rc1\n", "0.8.6-1\n".
        let head = raw.trim().split('-').next()?;
        let mut parts = head.split('.');
        let maj: u32 = parts.next()?.parse().ok()?;
        let min: u32 = parts.next()?.parse().ok()?;
        let patch: u32 = parts.next()?.parse().ok()?;
        Some((maj, min, patch))
    }

    fn read_zfs_bclone_enabled() -> Option<bool> {
        let raw = std::fs::read_to_string(
            "/sys/module/zfs/parameters/zfs_bclone_enabled",
        )
        .ok()?;
        Some(raw.trim() != "0")
    }
}
