//! `IClassFactory` one per command.
//!
//! COM asks for a class factory via `DllGetClassObject(clsid, iid,
//! &out)`; the factory's `CreateInstance` returns a brand-new
//! `CopyCommand` / `MoveCommand` wrapped in whatever interface the
//! caller asked for. Because `#[implement]` adds `IUnknown` and
//! `IExplorerCommand` to every vtable, the cast is handled by
//! `windows-core::Interface::query`.

#![cfg(windows)]

use std::ffi::c_void;

use windows::Win32::Foundation::{CLASS_E_NOAGGREGATION, E_POINTER};
use windows::Win32::System::Com::{IClassFactory, IClassFactory_Impl};
use windows::core::{BOOL, GUID, IUnknown, Interface, Ref, Result as WinResult};
use windows_core::ComObject;
use windows_implement::implement;

use crate::com::{CopyCommand, MoveCommand};

/// Class factory for the Copy verb.
#[implement(IClassFactory)]
pub struct CopyFactory;

/// Class factory for the Move verb.
#[implement(IClassFactory)]
pub struct MoveFactory;

impl IClassFactory_Impl for CopyFactory_Impl {
    fn CreateInstance(
        &self,
        punkouter: Ref<'_, IUnknown>,
        riid: *const GUID,
        ppvobject: *mut *mut c_void,
    ) -> WinResult<()> {
        if !punkouter.is_null() {
            return Err(CLASS_E_NOAGGREGATION.into());
        }
        let unknown: IUnknown = ComObject::new(CopyCommand).to_interface();
        qi(unknown, riid, ppvobject)
    }

    fn LockServer(&self, flock: BOOL) -> WinResult<()> {
        crate::dll::lock_server(flock.as_bool());
        Ok(())
    }
}

impl IClassFactory_Impl for MoveFactory_Impl {
    fn CreateInstance(
        &self,
        punkouter: Ref<'_, IUnknown>,
        riid: *const GUID,
        ppvobject: *mut *mut c_void,
    ) -> WinResult<()> {
        if !punkouter.is_null() {
            return Err(CLASS_E_NOAGGREGATION.into());
        }
        let unknown: IUnknown = ComObject::new(MoveCommand).to_interface();
        qi(unknown, riid, ppvobject)
    }

    fn LockServer(&self, flock: BOOL) -> WinResult<()> {
        crate::dll::lock_server(flock.as_bool());
        Ok(())
    }
}

/// QueryInterface the just-constructed `IUnknown` for the IID the
/// caller asked for, write into `ppvobject`, translate HRESULT.
fn qi(unknown: IUnknown, riid: *const GUID, ppvobject: *mut *mut c_void) -> WinResult<()> {
    if ppvobject.is_null() {
        return Err(E_POINTER.into());
    }
    // SAFETY: caller guarantees a valid out-pointer.
    unsafe {
        *ppvobject = core::ptr::null_mut();
    }
    if riid.is_null() {
        return Err(E_POINTER.into());
    }
    // SAFETY: riid is a valid pointer to a GUID supplied by COM.
    let iid = unsafe { &*riid };
    // SAFETY: `ppvobject` is a live out-pointer; `iid` is live for
    // the call duration.
    let hr = unsafe { unknown.query(iid, ppvobject) };
    hr.ok()
}
