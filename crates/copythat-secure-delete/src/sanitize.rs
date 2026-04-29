//! Phase 44 — SSD-aware whole-drive sanitization.
//!
//! Builds on Phase 4's per-file shredder with the recognition that
//! multi-pass overwrite is **meaningless** on flash + CoW
//! filesystems. The right answer for purging a whole flash drive is
//! one of:
//!
//! - **NVMe Sanitize / Crypto Erase** — the drive's own sanitize
//!   command (NVM Express §5.24); the controller cycles internal
//!   key material so every NAND cell becomes unreadable in O(ms).
//! - **NVMe Format with Secure Erase** — drives that don't expose
//!   `Sanitize` often still expose `Format NVM` with the secure-
//!   erase setting (NVM Express §5.14, FSE bit).
//! - **ATA SECURE ERASE** — the SATA equivalent for legacy SSDs.
//! - **OPAL Crypto Erase** — Self-Encrypting Drives whose firmware
//!   exposes the TCG OPAL command set; `RevertSP` rotates the
//!   media-encryption key.
//!
//! All four require root / Administrator on the host because they
//! either ioctl a raw block device or shell out to a setuid binary
//! (`nvme-cli`, `hdparm`). The Phase 17 privileged helper
//! (`copythat-helper`) is the broker: this crate hands it a typed
//! request, the helper runs the actual command, this crate parses
//! the typed response.
//!
//! # What's here today (Phase 44 first cut)
//!
//! - [`SsdSanitizeMode`] — the five supported modes.
//! - [`SanitizeCapabilities`] — what a particular device supports.
//! - [`SanitizeReport`] — final record.
//! - [`SanitizeHelper`] trait — pluggable broker the engine talks to.
//!   Real implementations will live in `copythat-platform` (Windows
//!   `DeviceIoControl(IOCTL_STORAGE_SECURITY_PROTOCOL_OUT)`) and a
//!   future `copythat-helper` Linux binary (shells out to
//!   `nvme-cli` / `hdparm`). The bundled
//!   [`NoopSanitizeHelper`] returns "no helper installed" so
//!   the public API works in test + CI without root.
//! - [`whole_drive_sanitize`] — the async entrypoint. Walks
//!   capabilities first, dispatches the requested mode through the
//!   helper, emits events on the same channel as the per-file
//!   shredder so a UI can route both through the same widget.
//! - [`sanitize_capabilities`] — capability probe. Read-only;
//!   does NOT require root (the helper layer probes via
//!   `IOCTL_STORAGE_QUERY_PROPERTY` on Windows or
//!   `nvme id-ctrl --output-format=json` on Linux, both of which
//!   are user-mode).
//! - [`is_cow_filesystem`] — CoW detection. When the per-file
//!   shredder is asked to shred a file on CoW (Btrfs / ZFS / APFS)
//!   it now refuses with [`crate::ShredErrorKind::ShredMeaningless`]
//!   and points the user at whole-drive sanitize.
//!
//! # What's deferred (follow-up Phase 44.1)
//!
//! - Concrete `SanitizeHelper` impl on each platform. The trait is
//!   stable; the impl is a separate concern requiring kernel-mode
//!   IOCTL bindings + a privileged helper IPC contract.
//! - The Tauri UI's three-confirmation dialog and per-mode "ALL
//!   DATA WILL BE DESTROYED" prose.
//! - Smoke against a real loop device — Phase 4's smoke uses
//!   `tempfile::tempfile_in` for byte-level testing; Phase 44's
//!   smoke uses a mocked helper.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::mpsc;

use copythat_core::CopyControl;

use crate::error::{ShredError, ShredErrorKind};
use crate::event::ShredEvent;

