//! Phase 44.1e + 44.2e — Windows [`SanitizeHelper`] impl.
//!
//! **Stub through Phase 44.2.** The full Windows path requires
//! `DeviceIoControl(IOCTL_STORAGE_SECURITY_PROTOCOL_OUT)` calls
//! against the TCG OPAL command set, which is a few hundred lines
//! of unsafe FFI per command + careful data-structure marshaling
//! (`STORAGE_PROTOCOL_DATA_DESCRIPTOR`, the OPAL-specific
//! `Subsystem Class Driver` SECURITY-protocol headers, the SCSI
//! command DBLs). Per the workspace's "unsafe lives only in
//! `copythat-platform`" invariant, that work belongs in a new
//! `copythat-platform::sanitize` module.
//!
//! Phase 44.2 explicitly defers this to Phase 44.3 because:
//! 1. Hardware-validation cost. The OPAL command sequence
//!    (StartSession on Admin SP → RevertSP → CloseSession)
//!    requires a real Self-Encrypting Drive on a Windows test bed
//!    to verify. Shipping untested destructive code against
//!    arbitrary user drives is the wrong tradeoff.
//! 2. The capability probe via
//!    `IOCTL_STORAGE_QUERY_PROPERTY(StorageDeviceTrimProperty)`
//!    is testable on any Windows machine and would be a
//!    reasonable Phase 44.2 deliverable, but it's load-bearing
//!    only as input to the destructive paths — without those, the
//!    probe alone has no UI consumer.
//!
//! Phase 44.3 will land the full impl: capability probe, OPAL
//! crypto-erase via `IOCTL_STORAGE_SECURITY_PROTOCOL_OUT`, and
//! ATA Secure Erase via `ATA_PASS_THROUGH_DIRECT`. The trait
//! contract this stub implements is stable; only the function
//! bodies change.

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
        // Phase 44.2 stub: report no modes so the UI can render the
        // picker without crashing. The real probe via
        // `DeviceIoControl(IOCTL_STORAGE_QUERY_PROPERTY,
        // StorageDeviceTrimProperty)` lands in Phase 44.3 alongside
        // the OPAL command-set marshaling.
        //
        // Phase 44.2 also validates the device path for shape so
        // a misconfigured caller gets a clear error rather than a
        // silent NotSupported. Windows physical drives are
        // `\\.\PhysicalDriveN`; volumes are `\\.\X:`.
        let s = device.to_string_lossy();
        let plausible =
            s.starts_with(r"\\.\PhysicalDrive") || s.starts_with(r"\\.\") || s.starts_with(r"\\?\");
        if !plausible {
            return Err(format!(
                "device path {device:?} doesn't match the Windows physical-drive shape \
                 (\\\\.\\PhysicalDriveN); refused"
            ));
        }
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
             is deferred to Phase 44.3 — implementing the TCG OPAL command set without a real \
             Self-Encrypting Drive on a Windows test bed would ship untested destructive code \
             against user data. The trait stub is stable; the body is what changes."
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

    fn run_opal_psid_revert_blocking(&self, _device: &Path, _psid: &str) -> Result<(), String> {
        Err(
            "Windows TCG OPAL PSID-revert via DeviceIoControl(IOCTL_STORAGE_SECURITY_PROTOCOL_OUT) \
             is deferred to Phase 44.3 (same hardware-validation gate as the sanitize path)."
                .into(),
        )
    }
}
