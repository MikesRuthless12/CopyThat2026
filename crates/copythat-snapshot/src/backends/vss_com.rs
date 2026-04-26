//! Phase 17 follow-up — direct `IVssBackupComponents` COM port.
//!
//! Replaces the PowerShell + WMI shellouts in [`super::vss`] with
//! native `IVssBackupComponents` calls via the `winapi` 0.3
//! bindings. Two wins over the PowerShell path:
//!
//! 1. **No format-string interpolation.** Every value crosses the
//!    boundary as a typed COM argument; there's no shell to escape
//!    against, so the validate-then-interpolate pattern is gone.
//! 2. **No PowerShell startup tax.** Each shadow create/release
//!    on the PowerShell path costs ~300–700 ms of `powershell.exe`
//!    boot before any work runs. The COM path enters VSS directly.
//!
//! Gated behind the `vss-com` feature flag. The `vss.rs` code
//! still picks the PowerShell path by default; flip via
//! `--features vss-com` once a Windows VSS test environment has
//! verified the COM bindings work end-to-end. The
//! `tests/lock_file.ps1` + `tests/vss_com_smoke.ps1` scripts in
//! the repo root drive that verification.
//!
//! ## What this module does NOT cover
//!
//! - **Writer involvement.** A "real" VSS backup notifies every
//!   registered VSS writer (`GatherWriterMetadata` + per-writer
//!   PrepareForBackup callbacks) so apps like SQL Server can
//!   freeze their journals before the snapshot fires. CopyThat
//!   only needs file-system-consistent reads; we set
//!   `bSelectComponents=false` and skip the writer dance entirely.
//!   Microsoft documents this shape as supported for "no-writer"
//!   backup callers (the same surface `Get-WmiObject
//!   Win32_ShadowCopy::Create` uses under the hood).
//! - **Persistent shadows.** Context is `VSS_CTX_APP_ROLLBACK`
//!   (`PERSISTENT | NO_AUTO_RELEASE`) so the shadow survives
//!   `IVssBackupComponents::Release` at the end of
//!   `create_shadow_via_com`. `release_shadow_via_com` opens a
//!   fresh `IVssBackupComponents` and calls `DeleteSnapshots`
//!   to tear the shadow down. The non-persistent `VSS_CTX_BACKUP`
//!   would auto-release on instance teardown — the resulting
//!   `VSS_E_OBJECT_NOT_FOUND` (HRESULT 0x80042308) on the second
//!   call surfaced this on a real Windows admin run.
//! - **Mount points.** The returned `\\?\GLOBALROOT\Device\HarddiskVolumeShadowCopyN`
//!   path is read directly; we don't expose a drive-letter mount.

#![cfg(all(windows, feature = "vss-com"))]

use std::ptr;
use std::sync::Once;

use winapi::shared::guiddef::GUID;
use winapi::shared::minwindef::TRUE;
use winapi::shared::ntdef::LONG;
use winapi::shared::winerror::{HRESULT, S_FALSE, S_OK};
use winapi::um::combaseapi::{CoInitializeEx, CoInitializeSecurity};
use winapi::um::objbase::COINIT_MULTITHREADED;
use winapi::um::objidlbase::EOAC_NONE;
use winapi::um::vsbackup::{
    CreateVssBackupComponents, IVssBackupComponents, VssFreeSnapshotProperties,
};
use winapi::um::vss::{
    IVssAsync, VSS_BT_FULL, VSS_CTX_APP_ROLLBACK, VSS_OBJECT_SNAPSHOT, VSS_SNAPSHOT_PROP,
};
use winapi::um::vsserror::VSS_S_ASYNC_FINISHED;

/// Default `IVssAsync::Wait` timeout for `PrepareForBackup` /
/// `DoSnapshotSet` polls. VSS calls into every registered
/// writer; on a busy system this can take several seconds.
/// 60 s matches the upper end of typical Windows backup tooling.
const ASYNC_WAIT_TIMEOUT_MS: u32 = 60_000;

/// Per-process / per-thread COM init guard. CoInitializeEx must be
/// called on every thread that uses COM; calling it on an already-
/// initialised thread returns `S_FALSE` which we treat as success.
/// `CoInitializeSecurity` is process-global and must run before any
/// COM activation; the `Once` guard makes the call idempotent.
static SECURITY_INIT: Once = Once::new();