/// Hardware-level sanitization modes. In rough preference order
/// (most thorough → least): NVMe Sanitize Crypto > OPAL Crypto >
/// NVMe Sanitize Block > NVMe Format > ATA Secure Erase.
///
/// **macOS free-space TRIM** (`diskutil secureErase freespace 0
/// disk0`) is intentionally NOT a variant of this enum because it
/// is not equivalent to crypto-erase or block-erase — it only TRIMs
/// the *currently free* blocks, leaving the live filesystem
/// untouched. The Phase 44 spec mentions it as the macOS fallback
/// when no OPAL drive is available; a follow-up phase will surface
/// it through a separate `free_space_trim()` API rather than
/// pretend it lives in the same trust class as a true sanitize.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SsdSanitizeMode {
    /// NVMe Format with Secure Erase (NVM Express §5.14, FSE=1).
    /// Cryptographically erases when the drive supports it; falls
    /// back to a block erase otherwise.
    NvmeFormat,
    /// NVMe Sanitize, Block Erase (NVM Express §5.24, action=1).
    /// Overwrites every block; takes longer than crypto but doesn't
    /// require encryption support.
    NvmeSanitizeBlock,
    /// NVMe Sanitize, Crypto Erase (NVM Express §5.24, action=2).
    /// Rotates the media-encryption key; instant. Requires the drive
    /// to be configured with internal encryption (most modern NVMe
    /// drives are).
    NvmeSanitizeCrypto,
    /// ATA SECURE ERASE — for SATA SSDs without NVMe support.
    /// `hdparm --user-master u --security-erase NULL /dev/sda`.
    AtaSecureErase,
    /// TCG OPAL Crypto Erase via `RevertSP`. For Self-Encrypting
    /// Drives. Requires the OPAL admin password (the helper prompts
    /// or accepts it via stdin).
    OpalCryptoErase,
}

impl SsdSanitizeMode {
    /// Stable short identifier — surfaces in UI strings + helper
    /// IPC. Never internationalised; the Fluent keys carry the
    /// localised label.
    pub fn name(self) -> &'static str {
        match self {
            SsdSanitizeMode::NvmeFormat => "nvme-format",
            SsdSanitizeMode::NvmeSanitizeBlock => "nvme-sanitize-block",
            SsdSanitizeMode::NvmeSanitizeCrypto => "nvme-sanitize-crypto",
            SsdSanitizeMode::AtaSecureErase => "ata-secure-erase",
            SsdSanitizeMode::OpalCryptoErase => "opal-crypto-erase",
        }
    }

    /// All five modes in declaration order. Useful for capability
    /// probes.
    pub const ALL: &'static [SsdSanitizeMode] = &[
        SsdSanitizeMode::NvmeFormat,
        SsdSanitizeMode::NvmeSanitizeBlock,
        SsdSanitizeMode::NvmeSanitizeCrypto,
        SsdSanitizeMode::AtaSecureErase,
        SsdSanitizeMode::OpalCryptoErase,
    ];
}

/// What a particular device reports it can do. Returned by
/// [`sanitize_capabilities`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SanitizeCapabilities {
    /// Whether `Trim` / `discard` is supported (used by the
    /// "free-space TRIM" fallback path).
    pub trim: bool,
    /// Set of modes this device's controller / firmware claims to
    /// support. The helper reads `SANITIZE_CAPABILITIES_RESPONSE`
    /// (Windows) or `nvme id-ctrl --output-format=json | jq .sanicap`
    /// (Linux) to populate this.
    pub modes: Vec<SsdSanitizeMode>,
    /// Stable bus type label (`"nvme"`, `"sata"`, `"usb"`,
    /// `"unknown"`) — informational only.
    pub bus: String,
    /// Drive-reported model string. Used in the UI confirmation
    /// dialog so the user can sanity-check before three-confirming
    /// "ALL DATA WILL BE DESTROYED."
    pub model: String,
}

impl SanitizeCapabilities {
    /// Whether the device supports a **guaranteed** cryptographic-
    /// erase mode (instant, defeats forensic recovery, no fallback
    /// to block erase). UI code branching on "is this an instant
    /// op?" should call this; UI labelling like "ALL DATA WILL BE
    /// CRYPTOGRAPHICALLY DESTROYED IN MILLISECONDS" must use this
    /// predicate. Excludes `NvmeFormat` because its FSE bit can
    /// fall back to block erase silently (NVM Express §5.14).
    pub fn has_guaranteed_crypto_erase(&self) -> bool {
        self.modes.contains(&SsdSanitizeMode::NvmeSanitizeCrypto)
            || self.modes.contains(&SsdSanitizeMode::OpalCryptoErase)
    }

