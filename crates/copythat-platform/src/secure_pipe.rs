//! Phase 17d — secure (DACL-restricted) named-pipe server for the
//! privilege-escalation helper handshake.
//!
//! Tokio's `ServerOptions::create` doesn't expose `SECURITY_ATTRIBUTES`,
//! so by default the kernel applies its built-in pipe DACL — which on a
//! per-user pipe namespace grants `GENERIC_READ|GENERIC_WRITE` to every
//! process in the same desktop session. That's wider than the helper
//! handshake needs: the only legitimate callers are (a) the unprivileged
//! main app that creates the pipe and (b) the elevated `copythat-helper`
//! child it just spawned.
//!
//! This module wraps the raw `CreateNamedPipeW` Win32 call with a
//! hand-built `SECURITY_DESCRIPTOR` whose DACL grants access to exactly
//! two principals:
//!
//! - The current process's logon user SID — so the unprivileged parent
//!   can read + write its own end.
//! - `BUILTIN\Administrators` — so the elevated helper (running under the
//!   user's elevated token for the duration of the consent) can read +
//!   write the other end.
//!
//! Everyone else is implicitly denied, so a non-admin attacker in the
//! same desktop session cannot race to connect to the helper-side end
//! before the legitimate helper does. This is the load-bearing security
//! property of the elevated-retry path — see `copythat-helper::spawn`.
//!
//! All FFI is confined to this module; callers see a single safe
//! [`create_secure_named_pipe_server`] entry point returning a standard
//! `tokio::net::windows::named_pipe::NamedPipeServer`.
//!
//! NOTE (future DRY): `copythat-snapshot::backends::win_pipe_security`
//! carries a near-identical copy for the VSS helper. Both should
//! eventually call this one; the VSS copy is intentionally left in place
//! for now to avoid coupling the snapshot crate's release cadence to the
//! platform crate.

#![cfg(windows)]

use std::ffi::OsStr;
use std::io;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::io::RawHandle;
use std::ptr;

