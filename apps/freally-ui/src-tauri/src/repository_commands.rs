//! Phase 49 — Tauri IPC for the unified content-addressed chunk
//! [`Repository`](freally_chunk::Repository), surfaced by the Library
//! tab.
//!
//! Two read-only commands:
//!
//! - `repository_stats()` — the dedup hero numbers (bytes stored vs
//!   effective, distinct chunks, snapshot count, saved ratio).
//! - `repository_snapshots()` — the unified snapshot timeline
//!   (copy / sync / version / backup), oldest first.
//!
//! Both commands read the **shared** `Arc<Repository>` opened once at
//! startup and held in [`AppState`] (Phase 49b). Holding one handle is
//! what makes a process-lifetime open safe: redb takes an exclusive file
//! lock on `index.redb`, so the recovery web UI + mount features now share
//! this same handle instead of racing a second `open`. When the startup
//! open failed (no chunk store, or momentarily locked), `state.repository`
//! is `None` and the commands return the typed `"repository-unavailable"`
//! string the Library tab keys its empty state on. The reads are blocking,
//! so each command hops to a `spawn_blocking` worker rather than stalling
//! the async runtime.

use serde::Serialize;

use crate::state::AppState;

/// Wire shape for [`freally_chunk::RepoStats`] plus the derived
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
    /// Phase 49h — on-disk (stored) size of the distinct reachable chunks.
    pub physical_unique_bytes: u64,
    /// Phase 49h — fraction saved by compression, in `0.0..=1.0`.
    pub compression_ratio: f64,
}

/// Wire shape for [`freally_chunk::UnifiedSnapshot`].
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
    /// Phase 49p — pinned snapshots are protected from prune.
    pub pinned: bool,
    /// Phase 49p — user-editable description (empty if unset).
    pub description: String,
}

/// `repository_stats()` — the dedup overview for the Library header
/// "hero" readout.
#[tauri::command]
pub async fn repository_stats(state: tauri::State<'_, AppState>) -> Result<RepoStatsDto, String> {
    let repo = state.repository().ok_or("repository-unavailable")?;
    let stats = tokio::task::spawn_blocking(move || repo.stats())
        .await
        .map_err(|e| format!("repository stats task: {e}"))?
        .map_err(|e| format!("repository stats: {e}"))?;
    Ok(RepoStatsDto {
        stored_bytes: stats.stored_bytes,
        unique_bytes: stats.unique_bytes,
        effective_bytes: stats.effective_bytes,
        snapshot_count: stats.snapshot_count,
        chunk_count: stats.chunk_count,
        saved_ratio: stats.saved_ratio(),
        physical_unique_bytes: stats.physical_unique_bytes,
        compression_ratio: stats.compression_ratio(),
    })
}

/// `repository_snapshots()` — the unified snapshot timeline, oldest
/// first. Reads the lightweight summaries index (no chunk manifests).
#[tauri::command]
pub async fn repository_snapshots(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<RepoSnapshotDto>, String> {
    let repo = state.repository().ok_or("repository-unavailable")?;
    let snaps = tokio::task::spawn_blocking(move || repo.snapshots())
        .await
        .map_err(|e| format!("repository snapshots task: {e}"))?
        .map_err(|e| format!("repository snapshots: {e}"))?;
    Ok(snaps
        .into_iter()
        .map(|s| RepoSnapshotDto {
            id: s.id,
            kind: s.kind.as_str().to_string(),
            created_at_ms: s.created_at_ms,
            label: s.label,
            file_count: s.file_count,
            total_size: s.total_size,
            pinned: s.pinned,
            description: s.description,
        })
        .collect())
}

// ---------------------------------------------------------------------
// Phase 49o — snapshot diff / compare.
// ---------------------------------------------------------------------

/// One file's diff between two snapshots. Mirrors `freally_chunk::FileDiff`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileDiffDto {
    pub path: String,
    /// `"added"` / `"removed"` / `"modified"` / `"unchanged"`.
    pub change: String,
    pub old_size: Option<u64>,
    pub new_size: Option<u64>,
    pub chunks_shared: u64,
    pub chunks_changed: u64,
    pub bytes_added: u64,
}

/// Diff between two snapshots. Mirrors `freally_chunk::SnapshotDiff`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotDiffDto {
    pub from_id: u64,
    pub to_id: u64,
    pub files: Vec<FileDiffDto>,
    pub added: u64,
    pub removed: u64,
    pub modified: u64,
    pub unchanged: u64,
    pub bytes_added: u64,
}

fn change_str(c: freally_chunk::FileChange) -> &'static str {
    match c {
        freally_chunk::FileChange::Added => "added",
        freally_chunk::FileChange::Removed => "removed",
        freally_chunk::FileChange::Modified => "modified",
        freally_chunk::FileChange::Unchanged => "unchanged",
    }
}

