//! Queue CRUD + pub/sub + state transitions.

mod common;

use std::path::PathBuf;
use std::time::Duration;

use copythat_core::{CopyError, CopyErrorKind, JobKind, JobState, Queue, QueueEvent};
use tokio::sync::broadcast;

fn src(id: u32) -> PathBuf {
    PathBuf::from(format!("/virtual/src/{id}"))
}
fn dst(id: u32) -> PathBuf {
    PathBuf::from(format!("/virtual/dst/{id}"))
}

async fn next_event(rx: &mut broadcast::Receiver<QueueEvent>) -> QueueEvent {
    tokio::time::timeout(Duration::from_secs(2), rx.recv())
        .await
        .expect("timed out waiting for QueueEvent")
        .expect("broadcast closed")
}

#[tokio::test]
async fn add_and_snapshot_contains_the_job() {
    let q = Queue::new();
    let mut rx = q.subscribe();
    let (id, _ctrl) = q.add(JobKind::Copy, src(1), Some(dst(1)));

    assert!(matches!(next_event(&mut rx).await, QueueEvent::JobAdded(x) if x == id));

    let snap = q.snapshot();
    assert_eq!(snap.len(), 1);
    assert_eq!(snap[0].id, id);
    assert_eq!(snap[0].kind, JobKind::Copy);
    assert_eq!(snap[0].state, JobState::Pending);
}

#[tokio::test]
async fn start_then_progress_then_complete_emits_expected_events() {
    let q = Queue::new();
    let mut rx = q.subscribe();
    let (id, _) = q.add(JobKind::Copy, src(1), Some(dst(1)));
    let _added = next_event(&mut rx).await;

    q.start(id);
    assert!(matches!(next_event(&mut rx).await, QueueEvent::JobStarted(x) if x == id));

    q.set_progress(id, 500, 1000, 1, 2);
    let progress = next_event(&mut rx).await;
    match progress {
        QueueEvent::JobProgress {
            bytes_done,
            bytes_total,
            files_done,
            files_total,
            ..
        } => {
            assert_eq!(bytes_done, 500);
            assert_eq!(bytes_total, 1000);
            assert_eq!(files_done, 1);
            assert_eq!(files_total, 2);
        }
        other => panic!("expected JobProgress, got {other:?}"),
    }

    q.mark_completed(id);
    assert!(matches!(next_event(&mut rx).await, QueueEvent::JobCompleted(x) if x == id));

    let job = q.get(id).unwrap();
    assert_eq!(job.state, JobState::Succeeded);
    assert!(job.finished_at.is_some());
}

#[tokio::test]
async fn pause_resume_cancel_drive_the_control_and_emit_events() {
    let q = Queue::new();
    let mut rx = q.subscribe();
    let (id, ctrl) = q.add(JobKind::Copy, src(1), Some(dst(1)));
    let _added = next_event(&mut rx).await;
    q.start(id);
    let _started = next_event(&mut rx).await;

    q.pause_job(id);
    assert!(matches!(next_event(&mut rx).await, QueueEvent::JobPaused(x) if x == id));
    assert!(ctrl.is_paused());
    assert_eq!(q.get(id).unwrap().state, JobState::Paused);

    q.resume_job(id);
    assert!(matches!(next_event(&mut rx).await, QueueEvent::JobResumed(x) if x == id));
    assert!(!ctrl.is_paused());
    assert_eq!(q.get(id).unwrap().state, JobState::Running);

    q.cancel_job(id);
    assert!(matches!(next_event(&mut rx).await, QueueEvent::JobCancelled(x) if x == id));
    assert!(ctrl.is_cancelled());
    assert_eq!(q.get(id).unwrap().state, JobState::Cancelled);
}

#[tokio::test]
async fn reorder_moves_the_job_and_emits_index() {
    let q = Queue::new();
    let (a, _) = q.add(JobKind::Copy, src(1), None);
    let (b, _) = q.add(JobKind::Copy, src(2), None);
    let (c, _) = q.add(JobKind::Copy, src(3), None);

    let mut rx = q.subscribe(); // subscribe after so we don't see JobAdded
    q.reorder(a, 2);
    match next_event(&mut rx).await {
        QueueEvent::JobReordered { id, new_index } => {
            assert_eq!(id, a);
            assert_eq!(new_index, 2);
        }
        other => panic!("expected JobReordered, got {other:?}"),
    }

    let snap = q.snapshot();
    assert_eq!(snap[0].id, b);
    assert_eq!(snap[1].id, c);
    assert_eq!(snap[2].id, a);
}

#[tokio::test]
async fn reorder_clamps_to_bounds() {
    let q = Queue::new();
    let (a, _) = q.add(JobKind::Copy, src(1), None);
    let (_, _) = q.add(JobKind::Copy, src(2), None);
    let (_, _) = q.add(JobKind::Copy, src(3), None);
    q.reorder(a, 99);
    let snap = q.snapshot();
    assert_eq!(snap.last().unwrap().id, a);
}

#[tokio::test]
async fn mark_failed_records_error_on_job() {
    let q = Queue::new();
    let (id, _) = q.add(JobKind::Copy, src(1), Some(dst(1)));
    let err = CopyError {
        kind: CopyErrorKind::PermissionDenied,
        src: src(1),
        dst: dst(1),
        raw_os_error: Some(5),
        message: "nope".into(),
    };
    q.mark_failed(id, err.clone());
    let job = q.get(id).unwrap();
    assert_eq!(job.state, JobState::Failed);
    assert_eq!(
        job.last_error.as_ref().unwrap().kind,
        CopyErrorKind::PermissionDenied
    );
}

#[tokio::test]
async fn subscribers_see_independent_streams() {
    let q = Queue::new();
    let mut a = q.subscribe();
    let mut b = q.subscribe();

    let (id, _) = q.add(JobKind::Copy, src(1), None);
    assert!(matches!(next_event(&mut a).await, QueueEvent::JobAdded(x) if x == id));
    assert!(matches!(next_event(&mut b).await, QueueEvent::JobAdded(x) if x == id));
}

#[tokio::test]
async fn remove_cancels_then_drops() {
    let q = Queue::new();
    let (id, ctrl) = q.add(JobKind::Copy, src(1), None);
    q.start(id);
    q.remove(id);
    assert!(ctrl.is_cancelled());
    assert!(q.get(id).is_none());
    assert!(q.is_empty());
}
