//! Phase 28 — tray-resident Drop Stack.
//!
//! The Drop Stack is a persistent, cross-session list of file paths
//! the user has gathered from various places (Explorer drag, main
//! window context menu, CLI `copythat stack add`, drag onto the
//! Drop Stack window) and can then drag / button-dispatch to a
//! destination in one shot.
//!
//! This module owns:
//!
//! - [`DropStackRegistry`] — the in-memory state, shared via
//!   `AppState::dropstack`. Wrapped in a `RwLock<Inner>` so reads
//!   are lock-free and mutations are serialised.
//! - **JSON persistence** at `<config-dir>/dropstack.json`. Saved
//!   atomically on every mutation (stage to `.tmp` + rename) and
//!   re-loaded on startup.
//! - **Startup revalidation**: paths that no longer resolve are
//!   dropped from the stack and returned as `Vec<PathBuf>` so the
//!   frontend can surface a one-time toast.
//! - IPC commands consumed by `DropStack.svelte` and the main
//!   window's ContextMenu — [`dropstack_add`], [`dropstack_remove`],
//!   [`dropstack_clear`], [`dropstack_list`], [`dropstack_toggle_window`],
//!   [`dropstack_copy_all_to`], [`dropstack_move_all_to`].
//!
//! **Phase 29 native DnD polish:**
//!
//! - In-app drag source: Drop Stack rows are HTML5 draggable; the
//!   browser-level `dragstart` → `dataTransfer.setDragImage` path
//!   paints a themed thumbnail via `applyDragThumbnail`, and the
//!   payload lists the staged paths so any in-app `DropTarget`
//!   component receives them.
//! - Drop targets (both main window and Drop Stack window) rely on
//!   Tauri 2's built-in OS-native drop handling:
//!     - **Windows** — `IDropTarget` via the `dragDropEnabled` flag
//!       in `tauri.conf.json` / `.drag_and_drop(true)` on the
//!       `WebviewWindowBuilder`. Win11's OS-composited drag preview
//!       comes for free when the source app is Win11-aware.
//!     - **macOS** — `NSDraggingDestination` via the same flag; file
//!       URLs arrive rich (with previews) because WebKit bridges the
//!       pasteboard contents directly.
//!     - **Linux** — GTK DnD source/target. No platform-specific
//!       flag needed.
//! - **OS-native drag source to external apps** (dragging a Drop
//!   Stack row into Explorer / Finder / Nautilus) still requires a
//!   native IDataObject / NSPasteboardItem / GTK drag-source bridge.
//!   Tracked as Phase 29b behind the `tauri-plugin-drag` crate; Phase
//!   29 ships the HTML5 drag source so in-window drop targets work
//!   today and the external-app path stays forwards-compatible with
//!   a future plugin attach.

use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager, State, WebviewUrl, WebviewWindowBuilder};

use crate::ipc_safety::{err_string, validate_ipc_path, validate_ipc_paths};
use crate::state::AppState;

/// Event name emitted on every registry mutation. Payload is the
/// current serialised list, i.e. the same shape
/// [`DropStackEntryDto`]s that [`dropstack_list`] returns.
pub const EVENT_DROPSTACK_CHANGED: &str = "dropstack-changed";

/// Event emitted when startup revalidation drops a path from the
/// stack. Payload is [`DropStackPathMissingDto`]. The frontend
/// renders a one-time toast per missing path.
pub const EVENT_DROPSTACK_PATH_MISSING: &str = "dropstack-path-missing";

/// Tauri window label for the Drop Stack window. A single window;
/// `open_or_focus` creates the first time, focuses on subsequent
/// calls.
pub const DROPSTACK_WINDOW_LABEL: &str = "dropstack";

/// Cap on how many paths the stack can hold at once. The window is
/// virtualised so the cap is about "don't blow up the JSON file",
/// not UI responsiveness. 2 000 paths is 200–400 KiB of JSON — fast
/// enough to re-save on every mutation.
const MAX_ENTRIES: usize = 2_000;

