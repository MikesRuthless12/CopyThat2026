//! `History` — the crate's single entry point.
//!
//! Wraps a `rusqlite::Connection` behind a Mutex + an Arc so every
//! Tauri command can clone a cheap handle out of `AppState`. All
//! public methods are `async fn` and hop to `spawn_blocking`
//! internally — SQLite is a synchronous library, and we refuse to
//! let a disk stall wedge the tokio runtime.
//!
//! Concurrency model: SQLite enforces a single writer per database,
//! so we serialise writes behind a Mutex rather than taking the cost
//! of a connection pool. The UI workload is bounded (handful of
//! writes per tree, ad-hoc reads when the drawer opens) — a pool
//! is worth it only for server-class traffic, which this never
//! sees.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use directories::ProjectDirs;
use rusqlite::{Connection, params};
use tokio::task;

use crate::error::HistoryError;
use crate::migrations;
use crate::types::{
    DEFAULT_SEARCH_LIMIT, DayTotal, HistoryFilter, ItemRow, JobRowId, JobSummary, KindBreakdown,
    Totals,
};

/// Cheap clonable handle. Every `tauri::command` takes one out of
/// `AppState`; cloning is `Arc::clone`.
#[derive(Clone)]
pub struct History {
    inner: Arc<Inner>,
}

struct Inner {
    /// Path on disk. Kept around for the Tauri layer's "show in
    /// folder" affordance and for `Display` on errors.
    #[allow(dead_code)]
    path: PathBuf,
    /// Single writer-locked connection. We could use a `ReentrantMutex`
    /// to allow recursive reads, but the public surface is flat — a
    /// plain `Mutex` covers it.
    conn: Mutex<Connection>,
}

impl History {
    /// Open (or create) the default on-disk history at the OS user-
    /// data directory. On Windows:
    /// `%LOCALAPPDATA%\CopyThat 2026\history.db`. On macOS:
    /// `~/Library/Application Support/com.copythat.desktop/history.db`.
    /// On Linux: `$XDG_DATA_HOME/copythat-2026/history.db`
    /// (default `~/.local/share/copythat-2026/`).
    pub async fn open_default() -> Result<Self, HistoryError> {
        let path = default_db_path()?;
        Self::open_at(path).await
    }

    /// Open a history at an explicit path. Used by tests + the
    /// Phase 12 Settings "custom database path" knob.
    pub async fn open_at(path: PathBuf) -> Result<Self, HistoryError> {
        let path_for_worker = path.clone();
        let conn = task::spawn_blocking(move || -> Result<Connection, HistoryError> {
            // Ensure the parent directory exists — first run on a
            // fresh user profile may land here.
            if let Some(parent) = path_for_worker.parent()
                && !parent.as_os_str().is_empty()
            {
                std::fs::create_dir_all(parent)?;
            }
            let mut conn = Connection::open(&path_for_worker)?;
            migrations::apply_pending(&mut conn)?;
            Ok(conn)
        })
        .await
        .map_err(|_| HistoryError::WorkerPanicked)??;

        Ok(Self {
            inner: Arc::new(Inner {
                path,
                conn: Mutex::new(conn),
            }),
        })
    }

    /// In-memory database — isolated per call, never persists. Used
    /// by tests and by the Tauri harness when the app is launched
    /// with `--no-history`. Each in-memory DB is independent: two
    /// calls produce two separate stores.
    pub async fn open_in_memory() -> Result<Self, HistoryError> {
        let conn = task::spawn_blocking(|| -> Result<Connection, HistoryError> {
            let mut conn = Connection::open_in_memory()?;
            migrations::apply_pending(&mut conn)?;
            Ok(conn)
        })
        .await
        .map_err(|_| HistoryError::WorkerPanicked)??;

        Ok(Self {
            inner: Arc::new(Inner {
                path: PathBuf::from(":memory:"),
                conn: Mutex::new(conn),
            }),
        })
    }

    /// On-disk location. Returns `":memory:"` for an in-memory DB.
    pub fn db_path(&self) -> &Path {
        &self.inner.path
    }

    // ---------- writes ----------

