//! The copy loop.
//!
//! Shape: `BufReader -> fill_buf/consume -> write_all -> BufWriter ->
//! destination File`. Between every buffer we check `CopyControl`,
//! throttle progress events, and accumulate the total. Metadata
//! (permissions + mtime/atime) is applied after the byte copy
//! succeeds; cleanup of the partial destination runs on any failure
//! unless `keep_partial` overrides it.

use std::path::Path;
use std::time::{Duration, Instant};

use filetime::FileTime;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::sync::mpsc;

use crate::control::CopyControl;
use crate::error::CopyError;
use crate::event::{CopyEvent, CopyReport};
use crate::options::{CopyOptions, CopyStrategy, FastCopyHookOutcome};
use crate::verify::Hasher;

const PROGRESS_MIN_BYTES: u64 = 16 * 1024;
const PROGRESS_MIN_INTERVAL: Duration = Duration::from_millis(50);

/// Copy a single regular file from `src` to `dst`.
///
/// See crate-level docs for the public contract; the loop is documented
/// inline here. Returns `Ok(CopyReport)` on success, `Err(CopyError)`
/// on I/O failure or caller-requested cancellation.
pub async fn copy_file(
    src: &Path,
    dst: &Path,
    opts: CopyOptions,
    ctrl: CopyControl,
    events: mpsc::Sender<CopyEvent>,
) -> Result<CopyReport, CopyError> {
    let src_path = src.to_path_buf();
    let dst_path = dst.to_path_buf();

    let metadata_result = if opts.follow_symlinks {
        tokio::fs::metadata(&src_path).await
    } else {
        tokio::fs::symlink_metadata(&src_path).await
    };
    let src_metadata = metadata_result.map_err(|e| CopyError::from_io(&src_path, &dst_path, e))?;

    if !opts.follow_symlinks && src_metadata.file_type().is_symlink() {
        return copy_symlink(&src_path, &dst_path, &opts, &events).await;
    }

    // Phase 6: consult the fast-copy hook before opening files for the
    // standard async loop. Bypassed entirely when verify is enabled —
    // the verify pipeline relies on hashing source bytes during the
    // write loop, which the hook can't do without a separate read pass
    // that defeats the integration's perf win.
    if opts.verify.is_none()
        && opts.strategy != CopyStrategy::AlwaysAsync
        && let Some(hook) = opts.fast_copy_hook.clone()
    {
        match hook
            .try_copy(
                src_path.clone(),
                dst_path.clone(),
                opts.clone(),
                ctrl.clone(),
                events.clone(),
            )
            .await
        {
            Ok(FastCopyHookOutcome::Done(report)) => return Ok(report),
            Ok(FastCopyHookOutcome::NotSupported) => {
                if opts.strategy == CopyStrategy::AlwaysFast {
                    let err = CopyError {
                        kind: crate::error::CopyErrorKind::IoOther,
                        src: src_path.clone(),
                        dst: dst_path.clone(),
                        raw_os_error: None,
                        message: "no fast path available (CopyStrategy::AlwaysFast)".to_string(),
                    };
                    let _ = events.send(CopyEvent::Failed { err: err.clone() }).await;
                    return Err(err);
                }
                // Auto / NoReflink: fall through to the async engine below.
            }
            Err(err) => {
                if !opts.keep_partial {
                    let _ = tokio::fs::remove_file(&dst_path).await;
                }
                let _ = events.send(CopyEvent::Failed { err: err.clone() }).await;
                return Err(err);
            }
        }
    }

    let total = src_metadata.len();
    // Phase 13c — pick a buffer size that matches the file:
    // - Small files shrink the buffer so we don't overallocate
    // - Mid-size files use the configured value verbatim
    // - Multi-GiB files bump up to 4 MiB for better memory pipelining
    let buf_size = opts.buffer_size_for_file(total);

    let src_file = open_src_with_retry(&src_path)
        .await
        .map_err(|e| CopyError::from_io(&src_path, &dst_path, e))?;

    let mut open = OpenOptions::new();
    open.write(true).truncate(true);
    if opts.fail_if_exists {
        open.create_new(true);
    } else {
        open.create(true);
    }
    let dst_file = open
        .open(&dst_path)
        .await
        .map_err(|e| CopyError::from_io(&src_path, &dst_path, e))?;

    let _ = events
        .send(CopyEvent::Started {
            src: src_path.clone(),
            dst: dst_path.clone(),
            total_bytes: total,
        })
        .await;

    let mut reader = BufReader::with_capacity(buf_size, src_file);
    let mut writer = BufWriter::with_capacity(buf_size, dst_file);

    // Source-side hasher: when verify is enabled we reuse the bytes the
    // copy loop is already reading. Dst-side hashing is a separate
    // post-pass below.
    let mut src_hasher: Option<Box<dyn Hasher>> = opts.verify.as_ref().map(|v| v.make());

    let started_at = Instant::now();
    let mut copied: u64 = 0;
    let mut last_emit_at = started_at;
    let mut last_emit_bytes: u64 = 0;
    let mut was_paused = false;

    let loop_result: Result<(), CopyError> = loop {
        if ctrl.is_cancelled() {
            break Err(CopyError::cancelled(&src_path, &dst_path));
        }
        if ctrl.is_paused() {
            if !was_paused {
                let _ = events.send(CopyEvent::Paused).await;
                was_paused = true;
            }
            ctrl.wait_while_paused().await;
            if ctrl.is_cancelled() {
                break Err(CopyError::cancelled(&src_path, &dst_path));
            }
            if was_paused {
                let _ = events.send(CopyEvent::Resumed).await;
                was_paused = false;
            }
            continue;
        }

        let buf = match reader.fill_buf().await {
            Ok(b) => b,
            Err(e) => break Err(CopyError::from_io(&src_path, &dst_path, e)),
        };
        if buf.is_empty() {
            break Ok(());
        }
        let n = buf.len();
        if let Err(e) = writer.write_all(buf).await {
            break Err(CopyError::from_io(&src_path, &dst_path, e));
        }
        if let Some(h) = src_hasher.as_mut() {
            h.update(buf);
        }
        reader.consume(n);
        copied += n as u64;

        let now = Instant::now();
        if copied.saturating_sub(last_emit_bytes) >= PROGRESS_MIN_BYTES
            && now.duration_since(last_emit_at) >= PROGRESS_MIN_INTERVAL
        {
            let elapsed = now.duration_since(started_at);
            let rate = rate_bps(copied, elapsed);
            let _ = events
                .send(CopyEvent::Progress {
                    bytes: copied,
                    total,
                    rate_bps: rate,
                })
                .await;
            last_emit_at = now;
            last_emit_bytes = copied;
        }
    };

    match loop_result {
        Ok(()) => {
            if let Err(e) = writer.flush().await {
                return fail(
                    &src_path,
                    &dst_path,
                    &opts,
                    &events,
                    CopyError::from_io(&src_path, &dst_path, e),
                    writer,
                    reader,
                )
                .await;
            }
            // Fsync semantics:
            //   - If the caller set `fsync_on_close`, always sync.
            //   - Otherwise, when verify is enabled and
            //     `fsync_before_verify` is on (default), sync so the
            //     post-pass reads the freshly-written bytes without
            //     racing the page cache.
            let should_fsync =
                opts.fsync_on_close || (opts.verify.is_some() && opts.fsync_before_verify);
            if should_fsync {
                if let Err(e) = writer.get_mut().sync_all().await {
                    return fail(
                        &src_path,
                        &dst_path,
                        &opts,
                        &events,
                        CopyError::from_io(&src_path, &dst_path, e),
                        writer,
                        reader,
                    )
                    .await;
                }
            }
            drop(writer);
            drop(reader);

            if let Err(e) =
                preserve_metadata(src_path.clone(), dst_path.clone(), &src_metadata, &opts).await
            {
                return finalize_error(&opts, &events, e, &dst_path).await;
            }

            let elapsed = started_at.elapsed();
            let rate = rate_bps(copied, elapsed);
            let _ = events
                .send(CopyEvent::Progress {
                    bytes: copied,
                    total,
                    rate_bps: rate,
                })
                .await;

            // Verify pass: re-hash the destination and compare against
            // the source-side hash we built during the write loop.
            if let (Some(verifier), Some(src_h)) = (opts.verify.as_ref(), src_hasher.take()) {
                match run_verify_pass(&src_path, &dst_path, &opts, verifier, src_h, &ctrl, &events)
                    .await
                {
                    Ok(()) => {}
                    Err(err) => {
                        return finalize_error(&opts, &events, err, &dst_path).await;
                    }
                }
            }

            let _ = events
                .send(CopyEvent::Completed {
                    bytes: copied,
                    duration: elapsed,
                    rate_bps: rate,
                })
                .await;
            Ok(CopyReport {
                src: src_path,
                dst: dst_path,
                bytes: copied,
                duration: elapsed,
                rate_bps: rate,
            })
        }
        Err(err) => fail(&src_path, &dst_path, &opts, &events, err, writer, reader).await,
    }
}

