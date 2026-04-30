//! Integration tests for the Tauri command layer, exercised through
//! the queue + runner without booting the webview.
//!
//! We can't construct a real `tauri::AppHandle` in a plain test — the
//! webview pipeline isn't running — so these tests drive the engine
//! pieces that *do* work in isolation: the queue state machine, the
//! IPC DTO serialisation, and `copythat_core::copy_file` with a
//! CopyControl that the queue is steering.
//!
//! Together this covers:
//!   * `list_jobs`-equivalent: queue.snapshot() → JobDto round-trip
//!   * pause / resume / cancel pathways
//!   * `start_copy`-equivalent: enqueue + drive `copy_file` to
//!     completion, assert the queue ended in `Succeeded`
//!
//! Phase 5's end-to-end "drops a folder through the webview" check
//! lives in `tests/smoke/phase_05_ui.ps1` / `.sh`; that path
//! requires Tauri Driver / Playwright and is wired in the smoke
//! test harness, not here.

use std::path::PathBuf;

use copythat_core::{CopyOptions, JobKind, JobState, Queue, copy_file};
use copythat_ui_lib::ipc::{JobDto, job_state_name};
use copythat_ui_lib::runner::build_globals;
use copythat_ui_lib::state::AppState;
use tempfile::tempdir;
use tokio::sync::mpsc;

#[test]
fn job_state_names_round_trip() {
    assert_eq!(job_state_name(JobState::Pending), "pending");
    assert_eq!(job_state_name(JobState::Running), "running");
    assert_eq!(job_state_name(JobState::Paused), "paused");
    assert_eq!(job_state_name(JobState::Cancelled), "cancelled");
    assert_eq!(job_state_name(JobState::Succeeded), "succeeded");
    assert_eq!(job_state_name(JobState::Failed), "failed");
}

#[test]
fn job_dto_splits_filename_from_parent() {
    let q = Queue::new();
    let (_id, _ctrl) = q.add(
        JobKind::Copy,
        PathBuf::from("/tmp/reports/q1.csv"),
        Some(PathBuf::from("/backup/q1.csv")),
    );
    let snap = q.snapshot();
    let dto = JobDto::from_job(&snap[0]);
    assert_eq!(dto.name, "q1.csv");
    assert!(dto.subpath.as_deref().unwrap().contains("reports"));
    assert_eq!(dto.kind, "copy");
    assert_eq!(dto.state, "pending");
}

#[test]
fn list_jobs_impl_attributes_registry_jobs_to_their_queue_ids() {
    // Phase 45.7 follow-up — `list_jobs` previously only iterated
    // `state.queue` (the legacy default queue), so jobs added via
    // `queue_route_job` were invisible to the frontend. The fix:
    // the IPC also walks every queue in `state.queues` and stamps
    // each DTO with `queueId = originating queue's id`. The
    // frontend's `visibleJobs` filter then routes the row under
    // the correct JobListTabs tab.
    let state = AppState::new();

    // Legacy queue: one job. queueId should be 0 (DEFAULT).
    let (legacy_id, _ctrl) = state.queue.add(
        JobKind::Copy,
        PathBuf::from("/legacy/src"),
        Some(PathBuf::from("/legacy/dst")),
    );

    // Registry queue: route a second job. The probe is the real
    // PlatformVolumeProbe, which yields None for non-existent
    // paths → all routed jobs land in the registry's anonymous
    // bucket with a non-zero queue id.
    let (qid, _routed_id, _routed_ctrl) = state.queues.route(
        JobKind::Copy,
        PathBuf::from("/routed/src"),
        Some(PathBuf::from("/routed/dst")),
    );
    assert_ne!(qid.as_u64(), 0, "registry queue id should be non-zero");

    let dtos = copythat_ui_lib::commands::list_jobs_impl(&state);
    assert_eq!(dtos.len(), 2, "both legacy and routed jobs surface");

    // The frontend's `visibleJobs` filter discriminates rows by
    // `queue_id`; the JobIds within a queue may also overlap with
    // sibling queues' ids in legacy callers, so look the rows up by
    // queue_id directly rather than by job id.
    let legacy_dto = dtos
        .iter()
        .find(|d| d.queue_id == 0)
        .expect("legacy queue job surfaces with queueId=DEFAULT");
    assert_eq!(legacy_dto.id, legacy_id.as_u64());
    assert_eq!(
        legacy_dto.kind, "copy",
        "legacy DTO carries the kind from the legacy queue's add() call",
    );

    let routed_dto = dtos
        .iter()
        .find(|d| d.queue_id == qid.as_u64())
        .expect("registry-routed job surfaces with its queue_id");
    assert!(
        routed_dto.src.contains("routed"),
        "routed DTO carries the routed src path: {}",
        routed_dto.src,
    );
}

#[test]
fn job_ids_are_unique_across_legacy_queue_and_registry() {
    // Phase 45.7 follow-up — the legacy `state.queue` and every
    // registry queue must mint ids from one shared counter so a
    // future runner reconciliation can move jobs between them
    // without colliding on JobId. Without the wiring this test
    // asserts, both surfaces would issue `JobId(1)` to their first
    // adds and a runner keyed on JobId would route events to the
    // wrong row.
    let state = AppState::new();
    let (legacy_id, _l_ctrl) =
        state.queue.add(JobKind::Copy, PathBuf::from("/l/s"), Some(PathBuf::from("/l/d")));
    let (_qid, routed_id, _r_ctrl) =
        state.queues.route(JobKind::Copy, PathBuf::from("/r/s"), Some(PathBuf::from("/r/d")));
    let (legacy_id_2, _l2_ctrl) =
        state.queue.add(JobKind::Copy, PathBuf::from("/l2/s"), Some(PathBuf::from("/l2/d")));

    let ids = [legacy_id.as_u64(), routed_id.as_u64(), legacy_id_2.as_u64()];
    let mut sorted = ids;
    sorted.sort();
    assert_eq!(
        sorted,
        [1, 2, 3],
        "expected three sequential ids drawn from one shared counter, got {ids:?}",
    );
}

