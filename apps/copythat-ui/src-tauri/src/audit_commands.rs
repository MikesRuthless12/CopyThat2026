//! Phase 34 — Tauri-side audit-log integration.
//!
//! Owns the [`AuditRegistry`] stored inside
//! [`crate::state::AppState::audit`]. The registry holds
//! `Arc<AuditSink>` when the user has audit logging enabled in
//! Settings → Advanced → Audit log; clone-cheap, serialised via
//! its own internal mutex. The runner calls the `record_*` helpers
//! once per lifecycle transition so the audit log stays in lockstep
//! with the user-visible queue state.
//!
//! All helpers are best-effort: a failing `sink.record(...)` logs to
//! stderr and returns. The audit log's purpose is *durability* for
//! later review — dropping an event would be bad, but throwing in
//! the middle of a copy loop would be worse. This matches the
//! behaviour of the Phase 20 journal + the Phase 9 history sink.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use crate::ipc_safety::{err_string, validate_ipc_path};
use chrono::Utc;
use copythat_audit::{
    AuditError, AuditEvent, AuditFormat, AuditSink, RotationPolicy, WormMode, is_worm_supported,
};
use copythat_settings::AuditSettings;
use serde::Serialize;
use sha2::Digest;

/// In-process owner of the live audit sink. Clone-cheap; the
/// underlying `Option<Arc<AuditSink>>` is guarded by a `Mutex` so
/// `update_settings` can swap the sink atomically against a
/// concurrent `record_*` from the runner.
#[derive(Clone, Default)]
pub struct AuditRegistry {
    inner: Arc<Mutex<Option<Arc<AuditSink>>>>,
}

impl AuditRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Replace (or clear) the current sink. `None` disables audit
    /// logging; the previous sink is dropped — its file handle
    /// closes, WORM attribute is preserved.
    pub fn set(&self, sink: Option<Arc<AuditSink>>) {
        let mut guard = self.inner.lock().expect("audit registry poisoned");
        *guard = sink;
    }

    /// Snapshot the current sink for recording. Returns `None`
    /// when audit is disabled; the caller short-circuits.
    pub fn snapshot(&self) -> Option<Arc<AuditSink>> {
        self.inner.lock().expect("audit registry poisoned").clone()
    }

    /// Active or not. Used by IPC + smoke tests.
    pub fn is_active(&self) -> bool {
        self.snapshot().is_some()
    }

    /// Append an event via the current sink if one exists. Swallows
    /// per-record IO errors after logging to stderr — callers in
    /// the hot copy path never observe audit failures.
    pub fn record(&self, event: &AuditEvent) {
        if let Some(sink) = self.snapshot()
            && let Err(e) = sink.record(event)
        {
            eprintln!("[audit] record {} failed: {e}", event.signature());
        }
    }
}

/// Derive the user-facing display value. Falls back to the OS user
/// token on Unix / Windows, then to "unknown".
pub fn current_user() -> String {
    std::env::var("USER")
        .ok()
        .or_else(|| std::env::var("USERNAME").ok())
        .unwrap_or_else(|| "unknown".into())
}

/// Current host name, or "localhost" when the OS refuses to
/// supply one.
pub fn current_host() -> String {
    gethostname::gethostname()
        .to_string_lossy()
        .trim()
        .to_string()
}

/// Translate the persisted Settings struct into the typed audit
/// crate inputs. Unknown enum string values degrade to the defaults
/// (`JsonLines` / `Off`) so a hand-edited TOML can't crash the
/// loader.
fn parse_audit_settings(cfg: &AuditSettings) -> (AuditFormat, WormMode, RotationPolicy, PathBuf) {
    let format = AuditFormat::parse(&cfg.format).unwrap_or_default();
    let worm = match cfg.worm.to_ascii_lowercase().as_str() {
        "on" => WormMode::On,
        _ => WormMode::Off,
    };
    let rotation = RotationPolicy {
        max_size: cfg.max_size_bytes,
    };
    let path = if cfg.file_path.is_empty() {
        default_audit_path()
    } else {
        PathBuf::from(&cfg.file_path)
    };
    (format, worm, rotation, path)
}

/// `<config-dir>/audit/copythat-audit.log`. Mirrors the pattern
/// the Phase 9 history / Phase 28 dropstack persistence use.
pub fn default_audit_path() -> PathBuf {
    if let Some(dirs) = directories::ProjectDirs::from("com", "CopyThat", "CopyThat2026") {
        return dirs.config_dir().join("audit").join("copythat-audit.log");
    }
    // Sandboxed / no-home fallback keeps the sink pathable even in
    // CI containers where `directories` returns None. The runner
    // only opens a sink when the setting is on, so a missing config
    // dir only matters when the user is actively enabling audit.
    PathBuf::from("copythat-audit.log")
}

