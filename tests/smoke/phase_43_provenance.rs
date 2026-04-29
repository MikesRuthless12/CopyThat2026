//! Phase 43 smoke test — forensic chain-of-custody manifests.
//!
//! What this proves:
//!
//! 1. With `CopyOptions::provenance` set to a wired
//!    [`CopyThatProvenanceSink`], `copy_tree` runs unchanged: every
//!    file in the source tree lands at the destination byte-for-byte.
//! 2. The sink accumulates a [`FileRecord`] per file. After
//!    `copy_tree` returns, `finalize_to_path` writes a canonical-CBOR
//!    manifest with a valid ed25519 signature over the
//!    signing-bytes view.
//! 3. `verify_manifest` reports all files clean against the freshly-
//!    copied destination tree.
//! 4. Tampering with one byte in one destination file flips that
//!    file's outcome to `Tampered { .. }` while every other file
//!    remains `Ok`.
//!
//! TSA timestamping is intentionally NOT exercised: the spec gates
//! it behind a feature flag (`tsa`) because `freetsa.org/tsr` is
//! external and flaky in CI. The TSA path is unit-tested in
//! `crates/copythat-provenance/src/sink.rs`.
//!
//! Runtime: <1 s. The test copies five small files (a few KiB each).

use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use copythat_core::{CopyControl, CopyEvent, ProvenancePolicy, TreeOptions, copy_tree};
use copythat_provenance::{
    CopyThatProvenanceSink, DEFAULT_MANIFEST_FILENAME, SinkConfig, VerificationOutcome,
    generate_signing_key, verify_manifest,
};
use tempfile::TempDir;
use tokio::sync::mpsc;

#[tokio::test(flavor = "current_thread")]
async fn phase_43_end_to_end() {
    run().await;
}

