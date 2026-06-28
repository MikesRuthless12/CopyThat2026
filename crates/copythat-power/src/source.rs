//! Probe traits + concrete implementations that the tokio poller
//! reads each tick.
//!
//! Keeping each dimension behind its own tiny trait makes the poller
//! trivial (sample each probe, compare to last-known, emit on
//! transition) and the test harness free ([`SyntheticProbes`] lets
//! the smoke inject any reading).

use std::sync::{Arc, Mutex};

use crate::event::{NetworkClass, ThermalKind};

// ---------------------------------------------------------------------
// Traits — the surface the poller consumes
// ---------------------------------------------------------------------

/// Battery probe — returns a snapshot or a "no battery" signal.
pub trait BatteryProbe: Send + Sync + 'static {
    fn snapshot(&self) -> Option<BatterySnapshot>;
}

/// Snapshot returned by [`BatteryProbe::snapshot`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BatterySnapshot {
    /// `true` if the system is discharging (i.e. unplugged).
    pub on_battery: bool,
    /// Charge percentage `0.0..=100.0`. `f32::NAN` if the OS didn't
    /// report one.
    pub percent: f32,
}

impl BatterySnapshot {
    pub fn plugged_in(percent: f32) -> Self {
        Self {
            on_battery: false,
            percent,
        }
    }

    pub fn on_battery(percent: f32) -> Self {
        Self {
            on_battery: true,
            percent,
        }
    }
}

pub trait PresentationProbe: Send + Sync + 'static {
    fn is_presenting(&self) -> bool;
}

pub trait FullscreenProbe: Send + Sync + 'static {
    fn is_fullscreen(&self) -> bool;
}

pub trait ThermalProbe: Send + Sync + 'static {
    fn is_throttling(&self) -> (bool, ThermalKind);
}

pub trait NetworkProbe: Send + Sync + 'static {
    fn class(&self) -> NetworkClass;
}

// ---------------------------------------------------------------------
// Real cross-platform battery probe via the `battery` crate
// ---------------------------------------------------------------------

/// Cross-platform battery snapshot using the `battery` crate. Returns
/// `None` on hosts without a battery (desktops, VMs, CI runners).
pub fn battery_snapshot() -> Option<BatterySnapshot> {
    let manager = battery::Manager::new().ok()?;
    let mut best: Option<BatterySnapshot> = None;
    for result in manager.batteries().ok()? {
        let Ok(bat) = result else {
            continue;
        };
        // Phase 31: report the first battery the manager yields.
        // Multi-battery laptops (ThinkPad, ROG) expose the internal
        // + external pack separately — treating the first as
        // authoritative matches what Windows' own power indicator
        // does in practice.
        let state = bat.state();
        let on_battery = matches!(state, battery::State::Discharging | battery::State::Empty);
        // `state_of_charge()` returns a unitless ratio; convert to
        // percent. `value()` accesses the inner `f32` ratio.
        let percent = bat.state_of_charge().value * 100.0;
        best = Some(BatterySnapshot {
            on_battery,
            percent,
        });
        break;
    }
    best
}

/// Real-battery probe that polls the `battery` crate every call.
/// Cheap (the manager is reopened per call because the crate's
/// `Manager` isn't `Sync`). Returns `None` when no battery is found.
pub struct RealBatteryProbe;

impl BatteryProbe for RealBatteryProbe {
    fn snapshot(&self) -> Option<BatterySnapshot> {
        battery_snapshot()
    }
}

// ---------------------------------------------------------------------
// Stub probes — return the "nothing adverse" reading by default
// ---------------------------------------------------------------------

/// Presentation probe stub — always reports "not presenting".
/// Used as the cross-platform fallback when the per-OS probe
/// can't initialise (no DBus session on Linux headless, etc.).
pub struct StubPresentationProbe;

impl PresentationProbe for StubPresentationProbe {
    fn is_presenting(&self) -> bool {
        false
    }
}

/// Fullscreen probe stub — always reports "not fullscreen".
pub struct StubFullscreenProbe;

impl FullscreenProbe for StubFullscreenProbe {
    fn is_fullscreen(&self) -> bool {
        false
    }
}

// ---------------------------------------------------------------------
// Phase 31b — real OS-specific presentation / fullscreen probes.
// ---------------------------------------------------------------------

