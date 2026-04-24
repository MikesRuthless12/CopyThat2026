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
    FilterSet, Job, JobId, JobKind, JobState, MoveOptions, Queue, TreeOptions, copy_file,
    copy_tree, move_file, move_tree,
};
use copythat_history::{ItemRow, JobRowId, JobSummary};
use tauri::{AppHandle, Emitter};
use tokio::sync::mpsc;

use crate::ipc::{
    CollisionPromptDto, EVENT_COLLISION_RAISED, EVENT_ERROR_RAISED, EVENT_GLOBALS_TICK,
    EVENT_JOB_ADDED, EVENT_JOB_CANCELLED, EVENT_JOB_COMPLETED, EVENT_JOB_FAILED, EVENT_JOB_PAUSED,
    EVENT_JOB_PROGRESS, EVENT_JOB_RESUMED, EVENT_JOB_STARTED, EVENT_META_TRANSLATED_TO_APPLEDOUBLE,
    EVENT_SNAPSHOT_CREATED, EVENT_SPARSENESS_NOT_SUPPORTED, ErrorPromptDto, GlobalsDto, JobDto,
    JobFailedDto, JobIdDto, JobProgressDto, MetaTranslatedToAppleDoubleDto, SnapshotCreatedDto,
    SparsenessNotSupportedDto,
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
    /// Phase 13c — maximum concurrent file copies inside a tree.
    /// Resolved at enqueue time from `Settings.transfer.concurrency`
    /// plus `copythat_platform::recommend_concurrency` for the
    /// `Auto` case. `None` falls back to the engine's built-in
    /// default (4) so tests that construct `RunJob` by hand still work.
    pub tree_concurrency: Option<usize>,
    /// Phase 14a — enumeration-time filters built from
    /// `Settings.filters`. `None` or an empty `FilterSet` leaves the
    /// pre-14a behaviour unchanged (every discovered entry enters
    /// the plan).
    pub filters: Option<FilterSet>,
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
        tree_concurrency,
        filters,
    } = job;
    eprintln!(
        "[run_job] id={} kind={:?} src={} dst={:?}",
        id.as_u64(),
        kind,
        src.display(),
        dst.as_ref().map(|p| p.display().to_string())
    );
    state.queue.start(id);
    let _ = app.emit(EVENT_JOB_STARTED, JobIdDto { id: id.as_u64() });
    emit_globals(&app, &state);

    // Phase 34 — fire a `JobStarted` audit record on the active
    // sink. No-op when audit is disabled.
    let job_started_at = std::time::Instant::now();
    let audit_job_id = format!("job-{}", id.as_u64());
    let kind_wire_audit = match kind {
        JobKind::Copy => "copy",
        JobKind::Move => "move",
        JobKind::Delete => "delete",
        JobKind::SecureDelete => "secure-delete",
        JobKind::Verify => "verify",
    };
    crate::audit_commands::record_job_started(
        &state.audit,
        &audit_job_id,
        kind_wire_audit,
        &src,
        dst.as_deref(),
    );

    // Phase 9 — record the job-start into the SQLite history. `None`
    // return means history is disabled (in-memory AppState or disk
    // open failed at boot); the runner still works, just without
    // persistence.
    let history_row = record_history_start(&state, kind, &src, dst.as_deref(), &verifier).await;

    let (tx, rx) = mpsc::channel::<CopyEvent>(EVENT_CHANNEL);

    let app_for_events = app.clone();
    let state_for_events = state.clone();
    let forwarder = tokio::spawn(forward_events(
        app_for_events,
        state_for_events,
        id,
        rx,
        history_row,
    ));

    let mut copy_opts_with_verify = copy_opts;
    copy_opts_with_verify.verify = verifier;

    // Phase 20 — wire a per-job journal sink onto the CopyOptions
    // so the engine checkpoints into the redb-backed journal every
    // 50 ms. The runner allocates the JobRowId once per queued job;
    // single-file copies use file_idx=0, tree copies advance through
    // a shared atomic so each per-file copy_file invocation gets a
    // monotonic index. Failure to begin the journal entry is non-
    // fatal — the copy still runs, just without resume on the next
    // launch.
    let journal_row = state.journal.as_ref().and_then(|j| {
        let kind_wire = match kind {
            JobKind::Copy => "copy",
            JobKind::Move => "move",
            JobKind::Delete => "delete",
            JobKind::SecureDelete => "secure-delete",
            JobKind::Verify => "verify",
        };
        let rec = copythat_journal::JobRecord::new(kind_wire, src.clone(), dst.clone());
        match j.begin_job(rec) {
            Ok(row) => Some(row),
            Err(e) => {
                eprintln!("[run_job] journal begin_job failed: {e}");
                None
            }
        }
    });
    if let (Some(j), Some(row)) = (state.journal.as_ref(), journal_row) {
        copy_opts_with_verify.journal = Some(std::sync::Arc::new(
            copythat_journal::CopyThatJournalSink::new(j.clone(), row),
        ));
    }
    // Clone the trait-object handle so we can call the per-job
    // terminator (`finish_job_*`) after the engine returns; the
    // engine itself only ever calls per-file methods.
    let journal_sink_for_terminator = copy_opts_with_verify.journal.clone();

    // Phase 21 — attach the AppState's shared bandwidth-shaping
    // bucket. Always wired (even when the rate is unlimited) so a
    // runtime `set_rate` call from the Settings IPC handler takes
    // effect on every in-flight job without re-enqueuing. The sink
    // is cheap to clone and short-circuits when the shape's current
    // rate is `None`.
    copy_opts_with_verify.shape = Some(std::sync::Arc::new(
        copythat_shape::CopyThatShapeSink::new(state.shape.clone()),
    ));

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
            eprintln!(
                "[run_job] source_is_dir={} — about to call {}",
                source_is_dir,
                if source_is_dir {
                    "copy_tree"
                } else {
                    "copy_file"
                }
            );
            if source_is_dir {
                let tree_opts = TreeOptions {
                    file: copy_opts_with_verify,
                    collision: collision_policy,
                    on_error: error_policy,
                    concurrency: tree_concurrency.unwrap_or(TreeOptions::default().concurrency),
                    filters: filters.clone(),
                    ..TreeOptions::default()
                };
                let out = copy_tree(&src, &dst_path, tree_opts, ctrl, tx.clone())
                    .await
                    .map(|_| ());
                eprintln!(
                    "[run_job] copy_tree returned: {:?}",
                    out.as_ref().map(|_| "ok").map_err(|e| &e.message)
                );
                out
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

    // Terminal status for both the queue-level emit and the
    // history-level finish stamp.
    let terminal_status: &'static str = match &result {
        Ok(()) => {
            if state
                .queue
                .get(id)
                .map(|j| j.state == JobState::Cancelled)
                .unwrap_or(false)
            {
                "cancelled"
            } else {
                "succeeded"
            }
        }
        Err(err) if err.is_cancelled() => "cancelled",
        Err(_) => "failed",
    };

    // Phase 20 — terminal journal call. The engine's per-file
    // `finish_file` already captured each file's BLAKE3; this
    // promotes the job-level row to a terminal status so it stops
    // showing up in `unfinished()` on the next launch.
    if let Some(sink) = journal_sink_for_terminator.as_ref() {
        match terminal_status {
            "succeeded" => sink.finish_job_succeeded(),
            "cancelled" => sink.finish_job_cancelled(),
            _ => sink.finish_job_failed(),
        }
    }

    match result {
        Ok(()) => {
            // Check cancel: a cancelled job may return Ok from
            // copy_file if the cancel raced the final write.
            if terminal_status == "cancelled" {
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
                // Single-file / early-abort failure path: the
                // engine didn't loop through forward_events long
                // enough to emit a per-file FileError or Completed,
                // so log the failure as a one-row item now. (Tree
                // paths already self-emit per-file events above.)
                if let Some(row) = history_row
                    && !err.src.as_os_str().is_empty()
                {
                    let history = state.history.clone();
                    if let Some(h) = history {
                        let item = ItemRow {
                            job_row_id: row.as_i64(),
                            src: err.src.clone(),
                            dst: err.dst.clone(),
                            size: 0,
                            status: "failed".into(),
                            hash_hex: None,
                            error_code: Some(crate::errors::kind_name(err.kind).to_string()),
                            error_msg: Some(err.message.clone()),
                            timestamp_ms: now_ms() as i64,
                        };
                        let _ = h.record_item(&item).await;
                    }
                }
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

    // Phase 22 — drop per-job collision state (apply-all cache +
    // accumulated rules) so a later job with the same id (not
    // possible today but defensive) doesn't inherit stale choices.
    state.collisions.clear_job(id.as_u64());

    // Phase 9 — stamp the terminal status + final totals into the
    // history row.
    let snapshot_for_totals = state.queue.get(id);
    let total_bytes = snapshot_for_totals
        .as_ref()
        .map(|j| j.bytes_done)
        .unwrap_or(0);
    let files_ok = snapshot_for_totals
        .as_ref()
        .map(|j| j.files_done)
        .unwrap_or(0);
    let files_failed = snapshot_for_totals
        .as_ref()
        .and_then(|j| j.files_total.checked_sub(j.files_done))
        .unwrap_or(0);
    if let Some(row) = history_row
        && let Some(history) = &state.history
    {
        let _ = history
            .record_finish(row, terminal_status, total_bytes, files_ok, files_failed)
            .await;
    }

    // Phase 34 — fire `JobCompleted` into the audit sink with the
    // terminal status + the same totals history records. The ms
    // duration is measured from the start of `run_job` so it
    // reflects wall-clock runtime including queue-to-start latency.
    crate::audit_commands::record_job_completed(
        &state.audit,
        &audit_job_id,
        terminal_status,
        files_ok,
        files_failed,
        total_bytes,
        job_started_at.elapsed().as_millis() as u64,
    );

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
    history_row: Option<JobRowId>,
) {
    let mut last_files_total: u64 = 0;
    let mut last_bytes_total: u64 = 0;
    // Phase 16 — once the engine fires `TreeStarted`, per-file
    // `Started` events inside that tree must NOT reset the queue's
    // files-total back to 1 (which was the old single-file fallback
    // behaviour). Track whether we're inside a tree so the Started
    // branch knows to leave `last_files_total` / `last_bytes_total`
    // alone.
    let mut in_tree_mode = false;
    // Track how many files have actually started inside the tree so
    // we can show 1/103 → 2/103 → … as the engine progresses. The
    // TreeProgress event pushes the authoritative `files_done`
    // later; this is just so Started-before-first-TreeProgress
    // doesn't regress the counter.
    let mut tree_files_started: u64 = 0;
    // Phase 9 — per-file history bookkeeping. `Started` sets the
    // in-flight item; `VerifyCompleted` stashes its hash;
    // `Completed` flushes it as a successful row. `FileError`
    // records a failure row directly.
    let mut current_item: Option<ItemInFlight> = None;

    // Phase 13d — per-file UI activity feed. Monotonic `seq` gives
    // the frontend a stable ordering key so out-of-order emissions
    // (unlikely but allowed) still render consistently. `last_emit`
    // rate-limits Progress ticks so a many-small-files tree doesn't
    // spam the event bus.
    let job_id_u64 = id.as_u64();
    let mut activity_seq: u64 = 0;
    let mut last_activity_src: std::path::PathBuf = std::path::PathBuf::new();
    let mut last_progress_emit = std::time::Instant::now();
    let progress_min_interval = std::time::Duration::from_millis(120);

    while let Some(evt) = rx.recv().await {
        match evt {
            CopyEvent::Started {
                src,
                dst,
                total_bytes,
            } => {
                if !in_tree_mode {
                    // Real single-file job — Started is the only size
                    // signal we'll get.
                    last_bytes_total = total_bytes;
                    last_files_total = 1;
                    state.queue.set_progress(id, 0, total_bytes, 0, 1);
                } else {
                    // Tree job: TreeProgress is the authoritative
                    // source for the aggregate counters, so we
                    // touch only per-file bookkeeping here. A
                    // write to the queue on every Started was
                    // clobbering `bytes_done` / `files_done` back
                    // to 0, which produced the oscillating top
                    // counter the user reported.
                    tree_files_started = tree_files_started.saturating_add(1);
                }
                // Per-file activity ping: "a file just started". Fires
                // for both single-file jobs and per-file starts inside
                // a tree (the engine emits Started for each file).
                activity_seq += 1;
                let _ = app.emit(
                    crate::ipc::EVENT_FILE_ACTIVITY,
                    crate::ipc::FileActivityDto {
                        job_id: job_id_u64,
                        seq: activity_seq,
                        phase: "start",
                        src: src.to_string_lossy().into_owned(),
                        dst: dst.to_string_lossy().into_owned(),
                        bytes_done: 0,
                        bytes_total: total_bytes,
                        is_dir: false,
                        message: None,
                    },
                );
                last_activity_src = src.clone();
                last_progress_emit = std::time::Instant::now();
                if history_row.is_some() {
                    let _ = total_bytes; // size is taken from the Completed event
                    current_item = Some(ItemInFlight {
                        src,
                        dst,
                        hash_hex: None,
                    });
                }
            }
            CopyEvent::TreeEnumerating {
                files_so_far,
                bytes_so_far,
            } => {
                // Streaming walk is in progress AND copies are
                // already running in parallel. Only update the
                // denominator (files_total, bytes_total) — never
                // touch the numerator, otherwise every walker tick
                // stomps the `files_done / bytes_done` the copiers
                // have already advanced. That bug showed up as a
                // top counter stuck on `0 B / 852 GiB` while the
                // Activity list showed dozens of completed rows.
                in_tree_mode = true;
                last_files_total = files_so_far;
                last_bytes_total = bytes_so_far;
                let (cur_bytes, cur_files) = state
                    .queue
                    .get(id)
                    .map(|j| (j.bytes_done, j.files_done))
                    .unwrap_or((0, 0));
                state
                    .queue
                    .set_progress(id, cur_bytes, bytes_so_far, cur_files, files_so_far);
                emit_progress(
                    &app,
                    &state,
                    id,
                    cur_bytes,
                    bytes_so_far,
                    cur_files,
                    files_so_far,
                    0,
                );
            }
            CopyEvent::TreeStarted {
                total_files,
                total_bytes,
                ..
            } => {
                in_tree_mode = true;
                tree_files_started = 0;
                last_files_total = total_files;
                last_bytes_total = total_bytes;
                state.queue.set_progress(id, 0, total_bytes, 0, total_files);
                // Fire the tree totals at the UI *immediately* so the
                // JobRow ring + bottom ProgressBar show the right
                // denominator (e.g. "0 / 103", "0 B / 4.9 GiB")
                // before the first per-file Progress event lands.
                // Without this the ring would sit at whatever the
                // initial enqueue default was (`0 / 1`) until the
                // first TreeProgress ticks in a few hundred ms later.
                emit_progress(&app, &state, id, 0, total_bytes, 0, total_files, 0);
            }
            CopyEvent::Progress {
                bytes,
                total,
                rate_bps,
            } => {
                // Only let per-file Progress events drive the queue's
                // aggregate counters in *single-file* mode. Inside a
                // tree, TreeProgress is the authoritative aggregate
                // source — letting per-file Progress write here
                // would clobber `files_total` back to 1 and
                // `bytes_total` to the current file's size, which is
                // what made the ring + bottom bar show the active
                // file instead of the tree total.
                if !in_tree_mode {
                    state.queue.set_progress(id, bytes, total, 0, 1);
                    emit_progress(&app, &state, id, bytes, total, 0, 1, rate_bps);
                }
                // Rate-limited per-file progress tick for the
                // activity list. This is the per-row bar that IS
                // meant to follow the individual file — tree mode
                // or not — so it stays outside the `if` above.
                if last_progress_emit.elapsed() >= progress_min_interval {
                    activity_seq += 1;
                    let _ = app.emit(
                        crate::ipc::EVENT_FILE_ACTIVITY,
                        crate::ipc::FileActivityDto {
                            job_id: job_id_u64,
                            seq: activity_seq,
                            phase: "progress",
                            src: last_activity_src.to_string_lossy().into_owned(),
                            dst: String::new(),
                            bytes_done: bytes,
                            bytes_total: total,
                            is_dir: false,
                            message: None,
                        },
                    );
                    last_progress_emit = std::time::Instant::now();
                }
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
            CopyEvent::VerifyCompleted {
                algorithm: _,
                src_hex,
                ..
            } => {
                // Stash the source hash on the in-flight item so
                // Completed can persist it. Engine still returns
                // the final Result separately.
                if let Some(item) = current_item.as_mut() {
                    item.hash_hex = Some(src_hex);
                }
            }
            CopyEvent::VerifyFailed { .. } => {
                // Engine returns VerifyFailed as part of the final
                // Result. No state transition here.
            }
            CopyEvent::Completed { bytes, .. } => {
                // In tree mode, per-file Completed events fire as
                // each file finishes — but TreeProgress (fired by
                // the tree engine right after) is the authoritative
                // update for files_done / files_total. Writing the
                // max-of-everything here was jamming files_done all
                // the way to the current discovered total, which
                // TreeProgress then "corrected" back on the next
                // tick → the oscillating counter. Only touch the
                // queue in single-file mode.
                if !in_tree_mode {
                    state.queue.set_progress(
                        id,
                        bytes,
                        last_bytes_total.max(bytes),
                        last_files_total.max(1),
                        last_files_total.max(1),
                    );
                }
                // Per-file "done" ping — flip the row's icon to a
                // checkmark on the frontend side.
                activity_seq += 1;
                let _ = app.emit(
                    crate::ipc::EVENT_FILE_ACTIVITY,
                    crate::ipc::FileActivityDto {
                        job_id: job_id_u64,
                        seq: activity_seq,
                        phase: "done",
                        src: last_activity_src.to_string_lossy().into_owned(),
                        dst: String::new(),
                        bytes_done: bytes,
                        bytes_total: bytes,
                        is_dir: false,
                        message: None,
                    },
                );
                // Phase 9 — persist the item row. Only if history
                // is active and we saw a matching `Started`.
                if let (Some(row), Some(item)) = (history_row, current_item.take())
                    && let Some(history) = &state.history
                {
                    // Phase 34 — record FileCopied in the audit sink
                    // before handing ownership of `item` to the
                    // history row. Hash is whatever Verify emitted;
                    // empty when `verify = None` was configured.
                    let audit_job_id_local = format!("job-{job_id_u64}");
                    crate::audit_commands::record_file_copied(
                        &state.audit,
                        &audit_job_id_local,
                        &item.src,
                        &item.dst,
                        item.hash_hex.as_deref(),
                        bytes,
                    );
                    let entry = ItemRow {
                        job_row_id: row.as_i64(),
                        src: item.src,
                        dst: item.dst,
                        size: bytes,
                        status: "ok".into(),
                        hash_hex: item.hash_hex,
                        error_code: None,
                        error_msg: None,
                        timestamp_ms: now_ms() as i64,
                    };
                    let _ = history.record_item(&entry).await;
                } else {
                    // Tree mode without history still wants the audit
                    // file-copied record.
                    let audit_job_id_local = format!("job-{job_id_u64}");
                    crate::audit_commands::record_file_copied(
                        &state.audit,
                        &audit_job_id_local,
                        &last_activity_src,
                        std::path::Path::new(""),
                        None,
                        bytes,
                    );
                }
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
                // Flip the row to an error icon in the live list.
                activity_seq += 1;
                let _ = app.emit(
                    crate::ipc::EVENT_FILE_ACTIVITY,
                    crate::ipc::FileActivityDto {
                        job_id: job_id_u64,
                        seq: activity_seq,
                        phase: "error",
                        src: err.src.to_string_lossy().into_owned(),
                        dst: err.dst.to_string_lossy().into_owned(),
                        bytes_done: 0,
                        bytes_total: 0,
                        is_dir: false,
                        message: Some(err.message.clone()),
                    },
                );
                // Phase 9 — also record the item in history.
                let kind_str = crate::errors::kind_name(err.kind);
                // Phase 34 — mirror the item failure into the audit
                // sink; active-only when the feature is enabled.
                let audit_job_id_local = format!("job-{job_id_u64}");
                crate::audit_commands::record_file_failed(
                    &state.audit,
                    &audit_job_id_local,
                    &err.src,
                    kind_str,
                    &err.message,
                );
                if let (Some(row), Some(history)) = (history_row, state.history.clone()) {
                    let entry = ItemRow {
                        job_row_id: row.as_i64(),
                        src: err.src.clone(),
                        dst: err.dst.clone(),
                        size: 0,
                        status: "failed".into(),
                        hash_hex: None,
                        error_code: Some(kind_str.to_string()),
                        error_msg: Some(err.message.clone()),
                        timestamp_ms: now_ms() as i64,
                    };
                    let _ = history.record_item(&entry).await;
                }
                // Drop any in-flight item that belonged to the
                // failing file so a subsequent Completed from a
                // later file doesn't inherit stale paths.
                current_item = None;
            }
            CopyEvent::Collision(mut coll) => {
                let job_id = id.as_u64();
                // Honour a prior "Apply to all" decision without
                // bothering the user again.
                if let Some(cached) = state.collisions.cached_resolution(job_id) {
                    coll.resolve(cached);
                    continue;
                }
                // Phase 22 — consult the job's accumulated conflict
                // rules (seeded from the active ConflictProfile,
                // appended to by "Apply to all of this extension"
                // bulk actions). First match wins; fallback applies
                // if nothing matched.
                let src_path = coll.src.clone();
                let dst_path = coll.dst.clone();
                let (src_size, src_modified_ms) = peek_meta(&src_path).await;
                let (dst_size, dst_modified_ms) = peek_meta(&dst_path).await;
                if let Some((rule_res, matched_pattern)) =
                    state.collisions.consult_rules(job_id, &src_path, None)
                {
                    if let Some(resolved) = crate::collisions::apply_rule_resolution(
                        rule_res,
                        src_size,
                        dst_size,
                        src_modified_ms.map(|m| m as i64),
                        dst_modified_ms.map(|m| m as i64),
                        &dst_path,
                    ) {
                        let engine_wire = crate::collisions::resolution_name(&resolved);
                        // Phase 34 — audit the rule-auto-resolved
                        // decision. Interactive-resolve audits live
                        // in `commands::resolve_collision` so the
                        // user-facing picker records with the
                        // right attribution.
                        let audit_job_id_local = format!("job-{job_id}");
                        crate::audit_commands::record_collision_resolved(
                            &state.audit,
                            &audit_job_id_local,
                            &src_path,
                            &dst_path,
                            engine_wire,
                        );
                        let _ = app.emit(
                            crate::ipc::EVENT_COLLISION_AUTO_RESOLVED,
                            crate::ipc::CollisionAutoResolvedDto {
                                job_id,
                                src: src_path.to_string_lossy().into_owned(),
                                dst: dst_path.to_string_lossy().into_owned(),
                                resolution: engine_wire,
                                rule_resolution: rule_res.as_str(),
                                matched_rule_pattern: matched_pattern,
                            },
                        );
                        coll.resolve(resolved);
                        continue;
                    }
                    // KeepBoth with a 10 000-name search miss — fall
                    // through to the interactive prompt below.
                }
                // Extract the oneshot before we reach for the paths
                // (taking moves the Sender out of the Option).
                let Some(tx) = coll.resolver.take() else {
                    // Cloned placeholder from a broadcast subscriber —
                    // nothing to drive here.
                    continue;
                };
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
            CopyEvent::SnapshotCreated {
                kind,
                original,
                snap_mount,
            } => {
                let _ = app.emit(
                    EVENT_SNAPSHOT_CREATED,
                    SnapshotCreatedDto {
                        job_id: id.as_u64(),
                        kind,
                        original: original.to_string_lossy().into_owned(),
                        snap_mount: snap_mount.to_string_lossy().into_owned(),
                    },
                );
            }
            CopyEvent::SparsenessNotSupported { dst_fs } => {
                let _ = app.emit(
                    EVENT_SPARSENESS_NOT_SUPPORTED,
                    SparsenessNotSupportedDto {
                        job_id: id.as_u64(),
                        dst_fs: dst_fs.to_string(),
                    },
                );
            }
            CopyEvent::MetaTranslatedToAppleDouble { ext } => {
                let _ = app.emit(
                    EVENT_META_TRANSLATED_TO_APPLEDOUBLE,
                    MetaTranslatedToAppleDoubleDto {
                        job_id: id.as_u64(),
                        ext,
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

/// In-flight per-file bookkeeping used by `forward_events` to
/// pair a `Started { src, dst }` event with the matching
/// `Completed` or `FileError` so the history's `items` row
/// carries the right path + hash.
struct ItemInFlight {
    src: std::path::PathBuf,
    dst: std::path::PathBuf,
    hash_hex: Option<String>,
}

/// Phase 9 — insert the `jobs` row at job start. Returns `None` if
/// history is disabled (e.g. in-memory `AppState` used by tests)
/// or if the insert failed; the runner continues regardless.
async fn record_history_start(
    state: &AppState,
    kind: copythat_core::JobKind,
    src: &std::path::Path,
    dst: Option<&std::path::Path>,
    verifier: &Option<copythat_core::Verifier>,
) -> Option<JobRowId> {
    let history = state.history.as_ref()?;
    let summary = JobSummary {
        row_id: 0,
        kind: kind_to_wire(kind).to_string(),
        status: "running".into(),
        started_at_ms: now_ms() as i64,
        finished_at_ms: None,
        src_root: src.to_path_buf(),
        dst_root: dst.map(|p| p.to_path_buf()).unwrap_or_default(),
        total_bytes: 0,
        files_ok: 0,
        files_failed: 0,
        // The `Verifier` value doesn't expose its algorithm name
        // publicly today; Phase 11+ can thread that through. For
        // now `"sha256"` vs `None` is the interesting distinction
        // the History column needs — if *any* verifier is set,
        // record a placeholder so queries on `verify_algo IS NOT
        // NULL` still work.
        verify_algo: verifier.as_ref().map(|_| "configured".to_string()),
        options_json: None,
    };
    history.record_start(&summary).await.ok()
}

fn kind_to_wire(kind: copythat_core::JobKind) -> &'static str {
    use copythat_core::JobKind;
    match kind {
        JobKind::Copy => "copy",
        JobKind::Move => "move",
        JobKind::Delete => "delete",
        JobKind::SecureDelete => "secure-delete",
        JobKind::Verify => "verify",
    }
}
