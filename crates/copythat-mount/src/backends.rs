//! Phase 33 — `MountBackend` trait + test-only `NoopBackend`.
//!
//! The trait is Phase 33a's contract between this crate and the
//! runner. Phase 33b lands two concrete impls behind the `fuse` and
//! `winfsp` feature flags; the crate's default build exposes only
//! [`NoopBackend`], which validates the surrounding machinery
//! (mountpoint checks, handle Drop, tree wiring) without kernel IO.

use std::path::Path;
use std::sync::{Arc, Mutex};

use copythat_core::safety::validate_path_no_traversal;

use crate::error::MountError;
use crate::handle::MountHandle;
use crate::tree::MountLayout;

/// Trait every platform backend implements. Phase 33a only ships
/// [`NoopBackend`]; real `fuser` + `winfsp` impls land in 33b.
pub trait MountBackend {
    /// Mount `layout`-filtered archive content at `mountpoint`.
    /// The returned [`MountHandle`] unmounts on Drop.
    fn mount(
        &self,
        mountpoint: &Path,
        layout: MountLayout,
    ) -> Result<MountHandle, MountError>;
}

/// Per-backend mount session. Owned by [`MountHandle`] — the
/// `unmount_on_drop` method runs at handle-drop time.
pub trait MountSession: Send {
    fn unmount_on_drop(&mut self) -> Result<(), MountError>;
}

/// Shared mountpoint-empty check. Extracted so the real 33b
/// backends can reuse it. Wraps
/// `copythat_core::safety::validate_path_no_traversal` so a caller
/// can't smuggle `../..` through the IPC boundary.
pub fn validate_mountpoint(mountpoint: &Path) -> Result<(), MountError> {
    validate_path_no_traversal(mountpoint)
        .map_err(|e| MountError::UnsafeMountpoint(e.to_string()))?;

    if mountpoint.exists() {
        if !mountpoint.is_dir() {
            return Err(MountError::MountpointNotEmpty(mountpoint.to_path_buf()));
        }
        let mut entries = std::fs::read_dir(mountpoint)?;
        if entries.next().is_some() {
            return Err(MountError::MountpointNotEmpty(mountpoint.to_path_buf()));
        }
    }
    Ok(())
}

/// Test-only backend that performs zero kernel IO. Records
/// unmount invocations so tests can assert the Drop contract.
#[derive(Debug, Default, Clone)]
pub struct NoopBackend {
    unmount_counter: Arc<Mutex<u32>>,
}

impl NoopBackend {
    /// Shared counter — bumped every time a session's
    /// `unmount_on_drop` fires.
    pub fn unmount_counter(&self) -> Arc<Mutex<u32>> {
        self.unmount_counter.clone()
    }
}

struct NoopSession {
    counter: Arc<Mutex<u32>>,
}

impl MountSession for NoopSession {
    fn unmount_on_drop(&mut self) -> Result<(), MountError> {
        let mut guard = self.counter.lock().expect("counter poisoned");
        *guard += 1;
        Ok(())
    }
}

impl MountBackend for NoopBackend {
    fn mount(
        &self,
        mountpoint: &Path,
        _layout: MountLayout,
    ) -> Result<MountHandle, MountError> {
        validate_mountpoint(mountpoint)?;
        let session = NoopSession {
            counter: self.unmount_counter.clone(),
        };
        Ok(MountHandle::new(mountpoint.to_path_buf(), Box::new(session)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_mountpoint_accepts_empty_dir() {
        let tmp = tempfile::tempdir().expect("tempdir");
        validate_mountpoint(tmp.path()).expect("empty dir ok");
    }

    #[test]
    fn validate_mountpoint_rejects_nonempty_dir() {
        let tmp = tempfile::tempdir().expect("tempdir");
        std::fs::write(tmp.path().join("stray.txt"), b"hi").expect("write");
        let err = validate_mountpoint(tmp.path()).expect_err("must reject");
        assert!(matches!(err, MountError::MountpointNotEmpty(_)));
    }

    #[test]
    fn validate_mountpoint_rejects_traversal_component() {
        let err = validate_mountpoint(Path::new("../escape")).expect_err("must reject");
        assert!(matches!(err, MountError::UnsafeMountpoint(_)));
    }

    #[test]
    fn noop_mount_and_drop_increments_counter() {
        let backend = NoopBackend::default();
        let tmp = tempfile::tempdir().expect("tempdir");
        let handle = backend
            .mount(tmp.path(), MountLayout::all())
            .expect("mount");
        drop(handle);
        assert_eq!(*backend.unmount_counter().lock().expect("lock"), 1);
    }
}
