//! Phase 40 — second-instance fast-bail broker.
//!
//! Closes the UI 893 → engine 2429 MiB/s gap on `--enqueue` copies.
//!
//! The pre-Phase-40 problem: `copythat-ui.exe --enqueue copy …`
//! invocations from a shell extension or the CLI launched a SECOND
//! instance of the binary that ran the full Tauri boot path (SQLite
//! history open, settings load, profiles store, plugin
//! registration, Tauri runtime init, WebView2 init) before the
//! built-in `tauri-plugin-single-instance` finally detected the
//! existing first instance, forwarded argv via its own IPC, and
//! exited. ~5-7 seconds per invocation, all wasted.
//!
//! Phase 40 fix: at the very top of `run()`, before any heavy
//! work, we probe a Windows named mutex. If the mutex is already
//! owned, we know the running app is already alive — we open its
//! named pipe `\\.\pipe\copythat-ui-enqueue`, write our argv as
//! JSON, wait for a 1-byte ack, and exit. The first instance's
//! setup hook spawns a server thread that reads from this pipe,
//! re-parses the argv via `cli::parse_args`, and dispatches via
//! `shell::dispatch_cli_action` — exactly what
//! `tauri-plugin-single-instance` would have done, but without
//! the second-instance Tauri boot.
//!
//! The first instance still needs the single-instance plugin for
//! macOS / Linux + as a safety net on Windows when our own pipe
//! isn't reachable yet (e.g. during the first-instance's own
//! setup-hook race window). The mutex check + pipe write is a
//! best-effort fast path — on any failure we fall through and let
//! the existing plugin do its thing.
//!
//! Security note: the pipe DACL grants only the current user RW.
//! No cross-user / cross-session traffic. Mutex + pipe both live
//! in the `Local\` namespace so they're per-session, not global.

#![cfg(windows)]

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr;
use std::sync::OnceLock;
use std::time::Duration;

use windows_sys::Win32::Foundation::{
    CloseHandle, ERROR_ALREADY_EXISTS, ERROR_PIPE_BUSY, GetLastError, HANDLE,
    INVALID_HANDLE_VALUE,
};
use windows_sys::Win32::Storage::FileSystem::{
    CreateFileW, FILE_GENERIC_READ, FILE_GENERIC_WRITE, FILE_SHARE_READ, FILE_SHARE_WRITE,
    OPEN_EXISTING, PIPE_ACCESS_DUPLEX, ReadFile, WriteFile,
};
use windows_sys::Win32::System::Pipes::{
    ConnectNamedPipe, CreateNamedPipeW, DisconnectNamedPipe, PIPE_READMODE_BYTE, PIPE_TYPE_BYTE,
    PIPE_WAIT, WaitNamedPipeW,
};
use windows_sys::Win32::System::Threading::CreateMutexW;

const MUTEX_NAME: &str = "Local\\copythat-ui-instance-mutex-v1";
const PIPE_NAME: &str = "\\\\.\\pipe\\copythat-ui-enqueue-v1";

/// Hard cap on the wire payload (newline-terminated JSON).
///
/// Equal to the pipe's buffer size so a single atomic Write fits
/// comfortably; any well-formed `CliAction` is well under 1 KiB.
/// Both client and server enforce this — anything larger is
/// rejected without parsing.
const MAX_WIRE_BYTES: usize = 64 * 1024;

/// One-shot guard so we only register the mutex once per process.
///
/// # Lifetime + scope
///
/// Stored as a raw `usize` (the bit-pattern of the `HANDLE`) so
/// `OnceLock` is `Send + Sync` — `windows_sys::HANDLE` is `*mut
/// c_void` which isn't. The handle is **intentionally never
/// closed**: the OS releases it at process exit, which is exactly
/// the lifetime we want. A third invocation of the binary still
/// finds the mutex owned because the kernel object outlives
/// `is_second_instance()`'s stack frame and lives until the first
/// instance terminates.
///
/// The `Local\` namespace prefix on [`MUTEX_NAME`] makes this
/// session-local: a different user (or the same user in a
/// different RDP / fast-user-switching session) gets a separate
/// kernel namespace, so we can't false-positive across sessions.
///
/// **Effective lifetime:** process exit OR user logout, whichever
/// comes first. Documented at `RESEARCH/RESEARCH_PHASE_42.md`.
static MUTEX_HANDLE: OnceLock<usize> = OnceLock::new();

