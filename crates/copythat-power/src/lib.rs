//! Phase 31 — power-aware copying.
//!
//! Watches the running host's battery / metered-network / presentation
//! / fullscreen / thermal state and emits [`PowerEvent`]s on a
//! broadcast channel. The Tauri runner subscribes, maps each event
//! through the user's [`PowerPolicies`], and pauses / resumes /
//! bandwidth-caps active jobs accordingly.
//!
//! # Scope
//!
//! Phase 31 ships the full event + policy + bus + action machinery
//! and a real cross-platform battery probe via the `battery` crate.
//! Presentation / fullscreen detection and thermal throttling are
//! stubbed — the probe traits are in place, returning `false` /
//! `Unknown` by default — so the runner and UI exercise the full
//! pause-path end-to-end while the OS FFI probes land in Phase 31b:
//!
//! - Windows: `SHQueryUserNotificationState` returning `QUNS_BUSY` /
//!   `QUNS_PRESENTATION_MODE`.
//! - macOS: `IOPMAssertionCreateWithName` introspection +
//!   `CGDisplayIsCaptured`.
//! - Linux: DBus `org.freedesktop.ScreenSaver` inhibit state +
//!   `_NET_WM_STATE_FULLSCREEN` X11 / GNOME Shell DBus.
//!
//! The [`crate::source::SyntheticProbes`] type gives the smoke test
//! and IPC `inject_power_event` command a way to drive
//! [`PowerEvent`]s without the real probes, so feature work that
//! consumes the power bus can ship ahead of the platform FFI.

#![forbid(unsafe_code)]

pub mod bus;
pub mod event;
pub mod policy;
pub mod source;

pub use bus::{PowerBus, PowerBusError, PowerSubscriber};
pub use event::{NetworkClass, PowerEvent, ThermalKind};
pub use policy::{
    BatteryPolicy, FullscreenPolicy, NetworkPolicy, PowerAction, PowerPolicies, PowerReason,
    PowerState, PresentationPolicy, ThermalPolicy, apply_event, compute_action,
};
pub use source::{
    BatteryProbe, BatterySnapshot, FullscreenProbe, NullProbes, PresentationProbe, ProbeSet,
    SyntheticProbes, ThermalProbe, battery_snapshot,
};
