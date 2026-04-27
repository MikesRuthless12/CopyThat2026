//! Per-copy configuration.

use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;

use tokio::sync::mpsc;

use crate::control::CopyControl;
use crate::error::CopyError;
use crate::event::{CopyEvent, CopyReport};
use crate::filter::FilterSet;
use crate::verify::Verifier;

/// Default per-file copy buffer size. 1 MiB.
pub const DEFAULT_BUFFER_SIZE: usize = 1024 * 1024; // 1 MiB
/// Minimum buffer size accepted by the engine. 64 KiB.
pub const MIN_BUFFER_SIZE: usize = 64 * 1024; // 64 KiB
/// Maximum buffer size accepted by the engine. 16 MiB.
pub const MAX_BUFFER_SIZE: usize = 16 * 1024 * 1024; // 16 MiB

/// What the engine should do when a file can't be opened for read
/// because another process holds an exclusive lock
/// (`ERROR_SHARING_VIOLATION` on Windows, `EBUSY` on certain Linux FUSE
/// mounts, `NSFileLockingError` on macOS).
///
/// Added in Phase 19b to let the user opt into filesystem-snapshot
/// reads (VSS / ZFS / Btrfs / APFS). `Retry` preserves the Phase 14
/// behaviour so older call sites see no change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LockedFilePolicy {
    /// Short exponential-backoff retry (50 / 100 / 200 ms) inside
    /// `copy_file` itself. Handles the common "Office has the .docx
    /// open for write, saves in a few hundred ms" case without any
    /// escalation. The default.
    #[default]
    Retry,
    /// Skip the file after the retry loop is exhausted. The tree
    /// engine records a `FileError` / `errored++` and moves on.
    Skip,
    /// After retry exhausts, ask
    /// [`CopyOptions::snapshot_hook`](super::CopyOptions::snapshot_hook)
    /// for a snapshot-side path and reopen the read against it. On
    /// success, [`CopyEvent::SnapshotCreated`]
    /// is emitted so the UI can render a "📷 Reading from VSS
    /// snapshot of C:" badge. On failure falls through to a typed
    /// error (respecting the tree-level `on_error` policy).
    Snapshot,
    /// Emit a one-time prompt to the UI asking which of the above to
    /// apply (and whether to remember per-volume). The runner layer
    /// translates the user's answer into one of the above and feeds
    /// it back into the engine. Equivalent to `Retry` if no prompter
    /// is attached.
    Ask,
}

/// Opaque lease returned by a [`SnapshotHook`] — a snapshot-side path
/// the engine opens in place of the locked live source, plus a Drop
/// guard that releases the snapshot when the lease is dropped.
///
/// The engine holds the lease for the duration of the copy; dropping
/// it after `copy_file` returns is what releases the underlying
/// snapshot.
#[derive(Debug)]
pub struct SnapshotLease {
    /// The path the engine should open for read in place of the
    /// original locked source.
    pub translated: PathBuf,
    /// Stable wire string — `"vss"` / `"zfs"` / `"btrfs"` / `"apfs"`.
    pub kind_wire: &'static str,
    /// Live-source root the snapshot covers. The UI renders this
    /// alongside the badge — "VSS snapshot of `C:\`".
    pub original_root: PathBuf,
    /// Root mount of the snapshot itself — used by the UI mostly for
    /// debugging / verbose modes.
    pub mount_root: PathBuf,
    /// The lease's RAII guard. Dropping this calls the backend's
    /// `zfs destroy` / `btrfs subvolume delete` / VSS `Delete()`.
    pub guard: Box<dyn SnapshotGuard>,
}

/// Marker trait for a `SnapshotLease` drop guard.
///
/// Implementors hold whatever state the backend needs to tear the
/// snapshot down; the engine never inspects it. The tearing-down
/// logic runs in the impl's `Drop` — this trait exists purely so
/// [`SnapshotLease::guard`] can hold a trait object.
pub trait SnapshotGuard: Send + Sync + std::fmt::Debug {}

/// Phase 20 — what the engine should do with an existing partial
/// destination.
///
/// Returned by [`JournalSink::resume_plan`] when the engine sees
/// `dst.exists() && dst.metadata().len() < expected_total`. Mirrors
/// the same enum in `copythat_journal::types`; kept in core so the
/// engine doesn't need a journal dep.
#[derive(Debug, Clone, PartialEq)]
pub enum ResumePlan {
    /// Re-hash the destination's first `offset` bytes via BLAKE3
    /// and compare against `src_hash_at_offset`. On match, seek both
    /// files to `offset` and continue. On mismatch, the engine emits
    /// [`CopyEvent::ResumeAborted`]
    /// and falls back to a full restart.
    Resume {
        /// Bytes the engine had already streamed when the previous
        /// run was interrupted.
        offset: u64,
        /// BLAKE3 of the first `offset` source bytes from the prior run.
        src_hash_at_offset: [u8; 32],
    },
    /// Nothing reusable — start over from byte 0.
    Restart,
    /// The destination is already the right size and the
    /// checkpoint's `final_hash` matches a re-hash of the existing
    /// destination. Skip the copy entirely.
    AlreadyComplete {
        /// BLAKE3 of the source recorded by the prior run.
        final_hash: [u8; 32],
    },
}

