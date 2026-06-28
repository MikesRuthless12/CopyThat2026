//! Phase 31b — active-connection metering class probe.
//!
//! The power policy's "on metered" / "on cellular" rules need the
//! current internet connection's cost class. Windows answers via WinRT
//! [`NetworkInformation`] (connection cost + WWAN/cellular flag). macOS
//! / Linux / other targets return `Unmetered` for now — the
//! `NWPathMonitor` (macOS) and NetworkManager-DBus (Linux) bridges are a
//! follow-up; until they land the metered/cellular rules never fire on
//! those hosts.
//!
//! [`NetworkInformation`]: https://learn.microsoft.com/uwp/api/windows.networking.connectivity.networkinformation

/// Coarse metering class for the active internet connection. Mirrors the
/// variants of `copythat_shape::NetworkClass` without taking that crate
/// as a dependency here (the power crate maps between the two).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkCostClass {
    /// Flat-rate / unrestricted (home Wi-Fi, wired LAN).
    Unmetered,
    /// Metered: a data cap (`Fixed`) or pay-per-byte (`Variable`).
    Metered,
    /// Cellular / WWAN — treated as the most restrictive class.
    Cellular,
}

/// Probe the active internet connection's metering class.
///
/// **Best-effort.** Returns [`NetworkCostClass::Unmetered`] on any error
/// or when no connection profile is available (offline), matching the
/// presence probes' fail-safe convention. Non-Windows targets always
/// return `Unmetered` until the per-OS bridges land.
pub fn current_cost_class() -> NetworkCostClass {
    #[cfg(target_os = "windows")]
    {
        windows_impl::query().unwrap_or(NetworkCostClass::Unmetered)
    }
    #[cfg(not(target_os = "windows"))]
    {
        NetworkCostClass::Unmetered
    }
}

#[cfg(target_os = "windows")]
mod windows_impl {
    use std::sync::Once;

    use windows::Networking::Connectivity::{NetworkCostType, NetworkInformation};
    use windows::Win32::System::Com::CoIncrementMTAUsage;

    use super::NetworkCostClass;

    static MTA: Once = Once::new();

    /// WinRT factory activation requires an initialized apartment. The
    /// power poller calls us from a tokio worker thread that isn't COM-
    /// initialized, so register a process-wide MTA once. It persists for
    /// the process lifetime, letting WinRT calls succeed from any thread
    /// without per-call `CoInitialize`/uninit churn.
    fn ensure_apartment() {
        MTA.call_once(|| {
            // SAFETY: documented COM API; takes no input and returns a
            // usage cookie. We intentionally drop (leak) the cookie so
            // the MTA registration lives for the whole process.
            let _ = unsafe { CoIncrementMTAUsage() };
        });
    }

    pub fn query() -> Option<NetworkCostClass> {
        ensure_apartment();
        // The profile backing the default internet route; errors when
        // offline (no route) -> None -> Unmetered upstream.
        let profile = NetworkInformation::GetInternetConnectionProfile().ok()?;
        // Cellular wins: it's the most restrictive, and the user's "on
        // cellular" rule should take precedence over plain "metered".
        if profile.IsWwanConnectionProfile().unwrap_or(false) {
            return Some(NetworkCostClass::Cellular);
        }
        let cost = profile.GetConnectionCost().ok()?;
        // Anything other than Unrestricted is "metered" for policy
        // purposes (Fixed = data cap, Variable = pay-per-byte).
        let metered = !matches!(cost.NetworkCostType().ok()?, NetworkCostType::Unrestricted);
        Some(if metered {
            NetworkCostClass::Metered
        } else {
            NetworkCostClass::Unmetered
        })
    }
}
