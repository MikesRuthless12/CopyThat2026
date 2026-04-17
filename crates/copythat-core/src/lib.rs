//! `copythat-core` — async byte-exact copy engine + in-memory job queue.
//!
//! # What's here today (Phases 1 + 2)
//!
//! - `copy_file` / `move_file` — async single-file primitives with
//!   pause / resume / cancel, progress events, buffer-size tuning,
//!   metadata preservation.
//! - `copy_tree` / `move_tree` — whole-directory operations with
//!   bounded-concurrency per-file copies, a `CollisionPolicy` for
//!   existing destinations, tree-level aggregate progress events,
//!   and an EXDEV fallback for cross-volume `move_tree`.
//! - `Queue` + `Job` + `QueueEvent` — in-memory job tracking with
//!   broadcast pub/sub. Executes nothing by itself; the caller
//!   (currently the future Tauri bridge, Phase 5) drives jobs through
//!   `start` / `set_progress` / `mark_completed` / `mark_failed`.
//! - `CopyControl` — cloneable steering handle shared between the
//!   caller and the running task. Also how the queue pauses / resumes
//!   / cancels a job.
//! - `CopyEvent` — one enum for every lifecycle notification
//!   (per-file Started / Progress / Paused / Resumed / Completed /
//!   Failed + tree-level TreeStarted / TreeProgress / TreeCompleted +
//!   Collision).
//! - `CopyError` / `CopyErrorKind` — typed failure, classified into
//!   the small set the UI and retry policy branch on.
//!
//! Added in Phase 3:
//! - `CopyOptions::verify` — when `Some(verifier)`, the copy loop
//!   feeds source bytes into a streaming hasher during the normal
//!   read pass (no re-read), then post-hashes the destination and
//!   compares. Mismatch emits `CopyEvent::VerifyFailed` and fails the
//!   copy with `CopyErrorKind::VerifyFailed`. The `Verifier` /
//!   `Hasher` abstraction lives in `copythat_core::verify`;
//!   `copythat-hash` plugs in the concrete algorithms.
//!
//! Not yet implemented (deferred by design):
//! - Platform fast paths (CopyFileExW, copyfile, copy_file_range,
//!   reflink) — Phase 6.
//! - Secure delete — Phase 4.
//! - Queue persistence — Phase 10.
//!
//! # Example (single-file)
//!
//! ```no_run
//! use copythat_core::{copy_file, CopyControl, CopyEvent, CopyOptions};
//! use std::path::Path;
//! use tokio::sync::mpsc;
//!
//! # async fn demo() -> Result<(), copythat_core::CopyError> {
//! let (tx, mut rx) = mpsc::channel::<CopyEvent>(64);
//! let ctrl = CopyControl::new();
//! let ctrl_for_ui = ctrl.clone();
//!
//! let copy = tokio::spawn(async move {
//!     copy_file(
//!         Path::new("big.iso"),
//!         Path::new("big.iso.copy"),
//!         CopyOptions::default(),
//!         ctrl,
//!         tx,
//!     )
//!     .await
//! });
//!
//! while let Some(evt) = rx.recv().await {
//!     match evt {
//!         CopyEvent::Progress { bytes, total, .. } => {
//!             println!("{}/{}", bytes, total);
//!         }
//!         CopyEvent::Completed { .. } => break,
//!         CopyEvent::Failed { err } => return Err(err),
//!         _ => {}
//!     }
//! }
//! let _ = copy.await;
//! # let _ = ctrl_for_ui;
//! # Ok(())
//! # }
//! ```

#![forbid(unsafe_code)]

pub mod collision;
mod control;
mod engine;
mod error;
mod event;
mod options;
pub mod queue;
mod tree;
pub mod verify;

pub use collision::CollisionPolicy;
pub use control::CopyControl;
pub use engine::copy_file;
pub use error::{CopyError, CopyErrorKind};
pub use event::{Collision, CollisionResolution, CopyEvent, CopyReport, TreeReport};
pub use options::{
    CopyOptions, DEFAULT_BUFFER_SIZE, DEFAULT_TREE_CONCURRENCY, MAX_BUFFER_SIZE, MIN_BUFFER_SIZE,
    MoveOptions, TreeOptions,
};
pub use queue::{Job, JobId, JobKind, JobState, Queue, QueueEvent};
pub use tree::{copy_tree, move_file, move_tree};
pub use verify::{Hasher, Verifier};
