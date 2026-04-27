//! Per-path debouncer with priority ordering + atomic-save collapse.
//!
//! The debouncer is a pure state machine — no threads, no I/O, no
//! timers. The [`crate::Watcher`] is responsible for feeding it events and
//! calling [`DebounceQueue::flush`] on a wall-clock tick. That split
//! keeps the logic fully deterministic for unit tests: seed a queue,
//! feed a canned event sequence, flush at a controllable instant,
//! assert the emitted list.
//!
//! # Priority
//!
//! When a single path accumulates multiple events within the
//! debounce window, the queue keeps only the highest-priority one:
//!
//! ```text
//! Removed > Renamed > Modified > Created
//! ```
//!
//! Rationale (same as Syncthing / fswatch):
//!
//! - `Removed` dominates — the file is gone; nothing else matters.
//! - `Renamed` dominates `Modified` and `Created` — a rename carries
//!   both paths, the engine needs to see the pair.
//! - `Modified` dominates `Created` — if a file appears and is
//!   written within the window, the consumer only needs the final
//!   state, which is what `Modified` surfaces (with contents ready
//!   to read).
//!
//! # Atomic-save collapse
//!
//! Separate from the debouncer proper: the [`AtomicSaveTracker`]
//! records `Created(tmp)` events for the duration of
//! `atomic_window`. When a `Renamed(tmp, final)` arrives and `tmp`
//! matches a recent temp creation, the tracker rewrites the event
//! to `Modified(final)` and suppresses the pair. That's what turns
//! the vim `:wq` dance (write to `.file.swp`, fsync, rename over,
//! 5 inotify events total) into a single `Modified(file)`.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use crate::event::FsEvent;

/// Relative ordering of event kinds. Higher priority dominates when
/// multiple events collapse onto one debounced path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventPriority {
    Created = 0,
    Modified = 1,
    Renamed = 2,
    Removed = 3,
}

impl EventPriority {
    pub fn of(evt: &FsEvent) -> Self {
        match evt {
            FsEvent::Created(_) => Self::Created,
            FsEvent::Modified(_) => Self::Modified,
            FsEvent::Renamed(_, _) => Self::Renamed,
            FsEvent::Removed(_) => Self::Removed,
        }
    }
}

/// One pending entry in the debounce queue.
#[derive(Debug, Clone)]
pub struct DebouncedEvent {
    pub event: FsEvent,
    /// When this path's debounce window started. Further events on
    /// the same path update `event` (by priority) but not `start` —
    /// so a path that keeps getting modified flushes exactly once
    /// per window, not once per event.
    pub start: Instant,
}

/// A batched debouncer. Cheap to construct; pushes are O(1) average
/// (hashmap entry + priority compare); flush is O(N) over the
/// current pending map.
#[derive(Debug)]
pub struct DebounceQueue {
    pending: HashMap<PathBuf, DebouncedEvent>,
    window: Duration,
}

impl DebounceQueue {
    pub fn new(window: Duration) -> Self {
        Self {
            pending: HashMap::new(),
            window,
        }
    }

    pub fn window(&self) -> Duration {
        self.window
    }

    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }

    /// Push a raw event. Collapses against any existing pending
    /// event for the same canonical path using [`EventPriority`].
    pub fn push(&mut self, event: FsEvent, now: Instant) {
        let key = event.subject().to_path_buf();
        let new_pri = EventPriority::of(&event);
        match self.pending.get_mut(&key) {
            Some(existing) => {
                let existing_pri = EventPriority::of(&existing.event);
                if new_pri > existing_pri {
                    existing.event = event;
                }
                // `start` stays pinned — see the docstring on
                // DebouncedEvent.
            }
            None => {
                self.pending
                    .insert(key, DebouncedEvent { event, start: now });
            }
        }
    }

    /// Remove every pending entry whose window has fully elapsed as
    /// of `now`, sorted by subject path for stable output. The
    /// returned events are the emission batch.
    pub fn flush(&mut self, now: Instant) -> Vec<FsEvent> {
        let mut ready: Vec<(PathBuf, DebouncedEvent)> = self
            .pending
            .iter()
            .filter(|(_, de)| now.saturating_duration_since(de.start) >= self.window)
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        ready.sort_by(|a, b| a.0.cmp(&b.0));
        for (k, _) in &ready {
            self.pending.remove(k);
        }
        ready.into_iter().map(|(_, de)| de.event).collect()
    }

    /// Flush everything unconditionally. Used on shutdown.
    pub fn drain_all(&mut self) -> Vec<FsEvent> {
        let mut events: Vec<(PathBuf, FsEvent)> =
            self.pending.drain().map(|(k, de)| (k, de.event)).collect();
        events.sort_by(|a, b| a.0.cmp(&b.0));
        events.into_iter().map(|(_, e)| e).collect()
    }
}

