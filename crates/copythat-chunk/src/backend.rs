//! Phase 50g — pluggable blob storage for pack files.
//!
//! `ChunkStore` routes its pack READS (`get` / `get_range`), plus `list`,
//! `delete`, and `size`, through a [`BlobBackend`], so those paths are
//! storage-agnostic. The trait is object-store-shaped (whole-object put/get,
//! NO append), and a [`BlobBackend::get_range`] fast path lets seekable
//! backends avoid reading a whole pack per chunk (the default reads the whole
//! object and slices).
//!
//! The only backend that ships is [`LocalFsBackend`], whose root **is** the
//! store's `packs/` directory — so the pack WRITES that the store still
//! performs directly on the local filesystem (the active pack is appended in
//! place) land in exactly the files the backend reads back. A genuinely remote
//! backend (S3 / B2 / WebDAV / SFTP) would additionally need a write path that
//! buffers the active pack locally and flushes it via [`BlobBackend::put`] on
//! seal; that seal-and-upload path is **not yet implemented**, and there is no
//! public API to construct a `ChunkStore` over a non-local backend — so the
//! forward-looking read scaffolding here cannot be reached with a
//! not-yet-complete remote write path.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;

use crate::error::{ChunkStoreError, Result};

/// Object-store-shaped storage for named pack blobs.
pub trait BlobBackend: Send + Sync {
    /// Fetch the whole object `name`, or `None` if absent.
    fn get(&self, name: &str) -> Result<Option<Vec<u8>>>;

    /// Fetch `[offset, offset+len)` of object `name`. Default: whole `get`
    /// then slice; seekable backends override for a cheap range read.
    fn get_range(&self, name: &str, offset: u64, len: usize) -> Result<Option<Vec<u8>>> {
        let Some(bytes) = self.get(name)? else {
            return Ok(None);
        };
        let start = (offset as usize).min(bytes.len());
        let end = start.saturating_add(len).min(bytes.len());
        Ok(Some(bytes[start..end].to_vec()))
    }

    /// Store `bytes` under `name` (overwrites any existing object).
    fn put(&self, name: &str, bytes: &[u8]) -> Result<()>;

    /// Delete `name`; returns the bytes freed (0 if absent — idempotent).
    fn delete(&self, name: &str) -> Result<u64>;

    /// Every object name beginning with `prefix`.
    fn list(&self, prefix: &str) -> Result<Vec<String>>;

    /// Whether `name` exists.
    fn exists(&self, name: &str) -> Result<bool>;

    /// Byte size of object `name`, or `None` if absent. Default reads the
    /// whole object; local / HEAD-capable backends override.
    fn size(&self, name: &str) -> Result<Option<u64>> {
        Ok(self.get(name)?.map(|b| b.len() as u64))
    }
}

/// Local-filesystem backend — the default; byte-for-byte the pre-50g
/// behaviour (objects are files under `root`).
#[derive(Debug)]
pub struct LocalFsBackend {
    root: PathBuf,
}

impl LocalFsBackend {
    /// Store objects as files under `root` (created on first write).
    #[must_use]
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    fn path(&self, name: &str) -> PathBuf {
        self.root.join(name)
    }
}

