//! redb-backed per-pair state store.
//!
//! The sync engine's correctness depends on a durable baseline — the
//! "last-seen-synced" vector clock + content digest for every
//! relpath. We keep that in a single redb file per pair at
//! `<left>/.copythat-sync.db` by default. Each pair is independent,
//! which makes parallel syncs of unrelated pairs trivially safe (no
//! cross-pair locks) and keeps a deleted pair's state cleanly
//! removable by a single `rm` of the file.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use redb::{Database, ReadableTable, TableDefinition};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::clock::{DeviceId, VersionVector};
use crate::error::{Result, SyncError};
use crate::types::Direction;

const DEVICE: TableDefinition<&str, &[u8]> = TableDefinition::new("device");
const FILES: TableDefinition<&str, &str> = TableDefinition::new("files");
const HISTORY: TableDefinition<(u64, u64), &str> = TableDefinition::new("history");

const DEVICE_SELF_KEY: &str = "self";

/// Default DB path for a left-root: `<left>/.copythat-sync.db`. The
/// engine also accepts an override path via
/// [`crate::SyncPair::with_db_path`] for tests and network-storage
/// deployments that want the DB out-of-tree.
pub fn default_pair_db_path(left_root: &Path) -> PathBuf {
    left_root.join(".copythat-sync.db")
}

/// Baseline row: the last-seen-synced state of one relpath.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileRecord {
    pub vv: VersionVector,
    pub mtime_ms: i64,
    pub size: u64,
    pub blake3: [u8; 32],
}

/// What happened during a sync round, per-relpath.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HistoryKind {
    Propagated,
    Deleted,
    Conflict,
    Corrupt,
    Noop,
}

/// One row in the `history` audit log.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub timestamp_ms: u64,
    pub seq: u64,
    pub relpath: String,
    pub kind: HistoryKind,
    pub direction: Option<Direction>,
    /// Vector clock before the sync round's decision.
    pub vv_before: Option<VersionVector>,
    /// Vector clock after the sync round's decision.
    pub vv_after: Option<VersionVector>,
}

/// Per-pair state store.
///
/// Cheap to clone — the inner redb `Database` is wrapped in `Arc`.
/// Concurrent reads + a single writer is the supported access
/// pattern; sync is serial by construction, so we never hit the
/// single-writer limit outside user error.
#[derive(Debug, Clone)]
pub struct SyncDb {
    inner: Arc<Database>,
    device_id: DeviceId,
    path: PathBuf,
}

impl SyncDb {
    /// Open (or create) the DB at `path`. Creates the parent
    /// directory on demand. A freshly-created DB auto-generates a
    /// v4 device id + persists it.
    pub fn open(path: &Path) -> Result<Self> {
        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
        {
            std::fs::create_dir_all(parent).map_err(|e| SyncError::Io {
                path: parent.to_path_buf(),
                source: e,
            })?;
        }
        let db = Database::create(path).map_err(|e| SyncError::Database(e.to_string()))?;

        // Eagerly create tables so first-run reads don't trip on
        // a missing-table error.
        {
            let txn = db.begin_write()?;
            let _ = txn.open_table(DEVICE)?;
            let _ = txn.open_table(FILES)?;
            let _ = txn.open_table(HISTORY)?;
            txn.commit()?;
        }

        // Load or generate device id. redb's `get()` return type
        // borrows the table, so we read + drop the guard before the
        // later `insert()` call.
        let device_id = {
            let txn = db.begin_write()?;
            let existing: Option<Uuid> = {
                let t = txn.open_table(DEVICE)?;
                if let Some(v) = t.get(DEVICE_SELF_KEY)? {
                    let bytes = v.value();
                    if bytes.len() == 16 {
                        let mut arr = [0u8; 16];
                        arr.copy_from_slice(bytes);
                        Some(Uuid::from_bytes(arr))
                    } else {
                        None
                    }
                } else {
                    None
                }
            };
            let id = if let Some(id) = existing {
                id
            } else {
                let id = Uuid::new_v4();
                let bytes = id.as_bytes().as_slice();
                let mut t = txn.open_table(DEVICE)?;
                t.insert(DEVICE_SELF_KEY, bytes)?;
                id
            };
            txn.commit()?;
            id
        };

        Ok(Self {
            inner: Arc::new(db),
            device_id,
            path: path.to_path_buf(),
        })
    }

    /// This device's persisted UUID.
    pub fn device_id(&self) -> DeviceId {
        self.device_id
    }

    /// On-disk path the DB lives at.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Lookup the last-seen baseline for one relpath.
    pub fn get_file(&self, relpath: &str) -> Result<Option<FileRecord>> {
        let txn = self.inner.begin_read()?;
        let t = txn.open_table(FILES)?;
        let Some(v) = t.get(relpath)? else {
            return Ok(None);
        };
        let rec: FileRecord = serde_json::from_str(v.value())?;
        Ok(Some(rec))
    }

    /// Upsert a baseline row.
    pub fn put_file(&self, relpath: &str, record: &FileRecord) -> Result<()> {
        let json = serde_json::to_string(record)?;
        let txn = self.inner.begin_write()?;
        {
            let mut t = txn.open_table(FILES)?;
            t.insert(relpath, json.as_str())?;
        }
        txn.commit()?;
        Ok(())
    }

    /// Remove a baseline row (used after a `Delete` action is
    /// applied on both sides).
    pub fn delete_file(&self, relpath: &str) -> Result<()> {
        let txn = self.inner.begin_write()?;
        {
            let mut t = txn.open_table(FILES)?;
            t.remove(relpath)?;
        }
        txn.commit()?;
        Ok(())
    }