use tokio::net::windows::named_pipe::NamedPipeServer;
use windows_sys::Win32::Foundation::{BOOL, CloseHandle, FALSE, HANDLE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::Security::{
    ACCESS_ALLOWED_ACE, ACL, ACL_REVISION, AddAccessAllowedAce, AllocateAndInitializeSid, FreeSid,
    GetTokenInformation, InitializeAcl, InitializeSecurityDescriptor, PSECURITY_DESCRIPTOR, PSID,
    SECURITY_ATTRIBUTES, SECURITY_DESCRIPTOR, SECURITY_NT_AUTHORITY, SID_IDENTIFIER_AUTHORITY,
    SetSecurityDescriptorDacl, TOKEN_QUERY, TOKEN_USER, TokenUser,
};
use windows_sys::Win32::Storage::FileSystem::{
    FILE_FLAG_FIRST_PIPE_INSTANCE, FILE_FLAG_OVERLAPPED, PIPE_ACCESS_DUPLEX,
};
use windows_sys::Win32::System::Pipes::{
    CreateNamedPipeW, PIPE_READMODE_BYTE, PIPE_REJECT_REMOTE_CLIENTS, PIPE_TYPE_BYTE, PIPE_WAIT,
};
use windows_sys::Win32::System::SystemServices::SECURITY_DESCRIPTOR_REVISION;
use windows_sys::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

const SECURITY_BUILTIN_DOMAIN_RID: u32 = 0x20;
const DOMAIN_ALIAS_RID_ADMINS: u32 = 0x220;
const GENERIC_READ: u32 = 0x8000_0000;
const GENERIC_WRITE: u32 = 0x4000_0000;

/// Per-server pipe-side buffer size, matching tokio's
/// `ServerOptions::create` (64 KiB each direction).
const PIPE_BUF_BYTES: u32 = 64 * 1024;

/// `nMaxInstances`. Always 1 — the elevated helper is the only
/// legitimate client and the parent creates exactly one server end per
/// helper session.
const MAX_INSTANCES: u32 = 1;

/// Default pipe timeout in ms. Tokio's `ServerOptions::create` uses 0
/// (= 50 ms default); match it for behavioural parity.
const DEFAULT_TIMEOUT_MS: u32 = 0;

/// Build a tokio `NamedPipeServer` whose underlying Win32 pipe uses a
/// custom DACL granting access only to the current user and
/// `BUILTIN\Administrators`.
///
/// Mirrors the open mode + flags tokio's
/// `ServerOptions::new().first_pipe_instance(true).max_instances(1)
/// .reject_remote_clients(true).create(name)` would set, plus the
/// tightened security descriptor.
///
/// `name` must start with `\\.\pipe\` (validated up front).
pub fn create_secure_named_pipe_server(name: &str) -> io::Result<NamedPipeServer> {
    if !name.starts_with(r"\\.\pipe\") {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("pipe name must start with `\\\\.\\pipe\\`, got {name:?}"),
        ));
    }

    let wide: Vec<u16> = OsStr::new(name)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    let mut sd = SecurityDescriptor::new()?;
    let sa = SECURITY_ATTRIBUTES {
        nLength: std::mem::size_of::<SECURITY_ATTRIBUTES>() as u32,
        lpSecurityDescriptor: sd.as_psd(),
        bInheritHandle: FALSE,
    };

    // PIPE_ACCESS_DUPLEX layers with FILE_FLAG_* in the same DWORD.
    // FILE_FLAG_OVERLAPPED is required because tokio's
    // `NamedPipeServer::from_raw_handle` registers the handle with its
    // IOCP — synchronous pipes are rejected with ERROR_INVALID_PARAMETER.
    let open_mode: u32 = PIPE_ACCESS_DUPLEX | FILE_FLAG_FIRST_PIPE_INSTANCE | FILE_FLAG_OVERLAPPED;
    let pipe_mode: u32 =
        PIPE_TYPE_BYTE | PIPE_READMODE_BYTE | PIPE_WAIT | PIPE_REJECT_REMOTE_CLIENTS;

    // SAFETY: `wide` is a NUL-terminated UTF-16 buffer; `&sa` is a
    // stack-local with a valid SECURITY_DESCRIPTOR pointer; all u32 args
    // fit the documented Win32 ranges; the returned HANDLE is checked
    // against INVALID_HANDLE_VALUE below before tokio takes ownership.
    let handle: HANDLE = unsafe {
        CreateNamedPipeW(
            wide.as_ptr(),
            open_mode,
            pipe_mode,
            MAX_INSTANCES,
            PIPE_BUF_BYTES,
            PIPE_BUF_BYTES,
            DEFAULT_TIMEOUT_MS,
            &sa,
        )
    };
    if handle.is_null() || handle == INVALID_HANDLE_VALUE {
        return Err(io::Error::last_os_error());
    }

    // SAFETY: `handle` is a kernel handle to a freshly-created pipe;
    // tokio's `from_raw_handle` takes ownership and its Drop closes it.
    // We must not CloseHandle it ourselves.
    let server = unsafe { NamedPipeServer::from_raw_handle(handle as RawHandle)? };

    // CreateNamedPipeW copied what it needed from the descriptor, so the
    // backing allocations can drop now.
    drop(sd);

    Ok(server)
}

/// Owns the allocations behind a pipe `SECURITY_DESCRIPTOR`; Drop frees
/// them in the correct order.
struct SecurityDescriptor {
    sd: Box<SECURITY_DESCRIPTOR>,
    /// Backing storage for the ACL; lives as long as the SD refers to it.
    _dacl: Vec<u8>,
    /// Token-info buffer holding the current user's SID; the SD's DACL
    /// points into this allocation.
    _token_user_buf: Vec<u8>,
    admins_sid: PSID,
}

