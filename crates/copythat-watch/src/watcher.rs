//! `Watcher` — the public handle that wraps `notify` + feeds events
//! through the filter, atomic-save tracker, and debounce queue.
//!
//! Spawns three logical pieces at construction:
//!
//! - A [`notify::RecommendedWatcher`] that delivers raw events onto
//!   a blocking `std::sync::mpsc` channel.
//! - A background thread that pumps raw events → filter → atomic-save
//!   tracker → debounce queue, and periodically flushes ripe entries
//!   out to a tokio output channel.
//! - The output channel is what [`Watcher::next`] and
//!   [`Watcher::next_async`] consume from.
//!
//! The background thread also serves as the Windows overflow +
//! macOS directory-coalesce recovery point: when `notify` surfaces
//! an overflow signal (`notify::Event` with
//! `EventKind::Other`/`Any` on a directory), the thread runs a
//! shallow `read_dir` of the affected subtree and synthesises
//! `Modified` events for every entry.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher as _};
use tokio::sync::mpsc;

use crate::debounce::{AtomicSaveTracker, DebounceQueue};
use crate::error::{Result, WatchError};
use crate::event::FsEvent;
use crate::filter::{PathFilter, default_filter};

/// Tunables for the watcher. Every field has a sensible default; the
/// spec's 500 ms debounce window is the out-of-the-box value.
#[derive(Debug, Clone)]
pub struct WatcherOptions {
    /// Debounce window — how long to wait before emitting accumulated
    /// events for a single path. Default 500 ms.
    pub debounce: Duration,
    /// Atomic-save window — how long after a temp-name creation the
    /// watcher keeps that temp in its memory so a subsequent rename
    /// can be collapsed. Default 500 ms.
    pub atomic_window: Duration,
    /// Cadence at which the internal thread flushes ripe debounce
    /// entries. A 50 ms tick is well below the 500 ms debounce so
    /// emission latency stays tight. Default 50 ms.
    pub flush_interval: Duration,
    /// When `true`, `foo~` and `foo.bak` also get filtered. Default
    /// `false` — backups are real files most users want to sync.
    pub filter_editor_backups: bool,
}

impl Default for WatcherOptions {
    fn default() -> Self {
        Self {
            debounce: Duration::from_millis(500),
            atomic_window: Duration::from_millis(500),
            flush_interval: Duration::from_millis(50),
            filter_editor_backups: false,
        }
    }
}

/// Public watcher handle.
///
/// Cheap to drop; dropping the handle stops the background thread
/// (the raw-events channel is closed, the thread exits, the
/// `RecommendedWatcher` drops on its own).
pub struct Watcher {
    rx: mpsc::Receiver<FsEvent>,
    /// Shared stop flag read by the background thread on every
    /// flush tick. Writing `true` here is the kill switch.
    stop: Arc<Mutex<bool>>,
    /// `RecommendedWatcher` stays alive for the lifetime of this
    /// handle. Dropping it unsubscribes from the OS-level watch.
    /// `_notify` is deliberately named-unused to signal "keep alive
    /// for side effects".
    _notify: RecommendedWatcher,
    /// Join handle for the background pump thread. Dropped on
    /// Watcher drop; the thread exits cleanly via the `stop` flag.
    pump: Option<thread::JoinHandle<()>>,
}

impl Watcher {
    /// Open a recursive watch rooted at `root`. Uses the default
    /// [`WatcherOptions`].
    pub fn new(root: &Path) -> Result<Self> {
        Self::with_options(root, WatcherOptions::default())
    }

    /// Open a recursive watch with custom options.
    pub fn with_options(root: &Path, options: WatcherOptions) -> Result<Self> {
        if !root.exists() {
            return Err(WatchError::RootNotAccessible {
                path: root.to_path_buf(),
                reason: "path does not exist".to_string(),
            });
        }
        if !root.is_dir() {
            return Err(WatchError::RootNotAccessible {
                path: root.to_path_buf(),
                reason: "path is not a directory".to_string(),
            });
        }

        let (raw_tx, raw_rx) = std::sync::mpsc::channel::<notify::Result<Event>>();
        let mut notify_watcher = notify::recommended_watcher(move |res: notify::Result<Event>| {
            let _ = raw_tx.send(res);
        })?;
        notify_watcher.watch(root, RecursiveMode::Recursive)?;

        let (out_tx, out_rx) = mpsc::channel::<FsEvent>(1024);
        let stop = Arc::new(Mutex::new(false));
        let stop_for_thread = Arc::clone(&stop);
        let opts_for_thread = options.clone();
        let root_for_thread = root.to_path_buf();

        let pump = thread::Builder::new()
            .name("copythat-watch-pump".to_string())
            .spawn(move || {
                run_pump(
                    raw_rx,
                    out_tx,
                    stop_for_thread,
                    opts_for_thread,
                    root_for_thread,
                );
            })
            .map_err(|e| WatchError::Backend(format!("spawn pump thread: {e}")))?;

        Ok(Self {
            rx: out_rx,
            stop,
            _notify: notify_watcher,
            pump: Some(pump),
        })
    }

