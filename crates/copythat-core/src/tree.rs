//! Tree-level copy and move.
//!
//! `copy_tree` runs a streaming walker + dispatcher: a background
//! task feeds `Entry` items into a bounded channel as `walkdir`
//! yields them, while a dispatcher pulls entries off the channel
//! and processes them — directories are created at the destination
//! inline, files are handed off to a concurrency-limited worker
//! pool. The two phases overlap: copies start as soon as the first
//! directory has been seen, and the engine never holds the whole
//! tree in memory. Tree size is bounded only by the destination
//! volume, not by RAM — a 100 M-file / 2 PB source is the same
//! workload as a 10-file one, just longer.
//!
//! `move_tree` tries an atomic `rename` first; on cross-device
//! failure it falls back to `copy_tree` + a streaming, bottom-up
//! source-deletion walk.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use filetime::FileTime;
use tokio::sync::{Semaphore, mpsc, oneshot};
use tokio::task::JoinSet;

use crate::collision::{self, Decision};
use crate::control::CopyControl;
use crate::engine::copy_file;
use crate::error::{CopyError, CopyErrorKind};
use crate::event::{CopyEvent, ErrorPrompt, TreeReport};
use crate::options::{ErrorAction, ErrorPolicy, MoveOptions, TreeOptions};

/// Copy `src_dir` into `dst_dir`, preserving structure.
///
/// `src_dir` must be an existing directory. `dst_dir` is created if it
/// doesn't exist; existing files inside it follow `opts.collision`.
pub async fn copy_tree(
    src_dir: &Path,
    dst_dir: &Path,
    opts: TreeOptions,
    ctrl: CopyControl,
    events: mpsc::Sender<CopyEvent>,
) -> Result<TreeReport, CopyError> {
    copy_tree_inner(src_dir, dst_dir, opts, ctrl, events).await
}

/// Move a single file. Tries `rename` first, falls back to
/// copy-then-delete on EXDEV (or any rename failure when
/// `opts.strict_rename == false`).
pub async fn move_file(
    src: &Path,
    dst: &Path,
    opts: MoveOptions,
    ctrl: CopyControl,
    events: mpsc::Sender<CopyEvent>,
) -> Result<crate::event::CopyReport, CopyError> {
    // Fast path: atomic rename.
    match tokio::fs::rename(src, dst).await {
        Ok(()) => {
            let meta = tokio::fs::metadata(dst).await.ok();
            let bytes = meta.as_ref().map(|m| m.len()).unwrap_or(0);
            let report = crate::event::CopyReport {
                src: src.to_path_buf(),
                dst: dst.to_path_buf(),
                bytes,
                duration: Duration::ZERO,
                rate_bps: 0,
            };
            let _ = events
                .send(CopyEvent::Completed {
                    bytes,
                    duration: Duration::ZERO,
                    rate_bps: 0,
                })
                .await;
            return Ok(report);
        }
        Err(e) => {
            if opts.strict_rename || !is_cross_device(&e) {
                if opts.strict_rename {
                    return Err(CopyError::from_io(src, dst, e));
                }
                // If not strict and the error is something other than
                // CrossesDevices (e.g. destination exists), propagate
                // the IO error rather than silently falling back.
                if !is_cross_device(&e) {
                    return Err(CopyError::from_io(src, dst, e));
                }
            }
        }
    }

    // Slow path: copy + delete.
    let report = copy_file(src, dst, opts.copy.clone(), ctrl.clone(), events.clone()).await?;
    if ctrl.is_cancelled() {
        return Err(CopyError::cancelled(src, dst));
    }
    if let Err(e) = tokio::fs::remove_file(src).await {
        return Err(CopyError::from_io(src, dst, e));
    }
    Ok(report)
}