/// Run the verify post-pass: stream the destination through a fresh
/// hasher, compare against the source-side digest we accumulated during
/// the write loop, and emit verify events.
async fn run_verify_pass(
    src_path: &Path,
    dst_path: &Path,
    opts: &CopyOptions,
    verifier: &crate::verify::Verifier,
    src_hasher: Box<dyn Hasher>,
    ctrl: &CopyControl,
    events: &mpsc::Sender<CopyEvent>,
) -> Result<(), CopyError> {
    let src_digest = src_hasher.finalize();
    let src_hex = hex_encode(&src_digest);
    let algorithm_name = verifier.name();

    // Dest size may have shifted (zero-byte files have no page cache
    // surprises, but we still metadata() so our VerifyStarted total is
    // accurate).
    let dst_meta = tokio::fs::metadata(dst_path)
        .await
        .map_err(|e| CopyError::from_io(src_path, dst_path, e))?;
    let total = dst_meta.len();

    let _ = events
        .send(CopyEvent::VerifyStarted {
            algorithm: algorithm_name,
            total_bytes: total,
        })
        .await;

    let dst_file = File::open(dst_path)
        .await
        .map_err(|e| CopyError::from_io(src_path, dst_path, e))?;
    let buf_size = opts.clamped_buffer_size();
    let mut reader = BufReader::with_capacity(buf_size, dst_file);
    let mut dst_hasher = verifier.make();
    let started_at = Instant::now();
    let mut processed: u64 = 0;
    let mut last_emit_at = started_at;
    let mut last_emit_bytes: u64 = 0;

    loop {
        if ctrl.is_cancelled() {
            return Err(CopyError::cancelled(src_path, dst_path));
        }
        if ctrl.is_paused() {
            ctrl.wait_while_paused().await;
            if ctrl.is_cancelled() {
                return Err(CopyError::cancelled(src_path, dst_path));
            }
            continue;
        }

        let buf = reader
            .fill_buf()
            .await
            .map_err(|e| CopyError::from_io(src_path, dst_path, e))?;
        if buf.is_empty() {
            break;
        }
        let n = buf.len();
        dst_hasher.update(buf);
        reader.consume(n);
        processed += n as u64;

        let now = Instant::now();
        if processed.saturating_sub(last_emit_bytes) >= PROGRESS_MIN_BYTES
            && now.duration_since(last_emit_at) >= PROGRESS_MIN_INTERVAL
        {
            let elapsed = now.duration_since(started_at);
            let rate = rate_bps(processed, elapsed);
            let _ = events
                .send(CopyEvent::VerifyProgress {
                    bytes: processed,
                    total,
                    rate_bps: rate,
                })
                .await;
            last_emit_at = now;
            last_emit_bytes = processed;
        }
    }

    let dst_digest = dst_hasher.finalize();
    let dst_hex = hex_encode(&dst_digest);
    let elapsed = started_at.elapsed();

    if src_digest == dst_digest {
        let _ = events
            .send(CopyEvent::VerifyCompleted {
                algorithm: algorithm_name,
                src_hex,
                dst_hex,
                duration: elapsed,
            })
            .await;
        Ok(())
    } else {
        let _ = events
            .send(CopyEvent::VerifyFailed {
                algorithm: algorithm_name,
                src_hex: src_hex.clone(),
                dst_hex: dst_hex.clone(),
            })
            .await;
        Err(CopyError::verify_failed(
            src_path,
            dst_path,
            algorithm_name,
            &src_hex,
            &dst_hex,
        ))
    }
}

