//! Phase 34 — enterprise-grade audit log export + WORM mode.
//!
//! # Scope
//!
//! Copy That's existing footer-drawer error log is an in-memory list
//! useful for a sysadmin watching a single desktop job. It does not
//! satisfy the enterprise requirements auditors actually ask for —
//! persistent, tamper-evident, machine-ingestible logs in a format
//! their SIEM already speaks. This crate closes that gap.
//!
//! - [`AuditEvent`] — the finite, typed set of events worth
//!   recording. The runner translates its high-level
//!   [`copythat_core::CopyEvent`] + queue state changes into one
//!   [`AuditEvent`] per user-visible transition (job start/end, file
//!   copied/failed, collision resolved, settings change, login,
//!   unauthorized action).
//! - [`AuditFormat`] — five wire formats. CSV + JSON-lines are
//!   Copy-That-internal; Syslog RFC 5424, ArcSight CEF, and QRadar
//!   LEEF are the three enterprise SIEMs cover.
//! - [`AuditSink`] — the append-only writer. Opens a file (or
//!   pipes-to-syslog bridge), appends one record per event, maintains
//!   a rolling BLAKE3 chain hash so the file is end-to-end tamper-
//!   evident, and optionally flips the file into [`WormMode::On`] so
//!   even a root user can't truncate it without explicit ACL removal.
//! - [`AuditLayer`] — a [`tracing_subscriber::Layer`] that fans
//!   events off the process-wide `tracing` bus into the active sink
//!   when [`AuditSink::install_tracing_layer`] has been called. The
//!   runner calls into the sink directly for structured events; the
//!   tracing layer is the safety net for ad-hoc log! calls that
//!   auditors want captured too.
//!
//! Phase 34 does *not* ship the "verify log" CLI (`copythat audit
//! verify …`) — that lands with the Phase 36 CLI and simply shells
//! into [`verify_chain`] here.
//!
//! The `worm` module is the only place in this crate where `unsafe`
//! is legal — each platform's append-only primitive is a raw FFI
//! call (Linux `FS_IOC_SETFLAGS`, macOS `chflags`, Windows
//! `SetFileAttributesW`). The rest of the crate stays pure Rust.

use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use chrono::{DateTime, SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod format;
pub mod layer;
pub mod verify;
// The WORM module wraps raw FFI (Linux ioctl, macOS chflags, Windows
// SetFileAttributesW); every unsafe block is isolated to this file.
// The workspace-wide `unsafe_code = "warn"` lint fires inside the
// raw blocks; allow-at-module-level is the standard escape hatch
// used elsewhere in the workspace (see `copythat-platform`).
#[allow(unsafe_code)]
pub mod worm;

pub use format::{
    CHAIN_HASH_HEX_LEN, cef_escape_extension, cef_escape_header, csv_header, format_record,
    leef_escape_extension, syslog_app_name,
};
pub use layer::AuditLayer;
pub use verify::{VerifyOutcome, VerifyReport, verify_chain};
pub use worm::{WormMode, apply_worm, is_worm_supported};

/// The set of wire formats [`AuditSink`] can serialize to. Enterprise
/// customers typically pick one of Syslog / CEF / LEEF to match the
/// SIEM they already operate; JSON-lines is the pragmatic default
/// for local / shell-pipeline inspection; CSV is the long-tail
/// compliance export format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AuditFormat {
    /// One row per event with a fixed column order.
    /// [`format::csv_header`] documents the column list.
    Csv,
    /// One JSON object per line. Preserves nested variant fields.
    JsonLines,
    /// RFC 5424 syslog with a structured-data block
    /// (`[copythat@32473 jobId="..." user="..."]`). Pipe to rsyslog
    /// via TCP/UDP/UDS in the consumer.
    Syslog,
    /// ArcSight Common Event Format 0:
    /// `CEF:0|CopyThat|CopyThat|<ver>|<sig>|<name>|<sev>|<extension>`.
    Cef,
    /// IBM QRadar Log Event Extended Format 2.0:
    /// `LEEF:2.0|CopyThat|CopyThat|<ver>|<eventID>|<tab-extension>`.
    Leef,
}

