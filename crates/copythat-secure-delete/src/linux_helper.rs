//! Phase 44.1d — Linux [`SanitizeHelper`] impl that shells out to
//! `nvme-cli` and `hdparm`.
//!
//! Both binaries require root. On a typical desktop that means the
//! caller (CLI / Tauri runner) launches the helper via the Phase 17
//! privileged broker (`copythat-helper`) or instructs the user to
//! re-run with `sudo`. This helper does NOT escalate privilege
//! itself — if the underlying command fails with EACCES / EPERM,
//! the failure surfaces to the caller verbatim.
//!
//! Capability probe: `nvme id-ctrl <dev> --output-format=json` →
//! parse the `sanicap` field. Bits 0-2 encode crypto-erase /
//! block-erase / overwrite support per NVM Express §5.24.
//! `hdparm -I <dev>` → grep for `SECURITY` block to detect ATA
//! Secure Erase support.
//!
//! Sanitize dispatch:
//! - [`SsdSanitizeMode::NvmeFormat`]   → `nvme format -s 1 <dev>`
//! - [`SsdSanitizeMode::NvmeSanitizeBlock`]  → `nvme sanitize -a 1 <dev>`
//! - [`SsdSanitizeMode::NvmeSanitizeCrypto`] → `nvme sanitize -a 2 <dev>`
//! - [`SsdSanitizeMode::AtaSecureErase`]
//!     → `hdparm --user-master u --security-set-pass NULL <dev>`
//!       then `hdparm --user-master u --security-erase NULL <dev>`
//! - [`SsdSanitizeMode::OpalCryptoErase`] → returns
//!   `not supported` (TCG OPAL via sedutil is a separate Phase
//!   44.2 wiring; the binary isn't part of every distro's stock
//!   image and the PSID flow needs a separate UX).
//!
//! Progress polling: when the helper is invoked via
//! [`crate::SanitizeHelper::run_sanitize_blocking_with_progress`],
//! the helper spawns the sanitize command in a separate thread and
//! polls `nvme sanitize-log <dev> --output-format=json` once per
//! second. The `sprog` field (0-65535) is mapped to the 0-100
//! percent the trait contract requires.

#![cfg(target_os = "linux")]

use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use crate::sanitize::{SanitizeCapabilities, SanitizeHelper, SsdSanitizeMode};

/// Phase 44.1d — Linux `SanitizeHelper` shelling out to `nvme-cli`
/// + `hdparm`. Construct one per process; cheap to keep around (no
/// state).
#[derive(Debug, Default, Clone)]
pub struct LinuxSanitizeHelper;

impl LinuxSanitizeHelper {
    /// Construct one. No-arg.
    pub fn new() -> Self {
        Self
    }
}

impl SanitizeHelper for LinuxSanitizeHelper {
    fn capabilities(&self, device: &Path) -> Result<SanitizeCapabilities, String> {
        // Detect whether the device is NVMe by checking
        // `/dev/nvme*` prefix. Cheap; the kernel's NVMe driver only
        // ever exposes `nvme*`-prefixed devices.
        let dev_str = device.to_string_lossy();
        let is_nvme = dev_str.starts_with("/dev/nvme");

        let mut caps = SanitizeCapabilities {
            trim: probe_trim(device),
            modes: Vec::new(),
            bus: if is_nvme { "nvme" } else { "sata" }.into(),
            model: probe_model(device).unwrap_or_else(|_| "unknown".into()),
        };

        if is_nvme {
            // `nvme id-ctrl <dev> --output-format=json | jq .sanicap`
            // — the SANICAP field tells us which Sanitize actions
            // the controller supports.
            match nvme_id_ctrl(device) {
                Ok(sanicap) => {
                    // Bit 0: Crypto Erase
                    // Bit 1: Block Erase
                    // Bit 2: Overwrite
                    if sanicap & 0b001 != 0 {
                        caps.modes.push(SsdSanitizeMode::NvmeSanitizeCrypto);
                    }
                    if sanicap & 0b010 != 0 {
                        caps.modes.push(SsdSanitizeMode::NvmeSanitizeBlock);
                    }
                    // Format with Secure Erase is always supported
                    // by NVMe controllers (NVM Express §5.14 FSE
                    // bit) — list it unconditionally for NVMe.
                    caps.modes.push(SsdSanitizeMode::NvmeFormat);
                }
                Err(_) => {
                    // nvme-cli not installed or call failed — list
                    // NvmeFormat optimistically (the driver always
                    // supports it) but skip the Sanitize variants.
                    caps.modes.push(SsdSanitizeMode::NvmeFormat);
                }
            }
        } else if hdparm_security_supported(device).unwrap_or(false) {
            caps.modes.push(SsdSanitizeMode::AtaSecureErase);
        }

        Ok(caps)
    }

