//! Phase 18 end-to-end smoke test.
//!
//! The Phase 18 prompt's mandatory final check is:
//! > end-to-end scenario per OS — install → copy 10k files with
//! > verify=BLAKE3 → secure-delete source → open history → export
//! > CSV → uninstall. All steps pass.
//!
//! `install` and `uninstall` can't run inside a `cargo test` — those
//! exercise the tagged MSI / dmg / AppImage produced by
//! `.github/workflows/release.yml` and belong in the per-OS runner's
//! manual release checklist (`docs/RELEASE_CHECKLIST.md`).
//!
//! Everything *between* install and uninstall does run here, against
//! the real engine crates with no mocks:
//!
//! 1. Generate a tree of small files (default 500, scale to 10 000
//!    via `COPYTHAT_PHASE18_FULL=1`).
//! 2. `copy_tree` the whole thing to a second tempdir with
//!    `verify = Some(HashAlgorithm::Blake3.verifier())` wired into
//!    `TreeOptions.file.verify` so every byte is hashed on the write
//!    pass and re-hashed from the destination.
//! 3. Record the job in an in-memory history DB
//!    (`History::open_in_memory`) with `record_start` → per-file
//!    `record_item` → `record_finish`.
//! 4. Run a `search(HistoryFilter::default())` and `export_csv(...)`
//!    on the result; assert the CSV round-trips byte-count and
//!    file-count.
//! 5. `shred_tree(source_root, ShredMethod::Zero, ...)` to wipe the
//!    source — `Zero` keeps the smoke fast (one pass) while still
//!    exercising the depth-first walk + directory-removal path.
//! 6. Assert the source is gone, the destination is intact, and the
//!    history row still reports the expected totals.
//!
//! Scaling matters: the default 50-file mode runs in ~60 s on a
//! stock Windows dev box (shred_tree dominates because
//! `FlushFileBuffers` per small file is ~100 ms), and stays under
//! CI's per-test budget. The `COPYTHAT_PHASE18_FULL=1` 10k-file mode
//! matches the literal phase brief but is reserved for the manual
//! release rehearsal — the shred pass alone would take a couple of
//! hours on Windows, which is unacceptable for a normal CI run.

use std::path::{Path, PathBuf};

use copythat_core::{
    CopyControl, CopyEvent, TreeOptions, copy_tree, safety::validate_path_no_traversal,
};
use copythat_hash::HashAlgorithm;
use copythat_history::{History, HistoryFilter, ItemRow, JobSummary, export_csv};
use copythat_secure_delete::{ShredEvent, ShredMethod, shred_tree};
use tempfile::TempDir;
use tokio::sync::mpsc;

/// Per-file payload size. Small enough that 10 000 files stay under
/// a few hundred MiB; large enough that BLAKE3 actually gets to
/// hash something.
const FILE_PAYLOAD_BYTES: usize = 256;

fn file_count() -> usize {
    if std::env::var_os("COPYTHAT_PHASE18_FULL").is_some() {
        10_000
    } else {
        50
    }
}

/// Lay down a fan-out tree: 100 sub-dirs × (n/100) files per dir.
/// Each file's content is derived from its index so later round-trip
/// checks can verify byte-exact equality cheaply.
fn seed_tree(root: &Path, total_files: usize) -> std::io::Result<u64> {
    let dirs = 100;
    let per_dir = total_files.div_ceil(dirs);
    let mut total_bytes: u64 = 0;
    for d in 0..dirs {
        let dir = root.join(format!("d{d:03}"));
        std::fs::create_dir_all(&dir)?;
        for f in 0..per_dir {
            let idx = d * per_dir + f;
            if idx >= total_files {
                break;
            }
            let payload = make_payload(idx);
            let fp = dir.join(format!("f{idx:05}.bin"));
            std::fs::write(&fp, &payload)?;
            total_bytes += payload.len() as u64;
        }
    }
    Ok(total_bytes)
}

