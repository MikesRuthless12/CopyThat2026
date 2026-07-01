//! The on-disk chunk store.
//!
//! Layout under `root`:
//!
//! ```text
//! <root>/
//!   index.redb               # chunk hash → (pack id, offset, len), manifests
//!   packs/
//!     pack-<uuid>.pack       # concatenated chunk bytes
//! ```
//!
//! One redb file backs two tables:
//!
//! - `chunks`: 32-byte BLAKE3 → 24-byte record `(pack_id: u128, offset: u64, len: u32, _pad: u32)`.
//! - `manifests`: UTF-8 key → JSON bytes of [`Manifest`].
//!
//! # Why pack files?
//!
//! FastCDC's min/avg/max of 512 KiB / 1 MiB / 4 MiB keeps *most*
//! chunks large enough that per-chunk syscall overhead is low. But
//! the tail of any text file drops below 64 KiB, and an all-zero
//! sparse region can cut into ~1 KiB fragments. Rather than one file
//! per chunk (inode bloat + directory listing cost) we append into
//! rolling pack files keyed by a random UUID. The redb index stores
//! the offset into the current pack, so reads become one open +
//! pread.
//!
//! A live pack is capped at 1 GiB by default — once exceeded, the
//! next `put` opens a fresh `pack-<uuid>.pack`. Stale packs after
//! eventual GC (Phase 49's reference counter) can be reclaimed by
//! deleting the file + dropping its rows from the index.
//!
//! # Atomicity
//!
//! Each `put` does three things in order:
//!
//! 1. Append the chunk bytes to the active pack file and `flush`.
//! 2. In a redb write transaction, insert the index row.
//! 3. Commit.
//!
//! A crash between step 1 and step 3 leaves unreferenced bytes at
//! the tail of the pack — harmless, trimmed on the next opening
//! `Repository::compact` call (Phase 49). A crash between step 2 and
//! step 3 cannot produce a half-committed index because redb is
//! ACID.

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use redb::{Database, ReadableTable, ReadableTableMetadata, TableDefinition};
use uuid::Uuid;

use crate::compress::{ChunkCodec, RepoCompression};
use crate::error::{ChunkStoreError, Result};
use crate::types::{Blake3Hash, Manifest, hex_of};

/// Table: chunk hash → packed locator.
///
/// The locator is serialised as a fixed 28-byte little-endian block
/// so we don't pull serde + bincode into the hot path:
///
/// ```text
/// bytes 0..16  : pack_id  (UUID as 128-bit LE)
/// bytes 16..24 : offset   (u64 LE)
/// bytes 24..28 : len      (u32 LE)
/// ```
const CHUNKS: TableDefinition<'_, &[u8], &[u8]> = TableDefinition::new("chunks");

/// Table: manifest key (UTF-8) → serialised manifest JSON.
const MANIFESTS: TableDefinition<'_, &str, &[u8]> = TableDefinition::new("manifests");

/// Stats table: bookkeeping values (active pack id + size).
const STATS: TableDefinition<'_, &str, &[u8]> = TableDefinition::new("stats");

/// Rollover threshold: a live pack beyond this size forces a new pack
/// to be created on the next `put`. 1 GiB is small enough to keep any
/// single pack under typical cloud-file-size limits (S3 single-PUT
/// max is 5 GiB, SMB is unbounded but `fstat` on huge pack files is
/// annoying to debug) and large enough to amortise the `open` cost
/// across many chunks.
pub const PACK_ROLLOVER_BYTES: u64 = 1024 * 1024 * 1024;

/// A chunk's locator inside the pack store.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkLocator {
    /// Pack file UUID. Resolved to a path via `<root>/packs/pack-<uuid>.pack`.
    pub pack_id: Uuid,
    /// Byte offset within the pack.
    pub offset: u64,
    /// LOGICAL (plaintext) length — the size `get` returns; unchanged
    /// meaning from the v0 format.
    pub len: u32,
    /// Phase 49h — bytes actually stored in the pack at `offset`
    /// (`== len` when `codec == None`). Legacy v0 records decode with
    /// `stored_len = len`.
    pub stored_len: u32,
    /// Phase 49h — how the stored bytes are encoded.
    pub codec: ChunkCodec,
}