fn hex_encode(bytes: &[u8]) -> String {
    // Minimal lowercase hex encoder — keeps `copythat-core` free of a
    // `hex` crate dependency. The happy path is small, error messages
    // don't warrant pulling in another crate.
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push(nibble(b >> 4));
        s.push(nibble(b & 0x0f));
    }
    s
}

fn nibble(n: u8) -> char {
    match n & 0x0f {
        0..=9 => (b'0' + n) as char,
        10..=15 => (b'a' + (n - 10)) as char,
        _ => unreachable!(),
    }
}

async fn fail(
    _src: &Path,
    dst: &Path,
    opts: &CopyOptions,
    events: &mpsc::Sender<CopyEvent>,
    err: CopyError,
    writer: BufWriter<File>,
    reader: BufReader<File>,
) -> Result<CopyReport, CopyError> {
    drop(writer);
    drop(reader);
    finalize_error(opts, events, err, dst).await
}

async fn finalize_error(
    opts: &CopyOptions,
    events: &mpsc::Sender<CopyEvent>,
    err: CopyError,
    dst: &Path,
) -> Result<CopyReport, CopyError> {
    if !opts.keep_partial {
        let _ = tokio::fs::remove_file(dst).await;
    }
    let _ = events.send(CopyEvent::Failed { err: err.clone() }).await;
    Err(err)
}

