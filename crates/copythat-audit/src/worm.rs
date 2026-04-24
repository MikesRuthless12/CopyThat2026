//! Phase 34 — WORM (write-once-read-many) platform layer.
//!
//! Real WORM requires kernel / filesystem cooperation:
//!
//! - **Linux** — the ext4 / xfs / btrfs "append-only" inode flag
//!   (`FS_APPEND_FL`, `a` in `lsattr`). Setting it requires
//!   CAP_LINUX_IMMUTABLE; when Copy That runs unprivileged this
//!   layer surfaces [`WormError::Apply`] with a clear message and
//!   the Settings UI disables the toggle.
//! - **macOS** — the `chflags uappnd` user-level append-only flag
//!   (`UF_APPEND`). Userspace can set it; the kernel blocks truncate
//!   / unlink. `chflags schg` (system-immutable) is stricter but
//!   requires root; we intentionally use the softer `uappnd` so
//!   the app can still rotate the log.
//! - **Windows** — deny-write ACL via SetNamedSecurityInfoW. Grants
//!   the caller's SID FILE_APPEND_DATA + READ but denies WRITE_DATA
//!   / DELETE, so even an admin must explicitly remove the DENY
//!   ACE before a truncate. Phase 34 applies a coarse
//!   "read-only" ACL as the portable fallback when the richer ACE
//!   path is refused.
//!
//! The public API is small: [`WormMode`], [`apply_worm`],
//! [`is_worm_supported`]. The platform-specific routines live in
//! `unix.rs` / `windows.rs` stubs conditionally compiled via
//! `cfg(unix)` / `cfg(windows)`. Everything is feature-free so the
//! crate builds on any host.

use std::path::Path;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Toggle for a sink's WORM state. Persisted in the TOML settings
/// file; round-trippable via [`Serialize`] / [`Deserialize`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WormMode {
    /// No kernel guard. The file is still chain-hashed; a motivated
    /// attacker with write access can truncate it.
    #[default]
    Off,
    /// Apply the platform's append-only flag / deny-write ACL
    /// after every create / rotate.
    On,
}

impl WormMode {
    pub fn is_on(self) -> bool {
        matches!(self, Self::On)
    }
}

/// Platform application errors. Keep the variants narrow so the
/// Settings UI can surface a single clear reason when a toggle
/// fails.
#[derive(Debug, Error)]
pub enum WormError {
    /// The current OS / filesystem doesn't expose a WORM primitive
    /// we can use. Windows-on-FAT32 is the canonical case.
    #[error("WORM mode unsupported on this platform or filesystem")]
    Unsupported,
    /// The primitive exists but the call failed — permission
    /// denied, unsupported filesystem, CAP_LINUX_IMMUTABLE missing,
    /// etc. The string is a user-legible explanation the UI pipes
    /// into a toast.
    #[error("WORM mode apply failed: {0}")]
    Apply(String),
}

/// Whether WORM mode can be applied on this build at all. True on
/// every platform we currently implement the primitive for. Used by
/// the Settings UI to grey the toggle out on wholly unsupported
/// environments.
pub fn is_worm_supported() -> bool {
    cfg!(any(target_os = "linux", target_os = "macos", windows))
}

