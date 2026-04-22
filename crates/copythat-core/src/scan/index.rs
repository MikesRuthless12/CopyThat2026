//! Phase 19a — top-level `main.db` index over every per-scan DB.
//!
//! On TeraCopy the equivalent is a single `main.db` that records
//! every scan the app has ever run, so the UI can sweep for pending
//! resumes on launch and present a resume prompt.
//!
//! We persist the same shape here: one row per `scan_id`, carrying
//! the absolute path of the per-scan DB, the optional linked job id,
//! creation timestamp, and the current lifecycle status.

use std::path::{Path, PathBuf};

use rusqlite::{Connection, params};

use crate::scan::schema::{self, SchemaError};
use crate::scan::types::{ScanId, ScanStatus};

/// On-disk path for the `main.db` index that sits alongside the
/// per-scan databases.
pub fn default_index_path() -> Option<PathBuf> {
    directories::ProjectDirs::from("com", "CopyThat", "CopyThat2026")
        .map(|p| p.config_dir().join("scans").join("main.db"))
}

/// Open (creating if needed) the `main.db` index at `path`. Applies
/// the `active_scans` migration and enables WAL mode so a resumer
/// reading the index doesn't stall a concurrent scan that wants to
/// update its own row.
pub fn open_index(path: &Path) -> Result<Connection, SchemaError> {
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent).map_err(|e| {
            SchemaError::Sqlite(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_CANTOPEN),
                Some(format!("cannot create {parent:?}: {e}")),
            ))
        })?;
    }
    let mut conn = Connection::open(path)?;
    apply_index_migrations(&mut conn)?;
    schema::apply_runtime_pragmas(&conn)?;
    Ok(conn)
}

const INDEX_MIGRATIONS: &[(u32, u32, &str)] = &[(0, 1, INDEX_V0_TO_V1)];

const INDEX_V0_TO_V1: &str = r#"
CREATE TABLE IF NOT EXISTS active_scans (
    scan_id    TEXT PRIMARY KEY,
    db_path    TEXT NOT NULL,
    job_id     TEXT,
    created_at INTEGER NOT NULL,
    status     TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_active_scans_status ON active_scans(status);
CREATE INDEX IF NOT EXISTS idx_active_scans_created ON active_scans(created_at);
"#;

fn apply_index_migrations(conn: &mut Connection) -> Result<(), SchemaError> {
    let mut current = schema::read_version(conn)?;
    if current >= 1 {
        return Ok(());
    }
    for (from, to, sql) in INDEX_MIGRATIONS {
        if *from != current {
            continue;
        }
        let tx = conn.transaction()?;
        tx.execute_batch(sql)?;
        tx.pragma_update(None, "user_version", *to)?;
        tx.commit()?;
        current = *to;
    }
    Ok(())
}

/// One row of `active_scans`. Surfaced by
/// [`list_unfinished`] so the app can offer to resume.
#[derive(Debug, Clone)]
pub struct ActiveScanRow {
    pub scan_id: ScanId,
    pub db_path: PathBuf,
    pub job_id: Option<String>,
    pub created_at_ms: i64,
    pub status: ScanStatus,
}

pub fn register(
    conn: &Connection,
    scan_id: ScanId,
    db_path: &Path,
    job_id: Option<&str>,
    created_at_ms: i64,
    status: ScanStatus,
) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT OR REPLACE INTO active_scans (scan_id, db_path, job_id, created_at, status)
             VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            scan_id.as_str(),
            db_path.to_string_lossy(),
            job_id,
            created_at_ms,
            status.as_str(),
        ],
    )?;
    Ok(())
}

pub fn update_status(
    conn: &Connection,
    scan_id: ScanId,
    status: ScanStatus,
) -> Result<(), rusqlite::Error> {
    conn.execute(
        "UPDATE active_scans SET status=?1 WHERE scan_id=?2",
        params![status.as_str(), scan_id.as_str()],
    )?;
    Ok(())
}

pub fn remove(conn: &Connection, scan_id: ScanId) -> Result<(), rusqlite::Error> {
    conn.execute(
        "DELETE FROM active_scans WHERE scan_id=?1",
        params![scan_id.as_str()],
    )?;
    Ok(())
}

/// Scans that are `Running` or `Paused` — i.e. candidates the UI
/// should offer to resume on launch. Ordered newest-first.
pub fn list_unfinished(conn: &Connection) -> Result<Vec<ActiveScanRow>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT scan_id, db_path, job_id, created_at, status
           FROM active_scans
          WHERE status IN ('Running','Paused')
       ORDER BY created_at DESC",
    )?;
    let rows = stmt.query_map([], row_to_active)?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}

/// Every row, regardless of status. Used by the Settings "Scan
/// database" pane listing.
pub fn list_all(conn: &Connection) -> Result<Vec<ActiveScanRow>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT scan_id, db_path, job_id, created_at, status
           FROM active_scans
       ORDER BY created_at DESC",
    )?;
    let rows = stmt.query_map([], row_to_active)?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}

fn row_to_active(row: &rusqlite::Row<'_>) -> rusqlite::Result<ActiveScanRow> {
    let scan_id_str: String = row.get(0)?;
    let db_path: String = row.get(1)?;
    let job_id: Option<String> = row.get(2)?;
    let created_at_ms: i64 = row.get(3)?;
    let status_str: String = row.get(4)?;
    let scan_id = ScanId::parse(&scan_id_str).map_err(|e| {
        rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e))
    })?;
    let status = ScanStatus::parse(&status_str).unwrap_or(ScanStatus::Failed);
    Ok(ActiveScanRow {
        scan_id,
        db_path: PathBuf::from(db_path),
        job_id,
        created_at_ms,
        status,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn fresh_index() -> (TempDir, Connection) {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("main.db");
        let conn = open_index(&path).unwrap();
        (tmp, conn)
    }

    #[test]
    fn register_then_list_unfinished_round_trip() {
        let (_tmp, conn) = fresh_index();
        let id = ScanId::new();
        register(
            &conn,
            id,
            Path::new("/tmp/scan-foo.db"),
            Some("job-1"),
            1_000,
            ScanStatus::Running,
        )
        .unwrap();
        let rows = list_unfinished(&conn).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].scan_id, id);
        assert_eq!(rows[0].status, ScanStatus::Running);
    }

    #[test]
    fn status_update_moves_rows_between_lists() {
        let (_tmp, conn) = fresh_index();
        let id = ScanId::new();
        register(
            &conn,
            id,
            Path::new("/tmp/s.db"),
            None,
            10,
            ScanStatus::Running,
        )
        .unwrap();
        assert_eq!(list_unfinished(&conn).unwrap().len(), 1);
        update_status(&conn, id, ScanStatus::Complete).unwrap();
        assert_eq!(list_unfinished(&conn).unwrap().len(), 0);
        assert_eq!(list_all(&conn).unwrap().len(), 1);
    }

    #[test]
    fn remove_drops_the_row() {
        let (_tmp, conn) = fresh_index();
        let id = ScanId::new();
        register(
            &conn,
            id,
            Path::new("/tmp/s.db"),
            None,
            10,
            ScanStatus::Running,
        )
        .unwrap();
        remove(&conn, id).unwrap();
        assert_eq!(list_all(&conn).unwrap().len(), 0);
    }
}
