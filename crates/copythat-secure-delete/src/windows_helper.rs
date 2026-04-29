//! Phase 44.1e — Windows [`SanitizeHelper`] impl.
//!
//! **First-cut stub.** The full Windows path requires
//! `DeviceIoControl(IOCTL_STORAGE_SECURITY_PROTOCOL_OUT)` calls
//! against the TCG OPAL command set, which is a few hundred lines
//! of unsafe FFI per command + careful data-structure marshaling
//! (`STORAGE_PROTOCOL_DATA_DESCRIPTOR`, the OPAL-specific
//! `Subsystem Class Driver` SECURITY-protocol headers, the SCSI
//! command DBLs). Per the workspace's "unsafe lives only in
//! `copythat-platform`" invariant, that work belongs in
//! `copythat-platform::sanitize` (Phase 44.2 follow-up); this
//! crate stays `#![forbid(unsafe_code)]`-clean.
//!
//! The current impl returns NotSupported for every mode. Capability
//! probe returns the standard "TRIM only" set so a UI can render
//! the picker without crashing — the user just sees "no firmware
//! sanitize available on this build" until 44.2 lands.

#![cfg(target_os = "windows")]

use std::path::Path;

use crate::sanitize::{SanitizeCapabilities, SanitizeHelper, SsdSanitizeMode};

/// Phase 44.1e — Windows `SanitizeHelper` (stub). Construct one
/// per process; cheap, no state.
#[derive(Debug, Default, Clone)]
pub struct WindowsSanitizeHelper;

impl WindowsSanitizeHelper {
    /// Construct one. No-arg.
    pub fn new() -> Self {
        Self
    }
}

impl SanitizeHelper for WindowsSanitizeHelper {
    fn capabilities(&self, device: &Path) -> Result<SanitizeCapabilities, String> {
        // Phase 44.1 stub: report no modes so the UI can render the
        // picker without crashing. The capability probe via
        // `DeviceIoControl(IOCTL_STORAGE_QUERY_PROPERTY)` for
        // StorageDeviceTrimProperty + StorageAdapterRpmbProperty
        // lands together with the OPAL impl in Phase 44.2.
        Ok(SanitizeCapabilities {
            trim: false,
            modes: Vec::new(),
            bus: "windows-stub".into(),
            model: device.display().to_string(),
        })
    }

    fn run_sanitize_blocking(
        &self,
        _device: &Path,
        _requested: SsdSanitizeMode,
    ) -> Result<SsdSanitizeMode, String> {
        Err(
            "Windows whole-drive sanitize via DeviceIoControl(IOCTL_STORAGE_SECURITY_PROTOCOL_OUT) \
             is deferred to Phase 44.2. The marshaling for the TCG OPAL command set requires \
             unsafe FFI that lives in copythat-platform; this crate stays \
             #![forbid(unsafe_code)]-clean."
                .into(),
        )
    }

    fn run_free_space_trim_blocking(&self, _device: &Path) -> Result<(), String> {
        Err(
            "Windows free-space TRIM goes through the OS's scheduled Storage Optimizer task \
             (`Optimize Drives` in the GUI). There is no documented one-shot per-device API; \
             callers should run `defrag.exe /L /O <volume>` themselves or route through the \
             Storage Optimizer COM service in a future phase."
                .into(),
        )
    }
}