/// Windows presentation probe — routes through
/// [`copythat_platform::presence::is_in_presentation_mode`] so the
/// raw `SHQueryUserNotificationState` FFI call lives in the only
/// crate where unsafe is allowed. Returns `true` only when the OS
/// reports `QUNS_PRESENTATION_MODE` (slideshow / focus-assist /
/// Do-Not-Disturb) — the state the Phase 31 `PresentationPolicy::Pause`
/// default acts on. Fullscreen Direct3D (games / fullscreen video) is
/// the separate [`RealFullscreenProbe`] signal and no longer trips the
/// presentation rule.
#[cfg(target_os = "windows")]
pub struct RealPresentationProbe;

#[cfg(target_os = "windows")]
impl PresentationProbe for RealPresentationProbe {
    fn is_presenting(&self) -> bool {
        copythat_platform::presence::is_in_presentation_mode()
    }
}

/// Windows fullscreen probe — strict subset of presentation mode;
/// only fires on `QUNS_RUNNING_D3D_FULL_SCREEN`.
#[cfg(target_os = "windows")]
pub struct RealFullscreenProbe;

#[cfg(target_os = "windows")]
impl FullscreenProbe for RealFullscreenProbe {
    fn is_fullscreen(&self) -> bool {
        copythat_platform::presence::is_in_fullscreen_mode()
    }
}

/// Non-Windows targets (Linux, macOS, and everything else) defer the
/// presentation + fullscreen signals to the stub for now.
///
/// The earlier Linux GNOME `org.gnome.SessionManager.IsInhibited` DBus
/// probe was removed: it called the **blocking** zbus API from inside
/// the tokio poller task ([`crate::bus`]'s `sample`), which panics with
/// "cannot start a runtime from within a runtime" and took the whole
/// poller down — disabling *all* power throttling on Linux — and it
/// reported `true` on *any* idle-inhibit (e.g. a playing video), which
/// is not "presenting". A non-blocking probe that distinguishes
/// presentation from fullscreen, validated on a real Linux/GNOME
/// session, is a Phase 31c follow-up; the macOS `IOPMAssertion` reads
/// land the same way. Until then battery + thermal still drive
/// throttling on these targets (the poller no longer panics).
#[cfg(not(target_os = "windows"))]
pub type RealPresentationProbe = StubPresentationProbe;

#[cfg(not(target_os = "windows"))]
pub type RealFullscreenProbe = StubFullscreenProbe;

/// Thermal probe stub — always reports "not throttling, unknown
/// kind". Phase 31 ships the x86 `raw-cpuid` hook behind the
/// `RealThermalProbe` type below; hosts without CPUID-leaf-6 (and
/// non-x86 architectures) fall through to this stub.
pub struct StubThermalProbe;

impl ThermalProbe for StubThermalProbe {
    fn is_throttling(&self) -> (bool, ThermalKind) {
        (false, ThermalKind::Unknown)
    }
}

/// Network probe stub — returns `copythat_shape::current_network_class()`
/// (a Phase 21 stub that always reports `Unmetered`). Used on targets
/// without a real per-OS metering probe.
pub struct StubNetworkProbe;

impl NetworkProbe for StubNetworkProbe {
    fn class(&self) -> NetworkClass {
        copythat_shape::current_network_class()
    }
}

/// Windows network probe — real metering / cellular detection via
/// [`copythat_platform::network`] (WinRT `NetworkInformation`). Maps the
/// platform cost class onto the engine's [`NetworkClass`] so the
/// "on metered" / "on cellular" power rules fire on real connections.
#[cfg(target_os = "windows")]
pub struct RealNetworkProbe;

#[cfg(target_os = "windows")]
impl NetworkProbe for RealNetworkProbe {
    fn class(&self) -> NetworkClass {
        use copythat_platform::network::NetworkCostClass;
        match copythat_platform::network::current_cost_class() {
            NetworkCostClass::Unmetered => NetworkClass::Unmetered,
            NetworkCostClass::Metered => NetworkClass::Metered,
            NetworkCostClass::Cellular => NetworkClass::Cellular,
        }
    }
}

/// Non-Windows network probe — the per-OS metering bridges
/// (`NWPathMonitor` on macOS, NetworkManager DBus on Linux) are
/// deferred, so reuse the stub's `Unmetered` answer. Defined as a unit
/// struct (not a type alias) so `RealNetworkProbe` is usable as a value
/// constructor in [`ProbeSet::production`] uniformly across targets
/// (mirrors [`RealThermalProbe`]).
#[cfg(not(target_os = "windows"))]
pub struct RealNetworkProbe;

#[cfg(not(target_os = "windows"))]
impl NetworkProbe for RealNetworkProbe {
    fn class(&self) -> NetworkClass {
        StubNetworkProbe.class()
    }
}

