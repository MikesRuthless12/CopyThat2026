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
//! Added in Phase 23:
//! - `copythat_core::sparse` — `ByteRange`, `SparseOps` trait,
//!   `DenseOnlySparseOps` fallback stub, `SparsenessMismatch`
//!   detail, `allocated_bytes` helper. `CopyOptions::preserve_
//!   sparseness: bool` + `CopyOptions::sparse_ops: Option<Arc<dyn
//!   SparseOps>>` drive a dedicated sparse-aware copy path that
//!   preserves hole layouts across the copy. The unsafe OS calls
//!   (`SEEK_HOLE` / `SEEK_DATA`, `FSCTL_QUERY_ALLOCATED_RANGES` /
//!   `FSCTL_SET_SPARSE`) live in `copythat_platform::sparse` so
//!   this crate stays `#![forbid(unsafe_code)]`-clean. Regression
//!   surfaces via `CopyEvent::SparsenessNotSupported { dst_fs }`
//!   on densifying filesystems and `CopyErrorKind::SparsenessMismatch`
//!   when the dst's allocated footprint balloons past the source's.
//!
//! Added in Phase 24:
//! - `copythat_core::meta` — `MetaSnapshot`, `NtfsStream`, `XattrEntry`,
//!   `PosixAclBlob`, `SeLinuxContext`, `FileCaps`, `ResourceForkBlob`,
//!   `FinderInfoBlob`, `MetaPolicy` (per-toggle gating including
//!   Mark-of-the-Web), `MetaApplyOutcome`, `MetaOps` trait, and the
//!   `NoopMetaOps` fallback. `CopyOptions::preserve_security_metadata:
//!   bool` + `CopyOptions::meta_policy: MetaPolicy` +
//!   `CopyOptions::meta_ops: Option<Arc<dyn MetaOps>>` drive a
//!   capture-on-source / apply-on-dst pass that runs after timestamps
//!   and permissions. Cross-FS destinations that can't hold the
//!   foreign metadata fall through to an `._<filename>` AppleDouble
//!   sidecar (when `MetaPolicy::appledouble_fallback` is on) and the
//!   engine surfaces `CopyEvent::MetaTranslatedToAppleDouble { ext }`.
//!   Platform syscalls (`FindFirstStreamW` / `BackupRead` on Windows,
//!   `xattr` family on Linux/macOS, `..namedfork/rsrc` for Carbon
//!   resource forks) live in `copythat_platform::meta` so this crate
//!   stays `#![forbid(unsafe_code)]`-clean.
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
// Phase 42 doc audit: warn-only on missing public-API docs so future
// additions are flagged without breaking the build on the historic
// undocumented surface (control flag accessors, error variants, etc.).
#![warn(missing_docs)]

pub mod collision;
mod control;
pub mod dryrun;
mod engine;
mod error;
mod event;
pub mod filter;
pub mod meta;
mod options;
pub mod provenance;
pub mod queue;
pub mod safety;
pub mod scan;
pub mod sparse;
pub mod translate;
mod tree;
pub mod verify;
pub mod versioning;

pub use collision::CollisionPolicy;
pub use control::CopyControl;
pub use dryrun::{
    ConflictKind, DryRunOptions, ReasonForReplacement, ReasonForSkip, RelPath, TreeDiff,
    compute_tree_diff,
};
pub use engine::copy_file;
pub use error::{CopyError, CopyErrorKind};
pub use event::{
    Collision, CollisionResolution, CopyEvent, CopyReport, ErrorPrompt, ResumeAbortReason,
    TreeReport,
};
pub use filter::{CompiledFilters, FilterError, FilterSet};
pub use meta::{
    FileCaps, FinderInfoBlob, MetaApplyOutcome, MetaOps, MetaPolicy, MetaSnapshot, NoopMetaOps,
    NtfsStream, PosixAclBlob, ResourceForkBlob, SeLinuxContext, XattrEntry,
};
pub use options::{
    ChunkStoreSink, CloudSink, CopyOptions, CopyStrategy, DEFAULT_BUFFER_SIZE,
    DEFAULT_TREE_CONCURRENCY, ErrorAction, ErrorPolicy, FastCopyHook, FastCopyHookOutcome,
    JournalSink, LockedFilePolicy, MAX_BUFFER_SIZE, MIN_BUFFER_SIZE, MoveOptions, ResumePlan,
    ShapeSink, SnapshotGuard, SnapshotHook, SnapshotLease, TransformOutcome, TransformSink,
    TreeOptions,
};
pub use provenance::{OutboardEncoder, ProvenancePolicy, ProvenanceSink};
pub use queue::{
    Job, JobId, JobKind, JobState, Queue, QueueEvent, QueueId, QueueMergeError, QueueRegistry,
    QueueRegistryEvent, VolumeProbe,
};
pub use safety::{
    PathSafetyError, is_no_follow_rejection, is_within_root, no_follow_open_flags, validate_all,
    validate_path_no_traversal,
};
pub use sparse::{ByteRange, DenseOnlySparseOps, SparseOps, SparsenessMismatch, allocated_bytes};
pub use translate::{
    LineEnding, LineEndingMode, LongPathStrategy, NormalizationMode, PathPolicy,
    ReservedNameStrategy, TargetOs, TranslateError, WINDOWS_MAX_PATH, WINDOWS_RESERVED_STEMS,
    apply_long_path_prefix, apply_reserved_suffix, default_text_extensions, detect_line_ending,
    detect_line_ending_in, is_reserved_windows_name, normalize_name, path_utf16_len,
    resolve_target_os, should_translate_extension, translate_content_line_endings, translate_path,
};
pub use tree::{copy_tree, copy_tree_from_scan, move_file, move_tree};
pub use verify::{Hasher, Verifier};
pub use versioning::{
    GfsPolicy, RetentionPolicy, VersionEntry, VersioningPolicy, VersioningSink, select_for_pruning,
};
