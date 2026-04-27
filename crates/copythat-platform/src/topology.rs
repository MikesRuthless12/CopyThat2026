//! Phase 42 — storage-topology probe.
//!
//! Wraps `IOCTL_STORAGE_QUERY_PROPERTY` so the engine can pick
//! media-class-aware buffer sizes, queue depths, and parallel-N
//! heuristics at runtime instead of guessing. The Phase 42 swarm
//! research found:
//!
//! - NVMe Gen3 → 256 KiB / QD 4-8; Gen4 → 512 KiB / QD 8-16; Gen5 →
//!   1 MiB / QD 16-32.
//! - SATA SSD → 256 KiB / QD 4-8.
//! - HDD (rotational) → 1-4 MiB / QD 1-2 (seek amortisation).
//! - USB UASP → 512 KiB / QD ≤ 4 (vendor bridges choke at deeper QD).
//! - SMB UNC → 256 KiB GbE, 1 MiB 10 GbE, 4 MiB SMB Direct 25 GbE.
//!
//! Probe is best-effort: every helper returns `Option<T>` where `None`
//! means "couldn't determine, caller should fall back to the
//! conservative default".
//!
//! Cached per-volume by serial number so the IOCTL only runs once per
//! drive per process.

use std::path::{Path, PathBuf};

/// Storage bus class as reported by `STORAGE_DEVICE_DESCRIPTOR.BusType`.
///
/// Mirrors `STORAGE_BUS_TYPE` from `winioctl.h` — narrowed to the
/// classes the buffer / QD heuristic actually distinguishes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BusType {
    /// `BusTypeNvme` (0x11) — assume Gen3 unless we have a way to
    /// detect Gen4/Gen5 (we don't, today).
    Nvme,
    /// `BusTypeSata` (0xB) — SATA SSD or SATA HDD; pair with
    /// [`MediaClass`] to disambiguate.
    Sata,
    /// `BusTypeUsb` (0x7) — USB / USB-attached SCSI.
    Usb,
    /// `BusTypeSd` (0xC) — SD card.
    Sd,
    /// `BusTypeMmc` (0xD) — eMMC / MMC.
    Mmc,
    /// `BusTypeFileBackedVirtual` (0x15) — VHD/VHDX, including Hyper-V
    /// guest disks. Often parallel-friendly underneath.
    FileBackedVirtual,
    /// `BusTypeiScsi` (0x9) — iSCSI target.
    Iscsi,
    /// SMB / network UNC path (detected via path prefix, not via
    /// IOCTL).
    Smb,
    /// `BusTypeRAID` (0xF) — hardware RAID controller.
    Raid,
    /// Unknown / not in our enum.
    Other,
}

/// Whether the device reports a seek penalty. HDDs do (rotational);
/// SSDs do not.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaClass {
    /// Solid-state — no seek penalty.
    Ssd,
    /// Rotational — has seek penalty.
    Hdd,
    /// Probe failed or device returned indeterminate.
    Unknown,
}

/// Best-effort topology snapshot for a path's backing volume.
#[derive(Debug, Clone, Copy)]
pub struct VolumeTopology {
    pub bus_type: BusType,
    pub media_class: MediaClass,
    /// Physical sector size from `STORAGE_ACCESS_ALIGNMENT_DESCRIPTOR`.
    /// Common values: 512 (legacy), 4096 (Advanced Format), 8192
    /// (some enterprise NVMe).
    pub bytes_per_physical_sector: u32,
}

impl VolumeTopology {
    /// Conservative default when probing fails: 4 KiB sectors, SATA
    /// SSD-class. Matches what most desktop hardware looks like.
    pub fn conservative_default() -> Self {
        Self {
            bus_type: BusType::Other,
            media_class: MediaClass::Unknown,
            bytes_per_physical_sector: 4096,
        }
    }

