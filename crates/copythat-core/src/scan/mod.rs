//! Phase 19a — TeraCopy-style disk-backed file enumeration.
//!
//! The [`Scanner`] streams a `walkdir` walk of a source tree into a
//! per-job SQLite database (`scan-<uuid>.db`) under the OS user-
//! config directory. Every enumerated entry is persisted as a
//! `scan_items` row with size, mtime, kind, attribute bits, and an
//! optional BLAKE3 content hash. The [`ScanCursor`] streams rows
//! back in `rel_path ASC` order so the copy engine can read them
//! without a second walk of the filesystem.
//!
//! Memory characteristics:
//!
//! - Walker pumps [`ScanItem`] values into a bounded mpsc channel
//!   (capacity 1024).
//! - The DB-writer task drains 1000 rows per `BEGIN … COMMIT`
//!   transaction, updates `scan_progress` after each commit, and
//!   hands off `(rowid, path)` tuples to the hash workers when
//!   on-the-fly hashing is enabled.
//! - A 5-million-file walk never holds more than one batch in
//!   memory; RSS stays well below the 200 MiB Phase 19a target.
//!
//! Crash safety:
//!
//! - The scan DB opens in WAL mode with `synchronous=NORMAL`. A
//!   kill-9 between batch commits loses at most one in-flight
//!   batch — an order of magnitude better than the pre-Phase-19a
//!   in-memory-only plan that lost the whole walk.
//! - `scan_progress.last_visited_path` is updated atomically with
//!   each batch, so resume can short-circuit already-scanned
//!   subtrees.
//! - A top-level `main.db` under `<config-dir>/scans/main.db`
//!   indexes every per-scan DB with its current lifecycle status
//!   so the UI can offer to resume `Running` / `Paused` scans at
//!   launch.
//!
//! # Example
//!
//! ```no_run
//! # use std::path::Path;
//! # use tokio::sync::mpsc;
//! # use copythat_core::scan::{ScanControl, ScanEvent, ScanId, ScanOptions, Scanner};
//! # async fn demo() -> Result<(), copythat_core::CopyError> {
//! let (tx, mut rx) = mpsc::channel::<ScanEvent>(64);
//! let ctrl = ScanControl::new();
//! let scanner = Scanner::create(ScanId::new(), Path::new("/src"), ScanOptions::default())?;
//! let cursor = scanner.cursor_path();
//! let handle = tokio::spawn(scanner.run(ctrl.clone(), tx));
//! while let Some(evt) = rx.recv().await {
//!     if matches!(evt, ScanEvent::Completed { .. } | ScanEvent::Failed { .. } | ScanEvent::Cancelled) {
//!         break;
//!     }
//! }
//! let report = handle.await.expect("join")?;
//! # let _ = cursor; let _ = report;
//! # Ok(()) }
//! ```

mod control;
mod cursor;
mod event;
pub mod index;
pub mod schema;
mod types;

use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::{Instant, SystemTime};

use rusqlite::{Connection, params};
use tokio::sync::mpsc;

use crate::error::{CopyError, CopyErrorKind};
use crate::filter::CompiledFilters;
use crate::safety::validate_path_no_traversal;

pub use control::ScanControl;
pub use cursor::{ScanCursor, absolute as absolute_path_for};
pub use event::ScanEvent;
pub use types::{
    AttrFlags, EntryKind, ScanId, ScanItem, ScanOptions, ScanReport, ScanStats, ScanStatus,
};

/// Hard cap on the mpsc channel between the walker and the DB
/// writer. Walker produces faster than the writer commits on most
/// storage; 1024 gives a full batch of buffer without letting the
/// walker run away with RSS on a 5 M-file scan.
const CHANNEL_CAPACITY: usize = 1024;

/// Rows per `BEGIN … COMMIT`. Phase 19a's build prompt explicitly
/// names 1000.
const BATCH_SIZE: usize = 1000;