    fn run_sanitize_blocking(
        &self,
        device: &Path,
        requested: SsdSanitizeMode,
    ) -> Result<SsdSanitizeMode, String> {
        match requested {
            SsdSanitizeMode::NvmeFormat => {
                run_command("nvme", &["format", "-s", "1"], device)?;
                Ok(SsdSanitizeMode::NvmeFormat)
            }
            SsdSanitizeMode::NvmeSanitizeBlock => {
                run_command("nvme", &["sanitize", "-a", "1"], device)?;
                Ok(SsdSanitizeMode::NvmeSanitizeBlock)
            }
            SsdSanitizeMode::NvmeSanitizeCrypto => {
                run_command("nvme", &["sanitize", "-a", "2"], device)?;
                Ok(SsdSanitizeMode::NvmeSanitizeCrypto)
            }
            SsdSanitizeMode::AtaSecureErase => {
                // hdparm requires setting a password before erasing.
                run_command(
                    "hdparm",
                    &["--user-master", "u", "--security-set-pass", "NULL"],
                    device,
                )?;
                run_command(
                    "hdparm",
                    &["--user-master", "u", "--security-erase", "NULL"],
                    device,
                )?;
                Ok(SsdSanitizeMode::AtaSecureErase)
            }
            SsdSanitizeMode::OpalCryptoErase => Err(
                "TCG OPAL Crypto Erase requires sedutil + the drive's PSID; this is deferred to \
                 a future phase. Use NvmeSanitizeCrypto on NVMe SEDs in the meantime."
                    .into(),
            ),
        }
    }

    /// Run the sanitize with a 1Hz SPROG-polling sidecar.
    ///
    /// **Caller contract (post-review H2):** the polling thread is
    /// `std::thread::spawn` (not a tokio task) so it survives a
    /// drop of the parent `tokio::task::spawn_blocking` future.
    /// The atomic stop-flag is signalled when the main sanitize
    /// call returns; if the parent task is dropped mid-sanitize,
    /// the flag is never set and the OS thread continues polling
    /// indefinitely (same class of leak as the parent
    /// `whole_drive_sanitize` thread leak — see that function's
    /// `# Caller contract` heading). Don't drop the parent future.
    /// Phase 44.2 will rework this with a `Weak` handle so the
    /// poller exits when the strong refcount drops.
    ///
    /// **Per-poll timeout (post-review H3):** each
    /// `nvme sanitize-log` invocation is wrapped with a 2s
    /// timeout so a wedged controller can't pin the poller on a
    /// blocking ioctl.
    fn run_sanitize_blocking_with_progress(
        &self,
        device: &Path,
        requested: SsdSanitizeMode,
        progress: Arc<dyn Fn(u8) + Send + Sync + 'static>,
    ) -> Result<SsdSanitizeMode, String> {
        let device_for_poller = device.to_path_buf();
        let stop = Arc::new(AtomicBool::new(false));
        let stop_for_poller = Arc::clone(&stop);

        // Spawn a 1Hz polling thread that scrapes SPROG from
        // `nvme sanitize-log` and forwards percent through the
        // progress callback. Stops when the main sanitize call
        // returns (we set the atomic flag).
        let poller = thread::spawn(move || {
            while !stop_for_poller.load(Ordering::Relaxed) {
                if let Ok(percent) = nvme_sanitize_log_percent(&device_for_poller) {
                    progress(percent);
                }
                thread::sleep(Duration::from_secs(1));
            }
        });

        let result = self.run_sanitize_blocking(device, requested);

        // Signal the poller to stop. Joining is best-effort —
        // a misbehaving poller shouldn't block the sanitize report.
        stop.store(true, Ordering::Relaxed);
        let _ = poller.join();

        result
    }

