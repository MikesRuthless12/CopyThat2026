//! Phase 45.1 — `QueueRegistry` routing, merging, and F2-mode tests.
//!
//! These tests exercise the registry's drive-id routing in isolation
//! from `copythat-platform`'s real `volume_id` helper. A `FakeProbe`
//! maps fake paths (`/drive/A/...` / `/drive/B/...`) to deterministic
//! drive ids so the routing logic can be observed without touching
//! the host filesystem.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use copythat_core::{
    JobKind, JobState, QueueId, QueueMergeError, QueueRegistry, VolumeProbe,
};

#[derive(Debug)]
struct FakeProbe;

impl FakeProbe {
    fn drive_for(path: &Path) -> Option<u64> {
        let s = path.to_string_lossy();
        if s.contains("/drive/A") {
            Some(0xAAAA_AAAA)
        } else if s.contains("/drive/B") {
            Some(0xBBBB_BBBB)
        } else {
            None
        }
    }
}

impl VolumeProbe for FakeProbe {
    fn volume_id(&self, path: &Path) -> Option<u64> {
        FakeProbe::drive_for(path)
    }

    fn drive_label(&self, path: &Path) -> Option<String> {
        FakeProbe::drive_for(path).map(|id| format!("0x{id:08X}"))
    }
}

fn drive_a(name: &str) -> PathBuf {
    PathBuf::from(format!("/drive/A/{name}"))
}

fn drive_b(name: &str) -> PathBuf {
    PathBuf::from(format!("/drive/B/{name}"))
}

#[test]
fn three_jobs_to_drive_a_share_one_queue() {
    let reg = QueueRegistry::new().with_probe(Arc::new(FakeProbe));

    let (q1, _, _) = reg.route(JobKind::Copy, drive_a("src1"), Some(drive_a("dst1")));
    let (q2, _, _) = reg.route(JobKind::Copy, drive_a("src2"), Some(drive_a("dst2")));
    let (q3, _, _) = reg.route(JobKind::Copy, drive_a("src3"), Some(drive_a("dst3")));

    assert_eq!(q1, q2, "second job should land in the same drive-A queue");
    assert_eq!(q2, q3, "third job should land in the same drive-A queue");
    assert_eq!(reg.len(), 1, "registry should hold exactly one queue");
    let queue = reg.get(q1).expect("drive-A queue exists");
    assert_eq!(queue.len(), 3, "drive-A queue should hold three jobs");
}

#[test]
fn jobs_to_distinct_drives_get_distinct_queues() {
    let reg = QueueRegistry::new().with_probe(Arc::new(FakeProbe));

    let (qa, _, _) = reg.route(JobKind::Copy, drive_a("src1"), Some(drive_a("dst1")));
    let (qb, _, _) = reg.route(JobKind::Copy, drive_b("src1"), Some(drive_b("dst1")));

    assert_ne!(qa, qb, "drive-A and drive-B routes must land in different queues");
    assert_eq!(reg.len(), 2);
    assert_eq!(reg.get(qa).unwrap().len(), 1);
    assert_eq!(reg.get(qb).unwrap().len(), 1);
}

#[test]
fn merge_into_collapses_two_queues_into_one() {
    let reg = QueueRegistry::new().with_probe(Arc::new(FakeProbe));

    // 3 jobs to drive A, 1 to drive B → 2 queues.
    let (qa, _, _) = reg.route(JobKind::Copy, drive_a("s1"), Some(drive_a("d1")));
    let (qa2, _, _) = reg.route(JobKind::Copy, drive_a("s2"), Some(drive_a("d2")));
    let (qa3, _, _) = reg.route(JobKind::Copy, drive_a("s3"), Some(drive_a("d3")));
    let (qb, _, _) = reg.route(JobKind::Copy, drive_b("s1"), Some(drive_b("d1")));

    assert_eq!(qa, qa2);
    assert_eq!(qa, qa3);
    assert_ne!(qa, qb);
    assert_eq!(reg.len(), 2);

    reg.merge_into(qb, qa).expect("merge_into(B → A) succeeds");

    assert_eq!(reg.len(), 1, "B queue removed after merge");
    assert!(reg.get(qb).is_none(), "B queue id no longer resolvable");
    let merged = reg.get(qa).expect("A queue still present");
    assert_eq!(merged.len(), 4, "A queue should hold all four jobs");
}

