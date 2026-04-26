//! Phase 17 follow-up — secure named-pipe creation for the
//! Windows VSS helper handshake.
//!
//! Tokio's `ServerOptions::create` doesn't expose
//! `SECURITY_ATTRIBUTES`, so by default the kernel applies its
//! built-in pipe DACL — which on a per-user pipe namespace
//! grants `GENERIC_READ|GENERIC_WRITE` to the local interactive
//! group (i.e. every process running in the same desktop session).
//! That's wider than the helper handshake needs: the only callers
//! of these pipes are (a) the unprivileged main app that creates
//! them and (b) the elevated helper child it just spawned.
//!
//! This module wraps the raw `CreateNamedPipeW` Win32 call with a
//! hand-built `SECURITY_DESCRIPTOR` whose DACL grants pipe access
//! to exactly two principals:
//!
//! - The current process's logon user SID — so the unprivileged
//!   parent can read + write its own end.
//! - `BUILTIN\Administrators` — so the elevated helper (running
//!   under the user's elevated token, member of Administrators
//!   for the duration of the consent) can read + write the other
//!   end.
//!
//! Everyone else is implicitly denied. A non-admin attacker
//! running in the same desktop session can no longer race to
//! connect to the helper-side end of the pipe before the
//! legitimate helper does.
//!
//! The returned `tokio::NamedPipeServer` behaves identically to
//! one built via `ServerOptions::create` — same async I/O, same
//! `connect()` / `disconnect()` / Drop semantics. The only
//! observable difference is the tighter DACL.
//!
//! All FFI is confined to this module. Callers see a single safe
//! `create_secure_named_pipe_server` entry point.

#![cfg(windows)]

use std::ffi::OsStr;
use std::io;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::io::RawHandle;
use std::ptr;

use tokio::net::windows::named_pipe::NamedPipeServer;
use windows_sys::Win32::Foundation::{
    BOOL, CloseHandle, FALSE, HANDLE, INVALID_HANDLE_VALUE, LocalFree,
};
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

/// Default per-server pipe-side buffer size matching what tokio's
/// `ServerOptions::create` uses (64 KiB each direction).
const PIPE_BUF_BYTES: u32 = 64 * 1024;

/// Default pipe-server `nMaxInstances`. We always use 1 — the
/// elevated helper is the only legitimate client and the parent
/// creates exactly one server end per helper session.
const MAX_INSTANCES: u32 = 1;

/// Default-pipe-timeout in milliseconds. Tokio's
/// `ServerOptions::create` uses 0 (= 50 ms default); match that
/// for behavioural parity.
const DEFAULT_TIMEOUT_MS: u32 = 0;

/// Build a tokio `NamedPipeServer` whose underlying Win32 pipe
/// uses a custom DACL granting access only to the current user
/// and `BUILTIN\Administrators`.
///
/// Mirrors the open mode + flags tokio's
/// `ServerOptions::new().first_pipe_instance(true).max_instances(1)
/// .reject_remote_clients(true).create(name)` would set, with the
/// added security descriptor.
pub(crate) fn create_secure_named_pipe_server(name: &str) -> io::Result<NamedPipeServer> {
    // Validate name shape up front; the FFI call accepts wide
    // strings, but a name not starting with `\\.\pipe\` would
    // create a path-style file rather than a pipe.
    if !name.starts_with(r"\\.\pipe\") {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("pipe name must start with `\\\\.\\pipe\\`, got {name:?}"),
        ));
    }

    // Convert UTF-8 → UTF-16 with trailing NUL for the Win32
    // wide-string boundary.
    let wide: Vec<u16> = OsStr::new(name)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    // Build the security descriptor. The struct owns the
    // allocations until the pipe handle is created; after that
    // CreateNamedPipeW has copied the descriptor into the kernel
    // object and we can drop it.
    let mut sd = SecurityDescriptor::new()?;
    let mut sa = SECURITY_ATTRIBUTES {
        nLength: std::mem::size_of::<SECURITY_ATTRIBUTES>() as u32,
        lpSecurityDescriptor: sd.as_psd(),
        bInheritHandle: FALSE,
    };

    // CreateNamedPipeW expects open_mode + pipe_mode as plain
    // DWORDs. PIPE_ACCESS_DUPLEX is documented to layer with
    // FILE_FLAG_* in the same DWORD; all are u32 values. Cast
    // through a u32 binding so the type system doesn't infer one
    // of them as a strongly-typed alias and refuse the OR.
    //
    // FILE_FLAG_OVERLAPPED is required because tokio's
    // NamedPipeServer::from_raw_handle registers the handle with
    // its I/O completion port — synchronous pipes are rejected
    // with ERROR_INVALID_PARAMETER (87).
    let open_mode: u32 = (PIPE_ACCESS_DUPLEX as u32)
        | (FILE_FLAG_FIRST_PIPE_INSTANCE as u32)
        | (FILE_FLAG_OVERLAPPED as u32);
    let pipe_mode: u32 = (PIPE_TYPE_BYTE as u32)
        | (PIPE_READMODE_BYTE as u32)
        | (PIPE_WAIT as u32)
        | (PIPE_REJECT_REMOTE_CLIENTS as u32);

    // SAFETY: `wide` is a NUL-terminated UTF-16 buffer; `&mut sa`
    // is a stack-local with a valid SECURITY_DESCRIPTOR
    // pointer; all u32 args fit the documented Win32 ranges; the
    // returned HANDLE is checked against INVALID_HANDLE_VALUE
    // below before we hand it to tokio.
    let handle: HANDLE = unsafe {
        CreateNamedPipeW(
            wide.as_ptr(),
            open_mode,
            pipe_mode,
            MAX_INSTANCES,
            PIPE_BUF_BYTES,
            PIPE_BUF_BYTES,
            DEFAULT_TIMEOUT_MS,
            &mut sa,
        )
    };
    if handle.is_null() || handle == INVALID_HANDLE_VALUE {
        return Err(io::Error::last_os_error());
    }

    // SAFETY: handle is a kernel handle to a freshly-created
    // pipe. tokio's `from_raw_handle` takes ownership; we must
    // not call CloseHandle on it ourselves. The returned
    // NamedPipeServer's Drop closes it.
    let server = unsafe { NamedPipeServer::from_raw_handle(handle as RawHandle)? };

    // The descriptor's allocations are no longer referenced by
    // the kernel object (CreateNamedPipeW copied what it needed),
    // so we can let the SecurityDescriptor drop here.
    drop(sd);

    Ok(server)
}

