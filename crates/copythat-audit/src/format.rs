//! Phase 34 — wire-format serialisers.
//!
//! Each `format_*` function emits a complete line including the
//! trailing `\n`. The chain-hash column is included in the bytes
//! so the next record's BLAKE3 chain step runs over the full
//! previous record including its own hash field.
//!
//! The enterprise formats (Syslog / CEF / LEEF) have strict
//! escaping rules; the helpers here mirror each spec:
//!
//! - RFC 5424 §6 — structured-data parameter values must escape
//!   `\`, `"`, `]` with a preceding `\`.
//! - CEF v0 — header pipes are literal separators, so pipes and
//!   backslashes in any header field must escape. Extension keys
//!   are `key=value`; `=`, `\`, `\n`, `\r` in values escape.
//! - LEEF 2.0 — identical header rules, extension is tab-separated
//!   `key=value` pairs with the same in-value escapes as CEF.
//!
//! The escapers are pure — tested independently + reused from the
//! Phase 36 CLI verify path.

use chrono::{DateTime, SecondsFormat, Utc};
use serde::Serialize;

use crate::{AuditError, AuditEvent, AuditFormat, AuditSeverity, Result};

/// Hex length of the BLAKE3 chain hash included in every record.
/// 32 bytes encodes to 64 hex chars.
pub const CHAIN_HASH_HEX_LEN: usize = 64;

/// CSV column order. Kept stable across minor releases so
/// downstream spreadsheets keep working.
pub const CSV_COLUMNS: &[&str] = &[
    "event",
    "ts",
    "job_id",
    "user",
    "host",
    "severity",
    "detail",
    "prev_hash",
];

/// Header row the sink writes on first open / after rotation.
pub fn csv_header() -> String {
    format!("{}\n", CSV_COLUMNS.join(","))
}

/// Syslog RFC 5424 `APP-NAME` identifier. `copythat@32473` is the
/// structured-data section's enterprise ID; 32473 is IANA's
/// example enterprise number, which the spec explicitly reserves
/// for documentation / test / "I don't have a PEN yet" usage. A
/// real deployment sets their own via
/// `SettingsChanged.field = "audit.enterprise_id"` — not wired in
/// Phase 34, documented for Phase 36's CLI.
pub const fn syslog_app_name() -> &'static str {
    "CopyThat"
}

// ---------------------------------------------------------------------
// Dispatcher
// ---------------------------------------------------------------------

/// Serialize `event` to the line-protocol shape of `format`, prefixing
/// the chain-hash column. Returns the full line (trailing `\n`).
pub fn format_record(
    format: AuditFormat,
    event: &AuditEvent,
    prev_hash_hex: &str,
) -> Result<String> {
    debug_assert_eq!(prev_hash_hex.len(), CHAIN_HASH_HEX_LEN);
    match format {
        AuditFormat::Csv => format_csv(event, prev_hash_hex),
        AuditFormat::JsonLines => format_json_lines(event, prev_hash_hex),
        AuditFormat::Syslog => format_syslog(event, prev_hash_hex),
        AuditFormat::Cef => format_cef(event, prev_hash_hex),
        AuditFormat::Leef => format_leef(event, prev_hash_hex),
    }
}

// ---------------------------------------------------------------------
// CSV
// ---------------------------------------------------------------------

fn format_csv(event: &AuditEvent, prev_hash_hex: &str) -> Result<String> {
    let mut buf = Vec::new();
    {
        let mut wtr = csv::WriterBuilder::new()
            .has_headers(false)
            .terminator(csv::Terminator::Any(b'\n'))
            .from_writer(&mut buf);
        // Sanitise every column whose first character could be
        // interpreted as a formula by Excel / LibreOffice on
        // download. The four leading bytes `=`, `+`, `@`, `-` are
        // the canonical CSV-injection vectors; prefix them with a
        // single quote so the spreadsheet treats the cell as text.
        // Stable columns (signature, timestamp, severity token,
        // hex chain hash) can never trip the rule, so leave those
        // untouched to keep machine-parsable round-trips intact.
        let detail = csv_detail(event);
        let job_id = csv_sanitise_for_excel(event.job_id());
        let user = csv_sanitise_for_excel(event.user());
        let host = csv_sanitise_for_excel(host_field(event));
        let detail = csv_sanitise_for_excel(&detail);
        wtr.write_record([
            event.signature(),
            &event.ts_iso8601(),
            &job_id,
            &user,
            &host,
            severity_token(event.severity()),
            &detail,
            prev_hash_hex,
        ])
        .map_err(|e| AuditError::Format(format!("csv write: {e}")))?;
        wtr.flush()
            .map_err(|e| AuditError::Format(format!("csv flush: {e}")))?;
    }
    String::from_utf8(buf).map_err(|e| AuditError::Format(format!("csv utf8: {e}")))
}

