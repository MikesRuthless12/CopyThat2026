//! Phase 33b — runner integration for the Phase 33 mount crate.
//!
//! Owns a [`MountRegistry`] keyed on `job_row_id` so a job can be
//! mounted once at a time; repeat mount calls unmount the previous
//! handle and create a fresh one at the new mountpoint. IPC
//! commands surface list / mount / unmount to the Svelte layer,
//! plus a `mount_latest_on_launch` helper the runner calls from
//! `lib.rs::run` when the user has `Settings::mount.mount_on_launch`
//! toggled on.
//!
//! Phase 33b uses [`copythat_mount::NoopBackend`] so the flow works
//! end-to-end without kernel IO. Phase 33c swaps in the real
//! `fuser` / `winfsp` backends behind the crate's existing feature
//! flags (`--features fuse` / `--features winfsp`).

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use copythat_history::HistoryFilter;
use copythat_mount::{
    MountBackend, MountError, MountHandle, MountLayout, NoopBackend, default_backend_name,
};
use serde::Serialize;

use crate::state::AppState;

/// Live-mount registry. One handle per job; re-mounting a job
/// drops the previous handle (via `MountHandle::Drop`) before
/// replacing it.
#[derive(Clone, Default)]
pub struct MountRegistry {
    inner: Arc<Mutex<MountRegistryInner>>,
}

#[derive(Default)]
struct MountRegistryInner {
    /// Keyed on `job_row_id`. `Option<MountHandle>` so the handle
    /// can be taken out for explicit unmount without losing the
    /// map entry until the reply fires.
    handles: HashMap<i64, MountHandle>,
    /// Parallel mountpoint cache so `list_mounts` can reply
    /// without taking the handles lock beyond a snapshot.
    mountpoints: HashMap<i64, PathBuf>,
}

impl MountRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Replace the handle for `job_row_id`. Returns the previous
    /// mountpoint (if any) so the caller can surface the "swapped"
    /// UX.
    fn insert(&self, job_row_id: i64, handle: MountHandle) -> Option<PathBuf> {
        let mut guard = self.inner.lock().expect("mount registry poisoned");
        let mountpoint = handle.mountpoint().to_path_buf();
        let prev = guard.mountpoints.insert(job_row_id, mountpoint);
        // Drop the old handle explicitly so its Drop runs while the
        // lock is held, ensuring no race with a concurrent mount.
        guard.handles.insert(job_row_id, handle);
        prev
    }

    /// Take a handle out for explicit unmount. Returns `None` if
    /// the job isn't currently mounted.
    fn take(&self, job_row_id: i64) -> Option<(MountHandle, PathBuf)> {
        let mut guard = self.inner.lock().expect("mount registry poisoned");
        let handle = guard.handles.remove(&job_row_id)?;
        let mountpoint = guard
            .mountpoints
            .remove(&job_row_id)
            .unwrap_or_else(|| handle.mountpoint().to_path_buf());
        Some((handle, mountpoint))
    }

    /// Snapshot of every active mount. Cheap: clones a `HashMap`
    /// of `i64` → `PathBuf` without touching the handle slot.
    pub fn snapshot(&self) -> Vec<(i64, PathBuf)> {
        let guard = self.inner.lock().expect("mount registry poisoned");
        guard
            .mountpoints
            .iter()
            .map(|(k, v)| (*k, v.clone()))
            .collect()
    }
}

/// Tauri-wire mount status row.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MountDto {
    pub job_row_id: i64,
    pub mountpoint: String,
}

// ---------------------------------------------------------------------
// IPC commands
// ---------------------------------------------------------------------

/// Phase 33c — report which mount backend the current build
/// selected. Values: `"fuse"` (Linux/macOS with `--features fuse`),
/// `"winfsp"` (Windows with `--features winfsp`), or `"noop"`
/// (default build — every mount stays logical; kernel callbacks
/// land in Phase 33d).
#[tauri::command]
pub fn mount_backend_name() -> &'static str {
    default_backend_name()
}

/// Enumerate every currently-active mount.
#[tauri::command]
pub fn list_mounts(state: tauri::State<'_, AppState>) -> Vec<MountDto> {
    state
        .mounts
        .snapshot()
        .into_iter()
        .map(|(job_row_id, mountpoint)| MountDto {
            job_row_id,
            mountpoint: mountpoint.to_string_lossy().into_owned(),
        })
        .collect()
}