    fn run_free_space_trim_blocking(&self, _device: &Path) -> Result<(), String> {
        Err(
            "Linux free-space TRIM uses `fstrim <mountpoint>`, which operates on mount points \
             not block devices. This API takes a device path; mount-point TRIM is deferred to a \
             future phase that exposes a mount-aware variant."
                .into(),
        )
    }
}

/// Phase 44.1 post-review (Vuln 1) — accept a device path only if
/// it looks like a real block-device path. Rejects leading-dash
/// paths that getopt would parse as flags, paths containing `..`
/// components, and paths whose lexical form doesn't match
/// `/dev/...`. This is the security gate that closes the
/// argument-injection vector via the `SanitizeHelper::run_*` API
/// surface — the trait contract says "device is a &Path", but the
/// caller's input could be attacker-controlled (Tauri IPC arg).
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
                    "device path {device:?} contains relative components (..) — refused"
                ));
            }
        }
    }
    Ok(())
}

/// Run `cmd <args> -- <device>` and return Ok on success or a
/// typed error string. The `--` end-of-options separator BEFORE
/// the device argument is mandatory: nvme-cli and hdparm both use
/// getopt-style parsing that interleaves options with positionals,
/// so a device path like `--security-erase NULL /dev/sda` would
/// otherwise be parsed as flags. Phase 44.1 post-review (Vuln 1)
/// closes that argument-injection vector.
fn run_command(cmd: &str, args: &[&str], device: &Path) -> Result<(), String> {
    validate_device_path(device)?;
    let output = Command::new(cmd)
        .args(args)
        .arg("--")
        .arg(device)
        .output()
        .map_err(|e| format!("spawn {cmd}: {e}"))?;
    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!(
            "{cmd} {} -- {} failed (exit={:?}): {}",
            args.join(" "),
            device.display(),
            output.status.code(),
            stderr.trim()
        ))
    }
}

/// Phase 44.1d — parse `sanicap` from `nvme id-ctrl <dev>
/// --output-format=json`. SANICAP is a 32-bit field; bits 0-2
/// encode the per-action support. We only need the lower 3 bits.
///
/// Phase 44.1 post-review (H1) — accepts the value as decimal
/// `7` OR hex `0x7`. nvme-cli's JSON output has historically
/// shipped both forms across distro forks. The
/// `parse_decimal_or_hex` helper handles either.
fn nvme_id_ctrl(device: &Path) -> Result<u32, String> {
    validate_device_path(device)?;
    let output = Command::new("nvme")
        .arg("id-ctrl")
        .arg("--output-format=json")
        .arg("--")
        .arg(device)
        .output()
        .map_err(|e| format!("spawn nvme id-ctrl: {e}"))?;
    if !output.status.success() {
        return Err(format!(
            "nvme id-ctrl failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("\"sanicap\"") {
            if let Some(num_str) = rest.split(':').nth(1) {
                let cleaned: String = num_str
                    .trim()
                    .trim_end_matches(',')
                    .trim_matches('"')
                    .to_string();
                if let Some(n) = parse_decimal_or_hex(&cleaned) {
                    return Ok(n);
                }
            }
        }
    }
    Err("sanicap field not found in nvme id-ctrl output".into())
}

/// Parse a numeric value as decimal or hex (with `0x`/`0X` prefix).
/// Phase 44.1 post-review H1 — nvme-cli emits SANICAP/SPROG in
/// either form depending on version; the original parse only
/// accepted decimal and silently lost capabilities on hex-emitting
/// builds.
fn parse_decimal_or_hex(s: &str) -> Option<u32> {
    let s = s.trim();
    if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        u32::from_str_radix(hex, 16).ok()
    } else {
        s.parse::<u32>().ok()
    }
}

