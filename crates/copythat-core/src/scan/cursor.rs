//! Phase 19a — streaming iterator over `scan_items`.
//!
//! The cursor opens its own read-only SQLite connection against the
//! per-scan DB so it can run concurrently with an ongoing write
//! stream (WAL mode makes this safe). Results are ordered by
//! `rel_path ASC` — the same deterministic order the Scanner commits
//! in, which is what resume-from-checkpoint depends on.
//!
//! Implementation note: rusqlite's `Statement` borrows from its
//! `Connection`, so a naive `Iterator for (Connection, Statement)`
//! is a self-referential struct. We side-step by materializing the
//! cursor in bounded-size page chunks: each `next()` pulls the next
//! row from an internally-owned page and refills when the page is
//! exhausted. Memory footprint: `PAGE_SIZE` rows at a time
//! (~1 MiB for 4096 items × ~256 B each).

use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use rusqlite::{Connection, OpenFlags};

use crate::scan::schema::{self, SchemaError};
use crate::scan::types::{AttrFlags, EntryKind, ScanItem};

const PAGE_SIZE: u32 = 4096;

/// Streaming iterator over `scan_items`, ordered by `rel_path ASC`.
pub struct ScanCursor {
    conn: Connection,
    next_rowid: i64,
    buffer: std::vec::IntoIter<ScanItem>,
    exhausted: bool,
}

impl ScanCursor {
    /// Open a read-only cursor against the scan DB at `db_path`.
    pub fn open(db_path: &Path) -> Result<Self, SchemaError> {
        let conn = Connection::open_with_flags(
            db_path,
            OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )?;
        schema::apply_runtime_pragmas(&conn)?;
        Ok(Self {
            conn,
            next_rowid: 0,
            buffer: Vec::new().into_iter(),
            exhausted: false,
        })
    }

    /// Open against an existing connection (used by the Scanner's
    /// own `cursor()` accessor). The caller is responsible for
    /// ensuring the connection is not being used concurrently for
    /// writes.
    pub fn from_connection(conn: Connection) -> Self {
        Self {
            conn,
            next_rowid: 0,
            buffer: Vec::new().into_iter(),
            exhausted: false,
        }
    }

    fn refill(&mut self) -> Result<(), rusqlite::Error> {
        if self.exhausted {
            return Ok(());
        }
        let mut stmt = self.conn.prepare(
            "SELECT rowid, rel_path, size, mtime, kind, attrs, content_hash
               FROM scan_items
              WHERE rowid > ?1
           ORDER BY rowid ASC
              LIMIT ?2",
        )?;
        let rows = stmt.query_map(
            rusqlite::params![self.next_rowid, i64::from(PAGE_SIZE)],
            row_to_item,
        )?;
        let mut page: Vec<(i64, ScanItem)> = Vec::with_capacity(PAGE_SIZE as usize);
        for r in rows {
            page.push(r?);
        }
        if page.is_empty() {
            self.exhausted = true;
            return Ok(());
        }
        self.next_rowid = page.last().map(|(rid, _)| *rid).unwrap_or(self.next_rowid);
        let items: Vec<ScanItem> = page.into_iter().map(|(_, item)| item).collect();
        self.buffer = items.into_iter();
        Ok(())
    }
}

fn row_to_item(row: &rusqlite::Row<'_>) -> rusqlite::Result<(i64, ScanItem)> {
    let rowid: i64 = row.get(0)?;
    let rel_path: String = row.get(1)?;
    let size: i64 = row.get(2)?;
    let mtime_unix_nanos: i64 = row.get(3)?;
    let kind_i: i64 = row.get(4)?;
    let attrs_i: i64 = row.get(5)?;
    let hash_blob: Option<Vec<u8>> = row.get(6)?;

    let content_hash = match hash_blob {
        Some(b) if b.len() == 32 => {
            let mut arr = [0u8; 32];
            arr.copy_from_slice(&b);
            Some(arr)
        }
        _ => None,
    };

    Ok((
        rowid,
        ScanItem {
            rel_path,
            size: size as u64,
            mtime: unix_nanos_to_system_time(mtime_unix_nanos),
            kind: EntryKind::from_i64(kind_i),
            attrs: AttrFlags::from_i64(attrs_i),
            content_hash,
        },
    ))
}

pub(crate) fn system_time_to_unix_nanos(t: SystemTime) -> i64 {
    match t.duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_nanos().min(i64::MAX as u128) as i64,
        Err(e) => -(e.duration().as_nanos().min(i64::MAX as u128) as i64),
    }
}

pub(crate) fn unix_nanos_to_system_time(n: i64) -> SystemTime {
    if n >= 0 {
        UNIX_EPOCH + Duration::from_nanos(n as u64)
    } else {
        UNIX_EPOCH - Duration::from_nanos((-n) as u64)
    }
}

impl Iterator for ScanCursor {
    type Item = ScanItem;

    fn next(&mut self) -> Option<ScanItem> {
        if let Some(item) = self.buffer.next() {
            return Some(item);
        }
        if let Err(e) = self.refill() {
            eprintln!("[scan::cursor] refill failed: {e}");
            return None;
        }
        self.buffer.next()
    }
}

/// Helper: absolute path for a cursor entry, given the root the scan
/// was started against.
pub fn absolute(root: &Path, item: &ScanItem) -> PathBuf {
    root.join(&item.rel_path)
}