/// Bridge contract for the durable resume journal.
///
/// Implemented by `copythat_journal::CopyThatJournalSink`. The
/// engine calls `checkpoint` every `PROGRESS_MIN_INTERVAL` (50 ms)
/// with the running BLAKE3 of the source bytes already read; on
/// finish it calls `finish_file` with the final hash, and at job
/// teardown the runner calls one of the three `finish_job_*`
/// terminators.
///
/// All methods are infallible by design: a journal failure is never
/// allowed to abort a copy. Implementations swallow internal errors
/// (and may log them) so the engine treats the journal as
/// best-effort.
pub trait JournalSink: Send + Sync + std::fmt::Debug {
    /// Persist the running progress for `(file_idx, dst)`. Called
    /// from the engine's progress-throttle path so the on-disk row
    /// updates at most once per `PROGRESS_MIN_INTERVAL`.
    fn checkpoint(
        &self,
        file_idx: u64,
        dst: &std::path::Path,
        bytes_done: u64,
        expected_total: u64,
        hash_so_far: [u8; 32],
    );

    /// Mark the file as finished; capture the final BLAKE3 digest.
    fn finish_file(&self, file_idx: u64, final_hash: [u8; 32]);

    /// Decide what the engine should do with the existing partial
    /// destination at `(file_idx)`. Called once per file, before
    /// the first byte is written.
    fn resume_plan(&self, file_idx: u64) -> ResumePlan;

    /// Job-level terminator: succeeded — exactly one of the three
    /// `finish_job_*` methods fires per job lifecycle.
    fn finish_job_succeeded(&self);
    /// Job-level terminator: failed.
    fn finish_job_failed(&self);
    /// Job-level terminator: cancelled by the user.
    fn finish_job_cancelled(&self);
}

/// Bridge contract for a filesystem-snapshot source.
///
/// Implemented by `copythat_snapshot::CopyThatSnapshotHook`. Kept in
/// this crate so [`CopyOptions`] can hold a trait object without a
/// dependency cycle between `copythat-core` and `copythat-snapshot`.
///
/// The hook is consulted exactly once per locked source file, after
/// the engine's own sharing-violation backoff (`open_src_with_retry`)
/// has exhausted. If the hook returns `Err`, the engine surfaces the
/// error (as configured by the tree-level `on_error` policy). If the
/// hook returns `Ok(lease)`, the engine opens `lease.translated` for
/// read and carries on with the normal copy loop.
pub trait SnapshotHook: Send + Sync + std::fmt::Debug {
    /// Take a snapshot of the volume containing `src` and return a
    /// [`SnapshotLease`] whose `translated` path the engine can open
    /// for read.
    fn open_for_read<'a>(
        &'a self,
        src: PathBuf,
    ) -> Pin<Box<dyn Future<Output = Result<SnapshotLease, CopyError>> + Send + 'a>>;
}

/// Phase 21 — bridge contract for the bandwidth-shaping token bucket.
///
/// Implemented by `copythat_shape::CopyThatShapeSink`. Kept in this
/// crate so [`CopyOptions`] can hold a trait object without a
/// dependency cycle between `copythat-core` and `copythat-shape`.
///
/// The engine calls [`ShapeSink::permit`] after every buffered read,
/// passing the byte count it just consumed. Implementations are
/// expected to be best-effort: a sink that swallows scheduler glitches
/// or a `set_rate` race is preferable to one that panics from the hot
/// path. The future returned must be `Send` so it can be awaited from
/// inside the tree's per-file tasks.
pub trait ShapeSink: Send + Sync + std::fmt::Debug {
    /// Wait until `bytes` worth of cells are available, then return.
    /// A sink with no active cap returns an immediately-ready future
    /// (the engine still pays one `Box::pin` allocation per read; the
    /// `shape: None` fast path on `CopyOptions` avoids even that).
    fn permit<'a>(&'a self, bytes: u64) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>;
}

/// Phase 35 — bridge contract for on-the-fly encryption + compression.
///
/// Implemented by `copythat_crypt::CopyThatCryptHook`. Kept in this
/// crate so [`CopyOptions`] can hold a trait object without pulling
/// `copythat-crypt` into the core dep graph (same pattern as
/// [`ShapeSink`] / [`JournalSink`] / [`FastCopyHook`]).
///
/// The hook runs on a blocking thread (the engine wraps the call in
/// `spawn_blocking`) because both `age` and `zstd` are sync-first
/// libraries. The trait's one method consumes the source path + the
/// destination path and returns a [`TransformOutcome`] summarising
/// the bytes that flowed through the pipeline.
///
/// When a hook is installed and the sink returns a
/// [`TransformOutcome`] (i.e. the transform actually ran), the engine:
///
/// - Skips [`fast_copy_hook`](CopyOptions::fast_copy_hook) (fast
///   paths are byte-identical copies; they can't handle a
///   transformed destination).
/// - Skips [`verify`](CopyOptions::verify) (a byte-exact verify
///   against an encrypted / compressed destination would fail by
///   construction).
/// - Skips the write-after-verify fsync — the transform sink owns
///   the destination file for its entire lifetime.
/// - Emits [`crate::CopyEvent::CompressionSavings`] when the
///   outcome includes compression metrics.
pub trait TransformSink: Send + Sync + std::fmt::Debug {
    /// Execute the transform. Implementations open `src` + `dst` on
    /// the blocking thread they're invoked from, run the sync
    /// pipeline, and return the outcome.
    ///
    /// The engine calls this inside a `spawn_blocking` task; the
    /// future the trait returns is `Send + 'static` so it composes
    /// with the standard tokio task API.
    fn transform<'a>(
        &'a self,
        src: PathBuf,
        dst: PathBuf,
    ) -> Pin<Box<dyn Future<Output = Result<TransformOutcome, CopyError>> + Send + 'a>>;
}