/// `repository_diff` — compare two snapshots: per-file Added/Removed/
/// Modified/Unchanged plus the chunk-level incremental cost. Read-only
/// manifest math over the shared repository handle.
#[tauri::command]
pub async fn repository_diff(
    state: tauri::State<'_, AppState>,
    from_id: u64,
    to_id: u64,
) -> Result<SnapshotDiffDto, String> {
    let repo = state.repository().ok_or("repository-unavailable")?;
    let diff = tokio::task::spawn_blocking(move || {
        repo.diff_snapshots(
            freally_chunk::SnapshotId(from_id),
            freally_chunk::SnapshotId(to_id),
        )
    })
    .await
    .map_err(|e| format!("diff task: {e}"))?
    .map_err(|e| format!("diff: {e}"))?;
    Ok(SnapshotDiffDto {
        from_id: diff.from_id,
        to_id: diff.to_id,
        files: diff
            .files
            .into_iter()
            .map(|f| FileDiffDto {
                path: f.path,
                change: change_str(f.change).to_string(),
                old_size: f.old_size,
                new_size: f.new_size,
                chunks_shared: f.chunks_shared,
                chunks_changed: f.chunks_changed,
                bytes_added: f.bytes_added,
            })
            .collect(),
        added: diff.added,
        removed: diff.removed,
        modified: diff.modified,
        unchanged: diff.unchanged,
        bytes_added: diff.bytes_added,
    })
}

// ---------------------------------------------------------------------
// Phase 49r — statistics / report.
// ---------------------------------------------------------------------

/// Per-kind snapshot breakdown. Mirrors `freally_chunk::KindBreakdown`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KindBreakdownDto {
    pub kind: String,
    pub count: u64,
    pub effective_bytes: u64,
}

/// A growth-curve point. Mirrors `freally_chunk::GrowthPoint`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GrowthPointDto {
    pub ts_ms: i64,
    pub cumulative_unique_bytes: u64,
    pub snapshot_count: u64,
}

/// A top file. Mirrors `freally_chunk::TopFile`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopFileDto {
    pub path: String,
    pub versions: u64,
    pub max_size: u64,
}

/// The full repository report. Mirrors `freally_chunk::RepoReport`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoReportDto {
    pub stats: RepoStatsDto,
    pub by_kind: Vec<KindBreakdownDto>,
    pub growth: Vec<GrowthPointDto>,
    pub top_files: Vec<TopFileDto>,
    pub dedup_ratio: f64,
}

fn report_to_dto(r: freally_chunk::RepoReport) -> RepoReportDto {
    RepoReportDto {
        stats: RepoStatsDto {
            stored_bytes: r.stats.stored_bytes,
            unique_bytes: r.stats.unique_bytes,
            effective_bytes: r.stats.effective_bytes,
            snapshot_count: r.stats.snapshot_count,
            chunk_count: r.stats.chunk_count,
            saved_ratio: r.stats.saved_ratio(),
            physical_unique_bytes: r.stats.physical_unique_bytes,
            compression_ratio: r.stats.compression_ratio(),
        },
        by_kind: r
            .by_kind
            .into_iter()
            .map(|k| KindBreakdownDto {
                kind: k.kind.as_str().to_string(),
                count: k.count,
                effective_bytes: k.effective_bytes,
            })
            .collect(),
        growth: r
            .growth
            .into_iter()
            .map(|g| GrowthPointDto {
                ts_ms: g.ts_ms,
                cumulative_unique_bytes: g.cumulative_unique_bytes,
                snapshot_count: g.snapshot_count,
            })
            .collect(),
        top_files: r
            .top_files
            .into_iter()
            .map(|f| TopFileDto {
                path: f.path,
                versions: f.versions,
                max_size: f.max_size,
            })
            .collect(),
        dedup_ratio: r.dedup_ratio,
    }
}

/// `repository_report` — analytics (per-kind, growth, top files, dedup).
#[tauri::command]
pub async fn repository_report(
    state: tauri::State<'_, AppState>,
    top_n: usize,
) -> Result<RepoReportDto, String> {
    let repo = state.repository().ok_or("repository-unavailable")?;
    let report = tokio::task::spawn_blocking(move || repo.report(top_n))
        .await
        .map_err(|e| format!("report task: {e}"))?
        .map_err(|e| format!("report: {e}"))?;
    Ok(report_to_dto(report))
}

