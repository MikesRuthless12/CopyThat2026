//! Phase 49 (moonshot) — the unified sync + backup repository.
//!
//! Phase 27 gave us a content-defined [`ChunkStore`](crate::ChunkStore):
//! every distinct chunk is stored once, keyed by its BLAKE3 digest.
//! Phase 49 builds *one repository concept* on top of it so that a
//! copy job, a sync run, and a versioning checkpoint all land in the
//! same content-addressed store and become queryable from a single
//! timeline — "every byte stored once; every past state queryable from
//! the same view".
//!
//! # What this module is (and is not)
//!
//! [`Repository`] **owns** a [`ChunkStore`] as a sub-component and adds
//! a snapshot catalog in a sibling redb file (`<root>/repository.redb`):
//!
//! - a **snapshot catalog** ([`SNAPSHOTS`]) — the full [`Snapshot`]s,
//!   each a set of [`FileEntry`]s (logical path + its [`Manifest`])
//!   tagged with a [`SnapshotKind`] and a millisecond timestamp; and
//! - a **summaries index** ([`SUMMARIES`]) — one lightweight
//!   [`UnifiedSnapshot`] per snapshot so the timeline listing never
//!   deserializes a single chunk manifest.
//!
//! Chunk lifetime is managed by **reference counting at GC time**:
//! [`Repository::gc`] walks every manifest source — the snapshots *and*
//! the [`ChunkStore`]'s own Phase 27 `manifests` table — to compute the
//! reachable set, then sweeps everything else. (An earlier draft kept a
//! persisted `chunks_refcount` table, but GC necessarily recomputes
//! reachability from the manifests anyway, so the table was redundant
//! state that could only drift; [`Repository::chunk_refcount`] now
//! derives the count on demand.)
//!
//! Deliberately, this layer is **self-contained**: it pulls in no new
//! dependencies and stays synchronous + `#![forbid(unsafe_code)]` like
//! the rest of `copythat-chunk`. It does *not* embed the async
//! `copythat-history` / `copythat-sync` / `copythat-journal` databases
//! or the platform-`unsafe` `copythat-audit` sink — instead it is the
//! shared content + timeline layer those engines record *into*.
//!
//! # Timestamps
//!
//! Callers pass `created_at_ms: i64` — milliseconds since the Unix epoch
//! — matching the `*_ms` convention already used across
//! `copythat-history`, `-sync`, and `-journal`. The caller owns the
//! clock; the repository only ever orders by the value it was given.
//!
//! # Concurrency
//!
//! All mutating entry points take a shared *read* lease on an internal
//! [`RwLock`]; [`Repository::gc`] takes the exclusive *write* lease.
//! Recording snapshots can therefore run concurrently with each other,
//! but GC can never observe a half-written snapshot (chunks committed to
//! the store but whose snapshot row has not yet landed) — which would
//! otherwise let GC sweep live chunks.
//!
//! # Crash consistency
//!
//! Recording a snapshot writes chunks to the [`ChunkStore`] first (each
//! `put` is its own ACID redb transaction), then commits the snapshot
//! row + summary in a single `repository.redb` transaction. A crash
//! *between* the two leaves chunks that no snapshot references —
//! harmless orphans that the next [`Repository::gc`] reclaims. There is
//! no window in which a committed snapshot points at a missing chunk.
//!
//! # Example
//!
//! ```no_run
//! use copythat_chunk::{Repository, SnapshotKind};
//!
//! let repo = Repository::open(std::path::Path::new("/tmp/repo"))?;
//! let id = repo.snapshot_bytes(
//!     SnapshotKind::Copy,
//!     "nightly copy of /docs",
//!     1_700_000_000_000,
//!     &[("/docs/report.pdf", b"...file bytes...")],
//! )?;
//! println!("recorded snapshot {}", id.as_u64());
//! for s in repo.snapshots()? {
//!     println!("{:?} {} ({} files)", s.kind, s.label, s.file_count);
//! }
//! let report = repo.gc()?;
//! println!("gc swept {} orphan chunks", report.chunks_swept);
//! # Ok::<(), copythat_chunk::ChunkStoreError>(())
//! ```

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use copythat_core::filter::CompiledFilters;
use copythat_core::versioning::{RetentionPolicy, VersionEntry, select_for_pruning};
use redb::{Database, ReadableTable, ReadableTableMetadata, TableDefinition};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::chunker::Chunker;
use crate::compress::{ChunkCodec, RepoCompression};
use crate::error::{ChunkStoreError, Result};
use crate::keyfile::KeySlotInfo;
use crate::manifest::{chunk_into_store, materialise_file};
use crate::store::{ChunkStore, default_chunk_store_path};
use crate::types::{Blake3Hash, ChunkRef, Manifest, hex_of};

/// Snapshot catalog: snapshot id (`u64`) → JSON-serialised [`Snapshot`].
const SNAPSHOTS: TableDefinition<'_, u64, &[u8]> = TableDefinition::new("snapshots");

/// Lightweight per-snapshot summary ([`UnifiedSnapshot`]) so the
/// timeline listing never touches chunk manifests.
const SUMMARIES: TableDefinition<'_, u64, &[u8]> = TableDefinition::new("snapshot_summaries");

/// Monotonic-id allocator (`"next-snapshot-id"` → next free `u64`).
const SEQ: TableDefinition<'_, &str, u64> = TableDefinition::new("seq");

/// Key under [`SEQ`] holding the next snapshot id to hand out.
const SEQ_NEXT_SNAPSHOT_ID: &str = "next-snapshot-id";

/// What produced a snapshot. Unifies the three previously-separate
/// histories (Phase 9 copy log, Phase 25 sync state, Phase 42 version
/// checkpoints) plus an explicit backup snapshot, into one timeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SnapshotKind {
    /// A copy/move job (Phase 1–9).
    Copy,
    /// A two-way sync round (Phase 25).
    Sync,
    /// A version checkpoint of an overwritten/watched file (Phase 42).
    Version,
    /// An explicit, user-triggered backup snapshot of a folder.
    Backup,
}

impl SnapshotKind {
    /// Stable lowercase tag (matches the on-disk kebab-case form).
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Copy => "copy",
            Self::Sync => "sync",
            Self::Version => "version",
            Self::Backup => "backup",
        }
    }
}

/// Opaque identifier for a recorded [`Snapshot`]. Monotonic, starts at 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SnapshotId(pub u64);

impl SnapshotId {
    /// The raw `u64`.
    #[must_use]
    pub const fn as_u64(self) -> u64 {
        self.0
    }
}

impl std::fmt::Display for SnapshotId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// One file inside a [`Snapshot`]: a logical path plus the [`Manifest`]
/// describing the chunks that reconstruct it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileEntry {
    /// Logical path this entry was captured under. Opaque to the
    /// repository — callers typically use the source or destination
    /// absolute path.
    pub path: String,
    /// Chunk manifest (Phase 27). Concatenating its chunks'
    /// bytes reconstructs the file.
    pub manifest: Manifest,
}

/// A recorded point-in-time state of one or more files.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Snapshot {
    /// Monotonic id.
    pub id: u64,
    /// What produced this snapshot.
    pub kind: SnapshotKind,
    /// Caller-supplied capture time, milliseconds since the Unix epoch.
    pub created_at_ms: i64,
    /// Human-readable label (job description, source root, …).
    pub label: String,
    /// Phase 49e — the backup source this snapshot belongs to (the
    /// source's stable id), or `None` for ad-hoc copy/sync/version
    /// snapshots. `#[serde(default)]` so existing on-disk rows load as
    /// `None`; [`Repository::prune_source`] groups retention by this key.
    #[serde(default)]
    pub source_key: Option<String>,
    /// Phase 49p — user-editable free-text description (`#[serde(default)]`
    /// → old rows load empty).
    #[serde(default)]
    pub description: String,
    /// Phase 49p — user tags.
    #[serde(default)]
    pub tags: Vec<String>,
    /// Phase 49p — pinned snapshots are never pruned, regardless of policy.
    #[serde(default)]
    pub pinned: bool,
    /// Phase 49l — common directory prefix of this snapshot's files
    /// (computed at record time), used to group the Sources dashboard.
    /// `#[serde(default)]` → old rows load as `""` (unknown source).
    #[serde(default)]
    pub source: String,
    /// The files captured in this snapshot.
    pub files: Vec<FileEntry>,
}

impl Snapshot {
    /// The distinct chunk hashes this snapshot references.
    fn distinct_chunks(&self) -> HashSet<Blake3Hash> {
        let mut set = HashSet::new();
        for f in &self.files {
            for c in &f.manifest.chunks {
                set.insert(c.hash);
            }
        }
        set
    }

    /// Sum of the logical sizes of every file in this snapshot.
    fn total_size(&self) -> u64 {
        self.files.iter().map(|f| f.manifest.size).sum()
    }

    /// Build the lightweight summary persisted alongside the snapshot.
    fn summary(&self) -> UnifiedSnapshot {
        UnifiedSnapshot {
            id: self.id,
            kind: self.kind,
            created_at_ms: self.created_at_ms,
            label: self.label.clone(),
            source_key: self.source_key.clone(),
            description: self.description.clone(),
            tags: self.tags.clone(),
            pinned: self.pinned,
            file_count: self.files.len() as u64,
            total_size: self.total_size(),
            source: self.source.clone(),
        }
    }

    /// Phase 49l — the longest common directory prefix of this snapshot's
    /// file paths (handles both `/` and `\`), for the Sources dashboard.
    /// Falls back to `label` when the files share no prefix or the snapshot
    /// is empty.
    fn compute_source(&self) -> String {
        fn parts(p: &str) -> Vec<&str> {
            p.split(['/', '\\']).filter(|s| !s.is_empty()).collect()
        }
        let mut files = self.files.iter();
        let Some(first) = files.next() else {
            return self.label.clone();
        };
        // Directory components of the first file (drop the file name).
        let mut prefix: Vec<&str> = parts(&first.path);
        prefix.pop();
        for f in files {
            let p = parts(&f.path);
            let mut i = 0;
            while i < prefix.len() && i + 1 < p.len() && prefix[i] == p[i] {
                i += 1;
            }
            prefix.truncate(i);
            if prefix.is_empty() {
                break;
            }
        }
        if prefix.is_empty() {
            self.label.clone()
        } else {
            format!("/{}", prefix.join("/"))
        }
    }
}