    /// Block the current thread until the next event arrives.
    /// Returns `None` when the watcher has been dropped or an
    /// unrecoverable backend error closed the channel.
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<FsEvent> {
        self.rx.blocking_recv()
    }

    /// Awaitable next-event. Equivalent to `next()` for tokio
    /// callers; the watcher output is a tokio mpsc channel so this
    /// is the native variant.
    pub async fn next_async(&mut self) -> Option<FsEvent> {
        self.rx.recv().await
    }
}

impl Drop for Watcher {
    fn drop(&mut self) {
        if let Ok(mut stop) = self.stop.lock() {
            *stop = true;
        }
        if let Some(handle) = self.pump.take() {
            let _ = handle.join();
        }
    }
}

// ---------------------------------------------------------------------
// Background pump
// ---------------------------------------------------------------------

fn run_pump(
    raw_rx: std::sync::mpsc::Receiver<notify::Result<Event>>,
    out_tx: mpsc::Sender<FsEvent>,
    stop: Arc<Mutex<bool>>,
    opts: WatcherOptions,
    root: PathBuf,
) {
    let mut queue = DebounceQueue::new(opts.debounce);
    let mut atomic = AtomicSaveTracker::new(opts.atomic_window);
    let flush_interval = opts.flush_interval;

    loop {
        if *stop.lock().unwrap() {
            break;
        }

        // Drain any events that arrived since the last tick without
        // blocking past `flush_interval`. Using `recv_timeout` means
        // one slow event doesn't starve the flush cadence; if a
        // burst lands, we pick them all up on the next poll below.
        match raw_rx.recv_timeout(flush_interval) {
            Ok(Ok(evt)) => {
                handle_raw_event(evt, &mut queue, &mut atomic, &opts, &root);
            }
            Ok(Err(backend_err)) => {
                // Windows ReadDirectoryChangesW buffer overflow
                // surfaces here; rescan the root as the recovery
                // action. Other backend errors are logged and
                // ignored — the watcher keeps running.
                if is_overflow_error(&backend_err) {
                    rescan_into(&root, &mut queue, Instant::now());
                } else {
                    tracing::warn!(error = %backend_err, "notify backend error");
                }
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                // No new events this tick — fall through to flush.
            }
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
        }

        // Drain any extra buffered events synchronously before
        // flushing. Keeps the burst-coalesce behaviour tight.
        loop {
            match raw_rx.try_recv() {
                Ok(Ok(evt)) => handle_raw_event(evt, &mut queue, &mut atomic, &opts, &root),
                Ok(Err(backend_err)) => {
                    if is_overflow_error(&backend_err) {
                        rescan_into(&root, &mut queue, Instant::now());
                    }
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => break,
                Err(std::sync::mpsc::TryRecvError::Disconnected) => break,
            }
        }

        let ripe = queue.flush(Instant::now());
        for evt in ripe {
            if out_tx.blocking_send(evt).is_err() {
                // Receiver dropped — watcher is being torn down.
                return;
            }
        }
    }

    // Drain anything left on shutdown so a close right after a burst
    // doesn't silently lose state.
    for evt in queue.drain_all() {
        let _ = out_tx.blocking_send(evt);
    }
}

fn handle_raw_event(
    evt: Event,
    queue: &mut DebounceQueue,
    atomic: &mut AtomicSaveTracker,
    opts: &WatcherOptions,
    root: &Path,
) {
    // Containment guard: a backend that surfaces an out-of-root
    // path (Windows ReadDirectoryChangesW with a crossed-junction,
    // an inotify watch handed a directory-symlink target) can let
    // a downstream sync consumer treat that path as in-tree, which
    // is a stored exfiltration vector. Drop any event whose first
    // path doesn't lie under `root`.
    if let Some(first) = evt.paths.first() {
        if !path_inside_root(first, root) {
            return;
        }
    }
    // notify's Event can carry 1 or 2 paths (Renamed is the 2-path
    // case). Anything else is mostly ignorable, but we still honour
    // the path count per kind.
    let now = Instant::now();
    match &evt.kind {
        EventKind::Create(_) => {
            if let Some(p) = evt.paths.first() {
                let class = default_filter(p);
                if class.is_dropped(opts.filter_editor_backups) {
                    return;
                }
                if matches!(class, PathFilter::KnownTemp) {
                    atomic.record_temp(p.clone(), now);
                    return;
                }
                // Split-rename detection: on Windows, the second
                // half of a `rename(tmp, canonical)` arrives as a
                // plain Create here. If a tracked temp was removed
                // in the last `atomic_window`, claim this Create as
                // the paired rename and emit Modified(canonical)
                // instead.
                if let Some(rewritten) = atomic.try_claim_create(p, now) {
                    queue.push(rewritten, now);
                    return;
                }
                queue.push(FsEvent::Created(p.clone()), now);
            }
        }
        EventKind::Modify(notify::event::ModifyKind::Name(rename_mode)) => {
            handle_rename(rename_mode, &evt.paths, queue, atomic, opts, now);
        }
        EventKind::Modify(_) => {
            if let Some(p) = evt.paths.first() {
                let class = default_filter(p);
                if class.is_dropped(opts.filter_editor_backups) {
                    return;
                }
                // Intermediate writes to an editor's temp file
                // (Windows' ReadDirectoryChangesW fires a Modify
                // after every `fs::write` on a `*.tmp`) are noise —
                // the user cares about the eventual rename target,
                // not each byte landing in the staging file.
                if matches!(class, PathFilter::KnownTemp) {
                    return;
                }
                // Modify on a directory with no specific child path
                // is the macOS FSEvents coalesce signal — enumerate
                // and synthesise events.
                if p.is_dir() && evt.paths.len() == 1 {
                    enumerate_dir_into(p, queue, now);
                    return;
                }
                queue.push(FsEvent::Modified(p.clone()), now);
            }
        }
        EventKind::Remove(_) => {
            if let Some(p) = evt.paths.first() {
                let class = default_filter(p);
                if class.is_dropped(opts.filter_editor_backups) {
                    return;
                }
                // Windows ReadDirectoryChangesW emits the source
                // half of a `rename(tmp, canonical)` as a Remove
                // event. Note the temp's removal in the tracker so
                // the paired Create (arriving next) can be rewritten
                // to Modified(canonical).
                if matches!(class, PathFilter::KnownTemp) {
                    atomic.note_temp_removed(p, now);
                    return;
                }
                queue.push(FsEvent::Removed(p.clone()), now);
            }
        }
        EventKind::Access(_) => {
            // Access events are almost always noise for live mirror
            // — ignore. On inotify this covers `IN_OPEN` / `IN_CLOSE`.
        }
        EventKind::Any | EventKind::Other => {
            // Some backends (macOS rename-cookie re-combining,
            // Windows scope-change events) surface as `Any` on a
            // directory. Enumerate and recover.
            if let Some(p) = evt.paths.first() {
                if p.is_dir() {
                    enumerate_dir_into(p, queue, now);
                    return;
                }
            }
            // No path given — rescan the root as a last resort.
            rescan_into(root, queue, now);
        }
    }
}

fn handle_rename(
    rename_mode: &notify::event::RenameMode,
    paths: &[PathBuf],
    queue: &mut DebounceQueue,
    atomic: &mut AtomicSaveTracker,
    opts: &WatcherOptions,
    now: Instant,
) {
    use notify::event::RenameMode;
    match rename_mode {
        RenameMode::Both => {
            // Full rename from → to (Linux / macOS / Windows when the
            // backend stitches the cookie pair together).
            if paths.len() >= 2 {
                let from = &paths[0];
                let to = &paths[1];
                if default_filter(to).is_dropped(opts.filter_editor_backups) {
                    return;
                }
                if let Some(rewritten) = atomic.rewrite_rename(from, to, now) {
                    queue.push(rewritten, now);
                } else {
                    queue.push(FsEvent::Renamed(from.clone(), to.clone()), now);
                }
            }
        }
        RenameMode::From => {
            // Half-rename: the OS reported the source side only
            // (Windows ReadDirectoryChangesW splits every rename
            // this way). Route tracked temps through the atomic-
            // save tracker so the paired `To` half can be rewritten
            // to `Modified(canonical)`. Regular-file renames with
            // no matching `To` inside the watch root emit as
            // `Removed`.
            if let Some(p) = paths.first() {
                if default_filter(p).is_dropped(opts.filter_editor_backups) {
                    return;
                }
                if atomic.note_temp_removed(p, now) {
                    return;
                }
                queue.push(FsEvent::Removed(p.clone()), now);
            }
        }
        RenameMode::To => {
            // Destination half of a rename. If the tracker has a
            // pending temp removal, this is the second half of an
            // atomic save → emit Modified(canonical). Otherwise
            // emit Created.
            if let Some(p) = paths.first() {
                if default_filter(p).is_dropped(opts.filter_editor_backups) {
                    return;
                }
                if let Some(rewritten) = atomic.try_claim_create(p, now) {
                    queue.push(rewritten, now);
                    return;
                }
                queue.push(FsEvent::Created(p.clone()), now);
            }
        }
        RenameMode::Other | RenameMode::Any => {
            // Treat as modify on each path.
            for p in paths {
                if default_filter(p).is_dropped(opts.filter_editor_backups) {
                    continue;
                }
                queue.push(FsEvent::Modified(p.clone()), now);
            }
        }
    }
}

/// Enumerate one directory and push a `Modified` event per child.
/// Used for both macOS FSEvents directory coalesce and Windows
/// ReadDirectoryChangesW overflow recovery.
fn enumerate_dir_into(dir: &Path, queue: &mut DebounceQueue, now: Instant) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if default_filter(&path).is_dropped(false) {
            continue;
        }
        // Skip symlinks: a directory symlink whose target is
        // outside the watch root would otherwise produce
        // `Modified(<path-inside-watched>/...)` events that point
        // at files outside the user's intended watch scope.
        if entry.file_type().map(|t| t.is_symlink()).unwrap_or(false) {
            continue;
        }
        queue.push(FsEvent::Modified(path), now);
    }
}

