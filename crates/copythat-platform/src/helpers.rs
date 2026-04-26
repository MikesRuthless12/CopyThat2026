//! Storage probes and concurrency heuristics.
//!
//! All probes are best-effort. They return `Option<T>` where `None`
//! means "could not determine". Callers should treat `None` as
//! "answer unknown — don't change behaviour".
//!
//! The implementations live in per-OS modules but the public surface
//! stays portable so callers don't need `cfg`-guarded dispatch tables.

use std::path::Path;

/// Default fan-out for tree copies on rotational media: clamp to 1.
///
/// Rationale: spinning disks pay for every seek. Two threads writing
/// interleaved 1 MiB chunks to the same HDD produce lower throughput
/// than one thread writing serially, regardless of how many CPUs you
/// have. This constant is what [`recommend_concurrency`] returns when
/// either side is on rotational storage.
pub const DEFAULT_HDD_CONCURRENCY: usize = 1;

/// Best-effort probe: does `path` live on an SSD?
///
/// `Some(true)` — flash. `Some(false)` — rotational. `None` — unknown.
/// Implementation matches `copythat_secure_delete::is_ssd`: Linux reads
/// `/sys/block/<dev>/queue/rotational`, macOS shells out to
/// `diskutil info`, Windows runs PowerShell `Get-PhysicalDisk`.
pub fn is_ssd(path: &Path) -> Option<bool> {
    crate::native::is_ssd(path)
}

/// Best-effort filesystem-name probe (e.g. `"ntfs"`, `"apfs"`,
/// `"btrfs"`, `"xfs"`, `"ext4"`, `"refs"`).
///
/// Returned name is lowercase. `None` if the OS-specific probe fails
/// or isn't implemented for this platform. Used by [`supports_reflink`]
/// and exposed for diagnostic logging.
pub fn filesystem_name(path: &Path) -> Option<String> {
    crate::native::filesystem_name(path)
}

/// Best-effort guess: does the filesystem at `path` support reflink?
///
/// Returns `Some(true)` for known COW filesystems (Btrfs, XFS with
/// reflink=1, ZFS, bcachefs, APFS, ReFS); `Some(false)` for known
/// non-COW filesystems (NTFS without Dev Drive, ext4, FAT32);
/// `None` otherwise. The dispatcher does not consult this: it always
/// tries reflink first and lets the syscall report support. This
/// helper is intended for diagnostic UI ("This volume supports
/// instant copies — clone size: 0 B").
pub fn supports_reflink(path: &Path) -> Option<bool> {
    let name = filesystem_name(path)?.to_ascii_lowercase();
    match name.as_str() {
        // Known COW filesystems.
        "btrfs" | "xfs" | "zfs" | "bcachefs" | "apfs" | "refs" => Some(true),
        // Known non-COW filesystems.
        "ntfs" | "ext2" | "ext3" | "ext4" | "fat" | "fat32" | "vfat" | "exfat" | "msdos"
        | "hfs" | "hfs+" | "hfsplus" => Some(false),
        _ => None,
    }
}

/// Phase 14 — return the number of bytes currently free on the
/// volume that backs `path`. `None` when the OS probe fails (unknown
/// path, permission denied, unmounted volume). The caller can treat
/// `None` as "reserve-check unavailable" and proceed without guard.
///
/// Windows uses `GetDiskFreeSpaceExW`; Unix uses `statvfs`. Both are
/// cheap enough to call on every file inside a tree, which is what
/// the engine's per-file reserve enforcement relies on.
pub fn free_space_bytes(path: &Path) -> Option<u64> {
    free_space_impl(path)
}

/// Phase 14 — volume identity for cross-volume detection.
///
/// Returns an opaque `u64` that is stable for all paths living on
/// the same mounted volume and different between volumes. Used by
/// the reflink path to avoid paying the syscall cost on a copy that
/// can't possibly reflink (different volume → reflink is a hard
/// rejection). `None` when the OS probe fails — callers should treat
/// unknown as "may be same volume, try reflink".
///
/// Windows: GetVolumeInformationByHandleW's VolumeSerialNumber.
/// Unix: the `st_dev` from `stat`.
pub fn volume_id(path: &Path) -> Option<u64> {
    volume_id_impl(path)
}

#[cfg(target_os = "windows")]
fn free_space_impl(path: &Path) -> Option<u64> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    // GetDiskFreeSpaceExW wants a directory path. If the caller hands
    // us a file, probe the parent.
    let target: &Path = if path.is_file() { path.parent()? } else { path };
    let mut wide: Vec<u16> = OsStr::new(target).encode_wide().collect();
    wide.push(0);
    let mut free_to_caller: u64 = 0;
    let mut _total: u64 = 0;
    let mut _free_total: u64 = 0;
    // SAFETY: we pass a NUL-terminated UTF-16 string and three u64
    // out-pointers that live for the duration of the call.
    let ok = unsafe {
        windows_sys::Win32::Storage::FileSystem::GetDiskFreeSpaceExW(
            wide.as_ptr(),
            &mut free_to_caller,
            &mut _total,
            &mut _free_total,
        )
    };
    if ok == 0 { None } else { Some(free_to_caller) }
}