/// Buffer size for the streaming BLAKE3 hasher. Matches the async
/// copy loop's default so the page-cache footprint is the same.
const HASH_BUFFER_SIZE: usize = 1024 * 1024;

/// Per-scan SQLite database — streaming enumeration + optional
/// on-the-fly hashing + pause / resume / cancel.
pub struct Scanner {
    scan_id: ScanId,
    root: PathBuf,
    db_path: PathBuf,
    opts: ScanOptions,
    conn: Arc<Mutex<Connection>>,
    index_path: Option<PathBuf>,
}

impl Scanner {
    /// Create or re-open a scan DB. On a fresh `scan_id` the DB is
    /// initialised, registered in `main.db`, and its meta row stamped
    /// with the root path + start time. On a previously-seen
    /// `scan_id` (resume) the existing DB is re-opened, the runtime
    /// PRAGMAs are re-applied, and the status is flipped to `Running`.
    pub fn create(
        scan_id: ScanId,
        root: &Path,
        opts: ScanOptions,
    ) -> Result<Self, CopyError> {
        // Phase 17a — reject traversal payloads at the trust boundary
        // before opening any file handle.
        if let Err(e) = validate_path_no_traversal(root) {
            return Err(CopyError::path_escape(root, root, e));
        }
        let root = root.to_path_buf();

        let db_dir = match &opts.db_dir {
            Some(dir) => dir.clone(),
            None => default_scan_dir().ok_or_else(|| other_err(&root, "no OS config dir available"))?,
        };
        std::fs::create_dir_all(&db_dir).map_err(|e| {
            CopyError::from_io(&root, &db_dir, e)
        })?;
        let db_path = db_dir.join(format!("scan-{}.db", scan_id.as_str()));

        let mut conn = Connection::open(&db_path).map_err(|e| rusqlite_err(&root, &db_path, e))?;
        schema::apply_pending(&mut conn).map_err(|e| schema_err(&root, &db_path, e))?;
        schema::apply_runtime_pragmas(&conn).map_err(|e| rusqlite_err(&root, &db_path, e))?;

        // Seed / refresh scan_meta.
        let now_ms = now_ms();
        let existing_status: Option<String> = conn
            .query_row(
                "SELECT value FROM scan_meta WHERE key='status'",
                [],
                |r| r.get(0),
            )
            .ok();
        let is_resume = existing_status.as_deref().is_some_and(|s| {
            matches!(s, "Paused" | "Running")
        });
        set_meta(&conn, "root_path", &root.to_string_lossy()).map_err(|e| rusqlite_err(&root, &db_path, e))?;
        if !is_resume {
            set_meta(&conn, "started_at_ms", &now_ms.to_string()).map_err(|e| rusqlite_err(&root, &db_path, e))?;
        }
        set_meta(&conn, "hash_algorithm", if opts.hash_during_scan { "blake3-256" } else { "none" })
            .map_err(|e| rusqlite_err(&root, &db_path, e))?;
        set_meta(&conn, "status", ScanStatus::Running.as_str())
            .map_err(|e| rusqlite_err(&root, &db_path, e))?;

        // Register in main.db so resume sweeps can find us.
        let index_path = index::default_index_path();
        if let Some(ip) = &index_path
            && let Ok(idx) = index::open_index(ip)
        {
            let _ = index::register(
                &idx,
                scan_id,
                &db_path,
                None,
                now_ms,
                ScanStatus::Running,
            );
        }

        Ok(Self {
            scan_id,
            root,
            db_path,
            opts,
            conn: Arc::new(Mutex::new(conn)),
            index_path,
        })
    }

    pub fn scan_id(&self) -> ScanId {
        self.scan_id
    }

