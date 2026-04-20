//! DLL entry points.
//!
//! Four exports make this crate's `.dll` a legal COM in-proc server:
//!
//! - `DllGetClassObject` — COM calls this to get our class factory.
//! - `DllCanUnloadNow` — COM calls this periodically; we return
//!   `S_OK` only when every handed-out instance has been released.
//! - `DllRegisterServer` — `regsvr32` calls this to install the keys.
//! - `DllUnregisterServer` — `regsvr32 /u` calls this to remove them.
//!
//! We also keep a global module instance (`MODULE_HANDLE`) so we can
//! resolve the DLL's own path in `DllRegisterServer` without asking
//! the caller. The handle is set by `DllMain`.

#![cfg(windows)]

use std::ffi::c_void;
use std::sync::atomic::{AtomicIsize, AtomicUsize, Ordering};

use windows::Win32::Foundation::{
    CLASS_E_CLASSNOTAVAILABLE, E_FAIL, E_POINTER, HINSTANCE, HMODULE, MAX_PATH, S_FALSE, S_OK,
};
use windows::Win32::System::LibraryLoader::GetModuleFileNameW;
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
use windows::core::{BOOL, GUID, HRESULT, Interface};
use windows_core::ComObject;

use crate::consts::{CLSID_COPY, CLSID_MOVE};
use crate::factory::{CopyFactory, MoveFactory};
use crate::registry::{
    InstallScope, all_registration_keys, apply_registration, copy_interceptor_keys,
    delete_registration,
};

/// Global lock + instance count. `DllCanUnloadNow` returns
/// `S_OK` only when both drop to zero.
static LOCK_COUNT: AtomicIsize = AtomicIsize::new(0);
static INSTANCE_COUNT: AtomicUsize = AtomicUsize::new(0);

/// Cached handle to this DLL, set by `DllMain` on load.
static MODULE_HANDLE: AtomicIsize = AtomicIsize::new(0);

/// Increment / decrement the lock count. Called by the class
/// factory's `LockServer`; also exposed for future callers.
pub fn lock_server(lock: bool) {
    if lock {
        LOCK_COUNT.fetch_add(1, Ordering::SeqCst);
    } else {
        LOCK_COUNT.fetch_sub(1, Ordering::SeqCst);
    }
}

/// Bump the instance count when a factory hands out a new object;
/// decrement on drop. `ComObject::new` / Drop are the natural call
/// sites but would require wrapper types — the 0.x cycle keeps
/// things simple by always returning `S_OK` from `DllCanUnloadNow`
/// (see below), which is conservative: Windows will just reload the
/// DLL on next use.
pub fn instance_added() {
    INSTANCE_COUNT.fetch_add(1, Ordering::SeqCst);
}

pub fn instance_removed() {
    INSTANCE_COUNT.fetch_sub(1, Ordering::SeqCst);
}

/// DLL entry point. `windows` does not provide a Rust-friendly
/// `DllMain` wrapper in 0.61; we hand-roll the signature so we can
/// stash `HINSTANCE` on process attach.
///
/// # Safety
///
/// Called by the Windows loader; must not touch anything that needs
/// CRT init beyond what the loader has already done.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn DllMain(
    hinstance: HINSTANCE,
    reason: u32,
    _reserved: *mut c_void,
) -> BOOL {
    if reason == DLL_PROCESS_ATTACH {
        MODULE_HANDLE.store(hinstance.0 as isize, Ordering::SeqCst);
    }
    BOOL::from(true)
}

/// COM entry point: hand out a class factory for the requested CLSID.
///
/// # Safety
///
/// `rclsid` and `riid` must point at valid GUIDs; `ppv` must be a
/// valid out-pointer. COM guarantees this.
#[unsafe(no_mangle)]
pub unsafe extern "system" fn DllGetClassObject(
    rclsid: *const GUID,
    riid: *const GUID,
    ppv: *mut *mut c_void,
) -> HRESULT {
    if ppv.is_null() {
        return E_POINTER;
    }
    // SAFETY: caller hands us a valid out-pointer.
    unsafe {
        *ppv = std::ptr::null_mut();
    }
    if rclsid.is_null() || riid.is_null() {
        return E_POINTER;
    }
    // SAFETY: rclsid came from COM.
    let clsid = unsafe { *rclsid };
    let factory_unknown: windows::core::IUnknown = if clsid == CLSID_COPY {
        ComObject::new(CopyFactory).into_interface()
    } else if clsid == CLSID_MOVE {
        ComObject::new(MoveFactory).into_interface()
    } else {
        return CLASS_E_CLASSNOTAVAILABLE;
    };

    // SAFETY: riid is a valid GUID pointer, ppv is a valid out-ptr.
    unsafe {
        let iid = &*riid;
        match factory_unknown.query(iid, ppv) {
            code if code.is_ok() => S_OK,
            code => code,
        }
    }
}