/// Lightweight summary of a [`Snapshot`] for timeline listings — no
/// manifests, persisted in its own index so the whole catalog is cheap
/// to return.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnifiedSnapshot {
    /// Snapshot id.
    pub id: u64,
    /// What produced it.
    pub kind: SnapshotKind,
    /// Capture time, ms since epoch.
    pub created_at_ms: i64,
    /// Human-readable label.
    pub label: String,
    /// Phase 49e — backup source id this snapshot belongs to (`None`
    /// for ad-hoc copy/sync/version snapshots). Mirrors
    /// [`Snapshot::source_key`]; `#[serde(default)]` for old rows.
    #[serde(default)]
    pub source_key: Option<String>,
    /// Phase 49p — mirrors [`Snapshot::description`].
    #[serde(default)]
    pub description: String,
    /// Phase 49p — mirrors [`Snapshot::tags`].
    #[serde(default)]
    pub tags: Vec<String>,
    /// Phase 49p — mirrors [`Snapshot::pinned`] (drives the pin badge + prune-protect).
    #[serde(default)]
    pub pinned: bool,
    /// Number of files captured.
    pub file_count: u64,
    /// Sum of logical file sizes (the effective bytes this snapshot
    /// represents, before dedup).
    pub total_size: u64,
    /// Phase 49l — common directory prefix of the snapshot's files (mirrors
    /// [`Snapshot::source`]). `#[serde(default)]` → old summaries load `""`.
    #[serde(default)]
    pub source: String,
}

/// Phase 49l — one row of the Sources dashboard, folded from summaries only
/// (no pack/manifest I/O).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceSummary {
    /// The source directory prefix (`""` = unknown/unspecified).
    pub source: String,
    /// How many snapshots reference this source.
    pub snapshot_count: u64,
    /// Capture time of the newest snapshot for this source.
    pub latest_ms: i64,
    /// Kind of the newest snapshot.
    pub latest_kind: SnapshotKind,
    /// `total_size` of the newest snapshot for this source.
    pub latest_size: u64,
    /// `file_count` of the newest snapshot for this source.
    pub total_files: u64,
}

/// A single file resolved out of the timeline by [`Repository::snapshot_at`]
/// — carries the manifest so it can be handed straight to
/// [`Repository::restore`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileSnapshot {
    /// Logical path.
    pub path: String,
    /// The snapshot this version came from.
    pub snapshot_id: u64,
    /// What produced that snapshot.
    pub kind: SnapshotKind,
    /// When that snapshot was captured, ms since epoch.
    pub created_at_ms: i64,
    /// Chunk manifest for restoring the file.
    pub manifest: Manifest,
}

/// Aggregate numbers for the repository — drives the "💾 X stored,
/// serving Y effective (Z% saved)" hero readout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct RepoStats {
    /// Physical bytes on disk across all pack files (includes not-yet-
    /// reclaimed dead space, so this is "disk used", not "live data").
    pub stored_bytes: u64,
    /// Sum of the lengths of every *distinct* chunk referenced by a
    /// snapshot — the deduplicated logical size of the live content,
    /// independent of pack overhead. Always `<= effective_bytes`.
    pub unique_bytes: u64,
    /// Phase 49h — sum of the ON-DISK (stored, possibly zstd-compressed)
    /// sizes of those same distinct reachable chunks. `<= unique_bytes`;
    /// [`Self::compression_ratio`] is derived from the two.
    pub physical_unique_bytes: u64,
    /// Sum of logical file sizes across every snapshot (what the store
    /// would cost without dedup + history sharing).
    pub effective_bytes: u64,
    /// Number of snapshots in the catalog.
    pub snapshot_count: u64,
    /// Number of distinct chunks currently indexed.
    pub chunk_count: u64,
}

impl RepoStats {
    /// Fraction of effective bytes saved by deduplication, in `0.0..=1.0`.
    ///
    /// Computed from [`Self::unique_bytes`] (the live deduplicated
    /// content size), **not** from [`Self::stored_bytes`] — physical
    /// pack files can carry not-yet-reclaimed dead space that would
    /// otherwise make a working dedup look like "0% saved". Returns
    /// `0.0` for an empty repository.
    #[must_use]
    pub fn saved_ratio(&self) -> f64 {
        if self.effective_bytes == 0 {
            return 0.0;
        }
        // unique_bytes <= effective_bytes by construction; min() is
        // belt-and-suspenders so the ratio can never go negative.
        let unique = self.unique_bytes.min(self.effective_bytes) as f64;
        1.0 - (unique / self.effective_bytes as f64)
    }

    /// Phase 49h — fraction of unique logical bytes saved by COMPRESSION
    /// (on-disk physical vs logical of the distinct live chunks), in
    /// `0.0..=1.0`. Returns `0.0` when there's nothing stored or nothing
    /// compressed. Orthogonal to [`Self::saved_ratio`] (which is dedup).
    #[must_use]
    pub fn compression_ratio(&self) -> f64 {
        if self.unique_bytes == 0 {
            return 0.0;
        }
        let phys = self.physical_unique_bytes.min(self.unique_bytes) as f64;
        1.0 - (phys / self.unique_bytes as f64)
    }
}

/// Outcome of a [`Repository::gc`] pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct GcReport {
    /// Chunks whose index entry was removed because no manifest source
    /// references them any more.
    pub chunks_swept: u64,
    /// Pack files deleted because every chunk they held was swept.
    pub packs_removed: u64,
    /// Disk bytes reclaimed by deleting those whole pack files.
    ///
    /// Note: only *fully*-dead, non-active pack files are deleted.
    /// Reclaiming the dead bytes inside a partially-live pack requires
    /// rewriting its survivors into a fresh pack — that compaction is a
    /// deferred follow-up (see the module docs), so a GC that sweeps
    /// index entries out of the still-live active pack can legitimately
    /// report `chunks_swept > 0` with `bytes_reclaimed == 0`.
    pub bytes_reclaimed: u64,
}

/// Outcome of a [`Repository::compact`] compaction pass (Phase 49i).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CompactReport {
    /// Below-threshold packs whose survivors were rewritten + deleted.
    pub packs_compacted: u64,
    /// Live chunks moved into the active pack during compaction.
    pub chunks_moved: u64,
    /// Disk bytes reclaimed by deleting the compacted pack files.
    pub bytes_reclaimed: u64,
}

/// Outcome of a [`Repository::replicate_to`] pass (Phase 50h). A push of
/// this repo's snapshots (and only the chunks the destination is missing)
/// into another [`Repository`]. Re-running is idempotent — a snapshot whose
/// content already exists in the destination is skipped, and a chunk the
/// destination already holds is never re-sent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ReplicateReport {
    /// Snapshots newly written into the destination.
    pub snapshots_copied: u64,
    /// Snapshots skipped because identical content was already present.
    pub snapshots_skipped: u64,
    /// Distinct chunks transferred (destination was missing them).
    pub chunks_copied: u64,
    /// Distinct referenced chunks the destination already held (deduped away).
    pub chunks_present: u64,
    /// Logical (plaintext) bytes transferred.
    pub bytes_copied: u64,
}

/// Live progress for a maintenance pass (Phase 49i). `phase` is one of
/// `"mark"`, `"sweep"`, `"compact"`; `done`/`total` are meaningful for the
/// compaction phase.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaintenanceProgress {
    /// `"mark"` | `"sweep"` | `"compact"`.
    pub phase: &'static str,
    /// Items processed so far in this phase.
    pub done: u64,
    /// Total items in this phase (0 if not enumerable).
    pub total: u64,
}

/// Options for [`Repository::compact`] (Phase 49i).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CompactOptions {
    /// Compact a non-active pack only when its dead fraction is at least
    /// this (default 0.5 = "≥50% dead"). `1.0` = only fully-dead packs;
    /// `0.0` = rewrite every pack.
    pub min_dead_ratio: f64,
}

impl Default for CompactOptions {
    fn default() -> Self {
        Self {
            min_dead_ratio: 0.5,
        }
    }
}

/// Phase 49n — how deep [`Repository::verify`] checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerifyLevel {
    /// Index-only: every referenced chunk has a live index entry
    /// (`ChunkStore::locate`). Fast, no pack reads.
    Metadata,
    /// Read every referenced chunk back (`ChunkStore::get` re-hashes BLAKE3
    /// → `CorruptChunk` on bit-rot) AND verify each file: the concatenated
    /// chunk bytes must BLAKE3 to `manifest.file_hash`.
    ReadData,
}

/// Phase 49n — one damaged reference found by [`Repository::verify`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DamageKind {
    /// The chunk has no index entry (garbage-collected / never stored).
    Missing,
    /// The chunk's bytes failed BLAKE3 verification on read-back.
    Corrupt,
    /// The file's concatenated chunk bytes don't hash to `file_hash`.
    FileHashMismatch,
}

/// Phase 49n — one row of damage in a [`VerifyReport`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerifyDamage {
    /// The snapshot the damaged file belongs to.
    pub snapshot_id: u64,
    /// The file path within the snapshot.
    pub path: String,
    /// Hex chunk hash (empty for a `FileHashMismatch`, which is file-level).
    pub chunk_hash_hex: String,
    /// What kind of damage.
    pub kind: DamageKind,
}

/// Phase 49n — outcome of a verify pass.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct VerifyReport {
    /// Snapshots inspected.
    pub snapshots_checked: u64,
    /// Files inspected.
    pub files_checked: u64,
    /// Chunk references inspected.
    pub chunks_checked: u64,
    /// Every damaged reference found (empty = clean).
    pub damage: Vec<VerifyDamage>,
}

impl VerifyReport {
    /// `true` when no damage was found.
    #[must_use]
    pub fn is_clean(&self) -> bool {
        self.damage.is_empty()
    }
}

/// Outcome of a [`Repository::prune_source`] retention pass — the
/// catalog decision plus the [`Repository::gc`] it triggered to reclaim
/// the now-orphaned chunks. (Phase 49e.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PruneReport {
    /// Snapshots removed from the catalog by the retention policy.
    pub snapshots_removed: u64,
    /// Chunks swept by the follow-up [`Repository::gc`].
    pub chunks_swept: u64,
    /// Disk bytes reclaimed by deleting now-dead pack files.
    pub bytes_reclaimed: u64,
}

/// One file in a snapshot's flat tree listing — logical path + size, no
/// chunk manifest (the UI assembles the tree; manifests never cross the
/// IPC boundary). Returned by [`Repository::snapshot_tree`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotFileEntry {
    /// Logical path the file was captured under.
    pub path: String,
    /// Logical (uncompressed) size in bytes.
    pub size: u64,
}

/// How [`Repository::restore_paths`] resolves a destination file that
/// already exists. Same vocabulary as the engine's collision UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestoreConflict {
    /// Replace the existing file.
    Overwrite,
    /// Leave the existing file; don't restore this one.
    Skip,
    /// Restore alongside under a ` (1)` / ` (2)` … suffixed name.
    KeepBoth,
}

