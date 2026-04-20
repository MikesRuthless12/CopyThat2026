//! `IExplorerCommand` implementations for the Copy + Move verbs.
//!
//! Each struct exposes one canonical name / GUID / display string.
//! Everything else delegates into [`crate::spawn`] so the UI-facing
//! metadata and the action logic stay decoupled.
//!
//! `IExplorerCommand` is the modern replacement for `IContextMenu`:
//! Explorer calls our `Invoke` with an `IShellItemArray` carrying
//! every selected item, and we extract `SIGDN_FILESYSPATH` paths
//! from each item before handing them to `copythat --enqueue`.

#![cfg(windows)]

use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

use windows::Win32::Foundation::{E_NOTIMPL, E_POINTER};
use windows::Win32::System::Com::IBindCtx;
use windows::Win32::UI::Shell::{
    ECS_ENABLED, IEnumExplorerCommand, IExplorerCommand, IExplorerCommand_Impl, IShellItemArray,
    SIGDN_FILESYSPATH,
};
use windows::core::{BOOL, GUID, PWSTR, Ref, Result as WinResult};
use windows_implement::implement;

use crate::consts::{CLSID_COPY, CLSID_MOVE, DISPLAY_COPY, DISPLAY_MOVE};
use crate::spawn::{Verb, spawn_detached};

/// "Copy with Copy That" — queues a copy job per selected item.
#[implement(IExplorerCommand)]
pub struct CopyCommand;

/// "Move with Copy That" — queues a move job per selected item.
#[implement(IExplorerCommand)]
pub struct MoveCommand;

impl IExplorerCommand_Impl for CopyCommand_Impl {
    fn GetTitle(&self, _psiitemarray: Ref<'_, IShellItemArray>) -> WinResult<PWSTR> {
        alloc_pwstr(DISPLAY_COPY)
    }

    fn GetIcon(&self, _psiitemarray: Ref<'_, IShellItemArray>) -> WinResult<PWSTR> {
        // Not localised / themed for 0.x — Phase 16 packaging
        // embeds the icon and the registration flips this to
        // `"%ProgramFiles%\\CopyThat2026\\copythat-ui.exe,0"` so
        // Explorer can pull the resource.
        Err(E_NOTIMPL.into())
    }

    fn GetToolTip(&self, _psiitemarray: Ref<'_, IShellItemArray>) -> WinResult<PWSTR> {
        Err(E_NOTIMPL.into())
    }

    fn GetCanonicalName(&self) -> WinResult<GUID> {
        Ok(CLSID_COPY)
    }

    fn GetState(
        &self,
        _psiitemarray: Ref<'_, IShellItemArray>,
        _foktobeslow: BOOL,
    ) -> WinResult<u32> {
        Ok(ECS_ENABLED.0 as u32)
    }

    fn Invoke(
        &self,
        psiitemarray: Ref<'_, IShellItemArray>,
        _pbc: Ref<'_, IBindCtx>,
    ) -> WinResult<()> {
        invoke_with_verb(Verb::Copy, psiitemarray)
    }

    fn GetFlags(&self) -> WinResult<u32> {
        // ECF_DEFAULT == 0 — neither icon-only, separator, nor
        // hidden-when-disabled. Keeps Explorer's rendering logic
        // simple and matches every other regular verb.
        Ok(0)
    }

    fn EnumSubCommands(&self) -> WinResult<IEnumExplorerCommand> {
        // We are a leaf verb, not a submenu host.
        Err(E_NOTIMPL.into())
    }
}

impl IExplorerCommand_Impl for MoveCommand_Impl {
    fn GetTitle(&self, _psiitemarray: Ref<'_, IShellItemArray>) -> WinResult<PWSTR> {
        alloc_pwstr(DISPLAY_MOVE)
    }

    fn GetIcon(&self, _psiitemarray: Ref<'_, IShellItemArray>) -> WinResult<PWSTR> {
        Err(E_NOTIMPL.into())
    }

    fn GetToolTip(&self, _psiitemarray: Ref<'_, IShellItemArray>) -> WinResult<PWSTR> {
        Err(E_NOTIMPL.into())
    }

    fn GetCanonicalName(&self) -> WinResult<GUID> {
        Ok(CLSID_MOVE)
    }

    fn GetState(
        &self,
        _psiitemarray: Ref<'_, IShellItemArray>,
        _foktobeslow: BOOL,
    ) -> WinResult<u32> {
        Ok(ECS_ENABLED.0 as u32)
    }

    fn Invoke(
        &self,
        psiitemarray: Ref<'_, IShellItemArray>,
        _pbc: Ref<'_, IBindCtx>,
    ) -> WinResult<()> {
        invoke_with_verb(Verb::Move, psiitemarray)
    }

    fn GetFlags(&self) -> WinResult<u32> {
        Ok(0)
    }