/// Owns the four allocations that make up a pipe SECURITY_DESCRIPTOR:
///
/// - The current user's SID (heap, freed via CloseHandle on the
///   token + the boxed TOKEN_USER).
/// - The Administrators group SID (allocated by
///   AllocateAndInitializeSid; freed by FreeSid).
/// - The DACL buffer (`Vec<u8>`).
/// - The SECURITY_DESCRIPTOR struct itself (`Box<SECURITY_DESCRIPTOR>`).
///
/// Drop releases each in the correct order.
struct SecurityDescriptor {
    sd: Box<SECURITY_DESCRIPTOR>,
    /// Backing storage for the ACL. Lives as long as the
    /// SECURITY_DESCRIPTOR refers to it.
    _dacl: Vec<u8>,
    /// Token info buffer holding the current user's SID. The SID
    /// pointer inside the SD's DACL points into this allocation.
    _token_user_buf: Vec<u8>,
    admins_sid: PSID,
}

impl SecurityDescriptor {
    fn new() -> io::Result<Self> {
        let token_user_buf = current_user_token_user()?;
        // SAFETY: the buffer was sized + filled by GetTokenInformation;
        // the first field is a `TOKEN_USER` whose `User.Sid` points
        // into the same buffer.
        let user_sid: PSID =
            unsafe { (*(token_user_buf.as_ptr() as *const TOKEN_USER)).User.Sid };

        let admins_sid = allocate_admins_sid()?;

        // Build a DACL big enough for two ACEs. Each ACCESS_ALLOWED_ACE
        // header is 8 bytes; SID lengths vary (typical user SID is
        // ~28 bytes, Administrators SID is 16 bytes). Allocate with
        // generous slack; the actual size set by InitializeAcl is
        // what the kernel honours.
        let dacl_size: u32 = (std::mem::size_of::<ACL>()
            + 2 * (std::mem::size_of::<ACCESS_ALLOWED_ACE>()))
            as u32
            + 256;
        let mut dacl: Vec<u8> = vec![0u8; dacl_size as usize];

        // SAFETY: `dacl` is a freshly-zeroed buffer of `dacl_size`
        // bytes. `InitializeAcl` writes a valid ACL header.
        let init_ok: BOOL = unsafe {
            InitializeAcl(dacl.as_mut_ptr() as *mut ACL, dacl_size, ACL_REVISION as u32)
        };
        if init_ok == FALSE {
            // Drop will FreeSid the admins handle even on this
            // early return.
            unsafe { FreeSid(admins_sid) };
            return Err(io::Error::last_os_error());
        }

        // ACE 1 — current user, GENERIC_READ | GENERIC_WRITE.
        // SAFETY: `dacl` is initialised above; user_sid points to
        // a valid SID in the token_user_buf allocation.
        let add_ok: BOOL = unsafe {
            AddAccessAllowedAce(
                dacl.as_mut_ptr() as *mut ACL,
                ACL_REVISION as u32,
                GENERIC_READ | GENERIC_WRITE,
                user_sid,
            )
        };
        if add_ok == FALSE {
            unsafe { FreeSid(admins_sid) };
            return Err(io::Error::last_os_error());
        }

        // ACE 2 — Administrators, GENERIC_READ | GENERIC_WRITE.
        let add_ok: BOOL = unsafe {
            AddAccessAllowedAce(
                dacl.as_mut_ptr() as *mut ACL,
                ACL_REVISION as u32,
                GENERIC_READ | GENERIC_WRITE,
                admins_sid,
            )
        };
        if add_ok == FALSE {
            unsafe { FreeSid(admins_sid) };
            return Err(io::Error::last_os_error());
        }

        // Build the SD wrapping the DACL.
        let mut sd: Box<SECURITY_DESCRIPTOR> =
            Box::new(unsafe { std::mem::zeroed::<SECURITY_DESCRIPTOR>() });
        // SAFETY: sd is a freshly-zeroed allocation of the right
        // size; InitializeSecurityDescriptor sets its rev + size
        // fields.
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
        // SAFETY: `dacl` is initialised + has 2 ACEs; the SD owns
        // a non-owning pointer into it for the lifetime of this
        // SecurityDescriptor wrapper.
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
            // SAFETY: SID was returned by AllocateAndInitializeSid;
            // FreeSid is its documented inverse.
            unsafe { FreeSid(self.admins_sid) };
            self.admins_sid = std::ptr::null_mut();
        }
        // _dacl + _token_user_buf drop normally via Vec's Drop.
        // _sd drops via Box's Drop.
    }
}