/// Returns `S_OK` only when no instances + no locks — conservative
/// default is `S_FALSE` to keep the DLL pinned until we are certain
/// it is idle. Explorer calls this periodically; a premature unload
/// would orphan a pending `Invoke`.
#[unsafe(no_mangle)]
pub extern "system" fn DllCanUnloadNow() -> HRESULT {
    if LOCK_COUNT.load(Ordering::SeqCst) == 0 && INSTANCE_COUNT.load(Ordering::SeqCst) == 0 {
        S_OK
    } else {
        S_FALSE
    }
}

/// `regsvr32 copythat-shellext.dll` lands here.
///
/// Writes the per-user registration (HKCU). System-wide install is
/// reserved for the MSI installer, which calls `register_server` in
/// the rlib surface with [`InstallScope::LocalMachine`].
#[unsafe(no_mangle)]
pub extern "system" fn DllRegisterServer() -> HRESULT {
    match register_server(InstallScope::PerUser) {
        Ok(()) => S_OK,
        Err(_) => E_FAIL,
    }
}

/// `regsvr32 /u copythat-shellext.dll` lands here.
#[unsafe(no_mangle)]
pub extern "system" fn DllUnregisterServer() -> HRESULT {
    match unregister_server(InstallScope::PerUser) {
        Ok(()) => S_OK,
        Err(_) => E_FAIL,
    }
}

/// High-level registration helper. Usable from the rlib surface
/// (integration tests, packaging scripts).
pub fn register_server(scope: InstallScope) -> windows::core::Result<()> {
    let dll = dll_path()?;
    let dll_str = dll.to_string_lossy().into_owned();
    let keys = all_registration_keys(scope, &dll_str);
    apply_registration(&keys)
}

/// Inverse of [`register_server`].
pub fn unregister_server(scope: InstallScope) -> windows::core::Result<()> {
    // Rebuild the registration tuples with a dummy DLL path so we
    // can reuse the same key-list — `delete_registration` ignores
    // the value column.
    let keys = all_registration_keys(scope, "");
    delete_registration(&keys)
}

/// Opt-in TeraCopy-style default-copy-verb interceptor.
/// Windows Explorer's Ctrl-C / drag-copy funnel routes through our
/// `CopyCommand` while the key is present.
pub fn register_copy_interceptor() -> windows::core::Result<()> {
    let keys = copy_interceptor_keys(crate::consts::CLSID_COPY_STR);
    apply_registration(&keys)
}

/// Inverse of [`register_copy_interceptor`].
pub fn unregister_copy_interceptor() -> windows::core::Result<()> {
    let keys = copy_interceptor_keys(crate::consts::CLSID_COPY_STR);
    delete_registration(&keys)
}

/// Resolve the on-disk path of this DLL via `GetModuleFileNameW` on
/// the cached `HINSTANCE` from `DllMain`.
fn dll_path() -> windows::core::Result<std::path::PathBuf> {
    let hinst = HMODULE(MODULE_HANDLE.load(Ordering::SeqCst) as *mut c_void);
    let mut buf = [0u16; MAX_PATH as usize];
    // SAFETY: buf is a stack-allocated Vec<u16> of MAX_PATH — the
    // syscall writes at most that many code units + NUL.
    let len = unsafe { GetModuleFileNameW(Some(hinst), &mut buf) };
    if len == 0 {
        return Err(windows::core::Error::from_win32());
    }
    let s = String::from_utf16_lossy(&buf[..len as usize]);
    Ok(std::path::PathBuf::from(s))
}
