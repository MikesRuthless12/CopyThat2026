//! Phase 44 smoke test — SSD-aware whole-drive sanitize + CoW
//! per-file refusal.
//!
//! What this proves:
//!
//! 1. `sanitize_capabilities` runs through the bundled
//!    `NoopSanitizeHelper` and returns a populated
//!    `SanitizeCapabilities` (no modes, but the call doesn't
//!    crash and the typed result is shaped correctly).
//! 2. `whole_drive_sanitize` through a mock helper drives the
//!    `Started → Completed` event sequence and returns a
//!    `SanitizeReport` whose `mode` matches the request.
//! 3. The same call against a mock helper that fails surfaces the
//!    failure as `ShredErrorKind::IoOther` plus a `Failed` event.
//! 4. Pre-helper cancel via `CopyControl::cancel` short-circuits
//!    with `ShredErrorKind::Interrupted` BEFORE the helper is
//!    invoked.
//! 5. The `refuse_shred_on_cow` helper returns
//!    `ShredErrorKind::ShredMeaningless` with prose that points
//!    the user at whole-drive sanitize.
//!
//! The actual privileged helper (Linux nvme-cli / Windows
//! DeviceIoControl / macOS diskutil) is NOT exercised — Phase 44
//! first cut ships only the trait surface + the `Noop` fallback;
//! a follow-up phase wires the platform-native impls. This smoke
//! is the spec's "use a mocked privileged helper" path.
//!
//! Runtime: <100 ms. No disk writes; everything is in-memory state
//! against the mock helper.

use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use copythat_core::CopyControl;
use copythat_secure_delete::{
    NoopSanitizeHelper, SanitizeCapabilities, SanitizeHelper, ShredErrorKind, ShredEvent,
    SsdSanitizeMode, refuse_shred_on_cow, sanitize_capabilities, whole_drive_sanitize,
};
use tokio::sync::mpsc;

/// Mock helper exposed at the top level so multiple test functions
/// can construct it. Records invocations + returns scripted
/// outcomes.
#[derive(Debug, Default)]
struct ScriptedSanitizeHelper {
    capabilities_called: AtomicBool,
    run_called: AtomicBool,
    run_should_fail: bool,
}

impl SanitizeHelper for ScriptedSanitizeHelper {
    fn capabilities(&self, _device: &Path) -> Result<SanitizeCapabilities, String> {
        self.capabilities_called.store(true, Ordering::Relaxed);
        Ok(SanitizeCapabilities {
            trim: true,
            modes: vec![
                SsdSanitizeMode::NvmeSanitizeCrypto,
                SsdSanitizeMode::NvmeFormat,
            ],
            bus: "nvme".into(),
            model: "PHASE44-SMOKE-MOCK".into(),
        })
    }

    fn run_sanitize_blocking(
        &self,
        _device: &Path,
        requested: SsdSanitizeMode,
    ) -> Result<SsdSanitizeMode, String> {
        self.run_called.store(true, Ordering::Relaxed);
        if self.run_should_fail {
            Err("mock-induced failure".into())
        } else {
            Ok(requested)
        }
    }
}

#[tokio::test(flavor = "current_thread")]
async fn noop_helper_round_trips_capability_probe() {
    let helper = NoopSanitizeHelper::new();
    let caps =
        sanitize_capabilities(&helper, Path::new("/dev/nvme0")).expect("noop should not error");
    assert!(caps.modes.is_empty(), "noop helper reports no modes",);
    assert_eq!(caps.bus, "unknown");
    assert!(!caps.trim);
}

