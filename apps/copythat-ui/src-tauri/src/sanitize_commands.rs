//! Phase 44.2a — Tauri IPC bridge for whole-drive sanitize.
//!
//! Four commands:
//!
//! - `sanitize_list_devices()` — returns common block-device path
//!   candidates the user might pick. On Linux, scans `/sys/block`
//!   for `nvme*` / `sd*` entries. On macOS, parses `diskutil list`.
//!   On Windows, returns the empty list (the user types
//!   `\\.\\PhysicalDriveN` manually until Phase 44.3 wires
//!   `IOCTL_STORAGE_QUERY_PROPERTY` enumeration).
//! - `sanitize_capabilities(device)` — read-only probe via the
//!   platform helper. No elevation required.
//! - `sanitize_run(device, mode, model_typed)` — runs the sanitize
//!   through `whole_drive_sanitize`. Validates the typed
//!   drive-model matches the helper's reported model (third
//!   confirmation contract). Streams `sanitize-started` /
//!   `sanitize-progress` / `sanitize-completed` Tauri events to the
//!   frontend; returns the final `SanitizeReportDto` on success.
//! - `sanitize_free_space_trim(device)` — runs free-space TRIM via
//!   the platform helper. macOS only in Phase 44.2; Linux + Windows
//!   surface "not implemented" through the typed error.
//!
//! The platform helper itself is constructed once at startup
//! (`sanitize_helper()` factory below) and held inside an
//! `Arc<dyn SanitizeHelper>` per command invocation.

use std::path::PathBuf;
use std::sync::Arc;

use copythat_core::CopyControl;
use copythat_secure_delete::{
    SanitizeCapabilities, SanitizeHelper, SanitizeReport, ShredEvent, SsdSanitizeMode,
    free_space_trim, sanitize_capabilities, whole_drive_sanitize,
};
use serde::Serialize;
use tauri::{AppHandle, Emitter};
use tokio::sync::mpsc;

#[cfg(target_os = "linux")]
use copythat_secure_delete::LinuxSanitizeHelper;
#[cfg(target_os = "macos")]
use copythat_secure_delete::MacosSanitizeHelper;
#[cfg(target_os = "windows")]
use copythat_secure_delete::WindowsSanitizeHelper;

/// Build the platform's concrete [`SanitizeHelper`] impl. Phase
/// 44.2 picks the right one at compile time per `target_os`. A
/// future phase that wants pluggable helpers (e.g. a network-mode
/// sanitize broker) replaces this with a factory function on
/// `AppState`.
fn sanitize_helper() -> Arc<dyn SanitizeHelper> {
    #[cfg(target_os = "linux")]
    {
        Arc::new(LinuxSanitizeHelper::new())
    }
    #[cfg(target_os = "macos")]
    {
        Arc::new(MacosSanitizeHelper::new())
    }
    #[cfg(target_os = "windows")]
    {
        Arc::new(WindowsSanitizeHelper::new())
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        Arc::new(copythat_secure_delete::NoopSanitizeHelper::new())
    }
}

/// Wire shape for [`SanitizeCapabilities`]. `modes` is rendered as
/// the stable wire-name strings (`nvme-format`, `opal-crypto-erase`,
/// etc.) so JSON consumers don't need to track the Rust enum's
/// discriminant order.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SanitizeCapabilitiesDto {
    pub trim: bool,
    pub modes: Vec<&'static str>,
    pub bus: String,
    pub model: String,
    /// Phase 44.2 — derived predicate so the UI can branch on
    /// "is this a guaranteed instant op?" without rebuilding the
    /// match logic in JS.
    pub has_guaranteed_crypto_erase: bool,
}

impl From<SanitizeCapabilities> for SanitizeCapabilitiesDto {
    fn from(c: SanitizeCapabilities) -> Self {
        let has = c.has_guaranteed_crypto_erase();
        Self {
            trim: c.trim,
            modes: c.modes.iter().map(|m| m.name()).collect(),
            bus: c.bus,
            model: c.model,
            has_guaranteed_crypto_erase: has,
        }
    }
}

/// Wire shape for [`SanitizeReport`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SanitizeReportDto {
    pub device: String,
    pub mode: &'static str,
    pub duration_ms: u64,
}

impl From<SanitizeReport> for SanitizeReportDto {
    fn from(r: SanitizeReport) -> Self {
        Self {
            device: r.device.display().to_string(),
            mode: r.mode.name(),
            duration_ms: r.duration.as_millis() as u64,
        }
    }
}

/// Wire shape for the per-progress event the frontend listens for.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SanitizeProgressEvent {
    pub device: String,
    pub mode: &'static str,
    pub percent: u8,
}