/// RAII wrapper that releases an `IVssBackupComponents` interface
/// pointer on Drop. Using a wrapper avoids the manual ref-counting
/// dance every early-return would otherwise need.
struct BackupComponents {
    ptr: *mut IVssBackupComponents,
}

impl Drop for BackupComponents {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                ((*(*self.ptr).lpVtbl).parent.Release)(self.ptr as *mut _);
            }
            self.ptr = ptr::null_mut();
        }
    }
}

/// RAII wrapper that releases an `IVssAsync` on Drop.
struct AsyncOp {
    ptr: *mut IVssAsync,
}

impl Drop for AsyncOp {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                ((*(*self.ptr).lpVtbl).parent.Release)(self.ptr as *mut _);
            }
            self.ptr = ptr::null_mut();
        }
    }
}

/// RAII wrapper that calls `VssFreeSnapshotProperties` on Drop. The
/// VSS API allocates the strings inside `VSS_SNAPSHOT_PROP` from
/// the COM allocator; the caller must release them.
struct SnapshotProperties {
    props: VSS_SNAPSHOT_PROP,
    populated: bool,
}

impl Drop for SnapshotProperties {
    fn drop(&mut self) {
        if self.populated {
            unsafe { VssFreeSnapshotProperties(&mut self.props) };
            self.populated = false;
        }
    }
}

/// Initialise COM on the current thread + the process security
/// model on first call. Idempotent; subsequent calls are no-ops.
fn ensure_com_initialised() -> Result<(), String> {
    // SAFETY: CoInitializeEx is documented to be safe to call
    // multiple times per thread; the second-and-later calls
    // return S_FALSE which means "already initialised in this
    // mode", which is what we want.
    let hr =
        unsafe { CoInitializeEx(ptr::null_mut(), COINIT_MULTITHREADED) };
    if hr != S_OK && hr != S_FALSE {
        // RPC_E_CHANGED_MODE (0x80010106) — another part of the
        // process initialised COM in STA. We can still proceed
        // (the apartment is fine for VSS) but flag the case.
        if hr as u32 != 0x8001_0106 {
            return Err(format!("CoInitializeEx hr=0x{:x}", hr as u32));
        }
    }
    // CoInitializeSecurity is process-global; once is enough.
    // VSS docs recommend RPC_C_AUTHN_LEVEL_PKT_PRIVACY +
    // RPC_C_IMP_LEVEL_IDENTIFY for the security descriptor.
    SECURITY_INIT.call_once(|| {
        const RPC_C_AUTHN_LEVEL_PKT_PRIVACY: u32 = 6;
        const RPC_C_IMP_LEVEL_IDENTIFY: u32 = 2;
        // SAFETY: passing all-NULL/default args except the auth
        // level + impersonation level is the documented
        // recommended invocation for VSS clients.
        let hr = unsafe {
            CoInitializeSecurity(
                ptr::null_mut(),
                -1,
                ptr::null_mut(),
                ptr::null_mut(),
                RPC_C_AUTHN_LEVEL_PKT_PRIVACY,
                RPC_C_IMP_LEVEL_IDENTIFY,
                ptr::null_mut(),
                EOAC_NONE,
                ptr::null_mut(),
            )
        };
        // `RPC_E_TOO_LATE` (0x80010119) is benign — another
        // component already set process-wide security. We can
        // still operate.
        if hr != S_OK && (hr as u32) != 0x8001_0119 {
            // Don't fail the once-call (we'd retry forever);
            // log the error and move on. A subsequent VSS
            // call will surface the real failure if security
            // is genuinely wrong.
            eprintln!("[vss-com] CoInitializeSecurity hr=0x{:x}", hr as u32);
        }
    });
    Ok(())
}