/// Build a sink from the current persisted settings, or return
/// `None` when audit is disabled. `Err` surfaces a user-legible
/// string so the Settings UI can toast the reason a toggle was
/// refused (permission, missing path, WORM unsupported).
pub fn build_sink(cfg: &AuditSettings) -> Result<Option<Arc<AuditSink>>, String> {
    if !cfg.enabled {
        return Ok(None);
    }
    let (format, worm, rotation, path) = parse_audit_settings(cfg);
    AuditSink::open_with_rotation(&path, format, worm, rotation)
        .map(|s| Some(Arc::new(s)))
        .map_err(|e: AuditError| e.to_string())
}

/// SHA-256 of the settings TOML serialisation. Used for
/// `SettingsChanged.{before,after}_hash` so a diff of "what
/// changed" is recoverable without storing the whole blob.
pub fn settings_fingerprint(settings: &copythat_settings::Settings) -> String {
    let toml_blob = toml::to_string_pretty(settings).unwrap_or_default();
    let mut hasher = sha2::Sha256::new();
    hasher.update(toml_blob.as_bytes());
    hex::encode(hasher.finalize())
}

// ---------------------------------------------------------------------
// Tauri-wire DTOs
// ---------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditStatusDto {
    /// The setting value (pre-build). When `true` and `sink_active`
    /// is `false`, the sink failed to open and the UI should show
    /// the last reason.
    pub enabled: bool,
    pub sink_active: bool,
    pub format: String,
    pub worm: String,
    pub worm_supported: bool,
    pub path: String,
    pub max_size_bytes: u64,
    pub bytes_written: u64,
}

// ---------------------------------------------------------------------
// IPC commands
// ---------------------------------------------------------------------

#[tauri::command]
pub fn audit_status(state: tauri::State<'_, crate::state::AppState>) -> AuditStatusDto {
    let settings = state.settings_snapshot();
    let active = state.audit.snapshot();
    let (path, bytes, worm_val) = match active.as_deref() {
        Some(s) => (
            s.path().to_string_lossy().into_owned(),
            s.bytes_written(),
            match s.worm_mode() {
                WormMode::On => "on",
                WormMode::Off => "off",
            }
            .to_string(),
        ),
        None => {
            let derived = if settings.audit.file_path.is_empty() {
                default_audit_path().to_string_lossy().into_owned()
            } else {
                settings.audit.file_path.clone()
            };
            (derived, 0, settings.audit.worm.clone())
        }
    };
    AuditStatusDto {
        enabled: settings.audit.enabled,
        sink_active: active.is_some(),
        format: settings.audit.format.clone(),
        worm: worm_val,
        worm_supported: is_worm_supported(),
        path,
        max_size_bytes: settings.audit.max_size_bytes,
        bytes_written: bytes,
    }
}

/// Write one synthetic record through the current sink. Returns a
/// friendly error when audit is disabled or the sink is missing.
/// The Settings UI's "Test write" button calls this.
#[tauri::command]
pub fn audit_test_write(state: tauri::State<'_, crate::state::AppState>) -> Result<(), String> {
    let sink = state
        .audit
        .snapshot()
        .ok_or_else(|| "audit sink is not active".to_string())?;
    let evt = AuditEvent::LoginEvent {
        user: current_user(),
        host: current_host(),
        ts: Utc::now(),
    };
    sink.record(&evt).map_err(|e| e.to_string())
}

/// Verify the chain on the currently-configured log file. Used by
/// the Settings → "Verify chain" action + the Phase 36 CLI
/// subcommand. Returns a JSON-ready summary.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditVerifyDto {
    pub total: usize,
    pub matches: usize,
    pub mismatches: usize,
    pub missing: usize,
    pub failed_lines: Vec<usize>,
}

#[tauri::command]
pub fn audit_verify(
    state: tauri::State<'_, crate::state::AppState>,
) -> Result<AuditVerifyDto, String> {
    let settings = state.settings_snapshot();
    let format = AuditFormat::parse(&settings.audit.format).unwrap_or_default();
    let path = if settings.audit.file_path.is_empty() {
        default_audit_path()
    } else {
        PathBuf::from(&settings.audit.file_path)
    };
    let report = copythat_audit::verify_chain(&path, format).map_err(|e| e.to_string())?;
    Ok(AuditVerifyDto {
        total: report.total,
        matches: report.matches,
        mismatches: report.mismatches,
        missing: report.missing,
        failed_lines: report.failed_lines,
    })
}

/// Verify an arbitrary log file. Used by the Phase 36 CLI where the
/// log path isn't the one in Settings.
#[tauri::command]
pub fn audit_verify_file(path: String, format: String) -> Result<AuditVerifyDto, String> {
    // Phase 17e — every other path-typed Tauri command runs through
    // this gate; audit verification was the lone exception.
    let p = validate_ipc_path(&path).map_err(err_string)?;
    let fmt = AuditFormat::parse(&format).unwrap_or_default();
    let report = copythat_audit::verify_chain(&p, fmt).map_err(|e| e.to_string())?;
    Ok(AuditVerifyDto {
        total: report.total,
        matches: report.matches,
        mismatches: report.mismatches,
        missing: report.missing,
        failed_lines: report.failed_lines,
    })
}