#[test]
fn merge_into_self_is_noop() {
    let reg = QueueRegistry::new().with_probe(Arc::new(FakeProbe));
    let (qa, _, _) = reg.route(JobKind::Copy, drive_a("s"), Some(drive_a("d")));
    reg.merge_into(qa, qa).expect("self-merge is a no-op");
    assert_eq!(reg.len(), 1);
    assert_eq!(reg.get(qa).unwrap().len(), 1);
}

#[test]
fn merge_into_unknown_queue_returns_error() {
    let reg = QueueRegistry::new().with_probe(Arc::new(FakeProbe));
    let (qa, _, _) = reg.route(JobKind::Copy, drive_a("s"), Some(drive_a("d")));
    let bogus = QueueId::from_u64(9999);
    assert_eq!(
        reg.merge_into(bogus, qa),
        Err(QueueMergeError::UnknownSrc(bogus)),
    );
    assert_eq!(
        reg.merge_into(qa, bogus),
        Err(QueueMergeError::UnknownDst(bogus)),
    );
    assert_eq!(reg.len(), 1, "registry untouched on error");
}

#[test]
fn auto_enqueue_next_routes_into_running_queue() {
    let reg = QueueRegistry::new().with_probe(Arc::new(FakeProbe));

    // First job lands on drive A.
    let (qa, job_a, _) = reg.route(JobKind::Copy, drive_a("s1"), Some(drive_a("d1")));
    let queue_a = reg.get(qa).unwrap();
    queue_a.start(job_a);
    assert_eq!(
        queue_a.get(job_a).unwrap().state,
        JobState::Running,
        "drive-A job is now running",
    );

    // Without F2, a drive-B job would create a new queue. With F2 on,
    // it should land in the running drive-A queue instead.
    reg.auto_enqueue_next.store(true, std::sync::atomic::Ordering::Relaxed);
    let (qb, _, _) = reg.route(JobKind::Copy, drive_b("s1"), Some(drive_b("d1")));

    assert_eq!(
        qb, qa,
        "with F2 mode on, drive-B job should be routed into the running drive-A queue",
    );
    assert_eq!(reg.len(), 1, "no new queue spawned in F2 mode");
    assert_eq!(reg.get(qa).unwrap().len(), 2, "running queue grew by one");

    // Turning F2 off again restores normal drive-based routing.
    reg.auto_enqueue_next.store(false, std::sync::atomic::Ordering::Relaxed);
    let (qb2, _, _) = reg.route(JobKind::Copy, drive_b("s2"), Some(drive_b("d2")));
    assert_ne!(qb2, qa, "with F2 off, drive-B routes to a new queue again");
    assert_eq!(reg.len(), 2);
}

#[test]
fn registry_without_probe_groups_everything_into_one_queue() {
    // No probe installed → every dst falls into a single anonymous queue.
    let reg = QueueRegistry::new();
    let (q1, _, _) = reg.route(JobKind::Copy, drive_a("s1"), Some(drive_a("d1")));
    let (q2, _, _) = reg.route(JobKind::Copy, drive_b("s2"), Some(drive_b("d2")));
    assert_eq!(q1, q2);
    assert_eq!(reg.len(), 1);
    assert_eq!(reg.get(q1).unwrap().name(), "default");
}

#[test]
fn spawned_queue_name_is_drive_label() {
    let reg = QueueRegistry::new().with_probe(Arc::new(FakeProbe));
    let (qa, _, _) = reg.route(JobKind::Copy, drive_a("s"), Some(drive_a("d")));
    let queue = reg.get(qa).unwrap();
    let name = queue.name();
    assert!(
        name.contains("0xAAAAAAAA"),
        "expected queue name to embed the drive label, got {name:?}",
    );
}