    /// Whether the device supports any mode that **may** perform a
    /// cryptographic erase (including `NvmeFormat`, which can fall
    /// back to block erase). Use this for the broader "this device
    /// can do something fast" UX cue, not for exact-time labelling.
    pub fn may_crypto_erase(&self) -> bool {
        self.has_guaranteed_crypto_erase() || self.modes.contains(&SsdSanitizeMode::NvmeFormat)
    }

    /// Phase 44 follow-up note — Phase 44 first cut renamed
    /// `has_crypto_erase` to make the semantics explicit. Existing
    /// callers (none in tree) should use `has_guaranteed_crypto_erase`
    /// for "instant op" labelling and `may_crypto_erase` for "this
    /// device has a fast option" UX. The shim is intentionally NOT
    /// provided so the rename surfaces at compile time.
    #[deprecated(
        since = "0.19.84",
        note = "Phase 44 review M2: split into has_guaranteed_crypto_erase / may_crypto_erase \
                — NvmeFormat can fall back to block erase silently"
    )]
    pub fn has_crypto_erase(&self) -> bool {
        self.may_crypto_erase()
    }
}

/// Final record returned by [`whole_drive_sanitize`].
#[derive(Debug, Clone)]
pub struct SanitizeReport {
    /// Device path that was sanitized (`/dev/nvme0`, `\\.\PhysicalDrive2`).
    pub device: PathBuf,
    /// The mode that ran. May differ from the requested mode when
    /// the helper falls back (e.g. NvmeSanitizeCrypto requested but
    /// the drive only supports NvmeFormat — the helper picks the
    /// closest equivalent).
    pub mode: SsdSanitizeMode,
    /// Wall-clock duration of the sanitize call.
    pub duration: Duration,
}

/// Pluggable broker the engine talks to for the actual privileged
/// command. Real impls live in `copythat-platform` (Windows DeviceIoControl)
/// and a future `copythat-helper` (Linux nvme-cli / hdparm shell-outs);
/// [`NoopSanitizeHelper`] is the test stub that returns "platform helper
/// not available."
///
/// The trait is `Send + Sync` so an `Arc<dyn SanitizeHelper>` can flow
/// through the Tauri runner without cloning.
pub trait SanitizeHelper: Send + Sync + std::fmt::Debug {
    /// Probe the device's sanitize capabilities. Read-only; safe to
    /// call without prompting the user.
    fn capabilities(&self, device: &Path) -> Result<SanitizeCapabilities, String>;

    /// Run the requested sanitization. The implementation BLOCKS the
    /// calling thread for the duration of the call — the engine
    /// runs this on `tokio::task::spawn_blocking`. Returns the
    /// actual mode that ran (may differ from `requested` when the
    /// helper falls back).
    ///
    /// **Destructive.** Caller is responsible for the user's
    /// three-confirmation flow before calling this.
    fn run_sanitize_blocking(
        &self,
        device: &Path,
        requested: SsdSanitizeMode,
    ) -> Result<SsdSanitizeMode, String>;

    /// Phase 44.1 — extended sanitize that reports mid-operation
    /// progress through `progress(percent)`. Same semantics as
    /// [`run_sanitize_blocking`]; the only difference is that
    /// helpers that support polling (NVM Express §5.24.1 SPROG via
    /// `nvme sanitize-log`; Windows
    /// IOCTL_STORAGE_REINITIALIZE_MEDIA progress reads) call
    /// `progress(p)` at ~1 Hz with the current 0-100 percent.
    ///
    /// The default impl ignores `progress` and delegates to
    /// [`run_sanitize_blocking`] — backwards-compatible with
    /// pre-44.1 helper impls.
    ///
    /// `progress` is `Arc<dyn Fn(u8) + Send + Sync + 'static>` so
    /// the helper can clone + share it across worker threads if
    /// needed, and so `whole_drive_sanitize` can pre-bake the
    /// device + mode + events::Sender into the closure.
    fn run_sanitize_blocking_with_progress(
        &self,
        device: &Path,
        requested: SsdSanitizeMode,
        progress: std::sync::Arc<dyn Fn(u8) + Send + Sync + 'static>,
    ) -> Result<SsdSanitizeMode, String> {
        let _ = progress;
        self.run_sanitize_blocking(device, requested)
    }

