//! `copythat-platform` — OS-specific fast paths for byte-exact file copy.
//!
//! # What's here today (Phase 6)
//!
//! - [`fast_copy`] — async dispatcher that attempts, in order: reflink
//!   (instant, COW), the OS-native accelerated path
//!   (`CopyFileExW` / `copyfile(3)` / `copy_file_range(2)`), and
//!   finally the Phase 1 [`copythat_core::copy_file`] async loop.
//! - [`PlatformFastCopyHook`] — implementation of
//!   [`copythat_core::FastCopyHook`] that adapts [`fast_copy`] into the
//!   `CopyOptions::fast_copy_hook` slot. Drop one of these into
//!   `CopyOptions` and every per-file copy automatically tries the OS
//!   acceleration path.
//! - Storage probes: [`is_ssd`], [`filesystem_name`],
//!   [`supports_reflink`].
//! - [`recommend_concurrency`] — HDD heuristic: clamp tree concurrency
//!   to 1 when either side lives on rotational media (avoids seek
//!   thrash). SSDs return the unmodified default.
//!
//! # Strategy ordering
//!
//! 1. **Reflink** (`reflink-copy` crate). Instant on Btrfs, XFS with
//!    `reflink=1`, ZFS, bcachefs (Linux); APFS (macOS); ReFS / Dev
//!    Drive (Windows). Returns
//!    [`ChosenStrategy::Reflink`](crate::ChosenStrategy::Reflink) on
//!    success; falls through on `NotSupported`.
//! 2. **OS-native accelerated path:**
//!    - Linux: `copy_file_range(2)` in a loop; sendfile fallback for
//!      files <2 GiB; otherwise fall through. Reports
//!      [`ChosenStrategy::CopyFileRange`](crate::ChosenStrategy::CopyFileRange)
//!      or [`ChosenStrategy::Sendfile`](crate::ChosenStrategy::Sendfile).
//!    - macOS: `copyfile(3)` with `COPYFILE_ALL`. Reports
//!      [`ChosenStrategy::Copyfile`](crate::ChosenStrategy::Copyfile).
//!    - Windows: `CopyFileExW` with progress callback; uses
//!      `COPY_FILE_NO_BUFFERING` for files ≥256 MiB. Reports
//!      [`ChosenStrategy::CopyFileExW`](crate::ChosenStrategy::CopyFileExW).
//! 3. **Async fallback** — delegate to [`copythat_core::copy_file`].
//!    Reports [`ChosenStrategy::AsyncFallback`](crate::ChosenStrategy::AsyncFallback).
//!
//! Each fast path emits the same `Started` / `Progress` / `Completed`
//! events as the Phase 1 engine so a UI sees one timeline regardless
//! of which path actually moved the bytes.
//!
//! # Wiring example
//!
//! ```no_run
//! use std::sync::Arc;
//! use copythat_core::{CopyOptions, copy_file, CopyControl, CopyEvent};
//! use copythat_platform::PlatformFastCopyHook;
//! use tokio::sync::mpsc;
//!
//! # async fn demo() -> Result<(), copythat_core::CopyError> {
//! let (tx, _) = mpsc::channel::<CopyEvent>(64);
//! let opts = CopyOptions {
//!     fast_copy_hook: Some(Arc::new(PlatformFastCopyHook::default())),
//!     ..CopyOptions::default()
//! };
//! copy_file(
//!     std::path::Path::new("big.iso"),
//!     std::path::Path::new("big.iso.copy"),
//!     opts,
//!     CopyControl::new(),
//!     tx,
//! )
//! .await?;
//! # Ok(())
//! # }
//! ```

#![allow(unsafe_code)] // Justified per-module: every fast path is a raw FFI call.

mod dispatcher;
mod helpers;
mod hook;
mod native;
mod outcome;
mod reflink_path;

pub use dispatcher::fast_copy;
pub use helpers::{
    DEFAULT_HDD_CONCURRENCY, filesystem_name, free_space_bytes, is_ssd, recommend_concurrency,
    supports_reflink, volume_id,
};
pub use hook::PlatformFastCopyHook;
pub use outcome::{ChosenStrategy, FastCopyOutcome};