/// Move `src_dir` to `dst_dir`. Rename first, fall back to
/// `copy_tree` + bottom-up deletion on cross-device error.
pub async fn move_tree(
    src_dir: &Path,
    dst_dir: &Path,
    opts: MoveOptions,
    ctrl: CopyControl,
    events: mpsc::Sender<CopyEvent>,
) -> Result<TreeReport, CopyError> {
    match tokio::fs::rename(src_dir, dst_dir).await {
        Ok(()) => {
            let meta = tokio::fs::metadata(dst_dir).await.ok();
            let _ = meta; // we don't post-enumerate for report stats on atomic rename
            let report = TreeReport {
                root_src: src_dir.to_path_buf(),
                root_dst: dst_dir.to_path_buf(),
                files: 0,
                bytes: 0,
                duration: Duration::ZERO,
                rate_bps: 0,
                skipped: 0,
                errored: 0,
            };
            let _ = events
                .send(CopyEvent::TreeCompleted {
                    files: 0,
                    bytes: 0,
                    duration: Duration::ZERO,
                    rate_bps: 0,
                })
                .await;
            return Ok(report);
        }
        Err(e) => {
            if opts.strict_rename || !is_cross_device(&e) {
                return Err(CopyError::from_io(src_dir, dst_dir, e));
            }
        }
    }

    let tree_opts = TreeOptions {
        file: opts.copy.clone(),
        ..TreeOptions::default()
    };
    let report = copy_tree_inner(src_dir, dst_dir, tree_opts, ctrl.clone(), events.clone()).await?;
    if ctrl.is_cancelled() {
        return Err(CopyError::cancelled(src_dir, dst_dir));
    }

    // Streaming bottom-up source deletion. `walkdir::contents_first`
    // yields files before their containing directory, which is what
    // delete order wants — we never hold the full list in memory.
    let src_for_delete = src_dir.to_path_buf();
    let dst_for_delete = dst_dir.to_path_buf();
    let delete_result = tokio::task::spawn_blocking(move || -> Result<(), CopyError> {
        for entry in walkdir::WalkDir::new(&src_for_delete).contents_first(true) {
            let entry = match entry {
                Ok(e) => e,
                Err(e) => {
                    let denied = e
                        .io_error()
                        .map(|io| io.kind() == std::io::ErrorKind::PermissionDenied)
                        .unwrap_or(false);
                    if denied {
                        continue;
                    }
                    return Err(CopyError::from_io(
                        &src_for_delete,
                        &dst_for_delete,
                        std::io::Error::other(format!("walk error at {:?}: {e}", e.path())),
                    ));
                }
            };
            let path = entry.path();
            let ft = entry.file_type();
            let result = if ft.is_dir() {
                std::fs::remove_dir(path)
            } else {
                std::fs::remove_file(path)
            };
            if let Err(e) = result {
                match e.kind() {
                    std::io::ErrorKind::NotFound | std::io::ErrorKind::DirectoryNotEmpty => {}
                    _ => return Err(CopyError::from_io(path, &dst_for_delete, e)),
                }
            }
        }
        Ok(())
    })
    .await
    .map_err(|e| CopyError {
        kind: CopyErrorKind::IoOther,
        src: src_dir.to_path_buf(),
        dst: dst_dir.to_path_buf(),
        raw_os_error: None,
        message: format!("source-delete task panicked: {e}"),
    })?;
    delete_result?;
    Ok(report)
}

#[cfg(unix)]
fn is_cross_device(e: &std::io::Error) -> bool {
    // EXDEV = 18 on Linux, macOS, BSD. Also expose CrossesDevices on
    // recent Rust.
    matches!(e.raw_os_error(), Some(18))
        || e.kind().to_string().eq_ignore_ascii_case("crosses devices")
}

#[cfg(windows)]
fn is_cross_device(e: &std::io::Error) -> bool {
    // ERROR_NOT_SAME_DEVICE = 17. Rust's CrossesDevices kind is also
    // mapped on recent toolchains.
    matches!(e.raw_os_error(), Some(17))
        || e.kind().to_string().eq_ignore_ascii_case("crosses devices")
}

