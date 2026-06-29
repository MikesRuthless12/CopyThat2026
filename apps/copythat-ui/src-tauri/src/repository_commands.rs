//! Phase 49 — Tauri IPC for the unified content-addressed chunk
//! [`Repository`](copythat_chunk::Repository), surfaced by the Library
//! tab.
//!
//! Two read-only commands:
//!
//! - `repository_stats()` — the dedup hero numbers (bytes stored vs
//!   effective, distinct chunks, snapshot count, saved ratio).
//! - `repository_snapshots()` — the unified snapshot timeline
//!   (copy / sync / version / backup), oldest first.
//!
//! Each command opens the default repository **transiently** for the
//! duration of one read, then drops it. We deliberately do NOT cache a
//! process-lifetime handle: redb takes an exclusive file lock, and the
//! recovery web UI + mount features open the same default store on demand
//! — a persistent handle would block them. An open failure (no catalog
//! yet, or momentarily locked by another feature) surfaces as the typed
//! `"repository-unavailable"` string the Library tab keys its empty state
//! on. The reads are blocking, so each command hops to a `spawn_blocking`
//! worker rather than stalling the async runtime.

use serde::Serialize;

/// Wire shape for [`copythat_chunk::RepoStats`] plus the derived
/// `saved_ratio` (so the front end doesn't recompute it).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoStatsDto {
    /// Physical bytes on disk across pack files.
    pub stored_bytes: u64,
    /// Deduplicated logical size (sum of distinct chunk lengths).
    pub unique_bytes: u64,
    /// Sum of logical file sizes across every snapshot.
    pub effective_bytes: u64,
    /// Number of snapshots in the catalog.
    pub snapshot_count: u64,
    /// Number of distinct chunks indexed.
    pub chunk_count: u64,
    /// Fraction of effective bytes saved by dedup, in `0.0..=1.0`.
    pub saved_ratio: f64,
}

/// Wire shape for [`copythat_chunk::UnifiedSnapshot`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoSnapshotDto {
    /// Monotonic snapshot id.
    pub id: u64,
    /// `"copy"` / `"sync"` / `"version"` / `"backup"`.
    pub kind: String,
    /// Capture time, milliseconds since the Unix epoch.
    pub created_at_ms: i64,
    /// Human-readable label.
    pub label: String,
    /// Number of files captured.
    pub file_count: u64,
    /// Sum of logical file sizes (effective bytes this snapshot holds).
    pub total_size: u64,
}

/// Open the default repository transiently, mapping any open failure to
/// the typed `"repository-unavailable"` string (so the Library tab shows
/// its empty/unavailable state rather than a raw error). Must be called
/// from inside `spawn_blocking` — the open + reads are blocking.
fn open_repository() -> Result<copythat_chunk::Repository, String> {
    copythat_chunk::Repository::open_default().map_err(|_| "repository-unavailable".to_string())
}

/// `repository_stats()` — the dedup overview for the Library header
/// "hero" readout.
#[tauri::command]
pub async fn repository_stats() -> Result<RepoStatsDto, String> {
    let stats = tokio::task::spawn_blocking(|| {
        let repo = open_repository()?;
        repo.stats().map_err(|e| format!("repository stats: {e}"))
    })
    .await
    .map_err(|e| format!("repository stats task: {e}"))??;
    Ok(RepoStatsDto {
        stored_bytes: stats.stored_bytes,
        unique_bytes: stats.unique_bytes,
        effective_bytes: stats.effective_bytes,
        snapshot_count: stats.snapshot_count,
        chunk_count: stats.chunk_count,
        saved_ratio: stats.saved_ratio(),
    })
}

/// `repository_snapshots()` — the unified snapshot timeline, oldest
/// first. Reads the lightweight summaries index (no chunk manifests).
#[tauri::command]
pub async fn repository_snapshots() -> Result<Vec<RepoSnapshotDto>, String> {
    let snaps = tokio::task::spawn_blocking(|| {
        let repo = open_repository()?;
        repo.snapshots()
            .map_err(|e| format!("repository snapshots: {e}"))
    })
    .await
    .map_err(|e| format!("repository snapshots task: {e}"))??;
    Ok(snaps
        .into_iter()
        .map(|s| RepoSnapshotDto {
            id: s.id,
            kind: s.kind.as_str().to_string(),
            created_at_ms: s.created_at_ms,
            label: s.label,
            file_count: s.file_count,
            total_size: s.total_size,
        })
        .collect())
}