/// The chunk store.
///
/// Cheap to clone — the inner redb database is held behind an `Arc`
/// transparently by redb, and the active pack writer lives behind a
/// `Mutex` so concurrent `put` calls serialise correctly.
pub struct ChunkStore {
    root: PathBuf,
    packs_dir: PathBuf,
    db: Database,
    active: Mutex<ActivePack>,
    /// Phase 49i — active-pack rollover threshold. Overridable via
    /// [`Self::open_with_rollover`] so compaction tests can force many
    /// small packs; production uses [`PACK_ROLLOVER_BYTES`].
    rollover_bytes: u64,
    /// Phase 50g — pluggable pack-blob storage. `LocalFsBackend(packs_dir)`
    /// by default (byte-for-byte the pre-50g behaviour); the read path routes
    /// through it so the store can later live on a remote object backend.
    backend: Box<dyn crate::backend::BlobBackend>,
}

struct ActivePack {
    id: Uuid,
    path: PathBuf,
    size: u64,
}

/// Phase 49i — per-pack live accounting for the maintenance/compaction UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PackStat {
    /// Pack file UUID.
    pub pack_id: Uuid,
    /// On-disk size of the pack file.
    pub file_bytes: u64,
    /// Sum of `stored_len` over the live index rows pointing at this pack.
    pub live_bytes: u64,
    /// Count of live chunks in this pack.
    pub live_chunks: u64,
    /// Whether this is the active (currently-appended) pack — never compacted.
    pub is_active: bool,
}

impl std::fmt::Debug for ChunkStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChunkStore")
            .field("root", &self.root)
            .finish()
    }
}

impl ChunkStore {
    /// Open (or create) a chunk store rooted at `root`.
    ///
    /// Creates the directory + `packs/` subdir if missing and seeds
    /// the redb tables on first call.
    pub fn open(root: &Path) -> Result<Self> {
        Self::open_with_rollover(root, PACK_ROLLOVER_BYTES)
    }

    /// Open with a custom active-pack rollover threshold (Phase 49i). The
    /// production [`Self::open`] uses [`PACK_ROLLOVER_BYTES`]; a small value
    /// here forces many packs so compaction can be exercised in tests.
    pub fn open_with_rollover(root: &Path, rollover_bytes: u64) -> Result<Self> {
        std::fs::create_dir_all(root).map_err(|e| ChunkStoreError::Io {
            path: root.to_path_buf(),
            source: e,
        })?;
        let packs_dir = root.join("packs");
        std::fs::create_dir_all(&packs_dir).map_err(|e| ChunkStoreError::Io {
            path: packs_dir.clone(),
            source: e,
        })?;
        let db_path = root.join("index.redb");
        let db = Database::create(&db_path)?;
        // Materialise the tables on first open.
        {
            let txn = db.begin_write()?;
            let _ = txn.open_table(CHUNKS)?;
            let _ = txn.open_table(MANIFESTS)?;
            let _ = txn.open_table(STATS)?;
            txn.commit()?;
        }

        // Determine the active pack: either restore the one saved in
        // `stats`, or open a fresh one.
        let active = {
            let txn = db.begin_read()?;
            let stats = txn.open_table(STATS)?;
            let prev_id = stats
                .get("active_pack_id")?
                .and_then(|v| Uuid::from_slice(v.value()).ok());
            drop(stats);
            drop(txn);
            match prev_id {
                Some(id) => {
                    let path = packs_dir.join(format!("pack-{id}.pack"));
                    let size = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
                    ActivePack { id, path, size }
                }
                None => {
                    let id = Uuid::new_v4();
                    let path = packs_dir.join(format!("pack-{id}.pack"));
                    // Touch the file so later `append`s succeed.
                    File::create(&path).map_err(|e| ChunkStoreError::Io {
                        path: path.clone(),
                        source: e,
                    })?;
                    // Persist the id so a restart picks up the same pack.
                    let txn = db.begin_write()?;
                    {
                        let mut stats = txn.open_table(STATS)?;
                        stats.insert("active_pack_id", id.as_bytes().as_slice())?;
                    }
                    txn.commit()?;
                    ActivePack { id, path, size: 0 }
                }
            }
        };

        Ok(Self {
            root: root.to_path_buf(),
            backend: Box::new(crate::backend::LocalFsBackend::new(packs_dir.clone())),
            packs_dir,
            db,
            active: Mutex::new(active),
            rollover_bytes,
        })
    }