    /// Recommended buffer size in bytes for this topology, per the
    /// Phase 42 swarm research table.
    pub fn recommended_buffer_bytes(&self) -> usize {
        const KIB: usize = 1024;
        const MIB: usize = 1024 * 1024;
        match (self.bus_type, self.media_class) {
            // NVMe — assume Gen3-class minimum; deeper queues with
            // larger buffers go through the auto-overlapped path,
            // which has its own knob.
            (BusType::Nvme, _) => 1 * MIB,
            // SATA SSD: flat curve 128K-1M, settle on 256K.
            (BusType::Sata, MediaClass::Ssd) => 256 * KIB,
            // SATA HDD: cache-friendly large blocks.
            (BusType::Sata, MediaClass::Hdd) => 4 * MIB,
            (BusType::Sata, _) => 1 * MIB,
            // USB / UASP enclosures — moderate.
            (BusType::Usb, _) => 512 * KIB,
            // SD / MMC — small.
            (BusType::Sd, _) | (BusType::Mmc, _) => 256 * KIB,
            // VHDX / iSCSI / RAID — assume parallel-friendly large
            // buffers help.
            (BusType::FileBackedVirtual, _)
            | (BusType::Iscsi, _)
            | (BusType::Raid, _) => 4 * MIB,
            // SMB — the auto-overlapped path's 4 MiB / QD 8 default
            // works well in 10 GbE; smaller for slow links.
            (BusType::Smb, _) => 1 * MIB,
            // Unknown.
            (BusType::Other, _) => 1 * MIB,
        }
    }

    /// Recommended queue depth (in-flight outstanding I/Os) for this
    /// topology. Used by the auto-parallel heuristic in `windows.rs`
    /// and the overlapped-pipeline path in `windows_overlapped.rs`.
    pub fn recommended_queue_depth(&self) -> usize {
        match (self.bus_type, self.media_class) {
            (BusType::Nvme, _) => 8,
            (BusType::Sata, MediaClass::Ssd) => 4,
            (BusType::Sata, MediaClass::Hdd) => 1,
            (BusType::Sata, _) => 4,
            (BusType::Usb, _) => 4, // vendor bridges choke deeper
            (BusType::Sd, _) | (BusType::Mmc, _) => 2,
            (BusType::FileBackedVirtual, _)
            | (BusType::Iscsi, _)
            | (BusType::Raid, _) => 8,
            (BusType::Smb, _) => 8,
            (BusType::Other, _) => 4,
        }
    }

    /// True iff a same-file parallel-chunk copy is likely to **win**
    /// on this topology. Single-spindle NVMe / SATA / USB always
    /// regress (Phase 13c -25%/-76% measurements); only multi-spindle
    /// arrays / network paths benefit.
    pub fn parallel_chunk_friendly(&self) -> bool {
        matches!(
            self.bus_type,
            BusType::FileBackedVirtual
                | BusType::Iscsi
                | BusType::Raid
                | BusType::Smb
        )
    }
}

/// Probe the storage topology backing `path`. Returns `None` only when
/// every probe failed; callers should treat `None` as "use
/// `VolumeTopology::conservative_default`".
pub fn probe(path: &Path) -> Option<VolumeTopology> {
    probe_impl(path)
}

/// Returns `true` iff `path` is a Windows UNC path (SMB share). The
/// detection is purely string-based — `\\server\share` or
/// `\\?\UNC\server\share`. Both forms are recognised; mapped drive
/// letters that resolve to a UNC share are NOT detected here (we'd need
/// `WNetGetUniversalNameW` for that — out of scope for the simple flag
/// gate).
pub fn is_unc_path(path: &Path) -> bool {
    let s = path.as_os_str().to_string_lossy();
    let bytes = s.as_bytes();
    // `\\server\share` — but reject `\\?\X:` (non-UNC long-path prefix).
    // `\\?\UNC\server\share` — UNC behind the long-path prefix.
    if bytes.starts_with(b"\\\\?\\UNC\\") || bytes.starts_with(b"//?/UNC/") {
        return true;
    }
    if bytes.starts_with(b"\\\\?\\") || bytes.starts_with(b"//?/") {
        return false; // long-path-prefixed local path
    }
    bytes.starts_with(b"\\\\") || bytes.starts_with(b"//")
}

