//! restic repository **exporter** (Phase 50e).
//!
//! The inverse of [`super::restic`]: it takes a CDR-0
//! [`Repository`](crate::repository::Repository) and writes a complete,
//! encrypted **restic** (format v2) repository under `dst` that a real
//! `restic` binary — and, more importantly, our own
//! [`import_restic`](super::restic) — can read back byte-for-byte.
//!
//! # What it writes
//!
//! Every primitive here is the exact inverse of the importer's decrypt
//! path, reusing the same crates (no new dependency):
//!
//! - **Seal** ([`restic_seal`]): `IV(16) || AES-256-CTR(payload) ||
//!   Poly1305-AES(ciphertext)(16)`, with the Poly1305 key
//!   `mac.r || AES-128(mac.k, IV)` — the byte-for-byte inverse of
//!   `restic::restic_decrypt`.
//! - **Master key** wrapped by a **scrypt** KEK into `keys/<id>`; a
//!   `config` sealed with the master key (uncompressed bootstrap data).
//! - **Pack files** `data/<aa>/<id>`: each re-chunked file blob is sealed
//!   with the master key, appended in order, then a sealed pack header +
//!   `u32` length trailer; `id = SHA-256(whole file)`.
//! - A single **index** `index/<id>` mapping every blob to its pack slice,
//!   and one **snapshot** `snapshots/<id>` per CDR snapshot, each sealed
//!   with the master key (uncompressed — `import_restic` branches on a
//!   leading `0x02` for zstd, and plain JSON starts with `{`).
//!
//! restic chunk IDs aren't portable, so each file is re-chunked (reusing
//! the CDR manifest's own chunk boundaries — each ≤ 4 MiB, well under
//! restic's 8 MiB blob cap) and re-sealed; a blob's id is the SHA-256 of
//! its **plaintext**, exactly what the importer re-verifies on read-back.

// Staged ahead of its caller: the `migrate::export` dispatcher that
// invokes `export_restic` is wired separately, so every item here is
// momentarily unreachable from a crate-public root. The allowance turns
// into a no-op the instant that dispatcher lands.
#![allow(dead_code)]

use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};

use super::fill_random;
use aes::Aes128;
use aes::cipher::generic_array::GenericArray;
use aes::cipher::{BlockEncrypt, KeyInit, KeyIvInit, StreamCipher};
use base64::Engine as _;
use ctr::Ctr128BE;
use serde::Serialize;
use sha2::{Digest, Sha256};

use super::{MigrateError, MigrateReport};
use crate::manifest::materialise_range;
use crate::repository::{Repository, Snapshot};
use crate::types::{Manifest, hex_of};

type Aes256Ctr = Ctr128BE<aes::Aes256>;

/// scrypt cost parameters for the exported keyfile (restic's own
/// defaults: N a power of two, r, p). The importer requires N be a power
/// of two.
const SCRYPT_N: u32 = 32768;
const SCRYPT_R: u32 = 8;
const SCRYPT_P: u32 = 1;

/// Roll a pack file over once its sealed blobs pass this size, so a large
/// migration doesn't buffer one enormous pack in memory.
const PACK_ROLLOVER: usize = 16 * 1024 * 1024;

/// A known-valid restic chunker polynomial (degree-53 irreducible over
/// GF(2), lowercase hex). `import_restic` never reads `config`, so the
/// value only matters for a real `restic` binary; a fixed valid poly is
/// correct and avoids emitting a non-irreducible one.
const CHUNKER_POLYNOMIAL: &str = "36a6fc4419f26b";

/// A restic key (master key or the transient scrypt-derived KEK share
/// this layout — identical to the importer's `ResticKey`).
struct ResticKey {
    encrypt: [u8; 32],
    mac_k: [u8; 16],
    mac_r: [u8; 16],
}

/// Which restic blob class a chunk belongs to (drives the pack header
/// type byte + index `type` field, and keeps data/tree blobs in separate
/// packs like restic does).
#[derive(Clone, Copy)]
enum BlobKind {
    Data,
    Tree,
}