/// `sanitize_list_devices()` — best-effort enumeration. Returns
/// candidate block-device paths the user might pick. Empty list
/// when enumeration isn't implemented for the platform.
#[tauri::command]
pub fn sanitize_list_devices() -> Result<Vec<String>, String> {
    let helper = sanitize_helper();
    list_devices_via_helper(&*helper)
}

#[cfg(target_os = "linux")]
fn list_devices_via_helper(_helper: &dyn SanitizeHelper) -> Result<Vec<String>, String> {
    let entries = std::fs::read_dir("/sys/block").map_err(|e| format!("read /sys/block: {e}"))?;
    let mut out = Vec::new();
    for entry in entries.flatten() {
        let name = entry.file_name();
        let s = name.to_string_lossy();
        if s.starts_with("nvme") || s.starts_with("sd") || s.starts_with("vd") {
            out.push(format!("/dev/{s}"));
        }
    }
    out.sort();
    Ok(out)
}

#[cfg(target_os = "macos")]
fn list_devices_via_helper(_helper: &dyn SanitizeHelper) -> Result<Vec<String>, String> {
    // `diskutil list -plist` returns an XML plist with
    // `AllDisksAndPartitions`. We hand-parse the top-level
    // `<key>DeviceIdentifier</key><string>diskN</string>` entries
    // for the user-pickable disks. Free of any XML/plist crate
    // dep — narrow grep on the line shape.
    let output = std::process::Command::new("diskutil")
        .arg("list")
        .output()
        .map_err(|e| format!("spawn diskutil list: {e}"))?;
    if !output.status.success() {
        return Err(format!(
            "diskutil list failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    // The `diskutil list` text output starts each volume header
    // with `/dev/diskN (...)` or similar. Pull the leading token
    // from any line that matches. Phase 44.2 post-review (M2): the
    // id must be digits-only so partition/slice identifiers like
    // `disk0s1` (whole-disk-identifier with a partition suffix)
    // don't pollute the picker — sanitize is for whole drives,
    // not partitions.
    let mut out = Vec::new();
    for line in stdout.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("/dev/disk") {
            let id: String = rest.split_whitespace().next().unwrap_or("").to_string();
            if !id.is_empty() && id.chars().all(|c| c.is_ascii_digit()) {
                out.push(format!("/dev/disk{id}"));
            }
        }
    }
    out.sort();
    out.dedup();
    Ok(out)
}

#[cfg(target_os = "windows")]
fn list_devices_via_helper(_helper: &dyn SanitizeHelper) -> Result<Vec<String>, String> {
    // Phase 44.3c — enumerate physical drives via copythat-platform's
    // safe IOCTL_STORAGE_QUERY_PROPERTY wrapper. Probes
    // \\.\PhysicalDrive0..31 and returns those that opened
    // successfully. Empty list when the user has no drives or
    // when CreateFileW denies access on every probe (rare —
    // GENERIC_READ on physical-drive paths works for standard
    // users).
    Ok(copythat_platform::windows_enumerate_physical_drives())
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
fn list_devices_via_helper(_helper: &dyn SanitizeHelper) -> Result<Vec<String>, String> {
    Ok(Vec::new())
}

/// `sanitize_capabilities(device)` — read-only probe. Wraps
/// [`copythat_secure_delete::sanitize_capabilities`] and translates
/// to the JSON-friendly DTO.
#[tauri::command]
pub fn sanitize_capabilities_cmd(device: String) -> Result<SanitizeCapabilitiesDto, String> {
    let path = PathBuf::from(&device);
    let helper = sanitize_helper();
    let caps = sanitize_capabilities(&*helper, &path)
        .map_err(|e| format!("sanitize_capabilities: {e}"))?;
    Ok(SanitizeCapabilitiesDto::from(caps))
}

/// Parse a wire-name string back into [`SsdSanitizeMode`].
fn mode_from_wire(s: &str) -> Result<SsdSanitizeMode, String> {
    Ok(match s {
        "nvme-format" => SsdSanitizeMode::NvmeFormat,
        "nvme-sanitize-block" => SsdSanitizeMode::NvmeSanitizeBlock,
        "nvme-sanitize-crypto" => SsdSanitizeMode::NvmeSanitizeCrypto,
        "ata-secure-erase" => SsdSanitizeMode::AtaSecureErase,
        "opal-crypto-erase" => SsdSanitizeMode::OpalCryptoErase,
        "apfs-crypto-erase" => SsdSanitizeMode::ApfsCryptoErase,
        other => return Err(format!("unknown sanitize mode: {other:?}")),
    })
}

/// `sanitize_run(device, mode, model_typed)` — run the sanitize
/// through `whole_drive_sanitize`. The third-confirmation contract
/// is enforced HERE in the Tauri command, in addition to the UI
/// gate, so a malicious frontend (compromised webview, hostile
/// IPC caller) cannot bypass it just by skipping the UI checkboxes.
///
/// `model_typed` must equal the helper-reported model exactly. On
/// mismatch, the command returns an error WITHOUT invoking the
/// helper's destructive path.
#[tauri::command]
pub async fn sanitize_run(
    app: AppHandle,
    device: String,
    mode: String,
    model_typed: String,
) -> Result<SanitizeReportDto, String> {
    let path = PathBuf::from(&device);
    let mode = mode_from_wire(&mode)?;
    let helper = sanitize_helper();

    // Defense-in-depth third-confirmation gate. The UI also enforces
    // this; the IPC layer enforces it again because the UI is not
    // trusted. Phase 44.2 post-review (C2): both sides must be
    // ASCII before the byte-exact compare runs — defeats Unicode-
    // confusable attacks where a hostile JS payload submits a
    // model string with a Cyrillic 'а' that visually matches the
    // Latin 'a' the user thinks they typed. Real OEM model strings
    // (`SAMSUNG SSD 990 PRO`, `WD_BLACK SN850X`, etc.) are ASCII;
    // the false-refusal cost on edge-case non-ASCII model names
    // is acceptable for a destructive op.
    let caps = sanitize_capabilities(&*helper, &path)
        .map_err(|e| format!("sanitize_capabilities (pre-run): {e}"))?;
    let typed = model_typed.trim();
    let model = caps.model.trim();
    if !typed.is_ascii() || !model.is_ascii() {
        return Err(
            "model-name confirmation requires ASCII-only inputs (defeats Unicode-confusable bypass)"
                .to_string(),
        );
    }
    if typed != model {
        return Err(format!(
            "model-name confirmation mismatch — typed {typed:?}, drive reports {model:?}"
        ));
    }
    if !caps.modes.contains(&mode) {
        return Err(format!(
            "device {device} does not support {} (reported modes: {:?})",
            mode.name(),
            caps.modes.iter().map(|m| m.name()).collect::<Vec<_>>()
        ));
    }

    // Forward ShredEvents to Tauri events so the SanitizeTab can
    // bind a progress bar without polling.
    let (tx, mut rx) = mpsc::channel::<ShredEvent>(32);
    let app_for_pump = app.clone();
    let pump = tokio::spawn(async move {
        while let Some(evt) = rx.recv().await {
            match evt {
                ShredEvent::SanitizeStarted { device, mode } => {
                    let _ = app_for_pump.emit(
                        "sanitize-started",
                        SanitizeReportDto {
                            device: device.display().to_string(),
                            mode: mode.name(),
                            duration_ms: 0,
                        },
                    );
                }
                ShredEvent::SanitizeProgress {
                    device,
                    mode,
                    percent,
                } => {
                    let _ = app_for_pump.emit(
                        "sanitize-progress",
                        SanitizeProgressEvent {
                            device: device.display().to_string(),
                            mode: mode.name(),
                            percent,
                        },
                    );
                }
                ShredEvent::SanitizeCompleted {
                    device,
                    mode,
                    duration,
                } => {
                    let _ = app_for_pump.emit(
                        "sanitize-completed",
                        SanitizeReportDto {
                            device: device.display().to_string(),
                            mode: mode.name(),
                            duration_ms: duration.as_millis() as u64,
                        },
                    );
                }
                ShredEvent::Failed { err } => {
                    let _ = app_for_pump.emit("sanitize-failed", err.message);
                }
                _ => {}
            }
        }
    });

    let ctrl = CopyControl::new();
    let report = whole_drive_sanitize(helper, &path, mode, ctrl, tx)
        .await
        .map_err(|e| format!("whole_drive_sanitize: {e}"))?;

    let _ = pump.await;
    Ok(SanitizeReportDto::from(report))
}

/// `sanitize_free_space_trim(device)` — run free-space TRIM. macOS
/// only in Phase 44.2; Linux + Windows return the helper's
/// "not implemented" error verbatim.
#[tauri::command]
pub async fn sanitize_free_space_trim(device: String) -> Result<SanitizeReportDto, String> {
    let path = PathBuf::from(&device);
    let helper = sanitize_helper();
    let ctrl = CopyControl::new();
    let report = free_space_trim(helper, &path, ctrl)
        .await
        .map_err(|e| format!("free_space_trim: {e}"))?;
    Ok(SanitizeReportDto {
        device: report.device.display().to_string(),
        mode: "free-space-trim",
        duration_ms: report.duration.as_millis() as u64,
    })
}
