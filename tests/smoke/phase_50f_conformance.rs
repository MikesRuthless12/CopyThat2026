//! Phase 50f smoke — the Rust `CdrManifest` reader checks the SAME shared
//! conformance corpus that `cdr-py` (and any third-party impl) uses. Every
//! `valid-*` fixture must be accepted; every `invalid-*` fixture rejected.
//! Regenerate the corpus with `cargo run -p xtask -- gen-conformance`.

use std::path::PathBuf;

use copythat_chunk::cdr::CdrManifest;

fn corpus_dir() -> PathBuf {
    // CARGO_MANIFEST_DIR is the copythat-chunk crate dir; the corpus lives at
    // the workspace root under docs/spec/conformance.
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../docs/spec/conformance")
}

#[test]
fn rust_reader_agrees_with_the_conformance_corpus() {
    let dir = corpus_dir();
    assert!(
        dir.join("corpus.json").is_file(),
        "corpus.json present (run `xtask gen-conformance`)"
    );
    let schema = std::fs::read_to_string(dir.join("manifest.schema.json"))
        .expect("manifest.schema.json present");
    assert!(
        schema.trim_start().starts_with('{'),
        "manifest.schema.json is JSON"
    );

    let mut valid = 0;
    let mut invalid = 0;
    for ent in std::fs::read_dir(&dir).unwrap() {
        let path = ent.unwrap().path();
        if path.extension().and_then(|e| e.to_str()) != Some("cbor") {
            continue;
        }
        let name = path.file_stem().unwrap().to_str().unwrap().to_string();
        let bytes = std::fs::read(&path).unwrap();
        let result = CdrManifest::from_cbor(&bytes);
        if name.starts_with("valid") {
            assert!(result.is_ok(), "{name} must be ACCEPTED, got {result:?}");
            valid += 1;
        } else {
            assert!(result.is_err(), "{name} must be REJECTED, but it parsed");
            invalid += 1;
        }
    }
    assert!(valid >= 2, "corpus has valid fixtures (found {valid})");
    assert!(
        invalid >= 5,
        "corpus has one invalid per rule (found {invalid})"
    );
}
