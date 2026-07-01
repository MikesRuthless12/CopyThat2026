//! Phase 50f — generate the CDR-0 conformance corpus under
//! `docs/spec/conformance/`: valid + one-invalid-per-§5-rule manifest
//! fixtures (CBOR), a JSON index of expected accept/reject verdicts, and the
//! manifest JSON-Schema. Independent CDR-0 implementations (the Rust
//! `CdrManifest` and the Python `cdr-py`) test against this ONE source of
//! truth. Regenerate with `cargo run -p xtask -- gen-conformance`.

use std::path::Path;

use copythat_chunk::ChunkCodec;
use copythat_chunk::cdr::{CDR_ALGO, CDR_SPEC_VERSION, CdrChunkRef, CdrManifest};

const OUT_DIR: &str = "docs/spec/conformance";

/// A 32-byte digest fixture filled with `b`.
fn hash32(b: u8) -> Vec<u8> {
    vec![b; 32]
}

fn chunk(hash: u8, offset: u64, len: u32) -> CdrChunkRef {
    CdrChunkRef {
        hash: hash32(hash),
        offset,
        len,
        codec: ChunkCodec::None,
    }
}

fn base() -> CdrManifest {
    CdrManifest {
        spec_version: CDR_SPEC_VERSION,
        algo: CDR_ALGO.to_string(),
        file_hash: hash32(0xF1),
        size: 300,
        chunks: vec![chunk(0xA1, 0, 100), chunk(0xA2, 100, 200)],
    }
}

/// (name, manifest, accept?, reason).
fn fixtures() -> Vec<(&'static str, CdrManifest, bool, &'static str)> {
    // valid — single chunk.
    let mut single = base();
    single.size = 100;
    single.chunks = vec![chunk(0xB1, 0, 100)];

    // invalid — bad algorithm tag.
    let mut bad_algo = base();
    bad_algo.algo = "fastcdc-1999;bogus".to_string();

    // invalid — file_hash not 32 bytes.
    let mut bad_hash_len = base();
    bad_hash_len.file_hash = vec![0x11; 16];

    // invalid — chunks don't tile contiguously (gap between chunk 0 and 1).
    let mut noncontiguous = base();
    noncontiguous.chunks = vec![chunk(0xA1, 0, 100), chunk(0xA2, 150, 200)];

    // invalid — declared size != sum(chunk.len).
    let mut size_mismatch = base();
    size_mismatch.size = 999;

    // invalid — spec_version newer than this reader implements.
    let mut too_new = base();
    too_new.spec_version = CDR_SPEC_VERSION + 1;

    vec![
        (
            "valid-multi",
            base(),
            true,
            "well-formed two-chunk manifest",
        ),
        (
            "valid-single",
            single,
            true,
            "well-formed single-chunk manifest",
        ),
        (
            "invalid-bad-algo",
            bad_algo,
            false,
            "algorithm tag does not match the canonical CDR_ALGO",
        ),
        (
            "invalid-hash-len",
            bad_hash_len,
            false,
            "file_hash is not 32 bytes",
        ),
        (
            "invalid-noncontiguous",
            noncontiguous,
            false,
            "chunks do not tile the file contiguously from offset 0",
        ),
        (
            "invalid-size-mismatch",
            size_mismatch,
            false,
            "declared size != sum of chunk lengths",
        ),
        (
            "invalid-too-new",
            too_new,
            false,
            "spec_version is newer than this reader implements",
        ),
    ]
}

/// The §5 manifest JSON-Schema (the spec calls this "planned"). Kept in sync
/// with `CdrManifest` by hand — a schema, not generated, so it is readable.
const MANIFEST_SCHEMA: &str = r#"{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://copythat.dev/spec/cdr-0/manifest.schema.json",
  "title": "CDR-0 file manifest",
  "type": "object",
  "additionalProperties": false,
  "required": ["spec_version", "algo", "file_hash", "size", "chunks"],
  "properties": {
    "spec_version": { "type": "integer", "minimum": 0, "description": "CDR spec version; a reader rejects a value it does not implement (§8)." },
    "algo": { "type": "string", "description": "Chunk-algorithm tag; must equal the canonical CDR_ALGO for a v0 reader." },
    "file_hash": { "type": "string", "description": "BLAKE3-256 of the whole file (32 bytes; hex/bstr on the wire)." },
    "size": { "type": "integer", "minimum": 0, "description": "Total file size; equals sum(chunks[].len)." },
    "chunks": {
      "type": "array",
      "items": {
        "type": "object",
        "additionalProperties": false,
        "required": ["hash", "offset", "len"],
        "properties": {
          "hash": { "type": "string", "description": "BLAKE3-256 of the chunk plaintext (32 bytes)." },
          "offset": { "type": "integer", "minimum": 0, "description": "Byte offset of the chunk in the reconstructed file." },
          "len": { "type": "integer", "minimum": 0, "description": "Chunk length (<= 4 MiB per §2)." },
          "codec": { "type": "string", "enum": ["none", "zstd"], "description": "OPTIONAL (Phase 49h); default none. Orthogonal to identity." }
        }
      },
      "description": "Ordered; chunks tile the file contiguously from offset 0 and their bytes concatenate to reconstruct it."
    }
  }
}
"#;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let dir = Path::new(OUT_DIR);
    std::fs::create_dir_all(dir)?;

    let mut index_entries = Vec::new();
    for (name, manifest, accept, reason) in fixtures() {
        // to_cbor serialises the manifest regardless of validity, so the
        // invalid fixtures are real CBOR a reader must actively reject.
        let cbor = manifest.to_cbor()?;
        let file = format!("{name}.cbor");
        std::fs::write(dir.join(&file), &cbor)?;
        index_entries.push(format!(
            "    {{ \"fixture\": \"{file}\", \"expect\": \"{}\", \"reason\": \"{}\" }}",
            if accept { "accept" } else { "reject" },
            reason
        ));
    }

    let corpus = format!(
        "{{\n  \"about\": \"CDR-0 conformance corpus — generated by `cargo run -p xtask -- gen-conformance`. Each fixture is a CBOR-encoded manifest a conforming reader must accept or reject per docs/spec/CDR-0.md §5.\",\n  \"cases\": [\n{}\n  ]\n}}\n",
        index_entries.join(",\n")
    );
    std::fs::write(dir.join("corpus.json"), corpus)?;
    std::fs::write(dir.join("manifest.schema.json"), MANIFEST_SCHEMA)?;

    println!(
        "xtask gen-conformance: wrote {} fixtures + corpus.json + manifest.schema.json to {}",
        fixtures().len(),
        dir.display()
    );
    Ok(())
}