#[cfg(not(any(unix, windows)))]
fn is_cross_device(e: &std::io::Error) -> bool {
    e.kind().to_string().eq_ignore_ascii_case("crosses devices")
}

// ---------- internals ----------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EntryKind {
    Dir,
    File,
    Symlink,
}

#[derive(Debug, Clone)]
struct Entry {
    src: PathBuf,
    /// Path relative to `src_dir`.
    rel: PathBuf,
    kind: EntryKind,
}

#[derive(Debug, Default)]
struct Plan {
    entries: Vec<Entry>,
    total_files: u64,
    total_bytes: u64,
}

async fn copy_tree_inner(
    src_dir: &Path,
    dst_dir: &Path,
    opts: TreeOptions,
    ctrl: CopyControl,
    events: mpsc::Sender<CopyEvent>,
) -> Result<TreeReport, CopyError> {
    let src_root = src_dir.to_path_buf();
    let dst_root = dst_dir.to_path_buf();

    // Validate source.
    let src_meta = tokio::fs::metadata(&src_root)
        .await
        .map_err(|e| CopyError::from_io(&src_root, &dst_root, e))?;
    if !src_meta.is_dir() {
        return Err(CopyError {
            kind: CopyErrorKind::IoOther,
            src: src_root.clone(),
            dst: dst_root.clone(),
            raw_os_error: None,
            message: "copy_tree source is not a directory".to_string(),
        });
    }

    // Ensure destination root exists.
    if let Err(e) = tokio::fs::create_dir_all(&dst_root).await {
        let err = CopyError::from_io(&src_root, &dst_root, e);
        let _ = events.send(CopyEvent::Failed { err: err.clone() }).await;
        return Err(err);
    }

    // TreeStarted fires with zeros — with streaming enumeration we
    // don't know the final totals until the walker finishes. The
    // TreeEnumerating + TreeProgress events grow the UI's
    // denominator as discovery continues.
    let _ = events
        .send(CopyEvent::TreeStarted {
            root_src: src_root.clone(),
            root_dst: dst_root.clone(),
            total_files: 0,
            total_bytes: 0,
        })
        .await;

    let started = Instant::now();
    let bytes_done = Arc::new(AtomicU64::new(0));
    let files_done = Arc::new(AtomicU64::new(0));
    let skipped = Arc::new(AtomicU64::new(0));
    let errored = Arc::new(AtomicU64::new(0));
    // Growing denominator. Each chunk received from the walker adds
    // its own `total_files` / `total_bytes` into these counters;
    // per-file TreeProgress events read them as the "total so far".
    let files_total_so_far = Arc::new(AtomicU64::new(0));
    let bytes_total_so_far = Arc::new(AtomicU64::new(0));

    let on_error = opts.clamped_on_error();
    let semaphore = Arc::new(Semaphore::new(opts.clamped_concurrency()));
    let mut set: JoinSet<Result<FileOutcome, CopyError>> = JoinSet::new();

    // Dir accumulator for preserve_directory_times — we only remember
    // (src, dst) pairs, not whole Entry objects. Cheap enough to
    // keep for any realistic tree (10 M dirs × ~200 B = 2 GB only
    // at the extreme end; default walks stay well under that).
    let mut all_dirs: Vec<(PathBuf, PathBuf)> = Vec::new();

    // Spawn walker. Channel capacity 2 = one chunk can be in flight
    // while the dispatcher processes the previous one, modest
    // backpressure when the dispatcher falls behind.
    let (chunk_tx, mut chunk_rx) = mpsc::channel::<Plan>(2);
    let events_for_walker = events.clone();
    let follow_symlinks = opts.follow_symlinks_in_tree;
    let src_for_walker = src_root.clone();
    let walker_handle = tokio::task::spawn_blocking(move || {
        enumerate_streaming(
            src_for_walker,
            follow_symlinks,
            chunk_tx,
            events_for_walker,
        )
    });

    let mut aborted = false;
    let mut first_error: Option<CopyError> = None;

    // Consume chunks as the walker produces them.
    while let Some(chunk) = chunk_rx.recv().await {
        if ctrl.is_cancelled() {
            break;
        }

        // Grow the discovered totals. Per-file TreeProgress events
        // read these atomics as the denominator, so by the time the
        // user sees "X / Y" the Y is always ≥ what's been discovered.
        files_total_so_far.fetch_add(chunk.total_files, Ordering::Relaxed);
        bytes_total_so_far.fetch_add(chunk.total_bytes, Ordering::Relaxed);

        // Recreate this chunk's directories. Walkdir yields dirs
        // shallow-first within each chunk, so create_dir_all
        // (which no-ops on existing) lands in order.
        for entry in chunk.entries.iter().filter(|e| e.kind == EntryKind::Dir) {
            if ctrl.is_cancelled() {
                break;
            }
            let dst_path = dst_root.join(&entry.rel);
            if let Err(e) = tokio::fs::create_dir_all(&dst_path).await {
                let err = CopyError::from_io(&entry.src, &dst_path, e);
                if first_error.is_none() {
                    first_error = Some(err);
                }
                ctrl.cancel();
                break;
            }
            if opts.preserve_directory_times {
                all_dirs.push((entry.src.clone(), dst_path));
            }
        }

        // Spawn copies for file / symlink entries in this chunk.
        for entry in chunk.entries.into_iter().filter(|e| e.kind != EntryKind::Dir) {
            if ctrl.is_cancelled() {
                break;
            }
            let permit_owner = semaphore.clone();
            let ctrl_task = ctrl.clone();
            let events_task = events.clone();
            let opts_file = opts.file.clone();
            let collision = opts.collision.clone();
            let dst_root_task = dst_root.clone();
            let bytes_done_task = bytes_done.clone();
            let files_done_task = files_done.clone();
            let skipped_task = skipped.clone();
            let errored_task = errored.clone();
            let files_total_task = files_total_so_far.clone();
            let bytes_total_task = bytes_total_so_far.clone();
            let on_error_task = on_error;

            set.spawn(async move {
                let permit = permit_owner.acquire_owned().await.map_err(|_| CopyError {
                    kind: CopyErrorKind::IoOther,
                    src: entry.src.clone(),
                    dst: dst_root_task.join(&entry.rel),
                    raw_os_error: None,
                    message: "tree copy semaphore closed".to_string(),
                })?;

                let dst_initial = dst_root_task.join(&entry.rel);
                let decision =
                    collision::resolve(&collision, &entry.src, &dst_initial, &events_task).await;

                let outcome: Result<FileOutcome, CopyError> = match decision {
                    Decision::Skip => {
                        skipped_task.fetch_add(1, Ordering::Relaxed);
                        Ok(FileOutcome::Skipped)
                    }
                    Decision::Abort => Ok(FileOutcome::Aborted),
                    Decision::Write(dst_final) => match entry.kind {
                        EntryKind::Symlink => {
                            match copy_symlink_entry(&entry.src, &dst_final).await {
                                Ok(()) => Ok(FileOutcome::Done(0)),
                                Err(err) => {
                                    handle_per_file_error(
                                        err,
                                        on_error_task,
                                        &events_task,
                                        &errored_task,
                                    )
                                    .await
                                }
                            }
                        }
                        EntryKind::File => {
                            attempt_copy_with_policy(
                                &entry.src,
                                &dst_final,
                                &opts_file,
                                &ctrl_task,
                                &events_task,
                                on_error_task,
                                &errored_task,
                            )
                            .await
                        }
                        EntryKind::Dir => unreachable!("dirs filtered above"),
                    },
                };

                let outcome = outcome?;
                if let FileOutcome::Done(bytes) = &outcome {
                    let done_bytes =
                        bytes_done_task.fetch_add(*bytes, Ordering::Relaxed) + *bytes;
                    let done_files = files_done_task.fetch_add(1, Ordering::Relaxed) + 1;
                    let tot_files = files_total_task.load(Ordering::Relaxed);
                    let tot_bytes = bytes_total_task.load(Ordering::Relaxed);
                    let elapsed = started.elapsed();
                    let rate = rate_bps(done_bytes, elapsed);
                    let _ = events_task
                        .send(CopyEvent::TreeProgress {
                            files_done: done_files,
                            files_total: tot_files,
                            bytes_done: done_bytes,
                            bytes_total: tot_bytes,
                            rate_bps: rate,
                        })
                        .await;
                }

                drop(permit);
                Ok(outcome)
            });
        }
    }

    // Walker has closed the channel (or dispatcher cancelled).
    // Drain the remaining copy tasks.
    while let Some(joined) = set.join_next().await {
        match joined {
            Ok(Ok(FileOutcome::Aborted)) => {
                aborted = true;
                ctrl.cancel();
            }
            Ok(Ok(_)) => {}
            Ok(Err(err)) => {
                if first_error.is_none() {
                    first_error = Some(err);
                }
                ctrl.cancel();
            }
            Err(_join_err) => {
                if first_error.is_none() {
                    first_error = Some(CopyError {
                        kind: CopyErrorKind::IoOther,
                        src: src_root.clone(),
                        dst: dst_root.clone(),
                        raw_os_error: None,
                        message: "tree copy task panicked".to_string(),
                    });
                }
                ctrl.cancel();
            }
        }
    }

    // Join the walker task itself so we can surface walker errors
    // (permission-denied on the root, non-dir source, etc.).
    match walker_handle.await {
        Ok(Ok(())) => {}
        Ok(Err(e)) => {
            let err = CopyError::from_io(&src_root, &dst_root, e);
            if first_error.is_none() {
                first_error = Some(err);
            }
        }
        Err(join_err) => {
            if first_error.is_none() {
                first_error = Some(CopyError {
                    kind: CopyErrorKind::IoOther,
                    src: src_root.clone(),
                    dst: dst_root.clone(),
                    raw_os_error: None,
                    message: format!("walker task panicked: {join_err}"),
                });
            }
        }
    }

    if let Some(err) = first_error {
        let _ = events.send(CopyEvent::Failed { err: err.clone() }).await;
        return Err(err);
    }
    if aborted || ctrl.is_cancelled() {
        let err = CopyError::cancelled(&src_root, &dst_root);
        let _ = events.send(CopyEvent::Failed { err: err.clone() }).await;
        return Err(err);
    }

    // Directory times last, deepest-first so setting a parent's
    // mtime doesn't get invalidated by a later file-copy into its
    // children.
    if opts.preserve_directory_times {
        all_dirs.sort_by_key(|(_, dst)| std::cmp::Reverse(dst.components().count()));
        for (src, dst) in all_dirs {
            let src_md = match std::fs::metadata(&src) {
                Ok(m) => m,
                Err(_) => continue,
            };
            let atime = FileTime::from_last_access_time(&src_md);
            let mtime = FileTime::from_last_modification_time(&src_md);
            let _ = tokio::task::spawn_blocking(move || {
                filetime::set_file_times(&dst, atime, mtime)
            })
            .await;
        }
    }

    let elapsed = started.elapsed();
    let final_bytes = bytes_done.load(Ordering::Relaxed);
    let final_files = files_done.load(Ordering::Relaxed);
    let rate = rate_bps(final_bytes, elapsed);
    let _ = events
        .send(CopyEvent::TreeCompleted {
            files: final_files,
            bytes: final_bytes,
            duration: elapsed,
            rate_bps: rate,
        })
        .await;

    let report = TreeReport {
        root_src: src_root,
        root_dst: dst_root,
        files: final_files,
        bytes: final_bytes,
        duration: elapsed,
        rate_bps: rate,
        skipped: skipped.load(Ordering::Relaxed),
        errored: errored.load(Ordering::Relaxed),
    };
    Ok(report)
}

