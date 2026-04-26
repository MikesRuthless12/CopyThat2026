//! Phase 37 follow-up #2 — OS wake-lock primitives.
//!
//! [`WakeLock`] is a cross-platform RAII handle that, while alive,
//! asks the OS to inhibit screensaver / display sleep. The mobile
//! companion's "Keep desktop awake" toggle (`set_keep_awake`)
//! holds one of these for the duration of an active phone session.
//!
//! Per-platform backing:
//!
//! - **Windows** — `SetThreadExecutionState(ES_CONTINUOUS |
//!   ES_DISPLAY_REQUIRED | ES_SYSTEM_REQUIRED)` on acquire,
//!   `SetThreadExecutionState(ES_CONTINUOUS)` on release. The flags
//!   are session-wide, not thread-bound, despite the name; modern
//!   Windows applies them to the calling process.
//! - **macOS** — `IOPMAssertionCreateWithName` with
//!   `kIOPMAssertionTypePreventUserIdleDisplaySleep`, released via
//!   `IOPMAssertionRelease`.
//! - **Linux** — `org.freedesktop.ScreenSaver.Inhibit` over dbus
//!   (zbus 5). The cookie returned by Inhibit is passed back to
//!   UnInhibit on release.

use std::fmt;

/// RAII guard. Releasing the lock is automatic on drop; calling
/// [`Self::release`] explicitly is fine and idempotent.
pub struct WakeLock {
    #[allow(dead_code)] // Drop impl on `Inner` does the release work.
    inner: Inner,
}

/// Cross-platform error surface.
#[derive(Debug)]
pub enum WakeLockError {
    /// The platform syscall returned a non-success code.
    Platform(String),
    /// The platform's wake-lock primitive isn't available on this
    /// build (e.g. dbus is offline on Linux). Callers can ignore
    /// this — the wake-lock contract is best-effort.
    Unavailable(String),
}

impl fmt::Display for WakeLockError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Platform(s) => write!(f, "wake-lock platform: {s}"),
            Self::Unavailable(s) => write!(f, "wake-lock unavailable: {s}"),
        }
    }
}

impl std::error::Error for WakeLockError {}

/// Acquire a wake-lock that prevents the system from going to sleep
/// or starting the screensaver. Drop the returned [`WakeLock`] to
/// release it.
pub fn acquire_keep_awake() -> Result<WakeLock, WakeLockError> {
    Inner::acquire().map(|inner| WakeLock { inner })
}

impl WakeLock {
    /// Release the lock immediately. Idempotent — calling more than
    /// once is a no-op. Drop also releases.
    pub fn release(self) {
        drop(self);
    }
}

impl fmt::Debug for WakeLock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WakeLock").finish()
    }
}

// ---------------------------------------------------------------------
// Per-platform backing
// ---------------------------------------------------------------------

#[cfg(target_os = "windows")]
mod backend_windows {
    use super::WakeLockError;
    use windows_sys::Win32::System::Power::{
        ES_CONTINUOUS, ES_DISPLAY_REQUIRED, ES_SYSTEM_REQUIRED, SetThreadExecutionState,
    };

    pub(super) struct Inner;

    impl Inner {
        pub fn acquire() -> Result<Self, WakeLockError> {
            // SAFETY: SetThreadExecutionState is documented as
            // safe to call from any thread. The return value is the
            // previous flag set; 0 means the call failed and we
            // surface as a platform error.
            let prev = unsafe {
                SetThreadExecutionState(ES_CONTINUOUS | ES_DISPLAY_REQUIRED | ES_SYSTEM_REQUIRED)
            };
            if prev == 0 {
                return Err(WakeLockError::Platform(
                    "SetThreadExecutionState returned 0 (display state may be unavailable)".into(),
                ));
            }
            Ok(Self)
        }
    }

    impl Drop for Inner {
        fn drop(&mut self) {
            // Reset to ES_CONTINUOUS only — releases display +
            // system requirements while keeping the thread-state
            // baseline.
            unsafe {
                SetThreadExecutionState(ES_CONTINUOUS);
            }
        }
    }
}

#[cfg(target_os = "macos")]
mod backend_macos {
    use super::WakeLockError;
    use core_foundation_sys::base::{CFAllocatorRef, CFRelease, CFTypeRef, kCFAllocatorDefault};
    use core_foundation_sys::string::{
        CFStringCreateWithCString, CFStringRef, kCFStringEncodingUTF8,
    };
    use std::ffi::CString;
    use std::os::raw::{c_int, c_void};

    type IOPMAssertionID = u32;
    type IOPMAssertionLevel = u32;

    const K_IOPM_ASSERTION_LEVEL_ON: IOPMAssertionLevel = 255;
    const K_IO_RETURN_SUCCESS: c_int = 0;

    #[link(name = "IOKit", kind = "framework")]
    unsafe extern "C" {
        fn IOPMAssertionCreateWithName(
            assertion_type: CFStringRef,
            assertion_level: IOPMAssertionLevel,
            assertion_name: CFStringRef,
            assertion_id: *mut IOPMAssertionID,
        ) -> c_int;

        fn IOPMAssertionRelease(assertion_id: IOPMAssertionID) -> c_int;
    }