/// Tally returned by [`Repository::restore_paths`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreReport {
    /// Files written to the destination.
    pub restored: u64,
    /// Files skipped (existing + [`RestoreConflict::Skip`], or not present
    /// in the snapshot).
    pub skipped: u64,
    /// Files that errored (missing chunk, unwritable destination, or an
    /// unsafe logical path that would escape the restore root).
    pub failed: u64,
}

/// Summary of a [`Repository::snapshot_source`] run (Phase 49g).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceSnapshotSummary {
    /// The recorded snapshot's id.
    pub id: SnapshotId,
    /// Files captured (those that passed the filters).
    pub files: u64,
    /// Sum of the logical sizes of the captured files.
    pub bytes: u64,
    /// Symlinks + non-regular files skipped (counted, not captured).
    pub skipped_non_regular: u64,
}

/// Phase 49p — a global keep-last / keep-within prune policy. Pinned
/// snapshots are always retained regardless of these limits; a snapshot
/// survives if it is pinned, among the `keep_last` newest, OR newer than
/// the `keep_within_ms` cutoff (the limits union, restic-style). An
/// all-`None` policy prunes nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PrunePolicy {
    /// Keep the newest `n` snapshots across the whole repository.
    /// `None` = no count limit.
    pub keep_last: Option<u64>,
    /// Keep snapshots newer than `now_ms - keep_within_ms`. `None` = no
    /// age limit.
    pub keep_within_ms: Option<i64>,
}

/// The Phase 49 unified repository: a [`ChunkStore`] plus a snapshot
/// catalog and reference-counted garbage collection.
pub struct Repository {
    store: Arc<ChunkStore>,
    db: Database,
    chunker: Chunker,
    root: PathBuf,
    /// Phase 49h — codec policy applied to chunks recorded via this
    /// repository's snapshot paths (default `Off`). Existing chunks are
    /// never rewritten (dedup-by-hash; the first writer's codec wins).
    compression: RepoCompression,
    /// Mutators take a read lease; [`Repository::gc`] takes the write
    /// lease so it can never run while a snapshot is mid-record.
    gc_lock: RwLock<()>,
}

impl std::fmt::Debug for Repository {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Repository")
            .field("root", &self.root)
            .finish()
    }
}

impl Repository {
    /// Open (or create) a repository rooted at `root`.
    ///
    /// Lays out `<root>/index.redb` plus `<root>/packs/` (the
    /// [`ChunkStore`] sub-component) and `<root>/repository.redb` (the
    /// snapshot catalog). Both redb files are independent and each takes
    /// its own file lock.
    pub fn open(root: &Path) -> Result<Self> {
        Self::with_store(Arc::new(ChunkStore::open(root)?))
    }

    /// Build a repository over an **already-open** [`ChunkStore`], opening
    /// only the sibling `<root>/repository.redb` snapshot catalog.
    ///
    /// This is the constructor the application uses so the chunk store's
    /// single redb file lock (`index.redb`) is taken **once** and shared:
    /// the same `Arc<ChunkStore>` backs the delta-resume
    /// [`CopyThatChunkSink`](crate::CopyThatChunkSink), the recovery web
    /// UI, the mount backend, and this repository's snapshot catalog.
    /// [`Self::open`] delegates here after opening the store itself.
    pub fn with_store(store: Arc<ChunkStore>) -> Result<Self> {
        let root = store.root().to_path_buf();
        let db_path = root.join("repository.redb");
        let db = Database::create(&db_path)?;
        // Materialise the catalog tables on first open.
        {
            let txn = db.begin_write()?;
            let _ = txn.open_table(SNAPSHOTS)?;
            let _ = txn.open_table(SUMMARIES)?;
            let _ = txn.open_table(SEQ)?;
            txn.commit()?;
        }
        Ok(Self {
            store,
            db,
            chunker: Chunker::default(),
            root,
            compression: RepoCompression::default(),
            gc_lock: RwLock::new(()),
        })
    }

    /// Open at `root` with a chunk-compression policy (Phase 49h). Chunks
    /// recorded via the snapshot paths use `compression`; `Off` matches
    /// [`Self::open`]. Reads return plaintext regardless of codec.
    pub fn open_with_compression(root: &Path, compression: RepoCompression) -> Result<Self> {
        let mut repo = Self::open(root)?;
        repo.compression = compression;
        Ok(repo)
    }

    /// Set the chunk-compression policy for subsequent records (Phase 49h).
    pub fn set_compression(&mut self, compression: RepoCompression) {
        self.compression = compression;
    }

    /// The active chunk-compression policy.
    #[must_use]
    pub fn compression(&self) -> RepoCompression {
        self.compression
    }

    /// Create a NEW repository at `root`. Errors [`ChunkStoreError::AlreadyExists`]
    /// if one is already initialised there. Writes the CDR-0 descriptor
    /// (`cdr.toml`) and, when `password` is given, a `repo-key.json` access
    /// verifier. (Phase 49k.)
    pub fn create(root: &Path, password: Option<&str>) -> Result<Self> {
        if root.join("cdr.toml").is_file() || root.join("repository.redb").is_file() {
            return Err(ChunkStoreError::AlreadyExists(root.to_path_buf()));
        }
        let store = Arc::new(ChunkStore::open(root)?);
        let repo = Self::with_store(store)?;
        crate::migrate::write_cdr_descriptor(root)
            .map_err(|e| ChunkStoreError::Redb(format!("write descriptor: {e}")))?;
        if let Some(pw) = password {
            crate::keyfile::write(root, pw)?;
        }
        Ok(repo)
    }

    /// Open an EXISTING repository at `root`. Errors
    /// [`ChunkStoreError::NotInitialised`] if neither `cdr.toml` nor
    /// `repository.redb` exists; [`ChunkStoreError::Locked`] /
    /// [`ChunkStoreError::BadPassphrase`] on the passphrase gate. (Phase 49k.)
    pub fn open_existing(root: &Path, password: Option<&str>) -> Result<Self> {
        if !root.join("cdr.toml").is_file() && !root.join("repository.redb").is_file() {
            return Err(ChunkStoreError::NotInitialised(root.to_path_buf()));
        }
        crate::keyfile::verify(root, password)?;
        Self::open(root)
    }

    /// True if a passphrase verifier exists at `root` (UI: show the unlock
    /// field). Static — checkable before opening. (Phase 49k.)
    #[must_use]
    pub fn requires_passphrase(root: &Path) -> bool {
        crate::keyfile::exists(root)
    }

    /// Rotate the access credential `old` authenticates to `new`, leaving
    /// every *other* key slot intact. `old` must grant access (or be absent
    /// when no gate is set). Gates ACCESS only — Phase 51 owns at-rest
    /// encryption. (Phase 49k; slot-aware since Phase 50i.)
    pub fn change_password(&self, old: Option<&str>, new: &str) -> Result<()> {
        crate::keyfile::rotate(&self.root, old, new)
    }

    /// Phase 50i — the repository's key slots (labels + kinds; no secrets).
    /// Empty when the repository has no passphrase gate.
    pub fn list_keys(&self) -> Result<Vec<KeySlotInfo>> {
        crate::keyfile::list_slots(&self.root)
    }

    /// Phase 50i — add a new password slot `label` unlocking with
    /// `new_password`, so several people / devices can each open the repo.
    /// `auth` must already grant access; errors if `label` is taken.
    pub fn add_key(&self, auth: Option<&str>, new_password: &str, label: &str) -> Result<()> {
        crate::keyfile::add_slot(&self.root, auth, new_password, label)
    }

    /// Phase 50i — remove the key slot named `label`. Returns `false` if no
    /// such slot exists; errors if it is the last slot (removing it would
    /// silently unlock the repository).
    pub fn remove_key(&self, label: &str) -> Result<bool> {
        crate::keyfile::remove_slot(&self.root, label)
    }

    /// Phase 50i — generate a recovery key, returned **once** (only its
    /// verifier is stored; there is no way to recover the string later). It
    /// unlocks the repo like any password and replaces any prior recovery
    /// key. `auth` must already grant access.
    #[must_use = "the recovery key is shown only once and cannot be retrieved later"]
    pub fn generate_recovery_key(&self, auth: Option<&str>) -> Result<String> {
        crate::keyfile::generate_recovery(&self.root, auth)
    }

    /// Open the repository at the default user-data location
    /// (`<data-dir>/chunks`, the same root Phase 27 used for the chunk
    /// store).
    ///
    /// # Sharing the default store
    ///
    /// This opens `index.redb` at the default chunk-store path, which
    /// other features (mount, recovery) may already hold open via
    /// [`ChunkStore::open`]. redb takes an exclusive file lock, so a
    /// second concurrent opener fails. Until Phase 49 is wired into the
    /// app with a single shared store handle, treat `open_default()` as
    /// exclusive with those features.
    pub fn open_default() -> Result<Self> {
        Self::open(&default_chunk_store_path()?)
    }

    /// The repository root (the value passed to [`Self::open`]).
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Borrow the underlying [`ChunkStore`] sub-component — for the
    /// engine's delta-resume path, disk-usage readouts, and tests.
    #[must_use]
    pub fn store(&self) -> &ChunkStore {
        self.store.as_ref()
    }

    /// Hand out a clone of the shared [`ChunkStore`] handle, so the same
    /// open store can back the delta-resume
    /// [`CopyThatChunkSink`](crate::CopyThatChunkSink), the recovery
    /// server, and disk-usage readouts without a second redb `open`
    /// (which would deadlock on the exclusive `index.redb` lock).
    #[must_use]
    pub fn store_arc(&self) -> Arc<ChunkStore> {
        Arc::clone(&self.store)
    }

    // ----- recording -------------------------------------------------

    /// Chunk each `(path, bytes)`, store the chunks (dedup-aware), and
    /// record a [`Snapshot`] of kind `kind`. Returns the new id.
    ///
    /// In-memory entry point for callers that already hold the bytes;
    /// [`Self::snapshot_files`] streams from disk instead.
    pub fn snapshot_bytes(
        &self,
        kind: SnapshotKind,
        label: &str,
        created_at_ms: i64,
        files: &[(&str, &[u8])],
    ) -> Result<SnapshotId> {
        let _lease = self.read_lease()?;
        let mut entries = Vec::with_capacity(files.len());
        for (path, bytes) in files {
            let manifest = chunk_into_store(&self.store, &self.chunker, bytes, self.compression)?.1;
            entries.push(FileEntry {
                path: (*path).to_string(),
                manifest,
            });
        }
        self.record_inner(kind, label, None, created_at_ms, entries)
    }

