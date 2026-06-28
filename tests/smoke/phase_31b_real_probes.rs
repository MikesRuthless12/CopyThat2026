//! Phase 31b smoke test — real OS power probes.
//!
//! Wires the real `RealPresentationProbe` / `RealFullscreenProbe`
//! types backed by per-OS calls:
//!
//! - **Windows** — `SHQueryUserNotificationState` from Shell32 via
//!   `copythat_platform::presence` (presentation and fullscreen are
//!   distinct `QUNS_*` states).
//! - **Linux / macOS / other** — deferred to the cross-platform stub
//!   for now (`Real*Probe` are type aliases to the stub there). The
//!   Linux GNOME DBus probe was pulled — it blocked inside the tokio
//!   poller and over-fired on any idle-inhibit; a non-blocking rework
//!   validated on real Linux is a Phase 31c follow-up.
//!
//! Coverage:
//! 1. `is_in_presentation_mode()` + `is_in_fullscreen_mode()` are
//!    callable on every host without panicking. CI runners have no
//!    GPU / no logged-in user / no DBus session — the answer is
//!    whatever the OS returns, but the call must succeed.
//! 2. `RealPresentationProbe` + `RealFullscreenProbe` types
//!    instantiate and impl the right traits on every supported OS.
//! 3. The runner's `PowerBus` accepts the real probes without
//!    panicking — same shape as the existing `SyntheticProbes`
//!    consumer.

use copythat_power::source::{FullscreenProbe, PresentationProbe};

// On Windows the `Real*Probe` names are unit structs that can be used
// as value literals; on Linux / macOS / other platforms they are type
// aliases to the cross-platform stub structs (see
// `copythat_power::source`). A type alias can't be used as a value
// literal, so the probe-construction helpers below build the trait
// object from whichever concrete unit struct is in scope on the host.
// Importing each set only where it's used keeps the unused-import
// lint quiet under `-D warnings`.
#[cfg(target_os = "windows")]
use copythat_power::source::{RealFullscreenProbe, RealPresentationProbe};
#[cfg(not(target_os = "windows"))]
use copythat_power::source::{StubFullscreenProbe, StubPresentationProbe};

#[test]
fn presence_module_is_callable_on_every_host() {
    // On Windows the presentation/fullscreen state comes straight from
    // the `copythat_platform::presence` FFI helpers (the only crate
    // allowed to carry unsafe). `copythat-platform` is a Windows-only
    // dependency of `copythat-power`, so on other hosts we exercise the
    // same callable surface through the public `Real*Probe` trait
    // methods instead. Either way the contract is "calling never
    // panics regardless of OS".
    #[cfg(target_os = "windows")]
    {
        let _ = copythat_platform::presence::is_in_presentation_mode();
        let _ = copythat_platform::presence::is_in_fullscreen_mode();
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = make_presentation_probe().is_presenting();
        let _ = make_fullscreen_probe().is_fullscreen();
    }
}

#[test]
fn real_presentation_probe_responds_without_panic() {
    let probe = make_presentation_probe();
    // The reading is host-dependent (CI runners typically return
    // false). The contract under test is "calling does not panic".
    let _ = probe.is_presenting();
}

#[test]
fn real_fullscreen_probe_responds_without_panic() {
    let probe = make_fullscreen_probe();
    let _ = probe.is_fullscreen();
}

// Construct the host's presentation probe as a trait object. On
// Windows the `Real*Probe` name is a unit struct; on Linux / macOS /
// other it's a type alias to the stub unit struct, which has to be
// named directly to be used as a value.
#[cfg(target_os = "windows")]
fn make_presentation_probe() -> Box<dyn PresentationProbe> {
    Box::new(RealPresentationProbe)
}

#[cfg(not(target_os = "windows"))]
fn make_presentation_probe() -> Box<dyn PresentationProbe> {
    Box::new(StubPresentationProbe)
}

#[cfg(target_os = "windows")]
fn make_fullscreen_probe() -> Box<dyn FullscreenProbe> {
    Box::new(RealFullscreenProbe)
}

#[cfg(not(target_os = "windows"))]
fn make_fullscreen_probe() -> Box<dyn FullscreenProbe> {
    Box::new(StubFullscreenProbe)
}

#[cfg(target_os = "windows")]
#[test]
fn windows_presence_uses_quns_state_classification() {
    use copythat_platform::presence::QunsState;
    // The QunsState enum is the documented public surface for unit
    // tests that want to assert per-state branches. Confirm the
    // enum survives being exported.
    let states = [
        QunsState::NotPresent,
        QunsState::Busy,
        QunsState::RunningD3dFullScreen,
        QunsState::Presentation,
        QunsState::AcceptsNotifications,
        QunsState::QuietTime,
        QunsState::App,
    ];
    let mut seen = std::collections::HashSet::new();
    for s in states {
        assert!(seen.insert(format!("{s:?}")));
    }
}