impl AuditFormat {
    /// Stable kebab-case identifier used by the TOML settings file
    /// and Tauri IPC surface.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Csv => "csv",
            Self::JsonLines => "json-lines",
            Self::Syslog => "syslog",
            Self::Cef => "cef",
            Self::Leef => "leef",
        }
    }

    /// Parse the kebab-case string back to an enum. Used when
    /// Settings TOML loads an older value written under a different
    /// formatting pass.
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "csv" => Some(Self::Csv),
            "json-lines" | "jsonl" | "json" => Some(Self::JsonLines),
            "syslog" => Some(Self::Syslog),
            "cef" => Some(Self::Cef),
            "leef" => Some(Self::Leef),
            _ => None,
        }
    }
}

impl Default for AuditFormat {
    /// JSON-lines is the safe default — human-readable, deterministic,
    /// trivially tail-able. Enterprises pick Syslog / CEF / LEEF
    /// explicitly when they wire a collector up.
    fn default() -> Self {
        Self::JsonLines
    }
}

/// Severity tag applied to each record. Mapped into Syslog facility
/// / CEF severity / LEEF `sev` fields per format. Five levels
/// matching the `tracing` vocabulary so a log! at those levels
/// translates 1:1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AuditSeverity {
    Debug,
    Info,
    Notice,
    Warning,
    Error,
}

impl AuditSeverity {
    /// CEF / LEEF numeric severity (0–10).
    pub const fn cef_severity(self) -> u8 {
        match self {
            Self::Debug => 0,
            Self::Info => 3,
            Self::Notice => 5,
            Self::Warning => 7,
            Self::Error => 9,
        }
    }

    /// Syslog priority — facility 13 (log audit) plus severity.
    ///
    /// RFC 5424 §6.2.1: `PRI = (facility * 8) + severity`.
    pub const fn syslog_priority(self) -> u8 {
        // Facility 13 = "log audit". Severity maps: Debug=7, Info=6,
        // Notice=5, Warning=4, Error=3.
        let facility: u8 = 13;
        let severity: u8 = match self {
            Self::Debug => 7,
            Self::Info => 6,
            Self::Notice => 5,
            Self::Warning => 4,
            Self::Error => 3,
        };
        facility * 8 + severity
    }
}

/// One event worth appending to the log. Variants map 1:1 to the
/// brief's event list; each carries the minimum fields the SIEM
/// needs to triage without a round-trip to the job DB.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "event")]
pub enum AuditEvent {
    JobStarted {
        job_id: String,
        kind: String,
        src: String,
        dst: String,
        user: String,
        host: String,
        ts: DateTime<Utc>,
    },
    JobCompleted {
        job_id: String,
        status: String,
        files_ok: u64,
        files_failed: u64,
        bytes: u64,
        duration_ms: u64,
        ts: DateTime<Utc>,
    },
    FileCopied {
        job_id: String,
        src: String,
        dst: String,
        /// Hex-encoded 32-byte hash. Empty when `verify = None`
        /// was configured on the job.
        hash: String,
        size: u64,
        ts: DateTime<Utc>,
    },
    FileFailed {
        job_id: String,
        src: String,
        error_code: String,
        error_msg: String,
        ts: DateTime<Utc>,
    },
    CollisionResolved {
        job_id: String,
        src: String,
        dst: String,
        action: String,
        ts: DateTime<Utc>,
    },
    SettingsChanged {
        user: String,
        host: String,
        field: String,
        before_hash: String,
        after_hash: String,
        ts: DateTime<Utc>,
    },
    LoginEvent {
        user: String,
        host: String,
        ts: DateTime<Utc>,
    },
    UnauthorizedAccess {
        user: String,
        host: String,
        attempted_action: String,
        reason: String,
        ts: DateTime<Utc>,
    },
}