enum FileOutcome {
    Done(u64),
    Skipped,
    /// Per-file failure absorbed by the error policy (Skip or
    /// RetryN-exhausted). Separate from `Skipped` because the tree
    /// report tracks each count independently.
    Errored,
    Aborted,
}

/// Drive per-file copy with retry / ask / skip / abort semantics.
///
/// Split out of the task closure for readability. Returns:
/// - `Ok(FileOutcome::Done(bytes))` on success.
/// - `Ok(FileOutcome::Errored)` when the policy absorbed the error
///   (tree continues).
/// - `Ok(FileOutcome::Aborted)` when the engine was cancelled via
///   `CopyControl` mid-attempt.
/// - `Err(CopyError)` only on `ErrorPolicy::Abort` (fatal) or an
///   `ErrorAction::Abort` response.
async fn attempt_copy_with_policy(
    src: &Path,
    dst: &Path,
    opts_file: &crate::options::CopyOptions,
    ctrl: &CopyControl,
    events: &mpsc::Sender<CopyEvent>,
    policy: ErrorPolicy,
    errored: &Arc<AtomicU64>,
) -> Result<FileOutcome, CopyError> {
    let mut retries_left: u32 = match policy {
        ErrorPolicy::RetryN { max_attempts, .. } => max_attempts as u32,
        _ => 0,
    };
    let backoff = match policy {
        ErrorPolicy::RetryN { backoff_ms, .. } => Duration::from_millis(backoff_ms),
        _ => Duration::ZERO,
    };

    loop {
        let result = copy_file(src, dst, opts_file.clone(), ctrl.clone(), events.clone()).await;
        match result {
            Ok(report) => return Ok(FileOutcome::Done(report.bytes)),
            Err(err) if err.is_cancelled() => return Ok(FileOutcome::Aborted),
            Err(err) => {
                match policy {
                    ErrorPolicy::Abort => return Err(err),
                    ErrorPolicy::Skip => {
                        record_file_error(err, events, errored).await;
                        return Ok(FileOutcome::Errored);
                    }
                    ErrorPolicy::RetryN { .. } => {
                        if retries_left == 0 {
                            record_file_error(err, events, errored).await;
                            return Ok(FileOutcome::Errored);
                        }
                        retries_left -= 1;
                        if !backoff.is_zero() {
                            tokio::time::sleep(backoff).await;
                        }
                        continue;
                    }
                    ErrorPolicy::Ask => {
                        let (tx, rx) = oneshot::channel();
                        let prompt = ErrorPrompt::new(err.clone(), tx);
                        // If the receiver is gone (event channel
                        // closed), treat as Skip — same pattern as
                        // Collision::resolve.
                        if events.send(CopyEvent::ErrorPrompt(prompt)).await.is_err() {
                            record_file_error(err, events, errored).await;
                            return Ok(FileOutcome::Errored);
                        }
                        match rx.await {
                            Ok(ErrorAction::Retry) => continue,
                            Ok(ErrorAction::Skip) | Err(_) => {
                                record_file_error(err, events, errored).await;
                                return Ok(FileOutcome::Errored);
                            }
                            Ok(ErrorAction::Abort) => return Err(err),
                        }
                    }
                }
            }
        }
    }
}