/// Result payload the engine consumes from a [`TransformSink`].
/// Carries both the final destination size (for the
/// `CopyEvent::Completed` event) and optional compression metrics
/// (for the `CopyEvent::CompressionSavings` event).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TransformOutcome {
    /// The number of source bytes the sink read end-to-end. Used by
    /// the engine as the `bytes` value on `CopyEvent::Completed`.
    pub input_bytes: u64,
    /// Bytes actually written to the destination (post-transform).
    /// May be smaller than `input_bytes` when compression ran;
    /// identical when it didn't.
    pub output_bytes: u64,
    /// When the sink ran compression, this is the ratio
    /// `output_bytes / input_bytes`; `None` when compression was
    /// off. The engine uses this to decide whether to fire the
    /// `CompressionSavings` event.
    pub compression_ratio: Option<f64>,
    /// Whether encryption ran as part of the transform. Informational
    /// — today only used by the runner when displaying the job row
    /// badge.
    pub encrypted: bool,
}

/// Behaviour knobs for a single `copy_file` invocation.
#[derive(Debug, Clone)]
pub struct CopyOptions {
    /// Requested buffer size in bytes. Clamped to `[MIN_BUFFER_SIZE,
    /// MAX_BUFFER_SIZE]` by the engine; callers don't need to round.
    pub buffer_size: usize,
    /// If true, call `sync_all` on the destination before returning.
    /// Noticeably slower on spinning media; off by default.
    pub fsync_on_close: bool,
    /// If true, follow a symlinked source and copy the *target*. If
    /// false, clone the symlink itself at `dst`.
    pub follow_symlinks: bool,
    /// If true, copy mtime / atime from source to destination.
    pub preserve_times: bool,
    /// If true, copy the permission bits (mode on Unix, readonly bit on
    /// Windows) from source to destination.
    pub preserve_permissions: bool,
    /// If true, do NOT delete a partially-written destination when the
    /// copy fails or is cancelled. Leave it for the caller to inspect.
    pub keep_partial: bool,
    /// If true, refuse to overwrite an existing destination file and
    /// return `PermissionDenied`/`AlreadyExists`-flavoured error. The
    /// default (false) truncates and rewrites.
    pub fail_if_exists: bool,
    /// Optional post-copy verification.
    ///
    /// When `Some(verifier)`, the engine hashes the source stream
    /// during the normal read pass (no re-read) and hashes the
    /// destination via a dedicated post-pass. On mismatch it emits
    /// `CopyEvent::VerifyFailed` and fails the copy with
    /// `CopyErrorKind::VerifyFailed`. `copythat-hash` provides the
    /// standard set of algorithms via
    /// `HashAlgorithm::verifier()`.
    pub verify: Option<Verifier>,
    /// Automatically enable `fsync_on_close` when `verify` is `Some`.
    /// The destination post-pass reads the file immediately after the
    /// write loop, and on some filesystems (notably NFS and several
    /// network-backed shares) the post-pass can race page-cache state.
    /// Defaults to `true` — callers who know their filesystem can set
    /// it off.
    pub fsync_before_verify: bool,
    /// Which copy strategy the engine should attempt. See [`CopyStrategy`].
    /// Default is [`CopyStrategy::Auto`]. The strategy is only consulted
    /// when [`fast_copy_hook`](Self::fast_copy_hook) is also set;
    /// otherwise the engine always runs the async loop regardless of
    /// strategy.
    pub strategy: CopyStrategy,
    /// Optional bridge to the OS-native fast paths.
    ///
    /// When `Some`, `copy_file` consults the hook before opening files
    /// for the standard read/write loop. The hook is responsible for
    /// reflink, `CopyFileExW`, `copyfile(3)`, `copy_file_range(2)`, and
    /// any other syscall-level acceleration. Returning
    /// [`FastCopyHookOutcome::NotSupported`] tells the engine to fall
    /// through to its async loop. The bridge implementation lives in
    /// `copythat-platform` to keep this crate `#![forbid(unsafe_code)]`-clean.
    ///
    /// The hook is bypassed entirely when [`verify`](Self::verify) is
    /// `Some`, because the verify pipeline relies on hashing the source
    /// bytes during the write loop — fast paths don't expose the
    /// bytes, so verifying through them would require a third-pass
    /// re-read of both files and lose the integration's perf win.
    pub fast_copy_hook: Option<Arc<dyn FastCopyHook>>,
    /// Phase 19b — what to do when the source is open for exclusive
    /// write by another process. Defaults to `Retry` which preserves
    /// the pre-Phase-19b 50/100/200 ms backoff. Set to `Snapshot` to
    /// opt into the VSS / ZFS / Btrfs / APFS fallback when a
    /// [`snapshot_hook`](Self::snapshot_hook) is installed.
    pub on_locked: LockedFilePolicy,
    /// Phase 19b — filesystem-snapshot bridge.
    ///
    /// Consulted by the engine when `on_locked` is `Snapshot` and the
    /// short retry loop couldn't open the source. Only
    /// `copythat-snapshot::CopyThatSnapshotHook` implements this in
    /// tree; a custom hook can be wired for testing.
    pub snapshot_hook: Option<Arc<dyn SnapshotHook>>,
    /// Phase 20 — durable resume journal sink.
    ///
    /// When `Some`, the engine checkpoints the running BLAKE3 + byte
    /// offset every `PROGRESS_MIN_INTERVAL` (50 ms) so a power-cut
    /// mid-copy can resume from the last checkpoint without losing
    /// the prefix that already made it to disk. The engine probes
    /// `journal.resume_plan(file_idx)` once per file, before opening
    /// the source, and either resumes from a verified offset or
    /// starts over.
    ///
    /// Implemented by `copythat_journal::CopyThatJournalSink`. The
    /// `file_idx` the sink sees is engine-assigned: 0 for a
    /// single-file `copy_file`, monotonic across the streaming tree
    /// walker for `copy_tree`.
    pub journal: Option<Arc<dyn JournalSink>>,
    /// Phase 20 — when `journal` is `Some`, the engine reports its
    /// own file index here so the journal sink can correlate.
    /// Defaults to `0`. Tree wrappers pass through `file_idx` from
    /// the walker.
    pub journal_file_idx: u64,
    /// Phase 21 — bandwidth-shaping sink.
    ///
    /// When `Some`, the engine calls `shape.permit(read_len).await`
    /// after every buffered read. The sink is responsible for the
    /// GCRA token bucket, the schedule-driven rate updates, and the
    /// auto-throttle rules. `None` (the default) preserves the
    /// pre-Phase-21 behaviour: the copy loop runs at the storage
    /// device's native ceiling.
    ///
    /// Implemented by `copythat_shape::CopyThatShapeSink`. The sink
    /// is cheap to clone, so a single shared `Shape` can drive
    /// every concurrent copy in the runner without contention.
    pub shape: Option<Arc<dyn ShapeSink>>,
    /// Phase 23 — preserve source sparseness on the destination.
    ///
    /// When `true` (the default) and a
    /// [`sparse_ops`](Self::sparse_ops) hook is installed, the engine
    /// asks the hook for the source's extent layout, marks the
    /// destination sparse (NTFS `FSCTL_SET_SPARSE`, no-op on
    /// Linux/macOS), pre-sizes it via `set_len`, and writes only the
    /// allocated extents. The post-copy verify pass re-scans the
    /// destination and fails with `CopyErrorKind::SparsenessMismatch`
    /// if the layouts don't agree.
    ///
    /// `false` forces the engine's classic dense copy even when the
    /// source has holes (destination will be larger on disk than the
    /// source). `true` with `sparse_ops = None` silently falls back
    /// to the dense path — there is no way to honour the request
    /// without a hook.
    pub preserve_sparseness: bool,
    /// Phase 42 — paranoid verify mode.
    ///
    /// When `true` (and [`verify`](Self::verify) is also set), the
    /// engine forces a full filesystem flush (`FlushFileBuffers` on
    /// Windows, `fsync` on Unix) before the verify pass and asks the
    /// kernel to drop the destination's page-cache pages
    /// (`posix_fadvise(POSIX_FADV_DONTNEED)` on Linux; the `fsync`
    /// alone covers the Windows case via the cache manager's
    /// write-through behaviour). This catches three failure modes
    /// the default verify cannot:
    ///   1. Write-cache lying — drives that ack a write before
    ///      persistence (FUA ignored, volatile DRAM cache).
    ///   2. Silent destination bit-flips on platter / NAND.
    ///   3. Filesystem / driver bugs in the write path.
    ///
    /// Cost: ~50 % throughput reduction on the verify pass (the
    /// re-read is now uncached). Off by default; opt-in per-job.
    pub paranoid_verify: bool,
    /// Phase 42 — number of retries on `ERROR_SHARING_VIOLATION` /
    /// `ERROR_LOCK_VIOLATION` (Windows) when opening a source file
    /// that's transiently locked by another process.
    ///
    /// Default `3` matches Robocopy `/R:3`. Backoff is exponential:
    /// `sharing_violation_base_delay_ms << attempt`.
    pub sharing_violation_retries: u32,
    /// Phase 42 — base delay (in milliseconds) for the
    /// sharing-violation retry exponential backoff. Each retry
    /// doubles. Default `50` (50 ms / 100 ms / 200 ms — covers most
    /// short-lived AV / indexer locks).
    pub sharing_violation_base_delay_ms: u64,
    /// Phase 23 — extent-introspection bridge.
    ///
    /// Implemented by `copythat_platform::PlatformSparseOps`.
    /// `None` disables the sparse pathway even when
    /// [`preserve_sparseness`](Self::preserve_sparseness) is `true`.
    /// Kept here as a trait object so callers (the Tauri shell, CLI,
    /// test harnesses) can plug in alternate backends without pulling
    /// in the platform crate's unsafe FFI.
    pub sparse_ops: Option<Arc<dyn crate::sparse::SparseOps>>,
    /// Phase 24 — preserve out-of-band security metadata on the
    /// destination.
    ///
    /// When `true` (the default) and a [`meta_ops`](Self::meta_ops)
    /// hook is installed, the engine captures the source file's
    /// metadata snapshot (NTFS ADS / xattrs / POSIX ACLs / SELinux
    /// context / Linux file capabilities / macOS resource fork +
    /// FinderInfo) after the byte copy completes and re-applies it to
    /// the destination after timestamps + permissions. The
    /// per-stream [`meta_policy`](Self::meta_policy) decides which
    /// surfaces survive.
    ///
    /// `false` disables the apply pass entirely — useful when copying
    /// to a sandbox that cannot accept any of the foreign metadata
    /// (e.g. a tmpfs scratch dir for tests).
    pub preserve_security_metadata: bool,
    /// Phase 24 — per-toggle gating of the metadata apply pass.
    /// Defaults to "preserve everything" (including
    /// `preserve_motw` — the Mark-of-the-Web Zone.Identifier stream
    /// that SmartScreen / Office Protected View key off).
    pub meta_policy: crate::meta::MetaPolicy,
    /// Phase 24 — security-metadata bridge.
    ///
    /// Implemented by `copythat_platform::PlatformMetaOps`. `None`
    /// disables the metadata apply pass even when
    /// [`preserve_security_metadata`](Self::preserve_security_metadata)
    /// is `true`. Kept here as a trait object so callers (the Tauri
    /// shell, the CLI, test harnesses) can plug in alternate
    /// backends without pulling in the platform crate's unsafe FFI.
    pub meta_ops: Option<Arc<dyn crate::meta::MetaOps>>,
    /// Phase 27 — content-defined chunk store sink.
    ///
    /// When `Some`, the engine may consult the sink to report
    /// delta-resume savings, query whether a chunk is already present,
    /// and persist / retrieve per-file manifests. The concrete
    /// implementation lives in `copythat-chunk`
    /// (`CopyThatChunkSink`); kept here as a trait object so the core
    /// engine can stay independent of redb + fastcdc.
    ///
    /// `None` disables the chunk pathway entirely; the engine behaves
    /// exactly as it did pre-Phase-27.
    pub chunk_store: Option<Arc<dyn ChunkStoreSink>>,
    /// Phase 32 — cloud sink bridge.
    ///
    /// When `Some`, the engine routes destination writes through a
    /// registered cloud backend instead of the local filesystem. The
    /// [`CloudSink::put_blocking`] call is invoked with the full read
    /// buffer after the source is staged; the destination path is
    /// interpreted as a key on the remote backend (bucket root,
    /// container root, etc.). Kept as a trait object here so the core
    /// engine stays independent of the `opendal` + `keyring` +
    /// `async-trait` crates pulled in by `copythat-cloud`.
    ///
    /// Phase 32b wires the contract + adapter
    /// (`copythat_cloud::CopyThatCloudSink`); full streaming
    /// integration through `copy_file` follows in a dedicated phase
    /// once the engine's read path is refactored for non-blocking
    /// destination IO. `None` preserves pre-Phase-32 behaviour.
    pub cloud_sink: Option<Arc<dyn CloudSink>>,

