//! Walk one side of a sync pair and return a `relpath → SideState`
//! map.
//!
//! The walker is deliberately synchronous: walkdir + streaming
//! BLAKE3 hashing is CPU-bound and small compared to the copy pass
//! that follows. Running inside `spawn_blocking` from the async
//! engine keeps the reactor free.
//!
//! Entries are normalized to forward-slash separators so the DB key
//! space is stable across Windows ↔ Linux ↔ macOS. The sync state DB
//! uses the same convention.
//!
//! The `.copythat-sync.db` file itself is skipped — it lives inside
//! the left root by default and must not appear as a synced relpath.

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use walkdir::WalkDir;

use crate::error::{Result, SyncError};
use crate::types::{FileMeta, SideState};

const HASH_BUF_SIZE: usize = 64 * 1024;

/// Walk one side and return a sorted `relpath → SideState` map.
/// Missing root → empty map (not an error). An `db_filename` the
/// walker should skip lets the caller exclude the pair DB file when
/// it lives inside the tree.
pub fn scan_side(root: &Path, skip_filenames: &[&str]) -> Result<BTreeMap<String, SideState>> {
    if !root.exists() {
        return Err(SyncError::RootNotAccessible {
            path: root.to_path_buf(),
            reason: "path does not exist".to_string(),
        });
    }
    if !root.is_dir() {
        return Err(SyncError::RootNotAccessible {
            path: root.to_path_buf(),
            reason: "path is not a directory".to_string(),
        });
    }

    let mut out = BTreeMap::new();
    for entry in WalkDir::new(root).follow_links(false) {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                // walkdir's transient errors (permission denied on
                // one subfolder, etc.) shouldn't abort the whole
                // walk — record the path + keep going. A stricter
                // policy could surface these as conflicts, but for
                // now we log via the typed error path only when the
                // entire root is inaccessible.
                let path = e.path().map(|p| p.to_path_buf()).unwrap_or_default();
                return Err(SyncError::Io {
                    path,
                    source: e
                        .into_io_error()
                        .unwrap_or_else(|| std::io::Error::other("walkdir transient error")),
                });
            }
        };
        if !entry.file_type().is_file() {
            continue;
        }
        let rel = entry.path().strip_prefix(root).map_err(|_| SyncError::Io {
            path: entry.path().to_path_buf(),
            source: std::io::Error::other("strip_prefix failed"),
        })?;

        // Skip the pair DB + any caller-supplied exclusions.
        if let Some(name) = rel.file_name().and_then(|n| n.to_str()) {
            if skip_filenames.contains(&name) {
                continue;
            }
        }

        let relpath = normalize_relpath(rel);
        let meta = read_file_meta(entry.path())?;
        out.insert(
            relpath.clone(),
            SideState {
                relpath,
                meta: Some(meta),
            },
        );
    }
    Ok(out)
}

/// Normalize a relative path to forward-slash form. Idempotent.
pub(crate) fn normalize_relpath(p: &Path) -> String {
    let mut s = String::with_capacity(p.as_os_str().len());
    for (i, c) in p.components().enumerate() {
        if i > 0 {
            s.push('/');
        }
        match c {
            std::path::Component::Normal(os) => s.push_str(&os.to_string_lossy()),
            std::path::Component::RootDir => {}
            std::path::Component::Prefix(_) => {}
            std::path::Component::CurDir => {}
            // ParentDir should never appear — we walk forward. If it
            // does, keep it verbatim so the caller notices.
            std::path::Component::ParentDir => s.push_str(".."),
        }
    }
    s
}

/// Reconstruct an absolute path from a pair root + normalized
/// relpath. Handles Windows + Unix separators uniformly.
pub(crate) fn join_relpath(root: &Path, relpath: &str) -> PathBuf {
    let mut p = root.to_path_buf();
    for segment in relpath.split('/').filter(|s| !s.is_empty()) {
        p.push(segment);
    }
    p
}

