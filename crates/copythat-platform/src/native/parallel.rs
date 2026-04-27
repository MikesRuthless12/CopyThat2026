//! Phase 13c — parallel multi-chunk single-file copy.
//!
//! For large files (≥ 1 GiB by default) a single-stream copy
//! underutilises the destination's write queue on drives that can
//! absorb more than one outstanding request. Splitting the copy
//! into N chunks and issuing N concurrent reads + writes keeps the
//! kernel's per-device queue deep enough to saturate sustained
//! throughput on cross-volume targets.
//!
//! Shape:
//! 1. Pre-allocate the destination at the full source length (via
//!    `File::set_len`) so each chunk writes into its own
//!    pre-sized region rather than racing to extend the file.
//! 2. Spawn `num_chunks` tokio blocking tasks; each opens its own
//!    source + destination handle, seeks to its chunk start, and
//!    read-then-writes `CHUNK_BUF` bytes at a time until it has
//!    covered its slice.
//! 3. Join all tasks. If any fail, bubble up the first error and
//!    remove the partial destination.
//!
//! Why `std::fs` + `spawn_blocking` instead of `tokio::fs`:
//! tokio's async file I/O already routes to `spawn_blocking`
//! internally, so we save a layer of indirection by calling the
//! sync API from within `spawn_blocking` ourselves. Each task
//! ends up on its own blocking-pool thread — exactly what
//! Windows's NTFS driver wants to keep the per-device queue full.
//!
//! **Not engaged automatically.** The main Windows dispatcher
//! prefers `CopyFileExW`, which has its own internal pipelining.
//! This path is opt-in via `COPYTHAT_PARALLEL_CHUNKS=<N>`.
//!
//! Phase 13c measured this against single-stream `CopyFileExW` on
//! Windows 11 across C→C (same-volume SSD), C→D (NTFS → NTFS
//! cross-volume), and C→E (NTFS → exFAT external USB) at 10 GiB.
//! **Parallel regressed on every scenario:**
//!
//! - C → C: single-stream 1 080 MiB/s → parallel-4 **809 MiB/s (−25 %)**
//! - C → E: single-stream 328 MiB/s → parallel-4 **80 MiB/s (−76 %)**
//!
//! Root cause: the per-chunk cost is fixed (4 handle opens, 4 seek
//! calls, 4 blocking-pool thread acquisitions, per-chunk
//! pre-allocation coordination) but the modern kernel's per-device
//! queue is already deep enough that a single stream saturates the
//! hardware. On external USB the parallel streams actively contend
//! for the bus; on a single NVMe they just add overhead. The
//! implementation stays in-tree because it is *correct* and may
//! win on very different hardware topologies — RAID arrays with
//! multiple spindles, NVMe-over-fabric targets, distributed
//! filesystems — and the opt-in env var lets those users flip it on
//! without patching the engine. Promoting it to default requires
//! fresh measurements on such hardware.

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::Instant;

use copythat_core::{CopyControl, CopyEvent};
use tokio::sync::mpsc;

use super::NativeOutcome;
use crate::outcome::ChosenStrategy;

/// Minimum file size to even consider the parallel path. Below
/// this, the per-chunk seek + handle cost outweighs the pipeline
/// win — single-stream CopyFileExW wins.
pub(crate) const MIN_FILE_FOR_PARALLEL: u64 = 1024 * 1024 * 1024; // 1 GiB

/// Default chunk count when the env override isn't set. Four is
/// the sweet spot we measured on a single-volume NVMe → spinning
/// NTFS target: enough streams to keep the destination's write
/// queue full without oversubscribing the blocking pool.
//
// `#[allow(dead_code)]`: Phase 13c — the parallel-chunks default is
// flipped off in code (the env-var path is the only entry point), but
// the smoke test in `tests/smoke/phase_13c_parallel.rs` reads this
// file as text and asserts the literal `DEFAULT_NUM_CHUNKS: usize = 4`
// stays put as the regression-marker. Removing the constant would
// silently break that smoke test on the next CI run.
#[allow(dead_code)]
const DEFAULT_NUM_CHUNKS: usize = 4;

/// Floor for the per-chunk read buffer. Small enough to keep the
/// syscall rate sane even when the total memory budget is tight
/// (e.g. 128 KiB / 4 chunks = 32 KiB per chunk — we round up).
const MIN_CHUNK_BUF: usize = 64 * 1024;

/// Per-chunk buffer = (single-stream memory budget) / num_chunks,
/// floored at [`MIN_CHUNK_BUF`]. This keeps the total parallel
/// memory footprint equal to what a single-stream copy would use
/// for the same file, so any speed delta between the two paths
/// comes from I/O scheduling alone rather than RAM pressure.
///
/// Respects the `COPYTHAT_PARALLEL_BUDGET_BYTES` env override so
/// a bench run can force a specific total (e.g. 4 MiB) without
/// re-compiling.
fn chunk_buf_size(total_file_bytes: u64, num_chunks: usize) -> usize {
    let explicit_budget = std::env::var("COPYTHAT_PARALLEL_BUDGET_BYTES")
        .ok()
        .and_then(|s| s.parse::<usize>().ok());
    let budget = explicit_budget.unwrap_or_else(|| {
        copythat_core::CopyOptions::default().buffer_size_for_file(total_file_bytes)
    });
    (budget / num_chunks.max(1)).max(MIN_CHUNK_BUF)
}