    /// Phase 35 — on-the-fly encryption + compression sink.
    ///
    /// When `Some`, the engine short-circuits to the sink's
    /// `transform` method (on a `spawn_blocking` thread) instead of
    /// running its own read-write loop. The sink owns the pipeline:
    /// it opens the source + destination itself, chains zstd / age
    /// transforms, and returns the byte counts via
    /// [`TransformOutcome`].
    ///
    /// Activating the sink is mutually exclusive with
    /// [`fast_copy_hook`](Self::fast_copy_hook) (the transformed
    /// bytes are not byte-exact with the source) and with
    /// [`verify`](Self::verify) (post-copy hash would mismatch by
    /// construction). The engine applies those exclusions
    /// automatically when the sink is present.
    pub transform: Option<Arc<dyn TransformSink>>,

    /// Phase 17 follow-up — destination jail. When `Some`, the
    /// engine refuses to open any destination path that doesn't
    /// canonicalise to a descendant of the configured root.
    /// Closes the absolute-path-without-`..` case the lexical
    /// guard misses (e.g. `/etc/passwd` is path-traversal-free
    /// from a lexical standpoint, but it's outside any user-
    /// chosen staging root).
    ///
    /// The engine canonicalises both `dst` and `dest_jail_root`
    /// before the comparison, so symlinks at the dst-parent level
    /// are followed once and validated. Combined with the
    /// `O_NOFOLLOW` flag on the dst open itself, an attacker can't
    /// race a symlink from outside the jail into a name inside.
    ///
    /// `None` (default) preserves pre-Phase-17-followup behaviour.
    pub dest_jail_root: Option<std::path::PathBuf>,
}

