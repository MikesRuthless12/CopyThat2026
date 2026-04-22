//! Phase 8 — pending-error registry + error log.
//!
//! The runner receives `CopyEvent::ErrorPrompt` from the engine,
//! hands the enclosed `oneshot::Sender<ErrorAction>` to this
//! registry, and emits a DTO to the frontend. When the user clicks
//! Retry / Skip / Abort (or Skip-All-Of-Kind), the Svelte layer
//! invokes the `resolve_error` command which reaches
//! [`ErrorRegistry::resolve`] and fires the sender.
//!
//! The registry also retains an append-only log of every error that
//! surfaced (both user-resolved and policy-auto-resolved). Phase 9's
//! SQLite history will eventually persist it; Phase 8 keeps the log
//! in-memory, bounded to [`MAX_LOG_ENTRIES`] to prevent unbounded
//! growth on a runaway failing job.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use copythat_core::{CopyError, CopyErrorKind, ErrorAction};
use tokio::sync::oneshot;

/// Log is capped so a pathological "copy a million files, every
/// one fails" scenario does not eat memory. Newer entries push out
/// the oldest when the cap is hit.
pub const MAX_LOG_ENTRIES: usize = 10_000;

/// Cheap handle. Internally an `Arc<ErrorRegistryInner>` so every
/// Tauri command can share the same registry through `AppState`.
#[derive(Clone, Default)]
pub struct ErrorRegistry {
    inner: Arc<ErrorRegistryInner>,
}

#[derive(Default)]
struct ErrorRegistryInner {
    pending: Mutex<HashMap<u64, PendingError>>,
    log: Mutex<Vec<LoggedError>>,
    /// (job_id, localized_key) → action. When a user ticks "skip
    /// all of this kind" and chooses `Skip`, every subsequent error
    /// with the same key inside that job auto-resolves.
    kind_cache: Mutex<HashMap<(u64, &'static str), ErrorAction>>,
    next_id: AtomicU64,
}

/// A pending prompt awaiting user resolution.
pub struct PendingError {
    pub id: u64,
    pub job_id: u64,
    pub err: CopyError,
    pub resolver: oneshot::Sender<ErrorAction>,
    pub created_at_ms: u64,
}

/// A recorded error (resolved or auto-resolved). Cloneable so the
/// `error_log` command can hand a snapshot to the frontend.
#[derive(Debug, Clone)]
pub struct LoggedError {
    pub id: u64,
    pub job_id: u64,
    pub src: PathBuf,
    pub dst: PathBuf,
    pub kind: &'static str,
    pub localized_key: &'static str,
    pub message: String,
    pub raw_os_error: Option<i32>,
    pub timestamp_ms: u64,
    /// `"retry"` / `"skip"` / `"abort"` / `"auto-skip"` / `None` if
    /// the log entry records a failure with no resolution yet.
    pub resolution: Option<&'static str>,
}

impl ErrorRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Stash a prompt. Returns the assigned error-id; the runner
    /// ships that id in the IPC `error-raised` event.
    pub fn register(
        &self,
        job_id: u64,
        err: CopyError,
        resolver: oneshot::Sender<ErrorAction>,
    ) -> u64 {
        let id = self.inner.next_id.fetch_add(1, Ordering::Relaxed) + 1;
        let pending = PendingError {
            id,
            job_id,
            err,
            resolver,
            created_at_ms: now_ms(),
        };
        self.inner
            .pending
            .lock()
            .expect("error registry poisoned")
            .insert(id, pending);
        id
    }

    /// Look up a prior "Skip all of this kind" decision for the
    /// given job. The runner consults this *before* registering a
    /// fresh prompt so a cached decision short-circuits the UX.
    pub fn cached_action_for(&self, job_id: u64, err: &CopyError) -> Option<ErrorAction> {
        self.inner
            .kind_cache
            .lock()
            .expect("error registry poisoned")
            .get(&(job_id, err.localized_key()))
            .copied()
    }

    /// Resolve a pending error. `apply_to_all` caches the action so
    /// future errors of the same kind in the same job auto-resolve.
    pub fn resolve(
        &self,
        id: u64,
        action: ErrorAction,
        apply_to_all: bool,
    ) -> Result<ResolvedError, String> {
        let pending = self
            .inner
            .pending
            .lock()
            .expect("error registry poisoned")
            .remove(&id)
            .ok_or_else(|| format!("unknown error id: {id}"))?;

        if apply_to_all && action == ErrorAction::Skip {
            self.inner
                .kind_cache
                .lock()
                .expect("error registry poisoned")
                .insert((pending.job_id, pending.err.localized_key()), action);
        }

        self.append_log(LoggedError {
            id: pending.id,
            job_id: pending.job_id,
            src: pending.err.src.clone(),
            dst: pending.err.dst.clone(),
            kind: kind_name(pending.err.kind),
            localized_key: pending.err.localized_key(),
            message: pending.err.message.clone(),
            raw_os_error: pending.err.raw_os_error,
            timestamp_ms: now_ms(),
            resolution: Some(action_name(action)),
        });

        let _ = pending.resolver.send(action);

        Ok(ResolvedError {
            id: pending.id,
            job_id: pending.job_id,
            action,
        })
    }

