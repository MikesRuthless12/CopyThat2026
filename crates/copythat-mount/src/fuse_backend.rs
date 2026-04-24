//! Phase 33c — FUSE backend scaffolding.
//!
//! Enabled by `--features fuse` on Linux / macOS targets. The module
//! compiles only when the feature is on *and* the target supports
//! FUSE; everything else falls through to [`FuseBackend::new`]
//! returning a `MountError::BackendUnavailable`.
//!
//! Phase 33c's commit ships the type surface + the feature-gated
//! `fuser` dep wiring. The real kernel read callbacks that stream
//! chunks from the Phase 27 chunk store land in Phase 33d — until
//! then [`FuseBackend::mount`] surfaces
//! `MountError::BackendUnavailable` with a clear reason.

use std::path::Path;

use crate::backends::MountBackend;
use crate::error::MountError;
use crate::handle::MountHandle;
use crate::tree::MountLayout;

/// FUSE-backed mount. Constructed via `default_backend()` on hosts
/// where the `fuse` feature + target triples line up; elsewhere the
/// fallback to `BackendUnavailable` keeps the public API uniform.
#[derive(Debug, Default)]
pub struct FuseBackend {
    _phantom: (),
}

impl FuseBackend {
    /// Build a new FUSE backend. On hosts where the feature + target
    /// gate out, Phase 33c's `default_backend()` never constructs
    /// one of these directly — it routes to `NoopBackend`. This
    /// constructor stays present on every target so downstream
    /// crates can still name the type (e.g. for trait-object casts)
    /// without a cfg maze.
    pub fn new() -> Result<Self, MountError> {
        #[cfg(all(feature = "fuse", any(target_os = "linux", target_os = "macos")))]
        {
            Ok(Self { _phantom: () })
        }
        #[cfg(not(all(feature = "fuse", any(target_os = "linux", target_os = "macos"))))]
        {
            Err(MountError::BackendUnavailable(
                "fuse feature not enabled or unsupported on this target".into(),
            ))
        }
    }
}

impl MountBackend for FuseBackend {
    fn mount(
        &self,
        _mountpoint: &Path,
        _layout: MountLayout,
    ) -> Result<MountHandle, MountError> {
        #[cfg(all(feature = "fuse", any(target_os = "linux", target_os = "macos")))]
        {
            // Phase 33d — wire the real `fuser::Filesystem` impl here.
            // The session should spawn a dedicated thread that owns
            // the `BackgroundSession`, so the thread exit closes the
            // mount cleanly on `MountSession::unmount_on_drop`.
            Err(MountError::BackendUnavailable(
                "fuse backend scaffold — kernel callbacks land in Phase 33d".into(),
            ))
        }
        #[cfg(not(all(feature = "fuse", any(target_os = "linux", target_os = "macos"))))]
        {
            Err(MountError::BackendUnavailable(
                "fuse feature not enabled on this build".into(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mount_always_surfaces_backend_unavailable_until_33d() {
        let backend = FuseBackend::default();
        let tmp = tempfile::tempdir().expect("tempdir");
        let err = backend
            .mount(tmp.path(), MountLayout::all())
            .expect_err("33c scaffolds — real callbacks in 33d");
        assert!(matches!(err, MountError::BackendUnavailable(_)));
    }

    #[test]
    fn new_on_unsupported_platform_reports_backend_unavailable() {
        // On non-fuse targets, `FuseBackend::new` returns an error.
        // On fuse-supported targets with the feature enabled, it
        // returns `Ok` — either outcome is valid for this test's
        // invariant: `new` never panics.
        let _ = FuseBackend::new();
    }
}