#[tokio::test(flavor = "current_thread")]
async fn whole_drive_sanitize_through_mock_fires_started_and_completed() {
    let helper: Arc<dyn SanitizeHelper> = Arc::new(ScriptedSanitizeHelper::default());
    let (tx, mut rx) = mpsc::channel::<ShredEvent>(16);
    let ctrl = CopyControl::new();

    let report = whole_drive_sanitize(
        helper,
        Path::new("/dev/nvme0"),
        SsdSanitizeMode::NvmeSanitizeCrypto,
        ctrl,
        tx,
    )
    .await
    .expect("mock sanitize should succeed");

    assert_eq!(report.mode, SsdSanitizeMode::NvmeSanitizeCrypto);

    let mut started = false;
    let mut completed = false;
    while let Ok(evt) = rx.try_recv() {
        match evt {
            ShredEvent::SanitizeStarted { .. } => started = true,
            ShredEvent::SanitizeCompleted { .. } => completed = true,
            _ => {}
        }
    }
    assert!(started && completed, "expected Started + Completed events");
}

#[tokio::test(flavor = "current_thread")]
async fn whole_drive_sanitize_propagates_helper_failure() {
    let helper: Arc<dyn SanitizeHelper> = Arc::new(ScriptedSanitizeHelper {
        run_should_fail: true,
        ..ScriptedSanitizeHelper::default()
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
    .expect_err("scripted failure should propagate");
    assert_eq!(err.kind, ShredErrorKind::IoOther);
    assert!(err.message.contains("mock-induced failure"));

    let mut saw_failed = false;
    while let Ok(evt) = rx.try_recv() {
        if matches!(evt, ShredEvent::Failed { .. }) {
            saw_failed = true;
        }
    }
    assert!(saw_failed, "Failed event should fire on helper failure");
}

#[tokio::test(flavor = "current_thread")]
async fn pre_helper_cancel_short_circuits_with_interrupted() {
    // Hold the concrete Arc<ScriptedSanitizeHelper> ourselves so we
    // can read the atomic afterward; pass a coerced trait-object
    // clone into `whole_drive_sanitize`. No `unsafe` needed.
    let concrete = Arc::new(ScriptedSanitizeHelper::default());
    let helper_for_call: Arc<dyn SanitizeHelper> = concrete.clone();
    let (tx, _rx) = mpsc::channel::<ShredEvent>(16);
    let ctrl = CopyControl::new();
    ctrl.cancel();

    let err = whole_drive_sanitize(
        helper_for_call,
        Path::new("/dev/nvme0"),
        SsdSanitizeMode::NvmeFormat,
        ctrl,
        tx,
    )
    .await
    .expect_err("cancel-before-helper should error");
    assert_eq!(err.kind, ShredErrorKind::Interrupted);

    assert!(
        !concrete.run_called.load(Ordering::Relaxed),
        "helper.run must NOT have been called after pre-cancel"
    );
}

#[test]
fn refuse_shred_on_cow_classifies_as_meaningless() {
    let err = refuse_shred_on_cow(Path::new("/btrfs/secret.pdf"));
    assert_eq!(err.kind, ShredErrorKind::ShredMeaningless);
    assert!(
        err.message.contains("copy-on-write"),
        "error message should explain WHY shredding is meaningless"
    );
    assert!(
        err.message.contains("whole-drive sanitize"),
        "error message should point the user at the right alternative"
    );
}

#[test]
fn ssd_sanitize_mode_names_are_stable() {
    // Wire-stable across releases — UI / CLI / IPC consumers branch
    // on these strings.
    assert_eq!(SsdSanitizeMode::NvmeFormat.name(), "nvme-format");
    assert_eq!(
        SsdSanitizeMode::NvmeSanitizeBlock.name(),
        "nvme-sanitize-block"
    );
    assert_eq!(
        SsdSanitizeMode::NvmeSanitizeCrypto.name(),
        "nvme-sanitize-crypto"
    );
    assert_eq!(SsdSanitizeMode::AtaSecureErase.name(), "ata-secure-erase");
    assert_eq!(SsdSanitizeMode::OpalCryptoErase.name(), "opal-crypto-erase");
    assert_eq!(SsdSanitizeMode::ALL.len(), 5);
}