impl AuditEvent {
    /// Short, stable signature for CEF / LEEF `signatureId` /
    /// `eventID`. Mirrors the variant name in `UpperCamel`.
    pub const fn signature(&self) -> &'static str {
        match self {
            Self::JobStarted { .. } => "JobStarted",
            Self::JobCompleted { .. } => "JobCompleted",
            Self::FileCopied { .. } => "FileCopied",
            Self::FileFailed { .. } => "FileFailed",
            Self::CollisionResolved { .. } => "CollisionResolved",
            Self::SettingsChanged { .. } => "SettingsChanged",
            Self::LoginEvent { .. } => "LoginEvent",
            Self::UnauthorizedAccess { .. } => "UnauthorizedAccess",
        }
    }

    /// Human-readable short label — CEF uses this as the `name`
    /// header field, LEEF as the event "name" extension.
    pub const fn label(&self) -> &'static str {
        match self {
            Self::JobStarted { .. } => "Copy job started",
            Self::JobCompleted { .. } => "Copy job completed",
            Self::FileCopied { .. } => "File copied",
            Self::FileFailed { .. } => "File copy failed",
            Self::CollisionResolved { .. } => "Collision resolved",
            Self::SettingsChanged { .. } => "Settings changed",
            Self::LoginEvent { .. } => "Application login",
            Self::UnauthorizedAccess { .. } => "Unauthorized access",
        }
    }

    /// Default severity class per variant.
    pub const fn severity(&self) -> AuditSeverity {
        match self {
            Self::JobStarted { .. } => AuditSeverity::Info,
            Self::JobCompleted { .. } => AuditSeverity::Info,
            Self::FileCopied { .. } => AuditSeverity::Debug,
            Self::FileFailed { .. } => AuditSeverity::Warning,
            Self::CollisionResolved { .. } => AuditSeverity::Info,
            Self::SettingsChanged { .. } => AuditSeverity::Notice,
            Self::LoginEvent { .. } => AuditSeverity::Info,
            Self::UnauthorizedAccess { .. } => AuditSeverity::Error,
        }
    }

    /// Timestamp accessor — every variant carries one.
    pub const fn timestamp(&self) -> &DateTime<Utc> {
        match self {
            Self::JobStarted { ts, .. }
            | Self::JobCompleted { ts, .. }
            | Self::FileCopied { ts, .. }
            | Self::FileFailed { ts, .. }
            | Self::CollisionResolved { ts, .. }
            | Self::SettingsChanged { ts, .. }
            | Self::LoginEvent { ts, .. }
            | Self::UnauthorizedAccess { ts, .. } => ts,
        }
    }

    /// RFC 3339 / ISO-8601 representation (Zulu, 6-digit micros)
    /// used by every format's timestamp column.
    pub fn ts_iso8601(&self) -> String {
        self.timestamp()
            .to_rfc3339_opts(SecondsFormat::Micros, true)
    }

    /// Record the event's owning job id where the variant has one,
    /// otherwise an empty string. Used by Syslog structured-data
    /// + CEF extension fields so a SIEM can group records.
    pub fn job_id(&self) -> &str {
        match self {
            Self::JobStarted { job_id, .. }
            | Self::JobCompleted { job_id, .. }
            | Self::FileCopied { job_id, .. }
            | Self::FileFailed { job_id, .. }
            | Self::CollisionResolved { job_id, .. } => job_id.as_str(),
            Self::SettingsChanged { .. }
            | Self::LoginEvent { .. }
            | Self::UnauthorizedAccess { .. } => "",
        }
    }

    /// User attributed to the event (file events inherit from the
    /// owning job; the runner supplies the correlated value).
    pub fn user(&self) -> &str {
        match self {
            Self::JobStarted { user, .. }
            | Self::SettingsChanged { user, .. }
            | Self::LoginEvent { user, .. }
            | Self::UnauthorizedAccess { user, .. } => user.as_str(),
            _ => "",
        }
    }
}

/// How rotation behaves when the current log grows past
/// `max_size` bytes. Rolling keeps `<path>.1` with a single
/// backup; larger retention is the SIEM's job.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RotationPolicy {
    /// Byte threshold at which [`AuditSink::rotate`] moves the
    /// current file to `<path>.1`. Zero = rotation disabled.
    pub max_size: u64,
}