    /// Root directory (the value passed to `open`).
    #[must_use]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Total bytes across all pack files under `<root>/packs/`. Used
    /// by the spec's smoke case 3 and by the Settings "disk usage"
    /// readout.
    pub fn disk_usage_bytes(&self) -> Result<u64> {
        // Phase 50g — sum pack sizes via the backend.
        let mut total = 0u64;
        for name in self.backend.list("pack-")? {
            total += self.backend.size(&name)?.unwrap_or(0);
        }
        Ok(total)
    }

    /// Count of distinct chunks currently indexed. Drops as GC runs.
    pub fn chunk_count(&self) -> Result<u64> {
        let txn = self.db.begin_read()?;
        let tbl = txn.open_table(CHUNKS)?;
        Ok(tbl.len()?)
    }

    /// `true` if the store already has an entry for `hash`. No pack
    /// I/O, just a redb point-read.
    pub fn has(&self, hash: &Blake3Hash) -> Result<bool> {
        let txn = self.db.begin_read()?;
        let tbl = txn.open_table(CHUNKS)?;
        Ok(tbl.get(hash.as_slice())?.is_some())
    }

    /// Look up a chunk's locator without reading its bytes.
    pub fn locate(&self, hash: &Blake3Hash) -> Result<Option<ChunkLocator>> {
        let txn = self.db.begin_read()?;
        let tbl = txn.open_table(CHUNKS)?;
        let Some(v) = tbl.get(hash.as_slice())? else {
            return Ok(None);
        };
        Ok(Some(decode_locator(v.value())?))
    }

    /// Insert `plaintext` keyed by `hash`, encoding the stored bytes per
    /// `comp` (Phase 49h). On a dedup hit returns the EXISTING chunk's
    /// codec (first writer wins) so the caller's manifest records the
    /// truth; otherwise the codec actually used. The hash is always
    /// `BLAKE3(plaintext)`, independent of codec.
    pub fn put_chunk(
        &self,
        hash: Blake3Hash,
        plaintext: &[u8],
        comp: RepoCompression,
    ) -> Result<ChunkCodec> {
        // Point-read first: dedup fast path avoids the pack write + commit
        // cost. On a hit, return the EXISTING chunk's codec (first writer
        // wins) so the caller records the truth, not what it would pick.
        {
            let txn = self.db.begin_read()?;
            let tbl = txn.open_table(CHUNKS)?;
            if let Some(v) = tbl.get(hash.as_slice())? {
                return Ok(decode_locator(v.value())?.codec);
            }
        }

        let mut active = self
            .active
            .lock()
            .map_err(|_| ChunkStoreError::Redb("active pack mutex poisoned".into()))?;

        // Re-check dedup UNDER the active-pack lock. Two concurrent record
        // paths (e.g. a manual "Back up now" racing a scheduled tick) each
        // hold only a shared read lease, so both can miss the unlocked
        // point-read above for the same new chunk. Without this second check
        // both would append a physical copy — dead space gc can't reclaim
        // until a full compaction — and both would count it as a new chunk.
        {
            let txn = self.db.begin_read()?;
            let tbl = txn.open_table(CHUNKS)?;
            if let Some(v) = tbl.get(hash.as_slice())? {
                return Ok(decode_locator(v.value())?.codec);
            }
        }

        let (codec, stored) = comp.encode(plaintext);

        // Roll over the pack if it's grown past threshold.
        if active.size >= self.rollover_bytes {
            self.roll_locked(&mut active)?;
        }

        // Append the STORED bytes (codec-encoded) to the active pack file.
        let offset = active.size;
        {
            let mut f = OpenOptions::new()
                .append(true)
                .open(&active.path)
                .map_err(|e| ChunkStoreError::Io {
                    path: active.path.clone(),
                    source: e,
                })?;
            f.write_all(&stored).map_err(|e| ChunkStoreError::Io {
                path: active.path.clone(),
                source: e,
            })?;
            f.flush().map_err(|e| ChunkStoreError::Io {
                path: active.path.clone(),
                source: e,
            })?;
        }
        active.size += stored.len() as u64;

        // Index the new chunk: logical len = plaintext, stored_len = on-disk.
        let locator = ChunkLocator {
            pack_id: active.id,
            offset,
            len: plaintext.len() as u32,
            stored_len: stored.len() as u32,
            codec,
        };
        let encoded = encode_locator(&locator);
        let txn = self.db.begin_write()?;
        {
            let mut tbl = txn.open_table(CHUNKS)?;
            tbl.insert(hash.as_slice(), encoded.as_slice())?;
        }
        txn.commit()?;
        Ok(codec)
    }