    fn EnumSubCommands(&self) -> WinResult<IEnumExplorerCommand> {
        Err(E_NOTIMPL.into())
    }
}

/// Shared `Invoke` body: pull every filesystem path out of the
/// `IShellItemArray` and hand them to `spawn_detached`.
fn invoke_with_verb(verb: Verb, array: Ref<'_, IShellItemArray>) -> WinResult<()> {
    let Some(array) = array.as_ref() else {
        return Err(E_POINTER.into());
    };
    let paths = collect_paths(array)?;
    if paths.is_empty() {
        // No actionable items — treat as a no-op success. Explorer
        // will simply close the menu.
        return Ok(());
    }
    spawn_detached(verb, &paths).map_err(|e| {
        // `CreateProcessW` failures surface as `std::io::Error` via
        // the OsString-arg `Command::spawn`; convert to HRESULT.
        windows::core::Error::from_hresult(windows::core::HRESULT::from_win32(
            e.raw_os_error().unwrap_or(0) as u32,
        ))
    })
}

/// Walk an `IShellItemArray`, extracting each item's
/// `SIGDN_FILESYSPATH`. Items that don't have a filesystem path
/// (zip entries, virtual shell namespaces) are silently skipped
/// rather than failing the whole `Invoke` — the shell menu should
/// still queue whatever is queueable.
fn collect_paths(array: &IShellItemArray) -> WinResult<Vec<OsString>> {
    // SAFETY: `array` is a live COM pointer owned by Explorer for
    // the duration of the `Invoke` call.
    let count = unsafe { array.GetCount() }?;
    let mut out = Vec::with_capacity(count as usize);
    for i in 0..count {
        // SAFETY: 0 <= i < count; GetItemAt returns an item we own.
        let Ok(item) = (unsafe { array.GetItemAt(i) }) else {
            continue;
        };
        // SAFETY: the returned PWSTR is allocated with CoTaskMemAlloc
        // and we release it via `CoTaskMemFree` as soon as we have
        // the OsString copy in hand. Note: the windows crate wraps
        // the free in its `PWSTR::Drop` via the `BSTR`-like type —
        // but for raw PWSTRs we need manual cleanup.
        match unsafe { item.GetDisplayName(SIGDN_FILESYSPATH) } {
            Ok(pwstr) => {
                let os = pwstr_to_os(pwstr);
                // SAFETY: the PWSTR came from CoTaskMemAlloc via
                // IShellItem::GetDisplayName; free it per contract.
                unsafe {
                    windows::Win32::System::Com::CoTaskMemFree(Some(pwstr.0.cast()));
                }
                if !os.is_empty() {
                    out.push(os);
                }
            }
            Err(_) => continue,
        }
    }
    Ok(out)
}

/// Read a zero-terminated `PWSTR` into an `OsString` without
/// re-encoding — the Windows filesystem is UTF-16 and `copythat`'s
/// CLI parser is `OsString`-clean, so we preserve every code unit.
fn pwstr_to_os(p: PWSTR) -> OsString {
    if p.is_null() {
        return OsString::new();
    }
    // SAFETY: GetDisplayName returns a null-terminated UTF-16
    // string; walk until we hit the NUL.
    let mut len = 0;
    unsafe {
        while *p.0.add(len) != 0 {
            len += 1;
        }
    }
    // SAFETY: len is < isize::MAX because GetDisplayName returns a
    // MAX_PATH-ish buffer; slice length matches.
    let slice = unsafe { std::slice::from_raw_parts(p.0, len) };
    OsString::from_wide(slice)
}

/// Allocate a CoTaskMem-owned PWSTR holding `s` (UTF-16 + NUL).
/// Explorer takes ownership and frees it via `CoTaskMemFree`.
fn alloc_pwstr(s: &str) -> WinResult<PWSTR> {
    use windows::Win32::System::Com::CoTaskMemAlloc;

    // UTF-8 → UTF-16 + trailing NUL.
    let mut wide: Vec<u16> = s.encode_utf16().collect();
    wide.push(0);
    let bytes = wide.len() * std::mem::size_of::<u16>();
    // SAFETY: CoTaskMemAlloc returns a heap pointer owned by the
    // caller; we fill it with the UTF-16 contents then hand off.
    let raw = unsafe { CoTaskMemAlloc(bytes) };
    if raw.is_null() {
        return Err(windows::core::Error::from_hresult(
            windows::Win32::Foundation::E_OUTOFMEMORY,
        ));
    }
    let dst = raw as *mut u16;
    // SAFETY: `dst` came from CoTaskMemAlloc with `bytes` room;
    // source is a Rust-owned Vec<u16> of equal length.
    unsafe {
        std::ptr::copy_nonoverlapping(wide.as_ptr(), dst, wide.len());
    }
    Ok(PWSTR(dst))
}
