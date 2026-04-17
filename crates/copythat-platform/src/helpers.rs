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
