//! Phase 44.1d (macOS half) — `SanitizeHelper` impl that shells
//! out to `diskutil` for whole-drive operations and the OS's
//! built-in TRIM machinery for free-space TRIM.
//!
//! `diskutil` is part of the system, never requires elevation for
//! read-only metadata, and prompts the user via the GUI Authopen
//! dialog for write operations. This helper does NOT escalate
//! privilege itself; it just shells out and reports back.
//!
//! Capabilities: `diskutil info -plist <device>` returns an XML
//! plist whose `<key>SolidState</key>` flag tells us whether the
//! drive is flash. Whole-drive sanitize on macOS goes through
//! `diskutil secureErase` for HDDs (the secureErase variants like
//! `0` zero-fill and `2` 7-pass DoD are documented; on SSDs Apple
//! refuses the multi-pass options and only honours
//! `secureErase 0` which is just zero-fill — meaningless on flash).
//!
//! For NVMe + OPAL: macOS does NOT expose `nvme-cli`-style
//! sanitize commands to userland. The Phase 44.1 first cut returns
//! `not supported` for [`SsdSanitizeMode::NvmeSanitizeBlock`] /
//! [`SsdSanitizeMode::NvmeSanitizeCrypto`] / [`SsdSanitizeMode::OpalCryptoErase`]
//! on macOS. The user-facing recommendation is APFS native crypto
//! erase (cycle the volume's encryption key — APFS rotates the
//! per-volume class key, defeating forensic recovery on T2 / Apple
//! Silicon hardware where the SEP attests the rotation).
//!
//! Free-space TRIM: `diskutil secureErase freespace 0 <device>`.
//! This trims the unallocated regions on a flash drive. macOS
//! gates this on the user's authentication; the helper will
//! prompt automatically through the system service.

#![cfg(target_os = "macos")]

use std::path::Path;
use std::process::Command;

use crate::sanitize::{SanitizeCapabilities, SanitizeHelper, SsdSanitizeMode};

/// Phase 44.1d — macOS `SanitizeHelper` shelling out to
/// `diskutil`. Construct one per process; cheap to keep around.
#[derive(Debug, Default, Clone)]
pub struct MacosSanitizeHelper;

impl MacosSanitizeHelper {
    /// Construct one. No-arg.
    pub fn new() -> Self {
        Self
    }
}

impl SanitizeHelper for MacosSanitizeHelper {
    fn capabilities(&self, device: &Path) -> Result<SanitizeCapabilities, String> {
        // macOS doesn't expose NVMe Sanitize / OPAL crypto erase to
        // userland, so the `modes` set is conservative: free-space
        // TRIM (separate API; not a SsdSanitizeMode) is always
        // available; whole-drive sanitize is effectively
        // unsupported. Future phases may add a Secure Enclave
        // crypto-erase variant once the API surface is researched.
        let model = probe_model(device).unwrap_or_else(|_| "unknown".into());
        Ok(SanitizeCapabilities {
            trim: true, // diskutil supports free-space TRIM on flash
            modes: Vec::new(),
            bus: "macos-internal".into(),
            model,
        })
    }

    fn run_sanitize_blocking(
        &self,
        _device: &Path,
        _requested: SsdSanitizeMode,
    ) -> Result<SsdSanitizeMode, String> {
        Err(
            "macOS does not expose NVMe Sanitize / OPAL Crypto Erase to userland; the recommended \
             path is APFS native crypto-erase (rotate the volume's encryption key via \
             `diskutil apfs encryptVolume` or by deleting the encrypted volume). This helper's \
             sanitize path returns NotSupported by design."
                .into(),
        )
    }

    fn run_free_space_trim_blocking(&self, device: &Path) -> Result<(), String> {
        // Phase 44.1 post-review (Vuln 1) — validate the device path
        // before passing it to diskutil. Reject anything that's not
        // an absolute /dev/ path or that contains relative
        // components; close the argument-injection vector via the
        // `SanitizeHelper` API surface.
        validate_device_path(device)?;
        // `diskutil secureErase freespace 0 <device>` — TRIM the
        // free space on a flash volume. Level `0` is a single pass;
        // higher levels are no-ops on SSDs and rejected by diskutil.
        // No `--` separator: diskutil's argument parser does NOT
        // honour `--` as end-of-options (it's a positional CLI),
        // so the path validation above is the only defence.
        let output = Command::new("diskutil")
            .args(["secureErase", "freespace", "0"])
            .arg(device)
            .output()
            .map_err(|e| format!("spawn diskutil: {e}"))?;
        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!(
                "diskutil secureErase freespace 0 {} failed (exit={:?}): {}",
                device.display(),
                output.status.code(),
                stderr.trim()
            ))
        }
    }
}

/// Phase 44.1 post-review (Vuln 1) — same gate as the Linux helper:
/// reject device paths that don't look like real device nodes
/// (`/dev/...` on macOS) or that contain relative components.
fn validate_device_path(device: &Path) -> Result<(), String> {
    let s = device.to_string_lossy();
    if !s.starts_with("/dev/") {
        return Err(format!(
            "device path {device:?} is not a /dev/ block device path; refused"
        ));
    }
    for c in device.components() {
        match c {
            std::path::Component::Normal(_)
            | std::path::Component::RootDir
            | std::path::Component::Prefix(_) => {}
            std::path::Component::ParentDir | std::path::Component::CurDir => {
                return Err(format!(
                    "device path {device:?} contains relative components — refused"
                ));
            }
        }
    }
    Ok(())
}

/// Phase 44.1d — read the drive's model string via `diskutil info`.
/// Falls back to the BSD device name if the parse fails.
fn probe_model(device: &Path) -> Result<String, String> {
    validate_device_path(device)?;
    let output = Command::new("diskutil")
        .arg("info")
        .arg(device)
        .output()
        .map_err(|e| format!("spawn diskutil info: {e}"))?;
    if !output.status.success() {
        return Err(format!(
            "diskutil info failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("Device / Media Name:") {
            return Ok(rest.trim().to_string());
        }
    }
    Ok(device.display().to_string())
}
