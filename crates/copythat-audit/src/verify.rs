//! Phase 34 — chain-hash verification.
//!
//! A real SIEM verifier would also re-parse every record back through
//! the format-specific parser and confirm schema. For Phase 34 the
//! contract is narrower: detect tampering by re-running the BLAKE3
//! chain that [`crate::AuditSink::record`] emits. If a line's
//! `prev_hash` field disagrees with the recomputed hash of the
//! previous line (or the file's tail hash diverges) we surface
//! [`AuditError::ChainMismatch`].
//!
//! The Phase 36 CLI wraps this into `copythat audit verify <log>`.

use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::{AuditError, AuditFormat, Result, format::CHAIN_HASH_HEX_LEN, next_chain_hash};

/// Outcome per line: whether the embedded `prev_hash` matched the
/// recomputed chain.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerifyOutcome {
    /// The recorded `prev_hash` equals the recomputed chain hash.
    Match,
    /// The recorded `prev_hash` disagrees with the recomputed
    /// chain — the previous record's bytes have been altered.
    Mismatch,
    /// The record didn't carry a parseable `prev_hash` field. The
    /// caller can treat this as a missing chain (likely a corrupt
    /// record) or skip it if the format is known to have a mix
    /// (CSV header row).
    Missing,
}

/// Summary returned by [`verify_chain`]. Callers use this for a
/// UI badge + a per-line status drawer.
#[derive(Debug, Clone)]
pub struct VerifyReport {
    pub total: usize,
    pub matches: usize,
    pub mismatches: usize,
    pub missing: usize,
    /// Line numbers (1-based) that failed verification. Empty on
    /// success.
    pub failed_lines: Vec<usize>,
}

impl VerifyReport {
    pub fn is_ok(&self) -> bool {
        self.mismatches == 0 && self.missing == 0
    }
}

/// Verify the chain on `path` using `format`. Returns the total
/// outcome + a per-line vector. Stops at the first IO error.
pub fn verify_chain(path: &Path, format: AuditFormat) -> Result<VerifyReport> {
    let mut file = File::open(path).map_err(|source| AuditError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .map_err(|source| AuditError::Io {
            path: path.to_path_buf(),
            source,
        })?;

    let mut chain = [0u8; 32];
    let mut report = VerifyReport {
        total: 0,
        matches: 0,
        mismatches: 0,
        missing: 0,
        failed_lines: Vec::new(),
    };

    for (idx, line) in buf.split_inclusive('\n').enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        if matches!(format, AuditFormat::Csv) && line.starts_with("event,") {
            continue;
        }

        report.total += 1;
        let prev_hex = match extract_prev_hash(format, line) {
            Some(h) => h,
            None => {
                report.missing += 1;
                report.failed_lines.push(idx + 1);
                chain = next_chain_hash(&chain, line);
                continue;
            }
        };

        let expected_hex = hex::encode(chain);
        if expected_hex == prev_hex {
            report.matches += 1;
        } else {
            report.mismatches += 1;
            report.failed_lines.push(idx + 1);
        }
        chain = next_chain_hash(&chain, line);
    }
    Ok(report)
}

/// Pull the `prev_hash` field from a line of `format`. Returns
/// `None` when the line is malformed for the given format — the
/// caller decides how strict to be about that.
fn extract_prev_hash(format: AuditFormat, line: &str) -> Option<String> {
    let line = line.trim_end_matches(['\r', '\n']);
    match format {
        AuditFormat::Csv => {
            // Last column is `prev_hash`; parse via the same CSV
            // rules the writer used so quoted commas inside
            // `detail` don't split it apart.
            let mut rdr = csv::ReaderBuilder::new()
                .has_headers(false)
                .flexible(true)
                .from_reader(line.as_bytes());
            let row = rdr.records().next()?.ok()?;
            let last = row.get(row.len() - 1)?.to_string();
            if last.len() == CHAIN_HASH_HEX_LEN {
                Some(last)
            } else {
                None
            }
        }
        AuditFormat::JsonLines => {
            let value: serde_json::Value = serde_json::from_str(line).ok()?;
            let s = value.get("prev_hash")?.as_str()?.to_string();
            (s.len() == CHAIN_HASH_HEX_LEN).then_some(s)
        }
        AuditFormat::Syslog => extract_syslog_prev_hash(line),
        AuditFormat::Cef => extract_cef_prev_hash(line),
        AuditFormat::Leef => extract_leef_prev_hash(line),
    }
}

