//! Phase 31b — user-presence probe.
//!
//! Wraps Windows's `SHQueryUserNotificationState` in a safe Rust
//! API the (unsafe-forbidden) `copythat-power` crate can call. The
//! API answers two questions the power policy needs:
//!
//! - **`is_in_presentation_mode()`** — the user has explicitly
//!   asked the OS to suppress notifications (slideshow / focus
//!   assist / Do-Not-Disturb). The power policy pauses long copies
//!   here so the engine's I/O doesn't interrupt a presentation.
//! - **`is_in_fullscreen_mode()`** — Direct3D fullscreen specifically
//!   (games, fullscreen video). A *distinct* signal from presentation
//!   (a different `QUNS_*` state), so a fullscreen game/video drives
//!   the fullscreen rule, not the presentation rule.
//!
//! Linux + macOS + every other target return `false` here; the
//! Linux DBus screensaver query and the macOS `IOPMAssertion`
//! reads live in `copythat-power` directly because they don't
//! need unsafe FFI.

#![allow(unsafe_code)]

/// Returns `true` only when the OS reports `QUNS_PRESENTATION_MODE`
/// (slideshow / focus-assist / Do-Not-Disturb). Fullscreen Direct3D
/// is reported separately by [`is_in_fullscreen_mode`], so a
/// fullscreen game or video does **not** read as "presenting".
/// `false` on any error (the Phase 31 policy machine treats
/// "unknown" the same as "not presenting" — fail-safe).
///
/// **Best-effort snapshot.** Presence checks reflect the OS state at
/// the moment of the syscall and the platform state can change
/// between probe and decision (the user may exit fullscreen, dismiss
/// a slideshow, or accept-notifications between this call returning
/// and the caller acting on the result). Callers using this for
/// routing should treat it as a hint, not a guarantee — schedule a
/// re-probe before any policy action that could surprise the user.
pub fn is_in_presentation_mode() -> bool {
    #[cfg(target_os = "windows")]
    {
        // Match only the explicit presentation state. Fullscreen
        // Direct3D is a separate `QUNS_*` value handled by
        // `is_in_fullscreen_mode`, so a fullscreen game/video doesn't
        // also trip the presentation rule.
        windows_impl::query() == Some(QunsState::Presentation)
    }
    #[cfg(not(target_os = "windows"))]
    {
        false
    }
}

/// Returns `true` when the foreground process is in fullscreen
/// Direct3D mode specifically (`QUNS_RUNNING_D3D_FULL_SCREEN`).
/// A distinct signal from [`is_in_presentation_mode`] (a different
/// `QUNS_*` state), so the policy machine can pause on presentation
/// while still copying through a fullscreen game/video, or vice-versa.
///
/// Same best-effort caveat as [`is_in_presentation_mode`] — the
/// foreground app can exit fullscreen between this probe and the
/// caller's policy decision.
pub fn is_in_fullscreen_mode() -> bool {
    #[cfg(target_os = "windows")]
    {
        windows_impl::query() == Some(QunsState::RunningD3dFullScreen)
    }
    #[cfg(not(target_os = "windows"))]
    {
        false
    }
}

/// `QUERY_USER_NOTIFICATION_STATE` enum mirror — keep public so
/// integration tests can assert the per-state branches without
/// having to call the raw Win32 API.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum QunsState {
    /// `QUNS_NOT_PRESENT` — workstation locked / not signed in.
    NotPresent,
    /// `QUNS_BUSY` — the desktop is busy (modal dialog, app
    /// installing). Phase 31 doesn't act on this — copies are
    /// the kind of "background work" busy is meant to allow.
    Busy,
    /// `QUNS_RUNNING_D3D_FULL_SCREEN` — fullscreen Direct3D /
    /// fullscreen game / fullscreen video.
    RunningD3dFullScreen,
    /// `QUNS_PRESENTATION_MODE` — slideshow / Do-Not-Disturb /
    /// focus-assist active.
    Presentation,
    /// `QUNS_ACCEPTS_NOTIFICATIONS` — normal desktop usage.
    AcceptsNotifications,
    /// `QUNS_QUIET_TIME` — automatic quiet hours.
    QuietTime,
    /// `QUNS_APP` — app-specific quiet state.
    App,
}

#[cfg(target_os = "windows")]
mod windows_impl {
    use super::QunsState;
    use windows_sys::Win32::UI::Shell::{
        QUNS_ACCEPTS_NOTIFICATIONS, QUNS_APP, QUNS_BUSY, QUNS_NOT_PRESENT, QUNS_PRESENTATION_MODE,
        QUNS_QUIET_TIME, QUNS_RUNNING_D3D_FULL_SCREEN, SHQueryUserNotificationState,
    };

    /// Wrap `SHQueryUserNotificationState` in a safe shape. Returns
    /// `None` on any HRESULT failure or unrecognised state code.
    pub fn query() -> Option<QunsState> {
        let mut state: i32 = 0;
        // SAFETY: `SHQueryUserNotificationState` is a documented
        // Shell32 API (winapi-rs / windows-sys / Microsoft's own
        // C SDK all expose it the same way). It writes one i32
        // through `state_ptr` and returns an HRESULT. We pass a
        // valid stack pointer to a stack-local i32, check the
        // HRESULT before reading the out value, and never alias
        // the pointer.
        let hr = unsafe { SHQueryUserNotificationState(&mut state) };
        if hr < 0 {
            return None;
        }
        match state {
            v if v == QUNS_NOT_PRESENT => Some(QunsState::NotPresent),
            v if v == QUNS_BUSY => Some(QunsState::Busy),
            v if v == QUNS_RUNNING_D3D_FULL_SCREEN => Some(QunsState::RunningD3dFullScreen),
            v if v == QUNS_PRESENTATION_MODE => Some(QunsState::Presentation),
            v if v == QUNS_ACCEPTS_NOTIFICATIONS => Some(QunsState::AcceptsNotifications),
            v if v == QUNS_QUIET_TIME => Some(QunsState::QuietTime),
            v if v == QUNS_APP => Some(QunsState::App),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn presence_calls_do_not_panic() {
        // Smoke test — the API is opaque on this host (CI runners
        // have no GPU / no logged-in user / no Shell32). Calling
        // it must never panic; the return value is whatever the
        // OS reports.
        let _ = is_in_presentation_mode();
        let _ = is_in_fullscreen_mode();
    }
}
