//! OS-level network-class + power-state probes used by the Network
//! Settings tab's auto-throttle rules.
//!
//! Phase 21 ships the Rust surface and a stable wire enum; the
//! per-OS implementations are stubbed to "always Unmetered /
//! always PluggedIn" because the underlying APIs require platform-
//! specific FFI we don't want to thread through this crate yet:
//!
//! - **Windows**: `INetworkCostManager::GetCost` (combase / WinRT).
//! - **macOS**: `NWPathMonitor` from CoreFoundation FFI.
//! - **Linux**: NetworkManager DBus (`org.freedesktop.NetworkManager`)
//!   for connectivity class; `UPower` for battery state.
//!
//! These land alongside the Phase 31 power-aware copy work (when
//! the engine learns to pause on battery / low-throttle on cellular
//! independent of any user schedule). For now the auto-throttle
//! Settings rows still render and persist; their effect is
//! determined by the stub returns until those phases land.

/// Network metering classification — drives the "On metered Wi-Fi:
/// cap to X" Settings rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NetworkClass {
    #[default]
    Unmetered,
    Metered,
    Cellular,
}

impl NetworkClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Unmetered => "unmetered",
            Self::Metered => "metered",
            Self::Cellular => "cellular",
        }
    }
}

/// Battery / charging state — drives the "On battery: cap to X"
/// Settings rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PowerState {
    #[default]
    PluggedIn,
    OnBattery,
}

impl PowerState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PluggedIn => "plugged-in",
            Self::OnBattery => "on-battery",
        }
    }
}

/// Probe the OS for the active connection's metering class.
///
/// Stubbed in Phase 21 — see module docs. A non-stub implementation
/// must remain non-blocking (called from the same tokio interval
/// the schedule polls); on Windows that means caching the
/// `INetworkCostManager` COM pointer and asking it on demand.
pub fn current_network_class() -> NetworkClass {
    NetworkClass::default()
}

/// Probe the OS for the running power state.
///
/// Same stub-in-Phase-21 caveat as [`current_network_class`].
pub fn current_power_state() -> PowerState {
    PowerState::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_documented_stubs() {
        assert_eq!(current_network_class(), NetworkClass::Unmetered);
        assert_eq!(current_power_state(), PowerState::PluggedIn);
    }

    #[test]
    fn wire_strings_round_trip() {
        // The wire strings appear in the SettingsDto; lock them down
        // so a future enum-name change can't silently break saved
        // settings.
        assert_eq!(NetworkClass::Unmetered.as_str(), "unmetered");
        assert_eq!(NetworkClass::Metered.as_str(), "metered");
        assert_eq!(NetworkClass::Cellular.as_str(), "cellular");
        assert_eq!(PowerState::PluggedIn.as_str(), "plugged-in");
        assert_eq!(PowerState::OnBattery.as_str(), "on-battery");
    }
}