    /// Like [`Self::snapshot_bytes`] but tags the snapshot with a backup
    /// `source_key` for retention grouping (Phase 49e). In-memory
    /// companion to [`Self::snapshot_files_with_source`].
    pub fn snapshot_bytes_with_source(
        &self,
        kind: SnapshotKind,
        label: &str,
        source_key: &str,
        created_at_ms: i64,
        files: &[(&str, &[u8])],
    ) -> Result<SnapshotId> {
        let _lease = self.read_lease()?;
        let mut entries = Vec::with_capacity(files.len());
        for (path, bytes) in files {
            let manifest = chunk_into_store(&self.store, &self.chunker, bytes, self.compression)?.1;
            entries.push(FileEntry {
                path: (*path).to_string(),
                manifest,
            });
        }
        self.record_inner(kind, label, Some(source_key), created_at_ms, entries)
    }

    /// Like [`Self::snapshot_bytes`] but **streams** each file from disk
    /// through the rolling chunker, so a multi-gigabyte file never has
    /// to be held in memory at once.
    pub fn snapshot_files(
        &self,
        kind: SnapshotKind,
        label: &str,
        created_at_ms: i64,
        files: &[(&str, &Path)],
    ) -> Result<SnapshotId> {
        let _lease = self.read_lease()?;
        let mut entries = Vec::with_capacity(files.len());
        for (path, src) in files {
            let file = File::open(src).map_err(|e| ChunkStoreError::Io {
                path: src.to_path_buf(),
                source: e,
            })?;
            let manifest = self.ingest_reader(BufReader::new(file))?;
            entries.push(FileEntry {
                path: (*path).to_string(),
                manifest,
            });
        }
        self.record_inner(kind, label, None, created_at_ms, entries)
    }

    /// Like [`Self::snapshot_files`] but tags the snapshot with a backup
    /// `source_key` (the source's stable id), so [`Self::prune_source`]
    /// can later apply a retention policy to just this source's
    /// snapshots. The "Back up now" path uses this. (Phase 49e.)
    pub fn snapshot_files_with_source(
        &self,
        kind: SnapshotKind,
        label: &str,
        source_key: &str,
        created_at_ms: i64,
        files: &[(&str, &Path)],
    ) -> Result<SnapshotId> {
        let _lease = self.read_lease()?;
        let mut entries = Vec::with_capacity(files.len());
        for (path, src) in files {
            let file = File::open(src).map_err(|e| ChunkStoreError::Io {
                path: src.to_path_buf(),
                source: e,
            })?;
            let manifest = self.ingest_reader(BufReader::new(file))?;
            entries.push(FileEntry {
                path: (*path).to_string(),
                manifest,
            });
        }
        self.record_inner(kind, label, Some(source_key), created_at_ms, entries)
    }

    /// Phase 49g — walk `root`, apply gitignore-style `filters`, and
    /// record ONE snapshot of the surviving files (each streamed through
    /// the rolling chunker, so a multi-GB file never sits in memory).
    ///
    /// `filters.passes_dir(rel, meta)` prunes whole subtrees (e.g.
    /// `**/node_modules`); `filters.passes_file(rel, meta)` gates files
    /// (include whitelist, size/date range, hidden/system/readonly).
    /// Logical paths are source-root-relative + forward-slashed (the same
    /// convention the "Back up now" path uses, so the restore browser
    /// resolves them). Symlinks + non-regular files are skipped and
    /// counted; an unreadable directory is stepped over.
    pub fn snapshot_source(
        &self,
        kind: SnapshotKind,
        label: &str,
        source_key: Option<&str>,
        created_at_ms: i64,
        root: &Path,
        filters: &CompiledFilters,
    ) -> Result<SourceSnapshotSummary> {
        let _lease = self.read_lease()?;
        let mut entries: Vec<FileEntry> = Vec::new();
        let mut bytes = 0u64;
        let mut skipped = 0u64;
        let mut stack = vec![root.to_path_buf()];
        while let Some(dir) = stack.pop() {
            let Ok(rd) = std::fs::read_dir(&dir) else {
                continue; // unreadable dir — step over, like the GUI walker
            };
            for entry in rd.flatten() {
                let Ok(ft) = entry.file_type() else {
                    continue;
                };
                if ft.is_symlink() {
                    skipped += 1;
                    continue; // never follow symlinks
                }
                let abs = entry.path();
                let Ok(rel) = abs.strip_prefix(root) else {
                    continue;
                };
                let rel_key = rel.to_string_lossy().replace('\\', "/");
                if rel_key.is_empty() {
                    continue;
                }
                if ft.is_dir() {
                    // Descend unless a dir filter prunes it. A metadata read
                    // failure (transient sharing-lock, protected/reparse dir,
                    // dir removed mid-walk) must NEVER silently drop a whole
                    // subtree, so fail OPEN — descend — when metadata is
                    // unavailable.
                    let descend = match entry.metadata() {
                        Ok(meta) => filters.passes_dir(Path::new(&rel_key), &meta),
                        Err(_) => true,
                    };
                    if descend {
                        stack.push(abs);
                    }
                } else if ft.is_file() {
                    // Gate the file only when we can read its metadata; on a
                    // metadata failure fail OPEN (include it) rather than
                    // silently omit it from the backup — the ingest below
                    // surfaces a real error if the file is truly unreadable.
                    if let Ok(meta) = entry.metadata() {
                        if !filters.passes_file(Path::new(&rel_key), &meta) {
                            continue;
                        }
                    }
                    let file = File::open(&abs).map_err(|e| ChunkStoreError::Io {
                        path: abs.clone(),
                        source: e,
                    })?;
                    let manifest = self.ingest_reader(BufReader::new(file))?;
                    bytes += manifest.size;
                    entries.push(FileEntry {
                        path: rel_key,
                        manifest,
                    });
                } else {
                    skipped += 1; // block / char / fifo / socket
                }
            }
        }
        let files = entries.len() as u64;
        let id = self.record_inner(kind, label, source_key, created_at_ms, entries)?;
        Ok(SourceSnapshotSummary {
            id,
            files,
            bytes,
            skipped_non_regular: skipped,
        })
    }

    /// Record a snapshot from already-built [`FileEntry`]s (the engine
    /// path — the copy/sync engines build manifests as they run).
    pub fn record(
        &self,
        kind: SnapshotKind,
        label: &str,
        created_at_ms: i64,
        files: Vec<FileEntry>,
    ) -> Result<SnapshotId> {
        let _lease = self.read_lease()?;
        self.record_inner(kind, label, None, created_at_ms, files)
    }

    /// Phase 50h — replicate this repository's snapshots into `dst`, sending
    /// only the chunks `dst` is missing (content-addressed, so the transfer
    /// is inherently deduplicated — a chunk `dst` already holds, from any
    /// prior snapshot or source, is never re-sent).
    ///
    /// Idempotent: a snapshot whose *content* already exists in `dst` (same
    /// kind / time / label / files / chunks — everything but the local id) is
    /// skipped, so re-running only ships what changed since last time. Chunk
    /// integrity is checked on read (`ChunkStore::get` re-hashes), so a
    /// corrupt source pack aborts the replication rather than propagating.
    ///
    /// `dst` keeps its *own* compression policy: each copied chunk is
    /// re-encoded per `dst`'s [`Repository::compression`], and the replicated
    /// manifest's per-chunk codec is re-stamped to match how `dst` actually
    /// stored it — so the destination stays self-consistent regardless of the
    /// source's policy.
    ///
    /// The destination's GC lease is held for the whole pass, so `dst.gc()`
    /// can never observe a chunk written for a snapshot whose catalog row has
    /// not yet landed. The *source* needs no lease: replication only reads
    /// chunks that a live snapshot references, and a concurrent `self.gc()`
    /// only ever sweeps *unreferenced* chunks.
    pub fn replicate_to(&self, dst: &Repository) -> Result<ReplicateReport> {
        let _dst_lease = dst.read_lease()?;
        let mut report = ReplicateReport::default();

        // Content fingerprints already in the destination → idempotent re-runs.
        let mut present: HashSet<[u8; 32]> = HashSet::new();
        for s in dst.load_all_snapshots()? {
            present.insert(content_fingerprint(&s));
        }

        // Codec each distinct chunk is stored under in `dst`, accumulated
        // across the WHOLE replication so a chunk shared by many snapshots is
        // transferred + counted exactly once — not once per snapshot it
        // appears in (which over-reported `chunks_present`).
        let mut codec_of: HashMap<Blake3Hash, ChunkCodec> = HashMap::new();
        for snap in self.load_all_snapshots()? {
            if !present.insert(content_fingerprint(&snap)) {
                report.snapshots_skipped += 1;
                continue;
            }
            // Transfer every distinct chunk the destination lacks, recording
            // the codec `dst` stored each hash under (existing or fresh).
            for f in &snap.files {
                for c in &f.manifest.chunks {
                    if codec_of.contains_key(&c.hash) {
                        continue;
                    }
                    let codec = if let Some(loc) = dst.store().locate(&c.hash)? {
                        report.chunks_present += 1;
                        loc.codec
                    } else {
                        let bytes = self.store().get(&c.hash)?.ok_or_else(|| {
                            ChunkStoreError::MissingChunk {
                                hash: hex_of(&c.hash),
                            }
                        })?;
                        report.chunks_copied += 1;
                        report.bytes_copied += bytes.len() as u64;
                        dst.store().put_chunk(c.hash, &bytes, dst.compression())?
                    };
                    codec_of.insert(c.hash, codec);
                }
            }
            // Rebuild the files with destination-truthful codecs, then insert
            // the snapshot into `dst` preserving all metadata (fresh dst id).
            let files: Vec<FileEntry> = snap
                .files
                .iter()
                .map(|f| FileEntry {
                    path: f.path.clone(),
                    manifest: Manifest {
                        file_hash: f.manifest.file_hash,
                        size: f.manifest.size,
                        chunks: f
                            .manifest
                            .chunks
                            .iter()
                            .map(|c| ChunkRef {
                                codec: codec_of.get(&c.hash).copied().unwrap_or(c.codec),
                                ..*c
                            })
                            .collect(),
                    },
                })
                .collect();
            dst.insert_full_snapshot(&snap, files)?;
            report.snapshots_copied += 1;
        }
        Ok(report)
    }

