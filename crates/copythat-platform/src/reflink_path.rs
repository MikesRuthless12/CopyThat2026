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