/// CSV-injection guard: when a cell's first byte is one of the four
/// formula-trigger characters Excel / LibreOffice / Numbers
/// auto-evaluate (`=`, `+`, `@`, `-`), prefix the cell with a
/// literal single-quote so the spreadsheet engine treats it as text.
/// The single quote is the canonical Excel "this is a string"
/// escape; consumers that read the file with a real CSV parser
/// (Python pandas, Rust csv, etc.) get the original bytes back
/// because the quote is part of the cell value, not an opening
/// delimiter. Tab / `\r` / newline payloads are already handled by
/// the upstream `csv::Writer` quoting.
pub fn csv_sanitise_for_excel(raw: &str) -> String {
    if let Some(first) = raw.as_bytes().first()
        && matches!(first, b'=' | b'+' | b'@' | b'-')
    {
        let mut out = String::with_capacity(raw.len() + 1);
        out.push('\'');
        out.push_str(raw);
        out
    } else {
        raw.to_string()
    }
}

/// Flatten the event-specific payload into a single column so the
/// header stays stable. CSV consumers that want the full surface
/// parse JSON-lines instead.
fn csv_detail(event: &AuditEvent) -> String {
    match event {
        AuditEvent::JobStarted { kind, src, dst, .. } => {
            format!("kind={kind}; src={src}; dst={dst}")
        }
        AuditEvent::JobCompleted {
            status,
            files_ok,
            files_failed,
            bytes,
            duration_ms,
            ..
        } => format!(
            "status={status}; ok={files_ok}; failed={files_failed}; \
             bytes={bytes}; duration_ms={duration_ms}"
        ),
        AuditEvent::FileCopied {
            src,
            dst,
            hash,
            size,
            ..
        } => {
            format!("src={src}; dst={dst}; size={size}; hash={hash}")
        }
        AuditEvent::FileFailed {
            src,
            error_code,
            error_msg,
            ..
        } => format!("src={src}; code={error_code}; msg={error_msg}"),
        AuditEvent::CollisionResolved {
            src, dst, action, ..
        } => {
            format!("src={src}; dst={dst}; action={action}")
        }
        AuditEvent::SettingsChanged {
            field,
            before_hash,
            after_hash,
            ..
        } => format!("field={field}; before={before_hash}; after={after_hash}"),
        AuditEvent::LoginEvent { .. } => String::new(),
        AuditEvent::UnauthorizedAccess {
            attempted_action,
            reason,
            ..
        } => format!("action={attempted_action}; reason={reason}"),
    }
}

// ---------------------------------------------------------------------
// JSON-lines
// ---------------------------------------------------------------------

fn format_json_lines(event: &AuditEvent, prev_hash_hex: &str) -> Result<String> {
    #[derive(Serialize)]
    struct JsonRecord<'a> {
        #[serde(flatten)]
        event: &'a AuditEvent,
        severity: &'static str,
        prev_hash: &'a str,
    }
    let rec = JsonRecord {
        event,
        severity: severity_token(event.severity()),
        prev_hash: prev_hash_hex,
    };
    let mut line =
        serde_json::to_string(&rec).map_err(|e| AuditError::Format(format!("json: {e}")))?;
    line.push('\n');
    Ok(line)
}

// ---------------------------------------------------------------------
// Syslog RFC 5424
// ---------------------------------------------------------------------

fn format_syslog(event: &AuditEvent, prev_hash_hex: &str) -> Result<String> {
    // <PRI>1 TIMESTAMP HOSTNAME APP-NAME PROCID MSGID [SD-ID param="value"...] MSG
    let pri = event.severity().syslog_priority();
    let timestamp = event
        .timestamp()
        .to_rfc3339_opts(SecondsFormat::Micros, true);
    let host = host_field(event);
    let host = if host.is_empty() { "-" } else { host };
    let procid = std::process::id();
    let msgid = event.signature();
    let sd = syslog_structured_data(event, prev_hash_hex);
    let msg = event.label();
    Ok(format!(
        "<{pri}>1 {timestamp} {host} {app} {procid} {msgid} {sd} {msg}\n",
        app = syslog_app_name()
    ))
}

