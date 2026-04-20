//! Job runner — drives the copy / move engine for a single queued
//! job, forwards per-file + tree progress into the Tauri event bus,
//! and maintains the queue's lifecycle state so the UI can read it
//! back via `list_jobs`.
//!
//! One task per job, spawned by the invoking Tauri command. The task
//! owns the destination `mpsc::Sender<CopyEvent>` that the engine
//! emits on; a sibling task drains the receiver and forwards.
//!
//! Progress events are throttled by the engine itself (16 KiB / 50 ms,
//! per `PROGRESS_MIN_*` in `copythat-core::engine`). Each progress
//! tick here also synthesises a `globals-tick` event so the header
//! strip and footer can bind to aggregate counters without holding
//! their own state machine.

use std::path::PathBuf;
use std::sync::atomic::Ordering;

use copythat_core::{
    CollisionPolicy, CopyControl, CopyError, CopyErrorKind, CopyEvent, CopyOptions, ErrorPolicy,
    Job, JobId, JobKind, JobState, MoveOptions, Queue, TreeOptions, copy_file, copy_tree,
    move_file, move_tree,
};
use tauri::{AppHandle, Emitter};
use tokio::sync::mpsc;

use crate::ipc::{
    CollisionPromptDto, EVENT_COLLISION_RAISED, EVENT_ERROR_RAISED, EVENT_GLOBALS_TICK,
    EVENT_JOB_ADDED, EVENT_JOB_CANCELLED, EVENT_JOB_COMPLETED, EVENT_JOB_FAILED, EVENT_JOB_PAUSED,
    EVENT_JOB_PROGRESS, EVENT_JOB_RESUMED, EVENT_JOB_STARTED, ErrorPromptDto, GlobalsDto, JobDto,
    JobFailedDto, JobIdDto, JobProgressDto,
};
use crate::state::AppState;

/// How big is the per-job mpsc channel between engine and runner.
/// 512 is comfortable headroom for the 50-ms throttle in the copy
/// engine; a slow event drain won't back-pressure the copy loop into
/// stalling I/O.
const EVENT_CHANNEL: usize = 512;

/// Arguments for a single runner task. Grouped into a struct so
/// the command layer can hand the runner everything it needs
/// without tripping clippy's `too_many_arguments`, and so future
/// optional knobs (`CollisionPolicy`, `TreeOptions.concurrency`)
/// can slot in without reshaping every call site.
pub(crate) struct RunJob {
    pub app: AppHandle,
    pub state: AppState,
    pub id: JobId,
    pub kind: JobKind,
    pub src: PathBuf,
    pub dst: Option<PathBuf>,
    pub ctrl: CopyControl,
    pub verifier: Option<copythat_core::Verifier>,
    pub copy_opts: CopyOptions,
    /// Phase 8 — tree-level collision policy. Defaults to `Skip`
    /// at the engine level; the command layer upgrades to `Prompt`
    /// when the user leaves the setting on "Ask".
    pub collision_policy: CollisionPolicy,
    /// Phase 8 — how to respond to per-file failures inside a tree
    /// copy. Defaults to `Abort` (pre-8 behaviour); the command
    /// layer flips to `Ask` so the frontend can surface prompts.
    pub error_policy: ErrorPolicy,
}

