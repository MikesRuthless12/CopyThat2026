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
//!    [`ChosenStrategy::Reflink`] on
//!    success; falls through on `NotSupported`.
//! 2. **OS-native accelerated path:**
//!    - Linux: `copy_file_range(2)` in a loop; sendfile fallback for
//!      files <2 GiB; otherwise fall through. Reports
//!      [`ChosenStrategy::CopyFileRange`]
//!      or [`ChosenStrategy::Sendfile`].
//!    - macOS: `copyfile(3)` with `COPYFILE_ALL`. Reports
//!      [`ChosenStrategy::Copyfile`].
//!    - Windows: `CopyFileExW` with progress callback; uses
//!      `COPY_FILE_NO_BUFFERING` for files ≥256 MiB. Reports
//!      [`ChosenStrategy::CopyFileExW`].
//! 3. **Async fallback** — delegate to [`copythat_core::copy_file`].
//!    Reports [`ChosenStrategy::AsyncFallback`].
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

#![allow(unsafe_code)]
// Justified per-module: every fast path is a raw FFI call.
// Phase 42 doc audit: warn-only on missing public-API docs so future
// additions are flagged without breaking the build on the historic
// undocumented FFI surface (raw struct fields, syscall constants).
#![warn(missing_docs)]

pub mod attrs;
#[cfg(feature = "compio-experimental")]
pub mod compio_overlapped;
pub mod dedup;
mod dispatcher;
pub mod hardlink_set;
mod helpers;
mod hook;
pub mod meta;
mod native;
pub mod os;
mod outcome;
pub mod presence;
mod reflink_path;
pub mod smb;
pub mod sparse;
pub mod topology;
pub mod wake_lock;

pub use dedup::{DedupMode, DedupOptions, DedupOutcome, DedupStrategy, HardlinkPolicy, try_dedup};
pub use dispatcher::fast_copy;
pub use helpers::{
    DEFAULT_HDD_CONCURRENCY, filesystem_name, free_space_bytes, is_cow_filesystem, is_ssd,
    recommend_concurrency, supports_reflink, volume_id,
};
pub use hook::PlatformFastCopyHook;
pub use meta::PlatformMetaOps;
pub use outcome::{ChosenStrategy, FastCopyOutcome};
pub use smb::{SmbCompressionAlgo, SmbCompressionState, negotiate_smb_compression};
pub use sparse::PlatformSparseOps;
pub use wake_lock::{WakeLock, WakeLockError, acquire_keep_awake};