/// For paths that hit an error *before* `copy_file` ran (symlink
/// creation, etc.). Applies the same policy choice, minus the retry
/// loop (a symlink failure usually retries identically).
async fn handle_per_file_error(
    err: CopyError,
    policy: ErrorPolicy,
    events: &mpsc::Sender<CopyEvent>,
    errored: &Arc<AtomicU64>,
) -> Result<FileOutcome, CopyError> {
    match policy {
        ErrorPolicy::Abort => Err(err),
        ErrorPolicy::Skip | ErrorPolicy::RetryN { .. } => {
            record_file_error(err, events, errored).await;
            Ok(FileOutcome::Errored)
        }
        ErrorPolicy::Ask => {
            let (tx, rx) = oneshot::channel();
            let prompt = ErrorPrompt::new(err.clone(), tx);
            if events.send(CopyEvent::ErrorPrompt(prompt)).await.is_err() {
                record_file_error(err, events, errored).await;
                return Ok(FileOutcome::Errored);
            }
            match rx.await {
                Ok(ErrorAction::Retry) | Ok(ErrorAction::Skip) | Err(_) => {
                    // Retry doesn't make sense for non-copy_file
                    // failures; honour the user's intent as best we
                    // can — skip and keep going.
                    record_file_error(err, events, errored).await;
                    Ok(FileOutcome::Errored)
                }
                Ok(ErrorAction::Abort) => Err(err),
            }
        }
    }
}