impl SecurityDescriptor {
    fn new() -> io::Result<Self> {
        let token_user_buf = current_user_token_user()?;
        // SAFETY: the buffer was sized + filled by GetTokenInformation;
        // its prefix is a `TOKEN_USER` whose `User.Sid` points into it.
        let user_sid: PSID = unsafe { (*(token_user_buf.as_ptr() as *const TOKEN_USER)).User.Sid };

        let admins_sid = allocate_admins_sid()?;

        // DACL big enough for two ACEs plus generous slack; the size set
        // by InitializeAcl is what the kernel honours.
        let dacl_size: u32 = (std::mem::size_of::<ACL>()
            + 2 * (std::mem::size_of::<ACCESS_ALLOWED_ACE>())) as u32
            + 256;
        let mut dacl: Vec<u8> = vec![0u8; dacl_size as usize];

        // SAFETY: `dacl` is a freshly-zeroed buffer of `dacl_size` bytes.
        let init_ok: BOOL =
            unsafe { InitializeAcl(dacl.as_mut_ptr() as *mut ACL, dacl_size, ACL_REVISION) };
        if init_ok == FALSE {
            unsafe { FreeSid(admins_sid) };
            return Err(io::Error::last_os_error());
        }

        // ACE 1 — current user, GENERIC_READ | GENERIC_WRITE.
        // SAFETY: `dacl` is initialised; `user_sid` points to a valid SID.
        let add_ok: BOOL = unsafe {
            AddAccessAllowedAce(
                dacl.as_mut_ptr() as *mut ACL,
                ACL_REVISION,
                GENERIC_READ | GENERIC_WRITE,
                user_sid,
            )
        };
        if add_ok == FALSE {
            unsafe { FreeSid(admins_sid) };
            return Err(io::Error::last_os_error());
        }

        // ACE 2 — Administrators, GENERIC_READ | GENERIC_WRITE.
        // SAFETY: `dacl` is initialised; `admins_sid` is a valid SID.
        let add_ok: BOOL = unsafe {
            AddAccessAllowedAce(
                dacl.as_mut_ptr() as *mut ACL,
                ACL_REVISION,
                GENERIC_READ | GENERIC_WRITE,
                admins_sid,
            )
        };
        if add_ok == FALSE {
            unsafe { FreeSid(admins_sid) };
            return Err(io::Error::last_os_error());
        }

        let mut sd: Box<SECURITY_DESCRIPTOR> =
            Box::new(unsafe { std::mem::zeroed::<SECURITY_DESCRIPTOR>() });
        // SAFETY: `sd` is a freshly-zeroed allocation of the right size.
        let init_ok: BOOL = unsafe {
            InitializeSecurityDescriptor(
                &mut *sd as *mut _ as PSECURITY_DESCRIPTOR,
                SECURITY_DESCRIPTOR_REVISION,
            )
        };
        if init_ok == FALSE {
            unsafe { FreeSid(admins_sid) };
            return Err(io::Error::last_os_error());
        }
        // SAFETY: `dacl` is initialised + has 2 ACEs; the SD holds a
        // non-owning pointer into it for the lifetime of this wrapper.
        let set_ok: BOOL = unsafe {
            SetSecurityDescriptorDacl(
                &mut *sd as *mut _ as PSECURITY_DESCRIPTOR,
                /* bDaclPresent */ 1,
                dacl.as_mut_ptr() as *mut ACL,
                /* bDaclDefaulted */ 0,
            )
        };
        if set_ok == FALSE {
            unsafe { FreeSid(admins_sid) };
            return Err(io::Error::last_os_error());
        }

        Ok(Self {
            sd,
            _dacl: dacl,
            _token_user_buf: token_user_buf,
            admins_sid,
        })
    }

    fn as_psd(&mut self) -> PSECURITY_DESCRIPTOR {
        &mut *self.sd as *mut _ as PSECURITY_DESCRIPTOR
    }
}