    /// Insert a snapshot into this repository preserving all of `template`'s
    /// metadata (kind / time / label / source / tags / pin / description),
    /// under a fresh local id, with the given `files`. The replication insert
    /// path — unlike [`Self::record_inner`], which resets metadata for a
    /// freshly-captured snapshot. Runs under the caller's held GC lease.
    fn insert_full_snapshot(
        &self,
        template: &Snapshot,
        files: Vec<FileEntry>,
    ) -> Result<SnapshotId> {
        let id = self.allocate_snapshot_id()?;
        let snap = Snapshot {
            id,
            kind: template.kind,
            created_at_ms: template.created_at_ms,
            label: template.label.clone(),
            source_key: template.source_key.clone(),
            description: template.description.clone(),
            tags: template.tags.clone(),
            pinned: template.pinned,
            source: template.source.clone(),
            files,
        };
        let snap_bytes = serde_json::to_vec(&snap)?;
        let summary_bytes = serde_json::to_vec(&snap.summary())?;
        let txn = self.db.begin_write()?;
        {
            let mut snaps = txn.open_table(SNAPSHOTS)?;
            snaps.insert(id, snap_bytes.as_slice())?;
            let mut summaries = txn.open_table(SUMMARIES)?;
            summaries.insert(id, summary_bytes.as_slice())?;
        }
        txn.commit()?;
        Ok(SnapshotId(id))
    }

    /// Remove a snapshot from the catalog. Chunk bytes are **not** freed
    /// here — run [`Self::gc`] to reclaim chunks no manifest source
    /// references any more. Returns `false` if no such snapshot existed.
    pub fn remove_snapshot(&self, id: SnapshotId) -> Result<bool> {
        let _lease = self.read_lease()?;
        let txn = self.db.begin_write()?;
        let existed;
        {
            let mut snaps = txn.open_table(SNAPSHOTS)?;
            existed = snaps.remove(id.0)?.is_some();
            let mut summaries = txn.open_table(SUMMARIES)?;
            summaries.remove(id.0)?;
        }
        txn.commit()?;
        Ok(existed)
    }

    /// Remove many snapshots in ONE atomic catalog transaction. Chunk
    /// bytes are **not** freed here — run [`Self::gc`] afterwards (or use
    /// [`Self::prune_source`], which does). Returns how many of `ids`
    /// actually existed and were removed.
    pub fn remove_snapshots(&self, ids: &[SnapshotId]) -> Result<u64> {
        if ids.is_empty() {
            return Ok(0);
        }
        let _lease = self.read_lease()?;
        let txn = self.db.begin_write()?;
        let mut removed = 0u64;
        {
            let mut snaps = txn.open_table(SNAPSHOTS)?;
            let mut summaries = txn.open_table(SUMMARIES)?;
            for id in ids {
                if snaps.remove(id.0)?.is_some() {
                    removed += 1;
                }
                summaries.remove(id.0)?;
            }
        }
        txn.commit()?;
        Ok(removed)
    }

    /// Phase 49p — load a snapshot, apply `f`, and rewrite both its
    /// `SNAPSHOTS` row and its `SUMMARIES` index entry in one transaction
    /// (so the summary index never drifts from the full row). Returns
    /// `false` if the snapshot doesn't exist.
    fn edit_snapshot(&self, id: SnapshotId, f: impl FnOnce(&mut Snapshot)) -> Result<bool> {
        let _lease = self.read_lease()?;
        let txn = self.db.begin_write()?;
        let existed;
        {
            let mut snaps = txn.open_table(SNAPSHOTS)?;
            let current = snaps.get(id.0)?.map(|v| v.value().to_vec());
            if let Some(bytes) = current {
                let mut snap: Snapshot = serde_json::from_slice(&bytes)?;
                f(&mut snap);
                let snap_bytes = serde_json::to_vec(&snap)?;
                let summary_bytes = serde_json::to_vec(&snap.summary())?;
                snaps.insert(id.0, snap_bytes.as_slice())?;
                let mut summaries = txn.open_table(SUMMARIES)?;
                summaries.insert(id.0, summary_bytes.as_slice())?;
                existed = true;
            } else {
                existed = false;
            }
        }
        txn.commit()?;
        Ok(existed)
    }

    /// Phase 49p — set a snapshot's label. Returns `false` if absent.
    pub fn set_label(&self, id: SnapshotId, label: &str) -> Result<bool> {
        self.edit_snapshot(id, |s| s.label = label.to_string())
    }

    /// Phase 49p — set a snapshot's free-text description.
    pub fn set_description(&self, id: SnapshotId, description: &str) -> Result<bool> {
        self.edit_snapshot(id, |s| s.description = description.to_string())
    }

    /// Phase 49p — set a snapshot's tags.
    pub fn set_tags(&self, id: SnapshotId, tags: Vec<String>) -> Result<bool> {
        self.edit_snapshot(id, move |s| s.tags = tags)
    }

    /// Phase 49p — pin or unpin a snapshot. Pinned snapshots are never
    /// removed by [`Self::prune`].
    pub fn set_pinned(&self, id: SnapshotId, pinned: bool) -> Result<bool> {
        self.edit_snapshot(id, |s| s.pinned = pinned)
    }

    /// Phase 49e — which of a backup source's snapshots a retention
    /// `policy` would prune, **without** mutating anything. Drives the
    /// "would remove N" preview. Reuses the audited
    /// [`select_for_pruning`] pruner verbatim: the freshest snapshot is
    /// always kept, and only this source's snapshots
    /// (`source_key == Some(source_id)`) are considered.
    pub fn plan_source_prune(
        &self,
        source_id: &str,
        policy: &RetentionPolicy,
        now_ms: i64,
    ) -> Result<Vec<SnapshotId>> {
        let entries: Vec<VersionEntry> = self
            .snapshots()?
            .into_iter()
            .filter(|s| s.source_key.as_deref() == Some(source_id))
            .map(|s| VersionEntry {
                row_id: s.id as i64,
                ts_ms: s.created_at_ms,
                retained_until_ms: None,
            })
            .collect();
        Ok(select_for_pruning(&entries, policy, now_ms)
            .into_iter()
            .map(|id| SnapshotId(id as u64))
            .collect())
    }

    /// Phase 49e — apply a retention `policy` to ONE backup source's
    /// snapshots, delete the losers in one atomic catalog transaction,
    /// then [`Self::gc`] to reclaim the now-orphaned chunks.
    ///
    /// Two-step like restic's forget→prune: the catalog edit and the
    /// physical sweep are separate, and a crash between them leaves
    /// orphan chunks the next `gc()` reclaims (the existing
    /// crash-consistency contract). The freshest snapshot of the source
    /// is always retained; other sources and ad-hoc (`source_key: None`)
    /// snapshots are never touched. A policy that selects nothing skips
    /// the `gc()` entirely (a no-op `KeepAll` backup stays cheap).
    pub fn prune_source(
        &self,
        source_id: &str,
        policy: &RetentionPolicy,
        now_ms: i64,
    ) -> Result<PruneReport> {
        let drop_ids = self.plan_source_prune(source_id, policy, now_ms)?;
        if drop_ids.is_empty() {
            return Ok(PruneReport::default());
        }
        let removed = self.remove_snapshots(&drop_ids)?;
        let gc = self.gc()?;
        Ok(PruneReport {
            snapshots_removed: removed,
            chunks_swept: gc.chunks_swept,
            bytes_reclaimed: gc.bytes_reclaimed,
        })
    }

    /// Phase 49p — prune snapshots across the WHOLE repository by a global
    /// keep-last / keep-within policy, ALWAYS retaining pinned snapshots.
    /// Returns the removed ids (the caller runs [`Self::gc`] to reclaim the
    /// now-orphaned chunks). An all-`None` policy removes nothing.
    pub fn prune(&self, policy: &PrunePolicy, now_ms: i64) -> Result<Vec<SnapshotId>> {
        if policy.keep_last.is_none() && policy.keep_within_ms.is_none() {
            return Ok(Vec::new());
        }
        let mut summaries = self.snapshots()?;
        // Newest first so `keep_last` indexes the freshest snapshots.
        summaries.sort_by_key(|s| std::cmp::Reverse((s.created_at_ms, s.id)));
        let cutoff = policy.keep_within_ms.map(|w| now_ms.saturating_sub(w));
        let mut remove = Vec::new();
        for (idx, s) in summaries.iter().enumerate() {
            if s.pinned {
                continue; // pinned snapshots are never pruned
            }
            let within_keep_last = policy.keep_last.is_some_and(|n| (idx as u64) < n);
            let within_time = cutoff.is_some_and(|c| s.created_at_ms >= c);
            if !within_keep_last && !within_time {
                remove.push(SnapshotId(s.id));
            }
        }
        if !remove.is_empty() {
            self.remove_snapshots(&remove)?;
        }
        Ok(remove)
    }

    /// Stream a reader through the chunker into the store, returning the
    /// file's [`Manifest`]. Never buffers the whole input.
    fn ingest_reader<R: Read>(&self, reader: R) -> Result<Manifest> {
        let mut chunks = Vec::new();
        let mut offset = 0u64;
        let comp = self.compression;
        // `chunk_reader` yields window-relative offsets; we track the
        // absolute offset ourselves and use the full-file hash it
        // returns so there's no extra pass over the bytes.
        let file_hash = self.chunker.chunk_reader(reader, |chunk, slice| {
            let codec = self.store.put_chunk(chunk.hash, slice, comp)?;
            chunks.push(ChunkRef {
                hash: chunk.hash,
                offset,
                len: chunk.len,
                codec,
            });
            offset += u64::from(chunk.len);
            Ok(())
        })?;
        Ok(Manifest {
            file_hash,
            size: offset,
            chunks,
        })
    }

    /// Persist a snapshot row + its summary in one transaction. Callers
    /// must already hold the read lease.
    fn record_inner(
        &self,
        kind: SnapshotKind,
        label: &str,
        source_key: Option<&str>,
        created_at_ms: i64,
        files: Vec<FileEntry>,
    ) -> Result<SnapshotId> {
        let id = self.allocate_snapshot_id()?;
        let mut snap = Snapshot {
            id,
            kind,
            created_at_ms,
            label: label.to_string(),
            source_key: source_key.map(str::to_string),
            description: String::new(),
            tags: Vec::new(),
            pinned: false,
            source: String::new(),
            files,
        };
        // Phase 49l — derive + store the source prefix for the dashboard.
        snap.source = snap.compute_source();
        let snap_bytes = serde_json::to_vec(&snap)?;
        let summary_bytes = serde_json::to_vec(&snap.summary())?;

        let txn = self.db.begin_write()?;
        {
            let mut snaps = txn.open_table(SNAPSHOTS)?;
            snaps.insert(id, snap_bytes.as_slice())?;
            let mut summaries = txn.open_table(SUMMARIES)?;
            summaries.insert(id, summary_bytes.as_slice())?;
        }
        txn.commit()?;
        Ok(SnapshotId(id))
    }

