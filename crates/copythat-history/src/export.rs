//! CSV export for the History tab.
//!
//! Host-independent, sync, pure-Rust: the Tauri command hands a
//! `Vec<JobSummary>` to [`export_csv`] and writes the returned
//! `String` atomically via the Tauri layer's `std::fs::write`. The
//! round-trip is tested host-independently; see
//! `tests/smoke/phase_09_history.rs`.
//!
//! Columns (see the test for the exact header):
//! `id`, `started_at_ms`, `finished_at_ms`, `kind`, `status`,
//! `src_root`, `dst_root`, `total_bytes`, `files_ok`,
//! `files_failed`, `verify_algo`, `options_json`.

use crate::types::JobSummary;

/// RFC-4180 export. Every field is double-quoted; embedded double
/// quotes are doubled; newlines / commas inside fields are safe.
pub fn export_csv(rows: &[JobSummary]) -> String {
    let mut out = String::with_capacity(rows.len() * 128);
    out.push_str(
        "id,started_at_ms,finished_at_ms,kind,status,src_root,dst_root,\
         total_bytes,files_ok,files_failed,verify_algo,options_json\n",
    );
    for r in rows {
        push_field(&mut out, &r.row_id.to_string());
        out.push(',');
        push_field(&mut out, &r.started_at_ms.to_string());
        out.push(',');
        push_field(
            &mut out,
            &r.finished_at_ms.map(|v| v.to_string()).unwrap_or_default(),
        );
        out.push(',');
        push_field(&mut out, &r.kind);
        out.push(',');
        push_field(&mut out, &r.status);
        out.push(',');
        push_field(&mut out, &r.src_root.to_string_lossy());
        out.push(',');
        push_field(&mut out, &r.dst_root.to_string_lossy());
        out.push(',');
        push_field(&mut out, &r.total_bytes.to_string());
        out.push(',');
        push_field(&mut out, &r.files_ok.to_string());
        out.push(',');
        push_field(&mut out, &r.files_failed.to_string());
        out.push(',');
        push_field(&mut out, r.verify_algo.as_deref().unwrap_or(""));
        out.push(',');
        push_field(&mut out, r.options_json.as_deref().unwrap_or(""));
        out.push('\n');
    }
    out
}

fn push_field(out: &mut String, s: &str) {
    out.push('"');
    for c in s.chars() {
        if c == '"' {
            out.push_str("\"\"");
        } else {
            out.push(c);
        }
    }
    out.push('"');
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn sample() -> JobSummary {
        JobSummary {
            row_id: 7,
            kind: "copy".into(),
            status: "succeeded".into(),
            started_at_ms: 1_000,
            finished_at_ms: Some(1_500),
            src_root: PathBuf::from(r"C:\a\weird, path\file.bin"),
            dst_root: PathBuf::from(r"C:\b\out"),
            total_bytes: 2048,
            files_ok: 3,
            files_failed: 1,
            verify_algo: Some("sha256".into()),
            // Non-raw string so `\"` escapes to a real `"` character;
            // the CSV exporter should then double those.
            options_json: Some("{\"buffer\":\"1mb\",\"note\":\"ok\"}".into()),
        }
    }

    #[test]
    fn header_and_single_row_shape() {
        let csv = export_csv(&[sample()]);
        assert_eq!(csv.lines().count(), 2, "header + one body row");
        assert!(csv.starts_with("id,started_at_ms"));
    }

    #[test]
    fn embedded_commas_live_inside_quoted_field() {
        let csv = export_csv(&[sample()]);
        assert!(csv.contains("weird, path"));
    }

    #[test]
    fn embedded_quotes_double() {
        let csv = export_csv(&[sample()]);
        // Input JSON `{"buffer":"1mb","note":"ok"}` encodes as
        // `"{""buffer"":""1mb"",""note"":""ok""}"` in the CSV.
        assert!(
            csv.contains("\"\"ok\"\""),
            "expected `\"\"ok\"\"` in CSV, got:\n{csv}"
        );
    }

    #[test]
    fn optional_fields_render_as_empty_quoted_strings() {
        let mut row = sample();
        row.finished_at_ms = None;
        row.verify_algo = None;
        row.options_json = None;
        let csv = export_csv(&[row]);
        // Every field stays quoted; an empty value is `""`.
        assert!(csv.contains(",\"\","));
    }
}