/// Phase 44.1d — poll `nvme sanitize-log` and convert the SPROG
/// (0-65535) field to a 0-100 percent. Returns `Err` when the
/// command isn't available, blocks past a 2-second timeout, or
/// the parse fails (caller swallows errors and tries again next
/// tick).
///
/// Phase 44.1 post-review (H3) — wraps the spawn in a 2s timeout
/// because `nvme sanitize-log` can block on a busy controller (the
/// kernel NVMe driver may serialize the ioctl behind the active
/// sanitize). The original implementation could leak processes
/// indefinitely on a wedged drive.
fn nvme_sanitize_log_percent(device: &Path) -> Result<u8, String> {
    validate_device_path(device)?;
    let mut child = Command::new("nvme")
        .arg("sanitize-log")
        .arg("--output-format=json")
        .arg("--")
        .arg(device)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("spawn nvme sanitize-log: {e}"))?;

    // Poll with a 2-second hard limit. Cheap busy-wait at 50ms
    // resolution; this happens once per second from the parent
    // poller so the busy-wait loop is bounded and short.
    let started = std::time::Instant::now();
    let timeout = Duration::from_secs(2);
    loop {
        match child.try_wait() {
            Ok(Some(_status)) => break,
            Ok(None) => {
                if started.elapsed() > timeout {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Err("nvme sanitize-log exceeded 2s timeout".into());
                }
                std::thread::sleep(Duration::from_millis(50));
            }
            Err(e) => return Err(format!("nvme sanitize-log try_wait: {e}")),
        }
    }
    let output = child
        .wait_with_output()
        .map_err(|e| format!("wait_with_output: {e}"))?;
    if !output.status.success() {
        return Err("nvme sanitize-log failed".into());
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("\"sprog\"") {
            if let Some(num_str) = rest.split(':').nth(1) {
                let cleaned: String = num_str
                    .trim()
                    .trim_end_matches(',')
                    .trim_matches('"')
                    .to_string();
                if let Some(n) = parse_decimal_or_hex(&cleaned) {
                    let pct = ((n as u64 * 100) / 65_535) as u8;
                    return Ok(pct.min(100));
                }
            }
        }
    }
    Err("sprog field not found".into())
}

/// Phase 44.1d — best-effort TRIM-support probe. Reads
/// `/sys/block/<basename>/queue/discard_max_bytes`; non-zero means
/// TRIM is supported. This file is part of the kernel's stable
/// sysfs ABI.
fn probe_trim(device: &Path) -> bool {
    let Some(file_name) = device.file_name() else {
        return false;
    };
    let sysfs = format!(
        "/sys/block/{}/queue/discard_max_bytes",
        file_name.to_string_lossy()
    );
    std::fs::read_to_string(&sysfs)
        .ok()
        .and_then(|s| s.trim().parse::<u64>().ok())
        .map(|n| n > 0)
        .unwrap_or(false)
}

/// Phase 44.1d — read the drive's vendor-reported model string
/// from sysfs. `/sys/block/<basename>/device/model` (SCSI/SATA) or
/// `/sys/block/<basename>/device/model` (NVMe — the file name is
/// identical because the kernel's sysfs glue normalises both).
fn probe_model(device: &Path) -> Result<String, String> {
    let file_name = device
        .file_name()
        .ok_or_else(|| "device has no filename component".to_string())?;
    let sysfs = format!("/sys/block/{}/device/model", file_name.to_string_lossy());
    std::fs::read_to_string(&sysfs)
        .map(|s| s.trim().to_string())
        .map_err(|e| format!("read {sysfs}: {e}"))
}

/// Phase 44.1d — probe whether `hdparm` reports ATA Secure Erase
/// support. Greps the `-I` output for the `SECURITY` block — the
/// stable text format hdparm has shipped with for years.
fn hdparm_security_supported(device: &Path) -> Result<bool, String> {
    if validate_device_path(device).is_err() {
        return Ok(false);
    }
    let output = Command::new("hdparm")
        .arg("-I")
        .arg("--")
        .arg(device)
        .output()
        .map_err(|e| format!("spawn hdparm: {e}"))?;
    if !output.status.success() {
        return Ok(false);
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.contains("SECURITY") && stdout.contains("supported"))
}