impl Default for RotationPolicy {
    fn default() -> Self {
        // 10 MiB — sized to fit "one quarter of routine activity"
        // without surprising the user with a giant file.
        Self {
            max_size: 10 * 1024 * 1024,
        }
    }
}

/// Errors surfaced by [`AuditSink`] / [`verify_chain`]. IO is kept
/// separate from format errors so callers can surface "WORM mode
/// blocked this write" differently from "disk full".
#[derive(Debug, Error)]
pub enum AuditError {
    #[error("audit IO error on {path:?}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("audit format error: {0}")]
    Format(String),
    #[error("audit chain verification failed at line {line}: {reason}")]
    ChainMismatch { line: usize, reason: String },
    #[error("audit WORM mode unsupported on this platform")]
    WormUnsupported,
    #[error("audit WORM mode failed to apply: {0}")]
    WormApply(String),
}

pub type Result<T> = std::result::Result<T, AuditError>;

/// Append-only, chain-hashed log writer. A single sink is owned by
/// the Tauri runner (or the CLI in Phase 36) for the lifetime of the
/// process; [`Clone`] is intentionally not derived — all callers
/// share via `Arc<Mutex<AuditSink>>` so chain hashes stay linear.
pub struct AuditSink {
    path: PathBuf,
    format: AuditFormat,
    worm: WormMode,
    rotation: RotationPolicy,
    writer: Mutex<SinkInner>,
}

struct SinkInner {
    file: File,
    /// BLAKE3 hash of the previous line's canonical record bytes
    /// (exclusive of the chain-hash column itself). First line
    /// uses all-zero.
    chain_hash: [u8; 32],
    bytes_written: u64,
}

impl AuditSink {
    /// Open (or create) `path` for append. WORM application is
    /// requested via `worm`; the sink reports `AuditError::WormApply`
    /// if the OS refuses. A freshly-opened sink reads the last
    /// recorded chain hash if the file already exists so appends
    /// continue the chain.
    pub fn open(path: &Path, format: AuditFormat, worm: WormMode) -> Result<Self> {
        Self::open_with_rotation(path, format, worm, RotationPolicy::default())
    }

    /// Same as [`Self::open`] with a caller-supplied rotation
    /// policy. The UI's "Advanced → Audit log → Max size" slider
    /// flows through here.
    pub fn open_with_rotation(
        path: &Path,
        format: AuditFormat,
        worm: WormMode,
        rotation: RotationPolicy,
    ) -> Result<Self> {
        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
        {
            std::fs::create_dir_all(parent).map_err(|source| AuditError::Io {
                path: parent.to_path_buf(),
                source,
            })?;
        }

        // Resume the chain from the existing tail if any.
        let (prev_hash, existing_bytes) =
            read_tail_state(path, format).map_err(|source| AuditError::Io {
                path: path.to_path_buf(),
                source,
            })?;

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .map_err(|source| AuditError::Io {
                path: path.to_path_buf(),
                source,
            })?;

        let sink = Self {
            path: path.to_path_buf(),
            format,
            worm,
            rotation,
            writer: Mutex::new(SinkInner {
                file,
                chain_hash: prev_hash,
                bytes_written: existing_bytes,
            }),
        };

        // CSV needs a header row whenever the file is freshly
        // created (existing_bytes == 0). Other formats are
        // header-free.
        if existing_bytes == 0 && matches!(format, AuditFormat::Csv) {
            let header = csv_header();
            let mut guard = sink
                .writer
                .lock()
                .expect("audit sink writer mutex poisoned");
            guard
                .file
                .write_all(header.as_bytes())
                .map_err(|source| AuditError::Io {
                    path: sink.path.clone(),
                    source,
                })?;
            guard.bytes_written = header.len() as u64;
        }

        // Apply WORM only after the header write so the open
        // succeeded, matching the brief's "deny-write-after-create"
        // semantics.
        apply_worm(&sink.path, worm).map_err(|e| match e {
            worm::WormError::Unsupported => AuditError::WormUnsupported,
            worm::WormError::Apply(reason) => AuditError::WormApply(reason),
        })?;

        Ok(sink)
    }