impl BlobKind {
    /// Index-JSON `type` string (the importer ignores it; real restic reads it).
    const fn type_str(self) -> &'static str {
        match self {
            Self::Data => "data",
            Self::Tree => "tree",
        }
    }

    /// Pack-header entry type byte for an *uncompressed* blob.
    const fn header_byte(self) -> u8 {
        match self {
            Self::Data => 0,
            Self::Tree => 1,
        }
    }
}

fn io_err(path: &Path, source: std::io::Error) -> MigrateError {
    MigrateError::Io {
        path: path.to_path_buf(),
        source,
    }
}

fn enc_err(ctx: &str, e: impl std::fmt::Display) -> MigrateError {
    MigrateError::Format(format!("{ctx}: {e}"))
}

fn b64_encode(bytes: &[u8]) -> String {
    base64::engine::general_purpose::STANDARD.encode(bytes)
}

/// A fresh master key (`encrypt` 32 / `mac.k` 16 / `mac.r` 16).
fn random_master_key() -> ResticKey {
    let mut encrypt = [0u8; 32];
    let mut mac_k = [0u8; 16];
    let mut mac_r = [0u8; 16];
    fill_random(&mut encrypt);
    fill_random(&mut mac_k);
    fill_random(&mut mac_r);
    ResticKey {
        encrypt,
        mac_k,
        mac_r,
    }
}

/// Format `ms` since the Unix epoch as an RFC 3339 string (the form
/// `import_restic` parses back with `chrono`).
fn ms_to_rfc3339(ms: i64) -> String {
    chrono::DateTime::<chrono::Utc>::from_timestamp_millis(ms)
        .or_else(|| chrono::DateTime::<chrono::Utc>::from_timestamp_millis(0))
        .expect("epoch is representable")
        .to_rfc3339()
}

/// Seal one restic AEAD unit — the exact inverse of
/// `restic::restic_decrypt`. Output is `IV(16) || ciphertext ||
/// Poly1305-AES tag(16)`; the 16-byte random IV is used for BOTH the
/// AES-256-CTR initial counter AND the Poly1305 key prep.
fn restic_seal(key: &ResticKey, payload: &[u8]) -> Vec<u8> {
    let mut iv = [0u8; 16];
    fill_random(&mut iv);

    // AES-256-CTR is length-preserving + symmetric (same call the importer
    // uses to decrypt); IV is the full 128-bit big-endian counter.
    let mut ct = payload.to_vec();
    Aes256Ctr::new(
        GenericArray::from_slice(&key.encrypt),
        GenericArray::from_slice(&iv),
    )
    .apply_keystream(&mut ct);

    // Poly1305-AES key: s = AES-128(mac.k, IV); key = mac.r || s. Order
    // matters — mac.r first, AES(mac.k, IV) second — and the MAC covers
    // the CIPHERTEXT only, unpadded (classic Poly1305-AES).
    let aes = Aes128::new(GenericArray::from_slice(&key.mac_k));
    let mut s = *GenericArray::from_slice(&iv);
    aes.encrypt_block(&mut s);
    let mut poly_key = [0u8; 32];
    poly_key[..16].copy_from_slice(&key.mac_r);
    poly_key[16..].copy_from_slice(s.as_slice());
    let tag = poly1305::Poly1305::new(GenericArray::from_slice(&poly_key)).compute_unpadded(&ct);

    let mut out = Vec::with_capacity(16 + ct.len() + 16);
    out.extend_from_slice(&iv);
    out.extend_from_slice(&ct);
    out.extend_from_slice(tag.as_slice());
    out
}

/// Derive the scrypt KEK that wraps the master key — the inverse of
/// `restic::try_keyfile`. Splits the 64-byte output into
/// `encrypt || mac.k || mac.r`.
fn derive_kek(password: &str, salt: &[u8]) -> Result<ResticKey, MigrateError> {
    let params = scrypt::Params::new(SCRYPT_N.trailing_zeros() as u8, SCRYPT_R, SCRYPT_P, 64)
        .map_err(|e| enc_err("scrypt params", e))?;
    let mut kek = [0u8; 64];
    scrypt::scrypt(password.as_bytes(), salt, &params, &mut kek)
        .map_err(|e| enc_err("scrypt", e))?;
    Ok(ResticKey {
        encrypt: kek[0..32].try_into().expect("32"),
        mac_k: kek[32..48].try_into().expect("16"),
        mac_r: kek[48..64].try_into().expect("16"),
    })
}