    /// Insert `bytes` keyed by `hash` UNCOMPRESSED — a back-compat shim
    /// over [`Self::put_chunk`] with [`RepoCompression::Off`]. If the hash
    /// is already present, the insert is a no-op (dedup path).
    pub fn put(&self, hash: Blake3Hash, bytes: &[u8]) -> Result<()> {
        self.put_chunk(hash, bytes, RepoCompression::Off)
            .map(|_| ())
    }

    /// Read the raw bytes for `hash`. Returns `Ok(None)` when the
    /// hash isn't indexed; returns `CorruptChunk` when bytes read
    /// back from the pack don't BLAKE3 to the expected value.
    pub fn get(&self, hash: &Blake3Hash) -> Result<Option<Vec<u8>>> {
        let Some(loc) = self.locate(hash)? else {
            return Ok(None);
        };
        // Phase 50g — read through the blob backend (LocalFsBackend seeks the
        // pack file; a remote backend would range-get the object).
        let pack_name = format!("pack-{}.pack", loc.pack_id);
        let stored = self
            .backend
            .get_range(&pack_name, loc.offset, loc.stored_len as usize)?
            .ok_or_else(|| ChunkStoreError::MissingChunk { hash: hex_of(hash) })?;
        // Decode the stored bytes back to plaintext. The zstd decompress is
        // capacity-bounded by the known logical length, so a crafted frame
        // can't blow memory (no decompression bomb).
        let plaintext = match loc.codec {
            ChunkCodec::None => stored,
            ChunkCodec::Zstd => zstd::bulk::decompress(&stored, loc.len as usize)
                .map_err(|_| ChunkStoreError::CorruptChunk { hash: hex_of(hash) })?,
        };
        let got = blake3::hash(&plaintext);
        if got.as_bytes() != hash {
            return Err(ChunkStoreError::CorruptChunk { hash: hex_of(hash) });
        }
        Ok(Some(plaintext))
    }

    /// Persist a manifest under `key`. `key` is opaque to the store
    /// — callers typically use the destination's absolute path or a
    /// `<job-id>/<rel-path>` pair for the delta-resume lookup.
    pub fn put_manifest(&self, key: &str, manifest: &Manifest) -> Result<()> {
        let bytes = serde_json::to_vec(manifest)?;
        let txn = self.db.begin_write()?;
        {
            let mut tbl = txn.open_table(MANIFESTS)?;
            tbl.insert(key, bytes.as_slice())?;
        }
        txn.commit()?;
        Ok(())
    }

    /// Read a manifest by key. Returns `Ok(None)` when the key is
    /// absent.
    pub fn get_manifest(&self, key: &str) -> Result<Option<Manifest>> {
        let txn = self.db.begin_read()?;
        let tbl = txn.open_table(MANIFESTS)?;
        let Some(v) = tbl.get(key)? else {
            return Ok(None);
        };
        let manifest: Manifest = serde_json::from_slice(v.value())?;
        Ok(Some(manifest))
    }

    // ===== Phase 49 — Repository garbage-collection support =====
    //
    // These helpers are crate-private: only `Repository` (the Phase 49
    // unified store that owns a `ChunkStore` as a sub-component) calls
    // them, and only from its `gc()` mark-and-sweep. They are kept off
    // the public surface so the Phase 27 contract stays "put / get /
    // never lose a chunk" — deletion is a Repository-level decision
    // driven by reference counting, never an ad-hoc store operation.

    /// Snapshot every `(hash, locator)` currently in the chunk index.
    ///
    /// Used by `Repository::gc` to enumerate the live set so it can
    /// diff it against the set reachable from all snapshots. Reads a
    /// single redb transaction; the returned vector is a point-in-time
    /// copy, so a concurrent `put` after this call simply isn't seen by
    /// this GC pass (it will be on the next one).
    pub(crate) fn all_locators(&self) -> Result<Vec<(Blake3Hash, ChunkLocator)>> {
        let txn = self.db.begin_read()?;
        let tbl = txn.open_table(CHUNKS)?;
        let mut out = Vec::with_capacity(tbl.len()? as usize);
        for row in tbl.iter()? {
            let (k, v) = row?;
            let key = k.value();
            // Every key we ever insert is a 32-byte BLAKE3 digest
            // (`hash.as_slice()`); skip anything else defensively
            // rather than panicking on a corrupt index.
            if key.len() != 32 {
                continue;
            }
            let mut hash = [0u8; 32];
            hash.copy_from_slice(key);
            out.push((hash, decode_locator(v.value())?));
        }
        Ok(out)
    }

