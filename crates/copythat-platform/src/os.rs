//! Phase 42 — OS-version probe.
//!
//! CopyThat2026 targets Windows 11+ (build 22000+) as of Phase 42. A
//! handful of features ride 22H2 / 24H2 minimums and need runtime
//! detection: `COPY_FILE_ENABLE_SPARSE_COPY` (Win11 22H2+ flag in
//! `CopyFile2`) and the Win11 24H2 native block-cloning auto-engagement
//! inside `CopyFileExW` (lets us skip the explicit reflink probe on
//! same-volume ReFS / Dev Drive).
//!
//! On non-Windows targets the helpers are no-ops returning `false` —
//! callers should treat "false" as "feature not available, use the
//! older code path".
//!
//! Implementation: dynamically loads `RtlGetVersion` from `ntdll.dll`.
//! Avoids the deprecated `GetVersionExW` shim, which lies unless the
//! caller carries an explicit application manifest declaring Win11
//! support.

#[cfg(windows)]
mod imp {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use std::sync::OnceLock;

    use windows_sys::Win32::System::LibraryLoader::{GetModuleHandleW, GetProcAddress};
    use windows_sys::Win32::System::SystemInformation::OSVERSIONINFOW;

    /// Win11 21H2 (build 22000) is our **support floor** as of Phase 42.
    pub const WIN11_21H2_BUILD: u32 = 22000;
    /// Win11 22H2 (build 22621) is the gate for `COPY_FILE_ENABLE_SPARSE_COPY`.
    pub const WIN11_22H2_BUILD: u32 = 22621;
    /// Win11 24H2 (build 26100) is the gate for native block-cloning
    /// auto-engagement inside `CopyFileExW`.
    pub const WIN11_24H2_BUILD: u32 = 26100;

    type RtlGetVersionFn = unsafe extern "system" fn(*mut OSVERSIONINFOW) -> i32;

    fn build_number_inner() -> Option<u32> {
        // ntdll.dll is mapped into every Windows process, so
        // GetModuleHandleW is enough — we don't need LoadLibraryW
        // and the reference counting that comes with it.
        let module_name: Vec<u16> = OsStr::new("ntdll.dll")
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        // SAFETY: module_name is a NUL-terminated UTF-16 string;
        // GetModuleHandleW is thread-safe and returns a non-owning handle.
        let module = unsafe { GetModuleHandleW(module_name.as_ptr()) };
        if module.is_null() {
            return None;
        }
        let proc_name = c"RtlGetVersion";
        // SAFETY: module is non-null; proc_name is a NUL-terminated ASCII
        // string. GetProcAddress returns Option<extern fn> in
        // windows-sys 0.59.
        let f = unsafe { GetProcAddress(module, proc_name.as_ptr().cast()) }?;
        // SAFETY: RtlGetVersion's documented signature matches RtlGetVersionFn.
        let f: RtlGetVersionFn = unsafe { std::mem::transmute(f) };

        let mut info: OSVERSIONINFOW = unsafe { std::mem::zeroed() };
        info.dwOSVersionInfoSize = std::mem::size_of::<OSVERSIONINFOW>() as u32;
        // SAFETY: info is a properly-sized OSVERSIONINFOW.
        let status = unsafe { f(&mut info) };
        // STATUS_SUCCESS = 0.
        if status != 0 {
            return None;
        }
        Some(info.dwBuildNumber)
    }

    /// Cached Windows build number. `None` if `RtlGetVersion` isn't
    /// available (extremely unlikely on Win11+) — callers should treat
    /// `None` as "old OS, use the conservative path".
    pub fn build_number() -> Option<u32> {
        static CACHED: OnceLock<Option<u32>> = OnceLock::new();
        *CACHED.get_or_init(build_number_inner)
    }
}

#[cfg(windows)]
pub use imp::{WIN11_21H2_BUILD, WIN11_22H2_BUILD, WIN11_24H2_BUILD, build_number};

/// Returns `true` iff we're running on Windows 11 21H2 or later
/// (build 22000+). The Phase 42 support floor; sub-22000 builds are
/// not a target platform.
pub fn is_win11_or_later() -> bool {
    #[cfg(windows)]
    {
        build_number().is_some_and(|b| b >= WIN11_21H2_BUILD)
    }
    #[cfg(not(windows))]
    {
        false
    }
}

/// Returns `true` iff we're running on Windows 11 22H2 or later
/// (build 22621+). Gate for `COPY_FILE_ENABLE_SPARSE_COPY`.
pub fn is_win11_22h2_plus() -> bool {
    #[cfg(windows)]
    {
        build_number().is_some_and(|b| b >= WIN11_22H2_BUILD)
    }
    #[cfg(not(windows))]
    {
        false
    }
}

/// Returns `true` iff we're running on Windows 11 24H2 or later
/// (build 26100+). Gate for skipping the explicit reflink probe on
/// same-volume ReFS / Dev Drive — `CopyFileExW` auto-engages
/// `FSCTL_DUPLICATE_EXTENTS_TO_FILE` itself on 24H2+ (KB5034848+).
pub fn is_win11_24h2_plus() -> bool {
    #[cfg(windows)]
    {
        build_number().is_some_and(|b| b >= WIN11_24H2_BUILD)
    }
    #[cfg(not(windows))]
    {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(windows)]
    #[test]
    fn build_number_is_some_on_windows() {
        // On any modern Windows host the probe should succeed — even if
        // the host is older than our support floor we still get a number
        // back, just one that fails the threshold checks.
        let n = build_number();
        // We don't assert >= 22000 because the test runner could be on
        // an older box; we only assert the probe didn't panic / null out.
        assert!(n.is_some(), "RtlGetVersion probe returned None on Windows");
    }

    #[cfg(not(windows))]
    #[test]
    fn non_windows_helpers_return_false() {
        assert!(!is_win11_or_later());
        assert!(!is_win11_22h2_plus());
        assert!(!is_win11_24h2_plus());
    }
}