// ---------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------

/// Phase 42 wave-2 — process-shared probe cache. Lifted out of
/// `probe_impl` so the wave-2 poison-recovery tests can install a
/// pre-poisoned cache and verify the wave-1 `unwrap_or_else(into_inner)`
/// branch fires.
#[cfg(target_os = "windows")]
fn cache() -> &'static std::sync::Mutex<std::collections::HashMap<u64, VolumeTopology>> {
    use std::sync::Mutex;
    use std::sync::OnceLock;
    static CACHE: OnceLock<Mutex<std::collections::HashMap<u64, VolumeTopology>>> =
        OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(std::collections::HashMap::new()))
}

#[cfg(target_os = "windows")]
fn probe_impl(path: &Path) -> Option<VolumeTopology> {
    // Cache by volume serial — re-probing a 2 ms IOCTL on every per-file
    // copy in a 10 000-file tree is wasteful.
    let cache = cache();

    // Special-case UNC paths first — they have no `IOCTL_STORAGE_*` to
    // query.
    if is_unc_path(path) {
        return Some(VolumeTopology {
            bus_type: BusType::Smb,
            media_class: MediaClass::Unknown,
            bytes_per_physical_sector: 4096,
        });
    }

    let serial = crate::helpers::volume_id(path)?;
    {
        // Recover from a poisoned mutex rather than silently bypassing
        // the cache: another thread panicking with the lock held would
        // otherwise force every subsequent caller to re-probe the
        // 2 ms IOCTL forever.
        let guard = cache.lock().unwrap_or_else(|e| {
            eprintln!(
                "copythat-platform::topology: recovering from poisoned cache lock (read)"
            );
            e.into_inner()
        });
        if let Some(t) = guard.get(&serial) {
            return Some(*t);
        }
    }

    let topo = win_probe(path).unwrap_or_else(VolumeTopology::conservative_default);
    {
        let mut guard = cache.lock().unwrap_or_else(|e| {
            eprintln!(
                "copythat-platform::topology: recovering from poisoned cache lock (write)"
            );
            e.into_inner()
        });
        guard.insert(serial, topo);
    }
    Some(topo)
}