/// Bridge contract for the Phase 32 cloud sink.
///
/// Implemented by `copythat_cloud::CopyThatCloudSink`. Kept in
/// `copythat-core` so [`CopyOptions`] can hold a trait object without
/// pulling `opendal` / `keyring` / `async-trait` into every crate
/// that touches the engine's public API.
///
/// The single entrypoint ([`CloudSink::put_blocking`]) is
/// synchronous-and-blocking by design. Implementations translate
/// the call into whatever async machinery they own. The engine
/// calls it from the tokio-runtime-owned copy task, so spinning on
/// a `block_on` inside an implementation is not safe — implementations
/// should run the actual IO on a dedicated runtime / thread and
/// return its result synchronously.
pub trait CloudSink: Send + Sync + std::fmt::Debug {
    /// Stable display name of the backend this sink is bound to.
    /// Used by engine events + history records.
    fn backend_name(&self) -> &str;

    /// Upload `bytes` to `path` on the configured remote backend.
    /// Returns the number of bytes written on success.
    fn put_blocking(&self, path: &str, bytes: &[u8]) -> Result<u64, String>;

    /// Phase 32f — streaming upload. Reads `source_path` in
    /// `buffer_size` chunks and writes each chunk to the backend
    /// without buffering the full file in memory. `on_progress` is
    /// invoked after each chunk with the running byte count so the
    /// engine can emit `CopyEvent::Progress`.
    ///
    /// Default impl reads the whole file into a Vec and delegates
    /// to `put_blocking` — correct but not streaming.
    /// Implementations backed by a streaming transport (e.g.,
    /// `opendal::Operator::writer()`) should override. Returns the
    /// number of bytes written on success.
    fn put_stream_blocking(
        &self,
        path: &str,
        source_path: &std::path::Path,
        buffer_size: usize,
        on_progress: &dyn Fn(u64),
    ) -> Result<u64, String> {
        let _ = buffer_size;
        let bytes = std::fs::read(source_path).map_err(|e| e.to_string())?;
        let n = bytes.len() as u64;
        on_progress(n);
        self.put_blocking(path, &bytes)
    }
}