/// `repository_export_report` — write the report to `path` as `md` or
/// `json` (the front end supplies a save-dialog path).
#[tauri::command]
pub async fn repository_export_report(
    state: tauri::State<'_, AppState>,
    path: String,
    top_n: usize,
    format: String,
) -> Result<(), String> {
    let repo = state.repository().ok_or("repository-unavailable")?;
    let content = tokio::task::spawn_blocking(move || -> Result<String, String> {
        if format == "md" {
            repo.report_markdown(top_n).map_err(|e| e.to_string())
        } else {
            let r = repo.report(top_n).map_err(|e| e.to_string())?;
            serde_json::to_string_pretty(&r).map_err(|e| e.to_string())
        }
    })
    .await
    .map_err(|e| format!("export task: {e}"))??;
    std::fs::write(&path, content).map_err(|e| format!("write report: {e}"))
}

// ---------------------------------------------------------------------
// Phase 49p — snapshot pinning / metadata + pin-protected prune.
// ---------------------------------------------------------------------

/// `repository_set_pinned` — pin/unpin a snapshot (pinned survives prune).
#[tauri::command]
pub async fn repository_set_pinned(
    state: tauri::State<'_, AppState>,
    snapshot_id: u64,
    pinned: bool,
) -> Result<bool, String> {
    let repo = state.repository().ok_or("repository-unavailable")?;
    tokio::task::spawn_blocking(move || {
        repo.set_pinned(freally_chunk::SnapshotId(snapshot_id), pinned)
    })
    .await
    .map_err(|e| format!("set_pinned task: {e}"))?
    .map_err(|e| format!("set_pinned: {e}"))
}

/// `repository_set_label` — rename a snapshot.
#[tauri::command]
pub async fn repository_set_label(
    state: tauri::State<'_, AppState>,
    snapshot_id: u64,
    label: String,
) -> Result<bool, String> {
    let repo = state.repository().ok_or("repository-unavailable")?;
    tokio::task::spawn_blocking(move || {
        repo.set_label(freally_chunk::SnapshotId(snapshot_id), &label)
    })
    .await
    .map_err(|e| format!("set_label task: {e}"))?
    .map_err(|e| format!("set_label: {e}"))
}

/// `repository_set_description` — set a snapshot's description.
#[tauri::command]
pub async fn repository_set_description(
    state: tauri::State<'_, AppState>,
    snapshot_id: u64,
    description: String,
) -> Result<bool, String> {
    let repo = state.repository().ok_or("repository-unavailable")?;
    tokio::task::spawn_blocking(move || {
        repo.set_description(freally_chunk::SnapshotId(snapshot_id), &description)
    })
    .await
    .map_err(|e| format!("set_description task: {e}"))?
    .map_err(|e| format!("set_description: {e}"))
}

/// `repository_set_tags` — set a snapshot's tags.
#[tauri::command]
pub async fn repository_set_tags(
    state: tauri::State<'_, AppState>,
    snapshot_id: u64,
    tags: Vec<String>,
) -> Result<bool, String> {
    let repo = state.repository().ok_or("repository-unavailable")?;
    tokio::task::spawn_blocking(move || repo.set_tags(freally_chunk::SnapshotId(snapshot_id), tags))
        .await
        .map_err(|e| format!("set_tags task: {e}"))?
        .map_err(|e| format!("set_tags: {e}"))
}

/// `repository_prune_policy` — global keep-last / keep-within prune that
/// always protects pinned snapshots. Returns the removed ids (run
/// `repository_gc` afterwards to reclaim chunks).
#[tauri::command]
pub async fn repository_prune_policy(
    state: tauri::State<'_, AppState>,
    keep_last: Option<u64>,
    keep_within_ms: Option<i64>,
    now_ms: i64,
) -> Result<Vec<u64>, String> {
    let repo = state.repository().ok_or("repository-unavailable")?;
    let removed = tokio::task::spawn_blocking(move || {
        repo.prune(
            &freally_chunk::PrunePolicy {
                keep_last,
                keep_within_ms,
            },
            now_ms,
        )
    })
    .await
    .map_err(|e| format!("prune task: {e}"))?
    .map_err(|e| format!("prune: {e}"))?;
    Ok(removed.into_iter().map(|s| s.as_u64()).collect())
}

// ---------------------------------------------------------------------
// Phase 49h — at-rest compression policy (persisted; applied at open).
// ---------------------------------------------------------------------

/// Read the persisted at-rest compression policy (the settings mirror).
#[tauri::command]
pub fn repository_compression_get(
    state: tauri::State<'_, AppState>,
) -> freally_settings::RepoCompressionSettings {
    state.settings_snapshot().chunk_store.compression
}