    /// Insert a new `jobs` row in the `"running"` status and return
    /// the assigned primary key. Later calls (`record_item`,
    /// `record_finish`) use this id.
    pub async fn record_start(&self, job: &JobSummary) -> Result<JobRowId, HistoryError> {
        let job = job.clone();
        let inner = self.inner.clone();
        task::spawn_blocking(move || -> Result<JobRowId, HistoryError> {
            let conn = inner.conn.lock().expect("history conn poisoned");
            let src = path_to_str(&job.src_root);
            let dst = path_to_str(&job.dst_root);
            conn.execute(
                "INSERT INTO jobs
                    (kind, status, started_at_ms, finished_at_ms,
                     src_root, dst_root, total_bytes, files_ok, files_failed,
                     verify_algo, options_json)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                params![
                    job.kind,
                    job.status,
                    job.started_at_ms,
                    job.finished_at_ms,
                    src,
                    dst,
                    job.total_bytes as i64,
                    job.files_ok as i64,
                    job.files_failed as i64,
                    job.verify_algo,
                    job.options_json,
                ],
            )?;
            Ok(JobRowId(conn.last_insert_rowid()))
        })
        .await
        .map_err(|_| HistoryError::WorkerPanicked)?
    }

    /// Append one `items` row.
    pub async fn record_item(&self, row: &ItemRow) -> Result<(), HistoryError> {
        let row = row.clone();
        let inner = self.inner.clone();
        task::spawn_blocking(move || -> Result<(), HistoryError> {
            let conn = inner.conn.lock().expect("history conn poisoned");
            conn.execute(
                "INSERT INTO items
                    (job_id, src, dst, size, status, hash_hex,
                     error_code, error_msg, timestamp_ms)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                params![
                    row.job_row_id,
                    path_to_str(&row.src),
                    path_to_str(&row.dst),
                    row.size as i64,
                    row.status,
                    row.hash_hex,
                    row.error_code,
                    row.error_msg,
                    row.timestamp_ms,
                ],
            )?;
            Ok(())
        })
        .await
        .map_err(|_| HistoryError::WorkerPanicked)?
    }

    /// Close out a job: flip status, stamp `finished_at_ms`, update
    /// the totals. Usually called once per job; idempotent — calling
    /// it twice for the same row will simply overwrite the totals
    /// with the latest values.
    pub async fn record_finish(
        &self,
        row_id: JobRowId,
        status: &str,
        total_bytes: u64,
        files_ok: u64,
        files_failed: u64,
    ) -> Result<(), HistoryError> {
        let status = status.to_string();
        let inner = self.inner.clone();
        task::spawn_blocking(move || -> Result<(), HistoryError> {
            let conn = inner.conn.lock().expect("history conn poisoned");
            conn.execute(
                "UPDATE jobs
                    SET status = ?1,
                        finished_at_ms = ?2,
                        total_bytes = ?3,
                        files_ok = ?4,
                        files_failed = ?5
                  WHERE id = ?6",
                params![
                    status,
                    now_ms_sync(),
                    total_bytes as i64,
                    files_ok as i64,
                    files_failed as i64,
                    row_id.0,
                ],
            )?;
            Ok(())
        })
        .await
        .map_err(|_| HistoryError::WorkerPanicked)?
    }

    // ---------- reads ----------