    /// Phase 44.1 — macOS free-space TRIM (`diskutil secureErase
    /// freespace 0 <disk>`). Distinct from [`SsdSanitizeMode`]
    /// because free-space TRIM does NOT touch the live filesystem;
    /// it only TRIMs the unallocated blocks the FS reports as free.
    /// Useful as a defense-in-depth pass on flash where the user
    /// previously deleted sensitive files but didn't zero the
    /// destination first.
    ///
    /// Default impl returns `NotImplemented`. macOS helper impls
    /// override; Linux + Windows return `NotImplemented` because
    /// the spec only calls out the macOS path
    /// (Linux's `fstrim` runs on mount points not block devices,
    /// and Windows uses Storage Optimizer which is the user's
    /// "Optimize Drives" tool — neither maps cleanly to a
    /// "trim free space NOW" operation).
    fn run_free_space_trim_blocking(&self, device: &Path) -> Result<(), String> {
        let _ = device;
        Err(
            "free-space TRIM is not implemented for this platform; only macOS (diskutil) ships \
             a one-shot free-space TRIM in Phase 44.1. Linux's fstrim works on mount points; \
             Windows uses the OS's scheduled Storage Optimizer task."
                .into(),
        )
    }
}

/// Test / fallback `SanitizeHelper` that always reports "no
/// capabilities" and refuses to run. Ships with the crate so the
/// public API works in CI / tests / dev environments without root.
#[derive(Debug, Clone, Default)]
pub struct NoopSanitizeHelper;

impl NoopSanitizeHelper {
    /// Construct one. No-arg.
    pub fn new() -> Self {
        Self
    }
}

impl SanitizeHelper for NoopSanitizeHelper {
    fn capabilities(&self, _device: &Path) -> Result<SanitizeCapabilities, String> {
        Ok(SanitizeCapabilities {
            trim: false,
            modes: Vec::new(),
            bus: "unknown".into(),
            model: "no-helper-installed".into(),
        })
    }

    fn run_sanitize_blocking(
        &self,
        _device: &Path,
        _requested: SsdSanitizeMode,
    ) -> Result<SsdSanitizeMode, String> {
        Err(
            "no privileged sanitize helper installed; install copythat-helper (Linux/macOS) \
             or run as Administrator (Windows) so the platform-native ioctl path is wired"
                .into(),
        )
    }
}

/// Probe the device's sanitize capabilities. Wraps the helper's
/// [`SanitizeHelper::capabilities`] and translates the helper's
/// `Result<_, String>` into a typed [`ShredError`].
pub fn sanitize_capabilities(
    helper: &dyn SanitizeHelper,
    device: &Path,
) -> Result<SanitizeCapabilities, ShredError> {
    helper.capabilities(device).map_err(|e| ShredError {
        kind: ShredErrorKind::IoOther,
        path: device.to_path_buf(),
        raw_os_error: None,
        message: format!("sanitize_capabilities: {e}"),
    })
}