/// Set + persist the at-rest compression policy. Takes effect on the next
/// launch — the open repository's policy is fixed when it's opened.
#[tauri::command]
pub fn repository_compression_set(
    state: tauri::State<'_, AppState>,
    compression: freally_settings::RepoCompressionSettings,
) -> Result<(), String> {
    let mut guard = state.settings.write().map_err(|e| e.to_string())?;
    guard.chunk_store.compression = compression;
    let path = state.settings_path.as_path();
    if path.as_os_str().is_empty() {
        return Ok(());
    }
    guard
        .save_to(path)
        .map_err(|e| format!("save settings: {e}"))
}

/// Phase 49i — full compaction (quick gc + rewrite half-dead packs). Spawns
/// a 49j task, runs the blocking work off the async runtime with a progress
/// callback, and returns the task id immediately so the UI can watch it in
/// the Tasks center. Cancellable via `task_cancel`.
#[tauri::command]
pub async fn repository_compact(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
) -> Result<u64, String> {
    use std::sync::atomic::Ordering;

    let repo = state.repository().ok_or("repository-unavailable")?;
    let tasks = state.tasks.clone();
    let task = tasks.create("compact", "Full compaction");
    tasks.started(&app, task);
    let task_id = task.0;
    let cancel = tasks.cancel_flag(task);

    tokio::task::spawn_blocking(move || {
        let cancel_fn = || cancel.load(Ordering::Relaxed);
        let mut cb = |p: freally_chunk::MaintenanceProgress| {
            let frac = if p.total > 0 {
                p.done as f32 / p.total as f32
            } else {
                0.0
            };
            tasks.update(
                &app,
                task,
                frac,
                &format!("{} {}/{}", p.phase, p.done, p.total),
            );
        };
        match repo.compact(
            freally_chunk::CompactOptions::default(),
            &cancel_fn,
            &mut cb,
        ) {
            Ok((gc, cr)) => {
                if cancel_fn() {
                    tasks.cancelled(&app, task, "cancelled");
                } else {
                    let reclaimed = gc.bytes_reclaimed + cr.bytes_reclaimed;
                    tasks.complete(
                        &app,
                        task,
                        &format!(
                            "reclaimed {reclaimed} B · {} packs compacted · {} chunks swept",
                            cr.packs_compacted, gc.chunks_swept
                        ),
                    );
                }
            }
            Err(e) => tasks.fail(&app, task, &e.to_string()),
        }
    });

    Ok(task_id)
}

// ---------------------------------------------------------------------
// Phase 49k — multi-repository management (the "Repository" screen).
// ---------------------------------------------------------------------

/// One registered repository, for the wizard + switcher.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoEntryDto {
    pub id: String,
    pub name: String,
    pub path: String,
    pub created_at_ms: i64,
    pub active: bool,
    pub requires_passphrase: bool,
}

fn repo_entry_dto(e: &freally_settings::RepoEntry, active: &str) -> RepoEntryDto {
    RepoEntryDto {
        id: e.id.clone(),
        name: e.name.clone(),
        path: e.path.clone(),
        created_at_ms: e.created_at_ms,
        active: e.id == active,
        requires_passphrase: freally_chunk::Repository::requires_passphrase(std::path::Path::new(
            &e.path,
        )),
    }
}

/// Persist `settings` (no-op when the path is empty, e.g. tests).
fn persist_settings(state: &AppState, settings: &freally_settings::Settings) -> Result<(), String> {
    let path = state.settings_path.as_path();
    if path.as_os_str().is_empty() {
        return Ok(());
    }
    settings
        .save_to(path)
        .map_err(|e| format!("save settings: {e}"))
}

/// Register a `RepoEntry`, swap the live handle to `repo`, persist. Shared
/// tail of create/connect.
/// Open the repository at `path`, REUSING an already-open handle when `path`
/// resolves to the startup-default or the currently-active repository. redb
/// takes a mandatory exclusive file lock, so a second `open` of an already-open
/// path fails with "database already open" — re-selecting the active repo, or
/// connecting to the default chunk-store path, would otherwise error out. A
/// genuinely fresh open also gets the persisted at-rest compression policy
/// applied (Phase 49h), matching the startup path.
fn open_or_reuse(
    state: &AppState,
    path: &str,
    password: Option<&str>,
) -> Result<std::sync::Arc<freally_chunk::Repository>, String> {
    let canon = |p: &std::path::Path| std::fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf());
    let want = canon(std::path::Path::new(path));
    if let Some(def) = state.default_repository() {
        if canon(def.root()) == want {
            return Ok(def);
        }
    }
    if let Some(active) = state.repository() {
        if canon(active.root()) == want {
            return Ok(active);
        }
    }
    let mut repo = freally_chunk::Repository::open_existing(std::path::Path::new(path), password)
        .map_err(|e| e.to_string())?;
    repo.set_compression(crate::compression_from_settings(
        &state.settings_snapshot().chunk_store.compression,
    ));
    Ok(std::sync::Arc::new(repo))
}