    /// Search jobs by filter. Results ordered newest-first (by
    /// `started_at_ms DESC`) with a hard cap at
    /// [`DEFAULT_SEARCH_LIMIT`] so one noisy user profile doesn't
    /// blow up the IPC payload.
    pub async fn search(&self, filter: HistoryFilter) -> Result<Vec<JobSummary>, HistoryError> {
        let inner = self.inner.clone();
        task::spawn_blocking(move || -> Result<Vec<JobSummary>, HistoryError> {
            let conn = inner.conn.lock().expect("history conn poisoned");
            let limit = filter
                .limit
                .unwrap_or(DEFAULT_SEARCH_LIMIT)
                .min(DEFAULT_SEARCH_LIMIT);

            // Dynamic WHERE so each filter-None passes through
            // untouched. Named parameters would be cleaner but
            // rusqlite's `query_map` insists on consistent param
            // counts; building the vec here keeps it readable.
            let mut sql = String::from(
                "SELECT id, kind, status, started_at_ms, finished_at_ms,
                        src_root, dst_root, total_bytes, files_ok, files_failed,
                        verify_algo, options_json
                   FROM jobs",
            );
            let mut clauses: Vec<String> = Vec::new();
            let mut args: Vec<rusqlite::types::Value> = Vec::new();
            if let Some(since) = filter.started_since_ms {
                clauses.push(format!("started_at_ms >= ?{}", clauses.len() + 1));
                args.push(rusqlite::types::Value::Integer(since));
            }
            if let Some(until) = filter.started_until_ms {
                clauses.push(format!("started_at_ms <= ?{}", clauses.len() + 1));
                args.push(rusqlite::types::Value::Integer(until));
            }
            if let Some(kind) = filter.kind.as_deref() {
                clauses.push(format!("kind = ?{}", clauses.len() + 1));
                args.push(rusqlite::types::Value::Text(kind.to_string()));
            }
            if let Some(status) = filter.status.as_deref() {
                clauses.push(format!("status = ?{}", clauses.len() + 1));
                args.push(rusqlite::types::Value::Text(status.to_string()));
            }
            if let Some(text) = filter.text.as_deref()
                && !text.is_empty()
            {
                let needle = format!("%{}%", text.to_lowercase());
                let idx = clauses.len() + 1;
                clauses.push(format!(
                    "(LOWER(src_root) LIKE ?{idx} OR LOWER(dst_root) LIKE ?{idx})"
                ));
                args.push(rusqlite::types::Value::Text(needle));
            }
            if !clauses.is_empty() {
                sql.push_str(" WHERE ");
                sql.push_str(&clauses.join(" AND "));
            }
            sql.push_str(" ORDER BY started_at_ms DESC LIMIT ?");
            sql.push_str(&(clauses.len() + 1).to_string());
            args.push(rusqlite::types::Value::Integer(i64::from(limit)));

            let mut stmt = conn.prepare(&sql)?;
            let rows = stmt.query_map(rusqlite::params_from_iter(args), row_to_summary)?;
            let mut out = Vec::new();
            for row in rows {
                out.push(row?);
            }
            Ok(out)
        })
        .await
        .map_err(|_| HistoryError::WorkerPanicked)?
    }

    /// Every `items` row for the given `job_id`, oldest-first.
    pub async fn items_for(&self, job_id: JobRowId) -> Result<Vec<ItemRow>, HistoryError> {
        let inner = self.inner.clone();
        task::spawn_blocking(move || -> Result<Vec<ItemRow>, HistoryError> {
            let conn = inner.conn.lock().expect("history conn poisoned");
            let mut stmt = conn.prepare(
                "SELECT job_id, src, dst, size, status, hash_hex,
                        error_code, error_msg, timestamp_ms
                   FROM items
                  WHERE job_id = ?1
               ORDER BY timestamp_ms ASC, id ASC",
            )?;
            let rows = stmt.query_map(params![job_id.0], row_to_item)?;
            let mut out = Vec::new();
            for row in rows {
                out.push(row?);
            }
            Ok(out)
        })
        .await
        .map_err(|_| HistoryError::WorkerPanicked)?
    }

    /// Fetch one job by rowid. Returns `None` if the row was purged.
    pub async fn get(&self, job_id: JobRowId) -> Result<Option<JobSummary>, HistoryError> {
        let inner = self.inner.clone();
        task::spawn_blocking(move || -> Result<Option<JobSummary>, HistoryError> {
            let conn = inner.conn.lock().expect("history conn poisoned");
            let mut stmt = conn.prepare(
                "SELECT id, kind, status, started_at_ms, finished_at_ms,
                        src_root, dst_root, total_bytes, files_ok, files_failed,
                        verify_algo, options_json
                   FROM jobs
                  WHERE id = ?1",
            )?;
            let mut rows = stmt.query(params![job_id.0])?;
            if let Some(row) = rows.next()? {
                Ok(Some(row_to_summary(row)?))
            } else {
                Ok(None)
            }
        })
        .await
        .map_err(|_| HistoryError::WorkerPanicked)?
    }