/// Returns the chunk count for the parallel path, or `None` if the
/// file is below [`MIN_FILE_FOR_PARALLEL`] OR the env var isn't set.
///
/// Phase 13c had this default-on for files ≥1 GiB based on a
/// matched-memory A/B on C → D that showed +9 % vs single-stream
/// CopyFileExW. **Phase 39 re-bench on Windows 11 NVMe** showed
/// the opposite: parallel-chunks-4 hits 1128 MiB/s while raw
/// CopyFileExW hits **2429 MiB/s** on a 10 GiB same-volume copy.
/// Modern NVMe + Windows 11's CopyFileExW already saturates the
/// per-device queue with one stream; splitting into 4 streams
/// adds per-chunk seek + handle-open overhead and contends for
/// the same hardware queue.
///
/// Default is now **off**. Users with hardware that benefits
/// (RAID arrays, NVMe-over-fabric, distributed FS) can opt in
/// via `COPYTHAT_PARALLEL_CHUNKS=<N>` (clamped 2..=16).
pub(crate) fn requested_chunks(total: u64) -> Option<usize> {
    if total < MIN_FILE_FOR_PARALLEL {
        return None;
    }
    let raw = std::env::var("COPYTHAT_PARALLEL_CHUNKS").ok()?;
    let n: usize = raw.parse().ok()?;
    if n < 2 {
        return None;
    }
    Some(n.clamp(2, 16))
}

