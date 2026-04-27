//! Phase 19a — on-disk schema for the per-scan SQLite database.
//!
//! Migrations use `PRAGMA user_version`, mirroring
//! `copythat-history::migrations`. A scan DB is self-contained:
//! `scan_meta` holds root path / totals / status, `scan_items` holds
//! one row per observed entry (indexed on `rel_path` for the
//! cursor-order iteration that the copy engine consumes and on `size`
//! for the "biggest files first" sort the UI surfaces), and
//! `scan_progress` persists the last-committed point so a crash mid-
//! scan loses at most one in-flight batch.

use rusqlite::Connection;
use thiserror::Error;

/// Current on-disk schema version. Bump when appending to the private
/// `MIGRATIONS` table in this module.
pub const CURRENT_SCHEMA_VERSION: u32 = 1;

const MIGRATIONS: &[(u32, u32, &str)] = &[(0, 1, V0_TO_V1)];

/// Per-scan bootstrap:
///
/// - `scan_meta` — key/value store for the root path, timestamps,
///   running totals, hashing mode, and the scan status
///   (`Running` / `Paused` / `Complete` / `Cancelled` / `Failed`).
/// - `scan_items` — one row per observed entry. `rel_path` is the
///   sort key cursor iteration relies on. `content_hash` is a
///   32-byte BLOB populated when hashing is enabled, NULL otherwise.
/// - `scan_progress` — a single-row resume marker. Updated after
///   every batch flush so a kill-9 mid-scan loses ≤ one batch.
const V0_TO_V1: &str = r#"
CREATE TABLE IF NOT EXISTS scan_meta (
    key   TEXT PRIMARY KEY,
    value TEXT
);

CREATE TABLE IF NOT EXISTS scan_items (
    rowid        INTEGER PRIMARY KEY,
    rel_path     TEXT    NOT NULL,
    size         INTEGER NOT NULL,
    mtime        INTEGER NOT NULL,
    kind         INTEGER NOT NULL,
    attrs        INTEGER NOT NULL,
    content_hash BLOB,
    scanned_at   INTEGER NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_scan_items_path ON scan_items(rel_path);
CREATE INDEX IF NOT EXISTS idx_scan_items_size ON scan_items(size);

CREATE TABLE IF NOT EXISTS scan_progress (
    id                 INTEGER PRIMARY KEY CHECK (id = 1),
    last_visited_path  TEXT,
    files_visited      INTEGER NOT NULL DEFAULT 0,
    bytes_visited      INTEGER NOT NULL DEFAULT 0
);

INSERT OR IGNORE INTO scan_progress (id, last_visited_path, files_visited, bytes_visited)
VALUES (1, NULL, 0, 0);
"#;

/// Errors raised by the scan-DB schema migrator.
#[derive(Debug, Error)]
pub enum SchemaError {
    /// Underlying SQLite failure.
    #[error(transparent)]
    Sqlite(#[from] rusqlite::Error),
    /// The scan DB carries a newer schema than this build understands.
    /// Tuple is `(db_version, build_version)`.
    #[error("scan schema newer than this build (db={0}, build={1})")]
    Unsupported(u32, u32),
}

/// Apply every migration whose `from` matches the current user_version.
/// Returns cleanly when the DB already matches
/// [`CURRENT_SCHEMA_VERSION`].
pub fn apply_pending(conn: &mut Connection) -> Result<(), SchemaError> {
    let mut current = read_version(conn)?;
    if current == CURRENT_SCHEMA_VERSION {
        return Ok(());
    }
    if current > CURRENT_SCHEMA_VERSION {
        return Err(SchemaError::Unsupported(current, CURRENT_SCHEMA_VERSION));
    }

    for (from, to, sql) in MIGRATIONS {
        if *from != current {
            continue;
        }
        let tx = conn.transaction()?;
        tx.execute_batch(sql)?;
        tx.pragma_update(None, "user_version", *to)?;
        tx.commit()?;
        current = *to;
        if current == CURRENT_SCHEMA_VERSION {
            return Ok(());
        }
    }

    if current != CURRENT_SCHEMA_VERSION {
        return Err(SchemaError::Unsupported(current, CURRENT_SCHEMA_VERSION));
    }
    Ok(())
}

pub(crate) fn read_version(conn: &Connection) -> Result<u32, SchemaError> {
    let mut v: u32 = 0;
    conn.pragma_query(None, "user_version", |row| {
        v = row.get::<_, i64>(0)? as u32;
        Ok(())
    })?;
    Ok(v)
}

/// Shared PRAGMAs applied every time a scan DB is opened — not just
/// at first-run migrations. WAL lets a cursor reader stream rows
/// while the writer is still committing batches; `synchronous=NORMAL`
/// gives the batched insert loop enough durability (one batch may be
/// lost on power-cut) without the 2× slowdown of `FULL`; the page
/// cache is bounded so a 5 M-file scan doesn't balloon RSS via
/// SQLite's own cache alone.
pub(crate) fn apply_runtime_pragmas(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.pragma_update(None, "journal_mode", "WAL")?;
    conn.pragma_update(None, "synchronous", "NORMAL")?;
    conn.pragma_update(None, "temp_store", "MEMORY")?;
    // ~8 MiB page cache keeps the working set tight even at 5 M rows.
    conn.pragma_update(None, "cache_size", -8192i64)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fresh_db_lands_on_current_version() {
        let mut conn = Connection::open_in_memory().unwrap();
        apply_pending(&mut conn).unwrap();
        assert_eq!(read_version(&conn).unwrap(), CURRENT_SCHEMA_VERSION);
    }

    #[test]
    fn apply_twice_is_a_noop() {
        let mut conn = Connection::open_in_memory().unwrap();
        apply_pending(&mut conn).unwrap();
        apply_pending(&mut conn).unwrap();
        assert_eq!(read_version(&conn).unwrap(), CURRENT_SCHEMA_VERSION);
    }

    #[test]
    fn tables_and_indexes_exist() {
        let mut conn = Connection::open_in_memory().unwrap();
        apply_pending(&mut conn).unwrap();
        for name in ["scan_meta", "scan_items", "scan_progress"] {
            let n: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?1",
                    [name],
                    |row| row.get(0),
                )
                .unwrap();
            assert_eq!(n, 1, "table {name} missing");
        }
        for name in ["idx_scan_items_path", "idx_scan_items_size"] {
            let n: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM sqlite_master WHERE type='index' AND name=?1",
                    [name],
                    |row| row.get(0),
                )
                .unwrap();
            assert_eq!(n, 1, "index {name} missing");
        }
    }

    #[test]
    fn future_schema_errors() {
        let mut conn = Connection::open_in_memory().unwrap();
        conn.pragma_update(None, "user_version", 999i64).unwrap();
        let err = apply_pending(&mut conn).unwrap_err();
        matches!(err, SchemaError::Unsupported(999, 1));
    }

    #[test]
    fn scan_progress_seed_row_is_present() {
        let mut conn = Connection::open_in_memory().unwrap();
        apply_pending(&mut conn).unwrap();
        let n: i64 = conn
            .query_row("SELECT COUNT(*) FROM scan_progress WHERE id=1", [], |r| {
                r.get(0)
            })
            .unwrap();
        assert_eq!(n, 1);
    }
}