/// Run a whole-drive sanitize against `device` using `mode`. The
/// helper is invoked on `tokio::task::spawn_blocking` because the
/// underlying syscall (DeviceIoControl, ioctl, exec) blocks for the
/// full sanitize duration — for crypto erase that's milliseconds,
/// for block erase tens of minutes.
///
/// # Caller contract
///
/// 1. **Three-confirmation UX gate.** This function does NOT
///    prompt; it dispatches the destructive command immediately.
///    UI callers must hammer the user with three confirmations
///    (per Phase 44 spec) before invoking; non-UI callers
///    (CLI / IPC / tests) must enforce their own gate.
/// 2. **Cancel-after-dispatch is unsupported.** Once the helper
///    has been spawned, the underlying OS thread continues to
///    completion regardless of caller drops or task cancellation
///    (`tokio::task::spawn_blocking` does NOT cancel the OS
///    thread when the `JoinHandle` is dropped). A misbehaving
///    helper that wedges in `DeviceIoControl` or `nvme-cli`
///    outlives the engine task. Pre-helper cancel via `ctrl` IS
///    honoured and returns `ShredErrorKind::Interrupted`.
/// 3. **Helper trust.** The helper has direct access to the
///    underlying block device. A malicious helper can sanitize
///    any device the host has write access to. Caller is
///    responsible for verifying the helper binary's authenticity
///    (path, signature, etc.) before construction.
///
/// # Events
///
/// Emits [`ShredEvent::SanitizeStarted`] before the helper runs,
/// then [`ShredEvent::SanitizeCompleted`] on success or
/// [`ShredEvent::Failed`] on failure. The progress channel is the
/// same one the per-file shredder uses; UI code can route both
/// through one widget. **No mid-sanitize progress events ship in
/// Phase 44 first cut** — `NvmeSanitizeBlock` can take tens of
/// minutes with only a `Started` event visible to the UI; a
/// follow-up phase will plumb `SPROG` (NVM Express §5.24.1) into a
/// new `SanitizeProgress { percent }` variant.
pub async fn whole_drive_sanitize(
    helper: Arc<dyn SanitizeHelper>,
    device: &Path,
    mode: SsdSanitizeMode,
    ctrl: CopyControl,
    events: mpsc::Sender<ShredEvent>,
) -> Result<SanitizeReport, ShredError> {
    if ctrl.is_cancelled() {
        return Err(ShredError {
            kind: ShredErrorKind::Interrupted,
            path: device.to_path_buf(),
            raw_os_error: None,
            message: "sanitize cancelled before helper invocation".to_string(),
        });
    }

    let _ = events
        .send(ShredEvent::SanitizeStarted {
            device: device.to_path_buf(),
            mode,
        })
        .await;

    let started_at = std::time::Instant::now();
    let device_owned = device.to_path_buf();
    let helper_clone = Arc::clone(&helper);

    // Phase 44.1 — bake `(events, device, mode)` into a `Send + Sync`
    // closure the helper can call for SPROG / progress reads. The
    // closure's `events.blocking_send` blocks until the async
    // receiver picks the event up; failures (closed channel) are
    // swallowed because progress is advisory.
    let events_for_progress = events.clone();
    let device_for_progress = device.to_path_buf();
    let mode_for_progress = mode;
    let progress_callback: Arc<dyn Fn(u8) + Send + Sync + 'static> =
        Arc::new(move |percent: u8| {
            let clamped = percent.min(100);
            // Phase 44.1 post-review (M1) — `try_send` instead of
            // `blocking_send` so a sluggish UI consumer cannot pin
            // a tokio blocking-pool worker indefinitely. Progress
            // events are advisory by contract; dropping a tick on
            // backpressure is the correct behaviour.
            let _ = events_for_progress.try_send(ShredEvent::SanitizeProgress {
                device: device_for_progress.clone(),
                mode: mode_for_progress,
                percent: clamped,
            });
        });

    let outcome = tokio::task::spawn_blocking(move || {
        helper_clone.run_sanitize_blocking_with_progress(&device_owned, mode, progress_callback)
    })
    .await
    .map_err(|e| ShredError {
        kind: ShredErrorKind::IoOther,
        path: device.to_path_buf(),
        raw_os_error: None,
        message: format!("sanitize helper join failed: {e}"),
    })?;

    match outcome {
        Ok(actual_mode) => {
            let report = SanitizeReport {
                device: device.to_path_buf(),
                mode: actual_mode,
                duration: started_at.elapsed(),
            };
            let _ = events
                .send(ShredEvent::SanitizeCompleted {
                    device: device.to_path_buf(),
                    mode: actual_mode,
                    duration: report.duration,
                })
                .await;
            Ok(report)
        }
        Err(msg) => {
            let err = ShredError {
                kind: ShredErrorKind::IoOther,
                path: device.to_path_buf(),
                raw_os_error: None,
                message: format!("sanitize helper failed: {msg}"),
            };
            let _ = events.send(ShredEvent::Failed { err: err.clone() }).await;
            Err(err)
        }
    }
}