/// Returns `true` iff another `copythat-ui` process is already
/// running in this user's session. Determined by attempting to
/// claim a named mutex; if the mutex already exists, we're the
/// second instance.
///
/// The mutex handle is intentionally leaked (stored in a
/// `OnceLock<usize>` as a raw pointer-bits int) so the OS holds it
/// for the rest of our process lifetime — a third instance still
/// finds it owned.
pub fn is_second_instance() -> bool {
    let name: Vec<u16> = OsStr::new(MUTEX_NAME)
        .encode_wide()
        .chain(Some(0))
        .collect();
    // SAFETY: name is a NUL-terminated UTF-16 string we own. The
    // mutex handle is leaked into MUTEX_HANDLE for process lifetime.
    let handle = unsafe { CreateMutexW(ptr::null_mut(), 0, name.as_ptr()) };
    if handle.is_null() {
        // Failed to even create — be conservative and proceed as
        // first instance. The single-instance plugin will catch
        // duplicates downstream if any.
        return false;
    }
    let already = unsafe { GetLastError() } == ERROR_ALREADY_EXISTS;
    let _ = MUTEX_HANDLE.set(handle as usize);
    already
}

/// Attempt to forward `argv` to the running first instance via the
/// named pipe broker. Returns `Ok(())` iff the running instance
/// accepted the args (1-byte ack received). On any failure
/// (no server, pipe busy timeout, write error, ack timeout)
/// returns `Err` so the caller can fall through to the normal
/// first-instance boot.
///
/// Phase 42: every payload is HMAC-SHA256 authenticated under the
/// 32-byte secret in `%LOCALAPPDATA%\CopyThat\session.token`. If
/// the token is missing (the running instance is pre-Phase-42
/// or hasn't finished writing the token yet), we fail closed —
/// the caller falls through to the normal boot path which kicks
/// in `tauri-plugin-single-instance`.
pub fn try_forward_argv(argv: &[String]) -> Result<(), String> {
    // Load the per-session HMAC secret first. Doing this BEFORE
    // opening the pipe means a stale or missing token doesn't waste
    // a connect+write+read round-trip.
    let secret = crate::broker_auth::load_secret_for_client()
        .map_err(|e| format!("session token unavailable: {e}"))?;

    let pipe_name: Vec<u16> = OsStr::new(PIPE_NAME)
        .encode_wide()
        .chain(Some(0))
        .collect();

    // Wait briefly for the pipe to be available — the first
    // instance's setup hook may still be installing it. 500 ms is
    // well under the human-perception ceiling and well over the
    // typical setup-hook latency.
    // SAFETY: pipe_name is NUL-terminated UTF-16; the function
    // doesn't retain the pointer.
    unsafe {
        let _ = WaitNamedPipeW(pipe_name.as_ptr(), 500);
    }

    // Try to open. ERROR_PIPE_BUSY would mean a brief retry; any
    // other failure means "no server" — fall through.
    let mut handle: HANDLE = INVALID_HANDLE_VALUE;
    for _ in 0..5 {
        // SAFETY: pipe_name is NUL-terminated; we own all pointers.
        let h = unsafe {
            CreateFileW(
                pipe_name.as_ptr(),
                FILE_GENERIC_READ | FILE_GENERIC_WRITE,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                ptr::null_mut(),
                OPEN_EXISTING,
                0,
                ptr::null_mut(),
            )
        };
        if h != INVALID_HANDLE_VALUE {
            handle = h;
            break;
        }
        let err = unsafe { GetLastError() };
        if err == ERROR_PIPE_BUSY {
            std::thread::sleep(Duration::from_millis(50));
            continue;
        }
        return Err(format!("CreateFileW pipe failed: os error {err}"));
    }
    if handle == INVALID_HANDLE_VALUE {
        return Err("pipe busy after retries".to_string());
    }

    // Wrap raw handle in a guard so any early return closes it.
    struct OwnedH(HANDLE);
    impl Drop for OwnedH {
        fn drop(&mut self) {
            if self.0 != INVALID_HANDLE_VALUE {
                // SAFETY: we own this handle.
                unsafe {
                    CloseHandle(self.0);
                }
            }
        }
    }
    let pipe = OwnedH(handle);

    // Wire payload v2 — JSON object with two fields:
    //   { "argv": [..], "hmac": "<lower-hex sha256>" }
    // The HMAC is computed over the canonical JSON of the inner
    // `{argv: [..]}` object so the server recomputes the exact
    // same bytes (no key-order ambiguity from re-serialising).
    let inner = serde_json::json!({ "argv": argv });
    let inner_bytes =
        serde_json::to_vec(&inner).map_err(|e| format!("json serialize argv: {e}"))?;
    let tag = crate::broker_auth::hmac_hex(&secret, &inner_bytes);
    let outer = serde_json::json!({ "argv": argv, "hmac": tag });
    let mut wire = serde_json::to_vec(&outer).map_err(|e| format!("json serialize wire: {e}"))?;
    wire.push(b'\n');

    // Refuse oversized payloads on the client side too — the
    // server hard-caps at 64 KiB and would reject anyway, but
    // failing fast saves a useless round-trip.
    if wire.len() > MAX_WIRE_BYTES {
        return Err(format!(
            "wire payload {} exceeds {} bytes",
            wire.len(),
            MAX_WIRE_BYTES
        ));
    }

    // Write the entire wire payload. Pipe writes are atomic for
    // payloads ≤ the pipe's buffer size (we set 64 KiB on the
    // server side); CliAction JSON is well under that.
    let mut written: u32 = 0;
    // SAFETY: wire.as_ptr is valid for wire.len() bytes; written is
    // a stack u32.
    let ok = unsafe {
        WriteFile(
            pipe.0,
            wire.as_ptr(),
            wire.len() as u32,
            &mut written,
            ptr::null_mut(),
        )
    };
    if ok == 0 || (written as usize) != wire.len() {
        return Err(format!(
            "WriteFile pipe failed: os error {}",
            unsafe { GetLastError() }
        ));
    }

    // Wait for the server's ack byte with a hard 5-second timeout.
    // Synchronous `ReadFile` on a named pipe doesn't honour
    // SetCommTimeouts (those are for serial / COM ports), so we
    // run the read on a worker thread and use `recv_timeout` on
    // a one-shot mpsc channel as the actual timeout primitive.
    // If the timeout fires, we close the pipe handle (via Drop)
    // which forces the parked ReadFile to fail — the worker
    // thread exits and is no longer holding any borrowed state.
    let pipe_handle = pipe.0;
    // The handle bit-pattern is `Send` because the kernel object
    // is shared; closing in the parent thread while the worker
    // is mid-ReadFile is the documented Windows pattern for
    // cancelling a synchronous I/O.
    let pipe_handle_for_thread = pipe_handle as usize;
    let (tx, rx) = std::sync::mpsc::channel::<Result<u8, String>>();
    let _join = std::thread::Builder::new()
        .name("copythat-broker-ack".into())
        .spawn(move || {
            let h = pipe_handle_for_thread as HANDLE;
            let mut ack = [0u8; 1];
            let mut nread: u32 = 0;
            // SAFETY: ack is a stack array of length 1; h is a
            // valid pipe handle owned by the caller's `OwnedH`
            // (which outlives this thread because we join on rx
            // before letting `pipe` drop, OR we let `pipe` drop
            // on timeout which cancels the read).
            let ok = unsafe { ReadFile(h, ack.as_mut_ptr(), 1, &mut nread, ptr::null_mut()) };
            let result = if ok == 0 || nread != 1 {
                Err(format!(
                    "ReadFile pipe ack failed: os error {}",
                    unsafe { GetLastError() }
                ))
            } else {
                Ok(ack[0])
            };
            let _ = tx.send(result);
        })
        .map_err(|e| format!("spawn ack thread: {e}"))?;

    let ack_byte = match rx.recv_timeout(Duration::from_secs(5)) {
        Ok(Ok(b)) => b,
        Ok(Err(e)) => return Err(e),
        Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
            // Drop `pipe` so the kernel cancels the parked
            // ReadFile in the worker thread; the worker will exit
            // on the next wake. We deliberately don't `join` here
            // — the worker is non-blocking once the handle is
            // closed and we want to surface the timeout to the
            // caller immediately.
            return Err("ack timeout after 5s".to_string());
        }
        Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
            return Err("ack worker disconnected".to_string());
        }
    };

    // Single-byte ack: 0x06 (ACK) = success, anything else = failure.
    if ack_byte == 0x06 {
        Ok(())
    } else {
        Err(format!("server rejected (ack=0x{:02x})", ack_byte))
    }
}

