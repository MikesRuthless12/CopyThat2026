//! Phase 10 smoke test — cumulative totals + 30-day sparkline.
//!
//! Seeds a fresh in-memory History with 50 deterministic jobs
//! distributed across 30 days (mix of kinds + statuses + sizes),
//! then compares the results of `totals()` and `daily_totals()`
//! against a reference we compute in Rust from the same seed.
//! Bucket boundaries match what the SQL-side day-floor produces
//! (integer division by `86_400_000`).
//!
//! Note: the duration-ms aggregate is a *bound* check (`> 0`) rather
//! than an equality — `record_finish` stamps `finished_at_ms =
//! now()` which varies per test run. Phase 10 doesn't expose a
//! setter for `finished_at_ms`; Phase 14's "resume interrupted"
//! work will introduce one that this test can tighten up to.

use std::collections::BTreeMap;
use std::path::PathBuf;

use copythat_history::{History, JobSummary};

const DAY_MS: i64 = 86_400_000;

fn seed_job(i: usize) -> (JobSummary, &'static str, u64, u64) {
    // Deterministic distribution:
    //   - kind cycles copy/move every 5 jobs
    //   - every 7th job is failed; the rest succeed
    //   - started_at_ms spreads across 30 days
    //   - total_bytes = (i+1) * 1_000
    //   - files_ok  = (i % 4) + 1 on succeeded rows, 0 on failed
    let day = (i % 30) as i64;
    let started = day * DAY_MS + (i as i64) * 1_000;
    let kind = if (i / 5) % 2 == 0 { "copy" } else { "move" };
    let is_fail = i % 7 == 6;
    let status: &'static str = if is_fail { "failed" } else { "succeeded" };
    let bytes: u64 = ((i + 1) * 1_000) as u64;
    let files: u64 = if is_fail { 0 } else { ((i % 4) + 1) as u64 };

    let summary = JobSummary {
        row_id: 0,
        kind: kind.into(),
        status: "running".into(),
        started_at_ms: started,
        finished_at_ms: None,
        src_root: PathBuf::from(format!("/src/{i}")),
        dst_root: PathBuf::from(format!("/dst/{i}")),
        total_bytes: 0,
        files_ok: 0,
        files_failed: 0,
        verify_algo: None,
        options_json: None,
    };
    (summary, status, bytes, files)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn totals_and_daily_match_a_rust_computed_reference() {
    const N: usize = 50;
    let h = History::open_in_memory().await.expect("open");

    // Rust-side reference tallies — recomputed from the same seed.
    let mut ref_bytes = 0u64;
    let mut ref_files = 0u64;
    let mut ref_errors = 0u64;
    let mut ref_by_kind: BTreeMap<String, (u64, u64, u64)> = BTreeMap::new();
    let mut ref_daily: BTreeMap<i64, (u64, u64, u64)> = BTreeMap::new();

    for i in 0..N {
        let (summary, status, bytes, files) = seed_job(i);
        let id = h.record_start(&summary).await.expect("record_start");
        h.record_finish(id, status, bytes, files, u64::from(status == "failed"))
            .await
            .expect("record_finish");

        ref_bytes += bytes;
        ref_files += files;
        if status == "failed" {
            ref_errors += 1;
        }

        let k = ref_by_kind.entry(summary.kind.clone()).or_default();
        k.0 += bytes;
        k.1 += files;
        k.2 += 1;

        let day_ms = (summary.started_at_ms / DAY_MS) * DAY_MS;
        let d = ref_daily.entry(day_ms).or_default();
        d.0 += bytes;
        d.1 += files;
        d.2 += 1;
    }

    // ---- totals() -----------------------------------------------

    let totals = h.totals(None).await.expect("totals");
    assert_eq!(totals.jobs as usize, N);
    assert_eq!(totals.bytes, ref_bytes);
    assert_eq!(totals.files, ref_files);
    assert_eq!(totals.errors, ref_errors);
    // Duration is dynamic (record_finish stamps now_ms); just check
    // the aggregate summed something non-zero.
    assert!(totals.duration_ms > 0);

    for (kind, (bytes, files, jobs)) in &ref_by_kind {
        let got = totals
            .by_kind
            .get(kind.as_str())
            .unwrap_or_else(|| panic!("missing kind {kind}"));
        assert_eq!(got.bytes, *bytes, "kind {kind} bytes");
        assert_eq!(got.files, *files, "kind {kind} files");
        assert_eq!(got.jobs, *jobs, "kind {kind} jobs");
    }

    // ---- totals(since_ms) excludes older rows -----------------

    let midpoint = 15 * DAY_MS;
    let recent = h.totals(Some(midpoint)).await.expect("totals since");
    let mut ref_recent_bytes = 0u64;
    let mut ref_recent_jobs = 0u64;
    for i in 0..N {
        let (summary, _status, bytes, _files) = seed_job(i);
        if summary.started_at_ms >= midpoint {
            ref_recent_bytes += bytes;
            ref_recent_jobs += 1;
        }
    }
    assert_eq!(recent.bytes, ref_recent_bytes);
    assert_eq!(recent.jobs, ref_recent_jobs);

    // ---- daily_totals(0) matches the day-floored reference ---

    let daily = h.daily_totals(0).await.expect("daily");
    assert_eq!(daily.len(), ref_daily.len());
    for bucket in &daily {
        let want = ref_daily
            .get(&bucket.date_ms)
            .unwrap_or_else(|| panic!("missing day {}", bucket.date_ms));
        assert_eq!(bucket.bytes, want.0, "day {} bytes", bucket.date_ms);
        assert_eq!(bucket.files, want.1, "day {} files", bucket.date_ms);
        assert_eq!(bucket.jobs, want.2, "day {} jobs", bucket.date_ms);
    }
    // Oldest-first ordering.
    let dates: Vec<i64> = daily.iter().map(|b| b.date_ms).collect();
    let mut expected = dates.clone();
    expected.sort();
    assert_eq!(dates, expected);

    // ---- clear_all wipes everything ---------------------------

    let n = h.clear_all().await.expect("clear_all");
    assert_eq!(n, N as u64);
    let post = h.totals(None).await.unwrap();
    assert_eq!(post.jobs, 0);
    assert_eq!(post.bytes, 0);
    assert!(post.by_kind.is_empty());
    assert_eq!(h.daily_totals(0).await.unwrap().len(), 0);
}