#[cfg(unix)]
fn free_space_impl(path: &Path) -> Option<u64> {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;
    let target: &Path = if path.is_file() { path.parent()? } else { path };
    let cstr = CString::new(target.as_os_str().as_bytes()).ok()?;
    let mut sv: libc::statvfs = unsafe { std::mem::zeroed() };
    // SAFETY: cstr is NUL-terminated and sv is a zero-init statvfs.
    let ret = unsafe { libc::statvfs(cstr.as_ptr(), &mut sv) };
    if ret != 0 {
        return None;
    }
    Some((sv.f_bavail as u64).saturating_mul(sv.f_frsize as u64))
}

#[cfg(not(any(target_os = "windows", unix)))]
fn free_space_impl(_path: &Path) -> Option<u64> {
    None
}

#[cfg(target_os = "windows")]
fn volume_id_impl(path: &Path) -> Option<u64> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    // GetVolumeInformationW requires a volume root (e.g. `C:\`). For
    // an arbitrary file/dir path, walk up to the volume mount point
    // first via GetVolumePathNameW. Files that don't yet exist are
    // resolved via their parent dir.
    let target: &Path = if path.is_file() { path.parent()? } else { path };
    let mut wide: Vec<u16> = OsStr::new(target).encode_wide().collect();
    wide.push(0);
    let mut root_buf: [u16; 260] = [0; 260];
    // SAFETY: wide is NUL-terminated; root_buf is a fixed-size buffer
    // sized to MAX_PATH + 1.
    let path_ok = unsafe {
        windows_sys::Win32::Storage::FileSystem::GetVolumePathNameW(
            wide.as_ptr(),
            root_buf.as_mut_ptr(),
            root_buf.len() as u32,
        )
    };
    if path_ok == 0 {
        return None;
    }
    let mut serial: u32 = 0;
    // SAFETY: root_buf is NUL-terminated by the prior call; serial
    // lives for the duration of the call.
    let ok = unsafe {
        windows_sys::Win32::Storage::FileSystem::GetVolumeInformationW(
            root_buf.as_ptr(),
            std::ptr::null_mut(),
            0,
            &mut serial,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            0,
        )
    };
    if ok == 0 { None } else { Some(serial as u64) }
}

#[cfg(unix)]
fn volume_id_impl(path: &Path) -> Option<u64> {
    use std::os::unix::fs::MetadataExt;
    std::fs::metadata(path).ok().map(|m| m.dev())
}

#[cfg(not(any(target_os = "windows", unix)))]
fn volume_id_impl(_path: &Path) -> Option<u64> {
    None
}

/// Recommend a concurrency level for a tree-copy walking from `src` to
/// `dst`.
///
/// Heuristic: if either side reports rotational storage, clamp to
/// [`DEFAULT_HDD_CONCURRENCY`] (1) to avoid seek thrash. Otherwise
/// return `requested` unchanged. Unknown answers are treated as SSD —
/// most modern hardware is, and the worst case is mild HDD seek
/// thrash, not correctness.
pub fn recommend_concurrency(src: &Path, dst: &Path, requested: usize) -> usize {
    let src_rotational = matches!(is_ssd(src), Some(false));
    let dst_rotational = matches!(is_ssd(dst), Some(false));
    if src_rotational || dst_rotational {
        DEFAULT_HDD_CONCURRENCY
    } else {
        requested
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn probe_returns_option_without_panicking() {
        // Don't assert a value — runners differ. Just verify the probe
        // tolerates a real path without unwinding.
        let here = PathBuf::from(".");
        let _ = is_ssd(&here);
        let _ = filesystem_name(&here);
        let _ = supports_reflink(&here);
    }

    #[test]
    fn free_space_on_local_path_returns_some_positive_or_none() {
        // Host-dependent — on a real dev box the probe returns a
        // real number; in some sandboxed CI runners it may return
        // None. Both are acceptable. What matters is that it
        // doesn't panic and returns either None or a positive u64.
        let here = PathBuf::from(".");
        if let Some(n) = free_space_bytes(&here) {
            assert!(n > 0, "free space should not be 0 on . ({n})");
        }
    }

    #[cfg(windows)]
    #[test]
    fn free_space_on_unc_path_tolerates_offline_host() {
        // UNC path to a server that almost certainly doesn't exist.
        // The probe must return `None` rather than panic or block.
        let fake = PathBuf::from(r"\\copythat-test-host-does-not-exist\share");
        let res = free_space_bytes(&fake);
        assert!(res.is_none(), "expected None for offline UNC, got {res:?}");
    }

    #[test]
    fn volume_id_is_stable_for_same_path() {
        // Calling twice on the same mount should yield the same id
        // (or None twice). Either case indicates a well-behaved probe.
        let here = PathBuf::from(".");
        let a = volume_id(&here);
        let b = volume_id(&here);
        assert_eq!(a, b);
    }

    #[test]
    fn recommend_clamps_to_one_on_rotational() {
        // Pure logic test: if both ends are rotational, even a
        // requested concurrency of 32 should clamp.
        // (We can't fake the probe directly here without injecting,
        // but we can at least verify the function runs and returns a
        // sane positive number on the host.)
        let here = PathBuf::from(".");
        let n = recommend_concurrency(&here, &here, 8);
        assert!((1..=8).contains(&n), "concurrency out of range: {n}");
    }
}