    // ----- querying --------------------------------------------------

    /// Fetch a full snapshot by id (manifests included).
    pub fn snapshot(&self, id: SnapshotId) -> Result<Option<Snapshot>> {
        let txn = self.db.begin_read()?;
        let snaps = txn.open_table(SNAPSHOTS)?;
        let Some(v) = snaps.get(id.0)? else {
            return Ok(None);
        };
        Ok(Some(serde_json::from_slice(v.value())?))
    }

    /// Phase 49l — fold the timeline summaries into one row per source
    /// (newest snapshot wins the `latest_*` fields). `O(snapshots)`, reads
    /// only the summaries index — no pack/manifest I/O.
    pub fn sources(&self) -> Result<Vec<SourceSummary>> {
        use std::collections::HashMap;
        let mut map: HashMap<String, SourceSummary> = HashMap::new();
        for s in self.snapshots()? {
            let e = map
                .entry(s.source.clone())
                .or_insert_with(|| SourceSummary {
                    source: s.source.clone(),
                    snapshot_count: 0,
                    latest_ms: i64::MIN,
                    latest_kind: s.kind,
                    latest_size: 0,
                    total_files: 0,
                });
            e.snapshot_count += 1;
            if s.created_at_ms >= e.latest_ms {
                e.latest_ms = s.created_at_ms;
                e.latest_kind = s.kind;
                e.latest_size = s.total_size;
                e.total_files = s.file_count;
            }
        }
        let mut out: Vec<SourceSummary> = map.into_values().collect();
        out.sort_by(|a, b| b.latest_ms.cmp(&a.latest_ms));
        Ok(out)
    }

    /// The whole timeline as lightweight summaries, oldest first
    /// (ordered by `created_at_ms`, then id). Reads only the summaries
    /// index, so it never deserializes a chunk manifest.
    pub fn snapshots(&self) -> Result<Vec<UnifiedSnapshot>> {
        let txn = self.db.begin_read()?;
        let summaries = txn.open_table(SUMMARIES)?;
        let mut out = Vec::with_capacity(summaries.len()? as usize);
        for row in summaries.iter()? {
            let (_k, v) = row?;
            out.push(serde_json::from_slice::<UnifiedSnapshot>(v.value())?);
        }
        out.sort_by_key(|s| (s.created_at_ms, s.id));
        Ok(out)
    }

    /// Resolve the state of `path` as of `ts_ms`: the newest snapshot at
    /// or before `ts_ms` that contains `path`. Returns `None` if the
    /// file had not yet been captured by then.
    pub fn snapshot_at(&self, path: &str, ts_ms: i64) -> Result<Option<FileSnapshot>> {
        let mut best: Option<FileSnapshot> = None;
        for snap in self.load_all_snapshots()? {
            if snap.created_at_ms > ts_ms {
                continue;
            }
            let Some(entry) = snap.files.iter().find(|f| f.path == path) else {
                continue;
            };
            let newer = match &best {
                None => true,
                Some(b) => (snap.created_at_ms, snap.id) > (b.created_at_ms, b.snapshot_id),
            };
            if newer {
                best = Some(FileSnapshot {
                    path: entry.path.clone(),
                    snapshot_id: snap.id,
                    kind: snap.kind,
                    created_at_ms: snap.created_at_ms,
                    manifest: entry.manifest.clone(),
                });
            }
        }
        Ok(best)
    }

    /// Materialise a resolved [`FileSnapshot`] back to `dst`, pulling
    /// each chunk from the store. Errors with
    /// [`ChunkStoreError::MissingChunk`] if a referenced chunk has been
    /// garbage-collected.
    pub fn restore(&self, snapshot: &FileSnapshot, dst: &Path) -> Result<()> {
        materialise_file(&self.store, &snapshot.manifest, dst)
    }

    /// Flat file listing of a snapshot — `(path, logical size)` per file,
    /// no chunk manifests. The front end assembles the tree from this.
    pub fn snapshot_tree(&self, id: SnapshotId) -> Result<Vec<SnapshotFileEntry>> {
        Ok(self
            .snapshot(id)?
            .map(|s| {
                s.files
                    .iter()
                    .map(|f| SnapshotFileEntry {
                        path: f.path.clone(),
                        size: f.manifest.size,
                    })
                    .collect()
            })
            .unwrap_or_default())
    }

    /// Restore the chosen logical `paths` from snapshot `id` under
    /// `dst_root`. `strip_prefix`, when set, is removed from each logical
    /// path to preserve subtree shape; otherwise the path is sanitised to
    /// a safe relative form so a rooted / `..` snapshot path can never
    /// escape `dst_root`. Per-file existing-target handling follows
    /// `conflict`. Best-effort: a single file's failure is tallied and the
    /// rest still restore.
    pub fn restore_paths(
        &self,
        id: SnapshotId,
        paths: &[&str],
        dst_root: &Path,
        strip_prefix: Option<&str>,
        conflict: RestoreConflict,
    ) -> Result<RestoreReport> {
        let snap = self
            .snapshot(id)?
            .ok_or(ChunkStoreError::SnapshotNotFound(id.0))?;
        let mut rep = RestoreReport::default();
        for want in paths {
            let Some(entry) = snap.files.iter().find(|f| f.path == *want) else {
                rep.failed += 1;
                continue;
            };
            let Some(rel) = rel_dst(&entry.path, strip_prefix) else {
                rep.failed += 1; // unsafe path — never write outside dst_root
                continue;
            };
            let mut target = dst_root.join(&rel);
            if target.exists() {
                match conflict {
                    RestoreConflict::Skip => {
                        rep.skipped += 1;
                        continue;
                    }
                    RestoreConflict::KeepBoth => target = next_free_name(&target),
                    RestoreConflict::Overwrite => {}
                }
            }
            if let Some(parent) = target.parent()
                && std::fs::create_dir_all(parent).is_err()
            {
                rep.failed += 1;
                continue;
            }
            match materialise_file(&self.store, &entry.manifest, &target) {
                Ok(()) => rep.restored += 1,
                Err(_) => rep.failed += 1,
            }
        }
        Ok(rep)
    }

    /// Dry-run companion to [`Self::restore_paths`]: for each requested
    /// logical path present in the snapshot, whether its resolved target
    /// already exists under `dst_root`. Drives the UI's conflict step
    /// before any bytes are written, using the *same* path resolution as
    /// the real restore so the preview never disagrees with it.
    pub fn restore_preview(
        &self,
        id: SnapshotId,
        paths: &[&str],
        dst_root: &Path,
        strip_prefix: Option<&str>,
    ) -> Result<Vec<(String, bool)>> {
        let snap = self
            .snapshot(id)?
            .ok_or(ChunkStoreError::SnapshotNotFound(id.0))?;
        let mut out = Vec::with_capacity(paths.len());
        for want in paths {
            if !snap.files.iter().any(|f| f.path == *want) {
                continue;
            }
            let exists = rel_dst(want, strip_prefix)
                .map(|rel| dst_root.join(rel).exists())
                .unwrap_or(false);
            out.push(((*want).to_string(), exists));
        }
        Ok(out)
    }

    /// Aggregate stats for the hero readout.
    pub fn stats(&self) -> Result<RepoStats> {
        let snaps = self.load_all_snapshots()?;
        let effective_bytes = snaps.iter().map(Snapshot::total_size).sum();
        // Deduplicated logical size: each distinct chunk counted once.
        let mut unique: HashMap<Blake3Hash, u32> = HashMap::new();
        for s in &snaps {
            for f in &s.files {
                for c in &f.manifest.chunks {
                    unique.entry(c.hash).or_insert(c.len);
                }
            }
        }
        let unique_bytes = unique.values().map(|&l| u64::from(l)).sum();
        // Phase 49h — physical (on-disk, possibly-compressed) size of the
        // distinct reachable chunks, looked up from their locators.
        let locators: HashMap<Blake3Hash, u32> = self
            .store
            .all_locators()?
            .into_iter()
            .map(|(h, loc)| (h, loc.stored_len))
            .collect();
        let physical_unique_bytes = unique
            .keys()
            .filter_map(|h| locators.get(h))
            .map(|&sl| u64::from(sl))
            .sum();
        Ok(RepoStats {
            stored_bytes: self.store.disk_usage_bytes()?,
            unique_bytes,
            physical_unique_bytes,
            effective_bytes,
            snapshot_count: snaps.len() as u64,
            chunk_count: self.store.chunk_count()?,
        })
    }

    /// How many snapshots reference `hash` (0 if none). Derived on
    /// demand from the catalog — for diagnostics and tests.
    pub fn chunk_refcount(&self, hash: &Blake3Hash) -> Result<u64> {
        let mut count = 0u64;
        for snap in self.load_all_snapshots()? {
            if snap.distinct_chunks().contains(hash) {
                count += 1;
            }
        }
        Ok(count)
    }

    // ----- garbage collection ---------------------------------------

    /// Mark-and-sweep garbage collection.
    ///
    /// **Mark:** walk every manifest source — the snapshot catalog *and*
    /// the [`ChunkStore`]'s own Phase 27 `manifests` table (delta-resume
    /// baselines, and whatever the mount / recovery backends stored) —
    /// to build the reachable chunk set. Marking the Phase 27 manifests
    /// too is essential: the chunk store is shared, so a chunk reachable
    /// only through a delta-resume manifest must not be swept.
    ///
    /// **Sweep:** in a single pass over the chunk index, drop the index
    /// entry of any chunk not in the reachable set, then delete any
    /// non-active pack file whose chunks were *all* swept.
    ///
    /// Live data is never touched. GC takes the exclusive write lease,
    /// so it cannot run while a snapshot is mid-record (which would let
    /// it sweep chunks of a not-yet-committed snapshot).
    pub fn gc(&self) -> Result<GcReport> {
        self.gc_with_progress(&mut |_| {})
    }

    /// Quick clean (mark-sweep) with progress callbacks (Phase 49i). Takes
    /// the exclusive gc lease; deletes only fully-dead, non-active packs.
    pub fn gc_with_progress(&self, cb: &mut dyn FnMut(MaintenanceProgress)) -> Result<GcReport> {
        let _lease = self
            .gc_lock
            .write()
            .map_err(|_| ChunkStoreError::Redb("repository gc lock poisoned".into()))?;
        self.gc_locked(cb)
    }