    /// Drop the index rows for `hashes`. The chunk *bytes* remain in
    /// their pack file until the whole pack is reclaimed by
    /// [`Self::remove_pack_file`]; this only makes the chunks
    /// unreachable via `get` / `has`. No-op for hashes that are absent.
    pub(crate) fn remove_chunk_index_entries(&self, hashes: &[Blake3Hash]) -> Result<()> {
        if hashes.is_empty() {
            return Ok(());
        }
        let txn = self.db.begin_write()?;
        {
            let mut tbl = txn.open_table(CHUNKS)?;
            for h in hashes {
                tbl.remove(h.as_slice())?;
            }
        }
        txn.commit()?;
        Ok(())
    }

    /// The id of the pack currently being appended to. GC must never
    /// delete this pack — the active writer holds it open and future
    /// `put`s land in it.
    pub(crate) fn active_pack_id(&self) -> Result<Uuid> {
        let active = self
            .active
            .lock()
            .map_err(|_| ChunkStoreError::Redb("active pack mutex poisoned".into()))?;
        Ok(active.id)
    }

    /// Delete the pack file for `id` and return the number of bytes
    /// freed. Caller guarantees no live index row still points into it
    /// (and that it is not the active pack). A missing file is treated
    /// as already-reclaimed (`Ok(0)`), so GC is idempotent across a
    /// crash mid-sweep.
    pub(crate) fn remove_pack_file(&self, id: Uuid) -> Result<u64> {
        // Phase 50g — reclaim through the backend (LocalFs removes the file;
        // a remote backend deletes the object). Idempotent (0 if absent).
        self.backend.delete(&format!("pack-{id}.pack"))
    }

    /// Every Phase 27 manifest currently persisted in the `manifests`
    /// table. The Repository's GC treats these as additional reachability
    /// roots: the chunk store is shared, so a chunk referenced only by a
    /// delta-resume / mount / recovery manifest (never by a Repository
    /// snapshot) must not be swept.
    pub(crate) fn all_manifests(&self) -> Result<Vec<Manifest>> {
        let txn = self.db.begin_read()?;
        let tbl = txn.open_table(MANIFESTS)?;
        let mut out = Vec::with_capacity(tbl.len()? as usize);
        for row in tbl.iter()? {
            let (_k, v) = row?;
            out.push(serde_json::from_slice::<Manifest>(v.value())?);
        }
        Ok(out)
    }

    // ----- Phase 49i — maintenance / compaction -----

    /// Replace the active pack with a fresh one. Caller holds the `active`
    /// lock. The old active pack becomes a normal, compactable pack;
    /// persists the new id so a restart resumes it.
    fn roll_locked(&self, active: &mut ActivePack) -> Result<()> {
        let id = Uuid::new_v4();
        let path = self.packs_dir.join(format!("pack-{id}.pack"));
        File::create(&path).map_err(|e| ChunkStoreError::Io {
            path: path.clone(),
            source: e,
        })?;
        *active = ActivePack { id, path, size: 0 };
        let txn = self.db.begin_write()?;
        {
            let mut stats = txn.open_table(STATS)?;
            stats.insert("active_pack_id", active.id.as_bytes().as_slice())?;
        }
        txn.commit()?;
        Ok(())
    }

    /// Force a fresh active pack so the *current* active pack becomes
    /// compactable (its dead bytes can then be reclaimed). (Phase 49i.)
    pub(crate) fn roll_active_pack(&self) -> Result<()> {
        let mut active = self
            .active
            .lock()
            .map_err(|_| ChunkStoreError::Redb("active pack mutex poisoned".into()))?;
        self.roll_locked(&mut active)
    }