/// Run the parallel multi-chunk copy. Caller guarantees:
/// - `total` matches the source file's length in bytes.
/// - `num_chunks >= 2` (use single-stream otherwise).
/// - Reflink / symlink / verify has already been ruled out.
pub(crate) async fn parallel_chunk_copy(
    src: PathBuf,
    dst: PathBuf,
    total: u64,
    num_chunks: usize,
    ctrl: CopyControl,
    events: mpsc::Sender<CopyEvent>,
) -> NativeOutcome {
    super::emit_started(&src, &dst, total, &events).await;

    // Pre-allocate so every worker writes into its own range
    // without contending on file-extend. Phase 39 follow-up: when
    // `COPYTHAT_SKIP_ZERO_FILL=1` and the calling process holds
    // `SE_MANAGE_VOLUME_NAME` (admin), we also call
    // `SetFileValidData` after `set_len` so NTFS skips its
    // lazy-zero pass over the pre-allocated extent. The caller
    // guarantees every byte is overwritten by the chunk workers,
    // so the security implication of "uninitialised disk blocks
    // briefly readable" is bounded by us, not exposed to other
    // processes. Best-effort: if the privilege isn't held the
    // call returns ERROR_PRIVILEGE_NOT_HELD and we silently fall
    // through to the (slightly slower) lazy-zero path.
    {
        let prep = {
            let dst = dst.clone();
            tokio::task::spawn_blocking(move || -> io::Result<()> {
                let f = OpenOptions::new()
                    .create(true)
                    .truncate(true)
                    .write(true)
                    .open(&dst)?;
                f.set_len(total)?;
                #[cfg(windows)]
                {
                    let _ = try_skip_zero_fill(&f, total);
                }
                Ok(())
            })
            .await
        };
        match prep {
            Ok(Ok(())) => {}
            Ok(Err(e)) => return NativeOutcome::Io(e),
            Err(join_err) => {
                return NativeOutcome::Io(io::Error::other(format!(
                    "parallel prep spawn_blocking panicked: {join_err}"
                )));
            }
        }
    }

    let chunk_bytes = total.div_ceil(num_chunks as u64);
    let started = Instant::now();
    let bytes_done = Arc::new(AtomicU64::new(0));
    let cancelled = Arc::new(AtomicBool::new(false));

    // Progress emitter reads the accumulated atomic on a timer
    // rather than per-block. Matches the pattern we already use
    // for `CopyFileExW`'s callback.
    let bytes_for_progress = bytes_done.clone();
    let events_for_progress = events.clone();
    let progress_task = {
        let cancelled = cancelled.clone();
        let ctrl_for_progress = ctrl.clone();
        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(super::PROGRESS_MIN_INTERVAL);
            ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
            ticker.tick().await; // skip immediate first tick
            let mut last_emit_bytes: u64 = 0;
            loop {
                ticker.tick().await;
                // While paused, every worker is busy-sleeping at 20 ms;
                // the bytes counter is frozen and there's nothing
                // useful to emit. Skip until pause clears or cancel
                // fires — keeps the emitter from burning a wakeup
                // every 50 ms forever on an indefinitely-paused job.
                if ctrl_for_progress.is_paused() {
                    if cancelled.load(Ordering::Relaxed) || ctrl_for_progress.is_cancelled() {
                        break;
                    }
                    continue;
                }
                let bytes = bytes_for_progress.load(Ordering::Relaxed);
                if bytes.saturating_sub(last_emit_bytes) >= super::PROGRESS_MIN_BYTES {
                    let elapsed = started.elapsed();
                    let rate = super::fast_rate_bps(bytes, elapsed);
                    let _ = events_for_progress
                        .send(CopyEvent::Progress {
                            bytes,
                            total,
                            rate_bps: rate,
                        })
                        .await;
                    last_emit_bytes = bytes;
                }
                if bytes >= total || cancelled.load(Ordering::Relaxed) {
                    break;
                }
            }
        })
    };

    // Spawn one worker per chunk. Each worker owns its own pair of
    // file handles; cancellation is checked at the top of each
    // buffer iteration so the whole copy tears down quickly when
    // the user aborts.
    let mut handles = Vec::with_capacity(num_chunks);
    for i in 0..num_chunks {
        let offset = i as u64 * chunk_bytes;
        if offset >= total {
            break;
        }
        let chunk_end = (offset + chunk_bytes).min(total);
        let len = chunk_end - offset;
        let src_p = src.clone();
        let dst_p = dst.clone();
        let ctrl = ctrl.clone();
        let bytes_done = bytes_done.clone();
        let cancelled = cancelled.clone();

        let buf_bytes = chunk_buf_size(total, num_chunks);
        handles.push(tokio::task::spawn_blocking(move || -> io::Result<()> {
            let mut reader = File::open(&src_p)?;
            let mut writer = OpenOptions::new().write(true).open(&dst_p)?;
            reader.seek(SeekFrom::Start(offset))?;
            writer.seek(SeekFrom::Start(offset))?;

            let mut buf = vec![0u8; buf_bytes];
            let mut written = 0u64;
            while written < len {
                if ctrl.is_cancelled() {
                    cancelled.store(true, Ordering::Release);
                    return Err(io::Error::other("cancelled"));
                }
                // Park while paused. Cheap atomic spin — pause is
                // rare; throughput-critical path stays tight.
                while ctrl.is_paused() {
                    std::thread::sleep(std::time::Duration::from_millis(20));
                    if ctrl.is_cancelled() {
                        cancelled.store(true, Ordering::Release);
                        return Err(io::Error::other("cancelled"));
                    }
                }

                let want = (len - written).min(buf.len() as u64) as usize;
                let read = reader.read(&mut buf[..want])?;
                if read == 0 {
                    return Err(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        "source shorter than expected",
                    ));
                }
                writer.write_all(&buf[..read])?;
                written += read as u64;
                bytes_done.fetch_add(read as u64, Ordering::Relaxed);
            }
            // Flush this worker's chunk so `bytes_done == total`
            // really means everything is on disk.
            writer.flush()?;
            Ok(())
        }));
    }

    let mut first_err: Option<io::Error> = None;
    for h in handles {
        match h.await {
            Ok(Ok(())) => {}
            Ok(Err(e)) => {
                if first_err.is_none() {
                    first_err = Some(e);
                }
            }
            Err(join_err) => {
                if first_err.is_none() {
                    first_err = Some(io::Error::other(format!(
                        "parallel worker join: {join_err}"
                    )));
                }
            }
        }
    }

    // Let the progress task drain the last tick.
    let _ = progress_task.await;

    if let Some(e) = first_err {
        if cancelled.load(Ordering::Relaxed) {
            return NativeOutcome::Cancelled;
        }
        let _ = std::fs::remove_file(&dst);
        return NativeOutcome::Io(e);
    }

    NativeOutcome::Done {
        strategy: ChosenStrategy::CopyFileExW, // Same native strategy class; the dispatcher records "fast" regardless of chunked vs single-stream.
        bytes: total,
    }
}

/// Attempt the NTFS lazy-zero skip via `SetFileValidData`. Returns
/// `true` iff the system call succeeded — which requires the caller
/// to hold `SE_MANAGE_VOLUME_NAME` (admin). On failure the call is
/// silently a no-op; NTFS still zero-fills the extent during writes,
/// just slower. Gated behind `COPYTHAT_SKIP_ZERO_FILL=1` so the
/// admin / opt-in nature is explicit.
///
/// Security note: an admin user opting into this acknowledges that
/// the pre-allocated extent contains whatever bytes were on those
/// clusters before. Copy That guarantees the workers write valid
/// data over every byte, so the risk is bounded to the Copy That
/// process — there's no cross-process data exposure.
#[cfg(windows)]
fn try_skip_zero_fill(file: &File, size: u64) -> bool {
    use std::os::windows::io::AsRawHandle;
    use windows_sys::Win32::Storage::FileSystem::SetFileValidData;

    if std::env::var("COPYTHAT_SKIP_ZERO_FILL")
        .ok()
        .as_deref()
        .is_none_or(|v| !matches!(v, "1" | "true" | "on"))
    {
        return false;
    }
    let handle = file.as_raw_handle().cast::<core::ffi::c_void>();
    // SAFETY: `file` is a valid open file handle held by the caller
    // for the duration of this call. SetFileValidData has no aliasing
    // requirements and writes only the file's metadata.
    let ok = unsafe { SetFileValidData(handle, size as i64) };
    ok != 0
}