fn rate_bps(bytes: u64, elapsed: Duration) -> u64 {
    let secs = elapsed.as_secs_f64();
    if secs <= 0.0 {
        return 0;
    }
    (bytes as f64 / secs) as u64
}

async fn preserve_metadata(
    src: std::path::PathBuf,
    dst: std::path::PathBuf,
    src_metadata: &std::fs::Metadata,
    opts: &CopyOptions,
) -> Result<(), CopyError> {
    // Apply timestamps BEFORE permissions. On Windows a readonly
    // attribute blocks subsequent `SetFileTime` calls; on Unix the
    // ordering is harmless either way.
    if opts.preserve_times {
        let atime = FileTime::from_last_access_time(src_metadata);
        let mtime = FileTime::from_last_modification_time(src_metadata);
        let dst_for_blocking = dst.clone();
        let join = tokio::task::spawn_blocking(move || {
            filetime::set_file_times(&dst_for_blocking, atime, mtime)
        })
        .await;
        let io_result = match join {
            Ok(inner) => inner,
            Err(_) => {
                return Err(CopyError {
                    kind: crate::error::CopyErrorKind::IoOther,
                    src: src.clone(),
                    dst: dst.clone(),
                    raw_os_error: None,
                    message: "spawn_blocking panicked while setting timestamps".to_string(),
                });
            }
        };
        if let Err(e) = io_result {
            return Err(CopyError::from_io(&src, &dst, e));
        }
    }
    if opts.preserve_permissions {
        let perms = src_metadata.permissions();
        if let Err(e) = tokio::fs::set_permissions(&dst, perms).await {
            return Err(CopyError::from_io(&src, &dst, e));
        }
    }
    Ok(())
}

/// Clone a symlink source as a symlink at `dst` (i.e. `opts.follow_symlinks
/// == false`). No byte copy happens; progress events reflect a 0-byte
/// transfer for consistency with the normal path.
async fn copy_symlink(
    src: &Path,
    dst: &Path,
    opts: &CopyOptions,
    events: &mpsc::Sender<CopyEvent>,
) -> Result<CopyReport, CopyError> {
    let target = match tokio::fs::read_link(src).await {
        Ok(t) => t,
        Err(e) => return Err(CopyError::from_io(src, dst, e)),
    };
    let _ = events
        .send(CopyEvent::Started {
            src: src.to_path_buf(),
            dst: dst.to_path_buf(),
            total_bytes: 0,
        })
        .await;
    if opts.fail_if_exists && tokio::fs::symlink_metadata(dst).await.is_ok() {
        let err = CopyError {
            kind: crate::error::CopyErrorKind::PermissionDenied,
            src: src.to_path_buf(),
            dst: dst.to_path_buf(),
            raw_os_error: None,
            message: "destination already exists and fail_if_exists is set".to_string(),
        };
        let _ = events.send(CopyEvent::Failed { err: err.clone() }).await;
        return Err(err);
    }
    let start = Instant::now();
    let created = create_symlink(&target, dst).await;
    if let Err(e) = created {
        let err = CopyError::from_io(src, dst, e);
        let _ = events.send(CopyEvent::Failed { err: err.clone() }).await;
        return Err(err);
    }
    let elapsed = start.elapsed();
    let _ = events
        .send(CopyEvent::Completed {
            bytes: 0,
            duration: elapsed,
            rate_bps: 0,
        })
        .await;
    Ok(CopyReport {
        src: src.to_path_buf(),
        dst: dst.to_path_buf(),
        bytes: 0,
        duration: elapsed,
        rate_bps: 0,
    })
}