/// Pluggable copy-on-write filesystem probe (Phase 44.1).
/// `fn(&Path) -> Option<bool>` — `Some(true)` for CoW filesystems
/// (Btrfs / ZFS / APFS / bcachefs / ReFS), `Some(false)` for
/// in-place filesystems (NTFS / ext4 / FAT-family / network mounts),
/// `None` for unknown / probe-failed paths.
pub type CowProbe = fn(&Path) -> Option<bool>;

static COW_PROBE: std::sync::OnceLock<CowProbe> = std::sync::OnceLock::new();

/// Phase 44.1 — install the CoW filesystem probe. The Tauri runner /
/// CLI wires `copythat_platform::is_cow_filesystem` here at app
/// startup:
///
/// ```ignore
/// copythat_secure_delete::set_cow_probe(
///     copythat_platform::is_cow_filesystem,
/// );
/// ```
///
/// First call wins; subsequent calls are silent no-ops (the
/// underlying [`std::sync::OnceLock`] only accepts the first set).
/// Tests that need a different probe must construct a fresh process.
pub fn set_cow_probe(probe: CowProbe) {
    let _ = COW_PROBE.set(probe);
}

/// Detect whether `path` resides on a copy-on-write filesystem
/// (Btrfs / ZFS / APFS) or a thin-provisioned LVM. CoW filesystems
/// reuse blocks for new writes instead of overwriting in place, so
/// per-file overwrite is meaningless — the old blocks remain
/// readable until the FS reclaims them via TRIM/discard or the
/// pool's GC pass.
///
/// Phase 44.1 — when [`set_cow_probe`] has installed the platform-
/// native probe (`copythat_platform::is_cow_filesystem`), this
/// delegates. When no probe is installed, returns `false` so the
/// per-file shredder retains the pre-44.1 contract — a missing
/// probe must NOT cause a blanket refusal. A `false` result means
/// "not CoW or couldn't tell"; callers must NOT treat it as a
/// positive assertion of in-place overwriting.
pub fn is_cow_filesystem(path: &Path) -> bool {
    match COW_PROBE.get() {
        Some(probe) => probe(path).unwrap_or(false),
        None => {
            // No probe installed → preserve the Phase 4 contract.
            // The Tauri runner / CLI installs the platform probe
            // at startup; tests that want to exercise the refusal
            // path can install their own probe via `set_cow_probe`
            // (the smoke test passes a fixture probe that returns
            // true for paths under a tempdir marker).
            false
        }
    }
}

/// Phase 44.1 — final record returned by [`free_space_trim`].
/// Distinct from [`SanitizeReport`] because free-space TRIM does
/// not touch live data; the report carries only the device + the
/// wall-clock duration so a UI can show "free-space TRIM took N
/// seconds" without claiming any sanitize-grade guarantee.
#[derive(Debug, Clone)]
pub struct FreeSpaceTrimReport {
    /// The device that was TRIMmed (`/dev/disk2` on macOS, etc.).
    pub device: PathBuf,
    /// Wall-clock duration of the helper call.
    pub duration: Duration,
}

/// Phase 44.1 — async free-space TRIM entrypoint. Same wire-shape
/// as [`whole_drive_sanitize`] but doesn't carry an
/// [`SsdSanitizeMode`] — free-space TRIM is its own mode by
/// definition, and crossing it with the sanitize enum would
/// mislead UI labelling that branches on
/// [`SanitizeCapabilities::has_guaranteed_crypto_erase`].
///
/// macOS only in Phase 44.1 first cut (the spec calls out
/// `diskutil secureErase freespace 0 disk0`). Linux + Windows
/// helpers return `NotImplemented`; future phases may add Linux
/// `fstrim` integration via the mount-point indirection.
///
/// **Caller contract** mirrors `whole_drive_sanitize`:
/// pre-helper cancel honoured; cancel-after-dispatch unsupported;
/// caller must verify the helper binary's authenticity before
/// invocation.
pub async fn free_space_trim(
    helper: Arc<dyn SanitizeHelper>,
    device: &Path,
    ctrl: CopyControl,
) -> Result<FreeSpaceTrimReport, ShredError> {
    if ctrl.is_cancelled() {
        return Err(ShredError {
            kind: ShredErrorKind::Interrupted,
            path: device.to_path_buf(),
            raw_os_error: None,
            message: "free_space_trim cancelled before helper invocation".to_string(),
        });
    }

    let started_at = std::time::Instant::now();
    let device_owned = device.to_path_buf();
    let helper_clone = Arc::clone(&helper);
    let outcome = tokio::task::spawn_blocking(move || {
        helper_clone.run_free_space_trim_blocking(&device_owned)
    })
    .await
    .map_err(|e| ShredError {
        kind: ShredErrorKind::IoOther,
        path: device.to_path_buf(),
        raw_os_error: None,
        message: format!("free_space_trim helper join failed: {e}"),
    })?;

    match outcome {
        Ok(()) => Ok(FreeSpaceTrimReport {
            device: device.to_path_buf(),
            duration: started_at.elapsed(),
        }),
        Err(msg) => Err(ShredError {
            kind: ShredErrorKind::IoOther,
            path: device.to_path_buf(),
            raw_os_error: None,
            message: format!("free_space_trim helper failed: {msg}"),
        }),
    }
}

