//! Phase 33 — `MountHandle` — RAII-style unmount guard.
//!
//! A `MountHandle` is returned by [`crate::MountBackend::mount`]. It
//! owns the concrete per-backend session object; when dropped, the
//! session's `unmount_on_drop` method runs. Callers that need
//! explicit unmount (reporting errors, for example) can call
//! [`MountHandle::unmount`] — the handle is consumed and `Drop` does
//! nothing afterward.

use std::path::{Path, PathBuf};

use crate::backends::MountSession;
use crate::error::MountError;

/// Opaque handle to a live mount. Dropping unmounts best-effort.
pub struct MountHandle {
    mountpoint: PathBuf,
    session: Option<Box<dyn MountSession>>,
}

impl std::fmt::Debug for MountHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MountHandle")
            .field("mountpoint", &self.mountpoint)
            .field("live", &self.session.is_some())
            .finish()
    }
}

impl MountHandle {
    /// Construct with a live session. Invoked by backend `mount`
    /// implementations; not meant for direct consumer use.
    pub fn new(mountpoint: PathBuf, session: Box<dyn MountSession>) -> Self {
        Self {
            mountpoint,
            session: Some(session),
        }
    }

    /// Where this mount is visible in the host filesystem.
    pub fn mountpoint(&self) -> &Path {
        &self.mountpoint
    }

    /// Whether the underlying session is still live. After a
    /// successful [`Self::unmount`] this returns `false`.
    pub fn is_live(&self) -> bool {
        self.session.is_some()
    }

    /// Explicitly unmount. Consumes the handle so the implicit
    /// `Drop` becomes a no-op.
    pub fn unmount(mut self) -> Result<(), MountError> {
        if let Some(mut session) = self.session.take() {
            session.unmount_on_drop()?;
        }
        Ok(())
    }
}

impl Drop for MountHandle {
    fn drop(&mut self) {
        if let Some(mut session) = self.session.take() {
            // Best-effort: swallow the error. Callers that care
            // about the unmount outcome should call `unmount`
            // explicitly.
            let _ = session.unmount_on_drop();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::MountBackend;
    use crate::backends::NoopBackend;

    #[test]
    fn drop_invokes_unmount() {
        let backend = NoopBackend::default();
        let tmp = tempfile::tempdir().expect("tempdir");
        let handle = backend
            .mount(tmp.path(), crate::MountLayout::all(), &crate::backends::ArchiveRefs::default())
            .expect("mount");
        assert!(handle.is_live());
        let counter = backend.unmount_counter();
        assert_eq!(*counter.lock().expect("lock"), 0);
        drop(handle);
        assert_eq!(*counter.lock().expect("lock"), 1);
    }

    #[test]
    fn explicit_unmount_consumes_handle() {
        let backend = NoopBackend::default();
        let tmp = tempfile::tempdir().expect("tempdir");
        let handle = backend
            .mount(tmp.path(), crate::MountLayout::all(), &crate::backends::ArchiveRefs::default())
            .expect("mount");
        handle.unmount().expect("unmount");
        let counter = backend.unmount_counter();
        assert_eq!(*counter.lock().expect("lock"), 1);
    }
}