    /// Delete every job whose `started_at_ms` is older than `days *
    /// 86_400_000 ms` ago. Items cascade via the foreign-key rule.
    /// Returns the number of rows deleted from `jobs`.
    pub async fn purge_older_than(&self, days: u32) -> Result<u64, HistoryError> {
        let inner = self.inner.clone();
        // Clamp `days` to ~100 years so the multiplication below
        // can't overflow `i64` and produce a future cutoff that
        // wipes every row. Without this an IPC caller passing
        // `u32::MAX` (or close to it) would blow past `i64::MAX`
        // when multiplied by 86_400_000, wrap, and surface as a
        // bogus future timestamp.
        const MAX_DAYS: u32 = 36_500;
        let clamped_days = days.min(MAX_DAYS);
        task::spawn_blocking(move || -> Result<u64, HistoryError> {
            let conn = inner.conn.lock().expect("history conn poisoned");
            let cutoff = now_ms_sync()
                .saturating_sub(i64::from(clamped_days).saturating_mul(86_400_000));
            let n = conn.execute("DELETE FROM jobs WHERE started_at_ms < ?1", params![cutoff])?;
            Ok(n as u64)
        })
        .await
        .map_err(|_| HistoryError::WorkerPanicked)?
    }

    /// Wipe every row from both tables. Used by the Phase 10
    /// "Reset statistics" button. `VACUUM` is intentionally *not*
    /// run here — the file stays allocated so subsequent inserts
    /// don't re-grow from zero.
    pub async fn clear_all(&self) -> Result<u64, HistoryError> {
        let inner = self.inner.clone();
        task::spawn_blocking(move || -> Result<u64, HistoryError> {
            let conn = inner.conn.lock().expect("history conn poisoned");
            // Items cascade via the FK rule, but an explicit delete
            // also covers pathological cases where the cascade was
            // disabled (some SQLite builds disable it by default).
            conn.execute("DELETE FROM items", [])?;
            let n = conn.execute("DELETE FROM jobs", [])?;
            Ok(n as u64)
        })
        .await
        .map_err(|_| HistoryError::WorkerPanicked)?
    }

    // ---------- aggregates (Phase 10) ----------

    /// Lifetime aggregates. `since_ms` filters by
    /// `jobs.started_at_ms >= since_ms`; pass `None` for "all time".
    ///
    /// Computed in a single read-only transaction so the numbers
    /// are internally consistent even if the Tauri runner is
    /// writing a fresh row while this runs.
    pub async fn totals(&self, since_ms: Option<i64>) -> Result<Totals, HistoryError> {
        let inner = self.inner.clone();
        task::spawn_blocking(move || -> Result<Totals, HistoryError> {
            let conn = inner.conn.lock().expect("history conn poisoned");
            let tx = conn.unchecked_transaction()?;
            let (where_clause, since_param) = since_clause(since_ms);

            // Overall counters.
            let mut totals = Totals::default();
            let overall_sql = format!(
                "SELECT COUNT(*) AS jobs,
                        COALESCE(SUM(total_bytes), 0) AS bytes,
                        COALESCE(SUM(files_ok), 0) AS files,
                        COALESCE(SUM(CASE WHEN status = 'failed' THEN 1 ELSE 0 END), 0) AS errors,
                        COALESCE(SUM(CASE
                            WHEN finished_at_ms IS NOT NULL AND finished_at_ms >= started_at_ms
                            THEN finished_at_ms - started_at_ms
                            ELSE 0
                        END), 0) AS duration_ms
                   FROM jobs{where_clause}"
            );
            let mut stmt = tx.prepare(&overall_sql)?;
            let rows = stmt.query_map(rusqlite::params_from_iter(since_param.clone()), |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, i64>(2)?,
                    row.get::<_, i64>(3)?,
                    row.get::<_, i64>(4)?,
                ))
            })?;
            for r in rows {
                let (j, b, f, e, d) = r?;
                totals.jobs = j as u64;
                totals.bytes = b as u64;
                totals.files = f as u64;
                totals.errors = e as u64;
                totals.duration_ms = d as u64;
            }
            drop(stmt);

