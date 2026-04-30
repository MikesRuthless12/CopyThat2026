//! Phase 45.2 — Tauri IPC roundtrip tests for the named-queue surface.
//!
//! The Tauri command wrappers themselves require a live runtime + an
//! `AppHandle`, neither of which we can construct in a plain test.
//! Instead we exercise the `*_impl(&AppState, ...)` helpers that hold
//! the actual logic; the thin `#[tauri::command]` shells just forward
//! to those.
//!
//! Test plan:
//!   1. `queue_route_job_impl` lands a job → `queue_list_impl` shows it.
//!   2. Two destinations on the same logical drive share one queue.
//!   3. Two destinations on distinct drives spawn separate queues.
//!   4. `queue_merge_impl` collapses two queues + emits the merge event.
//!   5. `queue_set_f2_mode_impl` flips the registry's atomic flag.
//!   6. `queue_pin_destination_impl` persists + dedupes; rejects empties.
//!   7. `queue_get_pinned_impl` round-trips the persisted list.
//!   8. `queue_route_job_impl` rejects unknown wire-string kinds.

use std::sync::Arc;
use std::sync::atomic::Ordering;

use copythat_core::{QueueRegistry, QueueRegistryEvent, VolumeProbe};
use copythat_settings::{ProfileStore, Settings};
use copythat_ui_lib::queue_commands::{
    queue_get_pinned_impl, queue_list_impl, queue_merge_impl, queue_pin_destination_impl,
    queue_route_job_impl, queue_set_f2_mode_impl, queue_unpin_destination_impl,
};
use copythat_ui_lib::state::AppState;
use std::path::Path;

/// Deterministic VolumeProbe stub. Every path under `/drive/A/...`
/// hashes to drive 0xAAAA, every `/drive/B/...` to drive 0xBBBB,
/// everything else returns `None`. Drive labels echo the prefix
/// letter so we can assert spawned queue names.
#[derive(Debug)]
struct FakeProbe;

impl FakeProbe {
    fn id_for(path: &Path) -> Option<u64> {
        let s = path.to_string_lossy();
        if s.contains("/drive/A") {
            Some(0xAAAA)
        } else if s.contains("/drive/B") {
            Some(0xBBBB)
        } else {
            None
        }
    }
}

impl VolumeProbe for FakeProbe {
    fn volume_id(&self, path: &Path) -> Option<u64> {
        FakeProbe::id_for(path)
    }
    fn drive_label(&self, path: &Path) -> Option<String> {
        FakeProbe::id_for(path).map(|id| format!("0x{id:04X}"))
    }
}

/// Build an AppState whose `queues` registry uses our `FakeProbe`
/// instead of the real `PlatformVolumeProbe`. Avoids the test running
/// against the host filesystem's actual volume-id layout — every
/// path becomes deterministic.
fn fake_state() -> AppState {
    let mut state = AppState::new_with(
        None,
        Settings::default(),
        std::path::PathBuf::new(),
        ProfileStore::new(std::path::PathBuf::new()),
    );
    state.queues = QueueRegistry::new().with_probe(Arc::new(FakeProbe));
    state
}

#[test]
fn route_job_then_list_shows_one_queue_with_one_job() {
    let state = fake_state();
    let routed = queue_route_job_impl(&state, "copy", "/drive/A/src", Some("/drive/A/dst"))
        .expect("route succeeds");
    let list = queue_list_impl(&state);
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].id, routed.queue_id);
    assert_eq!(list[0].badge_count, 1);
    assert!(!list[0].running, "job is Pending, not Running");
    // Job ids are u64 sequentially from 1.
    assert_eq!(routed.job_id, 1);
}

#[test]
fn two_jobs_to_same_drive_share_one_queue() {
    let state = fake_state();
    let r1 = queue_route_job_impl(&state, "copy", "/drive/A/s1", Some("/drive/A/d1")).unwrap();
    let r2 = queue_route_job_impl(&state, "move", "/drive/A/s2", Some("/drive/A/d2")).unwrap();
    assert_eq!(r1.queue_id, r2.queue_id);
    let list = queue_list_impl(&state);
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].badge_count, 2);
}

#[test]
fn two_jobs_to_distinct_drives_spawn_two_queues() {
    let state = fake_state();
    let ra = queue_route_job_impl(&state, "copy", "/drive/A/s", Some("/drive/A/d")).unwrap();
    let rb = queue_route_job_impl(&state, "copy", "/drive/B/s", Some("/drive/B/d")).unwrap();
    assert_ne!(ra.queue_id, rb.queue_id);
    let list = queue_list_impl(&state);
    assert_eq!(list.len(), 2);
}

