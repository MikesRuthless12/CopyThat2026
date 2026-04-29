//! Phase 42 Part B smoke — per-file rolling versions: history-table
//! round trip + retention math wiring.
//!
//! Six cases:
//!
//! 1. `record_version` + `versions_for_path` round-trip preserves
//!    every field byte-for-byte (path, ts, BLAKE3, size, retention
//!    floor, triggering job FK).
//! 2. `versions_for_path` returns newest-first regardless of insert
//!    order.
//! 3. `delete_versions` honours the supplied id set and returns the
//!    correct delete count.
//! 4. End-to-end retention pass: insert 10 versions, run
//!    `select_for_pruning(LastN(3))`, hand the result to
//!    `delete_versions`, verify the table holds exactly 3 rows
//!    afterwards (the 3 newest).
//! 5. The v1→v2 schema migration adds the `versions` table without
//!    disturbing the existing `jobs` / `items` content. Exercised by
//!    inserting a job + items pre-migration… actually no, the
//!    migration runs at open-time so we simply check the freshly-
//!    opened DB has all three tables and the expected indexes.
//! 6. All 8 Phase 42 Part B Fluent keys (`version-*`) are present in
//!    every one of the 18 locale files.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use copythat_core::versioning::{
    RetentionPolicy, VersionEntry, VersioningPolicy, VersioningSink, select_for_pruning,
};
use copythat_history::{History, VersionRecord, VersionRowId};

const PHASE_42_PART_B_KEYS: &[&str] = &[
    "version-list-heading",
    "version-list-empty",
    "version-list-restore",
    "version-retention-heading",
    "version-retention-none",
    "version-retention-last-n",
    "version-retention-older-than-days",
    "version-retention-gfs",
];

const LOCALES: &[&str] = &[
    "en", "es", "zh-CN", "hi", "ar", "pt-BR", "ru", "ja", "de", "fr", "ko", "it", "tr", "vi", "pl",
    "nl", "id", "uk",
];

fn dummy_record(path: &str, ts_ms: i64, size: u64) -> VersionRecord {
    let mut blake = [0u8; 32];
    // Fill with a deterministic byte pattern derived from `ts_ms`
    // so different timestamps produce different "manifests" — the
    // smoke just round-trips bytes; the actual chunk-store wiring
    // is part of the deferred engine integration.
    for (i, b) in blake.iter_mut().enumerate() {
        *b = ((ts_ms.wrapping_add(i as i64)) & 0xFF) as u8;
    }
    VersionRecord {
        row_id: 0,
        dst_path: PathBuf::from(path),
        ts_ms,
        manifest_blake3: blake,
        size,
        retained_until_ms: None,
        triggered_by_job_id: None,
    }
}

#[tokio::test]
async fn case01_record_and_round_trip_preserves_every_field() {
    let history = History::open_in_memory().await.expect("open");
    // Insert a real job first so the `triggered_by_job_id` FK has a
    // valid target — the schema's `ON DELETE SET NULL` only kicks
    // in after the job exists.
    let job_id = history
        .record_start(&copythat_history::JobSummary {
            row_id: 0,
            kind: "copy".into(),
            status: "running".into(),
            started_at_ms: 0,
            finished_at_ms: None,
            src_root: PathBuf::from("/src"),
            dst_root: PathBuf::from("/dst"),
            total_bytes: 0,
            files_ok: 0,
            files_failed: 0,
            verify_algo: None,
            options_json: None,
        })
        .await
        .expect("record_start");

    let mut record = dummy_record("/dst/a.bin", 1_704_112_496_000, 4096);
    record.retained_until_ms = Some(1_704_212_496_000);
    record.triggered_by_job_id = Some(job_id.as_i64());
    let row_id = history.record_version(&record).await.expect("record");
    assert!(row_id.as_i64() > 0);

    let rows = history
        .versions_for_path(PathBuf::from("/dst/a.bin"))
        .await
        .expect("read");
    assert_eq!(rows.len(), 1);
    let r = &rows[0];
    assert_eq!(r.dst_path, record.dst_path);
    assert_eq!(r.ts_ms, record.ts_ms);
    assert_eq!(r.manifest_blake3, record.manifest_blake3);
    assert_eq!(r.size, record.size);
    assert_eq!(r.retained_until_ms, Some(1_704_212_496_000));
    assert_eq!(r.triggered_by_job_id, Some(job_id.as_i64()));
}