fn register_and_activate(
    state: &AppState,
    name: String,
    path: String,
    repo: std::sync::Arc<freally_chunk::Repository>,
) -> Result<RepoEntryDto, String> {
    let entry = freally_settings::RepoEntry {
        id: uuid::Uuid::new_v4().to_string(),
        name,
        path,
        created_at_ms: chrono::Utc::now().timestamp_millis(),
    };
    state.set_repository(Some(repo));
    let mut guard = state.settings.write().map_err(|e| e.to_string())?;
    guard.repository.repos.push(entry.clone());
    guard.repository.active = entry.id.clone();
    let active = guard.repository.active.clone();
    persist_settings(state, &guard)?;
    Ok(repo_entry_dto(&entry, &active))
}

/// The registered repositories.
#[tauri::command]
pub fn repository_list(state: tauri::State<'_, AppState>) -> Vec<RepoEntryDto> {
    let s = state.settings_snapshot();
    s.repository
        .repos
        .iter()
        .map(|e| repo_entry_dto(e, &s.repository.active))
        .collect()
}

/// The active repository, if one is registered + selected.
#[tauri::command]
pub fn repository_active(state: tauri::State<'_, AppState>) -> Option<RepoEntryDto> {
    let s = state.settings_snapshot();
    s.repository
        .repos
        .iter()
        .find(|e| e.id == s.repository.active)
        .map(|e| repo_entry_dto(e, &s.repository.active))
}

/// Create a NEW repository at `path`, register it, and make it active.
#[tauri::command]
pub fn repository_create(
    state: tauri::State<'_, AppState>,
    name: String,
    path: String,
    password: Option<String>,
) -> Result<RepoEntryDto, String> {
    let mut repo =
        freally_chunk::Repository::create(std::path::Path::new(&path), password.as_deref())
            .map_err(|e| e.to_string())?;
    // Apply the persisted at-rest compression policy (Phase 49h), matching the
    // startup path — otherwise a repo created after launch silently defaults to
    // Off while the UI still reports the configured policy.
    repo.set_compression(crate::compression_from_settings(
        &state.settings_snapshot().chunk_store.compression,
    ));
    register_and_activate(&state, name, path, std::sync::Arc::new(repo))
}

/// Connect to an EXISTING repository at `path`, register it, make it active.
#[tauri::command]
pub fn repository_connect(
    state: tauri::State<'_, AppState>,
    name: String,
    path: String,
    password: Option<String>,
) -> Result<RepoEntryDto, String> {
    let repo = open_or_reuse(&state, &path, password.as_deref())?;
    register_and_activate(&state, name, path, repo)
}

/// Switch the active repository. `id == ""` restores the startup default.
#[tauri::command]
pub fn repository_set_active(
    state: tauri::State<'_, AppState>,
    id: String,
    password: Option<String>,
) -> Result<Option<RepoEntryDto>, String> {
    if id.is_empty() {
        state.set_repository(state.default_repository());
        let mut guard = state.settings.write().map_err(|e| e.to_string())?;
        guard.repository.active = String::new();
        persist_settings(&state, &guard)?;
        return Ok(None);
    }
    let entry = state
        .settings_snapshot()
        .repository
        .repos
        .iter()
        .find(|e| e.id == id)
        .cloned()
        .ok_or("repository not found")?;
    let repo = open_or_reuse(&state, &entry.path, password.as_deref())?;
    state.set_repository(Some(repo));
    let mut guard = state.settings.write().map_err(|e| e.to_string())?;
    guard.repository.active = id.clone();
    persist_settings(&state, &guard)?;
    Ok(Some(repo_entry_dto(&entry, &id)))
}

/// Remove a repository from the list (data on disk is untouched). If it was
/// active, fall back to the startup default. (Named `disconnect` to avoid
/// the existing snapshot-`forget` command.)
#[tauri::command]
pub fn repository_disconnect(state: tauri::State<'_, AppState>, id: String) -> Result<(), String> {
    let mut guard = state.settings.write().map_err(|e| e.to_string())?;
    guard.repository.repos.retain(|e| e.id != id);
    if guard.repository.active == id {
        guard.repository.active = String::new();
        state.set_repository(state.default_repository());
    }
    persist_settings(&state, &guard)
}

/// Rotate the active repository's access passphrase (Phase 49k gate).
#[tauri::command]
pub fn repository_change_password(
    state: tauri::State<'_, AppState>,
    old: Option<String>,
    new: String,
) -> Result<(), String> {
    let repo = state.repository().ok_or("repository-unavailable")?;
    repo.change_password(old.as_deref(), &new)
        .map_err(|e| e.to_string())
}