    /// Record a failure that resolved via policy (not user-facing).
    /// Called by the runner when `CopyEvent::FileError` arrives
    /// — the engine already applied `Skip` or `RetryN`, we just log.
    pub fn log_auto(&self, job_id: u64, err: &CopyError) {
        let id = self.inner.next_id.fetch_add(1, Ordering::Relaxed) + 1;
        self.append_log(LoggedError {
            id,
            job_id,
            src: err.src.clone(),
            dst: err.dst.clone(),
            kind: kind_name(err.kind),
            localized_key: err.localized_key(),
            message: err.message.clone(),
            raw_os_error: err.raw_os_error,
            timestamp_ms: now_ms(),
            resolution: Some("auto-skip"),
        });
    }

    /// Snapshot of every logged error (oldest first). Used by the
    /// `error_log` command and the CSV / TXT exporters.
    pub fn log(&self) -> Vec<LoggedError> {
        self.inner
            .log
            .lock()
            .expect("error registry poisoned")
            .clone()
    }

    /// Wipe the in-memory log. The frontend exposes this via
    /// "Clear log" in the error-log drawer.
    pub fn clear_log(&self) {
        self.inner
            .log
            .lock()
            .expect("error registry poisoned")
            .clear();
    }

    /// Pending-count snapshot — exposed so the frontend can disable
    /// "Abort all" while there's still a prompt up waiting for
    /// input.
    pub fn pending_count(&self) -> usize {
        self.inner
            .pending
            .lock()
            .expect("error registry poisoned")
            .len()
    }

    fn append_log(&self, entry: LoggedError) {
        let mut log = self.inner.log.lock().expect("error registry poisoned");
        if log.len() >= MAX_LOG_ENTRIES {
            // Drop the oldest to make room.
            log.remove(0);
        }
        log.push(entry);
    }
}

#[derive(Debug, Clone)]
pub struct ResolvedError {
    pub id: u64,
    pub job_id: u64,
    pub action: ErrorAction,
}

/// Map an error-kind enum to the lowercase wire name the frontend /
/// CSV expect. Single source of truth for the mapping.
pub fn kind_name(kind: CopyErrorKind) -> &'static str {
    match kind {
        CopyErrorKind::NotFound => "not-found",
        CopyErrorKind::PermissionDenied => "permission-denied",
        CopyErrorKind::DiskFull => "disk-full",
        CopyErrorKind::Interrupted => "interrupted",
        CopyErrorKind::VerifyFailed => "verify-failed",
        CopyErrorKind::PathEscape => "path-escape",
        CopyErrorKind::IoOther => "io-other",
    }
}

/// Map an `ErrorAction` to the lowercase wire name.
pub fn action_name(action: ErrorAction) -> &'static str {
    match action {
        ErrorAction::Retry => "retry",
        ErrorAction::Skip => "skip",
        ErrorAction::Abort => "abort",
    }
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

/// Export the log as CSV. Columns:
/// `timestamp_ms,job_id,kind,localized_key,src,dst,raw_os_error,
///  resolution,message`.
///
/// Quote every field so embedded commas / newlines in paths or
/// messages don't break the grid.
pub fn export_csv(entries: &[LoggedError]) -> String {
    let mut out = String::with_capacity(entries.len() * 64);
    out.push_str(
        "timestamp_ms,job_id,kind,localized_key,src,dst,raw_os_error,resolution,message\n",
    );
    for e in entries {
        out.push_str(&csv_field(&e.timestamp_ms.to_string()));
        out.push(',');
        out.push_str(&csv_field(&e.job_id.to_string()));
        out.push(',');
        out.push_str(&csv_field(e.kind));
        out.push(',');
        out.push_str(&csv_field(e.localized_key));
        out.push(',');
        out.push_str(&csv_field(&e.src.to_string_lossy()));
        out.push(',');
        out.push_str(&csv_field(&e.dst.to_string_lossy()));
        out.push(',');
        out.push_str(&csv_field(
            &e.raw_os_error.map(|v| v.to_string()).unwrap_or_default(),
        ));
        out.push(',');
        out.push_str(&csv_field(e.resolution.unwrap_or("")));
        out.push(',');
        out.push_str(&csv_field(&e.message));
        out.push('\n');
    }
    out
}