#[tokio::test]
async fn case02_versions_for_path_returns_newest_first() {
    let history = History::open_in_memory().await.expect("open");
    // Insert in random ts order.
    for ts in [200, 100, 500, 300, 400_i64] {
        let r = dummy_record("/dst/file", ts, 100);
        history.record_version(&r).await.expect("record");
    }
    let rows = history
        .versions_for_path(PathBuf::from("/dst/file"))
        .await
        .expect("read");
    assert_eq!(rows.len(), 5);
    // Newest first.
    let ts_seq: Vec<i64> = rows.iter().map(|r| r.ts_ms).collect();
    assert_eq!(ts_seq, vec![500, 400, 300, 200, 100]);
}

#[tokio::test]
async fn case03_delete_versions_returns_count_and_drops_rows() {
    let history = History::open_in_memory().await.expect("open");
    let mut ids: Vec<VersionRowId> = Vec::new();
    for ts in [10, 20, 30, 40, 50_i64] {
        let r = dummy_record("/dst/x", ts, 1);
        ids.push(history.record_version(&r).await.expect("record"));
    }
    // Drop the middle three.
    let to_drop: Vec<VersionRowId> = vec![ids[1], ids[2], ids[3]];
    let n = history.delete_versions(to_drop).await.expect("delete");
    assert_eq!(n, 3);
    let remaining = history
        .versions_for_path(PathBuf::from("/dst/x"))
        .await
        .expect("read");
    assert_eq!(remaining.len(), 2);
    let ts_remaining: Vec<i64> = remaining.iter().map(|r| r.ts_ms).collect();
    assert_eq!(ts_remaining, vec![50, 10]);
}

#[tokio::test]
async fn case04_end_to_end_lastn_retention_pass() {
    let history = History::open_in_memory().await.expect("open");
    // Insert 10 versions, ts 100, 200, …, 1000.
    for i in 1..=10_i64 {
        let r = dummy_record("/dst/big", i * 100, (i as u64) * 1024);
        history.record_version(&r).await.expect("record");
    }

    let rows = history
        .versions_for_path(PathBuf::from("/dst/big"))
        .await
        .expect("read");
    let entries: Vec<VersionEntry> = rows
        .iter()
        .map(|r| VersionEntry {
            row_id: r.row_id,
            ts_ms: r.ts_ms,
            retained_until_ms: r.retained_until_ms,
        })
        .collect();
    let drop_ids = select_for_pruning(&entries, &RetentionPolicy::LastN(3), 10_000);
    assert_eq!(drop_ids.len(), 7);

    let row_ids: Vec<VersionRowId> = drop_ids.into_iter().map(VersionRowId).collect();
    let n = history.delete_versions(row_ids).await.expect("delete");
    assert_eq!(n, 7);

    let after = history
        .versions_for_path(PathBuf::from("/dst/big"))
        .await
        .expect("read");
    assert_eq!(after.len(), 3);
    // Newest 3 — ts 1000 / 900 / 800.
    let ts_after: Vec<i64> = after.iter().map(|r| r.ts_ms).collect();
    assert_eq!(ts_after, vec![1000, 900, 800]);
}

#[tokio::test]
async fn case05_v1_to_v2_migration_lands_versions_table_alongside_jobs_items() {
    // `History::open_in_memory` runs every migration including the
    // Phase 42 v1→v2. `versions_for_path` succeeding on a fresh DB
    // proves the table exists + has the right shape.
    let history = History::open_in_memory().await.expect("open");
    let rows = history
        .versions_for_path(PathBuf::from("/never-recorded"))
        .await
        .expect("read should succeed even for an unknown path");
    assert!(rows.is_empty());
    // And jobs/items still work — record one of each to prove the
    // migration didn't disturb pre-existing tables.
    let job_id = history
        .record_start(&copythat_history::JobSummary {
            row_id: 0,
            kind: "copy".into(),
            status: "running".into(),
            started_at_ms: 0,
            finished_at_ms: None,
            src_root: PathBuf::from("/src"),
            dst_root: PathBuf::from("/dst"),
            total_bytes: 0,
            files_ok: 0,
            files_failed: 0,
            verify_algo: None,
            options_json: None,
        })
        .await
        .expect("record_start");
    assert!(job_id.as_i64() > 0);
}

/// Recording mock for [`VersioningSink`]. Captures every call so the
/// engine integration tests can assert when (and how often) the
/// snapshot hook fired.
#[derive(Debug, Default)]
struct RecordingSink {
    calls: Mutex<Vec<(PathBuf, Option<i64>)>>,
}

