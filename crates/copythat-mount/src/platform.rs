//! Phase 33c — platform-default [`MountBackend`] selector.
//!
//! [`default_backend`] picks the right concrete backend for the
//! running target + enabled feature set, boxed as a
//! `Box<dyn MountBackend + Send + Sync>`. The Tauri runner calls this
//! once at startup and hands the box to the `mount_snapshot` IPC
//! command; tests and the smoke keep building [`NoopBackend`]
//! directly.
//!
//! Selection order:
//!
//! 1. `cfg(all(feature = "fuse", any(target_os = "linux", target_os = "macos")))` → [`crate::FuseBackend`]
//! 2. `cfg(all(feature = "winfsp", target_os = "windows"))` → [`crate::WinFspBackend`]
//! 3. Fallback → [`NoopBackend`]
//!
//! Phase 33c's real FUSE / WinFsp backends currently stub to
//! `MountError::BackendUnavailable` at `mount()` time (see
//! [`crate::FuseBackend`] + [`crate::WinFspBackend`] docs); Phase
//! 33d swaps the stubs for real kernel callback wiring, at which
//! point `default_backend` becomes the user-facing mount path.

use crate::backends::{MountBackend, NoopBackend};
#[cfg(all(feature = "fuse", any(target_os = "linux", target_os = "macos")))]
use crate::fuse_backend::FuseBackend;
#[cfg(all(feature = "winfsp", target_os = "windows"))]
use crate::winfsp_backend::WinFspBackend;

/// Pick the platform-appropriate backend. Always returns a boxed
/// `MountBackend` so the caller can treat it uniformly across
/// platforms.
pub fn default_backend() -> Box<dyn MountBackend + Send + Sync> {
    #[cfg(all(feature = "fuse", any(target_os = "linux", target_os = "macos")))]
    {
        return Box::new(FuseBackend::default());
    }
    #[cfg(all(feature = "winfsp", target_os = "windows"))]
    {
        return Box::new(WinFspBackend::default());
    }
    #[allow(unreachable_code)]
    {
        Box::new(NoopBackend::default())
    }
}

/// Returns the name of the selected backend kind — useful for the
/// UI "Mount backend: FUSE / WinFsp / (stub NoopBackend)" badge.
pub fn default_backend_name() -> &'static str {
    #[cfg(all(feature = "fuse", any(target_os = "linux", target_os = "macos")))]
    {
        return "fuse";
    }
    #[cfg(all(feature = "winfsp", target_os = "windows"))]
    {
        return "winfsp";
    }
    #[allow(unreachable_code)]
    {
        "noop"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MountLayout;

    #[test]
    fn default_backend_name_matches_current_feature_set() {
        let name = default_backend_name();
        // Without `fuse` / `winfsp` features, the default build
        // reports "noop" on every target.
        #[cfg(not(any(
            all(feature = "fuse", any(target_os = "linux", target_os = "macos")),
            all(feature = "winfsp", target_os = "windows"),
        )))]
        {
            assert_eq!(name, "noop");
        }
        // With a feature enabled on its matching platform, the
        // selector reports the right kernel backend.
        #[cfg(all(feature = "fuse", any(target_os = "linux", target_os = "macos")))]
        {
            assert_eq!(name, "fuse");
        }
        #[cfg(all(feature = "winfsp", target_os = "windows"))]
        {
            assert_eq!(name, "winfsp");
        }
    }

    #[test]
    fn default_backend_constructs_a_mount_backend() {
        let backend = default_backend();
        let tmp = tempfile::tempdir().expect("tempdir");
        // We can only assert `mount()` returns *some* result on the
        // default feature set (Noop succeeds; Fuse / WinFsp stubs
        // surface `BackendUnavailable`).
        let _ = backend.mount(
            tmp.path(),
            MountLayout::all(),
            &crate::backends::ArchiveRefs::default(),
        );
    }
}
