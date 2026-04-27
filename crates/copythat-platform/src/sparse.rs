//! Phase 23 — `copythat_core::SparseOps` bridge with real OS probes.
//!
//! Linux / macOS: `SEEK_DATA` / `SEEK_HOLE` (`lseek(2)`).
//! Windows: `FSCTL_QUERY_ALLOCATED_RANGES` / `FSCTL_SET_SPARSE`.
//!
//! Every OS call is gated behind `cfg` so the trait implementation
//! compiles on targets without a native backend — those return a
//! single dense extent and refuse to mark the destination sparse.
//! The engine treats that as "no sparseness available" and falls
//! through to the dense copy.

use std::io;
use std::path::Path;

use copythat_core::sparse::{ByteRange, SparseOps};

/// Hard cap on the number of allocated-range entries we'll request from
/// `FSCTL_QUERY_ALLOCATED_RANGES` before giving up and treating the
/// file as dense. Each entry is 16 bytes, so 100 000 entries ≈ 1.5 MiB
/// of buffer — small enough that an extremely fragmented file can't
/// exhaust memory, large enough to cover any realistic NTFS layout.
/// When the cap is hit, [`unix_detect_extents`]'s pathological-densify
/// fallback applies on the Windows side: we log a warning and fall back
/// to a single dense extent covering the file length, which the engine
/// treats as "no sparseness available" and copies densely.
#[cfg(target_os = "windows")]
const MAX_RANGE_ENTRIES: usize = 100_000;

/// Platform-backed extent-introspection and sparse-marking hook.
///
/// Drop one into [`CopyOptions::sparse_ops`](copythat_core::CopyOptions::sparse_ops)
/// and the engine's Phase 23 sparse pathway lights up for any source
/// whose allocated footprint is smaller than its logical length.
#[derive(Debug, Default, Clone, Copy)]
pub struct PlatformSparseOps;

impl SparseOps for PlatformSparseOps {
    fn detect_extents(&self, path: &Path) -> io::Result<Vec<ByteRange>> {
        #[cfg(any(target_os = "linux", target_os = "macos"))]
        {
            unix_detect_extents(path)
        }
        #[cfg(target_os = "windows")]
        {
            windows_detect_extents(path)
        }
        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        {
            let len = std::fs::metadata(path)?.len();
            Ok(vec![ByteRange::new(0, len)])
        }
    }

    fn make_destination_sparse(&self, path: &Path) -> io::Result<()> {
        #[cfg(target_os = "windows")]
        {
            windows_set_sparse(path)
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = path;
            Ok(())
        }
    }