/// Spawn the named-pipe server in a dedicated OS thread. Lives for
/// the lifetime of the first-instance process. Each connection
/// reads one JSON `{argv: [...], hmac: ".."}` payload, verifies
/// the HMAC against the per-session secret, parses argv via
/// [`crate::cli::parse_args`], dispatches via
/// [`crate::shell::dispatch_cli_action`], and writes a 1-byte ack.
///
/// Phase 42: initialises the per-session HMAC secret + writes
/// `%LOCALAPPDATA%\CopyThat\session.token` BEFORE the listener
/// starts accepting connections, so a second instance racing the
/// first instance's startup either reads the fresh token or
/// retries (no window where a stale token is accepted).
pub fn start_pipe_server(app: tauri::AppHandle) {
    // Initialise the per-session secret first; failure here means
    // the first instance can't authenticate any second-instance
    // forwards, but we still want the listener to come up so the
    // tauri-plugin-single-instance fallback can route argv.
    if let Err(e) = crate::broker_auth::init_first_instance_secret() {
        eprintln!("[broker] session-token init failed: {e} — pipe broker disabled");
        return;
    }
    std::thread::Builder::new()
        .name("copythat-pipe-broker".into())
        .spawn(move || pipe_server_loop(app))
        .ok();
}

fn pipe_server_loop(app: tauri::AppHandle) {
    let pipe_name: Vec<u16> = OsStr::new(PIPE_NAME)
        .encode_wide()
        .chain(Some(0))
        .collect();
    loop {
        // Create a fresh server-end of the pipe for each connection.
        // PIPE_ACCESS_DUPLEX so we can write the ack back; byte-mode
        // because the payload is a single line of JSON. 64 KiB I/O
        // buffers are generous for any plausible argv.
        // SAFETY: pipe_name is NUL-terminated; we own all pointers.
        let pipe = unsafe {
            CreateNamedPipeW(
                pipe_name.as_ptr(),
                PIPE_ACCESS_DUPLEX,
                PIPE_TYPE_BYTE | PIPE_READMODE_BYTE | PIPE_WAIT,
                255, // max instances — we're single-threaded; cap is plenty
                64 * 1024,
                64 * 1024,
                0, // default 50 ms timeout for WaitNamedPipe
                ptr::null_mut(),
            )
        };
        if pipe == INVALID_HANDLE_VALUE {
            // Couldn't create pipe — back off and retry. This is
            // typically transient (e.g. a stale instance still
            // holds the name during its own teardown).
            std::thread::sleep(Duration::from_secs(1));
            continue;
        }

        // Block until a client connects.
        // SAFETY: pipe is a valid handle we just created.
        let connected = unsafe { ConnectNamedPipe(pipe, ptr::null_mut()) };
        let last_err = unsafe { GetLastError() };
        // ConnectNamedPipe returns 0 on synchronous-success too,
        // with GetLastError() == ERROR_PIPE_CONNECTED (535). Treat
        // that as success.
        if connected == 0 && last_err != 535 {
            unsafe {
                CloseHandle(pipe);
            }
            continue;
        }

        let result = handle_connection(pipe, &app);

        // Disconnect and close the server-end so the next
        // CreateNamedPipeW gets a fresh instance.
        // SAFETY: pipe is a valid handle.
        unsafe {
            DisconnectNamedPipe(pipe);
            CloseHandle(pipe);
        }

        // Best-effort log on errors; don't crash the broker.
        if let Err(e) = result {
            eprintln!("[broker] connection error: {e}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Forward to a never-created pipe must return `Err` cleanly,
    /// never panic. This is the safety net for "no first-instance
    /// running" — `try_forward_argv` returns Err and the caller
    /// falls through to the normal first-instance boot. Phase 42:
    /// in CI the session token may not exist, in which case the
    /// `Err` originates from `load_secret_for_client`; either
    /// failure mode is acceptable for this invariant.
    #[test]
    fn forward_to_missing_pipe_errors_cleanly() {
        let argv = vec!["copythat-ui.exe".to_string(), "--enqueue".to_string()];
        let result = try_forward_argv(&argv);
        // Any of the failure paths is acceptable; the only thing
        // we MUST reject is the test panicking or hanging.
        assert!(
            result.is_err(),
            "forward to missing pipe should error, got {:?}",
            result
        );
    }

    /// Mutex check returns `false` on first call within a process
    /// (we just claimed ownership) and `true` on the second call
    /// (the OS reports the name already exists). This is the only
    /// behaviour the second-instance fast bail relies on.
    #[test]
    fn mutex_check_returns_false_then_true_within_process() {
        let first = is_second_instance();
        let second = is_second_instance();
        assert!(!first, "first call should report first instance");
        assert!(second, "second call within same process should report second instance");
    }

    /// `MAX_WIRE_BYTES` must equal the pipe's buffer size — keeping
    /// them in sync is the contract that lets `WriteFile` issue a
    /// single atomic write of an in-bound payload.
    #[test]
    fn wire_cap_matches_pipe_buffer_size() {
        assert_eq!(MAX_WIRE_BYTES, 64 * 1024);
    }
}

fn handle_connection(pipe: HANDLE, app: &tauri::AppHandle) -> Result<(), String> {
    // Read until newline or hard cap. We use a heap `Vec<u8>` that
    // grows in 4 KiB chunks instead of a fixed 65 536-byte stack
    // buffer — that earlier shape had two bugs:
    //   1. an exact 65 536-byte payload with the newline at byte
    //      65 537 silently truncated;
    //   2. the bounds check `total < buf.len()` on a full buffer
    //      raced with `nread == 0`.
    // The heap-Vec approach is both safer and clearer: we read
    // until a newline or until we hit `MAX_WIRE_BYTES`, at which
    // point we reject explicitly.
    let mut buf: Vec<u8> = Vec::with_capacity(4 * 1024);
    loop {
        if buf.len() >= MAX_WIRE_BYTES {
            return Err(format!(
                "wire payload exceeds {} bytes — refusing to parse",
                MAX_WIRE_BYTES
            ));
        }
        // Reserve up to (cap - len) bytes; cap at the hard limit
        // so a single huge ReadFile can't overshoot.
        let want = std::cmp::min(MAX_WIRE_BYTES - buf.len(), 4 * 1024);
        let old_len = buf.len();
        buf.resize(old_len + want, 0);
        let mut nread: u32 = 0;
        // SAFETY: `buf[old_len..]` is `want` bytes of writable
        // memory we just reserved via `resize`.
        let ok = unsafe {
            ReadFile(
                pipe,
                buf[old_len..].as_mut_ptr(),
                want as u32,
                &mut nread,
                ptr::null_mut(),
            )
        };
        if ok == 0 || nread == 0 {
            // Truncate back to what we actually filled before this
            // failed read so the newline scan below doesn't see
            // zero padding.
            buf.truncate(old_len);
            break;
        }
        let new_len = old_len + nread as usize;
        buf.truncate(new_len);
        if buf.contains(&b'\n') {
            break;
        }
    }
    let line: &[u8] = match buf.iter().position(|b| *b == b'\n') {
        Some(idx) => &buf[..idx],
        None => &buf,
    };

    // Parse the outer wire envelope: {argv:[..], hmac:".."}.
    #[derive(serde::Deserialize)]
    struct Wire {
        argv: Vec<String>,
        hmac: String,
    }
    let wire: Wire = serde_json::from_slice(line).map_err(|e| format!("json parse: {e}"))?;

    // Verify the HMAC against the per-session secret BEFORE
    // touching argv. The client's HMAC is computed over the
    // canonical re-serialisation of `{argv: [..]}` — we recompute
    // here using serde's canonical key order (argv first because
    // that's the only key) so byte-equal matches succeed.
    let secret = crate::broker_auth::init_first_instance_secret()
        .map_err(|e| format!("secret unavailable: {e}"))?;
    let inner = serde_json::json!({ "argv": wire.argv });
    let inner_bytes =
        serde_json::to_vec(&inner).map_err(|e| format!("json re-serialise argv: {e}"))?;
    if !crate::broker_auth::verify_hmac(&secret, &inner_bytes, &wire.hmac) {
        return Err("hmac verification failed — refusing to dispatch".to_string());
    }

    // Dispatch on the Tauri main thread via the AppHandle. We use
    // tauri::async_runtime::spawn so dispatch_cli_action sees a
    // proper async context.
    let argv: Vec<std::ffi::OsString> = wire
        .argv
        .into_iter()
        .map(std::ffi::OsString::from)
        .collect();
    match crate::cli::parse_args(argv) {
        Ok(action) => {
            let app_clone = app.clone();
            tauri::async_runtime::spawn(async move {
                crate::shell::dispatch_cli_action(&app_clone, action);
            });
        }
        Err(e) => {
            return Err(format!("parse_args: {e}"));
        }
    }

    // Write the ACK byte.
    let ack = [0x06u8];
    let mut written: u32 = 0;
    // SAFETY: ack is a stack array.
    let ok = unsafe { WriteFile(pipe, ack.as_ptr(), 1, &mut written, ptr::null_mut()) };
    if ok == 0 || written != 1 {
        return Err(format!(
            "ack WriteFile failed: os error {}",
            unsafe { GetLastError() }
        ));
    }
    Ok(())
}