// ---------------------------------------------------------------------
// Phase 49l — Sources dashboard (one row per source, summaries only).
// ---------------------------------------------------------------------

/// One source-dashboard row. Mirrors `freally_chunk::SourceSummary`.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoSourceDto {
    pub source: String,
    pub snapshot_count: u64,
    pub latest_ms: i64,
    pub latest_kind: String,
    pub latest_size: u64,
    pub total_files: u64,
}

#[tauri::command]
pub async fn repository_sources(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<RepoSourceDto>, String> {
    let repo = state.repository().ok_or("repository-unavailable")?;
    let sources = tokio::task::spawn_blocking(move || repo.sources())
        .await
        .map_err(|e| format!("sources task: {e}"))?
        .map_err(|e| format!("sources: {e}"))?;
    Ok(sources
        .into_iter()
        .map(|s| RepoSourceDto {
            source: s.source,
            snapshot_count: s.snapshot_count,
            latest_ms: s.latest_ms,
            latest_kind: s.latest_kind.as_str().to_string(),
            latest_size: s.latest_size,
            total_files: s.total_files,
        })
        .collect())
}

// ---------------------------------------------------------------------
// Phase 49n — snapshot verification & repair.
// ---------------------------------------------------------------------

/// One damaged reference. Mirrors `freally_chunk::VerifyDamage`.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyDamageDto {
    pub snapshot_id: u64,
    pub path: String,
    pub chunk_hash_hex: String,
    /// `"missing"` | `"corrupt"` | `"file-hash-mismatch"`.
    pub kind: String,
}

/// Outcome of a verify pass. Mirrors `freally_chunk::VerifyReport`.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyReportDto {
    pub snapshots_checked: u64,
    pub files_checked: u64,
    pub chunks_checked: u64,
    pub is_clean: bool,
    pub missing: u64,
    pub corrupt: u64,
    pub damage: Vec<VerifyDamageDto>,
}

fn verify_report_dto(r: freally_chunk::VerifyReport) -> VerifyReportDto {
    use freally_chunk::DamageKind;
    let missing = r
        .damage
        .iter()
        .filter(|d| d.kind == DamageKind::Missing)
        .count() as u64;
    let corrupt = r
        .damage
        .iter()
        .filter(|d| matches!(d.kind, DamageKind::Corrupt | DamageKind::FileHashMismatch))
        .count() as u64;
    VerifyReportDto {
        snapshots_checked: r.snapshots_checked,
        files_checked: r.files_checked,
        chunks_checked: r.chunks_checked,
        is_clean: r.is_clean(),
        missing,
        corrupt,
        damage: r
            .damage
            .into_iter()
            .map(|d| VerifyDamageDto {
                snapshot_id: d.snapshot_id,
                path: d.path,
                chunk_hash_hex: d.chunk_hash_hex,
                kind: match d.kind {
                    DamageKind::Missing => "missing",
                    DamageKind::Corrupt => "corrupt",
                    DamageKind::FileHashMismatch => "file-hash-mismatch",
                }
                .to_string(),
            })
            .collect(),
    }
}

/// Verify the repository (`deep` = read + re-hash every chunk, else
/// index-only). Runs off the async runtime; `snapshot_id` limits to one.
#[tauri::command]
pub async fn repository_verify(
    state: tauri::State<'_, AppState>,
    snapshot_id: Option<u64>,
    deep: bool,
) -> Result<VerifyReportDto, String> {
    let repo = state.repository().ok_or("repository-unavailable")?;
    let level = if deep {
        freally_chunk::VerifyLevel::ReadData
    } else {
        freally_chunk::VerifyLevel::Metadata
    };
    let only = snapshot_id.map(freally_chunk::SnapshotId);
    let report = tokio::task::spawn_blocking(move || repo.verify(only, level))
        .await
        .map_err(|e| format!("verify task: {e}"))?
        .map_err(|e| format!("verify: {e}"))?;
    Ok(verify_report_dto(report))
}

/// Repair = quarantine: verify, then remove every damaged snapshot + gc.
/// `apply == false` is a dry run (reports the would-remove ids only).
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RepairReportDto {
    pub removed_ids: Vec<u64>,
    pub bytes_reclaimed: u64,
    pub applied: bool,
}