// ---------------------------------------------------------------------
// x86 thermal probe via raw-cpuid (optional, cfg-gated)
// ---------------------------------------------------------------------

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod thermal_x86 {
    use super::{ThermalKind, ThermalProbe};

    /// Thermal-throttling probe driven by x86 CPUID leaf 6 (Thermal
    /// and Power Management).
    ///
    /// `raw-cpuid` surfaces the achieved-frequency + base-frequency
    /// fields of leaf 0x16 (on Skylake+). When achieved drops below
    /// base, the CPU is thermal- or power-throttling. Not perfect —
    /// some BIOSes strobe the values — but good enough for the
    /// "capping during a Zoom call because the fan is whining"
    /// signal the UI wants to render.
    pub struct RealThermalProbe;

    impl ThermalProbe for RealThermalProbe {
        fn is_throttling(&self) -> (bool, ThermalKind) {
            use raw_cpuid::CpuId;
            let cpuid = CpuId::new();
            let Some(proc_freq) = cpuid.get_processor_frequency_info() else {
                return (false, ThermalKind::Unknown);
            };
            let base = proc_freq.processor_base_frequency();
            let max = proc_freq.processor_max_frequency();
            if base == 0 || max == 0 {
                return (false, ThermalKind::Unknown);
            }
            // A CPU running at base freq is healthy; reported
            // achievable < base means a sustained throttle is active.
            // The CPUID fields are reported in MHz; we compare base
            // vs max and tolerate a 5 % slack to avoid flapping on
            // microbump reads.
            let slack = base.saturating_sub(base / 20);
            let throttling = max < slack;
            (throttling, ThermalKind::X86Cpuid)
        }
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use thermal_x86::RealThermalProbe;

/// Non-x86 fallback: no CPUID leaf 6, so reuse the stub's
/// "not throttling / unknown" answer. Defined as a unit struct (not a
/// `type` alias) so `RealThermalProbe` is usable as a value constructor
/// — e.g. `Arc::new(RealThermalProbe)` in [`ProbeSet::production`] —
/// uniformly across architectures.
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
pub struct RealThermalProbe;

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
impl ThermalProbe for RealThermalProbe {
    fn is_throttling(&self) -> (bool, ThermalKind) {
        StubThermalProbe.is_throttling()
    }
}

// ---------------------------------------------------------------------
// Bundled probe set (what the poller owns)
// ---------------------------------------------------------------------

/// One bundle of every probe the poller consults. Held by the Tauri
/// `AppState` for real runs; the smoke test uses [`NullProbes`] or
/// [`SyntheticProbes`] instead.
pub struct ProbeSet {
    pub battery: Arc<dyn BatteryProbe>,
    pub presentation: Arc<dyn PresentationProbe>,
    pub fullscreen: Arc<dyn FullscreenProbe>,
    pub thermal: Arc<dyn ThermalProbe>,
    pub network: Arc<dyn NetworkProbe>,
}

impl ProbeSet {
    /// Default production probe set: real `battery` + `raw-cpuid`
    /// thermal (on x86) + the real **Windows** presentation/fullscreen
    /// probes (Phase 31b — `SHQueryUserNotificationState` FFI). Linux,
    /// macOS, and other targets defer presentation/fullscreen to the
    /// stub via the `RealPresentationProbe` / `RealFullscreenProbe`
    /// type aliases (see their definition for why the Linux DBus probe
    /// was pulled — a Phase 31c follow-up), so this is safe everywhere.
    /// Network metering is real on **Windows** (WinRT `NetworkInformation`
    /// via `copythat_platform::network`); macOS / Linux / other targets
    /// defer to the stub (`NWPathMonitor` / NetworkManager DBus bridges
    /// are a follow-up).
    pub fn production() -> Self {
        Self {
            battery: Arc::new(RealBatteryProbe),
            presentation: Arc::new(RealPresentationProbe),
            fullscreen: Arc::new(RealFullscreenProbe),
            thermal: Arc::new(RealThermalProbe),
            network: Arc::new(RealNetworkProbe),
        }
    }
}

impl Default for ProbeSet {
    fn default() -> Self {
        Self::production()
    }
}

/// "Nothing connected" probe set — useful for CI runners and the
/// smoke test's baseline. Every probe returns its stub value.
pub struct NullProbes;

impl NullProbes {
    pub fn as_set() -> ProbeSet {
        ProbeSet {
            battery: Arc::new(NullBattery),
            presentation: Arc::new(StubPresentationProbe),
            fullscreen: Arc::new(StubFullscreenProbe),
            thermal: Arc::new(StubThermalProbe),
            network: Arc::new(StubNetworkProbe),
        }
    }
}

struct NullBattery;

impl BatteryProbe for NullBattery {
    fn snapshot(&self) -> Option<BatterySnapshot> {
        None
    }
}

// ---------------------------------------------------------------------
// Synthetic probes — the test-inject surface
// ---------------------------------------------------------------------

/// Thread-safe bundle of mutable "current readings" that the smoke
/// test mutates directly via the `set_*` helpers. Implements every
/// probe trait via `Arc<Mutex<Inner>>` so a single `Arc<SyntheticProbes>`
/// can be cloned into a `ProbeSet` for the poller *and* held by the
/// smoke test for mutation.
pub struct SyntheticProbes {
    inner: Mutex<Inner>,
}

#[derive(Clone, Copy)]
struct Inner {
    battery: Option<BatterySnapshot>,
    presenting: bool,
    fullscreen: bool,
    thermal: (bool, ThermalKind),
    network: NetworkClass,
}

impl SyntheticProbes {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            inner: Mutex::new(Inner {
                battery: None,
                presenting: false,
                fullscreen: false,
                thermal: (false, ThermalKind::Unknown),
                network: NetworkClass::Unmetered,
            }),
        })
    }

    pub fn set_battery(&self, snap: Option<BatterySnapshot>) {
        if let Ok(mut g) = self.inner.lock() {
            g.battery = snap;
        }
    }

    pub fn set_presenting(&self, presenting: bool) {
        if let Ok(mut g) = self.inner.lock() {
            g.presenting = presenting;
        }
    }

    pub fn set_fullscreen(&self, fullscreen: bool) {
        if let Ok(mut g) = self.inner.lock() {
            g.fullscreen = fullscreen;
        }
    }

    pub fn set_thermal(&self, throttling: bool, kind: ThermalKind) {
        if let Ok(mut g) = self.inner.lock() {
            g.thermal = (throttling, kind);
        }
    }

    pub fn set_network(&self, class: NetworkClass) {
        if let Ok(mut g) = self.inner.lock() {
            g.network = class;
        }
    }

    /// Wrap `self` in a [`ProbeSet`] that the poller can consume.
    /// Every trait-impl in the returned set is a `Clone` of the
    /// shared `Arc<Self>`, so mutations via `set_*` are visible to
    /// the poller on the very next tick.
    pub fn as_set(self: &Arc<Self>) -> ProbeSet {
        ProbeSet {
            battery: self.clone(),
            presentation: self.clone(),
            fullscreen: self.clone(),
            thermal: self.clone(),
            network: self.clone(),
        }
    }
}