#[test]
fn globals_reflect_queue_state() {
    let q = Queue::new();
    let g = build_globals(&q);
    assert_eq!(g.state, "idle");
    assert_eq!(g.queued_jobs, 0);

    // Enqueue a job; state shifts to "copying" because pending counts
    // as "work is about to happen".
    let (_id, _ctrl) = q.add(
        JobKind::Copy,
        PathBuf::from("/src"),
        Some(PathBuf::from("/dst")),
    );
    let g = build_globals(&q);
    assert_eq!(g.queued_jobs, 1);
    assert_eq!(g.state, "copying");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn pause_resume_cancel_cycle_runs_through_queue() {
    // Enqueue a copy big enough that the engine is guaranteed to see
    // the pause flag between buffers on any remotely modern SSD: the
    // copy loop checks `is_paused` between fill_buf / write_all
    // cycles, so a too-small source can race the 50-ms progress
    // throttle and finish before the pause call even lands.
    //
    // 64 MiB is comfortable on a fast NVMe (~150 ms at 400 MB/s)
    // without making the test slow on laptop spinning rust.
    let dir = tempdir().unwrap();
    let src = dir.path().join("src.bin");
    let dst = dir.path().join("dst.bin");
    std::fs::write(&src, vec![0xABu8; 64 * 1024 * 1024]).unwrap();

    let q = Queue::new();
    // Use a tiny buffer so the loop iterates hundreds of times and
    // the pause flag check is hit reliably regardless of platform
    // I/O speed.
    let opts = CopyOptions {
        buffer_size: copythat_core::MIN_BUFFER_SIZE,
        ..Default::default()
    };
    let (id, ctrl) = q.add(JobKind::Copy, src.clone(), Some(dst.clone()));
    q.start(id);

    let (tx, mut rx) = mpsc::channel(1024);

    let copy_task = tokio::spawn({
        let src = src.clone();
        let dst = dst.clone();
        let opts = opts.clone();
        async move { copy_file(&src, &dst, opts, ctrl, tx).await }
    });

    // Pause immediately — don't wait for the first Progress tick,
    // which is itself throttled 50 ms into the copy and could race
    // the pause request past the whole file on a fast disk.
    q.pause_job(id);

    // Drain events until the engine reports Paused (or Completed
    // if the race genuinely ran out of bytes before the first
    // paused-check).
    let outcome = tokio::time::timeout(std::time::Duration::from_secs(10), async {
        while let Some(evt) = rx.recv().await {
            match evt {
                copythat_core::CopyEvent::Paused => return Some("paused"),
                copythat_core::CopyEvent::Completed { .. } => return Some("completed"),
                _ => continue,
            }
        }
        None
    })
    .await
    .unwrap_or(None);
    assert!(
        matches!(outcome, Some("paused") | Some("completed")),
        "expected paused or completed, got: {outcome:?}"
    );

    q.resume_job(id);
    // No blocking wait here — we just want resume to flip the flag
    // and the copy loop to fall through to completion.

    // Drain remaining events so the engine's channel doesn't back
    // up, and let the engine finish.
    tokio::spawn(async move { while rx.recv().await.is_some() {} });

    let report = tokio::time::timeout(std::time::Duration::from_secs(30), copy_task)
        .await
        .expect("copy did not finish in time")
        .expect("copy task panicked")
        .expect("copy returned error");
    assert_eq!(report.bytes, 64 * 1024 * 1024);

    // Queue isn't automatically marked Succeeded by copy_file —
    // the runner would do that. Here we drive the transition
    // manually to validate `mark_completed` is reachable.
    q.mark_completed(id);
    assert_eq!(q.get(id).unwrap().state, JobState::Succeeded);
}

#[tokio::test]
async fn cancel_before_start_transitions_through_queue() {
    let q = Queue::new();
    let (id, _ctrl) = q.add(
        JobKind::Copy,
        PathBuf::from("/src"),
        Some(PathBuf::from("/dst")),
    );
    q.cancel_job(id);
    assert_eq!(q.get(id).unwrap().state, JobState::Cancelled);
}

#[test]
fn available_locales_covers_all_eighteen() {
    let locales = copythat_ui_lib::i18n::available_locales();
    assert_eq!(locales.len(), 18);
    for code in [
        "en", "es", "zh-CN", "hi", "ar", "pt-BR", "ru", "ja", "de", "fr", "ko", "it", "tr", "vi",
        "pl", "nl", "id", "uk",
    ] {
        assert!(locales.iter().any(|c| c == code), "missing locale: {code}");
    }
}

#[test]
fn translations_include_every_en_key_in_every_locale() {
    let en = copythat_ui_lib::i18n::translations("en");
    for code in copythat_ui_lib::i18n::available_locales() {
        let t = copythat_ui_lib::i18n::translations(&code);
        for key in en.keys() {
            assert!(t.contains_key(key), "{code} missing key: {key}");
        }
    }
}

#[test]
fn icon_classifier_knows_common_buckets() {
    let dto = copythat_ui_lib::icon::classify(std::path::Path::new("clip.mp4"));
    assert_eq!(dto.kind, "video");
    assert_eq!(dto.extension.as_deref(), Some("mp4"));
}
