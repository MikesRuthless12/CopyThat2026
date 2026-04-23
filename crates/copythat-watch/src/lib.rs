//! `copythat-watch` — real-time filesystem watcher with edge-case
//! hardening for the save patterns real editors actually use.
//!
//! [`Watcher`] wraps `notify::RecommendedWatcher` (ReadDirectoryChangesW
//! on Windows, FSEvents on macOS, inotify on Linux) and layers:
//!
//! - **Path filters.** Vim swap files (`.foo.swp` / `.swpx` / `.swx`),
//!   emacs lock files (`.#foo`), Office lock files (`~$foo.docx`), and
//!   optional editor backups (`foo~` / `foo.bak`) are dropped at the
//!   ingress before anything else sees them.
//! - **Atomic-save collapse.** When a CREATE on a temp name is
//!   followed within `atomic_window` by a RENAME of that temp to a
//!   final path, the watcher emits only `Modified(final)`. This covers
//!   every `write to tmp → fsync → rename over` editor save — vim,
//!   GEdit, JetBrains, VS Code, LibreOffice, Word — without a match
//!   table.
//! - **Office save dance.** A slightly richer variant of atomic save
//!   that also swallows the lock-file creation / removal pair around
//!   the canonical rename.
//! - **Per-path debounce.** All events for a single canonical path
//!   inside a debounce window (default 500 ms) coalesce into one
//!   emission using the highest-priority kind:
//!   `Removed > Renamed > Modified > Created`. A path that gets
//!   CREATED + MODIFIED + MODIFIED + MODIFIED within the window
//!   emits a single `Modified` event; a CREATED followed by REMOVED
//!   within the window emits a single `Removed`.
//! - **Windows overflow recovery.** When ReadDirectoryChangesW reports
//!   a buffer overflow (`notify::ErrorKind::Io` with `ERROR_NOTIFY_ENUM_DIR`
//!   or the crate's ScopeChanged rescan signal), the watcher queues a
//!   directory rescan and emits synthetic `Modified` events for every
//!   entry so no change is silently dropped.
//! - **macOS directory-coalesce recovery.** FSEvents sometimes emits
//!   "directory X changed" without naming a specific file inside.
//!   When detected, the watcher enumerates the directory and emits
//!   a `Modified` event per child so downstream consumers see a
//!   resolvable path.
//!
//! # Integration with Phase 25 sync
//!
//! The Phase 25 [`copythat-sync`](../copythat_sync/index.html) engine
//! holds a [`Watcher`] per pair when the user opts into "Live mirror
//! (real-time)". Each debounced [`FsEvent`] triggers a localized
//! re-sync of the affected subtree — not a full rescan. The watcher
//! is the trusted "something changed here" signal; the sync engine
//! handles the actual reconciliation + conflict detection.

#![forbid(unsafe_code)]

pub mod debounce;
pub mod error;
pub mod event;
pub mod filter;
pub mod watcher;

pub use debounce::{DebounceQueue, DebouncedEvent, EventPriority};
pub use error::{Result, WatchError};
pub use event::FsEvent;
pub use filter::{PathFilter, default_filter};
pub use watcher::{Watcher, WatcherOptions};
