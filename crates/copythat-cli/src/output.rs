//! JSON-Lines event emission + the human progress fallback.
//!
//! Every line on `stdout` (in `--json` mode) is exactly one JSON
//! object terminated by `\n`. Schema is intentionally minimal so
//! downstream automation can consume it with `jq` /
//! `serde_json::Deserializer::from_reader`.

use std::io::{self, Write};
use std::sync::Mutex;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Whether the CLI emits machine-readable JSON-Lines or human text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputMode {
    /// Default. Friendly progress prints to `stdout`.
    Human,
    /// `--json`. One JSON object per line.
    Json,
    /// `--quiet`. No `stdout` output at all.
    Quiet,
}

impl OutputMode {
    pub fn from_global(json: bool, quiet: bool) -> Self {
        if quiet {
            Self::Quiet
        } else if json {
            Self::Json
        } else {
            Self::Human
        }
    }
}

/// Tagged event emitted on `stdout` when `--json` is set.
///
/// The `kind` discriminator is the canonical string (`job.started`,
/// `job.progress`, `job.completed`, `job.failed`, `plan.action`,
/// `version`, `config.value`, `verify.ok`, `verify.failed`,
/// `info`, `error`). Renaming the variants is allowed; the wire-string
/// derived via `#[serde(rename_all)]` is the contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum JsonEventKind {
    JobStarted {
        job_id: String,
        src: String,
        dst: String,
        /// `"copy"`, `"move"`, `"verify"`, ... — the engine action
        /// the job is performing. Renamed from `kind` to avoid
        /// colliding with the outer `tag = "kind"` discriminator.
        operation: String,
    },
    JobProgress {
        job_id: String,
        bytes_done: u64,
        bytes_total: u64,
        rate_bps: u64,
    },
    JobCompleted {
        job_id: String,
        bytes: u64,
        files: u64,
        duration_ms: u64,
    },
    JobFailed {
        job_id: String,
        reason: String,
    },
    PlanAction {
        action: String,
        src: Option<String>,
        dst: Option<String>,
        bytes: Option<u64>,
        note: Option<String>,
    },
    PlanSummary {
        actions: u64,
        bytes: u64,
        already_done: u64,
    },
    Version {
        version: String,
        crate_name: String,
        rustc_known_at_compile: bool,
    },
    ConfigValue {
        key: String,
        value: serde_json::Value,
    },
    VerifyOk {
        path: String,
        algo: String,
        digest: String,
    },
    VerifyFailed {
        path: String,
        algo: String,
        expected: Option<String>,
        actual: String,
    },
    Info {
        message: String,
    },
    Error {
        message: String,
        code: u8,
    },
}

/// Wrapper that adds the timestamp + optional job identifier.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonEvent {
    pub ts: DateTime<Utc>,
    #[serde(flatten)]
    pub event: JsonEventKind,
}

impl JsonEvent {
    pub fn now(event: JsonEventKind) -> Self {
        Self {
            ts: Utc::now(),
            event,
        }
    }
}

/// Thread-safe writer guard so concurrent tasks don't interleave bytes
/// inside a single JSON object. The runtime hands every command its
/// own [`OutputWriter`] handle from [`OutputWriter::stdout`].
pub struct OutputWriter {
    mode: OutputMode,
    out: Mutex<Box<dyn Write + Send>>,
}

impl OutputWriter {
    pub fn stdout(mode: OutputMode) -> Self {
        Self {
            mode,
            out: Mutex::new(Box::new(io::stdout())),
        }
    }

    /// Test-only constructor for capturing output into a buffer.
    pub fn into_writer(mode: OutputMode, w: Box<dyn Write + Send>) -> Self {
        Self {
            mode,
            out: Mutex::new(w),
        }
    }

    pub fn mode(&self) -> OutputMode {
        self.mode
    }

    /// Emit one JSON-Lines record. A no-op outside `Json` mode.
    pub fn emit(&self, event: JsonEventKind) -> io::Result<()> {
        if self.mode != OutputMode::Json {
            return Ok(());
        }
        let wrapped = JsonEvent::now(event);
        let mut line = serde_json::to_vec(&wrapped)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        line.push(b'\n');
        let mut guard = self.out.lock().expect("output mutex poisoned");
        guard.write_all(&line)?;
        guard.flush()
    }

    /// Print human-readable progress. A no-op in JSON or Quiet mode.
    pub fn human(&self, line: &str) -> io::Result<()> {
        if self.mode != OutputMode::Human {
            return Ok(());
        }
        let mut guard = self.out.lock().expect("output mutex poisoned");
        writeln!(guard, "{line}")?;
        guard.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_mode_round_trips_version_event() {
        let event = JsonEventKind::Version {
            version: "1.0.0".into(),
            crate_name: "copythat-cli".into(),
            rustc_known_at_compile: true,
        };
        let buf = std::sync::Arc::new(std::sync::Mutex::new(Vec::<u8>::new()));
        struct ArcWriter(std::sync::Arc<std::sync::Mutex<Vec<u8>>>);
        impl Write for ArcWriter {
            fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
                self.0.lock().unwrap().extend_from_slice(b);
                Ok(b.len())
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
        let writer = OutputWriter::into_writer(
            OutputMode::Json,
            Box::new(ArcWriter(std::sync::Arc::clone(&buf))),
        );
        writer.emit(event).unwrap();
        let bytes = buf.lock().unwrap().clone();
        let text = String::from_utf8(bytes).unwrap();
        assert!(text.ends_with('\n'));
        let parsed: JsonEvent = serde_json::from_str(text.trim()).unwrap();
        match parsed.event {
            JsonEventKind::Version {
                version,
                crate_name,
                ..
            } => {
                assert_eq!(version, "1.0.0");
                assert_eq!(crate_name, "copythat-cli");
            }
            _ => panic!("unexpected variant"),
        }
    }

    #[test]
    fn human_mode_writes_plain_text() {
        let buf = std::sync::Arc::new(std::sync::Mutex::new(Vec::<u8>::new()));
        struct ArcWriter(std::sync::Arc<std::sync::Mutex<Vec<u8>>>);
        impl Write for ArcWriter {
            fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
                self.0.lock().unwrap().extend_from_slice(b);
                Ok(b.len())
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
        let writer = OutputWriter::into_writer(
            OutputMode::Human,
            Box::new(ArcWriter(std::sync::Arc::clone(&buf))),
        );
        writer.human("copy: foo -> bar (1.2 GiB)").unwrap();
        let text = String::from_utf8(buf.lock().unwrap().clone()).unwrap();
        assert_eq!(text, "copy: foo -> bar (1.2 GiB)\n");
    }
}
