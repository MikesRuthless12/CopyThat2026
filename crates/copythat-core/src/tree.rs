//! Tree-level copy and move.
//!
//! `copy_tree` is a two-phase operation:
//!   1. `spawn_blocking` a BFS walk of the source tree to gather the
//!      complete set of directories, files, and symlinks plus their
//!      total byte count. This gives accurate totals for progress.
//!   2. Recreate the directory shape at the destination, then drive
//!      bounded-concurrency per-file copies through `copy_file`.
//!
//! `move_tree` tries an atomic `rename` first; on cross-device failure
//! it falls back to `copy_tree` + bottom-up source deletion.

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
    copy_tree_inner(
        src_dir, dst_dir, opts, ctrl, events, /*is_move_fallback*/ false,
    )
    .await
    .map(|(report, _)| report)
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
    let (report, plan) = copy_tree_inner(
        src_dir,
        dst_dir,
        tree_opts,
        ctrl.clone(),
        events.clone(),
        true,
    )
    .await?;
    if ctrl.is_cancelled() {
        return Err(CopyError::cancelled(src_dir, dst_dir));
    }

    // Bottom-up source cleanup. Delete files first, then directories
    // in reverse-BFS order.
    let mut dirs_to_delete: Vec<PathBuf> = Vec::new();
    for entry in &plan.entries {
        match entry.kind {
            EntryKind::File | EntryKind::Symlink => {
                if let Err(e) = tokio::fs::remove_file(&entry.src).await {
                    // Ignore NotFound — entry may have already been
                    // unlinked (e.g. symlink that replaced a file
                    // during the walk).
                    if e.kind() != std::io::ErrorKind::NotFound {
                        return Err(CopyError::from_io(&entry.src, dst_dir, e));
                    }
                }
            }
            EntryKind::Dir => dirs_to_delete.push(entry.src.clone()),
        }
    }
    // Deepest first.
    dirs_to_delete.sort_by_key(|p| std::cmp::Reverse(p.components().count()));
    for dir in dirs_to_delete {
        if let Err(e) = tokio::fs::remove_dir(&dir).await
            && e.kind() != std::io::ErrorKind::NotFound
            && e.kind() != std::io::ErrorKind::DirectoryNotEmpty
        {
            return Err(CopyError::from_io(&dir, dst_dir, e));
        }
    }

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
    _is_move_fallback: bool,
) -> Result<(TreeReport, Plan), CopyError> {
    let src_root = src_dir.to_path_buf();
    let dst_root = dst_dir.to_path_buf();

    // Validate + enumerate.
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

    let plan = enumerate(src_root.clone(), opts.follow_symlinks_in_tree)
        .await
        .map_err(|e| CopyError::from_io(&src_root, &dst_root, e))?;

    let _ = events
        .send(CopyEvent::TreeStarted {
            root_src: src_root.clone(),
            root_dst: dst_root.clone(),
            total_files: plan.total_files,
            total_bytes: plan.total_bytes,
        })
        .await;

    // Ensure destination root exists.
    if let Err(e) = tokio::fs::create_dir_all(&dst_root).await {
        let err = CopyError::from_io(&src_root, &dst_root, e);
        let _ = events.send(CopyEvent::Failed { err: err.clone() }).await;
        return Err(err);
    }

    // Recreate directory skeleton first so per-file copies find their
    // parent ready. Ordering: shallowest-to-deepest (which is what
    // walkdir gives us for directories).
    for entry in plan.entries.iter().filter(|e| e.kind == EntryKind::Dir) {
        let dst_path = dst_root.join(&entry.rel);
        if let Err(e) = tokio::fs::create_dir_all(&dst_path).await {
            let err = CopyError::from_io(&entry.src, &dst_path, e);
            let _ = events.send(CopyEvent::Failed { err: err.clone() }).await;
            return Err(err);
        }
    }

    let started = Instant::now();
    let bytes_done = Arc::new(AtomicU64::new(0));
    let files_done = Arc::new(AtomicU64::new(0));
    let skipped = Arc::new(AtomicU64::new(0));
    let errored = Arc::new(AtomicU64::new(0));

    let on_error = opts.clamped_on_error();

    let semaphore = Arc::new(Semaphore::new(opts.clamped_concurrency()));
    let mut set: JoinSet<Result<FileOutcome, CopyError>> = JoinSet::new();

    // Prepare the file + symlink entries; dirs are already created.
    let file_entries: Vec<Entry> = plan
        .entries
        .iter()
        .filter(|e| e.kind != EntryKind::Dir)
        .cloned()
        .collect();

    for entry in file_entries {
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
        let on_error_task = on_error;
        let total_files = plan.total_files;
        let total_bytes = plan.total_bytes;
        let entry = entry.clone();

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
                    EntryKind::Symlink => match copy_symlink_entry(&entry.src, &dst_final).await {
                        Ok(()) => Ok(FileOutcome::Done(0)),
                        Err(err) => {
                            handle_per_file_error(err, on_error_task, &events_task, &errored_task)
                                .await
                        }
                    },
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
                let done_bytes = bytes_done_task.fetch_add(*bytes, Ordering::Relaxed) + *bytes;
                let done_files = files_done_task.fetch_add(1, Ordering::Relaxed) + 1;
                let elapsed = started.elapsed();
                let rate = rate_bps(done_bytes, elapsed);
                let _ = events_task
                    .send(CopyEvent::TreeProgress {
                        files_done: done_files,
                        files_total: total_files,
                        bytes_done: done_bytes,
                        bytes_total: total_bytes,
                        rate_bps: rate,
                    })
                    .await;
            }

            drop(permit);
            Ok(outcome)
        });
    }

    let mut aborted = false;
    let mut first_error: Option<CopyError> = None;
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
                // Cancel remaining tasks — one file's failure stops
                // the tree.
                ctrl.cancel();
            }
            Err(_join_err) => {
                // Task panicked. Report.
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

    if let Some(err) = first_error {
        let _ = events.send(CopyEvent::Failed { err: err.clone() }).await;
        return Err(err);
    }
    if aborted || ctrl.is_cancelled() {
        let err = CopyError::cancelled(&src_root, &dst_root);
        let _ = events.send(CopyEvent::Failed { err: err.clone() }).await;
        return Err(err);
    }

    // Directory times last: walkdir gives us directories shallow-first,
    // but times should be applied deepest-first so we don't write a
    // file into a directory after its mtime has been set.
    if opts.preserve_directory_times {
        let mut dirs: Vec<&Entry> = plan
            .entries
            .iter()
            .filter(|e| e.kind == EntryKind::Dir)
            .collect();
        dirs.sort_by_key(|e| std::cmp::Reverse(e.rel.components().count()));
        for dir in dirs {
            let src_md = match std::fs::metadata(&dir.src) {
                Ok(m) => m,
                Err(_) => continue,
            };
            let dst_path = dst_root.join(&dir.rel);
            let atime = FileTime::from_last_access_time(&src_md);
            let mtime = FileTime::from_last_modification_time(&src_md);
            let dst_clone = dst_path.clone();
            let _ = tokio::task::spawn_blocking(move || {
                filetime::set_file_times(&dst_clone, atime, mtime)
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
    Ok((report, plan))
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

async fn enumerate(root: PathBuf, follow_symlinks: bool) -> std::io::Result<Plan> {
    tokio::task::spawn_blocking(move || enumerate_sync(&root, follow_symlinks))
        .await
        .map_err(|e| std::io::Error::other(format!("walk task panicked: {e}")))?
}

fn enumerate_sync(root: &Path, follow_symlinks: bool) -> std::io::Result<Plan> {
    let mut plan = Plan::default();
    let walker = walkdir::WalkDir::new(root)
        .follow_links(follow_symlinks)
        .sort_by_file_name();
    for entry in walker {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                return Err(std::io::Error::other(format!(
                    "walk error at {:?}: {e}",
                    e.path()
                )));
            }
        };
        let path = entry.path();
        let rel = path.strip_prefix(root).unwrap_or(path).to_path_buf();
        if rel.as_os_str().is_empty() {
            // The root itself — tracked as the root dir at the
            // destination via create_dir_all earlier; no entry needed.
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
            plan.total_files += 1;
            plan.total_bytes += len;
        }
        plan.entries.push(Entry {
            src: path.to_path_buf(),
            rel,
            kind,
        });
    }
    Ok(plan)
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
    match md {
        Ok(m) if m.is_dir() => tokio::fs::symlink_dir(target, link).await,
        _ => tokio::fs::symlink_file(target, link).await,
    }
}

fn rate_bps(bytes: u64, elapsed: Duration) -> u64 {
    let secs = elapsed.as_secs_f64();
    if secs <= 0.0 {
        return 0;
    }
    (bytes as f64 / secs) as u64
}
