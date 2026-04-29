//! Phase 42 — pre-copy source-attribute probe.
//!
//! `GetFileAttributesExW` returns a `WIN32_FILE_ATTRIBUTE_DATA` whose
//! `dwFileAttributes` byte tells us — in a single ~50-microsecond
//! syscall — whether the source is a reparse point, sparse file,
//! NTFS-compressed file, EFS-encrypted file, or an OneDrive cloud
//! placeholder. The dispatcher uses these to make routing decisions
//! (e.g., engage `COPY_FILE_ENABLE_SPARSE_COPY` on Win11 22H2+,
//! warn the user before hydrating a 10 GB OneDrive placeholder, fall
//! back to manual junction recreation, etc.).
//!
//! On non-Windows targets this is a no-op stub returning a
//! conservative all-false [`SrcAttributes`]; the per-OS native paths
//! (Linux `copy_file_range`, macOS `copyfile(3)`) handle their own
//! flags / extended-attribute preservation.

use std::path::Path;

/// Snapshot of the Windows file-attribute byte for a source file.
#[derive(Debug, Clone, Copy, Default)]
pub struct SrcAttributes {
    /// `FILE_ATTRIBUTE_REPARSE_POINT` (0x400). Junction, symlink,
    /// OneDrive placeholder, dedup stub, AppExecLink, etc.
    pub is_reparse_point: bool,
    /// `FILE_ATTRIBUTE_SPARSE_FILE` (0x200). Source has unallocated
    /// zero ranges; preserving them avoids ballooning the destination.
    pub is_sparse: bool,
    /// `FILE_ATTRIBUTE_COMPRESSED` (0x800). NTFS LZX/XPRESS
    /// compression. CopyFileEx decompresses on read; manual
    /// `FSCTL_SET_COMPRESSION` re-applies on dest.
    pub is_compressed: bool,
    /// `FILE_ATTRIBUTE_ENCRYPTED` (0x4000). EFS file. CopyFileEx
    /// re-encrypts at dest with src keys; cross-volume to non-NTFS
    /// requires `OpenEncryptedFileRaw`.
    pub is_encrypted: bool,
    /// `FILE_ATTRIBUTE_OFFLINE` (0x1000). Legacy "not in primary
    /// storage" flag — set by HSM solutions including older OneDrive.
    pub is_offline: bool,
    /// `FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS` (0x400000). Modern
    /// OneDrive / Cloud Files API placeholder. Reading triggers a
    /// transparent download — potentially gigabytes of unintended
    /// network traffic.
    pub is_recall_on_data_access: bool,
}

impl SrcAttributes {
    /// True iff the source needs a non-default copy strategy beyond
    /// "open and stream bytes". Currently: any reparse point, any
    /// sparse file, or any cloud/offline placeholder.
    pub fn needs_special_handling(&self) -> bool {
        self.is_reparse_point || self.is_sparse || self.is_offline || self.is_recall_on_data_access
    }
}

/// Probe the source's file attributes. Returns `None` only when the
/// path doesn't exist or the syscall failed.
///
/// On non-Windows targets returns `Some(SrcAttributes::default())` —
/// the per-OS native copy paths handle their own metadata
/// preservation.
pub fn probe(path: &Path) -> Option<SrcAttributes> {
    probe_impl(path)
}

#[cfg(target_os = "windows")]
fn probe_impl(path: &Path) -> Option<SrcAttributes> {
    use std::ffi::OsStr;
    use std::mem::MaybeUninit;
    use std::os::windows::ffi::OsStrExt;

    use windows_sys::Win32::Storage::FileSystem::{
        GetFileAttributesExW, GetFileExInfoStandard, WIN32_FILE_ATTRIBUTE_DATA,
    };

    // Win32 attribute constants (stable ABI; inlined to avoid pulling
    // in a windows-sys feature module just for these).
    const FILE_ATTRIBUTE_SPARSE_FILE: u32 = 0x200;
    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x400;
    const FILE_ATTRIBUTE_COMPRESSED: u32 = 0x800;
    const FILE_ATTRIBUTE_OFFLINE: u32 = 0x1000;
    const FILE_ATTRIBUTE_ENCRYPTED: u32 = 0x4000;
    const FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS: u32 = 0x400000;

    let mut wide: Vec<u16> = OsStr::new(path).encode_wide().collect();
    wide.push(0);
    let mut data: MaybeUninit<WIN32_FILE_ATTRIBUTE_DATA> = MaybeUninit::zeroed();
    // SAFETY: wide is NUL-terminated; data is a properly-sized,
    // zero-initialised WIN32_FILE_ATTRIBUTE_DATA.
    let ok = unsafe {
        GetFileAttributesExW(
            wide.as_ptr(),
            GetFileExInfoStandard,
            data.as_mut_ptr().cast(),
        )
    };
    if ok == 0 {
        return None;
    }
    // SAFETY: ok != 0 means data is now initialised.
    let data = unsafe { data.assume_init() };
    let attrs = data.dwFileAttributes;

    Some(SrcAttributes {
        is_reparse_point: attrs & FILE_ATTRIBUTE_REPARSE_POINT != 0,
        is_sparse: attrs & FILE_ATTRIBUTE_SPARSE_FILE != 0,
        is_compressed: attrs & FILE_ATTRIBUTE_COMPRESSED != 0,
        is_encrypted: attrs & FILE_ATTRIBUTE_ENCRYPTED != 0,
        is_offline: attrs & FILE_ATTRIBUTE_OFFLINE != 0,
        is_recall_on_data_access: attrs & FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS != 0,
    })
}

#[cfg(not(target_os = "windows"))]
fn probe_impl(path: &Path) -> Option<SrcAttributes> {
    if !path.exists() {
        return None;
    }
    Some(SrcAttributes::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn probe_self_does_not_panic() {
        let here = PathBuf::from(".");
        let _ = probe(&here);
    }

    #[test]
    fn nonexistent_returns_none() {
        let nope = PathBuf::from("zzz-this-path-does-not-exist-zzz");
        assert!(probe(&nope).is_none());
    }

    #[test]
    fn default_attrs_need_no_special_handling() {
        let a = SrcAttributes::default();
        assert!(!a.needs_special_handling());
    }

    #[test]
    fn sparse_or_reparse_or_cloud_needs_special() {
        for setter in [
            (|a: &mut SrcAttributes| a.is_reparse_point = true) as fn(&mut SrcAttributes),
            |a: &mut SrcAttributes| a.is_sparse = true,
            |a: &mut SrcAttributes| a.is_offline = true,
            |a: &mut SrcAttributes| a.is_recall_on_data_access = true,
        ] {
            let mut a = SrcAttributes::default();
            setter(&mut a);
            assert!(a.needs_special_handling());
        }
    }
}