#[tauri::command]
pub async fn repository_repair(
    state: tauri::State<'_, AppState>,
    deep: bool,
    apply: bool,
) -> Result<RepairReportDto, String> {
    let repo = state.repository().ok_or("repository-unavailable")?;
    let level = if deep {
        freally_chunk::VerifyLevel::ReadData
    } else {
        freally_chunk::VerifyLevel::Metadata
    };
    let (removed, gc) = tokio::task::spawn_blocking(move || {
        let report = repo.verify(None, level)?;
        repo.repair_remove_damaged(&report, apply)
    })
    .await
    .map_err(|e| format!("repair task: {e}"))?
    .map_err(|e| format!("repair: {e}"))?;
    Ok(RepairReportDto {
        removed_ids: removed.into_iter().map(|s| s.0).collect(),
        bytes_reclaimed: gc.bytes_reclaimed,
        applied: apply,
    })
}

/// Phase 49b — the previously-deferred Phase 42 snapshot-on-overwrite
/// hook, realised against the unified [`Repository`](freally_chunk::Repository).
///
/// The engine calls
/// [`snapshot_before_overwrite`](freally_core::VersioningSink::snapshot_before_overwrite)
/// once per file — AFTER the collision policy resolves to "overwrite" but
/// BEFORE the destination is truncated — and already wraps the call in
/// `spawn_blocking`, so this impl does its (blocking) chunk-store work
/// directly. We capture the about-to-be-clobbered destination as a
/// `Version` snapshot so the Library timeline can roll a file back to a
/// prior state. Best-effort: a failure returns `Err` and the engine
/// proceeds with the copy (the snapshot is lost; the data is not).
#[derive(Debug)]
pub struct RepositoryVersioningSink {
    repo: std::sync::Arc<freally_chunk::Repository>,
}

impl RepositoryVersioningSink {
    /// Wrap the shared repository handle.
    #[must_use]
    pub fn new(repo: std::sync::Arc<freally_chunk::Repository>) -> Self {
        Self { repo }
    }
}

impl freally_core::VersioningSink for RepositoryVersioningSink {
    fn snapshot_before_overwrite(
        &self,
        dst: &std::path::Path,
        _triggered_by_job_id: Option<i64>,
    ) -> Result<bool, String> {
        // The engine already confirmed dst exists; re-check so a direct
        // caller can't trip a spurious read.
        if !dst.exists() {
            return Ok(false);
        }
        let label = dst.to_string_lossy().into_owned();
        let now = chrono::Utc::now().timestamp_millis();
        self.repo
            .snapshot_files(
                freally_chunk::SnapshotKind::Version,
                &label,
                now,
                &[(label.as_str(), dst)],
            )
            .map(|_| true)
            .map_err(|e| e.to_string())
    }
}

// ---------------------------------------------------------------------
// Phase 49d — restore browser.
// ---------------------------------------------------------------------

/// One file in a snapshot's flat tree (path + logical size).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotFileDto {
    pub path: String,
    pub size: u64,
}

/// Per-file existence check that drives the restore conflict step.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestoreConflictDto {
    pub path: String,
    pub exists: bool,
}

/// Tally of a restore run.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestoreReportDto {
    pub restored: u64,
    pub skipped: u64,
    pub failed: u64,
}

/// Wire shape for [`freally_chunk::GcReport`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GcReportDto {
    pub chunks_swept: u64,
    pub packs_removed: u64,
    pub bytes_reclaimed: u64,
}

/// Map the wire conflict string to the chunk-layer policy. Only the three
/// explicit choices act; anything unrecognised (a front-end typo, an
/// outdated webview) falls back to the SAFE non-destructive `Skip` rather
/// than silently overwriting the user's existing files.
fn parse_conflict(s: &str) -> freally_chunk::RestoreConflict {
    match s {
        "overwrite" => freally_chunk::RestoreConflict::Overwrite,
        "keep-both" => freally_chunk::RestoreConflict::KeepBoth,
        _ => freally_chunk::RestoreConflict::Skip,
    }
}

/// The strip-prefix that relativises a snapshot's logical paths for restore.
/// Copy / sync / version snapshots key files by their ABSOLUTE path;
/// restoring those verbatim would recreate the whole original tree under the
/// chosen destination. Stripping the deepest common directory of the
/// selected ABSOLUTE keys instead restores structure relative to the
/// copy/source root (a single file flattens to its name). Backup snapshots
/// already use source-relative keys → `None` (used as-is).
fn restore_strip_prefix(paths: &[String]) -> Option<String> {
    let first = paths.first()?;
    let win_abs = |s: &str| {
        let b = s.as_bytes();
        b.len() >= 2 && b[0].is_ascii_alphabetic() && b[1] == b':'
    };
    if !(first.starts_with('/') || first.starts_with('\\') || win_abs(first)) {
        return None; // relative keys (backup) — keep their subtree as-is
    }
    let norm: Vec<String> = paths.iter().map(|p| p.replace('\\', "/")).collect();
    // Common prefix over directory components (exclude each path's file name).
    let mut common: Vec<&str> = norm[0].split('/').collect();
    common.pop();
    for p in &norm[1..] {
        let parts: Vec<&str> = p.split('/').collect();
        let upto = parts.len().saturating_sub(1);
        let mut i = 0;
        while i < common.len() && i < upto && common[i] == parts[i] {
            i += 1;
        }
        common.truncate(i);
        if common.is_empty() {
            break;
        }
    }
    if common.is_empty() {
        None
    } else {
        Some(common.join("/"))
    }
}