/// Mount the snapshot for `job_row_id` at `mountpoint`.
/// Idempotent on the user side: mounting an already-mounted job
/// drops the previous handle first.
#[tauri::command]
pub async fn mount_snapshot(
    job_row_id: i64,
    mountpoint: String,
    state: tauri::State<'_, AppState>,
) -> Result<MountDto, String> {
    // Sanity-check the job row exists before we open a handle.
    // Returning a clear error up front saves an orphan unmount if
    // the user picked a non-existent row from a stale history.
    if let Some(history) = state.history.as_ref() {
        let filter = HistoryFilter {
            limit: None,
            ..Default::default()
        };
        let jobs = history
            .search(filter)
            .await
            .map_err(|e| format!("history search failed: {e}"))?;
        if !jobs.iter().any(|j| j.row_id == job_row_id) {
            return Err(format!("history row {job_row_id} not found"));
        }
    }

    let backend = NoopBackend::default();
    let mountpoint_path = PathBuf::from(&mountpoint);
    let handle = backend
        .mount(
            &mountpoint_path,
            MountLayout::all(),
            &copythat_mount::backends::ArchiveRefs::default(),
        )
        .map_err(|e: MountError| e.to_string())?;

    state.mounts.insert(job_row_id, handle);

    Ok(MountDto {
        job_row_id,
        mountpoint,
    })
}

/// Unmount the snapshot for `job_row_id`. Returns typed error when
/// the job isn't currently mounted.
#[tauri::command]
pub fn unmount_snapshot(job_row_id: i64, state: tauri::State<'_, AppState>) -> Result<(), String> {
    let (handle, _path) = state
        .mounts
        .take(job_row_id)
        .ok_or_else(|| format!("job {job_row_id} is not mounted"))?;
    handle.unmount().map_err(|e| e.to_string())?;
    Ok(())
}

/// Runner hook — if `Settings::mount.mount_on_launch` is on and a
/// path is configured, mount the most recent successful job there
/// at startup. Best-effort: failures log to stderr and the rest of
/// the launch proceeds.
pub async fn mount_latest_on_launch(state: &AppState) {
    let settings = state.settings_snapshot();
    if !settings.mount.mount_on_launch {
        return;
    }
    let mountpoint = settings.mount.mount_on_launch_path.clone();
    if mountpoint.is_empty() {
        return;
    }
    let Some(history) = state.history.as_ref() else {
        return;
    };

    let filter = HistoryFilter {
        status: Some("succeeded".into()),
        limit: Some(1),
        ..Default::default()
    };
    let jobs = match history.search(filter).await {
        Ok(j) => j,
        Err(e) => {
            eprintln!("copythat: mount-on-launch history lookup failed: {e}");
            return;
        }
    };
    let Some(latest) = jobs.first() else {
        return;
    };

    let backend = NoopBackend::default();
    let path = PathBuf::from(&mountpoint);
    match backend.mount(
        &path,
        MountLayout::all(),
        &copythat_mount::backends::ArchiveRefs::default(),
    ) {
        Ok(handle) => {
            state.mounts.insert(latest.row_id, handle);
        }
        Err(e) => {
            eprintln!(
                "copythat: mount-on-launch {} -> {} failed: {e}",
                latest.row_id, mountpoint
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_then_snapshot_lists_mountpoint() {
        let registry = MountRegistry::new();
        let backend = NoopBackend::default();
        let tmp = tempfile::tempdir().expect("tempdir");
        let handle = backend
            .mount(
                tmp.path(),
                MountLayout::all(),
                &copythat_mount::backends::ArchiveRefs::default(),
            )
            .expect("mount");
        registry.insert(42, handle);
        let snap = registry.snapshot();
        assert_eq!(snap.len(), 1);
        assert_eq!(snap[0].0, 42);
        assert_eq!(snap[0].1, tmp.path());
    }

    #[test]
    fn insert_same_id_twice_drops_previous_handle() {
        let registry = MountRegistry::new();
        let backend = NoopBackend::default();
        let counter = backend.unmount_counter();
        let tmp1 = tempfile::tempdir().expect("tempdir1");
        let tmp2 = tempfile::tempdir().expect("tempdir2");

        let h1 = backend
            .mount(
                tmp1.path(),
                MountLayout::all(),
                &copythat_mount::backends::ArchiveRefs::default(),
            )
            .expect("mount1");
        registry.insert(7, h1);
        assert_eq!(*counter.lock().unwrap(), 0, "first mount still live");

        let h2 = backend
            .mount(
                tmp2.path(),
                MountLayout::all(),
                &copythat_mount::backends::ArchiveRefs::default(),
            )
            .expect("mount2");
        registry.insert(7, h2);
        // Replacement dropped h1 → unmount_on_drop fired once.
        assert_eq!(*counter.lock().unwrap(), 1);

        // Snapshot reports the new mountpoint.
        let snap = registry.snapshot();
        assert_eq!(snap.len(), 1);
        assert_eq!(snap[0].1, tmp2.path());
    }

    #[test]
    fn take_then_explicit_unmount_cleans_registry() {
        let registry = MountRegistry::new();
        let backend = NoopBackend::default();
        let tmp = tempfile::tempdir().expect("tempdir");
        let handle = backend
            .mount(
                tmp.path(),
                MountLayout::all(),
                &copythat_mount::backends::ArchiveRefs::default(),
            )
            .expect("mount");
        registry.insert(3, handle);
        let (taken, path) = registry.take(3).expect("taken");
        assert_eq!(path, tmp.path());
        taken.unmount().expect("unmount");
        assert!(registry.snapshot().is_empty());
        // Re-take on a missing id returns None.
        assert!(registry.take(3).is_none());
    }
}