/// Export the log as a tab-separated plain-text report. Each record
/// is one line; no header row so it pastes cleanly into bug tickets.
pub fn export_txt(entries: &[LoggedError]) -> String {
    let mut out = String::with_capacity(entries.len() * 96);
    for e in entries {
        out.push_str(&format!(
            "[{ms}] job={job} kind={kind} os={os} {src} -> {dst}: {msg} ({res})\n",
            ms = e.timestamp_ms,
            job = e.job_id,
            kind = e.kind,
            os = e
                .raw_os_error
                .map(|v| v.to_string())
                .unwrap_or_else(|| "-".to_string()),
            src = e.src.display(),
            dst = e.dst.display(),
            msg = e.message,
            res = e.resolution.unwrap_or("-"),
        ));
    }
    out
}

fn csv_field(s: &str) -> String {
    // RFC-4180: double any quotes, wrap the whole field in quotes.
    let escaped = s.replace('"', "\"\"");
    format!("\"{escaped}\"")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn make_err(kind: CopyErrorKind, msg: &str) -> CopyError {
        CopyError {
            kind,
            src: Path::new("/src").to_path_buf(),
            dst: Path::new("/dst").to_path_buf(),
            raw_os_error: None,
            message: msg.to_string(),
        }
    }

    #[test]
    fn kind_names_are_stable_lowercase_kebab() {
        assert_eq!(kind_name(CopyErrorKind::NotFound), "not-found");
        assert_eq!(
            kind_name(CopyErrorKind::PermissionDenied),
            "permission-denied"
        );
        assert_eq!(kind_name(CopyErrorKind::DiskFull), "disk-full");
        assert_eq!(kind_name(CopyErrorKind::Interrupted), "interrupted");
        assert_eq!(kind_name(CopyErrorKind::VerifyFailed), "verify-failed");
        assert_eq!(kind_name(CopyErrorKind::IoOther), "io-other");
    }

    #[tokio::test]
    async fn register_then_resolve_fires_oneshot() {
        let reg = ErrorRegistry::new();
        let (tx, rx) = oneshot::channel::<ErrorAction>();
        let err = make_err(CopyErrorKind::PermissionDenied, "denied");
        let id = reg.register(42, err, tx);

        let resolved = reg.resolve(id, ErrorAction::Skip, false).unwrap();
        assert_eq!(resolved.id, id);
        assert_eq!(resolved.job_id, 42);
        assert_eq!(resolved.action, ErrorAction::Skip);
        assert_eq!(rx.await.unwrap(), ErrorAction::Skip);
        assert_eq!(reg.pending_count(), 0);
        assert_eq!(reg.log().len(), 1);
        assert_eq!(reg.log()[0].resolution, Some("skip"));
    }

    #[tokio::test]
    async fn apply_to_all_caches_skip_by_kind() {
        let reg = ErrorRegistry::new();
        let (tx1, _rx1) = oneshot::channel();
        let err1 = make_err(CopyErrorKind::PermissionDenied, "denied a");
        let id1 = reg.register(7, err1, tx1);
        reg.resolve(id1, ErrorAction::Skip, true).unwrap();

        // A second PermissionDenied error in the same job should
        // match the cache.
        let err2 = make_err(CopyErrorKind::PermissionDenied, "denied b");
        assert_eq!(reg.cached_action_for(7, &err2), Some(ErrorAction::Skip));

        // A different error kind should NOT match.
        let err3 = make_err(CopyErrorKind::DiskFull, "full");
        assert_eq!(reg.cached_action_for(7, &err3), None);

        // And a different job_id should not match either.
        assert_eq!(reg.cached_action_for(8, &err2), None);
    }

    #[test]
    fn log_cap_drops_oldest() {
        let reg = ErrorRegistry::new();
        for i in 0..(MAX_LOG_ENTRIES + 5) {
            let err = make_err(CopyErrorKind::IoOther, &format!("msg {i}"));
            reg.log_auto(1, &err);
        }
        let log = reg.log();
        assert_eq!(log.len(), MAX_LOG_ENTRIES);
        // Oldest 5 entries should have been dropped.
        assert_eq!(log[0].message, "msg 5");
        assert_eq!(
            log.last().unwrap().message,
            format!("msg {}", MAX_LOG_ENTRIES + 4)
        );
    }

    #[test]
    fn resolve_unknown_id_errors() {
        let reg = ErrorRegistry::new();
        assert!(reg.resolve(999, ErrorAction::Skip, false).is_err());
    }
}
