//! `copythat-core` — async byte-exact file copy engine.
//!
//! Phase 1 scope: a single-file `copy_file` primitive with pause / resume
//! / cancel, progress events, buffer-size tuning, and metadata
//! preservation (mtime, atime, permissions). No platform fast paths
//! (those land in Phase 6) and no tree / queue logic (Phase 2).
//!
//! # Public surface
//!
//! - [`copy_file`] — the async entry point.
//! - [`CopyOptions`] — per-call knobs (buffer size, fsync, follow
//!   symlinks, preserve metadata, keep_partial).
//! - [`CopyControl`] — cloneable steering handle: `pause` / `resume` /
//!   `cancel`.
//! - [`CopyEvent`] — progress / lifecycle notifications on an
//!   `mpsc::Sender`.
//! - [`CopyReport`] — final success record.
//! - [`CopyError`] / [`CopyErrorKind`] — typed failure, classified into
//!   the small set the UI and retry policy branch on.
//!
//! # Example
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
//! // Somewhere else: ctrl_for_ui.pause(); ctrl_for_ui.resume();
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

mod control;
mod engine;
mod error;
mod event;
mod options;

pub use control::CopyControl;
pub use engine::copy_file;
pub use error::{CopyError, CopyErrorKind};
pub use event::{CopyEvent, CopyReport};
pub use options::{CopyOptions, DEFAULT_BUFFER_SIZE, MAX_BUFFER_SIZE, MIN_BUFFER_SIZE};