            // Per-kind breakdown.
            let kind_sql = format!(
                "SELECT kind,
                        COALESCE(SUM(total_bytes), 0) AS bytes,
                        COALESCE(SUM(files_ok), 0) AS files,
                        COUNT(*) AS jobs
                   FROM jobs{where_clause}
                  GROUP BY kind
                  ORDER BY kind"
            );
            let mut stmt = tx.prepare(&kind_sql)?;
            let rows = stmt.query_map(rusqlite::params_from_iter(since_param), |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, i64>(2)?,
                    row.get::<_, i64>(3)?,
                ))
            })?;
            for r in rows {
                let (kind, b, f, j) = r?;
                totals.by_kind.insert(
                    kind,
                    KindBreakdown {
                        bytes: b as u64,
                        files: f as u64,
                        jobs: j as u64,
                    },
                );
            }

            Ok(totals)
        })
        .await
        .map_err(|_| HistoryError::WorkerPanicked)?
    }

    /// Per-day buckets for the sparkline. Results are oldest-first;
    /// callers fill gaps (days with zero jobs) client-side so the
    /// chart has a dense 30-point series.
    ///
    /// `since_ms` is typically "30 days ago at UTC midnight"; the
    /// UI computes that and hands it in.
    pub async fn daily_totals(&self, since_ms: i64) -> Result<Vec<DayTotal>, HistoryError> {
        let inner = self.inner.clone();
        task::spawn_blocking(move || -> Result<Vec<DayTotal>, HistoryError> {
            let conn = inner.conn.lock().expect("history conn poisoned");
            // SQLite's `strftime('%s', ...)` works on seconds; our
            // `started_at_ms` is milliseconds. Divide inline; group
            // by the floored day-count. `86400000` ms / day.
            let sql = "
                SELECT (started_at_ms / 86400000) * 86400000 AS day_ms,
                       COALESCE(SUM(total_bytes), 0) AS bytes,
                       COALESCE(SUM(files_ok), 0)   AS files,
                       COUNT(*)                      AS jobs
                  FROM jobs
                 WHERE started_at_ms >= ?1
              GROUP BY day_ms
              ORDER BY day_ms ASC";
            let mut stmt = conn.prepare(sql)?;
            let rows = stmt.query_map(params![since_ms], |row| {
                Ok(DayTotal {
                    date_ms: row.get(0)?,
                    bytes: row.get::<_, i64>(1)? as u64,
                    files: row.get::<_, i64>(2)? as u64,
                    jobs: row.get::<_, i64>(3)? as u64,
                })
            })?;
            let mut out = Vec::new();
            for r in rows {
                out.push(r?);
            }
            Ok(out)
        })
        .await
        .map_err(|_| HistoryError::WorkerPanicked)?
    }
}

/// Build the `WHERE started_at_ms >= ?1` suffix + the param vector
/// that goes with it. Returns an empty suffix + empty params when
/// the caller asked for "all time".
fn since_clause(since_ms: Option<i64>) -> (&'static str, Vec<rusqlite::types::Value>) {
    match since_ms {
        Some(v) => (
            " WHERE started_at_ms >= ?1",
            vec![rusqlite::types::Value::Integer(v)],
        ),
        None => ("", Vec::new()),
    }
}

fn row_to_summary(row: &rusqlite::Row<'_>) -> rusqlite::Result<JobSummary> {
    Ok(JobSummary {
        row_id: row.get(0)?,
        kind: row.get(1)?,
        status: row.get(2)?,
        started_at_ms: row.get(3)?,
        finished_at_ms: row.get(4)?,
        src_root: PathBuf::from(row.get::<_, String>(5)?),
        dst_root: PathBuf::from(row.get::<_, String>(6)?),
        total_bytes: row.get::<_, i64>(7)? as u64,
        files_ok: row.get::<_, i64>(8)? as u64,
        files_failed: row.get::<_, i64>(9)? as u64,
        verify_algo: row.get(10)?,
        options_json: row.get(11)?,
    })
}

