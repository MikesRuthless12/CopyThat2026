//! `copythat-history` — SQLite persistence for completed jobs.
//!
//! The history is append-only in intent: each `copy_file` / `copy_tree`
//! / `move_file` / `move_tree` invocation produces one `jobs` row and
//! one `items` row per file touched (success or failure). Phase 9's
//! UI "History" tab reads from this store; Phase 10 computes lifetime
//! statistics over the same tables.
//!
//! # Dependency contract
//!
//! - **Bundled SQLite** (`rusqlite = { features = ["bundled"] }`).
//!   The Tauri app ships a single binary on every platform; no
//!   system libsqlite3 requirement.
//! - **Hand-rolled migrator** (`PRAGMA user_version`). One crate
//!   boundary is enough; pulling `refinery` would double the
//!   dependency cost for two migrations.
//! - **`tokio::task::spawn_blocking`** for every async call.
//!   rusqlite is sync; blocking threads keep the tokio runtime
//!   scheduling other work while SQLite holds the database lock.
//!
//! # Public API sketch
//!
//! ```no_run
//! use copythat_history::{History, JobSummary, ItemRow, HistoryFilter};
//! use std::path::PathBuf;
//!
//! # async fn demo() -> Result<(), copythat_history::HistoryError> {
//! let history = History::open_default().await?;
//! let row_id = history.record_start(&JobSummary {
//!     kind: "copy".into(),
//!     status: "running".into(),
//!     started_at_ms: 0,
//!     finished_at_ms: None,
//!     src_root: std::path::PathBuf::from("/a"),
//!     dst_root: std::path::PathBuf::from("/b"),
//!     total_bytes: 0,
//!     files_ok: 0,
//!     files_failed: 0,
//!     verify_algo: None,
//!     options_json: None,
//!     row_id: 0,
//! }).await?;
//!
//! history.record_item(&ItemRow {
//!     job_row_id: row_id.as_i64(),
//!     src: "/a/x".into(),
//!     dst: "/b/x".into(),
//!     size: 1024,
//!     status: "ok".into(),
//!     hash_hex: None,
//!     error_code: None,
//!     error_msg: None,
//!     timestamp_ms: 0,
//! }).await?;
//!
//! history.record_finish(row_id, "succeeded", 1024, 1, 0).await?;
//!
//! let jobs = history.search(HistoryFilter::default()).await?;
//! println!("{} jobs recorded", jobs.len());
//! # Ok(()) }
//! ```

mod error;
mod export;
mod handle;
mod migrations;
mod types;

pub use error::HistoryError;
pub use export::export_csv;
pub use handle::History;
pub use types::{HistoryFilter, ItemRow, JobRowId, JobSummary};
