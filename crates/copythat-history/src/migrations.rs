//! Hand-rolled schema migrator.
//!
//! SQLite exposes `PRAGMA user_version` — a 32-bit slot stored in
//! the database header, zero on a fresh DB — so we track schema
//! revision there and bump it as migrations run. Each migration is
//! a `(from, to, sql)` triple; `apply_pending` walks the list and
//! executes any whose `from` matches the current version, wrapping
//! each transition in a SAVEPOINT so a failure mid-migration rolls
//! back cleanly.
//!
//! Adding a migration: append to [`MIGRATIONS`]. The `from` /`to`
//! ladder must be dense — the migrator bails if it can't step from
//! current to target.

use rusqlite::Connection;

use crate::error::HistoryError;

/// Current on-disk schema version. Bump when appending to
/// [`MIGRATIONS`]; `apply_pending` refuses to open a DB whose
/// `user_version` is higher (a downgrade scenario).
pub const CURRENT_SCHEMA_VERSION: u32 = 2;

/// `(from_version, to_version, sql)`. Must form a dense ladder from
/// 0 to [`CURRENT_SCHEMA_VERSION`].
const MIGRATIONS: &[(u32, u32, &str)] = &[(0, 1, V0_TO_V1), (1, 2, V1_TO_V2)];

/// Schema bootstrap. `jobs` carries lifecycle + totals; `items`
/// carries per-file rows linked to a job. Two indexes keep the UI
/// queries fast: `idx_jobs_started` for "recent first" ordering,
/// `idx_items_job` for the `items_for(job)` detail view.
///
/// Timestamps are stored as milliseconds-since-epoch `INTEGER`,
/// not ISO-8601 TEXT — cheaper to compare and range-scan in SQLite.
///
/// `ON DELETE CASCADE` on the items FK keeps `purge_older_than`
/// simple: deleting a jobs row sweeps the attached items in the
/// same statement.
const V0_TO_V1: &str = r#"
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS jobs (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    kind            TEXT    NOT NULL,
    status          TEXT    NOT NULL,
    started_at_ms   INTEGER NOT NULL,
    finished_at_ms  INTEGER,
    src_root        TEXT    NOT NULL,
    dst_root        TEXT    NOT NULL,
    total_bytes     INTEGER NOT NULL DEFAULT 0,
    files_ok        INTEGER NOT NULL DEFAULT 0,
    files_failed    INTEGER NOT NULL DEFAULT 0,
    verify_algo     TEXT,
    options_json    TEXT
);