#[cfg(target_os = "windows")]
fn win_probe(path: &Path) -> Option<VolumeTopology> {
    use std::ffi::OsStr;
    use std::mem::MaybeUninit;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr;

    use windows_sys::Win32::Foundation::{CloseHandle, GENERIC_READ, INVALID_HANDLE_VALUE};
    use windows_sys::Win32::Storage::FileSystem::{
        CreateFileW, FILE_SHARE_DELETE, FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,
    };
    use windows_sys::Win32::System::Ioctl::{
        IOCTL_STORAGE_QUERY_PROPERTY, PropertyStandardQuery, STORAGE_ACCESS_ALIGNMENT_DESCRIPTOR,
        STORAGE_DEVICE_DESCRIPTOR, STORAGE_PROPERTY_QUERY, StorageAccessAlignmentProperty,
        StorageDeviceProperty, StorageDeviceSeekPenaltyProperty, DEVICE_SEEK_PENALTY_DESCRIPTOR,
    };
    use windows_sys::Win32::System::IO::DeviceIoControl;

    // Need a path that names a volume. For an arbitrary file/dir,
    // walk up to the volume root.
    let probe_target: PathBuf = if path.is_file() {
        path.parent()?.to_path_buf()
    } else {
        path.to_path_buf()
    };
    let mut wide: Vec<u16> = OsStr::new(&probe_target).encode_wide().collect();
    wide.push(0);
    let mut root_buf = [0u16; 260];
    // SAFETY: wide is NUL-terminated; root_buf is sized to MAX_PATH+1.
    let ok = unsafe {
        windows_sys::Win32::Storage::FileSystem::GetVolumePathNameW(
            wide.as_ptr(),
            root_buf.as_mut_ptr(),
            root_buf.len() as u32,
        )
    };
    if ok == 0 {
        return None;
    }
    // Convert "C:\" → "\\.\C:" for the IOCTL handle.
    let len = root_buf.iter().position(|&c| c == 0).unwrap_or(root_buf.len());
    let root_str = String::from_utf16_lossy(&root_buf[..len]);
    let drive_letter = root_str
        .chars()
        .find(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_uppercase())?;
    let device_path = format!("\\\\.\\{}:", drive_letter);
    let mut device_wide: Vec<u16> = OsStr::new(&device_path).encode_wide().collect();
    device_wide.push(0);

    // Open the volume. No write access; share-all so we don't block
    // anyone else.
    // SAFETY: device_wide is NUL-terminated.
    let h = unsafe {
        CreateFileW(
            device_wide.as_ptr(),
            GENERIC_READ,
            FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
            ptr::null_mut(),
            OPEN_EXISTING,
            0,
            ptr::null_mut(),
        )
    };
    if h.is_null() || h == INVALID_HANDLE_VALUE {
        return None;
    }
    // Ensure CloseHandle on every exit path.
    struct HGuard(*mut core::ffi::c_void);
    impl Drop for HGuard {
        fn drop(&mut self) {
            // SAFETY: self.0 was obtained from CreateFileW above.
            unsafe { CloseHandle(self.0) };
        }
    }
    let _guard = HGuard(h);

    // --- Query StorageDeviceProperty (BusType) ---
    let mut query = STORAGE_PROPERTY_QUERY {
        PropertyId: StorageDeviceProperty,
        QueryType: PropertyStandardQuery,
        AdditionalParameters: [0],
    };
    let mut device_buf = [0u8; 1024];
    let mut bytes_returned: u32 = 0;
    // SAFETY: query is a valid STORAGE_PROPERTY_QUERY; device_buf is
    // a stack buffer at least sized to STORAGE_DEVICE_DESCRIPTOR + tail.
    let ok = unsafe {
        DeviceIoControl(
            h,
            IOCTL_STORAGE_QUERY_PROPERTY,
            (&mut query as *mut STORAGE_PROPERTY_QUERY).cast(),
            std::mem::size_of::<STORAGE_PROPERTY_QUERY>() as u32,
            device_buf.as_mut_ptr().cast(),
            device_buf.len() as u32,
            &mut bytes_returned,
            ptr::null_mut(),
        )
    };
    let bus_type = if ok != 0 && bytes_returned >= std::mem::size_of::<STORAGE_DEVICE_DESCRIPTOR>() as u32 {
        // SAFETY: device_buf holds a STORAGE_DEVICE_DESCRIPTOR after a
        // successful IOCTL with sufficient bytes_returned.
        let desc: &STORAGE_DEVICE_DESCRIPTOR = unsafe {
            &*(device_buf.as_ptr() as *const STORAGE_DEVICE_DESCRIPTOR)
        };
        match desc.BusType {
            0x7 => BusType::Usb,
            0x9 => BusType::Iscsi,
            0xB => BusType::Sata,
            0xC => BusType::Sd,
            0xD => BusType::Mmc,
            0xF => BusType::Raid,
            0x11 => BusType::Nvme,
            0x15 => BusType::FileBackedVirtual,
            _ => BusType::Other,
        }
    } else {
        BusType::Other
    };

    // --- Query SeekPenaltyProperty (HDD vs SSD) ---
    let mut query2 = STORAGE_PROPERTY_QUERY {
        PropertyId: StorageDeviceSeekPenaltyProperty,
        QueryType: PropertyStandardQuery,
        AdditionalParameters: [0],
    };
    let mut seek_desc: MaybeUninit<DEVICE_SEEK_PENALTY_DESCRIPTOR> = MaybeUninit::zeroed();
    let mut bytes2: u32 = 0;
    // SAFETY: query2 / seek_desc are properly sized for the IOCTL.
    let ok2 = unsafe {
        DeviceIoControl(
            h,
            IOCTL_STORAGE_QUERY_PROPERTY,
            (&mut query2 as *mut STORAGE_PROPERTY_QUERY).cast(),
            std::mem::size_of::<STORAGE_PROPERTY_QUERY>() as u32,
            seek_desc.as_mut_ptr().cast(),
            std::mem::size_of::<DEVICE_SEEK_PENALTY_DESCRIPTOR>() as u32,
            &mut bytes2,
            ptr::null_mut(),
        )
    };
    let media_class = if ok2 != 0
        && bytes2 >= std::mem::size_of::<DEVICE_SEEK_PENALTY_DESCRIPTOR>() as u32
    {
        // SAFETY: ok2 / bytes2 confirm the descriptor is filled in.
        let desc = unsafe { seek_desc.assume_init() };
        if desc.IncursSeekPenalty != 0 {
            MediaClass::Hdd
        } else {
            MediaClass::Ssd
        }
    } else {
        MediaClass::Unknown
    };

    // --- Query AccessAlignmentProperty (physical sector size) ---
    let mut query3 = STORAGE_PROPERTY_QUERY {
        PropertyId: StorageAccessAlignmentProperty,
        QueryType: PropertyStandardQuery,
        AdditionalParameters: [0],
    };
    let mut align_desc: MaybeUninit<STORAGE_ACCESS_ALIGNMENT_DESCRIPTOR> = MaybeUninit::zeroed();
    let mut bytes3: u32 = 0;
    // SAFETY: query3 / align_desc properly sized.
    let ok3 = unsafe {
        DeviceIoControl(
            h,
            IOCTL_STORAGE_QUERY_PROPERTY,
            (&mut query3 as *mut STORAGE_PROPERTY_QUERY).cast(),
            std::mem::size_of::<STORAGE_PROPERTY_QUERY>() as u32,
            align_desc.as_mut_ptr().cast(),
            std::mem::size_of::<STORAGE_ACCESS_ALIGNMENT_DESCRIPTOR>() as u32,
            &mut bytes3,
            ptr::null_mut(),
        )
    };
    let bytes_per_physical_sector = if ok3 != 0
        && bytes3 >= std::mem::size_of::<STORAGE_ACCESS_ALIGNMENT_DESCRIPTOR>() as u32
    {
        // SAFETY: ok3 / bytes3 confirm fill.
        let desc = unsafe { align_desc.assume_init() };
        if desc.BytesPerPhysicalSector >= 512 && desc.BytesPerPhysicalSector.is_power_of_two() {
            desc.BytesPerPhysicalSector
        } else {
            4096
        }
    } else {
        4096
    };

    Some(VolumeTopology {
        bus_type,
        media_class,
        bytes_per_physical_sector,
    })
}

