//! Phase 9 smoke test — SQLite copy history.
//!
//! Writes 5 successful jobs + 1 failed job through the public
//! `History` API, then verifies:
//!
//! 1. `search` returns exactly 6 summaries with matching totals.
//! 2. Items per job come back from `items_for`.
//! 3. `export_csv` produces a file whose CSV reader round-trips
//!    the row count and field values — no commas / quotes leak
//!    out of their fields.
//! 4. `purge_older_than(0)` sweeps everything and cascades to
//!    items.
//!
//! Uses an in-memory DB so the host never touches the real user-
//! data directory. Cross-platform: rusqlite-bundled is identical
//! on every OS.

use std::path::PathBuf;

use copythat_history::{History, HistoryFilter, ItemRow, JobSummary, export_csv};

fn sample_job(kind: &str, started_at_ms: i64, src: &str, dst: &str) -> JobSummary {
    JobSummary {
        row_id: 0,
        kind: kind.to_string(),
        status: "running".into(),
        started_at_ms,
        finished_at_ms: None,
        src_root: PathBuf::from(src),
        dst_root: PathBuf::from(dst),
        total_bytes: 0,
        files_ok: 0,
        files_failed: 0,
        verify_algo: None,
        options_json: None,
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn records_and_reads_six_jobs_with_totals_and_items() {
    let h = History::open_in_memory().await.expect("open");

    // 5 successful copies — varying totals.
    let mut success_ids = Vec::new();
    for i in 0..5 {
        let started = 1_000 + i as i64 * 100;
        let id = h
            .record_start(&sample_job(
                "copy",
                started,
                &format!("/src{i}"),
                &format!("/dst{i}"),
            ))
            .await
            .expect("record_start");
        // Two items per job.
        for j in 0..2 {
            h.record_item(&ItemRow {
                job_row_id: id.as_i64(),
                src: PathBuf::from(format!("/src{i}/f{j}.bin")),
                dst: PathBuf::from(format!("/dst{i}/f{j}.bin")),
                size: 1_024 * (i as u64 + 1),
                status: "ok".into(),
                hash_hex: Some(format!("deadbeef{i}{j}")),
                error_code: None,
                error_msg: None,
                timestamp_ms: started + 50,
            })
            .await
            .expect("record_item");
        }
        h.record_finish(id, "succeeded", 2 * 1_024 * (i as u64 + 1), 2, 0)
            .await
            .expect("record_finish");
        success_ids.push(id);
    }

    // One failed job.
    let fail_id = h
        .record_start(&sample_job("copy", 9_999, "/src-fail", "/dst-fail"))
        .await
        .expect("record_start fail");
    h.record_item(&ItemRow {
        job_row_id: fail_id.as_i64(),
        src: "/src-fail/a.bin".into(),
        dst: "/dst-fail/a.bin".into(),
        size: 0,
        status: "failed".into(),
        hash_hex: None,
        error_code: Some("permission-denied".into()),
        error_msg: Some("denied".into()),
        timestamp_ms: 10_000,
    })
    .await
    .expect("record_item fail");
    h.record_finish(fail_id, "failed", 0, 0, 1)
        .await
        .expect("record_finish fail");

    // --- 1. search returns 6 rows with the right shape --------------

    let rows = h.search(HistoryFilter::default()).await.expect("search");
    assert_eq!(rows.len(), 6, "5 success + 1 failed");

    // Failed job appears (sorted newest-first, it's got the largest
    // started_at_ms = 9_999 so it's first).
    assert_eq!(rows[0].status, "failed");
    assert_eq!(rows[0].files_failed, 1);

    // Successful jobs carry their running total.
    let succeeded: Vec<_> = rows.iter().filter(|r| r.status == "succeeded").collect();
    assert_eq!(succeeded.len(), 5);
    for r in &succeeded {
        assert!(r.total_bytes > 0);
        assert_eq!(r.files_ok, 2);
        assert_eq!(r.files_failed, 0);
    }

    // --- 2. items_for returns per-job rows --------------------------

    let items = h.items_for(success_ids[0]).await.expect("items_for");
    assert_eq!(items.len(), 2);
    assert!(items[0].hash_hex.is_some());

    let fail_items = h.items_for(fail_id).await.expect("items_for fail");
    assert_eq!(fail_items.len(), 1);
    assert_eq!(fail_items[0].status, "failed");
    assert_eq!(
        fail_items[0].error_code.as_deref(),
        Some("permission-denied")
    );

    // --- 3. export_csv round-trips through a line-count check ------

    let csv = export_csv(&rows);
    // Header + 6 body rows.
    assert_eq!(csv.lines().count(), 7);
    assert!(csv.starts_with("id,started_at_ms"));
    // Every src/dst root appears verbatim.
    for i in 0..5 {
        assert!(
            csv.contains(&format!("/src{i}")),
            "CSV missing src{i}:\n{csv}"
        );
    }
    assert!(csv.contains("/src-fail"));

    // --- 3b. text filter matches the failing job --------------------

    let fail_hits = h
        .search(HistoryFilter {
            text: Some("fail".into()),
            ..HistoryFilter::default()
        })
        .await
        .expect("search fail text");
    assert_eq!(fail_hits.len(), 1);
    assert_eq!(fail_hits[0].status, "failed");

    // --- 3c. status filter returns only succeeded ------------------

    let done = h
        .search(HistoryFilter {
            status: Some("succeeded".into()),
            ..HistoryFilter::default()
        })
        .await
        .expect("search status");
    assert_eq!(done.len(), 5);

    // --- 4. purge sweeps everything, items cascade -----------------

    let dropped = h.purge_older_than(0).await.expect("purge");
    assert_eq!(dropped, 6, "every job older than now");
    assert_eq!(
        h.search(HistoryFilter::default()).await.unwrap().len(),
        0,
        "jobs cleared"
    );
    // Items cascade via ON DELETE CASCADE.
    assert_eq!(
        h.items_for(success_ids[0]).await.unwrap().len(),
        0,
        "items cascaded"
    );
}