CREATE TABLE IF NOT EXISTS items (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    job_id          INTEGER NOT NULL REFERENCES jobs(id) ON DELETE CASCADE,
    src             TEXT    NOT NULL,
    dst             TEXT    NOT NULL,
    size            INTEGER NOT NULL DEFAULT 0,
    status          TEXT    NOT NULL,
    hash_hex        TEXT,
    error_code      TEXT,
    error_msg       TEXT,
    timestamp_ms    INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_jobs_started ON jobs(started_at_ms);
CREATE INDEX IF NOT EXISTS idx_items_job    ON items(job_id);
"#;

/// Phase 42 Part B — per-file rolling versions. The `versions` table
/// records one row each time the engine snapshots a destination file
/// before overwriting it. `manifest_blake3` is the BLAKE3 of the
/// chunk-store manifest the snapshot ingested into; the chunks
/// themselves live in the Phase 27 chunk store keyed by content hash.
///
/// `dst_path` is the destination path the version was captured for.
/// `ts` is milliseconds-since-epoch at snapshot time. `size` is the
/// size of the snapshot in bytes (= the size of the file at the
/// moment of capture). `retained_until` is an optional deadline (also
/// milliseconds-since-epoch) the retention pruner uses when the
/// active policy includes a hard floor; `NULL` = "no floor". Every
/// row optionally references the `jobs` row that *triggered* the
/// snapshot — i.e., the copy job whose overwrite caused us to keep
/// the prior content. The FK is `ON DELETE SET NULL` rather than
/// `CASCADE`: removing the triggering job from history shouldn't
/// silently delete the user's version snapshots; the versions remain
/// but lose the back-reference.
///
/// Two indexes: `idx_versions_path` for the per-file "list versions
/// of `<file>`" lookup, `idx_versions_ts` for the global "newest
/// versions across all files" pull.
const V1_TO_V2: &str = r#"
PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS versions (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    dst_path            TEXT    NOT NULL,
    ts                  INTEGER NOT NULL,
    manifest_blake3     BLOB    NOT NULL,
    size                INTEGER NOT NULL DEFAULT 0,
    retained_until      INTEGER,
    triggered_by_job_id INTEGER REFERENCES jobs(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_versions_path ON versions(dst_path);
CREATE INDEX IF NOT EXISTS idx_versions_ts   ON versions(ts);
"#;

/// Run every migration whose `from` matches the current user_version.
/// Exits cleanly (Ok) when the DB already matches
/// [`CURRENT_SCHEMA_VERSION`].
pub fn apply_pending(conn: &mut Connection) -> Result<(), HistoryError> {
    // Foreign keys are per-connection; enable before anything else.
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;

    let mut current = read_version(conn)?;
    if current == CURRENT_SCHEMA_VERSION {
        return Ok(());
    }
    if current > CURRENT_SCHEMA_VERSION {
        return Err(HistoryError::UnsupportedSchema(
            current,
            CURRENT_SCHEMA_VERSION,
        ));
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
        // Ladder was not dense, or a migration failed to bump
        // user_version. Fail loud so we don't silently ship a half-
        // migrated schema.
        return Err(HistoryError::UnsupportedSchema(
            current,
            CURRENT_SCHEMA_VERSION,
        ));
    }
    Ok(())
}

fn read_version(conn: &Connection) -> Result<u32, HistoryError> {
    let mut v: u32 = 0;
    conn.pragma_query(None, "user_version", |row| {
        v = row.get::<_, i64>(0)? as u32;
        Ok(())
    })?;
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fresh_db_lands_on_current_version() {
        let mut conn = Connection::open_in_memory().unwrap();
        apply_pending(&mut conn).unwrap();
        let v = read_version(&conn).unwrap();
        assert_eq!(v, CURRENT_SCHEMA_VERSION);
    }

    #[test]
    fn running_twice_is_a_noop() {
        let mut conn = Connection::open_in_memory().unwrap();
        apply_pending(&mut conn).unwrap();
        apply_pending(&mut conn).unwrap();
        assert_eq!(read_version(&conn).unwrap(), CURRENT_SCHEMA_VERSION);
    }

    #[test]
    fn tables_and_indexes_exist_post_migration() {
        let mut conn = Connection::open_in_memory().unwrap();
        apply_pending(&mut conn).unwrap();

        for name in ["jobs", "items", "versions"] {
            let n: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?1",
                    [name],
                    |row| row.get(0),
                )
                .unwrap();
            assert_eq!(n, 1, "table {name} missing");
        }
        for name in [
            "idx_jobs_started",
            "idx_items_job",
            "idx_versions_path",
            "idx_versions_ts",
        ] {
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
    fn fresh_db_lands_on_v2_with_versions_columns() {
        let mut conn = Connection::open_in_memory().unwrap();
        apply_pending(&mut conn).unwrap();
        assert_eq!(read_version(&conn).unwrap(), 2);
        // Spot-check the versions table column shape.
        let cols: Vec<String> = conn
            .prepare("SELECT name FROM pragma_table_info('versions')")
            .unwrap()
            .query_map([], |row| row.get::<_, String>(0))
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        for name in [
            "id",
            "dst_path",
            "ts",
            "manifest_blake3",
            "size",
            "retained_until",
            "triggered_by_job_id",
        ] {
            assert!(
                cols.iter().any(|c| c == name),
                "versions column {name} missing — got {cols:?}"
            );
        }
    }

    #[test]
    fn v1_dbs_migrate_forward_to_v2() {
        // Open a fresh DB but force user_version back to 1 to
        // simulate a Phase-39-era v1.0.0 install upgrading.
        let mut conn = Connection::open_in_memory().unwrap();
        // Apply v0→v1 only by running the v0→v1 SQL directly.
        conn.execute_batch(V0_TO_V1).unwrap();
        conn.pragma_update(None, "user_version", 1i64).unwrap();
        assert_eq!(read_version(&conn).unwrap(), 1);

        apply_pending(&mut conn).unwrap();

        assert_eq!(read_version(&conn).unwrap(), 2);
        let n: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='versions'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(n, 1, "v1→v2 migration didn't create versions");
    }

    #[test]
    fn future_schema_errors() {
        let conn = Connection::open_in_memory().unwrap();
        conn.pragma_update(None, "user_version", 999i64).unwrap();
        drop(conn);

        // Re-open with a fresh handle so `apply_pending` reads a
        // different value than the transient one above.
        let mut conn = Connection::open_in_memory().unwrap();
        conn.pragma_update(None, "user_version", 999i64).unwrap();
        let err = apply_pending(&mut conn).unwrap_err();
        match err {
            HistoryError::UnsupportedSchema(got, want) => {
                assert_eq!(got, 999);
                assert_eq!(want, CURRENT_SCHEMA_VERSION);
            }
            other => panic!("expected UnsupportedSchema, got {other:?}"),
        }
    }
}