// ----------------------------------------------------------------------
// on-disk JSON (serialize only — restic ignores field order + unknowns)
// ----------------------------------------------------------------------

#[derive(Serialize)]
struct KeyFileJson {
    created: String,
    username: String,
    hostname: String,
    kdf: String,
    #[serde(rename = "N")]
    n: u32,
    r: u32,
    p: u32,
    salt: String,
    data: String,
}

#[derive(Serialize)]
struct MasterKeyJson {
    mac: MacKeyJson,
    encrypt: String,
}

#[derive(Serialize)]
struct MacKeyJson {
    k: String,
    r: String,
}

#[derive(Serialize)]
struct ConfigJson {
    version: u32,
    id: String,
    chunker_polynomial: String,
}

#[derive(Serialize)]
struct IndexJson {
    packs: Vec<IndexPackJson>,
}

#[derive(Serialize)]
struct IndexPackJson {
    id: String,
    blobs: Vec<IndexBlobJson>,
}

#[derive(Serialize)]
struct IndexBlobJson {
    id: String,
    #[serde(rename = "type")]
    typ: &'static str,
    offset: u64,
    length: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    uncompressed_length: Option<u64>,
}

#[derive(Serialize)]
struct SnapshotJson {
    time: String,
    tree: String,
    paths: Vec<String>,
    hostname: String,
    username: String,
    program_version: String,
}

#[derive(Serialize)]
struct TreeJson {
    nodes: Vec<TreeNodeJson>,
}

#[derive(Serialize)]
struct TreeNodeJson {
    name: String,
    #[serde(rename = "type")]
    typ: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subtree: Option<String>,
}

// ----------------------------------------------------------------------
// directory trie → restic trees
// ----------------------------------------------------------------------

/// One entry in the reconstructed directory tree: a subdirectory or a
/// file (its ordered content blob ids).
enum Node {
    Dir(BTreeMap<String, Node>),
    File(Vec<String>),
}

/// Insert a file's normalized path `comps` (its content = `cids`) into the
/// directory trie `dir`.
/// Returns whether the file's content was actually placed at a retrievable
/// path in the tree. A degenerate path (no components after normalisation) or
/// a name/kind collision returns `false` so the caller can count it as
/// `skipped` rather than inflating the exported-file count while the restic
/// tree silently omits it.
fn insert_path(dir: &mut BTreeMap<String, Node>, comps: &[&str], cids: Vec<String>) -> bool {
    let Some((first, rest)) = comps.split_first() else {
        return false; // no name components — cannot place this file
    };
    if rest.is_empty() {
        // Don't clobber an existing directory node with a same-named file
        // (degenerate layout where `a` and `a/b` both exist); report the file
        // un-placed instead of destroying the directory subtree.
        if matches!(dir.get(*first), Some(Node::Dir(_))) {
            return false;
        }
        dir.insert((*first).to_string(), Node::File(cids));
        return true;
    }
    let child = dir
        .entry((*first).to_string())
        .or_insert_with(|| Node::Dir(BTreeMap::new()));
    match child {
        Node::Dir(children) => insert_path(children, rest, cids),
        // A name already used as a file is now needed as a directory — don't
        // silently discard the file's content to make room; report the deeper
        // file as un-placed.
        Node::File(_) => false,
    }
}

// ----------------------------------------------------------------------
// pack writer
// ----------------------------------------------------------------------

/// A sealed blob's placement, used to build both the pack trailer and the
/// index entry.
struct BlobMeta {
    id_hex: String,
    id_raw: [u8; 32],
    kind: BlobKind,
    offset: u64,
    length: u64,
}