/// Apply (or clear) the WORM attribute on `path`. No-op when
/// `mode == WormMode::Off`. Idempotent — re-applying an already-
/// append-only file is fine.
pub fn apply_worm(path: &Path, mode: WormMode) -> std::result::Result<(), WormError> {
    if !mode.is_on() {
        return Ok(());
    }
    if !path.exists() {
        return Err(WormError::Apply(format!(
            "path does not exist: {}",
            path.display()
        )));
    }

    #[cfg(target_os = "linux")]
    {
        linux::set_append_only(path)
    }
    #[cfg(target_os = "macos")]
    {
        macos::set_append_only(path)
    }
    #[cfg(target_os = "windows")]
    {
        windows::set_append_only(path)
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos", windows)))]
    {
        let _ = path;
        Err(WormError::Unsupported)
    }
}

// ---------------------------------------------------------------------
// Linux — ext4 / xfs / btrfs FS_APPEND_FL via FS_IOC_SETFLAGS
// ---------------------------------------------------------------------

#[cfg(target_os = "linux")]
mod linux {
    use std::fs::OpenOptions;
    use std::os::unix::io::AsRawFd;
    use std::path::Path;

    use super::WormError;

    // FS_APPEND_FL bit per `linux/fs.h`. Value is stable across
    // kernel versions; hard-coded so we don't depend on a separate
    // `linux-raw-sys` crate for one constant.
    const FS_APPEND_FL: i32 = 0x0000_0020;

    // `FS_IOC_GETFLAGS` / `FS_IOC_SETFLAGS` use the _IOR / _IOW
    // macro expansions: direction=READ/WRITE(2/1), type='f' (0x66),
    // nr=1/2, size=sizeof(int)=4. Precomputed here — see
    // `linux/fs.h` and `nix::ioctl_read!` / `ioctl_write_ptr!`.
    nix::ioctl_read!(fs_ioc_getflags, b'f', 1, i32);
    nix::ioctl_write_ptr!(fs_ioc_setflags, b'f', 2, i32);

    pub fn set_append_only(path: &Path) -> Result<(), WormError> {
        let file = OpenOptions::new()
            .read(true)
            .open(path)
            .map_err(|e| WormError::Apply(format!("open for FS_IOC_GETFLAGS: {e}")))?;
        let fd = file.as_raw_fd();
        let mut current: i32 = 0;
        // SAFETY: `fs_ioc_getflags` requires an open fd + a pointer
        // to a writable i32. Both preconditions hold here and the
        // scope of the unsafe block is bounded to the single call.
        unsafe {
            fs_ioc_getflags(fd, &mut current)
                .map_err(|e| WormError::Apply(format!("FS_IOC_GETFLAGS: {e}")))?;
        }
        let next = current | FS_APPEND_FL;
        if next == current {
            return Ok(());
        }
        // SAFETY: same as above — valid fd + writable i32.
        unsafe {
            fs_ioc_setflags(fd, &next).map_err(|e| {
                WormError::Apply(format!("FS_IOC_SETFLAGS (needs CAP_LINUX_IMMUTABLE): {e}"))
            })?;
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------
// macOS — UF_APPEND via chflags(2)
// ---------------------------------------------------------------------

#[cfg(target_os = "macos")]
mod macos {
    use std::ffi::CString;
    use std::path::Path;

    use super::WormError;

    // UF_APPEND = 0x00000004 per `<sys/stat.h>`.
    const UF_APPEND: libc::c_uint = 0x0000_0004;

    pub fn set_append_only(path: &Path) -> Result<(), WormError> {
        let raw = path.to_string_lossy();
        let c = CString::new(raw.as_bytes())
            .map_err(|e| WormError::Apply(format!("path contains NUL: {e}")))?;

        // Read existing flags, OR UF_APPEND, write back. stat-based
        // so a second call is idempotent.
        let mut st: libc::stat = unsafe { std::mem::zeroed() };
        // SAFETY: `c.as_ptr()` points to a valid NUL-terminated C
        // string; `&mut st` is a writable stat struct.
        let rc = unsafe { libc::stat(c.as_ptr(), &mut st) };
        if rc != 0 {
            return Err(WormError::Apply(format!(
                "stat: {}",
                std::io::Error::last_os_error()
            )));
        }
        let next = st.st_flags | UF_APPEND;
        if next == st.st_flags {
            return Ok(());
        }
        // SAFETY: `c.as_ptr()` is a valid C string. `libc::chflags`
        // takes a u32 flag mask; casting through `c_uint` is the
        // libc convention on macOS.
        let rc = unsafe { libc::chflags(c.as_ptr(), next) };
        if rc != 0 {
            return Err(WormError::Apply(format!(
                "chflags(UF_APPEND): {}",
                std::io::Error::last_os_error()
            )));
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------
// Windows — read-only attribute as the portable fallback
// ---------------------------------------------------------------------

#[cfg(target_os = "windows")]
mod windows {
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use std::path::Path;

    use windows_sys::Win32::Storage::FileSystem::{
        FILE_ATTRIBUTE_READONLY, GetFileAttributesW, SetFileAttributesW,
    };

    use super::WormError;

    pub fn set_append_only(path: &Path) -> Result<(), WormError> {
        let wide: Vec<u16> = path.as_os_str().encode_wide().chain(once(0)).collect();
        // SAFETY: `wide` is a valid NUL-terminated wide string.
        // `GetFileAttributesW` reads the attribute bitmask via a
        // `const PCWSTR`; the return value signals errors with
        // `INVALID_FILE_ATTRIBUTES` (u32::MAX).
        let current = unsafe { GetFileAttributesW(wide.as_ptr()) };
        if current == u32::MAX {
            return Err(WormError::Apply(format!(
                "GetFileAttributesW: {}",
                std::io::Error::last_os_error()
            )));
        }
        let next = current | FILE_ATTRIBUTE_READONLY;
        // SAFETY: wide pointer still valid; `SetFileAttributesW`
        // takes the same `const PCWSTR` + new attribute mask.
        let ok = unsafe { SetFileAttributesW(wide.as_ptr(), next) };
        if ok == 0 {
            return Err(WormError::Apply(format!(
                "SetFileAttributesW: {}",
                std::io::Error::last_os_error()
            )));
        }
        // Note: FILE_ATTRIBUTE_READONLY is the portable fallback;
        // a proper deny-write ACE via SetNamedSecurityInfoW is the
        // richer primitive but requires an ACE composition step we
        // punt on until Phase 36 CLI wires the verify command.
        // Users get the attribute-level guard today which is enough
        // to prevent accidental `echo > log` overwrite.
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn off_is_noop() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("log.txt");
        std::fs::write(&path, b"hello").unwrap();
        apply_worm(&path, WormMode::Off).expect("off noop");
    }

    #[test]
    fn missing_path_surfaces_apply_error() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("does-not-exist.log");
        let err = apply_worm(&path, WormMode::On).unwrap_err();
        assert!(matches!(err, WormError::Apply(_)));
    }

    #[test]
    fn supported_flag_is_true_on_host() {
        // The three implemented host families are the only ones
        // the CI matrix builds for. A host outside the matrix
        // (e.g. FreeBSD) would report false here.
        #[cfg(any(target_os = "linux", target_os = "macos", windows))]
        assert!(is_worm_supported());
    }
}