    /// The path passed to [`Self::open`] — exposed so callers can
    /// surface it in Settings UI without caching it separately.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// The active wire format.
    pub fn format(&self) -> AuditFormat {
        self.format
    }

    /// The active WORM state.
    pub fn worm_mode(&self) -> WormMode {
        self.worm
    }

    /// Append one record. Acquires an internal mutex so concurrent
    /// callers across threads stay serialised: the chain hash
    /// depends on write order, so parallel writes would produce
    /// unverifiable logs.
    pub fn record(&self, event: &AuditEvent) -> Result<()> {
        let mut guard = self
            .writer
            .lock()
            .expect("audit sink writer mutex poisoned");
        let prev_hex = hex::encode(guard.chain_hash);
        let line = format_record(self.format, event, &prev_hex)?;
        guard
            .file
            .write_all(line.as_bytes())
            .map_err(|source| AuditError::Io {
                path: self.path.clone(),
                source,
            })?;
        // Flush to kernel on every record — auditors care more
        // about durability than throughput. The OS page cache still
        // buffers across process boundaries.
        guard.file.flush().map_err(|source| AuditError::Io {
            path: self.path.clone(),
            source,
        })?;
        guard.chain_hash = next_chain_hash(&guard.chain_hash, &line);
        guard.bytes_written += line.len() as u64;

        let threshold = self.rotation.max_size;
        if threshold > 0 && guard.bytes_written >= threshold {
            drop(guard);
            self.rotate(threshold)?;
        }
        Ok(())
    }

    /// Rotate the log if the current size is >= `max_size`. Moves
    /// the current file to `<path>.1` (overwriting any previous
    /// rollover) and resets the chain hash + open a fresh file.
    /// Returns `Ok(true)` when rotation happened, `Ok(false)` when
    /// the file was still under the threshold.
    pub fn rotate(&self, max_size: u64) -> Result<bool> {
        let mut guard = self
            .writer
            .lock()
            .expect("audit sink writer mutex poisoned");
        if guard.bytes_written < max_size {
            return Ok(false);
        }
        // Drop the current file handle first so the rename succeeds
        // on Windows where an open append handle would block the
        // source rename.
        drop(std::mem::replace(
            &mut guard.file,
            File::create(&self.path).map_err(|source| AuditError::Io {
                path: self.path.clone(),
                source,
            })?,
        ));

        // Drop the placeholder file we just created so the rename
        // target is the original path, intact. We need this dance
        // only on Windows; on Unix the open-handle rename works
        // but we keep the flow identical so behaviour is portable.
        let rotated = rotated_path(&self.path);
        // Clear any previous rollover — the brief keeps a single
        // `.1` rolling backup.
        let _ = std::fs::remove_file(&rotated);
        std::fs::rename(&self.path, &rotated).map_err(|source| AuditError::Io {
            path: self.path.clone(),
            source,
        })?;

        let fresh = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(|source| AuditError::Io {
                path: self.path.clone(),
                source,
            })?;
        guard.file = fresh;
        guard.chain_hash = [0u8; 32];
        guard.bytes_written = 0;

        if matches!(self.format, AuditFormat::Csv) {
            let header = csv_header();
            guard
                .file
                .write_all(header.as_bytes())
                .map_err(|source| AuditError::Io {
                    path: self.path.clone(),
                    source,
                })?;
            guard.bytes_written = header.len() as u64;
        }

        drop(guard);
        // Re-apply WORM to the freshly-created file (the old one
        // retains its own WORM attribute across the rename).
        apply_worm(&self.path, self.worm).map_err(|e| match e {
            worm::WormError::Unsupported => AuditError::WormUnsupported,
            worm::WormError::Apply(reason) => AuditError::WormApply(reason),
        })?;
        Ok(true)
    }

    /// Install a [`tracing_subscriber::Layer`] that forwards every
    /// `target = "copythat::audit"` event to this sink. Returns the
    /// layer so the caller can register it with their Registry
    /// subscriber. Called once at app startup by the runner.
    pub fn install_tracing_layer(self: Arc<Self>) -> AuditLayer {
        AuditLayer::new(self)
    }