#[test]
fn merge_collapses_two_queues_into_one_and_emits_event() {
    let state = fake_state();

    // Subscribe before the routes so we capture every event the
    // registry fires during this test.
    let mut rx = state.queues.subscribe();

    let ra = queue_route_job_impl(&state, "copy", "/drive/A/s", Some("/drive/A/d")).unwrap();
    let rb = queue_route_job_impl(&state, "copy", "/drive/B/s", Some("/drive/B/d")).unwrap();

    queue_merge_impl(&state, rb.queue_id, ra.queue_id).expect("merge B → A");

    let list = queue_list_impl(&state);
    assert_eq!(list.len(), 1, "B queue removed after merge");
    assert_eq!(list[0].id, ra.queue_id);
    assert_eq!(list[0].badge_count, 2, "A absorbed both jobs");

    // Drain the event stream and look for a QueueMerged with the
    // expected (src, dst) pair. Other events (QueueAdded /
    // JobRouted / QueueRemoved) intersperse but we only assert
    // the merged event lands.
    let mut saw_merged = false;
    while let Ok(evt) = rx.try_recv() {
        if let QueueRegistryEvent::QueueMerged { src, dst } = evt
            && src.as_u64() == rb.queue_id
            && dst.as_u64() == ra.queue_id
        {
            saw_merged = true;
        }
    }
    assert!(saw_merged, "expected QueueMerged event after merge_into");
}

#[test]
fn merge_unknown_id_returns_error() {
    let state = fake_state();
    let r = queue_route_job_impl(&state, "copy", "/drive/A/s", Some("/drive/A/d")).unwrap();
    let err = queue_merge_impl(&state, 9999, r.queue_id).unwrap_err();
    assert!(err.contains("9999") || err.to_lowercase().contains("source"));
}

#[test]
fn set_f2_mode_flips_registry_flag() {
    let state = fake_state();
    assert!(!state.queues.auto_enqueue_next.load(Ordering::Relaxed));
    queue_set_f2_mode_impl(&state, true);
    assert!(state.queues.auto_enqueue_next.load(Ordering::Relaxed));
    queue_set_f2_mode_impl(&state, false);
    assert!(!state.queues.auto_enqueue_next.load(Ordering::Relaxed));
}

#[test]
fn pin_destination_persists_and_dedupes() {
    let state = fake_state();

    let after_first =
        queue_pin_destination_impl(&state, "Inbox", "/drive/A/inbox").expect("pin succeeds");
    assert_eq!(after_first.len(), 1);
    assert_eq!(after_first[0].label, "Inbox");
    assert_eq!(after_first[0].path, "/drive/A/inbox");

    // Re-pinning the same (label, path) is a no-op — list stays at 1.
    let after_dupe = queue_pin_destination_impl(&state, "Inbox", "/drive/A/inbox").unwrap();
    assert_eq!(after_dupe.len(), 1);

    // A different label-or-path appends a row.
    let after_second =
        queue_pin_destination_impl(&state, "Backup", "/drive/B/backup").unwrap();
    assert_eq!(after_second.len(), 2);

    // get_pinned reads back what was persisted in AppState.settings.
    let snapshot = queue_get_pinned_impl(&state);
    assert_eq!(snapshot.len(), 2);
    assert_eq!(snapshot[0].label, "Inbox");
    assert_eq!(snapshot[1].label, "Backup");
}

#[test]
fn pin_destination_rejects_empty_label_or_path() {
    let state = fake_state();
    assert!(queue_pin_destination_impl(&state, "", "/some/path").is_err());
    assert!(queue_pin_destination_impl(&state, "   ", "/some/path").is_err());
    assert!(queue_pin_destination_impl(&state, "Label", "").is_err());
    assert!(queue_pin_destination_impl(&state, "Label", "  ").is_err());
    // None of the failed attempts should have leaked a row through.
    assert!(queue_get_pinned_impl(&state).is_empty());
}

#[test]
fn unpin_destination_removes_match_and_is_idempotent() {
    let state = fake_state();
    queue_pin_destination_impl(&state, "Inbox", "/drive/A/inbox").unwrap();
    queue_pin_destination_impl(&state, "Backup", "/drive/B/backup").unwrap();
    assert_eq!(queue_get_pinned_impl(&state).len(), 2);

    // Removing an existing row drops it; the other row survives.
    let after = queue_unpin_destination_impl(&state, "Inbox", "/drive/A/inbox").unwrap();
    assert_eq!(after.len(), 1);
    assert_eq!(after[0].label, "Backup");

    // Removing a row that isn't there is a no-op (idempotent).
    let after_again = queue_unpin_destination_impl(&state, "Inbox", "/drive/A/inbox").unwrap();
    assert_eq!(after_again.len(), 1);

    // Whitespace in the inputs is trimmed before the comparison —
    // chatty UIs that round-trip user input shouldn't leak phantom
    // rows that escape removal.
    let after_trim =
        queue_unpin_destination_impl(&state, "  Backup  ", "  /drive/B/backup  ").unwrap();
    assert!(after_trim.is_empty());
}

#[test]
fn route_job_rejects_unknown_wire_kind() {
    let state = fake_state();
    let err = queue_route_job_impl(&state, "teleport", "/drive/A/s", Some("/drive/A/d"))
        .unwrap_err();
    assert!(err.contains("teleport"), "error mentions the bad kind: {err}");
}

#[test]
fn route_job_with_no_dst_lands_in_default_queue() {
    // With our FakeProbe, a `None` dst yields `None` drive id; the
    // anonymous-bucket path uses the default name.
    let state = fake_state();
    let r = queue_route_job_impl(&state, "delete", "/drive/A/s", None).unwrap();
    let list = queue_list_impl(&state);
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].id, r.queue_id);
    assert_eq!(list[0].name, "default");
}