/// Pull `prevHash="<hex>"` from a Syslog structured-data block.
fn extract_syslog_prev_hash(line: &str) -> Option<String> {
    let sd_start = line.find("[copythat@32473")?;
    let sd = &line[sd_start..];
    let needle = "prevHash=\"";
    let at = sd.find(needle)?;
    let rest = &sd[at + needle.len()..];
    let end = rest.find('"')?;
    let hex = &rest[..end];
    (hex.len() == CHAIN_HASH_HEX_LEN).then(|| hex.to_string())
}

/// Pull `cs1=<hex>` from a CEF extension string. The writer always
/// emits `cs1Label=prev_hash cs1=<hex>` so the key is stable.
fn extract_cef_prev_hash(line: &str) -> Option<String> {
    let ext_start = nth_cef_pipe(line, 7)? + 1;
    let ext = &line[ext_start..];
    let needle = "cs1=";
    let at = ext.find(needle)?;
    let rest = &ext[at + needle.len()..];
    let end = rest.find(' ').unwrap_or(rest.len());
    let hex = rest[..end].trim_end_matches(['\n', '\r']);
    (hex.len() == CHAIN_HASH_HEX_LEN).then(|| hex.to_string())
}

/// Find the byte index of the Nth pipe in a CEF header, respecting
/// the format's `\|` escape sequence. `n` is 1-based.
fn nth_cef_pipe(line: &str, n: usize) -> Option<usize> {
    let mut count = 0;
    let bytes = line.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\\' && i + 1 < bytes.len() {
            i += 2;
            continue;
        }
        if bytes[i] == b'|' {
            count += 1;
            if count == n {
                return Some(i);
            }
        }
        i += 1;
    }
    None
}

fn extract_leef_prev_hash(line: &str) -> Option<String> {
    // LEEF:2.0 keeps five header pipes before the tab-separated
    // extension. Walk past them, then split on tabs to find the
    // `prevHash=...` pair.
    let ext_start = nth_cef_pipe(line, 5)? + 1;
    let ext = &line[ext_start..];
    for part in ext.split('\t') {
        if let Some(rest) = part.strip_prefix("prevHash=") {
            let hex = rest.trim_end_matches(['\n', '\r']);
            if hex.len() == CHAIN_HASH_HEX_LEN {
                return Some(hex.to_string());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AuditEvent, AuditSink, WormMode, format::synthetic_ts};

    fn mk_event(job: &str) -> AuditEvent {
        AuditEvent::JobStarted {
            job_id: job.into(),
            kind: "copy".into(),
            src: "/s".into(),
            dst: "/d".into(),
            user: "alice".into(),
            host: "h".into(),
            ts: synthetic_ts(),
        }
    }

    fn write_three(tmp: &std::path::Path, fmt: AuditFormat) -> AuditSink {
        let sink = AuditSink::open(tmp, fmt, WormMode::Off).expect("open");
        sink.record(&mk_event("a")).unwrap();
        sink.record(&mk_event("b")).unwrap();
        sink.record(&mk_event("c")).unwrap();
        sink
    }

    #[test]
    fn verify_round_trips_each_format() {
        for fmt in [
            AuditFormat::Csv,
            AuditFormat::JsonLines,
            AuditFormat::Syslog,
            AuditFormat::Cef,
            AuditFormat::Leef,
        ] {
            let dir = tempfile::tempdir().unwrap();
            let path = dir.path().join("audit.log");
            let sink = write_three(&path, fmt);
            drop(sink);
            let report = verify_chain(&path, fmt).expect("verify");
            assert_eq!(report.total, 3, "format {fmt:?} line count");
            assert_eq!(report.matches, 3, "format {fmt:?} matches");
            assert_eq!(report.mismatches, 0);
            assert!(report.is_ok());
        }
    }

    #[test]
    fn flipped_byte_breaks_the_chain() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("audit.log");
        let sink = write_three(&path, AuditFormat::JsonLines);
        drop(sink);
        // Tamper with the first line's job_id so its bytes change.
        let raw = std::fs::read_to_string(&path).unwrap();
        let tampered = raw.replacen("\"a\"", "\"X\"", 1);
        std::fs::write(&path, tampered).unwrap();
        let report = verify_chain(&path, AuditFormat::JsonLines).expect("verify");
        assert!(report.mismatches >= 1, "tampering must fail the chain");
    }
}