    /// Per-pack live accounting. After a [`crate::Repository::gc`] sweep
    /// every index row is live, so `live_bytes(pack) = Σ stored_len` of the
    /// rows pointing at it and `dead = file_bytes - live_bytes`. A pack with
    /// zero live rows still appears (so callers see fully-dead packs).
    /// (Phase 49i.)
    pub(crate) fn pack_stats(&self) -> Result<Vec<PackStat>> {
        use std::collections::HashMap;
        let active_id = self.active_pack_id()?;
        let mut live: HashMap<Uuid, (u64, u64)> = HashMap::new();
        for (_h, loc) in self.all_locators()? {
            let e = live.entry(loc.pack_id).or_insert((0, 0));
            e.0 += u64::from(loc.stored_len);
            e.1 += 1;
        }
        let mut out = Vec::new();
        // Phase 50g — enumerate packs + their sizes via the backend.
        for name in self.backend.list("pack-")? {
            let Some(id_hex) = name
                .strip_prefix("pack-")
                .and_then(|s| s.strip_suffix(".pack"))
            else {
                continue;
            };
            let Ok(pack_id) = Uuid::parse_str(id_hex) else {
                continue;
            };
            let file_bytes = self.backend.size(&name)?.unwrap_or(0);
            let (live_bytes, live_chunks) = live.get(&pack_id).copied().unwrap_or((0, 0));
            out.push(PackStat {
                pack_id,
                file_bytes,
                live_bytes,
                live_chunks,
                is_active: pack_id == active_id,
            });
        }
        Ok(out)
    }

    /// Move every live chunk out of `pack_id` into the active pack
    /// (rolling as needed), repoint its index rows verbatim (same
    /// `stored_len`/`len`/`codec` — bytes copied, NO re-decompress), then
    /// delete the old pack file. Returns the bytes the old file freed.
    /// (Phase 49i.)
    ///
    /// **Crash safety:** each chunk is appended to the active pack and its
    /// index row repointed in one redb commit; the old pack is deleted only
    /// after *all* rows are repointed, so a crash mid-move just leaves a
    /// partially-drained pack the next pass finishes. **Cancellation** is
    /// cooperative (checked between chunks); on cancel the old pack is left
    /// intact and `Ok(0)` is returned.
    pub(crate) fn compact_pack(
        &self,
        pack_id: Uuid,
        cancel: &dyn Fn() -> bool,
        progress: &mut dyn FnMut(u64),
    ) -> Result<u64> {
        let active_id = self.active_pack_id()?;
        assert_ne!(pack_id, active_id, "cannot compact the active pack");

        // Point-in-time set of live chunks living in this pack.
        let live: Vec<(Blake3Hash, ChunkLocator)> = self
            .all_locators()?
            .into_iter()
            .filter(|(_, loc)| loc.pack_id == pack_id)
            .collect();

        let old_name = format!("pack-{pack_id}.pack");

        let mut moved = 0u64;
        for (hash, loc) in &live {
            if cancel() {
                return Ok(0);
            }
            // Read the STORED bytes verbatim via the backend (no decode —
            // codec preserved).
            let stored = self
                .backend
                .get_range(&old_name, loc.offset, loc.stored_len as usize)?
                .ok_or_else(|| ChunkStoreError::MissingChunk { hash: hex_of(hash) })?;
            // A short read (truncated / corrupt source pack — `get_range`
            // clamps to the object's real length) must not be silently
            // re-recorded under the original `stored_len`.
            if stored.len() != loc.stored_len as usize {
                return Err(ChunkStoreError::CorruptChunk { hash: hex_of(hash) });
            }

            // Append to the active pack (rolling if full), then repoint the
            // index row — committed together so a crash can't lose the chunk.
            let mut active = self
                .active
                .lock()
                .map_err(|_| ChunkStoreError::Redb("active pack mutex poisoned".into()))?;
            if active.size >= self.rollover_bytes {
                self.roll_locked(&mut active)?;
            }
            let new_offset = active.size;
            {
                let mut f = OpenOptions::new()
                    .append(true)
                    .open(&active.path)
                    .map_err(|e| ChunkStoreError::Io {
                        path: active.path.clone(),
                        source: e,
                    })?;
                f.write_all(&stored).map_err(|e| ChunkStoreError::Io {
                    path: active.path.clone(),
                    source: e,
                })?;
                f.flush().map_err(|e| ChunkStoreError::Io {
                    path: active.path.clone(),
                    source: e,
                })?;
            }
            active.size += stored.len() as u64;
            let new_loc = ChunkLocator {
                pack_id: active.id,
                offset: new_offset,
                len: loc.len,
                stored_len: loc.stored_len,
                codec: loc.codec,
            };
            let encoded = encode_locator(&new_loc);
            let txn = self.db.begin_write()?;
            {
                let mut tbl = txn.open_table(CHUNKS)?;
                tbl.insert(hash.as_slice(), encoded.as_slice())?;
            }
            txn.commit()?;
            drop(active);
            moved += 1;
            progress(moved);
        }

        // All rows repointed → reclaim the old pack via the backend.
        self.remove_pack_file(pack_id)
    }
}