fn syslog_structured_data(event: &AuditEvent, prev_hash_hex: &str) -> String {
    // `copythat@32473` — see `syslog_app_name()` doc. The params
    // carry the shared axes (`jobId`, `user`, `severity`, `prevHash`)
    // plus a short variant-specific summary so a SIEM's structured-
    // data indexer surfaces the right fields.
    let variant = variant_sd_params(event);
    let mut sd = format!(
        "[copythat@32473 jobId=\"{}\" user=\"{}\" severity=\"{}\" prevHash=\"{}\"",
        escape_syslog_param(event.job_id()),
        escape_syslog_param(event.user()),
        severity_token(event.severity()),
        prev_hash_hex,
    );
    for (key, value) in variant {
        sd.push(' ');
        sd.push_str(key);
        sd.push_str("=\"");
        sd.push_str(&escape_syslog_param(&value));
        sd.push('"');
    }
    sd.push(']');
    sd
}

fn variant_sd_params(event: &AuditEvent) -> Vec<(&'static str, String)> {
    match event {
        AuditEvent::JobStarted { kind, src, dst, .. } => vec![
            ("kind", kind.clone()),
            ("src", src.clone()),
            ("dst", dst.clone()),
        ],
        AuditEvent::JobCompleted {
            status,
            files_ok,
            files_failed,
            bytes,
            duration_ms,
            ..
        } => vec![
            ("status", status.clone()),
            ("filesOk", files_ok.to_string()),
            ("filesFailed", files_failed.to_string()),
            ("bytes", bytes.to_string()),
            ("durationMs", duration_ms.to_string()),
        ],
        AuditEvent::FileCopied {
            src,
            dst,
            hash,
            size,
            ..
        } => vec![
            ("src", src.clone()),
            ("dst", dst.clone()),
            ("size", size.to_string()),
            ("hash", hash.clone()),
        ],
        AuditEvent::FileFailed {
            src,
            error_code,
            error_msg,
            ..
        } => vec![
            ("src", src.clone()),
            ("errorCode", error_code.clone()),
            ("errorMsg", error_msg.clone()),
        ],
        AuditEvent::CollisionResolved {
            src, dst, action, ..
        } => vec![
            ("src", src.clone()),
            ("dst", dst.clone()),
            ("action", action.clone()),
        ],
        AuditEvent::SettingsChanged {
            field,
            before_hash,
            after_hash,
            ..
        } => vec![
            ("field", field.clone()),
            ("beforeHash", before_hash.clone()),
            ("afterHash", after_hash.clone()),
        ],
        AuditEvent::LoginEvent { .. } => Vec::new(),
        AuditEvent::UnauthorizedAccess {
            attempted_action,
            reason,
            ..
        } => vec![
            ("attemptedAction", attempted_action.clone()),
            ("reason", reason.clone()),
        ],
    }
}

/// RFC 5424 §6.3.3 — escape `"`, `\`, and `]` in SD-PARAM values.
fn escape_syslog_param(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    for ch in raw.chars() {
        match ch {
            '"' | '\\' | ']' => {
                out.push('\\');
                out.push(ch);
            }
            _ => out.push(ch),
        }
    }
    out
}

// ---------------------------------------------------------------------
// CEF (ArcSight)
// ---------------------------------------------------------------------

fn format_cef(event: &AuditEvent, prev_hash_hex: &str) -> Result<String> {
    let version = env!("CARGO_PKG_VERSION");
    let ext = cef_extension(event, prev_hash_hex);
    Ok(format!(
        "CEF:0|CopyThat|CopyThat|{version}|{sig}|{name}|{sev}|{ext}\n",
        sig = cef_escape_header(event.signature()),
        name = cef_escape_header(event.label()),
        sev = event.severity().cef_severity(),
    ))
}