/// One path in the stack.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropStackEntry {
    /// Absolute path as captured at add-time. May be a file or a
    /// directory — both are legal stack entries; directories
    /// dispatch as a whole tree via `copy_tree` when a destination
    /// is chosen.
    pub path: PathBuf,
    /// Unix epoch millis when the path was added. Used for the
    /// "Added N seconds ago" per-row subtitle and for stable
    /// ordering when the user hasn't manually re-ordered.
    pub added_at_ms: i64,
}

impl DropStackEntry {
    fn new(path: PathBuf) -> Self {
        Self {
            path,
            added_at_ms: now_ms(),
        }
    }
}

fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

/// On-disk serialisation shape. Version-bumped every time the
/// layout changes so a newer app opening an older file can migrate
/// rather than error.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct OnDisk {
    /// Schema version. `1` for Phase 28.
    version: u32,
    entries: Vec<DropStackEntry>,
}

/// Drop-stack registry. Cheap to clone — the inner state is behind
/// an `Arc<RwLock>`.
#[derive(Debug, Clone)]
pub struct DropStackRegistry {
    inner: Arc<RwLock<Inner>>,
    /// Path to the on-disk JSON. Set once at construction; never
    /// changes. Held as `Arc<PathBuf>` so handlers can reference it
    /// cheaply without another clone.
    path: Arc<PathBuf>,
}

#[derive(Debug, Default)]
struct Inner {
    entries: Vec<DropStackEntry>,
}

impl DropStackRegistry {
    /// Construct an empty registry that persists to `path`. Does no
    /// filesystem I/O — call [`DropStackRegistry::load`] to populate
    /// from disk, or leave empty.
    pub fn new(path: PathBuf) -> Self {
        Self {
            inner: Arc::new(RwLock::new(Inner::default())),
            path: Arc::new(path),
        }
    }