/// Streams re-chunked blobs into restic pack files and accumulates the
/// global index. Data and tree blobs go into separate pack streams
/// (restic convention).
struct Exporter {
    master: ResticKey,
    dst: PathBuf,
    data_buf: Vec<u8>,
    data_metas: Vec<BlobMeta>,
    tree_buf: Vec<u8>,
    tree_metas: Vec<BlobMeta>,
    /// Dedup across the whole repo, keyed by blob id (SHA-256 of plaintext).
    seen: HashSet<[u8; 32]>,
    index_packs: Vec<IndexPackJson>,
}

impl Exporter {
    fn new(dst: &Path, master: ResticKey) -> Self {
        Self {
            master,
            dst: dst.to_path_buf(),
            data_buf: Vec::new(),
            data_metas: Vec::new(),
            tree_buf: Vec::new(),
            tree_metas: Vec::new(),
            seen: HashSet::new(),
            index_packs: Vec::new(),
        }
    }

    /// Seal `plaintext` as a blob of `kind`, appending it to the matching
    /// pack stream, and return its id (64-hex SHA-256 of the plaintext).
    /// Deduplicates: identical content is stored + indexed once.
    fn put_blob(&mut self, plaintext: &[u8], kind: BlobKind) -> Result<String, MigrateError> {
        let id: [u8; 32] = Sha256::digest(plaintext).into();
        let id_hex = hex_of(&id);
        if !self.seen.insert(id) {
            return Ok(id_hex); // already stored + indexed
        }
        let sealed = restic_seal(&self.master, plaintext);
        let should_flush = {
            let (buf, metas) = match kind {
                BlobKind::Data => (&mut self.data_buf, &mut self.data_metas),
                BlobKind::Tree => (&mut self.tree_buf, &mut self.tree_metas),
            };
            let offset = buf.len() as u64;
            let length = sealed.len() as u64;
            buf.extend_from_slice(&sealed);
            metas.push(BlobMeta {
                id_hex: id_hex.clone(),
                id_raw: id,
                kind,
                offset,
                length,
            });
            buf.len() >= PACK_ROLLOVER
        };
        if should_flush {
            self.flush(kind)?;
        }
        Ok(id_hex)
    }

    /// Finalize the current pack of `kind`: append the sealed header +
    /// `u32` length trailer, write `data/<aa>/<pack_id>`, and record every
    /// blob in the global index. No-op when the pack is empty.
    fn flush(&mut self, kind: BlobKind) -> Result<(), MigrateError> {
        let metas = match kind {
            BlobKind::Data => std::mem::take(&mut self.data_metas),
            BlobKind::Tree => std::mem::take(&mut self.tree_metas),
        };
        if metas.is_empty() {
            return Ok(());
        }
        let mut file = match kind {
            BlobKind::Data => std::mem::take(&mut self.data_buf),
            BlobKind::Tree => std::mem::take(&mut self.tree_buf),
        };

        // Plaintext pack header: one 37-byte entry per (uncompressed) blob.
        let mut header = Vec::with_capacity(metas.len() * 37);
        for m in &metas {
            header.push(m.kind.header_byte());
            header.extend_from_slice(&(m.length as u32).to_le_bytes());
            header.extend_from_slice(&m.id_raw);
        }
        let sealed_header = restic_seal(&self.master, &header);
        file.extend_from_slice(&sealed_header);
        file.extend_from_slice(&(sealed_header.len() as u32).to_le_bytes());

        let pack_id: [u8; 32] = Sha256::digest(&file).into();
        let pack_hex = hex_of(&pack_id);
        let sub = self.dst.join("data").join(&pack_hex[..2]);
        std::fs::create_dir_all(&sub).map_err(|e| io_err(&sub, e))?;
        let path = sub.join(&pack_hex);
        std::fs::write(&path, &file).map_err(|e| io_err(&path, e))?;

        let blobs = metas
            .iter()
            .map(|m| IndexBlobJson {
                id: m.id_hex.clone(),
                typ: m.kind.type_str(),
                offset: m.offset,
                length: m.length,
                uncompressed_length: None,
            })
            .collect();
        self.index_packs.push(IndexPackJson {
            id: pack_hex,
            blobs,
        });
        Ok(())
    }

