//! The copy loop.
//!
//! Shape: `BufReader -> fill_buf/consume -> write_all -> BufWriter ->
//! destination File`. Between every buffer we check `CopyControl`,
//! throttle progress events, and accumulate the total. Metadata
//! (permissions + mtime/atime) is applied after the byte copy
//! succeeds; cleanup of the partial destination runs on any failure
//! unless `keep_partial` overrides it.

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use filetime::FileTime;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncSeekExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::sync::mpsc;

use crate::control::CopyControl;
use crate::error::CopyError;
use crate::event::{CopyEvent, CopyReport};
use crate::options::{
    CopyOptions, CopyStrategy, FastCopyHookOutcome, LockedFilePolicy, ResumePlan, SnapshotLease,
};
use crate::safety::validate_path_no_traversal;
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

    // Phase 17a — reject traversal / NUL-byte paths up front. The
    // check is lexical and filesystem-free; see
    // `copythat_core::safety` for the threat model.
    if let Err(e) = validate_path_no_traversal(&src_path) {
        return Err(CopyError::path_escape(&src_path, &dst_path, e));
    }
    if let Err(e) = validate_path_no_traversal(&dst_path) {
        return Err(CopyError::path_escape(&src_path, &dst_path, e));
    }

    let metadata_result = if opts.follow_symlinks {
        tokio::fs::metadata(&src_path).await
    } else {
        tokio::fs::symlink_metadata(&src_path).await
    };
    let src_metadata = metadata_result.map_err(|e| CopyError::from_io(&src_path, &dst_path, e))?;

    if !opts.follow_symlinks && src_metadata.file_type().is_symlink() {
        return copy_symlink(&src_path, &dst_path, &opts, &events).await;
    }

    // Phase 32d — cloud-sink early branch. When the caller has wired
    // `CopyOptions::cloud_sink`, route the write path straight to the
    // remote backend instead of opening a local destination file.
    // Bypasses sparse / fast-copy / verify / chunk-store / journal
    // pathways by design: those all assume a local dst handle, and
    // porting them to a `CloudSink` is the Phase 32e follow-up.
    // Symlinks + sparsity + verify on remote destinations land there.
    if let Some(sink) = opts.cloud_sink.clone() {
        return copy_file_to_cloud_sink(
            src_path,
            dst_path,
            opts,
            ctrl,
            events,
            src_metadata,
            sink,
        )
        .await;
    }

    // Phase 23 — sparse-file pre-flight. When the caller has wired a
    // `sparse_ops` hook and `preserve_sparseness` is on, detect the
    // source's extent layout up front. If the source actually contains
    // holes (allocated_bytes < logical_len), divert into the sparse
    // copy path, which preserves the hole layout on the destination.
    // If the destination filesystem doesn't support sparse files, emit
    // `SparsenessNotSupported` once and fall through to the dense
    // path. Bypassing `fast_copy_hook` is intentional: the sparse
    // path owns its own read/write strategy.
    if opts.preserve_sparseness
        && let Some(sparse_ops) = opts.sparse_ops.clone()
    {
        let total_len = src_metadata.len();
        let src_for_detect = src_path.clone();
        let detect_ops = sparse_ops.clone();
        let extents_result =
            tokio::task::spawn_blocking(move || detect_ops.detect_extents(&src_for_detect))
                .await
                .map_err(|e| {
                    CopyError::from_io(
                        &src_path,
                        &dst_path,
                        std::io::Error::other(format!("sparse detect join: {e}")),
                    )
                })?;

        if let Ok(extents) = extents_result {
            let allocated = crate::sparse::allocated_bytes(&extents);
            let is_sparse = allocated < total_len && !extents.is_empty()
                || (total_len > 0 && extents.is_empty());
            if is_sparse {
                // Probe the *destination parent* for FS support — the
                // dst doesn't exist yet so `supports_sparse(dst)` would
                // walk up to the parent anyway.
                let dst_probe = dst_path.parent().unwrap_or(&dst_path).to_path_buf();
                let probe_ops = sparse_ops.clone();
                let dst_supports =
                    tokio::task::spawn_blocking(move || probe_ops.supports_sparse(&dst_probe))
                        .await
                        .unwrap_or(false);

                if !dst_supports {
                    let _ = events
                        .send(CopyEvent::SparsenessNotSupported {
                            dst_fs: detect_dst_fs_label(&dst_path),
                        })
                        .await;
                    // Fall through to the classic dense path below.
                } else {
                    return copy_file_sparse_aware(
                        src_path,
                        dst_path,
                        opts,
                        ctrl,
                        events,
                        src_metadata,
                        total_len,
                        extents,
                        sparse_ops,
                    )
                    .await;
                }
            }
        }
        // `detect_extents` errored — fall through to the dense path.
        // The extent hook is an optimisation; a failure shouldn't
        // abort the copy.
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

    // Phase 20 — decide the resume strategy *before* opening dst, so
    // the open mode (truncate vs. seek-and-append) and the initial
    // copy offset are baked in from the start. `decide_resume`
    // emits `CopyEvent::ResumeAborted` on a mismatch and falls
    // through to a fresh start; it never produces a partial
    // destination of its own.
    let resume_decision = decide_resume(&dst_path, total, &opts, &events).await?;

    if matches!(resume_decision, ResumeDecision::AlreadyComplete) {
        // dst already matches the journal's final hash — emit the
        // lifecycle events the caller expects, mark the journal
        // file as finished (idempotent), and return without
        // re-opening any handles.
        let _ = events
            .send(CopyEvent::Started {
                src: src_path.clone(),
                dst: dst_path.clone(),
                total_bytes: total,
            })
            .await;
        let _ = events
            .send(CopyEvent::Completed {
                bytes: total,
                duration: Duration::ZERO,
                rate_bps: 0,
            })
            .await;
        return Ok(CopyReport {
            src: src_path,
            dst: dst_path,
            bytes: total,
            duration: Duration::ZERO,
            rate_bps: 0,
        });
    }

    let resume_offset = match &resume_decision {
        ResumeDecision::Resume { offset, .. } => *offset,
        _ => 0,
    };

    // Phase 19b — open the source, falling through to a snapshot if
    // the sharing-violation / busy retry is exhausted and the caller
    // opted into `LockedFilePolicy::Snapshot`. `_snapshot_lease` is
    // held across the whole copy so the RAII guard only runs once the
    // file finishes (success or failure).
    let (mut src_file, _snapshot_lease) =
        open_src_with_snapshot_fallback(&src_path, &dst_path, &opts, &events).await?;

    let mut open = OpenOptions::new();
    open.write(true);
    // On a fresh start we truncate; on a resume we keep the prefix
    // bytes intact and seek past them. `fail_if_exists` is honoured
    // only on fresh start — a resumed copy by definition expects
    // the dst to exist.
    if matches!(resume_decision, ResumeDecision::FreshStart) {
        open.truncate(true);
        if opts.fail_if_exists {
            open.create_new(true);
        } else {
            open.create(true);
        }
    } else {
        // Resume — dst must already exist.
        open.create(false);
    }
    let mut dst_file = open
        .open(&dst_path)
        .await
        .map_err(|e| CopyError::from_io(&src_path, &dst_path, e))?;

    // Seek both ends past the resumed prefix.
    if resume_offset > 0 {
        src_file
            .seek(std::io::SeekFrom::Start(resume_offset))
            .await
            .map_err(|e| CopyError::from_io(&src_path, &dst_path, e))?;
        dst_file
            .seek(std::io::SeekFrom::Start(resume_offset))
            .await
            .map_err(|e| CopyError::from_io(&src_path, &dst_path, e))?;
    }

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

    // Phase 20 — running BLAKE3 of the source bytes already
    // consumed. Independent of the verify hasher (which may be a
    // different algorithm or off entirely). The journal needs a
    // BLAKE3-shaped digest at every checkpoint so resume can
    // verify the prefix on the next launch.
    let mut journal_hasher: Option<blake3::Hasher> =
        opts.journal.as_ref().map(|_| blake3::Hasher::new());

    // On a successful resume, prime the journal hasher with the
    // prefix bytes from the destination — they are byte-identical
    // to the source's first `resume_offset` bytes by construction
    // (we just verified the BLAKE3 match in `decide_resume`).
    if let (
        ResumeDecision::Resume {
            prefix_bytes_hash, ..
        },
        Some(h),
    ) = (&resume_decision, journal_hasher.as_mut())
    {
        // We don't have the actual prefix bytes here, only their
        // hash. Use blake3's "prime with known digest" pattern by
        // re-reading the dst's prefix and feeding it. The prefix
        // re-read is the same length blake3 already chewed during
        // `decide_resume`, so the worst-case cost is one extra
        // sequential read of `resume_offset` bytes.
        prime_blake3_from_dst_prefix(h, &dst_path, resume_offset)
            .await
            .map_err(|e| CopyError::from_io(&src_path, &dst_path, e))?;
        let _ = prefix_bytes_hash; // currently unused; kept for asserts
    }

    let started_at = Instant::now();
    let mut copied: u64 = resume_offset;
    let mut last_emit_at = started_at;
    let mut last_emit_bytes: u64 = resume_offset;
    let mut was_paused = false;
    // Phase 20 — guarantee the first checkpoint after the
    // PROGRESS_MIN_BYTES boundary, regardless of wall time. Without
    // this, a copy that finishes faster than `PROGRESS_MIN_INTERVAL`
    // (e.g. 64 MiB on a fast NVMe) would never emit a checkpoint and
    // the journal would believe the file never started — the
    // resume probe on the next launch would fall back to a full
    // restart for no reason.
    let mut first_progress_emitted = false;
    let resume_started_at_offset = resume_offset;
    let _ = resume_started_at_offset;

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
        if let Some(h) = journal_hasher.as_mut() {
            h.update(buf);
        }
        reader.consume(n);
        copied += n as u64;

        // Phase 21 — bandwidth shaping. Awaiting after the consume +
        // copied increment means a SIGKILL between the write and the
        // permit can leave the journal slightly behind durable bytes
        // (which is the safer direction — the next checkpoint
        // catches up). The sink itself decides how long to block;
        // an unlimited shape returns instantly.
        if let Some(shape) = opts.shape.as_ref() {
            shape.permit(n as u64).await;
        }

        let now = Instant::now();
        if copied.saturating_sub(last_emit_bytes) >= PROGRESS_MIN_BYTES
            && (!first_progress_emitted
                || now.duration_since(last_emit_at) >= PROGRESS_MIN_INTERVAL)
        {
            first_progress_emitted = true;
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

            // Phase 20 — checkpoint to the journal on the same
            // cadence as the progress event. Best-effort: a journal
            // write failure is logged inside the sink but never
            // aborts the copy. fdatasync the dst first so the
            // bytes_done we report is bounded above by what's
            // actually durable on disk — a SIGKILL after the
            // checkpoint can lose unwritten dst bytes but never
            // record bytes that were never written.
            if let (Some(journal), Some(h)) = (opts.journal.as_ref(), journal_hasher.as_ref()) {
                let _ = writer.flush().await;
                if let Err(e) = writer.get_mut().sync_data().await {
                    // Sync failure is informational — journal will
                    // still record but the next resume will defensively
                    // restart if dst's actual length lags.
                    tracing_log_sync_failure(&dst_path, &e);
                }
                let hash_so_far: [u8; 32] = *h.finalize().as_bytes();
                journal.checkpoint(opts.journal_file_idx, &dst_path, copied, total, hash_so_far);
            }
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

            if let Err(e) = preserve_metadata_with_events(
                src_path.clone(),
                dst_path.clone(),
                &src_metadata,
                &opts,
                Some(&events),
            )
            .await
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

            // Phase 20 — finalize the journal entry for this file.
            // Captures the final BLAKE3 so a future resume probe sees
            // `AlreadyComplete` and can skip the copy entirely.
            if let (Some(journal), Some(h)) = (opts.journal.as_ref(), journal_hasher.take()) {
                let final_hash: [u8; 32] = *h.finalize().as_bytes();
                journal.finish_file(opts.journal_file_idx, final_hash);
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

/// Apply timestamps + permissions + Phase 24 security metadata to
/// the destination, emitting `MetaTranslatedToAppleDouble` on the
/// optional events channel when the metadata apply pass falls
/// through to the `._<filename>` sidecar.
async fn preserve_metadata_with_events(
    src: std::path::PathBuf,
    dst: std::path::PathBuf,
    src_metadata: &std::fs::Metadata,
    opts: &CopyOptions,
    events: Option<&mpsc::Sender<CopyEvent>>,
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
    // Phase 24 — security-metadata transfer. Runs after timestamps
    // and permissions because the apply side may want to write xattrs
    // that depend on the destination's mode bits being correct first
    // (POSIX ACLs in particular interact with the permission set).
    // Failures inside the apply pass are downgraded to `FileError`
    // events when the events channel is attached — losing a side-
    // band stream should not abort the byte copy that already
    // succeeded. The exception is a panic in the spawn_blocking
    // worker, which surfaces as `IoOther` so the test harness can
    // assert.
    if opts.preserve_security_metadata
        && let Some(meta_ops) = opts.meta_ops.clone()
    {
        let policy = opts.meta_policy;
        let outcome = crate::meta::transfer(meta_ops, src.clone(), dst.clone(), policy).await;
        match outcome {
            Ok(o) => {
                if o.translated_to_appledouble
                    && let Some(tx) = events
                {
                    let _ = tx
                        .send(CopyEvent::MetaTranslatedToAppleDouble {
                            ext: o.translated_extension,
                        })
                        .await;
                }
            }
            Err(e) => {
                // Down-grade to a non-fatal: the byte copy already
                // succeeded. The structured-log line lives in the
                // helper to keep the engine free of tracing deps.
                tracing_log_meta_failure(&dst, &e);
            }
        }
    }
    Ok(())
}

/// Phase 23 — sparse-preserving copy loop.
///
/// Called from `copy_file` after the pre-flight detected a sparse
/// source and a sparse-capable destination FS. Owns its own
/// read/write strategy: iterate the source's allocated extents,
/// seek-copy each one into the destination, and leave the gaps
/// untouched so the filesystem naturally preserves the hole layout.
///
/// Interactions:
/// - Honours `opts.shape` and `CopyControl` exactly like the dense
///   loop (pause / cancel / bandwidth permit per buffered read).
/// - Honours `opts.verify` via the standard byte-for-byte post-pass —
///   holes read back as zeros on both sides, so the hashes agree when
///   the layout was preserved.
/// - Bypasses `opts.fast_copy_hook`: the sparse path is the fast
///   path, and reflink preservation is a Phase 6 concern.
/// - Bypasses `opts.journal`: no sparse-aware resume in Phase 23.
/// - Enforces `allocated_bytes(dst) <= total_len` after the copy; a
///   fully-densified dst raises `CopyErrorKind::SparsenessMismatch`.
#[allow(clippy::too_many_arguments)]
async fn copy_file_sparse_aware(
    src_path: std::path::PathBuf,
    dst_path: std::path::PathBuf,
    opts: CopyOptions,
    ctrl: CopyControl,
    events: mpsc::Sender<CopyEvent>,
    src_metadata: std::fs::Metadata,
    total: u64,
    src_extents: Vec<crate::sparse::ByteRange>,
    sparse_ops: std::sync::Arc<dyn crate::sparse::SparseOps>,
) -> Result<CopyReport, CopyError> {
    // Open an empty destination: honour fail_if_exists, but always
    // truncate on success (the sparse path never resumes).
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

    // Mark the destination sparse *before* any data write. On Windows
    // NTFS this is `FSCTL_SET_SPARSE`; on Linux/macOS it's a no-op.
    // Failure falls through with the event already emitted — a
    // best-effort sparse mark is the right policy (e.g. ReFS may
    // refuse on some builds).
    {
        let ops = sparse_ops.clone();
        let dst_clone = dst_path.clone();
        let _ = tokio::task::spawn_blocking(move || ops.make_destination_sparse(&dst_clone)).await;
    }

    // Pre-size the destination so writes at offsets > current EOF
    // don't fault the OS into allocating the intervening bytes.
    {
        let dst_clone = dst_path.clone();
        let set_len_result = tokio::task::spawn_blocking(move || {
            let std_file = std::fs::OpenOptions::new().write(true).open(&dst_clone)?;
            std_file.set_len(total)?;
            Ok::<(), std::io::Error>(())
        })
        .await
        .map_err(|e| {
            CopyError::from_io(
                &src_path,
                &dst_path,
                std::io::Error::other(format!("set_len join: {e}")),
            )
        })?;
        if let Err(e) = set_len_result {
            return Err(CopyError::from_io(&src_path, &dst_path, e));
        }
    }

    // Source open with the same snapshot fallback the dense path uses.
    let (mut src_file, _snapshot_lease) =
        open_src_with_snapshot_fallback(&src_path, &dst_path, &opts, &events).await?;
    let mut dst_file = dst_file;

    let _ = events
        .send(CopyEvent::Started {
            src: src_path.clone(),
            dst: dst_path.clone(),
            total_bytes: total,
        })
        .await;

    let buf_size = opts.buffer_size_for_file(total);
    let mut buf = vec![0u8; buf_size];
    let started_at = Instant::now();
    let mut copied: u64 = 0;
    let mut last_emit_at = started_at;
    let mut last_emit_bytes: u64 = 0;
    let mut was_paused = false;

    let loop_result: Result<(), CopyError> = async {
        for extent in src_extents.iter() {
            if extent.is_empty() {
                continue;
            }
            // Seek both sides to the extent's offset. Holes between
            // extents are left unwritten on the destination.
            src_file
                .seek(std::io::SeekFrom::Start(extent.offset))
                .await
                .map_err(|e| CopyError::from_io(&src_path, &dst_path, e))?;
            dst_file
                .seek(std::io::SeekFrom::Start(extent.offset))
                .await
                .map_err(|e| CopyError::from_io(&src_path, &dst_path, e))?;

            let mut remaining = extent.len;
            while remaining > 0 {
                if ctrl.is_cancelled() {
                    return Err(CopyError::cancelled(&src_path, &dst_path));
                }
                if ctrl.is_paused() {
                    if !was_paused {
                        let _ = events.send(CopyEvent::Paused).await;
                        was_paused = true;
                    }
                    ctrl.wait_while_paused().await;
                    if ctrl.is_cancelled() {
                        return Err(CopyError::cancelled(&src_path, &dst_path));
                    }
                    if was_paused {
                        let _ = events.send(CopyEvent::Resumed).await;
                        was_paused = false;
                    }
                }

                let want = remaining.min(buf.len() as u64) as usize;
                let slice = &mut buf[..want];
                src_file
                    .read_exact(slice)
                    .await
                    .map_err(|e| CopyError::from_io(&src_path, &dst_path, e))?;
                dst_file
                    .write_all(slice)
                    .await
                    .map_err(|e| CopyError::from_io(&src_path, &dst_path, e))?;

                if let Some(shape) = opts.shape.as_ref() {
                    shape.permit(want as u64).await;
                }

                copied += want as u64;
                remaining -= want as u64;

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
            }
        }
        Ok(())
    }
    .await;

    if let Err(err) = loop_result {
        let _ = dst_file.flush().await;
        drop(dst_file);
        drop(src_file);
        if !opts.keep_partial {
            let _ = tokio::fs::remove_file(&dst_path).await;
        }
        let _ = events.send(CopyEvent::Failed { err: err.clone() }).await;
        return Err(err);
    }

    // Flush + optional fsync so the verify pass (or the post-copy
    // extent scan) reads bytes that are actually durable.
    dst_file
        .flush()
        .await
        .map_err(|e| CopyError::from_io(&src_path, &dst_path, e))?;
    let should_fsync = opts.fsync_on_close || (opts.verify.is_some() && opts.fsync_before_verify);
    if should_fsync && let Err(e) = dst_file.sync_all().await {
        return Err(CopyError::from_io(&src_path, &dst_path, e));
    }
    drop(dst_file);
    drop(src_file);

    preserve_metadata_with_events(
        src_path.clone(),
        dst_path.clone(),
        &src_metadata,
        &opts,
        Some(&events),
    )
    .await?;

    // Verify pass — full-file byte-for-byte hash. Holes read back as
    // zeros on both sides, so a successful hash compare confirms the
    // hole layout was preserved *and* the data matches. On top of
    // that we check the destination's allocated-byte count didn't
    // balloon (which would indicate the FS silently densified).
    if let Some(verifier) = opts.verify.as_ref() {
        // Rebuild a source-side hash by reading the full file
        // sequentially — simpler than weaving hole bytes into the
        // extent loop. This is still only one extra pass over the
        // source, matching the dense verify path's cost.
        let src_full = {
            let path = src_path.clone();
            let algo = verifier.clone();
            tokio::task::spawn_blocking(move || {
                let mut h = algo.make();
                let mut f = std::fs::File::open(&path)?;
                let mut buffer = vec![0u8; 1 << 20];
                use std::io::Read as _;
                loop {
                    let n = f.read(&mut buffer)?;
                    if n == 0 {
                        break;
                    }
                    h.update(&buffer[..n]);
                }
                Ok::<Box<dyn Hasher>, std::io::Error>(h)
            })
            .await
            .map_err(|e| {
                CopyError::from_io(
                    &src_path,
                    &dst_path,
                    std::io::Error::other(format!("verify-src join: {e}")),
                )
            })?
            .map_err(|e| CopyError::from_io(&src_path, &dst_path, e))?
        };
        run_verify_pass(
            &src_path, &dst_path, &opts, verifier, src_full, &ctrl, &events,
        )
        .await?;
    }

    // Phase 23 — post-copy extent scan. Allocated bytes on the
    // destination must not exceed the source's allocated footprint
    // plus one filesystem cluster of slop. Past that and we densified.
    let src_allocated: u64 = crate::sparse::allocated_bytes(&src_extents);
    let dst_extents_result = {
        let ops = sparse_ops.clone();
        let dst_clone = dst_path.clone();
        tokio::task::spawn_blocking(move || ops.detect_extents(&dst_clone))
            .await
            .unwrap_or_else(|_| Ok(Vec::new()))
    };
    let dst_allocated: u64 = match &dst_extents_result {
        Ok(ex) => crate::sparse::allocated_bytes(ex),
        Err(_) => total, // if we can't probe, assume worst-case dense
    };
    // Cluster slop: 1 MiB covers the common NTFS/ext4/APFS allocation
    // units and leaves headroom for a 4 KiB cluster FS with up to 256
    // extents of rounding slop. A well-behaved FS reports equal or
    // smaller.
    const SPARSE_SLOP_BYTES: u64 = 1 << 20;
    if dst_allocated > src_allocated + SPARSE_SLOP_BYTES && dst_allocated >= total {
        let empty: Vec<crate::sparse::ByteRange> = Vec::new();
        let dst_ex = dst_extents_result.as_ref().unwrap_or(&empty);
        let err = CopyError::sparseness_mismatch(&src_path, &dst_path, &src_extents, dst_ex);
        if !opts.keep_partial {
            let _ = tokio::fs::remove_file(&dst_path).await;
        }
        let _ = events.send(CopyEvent::Failed { err: err.clone() }).await;
        return Err(err);
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

/// Phase 23 — lookup for the short-name filesystem label the
/// `SparsenessNotSupported` event reports. Returns a stable wire
/// string even when the probe fails (`"unknown"`).
fn detect_dst_fs_label(_dst: &Path) -> &'static str {
    // copythat-core deliberately avoids the OS FFI; the platform
    // crate's `filesystem_name` gives the real answer. From core we
    // can only return "unknown" — the caller in Tauri can enrich the
    // event with the platform-probed label before forwarding it to
    // the UI.
    "unknown"
}

/// Phase 32e — chunked read of the source file into a Vec, emitting
/// `CopyEvent::Progress` after every chunk so the UI sees live
/// progress during large uploads. Observes `ctrl` between chunks
/// so cancel takes effect within a buffer-size slice.
async fn read_src_with_progress(
    src_path: &Path,
    dst_path: &Path,
    total_bytes: u64,
    buffer_size: usize,
    ctrl: &CopyControl,
    events: &mpsc::Sender<CopyEvent>,
    start: Instant,
) -> Result<Vec<u8>, CopyError> {
    use tokio::io::AsyncReadExt;

    let mut file = match tokio::fs::File::open(src_path).await {
        Ok(f) => f,
        Err(e) => return Err(CopyError::from_io(src_path, dst_path, e)),
    };
    let mut out: Vec<u8> = Vec::with_capacity(total_bytes as usize);
    let mut buf = vec![0u8; buffer_size];
    let mut bytes_done: u64 = 0;
    loop {
        if ctrl.is_cancelled() {
            return Err(CopyError {
                kind: crate::error::CopyErrorKind::Interrupted,
                src: src_path.to_path_buf(),
                dst: dst_path.to_path_buf(),
                raw_os_error: None,
                message: "cancelled during cloud-sink upload".into(),
            });
        }
        let n = match file.read(&mut buf).await {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) => return Err(CopyError::from_io(src_path, dst_path, e)),
        };
        out.extend_from_slice(&buf[..n]);
        bytes_done += n as u64;
        let rate_bps = rate_bps_from(bytes_done, start.elapsed());
        let _ = events
            .send(CopyEvent::Progress {
                bytes: bytes_done,
                total: total_bytes,
                rate_bps,
            })
            .await;
    }
    Ok(out)
}

/// Derive bytes-per-second for `Progress` events. Zero when the
/// elapsed window is degenerate (< 1 ms).
fn rate_bps_from(bytes_done: u64, elapsed: std::time::Duration) -> u64 {
    if elapsed.as_secs_f64() > 0.0 {
        (bytes_done as f64 / elapsed.as_secs_f64()) as u64
    } else {
        0
    }
}

/// Phase 32d — push a local source file to a remote [`crate::CloudSink`]
/// destination. Called from `copy_file` when `opts.cloud_sink` is
/// set; keeps the complex local-to-local paths uncluttered.
///
/// Reads the full source into memory (tractable for the typical
/// snapshot-shaped workload; Phase 32e switches to a streaming
/// reader + `opendal::Operator::writer()` chunking when local
/// memory pressure matters). `put_blocking` runs on a
/// `spawn_blocking` pool so the async copy task stays free.
async fn copy_file_to_cloud_sink(
    src_path: PathBuf,
    dst_path: PathBuf,
    opts: CopyOptions,
    ctrl: CopyControl,
    events: mpsc::Sender<CopyEvent>,
    src_metadata: std::fs::Metadata,
    sink: std::sync::Arc<dyn crate::options::CloudSink>,
) -> Result<CopyReport, CopyError> {
    let total_bytes = src_metadata.len();
    let _ = events
        .send(CopyEvent::Started {
            src: src_path.clone(),
            dst: dst_path.clone(),
            total_bytes,
        })
        .await;

    let _ = ctrl; // ctrl is observed between chunk reads below

    let start = Instant::now();

    // Phase 32e — chunked progress reporting. Rather than reading
    // the whole source in one shot, pull it in `opts.buffer_size`
    // chunks and emit `CopyEvent::Progress` after each. The bytes
    // still land in a single `Vec` and get flushed to the sink in
    // one `put_blocking` call — full streaming through the sink
    // (OpenDAL's writer API) lands in Phase 32f once the
    // `CopyTarget` trait gains a streaming-writer surface.
    let buffer_size = opts.buffer_size.clamp(
        crate::options::MIN_BUFFER_SIZE,
        crate::options::MAX_BUFFER_SIZE,
    );
    let bytes: Vec<u8> = match read_src_with_progress(
        &src_path,
        &dst_path,
        total_bytes,
        buffer_size,
        &ctrl,
        &events,
        start,
    )
    .await
    {
        Ok(b) => b,
        Err(err) => {
            let _ = events.send(CopyEvent::Failed { err: err.clone() }).await;
            return Err(err);
        }
    };

    let dst_key = dst_path.to_string_lossy().into_owned();
    let sink_for_blocking = sink.clone();
    let put_result = tokio::task::spawn_blocking(move || {
        sink_for_blocking.put_blocking(&dst_key, &bytes)
    })
    .await;

    let written = match put_result {
        Ok(Ok(n)) => n,
        Ok(Err(msg)) => {
            let err = CopyError::from_io(
                &src_path,
                &dst_path,
                std::io::Error::other(format!("cloud-sink put failed: {msg}")),
            );
            let _ = events.send(CopyEvent::Failed { err: err.clone() }).await;
            return Err(err);
        }
        Err(join_err) => {
            let err = CopyError::from_io(
                &src_path,
                &dst_path,
                std::io::Error::other(format!("cloud-sink worker join: {join_err}")),
            );
            let _ = events.send(CopyEvent::Failed { err: err.clone() }).await;
            return Err(err);
        }
    };

    let elapsed = start.elapsed();
    let rate_bps = rate_bps_from(written, elapsed);
    let _ = events
        .send(CopyEvent::Progress {
            bytes: written,
            total: total_bytes,
            rate_bps,
        })
        .await;
    let _ = events
        .send(CopyEvent::Completed {
            bytes: written,
            duration: elapsed,
            rate_bps,
        })
        .await;
    Ok(CopyReport {
        src: src_path,
        dst: dst_path,
        bytes: written,
        duration: elapsed,
        rate_bps,
    })
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

/// Phase 20 — what the resume probe decided. Drives the dst open mode
/// (truncate vs. append-and-seek), the initial value of `copied`, and
/// whether the engine should skip the copy loop entirely.
#[derive(Debug)]
enum ResumeDecision {
    /// No journal, or journal said `Restart`, or any prefix
    /// verification failed. Open dst with truncate, copy from byte 0.
    FreshStart,
    /// Journal said `Resume` *and* the prefix re-hash matched. Open
    /// dst without truncate, seek both ends to `offset`, continue
    /// from there. `prefix_bytes_hash` is the BLAKE3 of the verified
    /// dst prefix — kept for asserts and for future "verify prefix
    /// also after resume" passes.
    Resume {
        offset: u64,
        prefix_bytes_hash: [u8; 32],
    },
    /// Journal said `AlreadyComplete` *and* the destination's full-
    /// file hash matched. Skip the copy loop; the caller short-
    /// circuits to a synthetic `Completed` event.
    AlreadyComplete,
}

/// Probe the journal + the existing destination and decide whether
/// to resume, restart, or skip-as-already-done.
///
/// Emits `CopyEvent::ResumeAborted` on every fall-through-to-restart
/// path so the UI can surface "we tried to resume but had to start
/// over" instead of silently rewriting the prefix bytes.
async fn decide_resume(
    dst_path: &Path,
    expected_total: u64,
    opts: &CopyOptions,
    events: &mpsc::Sender<CopyEvent>,
) -> Result<ResumeDecision, CopyError> {
    let Some(journal) = opts.journal.as_ref() else {
        return Ok(ResumeDecision::FreshStart);
    };
    let dst_meta = match tokio::fs::metadata(dst_path).await {
        Ok(m) => m,
        // dst doesn't exist (or stat failed) — fresh start, no event.
        Err(_) => return Ok(ResumeDecision::FreshStart),
    };
    let dst_len = dst_meta.len();
    if dst_len == 0 {
        return Ok(ResumeDecision::FreshStart);
    }

    let plan = journal.resume_plan(opts.journal_file_idx);
    match plan {
        ResumePlan::Restart => Ok(ResumeDecision::FreshStart),
        ResumePlan::AlreadyComplete { final_hash } => {
            if dst_len != expected_total {
                let _ = events
                    .send(CopyEvent::ResumeAborted {
                        reason: "dst-length-mismatch",
                        offset: 0,
                    })
                    .await;
                return Ok(ResumeDecision::FreshStart);
            }
            let computed = match hash_dst_prefix(dst_path, dst_len).await {
                Ok(h) => h,
                Err(_) => {
                    let _ = events
                        .send(CopyEvent::ResumeAborted {
                            reason: "dst-read-failed",
                            offset: 0,
                        })
                        .await;
                    return Ok(ResumeDecision::FreshStart);
                }
            };
            if computed == final_hash {
                Ok(ResumeDecision::AlreadyComplete)
            } else {
                let _ = events
                    .send(CopyEvent::ResumeAborted {
                        reason: "complete-hash-mismatch",
                        offset: 0,
                    })
                    .await;
                Ok(ResumeDecision::FreshStart)
            }
        }
        ResumePlan::Resume {
            offset,
            src_hash_at_offset,
        } => {
            // Conservative: when the user opted into post-copy
            // verify with a non-BLAKE3 algorithm, the partial
            // source-side hasher state would mismatch the journal's
            // BLAKE3. Restart from scratch in that case rather than
            // ship a verify mismatch on resume.
            if opts.verify.is_some() {
                let _ = events
                    .send(CopyEvent::ResumeAborted {
                        reason: "verify-incompatible",
                        offset,
                    })
                    .await;
                return Ok(ResumeDecision::FreshStart);
            }
            if dst_len < offset {
                // The journal optimistically counted bytes that
                // didn't actually hit the disk before SIGKILL.
                // Safer to restart than to chase an offset that
                // isn't there.
                let _ = events
                    .send(CopyEvent::ResumeAborted {
                        reason: "dst-shrunk",
                        offset,
                    })
                    .await;
                return Ok(ResumeDecision::FreshStart);
            }
            let computed = match hash_dst_prefix(dst_path, offset).await {
                Ok(h) => h,
                Err(_) => {
                    let _ = events
                        .send(CopyEvent::ResumeAborted {
                            reason: "dst-read-failed",
                            offset,
                        })
                        .await;
                    return Ok(ResumeDecision::FreshStart);
                }
            };
            if computed == src_hash_at_offset {
                Ok(ResumeDecision::Resume {
                    offset,
                    prefix_bytes_hash: computed,
                })
            } else {
                let _ = events
                    .send(CopyEvent::ResumeAborted {
                        reason: "prefix-hash-mismatch",
                        offset,
                    })
                    .await;
                Ok(ResumeDecision::FreshStart)
            }
        }
    }
}

/// Stream-hash the first `n` bytes of `path` with BLAKE3. Used by
/// both branches of `decide_resume`. 64 KiB read buffer matches
/// the `BufReader` default and is a safe lower bound on every
/// supported filesystem.
async fn hash_dst_prefix(path: &Path, n: u64) -> std::io::Result<[u8; 32]> {
    let mut f = tokio::fs::File::open(path).await?;
    let mut hasher = blake3::Hasher::new();
    let mut remaining = n;
    let mut buf = vec![0u8; 64 * 1024];
    while remaining > 0 {
        let to_read = std::cmp::min(buf.len() as u64, remaining) as usize;
        let read = f.read(&mut buf[..to_read]).await?;
        if read == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "dst shorter than requested prefix",
            ));
        }
        hasher.update(&buf[..read]);
        remaining -= read as u64;
    }
    Ok(*hasher.finalize().as_bytes())
}

/// Re-feed the dst prefix into a journal hasher so that, after a
/// successful resume, the running BLAKE3 represents the *whole*
/// source content (prefix + the bytes the engine is about to copy),
/// not just the post-resume tail.
async fn prime_blake3_from_dst_prefix(
    hasher: &mut blake3::Hasher,
    dst_path: &Path,
    n: u64,
) -> std::io::Result<()> {
    let mut f = tokio::fs::File::open(dst_path).await?;
    let mut remaining = n;
    let mut buf = vec![0u8; 64 * 1024];
    while remaining > 0 {
        let to_read = std::cmp::min(buf.len() as u64, remaining) as usize;
        let read = f.read(&mut buf[..to_read]).await?;
        if read == 0 {
            break;
        }
        hasher.update(&buf[..read]);
        remaining -= read as u64;
    }
    Ok(())
}

/// Best-effort sync-failure logger. Kept as a thin wrapper so the
/// tracing dep stays internal — the engine itself does not panic
/// or surface the error; it's informational.
fn tracing_log_sync_failure(_dst: &Path, _err: &std::io::Error) {
    // Intentionally a no-op until structured logging is wired
    // workspace-wide. Keeping the call site so the future logger
    // lands here without engine churn.
}

/// Best-effort meta-apply failure logger. Phase 24's apply pass is
/// strictly post-byte-copy — losing xattrs / ADS / ACLs should not
/// abort the operation that already succeeded.
fn tracing_log_meta_failure(_dst: &Path, _err: &std::io::Error) {
    // Same intentional no-op as `tracing_log_sync_failure`.
}

/// Phase 19b — open the source, then fall through to a snapshot if
/// the short retry loop exhausts and the caller requested it.
///
/// The returned `Option<SnapshotLease>` is held by the caller for the
/// full duration of the copy; its `Drop` releases the backend
/// snapshot. On the happy path (no lock) the Option is `None`.
async fn open_src_with_snapshot_fallback(
    src: &Path,
    dst: &Path,
    opts: &CopyOptions,
    events: &mpsc::Sender<CopyEvent>,
) -> Result<(File, Option<SnapshotLease>), CopyError> {
    match open_src_with_retry(src).await {
        Ok(f) => Ok((f, None)),
        Err(e) if is_sharing_violation(&e) => {
            match opts.on_locked {
                LockedFilePolicy::Snapshot => {
                    let Some(hook) = opts.snapshot_hook.clone() else {
                        return Err(CopyError::from_io(src, dst, e));
                    };
                    let lease = hook.open_for_read(src.to_path_buf()).await?;
                    let _ = events
                        .send(CopyEvent::SnapshotCreated {
                            kind: lease.kind_wire,
                            original: src.to_path_buf(),
                            snap_mount: lease.mount_root.clone(),
                        })
                        .await;
                    let translated = lease.translated.clone();
                    match open_src_with_retry(&translated).await {
                        Ok(f) => Ok((f, Some(lease))),
                        Err(open_err) => Err(CopyError::from_io(src, dst, open_err)),
                    }
                }
                // Retry / Skip / Ask all fall through to the
                // sharing-violation error unchanged — Retry already
                // ran inside open_src_with_retry, Skip is applied by
                // the tree layer, Ask gets upgraded to one of the
                // others by the runner before the engine is entered.
                LockedFilePolicy::Retry | LockedFilePolicy::Skip | LockedFilePolicy::Ask => {
                    Err(CopyError::from_io(src, dst, e))
                }
            }
        }
        Err(e) => Err(CopyError::from_io(src, dst, e)),
    }
}

/// True when `err` indicates the source file is exclusively locked by
/// another process. The rules are per-OS:
///
/// - Windows: `ERROR_SHARING_VIOLATION` (32) or `ERROR_LOCK_VIOLATION` (33).
/// - Unix: `EBUSY` (16 on Linux) — triggered by certain FUSE mounts
///   (fuse-overlayfs holding the underlying inode) and by kernel
///   modules that refuse open-for-read on live files.
fn is_sharing_violation(err: &std::io::Error) -> bool {
    match err.raw_os_error() {
        #[cfg(windows)]
        Some(32) | Some(33) => true,
        #[cfg(unix)]
        Some(code) => code == libc_ebusy(),
        _ => false,
    }
}

#[cfg(unix)]
#[inline]
#[allow(non_snake_case)]
fn libc_ebusy() -> i32 {
    // EBUSY is 16 on Linux / macOS / BSD. We avoid pulling in the
    // `libc` crate here — copythat-core is unsafe-code-free — so
    // hardcode the well-known value.
    16
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