/// State tracker for atomic-save detection.
///
/// Handles the two rename shapes `notify` delivers across platforms:
///
/// - **Unified rename** (`RenameMode::Both` — Linux inotify,
///   macOS FSEvents with cookie stitching): the watcher calls
///   [`AtomicSaveTracker::rewrite_rename`] with both halves and the
///   tracker returns `Modified(final)` if `from` was a recent temp.
/// - **Split rename** (Windows ReadDirectoryChangesW emits
///   `Remove(tmp) + Create(canonical)` as two distinct events
///   within a few ms): the watcher calls
///   [`AtomicSaveTracker::note_temp_removed`] on the `Remove` half,
///   then [`AtomicSaveTracker::try_claim_create`] on the `Create`
///   half. When a non-temp `Create` arrives within
///   `atomic_window` of a tracked temp removal, the tracker claims
///   it and returns `Modified(created)`.
///
/// The split-rename heuristic trades a narrow false-positive window
/// (user deletes a `.tmp` file and simultaneously creates an
/// unrelated regular file within 500 ms) for "vim / Office / etc
/// saves always collapse to one Modified on Windows". Editors are
/// the 99 % case; the false positive is harmless — a `Modified`
/// event on a newly-created file triggers the same sync action a
/// `Created` would.
#[derive(Debug)]
pub struct AtomicSaveTracker {
    /// Recent temp-name creations keyed by absolute path.
    recent_temps: HashMap<PathBuf, Instant>,
    /// Timestamps of recent temp-name removals. When a non-temp
    /// Create arrives within `window` of any of these, we treat it
    /// as the second half of a split rename.
    recent_temp_removals: Vec<Instant>,
    window: Duration,
}

impl AtomicSaveTracker {
    pub fn new(window: Duration) -> Self {
        Self {
            recent_temps: HashMap::new(),
            recent_temp_removals: Vec::new(),
            window,
        }
    }

    /// Record a temp-name `Created(tmp)` event. Call this for every
    /// `Created` whose path classifies as `KnownTemp`.
    pub fn record_temp(&mut self, tmp: PathBuf, now: Instant) {
        self.purge_stale(now);
        self.recent_temps.insert(tmp, now);
    }

    /// Try to rewrite a unified `Renamed(from, to)` event as a
    /// `Modified(to)` when `from` is a recent temp. Returns the
    /// rewritten event when it applies, `None` otherwise.
    pub fn rewrite_rename(&mut self, from: &Path, to: &Path, now: Instant) -> Option<FsEvent> {
        self.purge_stale(now);
        if self.recent_temps.remove(from).is_some() {
            Some(FsEvent::Modified(to.to_path_buf()))
        } else {
            None
        }
    }

    /// Note that a tracked temp file was removed. Call this for
    /// every `Remove` whose path is a recently-recorded temp. The
    /// split-rename detector uses this as the "something temp
    /// disappeared recently" signal.
    ///
    /// Returns `true` when the removal corresponded to a tracked
    /// temp (caller should drop the `Remove` event), `false`
    /// otherwise (caller should emit the event normally).
    pub fn note_temp_removed(&mut self, tmp: &Path, now: Instant) -> bool {
        self.purge_stale(now);
        if self.recent_temps.remove(tmp).is_some() {
            self.recent_temp_removals.push(now);
            true
        } else {
            false
        }
    }