impl BlobBackend for LocalFsBackend {
    fn get(&self, name: &str) -> Result<Option<Vec<u8>>> {
        let path = self.path(name);
        match std::fs::read(&path) {
            Ok(b) => Ok(Some(b)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(ChunkStoreError::Io { path, source: e }),
        }
    }

    fn get_range(&self, name: &str, offset: u64, len: usize) -> Result<Option<Vec<u8>>> {
        use std::io::{Read, Seek, SeekFrom};
        let path = self.path(name);
        let mut f = match std::fs::File::open(&path) {
            Ok(f) => f,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(ChunkStoreError::Io { path, source: e }),
        };
        f.seek(SeekFrom::Start(offset))
            .map_err(|e| ChunkStoreError::Io {
                path: path.clone(),
                source: e,
            })?;
        // Clamp to the object's actual length instead of `read_exact`, so an
        // out-of-range read on a short/truncated pack returns the AVAILABLE
        // bytes — matching the trait's default `get_range` contract that
        // `MemBackend` follows. Callers length-check the result and surface a
        // clean `CorruptChunk`, rather than this backend diverging with a raw
        // `Io` error the restore/mount paths don't expect.
        let mut buf = vec![0u8; len];
        let mut filled = 0usize;
        while filled < len {
            match f.read(&mut buf[filled..]) {
                Ok(0) => break,
                Ok(n) => filled += n,
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {}
                Err(e) => return Err(ChunkStoreError::Io { path, source: e }),
            }
        }
        buf.truncate(filled);
        Ok(Some(buf))
    }

    fn put(&self, name: &str, bytes: &[u8]) -> Result<()> {
        if let Some(parent) = self.path(name).parent() {
            std::fs::create_dir_all(parent).map_err(|e| ChunkStoreError::Io {
                path: parent.to_path_buf(),
                source: e,
            })?;
        }
        let path = self.path(name);
        std::fs::write(&path, bytes).map_err(|e| ChunkStoreError::Io { path, source: e })
    }

    fn delete(&self, name: &str) -> Result<u64> {
        let path = self.path(name);
        let freed = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
        match std::fs::remove_file(&path) {
            Ok(()) => Ok(freed),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(0),
            Err(e) => Err(ChunkStoreError::Io { path, source: e }),
        }
    }

    fn list(&self, prefix: &str) -> Result<Vec<String>> {
        let mut out = Vec::new();
        let rd = match std::fs::read_dir(&self.root) {
            Ok(rd) => rd,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(out),
            Err(e) => {
                return Err(ChunkStoreError::Io {
                    path: self.root.clone(),
                    source: e,
                });
            }
        };
        for ent in rd {
            let ent = ent.map_err(|e| ChunkStoreError::Io {
                path: self.root.clone(),
                source: e,
            })?;
            if let Some(name) = ent.file_name().to_str() {
                if name.starts_with(prefix) {
                    out.push(name.to_string());
                }
            }
        }
        Ok(out)
    }

    fn exists(&self, name: &str) -> Result<bool> {
        Ok(self.path(name).exists())
    }

    fn size(&self, name: &str) -> Result<Option<u64>> {
        let path = self.path(name);
        match std::fs::metadata(&path) {
            Ok(m) => Ok(Some(m.len())),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(ChunkStoreError::Io { path, source: e }),
        }
    }
}

/// In-memory backend — an ephemeral test double / non-FS backend that drives
/// the same `ChunkStore` code path a real object store would.
#[derive(Debug, Default)]
pub struct MemBackend {
    blobs: Mutex<HashMap<String, Vec<u8>>>,
}

impl MemBackend {
    fn lock(&self) -> std::sync::MutexGuard<'_, HashMap<String, Vec<u8>>> {
        self.blobs.lock().unwrap_or_else(|e| e.into_inner())
    }
}

impl BlobBackend for MemBackend {
    fn get(&self, name: &str) -> Result<Option<Vec<u8>>> {
        Ok(self.lock().get(name).cloned())
    }

    fn put(&self, name: &str, bytes: &[u8]) -> Result<()> {
        self.lock().insert(name.to_string(), bytes.to_vec());
        Ok(())
    }

    fn delete(&self, name: &str) -> Result<u64> {
        Ok(self
            .lock()
            .remove(name)
            .map(|v| v.len() as u64)
            .unwrap_or(0))
    }

    fn list(&self, prefix: &str) -> Result<Vec<String>> {
        Ok(self
            .lock()
            .keys()
            .filter(|k| k.starts_with(prefix))
            .cloned()
            .collect())
    }

    fn exists(&self, name: &str) -> Result<bool> {
        Ok(self.lock().contains_key(name))
    }

    fn size(&self, name: &str) -> Result<Option<u64>> {
        Ok(self.lock().get(name).map(|v| v.len() as u64))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roundtrip(be: &dyn BlobBackend) {
        assert!(!be.exists("a.pack").unwrap());
        assert_eq!(be.get("a.pack").unwrap(), None);
        be.put("a.pack", b"hello world").unwrap();
        assert!(be.exists("a.pack").unwrap());
        assert_eq!(
            be.get("a.pack").unwrap().as_deref(),
            Some(&b"hello world"[..])
        );
        // range read (default or seek fast path).
        assert_eq!(
            be.get_range("a.pack", 6, 5).unwrap().as_deref(),
            Some(&b"world"[..])
        );
        be.put("b.pack", b"xx").unwrap();
        let mut names = be.list("").unwrap();
        names.sort();
        assert_eq!(names, vec!["a.pack".to_string(), "b.pack".to_string()]);
        assert_eq!(be.delete("a.pack").unwrap(), 11);
        assert_eq!(be.delete("a.pack").unwrap(), 0); // idempotent
        assert!(!be.exists("a.pack").unwrap());
    }

    #[test]
    fn local_fs_backend_roundtrips() {
        let tmp = tempfile::tempdir().unwrap();
        roundtrip(&LocalFsBackend::new(tmp.path().to_path_buf()));
    }

    #[test]
    fn mem_backend_roundtrips() {
        roundtrip(&MemBackend::default());
    }
}
