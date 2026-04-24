//! Phase 33c — WinFsp backend scaffolding.
//!
//! Enabled by `--features winfsp` on Windows targets. The feature
//! pulls in `winfsp-sys`, which needs both the WinFsp driver
//! installed on the build machine *and* libclang for `bindgen`.
//! When the feature is off (or on a non-Windows build), this module
//! compiles but [`WinFspBackend::mount`] surfaces
//! `MountError::BackendUnavailable` with a clear reason.
//!
//! Phase 33c ships the type surface + feature plumbing. The real
//! WinFsp file-system callback implementation lands in Phase 33d
//! alongside the `fuser` kernel wiring.

use std::path::Path;

use crate::backends::MountBackend;
use crate::error::MountError;
use crate::handle::MountHandle;
use crate::tree::MountLayout;

/// WinFsp-backed mount. Pair of [`crate::FuseBackend`] on Windows.
#[derive(Debug, Default)]
pub struct WinFspBackend {
    _phantom: (),
}

impl WinFspBackend {
    pub fn new() -> Result<Self, MountError> {
        #[cfg(all(feature = "winfsp", target_os = "windows"))]
        {
            Ok(Self { _phantom: () })
        }
        #[cfg(not(all(feature = "winfsp", target_os = "windows")))]
        {
            Err(MountError::BackendUnavailable(
                "winfsp feature not enabled or not on Windows".into(),
            ))
        }
    }
}

impl MountBackend for WinFspBackend {
    fn mount(
        &self,
        _mountpoint: &Path,
        _layout: MountLayout,
    ) -> Result<MountHandle, MountError> {
        #[cfg(all(feature = "winfsp", target_os = "windows"))]
        {
            // Phase 33d — wire `winfsp-sys` FSP callbacks here.
            // WinFsp's model is a callback table registered via
            // `FspFileSystemCreate` + `FspFileSystemSetMountPoint` +
            // `FspFileSystemStartDispatcher`; the `MountSession`
            // impl's `unmount_on_drop` calls `FspFileSystemStop` +
            // `FspFileSystemDelete`.
            Err(MountError::BackendUnavailable(
                "winfsp backend scaffold — kernel callbacks land in Phase 33d".into(),
            ))
        }
        #[cfg(not(all(feature = "winfsp", target_os = "windows")))]
        {
            Err(MountError::BackendUnavailable(
                "winfsp feature not enabled on this build".into(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mount_always_surfaces_backend_unavailable_until_33d() {
        let backend = WinFspBackend::default();
        let tmp = tempfile::tempdir().expect("tempdir");
        let err = backend
            .mount(tmp.path(), MountLayout::all())
            .expect_err("33c scaffolds — real callbacks in 33d");
        assert!(matches!(err, MountError::BackendUnavailable(_)));
    }
}