/// v1 on-disk locator layout (34 bytes), written from Phase 49h onward:
/// `0..16 pack_id | 16..24 offset | 24..28 stored_len | 28..32 len |
/// 32 codec | 33 reserved(0)`. The legacy v0 layout (28 bytes) is still
/// READ (never written again): `… | 24..28 len → {stored_len = len,
/// codec = None}` — so existing chunk stores keep working with zero
/// rewrite (the dedup key never moved; old records ARE uncompressed).
fn encode_locator(loc: &ChunkLocator) -> [u8; 34] {
    let mut out = [0u8; 34];
    out[0..16].copy_from_slice(loc.pack_id.as_bytes());
    out[16..24].copy_from_slice(&loc.offset.to_le_bytes());
    out[24..28].copy_from_slice(&loc.stored_len.to_le_bytes());
    out[28..32].copy_from_slice(&loc.len.to_le_bytes());
    out[32] = loc.codec.as_u8();
    // out[33] reserved = 0
    out
}

fn decode_locator(bytes: &[u8]) -> Result<ChunkLocator> {
    let mut id_bytes = [0u8; 16];
    let mut off_bytes = [0u8; 8];
    match bytes.len() {
        28 => {
            // v0 — legacy uncompressed record.
            id_bytes.copy_from_slice(&bytes[0..16]);
            off_bytes.copy_from_slice(&bytes[16..24]);
            let mut len_bytes = [0u8; 4];
            len_bytes.copy_from_slice(&bytes[24..28]);
            let len = u32::from_le_bytes(len_bytes);
            Ok(ChunkLocator {
                pack_id: Uuid::from_bytes(id_bytes),
                offset: u64::from_le_bytes(off_bytes),
                len,
                stored_len: len,
                codec: ChunkCodec::None,
            })
        }
        34 => {
            id_bytes.copy_from_slice(&bytes[0..16]);
            off_bytes.copy_from_slice(&bytes[16..24]);
            let mut sl = [0u8; 4];
            sl.copy_from_slice(&bytes[24..28]);
            let mut ln = [0u8; 4];
            ln.copy_from_slice(&bytes[28..32]);
            let codec = ChunkCodec::from_u8(bytes[32]).ok_or_else(|| {
                ChunkStoreError::Redb(format!("unknown chunk codec tag {}", bytes[32]))
            })?;
            Ok(ChunkLocator {
                pack_id: Uuid::from_bytes(id_bytes),
                offset: u64::from_le_bytes(off_bytes),
                stored_len: u32::from_le_bytes(sl),
                len: u32::from_le_bytes(ln),
                codec,
            })
        }
        n => Err(ChunkStoreError::Redb(format!(
            "corrupt chunk locator: {n} bytes (expected 28 or 34)"
        ))),
    }
}

