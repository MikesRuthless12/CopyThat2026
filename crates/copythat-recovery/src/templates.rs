//! askama Template structs.
//!
//! Templates live under `templates/` next to `Cargo.toml` and are
//! compiled into the binary by askama's proc-macro. A typo in any
//! template variable surfaces as a `cargo build` error rather than a
//! 500 at runtime.

use askama::Template;

/// Per-job row on the landing + jobs-list pages.
pub(crate) struct JobView {
    pub row_id: i64,
    pub kind: String,
    pub status: String,
    pub started_iso: String,
    pub src_root: String,
    pub dst_root: String,
    pub total_bytes_human: String,
    pub files_ok: u64,
    pub files_failed: u64,
}

/// Per-item row on the job-detail page.
pub(crate) struct ItemView {
    pub src: String,
    pub dst: String,
    pub size_human: String,
    pub status: String,
    pub error_msg: Option<String>,
}

#[derive(Template)]
#[template(path = "landing.html")]
pub(crate) struct LandingTemplate {
    pub recent: Vec<JobView>,
}

#[derive(Template)]
#[template(path = "jobs.html")]
pub(crate) struct JobsTemplate {
    pub jobs: Vec<JobView>,
    pub kind_filter: String,
    pub status_filter: String,
    pub text_filter: String,
}

#[derive(Template)]
#[template(path = "job_detail.html")]
pub(crate) struct JobDetailTemplate {
    pub job: JobView,
    pub items: Vec<ItemView>,
}

#[derive(Template)]
#[template(path = "restore.html")]
pub(crate) struct RestoreTemplate {
    pub jobs: Vec<JobView>,
}

#[derive(Template)]
#[template(path = "error.html")]
pub(crate) struct ErrorTemplate {
    pub status_code: u16,
    pub status_text: String,
    pub message: String,
}

/// Format a byte count into "1.2 GiB" / "5.6 MiB" / "823 B" for
/// human display in the UI.
pub(crate) fn human_bytes(n: u64) -> String {
    const UNITS: &[&str] = &["B", "KiB", "MiB", "GiB", "TiB", "PiB"];
    if n < 1024 {
        return format!("{n} B");
    }
    let mut value = n as f64;
    let mut idx = 0;
    while value >= 1024.0 && idx + 1 < UNITS.len() {
        value /= 1024.0;
        idx += 1;
    }
    format!("{value:.2} {}", UNITS[idx])
}

/// ISO-8601 UTC representation of a Unix-epoch-millisecond timestamp.
/// Avoids pulling `chrono`/`time` into the dep tree just to render a
/// timestamp string.
pub(crate) fn iso_from_ms(ms: i64) -> String {
    if ms <= 0 {
        return "—".to_string();
    }
    // Days since epoch + remaining seconds; the calendar math below
    // is a Howard Hinnant–style civil_from_days conversion.
    let secs = ms / 1000;
    let days_since_epoch = secs.div_euclid(86_400);
    let secs_in_day = secs.rem_euclid(86_400) as u64;
    let h = secs_in_day / 3600;
    let m = (secs_in_day / 60) % 60;
    let s = secs_in_day % 60;
    let (y, mo, d) = civil_from_days(days_since_epoch);
    format!("{y:04}-{mo:02}-{d:02} {h:02}:{m:02}:{s:02}Z")
}

fn civil_from_days(z: i64) -> (i32, u32, u32) {
    // Howard Hinnant's date algorithm. Days since 1970-01-01 →
    // proleptic Gregorian (year, month, day).
    let z = z + 719_468;
    let era = if z >= 0 {
        z / 146_097
    } else {
        (z - 146_096) / 146_097
    };
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y as i32, m as u32, d as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn human_bytes_picks_unit_correctly() {
        assert_eq!(human_bytes(0), "0 B");
        assert_eq!(human_bytes(512), "512 B");
        assert_eq!(human_bytes(1024), "1.00 KiB");
        assert_eq!(human_bytes(1536), "1.50 KiB");
        assert_eq!(human_bytes(1024 * 1024), "1.00 MiB");
        assert_eq!(human_bytes(1024u64.pow(3)), "1.00 GiB");
    }

    #[test]
    fn iso_from_ms_handles_known_dates() {
        assert_eq!(iso_from_ms(0), "—");
        // 2026-04-28 14:30:00 UTC = 1_777_473_000_000 ms
        let s = iso_from_ms(1_777_473_000_000);
        assert!(s.starts_with("2026-04-"), "got `{s}`");
        assert!(s.ends_with("Z"));
    }
}