    pub fn db_path(&self) -> &Path {
        &self.db_path
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Path usable with [`ScanCursor::open`]. Convenience for
    /// consumers that want a fresh read-only connection instead of
    /// reaching through `self.conn`.
    pub fn cursor_path(&self) -> PathBuf {
        self.db_path.clone()
    }

    /// Snapshot of current scan counters, read straight from the DB.
    pub fn stats(&self) -> Result<ScanStats, CopyError> {
        let conn = self.conn.lock().expect("scan conn poisoned");
        read_stats(&conn).map_err(|e| rusqlite_err(&self.root, &self.db_path, e))
    }

    /// Drive the scan to completion.
    ///
    /// Runs the walker + DB-writer + optional hash workers until the
    /// tree is exhausted or [`ScanControl::cancel`] fires. The final
    /// `scan_meta.status` transitions to `Complete` / `Cancelled` /
    /// `Failed`.
    pub async fn run(
        self,
        ctrl: ScanControl,
        events: mpsc::Sender<ScanEvent>,
    ) -> Result<ScanReport, CopyError> {
        let Scanner {
            scan_id,
            root,
            db_path,
            opts,
            conn,
            index_path,
        } = self;

        let _ = events
            .send(ScanEvent::Started {
                scan_id,
                root: root.clone(),
                db_path: db_path.clone(),
            })
            .await;

        let started = Instant::now();
        let files_discovered = Arc::new(AtomicU64::new(0));
        let bytes_discovered = Arc::new(AtomicU64::new(0));
        let files_written = Arc::new(AtomicU64::new(0));
        let files_hashed = Arc::new(AtomicU64::new(0));

        // Compile filters up front (invalid glob → fail immediately).
        let compiled_filters: Option<Arc<CompiledFilters>> = match &opts.filters {
            Some(set) if !set.is_empty() => match set.compile() {
                Ok(c) => Some(Arc::new(c)),
                Err(e) => {
                    let err = CopyError {
                        kind: CopyErrorKind::IoOther,
                        src: root.clone(),
                        dst: db_path.clone(),
                        raw_os_error: None,
                        message: e.to_string(),
                    };
                    finalize_status(&conn, &index_path, scan_id, ScanStatus::Failed);
                    let _ = events.send(ScanEvent::Failed { err: err.clone() }).await;
                    return Err(err);
                }
            },
            _ => None,
        };

        // Walker → DB writer channel.
        let (item_tx, mut item_rx) = mpsc::channel::<RawItem>(CHANNEL_CAPACITY);

        // Hash request channel (writer → hash workers). Only used
        // when hash_during_scan is set.
        let (hash_tx, hash_rx) = mpsc::channel::<HashRequest>(CHANNEL_CAPACITY);
        let hash_rx = Arc::new(tokio::sync::Mutex::new(hash_rx));
        let maybe_hash_tx = opts.hash_during_scan.then(|| hash_tx.clone());
        drop(hash_tx);

        // Spawn walker.
        let walker_root = root.clone();
        let walker_ctrl = ctrl.clone();
        let walker_files_disc = files_discovered.clone();
        let walker_bytes_disc = bytes_discovered.clone();
        let walker_events = events.clone();
        let walker_filters = compiled_filters.clone();
        let follow = opts.follow_symlinks;
        let progress_every = opts.progress_every;
        let walker_handle = tokio::task::spawn_blocking(move || {
            enumerate_into_channel(
                walker_root,
                follow,
                walker_filters,
                progress_every,
                walker_ctrl,
                walker_files_disc,
                walker_bytes_disc,
                walker_events,
                item_tx,
            )
        });

        // Spawn hash workers (if enabled).
        let mut hash_worker_handles = Vec::new();
        let hash_worker_count = if opts.hash_during_scan {
            opts.hash_workers.clamp(1, 64)
        } else {
            0
        };
        for _ in 0..hash_worker_count {
            let root_i = root.clone();
            let conn_i = conn.clone();
            let ctrl_i = ctrl.clone();
            let counter = files_hashed.clone();
            let rx = hash_rx.clone();
            hash_worker_handles.push(tokio::spawn(async move {
                hash_worker_loop(root_i, conn_i, ctrl_i, counter, rx).await
            }));
        }

        // DB writer loop — batch-commit rows from the walker.
        let writer_ctrl = ctrl.clone();
        let writer_conn = conn.clone();
        let writer_root = root.clone();
        let writer_db_path = db_path.clone();
        let writer_events = events.clone();
        let writer_files_written = files_written.clone();
        let writer_files_disc = files_discovered.clone();
        let writer_bytes_disc = bytes_discovered.clone();
        let writer_files_hashed = files_hashed.clone();
        let writer_hash_tx = maybe_hash_tx;
        let writer_progress_every = opts.progress_every;
        let mut first_error: Option<CopyError> = None;

        let mut batch: Vec<RawItem> = Vec::with_capacity(BATCH_SIZE);
        loop {
            if writer_ctrl.is_cancelled() {
                break;
            }
            if writer_ctrl.is_paused() {
                let _ = writer_events.send(ScanEvent::Paused).await;
                writer_ctrl.wait_while_paused().await;
                if writer_ctrl.is_cancelled() {
                    break;
                }
                let _ = writer_events.send(ScanEvent::Resumed).await;
            }
            match item_rx.recv().await {
                Some(item) => {
                    batch.push(item);
                    if batch.len() >= BATCH_SIZE {
                        match commit_batch(
                            &writer_conn,
                            &mut batch,
                            &writer_files_written,
                            &writer_hash_tx,
                        )
                        .await
                        {
                            Ok((last_path, n)) => {
                                let _ = writer_events
                                    .send(ScanEvent::BatchFlushed {
                                        batch_size: n,
                                        files_committed: writer_files_written.load(Ordering::Relaxed),
                                        last_rel_path: last_path,
                                    })
                                    .await;
                                if writer_progress_every > 0 {
                                    let _ = writer_events
                                        .send(ScanEvent::Progress {
                                            files_discovered: writer_files_disc.load(Ordering::Relaxed),
                                            bytes_discovered: writer_bytes_disc.load(Ordering::Relaxed),
                                            files_written: writer_files_written.load(Ordering::Relaxed),
                                            files_hashed: writer_files_hashed.load(Ordering::Relaxed),
                                        })
                                        .await;
                                }
                            }
                            Err(e) => {
                                let err = rusqlite_err(&writer_root, &writer_db_path, e);
                                if first_error.is_none() {
                                    first_error = Some(err.clone());
                                }
                                writer_ctrl.cancel();
                                break;
                            }
                        }
                    }
                }
                None => break,
            }
        }
        // Trailing partial batch (if any).
        if !batch.is_empty() && first_error.is_none() && !writer_ctrl.is_cancelled() {
            match commit_batch(
                &writer_conn,
                &mut batch,
                &writer_files_written,
                &writer_hash_tx,
            )
            .await
            {
                Ok((last_path, n)) => {
                    let _ = writer_events
                        .send(ScanEvent::BatchFlushed {
                            batch_size: n,
                            files_committed: writer_files_written.load(Ordering::Relaxed),
                            last_rel_path: last_path,
                        })
                        .await;
                }
                Err(e) => {
                    let err = rusqlite_err(&writer_root, &writer_db_path, e);
                    if first_error.is_none() {
                        first_error = Some(err);
                    }
                }
            }
        }
        batch.clear();

        // Close the hash-request channel; workers exit once drained.
        drop(writer_hash_tx);

        // Join walker.
        match walker_handle.await {
            Ok(Ok(())) => {}
            Ok(Err(e)) => {
                let err = CopyError::from_io(&root, &db_path, e);
                if first_error.is_none() {
                    first_error = Some(err);
                }
            }
            Err(join_err) => {
                if first_error.is_none() {
                    first_error = Some(CopyError {
                        kind: CopyErrorKind::IoOther,
                        src: root.clone(),
                        dst: db_path.clone(),
                        raw_os_error: None,
                        message: format!("scan walker task panicked: {join_err}"),
                    });
                }
            }
        }

        // Drain hash workers.
        for h in hash_worker_handles {
            let _ = h.await;
        }

        // Finalize status. Cancel wins over a writer error, and an
        // error wins over a clean completion.
        let elapsed = started.elapsed();
        let (terminal_event, terminal_status, err_out) = if let Some(err) = first_error {
            (
                Some(ScanEvent::Failed { err: err.clone() }),
                ScanStatus::Failed,
                Err(err),
            )
        } else if ctrl.is_cancelled() {
            (Some(ScanEvent::Cancelled), ScanStatus::Cancelled, Err(CopyError::cancelled(&root, &db_path)))
        } else {
            (
                Some(ScanEvent::Completed {
                    files: files_written.load(Ordering::Relaxed),
                    bytes: bytes_discovered.load(Ordering::Relaxed),
                    hashed_files: files_hashed.load(Ordering::Relaxed),
                    duration: elapsed,
                }),
                ScanStatus::Complete,
                Ok(()),
            )
        };
        {
            let c = conn.lock().expect("scan conn poisoned");
            let _ = set_meta(&c, "finished_at_ms", &now_ms().to_string());
            let _ = set_meta(
                &c,
                "total_files",
                &files_written.load(Ordering::Relaxed).to_string(),
            );
            let _ = set_meta(
                &c,
                "total_bytes",
                &bytes_discovered.load(Ordering::Relaxed).to_string(),
            );
            let _ = set_meta(&c, "status", terminal_status.as_str());
        }
        if let Some(ip) = &index_path
            && let Ok(idx) = index::open_index(ip)
        {
            let _ = index::update_status(&idx, scan_id, terminal_status);
        }
        if let Some(evt) = terminal_event {
            let _ = events.send(evt).await;
        }

        match err_out {
            Ok(()) => {
                let stats = {
                    let c = conn.lock().expect("scan conn poisoned");
                    read_stats(&c).unwrap_or_default()
                };
                Ok(ScanReport {
                    scan_id,
                    db_path,
                    root,
                    stats,
                    duration: elapsed,
                    hashed_files: files_hashed.load(Ordering::Relaxed),
                })
            }
            Err(e) => Err(e),
        }
    }
}

// ---------- internals ----------

/// Raw entry pulled off the walker. Lives in the mpsc buffer until
/// the writer picks it up and turns it into a `scan_items` row.
struct RawItem {
    rel_path: String,
    abs_path: PathBuf,
    size: u64,
    mtime: SystemTime,
    kind: EntryKind,
    attrs: AttrFlags,
}

struct HashRequest {
    rowid: i64,
    abs_path: PathBuf,
}

async fn commit_batch(
    conn: &Arc<Mutex<Connection>>,
    batch: &mut Vec<RawItem>,
    files_written: &Arc<AtomicU64>,
    hash_tx: &Option<mpsc::Sender<HashRequest>>,
) -> Result<(String, u32), rusqlite::Error> {
    let taken: Vec<RawItem> = std::mem::take(batch);
    let conn = conn.clone();
    let fw = files_written.clone();
    let collect_hashes = hash_tx.is_some();
    // Offload the whole batch commit onto a blocking thread so the
    // tokio runtime doesn't stall on fsync.
    let (last_rel, written, hash_reqs) = tokio::task::spawn_blocking(
        move || -> Result<(String, u32, Vec<HashRequest>), rusqlite::Error> {
            let mut guard = conn.lock().expect("scan conn poisoned");
            let tx = guard.transaction()?;
            let mut last_rel = String::new();
            let mut files_count = 0u32;
            let mut bytes_count: i64 = 0;
            let mut hash_reqs = Vec::with_capacity(taken.len());
            let scanned_at = now_ms();
            {
                let mut stmt = tx.prepare(
                    "INSERT OR IGNORE INTO scan_items
                        (rel_path, size, mtime, kind, attrs, content_hash, scanned_at)
                     VALUES (?1, ?2, ?3, ?4, ?5, NULL, ?6)",
                )?;
                for item in &taken {
                    let n = stmt.execute(params![
                        item.rel_path,
                        item.size as i64,
                        cursor::system_time_to_unix_nanos(item.mtime),
                        item.kind.to_i64(),
                        item.attrs.to_i64(),
                        scanned_at,
                    ])?;
                    if n == 1 {
                        if collect_hashes && item.kind == EntryKind::File && item.size > 0 {
                            let rowid = tx.last_insert_rowid();
                            hash_reqs.push(HashRequest {
                                rowid,
                                abs_path: item.abs_path.clone(),
                            });
                        }
                        bytes_count = bytes_count.saturating_add(item.size as i64);
                        files_count += 1;
                        last_rel = item.rel_path.clone();
                    }
                }
            }
            // Update the resume marker inside the same transaction.
            if !last_rel.is_empty() {
                tx.execute(
                    "UPDATE scan_progress
                        SET last_visited_path=?1,
                            files_visited = files_visited + ?2,
                            bytes_visited = bytes_visited + ?3
                      WHERE id=1",
                    params![&last_rel, files_count as i64, bytes_count],
                )?;
            }
            tx.commit()?;
            fw.fetch_add(files_count as u64, Ordering::Relaxed);
            Ok((last_rel, files_count, hash_reqs))
        },
    )
    .await
    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(join_err_to_io(e))))??;

    if let Some(tx) = hash_tx {
        for req in hash_reqs {
            if tx.send(req).await.is_err() {
                break;
            }
        }
    }

    Ok((last_rel, written))
}