/// Default path: `<data-dir>/chunks/`. Sits next to `history.db` +
/// `copythat-journal.redb` under the Freally File Manager project dir.
pub fn default_chunk_store_path() -> Result<PathBuf> {
    let dirs = directories::ProjectDirs::from("com", "CopyThat", "CopyThat2026")
        .ok_or(ChunkStoreError::NoDataDir)?;
    Ok(dirs.data_dir().join("chunks"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Manifest;

    #[test]
    fn locator_v0_v1_round_trip_and_compat() {
        let id = Uuid::new_v4();
        let v1 = ChunkLocator {
            pack_id: id,
            offset: 4096,
            len: 1000,
            stored_len: 400,
            codec: ChunkCodec::Zstd,
        };
        let bytes = encode_locator(&v1);
        assert_eq!(bytes.len(), 34);
        assert_eq!(decode_locator(&bytes).unwrap(), v1);

        // A legacy 28-byte v0 record decodes as uncompressed (stored_len == len).
        let mut v0 = [0u8; 28];
        v0[0..16].copy_from_slice(id.as_bytes());
        v0[16..24].copy_from_slice(&4096u64.to_le_bytes());
        v0[24..28].copy_from_slice(&1000u32.to_le_bytes());
        let decoded = decode_locator(&v0).unwrap();
        assert_eq!(decoded.pack_id, id);
        assert_eq!(decoded.len, 1000);
        assert_eq!(decoded.stored_len, 1000);
        assert_eq!(decoded.codec, ChunkCodec::None);

        // Wrong length + unknown codec tag are typed errors, not panics.
        assert!(decode_locator(&[0u8; 20]).is_err());
        let mut bad = encode_locator(&v1);
        bad[32] = 99;
        assert!(decode_locator(&bad).is_err());
    }

    #[test]
    fn open_creates_layout() {
        let tmp = tempfile::tempdir().unwrap();
        let store = ChunkStore::open(tmp.path()).unwrap();
        assert!(store.root().exists());
        assert!(store.root().join("index.redb").exists());
        assert!(store.root().join("packs").exists());
    }

    #[test]
    fn put_get_round_trip() {
        let tmp = tempfile::tempdir().unwrap();
        let store = ChunkStore::open(tmp.path()).unwrap();
        let bytes = b"hello world chunk bytes".to_vec();
        let hash = *blake3::hash(&bytes).as_bytes();
        assert!(!store.has(&hash).unwrap());
        store.put(hash, &bytes).unwrap();
        assert!(store.has(&hash).unwrap());
        let got = store.get(&hash).unwrap().unwrap();
        assert_eq!(got, bytes);
    }

    #[test]
    fn put_dedup_is_noop() {
        let tmp = tempfile::tempdir().unwrap();
        let store = ChunkStore::open(tmp.path()).unwrap();
        let bytes = vec![42u8; 10_000];
        let hash = *blake3::hash(&bytes).as_bytes();
        store.put(hash, &bytes).unwrap();
        let size_before = store.disk_usage_bytes().unwrap();
        // Same bytes, same hash → already-indexed fast path; pack
        // size must not grow.
        store.put(hash, &bytes).unwrap();
        let size_after = store.disk_usage_bytes().unwrap();
        assert_eq!(size_before, size_after);
        assert_eq!(store.chunk_count().unwrap(), 1);
    }

    #[test]
    fn manifest_round_trip() {
        let tmp = tempfile::tempdir().unwrap();
        let store = ChunkStore::open(tmp.path()).unwrap();
        let m = Manifest {
            file_hash: [7u8; 32],
            size: 1234,
            chunks: vec![],
        };
        assert!(store.get_manifest("k").unwrap().is_none());
        store.put_manifest("k", &m).unwrap();
        let got = store.get_manifest("k").unwrap().unwrap();
        assert_eq!(got, m);
    }

    #[test]
    fn reopens_pick_up_existing_packs() {
        let tmp = tempfile::tempdir().unwrap();
        let bytes = vec![1u8; 512];
        let hash = *blake3::hash(&bytes).as_bytes();
        {
            let store = ChunkStore::open(tmp.path()).unwrap();
            store.put(hash, &bytes).unwrap();
        }
        // Reopen — the chunk should still be there without a rebuild.
        let store2 = ChunkStore::open(tmp.path()).unwrap();
        assert!(store2.has(&hash).unwrap());
        assert_eq!(store2.get(&hash).unwrap().unwrap(), bytes);
    }

    #[test]
    fn corrupt_chunk_detected() {
        let tmp = tempfile::tempdir().unwrap();
        let store = ChunkStore::open(tmp.path()).unwrap();
        let bytes = vec![9u8; 2048];
        let hash = *blake3::hash(&bytes).as_bytes();
        store.put(hash, &bytes).unwrap();
        // Corrupt the pack on disk.
        let pack = std::fs::read_dir(store.root().join("packs"))
            .unwrap()
            .next()
            .unwrap()
            .unwrap()
            .path();
        let mut contents = std::fs::read(&pack).unwrap();
        contents[0] ^= 0xff;
        std::fs::write(&pack, contents).unwrap();
        // Get should raise CorruptChunk rather than returning the
        // wrong bytes.
        let err = store.get(&hash).unwrap_err();
        assert!(matches!(err, ChunkStoreError::CorruptChunk { .. }));
    }
}