    /// Re-chunk one file by reusing its CDR manifest boundaries — each CDR
    /// chunk becomes one restic data blob (in order), so the blobs
    /// concatenate to the file and identical content dedups.
    fn chunk_file(
        &mut self,
        manifest: &Manifest,
        plaintext: &[u8],
    ) -> Result<Vec<String>, MigrateError> {
        let mut cids = Vec::with_capacity(manifest.chunks.len());
        for c in &manifest.chunks {
            let start = c.offset as usize;
            let end = start + c.len as usize;
            cids.push(self.put_blob(&plaintext[start..end], BlobKind::Data)?);
        }
        Ok(cids)
    }

    /// Build restic tree blobs bottom-up from the directory trie and
    /// return the (root) tree id. Nodes are name-sorted (BTreeMap order).
    fn build_tree(&mut self, dir: &BTreeMap<String, Node>) -> Result<String, MigrateError> {
        let mut nodes = Vec::with_capacity(dir.len());
        for (name, node) in dir {
            match node {
                Node::Dir(children) => {
                    let subtree = self.build_tree(children)?;
                    nodes.push(TreeNodeJson {
                        name: name.clone(),
                        typ: "dir",
                        content: None,
                        subtree: Some(subtree),
                    });
                }
                Node::File(cids) => {
                    nodes.push(TreeNodeJson {
                        name: name.clone(),
                        typ: "file",
                        content: Some(cids.clone()),
                        subtree: None,
                    });
                }
            }
        }
        let json = serde_json::to_vec(&TreeJson { nodes }).map_err(|e| enc_err("tree json", e))?;
        self.put_blob(&json, BlobKind::Tree)
    }

    /// Seal + write one `snapshots/<id>` file (uncompressed repo file). The
    /// filename is the SHA-256 of the sealed bytes.
    fn write_snapshot(&self, snap: &Snapshot, root_tree: &str) -> Result<(), MigrateError> {
        let path_label = if snap.source.is_empty() {
            "/".to_string()
        } else {
            snap.source.clone()
        };
        let snapshot = SnapshotJson {
            time: ms_to_rfc3339(snap.created_at_ms),
            tree: root_tree.to_string(),
            paths: vec![path_label],
            hostname: "copythat".to_string(),
            username: "copythat".to_string(),
            program_version: format!("copythat-chunk {}", env!("CARGO_PKG_VERSION")),
        };
        let json = serde_json::to_vec(&snapshot).map_err(|e| enc_err("snapshot json", e))?;
        let sealed = restic_seal(&self.master, &json);
        let id: [u8; 32] = Sha256::digest(&sealed).into();
        let path = self.dst.join("snapshots").join(hex_of(&id));
        std::fs::write(&path, &sealed).map_err(|e| io_err(&path, e))
    }

    /// Flush the remaining packs and write the single sealed `index/<id>`.
    fn finish(&mut self) -> Result<(), MigrateError> {
        self.flush(BlobKind::Data)?;
        self.flush(BlobKind::Tree)?;
        let index = IndexJson {
            packs: std::mem::take(&mut self.index_packs),
        };
        let json = serde_json::to_vec(&index).map_err(|e| enc_err("index json", e))?;
        let sealed = restic_seal(&self.master, &json);
        let id: [u8; 32] = Sha256::digest(&sealed).into();
        let path = self.dst.join("index").join(hex_of(&id));
        std::fs::write(&path, &sealed).map_err(|e| io_err(&path, e))
    }
}