/// Wait for an IVssAsync to complete. Returns Ok when status is
/// `VSS_S_ASYNC_FINISHED` or `S_OK`; an Err otherwise carrying
/// the underlying HRESULT.
fn wait_for_async(op: &AsyncOp, label: &str) -> Result<(), String> {
    if op.ptr.is_null() {
        return Err(format!("{label}: null IVssAsync"));
    }
    // SAFETY: vtable accessor on a non-null IVssAsync; all
    // arguments are stack-locals + ptr::null_mut where allowed.
    let hr = unsafe { ((*(*op.ptr).lpVtbl).Wait)(op.ptr, ASYNC_WAIT_TIMEOUT_MS) };
    if hr != S_OK {
        return Err(format!("{label}: Wait hr=0x{:x}", hr as u32));
    }
    let mut status: HRESULT = 0;
    let mut reserved: HRESULT = 0;
    let hr = unsafe {
        ((*(*op.ptr).lpVtbl).QueryStatus)(op.ptr, &mut status, &mut reserved)
    };
    if hr != S_OK {
        return Err(format!("{label}: QueryStatus hr=0x{:x}", hr as u32));
    }
    if status != S_OK && status != VSS_S_ASYNC_FINISHED {
        return Err(format!(
            "{label}: async finished with status=0x{:x}",
            status as u32
        ));
    }
    Ok(())
}

/// Format a Windows GUID into the brace-wrapped uppercase form WMI
/// hands back from `Win32_ShadowCopy.ID` so the COM port returns
/// the same shape the PowerShell path does:
/// `{XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX}`.
fn format_guid(guid: &GUID) -> String {
    format!(
        "{{{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}}}",
        guid.Data1,
        guid.Data2,
        guid.Data3,
        guid.Data4[0],
        guid.Data4[1],
        guid.Data4[2],
        guid.Data4[3],
        guid.Data4[4],
        guid.Data4[5],
        guid.Data4[6],
        guid.Data4[7],
    )
}

/// Inverse of [`format_guid`]. Accepts the canonical
/// `{XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX}` shape (case-
/// insensitive); refuses anything else.
fn parse_guid(s: &str) -> Result<GUID, String> {
    let bytes = s.as_bytes();
    if bytes.len() != 38 || bytes[0] != b'{' || bytes[37] != b'}' {
        return Err(format!("malformed GUID {s:?}: missing braces or wrong length"));
    }
    let inner = &s[1..37];
    let parts: Vec<&str> = inner.split('-').collect();
    if parts.len() != 5
        || parts[0].len() != 8
        || parts[1].len() != 4
        || parts[2].len() != 4
        || parts[3].len() != 4
        || parts[4].len() != 12
    {
        return Err(format!("malformed GUID {s:?}: wrong segment lengths"));
    }
    let data1 = u32::from_str_radix(parts[0], 16)
        .map_err(|e| format!("GUID Data1: {e}"))?;
    let data2 = u16::from_str_radix(parts[1], 16)
        .map_err(|e| format!("GUID Data2: {e}"))?;
    let data3 = u16::from_str_radix(parts[2], 16)
        .map_err(|e| format!("GUID Data3: {e}"))?;
    let mut data4 = [0u8; 8];
    let part3_bytes: Vec<u8> = (0..2)
        .map(|i| {
            u8::from_str_radix(&parts[3][i * 2..i * 2 + 2], 16)
                .map_err(|e| format!("GUID Data4[{i}]: {e}"))
        })
        .collect::<Result<_, _>>()?;
    let part4_bytes: Vec<u8> = (0..6)
        .map(|i| {
            u8::from_str_radix(&parts[4][i * 2..i * 2 + 2], 16)
                .map_err(|e| format!("GUID Data4[{}]: {e}", i + 2))
        })
        .collect::<Result<_, _>>()?;
    data4[..2].copy_from_slice(&part3_bytes);
    data4[2..].copy_from_slice(&part4_bytes);
    Ok(GUID {
        Data1: data1,
        Data2: data2,
        Data3: data3,
        Data4: data4,
    })
}

/// UTF-8 → wide NUL-terminated, for VSS APIs that take `*mut WCHAR`.
fn to_wide_zstr(s: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
}

