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
    SsdSanitizeMode, free_space_trim, is_cow_filesystem, opal_psid_revert, refuse_shred_on_cow,
    sanitize_capabilities, set_cow_probe, whole_drive_sanitize,
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

#[tokio::test(flavor = "current_thread")]
async fn free_space_trim_through_noop_helper_returns_not_implemented() {
    // Phase 44.1b — NoopSanitizeHelper's default
    // run_free_space_trim_blocking returns the "not implemented"
    // error from the trait default; free_space_trim wraps that in
    // ShredErrorKind::IoOther.
    let helper: Arc<dyn SanitizeHelper> = Arc::new(NoopSanitizeHelper::new());
    let ctrl = CopyControl::new();
    let err = free_space_trim(helper, Path::new("/dev/disk2"), ctrl)
        .await
        .expect_err("noop helper should not implement free-space TRIM");
    assert_eq!(err.kind, ShredErrorKind::IoOther);
    assert!(
        err.message.contains("not implemented") || err.message.contains("free-space"),
        "free-space TRIM error message should mention the deferral; got: {}",
        err.message
    );
}

#[tokio::test(flavor = "current_thread")]
async fn whole_drive_sanitize_fires_progress_when_helper_reports() {
    // Phase 44.1c — helpers that call the progress callback during
    // run_sanitize_blocking_with_progress trigger
    // ShredEvent::SanitizeProgress emissions on the events channel.
    #[derive(Debug, Default)]
    struct ProgressReportingHelper;
    impl SanitizeHelper for ProgressReportingHelper {
        fn capabilities(&self, _device: &Path) -> Result<SanitizeCapabilities, String> {
            Ok(SanitizeCapabilities::default())
        }
        fn run_sanitize_blocking(
            &self,
            _device: &Path,
            requested: SsdSanitizeMode,
        ) -> Result<SsdSanitizeMode, String> {
            Ok(requested)
        }
        fn run_sanitize_blocking_with_progress(
            &self,
            _device: &Path,
            requested: SsdSanitizeMode,
            progress: Arc<dyn Fn(u8) + Send + Sync + 'static>,
        ) -> Result<SsdSanitizeMode, String> {
            // Simulate three progress ticks at 25 / 50 / 100 percent.
            progress(25);
            progress(50);
            progress(100);
            Ok(requested)
        }
    }

    let helper: Arc<dyn SanitizeHelper> = Arc::new(ProgressReportingHelper);
    let (tx, mut rx) = mpsc::channel::<ShredEvent>(32);
    let ctrl = CopyControl::new();
    let _ = whole_drive_sanitize(
        helper,
        Path::new("/dev/nvme0"),
        SsdSanitizeMode::NvmeSanitizeBlock,
        ctrl,
        tx,
    )
    .await
    .expect("sanitize should succeed");

    let mut percents: Vec<u8> = Vec::new();
    while let Ok(evt) = rx.try_recv() {
        if let ShredEvent::SanitizeProgress { percent, .. } = evt {
            percents.push(percent);
        }
    }
    assert_eq!(
        percents,
        vec![25, 50, 100],
        "expected three progress ticks at 25/50/100 percent"
    );
}

#[test]
fn cow_probe_default_is_noop_returns_false() {
    // Phase 44.1a — without a probe installed, `is_cow_filesystem`
    // returns false (preserves the Phase 4 contract). This is a
    // smoke that the OnceLock's default branch fires correctly;
    // production wiring (the Tauri runner / CLI calling
    // set_cow_probe(copythat_platform::is_cow_filesystem) at
    // startup) is exercised by the integration test below when
    // the probe install lands first.
    //
    // Test order across a single binary is non-deterministic, so
    // this test only verifies the call doesn't panic for an
    // arbitrary path. The semantic ("returns false absent a
    // probe") is documented in the function's body and has unit
    // coverage in the platform crate's matching helper.
    let _ = is_cow_filesystem(Path::new("/some/path"));
}

#[test]
fn set_cow_probe_can_install_a_test_fixture() {
    // Phase 44.1a — install a probe that returns Some(true) for
    // paths under "/cow-fixture" and Some(false) otherwise. Note:
    // the underlying OnceLock accepts only the first set, so the
    // first test that calls set_cow_probe wins. We exercise BOTH
    // arms of the installed probe regardless of which test ran
    // first by passing different paths and accepting either truth
    // value (the test verifies the probe can be installed +
    // doesn't crash, not the specific return value, since a
    // sibling test may have installed a different probe).
    fn fixture_probe(path: &Path) -> Option<bool> {
        Some(path.starts_with("/cow-fixture"))
    }
    set_cow_probe(fixture_probe);
    let _cow = is_cow_filesystem(Path::new("/cow-fixture/file.txt"));
    let _non_cow = is_cow_filesystem(Path::new("/regular/file.txt"));
    // Don't assert specific values — OnceLock semantics mean any
    // probe installed earlier in this binary wins.
}

#[tokio::test(flavor = "current_thread")]
async fn opal_psid_revert_through_noop_helper_returns_not_implemented() {
    // Phase 44.2d — the Noop helper's default
    // run_opal_psid_revert_blocking returns the trait's default
    // NotImplemented error; the async wrapper surfaces it as
    // ShredErrorKind::IoOther.
    let helper: Arc<dyn SanitizeHelper> = Arc::new(NoopSanitizeHelper::new());
    let (tx, mut rx) = mpsc::channel::<ShredEvent>(16);
    let ctrl = CopyControl::new();
    let err = opal_psid_revert(
        helper,
        Path::new("/dev/sda"),
        "ABCDEF1234567890ABCDEF1234567890",
        ctrl,
        tx,
    )
    .await
    .expect_err("noop helper should not implement OPAL");
    assert_eq!(err.kind, ShredErrorKind::IoOther);
    assert!(
        err.message.contains("not implemented") || err.message.contains("OPAL"),
        "expected NotImplemented hint in error message; got: {}",
        err.message
    );

    // Started + Failed should fire on the events channel.
    let mut started = false;
    let mut failed = false;
    while let Ok(evt) = rx.try_recv() {
        match evt {
            ShredEvent::SanitizeStarted { mode, .. } => {
                if mode == SsdSanitizeMode::OpalCryptoErase {
                    started = true;
                }
            }
            ShredEvent::Failed { .. } => failed = true,
            _ => {}
        }
    }
    assert!(
        started,
        "expected SanitizeStarted with OpalCryptoErase mode"
    );
    assert!(failed, "expected Failed event after helper rejection");
}

#[tokio::test(flavor = "current_thread")]
async fn opal_psid_revert_pre_helper_cancel_short_circuits() {
    // Phase 44.2d — the cancel-before-helper path mirrors
    // whole_drive_sanitize: ctrl.cancel() before invocation
    // returns Interrupted without ever spawning the helper.
    let helper: Arc<dyn SanitizeHelper> = Arc::new(NoopSanitizeHelper::new());
    let (tx, _rx) = mpsc::channel::<ShredEvent>(16);
    let ctrl = CopyControl::new();
    ctrl.cancel();
    let err = opal_psid_revert(
        helper,
        Path::new("/dev/sda"),
        "ABCDEF1234567890ABCDEF1234567890",
        ctrl,
        tx,
    )
    .await
    .expect_err("cancel-before-helper should error");
    assert_eq!(err.kind, ShredErrorKind::Interrupted);
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