async fn record_file_error(
    err: CopyError,
    events: &mpsc::Sender<CopyEvent>,
    errored: &Arc<AtomicU64>,
) {
    errored.fetch_add(1, Ordering::Relaxed);
    let _ = events.send(CopyEvent::FileError { err }).await;
}

/// Streaming enumerator. Runs in a `spawn_blocking` task, pushes
/// `Plan` chunks of up to `CHUNK_SIZE` entries through the channel
/// as walkdir yields them, and emits `TreeEnumerating` progress
/// ticks every `PROGRESS_EMIT_EVERY` discovered files. No in-memory
/// cap on total tree size — memory is bounded to one chunk at a
/// time (~60 MB per 100 k-entry chunk on Windows paths).
fn enumerate_streaming(
    root: PathBuf,
    follow_symlinks: bool,
    chunk_tx: mpsc::Sender<Plan>,
    events: mpsc::Sender<CopyEvent>,
) -> std::io::Result<()> {
    // Chunk size picks a point where the dispatcher's per-chunk
    // overhead (mkdir inline pass, spawn loop) is amortized across
    // enough entries to matter, without holding a huge batch in
    // memory. 100 k entries × ~500 B per Entry ≈ 50 MB peak.
    const CHUNK_SIZE: usize = 100_000;
    const PROGRESS_EMIT_EVERY: u64 = 500;

    eprintln!(
        "[tree::enumerate_streaming] begin root={}",
        root.display()
    );

    let mut current = Plan::default();
    let mut total_files: u64 = 0;
    let mut total_bytes: u64 = 0;
    let mut last_emitted: u64 = 0;
    let mut skipped_denied: u64 = 0;
    let mut chunks_sent: u64 = 0;

    let walker = walkdir::WalkDir::new(&root)
        .follow_links(follow_symlinks)
        .sort_by_file_name();
    for entry in walker {
        if total_files >= last_emitted + PROGRESS_EMIT_EVERY {
            let _ = events.try_send(CopyEvent::TreeEnumerating {
                files_so_far: total_files,
                bytes_so_far: total_bytes,
            });
            last_emitted = total_files;
        }
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                let denied = e
                    .io_error()
                    .map(|io| io.kind() == std::io::ErrorKind::PermissionDenied)
                    .unwrap_or(false);
                if denied {
                    skipped_denied = skipped_denied.saturating_add(1);
                    continue;
                }
                return Err(std::io::Error::other(format!(
                    "walk error at {:?}: {e}",
                    e.path()
                )));
            }
        };
        let path = entry.path();
        let rel = path.strip_prefix(&root).unwrap_or(path).to_path_buf();
        if rel.as_os_str().is_empty() {
            continue;
        }
        let ft = entry.file_type();
        let kind = if ft.is_dir() {
            EntryKind::Dir
        } else if ft.is_symlink() {
            EntryKind::Symlink
        } else {
            EntryKind::File
        };
        let len = if kind == EntryKind::File {
            entry.metadata().map(|m| m.len()).unwrap_or(0)
        } else {
            0
        };
        if kind == EntryKind::File {
            current.total_files += 1;
            current.total_bytes += len;
            total_files += 1;
            total_bytes += len;
        }
        current.entries.push(Entry {
            src: path.to_path_buf(),
            rel,
            kind,
        });

        if current.entries.len() >= CHUNK_SIZE {
            let ready = std::mem::take(&mut current);
            if chunk_tx.blocking_send(ready).is_err() {
                // Receiver dropped — dispatcher cancelled mid-walk.
                // Stop walking; the dispatcher handles teardown.
                eprintln!(
                    "[tree::enumerate_streaming] receiver dropped after {} chunks; stopping",
                    chunks_sent
                );
                return Ok(());
            }
            chunks_sent += 1;
        }
    }

    // Final emit so the UI counter lands on the real total.
    let _ = events.try_send(CopyEvent::TreeEnumerating {
        files_so_far: total_files,
        bytes_so_far: total_bytes,
    });

    // Send trailing partial chunk if any entries remain.
    if !current.entries.is_empty() {
        let _ = chunk_tx.blocking_send(current);
        chunks_sent += 1;
    }
    // Dropping chunk_tx here closes the channel — the dispatcher
    // will see `None` from `recv()` and exit its loop cleanly.
    drop(chunk_tx);

    eprintln!(
        "[tree::enumerate_streaming] done total_files={} total_bytes={} chunks={} skipped_denied={}",
        total_files, total_bytes, chunks_sent, skipped_denied
    );
    Ok(())
}

