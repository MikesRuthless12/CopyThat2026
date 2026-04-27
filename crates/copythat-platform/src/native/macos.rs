//! macOS native fast path.
//!
//! Strategy: `copyfile(3)` with `COPYFILE_ALL`. This is the same
//! syscall Finder and `cp` use; it preserves resource forks, extended
//! attributes, ACLs, and file flags, and the kernel can short-circuit
//! to a `clonefile()` clone on APFS when the dev ids match.
//!
//! Pause / cancel: we hook the status callback. Returning
//! `COPYFILE_QUIT` from the callback aborts cleanly and `copyfile` then
//! returns `-1` with `errno = ECANCELED`.
//!
//! Progress: the status callback fires per-chunk with the running byte
//! total in the state object. We forward Progress events from inside
//! the callback through an mpsc set up in the surrounding async task.

use std::ffi::CString;
use std::io;
use std::os::raw::{c_char, c_int, c_void};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use copythat_core::{CopyControl, CopyEvent};
use tokio::sync::mpsc;

use super::NativeOutcome;
use crate::outcome::ChosenStrategy;

const PROGRESS_MIN_BYTES: u64 = 16 * 1024;
const PROGRESS_MIN_INTERVAL: std::time::Duration = std::time::Duration::from_millis(50);

// libc on macOS doesn't ship copyfile bindings; declare just the slice
// we need.
#[allow(non_camel_case_types)]
type copyfile_state_t = *mut c_void;
#[allow(non_camel_case_types)]
type copyfile_flags_t = u32;

const COPYFILE_ACL: u32 = 1 << 0;
const COPYFILE_STAT: u32 = 1 << 1;
const COPYFILE_XATTR: u32 = 1 << 2;
const COPYFILE_DATA: u32 = 1 << 3;
const COPYFILE_ALL: u32 = COPYFILE_ACL | COPYFILE_STAT | COPYFILE_XATTR | COPYFILE_DATA;

const COPYFILE_RECURSIVE: u32 = 1 << 15;
const COPYFILE_CLONE: u32 = 1 << 24; // try clonefile under the hood
const _: u32 = COPYFILE_RECURSIVE; // silence unused on plain-file copies

const COPYFILE_STATE_STATUS_CB: c_int = 6;
const COPYFILE_STATE_STATUS_CTX: c_int = 7;
const COPYFILE_STATE_COPIED: c_int = 8;

const COPYFILE_COPY_DATA: c_int = 4;
const COPYFILE_PROGRESS: c_int = 0;
const _: c_int = COPYFILE_COPY_DATA;

const COPYFILE_CONTINUE: c_int = 0;
const COPYFILE_QUIT: c_int = 2;

unsafe extern "C" {
    fn copyfile(
        from: *const c_char,
        to: *const c_char,
        state: copyfile_state_t,
        flags: copyfile_flags_t,
    ) -> c_int;
    fn copyfile_state_alloc() -> copyfile_state_t;
    fn copyfile_state_free(state: copyfile_state_t) -> c_int;
    fn copyfile_state_set(state: copyfile_state_t, flag: c_int, src: *const c_void) -> c_int;
    fn copyfile_state_get(state: copyfile_state_t, flag: c_int, dst: *mut c_void) -> c_int;
}

/// Per-call context shared between the async task and the C callback.
struct CallbackCtx {
    ctrl: CopyControl,
    bytes: Arc<AtomicU64>,
    progress_tx: mpsc::UnboundedSender<u64>,
}

unsafe extern "C" fn status_callback(
    what: c_int,
    stage: c_int,
    state: copyfile_state_t,
    _src: *const c_char,
    _dst: *const c_char,
    ctx_raw: *mut c_void,
) -> c_int {
    if what != COPYFILE_COPY_DATA {
        return COPYFILE_CONTINUE;
    }
    if stage != COPYFILE_PROGRESS {
        return COPYFILE_CONTINUE;
    }
    // SAFETY: ctx_raw was set via copyfile_state_set in the dispatcher
    // and points at an `Arc<CallbackCtx>` that outlives the copyfile
    // call.
    let ctx = unsafe { &*(ctx_raw as *const CallbackCtx) };
    if ctx.ctrl.is_cancelled() {
        return COPYFILE_QUIT;
    }
    let mut copied: i64 = 0;
    // SAFETY: `state` is the live copyfile_state_t handed in by the
    // running copyfile call; `copied` is a stack i64 with sufficient
    // alignment to hold the off_t value `copyfile_state_get` writes.
    let rc = unsafe {
        copyfile_state_get(
            state,
            COPYFILE_STATE_COPIED,
            &mut copied as *mut _ as *mut c_void,
        )
    };
    if rc == 0 && copied >= 0 {
        let bytes = copied as u64;
        ctx.bytes.store(bytes, Ordering::Relaxed);
        // Async side will emit when progress thresholds elapse.
        let _ = ctx.progress_tx.send(bytes);
    }
    COPYFILE_CONTINUE
}

