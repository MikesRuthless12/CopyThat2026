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
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use redb::{Database, ReadableTableMetadata, TableDefinition};
use uuid::Uuid;

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
    /// Length in bytes. Also the hint for the `get` read.
    pub len: u32,
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
}

struct ActivePack {
    id: Uuid,
    path: PathBuf,
    size: u64,
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
            packs_dir,
            db,
            active: Mutex::new(active),
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
        let mut total = 0u64;
        let rd = std::fs::read_dir(&self.packs_dir).map_err(|e| ChunkStoreError::Io {
            path: self.packs_dir.clone(),
            source: e,
        })?;
        for ent in rd {
            let ent = ent.map_err(|e| ChunkStoreError::Io {
                path: self.packs_dir.clone(),
                source: e,
            })?;
            if let Ok(m) = ent.metadata() {
                total += m.len();
            }
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
        Ok(Some(decode_locator(v.value())))
    }

    /// Insert `bytes` keyed by `hash`. If the hash is already
    /// present, the insert is a no-op (dedup path). Callers that
    /// care can check `has` first, but `put` is the canonical entry.
    pub fn put(&self, hash: Blake3Hash, bytes: &[u8]) -> Result<()> {
        // Point-read first: dedup fast path avoids the pack write +
        // transaction commit cost.
        {
            let txn = self.db.begin_read()?;
            let tbl = txn.open_table(CHUNKS)?;
            if tbl.get(hash.as_slice())?.is_some() {
                return Ok(());
            }
        }

        let mut active = self
            .active
            .lock()
            .map_err(|_| ChunkStoreError::Redb("active pack mutex poisoned".into()))?;

        // Roll over the pack if it's grown past threshold.
        if active.size >= PACK_ROLLOVER_BYTES {
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
        }

        // Append bytes to the active pack file.
        let offset = active.size;
        {
            let mut f = OpenOptions::new()
                .append(true)
                .open(&active.path)
                .map_err(|e| ChunkStoreError::Io {
                    path: active.path.clone(),
                    source: e,
                })?;
            f.write_all(bytes).map_err(|e| ChunkStoreError::Io {
                path: active.path.clone(),
                source: e,
            })?;
            f.flush().map_err(|e| ChunkStoreError::Io {
                path: active.path.clone(),
                source: e,
            })?;
        }
        active.size += bytes.len() as u64;

        // Index the new chunk.
        let locator = ChunkLocator {
            pack_id: active.id,
            offset,
            len: bytes.len() as u32,
        };
        let encoded = encode_locator(&locator);
        let txn = self.db.begin_write()?;
        {
            let mut tbl = txn.open_table(CHUNKS)?;
            tbl.insert(hash.as_slice(), encoded.as_slice())?;
        }
        txn.commit()?;
        Ok(())
    }

    /// Read the raw bytes for `hash`. Returns `Ok(None)` when the
    /// hash isn't indexed; returns `CorruptChunk` when bytes read
    /// back from the pack don't BLAKE3 to the expected value.
    pub fn get(&self, hash: &Blake3Hash) -> Result<Option<Vec<u8>>> {
        let Some(loc) = self.locate(hash)? else {
            return Ok(None);
        };
        let pack_path = self.packs_dir.join(format!("pack-{}.pack", loc.pack_id));
        let mut f = File::open(&pack_path).map_err(|e| ChunkStoreError::Io {
            path: pack_path.clone(),
            source: e,
        })?;
        f.seek(SeekFrom::Start(loc.offset))
            .map_err(|e| ChunkStoreError::Io {
                path: pack_path.clone(),
                source: e,
            })?;
        let mut buf = vec![0u8; loc.len as usize];
        f.read_exact(&mut buf).map_err(|e| ChunkStoreError::Io {
            path: pack_path.clone(),
            source: e,
        })?;
        let got = blake3::hash(&buf);
        if got.as_bytes() != hash {
            return Err(ChunkStoreError::CorruptChunk { hash: hex_of(hash) });
        }
        Ok(Some(buf))
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
}

fn encode_locator(loc: &ChunkLocator) -> [u8; 28] {
    let mut out = [0u8; 28];
    out[0..16].copy_from_slice(loc.pack_id.as_bytes());
    out[16..24].copy_from_slice(&loc.offset.to_le_bytes());
    out[24..28].copy_from_slice(&loc.len.to_le_bytes());
    out
}

fn decode_locator(bytes: &[u8]) -> ChunkLocator {
    debug_assert_eq!(bytes.len(), 28, "locator must be 28 bytes");
    let mut id_bytes = [0u8; 16];
    id_bytes.copy_from_slice(&bytes[0..16]);
    let mut off_bytes = [0u8; 8];
    off_bytes.copy_from_slice(&bytes[16..24]);
    let mut len_bytes = [0u8; 4];
    len_bytes.copy_from_slice(&bytes[24..28]);
    ChunkLocator {
        pack_id: Uuid::from_bytes(id_bytes),
        offset: u64::from_le_bytes(off_bytes),
        len: u32::from_le_bytes(len_bytes),
    }
}

/// Default path: `<data-dir>/chunks/`. Sits next to `history.db` +
/// `copythat-journal.redb` under the Copy That project dir.
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