async fn copy_symlink_entry(src: &Path, dst: &Path) -> Result<(), CopyError> {
    // Best effort: remove dst if present, then re-create the symlink.
    let _ = tokio::fs::remove_file(dst).await;
    let target = tokio::fs::read_link(src)
        .await
        .map_err(|e| CopyError::from_io(src, dst, e))?;
    create_symlink(&target, dst, src)
        .await
        .map_err(|e| CopyError::from_io(src, dst, e))
}

#[cfg(unix)]
async fn create_symlink(target: &Path, link: &Path, _probe: &Path) -> std::io::Result<()> {
    tokio::fs::symlink(target, link).await
}

#[cfg(windows)]
async fn create_symlink(target: &Path, link: &Path, probe: &Path) -> std::io::Result<()> {
    // Probe the *source-side* target to decide file vs. dir symlink.
    let src_target = probe
        .parent()
        .map(|p| p.join(target))
        .unwrap_or_else(|| target.to_path_buf());
    let md = tokio::fs::metadata(&src_target).await;
    let is_dir = matches!(md, Ok(ref m) if m.is_dir());

    let first = if is_dir {
        tokio::fs::symlink_dir(target, link).await
    } else {
        tokio::fs::symlink_file(target, link).await
    };
    match first {
        Ok(()) => return Ok(()),
        // ERROR_PRIVILEGE_NOT_HELD — process lacks SeCreateSymbolicLink
        // privilege *and* Developer Mode isn't on. Fall back to
        // copying the *target* contents into the destination, so the
        // tree still lands usable data even when the symlink can't
        // be recreated. Directories can't be flattened this way —
        // surface the original error for the tree engine's
        // per-file policy to handle.
        Err(e) if e.raw_os_error() == Some(1314) => {
            if is_dir {
                return Err(e);
            }
        }
        Err(e) => return Err(e),
    }
    // Unprivileged fallback: copy the resolved target as a regular
    // file. User ends up with a plain file where the source had a
    // symlink, but no data is lost.
    tokio::fs::copy(&src_target, link).await.map(|_| ())
}

fn rate_bps(bytes: u64, elapsed: Duration) -> u64 {
    let secs = elapsed.as_secs_f64();
    if secs <= 0.0 {
        return 0;
    }
    (bytes as f64 / secs) as u64
}