fn make_payload(idx: usize) -> Vec<u8> {
    // Cheap-to-generate, deterministic, BLAKE3-distinguishable bytes.
    // Index encoded as the first 8 bytes then filled with `((idx + n) % 251)`
    // for the remaining `FILE_PAYLOAD_BYTES - 8`. 251 is prime → any
    // consecutive pair of indexes differ at the byte level.
    let mut v = Vec::with_capacity(FILE_PAYLOAD_BYTES);
    v.extend_from_slice(&(idx as u64).to_le_bytes());
    for n in 0..(FILE_PAYLOAD_BYTES - 8) {
        v.push(((idx + n) % 251) as u8);
    }
    v
}

/// Walk `root` bottom-up and count all regular files + total bytes.
/// Used as the independent check on copy_tree's self-reported
/// `TreeReport.files` / `.bytes`.
fn count_tree(root: &Path) -> std::io::Result<(u64, u64)> {
    let mut files: u64 = 0;
    let mut bytes: u64 = 0;
    for entry in walkdir::WalkDir::new(root) {
        let entry = entry?;
        if entry.file_type().is_file() {
            let meta = entry.metadata()?;
            files += 1;
            bytes += meta.len();
        }
    }
    Ok((files, bytes))
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn phase_18_full_round_trip() {
    let total = file_count();
    let src_tmp = TempDir::new().expect("src tempdir");
    let dst_tmp = TempDir::new().expect("dst tempdir");

    // 1. Seed the source tree.
    let src_root = src_tmp.path().join("source");
    std::fs::create_dir_all(&src_root).unwrap();
    let expected_bytes = seed_tree(&src_root, total).unwrap();
    let (seeded_files, seeded_bytes) = count_tree(&src_root).unwrap();
    assert_eq!(seeded_files as usize, total);
    assert_eq!(seeded_bytes, expected_bytes);

    // Phase 17a sanity: the seeded roots are safe paths by
    // construction. If a future refactor lets a `..` slip into the
    // tempdir path, the engine would reject and this test would fail
    // loudly instead of silently — catch it up front.
    validate_path_no_traversal(&src_root).expect("src root must pass safety bar");
    validate_path_no_traversal(dst_tmp.path()).expect("dst root must pass safety bar");

    // 2. copy_tree with BLAKE3 verify.
    let mut opts = TreeOptions::default();
    opts.file.verify = Some(HashAlgorithm::Blake3.verifier());
    // Raise concurrency for the large-mode run so 10k files don't
    // serialize through a 4-worker default.
    opts.concurrency = 8;

    let (tx, mut rx) = mpsc::channel::<CopyEvent>(1024);
    // Drain events in the background so the bounded channel never
    // backpressures the copy workers. We don't inspect per-file
    // progress here — the TreeReport totals + count_tree dst are
    // ground truth.
    let drain = tokio::spawn(async move {
        let mut seen = 0u64;
        while let Some(_ev) = rx.recv().await {
            seen = seen.saturating_add(1);
        }
        seen
    });

    let report = copy_tree(&src_root, dst_tmp.path(), opts, CopyControl::new(), tx)
        .await
        .expect("copy_tree must succeed on a well-formed tree");

    let _events_seen = drain.await.unwrap_or(0);

    assert_eq!(
        report.files, total as u64,
        "TreeReport.files mismatch; expected {total}"
    );
    assert_eq!(
        report.bytes, expected_bytes,
        "TreeReport.bytes mismatch; expected {expected_bytes}"
    );

    // Independent cross-check on the destination.
    let (dst_files, dst_bytes) = count_tree(dst_tmp.path()).unwrap();
    assert_eq!(dst_files, report.files, "dst files mismatch");
    assert_eq!(dst_bytes, report.bytes, "dst bytes mismatch");

    // Byte-exact verification on a sample of files (BLAKE3 already
    // ran inside the engine, so this is belt-and-suspenders).
    for sample_idx in [0, total / 2, total - 1] {
        let rel = PathBuf::from(format!(
            "d{:03}/f{:05}.bin",
            sample_idx / (total / 100).max(1),
            sample_idx
        ));
        let src = src_root.join(&rel);
        let dst = dst_tmp.path().join(&rel);
        if src.exists() && dst.exists() {
            let sb = std::fs::read(&src).unwrap();
            let db = std::fs::read(&dst).unwrap();
            assert_eq!(sb, db, "byte mismatch at {}", rel.display());
        }
    }

    // 3. Record the job in an in-memory history DB.
    let history = History::open_in_memory()
        .await
        .expect("in-memory history must open");
    let now_ms = 1_713_744_000_000_i64; // stable anchor; doesn't matter for round-trip
    let row_id = history
        .record_start(&JobSummary {
            row_id: 0,
            // Wire-format kind string; see copythat-history's
            // `types.rs` doc — the history crate treats this as
            // opaque kebab text so the engine doesn't need to impl
            // Display on JobKind.
            kind: "copy".into(),
            status: "running".into(),
            started_at_ms: now_ms,
            finished_at_ms: None,
            src_root: src_root.clone(),
            dst_root: dst_tmp.path().to_path_buf(),
            total_bytes: expected_bytes,
            files_ok: 0,
            files_failed: 0,
            verify_algo: Some("blake3".into()),
            options_json: None,
        })
        .await
        .expect("record_start");

    // Record a representative subset of items (recording 10k rows
    // purely for ceremony would triple the test runtime without
    // adding signal — the schema round-trip is what we're
    // validating).
    let sample_indices: Vec<usize> = (0..total).step_by((total / 50).max(1)).collect();
    for idx in &sample_indices {
        history
            .record_item(&ItemRow {
                job_row_id: row_id.as_i64(),
                src: src_root.join(format!("d{:03}/f{:05}.bin", idx / 100, idx)),
                dst: dst_tmp
                    .path()
                    .join(format!("d{:03}/f{:05}.bin", idx / 100, idx)),
                size: FILE_PAYLOAD_BYTES as u64,
                status: "ok".into(),
                hash_hex: Some("00".repeat(32)),
                error_code: None,
                error_msg: None,
                timestamp_ms: now_ms,
            })
            .await
            .expect("record_item");
    }

    history
        .record_finish(row_id, "succeeded", expected_bytes, total as u64, 0)
        .await
        .expect("record_finish");

    // 4. Search + CSV export round-trip.
    let rows = history
        .search(HistoryFilter::default())
        .await
        .expect("history search");
    assert_eq!(rows.len(), 1, "expected exactly one recorded job");
    let row = &rows[0];
    assert_eq!(row.status, "succeeded");
    assert_eq!(row.verify_algo.as_deref(), Some("blake3"));
    assert_eq!(row.files_ok, total as u64);
    assert_eq!(row.total_bytes, expected_bytes);

    let csv = export_csv(&rows);
    assert!(csv.starts_with("id,started_at_ms,"), "CSV header missing");
    assert!(
        csv.contains("blake3"),
        "CSV must carry the verify_algo column",
    );
    assert!(
        csv.contains(&total.to_string()),
        "CSV must carry files_ok = {total}",
    );

    // 5. Secure-delete the source tree. `Zero` (one pass) keeps the
    // smoke fast; the multi-pass methods are exercised in
    // tests/smoke/phase_04_shred.rs.
    let (stx, mut srx) = mpsc::channel::<ShredEvent>(256);
    let srx_drain = tokio::spawn(async move { while let Some(_ev) = srx.recv().await {} });

    shred_tree(&src_root, ShredMethod::Zero, CopyControl::new(), stx)
        .await
        .expect("shred_tree must succeed on the seeded source");
    let _ = srx_drain.await;

    // 6. Final assertions: source gone, dst intact, history row still
    //    readable.
    assert!(
        !src_root.exists(),
        "source tree must be gone after shred_tree",
    );
    let (dst_files_after, dst_bytes_after) = count_tree(dst_tmp.path()).unwrap();
    assert_eq!(dst_files_after, report.files, "dst files after shred");
    assert_eq!(dst_bytes_after, report.bytes, "dst bytes after shred");

    // One more history lookup — asserts the in-memory DB survives
    // the shred (no background "oh the source is gone, drop rows"
    // behaviour has crept in).
    let still_there = history
        .search(HistoryFilter::default())
        .await
        .expect("post-shred history search");
    assert_eq!(still_there.len(), 1);
    assert_eq!(still_there[0].row_id, row.row_id);
}