fn join_err_to_io(e: tokio::task::JoinError) -> std::io::Error {
    std::io::Error::other(format!("scan batch task panicked: {e}"))
}

async fn hash_worker_loop(
    root: PathBuf,
    conn: Arc<Mutex<Connection>>,
    ctrl: ScanControl,
    counter: Arc<AtomicU64>,
    rx: Arc<tokio::sync::Mutex<mpsc::Receiver<HashRequest>>>,
) {
    loop {
        if ctrl.is_cancelled() {
            return;
        }
        if ctrl.is_paused() {
            ctrl.wait_while_paused().await;
            if ctrl.is_cancelled() {
                return;
            }
        }
        let req = {
            let mut guard = rx.lock().await;
            match guard.recv().await {
                Some(r) => r,
                None => return,
            }
        };
        let _ = root.as_path();
        let rowid = req.rowid;
        let path = req.abs_path.clone();
        let conn_h = conn.clone();
        let result = tokio::task::spawn_blocking(move || -> std::io::Result<[u8; 32]> {
            hash_file_blake3(&path)
        })
        .await;
        match result {
            Ok(Ok(digest)) => {
                let conn_u = conn_h.clone();
                let _ = tokio::task::spawn_blocking(move || -> Result<(), rusqlite::Error> {
                    let guard = conn_u.lock().expect("scan conn poisoned");
                    guard.execute(
                        "UPDATE scan_items SET content_hash=?1 WHERE rowid=?2",
                        params![&digest[..], rowid],
                    )?;
                    Ok(())
                })
                .await;
                counter.fetch_add(1, Ordering::Relaxed);
            }
            Ok(Err(_)) | Err(_) => {
                // Hashing is best-effort; leave content_hash NULL.
            }
        }
    }
}