/// Read a Windows wide string out of a `*const u16` until the
/// first NUL. Used for `m_pwszSnapshotDeviceObject`.
///
/// SAFETY: caller must ensure `p` is either null or points at a
/// NUL-terminated UTF-16 sequence whose memory remains valid for
/// the read.
unsafe fn wide_to_string(p: *const u16) -> String {
    if p.is_null() {
        return String::new();
    }
    let mut len = 0usize;
    // SAFETY: we walk forward until the first 0, capped at 32 KiB.
    // The cap defends against a misbehaving wide buffer with no
    // NUL terminator; VSS device paths are well under 1 KiB.
    unsafe {
        while *p.add(len) != 0 {
            len += 1;
            if len > 32 * 1024 {
                break;
            }
        }
        let slice = std::slice::from_raw_parts(p, len);
        String::from_utf16_lossy(slice)
    }
}

/// Mint a non-persistent client-accessible shadow of `volume`.
/// Returns `(shadow_id, device_path)` matching the PowerShell
/// path's wire shape so callers can swap implementations without
/// changing parsing code.
///
/// `volume` must be the canonical `[A-Za-z]:\` form. Caller is
/// responsible for validating; this function trusts the input.
#[allow(dead_code)]
pub fn create_shadow_via_com(volume: &str) -> Result<(String, String), String> {
    ensure_com_initialised()?;

    // 1. Create the IVssBackupComponents instance.
    let mut backup_ptr: *mut IVssBackupComponents = ptr::null_mut();
    // SAFETY: CreateVssBackupComponents is a documented exported
    // function; passing &mut ptr to receive the new interface is
    // its standard contract.
    let hr = unsafe { CreateVssBackupComponents(&mut backup_ptr) };
    if hr != S_OK {
        return Err(format!("CreateVssBackupComponents hr=0x{:x}", hr as u32));
    }
    let backup = BackupComponents { ptr: backup_ptr };
    if backup.ptr.is_null() {
        return Err("CreateVssBackupComponents returned null pointer".into());
    }

    // 2. InitializeForBackup(NULL).
    let hr = unsafe {
        ((*(*backup.ptr).lpVtbl).InitializeForBackup)(backup.ptr, ptr::null_mut())
    };
    if hr != S_OK {
        return Err(format!("InitializeForBackup hr=0x{:x}", hr as u32));
    }

    // 3. SetBackupState(false, true, VSS_BT_FULL, false). VSS
    // requires this even for no-writer backups; it sets the
    // metadata the snapshot context interprets.
    // winapi 0.3's IVssBackupComponents::SetBackupState takes Rust
    // `bool` (not the `BOOL` typedef the docs reference); pass the
    // expected boolean values directly.
    let hr = unsafe {
        ((*(*backup.ptr).lpVtbl).SetBackupState)(
            backup.ptr,
            false, // bSelectComponents — no per-writer component picking
            true,  // bBackupBootableSystemState
            VSS_BT_FULL,
            false, // bPartialFileSupport
        )
    };
    if hr != S_OK {
        return Err(format!("SetBackupState hr=0x{:x}", hr as u32));
    }

    // 4. SetContext(VSS_CTX_APP_ROLLBACK).
    //    Persistent + no-auto-release so the shadow outlives this
    //    `IVssBackupComponents` instance. `release_shadow_via_com`
    //    explicitly tears it down with DeleteSnapshots.
    let hr = unsafe {
        ((*(*backup.ptr).lpVtbl).SetContext)(backup.ptr, VSS_CTX_APP_ROLLBACK as LONG)
    };
    if hr != S_OK {
        return Err(format!("SetContext hr=0x{:x}", hr as u32));
    }

    // 5. StartSnapshotSet.
    let mut snapshot_set_id: GUID = unsafe { std::mem::zeroed() };
    let hr = unsafe {
        ((*(*backup.ptr).lpVtbl).StartSnapshotSet)(backup.ptr, &mut snapshot_set_id)
    };
    if hr != S_OK {
        return Err(format!("StartSnapshotSet hr=0x{:x}", hr as u32));
    }

    // 6. AddToSnapshotSet(volume, GUID_NULL, &shadow_id).
    let mut shadow_id: GUID = unsafe { std::mem::zeroed() };
    let provider_id: GUID = unsafe { std::mem::zeroed() }; // GUID_NULL
    let mut wide_volume = to_wide_zstr(volume);
    let hr = unsafe {
        ((*(*backup.ptr).lpVtbl).AddToSnapshotSet)(
            backup.ptr,
            wide_volume.as_mut_ptr(),
            provider_id,
            &mut shadow_id,
        )
    };
    if hr != S_OK {
        return Err(format!("AddToSnapshotSet hr=0x{:x}", hr as u32));
    }

    // 7. PrepareForBackup — async; poll until done.
    let mut prepare_async: *mut IVssAsync = ptr::null_mut();
    let hr = unsafe {
        ((*(*backup.ptr).lpVtbl).PrepareForBackup)(backup.ptr, &mut prepare_async)
    };
    if hr != S_OK {
        return Err(format!("PrepareForBackup hr=0x{:x}", hr as u32));
    }
    let prepare = AsyncOp { ptr: prepare_async };
    wait_for_async(&prepare, "PrepareForBackup")?;
    drop(prepare); // explicit so the IVssAsync releases now

    // 8. DoSnapshotSet — async; poll until done.
    let mut do_async: *mut IVssAsync = ptr::null_mut();
    let hr = unsafe {
        ((*(*backup.ptr).lpVtbl).DoSnapshotSet)(backup.ptr, &mut do_async)
    };
    if hr != S_OK {
        return Err(format!("DoSnapshotSet hr=0x{:x}", hr as u32));
    }
    let do_op = AsyncOp { ptr: do_async };
    wait_for_async(&do_op, "DoSnapshotSet")?;
    drop(do_op);

    // 9. GetSnapshotProperties → device path.
    let mut props_holder = SnapshotProperties {
        props: unsafe { std::mem::zeroed() },
        populated: false,
    };
    let hr = unsafe {
        ((*(*backup.ptr).lpVtbl).GetSnapshotProperties)(
            backup.ptr,
            shadow_id,
            &mut props_holder.props,
        )
    };
    if hr != S_OK {
        return Err(format!("GetSnapshotProperties hr=0x{:x}", hr as u32));
    }
    props_holder.populated = true;

    let device_path = unsafe {
        wide_to_string(props_holder.props.m_pwszSnapshotDeviceObject)
    };
    let shadow_id_str = format_guid(&shadow_id);

    // The BackupComponents Drop releases the IVssBackupComponents
    // pointer here; the snapshot itself stays alive because we
    // requested non-persistent + the COM library holds its own
    // ref via the snapshot set. Re-acquiring the SD via a fresh
    // BackupComponents is how `release_shadow_via_com` finds it.
    Ok((shadow_id_str, device_path))
}