// ---------------------------------------------------------------------
// Runner helpers
// ---------------------------------------------------------------------

/// `JobStarted` — emitted once per queued job at the top of
/// `run_job`.
pub fn record_job_started(
    registry: &AuditRegistry,
    job_id: &str,
    kind: &str,
    src: &Path,
    dst: Option<&Path>,
) {
    if !registry.is_active() {
        return;
    }
    let evt = AuditEvent::JobStarted {
        job_id: job_id.to_string(),
        kind: kind.to_string(),
        src: src.to_string_lossy().into_owned(),
        dst: dst
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_default(),
        user: current_user(),
        host: current_host(),
        ts: Utc::now(),
    };
    registry.record(&evt);
}

/// `JobCompleted` — emitted once per queued job at the terminal
/// transition (Ok / Cancelled / Failed all flow through).
pub fn record_job_completed(
    registry: &AuditRegistry,
    job_id: &str,
    status: &str,
    files_ok: u64,
    files_failed: u64,
    bytes: u64,
    duration_ms: u64,
) {
    if !registry.is_active() {
        return;
    }
    let evt = AuditEvent::JobCompleted {
        job_id: job_id.to_string(),
        status: status.to_string(),
        files_ok,
        files_failed,
        bytes,
        duration_ms,
        ts: Utc::now(),
    };
    registry.record(&evt);
}

pub fn record_file_copied(
    registry: &AuditRegistry,
    job_id: &str,
    src: &Path,
    dst: &Path,
    hash: Option<&str>,
    size: u64,
) {
    if !registry.is_active() {
        return;
    }
    let evt = AuditEvent::FileCopied {
        job_id: job_id.to_string(),
        src: src.to_string_lossy().into_owned(),
        dst: dst.to_string_lossy().into_owned(),
        hash: hash.unwrap_or("").to_string(),
        size,
        ts: Utc::now(),
    };
    registry.record(&evt);
}

pub fn record_file_failed(
    registry: &AuditRegistry,
    job_id: &str,
    src: &Path,
    error_code: &str,
    error_msg: &str,
) {
    if !registry.is_active() {
        return;
    }
    let evt = AuditEvent::FileFailed {
        job_id: job_id.to_string(),
        src: src.to_string_lossy().into_owned(),
        error_code: error_code.to_string(),
        error_msg: error_msg.to_string(),
        ts: Utc::now(),
    };
    registry.record(&evt);
}

pub fn record_collision_resolved(
    registry: &AuditRegistry,
    job_id: &str,
    src: &Path,
    dst: &Path,
    action: &str,
) {
    if !registry.is_active() {
        return;
    }
    let evt = AuditEvent::CollisionResolved {
        job_id: job_id.to_string(),
        src: src.to_string_lossy().into_owned(),
        dst: dst.to_string_lossy().into_owned(),
        action: action.to_string(),
        ts: Utc::now(),
    };
    registry.record(&evt);
}

pub fn record_settings_changed(
    registry: &AuditRegistry,
    field: &str,
    before_hash: &str,
    after_hash: &str,
) {
    if !registry.is_active() {
        return;
    }
    let evt = AuditEvent::SettingsChanged {
        user: current_user(),
        host: current_host(),
        field: field.to_string(),
        before_hash: before_hash.to_string(),
        after_hash: after_hash.to_string(),
        ts: Utc::now(),
    };
    registry.record(&evt);
}

pub fn record_login(registry: &AuditRegistry) {
    if !registry.is_active() {
        return;
    }
    let evt = AuditEvent::LoginEvent {
        user: current_user(),
        host: current_host(),
        ts: Utc::now(),
    };
    registry.record(&evt);
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn registry_starts_inactive() {
        let r = AuditRegistry::new();
        assert!(!r.is_active());
        r.record(&AuditEvent::LoginEvent {
            user: "u".into(),
            host: "h".into(),
            ts: Utc::now(),
        });
    }

    #[test]
    fn build_sink_respects_disabled_flag() {
        let cfg = AuditSettings::default();
        assert!(!cfg.enabled);
        let sink = build_sink(&cfg).expect("disabled build_sink is ok");
        assert!(sink.is_none());
    }

    #[test]
    fn set_sink_activates_registry() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("audit.log");
        let cfg = AuditSettings {
            enabled: true,
            file_path: path.to_string_lossy().into_owned(),
            format: "json-lines".into(),
            ..AuditSettings::default()
        };
        let sink = build_sink(&cfg).expect("enabled build_sink").expect("sink");
        let registry = AuditRegistry::new();
        registry.set(Some(sink));
        assert!(registry.is_active());
        record_login(&registry);
        let contents = std::fs::read_to_string(&path).unwrap();
        assert!(contents.contains("login-event"));
    }

    #[test]
    fn fingerprint_changes_with_setting_change() {
        let mut a = copythat_settings::Settings::default();
        let fa = settings_fingerprint(&a);
        a.audit.enabled = true;
        let fb = settings_fingerprint(&a);
        assert_ne!(fa, fb);
    }
}