    /// On-disk path the registry persists to.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Load from disk, revalidating every stored path against the
    /// filesystem. Returns the paths that were dropped (because they
    /// no longer resolve) so the caller can fire one toast per.
    ///
    /// A missing file (first launch) is not an error — the registry
    /// is initialised empty.
    pub fn load(&self) -> Result<Vec<PathBuf>, DropStackError> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }
        let raw = std::fs::read_to_string(&*self.path).map_err(|e| DropStackError::Io {
            path: (*self.path).clone(),
            source: e,
        })?;
        let parsed: OnDisk = serde_json::from_str(&raw).map_err(|e| DropStackError::Parse {
            path: (*self.path).clone(),
            message: e.to_string(),
        })?;
        // Refuse to load future-version files: deserialising under
        // the v1 shape when the on-disk shape is v2 could silently
        // discard fields and lose data on the next save. The
        // `version` field on `OnDisk` was previously declared but
        // never inspected — surfacing the mismatch now means a
        // newer build that wrote a v2 file is safely refused by an
        // older build instead of being silently re-encoded as v1.
        const SUPPORTED_VERSION: u32 = 1;
        if parsed.version > SUPPORTED_VERSION {
            return Err(DropStackError::Parse {
                path: (*self.path).clone(),
                message: format!(
                    "dropstack.json on-disk version {} is newer than this build (supports {SUPPORTED_VERSION}); upgrade the app",
                    parsed.version,
                ),
            });
        }

        let mut missing: Vec<PathBuf> = Vec::new();
        let mut kept: Vec<DropStackEntry> = Vec::with_capacity(parsed.entries.len());
        for e in parsed.entries {
            if e.path.exists() {
                kept.push(e);
            } else {
                missing.push(e.path);
            }
        }

        {
            let mut guard = self.inner.write().map_err(|_| DropStackError::Poisoned)?;
            guard.entries = kept;
        }
        // Persist the trimmed list immediately so a later crash
        // doesn't re-present the missing paths on the next launch.
        if !missing.is_empty() {
            self.save()?;
        }
        Ok(missing)
    }

    /// Current entries as a cheap read-locked snapshot clone. The
    /// returned `Vec` is independent of the registry — callers can
    /// iterate without holding a lock.
    pub fn snapshot(&self) -> Vec<DropStackEntry> {
        self.inner
            .read()
            .map(|g| g.entries.clone())
            .unwrap_or_default()
    }

    /// Current entry count. Used by the tray tooltip.
    #[must_use]
    pub fn len(&self) -> usize {
        self.inner.read().map(|g| g.entries.len()).unwrap_or(0)
    }

    /// `true` when no entries are staged.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Add one or more paths. Duplicates (exact path match) are
    /// silently skipped. Paths are canonicalised lightly — only
    /// leading/trailing whitespace is stripped; absolute/relative
    /// resolution is the caller's responsibility (the IPC command
    /// turns OS drag paths into absolute before hand-off).
    ///
    /// Returns the number of *new* entries that actually landed.
    /// Trims to the registry cap if necessary (oldest-first).
    pub fn add(&self, paths: impl IntoIterator<Item = PathBuf>) -> Result<usize, DropStackError> {
        let mut added = 0usize;
        {
            let mut guard = self.inner.write().map_err(|_| DropStackError::Poisoned)?;
            for p in paths {
                // Skip empties + duplicates.
                if p.as_os_str().is_empty() {
                    continue;
                }
                if guard.entries.iter().any(|e| e.path == p) {
                    continue;
                }
                guard.entries.push(DropStackEntry::new(p));
                added += 1;
                // Enforce the cap — oldest-first trimming.
                while guard.entries.len() > MAX_ENTRIES {
                    guard.entries.remove(0);
                }
            }
        }
        if added > 0 {
            self.save()?;
        }
        Ok(added)
    }

    /// Remove a single entry by exact-path match. Returns `true` if
    /// the path was present.
    pub fn remove(&self, path: &Path) -> Result<bool, DropStackError> {
        let removed = {
            let mut guard = self.inner.write().map_err(|_| DropStackError::Poisoned)?;
            let before = guard.entries.len();
            guard.entries.retain(|e| e.path != path);
            before != guard.entries.len()
        };
        if removed {
            self.save()?;
        }
        Ok(removed)
    }

    /// Drop every entry. Always persists (so the JSON file reflects
    /// the cleared state even if it was already empty — harmless).
    pub fn clear(&self) -> Result<(), DropStackError> {
        {
            let mut guard = self.inner.write().map_err(|_| DropStackError::Poisoned)?;
            guard.entries.clear();
        }
        self.save()
    }

    /// Persist to disk atomically: write to `<path>.tmp`, then
    /// rename over. Never partial-writes the canonical file.
    pub fn save(&self) -> Result<(), DropStackError> {
        if let Some(parent) = self.path.parent()
            && !parent.as_os_str().is_empty()
        {
            std::fs::create_dir_all(parent).map_err(|e| DropStackError::Io {
                path: parent.to_path_buf(),
                source: e,
            })?;
        }
        let entries = self.snapshot();
        let on_disk = OnDisk {
            version: 1,
            entries,
        };
        let json = serde_json::to_string_pretty(&on_disk).map_err(|e| DropStackError::Parse {
            path: (*self.path).clone(),
            message: e.to_string(),
        })?;
        let tmp = self.path.with_extension("json.tmp");
        std::fs::write(&tmp, json).map_err(|e| DropStackError::Io {
            path: tmp.clone(),
            source: e,
        })?;
        std::fs::rename(&tmp, &*self.path).map_err(|e| {
            let _ = std::fs::remove_file(&tmp);
            DropStackError::Io {
                path: (*self.path).clone(),
                source: e,
            }
        })?;
        Ok(())
    }
}

/// Every error surface from the registry.
#[derive(Debug, thiserror::Error)]
pub enum DropStackError {
    #[error("I/O error at {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("parse error at {path}: {message}")]
    Parse { path: PathBuf, message: String },
    #[error("internal lock poisoned")]
    Poisoned,
}

impl serde::Serialize for DropStackError {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.to_string())
    }
}

// ---------------------------------------------------------------------
// IPC DTOs
// ---------------------------------------------------------------------

/// Wire shape of a single entry. Same fields as [`DropStackEntry`],
/// camelCase for idiomatic TypeScript.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DropStackEntryDto {
    pub path: String,
    pub added_at_ms: i64,
}