/// Phase 14 — open the source file with the widest possible share
/// mode and retry on sharing violations. Lets us copy a file that
/// another process has open for read/write/delete (common on Windows
/// — log files being written, loaded DLLs, Office documents with an
/// exclusive lock). Unix kernels don't block reads on open files,
/// so this compiles down to a plain `File::open` there.
async fn open_src_with_retry(src: &Path) -> std::io::Result<File> {
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        // FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE.
        // Tells the OS we are OK with others writing / deleting
        // while we read — matches what `robocopy /B` would use in
        // backup semantics mode.
        const SHARE_ALL: u32 = 0x1 | 0x2 | 0x4;
        let mut last_err: Option<std::io::Error> = None;
        for attempt in 0..3u32 {
            let mut opts = std::fs::OpenOptions::new();
            opts.read(true).share_mode(SHARE_ALL);
            let res = {
                let path = src.to_path_buf();
                tokio::task::spawn_blocking(move || opts.open(&path)).await
            };
            match res {
                Ok(Ok(std_file)) => return Ok(File::from_std(std_file)),
                Ok(Err(e)) => {
                    // ERROR_SHARING_VIOLATION = 32. Retry with
                    // exponential backoff; most short-lived locks
                    // clear within a few hundred ms.
                    if e.raw_os_error() == Some(32) {
                        let ms = 50u64 << attempt; // 50, 100, 200
                        tokio::time::sleep(std::time::Duration::from_millis(ms)).await;
                        last_err = Some(e);
                        continue;
                    }
                    return Err(e);
                }
                Err(join_err) => return Err(std::io::Error::other(format!("join: {join_err}"))),
            }
        }
        Err(last_err.unwrap_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::WouldBlock,
                "sharing-violation-retries-exhausted",
            )
        }))
    }
    #[cfg(not(windows))]
    {
        File::open(src).await
    }
}

#[cfg(unix)]
async fn create_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    tokio::fs::symlink(target, link).await
}

#[cfg(windows)]
async fn create_symlink(target: &Path, link: &Path) -> std::io::Result<()> {
    // Windows distinguishes file vs. directory symlinks. Probe the
    // target; if it's a directory, use the directory variant so the
    // resulting link actually points to something traversable.
    let md = tokio::fs::metadata(target).await;
    match md {
        Ok(m) if m.is_dir() => tokio::fs::symlink_dir(target, link).await,
        _ => tokio::fs::symlink_file(target, link).await,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rate_bps_sane() {
        assert_eq!(rate_bps(0, Duration::from_secs(1)), 0);
        assert_eq!(rate_bps(100, Duration::ZERO), 0);
        assert_eq!(rate_bps(1000, Duration::from_secs(2)), 500);
    }

    #[test]
    fn buffer_size_is_clamped() {
        let tiny = CopyOptions {
            buffer_size: 1,
            ..Default::default()
        };
        assert_eq!(tiny.clamped_buffer_size(), crate::options::MIN_BUFFER_SIZE);
        let huge = CopyOptions {
            buffer_size: usize::MAX,
            ..Default::default()
        };
        assert_eq!(huge.clamped_buffer_size(), crate::options::MAX_BUFFER_SIZE);
        let ok = CopyOptions {
            buffer_size: 256 * 1024,
            ..Default::default()
        };
        assert_eq!(ok.clamped_buffer_size(), 256 * 1024);
    }
}