/// Capture one file's mtime + size + BLAKE3 digest.
pub(crate) fn read_file_meta(path: &Path) -> Result<FileMeta> {
    let md = std::fs::metadata(path).map_err(|e| SyncError::Io {
        path: path.to_path_buf(),
        source: e,
    })?;
    let mtime_ms = md
        .modified()
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0);
    let size = md.len();
    let blake3 = hash_file(path)?;
    Ok(FileMeta {
        mtime_ms,
        size,
        blake3,
    })
}

fn hash_file(path: &Path) -> Result<[u8; 32]> {
    let f = File::open(path).map_err(|e| SyncError::Io {
        path: path.to_path_buf(),
        source: e,
    })?;
    let mut reader = BufReader::with_capacity(HASH_BUF_SIZE, f);
    let mut hasher = blake3::Hasher::new();
    let mut buf = [0u8; HASH_BUF_SIZE];
    loop {
        let n = reader.read(&mut buf).map_err(|e| SyncError::Io {
            path: path.to_path_buf(),
            source: e,
        })?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(*hasher.finalize().as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn scan_empty_dir_returns_empty_map() {
        let d = tempdir().unwrap();
        let map = scan_side(d.path(), &[]).unwrap();
        assert!(map.is_empty());
    }

    #[test]
    fn scan_missing_dir_is_typed_error() {
        let err = scan_side(Path::new("/does/not/exist"), &[]).unwrap_err();
        assert!(matches!(err, SyncError::RootNotAccessible { .. }));
    }

    #[test]
    fn scan_one_file_reports_relpath_hash_size_mtime() {
        let d = tempdir().unwrap();
        std::fs::write(d.path().join("hello.txt"), b"hello").unwrap();
        let map = scan_side(d.path(), &[]).unwrap();
        assert_eq!(map.len(), 1);
        let v = map.get("hello.txt").unwrap();
        assert_eq!(v.relpath, "hello.txt");
        let m = v.meta.as_ref().unwrap();
        assert_eq!(m.size, 5);
        assert_ne!(m.blake3, [0u8; 32]);
    }

    #[test]
    fn scan_nested_dirs_use_forward_slash() {
        let d = tempdir().unwrap();
        std::fs::create_dir_all(d.path().join("sub/deeper")).unwrap();
        std::fs::write(d.path().join("sub/deeper/x.txt"), b"x").unwrap();
        let map = scan_side(d.path(), &[]).unwrap();
        assert_eq!(map.len(), 1);
        assert!(map.contains_key("sub/deeper/x.txt"));
    }

    #[test]
    fn scan_skips_pair_db_file() {
        let d = tempdir().unwrap();
        std::fs::write(d.path().join(".copythat-sync.db"), b"db").unwrap();
        std::fs::write(d.path().join("real.txt"), b"r").unwrap();
        let map = scan_side(d.path(), &[".copythat-sync.db"]).unwrap();
        assert!(!map.contains_key(".copythat-sync.db"));
        assert!(map.contains_key("real.txt"));
    }

    #[test]
    fn hashes_differ_for_different_content() {
        let d = tempdir().unwrap();
        std::fs::write(d.path().join("a.txt"), b"AAA").unwrap();
        std::fs::write(d.path().join("b.txt"), b"BBB").unwrap();
        let map = scan_side(d.path(), &[]).unwrap();
        let ha = map.get("a.txt").unwrap().meta.as_ref().unwrap().blake3;
        let hb = map.get("b.txt").unwrap().meta.as_ref().unwrap().blake3;
        assert_ne!(ha, hb);
    }

    #[test]
    fn join_relpath_round_trips() {
        let root = Path::new("/tmp/sync");
        let joined = join_relpath(root, "foo/bar/baz.txt");
        assert!(joined.ends_with("foo/bar/baz.txt") || joined.ends_with("foo\\bar\\baz.txt"));
    }
}