fn row_to_item(row: &rusqlite::Row<'_>) -> rusqlite::Result<ItemRow> {
    Ok(ItemRow {
        job_row_id: row.get(0)?,
        src: PathBuf::from(row.get::<_, String>(1)?),
        dst: PathBuf::from(row.get::<_, String>(2)?),
        size: row.get::<_, i64>(3)? as u64,
        status: row.get(4)?,
        hash_hex: row.get(5)?,
        error_code: row.get(6)?,
        error_msg: row.get(7)?,
        timestamp_ms: row.get(8)?,
    })
}

/// Lossy path-to-string conversion. SQLite `TEXT` is UTF-8; on the
/// handful of Windows filenames that aren't valid UTF-8 we take the
/// lossy transform rather than refuse to record the history row.
/// Phase 17 security review will revisit if this matters.
fn path_to_str(p: &Path) -> String {
    p.to_string_lossy().into_owned()
}

fn now_ms_sync() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

/// Resolve the default on-disk path. Exposed as a free function so
/// the Tauri layer can show "database: <path>" in the About dialog
/// without opening the DB.
pub fn default_db_path() -> Result<PathBuf, HistoryError> {
    let dirs =
        ProjectDirs::from("com", "CopyThat", "CopyThat2026").ok_or(HistoryError::NoDataDir)?;
    let dir = dirs.data_dir().to_path_buf();
    Ok(dir.join("history.db"))
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn fresh() -> History {
        History::open_in_memory().await.expect("open")
    }

    fn dummy_job() -> JobSummary {
        JobSummary {
            row_id: 0,
            kind: "copy".into(),
            status: "running".into(),
            started_at_ms: 1_000,
            finished_at_ms: None,
            src_root: PathBuf::from("/src"),
            dst_root: PathBuf::from("/dst"),
            total_bytes: 0,
            files_ok: 0,
            files_failed: 0,
            verify_algo: None,
            options_json: None,
        }
    }

    #[tokio::test]
    async fn record_start_returns_monotonic_row_ids() {
        let h = fresh().await;
        let a = h.record_start(&dummy_job()).await.unwrap();
        let b = h.record_start(&dummy_job()).await.unwrap();
        assert!(b.0 > a.0);
    }

    #[tokio::test]
    async fn record_finish_updates_status_and_totals() {
        let h = fresh().await;
        let id = h.record_start(&dummy_job()).await.unwrap();
        h.record_finish(id, "succeeded", 100, 2, 0).await.unwrap();
        let got = h.get(id).await.unwrap().expect("row");
        assert_eq!(got.status, "succeeded");
        assert_eq!(got.total_bytes, 100);
        assert_eq!(got.files_ok, 2);
        assert!(got.finished_at_ms.is_some());
    }

    #[tokio::test]
    async fn search_filters_by_kind_and_status() {
        let h = fresh().await;
        let a = h.record_start(&dummy_job()).await.unwrap();
        let mut mv = dummy_job();
        mv.kind = "move".into();
        mv.started_at_ms = 2_000;
        h.record_start(&mv).await.unwrap();
        h.record_finish(a, "succeeded", 0, 0, 0).await.unwrap();

        let copy_done = h
            .search(HistoryFilter {
                kind: Some("copy".into()),
                status: Some("succeeded".into()),
                ..HistoryFilter::default()
            })
            .await
            .unwrap();
        assert_eq!(copy_done.len(), 1);
        assert_eq!(copy_done[0].kind, "copy");

        let moves = h
            .search(HistoryFilter {
                kind: Some("move".into()),
                ..HistoryFilter::default()
            })
            .await
            .unwrap();
        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0].kind, "move");
    }

    #[tokio::test]
    async fn search_text_matches_both_roots() {
        let h = fresh().await;
        let mut j = dummy_job();
        j.src_root = PathBuf::from("/home/kevin/photos");
        j.dst_root = PathBuf::from("/mnt/backup");
        h.record_start(&j).await.unwrap();

        let hits = h
            .search(HistoryFilter {
                text: Some("photos".into()),
                ..HistoryFilter::default()
            })
            .await
            .unwrap();
        assert_eq!(hits.len(), 1);

        let hits2 = h
            .search(HistoryFilter {
                text: Some("backup".into()),
                ..HistoryFilter::default()
            })
            .await
            .unwrap();
        assert_eq!(hits2.len(), 1);

        let miss = h
            .search(HistoryFilter {
                text: Some("nonexistent".into()),
                ..HistoryFilter::default()
            })
            .await
            .unwrap();
        assert_eq!(miss.len(), 0);
    }

    #[tokio::test]
    async fn items_for_returns_inserted_rows_oldest_first() {
        let h = fresh().await;
        let id = h.record_start(&dummy_job()).await.unwrap();
        for i in 0..3 {
            h.record_item(&ItemRow {
                job_row_id: id.0,
                src: PathBuf::from(format!("/s{i}")),
                dst: PathBuf::from(format!("/d{i}")),
                size: i * 100,
                status: "ok".into(),
                hash_hex: None,
                error_code: None,
                error_msg: None,
                timestamp_ms: (i as i64) * 10,
            })
            .await
            .unwrap();
        }
        let items = h.items_for(id).await.unwrap();
        assert_eq!(items.len(), 3);
        assert_eq!(items[0].size, 0);
        assert_eq!(items[2].size, 200);
    }

    #[tokio::test]
    async fn purge_older_than_drops_jobs_and_cascades_items() {
        let h = fresh().await;
        let id = h.record_start(&dummy_job()).await.unwrap(); // started_at_ms = 1_000
        h.record_item(&ItemRow {
            job_row_id: id.0,
            src: "/s".into(),
            dst: "/d".into(),
            size: 0,
            status: "ok".into(),
            hash_hex: None,
            error_code: None,
            error_msg: None,
            timestamp_ms: 0,
        })
        .await
        .unwrap();

        // Purging with days = 0 removes anything strictly before
        // "now"; the 1_000 ms timestamp we stamped is ancient.
        let dropped = h.purge_older_than(0).await.unwrap();
        assert_eq!(dropped, 1);
        assert!(h.get(id).await.unwrap().is_none());
        assert_eq!(h.items_for(id).await.unwrap().len(), 0);
    }

    #[tokio::test]
    async fn search_limit_is_capped() {
        let h = fresh().await;
        for i in 0..10 {
            let mut j = dummy_job();
            j.started_at_ms = i;
            h.record_start(&j).await.unwrap();
        }
        let rows = h
            .search(HistoryFilter {
                limit: Some(3),
                ..HistoryFilter::default()
            })
            .await
            .unwrap();
        assert_eq!(rows.len(), 3);
        // Newest-first ordering — started_at_ms = 9 first.
        assert_eq!(rows[0].started_at_ms, 9);
    }

    // ---------- Phase 10 aggregates ----------

    #[tokio::test]
    async fn totals_sums_bytes_files_jobs_errors_duration() {
        let h = fresh().await;
        // Three copy jobs: two succeed (1000 + 2000 bytes, 2 + 3
        // files, each 50 ms duration), one fails (no bytes / files,
        // but the duration_ms still includes the 25-ms window).
        for (bytes, files, status, dur) in [
            (1_000u64, 2u64, "succeeded", 50i64),
            (2_000u64, 3u64, "succeeded", 50i64),
            (0u64, 0u64, "failed", 25i64),
        ] {
            let mut j = dummy_job();
            j.started_at_ms = 10_000;
            j.finished_at_ms = Some(10_000 + dur);
            j.status = status.into();
            j.total_bytes = bytes;
            j.files_ok = files;
            let id = h.record_start(&j).await.unwrap();
            h.record_finish(id, status, bytes, files, 0).await.unwrap();
        }
        let t = h.totals(None).await.unwrap();
        // record_finish stamps `finished_at_ms = now()`, so the
        // duration calc in SQL is "now - 10_000" per job — at least
        // a few milliseconds. The test just asserts monotone > 0.
        assert_eq!(t.jobs, 3);
        assert_eq!(t.bytes, 3_000);
        assert_eq!(t.files, 5);
        assert_eq!(t.errors, 1);
        assert!(t.duration_ms > 0);

        let copy = t.by_kind.get("copy").expect("by_kind must include `copy`");
        assert_eq!(copy.jobs, 3);
        assert_eq!(copy.bytes, 3_000);
        assert_eq!(copy.files, 5);
    }

    #[tokio::test]
    async fn totals_by_kind_splits_copy_and_move() {
        let h = fresh().await;

        let mut copy = dummy_job();
        copy.started_at_ms = 10_000;
        copy.total_bytes = 500;
        copy.files_ok = 1;
        let id = h.record_start(&copy).await.unwrap();
        h.record_finish(id, "succeeded", 500, 1, 0).await.unwrap();

        let mut mv = dummy_job();
        mv.kind = "move".into();
        mv.started_at_ms = 20_000;
        mv.total_bytes = 2_000;
        mv.files_ok = 4;
        let id = h.record_start(&mv).await.unwrap();
        h.record_finish(id, "succeeded", 2_000, 4, 0).await.unwrap();

        let t = h.totals(None).await.unwrap();
        let c = t.by_kind.get("copy").unwrap();
        assert_eq!(c.bytes, 500);
        assert_eq!(c.files, 1);
        let m = t.by_kind.get("move").unwrap();
        assert_eq!(m.bytes, 2_000);
        assert_eq!(m.files, 4);
    }

    #[tokio::test]
    async fn totals_since_filter_excludes_older_rows() {
        let h = fresh().await;
        for ts in [1_000i64, 5_000, 9_000] {
            let mut j = dummy_job();
            j.started_at_ms = ts;
            j.total_bytes = 100;
            j.files_ok = 1;
            let id = h.record_start(&j).await.unwrap();
            h.record_finish(id, "succeeded", 100, 1, 0).await.unwrap();
        }
        let recent = h.totals(Some(5_000)).await.unwrap();
        assert_eq!(recent.jobs, 2);
        assert_eq!(recent.bytes, 200);
    }

    #[tokio::test]
    async fn daily_totals_buckets_by_utc_day() {
        let h = fresh().await;
        const DAY: i64 = 86_400_000;
        // Three jobs on the same UTC day, two on the next, one on
        // the day after that. SQL floors to day_ms via integer
        // division; we mirror that here when picking timestamps.
        for ts in [
            10 * DAY + 1_000,
            10 * DAY + 50_000,
            10 * DAY + 3_600_000,
            11 * DAY + 10_000,
            11 * DAY + 20_000,
            12 * DAY + 100,
        ] {
            let mut j = dummy_job();
            j.started_at_ms = ts;
            j.total_bytes = 100;
            j.files_ok = 1;
            let id = h.record_start(&j).await.unwrap();
            h.record_finish(id, "succeeded", 100, 1, 0).await.unwrap();
        }
        let buckets = h.daily_totals(0).await.unwrap();
        assert_eq!(buckets.len(), 3);
        assert_eq!(buckets[0].date_ms, 10 * DAY);
        assert_eq!(buckets[0].jobs, 3);
        assert_eq!(buckets[0].bytes, 300);
        assert_eq!(buckets[1].date_ms, 11 * DAY);
        assert_eq!(buckets[1].jobs, 2);
        assert_eq!(buckets[2].date_ms, 12 * DAY);
        assert_eq!(buckets[2].jobs, 1);
    }

    #[tokio::test]
    async fn clear_all_wipes_jobs_and_items() {
        let h = fresh().await;
        let id = h.record_start(&dummy_job()).await.unwrap();
        h.record_item(&ItemRow {
            job_row_id: id.as_i64(),
            src: "/s".into(),
            dst: "/d".into(),
            size: 1,
            status: "ok".into(),
            hash_hex: None,
            error_code: None,
            error_msg: None,
            timestamp_ms: 0,
        })
        .await
        .unwrap();
        assert_eq!(h.totals(None).await.unwrap().jobs, 1);

        let dropped = h.clear_all().await.unwrap();
        assert_eq!(dropped, 1);
        assert_eq!(h.totals(None).await.unwrap().jobs, 0);
        assert_eq!(h.items_for(id).await.unwrap().len(), 0);
    }
}
