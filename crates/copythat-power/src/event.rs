//! [`PowerEvent`] — the cross-dimensional event stream that the
//! power bus carries.
//!
//! Each variant corresponds to a state transition on one dimension.
//! The poller emits a variant only when it observes a change — a
//! constant-state poll emits nothing — so subscribers can assume
//! every received event is *news*.

use serde::{Deserialize, Serialize};

/// Re-export of `copythat_shape::NetworkClass` so callers can import
/// both from one place. Phase 21 owns the canonical enum (it drives
/// the Network Settings auto-throttle); Phase 31 reuses it verbatim
/// in [`PowerEvent::NetworkClassChanged`].
pub use copythat_shape::NetworkClass;

/// A single transition on one power-state dimension.
///
/// Marked `#[non_exhaustive]` so future phases (Phase 31b adds real
/// presentation/fullscreen probes; Phase 44 may add whole-drive
/// secure-erase completion signals here) can extend without breaking
/// downstream `match`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum PowerEvent {
    /// Battery state changed — plugged/unplugged or percent crossed
    /// a threshold that the caller cares about (the poller emits this
    /// on every significant percent delta so the UI can render live
    /// battery readouts without needing its own poll).
    BatteryStateChanged {
        /// `true` if the system is currently running on battery.
        on_battery: bool,
        /// Current charge percentage in the range `0.0..=100.0`.
        /// `f32::NAN` when the OS didn't report a value.
        percent: f32,
    },
    /// Metered-network classification flipped. Mirrors Phase 21's
    /// `current_network_class()` but wired through the bus so power-
    /// aware logic doesn't have to poll the shape crate directly.
    NetworkClassChanged { class: NetworkClass },
    /// Presentation (Zoom / Teams / Keynote / PowerPoint etc.)
    /// started or stopped. Driven by [`crate::source::PresentationProbe`].
    PresentationStateChanged { presenting: bool },
    /// Any app entered or exited fullscreen — anchored on the
    /// foreground window. Games, video players, and browser
    /// full-screen mode all surface here.
    FullscreenChanged { fullscreen: bool },
    /// CPU thermal throttling state — `true` when achieved frequency
    /// drops below base frequency (x86) or when the OS explicitly
    /// signals a thermal pressure level on ARM.
    ThermalChanged { throttling: bool, kind: ThermalKind },
}

/// How the thermal probe arrived at its conclusion. `Unknown` is the
/// honest default on Apple Silicon where the `powermetrics` channel
/// requires elevation we refuse to demand at copy time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ThermalKind {
    /// Could not determine — e.g. non-x86 host without a privileged
    /// helper, or the CPU didn't expose the needed leaf.
    #[default]
    Unknown,
    /// x86 CPUID leaf 6 (Thermal and Power Management) — the
    /// achieved-frequency MSR + base-frequency comparison the
    /// `raw-cpuid` crate surfaces.
    X86Cpuid,
    /// Placeholder for future platform sources (macOS
    /// `IOPowerSources` / Linux `/sys/class/thermal`). Carried as a
    /// distinct variant so a Phase 31b addition doesn't break the
    /// wire format.
    OsReported,
}