/// Phase 44 — refuse to per-file-shred when the filesystem makes
/// the operation meaningless. Mirrors the existing
/// `ShredErrorKind::PurgeNotSupported` shape so UI code can route
/// the localised explanation through the same renderer.
pub fn refuse_shred_on_cow(path: &Path) -> ShredError {
    ShredError {
        kind: ShredErrorKind::ShredMeaningless,
        path: path.to_path_buf(),
        raw_os_error: None,
        message: "per-file shred on a copy-on-write filesystem (Btrfs / ZFS / APFS) does not \
             overwrite the underlying blocks — the FS reuses storage on the next write. \
             Use whole-drive sanitize (NVMe Sanitize Crypto / OPAL Crypto Erase) plus \
             full-disk-encryption key rotation instead."
            .to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};

    /// Mock helper that records invocations + returns scripted
    /// results. The Phase 44 smoke test uses this to exercise
    /// `whole_drive_sanitize` end-to-end without hardware.
    #[derive(Debug, Default)]
    pub struct MockSanitizeHelper {
        pub capabilities_called: AtomicBool,
        pub run_called: AtomicBool,
        pub run_should_fail: bool,
    }

    impl SanitizeHelper for MockSanitizeHelper {
        fn capabilities(&self, _device: &Path) -> Result<SanitizeCapabilities, String> {
            self.capabilities_called.store(true, Ordering::Relaxed);
            Ok(SanitizeCapabilities {
                trim: true,
                modes: vec![
                    SsdSanitizeMode::NvmeSanitizeCrypto,
                    SsdSanitizeMode::NvmeFormat,
                ],
                bus: "nvme".into(),
                model: "MOCK NVMe SSD".into(),
            })
        }

        fn run_sanitize_blocking(
            &self,
            _device: &Path,
            requested: SsdSanitizeMode,
        ) -> Result<SsdSanitizeMode, String> {
            self.run_called.store(true, Ordering::Relaxed);
            if self.run_should_fail {
                Err("mock-induced failure".to_string())
            } else {
                Ok(requested)
            }
        }
    }

    #[test]
    fn ssd_sanitize_mode_name_is_stable() {
        for m in SsdSanitizeMode::ALL {
            let name = m.name();
            assert!(!name.is_empty());
            assert!(
                name.starts_with("nvme-") || name.starts_with("ata-") || name.starts_with("opal-")
            );
        }
    }

    #[test]
    fn capabilities_helper_is_callable() {
        let helper = NoopSanitizeHelper::new();
        let caps = sanitize_capabilities(&helper, Path::new("/dev/nvme0")).unwrap();
        assert!(caps.modes.is_empty(), "noop helper should report no modes");
        assert_eq!(caps.bus, "unknown");
    }

    #[test]
    fn noop_helper_refuses_run() {
        let helper = NoopSanitizeHelper::new();
        let err = helper
            .run_sanitize_blocking(Path::new("/dev/nvme0"), SsdSanitizeMode::NvmeSanitizeCrypto)
            .unwrap_err();
        assert!(err.contains("no privileged sanitize helper"));
    }

    #[test]
    fn refuse_shred_on_cow_carries_meaningless_kind() {
        let err = refuse_shred_on_cow(Path::new("/btrfs/secret.pdf"));
        assert_eq!(err.kind, ShredErrorKind::ShredMeaningless);
        assert!(err.message.contains("copy-on-write"));
    }

    #[test]
    fn capabilities_predicates_split_correctly() {
        // Phase 44 review M2 — has_guaranteed_crypto_erase excludes
        // NvmeFormat (which can fall back to block); may_crypto_erase
        // includes it.
        let mut caps = SanitizeCapabilities::default();
        assert!(!caps.has_guaranteed_crypto_erase());
        assert!(!caps.may_crypto_erase());

        caps.modes.push(SsdSanitizeMode::AtaSecureErase);
        assert!(!caps.has_guaranteed_crypto_erase());
        assert!(!caps.may_crypto_erase());

        caps.modes.push(SsdSanitizeMode::NvmeFormat);
        assert!(
            !caps.has_guaranteed_crypto_erase(),
            "NvmeFormat alone is not a guaranteed crypto erase"
        );
        assert!(caps.may_crypto_erase(), "NvmeFormat may crypto-erase");

        caps.modes.push(SsdSanitizeMode::NvmeSanitizeCrypto);
        assert!(caps.has_guaranteed_crypto_erase());
        assert!(caps.may_crypto_erase());
    }

    #[tokio::test(flavor = "current_thread")]
    async fn whole_drive_sanitize_through_mock_helper() {
        let helper: Arc<dyn SanitizeHelper> = Arc::new(MockSanitizeHelper::default());
        let (tx, mut rx) = mpsc::channel::<ShredEvent>(16);
        let ctrl = CopyControl::new();

        let report = whole_drive_sanitize(
            helper.clone(),
            Path::new("/dev/nvme0"),
            SsdSanitizeMode::NvmeSanitizeCrypto,
            ctrl,
            tx,
        )
        .await
        .expect("mock sanitize should succeed");

        assert_eq!(report.mode, SsdSanitizeMode::NvmeSanitizeCrypto);
        assert_eq!(report.device, Path::new("/dev/nvme0"));

        // Drain the channel — should see Started + Completed.
        let mut saw_started = false;
        let mut saw_completed = false;
        while let Ok(evt) = rx.try_recv() {
            match evt {
                ShredEvent::SanitizeStarted { .. } => saw_started = true,
                ShredEvent::SanitizeCompleted { .. } => saw_completed = true,
                _ => {}
            }
        }
        assert!(saw_started);
        assert!(saw_completed);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn whole_drive_sanitize_propagates_helper_failure() {
        let helper: Arc<dyn SanitizeHelper> = Arc::new(MockSanitizeHelper {
            run_should_fail: true,
            ..MockSanitizeHelper::default()
        });
        let (tx, mut rx) = mpsc::channel::<ShredEvent>(16);
        let ctrl = CopyControl::new();

        let err = whole_drive_sanitize(
            helper,
            Path::new("/dev/nvme0"),
            SsdSanitizeMode::NvmeSanitizeBlock,
            ctrl,
            tx,
        )
        .await
        .unwrap_err();
        assert_eq!(err.kind, ShredErrorKind::IoOther);
        assert!(err.message.contains("mock-induced failure"));

        // Drain — should see Started + Failed.
        let mut saw_failed = false;
        while let Ok(evt) = rx.try_recv() {
            if let ShredEvent::Failed { .. } = evt {
                saw_failed = true;
            }
        }
        assert!(saw_failed);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn whole_drive_sanitize_honors_pre_helper_cancel() {
        let helper: Arc<dyn SanitizeHelper> = Arc::new(MockSanitizeHelper::default());
        let (tx, _rx) = mpsc::channel::<ShredEvent>(16);
        let ctrl = CopyControl::new();
        ctrl.cancel();

        let err = whole_drive_sanitize(
            helper,
            Path::new("/dev/nvme0"),
            SsdSanitizeMode::NvmeFormat,
            ctrl,
            tx,
        )
        .await
        .unwrap_err();
        assert_eq!(err.kind, ShredErrorKind::Interrupted);
    }
}