impl From<&DropStackEntry> for DropStackEntryDto {
    fn from(e: &DropStackEntry) -> Self {
        Self {
            path: e.path.to_string_lossy().into_owned(),
            added_at_ms: e.added_at_ms,
        }
    }
}

/// Payload for [`EVENT_DROPSTACK_PATH_MISSING`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DropStackPathMissingDto {
    pub path: String,
}

fn emit_changed(app: &AppHandle, registry: &DropStackRegistry) {
    let payload: Vec<DropStackEntryDto> = registry.snapshot().iter().map(Into::into).collect();
    let _ = app.emit(EVENT_DROPSTACK_CHANGED, payload);
}

// ---------------------------------------------------------------------
// Tauri commands
// ---------------------------------------------------------------------

/// Resolve the default on-disk path for the Drop Stack JSON. Sits
/// next to `settings.toml` under the OS config dir.
pub fn default_dropstack_path() -> Option<PathBuf> {
    directories::ProjectDirs::from("com", "CopyThat", "CopyThat2026")
        .map(|d| d.config_dir().join("dropstack.json"))
}

/// Add one or more paths to the stack. Idempotent on duplicates.
#[tauri::command]
pub async fn dropstack_add(
    app: AppHandle,
    state: State<'_, AppState>,
    paths: Vec<String>,
) -> Result<usize, String> {
    let registry = state.dropstack.clone();
    // Phase 17e — gate every path through the IPC validator before
    // it reaches the persistent JSON store. Anything tainted (`..`,
    // NUL, U+FFFD lossy WTF-16) would otherwise live in
    // `dropstack.json` indefinitely and inherit any future
    // pre-dispatch reader that forgets to re-validate.
    let path_bufs: Vec<PathBuf> = validate_ipc_paths(&paths).map_err(err_string)?;
    let added = registry.add(path_bufs).map_err(|e| e.to_string())?;
    if added > 0 {
        emit_changed(&app, &registry);
    }
    Ok(added)
}

/// Remove a single path by exact match.
#[tauri::command]
pub async fn dropstack_remove(
    app: AppHandle,
    state: State<'_, AppState>,
    path: String,
) -> Result<bool, String> {
    let registry = state.dropstack.clone();
    let removed = registry
        .remove(Path::new(&path))
        .map_err(|e| e.to_string())?;
    if removed {
        emit_changed(&app, &registry);
    }
    Ok(removed)
}

/// Drop every staged path.
#[tauri::command]
pub async fn dropstack_clear(app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    let registry = state.dropstack.clone();
    registry.clear().map_err(|e| e.to_string())?;
    emit_changed(&app, &registry);
    Ok(())
}

/// List current entries. Returns oldest-first (insertion order).
#[tauri::command]
pub async fn dropstack_list(state: State<'_, AppState>) -> Result<Vec<DropStackEntryDto>, String> {
    let snap = state.dropstack.snapshot();
    Ok(snap.iter().map(Into::into).collect())
}