/// Bridge contract for the Phase 27 content-defined chunk store.
///
/// Implemented by `copythat_chunk::CopyThatChunkSink`. Kept in
/// `copythat-core` so [`CopyOptions`] can hold a trait object without
/// pulling the redb + fastcdc + blake3 dependencies into every crate
/// that touches the engine's public API.
///
/// The engine consults the sink at two points:
///
/// 1. **Pre-open**, to ask for an existing manifest keyed by the
///    destination path. If one is returned, it caches the chunk
///    hashes and, after the fresh manifest is computed, only
///    re-writes the deltas.
/// 2. **Post-copy**, to persist the freshly-built manifest so the
///    next retry can benefit from the same delta plan.
///
/// All methods are infallible by design (failures are swallowed
/// inside the implementation and may be logged): a chunk-store
/// outage should never break a copy, only defeat the dedup /
/// delta-resume optimisation.
pub trait ChunkStoreSink: Send + Sync + std::fmt::Debug {
    /// Look up a previously-persisted manifest by key. Returns
    /// serialised bytes so the trait stays independent of the chunk
    /// crate's `Manifest` type.
    fn get_manifest(&self, key: &str) -> Option<Vec<u8>>;
    /// Persist a manifest under `key`.
    fn put_manifest(&self, key: &str, serialised: &[u8]);
    /// Quick "is this chunk already indexed?" probe.
    fn has_chunk(&self, hash: &[u8; 32]) -> bool;
}

/// User-selectable copy strategy.
///
/// Controls which acceleration paths `copy_file` attempts when a
/// [`FastCopyHook`] is installed. With no hook installed, the engine
/// always uses the async byte-by-byte loop regardless of strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CopyStrategy {
    /// Try reflink → OS-native → async fallback. The default.
    #[default]
    Auto,
    /// Skip every fast path; always use the async byte-by-byte engine.
    /// Useful for benchmarks and for filesystems where reflink / native
    /// shortcuts have known correctness issues.
    AlwaysAsync,
    /// Try reflink → OS-native; if neither is available, surface an
    /// `IoOther` error rather than silently falling through to the
    /// async engine. Useful for tests that need to assert a specific
    /// fast path actually fired.
    AlwaysFast,
    /// Skip the reflink attempt; OS-native and async fallback still apply.
    /// Useful when the user has observed reflink overhead on a particular
    /// filesystem (rare, but documented for parity with TeraCopy).
    NoReflink,
}

/// Outcome a [`FastCopyHook`] reports back to the engine.
#[derive(Debug)]
pub enum FastCopyHookOutcome {
    /// Hook handled the copy. The included [`CopyReport`] is the truth
    /// the engine returns to its caller.
    Done(CopyReport),
    /// Hook tried every applicable strategy and none was supported on
    /// this src / dst pair. The engine should fall through to its async
    /// loop (unless [`CopyStrategy::AlwaysFast`] was requested, in
    /// which case the engine surfaces an error instead).
    NotSupported,
}

/// Bridge contract for the OS-native fast paths.
///
/// Implemented by `copythat-platform::PlatformFastCopyHook`. Kept in
/// this crate so [`CopyOptions`] can hold a trait object without a
/// dependency cycle.
///
/// The hook receives a *clone* of the active [`CopyOptions`] including
/// itself; implementations must not recursively call back into
/// [`crate::copy_file`] with the same options or they will infinite-loop.
/// Real implementations dispatch to the relevant syscall directly.
pub trait FastCopyHook: Send + Sync + std::fmt::Debug {
    /// Try to copy `src` to `dst` using a fast path. Emits Started /
    /// Progress / Completed events on `events` exactly like the async
    /// engine would. Honours `ctrl` for pause / cancel where the
    /// underlying syscall supports it (most do).
    fn try_copy<'a>(
        &'a self,
        src: PathBuf,
        dst: PathBuf,
        opts: CopyOptions,
        ctrl: CopyControl,
        events: mpsc::Sender<CopyEvent>,
    ) -> Pin<Box<dyn Future<Output = Result<FastCopyHookOutcome, CopyError>> + Send + 'a>>;
}

impl Default for CopyOptions {
    fn default() -> Self {
        Self {
            buffer_size: DEFAULT_BUFFER_SIZE,
            fsync_on_close: false,
            follow_symlinks: true,
            preserve_times: true,
            preserve_permissions: true,
            keep_partial: false,
            fail_if_exists: false,
            verify: None,
            fsync_before_verify: true,
            strategy: CopyStrategy::Auto,
            fast_copy_hook: None,
            on_locked: LockedFilePolicy::default(),
            snapshot_hook: None,
            journal: None,
            journal_file_idx: 0,
            shape: None,
            preserve_sparseness: true,
            paranoid_verify: false,
            sharing_violation_retries: 3,
            sharing_violation_base_delay_ms: 50,
            sparse_ops: None,
            preserve_security_metadata: true,
            meta_policy: crate::meta::MetaPolicy::default(),
            meta_ops: None,
            chunk_store: None,
            cloud_sink: None,
            transform: None,
            dest_jail_root: None,
        }
    }
}

impl CopyOptions {
    /// Return [`CopyOptions::buffer_size`] clamped to the engine's
    /// [`MIN_BUFFER_SIZE`] / [`MAX_BUFFER_SIZE`] window.
    pub fn clamped_buffer_size(&self) -> usize {
        self.buffer_size.clamp(MIN_BUFFER_SIZE, MAX_BUFFER_SIZE)
    }