    pub(super) struct Inner {
        id: IOPMAssertionID,
    }

    impl Inner {
        pub fn acquire() -> Result<Self, WakeLockError> {
            let assertion_type = make_cfstring("PreventUserIdleDisplaySleep")
                .ok_or_else(|| WakeLockError::Platform("CFString assertion-type".into()))?;
            let assertion_name = make_cfstring("Copy That mobile companion")
                .ok_or_else(|| WakeLockError::Platform("CFString name".into()))?;
            let mut id: IOPMAssertionID = 0;
            // SAFETY: IOPMAssertionCreateWithName is documented to
            // accept any thread. The CFStringRefs are owned by us;
            // we release them after the call.
            let rc = unsafe {
                IOPMAssertionCreateWithName(
                    assertion_type,
                    K_IOPM_ASSERTION_LEVEL_ON,
                    assertion_name,
                    &mut id,
                )
            };
            unsafe {
                CFRelease(assertion_type as CFTypeRef);
                CFRelease(assertion_name as CFTypeRef);
            }
            if rc != K_IO_RETURN_SUCCESS {
                return Err(WakeLockError::Platform(format!(
                    "IOPMAssertionCreateWithName returned {rc}"
                )));
            }
            Ok(Self { id })
        }
    }

    impl Drop for Inner {
        fn drop(&mut self) {
            unsafe {
                IOPMAssertionRelease(self.id);
            }
        }
    }

    fn make_cfstring(s: &str) -> Option<CFStringRef> {
        let cstr = CString::new(s).ok()?;
        let alloc: CFAllocatorRef = unsafe { kCFAllocatorDefault };
        let ptr = unsafe { CFStringCreateWithCString(alloc, cstr.as_ptr(), kCFStringEncodingUTF8) };
        if ptr.is_null() { None } else { Some(ptr) }
    }

    #[allow(dead_code)]
    fn _unused(p: *mut c_void) {
        let _ = p;
    }
}

#[cfg(target_os = "linux")]
mod backend_linux {
    use super::WakeLockError;
    use zbus::{Connection, blocking};

    pub(super) struct Inner {
        conn: blocking::Connection,
        cookie: u32,
    }

    #[zbus::proxy(
        interface = "org.freedesktop.ScreenSaver",
        default_service = "org.freedesktop.ScreenSaver",
        default_path = "/org/freedesktop/ScreenSaver"
    )]
    trait ScreenSaver {
        fn inhibit(&self, application_name: &str, reason_for_inhibit: &str) -> zbus::Result<u32>;
        fn un_inhibit(&self, cookie: u32) -> zbus::Result<()>;
    }

    impl Inner {
        pub fn acquire() -> Result<Self, WakeLockError> {
            let conn = blocking::Connection::session()
                .map_err(|e| WakeLockError::Unavailable(format!("dbus session: {e}")))?;
            let proxy = ScreenSaverProxyBlocking::new(&conn)
                .map_err(|e| WakeLockError::Unavailable(format!("ScreenSaver proxy: {e}")))?;
            let cookie = proxy
                .inhibit("Copy That", "Mobile companion connected")
                .map_err(|e| WakeLockError::Platform(format!("Inhibit: {e}")))?;
            Ok(Self { conn, cookie })
        }
    }

    impl Drop for Inner {
        fn drop(&mut self) {
            if let Ok(proxy) = ScreenSaverProxyBlocking::new(&self.conn) {
                let _ = proxy.un_inhibit(self.cookie);
            }
        }
    }

    // Suppress the unused-import warning when zbus isn't pulled in
    // by another module on this target.
    #[allow(dead_code)]
    fn _unused(_: Connection) {}
}

#[cfg(target_os = "windows")]
use backend_windows::Inner;

#[cfg(target_os = "macos")]
use backend_macos::Inner;

#[cfg(target_os = "linux")]
use backend_linux::Inner;

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
mod backend_other {
    use super::WakeLockError;

    pub(super) struct Inner;

    impl Inner {
        pub fn acquire() -> Result<Self, WakeLockError> {
            Err(WakeLockError::Unavailable(
                "wake-lock not implemented for this platform".into(),
            ))
        }
    }
}

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
use backend_other::Inner;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acquire_and_release_does_not_panic() {
        // Best-effort assertion: on a CI host without dbus / IOKit
        // / Win32 the call may surface `Unavailable`. Either
        // outcome is acceptable; what we're testing is that the
        // surface is callable at all.
        match acquire_keep_awake() {
            Ok(lock) => lock.release(),
            Err(WakeLockError::Unavailable(_)) => {}
            Err(WakeLockError::Platform(e)) => {
                // Some test environments (sandboxed CI) reject the
                // call; treat as soft-success since the lock is
                // best-effort by contract.
                eprintln!("wake-lock platform error (allowed): {e}");
            }
        }
    }
}