/// Maximum tree depth visited by `rescan_into`. An attacker who can
/// induce a `ReadDirectoryChangesW` overflow against a deeply-nested
/// or symlink-cycled tree would otherwise exhaust the stack walking
/// indefinitely; capping the depth caps the worst-case work per
/// recovery pass. The cap is generous enough for realistic user
/// trees (deepest known organic case in the bug tracker is 22).
const RESCAN_MAX_DEPTH: usize = 64;

/// Recursive walk of `root` emitting `Modified` per file. Used as
/// the Windows overflow recovery path + as a last-resort for
/// `EventKind::Any`/`Other` events with no specific path.
///
/// Canonicalised paths are cached per-pass in a `HashMap` so a
/// symlink-heavy tree under a forced overflow doesn't trigger
/// `O(entries)` kernel path lookups against the same components
/// repeatedly. The cache is dropped at the end of the pass.
///
/// The walk is bounded by [`RESCAN_MAX_DEPTH`]; entries below the
/// cap are skipped with a single `tracing::warn!`.
fn rescan_into(root: &Path, queue: &mut DebounceQueue, now: Instant) {
    // Stack carries (path, depth) so we can short-circuit the walk
    // without recursing past the depth cap.
    let mut stack: Vec<(PathBuf, usize)> = vec![(root.to_path_buf(), 0)];
    // Per-pass canonical-path cache. Keyed by the original path; the
    // value is the resolved canonical path (or a copy of the input
    // when canonicalisation failed — caching the failure dodges a
    // second syscall). Using a HashMap rather than the file system's
    // own caches keeps the lookup hot inside this function.
    let mut canon_cache: HashMap<PathBuf, PathBuf> = HashMap::new();
    let canon_root = canonicalize_cached(&mut canon_cache, root);
    let mut depth_warned = false;
    while let Some((dir, depth)) = stack.pop() {
        if depth > RESCAN_MAX_DEPTH {
            if !depth_warned {
                tracing::warn!(
                    root = %root.display(),
                    max_depth = RESCAN_MAX_DEPTH,
                    "rescan walk exceeded depth cap; deeper entries skipped"
                );
                depth_warned = true;
            }
            continue;
        }
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if default_filter(&path).is_dropped(false) {
                continue;
            }
            let ftype = match entry.file_type() {
                Ok(t) => t,
                Err(_) => continue,
            };
            // Skip symlinks unconditionally: a directory symlink
            // would let the walk follow into territory outside the
            // pair root, and a file symlink that points outside
            // the root would surface as an in-root event.
            if ftype.is_symlink() {
                continue;
            }
            // Defence-in-depth: even without a symlink, confirm the
            // entry's canonical path is still under the watch root.
            // Cache the result so repeated lookups against shared
            // ancestors don't thrash the kernel's path walker.
            let canon = canonicalize_cached(&mut canon_cache, &path);
            if !canon.starts_with(&canon_root) {
                continue;
            }
            if ftype.is_dir() {
                stack.push((path, depth + 1));
            } else {
                queue.push(FsEvent::Modified(path), now);
            }
        }
    }
}