    /// Try to claim a `Created(canonical)` event as the second half
    /// of a split rename. Returns `Some(Modified(canonical))` when a
    /// pending temp removal is present; `None` otherwise (caller
    /// should emit the `Created` normally).
    pub fn try_claim_create(&mut self, canonical: &Path, now: Instant) -> Option<FsEvent> {
        self.purge_stale(now);
        if !self.recent_temp_removals.is_empty() {
            // Consume one pending removal; first-in-first-out so a
            // multi-temp burst claims successive creates in order.
            self.recent_temp_removals.remove(0);
            Some(FsEvent::Modified(canonical.to_path_buf()))
        } else {
            None
        }
    }

    fn purge_stale(&mut self, now: Instant) {
        self.recent_temps
            .retain(|_, t| now.saturating_duration_since(*t) < self.window);
        self.recent_temp_removals
            .retain(|t| now.saturating_duration_since(*t) < self.window);
    }

    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.recent_temps.len()
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) fn pending_removals(&self) -> usize {
        self.recent_temp_removals.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn at(ms_from_base: u64) -> Instant {
        let base = Instant::now();
        base + Duration::from_millis(ms_from_base)
    }

    #[test]
    fn priority_ordering_is_total_and_expected() {
        assert!(EventPriority::Removed > EventPriority::Renamed);
        assert!(EventPriority::Renamed > EventPriority::Modified);
        assert!(EventPriority::Modified > EventPriority::Created);
    }

    #[test]
    fn push_then_flush_empty_before_window() {
        let mut q = DebounceQueue::new(Duration::from_millis(500));
        let base = Instant::now();
        q.push(FsEvent::Modified("/a".into()), base);
        let flushed = q.flush(base + Duration::from_millis(100));
        assert!(flushed.is_empty());
        assert!(!q.is_empty());
    }

    #[test]
    fn push_then_flush_after_window() {
        let mut q = DebounceQueue::new(Duration::from_millis(100));
        let base = Instant::now();
        q.push(FsEvent::Modified("/a".into()), base);
        let flushed = q.flush(base + Duration::from_millis(150));
        assert_eq!(flushed.len(), 1);
        assert_eq!(flushed[0], FsEvent::Modified("/a".into()));
        assert!(q.is_empty());
    }

    #[test]
    fn multiple_modifies_collapse_to_one() {
        let mut q = DebounceQueue::new(Duration::from_millis(100));
        let base = Instant::now();
        for _ in 0..5 {
            q.push(FsEvent::Modified("/a".into()), base);
        }
        let flushed = q.flush(base + Duration::from_millis(150));
        assert_eq!(flushed.len(), 1);
    }

    #[test]
    fn removed_dominates_modified() {
        let mut q = DebounceQueue::new(Duration::from_millis(100));
        let base = Instant::now();
        q.push(FsEvent::Modified("/a".into()), base);
        q.push(FsEvent::Removed("/a".into()), base);
        let flushed = q.flush(base + Duration::from_millis(150));
        assert_eq!(flushed, vec![FsEvent::Removed("/a".into())]);
    }

    #[test]
    fn modified_dominates_created() {
        let mut q = DebounceQueue::new(Duration::from_millis(100));
        let base = Instant::now();
        q.push(FsEvent::Created("/a".into()), base);
        q.push(FsEvent::Modified("/a".into()), base);
        let flushed = q.flush(base + Duration::from_millis(150));
        assert_eq!(flushed, vec![FsEvent::Modified("/a".into())]);
    }

    #[test]
    fn renamed_dominates_modified() {
        let mut q = DebounceQueue::new(Duration::from_millis(100));
        let base = Instant::now();
        q.push(FsEvent::Modified("/a".into()), base);
        q.push(FsEvent::Renamed("/other".into(), "/a".into()), base);
        let flushed = q.flush(base + Duration::from_millis(150));
        assert_eq!(
            flushed,
            vec![FsEvent::Renamed("/other".into(), "/a".into())]
        );
    }

    #[test]
    fn lower_priority_doesnt_replace_higher() {
        let mut q = DebounceQueue::new(Duration::from_millis(100));
        let base = Instant::now();
        q.push(FsEvent::Removed("/a".into()), base);
        q.push(FsEvent::Modified("/a".into()), base);
        let flushed = q.flush(base + Duration::from_millis(150));
        assert_eq!(flushed, vec![FsEvent::Removed("/a".into())]);
    }

    #[test]
    fn flush_sorts_by_subject_path() {
        let mut q = DebounceQueue::new(Duration::from_millis(100));
        let base = Instant::now();
        q.push(FsEvent::Modified("/z".into()), base);
        q.push(FsEvent::Modified("/a".into()), base);
        q.push(FsEvent::Modified("/m".into()), base);
        let flushed = q.flush(base + Duration::from_millis(150));
        assert_eq!(
            flushed,
            vec![
                FsEvent::Modified("/a".into()),
                FsEvent::Modified("/m".into()),
                FsEvent::Modified("/z".into()),
            ]
        );
    }

    #[test]
    fn drain_all_empties_queue() {
        let mut q = DebounceQueue::new(Duration::from_millis(100));
        let base = Instant::now();
        q.push(FsEvent::Modified("/a".into()), base);
        q.push(FsEvent::Modified("/b".into()), base);
        let drained = q.drain_all();
        assert_eq!(drained.len(), 2);
        assert!(q.is_empty());
    }

    #[test]
    fn start_pins_on_first_push_not_subsequent() {
        // A path that keeps getting modified should flush exactly
        // one window after the FIRST push, not keep resetting.
        let mut q = DebounceQueue::new(Duration::from_millis(100));
        let base = Instant::now();
        q.push(FsEvent::Modified("/a".into()), base);
        q.push(
            FsEvent::Modified("/a".into()),
            base + Duration::from_millis(50),
        );
        q.push(
            FsEvent::Modified("/a".into()),
            base + Duration::from_millis(90),
        );
        // At base + 110ms the window has elapsed from the first push.
        let flushed = q.flush(base + Duration::from_millis(110));
        assert_eq!(flushed.len(), 1);
    }

    // --- AtomicSaveTracker ---

    #[test]
    fn atomic_save_tracker_collapses_rename_to_modified() {
        let mut t = AtomicSaveTracker::new(Duration::from_millis(500));
        let base = Instant::now();
        let tmp = PathBuf::from("/project/.file.txt.swp");
        let final_path = PathBuf::from("/project/file.txt");
        t.record_temp(tmp.clone(), base);
        let rewritten = t
            .rewrite_rename(&tmp, &final_path, base + Duration::from_millis(10))
            .expect("rewrite should fire");
        assert_eq!(rewritten, FsEvent::Modified(final_path));
    }

    #[test]
    fn atomic_save_tracker_misses_stale_temp() {
        let mut t = AtomicSaveTracker::new(Duration::from_millis(100));
        let base = Instant::now();
        let tmp = PathBuf::from("/p/temp.tmp");
        t.record_temp(tmp.clone(), base);
        // Rename arrives after the window elapses — not a match.
        let rewritten = t.rewrite_rename(
            &tmp,
            &PathBuf::from("/p/file.txt"),
            base + Duration::from_millis(200),
        );
        assert!(rewritten.is_none());
    }

    #[test]
    fn atomic_save_tracker_purges_old_entries_on_record() {
        let mut t = AtomicSaveTracker::new(Duration::from_millis(100));
        let base = Instant::now();
        t.record_temp(PathBuf::from("/a.tmp"), base);
        t.record_temp(PathBuf::from("/b.tmp"), base + Duration::from_millis(200));
        // First entry should have been purged when /b.tmp was recorded.
        assert_eq!(t.len(), 1);
    }

    // Silence unused import warnings in minimal test builds.
    #[allow(dead_code)]
    fn _unused(_: Instant) {}
    #[allow(dead_code)]
    fn _unused2(_: Duration) {}
    #[allow(dead_code)]
    fn _unused3(_: Instant) -> Instant {
        at(0)
    }
}