    /// The mark-sweep body, assuming the caller already holds the gc write
    /// lease (so [`Self::compact`] can reuse it — the `RwLock` write guard
    /// is not reentrant).
    fn gc_locked(&self, cb: &mut dyn FnMut(MaintenanceProgress)) -> Result<GcReport> {
        cb(MaintenanceProgress {
            phase: "mark",
            done: 0,
            total: 0,
        });
        // --- Mark: every chunk reachable from any manifest source.
        let mut reachable: HashSet<Blake3Hash> = HashSet::new();
        for snap in self.load_all_snapshots()? {
            for f in &snap.files {
                for c in &f.manifest.chunks {
                    reachable.insert(c.hash);
                }
            }
        }
        for manifest in self.store.all_manifests()? {
            for c in &manifest.chunks {
                reachable.insert(c.hash);
            }
        }

        // --- Sweep: one pass collects orphans + classifies packs.
        let all = self.store.all_locators()?;
        let active = self.store.active_pack_id()?;
        cb(MaintenanceProgress {
            phase: "sweep",
            done: 0,
            total: all.len() as u64,
        });
        let mut orphans: Vec<Blake3Hash> = Vec::new();
        let mut live_packs: HashSet<Uuid> = HashSet::new();
        let mut all_packs: HashSet<Uuid> = HashSet::new();
        for (h, loc) in &all {
            all_packs.insert(loc.pack_id);
            if reachable.contains(h) {
                live_packs.insert(loc.pack_id);
            } else {
                orphans.push(*h);
            }
        }
        self.store.remove_chunk_index_entries(&orphans)?;

        let mut packs_removed = 0u64;
        let mut bytes_reclaimed = 0u64;
        for pid in all_packs {
            if pid != active && !live_packs.contains(&pid) {
                bytes_reclaimed += self.store.remove_pack_file(pid)?;
                packs_removed += 1;
            }
        }

        let report = GcReport {
            chunks_swept: orphans.len() as u64,
            packs_removed,
            bytes_reclaimed,
        };
        tracing::debug!(
            chunks_swept = report.chunks_swept,
            packs_removed = report.packs_removed,
            bytes_reclaimed = report.bytes_reclaimed,
            "repository gc complete"
        );
        Ok(report)
    }

    /// Full compact: quick clean, then rewrite below-threshold packs to
    /// reclaim the dead bytes inside partially-live packs (Phase 49i).
    /// Holds the exclusive gc lease for its full duration, so it can never
    /// interleave with a snapshot record. **Cancellation** is cooperative
    /// (checked between packs + chunks); on cancel the store is left
    /// consistent (moved chunks at their new home, the rest untouched).
    pub fn compact(
        &self,
        opts: CompactOptions,
        cancel: &dyn Fn() -> bool,
        cb: &mut dyn FnMut(MaintenanceProgress),
    ) -> Result<(GcReport, CompactReport)> {
        let _lease = self
            .gc_lock
            .write()
            .map_err(|_| ChunkStoreError::Redb("repository gc lock poisoned".into()))?;

        // Quick clean first (removes fully-dead packs + orphan rows).
        let gc = self.gc_locked(cb)?;

        // Roll the active pack so ITS dead bytes also become compactable.
        self.store.roll_active_pack()?;

        // Pick the non-active packs that are at least `min_dead_ratio` dead.
        let targets: Vec<Uuid> = self
            .store
            .pack_stats()?
            .into_iter()
            .filter(|p| !p.is_active && p.file_bytes > 0)
            .filter(|p| {
                let dead = p.file_bytes.saturating_sub(p.live_bytes);
                dead as f64 >= p.file_bytes as f64 * opts.min_dead_ratio
            })
            .map(|p| p.pack_id)
            .collect();

        let mut report = CompactReport::default();
        let total = targets.len() as u64;
        for (i, pid) in targets.iter().enumerate() {
            if cancel() {
                break;
            }
            cb(MaintenanceProgress {
                phase: "compact",
                done: i as u64,
                total,
            });
            let mut moved = 0u64;
            let freed = self.store.compact_pack(*pid, cancel, &mut |m| moved = m)?;
            report.chunks_moved += moved;
            if freed > 0 {
                report.packs_compacted += 1;
                report.bytes_reclaimed += freed;
            }
        }
        cb(MaintenanceProgress {
            phase: "compact",
            done: total,
            total,
        });
        Ok((gc, report))
    }

    /// Phase 49n — verify all snapshots (or one). `Metadata` checks every
    /// chunk has a live index entry; `ReadData` reads + re-hashes every
    /// chunk AND checks each file's concatenated bytes hash to its
    /// `file_hash`. Takes the shared read lease so a gc can't run mid-pass.
    pub fn verify(&self, only: Option<SnapshotId>, level: VerifyLevel) -> Result<VerifyReport> {
        let _lease = self.read_lease()?;
        let mut report = VerifyReport::default();
        for snap in self.load_all_snapshots()? {
            if let Some(only) = only {
                if snap.id != only.0 {
                    continue;
                }
            }
            report.snapshots_checked += 1;
            for f in &snap.files {
                report.files_checked += 1;
                let mut hasher = blake3::Hasher::new();
                let mut file_ok = true;
                for c in &f.manifest.chunks {
                    report.chunks_checked += 1;
                    match level {
                        VerifyLevel::Metadata => {
                            if self.store.locate(&c.hash)?.is_none() {
                                report.damage.push(VerifyDamage {
                                    snapshot_id: snap.id,
                                    path: f.path.clone(),
                                    chunk_hash_hex: crate::types::hex_of(&c.hash),
                                    kind: DamageKind::Missing,
                                });
                                file_ok = false;
                            }
                        }
                        VerifyLevel::ReadData => match self.store.get(&c.hash) {
                            Ok(Some(bytes)) => {
                                hasher.update(&bytes);
                            }
                            Ok(None) => {
                                report.damage.push(VerifyDamage {
                                    snapshot_id: snap.id,
                                    path: f.path.clone(),
                                    chunk_hash_hex: crate::types::hex_of(&c.hash),
                                    kind: DamageKind::Missing,
                                });
                                file_ok = false;
                            }
                            Err(_) => {
                                report.damage.push(VerifyDamage {
                                    snapshot_id: snap.id,
                                    path: f.path.clone(),
                                    chunk_hash_hex: crate::types::hex_of(&c.hash),
                                    kind: DamageKind::Corrupt,
                                });
                                file_ok = false;
                            }
                        },
                    }
                }
                // ReadData only: whole-file hash check (skipped when a chunk
                // was already flagged — the file can't reconstruct anyway).
                if level == VerifyLevel::ReadData
                    && file_ok
                    && hasher.finalize().as_bytes() != &f.manifest.file_hash
                {
                    report.damage.push(VerifyDamage {
                        snapshot_id: snap.id,
                        path: f.path.clone(),
                        chunk_hash_hex: String::new(),
                        kind: DamageKind::FileHashMismatch,
                    });
                }
            }
        }
        Ok(report)
    }

    /// Phase 49n — repair = quarantine. Remove every snapshot with ANY
    /// damage (so it can never mis-restore), then `gc()` to reclaim the
    /// now-orphaned chunks. `apply == false` is a dry run (returns the
    /// would-remove ids + an empty `GcReport`, changing nothing). Byte-level
    /// chunk repair needs a redundancy source — out of scope; this is the
    /// safe, feasible repair.
    pub fn repair_remove_damaged(
        &self,
        report: &VerifyReport,
        apply: bool,
    ) -> Result<(Vec<SnapshotId>, GcReport)> {
        let mut ids: Vec<u64> = report.damage.iter().map(|d| d.snapshot_id).collect();
        ids.sort_unstable();
        ids.dedup();
        let damaged: Vec<SnapshotId> = ids.into_iter().map(SnapshotId).collect();
        if !apply || damaged.is_empty() {
            return Ok((damaged, GcReport::default()));
        }
        self.remove_snapshots(&damaged)?;
        let gc = self.gc()?;
        Ok((damaged, gc))
    }

    // ----- internals -------------------------------------------------

    /// Acquire the shared mutator lease (read side of the GC lock).
    fn read_lease(&self) -> Result<std::sync::RwLockReadGuard<'_, ()>> {
        self.gc_lock
            .read()
            .map_err(|_| ChunkStoreError::Redb("repository gc lock poisoned".into()))
    }

    /// Hand out the next monotonic snapshot id (starts at 1).
    fn allocate_snapshot_id(&self) -> Result<u64> {
        let txn = self.db.begin_write()?;
        let next;
        {
            let mut seq = txn.open_table(SEQ)?;
            let cur = seq
                .get(SEQ_NEXT_SNAPSHOT_ID)?
                .map(|v| v.value())
                .unwrap_or(1);
            next = cur;
            seq.insert(SEQ_NEXT_SNAPSHOT_ID, cur + 1)?;
        }
        txn.commit()?;
        Ok(next)
    }

    /// Load every snapshot (full, with manifests) from the catalog.
    pub(crate) fn load_all_snapshots(&self) -> Result<Vec<Snapshot>> {
        let txn = self.db.begin_read()?;
        let snaps = txn.open_table(SNAPSHOTS)?;
        let mut out = Vec::with_capacity(snaps.len()? as usize);
        for row in snaps.iter()? {
            let (_k, v) = row?;
            out.push(serde_json::from_slice::<Snapshot>(v.value())?);
        }
        Ok(out)
    }
}

/// Phase 50h — a stable CONTENT fingerprint of a snapshot for replication's
/// "already present in the destination?" check. Two snapshots match iff their
/// kind, capture time, and file content agree; the local id, the per-chunk
/// storage codec, and every user-editable annotation (label / description /
/// tags / pin / derived source) are normalised out, so neither a differing
/// destination compression policy nor a post-replication metadata edit
/// re-copies a snapshot. (`serde_json` emits struct fields + vec elements in a
/// fixed order, so the serialization is deterministic.)
fn content_fingerprint(snap: &Snapshot) -> [u8; 32] {
    let mut probe = snap.clone();
    probe.id = 0;
    // Identity is CONTENT-only: normalise out every field a user can edit
    // AFTER replication — label, description, tags, pin, and the derived
    // source — so a later set_label/set_tags/set_pinned/set_description on
    // either side never de-syncs the "already present" check and duplicates
    // the snapshot on the next run.
    probe.label = String::new();
    probe.description = String::new();
    probe.tags = Vec::new();
    probe.pinned = false;
    probe.source = String::new();
    // The per-chunk `codec` is a *destination* storage detail — the same
    // logical chunk is stored uncompressed in one repo and zstd in another —
    // so normalize it out too. Otherwise a snapshot replicated into a
    // differently-compressed destination would never match on re-run.
    for f in &mut probe.files {
        for c in &mut f.manifest.chunks {
            c.codec = ChunkCodec::default();
        }
    }
    let bytes = serde_json::to_vec(&probe).unwrap_or_default();
    *blake3::hash(&bytes).as_bytes()
}