/// Write `keys/<id>`: scrypt a fresh KEK from `password` + a random salt,
/// wrap the master key with it, and frame the keyfile JSON. The filename
/// is the SHA-256 of the keyfile's exact bytes.
fn write_keyfile(dst: &Path, password: &str, master: &ResticKey) -> Result<(), MigrateError> {
    let mut salt = [0u8; 64];
    fill_random(&mut salt);
    let kek = derive_kek(password, &salt)?;

    let master_json = MasterKeyJson {
        mac: MacKeyJson {
            k: b64_encode(&master.mac_k),
            r: b64_encode(&master.mac_r),
        },
        encrypt: b64_encode(&master.encrypt),
    };
    let master_bytes =
        serde_json::to_vec(&master_json).map_err(|e| enc_err("master key json", e))?;
    let sealed = restic_seal(&kek, &master_bytes);

    let kf = KeyFileJson {
        created: ms_to_rfc3339(0),
        username: "copythat".to_string(),
        hostname: "copythat".to_string(),
        kdf: "scrypt".to_string(),
        n: SCRYPT_N,
        r: SCRYPT_R,
        p: SCRYPT_P,
        salt: b64_encode(&salt),
        data: b64_encode(&sealed),
    };
    let kf_bytes = serde_json::to_vec(&kf).map_err(|e| enc_err("keyfile json", e))?;
    let id: [u8; 32] = Sha256::digest(&kf_bytes).into();
    let path = dst.join("keys").join(hex_of(&id));
    std::fs::write(&path, &kf_bytes).map_err(|e| io_err(&path, e))
}

/// Write `<dst>/config`: `{version, id, chunker_polynomial}` sealed with
/// the master key, UNCOMPRESSED (bootstrap data). `import_restic` never
/// reads it, but [`super::RepoFormat::detect`] requires the file to exist.
fn write_config(dst: &Path, master: &ResticKey, repo_id_hex: &str) -> Result<(), MigrateError> {
    let config = ConfigJson {
        version: 2,
        id: repo_id_hex.to_string(),
        chunker_polynomial: CHUNKER_POLYNOMIAL.to_string(),
    };
    let json = serde_json::to_vec(&config).map_err(|e| enc_err("config json", e))?;
    let sealed = restic_seal(master, &json);
    let path = dst.join("config");
    std::fs::write(&path, &sealed).map_err(|e| io_err(&path, e))
}

/// Export every snapshot of the CDR-0 repository `src` into a fresh,
/// encrypted restic (v2) repository under `dst`, unlockable with
/// `password`. The result round-trips through
/// [`import_restic`](super::restic) to byte-identical files.
pub(super) fn export_restic(
    src: &Repository,
    dst: &Path,
    password: &str,
) -> Result<MigrateReport, MigrateError> {
    for sub in ["keys", "index", "snapshots", "data"] {
        let p = dst.join(sub);
        std::fs::create_dir_all(&p).map_err(|e| io_err(&p, e))?;
    }

    let master = random_master_key();
    let mut repo_id = [0u8; 32];
    fill_random(&mut repo_id);
    let repo_id_hex = hex_of(&repo_id);

    write_keyfile(dst, password, &master)?;
    write_config(dst, &master, &repo_id_hex)?;

    let mut exporter = Exporter::new(dst, master);
    let mut report = MigrateReport::default();
    for snap in src.load_all_snapshots()? {
        let mut trie: BTreeMap<String, Node> = BTreeMap::new();
        for fe in &snap.files {
            // Pull the whole file (parity with the importer, which also
            // buffers one file at a time), then re-chunk it.
            let plaintext =
                materialise_range(src.store(), &fe.manifest, 0, fe.manifest.size as usize)?;
            let cids = exporter.chunk_file(&fe.manifest, &plaintext)?;
            let comps: Vec<&str> = fe
                .path
                .split(['/', '\\'])
                .filter(|s| !s.is_empty())
                .collect();
            if insert_path(&mut trie, &comps, cids) {
                report.files += 1;
            } else {
                // Degenerate/colliding path — content blobs were written but no
                // tree node references them; count it truthfully as skipped
                // rather than reporting a file the snapshot doesn't contain.
                report.skipped += 1;
            }
        }
        // The root tree must be packed (so its id resolves in the index)
        // before the snapshot names it.
        let root_tree = exporter.build_tree(&trie)?;
        exporter.write_snapshot(&snap, &root_tree)?;
        report.snapshots += 1;
    }

    exporter.finish()?;
    Ok(report)
}