/// Entry point for a newly-enqueued job. Caller has already added
/// the job to the queue and emitted `job-added`; this task drives
/// the engine and cleans up.
pub(crate) async fn run_job(job: RunJob) {
    let RunJob {
        app,
        state,
        id,
        kind,
        src,
        dst,
        ctrl,
        verifier,
        copy_opts,
        collision_policy,
        error_policy,
    } = job;
    state.queue.start(id);
    let _ = app.emit(EVENT_JOB_STARTED, JobIdDto { id: id.as_u64() });
    emit_globals(&app, &state);

    let (tx, rx) = mpsc::channel::<CopyEvent>(EVENT_CHANNEL);

    let app_for_events = app.clone();
    let state_for_events = state.clone();
    let forwarder = tokio::spawn(forward_events(app_for_events, state_for_events, id, rx));

    let mut copy_opts_with_verify = copy_opts;
    copy_opts_with_verify.verify = verifier;

    let result: Result<(), CopyError> = match kind {
        JobKind::Copy => {
            let Some(dst_path) = dst.clone() else {
                finish_fail(
                    &app,
                    &state,
                    id,
                    CopyError {
                        kind: CopyErrorKind::IoOther,
                        src: src.clone(),
                        dst: PathBuf::new(),
                        raw_os_error: None,
                        message: "copy job has no destination".to_string(),
                    },
                )
                .await;
                return;
            };
            let source_is_dir = tokio::fs::metadata(&src)
                .await
                .map(|m| m.is_dir())
                .unwrap_or(false);
            if source_is_dir {
                let tree_opts = TreeOptions {
                    file: copy_opts_with_verify,
                    collision: collision_policy,
                    on_error: error_policy,
                    ..TreeOptions::default()
                };
                copy_tree(&src, &dst_path, tree_opts, ctrl, tx.clone())
                    .await
                    .map(|_| ())
            } else {
                copy_file(&src, &dst_path, copy_opts_with_verify, ctrl, tx.clone())
                    .await
                    .map(|_| ())
            }
        }
        JobKind::Move => {
            let Some(dst_path) = dst.clone() else {
                finish_fail(
                    &app,
                    &state,
                    id,
                    CopyError {
                        kind: CopyErrorKind::IoOther,
                        src: src.clone(),
                        dst: PathBuf::new(),
                        raw_os_error: None,
                        message: "move job has no destination".to_string(),
                    },
                )
                .await;
                return;
            };
            let source_is_dir = tokio::fs::metadata(&src)
                .await
                .map(|m| m.is_dir())
                .unwrap_or(false);
            let move_opts = MoveOptions {
                copy: copy_opts_with_verify,
                ..MoveOptions::default()
            };
            if source_is_dir {
                move_tree(&src, &dst_path, move_opts, ctrl, tx.clone())
                    .await
                    .map(|_| ())
            } else {
                move_file(&src, &dst_path, move_opts, ctrl, tx.clone())
                    .await
                    .map(|_| ())
            }
        }
        JobKind::Delete | JobKind::SecureDelete | JobKind::Verify => {
            // Phase 5 wires copy + move only. The enum carries the
            // rest so the queue is forward-compatible, but enqueueing
            // these today fails cleanly so the UI doesn't pretend to
            // have started work it isn't doing.
            Err(CopyError {
                kind: CopyErrorKind::IoOther,
                src: src.clone(),
                dst: dst.clone().unwrap_or_default(),
                raw_os_error: None,
                message: format!("job kind `{kind:?}` is not wired into the Phase 5 runner yet"),
            })
        }
    };

    drop(tx);
    let _ = forwarder.await;

    match result {
        Ok(()) => {
            // Check cancel: a cancelled job may return Ok from
            // copy_file if the cancel raced the final write.
            if state
                .queue
                .get(id)
                .map(|j| j.state == JobState::Cancelled)
                .unwrap_or(false)
            {
                let _ = app.emit(EVENT_JOB_CANCELLED, JobIdDto { id: id.as_u64() });
            } else {
                state.queue.mark_completed(id);
                let _ = app.emit(EVENT_JOB_COMPLETED, JobIdDto { id: id.as_u64() });
            }
        }
        Err(err) => {
            if err.is_cancelled() {
                // Runner already flipped state to Cancelled via
                // `Queue::cancel_job`; just emit the UI-facing event.
                let _ = app.emit(EVENT_JOB_CANCELLED, JobIdDto { id: id.as_u64() });
            } else {
                state.queue.mark_failed(id, err.clone());
                let _ = app.emit(
                    EVENT_JOB_FAILED,
                    JobFailedDto {
                        id: id.as_u64(),
                        message: err.message.clone(),
                    },
                );
            }
        }
    }
    emit_globals(&app, &state);
}

/// Re-emit a terminal failure with a synthesised error. Keeps the
/// "no destination" edge case one line at the call site.
async fn finish_fail(app: &AppHandle, state: &AppState, id: JobId, err: CopyError) {
    state.queue.mark_failed(id, err.clone());
    let _ = app.emit(
        EVENT_JOB_FAILED,
        JobFailedDto {
            id: id.as_u64(),
            message: err.message.clone(),
        },
    );
    emit_globals(app, state);
}