    /// Snapshot the current chain hash — used by tests + the
    /// Phase 36 CLI verify subcommand fast-path check.
    pub fn current_chain_hash(&self) -> [u8; 32] {
        let guard = self
            .writer
            .lock()
            .expect("audit sink writer mutex poisoned");
        guard.chain_hash
    }

    /// Current file length — tests exercise `rotate` thresholds
    /// by reading this.
    pub fn bytes_written(&self) -> u64 {
        self.writer
            .lock()
            .expect("audit sink writer mutex poisoned")
            .bytes_written
    }
}

/// Deterministic BLAKE3 chain step. `prev` is 32 bytes of zeros for
/// the first record; otherwise it's the hash of the *previous
/// record's bytes* (including its chain-hash column). Each record
/// embeds `hex::encode(prev)` so parsers can recompute the chain.
pub fn next_chain_hash(prev: &[u8; 32], record_bytes: &str) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(prev);
    hasher.update(record_bytes.as_bytes());
    *hasher.finalize().as_bytes()
}

/// Compute the path that rotation moves the current log to. Same
/// file name + `.1` suffix. Exposed for tests.
pub fn rotated_path(path: &Path) -> PathBuf {
    let file_name = path.file_name().map(|n| n.to_os_string());
    match file_name {
        Some(name) => {
            let mut rotated = name;
            rotated.push(".1");
            path.with_file_name(rotated)
        }
        None => path.with_extension("1"),
    }
}

/// Scan the existing file (if any) to recover the last chain hash
/// so an append continues the chain. For unknown / unrecognisable
/// contents we fall back to a zero hash — the verify step will
/// surface the inconsistency.
fn read_tail_state(path: &Path, format: AuditFormat) -> io::Result<([u8; 32], u64)> {
    use std::io::Read;

    if !path.exists() {
        return Ok(([0u8; 32], 0));
    }
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    let len = buf.len() as u64;
    if buf.is_empty() {
        return Ok(([0u8; 32], 0));
    }

    // Walk the file line-by-line, recomputing the chain so the
    // resumed state is self-consistent. Non-record lines (the CSV
    // header, a blank line at EOF) are skipped.
    let mut chain = [0u8; 32];
    for line in buf.split_inclusive('\n') {
        if line.trim().is_empty() {
            continue;
        }
        if matches!(format, AuditFormat::Csv) && line.starts_with("event,") {
            continue;
        }
        chain = next_chain_hash(&chain, line);
    }
    Ok((chain, len))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn sample_job_started() -> AuditEvent {
        AuditEvent::JobStarted {
            job_id: "j-1".into(),
            kind: "copy".into(),
            src: "/tmp/src".into(),
            dst: "/tmp/dst".into(),
            user: "alice".into(),
            host: "host1".into(),
            ts: Utc.with_ymd_and_hms(2026, 4, 24, 12, 0, 0).unwrap(),
        }
    }

    #[test]
    fn format_roundtrips_through_parse() {
        for fmt in [
            AuditFormat::Csv,
            AuditFormat::JsonLines,
            AuditFormat::Syslog,
            AuditFormat::Cef,
            AuditFormat::Leef,
        ] {
            let repr = fmt.as_str();
            assert_eq!(AuditFormat::parse(repr), Some(fmt));
        }
        assert!(AuditFormat::parse("nope").is_none());
    }

    #[test]
    fn event_signature_matches_variant() {
        let e = sample_job_started();
        assert_eq!(e.signature(), "JobStarted");
        assert_eq!(e.severity(), AuditSeverity::Info);
    }

    #[test]
    fn chain_hash_is_deterministic() {
        let prev = [0u8; 32];
        let h1 = next_chain_hash(&prev, "line-a\n");
        let h2 = next_chain_hash(&prev, "line-a\n");
        assert_eq!(h1, h2);
        let h3 = next_chain_hash(&h1, "line-b\n");
        assert_ne!(h3, h1);
    }

    #[test]
    fn rotated_path_appends_one() {
        let path = Path::new("/var/log/copythat.log");
        assert_eq!(rotated_path(path), PathBuf::from("/var/log/copythat.log.1"));
    }
}
