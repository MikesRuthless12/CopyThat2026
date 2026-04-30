//! Phase 44.3b — Windows whole-drive capability probe via
//! `IOCTL_STORAGE_QUERY_PROPERTY`.
//!
//! Returns the device's vendor / product / serial strings plus the
//! TRIM-support bit. Used by `copythat-secure-delete::WindowsSanitizeHelper`
//! to populate `SanitizeCapabilities` without falling back to the
//! Phase 44.2 stub. The actual destructive paths (TCG OPAL crypto-
//! erase via `IOCTL_STORAGE_SECURITY_PROTOCOL_OUT`) defer to Phase
//! 44.4 because they need hardware-validation on a real
//! Self-Encrypting Drive.
//!
//! On non-Windows this module is a no-op; the public API returns
//! `None` so callers keep their existing fallback behaviour.

use std::path::Path;

/// Phase 44.3b — minimal device descriptor returned by the
/// capability probe. `model` is the user-pickable string the UI
/// renders next to the path; `trim_supported` is the answer to
/// "would a TRIM ioctl be honoured by this device".
///
/// Phase 44.4a extends this with `nvme_sanicap` — when `Some`, the
/// raw 32-bit SANICAP value from the NVMe identify-controller
/// response (NVM Express §5.15.2.2). Bits 0-2 encode crypto-erase /
/// block-erase / overwrite support; [`nvme_sanicap_modes`] decodes
/// them into the typed mode-name list the UI consumes.
#[derive(Debug, Clone, Default)]
pub struct WindowsDeviceInfo {
    /// Product / model string (e.g., "Samsung SSD 990 PRO 2TB").
    pub model: String,
    /// Vendor string (e.g., "Samsung"). Often empty on consumer NVMe
    /// drives where the vendor field is the same as model.
    pub vendor: String,
    /// Serial number string when the controller reports one.
    pub serial: String,
    /// Whether `IOCTL_STORAGE_TRIM` is supported. Read from
    /// `DEVICE_TRIM_DESCRIPTOR` (`StorageDeviceTrimProperty`).
    pub trim_supported: bool,
    /// Phase 44.4a — raw SANICAP from the NVMe identify-controller
    /// response (NVM Express §5.15.2.2). `None` for non-NVMe
    /// devices or when the IOCTL probe failed.
    pub nvme_sanicap: Option<u32>,
}

/// Phase 44.4a — decode a SANICAP value into the supported NVMe
/// sanitize mode-name strings (matches the wire-stable strings
/// `copythat-secure-delete::SsdSanitizeMode::name()` returns).
/// `NvmeFormat` is always supported on NVMe controllers (NVM
/// Express §5.14 FSE bit) so it's added unconditionally when
/// SANICAP is present.
pub fn nvme_sanicap_modes(sanicap: u32) -> Vec<&'static str> {
    let mut out: Vec<&'static str> = Vec::new();
    if sanicap & 0b001 != 0 {
        out.push("nvme-sanitize-crypto");
    }
    if sanicap & 0b010 != 0 {
        out.push("nvme-sanitize-block");
    }
    out.push("nvme-format");
    out
}

/// Phase 44.3b — probe the device at `device_path` (typically
/// `\\.\PhysicalDriveN`) and return its model / vendor / serial /
/// trim bit. Returns `None` when:
/// - The platform is not Windows.
/// - The device path can't be opened (doesn't exist, access
///   denied, etc.).
/// - The IOCTL fails or returns malformed bytes.
///
/// This is read-only — no elevation required for opening
/// `\\.\PhysicalDriveN` with `GENERIC_READ` (Windows enforces this
/// at the I/O Manager layer; standard users get the device handle
/// in read-only mode).
pub fn windows_query_device_info(device_path: &Path) -> Option<WindowsDeviceInfo> {
    win_query(device_path)
}