async fn run() {
    let tmp = TempDir::new().expect("tempdir");
    let src_root = tmp.path().join("src");
    let dst_root = tmp.path().join("dst");

    // 1. Lay down five files. Mix nested + top-level so rel_path
    //    derivation is exercised. Mix sizes so per-file Bao
    //    outboards span the single-chunk and multi-chunk cases (>=
    //    1 KiB triggers tree construction).
    let files: &[(&str, Vec<u8>)] = &[
        ("alpha.txt", b"alpha bytes\n".to_vec()),
        ("beta.txt", vec![0xBE; 2048]), // multi-chunk for Bao
        ("nested/gamma.txt", b"gamma!\n".to_vec()),
        ("nested/deep/delta.bin", vec![0xDE; 4096]), // larger multi-chunk
        ("epsilon.cfg", b"key=value\n".to_vec()),
    ];
    for (rel, bytes) in files {
        let path = src_root.join(rel);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("mkdir src");
        }
        std::fs::write(&path, bytes).expect("write src");
    }
    std::fs::create_dir_all(&dst_root).expect("mkdir dst");

    // 2. Build the sink with a fresh signing key so the manifest is
    //    signed (the spec smoke explicitly exercises the signing
    //    path; TSA stays off).
    let mut sink_config = SinkConfig::new(src_root.clone(), dst_root.clone());
    sink_config.signing_key = Some(generate_signing_key());
    let sink = CopyThatProvenanceSink::new(sink_config);

    // 3. Drive copy_tree with the provenance policy attached. The
    //    engine wires `make_encoder` / `record_file` per file from
    //    the source-read pass.
    let mut tree_opts = TreeOptions::default();
    tree_opts.file.provenance = Some(ProvenancePolicy {
        sink: sink.clone() as Arc<dyn copythat_core::ProvenanceSink>,
    });

    let (tx, mut rx) = mpsc::channel::<CopyEvent>(256);
    let drain = tokio::spawn(async move {
        while let Some(_evt) = rx.recv().await {
            // Drain the channel without inspecting; the smoke test
            // asserts on filesystem + manifest state rather than
            // event ordering.
        }
    });

    let report = copy_tree(&src_root, &dst_root, tree_opts, CopyControl::new(), tx)
        .await
        .expect("copy_tree should succeed with provenance enabled");

    let _ = drain.await.ok();
    assert_eq!(
        report.files,
        files.len() as u64,
        "expected {} files copied, got {}",
        files.len(),
        report.files
    );

    // 4. Sanity: every destination file matches its source.
    for (rel, bytes) in files {
        let dst_bytes = std::fs::read(dst_root.join(rel)).expect("read dst");
        assert_eq!(&dst_bytes, bytes, "destination diverges for {rel}");
    }

    // 5. Sink should have buffered one record per file.
    assert_eq!(
        sink.file_count(),
        files.len(),
        "sink should hold one FileRecord per file"
    );

    // 6. Finalise the manifest, verify it.
    let manifest_path = dst_root.join(DEFAULT_MANIFEST_FILENAME);
    let manifest = sink
        .finalize_to_path(&manifest_path)
        .expect("finalize_to_path");
    assert!(manifest.signature.is_some(), "manifest should be signed");
    assert_eq!(manifest.files.len(), files.len());
    assert!(manifest_path.exists(), "manifest should land on disk");

    let report = verify_manifest(&manifest_path, None).expect("verify_manifest");
    assert!(
        report.all_clean(),
        "fresh manifest should verify clean: {report:?}"
    );
    assert_eq!(report.ok_count, files.len());
    assert_eq!(report.tampered_count, 0);
    assert_eq!(report.signature_ok, Some(true));

    // 7. Tamper one byte in the second file. The tampered file
    //    should flip to `Tampered { .. }`; every other file should
    //    remain `Ok`. The Merkle root of the manifest is unchanged
    //    (the manifest itself wasn't tampered) — only the per-file
    //    re-hash diverges.
    let target = dst_root.join(files[1].0);
    let mut bytes = std::fs::read(&target).expect("read target");
    bytes[0] ^= 0xFF;
    std::fs::write(&target, bytes).expect("rewrite tampered");

    let report2 = verify_manifest(&manifest_path, None).expect("verify_manifest after tamper");
    assert!(
        !report2.all_clean(),
        "tampered tree should fail verification"
    );
    assert_eq!(report2.tampered_count, 1, "exactly one file should flip");
    assert_eq!(
        report2.ok_count,
        files.len() - 1,
        "every other file should remain Ok"
    );

    // The tampered path should be `files[1].0` (beta.txt by default).
    let (path, outcome) = report2
        .per_file
        .iter()
        .find(|(_, o)| matches!(o, VerificationOutcome::Tampered { .. }))
        .expect("expected exactly one tampered outcome");
    assert!(
        path.ends_with(
            files[1]
                .0
                .replace('/', std::path::MAIN_SEPARATOR_STR)
                .as_str()
        ) || path.ends_with(Path::new(files[1].0)),
        "tampered path mismatch: got {path:?}"
    );
    let _ = outcome;

    // 8. Manifest signature is still valid (we didn't touch the
    //    manifest itself — only a file). The signature_ok stays
    //    Some(true); only per-file outcomes diverge.
    assert_eq!(
        report2.signature_ok,
        Some(true),
        "tampering a file should NOT invalidate the manifest signature"
    );

    // 9. Robustness: the manifest file size is bounded. With Bao
    //    outboards on, a few-KiB tree should produce a manifest in
    //    the low-KiB range. Failing this guards against accidental
    //    runaway buffer growth.
    let manifest_size = std::fs::metadata(&manifest_path)
        .expect("manifest stat")
        .len();
    assert!(
        manifest_size < 64 * 1024,
        "manifest grew unexpectedly large: {manifest_size} bytes"
    );

    // 10. Sanity: the test as a whole should run in well under
    //     a second on any reasonable host. We don't wall-clock-fail
    //     here; the assertion is implicit in `tokio::main` not
    //     hanging. Sleep briefly to give any straggler tasks time
    //     to drop their handles cleanly.
    tokio::time::sleep(Duration::from_millis(10)).await;
}