/// Release a previously-minted shadow. Mirrors the wire-shape
/// the PowerShell path uses (`{GUID}` shadow id).
#[allow(dead_code)]
pub fn release_shadow_via_com(shadow_id_str: &str) -> Result<(), String> {
    ensure_com_initialised()?;
    let shadow_id = parse_guid(shadow_id_str)?;

    let mut backup_ptr: *mut IVssBackupComponents = ptr::null_mut();
    let hr = unsafe { CreateVssBackupComponents(&mut backup_ptr) };
    if hr != S_OK {
        return Err(format!("CreateVssBackupComponents hr=0x{:x}", hr as u32));
    }
    let backup = BackupComponents { ptr: backup_ptr };
    if backup.ptr.is_null() {
        return Err("CreateVssBackupComponents returned null pointer".into());
    }

    let hr = unsafe {
        ((*(*backup.ptr).lpVtbl).InitializeForBackup)(backup.ptr, ptr::null_mut())
    };
    if hr != S_OK {
        return Err(format!("InitializeForBackup (release) hr=0x{:x}", hr as u32));
    }
    // Match the context used at create-time so DeleteSnapshots
    // sees the persistent shadow (non-persistent VSS_CTX_BACKUP
    // would auto-release on the create-side instance teardown
    // and surface VSS_E_OBJECT_NOT_FOUND here).
    let hr = unsafe {
        ((*(*backup.ptr).lpVtbl).SetContext)(backup.ptr, VSS_CTX_APP_ROLLBACK as LONG)
    };
    if hr != S_OK {
        return Err(format!("SetContext (release) hr=0x{:x}", hr as u32));
    }

    // DeleteSnapshots(shadow_id, VSS_OBJECT_SNAPSHOT, force=TRUE,
    //                 &deleted_count, &nondeleted_id).
    let mut deleted: LONG = 0;
    let mut nondeleted_id: GUID = unsafe { std::mem::zeroed() };
    let hr = unsafe {
        ((*(*backup.ptr).lpVtbl).DeleteSnapshots)(
            backup.ptr,
            shadow_id,
            VSS_OBJECT_SNAPSHOT,
            TRUE,
            &mut deleted,
            &mut nondeleted_id,
        )
    };
    if hr != S_OK {
        return Err(format!(
            "DeleteSnapshots hr=0x{:x} deleted={deleted}",
            hr as u32
        ));
    }
    if deleted < 1 {
        return Err(format!(
            "DeleteSnapshots reported {deleted} shadows deleted (expected at least 1)"
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_guid_round_trips_through_parse_guid() {
        let original = GUID {
            Data1: 0xAABB_CCDD,
            Data2: 0x1122,
            Data3: 0x3344,
            Data4: [0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC],
        };
        let s = format_guid(&original);
        assert_eq!(s, "{AABBCCDD-1122-3344-5566-778899AABBCC}");
        let parsed = parse_guid(&s).expect("round-trip");
        assert_eq!(parsed.Data1, original.Data1);
        assert_eq!(parsed.Data2, original.Data2);
        assert_eq!(parsed.Data3, original.Data3);
        assert_eq!(parsed.Data4, original.Data4);
    }

    #[test]
    fn parse_guid_rejects_malformed_input() {
        for bad in &[
            "",
            "{}",
            "{AABBCCDD-1122-3344-5566-778899AABBCC", // missing close brace
            "AABBCCDD-1122-3344-5566-778899AABBCC",   // missing braces entirely
            "{AABBCCDD11223344556677889AABBCC}",      // missing hyphens
            "{ZZZZZZZZ-1122-3344-5566-778899AABBCC}", // non-hex
        ] {
            assert!(
                parse_guid(bad).is_err(),
                "expected parse_guid to reject {bad:?}"
            );
        }
    }

    #[test]
    fn to_wide_zstr_terminates_with_nul() {
        let w = to_wide_zstr("C:\\");
        assert_eq!(*w.last().unwrap(), 0u16, "must be NUL-terminated");
        assert_eq!(w[0], b'C' as u16);
        assert_eq!(w[1], b':' as u16);
        assert_eq!(w[2], b'\\' as u16);
    }

    /// VSS smoke — creates a shadow of the system drive, prints
    /// the device path, releases. Requires Administrator + a
    /// working VSS service. Ignored by default; run via:
    ///
    /// ```bash
    /// cargo test -p copythat-snapshot --features vss-com -- --ignored
    /// ```
    #[test]
    #[ignore = "requires Administrator + Windows VSS service"]
    fn vss_com_create_release_round_trip() {
        let volume = std::env::var("COPYTHAT_VSS_TEST_VOLUME")
            .unwrap_or_else(|_| r"C:\".to_string());
        let (shadow_id, device_path) = create_shadow_via_com(&volume)
            .expect("create_shadow_via_com");
        eprintln!("shadow_id   = {shadow_id}");
        eprintln!("device_path = {device_path}");
        assert!(
            shadow_id.starts_with('{') && shadow_id.ends_with('}'),
            "shadow_id should be brace-wrapped GUID"
        );
        assert!(
            device_path.starts_with(r"\\?\GLOBALROOT\Device\HarddiskVolumeShadowCopy"),
            "device_path should be VSS device-object form"
        );
        release_shadow_via_com(&shadow_id).expect("release_shadow_via_com");
    }

    /// End-to-end locked-file probe: while `tests/lock_file.ps1` keeps
    /// `COPYTHAT_VSS_LOCKED_FILE_PATH` exclusively open, this test
    /// proves the snapshot fallback's contract — direct read fails
    /// with a sharing violation, but the same file read off the
    /// shadow's GLOBALROOT path returns the bytes, which we then copy
    /// to `COPYTHAT_VSS_DEST_DIR` and byte-compare.
    ///
    /// Driven by `tests\vss_com_smoke.ps1 -LockedFilePath ...`.
    /// Requires Administrator + a running `lock_file.ps1` holding the
    /// target file; ignored by default.
    #[test]
    #[ignore = "requires Administrator + a running lock_file.ps1"]
    fn vss_com_copy_locked_file_via_shadow() {
        use std::fs;
        use std::path::{Path, PathBuf};

        let locked = std::env::var("COPYTHAT_VSS_LOCKED_FILE_PATH")
            .expect("COPYTHAT_VSS_LOCKED_FILE_PATH must be set");
        let dest_dir = std::env::var("COPYTHAT_VSS_DEST_DIR")
            .expect("COPYTHAT_VSS_DEST_DIR must be set");

        let locked_path = PathBuf::from(&locked);
        assert!(
            locked_path.is_absolute(),
            "COPYTHAT_VSS_LOCKED_FILE_PATH must be absolute: {locked:?}"
        );

        // (1) Confirm the file IS exclusively locked. Direct
        // `OpenOptions::read` should fail with ERROR_SHARING_VIOLATION
        // (raw os error 32) — that's the situation the snapshot
        // fallback exists to recover from.
        let direct_err = fs::OpenOptions::new()
            .read(true)
            .open(&locked_path)
            .err()
            .expect(
                "direct read of locked file unexpectedly succeeded — is lock_file.ps1 running?",
            );
        let raw = direct_err.raw_os_error();
        eprintln!("direct read err: {direct_err:?} (raw_os_error={raw:?})");
        assert_eq!(
            raw,
            Some(32),
            "expected ERROR_SHARING_VIOLATION (32) on direct read"
        );

        // (2) Resolve the volume to snapshot. We require
        // `[A-Za-z]:\<rest>` — the same shape `applies_to` enforces
        // upstream. `Path::components` would normalise away the
        // leading prefix on Windows, so go to the bytes.
        let s = locked_path.to_string_lossy().into_owned();
        let bytes = s.as_bytes();
        assert!(
            bytes.len() >= 3
                && bytes[0].is_ascii_alphabetic()
                && bytes[1] == b':'
                && (bytes[2] == b'\\' || bytes[2] == b'/'),
            "locked path must start with `X:\\`: {s:?}"
        );
        let drive = format!("{}:\\", (bytes[0] as char).to_ascii_uppercase());
        let rest = &s[3..]; // strip the `X:\` prefix

        // (3) Mint the shadow.
        let (shadow_id, device_path) = create_shadow_via_com(&drive)
            .expect("create_shadow_via_com");
        eprintln!("shadow_id   = {shadow_id}");
        eprintln!("device_path = {device_path}");

        // Wrap the rest of the test so we always release the shadow,
        // even on assertion failure.
        let result = std::panic::catch_unwind(|| {
            // (4) Read the file off the shadow.
            let shadow_path = format!("{device_path}\\{rest}");
            eprintln!("shadow_path = {shadow_path}");
            let bytes_via_shadow = fs::read(&shadow_path)
                .expect("read locked file via shadow GLOBALROOT path");
            assert!(
                !bytes_via_shadow.is_empty(),
                "shadow read returned empty file"
            );

            // (5) Verify the deterministic pattern lock_file.ps1 wrote
            // (0..255 repeating, default 4096 bytes).
            for (i, b) in bytes_via_shadow.iter().enumerate() {
                assert_eq!(
                    *b as usize,
                    i % 256,
                    "byte {i} = 0x{b:02X}, expected 0x{:02X}",
                    i % 256
                );
            }

            // (6) Copy to the destination folder + byte-compare.
            let dest_dir_p = Path::new(&dest_dir);
            fs::create_dir_all(dest_dir_p).expect("create dest dir");
            let dest_file = dest_dir_p.join(
                locked_path
                    .file_name()
                    .expect("locked path has a file name"),
            );
            fs::write(&dest_file, &bytes_via_shadow).expect("write copy to dest");

            let dest_bytes = fs::read(&dest_file).expect("read copy back");
            assert_eq!(
                bytes_via_shadow, dest_bytes,
                "destination copy does not match shadow read"
            );
            eprintln!(
                "copied {} bytes from shadow -> {}",
                bytes_via_shadow.len(),
                dest_file.display()
            );
        });

        // (7) Always release.
        release_shadow_via_com(&shadow_id).expect("release_shadow_via_com");

        if let Err(p) = result {
            std::panic::resume_unwind(p);
        }
    }
}