#[cfg(target_os = "windows")]
fn win_query(device_path: &Path) -> Option<WindowsDeviceInfo> {
    use std::ffi::OsStr;
    use std::mem::MaybeUninit;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr;

    use windows_sys::Win32::Foundation::{CloseHandle, GENERIC_READ, INVALID_HANDLE_VALUE};
    use windows_sys::Win32::Storage::FileSystem::{
        CreateFileW, FILE_SHARE_DELETE, FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,
    };
    use windows_sys::Win32::System::IO::DeviceIoControl;
    use windows_sys::Win32::System::Ioctl::{
        DEVICE_TRIM_DESCRIPTOR, IOCTL_STORAGE_QUERY_PROPERTY, PropertyStandardQuery,
        STORAGE_DEVICE_DESCRIPTOR, STORAGE_PROPERTY_QUERY, StorageDeviceProperty,
        StorageDeviceTrimProperty,
    };

    // Open the device read-only. `\\.\PhysicalDriveN` accepts
    // GENERIC_READ from any user.
    let mut wide: Vec<u16> = OsStr::new(device_path).encode_wide().collect();
    wide.push(0);
    // SAFETY: wide is NUL-terminated; CreateFileW reads it as a
    // UTF-16 string.
    let h = unsafe {
        CreateFileW(
            wide.as_ptr(),
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

    let mut info = WindowsDeviceInfo::default();

    // --- StorageDeviceProperty: vendor / product / serial ---
    let mut query = STORAGE_PROPERTY_QUERY {
        PropertyId: StorageDeviceProperty,
        QueryType: PropertyStandardQuery,
        AdditionalParameters: [0],
    };
    // 1024 bytes is enough for the descriptor + the variable-length
    // tail that holds the strings. Keep on the stack.
    let mut buf = [0u8; 1024];
    let mut returned: u32 = 0;
    // SAFETY: query is a valid STORAGE_PROPERTY_QUERY; buf is a
    // 1024-byte stack buffer well in excess of STORAGE_DEVICE_DESCRIPTOR.
    let ok = unsafe {
        DeviceIoControl(
            h,
            IOCTL_STORAGE_QUERY_PROPERTY,
            (&mut query as *mut STORAGE_PROPERTY_QUERY).cast(),
            std::mem::size_of::<STORAGE_PROPERTY_QUERY>() as u32,
            buf.as_mut_ptr().cast(),
            buf.len() as u32,
            &mut returned,
            ptr::null_mut(),
        )
    };
    if ok != 0 && returned >= std::mem::size_of::<STORAGE_DEVICE_DESCRIPTOR>() as u32 {
        // SAFETY: ok + returned confirm the descriptor is filled in;
        // the strings live at the byte offsets the descriptor
        // points to (inside the same buf).
        let desc: &STORAGE_DEVICE_DESCRIPTOR =
            unsafe { &*(buf.as_ptr() as *const STORAGE_DEVICE_DESCRIPTOR) };
        info.vendor = read_offset_string(&buf, desc.VendorIdOffset as usize);
        info.model = read_offset_string(&buf, desc.ProductIdOffset as usize);
        info.serial = read_offset_string(&buf, desc.SerialNumberOffset as usize);
    }

    // --- StorageDeviceTrimProperty: trim_supported ---
    let mut trim_query = STORAGE_PROPERTY_QUERY {
        PropertyId: StorageDeviceTrimProperty,
        QueryType: PropertyStandardQuery,
        AdditionalParameters: [0],
    };
    let mut trim_desc: MaybeUninit<DEVICE_TRIM_DESCRIPTOR> = MaybeUninit::zeroed();
    let mut trim_returned: u32 = 0;
    // SAFETY: trim_query is valid; trim_desc is MaybeUninit::zeroed
    // and exactly DEVICE_TRIM_DESCRIPTOR-sized.
    let ok = unsafe {
        DeviceIoControl(
            h,
            IOCTL_STORAGE_QUERY_PROPERTY,
            (&mut trim_query as *mut STORAGE_PROPERTY_QUERY).cast(),
            std::mem::size_of::<STORAGE_PROPERTY_QUERY>() as u32,
            trim_desc.as_mut_ptr().cast(),
            std::mem::size_of::<DEVICE_TRIM_DESCRIPTOR>() as u32,
            &mut trim_returned,
            ptr::null_mut(),
        )
    };
    if ok != 0 && trim_returned >= std::mem::size_of::<DEVICE_TRIM_DESCRIPTOR>() as u32 {
        // SAFETY: ok + trim_returned confirm the struct is filled in.
        let desc = unsafe { trim_desc.assume_init() };
        info.trim_supported = desc.TrimEnabled != 0;
    }

    // --- Phase 44.4a: NVMe SANICAP via StorageAdapterProtocolSpecificProperty ---
    info.nvme_sanicap = win_query_nvme_sanicap(h);

    Some(info)
}

/// Phase 44.4a — query NVMe SANICAP via
/// `IOCTL_STORAGE_QUERY_PROPERTY` with
/// `StorageAdapterProtocolSpecificProperty` and an NVMe Identify
/// Controller request. SANICAP is at byte offset 331-334 of the
/// 4096-byte identify-controller response (NVM Express §5.15.2.2,
/// little-endian u32).
///
/// Returns `None` for:
/// - non-NVMe devices (`ProtocolDataLength` returns 0 or IOCTL fails)
/// - older Windows versions that don't expose the protocol-
///   specific query (Windows 8.1+ has it; Windows 7 returns
///   ERROR_INVALID_FUNCTION which we map to None)
/// - drivers that don't honour the request (some USB-NVMe bridge
///   firmware in particular)
///
/// The request buffer is a `STORAGE_PROPERTY_QUERY` header
/// followed by a `STORAGE_PROTOCOL_SPECIFIC_DATA` payload — built
/// here from raw bytes because the windows-sys 0.59 bindings for
/// `STORAGE_PROTOCOL_SPECIFIC_DATA` are present but the field
/// names vary across Windows SDK versions; using raw bytes pins
/// the layout to what the kernel ABI actually wants.
#[cfg(target_os = "windows")]
fn win_query_nvme_sanicap(h: *mut core::ffi::c_void) -> Option<u32> {
    use std::ptr;
    use windows_sys::Win32::System::IO::DeviceIoControl;
    use windows_sys::Win32::System::Ioctl::IOCTL_STORAGE_QUERY_PROPERTY;

    // Constants from `ntddstor.h` / `ntddstor.h` Windows SDK:
    //   StorageAdapterProtocolSpecificProperty = 49
    //   PropertyStandardQuery = 0
    //   ProtocolTypeNvme = 3
    //   NVMeDataTypeIdentify = 1
    //   NVME_IDENTIFY_CNS_CONTROLLER = 1
    const STORAGE_ADAPTER_PROTOCOL_SPECIFIC_PROPERTY: u32 = 49;
    const PROPERTY_STANDARD_QUERY: u32 = 0;
    const PROTOCOL_TYPE_NVME: u32 = 3;
    const NVME_DATA_TYPE_IDENTIFY: u32 = 1;
    const NVME_IDENTIFY_CNS_CONTROLLER: u32 = 1;
    const NVME_IDENTIFY_RESPONSE_LEN: usize = 4096;
    const SANICAP_OFFSET_IN_IDENTIFY: usize = 331;

    // Build the request buffer:
    //   STORAGE_PROPERTY_QUERY (8 bytes header)
    //   STORAGE_PROTOCOL_SPECIFIC_DATA (40 bytes)
    // The kernel reads these as a single contiguous structure.
    // Total request size: 48 bytes.
    let mut request = [0u8; 48];
    // STORAGE_PROPERTY_QUERY
    request[0..4].copy_from_slice(&STORAGE_ADAPTER_PROTOCOL_SPECIFIC_PROPERTY.to_le_bytes());
    request[4..8].copy_from_slice(&PROPERTY_STANDARD_QUERY.to_le_bytes());
    // STORAGE_PROTOCOL_SPECIFIC_DATA
    request[8..12].copy_from_slice(&PROTOCOL_TYPE_NVME.to_le_bytes());
    request[12..16].copy_from_slice(&NVME_DATA_TYPE_IDENTIFY.to_le_bytes());
    request[16..20].copy_from_slice(&NVME_IDENTIFY_CNS_CONTROLLER.to_le_bytes());
    // ProtocolDataRequestSubValue = 0 (controller identify, namespace 0)
    // bytes 20..24 already 0
    // ProtocolDataOffset = sizeof(STORAGE_PROTOCOL_DATA_DESCRIPTOR) = 40
    request[24..28].copy_from_slice(&40_u32.to_le_bytes());
    // ProtocolDataLength = 4096 (NVMe identify response size)
    request[28..32].copy_from_slice(&4096_u32.to_le_bytes());
    // FixedProtocolReturnData = 0 (we want the full response)
    // bytes 32..36 already 0
    // ProtocolDataRequestSubValue 2/3/4 = 0
    // bytes 36..48 already 0

    // Output buffer: STORAGE_PROTOCOL_DATA_DESCRIPTOR (40 bytes) +
    // identify-controller response (4096 bytes).
    const STORAGE_PROTOCOL_DATA_DESCRIPTOR_LEN: usize = 40;
    let mut response = vec![0u8; STORAGE_PROTOCOL_DATA_DESCRIPTOR_LEN + NVME_IDENTIFY_RESPONSE_LEN];
    let mut returned: u32 = 0;
    // SAFETY: request is a 48-byte buffer matching the kernel's
    // expected STORAGE_PROPERTY_QUERY + STORAGE_PROTOCOL_SPECIFIC_DATA
    // layout; response is large enough for the descriptor + identify
    // payload.
    let ok = unsafe {
        DeviceIoControl(
            h,
            IOCTL_STORAGE_QUERY_PROPERTY,
            request.as_mut_ptr().cast(),
            request.len() as u32,
            response.as_mut_ptr().cast(),
            response.len() as u32,
            &mut returned,
            ptr::null_mut(),
        )
    };
    if ok == 0 || (returned as usize) < STORAGE_PROTOCOL_DATA_DESCRIPTOR_LEN + 332 {
        return None;
    }
    // Skip the descriptor; the identify response starts at byte 40.
    let identify_start = STORAGE_PROTOCOL_DATA_DESCRIPTOR_LEN;
    if identify_start + SANICAP_OFFSET_IN_IDENTIFY + 4 > response.len() {
        return None;
    }
    let bytes = &response[identify_start + SANICAP_OFFSET_IN_IDENTIFY
        ..identify_start + SANICAP_OFFSET_IN_IDENTIFY + 4];
    let mut sanicap_bytes = [0u8; 4];
    sanicap_bytes.copy_from_slice(bytes);
    Some(u32::from_le_bytes(sanicap_bytes))
}

#[cfg(not(target_os = "windows"))]
#[allow(dead_code)]
fn win_query_nvme_sanicap(_h: *mut core::ffi::c_void) -> Option<u32> {
    None
}

#[cfg(target_os = "windows")]
fn read_offset_string(buf: &[u8], offset: usize) -> String {
    if offset == 0 || offset >= buf.len() {
        return String::new();
    }
    // The string is NUL-terminated within the buffer's tail.
    let tail = &buf[offset..];
    let end = tail.iter().position(|&b| b == 0).unwrap_or(tail.len());
    String::from_utf8_lossy(&tail[..end]).trim().to_string()
}

#[cfg(not(target_os = "windows"))]
fn win_query(_device_path: &Path) -> Option<WindowsDeviceInfo> {
    None
}

/// Phase 44.3c — enumerate Windows physical drives by probing
/// `\\.\PhysicalDrive0` through `\\.\PhysicalDriveN` until
/// `CreateFileW` returns ERROR_FILE_NOT_FOUND. Returns the list of
/// device paths that opened successfully.
///
/// Capped at 32 drives — typical workstations have 1-4 physical
/// drives; 32 is the highest disk number Windows' physical-drive
/// namespace exposes by convention. Servers with more drives can
/// extend the cap.
///
/// On non-Windows returns an empty list.
pub fn windows_enumerate_physical_drives() -> Vec<String> {
    enum_windows()
}

#[cfg(target_os = "windows")]
fn enum_windows() -> Vec<String> {
    // Phase 44.3 post-review (M1) — break on 4 consecutive misses
    // instead of always probing 32 indices. CreateFileW on a
    // present-but-parked SATA HDD or USB-bridge NVMe spins it up;
    // the prior 32-iteration loop woke every external sleeping
    // drive on every enumeration. 4 consecutive misses is enough
    // to handle PhysicalDrive2 + PhysicalDrive3 missing while
    // PhysicalDrive4 is the next real device — covers the typical
    // workstation layout (1-4 drives, sometimes with gaps in the
    // numbering after USB unplugs) without burning latency on a
    // server with sparse drive numbering.
    let mut out = Vec::new();
    let mut consecutive_misses = 0u32;
    for n in 0..32u32 {
        let path = format!(r"\\.\PhysicalDrive{n}");
        let p = std::path::PathBuf::from(&path);
        if win_query(&p).is_some() {
            out.push(path);
            consecutive_misses = 0;
        } else {
            consecutive_misses += 1;
            if consecutive_misses >= 4 {
                break;
            }
        }
    }
    out
}

#[cfg(not(target_os = "windows"))]
fn enum_windows() -> Vec<String> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nvme_sanicap_modes_decodes_crypto_only() {
        let modes = nvme_sanicap_modes(0b001);
        assert_eq!(modes, vec!["nvme-sanitize-crypto", "nvme-format"]);
    }

    #[test]
    fn nvme_sanicap_modes_decodes_block_only() {
        let modes = nvme_sanicap_modes(0b010);
        assert_eq!(modes, vec!["nvme-sanitize-block", "nvme-format"]);
    }

    #[test]
    fn nvme_sanicap_modes_decodes_both_sanitize_modes() {
        let modes = nvme_sanicap_modes(0b011);
        assert_eq!(
            modes,
            vec!["nvme-sanitize-crypto", "nvme-sanitize-block", "nvme-format"]
        );
    }

    #[test]
    fn nvme_sanicap_modes_zero_returns_only_format() {
        // SANICAP=0 means no NVMe Sanitize support; nvme-format is
        // always available as a fallback (it's a separate command,
        // not gated by SANICAP bits — see NVM Express §5.14).
        let modes = nvme_sanicap_modes(0);
        assert_eq!(modes, vec!["nvme-format"]);
    }

    #[test]
    fn nvme_sanicap_modes_ignores_reserved_bits() {
        // Bits 3-31 are reserved per NVMe spec; the decoder must
        // not surface garbage modes if the controller reports
        // anything in them. Probe with bit 7 set.
        let modes = nvme_sanicap_modes(0b1000_0000);
        assert_eq!(modes, vec!["nvme-format"]);
    }
}