fn hash_file_blake3(path: &Path) -> std::io::Result<[u8; 32]> {
    let mut hasher = blake3::Hasher::new();
    let mut file = std::fs::File::open(path)?;
    let mut buf = vec![0u8; HASH_BUFFER_SIZE];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(*hasher.finalize().as_bytes())
}

#[allow(clippy::too_many_arguments)]
fn enumerate_into_channel(
    root: PathBuf,
    follow: bool,
    filters: Option<Arc<CompiledFilters>>,
    progress_every: u32,
    ctrl: ScanControl,
    files_disc: Arc<AtomicU64>,
    bytes_disc: Arc<AtomicU64>,
    events: mpsc::Sender<ScanEvent>,
    item_tx: mpsc::Sender<RawItem>,
) -> std::io::Result<()> {
    let mut it = walkdir::WalkDir::new(&root)
        .follow_links(follow)
        .sort_by_file_name()
        .into_iter();
    let mut since_last_progress: u32 = 0;
    while let Some(entry) = it.next() {
        if ctrl.is_cancelled() {
            return Ok(());
        }
        if ctrl.is_paused() {
            // Synchronous walker — spin in a tight sleep instead of
            // .await. The blocking task runtime lets us `park` via
            // a short thread-sleep without wedging anything.
            while ctrl.is_paused() && !ctrl.is_cancelled() {
                std::thread::sleep(std::time::Duration::from_millis(25));
            }
            if ctrl.is_cancelled() {
                return Ok(());
            }
        }
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                let denied = e
                    .io_error()
                    .map(|io| io.kind() == std::io::ErrorKind::PermissionDenied)
                    .unwrap_or(false);
                if denied {
                    continue;
                }
                return Err(std::io::Error::other(format!(
                    "scan walk error at {:?}: {e}",
                    e.path()
                )));
            }
        };
        let path = entry.path();
        let rel = path.strip_prefix(&root).unwrap_or(path).to_path_buf();
        if rel.as_os_str().is_empty() {
            continue;
        }
        let ft = entry.file_type();
        let kind = if ft.is_dir() {
            EntryKind::Dir
        } else if ft.is_symlink() {
            EntryKind::Symlink
        } else if ft.is_file() {
            EntryKind::File
        } else {
            EntryKind::Other
        };
        let meta = entry.metadata().ok();

        if let Some(f) = filters.as_deref() {
            match kind {
                EntryKind::Dir => {
                    if let Some(m) = meta.as_ref()
                        && !f.passes_dir(&rel, m)
                    {
                        it.skip_current_dir();
                        continue;
                    }
                }
                EntryKind::File | EntryKind::Symlink | EntryKind::Other => {
                    if let Some(m) = meta.as_ref()
                        && !f.passes_file(&rel, m)
                    {
                        continue;
                    }
                }
            }
        }

        let size = if matches!(kind, EntryKind::File) {
            meta.as_ref().map(|m| m.len()).unwrap_or(0)
        } else {
            0
        };
        let mtime = meta
            .as_ref()
            .and_then(|m| m.modified().ok())
            .unwrap_or(SystemTime::UNIX_EPOCH);
        let attrs = meta.as_ref().map(attr_flags_for).unwrap_or_default();

        if matches!(kind, EntryKind::File) {
            files_disc.fetch_add(1, Ordering::Relaxed);
            bytes_disc.fetch_add(size, Ordering::Relaxed);
        }

        let rel_path = rel.to_string_lossy().replace('\\', "/");
        let item = RawItem {
            rel_path,
            abs_path: path.to_path_buf(),
            size,
            mtime,
            kind,
            attrs,
        };

        if item_tx.blocking_send(item).is_err() {
            // Dispatcher dropped — stop walking.
            return Ok(());
        }

        if progress_every > 0 {
            since_last_progress = since_last_progress.saturating_add(1);
            if since_last_progress >= progress_every {
                since_last_progress = 0;
                let _ = events.try_send(ScanEvent::Progress {
                    files_discovered: files_disc.load(Ordering::Relaxed),
                    bytes_discovered: bytes_disc.load(Ordering::Relaxed),
                    files_written: 0,
                    files_hashed: 0,
                });
            }
        }
    }
    Ok(())
}