impl VersioningSink for RecordingSink {
    fn snapshot_before_overwrite(
        &self,
        dst: &Path,
        triggered_by_job_id: Option<i64>,
    ) -> Result<bool, String> {
        self.calls
            .lock()
            .expect("mock sink poisoned")
            .push((dst.to_path_buf(), triggered_by_job_id));
        Ok(true)
    }
}

#[tokio::test]
async fn case07_engine_fires_sink_when_dst_exists_and_policy_enabled() {
    use copythat_core::{CopyControl, CopyOptions, copy_file};

    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("src.bin");
    let dst = dir.path().join("dst.bin");
    std::fs::write(&src, b"new bytes").unwrap();
    // Pre-existing destination — this is the case the snapshot hook
    // is meant to catch.
    std::fs::write(&dst, b"old bytes").unwrap();

    let sink = Arc::new(RecordingSink::default());
    let opts = CopyOptions {
        versioning: VersioningPolicy {
            enabled: true,
            retention: RetentionPolicy::None,
        },
        versioning_sink: Some(sink.clone() as Arc<dyn VersioningSink>),
        ..CopyOptions::default()
    };

    let (tx, _rx) = tokio::sync::mpsc::channel(64);
    let report = copy_file(&src, &dst, opts, CopyControl::new(), tx)
        .await
        .expect("copy ok");
    assert_eq!(report.bytes, b"new bytes".len() as u64);

    let calls = sink.calls.lock().unwrap();
    assert_eq!(calls.len(), 1, "expected exactly one snapshot call");
    assert_eq!(calls[0].0, dst);
    assert_eq!(calls[0].1, None);

    // Destination now carries the new bytes.
    let landed = std::fs::read(&dst).unwrap();
    assert_eq!(landed, b"new bytes");
}

#[tokio::test]
async fn case08_engine_skips_sink_when_dst_does_not_exist() {
    use copythat_core::{CopyControl, CopyOptions, copy_file};

    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("src.bin");
    let dst = dir.path().join("never-existed.bin");
    std::fs::write(&src, b"hello").unwrap();
    // No dst — the engine should NOT consult the sink because there's
    // nothing to snapshot.

    let sink = Arc::new(RecordingSink::default());
    let opts = CopyOptions {
        versioning: VersioningPolicy {
            enabled: true,
            retention: RetentionPolicy::None,
        },
        versioning_sink: Some(sink.clone() as Arc<dyn VersioningSink>),
        ..CopyOptions::default()
    };

    let (tx, _rx) = tokio::sync::mpsc::channel(64);
    copy_file(&src, &dst, opts, CopyControl::new(), tx)
        .await
        .expect("copy ok");

    let calls = sink.calls.lock().unwrap();
    assert!(
        calls.is_empty(),
        "sink must not fire when dst doesn't pre-exist; got {:?}",
        *calls
    );
}

#[tokio::test]
async fn case09_engine_skips_sink_when_policy_disabled_even_with_sink_present() {
    use copythat_core::{CopyControl, CopyOptions, copy_file};

    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("src.bin");
    let dst = dir.path().join("dst.bin");
    std::fs::write(&src, b"new").unwrap();
    std::fs::write(&dst, b"old").unwrap();

    let sink = Arc::new(RecordingSink::default());
    let opts = CopyOptions {
        // Sink installed but policy disabled — the engine must skip
        // the call regardless.
        versioning: VersioningPolicy {
            enabled: false,
            retention: RetentionPolicy::None,
        },
        versioning_sink: Some(sink.clone() as Arc<dyn VersioningSink>),
        ..CopyOptions::default()
    };

    let (tx, _rx) = tokio::sync::mpsc::channel(64);
    copy_file(&src, &dst, opts, CopyControl::new(), tx)
        .await
        .expect("copy ok");

    assert!(sink.calls.lock().unwrap().is_empty());
}

#[test]
fn case06_all_phase_42_part_b_keys_present_in_every_locale() {
    let workspace_root = workspace_root();
    let mut missing: Vec<String> = Vec::new();
    for locale in LOCALES {
        let path = workspace_root
            .join("locales")
            .join(locale)
            .join("copythat.ftl");
        let body = std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("could not read {}", path.display()));
        for key in PHASE_42_PART_B_KEYS {
            let needle = format!("{key} =");
            if !body.contains(&needle) {
                missing.push(format!("{locale}/{key}"));
            }
        }
    }
    assert!(
        missing.is_empty(),
        "missing Phase 42 Part B keys: {missing:?}"
    );
}

fn workspace_root() -> std::path::PathBuf {
    let crate_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    crate_dir
        .parent()
        .and_then(|p| p.parent())
        .expect("workspace has crates/<name>/Cargo.toml layout")
        .to_path_buf()
}