/// Map a snapshot's logical `path` to a SAFE relative path under the
/// restore root. With `strip_prefix`, the prefix is removed (to preserve
/// subtree shape); the result keeps only `Normal` components, so a rooted
/// or drive-prefixed path is de-rooted and a `..` traversal is refused
/// (`None`). Returns `None` for NUL bytes, or when nothing safe remains —
/// the caller counts those as failures and never writes outside the root.
fn rel_dst(path: &str, strip_prefix: Option<&str>) -> Option<PathBuf> {
    if path.contains('\0') {
        return None;
    }
    let stripped: &str = match strip_prefix {
        Some(prefix) if !prefix.is_empty() => match path.strip_prefix(prefix) {
            // Only strip on a path-component boundary, so a prefix that is a
            // partial segment of a sibling (prefix `photos`, path
            // `photos2024/x`) is never mis-stripped into `2024/x`.
            Some(rest) => match rest.as_bytes().first() {
                None => rest,
                Some(b'/') | Some(b'\\') => &rest[1..],
                _ => path,
            },
            None => path,
        },
        _ => path,
    };
    let norm = stripped.replace('\\', "/");
    let mut rel = PathBuf::new();
    for comp in Path::new(&norm).components() {
        match comp {
            std::path::Component::Normal(c) => rel.push(c),
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => return None, // traversal — refuse
            std::path::Component::RootDir | std::path::Component::Prefix(_) => {} // de-root
        }
    }
    if rel.as_os_str().is_empty() {
        rel.push(Path::new(&norm).file_name()?);
    }
    Some(rel)
}

/// First non-existent ` (n)`-suffixed sibling of `target`, for
/// [`RestoreConflict::KeepBoth`]. Mirrors the sync engine's
/// conflict-path suffixing.
fn next_free_name(target: &Path) -> PathBuf {
    if !target.exists() {
        return target.to_path_buf();
    }
    let parent = target.parent().unwrap_or_else(|| Path::new(""));
    let stem = target
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("restored");
    let ext = target.extension().and_then(|s| s.to_str());
    for n in 1..10_000u32 {
        let name = match ext {
            Some(e) => format!("{stem} ({n}).{e}"),
            None => format!("{stem} ({n})"),
        };
        let cand = parent.join(name);
        if !cand.exists() {
            return cand;
        }
    }
    target.to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lcg_bytes(seed: u64, len: usize) -> Vec<u8> {
        let mut out = vec![0u8; len];
        let mut s = seed;
        for b in &mut out {
            s = s
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            *b = (s >> 33) as u8;
        }
        out
    }

    #[test]
    fn open_lays_out_both_dbs() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::open(tmp.path()).unwrap();
        assert!(repo.root().join("index.redb").exists());
        assert!(repo.root().join("packs").exists());
        assert!(repo.root().join("repository.redb").exists());
    }

    #[test]
    fn record_then_list_and_fetch() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::open(tmp.path()).unwrap();
        let bytes = lcg_bytes(1, 1024 * 1024);
        let id = repo
            .snapshot_bytes(SnapshotKind::Copy, "first", 1000, &[("/a", &bytes)])
            .unwrap();
        assert_eq!(id.as_u64(), 1);

        let list = repo.snapshots().unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].kind, SnapshotKind::Copy);
        assert_eq!(list[0].file_count, 1);
        assert_eq!(list[0].total_size, bytes.len() as u64);

        let full = repo.snapshot(id).unwrap().unwrap();
        assert_eq!(full.files.len(), 1);
        assert_eq!(full.files[0].path, "/a");
    }

    #[test]
    fn snapshots_ordered_by_time() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::open(tmp.path()).unwrap();
        let b = lcg_bytes(7, 4096);
        repo.snapshot_bytes(SnapshotKind::Sync, "later", 3000, &[("/x", &b)])
            .unwrap();
        repo.snapshot_bytes(SnapshotKind::Copy, "earlier", 1000, &[("/x", &b)])
            .unwrap();
        let list = repo.snapshots().unwrap();
        assert_eq!(list[0].created_at_ms, 1000);
        assert_eq!(list[1].created_at_ms, 3000);
    }

    #[test]
    fn snapshot_files_streams_and_restores() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::open(tmp.path()).unwrap();
        let bytes = lcg_bytes(99, 3 * 1024 * 1024);
        let src = tmp.path().join("src.bin");
        std::fs::write(&src, &bytes).unwrap();

        repo.snapshot_files(SnapshotKind::Backup, "stream", 1000, &[("/s", &src)])
            .unwrap();
        let fs = repo.snapshot_at("/s", 9999).unwrap().unwrap();
        let dst = tmp.path().join("out.bin");
        repo.restore(&fs, &dst).unwrap();
        assert_eq!(std::fs::read(&dst).unwrap(), bytes);
    }

    #[test]
    fn snapshot_at_picks_newest_at_or_before() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::open(tmp.path()).unwrap();
        let v1 = lcg_bytes(10, 4096);
        let mut v2 = v1.clone();
        v2[0] ^= 0xff;
        repo.snapshot_bytes(SnapshotKind::Version, "v1", 1000, &[("/f", &v1)])
            .unwrap();
        repo.snapshot_bytes(SnapshotKind::Version, "v2", 2000, &[("/f", &v2)])
            .unwrap();

        let at1500 = repo.snapshot_at("/f", 1500).unwrap().unwrap();
        assert_eq!(at1500.created_at_ms, 1000);
        let dst = tmp.path().join("out1");
        repo.restore(&at1500, &dst).unwrap();
        assert_eq!(std::fs::read(&dst).unwrap(), v1);

        let at2500 = repo.snapshot_at("/f", 2500).unwrap().unwrap();
        assert_eq!(at2500.created_at_ms, 2000);

        assert!(repo.snapshot_at("/f", 500).unwrap().is_none());
        assert!(repo.snapshot_at("/nope", 9999).unwrap().is_none());
    }

    #[test]
    fn stats_saved_ratio_reflects_dedup() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::open(tmp.path()).unwrap();
        let bytes = lcg_bytes(20, 2 * 1024 * 1024);
        // Same content in three snapshots → effective 3×, unique 1×.
        repo.snapshot_bytes(SnapshotKind::Copy, "s1", 1000, &[("/a", &bytes)])
            .unwrap();
        repo.snapshot_bytes(SnapshotKind::Sync, "s2", 2000, &[("/b", &bytes)])
            .unwrap();
        repo.snapshot_bytes(SnapshotKind::Version, "s3", 3000, &[("/c", &bytes)])
            .unwrap();
        let stats = repo.stats().unwrap();
        assert_eq!(stats.effective_bytes, 3 * bytes.len() as u64);
        // unique ≈ one copy of the content.
        assert!(stats.unique_bytes <= bytes.len() as u64 + 4096);
        // ~2/3 saved; never collapses to 0 despite pack overhead.
        assert!(stats.saved_ratio() > 0.6, "ratio {}", stats.saved_ratio());
    }

    #[test]
    fn chunk_refcount_counts_snapshots() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::open(tmp.path()).unwrap();
        let bytes = lcg_bytes(21, 2 * 1024 * 1024);
        let first = repo.chunker.chunk_bytes(&bytes)[0].hash;
        repo.snapshot_bytes(SnapshotKind::Copy, "s1", 1000, &[("/a", &bytes)])
            .unwrap();
        assert_eq!(repo.chunk_refcount(&first).unwrap(), 1);
        repo.snapshot_bytes(SnapshotKind::Sync, "s2", 2000, &[("/b", &bytes)])
            .unwrap();
        assert_eq!(repo.chunk_refcount(&first).unwrap(), 2);
    }

    #[test]
    fn gc_sweeps_orphans_keeps_live() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::open(tmp.path()).unwrap();
        let a = lcg_bytes(30, 2 * 1024 * 1024);
        let b = lcg_bytes(31, 2 * 1024 * 1024);
        let a_chunk = repo.chunker.chunk_bytes(&a)[0].hash;

        let s1 = repo
            .snapshot_bytes(SnapshotKind::Copy, "a", 1000, &[("/a", &a)])
            .unwrap();
        repo.snapshot_bytes(SnapshotKind::Version, "b", 2000, &[("/b", &b)])
            .unwrap();
        assert!(repo.store().has(&a_chunk).unwrap());

        let untouched = repo.gc().unwrap();
        assert_eq!(untouched.chunks_swept, 0, "gc must not touch live data");
        assert!(repo.store().has(&a_chunk).unwrap());

        assert!(repo.remove_snapshot(s1).unwrap());
        let report = repo.gc().unwrap();
        assert!(report.chunks_swept >= 1, "expected ≥1 sweep");
        assert!(!repo.store().has(&a_chunk).unwrap(), "orphan still present");
        assert_eq!(repo.chunk_refcount(&a_chunk).unwrap(), 0);

        let fs = repo.snapshot_at("/b", 9999).unwrap().unwrap();
        let dst = tmp.path().join("b.out");
        repo.restore(&fs, &dst).unwrap();
        assert_eq!(std::fs::read(&dst).unwrap(), b);
    }

    #[test]
    fn gc_preserves_phase27_manifest_chunks() {
        // A chunk referenced only by the ChunkStore's Phase 27 manifests
        // table (delta-resume / mount / recovery) — never by a Repository
        // snapshot — must survive gc.
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::open(tmp.path()).unwrap();
        let data = lcg_bytes(42, 2 * 1024 * 1024);
        // Ingest via the Phase 27 path (writes the `manifests` table).
        let (_stats, manifest) =
            crate::ingest_bytes(repo.store(), &Chunker::default(), &data, "/delta/key").unwrap();
        let witness = manifest.chunks[0].hash;
        assert!(repo.store().has(&witness).unwrap());

        // No Repository snapshot references it, yet gc must keep it.
        let report = repo.gc().unwrap();
        assert_eq!(report.chunks_swept, 0, "Phase 27 chunks must be reachable");
        assert!(repo.store().has(&witness).unwrap());
    }

    #[test]
    fn gc_on_empty_repo_is_noop() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = Repository::open(tmp.path()).unwrap();
        let report = repo.gc().unwrap();
        assert_eq!(report.chunks_swept, 0);
        assert_eq!(report.packs_removed, 0);
    }
}