/// Drain the engine's event channel and forward into Tauri events.
///
/// Each incoming `CopyEvent` updates the queue's live counters,
/// emits a typed IPC event, and — on every progress tick — fires
/// a globals summary so header + footer stay in sync.
async fn forward_events(
    app: AppHandle,
    state: AppState,
    id: JobId,
    mut rx: mpsc::Receiver<CopyEvent>,
) {
    let mut last_files_total: u64 = 0;
    let mut last_bytes_total: u64 = 0;

    while let Some(evt) = rx.recv().await {
        match evt {
            CopyEvent::Started { total_bytes, .. } => {
                last_bytes_total = total_bytes;
                last_files_total = 1;
                state.queue.set_progress(id, 0, total_bytes, 0, 1);
            }
            CopyEvent::TreeStarted {
                total_files,
                total_bytes,
                ..
            } => {
                last_files_total = total_files;
                last_bytes_total = total_bytes;
                state.queue.set_progress(id, 0, total_bytes, 0, total_files);
            }
            CopyEvent::Progress {
                bytes,
                total,
                rate_bps,
            } => {
                state.queue.set_progress(id, bytes, total, 0, 1);
                emit_progress(&app, &state, id, bytes, total, 0, 1, rate_bps);
            }
            CopyEvent::TreeProgress {
                bytes_done,
                bytes_total,
                files_done,
                files_total,
                rate_bps,
            } => {
                state
                    .queue
                    .set_progress(id, bytes_done, bytes_total, files_done, files_total);
                emit_progress(
                    &app,
                    &state,
                    id,
                    bytes_done,
                    bytes_total,
                    files_done,
                    files_total,
                    rate_bps,
                );
            }
            CopyEvent::Paused => {
                let _ = app.emit(EVENT_JOB_PAUSED, JobIdDto { id: id.as_u64() });
                emit_globals(&app, &state);
            }
            CopyEvent::Resumed => {
                let _ = app.emit(EVENT_JOB_RESUMED, JobIdDto { id: id.as_u64() });
                emit_globals(&app, &state);
            }
            CopyEvent::VerifyStarted { total_bytes, .. } => {
                // Treat verify as a second pass: reset the denominator
                // so the ring restarts at 0%. Progress is continuous
                // after that.
                state
                    .queue
                    .set_progress(id, 0, total_bytes, last_files_total, last_files_total);
            }
            CopyEvent::VerifyProgress {
                bytes,
                total,
                rate_bps,
            } => {
                state
                    .queue
                    .set_progress(id, bytes, total, last_files_total, last_files_total);
                emit_progress(
                    &app,
                    &state,
                    id,
                    bytes,
                    total,
                    last_files_total,
                    last_files_total,
                    rate_bps,
                );
            }
            CopyEvent::VerifyCompleted { .. } | CopyEvent::VerifyFailed { .. } => {
                // Engine will return the final Result — no state
                // transition here.
            }
            CopyEvent::Completed { bytes, .. } => {
                state.queue.set_progress(
                    id,
                    bytes,
                    last_bytes_total.max(bytes),
                    last_files_total.max(1),
                    last_files_total.max(1),
                );
            }
            CopyEvent::TreeCompleted { bytes, files, .. } => {
                state.queue.set_progress(id, bytes, bytes, files, files);
            }
            CopyEvent::Failed { .. } => {
                // Terminal per-file / per-tree failure — handled after
                // the engine returns from the top-level copy_* call.
            }
            CopyEvent::FileError { err } => {
                // Per-file failure absorbed by the engine's error
                // policy (Skip / exhausted RetryN). Log it and keep
                // going; the tree continues.
                state.errors.log_auto(id.as_u64(), &err);
            }
            CopyEvent::Collision(mut coll) => {
                let job_id = id.as_u64();
                // Honour a prior "Apply to all" decision without
                // bothering the user again.
                if let Some(cached) = state.collisions.cached_resolution(job_id) {
                    coll.resolve(cached);
                    continue;
                }
                // Extract the oneshot before we reach for the paths
                // (taking moves the Sender out of the Option).
                let Some(tx) = coll.resolver.take() else {
                    // Cloned placeholder from a broadcast subscriber —
                    // nothing to drive here.
                    continue;
                };
                let src_path = coll.src.clone();
                let dst_path = coll.dst.clone();
                let (src_size, src_modified_ms) = peek_meta(&src_path).await;
                let (dst_size, dst_modified_ms) = peek_meta(&dst_path).await;
                let new_id =
                    state
                        .collisions
                        .register(job_id, src_path.clone(), dst_path.clone(), tx);
                let _ = app.emit(
                    EVENT_COLLISION_RAISED,
                    CollisionPromptDto {
                        id: new_id,
                        job_id,
                        src: src_path.to_string_lossy().into_owned(),
                        dst: dst_path.to_string_lossy().into_owned(),
                        src_size,
                        src_modified_ms,
                        dst_size,
                        dst_modified_ms,
                    },
                );
            }
            CopyEvent::ErrorPrompt(mut prompt) => {
                let job_id = id.as_u64();
                // Honour an earlier "Skip all of this kind" decision.
                if let Some(cached) = state.errors.cached_action_for(job_id, &prompt.err) {
                    prompt.resolve(cached);
                    continue;
                }
                let Some(tx) = prompt.resolver.take() else {
                    continue;
                };
                let err = prompt.err.clone();
                let new_id = state.errors.register(job_id, err.clone(), tx);
                let _ = app.emit(
                    EVENT_ERROR_RAISED,
                    ErrorPromptDto {
                        id: new_id,
                        job_id,
                        src: err.src.to_string_lossy().into_owned(),
                        dst: err.dst.to_string_lossy().into_owned(),
                        kind: crate::errors::kind_name(err.kind),
                        localized_key: err.localized_key(),
                        message: err.message.clone(),
                        raw_os_error: err.raw_os_error,
                        created_at_ms: now_ms(),
                    },
                );
            }
            // `CopyEvent` is `#[non_exhaustive]`; any future variant
            // is forwarded as a no-op so the runner never panics on
            // an unknown lifecycle event.
            _ => {}
        }
    }
}