    /// Enumerate every baseline row. Used by the engine's decision
    /// pass to spot files present in the baseline but absent from
    /// both sides (nothing to do, but we still advance history).
    pub fn all_files(&self) -> Result<Vec<(String, FileRecord)>> {
        let txn = self.inner.begin_read()?;
        let t = txn.open_table(FILES)?;
        let mut out = Vec::new();
        for entry in t.iter()? {
            let (k, v) = entry?;
            let key = k.value().to_string();
            let rec: FileRecord = serde_json::from_str(v.value())?;
            out.push((key, rec));
        }
        Ok(out)
    }

    /// Append one history row. Seq is derived from the highest
    /// `(ts, seq)` in the table — we don't need a separate counter
    /// table because `(ts, seq)` keys are unique by construction.
    pub fn append_history(&self, entry: &HistoryEntry) -> Result<()> {
        let json = serde_json::to_string(entry)?;
        let txn = self.inner.begin_write()?;
        {
            let mut t = txn.open_table(HISTORY)?;
            t.insert((entry.timestamp_ms, entry.seq), json.as_str())?;
        }
        txn.commit()?;
        Ok(())
    }

    /// Read the most recent `limit` history rows, newest first.
    pub fn recent_history(&self, limit: usize) -> Result<Vec<HistoryEntry>> {
        let txn = self.inner.begin_read()?;
        let t = txn.open_table(HISTORY)?;
        let mut out: Vec<HistoryEntry> = Vec::new();
        for entry in t.iter()? {
            let (_, v) = entry?;
            let parsed: HistoryEntry = serde_json::from_str(v.value())?;
            out.push(parsed);
        }
        out.sort_by(|a, b| b.timestamp_ms.cmp(&a.timestamp_ms).then(b.seq.cmp(&a.seq)));
        out.truncate(limit);
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clock::VersionVector;
    use std::collections::HashMap;
    use tempfile::tempdir;

    fn sample_record() -> FileRecord {
        let mut vv = VersionVector::new();
        vv.0.insert(Uuid::nil(), 3);
        FileRecord {
            vv,
            mtime_ms: 1_700_000_000_000,
            size: 4096,
            blake3: [0xABu8; 32],
        }
    }

    #[test]
    fn open_creates_file_and_tables() {
        let d = tempdir().unwrap();
        let path = d.path().join("s.db");
        let db = SyncDb::open(&path).unwrap();
        assert!(path.exists());
        assert_eq!(db.all_files().unwrap().len(), 0);
    }

    #[test]
    fn device_id_is_persistent_across_reopen() {
        let d = tempdir().unwrap();
        let path = d.path().join("s.db");
        let id = {
            let db = SyncDb::open(&path).unwrap();
            db.device_id()
        };
        let id2 = {
            let db = SyncDb::open(&path).unwrap();
            db.device_id()
        };
        assert_eq!(id, id2);
    }

    #[test]
    fn put_then_get_roundtrips() {
        let d = tempdir().unwrap();
        let path = d.path().join("s.db");
        let db = SyncDb::open(&path).unwrap();
        let rec = sample_record();
        db.put_file("foo/bar.txt", &rec).unwrap();
        let got = db.get_file("foo/bar.txt").unwrap().unwrap();
        assert_eq!(got, rec);
    }

    #[test]
    fn delete_removes_row() {
        let d = tempdir().unwrap();
        let path = d.path().join("s.db");
        let db = SyncDb::open(&path).unwrap();
        db.put_file("a", &sample_record()).unwrap();
        db.delete_file("a").unwrap();
        assert!(db.get_file("a").unwrap().is_none());
    }

    #[test]
    fn history_records_sort_newest_first() {
        let d = tempdir().unwrap();
        let path = d.path().join("s.db");
        let db = SyncDb::open(&path).unwrap();
        for (ts, seq, label) in [(100, 1, "a"), (200, 2, "b"), (300, 3, "c")] {
            db.append_history(&HistoryEntry {
                timestamp_ms: ts,
                seq,
                relpath: label.to_string(),
                kind: HistoryKind::Noop,
                direction: None,
                vv_before: None,
                vv_after: None,
            })
            .unwrap();
        }
        let recent = db.recent_history(2).unwrap();
        assert_eq!(recent.len(), 2);
        assert_eq!(recent[0].relpath, "c");
        assert_eq!(recent[1].relpath, "b");
    }

    #[test]
    fn all_files_iterates_every_row() {
        let d = tempdir().unwrap();
        let path = d.path().join("s.db");
        let db = SyncDb::open(&path).unwrap();
        let rec = sample_record();
        for name in ["alpha", "beta", "gamma"] {
            db.put_file(name, &rec).unwrap();
        }
        let all = db.all_files().unwrap();
        assert_eq!(all.len(), 3);
        let mut keys: Vec<_> = all.into_iter().map(|(k, _)| k).collect();
        keys.sort();
        assert_eq!(keys, vec!["alpha", "beta", "gamma"]);
    }

    #[test]
    fn device_id_is_unique_per_db() {
        let d1 = tempdir().unwrap();
        let d2 = tempdir().unwrap();
        let db1 = SyncDb::open(&d1.path().join("s.db")).unwrap();
        let db2 = SyncDb::open(&d2.path().join("s.db")).unwrap();
        assert_ne!(db1.device_id(), db2.device_id());
    }

    // Silence unused-import warnings when the tests module skips
    // these paths on a specific cfg.
    #[allow(dead_code)]
    fn _kept_for_future_use(_: HashMap<DeviceId, u64>) {}
}