    /// Phase 13c — dynamic buffer sizing.
    ///
    /// Pick the buffer size the async-fallback loop should actually
    /// use for a file of `file_size` bytes. Rules:
    ///
    /// - **File smaller than the configured buffer**: shrink to the
    ///   next sector-aligned size above the file size (clamped to
    ///   [`MIN_BUFFER_SIZE`]). A 50 KiB file doesn't need a 1 MiB
    ///   allocation; shrinking cuts the per-file memory cost for
    ///   many-small-file trees without hurting throughput.
    /// - **File between 1 MiB and 4 GiB**: use the configured
    ///   buffer verbatim. The Phase 13b buffer-sweep bench confirmed
    ///   1 MiB is the optimum for this band on typical desktop
    ///   hardware (the bench ran on Windows 11 / NTFS).
    /// - **File larger than 4 GiB**: step up toward [`MAX_BUFFER_SIZE`]
    ///   to give the memory subsystem bigger contiguous chunks to
    ///   pipeline. The jump is capped at 4 MiB to stay well clear
    ///   of `BufReader` / `BufWriter`'s internal allocation pain
    ///   point.
    ///
    /// The returned value is always within `[MIN_BUFFER_SIZE,
    /// MAX_BUFFER_SIZE]` and never larger than the caller-configured
    /// `buffer_size` unless the file is > 4 GiB (where the caller's
    /// configured value is treated as a floor rather than a cap).
    pub fn buffer_size_for_file(&self, file_size: u64) -> usize {
        let configured = self.clamped_buffer_size();
        if file_size == 0 {
            return MIN_BUFFER_SIZE;
        }
        // Step 1: cap to the file size (no point allocating more
        // than the file can fill). Align up to the minimum buffer
        // size so we never under-shoot the sector-alignment floor
        // the engine assumes downstream.
        let file_cap = ((file_size as usize).saturating_add(MIN_BUFFER_SIZE - 1))
            .min(configured.max(MIN_BUFFER_SIZE));
        let base = file_cap.max(MIN_BUFFER_SIZE);
        // Step 2: for truly huge files, step up above the
        // configured value — amortizes the per-syscall cost over
        // more bytes at the expense of a larger steady-state RSS.
        const HUGE_FILE_THRESHOLD: u64 = 4 * 1024 * 1024 * 1024; // 4 GiB
        const HUGE_FILE_BUFFER: usize = 4 * 1024 * 1024; // 4 MiB
        if file_size >= HUGE_FILE_THRESHOLD && base < HUGE_FILE_BUFFER {
            return HUGE_FILE_BUFFER.clamp(MIN_BUFFER_SIZE, MAX_BUFFER_SIZE);
        }
        base.clamp(MIN_BUFFER_SIZE, MAX_BUFFER_SIZE)
    }
}

/// Behaviour knobs for `move_file` / `move_tree`.
///
/// Move is modelled as "rename if possible, otherwise copy-then-delete".
/// The copy phase reuses [`CopyOptions`]; these extra knobs govern the
/// *move* layer.
#[derive(Debug, Clone)]
pub struct MoveOptions {
    /// Settings passed through to the internal `copy_file` / `copy_tree`
    /// call on the cross-device fallback path.
    pub copy: CopyOptions,
    /// If true, when the same-volume `rename` fails with anything other
    /// than `CrossesDevices`, surface the error instead of falling back
    /// to copy-then-delete. Defaults to false.
    pub strict_rename: bool,
}

impl Default for MoveOptions {
    fn default() -> Self {
        Self {
            copy: CopyOptions {
                // fsync the destination on the move fallback — we
                // unlink the source afterwards, so the cost of an
                // extra sync is justified by not losing data on a
                // crash between flush and unlink.
                fsync_on_close: true,
                ..CopyOptions::default()
            },
            strict_rename: false,
        }
    }
}

/// Default concurrency for `copy_tree` / `move_tree`. Deliberately
/// conservative — Phase 6 will pick this from per-volume SSD / HDD
/// detection.
pub const DEFAULT_TREE_CONCURRENCY: usize = 4;

/// What the tree engine should do when a per-file copy fails.
///
/// Separate from `CollisionPolicy` — that resolves "destination already
/// exists" *before* the copy starts; this governs "the copy started
/// and the filesystem said no" (permission denied, disk full,
/// interrupted, …).
///
/// Added in Phase 8 so the UI can surface a retry / skip dialog. The
/// default is `Abort` to preserve the pre-Phase-8 tree semantics
/// (one failure aborts the whole tree); the Tauri runner opts into
/// `Ask` when the user has not overridden the Settings policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ErrorPolicy {
    /// Emit `CopyEvent::ErrorPrompt` and block the failing file's
    /// task until the consumer replies via a one-shot. The consumer
    /// picks `ErrorAction::Retry` / `Skip` / `Abort`.
    Ask,
    /// Record the error as a `CopyEvent::FileError` event + `errored`
    /// counter increment, then continue the tree.
    Skip,
    /// Retry the failing copy up to `max_attempts` times with a
    /// fixed backoff, then fall through to `Skip` on exhaustion.
    RetryN {
        /// Maximum re-tries (does NOT count the initial attempt).
        /// Clamped to `[0, 10]` by the engine.
        max_attempts: u8,
        /// Sleep between retries, in milliseconds.
        /// Clamped to `[0, 10_000]` by the engine.
        backoff_ms: u64,
    },
    /// Cancel the whole tree on first per-file failure. Pre-Phase-8
    /// behaviour; kept as the default so existing callers see no
    /// behaviour change until they opt in.
    #[default]
    Abort,
}

/// Consumer response to a `CopyEvent::ErrorPrompt`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorAction {
    /// Re-run `copy_file` for the failing entry. If the retry also
    /// fails, the engine emits another `ErrorPrompt` (so a loop of
    /// "keep retrying" is up to the consumer).
    Retry,
    /// Record the error as a `FileError` event + `errored` counter;
    /// continue the rest of the tree.
    Skip,
    /// Cancel the tree. Same outcome as `ErrorPolicy::Abort`.
    Abort,
}