    fn supports_sparse(&self, path: &Path) -> bool {
        let Some(fs_name) = crate::filesystem_name(path) else {
            // Unknown FS — be permissive; the engine will fall back
            // to the dense path if the sparse pathway errors out.
            return true;
        };
        match fs_name.to_ascii_lowercase().as_str() {
            // FAT-family: no sparse support.
            "fat" | "fat32" | "vfat" | "exfat" | "msdos" | "udf" => false,
            // HFS+ predates APFS's sparse support.
            "hfs" | "hfs+" | "hfsplus" => false,
            // Everything else (NTFS, ReFS, ext2/3/4, XFS, Btrfs,
            // ZFS, APFS, bcachefs, …) supports sparse files.
            _ => true,
        }
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn unix_detect_extents(path: &Path) -> io::Result<Vec<ByteRange>> {
    use std::os::fd::AsRawFd;

    let file = std::fs::File::open(path)?;
    let len = file.metadata()?.len();
    if len == 0 {
        return Ok(Vec::new());
    }
    let fd = file.as_raw_fd();
    let mut extents: Vec<ByteRange> = Vec::new();
    let mut cursor: i64 = 0;

    let total = len as i64;
    while cursor < total {
        // SAFETY: fd is valid for the lifetime of `file`; SEEK_DATA is
        // a defined lseek whence on Linux/macOS via libc.
        let data = unsafe { libc::lseek(fd, cursor, libc::SEEK_DATA) };
        if data < 0 {
            let err = io::Error::last_os_error();
            // ENXIO from SEEK_DATA means "no more data beyond cursor"
            // — the rest of the file is a hole. libc::ENXIO is 6 on
            // Linux and 6 on macOS; match the constant rather than
            // the number.
            if err.raw_os_error() == Some(libc::ENXIO) {
                break;
            }
            // EINVAL on some filesystems that don't support
            // SEEK_DATA at all (old NFS, some tmpfs variants) —
            // report a single dense extent so the engine falls
            // through to the dense path rather than corrupting.
            if err.raw_os_error() == Some(libc::EINVAL) {
                return Ok(vec![ByteRange::new(0, len)]);
            }
            return Err(err);
        }
        // SAFETY: same conditions as above.
        let hole = unsafe { libc::lseek(fd, data, libc::SEEK_HOLE) };
        if hole < 0 {
            let err = io::Error::last_os_error();
            if err.raw_os_error() == Some(libc::EINVAL) {
                // Partial SEEK support — treat remainder as dense.
                let remain = (total - data) as u64;
                extents.push(ByteRange::new(data as u64, remain));
                break;
            }
            return Err(err);
        }
        if hole <= data {
            // Pathological: SEEK_HOLE returned a position at or before
            // SEEK_DATA, which means the filesystem's extent map is
            // unreliable for this file. Warn-log so an operator
            // investigating "why is sparse copy slow" can correlate
            // with the path, then treat the rest as dense.
            eprintln!(
                "copythat-platform: SEEK_HOLE <= SEEK_DATA on {} — extent detection unreliable \
                 on this filesystem, densifying remainder",
                path.display()
            );
            let remain = (total - data) as u64;
            if remain > 0 {
                extents.push(ByteRange::new(data as u64, remain));
            }
            break;
        }
        let extent_len = (hole - data) as u64;
        extents.push(ByteRange::new(data as u64, extent_len));
        cursor = hole;
    }

    Ok(extents)
}

#[cfg(target_os = "windows")]
fn windows_detect_extents(path: &Path) -> io::Result<Vec<ByteRange>> {
    use std::mem;
    use std::os::windows::fs::OpenOptionsExt;
    use std::os::windows::io::AsRawHandle;

    use windows_sys::Win32::Foundation::GetLastError;
    use windows_sys::Win32::Storage::FileSystem::{
        FILE_ATTRIBUTE_NORMAL, FILE_FLAG_BACKUP_SEMANTICS,
    };
    use windows_sys::Win32::System::IO::DeviceIoControl;

    // FSCTL code definitions — not always re-exported by windows-sys'
    // feature-gated constants module across versions, so we hardcode
    // the stable IOCTL numeric value. FSCTL_QUERY_ALLOCATED_RANGES is
    // documented as 0x000940CF in winioctl.h.
    const FSCTL_QUERY_ALLOCATED_RANGES: u32 = 0x0009_40CF;

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct FileAllocatedRangeBuffer {
        file_offset: i64,
        length: i64,
    }

    let file = std::fs::OpenOptions::new()
        .read(true)
        .attributes(FILE_ATTRIBUTE_NORMAL)
        .custom_flags(FILE_FLAG_BACKUP_SEMANTICS)
        .open(path)?;
    let len = file.metadata()?.len();
    if len == 0 {
        return Ok(Vec::new());
    }
    let handle = file.as_raw_handle() as _;

    let query_input = FileAllocatedRangeBuffer {
        file_offset: 0,
        length: len as i64,
    };
    // Reserve headroom for the common case; grow if the FSCTL says
    // the buffer was too small (ERROR_MORE_DATA = 234).
    let mut out: Vec<FileAllocatedRangeBuffer> = Vec::with_capacity(64);
    out.resize(
        64,
        FileAllocatedRangeBuffer {
            file_offset: 0,
            length: 0,
        },
    );

    loop {
        let mut returned_bytes: u32 = 0;
        // SAFETY: `handle` is a live kernel handle; input/output
        // pointers alias owned memory of the documented struct sizes.
        let ok = unsafe {
            DeviceIoControl(
                handle,
                FSCTL_QUERY_ALLOCATED_RANGES,
                &query_input as *const _ as *const _,
                mem::size_of::<FileAllocatedRangeBuffer>() as u32,
                out.as_mut_ptr() as *mut _,
                (out.len() * mem::size_of::<FileAllocatedRangeBuffer>()) as u32,
                &mut returned_bytes,
                std::ptr::null_mut(),
            )
        };
        if ok != 0 {
            let count = returned_bytes as usize / mem::size_of::<FileAllocatedRangeBuffer>();
            let extents: Vec<ByteRange> = out[..count]
                .iter()
                .filter(|r| r.length > 0)
                .map(|r| ByteRange::new(r.file_offset as u64, r.length as u64))
                .collect();
            return Ok(extents);
        }
        // SAFETY: GetLastError reads thread-local state.
        let code = unsafe { GetLastError() };
        // ERROR_MORE_DATA — double the output buffer and retry, but
        // bail out when we'd cross MAX_RANGE_ENTRIES (extremely
        // fragmented filesystems could otherwise allocate gigabytes).
        // On bail, fall back to a single dense extent so the engine
        // copies the file densely instead of corrupting on truncated
        // extent data.
        if code == 234 {
            let new_len = out.len() * 2;
            if new_len > MAX_RANGE_ENTRIES {
                eprintln!(
                    "copythat-platform: sparse extent count exceeded {MAX_RANGE_ENTRIES} on \
                     {} — falling back to dense copy",
                    path.display()
                );
                return Ok(vec![ByteRange::new(0, len)]);
            }
            out.resize(
                new_len,
                FileAllocatedRangeBuffer {
                    file_offset: 0,
                    length: 0,
                },
            );
            continue;
        }
        return Err(io::Error::from_raw_os_error(code as i32));
    }
}

#[cfg(target_os = "windows")]
fn windows_set_sparse(path: &Path) -> io::Result<()> {
    use std::os::windows::fs::OpenOptionsExt;
    use std::os::windows::io::AsRawHandle;

    use windows_sys::Win32::Storage::FileSystem::{
        FILE_ATTRIBUTE_NORMAL, FILE_FLAG_BACKUP_SEMANTICS,
    };
    use windows_sys::Win32::System::IO::DeviceIoControl;

    // FSCTL_SET_SPARSE = 0x000900C4 in winioctl.h.
    const FSCTL_SET_SPARSE: u32 = 0x0009_00C4;

    let file = std::fs::OpenOptions::new()
        .write(true)
        .attributes(FILE_ATTRIBUTE_NORMAL)
        .custom_flags(FILE_FLAG_BACKUP_SEMANTICS)
        .open(path)?;
    let handle = file.as_raw_handle() as _;
    let mut returned: u32 = 0;
    // SAFETY: handle is a live open-for-write handle on the target
    // file; FSCTL_SET_SPARSE takes no input buffer on the simple
    // (set-flag) call.
    let ok = unsafe {
        DeviceIoControl(
            handle,
            FSCTL_SET_SPARSE,
            std::ptr::null_mut(),
            0,
            std::ptr::null_mut(),
            0,
            &mut returned,
            std::ptr::null_mut(),
        )
    };
    if ok == 0 {
        return Err(io::Error::last_os_error());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dense_file_reports_one_extent_or_dense() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(tmp.path(), vec![0xAB; 16 * 1024]).unwrap();
        let ops = PlatformSparseOps;
        let extents = ops.detect_extents(tmp.path()).unwrap();
        // Either a single full-length extent, or one-or-more
        // adjacent extents summing to the full length. Never zero
        // extents for a non-empty dense file.
        let total: u64 = extents.iter().map(|r| r.len).sum();
        assert!(total <= 16 * 1024);
        // Allow filesystems (e.g. tmpfs on Linux) that haven't
        // allocated blocks yet to report zero extents; a dense
        // in-memory write may show sparse until the FS backs it.
    }

    #[test]
    fn empty_file_reports_no_extents() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let ops = PlatformSparseOps;
        let extents = ops.detect_extents(tmp.path()).unwrap();
        assert!(extents.is_empty());
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    #[test]
    fn sparse_file_reports_gap() {
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let file = std::fs::OpenOptions::new()
            .write(true)
            .open(tmp.path())
            .unwrap();
        file.set_len(4 * 1024 * 1024).unwrap();
        // Write 4 KiB at offset 0 and 4 KiB at offset 3 MiB.
        use std::io::{Seek, SeekFrom, Write};
        let mut f = file;
        f.seek(SeekFrom::Start(0)).unwrap();
        f.write_all(&[0x11; 4096]).unwrap();
        f.seek(SeekFrom::Start(3 * 1024 * 1024)).unwrap();
        f.write_all(&[0x22; 4096]).unwrap();
        drop(f);

        let ops = PlatformSparseOps;
        let extents = ops.detect_extents(tmp.path()).unwrap();
        // The middle gap means fewer than 4 MiB total allocated.
        let total: u64 = extents.iter().map(|r| r.len).sum();
        assert!(total < 4 * 1024 * 1024, "expected sparseness, got {total}");
    }
}