#[cfg(not(target_os = "windows"))]
fn probe_impl(_path: &Path) -> Option<VolumeTopology> {
    // Non-Windows: stub returning the conservative default. Linux
    // / macOS already get correct buffer sizing through the per-OS
    // copy_file_range / copyfile paths; topology probing is wired
    // for Windows-specific NVMe / USB / RAID / SMB heuristics.
    Some(VolumeTopology::conservative_default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn unc_path_detection() {
        assert!(is_unc_path(Path::new(r"\\server\share")));
        assert!(is_unc_path(Path::new(r"\\server\share\file.txt")));
        assert!(is_unc_path(Path::new(r"\\?\UNC\server\share")));
        assert!(is_unc_path(Path::new("//server/share")));
        // Long-path-prefixed local — NOT a UNC.
        assert!(!is_unc_path(Path::new(r"\\?\C:\foo\bar")));
        // Plain local.
        assert!(!is_unc_path(Path::new(r"C:\foo\bar")));
        assert!(!is_unc_path(Path::new("/usr/local/bin")));
    }

    #[test]
    fn conservative_default_recommends_1mib() {
        let d = VolumeTopology::conservative_default();
        assert_eq!(d.recommended_buffer_bytes(), 1024 * 1024);
        assert_eq!(d.bytes_per_physical_sector, 4096);
    }

    #[test]
    fn hdd_recommends_4mib_qd1() {
        let h = VolumeTopology {
            bus_type: BusType::Sata,
            media_class: MediaClass::Hdd,
            bytes_per_physical_sector: 4096,
        };
        assert_eq!(h.recommended_buffer_bytes(), 4 * 1024 * 1024);
        assert_eq!(h.recommended_queue_depth(), 1);
    }

    #[test]
    fn usb_recommends_low_qd() {
        let u = VolumeTopology {
            bus_type: BusType::Usb,
            media_class: MediaClass::Ssd,
            bytes_per_physical_sector: 4096,
        };
        assert_eq!(u.recommended_queue_depth(), 4);
        assert!(!u.parallel_chunk_friendly());
    }

    #[test]
    fn raid_smb_iscsi_are_parallel_friendly() {
        for bt in [BusType::Raid, BusType::Smb, BusType::Iscsi, BusType::FileBackedVirtual] {
            let t = VolumeTopology {
                bus_type: bt,
                media_class: MediaClass::Ssd,
                bytes_per_physical_sector: 4096,
            };
            assert!(t.parallel_chunk_friendly(), "expected parallel-friendly: {:?}", bt);
        }
    }

    #[cfg(windows)]
    #[test]
    fn probe_does_not_panic_on_local_path() {
        let _ = probe(Path::new("."));
    }

    /// Phase 42 wave-2 — when a thread panics holding the topology
    /// cache lock, the next caller must recover via
    /// `unwrap_or_else(into_inner)` rather than propagating the
    /// poison. Without that, every subsequent `probe()` would re-run
    /// the 2 ms IOCTL forever, defeating the cache and adding latency
    /// to every per-file copy in a 10 000-file tree.
    #[cfg(windows)]
    #[test]
    fn probe_recovers_from_poisoned_cache_lock() {
        use std::panic;
        use std::thread;

        // Step 1: poison the cache by panicking inside a guard.
        // We use `catch_unwind` so the test process keeps running.
        let result = thread::spawn(|| {
            let cache_ref = super::cache();
            let _guard = cache_ref.lock().expect("first lock");
            panic!("intentional poison");
        })
        .join();
        assert!(result.is_err(), "the poison thread must have panicked");
        assert!(
            super::cache().is_poisoned(),
            "cache mutex must be poisoned after the panicking thread released it"
        );

        // Step 2: probe from a fresh thread; the wave-1 fix must
        // recover, not re-panic. Use `catch_unwind` to assert that
        // probe returns rather than unwinds.
        let probe_outcome = panic::catch_unwind(|| {
            // Use `.` so we don't require any specific volume to
            // exist; probe walks up to the volume root.
            probe(Path::new("."))
        });
        let topo_opt = probe_outcome
            .expect("probe must not panic when the cache lock is poisoned");

        // Recovery means we still get a sensible answer back. On
        // Windows local paths the probe either succeeds (live volume)
        // or returns the conservative default — either way it's
        // `Some(...)`, never `None` for a local path.
        assert!(
            topo_opt.is_some(),
            "probe should return Some(VolumeTopology) after poison recovery"
        );

        // Defensive sanity check on the returned topology: the
        // physical sector size must be a power of two ≥ 512 (the
        // probe normalises this; conservative default is 4096).
        let topo = topo_opt.unwrap();
        assert!(
            topo.bytes_per_physical_sector >= 512
                && topo.bytes_per_physical_sector.is_power_of_two(),
            "post-recovery topology has invalid sector size: {}",
            topo.bytes_per_physical_sector
        );
    }
}