impl BatteryProbe for SyntheticProbes {
    fn snapshot(&self) -> Option<BatterySnapshot> {
        self.inner.lock().ok().and_then(|g| g.battery)
    }
}

impl PresentationProbe for SyntheticProbes {
    fn is_presenting(&self) -> bool {
        self.inner
            .lock()
            .ok()
            .map(|g| g.presenting)
            .unwrap_or(false)
    }
}

impl FullscreenProbe for SyntheticProbes {
    fn is_fullscreen(&self) -> bool {
        self.inner
            .lock()
            .ok()
            .map(|g| g.fullscreen)
            .unwrap_or(false)
    }
}

impl ThermalProbe for SyntheticProbes {
    fn is_throttling(&self) -> (bool, ThermalKind) {
        self.inner
            .lock()
            .ok()
            .map(|g| g.thermal)
            .unwrap_or((false, ThermalKind::Unknown))
    }
}

impl NetworkProbe for SyntheticProbes {
    fn class(&self) -> NetworkClass {
        self.inner
            .lock()
            .ok()
            .map(|g| g.network)
            .unwrap_or(NetworkClass::Unmetered)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn synthetic_probes_round_trip() {
        let probes = SyntheticProbes::new();
        assert_eq!(probes.snapshot(), None);
        assert!(!probes.is_presenting());
        assert!(!probes.is_fullscreen());

        probes.set_battery(Some(BatterySnapshot::on_battery(42.0)));
        probes.set_presenting(true);

        let snap = probes.snapshot().expect("battery set");
        assert!(snap.on_battery);
        assert!((snap.percent - 42.0).abs() < f32::EPSILON);
        assert!(probes.is_presenting());
        assert!(!probes.is_fullscreen());
    }

    #[test]
    fn as_set_gives_live_view() {
        let probes = SyntheticProbes::new();
        let set = probes.as_set();
        probes.set_presenting(true);
        assert!(set.presentation.is_presenting());
    }

    #[test]
    fn null_battery_reports_none() {
        let set = NullProbes::as_set();
        assert!(set.battery.snapshot().is_none());
    }
}