#[cfg(windows)]
fn attr_flags_for(meta: &std::fs::Metadata) -> AttrFlags {
    use std::os::windows::fs::MetadataExt;
    const FILE_ATTRIBUTE_HIDDEN: u32 = 0x2;
    const FILE_ATTRIBUTE_SYSTEM: u32 = 0x4;
    let mut a = AttrFlags::empty();
    let fa = meta.file_attributes();
    if fa & FILE_ATTRIBUTE_HIDDEN != 0 {
        a.set(AttrFlags::HIDDEN, true);
    }
    if fa & FILE_ATTRIBUTE_SYSTEM != 0 {
        a.set(AttrFlags::SYSTEM, true);
    }
    if meta.permissions().readonly() {
        a.set(AttrFlags::READONLY, true);
    }
    a
}

#[cfg(not(windows))]
fn attr_flags_for(meta: &std::fs::Metadata) -> AttrFlags {
    let mut a = AttrFlags::empty();
    if meta.permissions().readonly() {
        a.set(AttrFlags::READONLY, true);
    }
    a
}

fn read_stats(conn: &Connection) -> Result<ScanStats, rusqlite::Error> {
    let mut stats = ScanStats::default();
    let mut stmt = conn.prepare(
        "SELECT kind, COUNT(*), COALESCE(SUM(size), 0) FROM scan_items GROUP BY kind",
    )?;
    let rows = stmt.query_map([], |r| {
        Ok((r.get::<_, i64>(0)?, r.get::<_, i64>(1)?, r.get::<_, i64>(2)?))
    })?;
    for r in rows {
        let (k, c, b) = r?;
        let kind = EntryKind::from_i64(k);
        let count = c as u64;
        let bytes = b as u64;
        if matches!(kind, EntryKind::File) {
            stats.total_files = count;
            stats.total_bytes = bytes;
        }
        stats.by_kind.insert(kind, count);
    }
    Ok(stats)
}