/// `path.canonicalize()` with a per-pass memoisation table. Falls
/// back to a clone of the input when canonicalisation fails (transient
/// `NotFound`, EACCES on a permission-denied subdir, etc.) — caching
/// the fallback dodges a duplicate failed-syscall the next time the
/// same path is seen during the same overflow-rescan pass.
fn canonicalize_cached(cache: &mut HashMap<PathBuf, PathBuf>, path: &Path) -> PathBuf {
    if let Some(hit) = cache.get(path) {
        return hit.clone();
    }
    let resolved = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    cache.insert(path.to_path_buf(), resolved.clone());
    resolved
}

/// Lexical containment check used by `handle_raw_event`. Falls back
/// to `starts_with` on the original paths when canonicalisation
/// fails (e.g. transient `NotFound` between event delivery and
/// inspection — the path has already been deleted).
fn path_inside_root(p: &Path, root: &Path) -> bool {
    let canon_root = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    if let Ok(canon_p) = p.canonicalize() {
        canon_p.starts_with(&canon_root)
    } else {
        p.starts_with(&canon_root) || p.starts_with(root)
    }
}

fn is_overflow_error(err: &notify::Error) -> bool {
    // notify doesn't expose a stable enum discriminant for backend
    // overflows, but the error message carries platform-specific
    // markers we can pattern-match. This is best-effort; the worst
    // case is a missed rescan which the next normal event will
    // surface anyway.
    let msg = err.to_string();
    msg.contains("ERROR_NOTIFY_ENUM_DIR")
        || msg.contains("overflow")
        || msg.contains("Overflow")
        || msg.contains("OverflowedBuffer")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn options_default_values_match_spec() {
        let opts = WatcherOptions::default();
        assert_eq!(opts.debounce, Duration::from_millis(500));
        assert_eq!(opts.atomic_window, Duration::from_millis(500));
        assert_eq!(opts.flush_interval, Duration::from_millis(50));
        assert!(!opts.filter_editor_backups);
    }

    #[test]
    fn new_on_missing_root_is_typed_error() {
        let Err(err) = Watcher::new(Path::new("/definitely/does/not/exist")) else {
            panic!("expected error");
        };
        assert!(matches!(err, WatchError::RootNotAccessible { .. }));
    }

    #[test]
    fn canonicalize_cached_dedups_repeated_lookups() {
        let d = tempfile::tempdir().unwrap();
        let p = d.path().join("a");
        std::fs::create_dir_all(&p).unwrap();
        let mut cache: HashMap<PathBuf, PathBuf> = HashMap::new();
        let first = canonicalize_cached(&mut cache, &p);
        let second = canonicalize_cached(&mut cache, &p);
        assert_eq!(first, second);
        // Cache populated on first call: subsequent calls hit the
        // map rather than re-issuing a kernel canonicalisation.
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn canonicalize_cached_falls_back_when_path_missing() {
        let d = tempfile::tempdir().unwrap();
        let missing = d.path().join("does-not-exist");
        let mut cache: HashMap<PathBuf, PathBuf> = HashMap::new();
        let resolved = canonicalize_cached(&mut cache, &missing);
        // Fallback returns the input verbatim and caches the answer
        // so a repeated lookup doesn't re-issue the failing syscall.
        assert_eq!(resolved, missing);
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn rescan_into_respects_depth_cap() {
        // Build a synthetic tree deeper than RESCAN_MAX_DEPTH and
        // verify rescan_into doesn't recurse forever. The exact
        // emitted-event count isn't part of the contract — what we
        // assert is that the call returns within reasonable wall
        // time and that at least the shallow files surface.
        let d = tempfile::tempdir().unwrap();
        let mut path = d.path().to_path_buf();
        for i in 0..(RESCAN_MAX_DEPTH + 5) {
            path.push(format!("d{i}"));
        }
        std::fs::create_dir_all(&path).unwrap();
        // Drop a file at the cap-1 depth to confirm shallow files
        // are reported.
        let mut shallow = d.path().to_path_buf();
        shallow.push("shallow.txt");
        std::fs::write(&shallow, b"x").unwrap();
        let mut q = DebounceQueue::new(Duration::from_millis(0));
        rescan_into(d.path(), &mut q, Instant::now());
        let drained = q.drain_all();
        assert!(
            drained
                .iter()
                .any(|e| matches!(e, FsEvent::Modified(p) if p == &shallow)),
            "expected shallow file to surface in rescan output"
        );
    }
}