/// Emit one progress event + globals tick. Rate from the engine is
/// the live instantaneous figure; ETA is derived from bytes-remaining
/// at that rate. When rate is 0 we return `None` — the frontend
/// paints "calculating…" rather than a bogus infinity.
#[allow(clippy::too_many_arguments)]
fn emit_progress(
    app: &AppHandle,
    state: &AppState,
    id: JobId,
    bytes_done: u64,
    bytes_total: u64,
    files_done: u64,
    files_total: u64,
    rate_bps: u64,
) {
    let eta = eta_seconds(bytes_done, bytes_total, rate_bps);
    let _ = app.emit(
        EVENT_JOB_PROGRESS,
        JobProgressDto {
            id: id.as_u64(),
            bytes_done,
            bytes_total,
            files_done,
            files_total,
            rate_bps,
            eta_seconds: eta,
        },
    );
    emit_globals(app, state);
}

fn eta_seconds(bytes_done: u64, bytes_total: u64, rate_bps: u64) -> Option<u64> {
    if rate_bps == 0 || bytes_total == 0 || bytes_done >= bytes_total {
        return None;
    }
    let remaining = bytes_total - bytes_done;
    Some(remaining / rate_bps)
}

/// Walk the queue snapshot and derive a `GlobalsDto`. Cheap —
/// snapshot is O(n) in job count, which is bounded in practice.
pub fn build_globals(queue: &Queue) -> GlobalsDto {
    let jobs = queue.snapshot();
    let mut active = 0u64;
    let mut queued = 0u64;
    let mut paused = 0u64;
    let mut failed = 0u64;
    let mut succeeded = 0u64;
    let mut bytes_done = 0u64;
    let mut bytes_total = 0u64;
    let mut errors = 0u64;

    for job in &jobs {
        match job.state {
            JobState::Running => active += 1,
            JobState::Pending => queued += 1,
            JobState::Paused => paused += 1,
            JobState::Failed => {
                failed += 1;
                errors += 1;
            }
            JobState::Succeeded => succeeded += 1,
            JobState::Cancelled => {}
        }
        // Only count live jobs in running totals so a completed job
        // doesn't double its bytes into the "current batch" bar.
        if matches!(
            job.state,
            JobState::Running | JobState::Paused | JobState::Pending
        ) {
            bytes_done += job.bytes_done;
            bytes_total += job.bytes_total;
        }
    }

    let state = overall_state(active, paused, failed, &jobs);
    GlobalsDto {
        state,
        active_jobs: active,
        queued_jobs: queued,
        paused_jobs: paused,
        failed_jobs: failed,
        succeeded_jobs: succeeded,
        bytes_done,
        bytes_total,
        // Rate is carried per-job in `job-progress`; the frontend
        // sums live rates across running jobs for the global figure.
        // The Rust side emits 0 here and the frontend replaces it.
        rate_bps: 0,
        eta_seconds: None,
        errors,
    }
}

fn overall_state(active: u64, paused: u64, failed: u64, jobs: &[Job]) -> &'static str {
    if active > 0 {
        "copying"
    } else if paused > 0 {
        "paused"
    } else if failed > 0 {
        "error"
    } else if jobs.iter().any(|j| j.state == JobState::Pending) {
        "copying"
    } else {
        "idle"
    }
}

fn emit_globals(app: &AppHandle, state: &AppState) {
    let g = build_globals(&state.queue);
    state.globals.fetch_add(1, Ordering::Relaxed);
    let _ = app.emit(EVENT_GLOBALS_TICK, g);
}

/// Helper for the "job added" emit — the command layer calls this
/// after `queue.add` so the UI learns about the job before the
/// runner task even starts.
pub fn emit_job_added(app: &AppHandle, dto: JobDto) {
    let _ = app.emit(EVENT_JOB_ADDED, dto);
}

/// Read size + mtime for a filesystem path, returning `(None, None)`
/// if the path is missing or the call fails. Used by the collision
/// prompt to prime the modal's source / destination preview panes.
async fn peek_meta(path: &std::path::Path) -> (Option<u64>, Option<u64>) {
    let Ok(md) = tokio::fs::metadata(path).await else {
        return (None, None);
    };
    let size = Some(md.len());
    let mtime = md
        .modified()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_millis() as u64);
    (size, mtime)
}

fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}