impl Drop for SecurityDescriptor {
    fn drop(&mut self) {
        if !self.admins_sid.is_null() {
            // SAFETY: SID came from AllocateAndInitializeSid; FreeSid is
            // its documented inverse.
            unsafe { FreeSid(self.admins_sid) };
            self.admins_sid = ptr::null_mut();
        }
        // _dacl + _token_user_buf drop via Vec; sd drops via Box.
    }
}

/// Allocate the `BUILTIN\Administrators` SID (S-1-5-32-544). Caller must
/// FreeSid the result (the `SecurityDescriptor` wrapper does so in Drop).
fn allocate_admins_sid() -> io::Result<PSID> {
    let mut admins: PSID = ptr::null_mut();
    let mut nt_authority = SECURITY_NT_AUTHORITY;
    // SAFETY: SECURITY_NT_AUTHORITY is a windows-sys static; the two RIDs
    // match Administrators; remaining sub-authority slots are 0.
    let ok: BOOL = unsafe {
        AllocateAndInitializeSid(
            &mut nt_authority as *mut SID_IDENTIFIER_AUTHORITY,
            2,
            SECURITY_BUILTIN_DOMAIN_RID,
            DOMAIN_ALIAS_RID_ADMINS,
            0,
            0,
            0,
            0,
            0,
            0,
            &mut admins,
        )
    };
    if ok == FALSE {
        return Err(io::Error::last_os_error());
    }
    Ok(admins)
}

/// Read the current process's logon user SID into a heap buffer whose
/// prefix is a valid `TOKEN_USER` followed by the SID body.
fn current_user_token_user() -> io::Result<Vec<u8>> {
    let mut token_handle: HANDLE = INVALID_HANDLE_VALUE;
    // SAFETY: GetCurrentProcess returns a pseudo-handle; OpenProcessToken
    // with TOKEN_QUERY is the documented way to fetch the user SID.
    let ok: BOOL = unsafe { OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token_handle) };
    if ok == FALSE {
        return Err(io::Error::last_os_error());
    }
    let mut needed: u32 = 0;
    // SAFETY: documented to return FALSE + write the required size into
    // `needed` when the buffer is too small; we pass a 0-byte probe.
    let _ =
        unsafe { GetTokenInformation(token_handle, TokenUser, ptr::null_mut(), 0, &mut needed) };
    if needed == 0 {
        unsafe { CloseHandle(token_handle) };
        return Err(io::Error::other(
            "GetTokenInformation probe returned 0 bytes",
        ));
    }
    let mut buf = vec![0u8; needed as usize];
    // SAFETY: `buf` is `needed` bytes; the call writes a TOKEN_USER + SID.
    let ok: BOOL = unsafe {
        GetTokenInformation(
            token_handle,
            TokenUser,
            buf.as_mut_ptr() as *mut std::ffi::c_void,
            needed,
            &mut needed,
        )
    };
    let close_err = if ok == FALSE {
        Some(io::Error::last_os_error())
    } else {
        None
    };
    // SAFETY: token_handle was opened above.
    unsafe { CloseHandle(token_handle) };
    if let Some(e) = close_err {
        return Err(e);
    }
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The function returns a usable tokio server handle on a fresh pipe
    /// name. We don't parse the DACL here (that needs GetSecurityInfo);
    /// the invariant under test is that the FFI call path succeeds and
    /// yields a `NamedPipeServer`. Driven through a tokio runtime because
    /// `from_raw_handle` registers the handle with tokio's reactor.
    #[tokio::test]
    async fn create_secure_pipe_server_smoke() {
        let name = format!(r"\\.\pipe\copythat-helper-test-{}", "ab".repeat(32));
        let server = create_secure_named_pipe_server(&name);
        assert!(
            server.is_ok(),
            "create_secure_named_pipe_server: {server:?}"
        );
        // Drop closes the handle.
    }

    #[tokio::test]
    async fn rejects_non_pipe_name() {
        let server = create_secure_named_pipe_server("not-a-pipe");
        assert!(server.is_err());
    }
}
