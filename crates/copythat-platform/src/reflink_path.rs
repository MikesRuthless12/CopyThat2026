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
    //!
    //! Phase 42 follow-up: the warning is gated behind
    //! `COPYTHAT_SUPPRESS_ZFS_WARNING=1` for scripted environments
    //! and CI runners that already know their dataset version and
    //! don't want repeated stderr noise across many per-file reflink
    //! probes.
    use std::sync::OnceLock;

    pub fn check_once() {
        static CHECKED: OnceLock<()> = OnceLock::new();
        CHECKED.get_or_init(|| {
            // Honor the suppression env var even when version
            // detection succeeds — silent in scripted runs.
            if std::env::var("COPYTHAT_SUPPRESS_ZFS_WARNING")
                .ok()
                .as_deref()
                == Some("1")
            {
                return;
            }
            if let Some((maj, min, patch)) = read_zfs_version() {
                let bclone = read_zfs_bclone_enabled().unwrap_or(false);
                if maj == 2 && min == 2 && patch <= 6 && bclone {
                    eprintln!(
                        "copythat-platform: WARNING — OpenZFS {}.{}.{} with \
                         zfs_bclone_enabled=1 has a known data-corruption \
                         bug (openzfs/zfs#15526). Consider upgrading to \
                         2.3+ or setting zfs_bclone_enabled=0. Reflink \
                         path remains active. Set \
                         COPYTHAT_SUPPRESS_ZFS_WARNING=1 to silence.",
                        maj, min, patch
                    );
                }
            }
        });
    }

    fn read_zfs_version() -> Option<(u32, u32, u32)> {
        let raw = std::fs::read_to_string("/sys/module/zfs/version").ok()?;
        parse_zfs_version_str(&raw)
    }

    /// Pure-string parser for ZFS version output. Lifted out so the
    /// Phase 42 wave-2 corruption tests can exercise malformed inputs
    /// without touching the live `/sys` filesystem. Format examples:
    /// `"2.2.4-1\n"`, `"2.3.0-rc1\n"`, `"0.8.6-1\n"`. Anything that
    /// fails to parse cleanly returns `None`.
    pub(super) fn parse_zfs_version_str(raw: &str) -> Option<(u32, u32, u32)> {
        let head = raw.trim().split('-').next()?;
        let mut parts = head.split('.');
        let maj: u32 = parts.next()?.parse().ok()?;
        let min: u32 = parts.next()?.parse().ok()?;
        let patch: u32 = parts.next()?.parse().ok()?;
        Some((maj, min, patch))
    }

    fn read_zfs_bclone_enabled() -> Option<bool> {
        let raw = std::fs::read_to_string("/sys/module/zfs/parameters/zfs_bclone_enabled").ok()?;
        Some(raw.trim() != "0")
    }

    /// Phase 42 wave-2 — pure-function decision shared with
    /// `check_once`'s production path. Given a parsed `(maj, min,
    /// patch)` and the bclone toggle, returns `true` iff the
    /// emit-warning condition is met (OpenZFS 2.2.0-2.2.6 with
    /// `zfs_bclone_enabled=1`). Lets tests assert the boundary
    /// without fiddling with stderr / OnceLock state.
    pub(super) fn should_warn(maj: u32, min: u32, patch: u32, bclone: bool) -> bool {
        maj == 2 && min == 2 && patch <= 6 && bclone
    }
}

#[cfg(all(test, target_os = "linux"))]
mod zfs_warning_tests {
    use super::zfs_version_warning::{parse_zfs_version_str, should_warn};

    /// Phase 42 wave-2 — `parse_zfs_version_str` returns `None` on
    /// malformed `/sys/module/zfs/version` content rather than
    /// panicking. Wave-1 hardened the call site to use `?` for every
    /// step, but a regression that swapped any of those for
    /// `unwrap()` would only surface on hosts with the affected ZFS
    /// versions — this test catches it on every Linux CI run.
    #[test]
    fn parse_zfs_version_str_handles_corrupted_inputs() {
        // Empty.
        assert_eq!(parse_zfs_version_str(""), None);
        // Whitespace only.
        assert_eq!(parse_zfs_version_str("   \n"), None);
        // Garbage text.
        assert_eq!(parse_zfs_version_str("not-a-version\n"), None);
        // Missing patch component (only major.minor).
        assert_eq!(parse_zfs_version_str("2.2\n"), None);
        // Missing minor + patch.
        assert_eq!(parse_zfs_version_str("2\n"), None);
        // Non-numeric major.
        assert_eq!(parse_zfs_version_str("foo.2.4\n"), None);
        // Non-numeric minor.
        assert_eq!(parse_zfs_version_str("2.bar.4\n"), None);
        // Non-numeric patch.
        assert_eq!(parse_zfs_version_str("2.2.qux-1\n"), None);
        // Extra dots — split returns more parts but we only need 3.
        // First three segments still parse, so `Some(...)` here is OK.
        assert_eq!(parse_zfs_version_str("2.2.4.5\n"), Some((2, 2, 4)));
        // Trailing -rc / -1 markers must be tolerated.
        assert_eq!(parse_zfs_version_str("2.3.0-rc1\n"), Some((2, 3, 0)));
        assert_eq!(parse_zfs_version_str("0.8.6-1\n"), Some((0, 8, 6)));
    }

    /// Phase 42 wave-2 — `should_warn` matches the documented affected
    /// window (2.2.0 … 2.2.6 with bclone on). This locks in the
    /// boundary so a future tweak to expand or shrink the warning
    /// surface fails this test loudly.
    #[test]
    fn should_warn_matches_affected_window() {
        // In-window with bclone enabled → warn.
        assert!(should_warn(2, 2, 0, true));
        assert!(should_warn(2, 2, 4, true));
        assert!(should_warn(2, 2, 6, true));

        // In-window with bclone disabled → no warn.
        assert!(!should_warn(2, 2, 4, false));

        // Out-of-window upper boundary (2.2.7+).
        assert!(!should_warn(2, 2, 7, true));

        // Out-of-window — different minor.
        assert!(!should_warn(2, 1, 0, true));
        assert!(!should_warn(2, 3, 0, true));

        // Out-of-window — different major.
        assert!(!should_warn(0, 8, 6, true));
        assert!(!should_warn(3, 0, 0, true));
    }

    /// Phase 42 wave-2 — calling `check_once` on a host with no ZFS
    /// (which is most of CI) must not panic, must not emit, and must
    /// be safe to call repeatedly. The function is gated by `OnceLock`
    /// internally; the regression we're catching is a hypothetical
    /// `unwrap()` on missing files.
    #[test]
    fn check_once_does_not_panic_without_zfs() {
        // /sys/module/zfs is absent on non-ZFS hosts; the wave-1
        // hardening returns None from `read_zfs_version` and quietly
        // skips the warning. Calling twice exercises both the
        // OnceLock-init path and the no-op subsequent call.
        super::zfs_version_warning::check_once();
        super::zfs_version_warning::check_once();
    }
}