/// Open (or focus) the Drop Stack window. Creates it lazily the
/// first time — subsequent calls just raise + focus.
#[tauri::command]
pub async fn dropstack_toggle_window(app: AppHandle) -> Result<(), String> {
    if let Some(win) = app.get_webview_window(DROPSTACK_WINDOW_LABEL) {
        let visible = win.is_visible().unwrap_or(false);
        if visible {
            let _ = win.hide();
        } else {
            let _ = win.show();
            let _ = win.set_focus();
        }
        return Ok(());
    }
    let url = WebviewUrl::App("dropstack.html".into());
    WebviewWindowBuilder::new(&app, DROPSTACK_WINDOW_LABEL, url)
        .title("Drop Stack — Copy That")
        .inner_size(380.0, 520.0)
        .min_inner_size(320.0, 240.0)
        .resizable(true)
        .always_on_top(
            app.try_state::<AppState>()
                .map(|s| s.settings_snapshot().drop_stack.always_on_top)
                .unwrap_or(false),
        )
        .skip_taskbar(false)
        // Explorer drag onto this window delivers paths via the
        // Tauri window-level `drag-drop` event; the Svelte layer
        // feeds them into `dropstack_add`.
        .drag_and_drop(true)
        .build()
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Copy every staged path to `destination` and clear the stack on
/// success. Returns the list of paths that were enqueued. Uses the
/// same `shell::enqueue_jobs` entry point as `start_copy` so
/// collision / conflict / journal behaviour is identical.
#[tauri::command]
pub async fn dropstack_copy_all_to(
    app: AppHandle,
    state: State<'_, AppState>,
    destination: String,
) -> Result<Vec<String>, String> {
    dispatch_all(app, state, destination, copythat_core::JobKind::Copy).await
}

/// Move every staged path to `destination`. Same semantics as
/// `dropstack_copy_all_to` but using the move kind.
#[tauri::command]
pub async fn dropstack_move_all_to(
    app: AppHandle,
    state: State<'_, AppState>,
    destination: String,
) -> Result<Vec<String>, String> {
    dispatch_all(app, state, destination, copythat_core::JobKind::Move).await
}

async fn dispatch_all(
    app: AppHandle,
    state: State<'_, AppState>,
    destination: String,
    kind: copythat_core::JobKind,
) -> Result<Vec<String>, String> {
    let snap = state.dropstack.snapshot();
    let paths: Vec<String> = snap
        .iter()
        .map(|e| e.path.to_string_lossy().into_owned())
        .collect();
    if paths.is_empty() {
        return Ok(paths);
    }
    // Phase 17e — same IPC gate `enqueue` uses; closes the U+FFFD
    // bypass that bare `validate_path_no_traversal` misses.
    let dst_root = validate_ipc_path(&destination).map_err(err_string)?;

    // Defaults inherited from live Settings so behaviour matches
    // a drag-drop from the main window. No per-enqueue overrides
    // — the user already gathered the files deliberately.
    let settings = state.settings_snapshot();
    let copy_opts = copythat_core::CopyOptions {
        buffer_size: settings.transfer.effective_buffer_size(),
        verify: None,
        ..Default::default()
    };
    let verifier = None;
    let collision_policy = copythat_core::CollisionPolicy::Prompt;
    let error_policy = copythat_core::ErrorPolicy::default();

    let srcs: Vec<PathBuf> = paths.iter().map(PathBuf::from).collect();
    for src in &srcs {
        if let Err(e) = copythat_core::validate_path_no_traversal(src) {
            return Err(e.localized_key().to_string());
        }
    }

    let ids = crate::shell::enqueue_jobs(
        &app,
        state.inner(),
        kind,
        srcs,
        &dst_root,
        copy_opts,
        verifier,
        collision_policy,
        error_policy,
        None,
        None,
    );
    // Clear only if we actually enqueued anything; if `ids` came
    // back empty the user gave us an empty list after filtering.
    if !ids.is_empty() {
        state.dropstack.clear().map_err(|e| e.to_string())?;
        emit_changed(&app, &state.dropstack);
    }
    Ok(paths)
}

/// Announce a startup-time "path no longer resolves" drop. Called
/// once per missing path by `lib.rs::run` after the registry's
/// initial `load` returns.
pub fn emit_path_missing(app: &AppHandle, path: &Path) {
    let _ = app.emit(
        EVENT_DROPSTACK_PATH_MISSING,
        DropStackPathMissingDto {
            path: path.to_string_lossy().into_owned(),
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_registry_is_empty() {
        let tmp = tempfile::tempdir().unwrap();
        let reg = DropStackRegistry::new(tmp.path().join("ds.json"));
        assert!(reg.is_empty());
        assert_eq!(reg.snapshot().len(), 0);
    }

    #[test]
    fn add_then_save_and_reload_preserves_entries() {
        let tmp = tempfile::tempdir().unwrap();
        // Stage real files so revalidation on load keeps them.
        let a = tmp.path().join("a.txt");
        let b = tmp.path().join("b.txt");
        std::fs::write(&a, "A").unwrap();
        std::fs::write(&b, "B").unwrap();

        let reg = DropStackRegistry::new(tmp.path().join("ds.json"));
        reg.add([a.clone(), b.clone()]).unwrap();
        assert_eq!(reg.len(), 2);

        // Fresh registry pointed at the same file; load should bring
        // both paths back.
        let reg2 = DropStackRegistry::new(tmp.path().join("ds.json"));
        let missing = reg2.load().unwrap();
        assert!(missing.is_empty());
        let snap = reg2.snapshot();
        assert_eq!(snap.len(), 2);
        assert!(snap.iter().any(|e| e.path == a));
        assert!(snap.iter().any(|e| e.path == b));
    }

    #[test]
    fn reload_drops_missing_paths_and_returns_them() {
        let tmp = tempfile::tempdir().unwrap();
        let kept = tmp.path().join("kept.txt");
        let deleted = tmp.path().join("deleted.txt");
        std::fs::write(&kept, "K").unwrap();
        std::fs::write(&deleted, "D").unwrap();

        let reg = DropStackRegistry::new(tmp.path().join("ds.json"));
        reg.add([kept.clone(), deleted.clone()]).unwrap();
        std::fs::remove_file(&deleted).unwrap();

        let reg2 = DropStackRegistry::new(tmp.path().join("ds.json"));
        let missing = reg2.load().unwrap();
        assert_eq!(missing, vec![deleted.clone()]);
        assert_eq!(reg2.len(), 1);
        assert_eq!(reg2.snapshot()[0].path, kept);

        // The trimmed list is written back so a second cold open
        // does not re-present the missing path.
        let reg3 = DropStackRegistry::new(tmp.path().join("ds.json"));
        let missing3 = reg3.load().unwrap();
        assert!(missing3.is_empty());
        assert_eq!(reg3.len(), 1);
    }

    #[test]
    fn duplicates_are_skipped() {
        let tmp = tempfile::tempdir().unwrap();
        let p = tmp.path().join("f.txt");
        std::fs::write(&p, "x").unwrap();
        let reg = DropStackRegistry::new(tmp.path().join("ds.json"));
        let first = reg.add([p.clone()]).unwrap();
        let second = reg.add([p.clone()]).unwrap();
        assert_eq!(first, 1);
        assert_eq!(second, 0);
        assert_eq!(reg.len(), 1);
    }

    #[test]
    fn remove_and_clear_work() {
        let tmp = tempfile::tempdir().unwrap();
        let a = tmp.path().join("a.txt");
        let b = tmp.path().join("b.txt");
        std::fs::write(&a, "A").unwrap();
        std::fs::write(&b, "B").unwrap();
        let reg = DropStackRegistry::new(tmp.path().join("ds.json"));
        reg.add([a.clone(), b.clone()]).unwrap();
        assert!(reg.remove(&a).unwrap());
        assert!(!reg.remove(&a).unwrap(), "second remove is idempotent");
        assert_eq!(reg.len(), 1);
        reg.clear().unwrap();
        assert!(reg.is_empty());
    }

    #[test]
    fn cap_trims_oldest_first() {
        let tmp = tempfile::tempdir().unwrap();
        let reg = DropStackRegistry::new(tmp.path().join("ds.json"));
        // Sidestep revalidation by skipping load(); we just want
        // the in-memory cap behaviour.
        let mut adds: Vec<PathBuf> = Vec::new();
        for i in 0..(MAX_ENTRIES + 5) {
            adds.push(PathBuf::from(format!("/tmp/virt/{i}")));
        }
        reg.add(adds).unwrap();
        assert_eq!(reg.len(), MAX_ENTRIES);
        // First 5 entries should have been trimmed.
        let snap = reg.snapshot();
        assert_eq!(
            snap.first().unwrap().path,
            PathBuf::from("/tmp/virt/5"),
            "oldest 5 entries trimmed"
        );
    }

    #[test]
    fn load_missing_file_is_empty_ok() {
        let tmp = tempfile::tempdir().unwrap();
        let reg = DropStackRegistry::new(tmp.path().join("does-not-exist.json"));
        let missing = reg.load().unwrap();
        assert!(missing.is_empty());
        assert!(reg.is_empty());
    }
}