/// Behaviour knobs for `copy_tree` / `move_tree`.
#[derive(Debug, Clone)]
pub struct TreeOptions {
    /// Per-file copy behaviour. Applied uniformly to every file in the
    /// tree.
    pub file: CopyOptions,
    /// How to resolve an existing destination. Default: `Skip`.
    pub collision: crate::collision::CollisionPolicy,
    /// What to do when a per-file copy *fails* (as opposed to "dst
    /// already exists"). Default: `Abort` — pre-Phase-8 behaviour.
    pub on_error: ErrorPolicy,
    /// Maximum concurrent file copies. Clamped to `[1, 64]`.
    pub concurrency: usize,
    /// If true, follow symlinks found *inside* the source tree and
    /// descend into the target. If false (default), reproduce them as
    /// symlinks at the destination — matches the intuitive "copy this
    /// folder, do not chase shortcuts" behaviour and prevents cycles.
    pub follow_symlinks_in_tree: bool,
    /// If true, preserve mtime / atime on every *directory* in
    /// addition to every file. Defaults to true.
    pub preserve_directory_times: bool,
    /// Phase 14 — destination free-space reserve, in bytes.
    ///
    /// When `> 0`, the tree engine re-probes the destination volume's
    /// free bytes before each file and halts cleanly (emitting a
    /// `TreeStopped` event + terminal state "succeeded_partial") if
    /// writing the next file would push free space below this reserve.
    /// `0` disables the guard; the Phase 1-13 behaviour (copy until
    /// the volume is physically full) is preserved for opt-out
    /// callers. The UI preflight check surfaces overflow before the
    /// engine is even entered; this field is the safety net that
    /// catches files that grow during the copy or partial-fit
    /// selections that still happen to overflow.
    pub reserve_dst_bytes: u64,
    /// Phase 14a — enumeration-time include/exclude filters.
    ///
    /// When `Some(set)` and `set.is_empty() == false`, the walker
    /// compiles the filter once and applies it per entry: files
    /// failing the include / exclude / size / date / attribute checks
    /// are omitted from the plan entirely, and directories matching
    /// an exclude glob prune their whole subtree. See
    /// [`crate::filter`] for the precise semantics.
    pub filters: Option<FilterSet>,
}

impl Default for TreeOptions {
    fn default() -> Self {
        Self {
            file: CopyOptions::default(),
            collision: crate::collision::CollisionPolicy::Skip,
            on_error: ErrorPolicy::default(),
            concurrency: DEFAULT_TREE_CONCURRENCY,
            follow_symlinks_in_tree: false,
            preserve_directory_times: true,
            reserve_dst_bytes: 0,
            filters: None,
        }
    }
}

impl TreeOptions {
    pub(crate) fn clamped_concurrency(&self) -> usize {
        self.concurrency.clamp(1, 64)
    }

    /// Clamp the RetryN knobs so a pathological config (e.g.
    /// `max_attempts: 255, backoff_ms: u64::MAX`) can't freeze the
    /// tree. Non-`RetryN` policies pass through unchanged.
    pub(crate) fn clamped_on_error(&self) -> ErrorPolicy {
        match self.on_error {
            ErrorPolicy::RetryN {
                max_attempts,
                backoff_ms,
            } => ErrorPolicy::RetryN {
                max_attempts: max_attempts.min(10),
                backoff_ms: backoff_ms.min(10_000),
            },
            other => other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer_size_for_file_tiny_file_shrinks_below_configured() {
        let opts = CopyOptions::default();
        // 50 KiB file, 1 MiB configured → shrink but stay at/above MIN.
        let buf = opts.buffer_size_for_file(50 * 1024);
        assert!(buf >= MIN_BUFFER_SIZE);
        assert!(
            buf < DEFAULT_BUFFER_SIZE,
            "tiny files should not allocate the full 1 MiB default"
        );
    }

    #[test]
    fn buffer_size_for_file_zero_file_returns_min() {
        let opts = CopyOptions::default();
        assert_eq!(opts.buffer_size_for_file(0), MIN_BUFFER_SIZE);
    }

    #[test]
    fn buffer_size_for_file_medium_file_uses_configured() {
        // 100 MiB file with default (1 MiB) buffer → use 1 MiB verbatim.
        let opts = CopyOptions::default();
        let buf = opts.buffer_size_for_file(100 * 1024 * 1024);
        assert_eq!(buf, DEFAULT_BUFFER_SIZE);
    }

    #[test]
    fn buffer_size_for_file_huge_file_bumps_up() {
        // 10 GiB file should step up from 1 MiB default to 4 MiB.
        let opts = CopyOptions::default();
        let buf = opts.buffer_size_for_file(10 * 1024 * 1024 * 1024);
        assert_eq!(buf, 4 * 1024 * 1024);
    }

    #[test]
    fn buffer_size_for_file_respects_max_ceiling() {
        // Even a 1 TiB file must not exceed MAX_BUFFER_SIZE.
        let opts = CopyOptions {
            buffer_size: MAX_BUFFER_SIZE * 2,
            ..Default::default()
        };
        let buf = opts.buffer_size_for_file(1024u64.pow(4));
        assert!(buf <= MAX_BUFFER_SIZE);
    }

    #[test]
    fn buffer_size_for_file_small_file_caps_at_configured() {
        // 10 KiB file, 64 KiB configured → return MIN (64 KiB), not 10 KiB.
        let opts = CopyOptions {
            buffer_size: MIN_BUFFER_SIZE,
            ..Default::default()
        };
        let buf = opts.buffer_size_for_file(10 * 1024);
        assert_eq!(buf, MIN_BUFFER_SIZE);
    }

    #[test]
    fn buffer_size_for_file_does_not_exceed_configured_on_medium() {
        // 2 GiB with a user-configured 2 MiB should stay at 2 MiB
        // (not bump up to 4 MiB — HUGE_FILE_THRESHOLD guards that).
        let opts = CopyOptions {
            buffer_size: 2 * 1024 * 1024,
            ..Default::default()
        };
        let buf = opts.buffer_size_for_file(2 * 1024 * 1024 * 1024);
        assert_eq!(buf, 2 * 1024 * 1024);
    }
}