/// Allocate the BUILTIN\Administrators SID via
/// AllocateAndInitializeSid. Caller must FreeSid the returned
/// handle (the `SecurityDescriptor` wrapper does this in Drop).
fn allocate_admins_sid() -> io::Result<PSID> {
    let mut admins: PSID = ptr::null_mut();
    // SAFETY: SECURITY_NT_AUTHORITY is a static const exposed by
    // windows-sys; the function takes a pointer to it. The two
    // sub-authority RIDs match Administrators (S-1-5-32-544).
    // The remaining 6 sub-authority slots are 0.
    let mut nt_authority = SECURITY_NT_AUTHORITY;
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

/// Read the current process's logon user SID into a heap buffer.
/// The returned `Vec<u8>`'s prefix bytes are a valid `TOKEN_USER`
/// followed by the SID it points at; callers cast the buffer's
/// start to `*const TOKEN_USER`.
fn current_user_token_user() -> io::Result<Vec<u8>> {
    let mut token_handle: HANDLE = INVALID_HANDLE_VALUE;
    // SAFETY: GetCurrentProcess returns a pseudo-handle to this
    // process; OpenProcessToken with TOKEN_QUERY is the documented
    // way to fetch the user SID.
    let ok: BOOL = unsafe {
        OpenProcessToken(
            GetCurrentProcess(),
            TOKEN_QUERY,
            &mut token_handle,
        )
    };
    if ok == FALSE {
        return Err(io::Error::last_os_error());
    }
    // First call: probe size.
    let mut needed: u32 = 0;
    // SAFETY: GetTokenInformation is documented to return FALSE
    // with ERROR_INSUFFICIENT_BUFFER on a too-small buffer and
    // write the required size into `needed`. We pass a 0-byte
    // buffer for the probe.
    let _ = unsafe {
        GetTokenInformation(
            token_handle,
            TokenUser,
            ptr::null_mut(),
            0,
            &mut needed,
        )
    };
    if needed == 0 {
        // SAFETY: token_handle was opened above.
        unsafe { CloseHandle(token_handle) };
        return Err(io::Error::other(
            "GetTokenInformation probe returned 0 bytes",
        ));
    }
    let mut buf = vec![0u8; needed as usize];
    // SAFETY: buf is `needed` bytes; the call writes a TOKEN_USER
    // followed by the SID body into it.
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
    // SAFETY: token_handle was opened above; close it before
    // returning the buffer.
    unsafe { CloseHandle(token_handle) };
    if let Some(e) = close_err {
        return Err(e);
    }
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Smoke test: the function returns a usable server handle on
    /// a fresh pipe name. We don't assert DACL contents here
    /// (that would require GetSecurityInfo + a parser test); the
    /// invariant we care about is that the call succeeds and
    /// produces a tokio NamedPipeServer. Driven through a tokio
    /// runtime because `NamedPipeServer::from_raw_handle` registers
    /// the handle with tokio's I/O reactor.
    #[tokio::test]
    async fn create_secure_pipe_server_smoke() {
        let id: u128 = u128::from_le_bytes(
            (0..16).map(|_| 0xAA_u8).collect::<Vec<u8>>().try_into().unwrap(),
        );
        let name = format!(r"\\.\pipe\copythat-snap-test-{id:032x}");
        let server = create_secure_named_pipe_server(&name);
        assert!(server.is_ok(), "create_secure_named_pipe_server: {server:?}");
        // Drop closes the handle.
    }
}

// Suppress warnings for the LocalFree import; reserved for future
// SetEntriesInAcl-based DACL builds where the API allocates the
// returned ACL itself.
#[allow(dead_code)]
fn _local_free_marker(p: *mut std::ffi::c_void) {
    unsafe { LocalFree(p as *mut _) };
}