fn cef_extension(event: &AuditEvent, prev_hash_hex: &str) -> String {
    // Standard CEF CDT keys are reused where they fit; custom
    // cs* / cn* slots cover the rest.
    let ts = event.ts_iso8601();
    let host = host_field(event);
    let mut parts: Vec<(&'static str, String)> = vec![
        ("rt", ts.clone()),
        ("dhost", host.to_string()),
        ("duser", event.user().to_string()),
        ("cs1Label", "prev_hash".into()),
        ("cs1", prev_hash_hex.to_string()),
    ];
    match event {
        AuditEvent::JobStarted {
            job_id,
            kind,
            src,
            dst,
            ..
        } => {
            parts.push(("cs2Label", "jobId".into()));
            parts.push(("cs2", job_id.clone()));
            parts.push(("cs3Label", "kind".into()));
            parts.push(("cs3", kind.clone()));
            parts.push(("fname", src.clone()));
            parts.push(("filePath", dst.clone()));
        }
        AuditEvent::JobCompleted {
            job_id,
            status,
            files_ok,
            files_failed,
            bytes,
            duration_ms,
            ..
        } => {
            parts.push(("cs2Label", "jobId".into()));
            parts.push(("cs2", job_id.clone()));
            parts.push(("cs3Label", "status".into()));
            parts.push(("cs3", status.clone()));
            parts.push(("cn1Label", "filesOk".into()));
            parts.push(("cn1", files_ok.to_string()));
            parts.push(("cn2Label", "filesFailed".into()));
            parts.push(("cn2", files_failed.to_string()));
            parts.push(("cn3Label", "bytes".into()));
            parts.push(("cn3", bytes.to_string()));
            parts.push(("cfp1Label", "durationMs".into()));
            parts.push(("cfp1", duration_ms.to_string()));
        }
        AuditEvent::FileCopied {
            job_id,
            src,
            dst,
            hash,
            size,
            ..
        } => {
            parts.push(("cs2Label", "jobId".into()));
            parts.push(("cs2", job_id.clone()));
            parts.push(("fname", src.clone()));
            parts.push(("filePath", dst.clone()));
            parts.push(("fsize", size.to_string()));
            parts.push(("fileHash", hash.clone()));
        }
        AuditEvent::FileFailed {
            job_id,
            src,
            error_code,
            error_msg,
            ..
        } => {
            parts.push(("cs2Label", "jobId".into()));
            parts.push(("cs2", job_id.clone()));
            parts.push(("fname", src.clone()));
            parts.push(("reason", error_msg.clone()));
            parts.push(("outcome", error_code.clone()));
        }
        AuditEvent::CollisionResolved {
            job_id,
            src,
            dst,
            action,
            ..
        } => {
            parts.push(("cs2Label", "jobId".into()));
            parts.push(("cs2", job_id.clone()));
            parts.push(("fname", src.clone()));
            parts.push(("filePath", dst.clone()));
            parts.push(("act", action.clone()));
        }
        AuditEvent::SettingsChanged {
            field,
            before_hash,
            after_hash,
            ..
        } => {
            parts.push(("cs2Label", "field".into()));
            parts.push(("cs2", field.clone()));
            parts.push(("cs3Label", "beforeHash".into()));
            parts.push(("cs3", before_hash.clone()));
            parts.push(("cs4Label", "afterHash".into()));
            parts.push(("cs4", after_hash.clone()));
        }
        AuditEvent::LoginEvent { .. } => {
            parts.push(("act", "login".into()));
        }
        AuditEvent::UnauthorizedAccess {
            attempted_action,
            reason,
            ..
        } => {
            parts.push(("act", attempted_action.clone()));
            parts.push(("reason", reason.clone()));
            parts.push(("outcome", "denied".into()));
        }
    }
    parts
        .into_iter()
        .map(|(k, v)| format!("{k}={}", cef_escape_extension(&v)))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Escape CEF header-field characters: `\` and `|`.
pub fn cef_escape_header(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    for ch in raw.chars() {
        match ch {
            '\\' | '|' => {
                out.push('\\');
                out.push(ch);
            }
            _ => out.push(ch),
        }
    }
    out
}

/// Escape CEF extension-value characters: `\`, `=`, `\r`, `\n`.
pub fn cef_escape_extension(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    for ch in raw.chars() {
        match ch {
            '\\' | '=' => {
                out.push('\\');
                out.push(ch);
            }
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            _ => out.push(ch),
        }
    }
    out
}

// ---------------------------------------------------------------------
// LEEF (QRadar)
// ---------------------------------------------------------------------

fn format_leef(event: &AuditEvent, prev_hash_hex: &str) -> Result<String> {
    let version = env!("CARGO_PKG_VERSION");
    let ext = leef_extension(event, prev_hash_hex);
    // Header fields use pipe separators; extension is tab-separated
    // key=value pairs. The QRadar parser is strict about the five
    // pipe-separated header tokens so escape them identically to
    // CEF.
    Ok(format!(
        "LEEF:2.0|CopyThat|CopyThat|{version}|{eid}|{ext}\n",
        eid = cef_escape_header(event.signature()),
    ))
}

fn leef_extension(event: &AuditEvent, prev_hash_hex: &str) -> String {
    let ts = event.ts_iso8601();
    let host = host_field(event);
    let mut parts: Vec<(&'static str, String)> = vec![
        ("devTime", ts.clone()),
        ("dstHost", host.to_string()),
        ("usrName", event.user().to_string()),
        ("sev", event.severity().cef_severity().to_string()),
        ("prevHash", prev_hash_hex.to_string()),
        ("signature", event.signature().to_string()),
    ];
    match event {
        AuditEvent::JobStarted {
            job_id,
            kind,
            src,
            dst,
            ..
        } => {
            parts.push(("jobId", job_id.clone()));
            parts.push(("cat", kind.clone()));
            parts.push(("src", src.clone()));
            parts.push(("dst", dst.clone()));
        }
        AuditEvent::JobCompleted {
            job_id,
            status,
            files_ok,
            files_failed,
            bytes,
            duration_ms,
            ..
        } => {
            parts.push(("jobId", job_id.clone()));
            parts.push(("status", status.clone()));
            parts.push(("filesOk", files_ok.to_string()));
            parts.push(("filesFailed", files_failed.to_string()));
            parts.push(("bytes", bytes.to_string()));
            parts.push(("durationMs", duration_ms.to_string()));
        }
        AuditEvent::FileCopied {
            job_id,
            src,
            dst,
            hash,
            size,
            ..
        } => {
            parts.push(("jobId", job_id.clone()));
            parts.push(("src", src.clone()));
            parts.push(("dst", dst.clone()));
            parts.push(("size", size.to_string()));
            parts.push(("hash", hash.clone()));
        }
        AuditEvent::FileFailed {
            job_id,
            src,
            error_code,
            error_msg,
            ..
        } => {
            parts.push(("jobId", job_id.clone()));
            parts.push(("src", src.clone()));
            parts.push(("errorCode", error_code.clone()));
            parts.push(("errorMsg", error_msg.clone()));
        }
        AuditEvent::CollisionResolved {
            job_id,
            src,
            dst,
            action,
            ..
        } => {
            parts.push(("jobId", job_id.clone()));
            parts.push(("src", src.clone()));
            parts.push(("dst", dst.clone()));
            parts.push(("action", action.clone()));
        }
        AuditEvent::SettingsChanged {
            field,
            before_hash,
            after_hash,
            ..
        } => {
            parts.push(("field", field.clone()));
            parts.push(("beforeHash", before_hash.clone()));
            parts.push(("afterHash", after_hash.clone()));
        }
        AuditEvent::LoginEvent { .. } => {}
        AuditEvent::UnauthorizedAccess {
            attempted_action,
            reason,
            ..
        } => {
            parts.push(("attemptedAction", attempted_action.clone()));
            parts.push(("reason", reason.clone()));
        }
    }
    parts
        .into_iter()
        .map(|(k, v)| format!("{k}={}", leef_escape_extension(&v)))
        .collect::<Vec<_>>()
        .join("\t")
}

/// Escape LEEF extension values — same rules as CEF extension.
pub fn leef_escape_extension(raw: &str) -> String {
    cef_escape_extension(raw)
}

// ---------------------------------------------------------------------
// Shared helpers
// ---------------------------------------------------------------------

fn severity_token(sev: AuditSeverity) -> &'static str {
    match sev {
        AuditSeverity::Debug => "debug",
        AuditSeverity::Info => "info",
        AuditSeverity::Notice => "notice",
        AuditSeverity::Warning => "warning",
        AuditSeverity::Error => "error",
    }
}

fn host_field(event: &AuditEvent) -> &str {
    match event {
        AuditEvent::JobStarted { host, .. }
        | AuditEvent::SettingsChanged { host, .. }
        | AuditEvent::LoginEvent { host, .. }
        | AuditEvent::UnauthorizedAccess { host, .. } => host.as_str(),
        _ => "",
    }
}

/// Placeholder UTC timestamp used in doc-tests + unit-tests. Real
/// callers always pass a live `DateTime<Utc>` via the event.
#[doc(hidden)]
pub fn synthetic_ts() -> DateTime<Utc> {
    use chrono::TimeZone;
    Utc.with_ymd_and_hms(2026, 4, 24, 12, 0, 0).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> AuditEvent {
        AuditEvent::JobStarted {
            job_id: "job-7".into(),
            kind: "copy".into(),
            src: "C:\\src".into(),
            dst: "D:\\dst".into(),
            user: "alice".into(),
            host: "ws-1".into(),
            ts: synthetic_ts(),
        }
    }

    #[test]
    fn csv_header_has_stable_columns() {
        assert_eq!(
            csv_header(),
            "event,ts,job_id,user,host,severity,detail,prev_hash\n"
        );
    }

    #[test]
    fn cef_escape_header_escapes_pipe_and_backslash() {
        assert_eq!(cef_escape_header("a|b\\c"), "a\\|b\\\\c");
    }

    #[test]
    fn cef_escape_extension_escapes_equals_and_newline() {
        assert_eq!(cef_escape_extension("k=v\nx"), "k\\=v\\nx");
    }

    #[test]
    fn syslog_param_escapes_quote_and_bracket() {
        assert_eq!(escape_syslog_param("a\"b]c\\d"), "a\\\"b\\]c\\\\d");
    }

    #[test]
    fn json_line_parses_back() {
        let line = format_json_lines(&sample(), &"00".repeat(32)).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(line.trim_end()).unwrap();
        assert_eq!(parsed["event"], "job-started");
        assert_eq!(parsed["job_id"], "job-7");
        assert_eq!(parsed["severity"], "info");
        assert_eq!(parsed["prev_hash"].as_str().unwrap().len(), 64);
    }

    #[test]
    fn csv_line_has_eight_columns() {
        let line = format_csv(&sample(), &"00".repeat(32)).unwrap();
        let rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(line.as_bytes());
        let row = rdr.into_records().next().unwrap().unwrap();
        assert_eq!(row.len(), 8);
        assert_eq!(row.get(0).unwrap(), "JobStarted");
    }

    #[test]
    fn syslog_line_starts_with_priority() {
        let line = format_syslog(&sample(), &"00".repeat(32)).unwrap();
        assert!(line.starts_with("<110>1 ")); // facility 13 * 8 + info(6) = 110
        assert!(line.contains("[copythat@32473"));
        assert!(line.contains("jobId=\"job-7\""));
    }

    #[test]
    fn cef_line_has_eight_pipes() {
        let line = format_cef(&sample(), &"00".repeat(32)).unwrap();
        let pipes = line.bytes().filter(|b| *b == b'|').count();
        // 7 separators between 8 header sections, plus any escaped
        // pipes in the extension (none in the sample).
        assert_eq!(pipes, 7);
        assert!(line.starts_with("CEF:0|CopyThat|CopyThat|"));
        assert!(line.contains("cs2=job-7"));
    }

    #[test]
    fn leef_line_uses_tab_extension() {
        let line = format_leef(&sample(), &"00".repeat(32)).unwrap();
        assert!(line.starts_with("LEEF:2.0|CopyThat|CopyThat|"));
        assert!(line.contains("\tjobId=job-7"));
        assert!(line.contains("\tsignature=JobStarted"));
    }

    #[test]
    fn csv_sanitise_for_excel_prefixes_formula_triggers() {
        // The four formula-trigger characters all auto-execute in
        // Excel / LibreOffice when an unquoted cell starts with
        // them. The sanitiser must single-quote the cell so the
        // spreadsheet treats it as text, while leaving non-trigger
        // cells unchanged.
        for (input, expected) in [
            ("=cmd|' /c calc'!A1", "'=cmd|' /c calc'!A1"),
            ("+1+1", "'+1+1"),
            ("@SUM(A1:A2)", "'@SUM(A1:A2)"),
            ("-2+3", "'-2+3"),
            ("HYPERLINK(\"x\")", "HYPERLINK(\"x\")"),
            ("alice", "alice"),
            ("", ""),
        ] {
            assert_eq!(csv_sanitise_for_excel(input), expected, "input {input:?}");
        }
    }

    #[test]
    fn csv_record_with_injection_payload_is_sanitised() {
        // Build an event whose user field carries a formula
        // payload — the kind of data an attacker can plant via a
        // login attempt — and confirm the CSV row's third column
        // has the leading quote prefix.
        let mut evt = sample();
        if let AuditEvent::JobStarted { user, .. } = &mut evt {
            *user = "=cmd|' /c calc'".into();
        }
        let line = format_csv(&evt, &"00".repeat(32)).unwrap();
        let rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(line.as_bytes());
        let row = rdr.into_records().next().unwrap().unwrap();
        let user_col = row.get(3).unwrap();
        assert!(
            user_col.starts_with('\''),
            "user column must be quoted to defuse the formula, got: {user_col:?}",
        );
    }
}