/// `snapshot_tree` — flat file listing of a snapshot for the restore tree.
#[tauri::command]
pub async fn snapshot_tree(
    state: tauri::State<'_, AppState>,
    snapshot_id: u64,
) -> Result<Vec<SnapshotFileDto>, String> {
    let repo = state.repository().ok_or("repository-unavailable")?;
    let entries = tokio::task::spawn_blocking(move || {
        repo.snapshot_tree(freally_chunk::SnapshotId(snapshot_id))
    })
    .await
    .map_err(|e| format!("snapshot tree task: {e}"))?
    .map_err(|e| format!("snapshot tree: {e}"))?;
    Ok(entries
        .into_iter()
        .map(|e| SnapshotFileDto {
            path: e.path,
            size: e.size,
        })
        .collect())
}

/// `restore_preview` — which selected files already exist at the target.
#[tauri::command]
pub async fn restore_preview(
    state: tauri::State<'_, AppState>,
    snapshot_id: u64,
    paths: Vec<String>,
    dst_root: String,
) -> Result<Vec<RestoreConflictDto>, String> {
    let repo = state.repository().ok_or("repository-unavailable")?;
    let dst = std::path::PathBuf::from(dst_root);
    let strip = restore_strip_prefix(&paths);
    let pairs = tokio::task::spawn_blocking(move || {
        let refs: Vec<&str> = paths.iter().map(String::as_str).collect();
        repo.restore_preview(
            freally_chunk::SnapshotId(snapshot_id),
            &refs,
            &dst,
            strip.as_deref(),
        )
    })
    .await
    .map_err(|e| format!("restore preview task: {e}"))?
    .map_err(|e| format!("restore preview: {e}"))?;
    Ok(pairs
        .into_iter()
        .map(|(path, exists)| RestoreConflictDto { path, exists })
        .collect())
}

/// `restore_paths` — restore the selected files under `dst_root`.
#[tauri::command]
pub async fn restore_paths(
    state: tauri::State<'_, AppState>,
    snapshot_id: u64,
    paths: Vec<String>,
    dst_root: String,
    conflict: String,
) -> Result<RestoreReportDto, String> {
    let repo = state.repository().ok_or("repository-unavailable")?;
    let dst = std::path::PathBuf::from(dst_root);
    let policy = parse_conflict(&conflict);
    let strip = restore_strip_prefix(&paths);
    let report = tokio::task::spawn_blocking(move || {
        let refs: Vec<&str> = paths.iter().map(String::as_str).collect();
        repo.restore_paths(
            freally_chunk::SnapshotId(snapshot_id),
            &refs,
            &dst,
            strip.as_deref(),
            policy,
        )
    })
    .await
    .map_err(|e| format!("restore task: {e}"))?
    .map_err(|e| format!("restore: {e}"))?;
    Ok(RestoreReportDto {
        restored: report.restored,
        skipped: report.skipped,
        failed: report.failed,
    })
}

/// `repository_forget` — drop a snapshot from the catalog. Chunk bytes are
/// reclaimed by a later `repository_gc`, not here.
#[tauri::command]
pub async fn repository_forget(
    state: tauri::State<'_, AppState>,
    snapshot_id: u64,
) -> Result<bool, String> {
    let repo = state.repository().ok_or("repository-unavailable")?;
    tokio::task::spawn_blocking(move || {
        repo.remove_snapshot(freally_chunk::SnapshotId(snapshot_id))
    })
    .await
    .map_err(|e| format!("forget task: {e}"))?
    .map_err(|e| format!("forget: {e}"))
}

/// `repository_gc` — mark-and-sweep unreferenced chunks; reclaim space.
#[tauri::command]
pub async fn repository_gc(state: tauri::State<'_, AppState>) -> Result<GcReportDto, String> {
    let repo = state.repository().ok_or("repository-unavailable")?;
    let report = tokio::task::spawn_blocking(move || repo.gc())
        .await
        .map_err(|e| format!("gc task: {e}"))?
        .map_err(|e| format!("gc: {e}"))?;
    Ok(GcReportDto {
        chunks_swept: report.chunks_swept,
        packs_removed: report.packs_removed,
        bytes_reclaimed: report.bytes_reclaimed,
    })
}