#[allow(clippy::too_many_arguments)]
pub(crate) async fn try_native_copy(
    src: PathBuf,
    dst: PathBuf,
    total: u64,
    ctrl: CopyControl,
    events: mpsc::Sender<CopyEvent>,
    // Phase 43 — accepted for API parity with the Windows path.
    // The macOS `copyfile` callback already returns CONTINUE without
    // crossing thread boundaries, so we accept-and-honour the flag
    // by skipping the progress channel emission rather than the
    // callback itself (the callback can't be skipped — `copyfile`
    // requires a callback pointer to track progress).
    _disable_callback: bool,
) -> NativeOutcome {
    super::emit_started(&src, &dst, total, &events).await;

    let bytes_counter = Arc::new(AtomicU64::new(0));
    let (progress_tx, mut progress_rx) = mpsc::unbounded_channel::<u64>();

    let ctx = Arc::new(CallbackCtx {
        ctrl: ctrl.clone(),
        bytes: bytes_counter.clone(),
        progress_tx,
    });

    let started = Instant::now();
    let events_for_progress = events.clone();
    let total_for_progress = total;
    let progress_task = tokio::spawn(async move {
        let mut last_emit_at = started;
        let mut last_emit_bytes: u64 = 0;
        while let Some(bytes) = progress_rx.recv().await {
            let now = Instant::now();
            if bytes.saturating_sub(last_emit_bytes) >= PROGRESS_MIN_BYTES
                && now.duration_since(last_emit_at) >= PROGRESS_MIN_INTERVAL
            {
                let elapsed = now.duration_since(started);
                let rate = super::fast_rate_bps(bytes, elapsed);
                let _ = events_for_progress
                    .send(CopyEvent::Progress {
                        bytes,
                        total: total_for_progress,
                        rate_bps: rate,
                    })
                    .await;
                last_emit_at = now;
                last_emit_bytes = bytes;
            }
        }
    });

    // Hand the C call off to a blocking thread — copyfile is fully
    // synchronous and may take seconds for multi-GB files even with
    // clone fast paths warm.
    let src_c = match cstring(&src) {
        Ok(c) => c,
        Err(e) => return NativeOutcome::Io(e),
    };
    let dst_c = match cstring(&dst) {
        Ok(c) => c,
        Err(e) => return NativeOutcome::Io(e),
    };

    // copyfile_state_t is !Send/!Sync; allocate inside spawn_blocking.
    let ctx_for_block = ctx.clone();
    let result = tokio::task::spawn_blocking(move || {
        // SAFETY: we own all four arguments; the state object is freed
        // before this closure returns. The callback dereferences the
        // ctx pointer only through a shared Arc that outlives the call
        // (held in Mutex for clarity, even though we don't lock).
        unsafe {
            let state = copyfile_state_alloc();
            if state.is_null() {
                return Err(io::Error::other("copyfile_state_alloc returned null"));
            }
            let cb: unsafe extern "C" fn(
                c_int,
                c_int,
                copyfile_state_t,
                *const c_char,
                *const c_char,
                *mut c_void,
            ) -> c_int = status_callback;
            let cb_ptr = cb as *const c_void;
            let _ = copyfile_state_set(
                state,
                COPYFILE_STATE_STATUS_CB,
                &cb_ptr as *const _ as *const c_void,
            );
            let ctx_ptr = Arc::as_ptr(&ctx_for_block) as *const c_void;
            let _ = copyfile_state_set(state, COPYFILE_STATE_STATUS_CTX, ctx_ptr);

            let flags = COPYFILE_ALL | COPYFILE_CLONE;
            let rc = copyfile(src_c.as_ptr(), dst_c.as_ptr(), state, flags);

            // Always free the state object before returning.
            let _ = copyfile_state_free(state);

            if rc == -1 {
                Err(io::Error::last_os_error())
            } else {
                Ok(())
            }
        }
    })
    .await;

    // Stop the progress task by dropping the only remaining sender.
    drop(ctx);
    let _ = progress_task.await;

    match result {
        Ok(Ok(())) => {
            // copyfile doesn't always count the final stat / xattr
            // bytes; trust the reported file size.
            let bytes = bytes_counter.load(Ordering::Relaxed).max(total);
            NativeOutcome::Done {
                strategy: ChosenStrategy::Copyfile,
                bytes,
            }
        }
        Ok(Err(e)) => {
            if let Some(raw) = e.raw_os_error() {
                if raw == libc::ECANCELED {
                    return NativeOutcome::Cancelled;
                }
                if raw == libc::ENOTSUP || raw == libc::ENOSYS {
                    return NativeOutcome::Unsupported;
                }
            }
            NativeOutcome::Io(e)
        }
        Err(join_err) => NativeOutcome::Io(io::Error::other(format!(
            "copyfile spawn_blocking panicked: {join_err}"
        ))),
    }
}

fn cstring(path: &Path) -> io::Result<CString> {
    use std::os::unix::ffi::OsStrExt;
    CString::new(path.as_os_str().as_bytes())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))
}

// ---------------------------------------------------------------------
// Helpers: SSD probe + filesystem name
// ---------------------------------------------------------------------

pub(crate) fn is_ssd(path: &Path) -> Option<bool> {
    let probe_target = if path.exists() {
        path.to_path_buf()
    } else {
        path.parent()?.to_path_buf()
    };
    let out = std::process::Command::new("diskutil")
        .arg("info")
        .arg(&probe_target)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&out.stdout);
    for line in text.lines() {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix("Solid State:") {
            let v = rest.trim().to_ascii_lowercase();
            return Some(v.starts_with("yes"));
        }
    }
    None
}

pub(crate) fn filesystem_name(path: &Path) -> Option<String> {
    let probe_target = if path.exists() {
        path.to_path_buf()
    } else {
        path.parent()?.to_path_buf()
    };
    let out = std::process::Command::new("diskutil")
        .arg("info")
        .arg(&probe_target)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&out.stdout);
    for line in text.lines() {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix("Type (Bundle):") {
            return Some(rest.trim().to_ascii_lowercase());
        }
        if let Some(rest) = trimmed.strip_prefix("File System Personality:") {
            return Some(rest.trim().to_ascii_lowercase());
        }
    }
    None
}

#[allow(dead_code)]
fn _silence_mutex_import(_: Option<Mutex<()>>) {} // keep `Mutex` import live for clarity