fn set_meta(conn: &Connection, key: &str, value: &str) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO scan_meta (key, value) VALUES (?1, ?2)
           ON CONFLICT(key) DO UPDATE SET value=excluded.value",
        params![key, value],
    )?;
    Ok(())
}

fn finalize_status(
    conn: &Arc<Mutex<Connection>>,
    index_path: &Option<PathBuf>,
    scan_id: ScanId,
    status: ScanStatus,
) {
    {
        let guard = conn.lock().expect("scan conn poisoned");
        let _ = set_meta(&guard, "status", status.as_str());
    }
    if let Some(ip) = index_path
        && let Ok(idx) = index::open_index(ip)
    {
        let _ = index::update_status(&idx, scan_id, status);
    }
}

fn now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

fn default_scan_dir() -> Option<PathBuf> {
    directories::ProjectDirs::from("com", "CopyThat", "CopyThat2026")
        .map(|p| p.config_dir().join("scans"))
}

fn rusqlite_err(src: &Path, dst: &Path, e: rusqlite::Error) -> CopyError {
    CopyError {
        kind: CopyErrorKind::IoOther,
        src: src.to_path_buf(),
        dst: dst.to_path_buf(),
        raw_os_error: None,
        message: format!("scan db error: {e}"),
    }
}

fn schema_err(src: &Path, dst: &Path, e: schema::SchemaError) -> CopyError {
    CopyError {
        kind: CopyErrorKind::IoOther,
        src: src.to_path_buf(),
        dst: dst.to_path_buf(),
        raw_os_error: None,
        message: format!("scan schema error: {e}"),
    }
}

fn other_err(src: &Path, msg: &str) -> CopyError {
    CopyError {
        kind: CopyErrorKind::IoOther,
        src: src.to_path_buf(),
        dst: src.to_path_buf(),
        raw_os_error: None,
        message: msg.to_string(),
    }
}

