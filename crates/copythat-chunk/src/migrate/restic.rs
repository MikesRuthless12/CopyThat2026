//! restic repository importer (Phase 50).
//!
//! Reads an encrypted **restic** repository (format v1 + v2) and
//! reconstructs each file's bytes, which [`super::migrate`] re-ingests
//! into a CDR-0 [`Repository`]. restic's chunk IDs are not portable
//! (per-repo random polynomial + SHA-256 of plaintext), so we
//! reconstruct file bytes and let the CDR store re-chunk with its own
//! FastCDC + BLAKE3 — a faithful content migration.
//!
//! # Crypto
//!
//! - **KDF:** scrypt over the passphrase + the keyfile salt → a 64-byte
//!   key-encryption key (`encrypt[0..32]`, `mac.k[32..48]`,
//!   `mac.r[48..64]`).
//! - **AEAD:** AES-256-CTR with **Poly1305-AES** authentication. Each
//!   sealed unit is `IV(16) || ciphertext || MAC(16)`; the Poly1305 key
//!   is `mac.r || AES-128(mac.k, IV)` and the MAC covers the ciphertext.
//!   The KEK unwraps the keyfile's `data` field → the master key, which
//!   decrypts the index / snapshots / trees / data blobs.
//! - **Compression (v2):** standalone index/snapshot files carry a
//!   leading `0x02` version byte → zstd; pack blobs are zstd when their
//!   index entry has `uncompressed_length`.
//!
//! Validated against a real `restic 0.17.3` v2 repository fixture
//! (`tests/fixtures/restic-repo`, passphrase `testpass`).

use std::collections::{HashMap, HashSet};
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

use aes::Aes128;
use aes::cipher::generic_array::GenericArray;
use aes::cipher::{BlockEncrypt, KeyInit, KeyIvInit, StreamCipher};
use base64::Engine as _;
use ctr::Ctr128BE;
use serde::Deserialize;
use sha2::{Digest, Sha256};

use super::{MigrateError, MigrateReport};
use crate::chunker::Chunker;
use crate::repository::{FileEntry, Repository, SnapshotKind};

type Aes256Ctr = Ctr128BE<aes::Aes256>;

/// A restic key: the master key, and (transiently) the scrypt-derived
/// key-encryption key, share this layout.
struct ResticKey {
    encrypt: [u8; 32],
    mac_k: [u8; 16],
    mac_r: [u8; 16],
}

#[derive(Deserialize)]
struct KeyFile {
    kdf: String,
    #[serde(rename = "N")]
    n: u32,
    r: u32,
    p: u32,
    salt: String,
    data: String,
}

#[derive(Deserialize)]
struct MasterKeyJson {
    mac: MacKeyJson,
    encrypt: String,
}

#[derive(Deserialize)]
struct MacKeyJson {
    k: String,
    r: String,
}

#[derive(Deserialize)]
struct SnapshotJson {
    tree: String,
    #[serde(default)]
    time: String,
    #[serde(default)]
    hostname: String,
    #[serde(default)]
    paths: Vec<String>,
}

#[derive(Deserialize)]
struct IndexJson {
    #[serde(default)]
    packs: Vec<IndexPack>,
    /// IDs of the index files this one replaces. restic's prune/repack
    /// rewrites the index and lists the obsolete index IDs here.
    #[serde(default)]
    supersedes: Vec<String>,
}

#[derive(Deserialize)]
struct IndexPack {
    id: String,
    blobs: Vec<IndexBlob>,
}

#[derive(Deserialize)]
struct IndexBlob {
    id: String,
    offset: u64,
    length: u64,
    #[serde(default)]
    uncompressed_length: Option<u64>,
}

#[derive(Deserialize)]
struct TreeJson {
    #[serde(default)]
    nodes: Vec<TreeNode>,
}

#[derive(Deserialize)]
struct TreeNode {
    name: String,
    #[serde(rename = "type")]
    typ: String,
    // restic uses explicit `null` for the unused side: directory nodes
    // carry `"content": null`, file nodes `"subtree": null`. `Option`
    // accepts both null and absent.
    #[serde(default)]
    content: Option<Vec<String>>,
    #[serde(default)]
    subtree: Option<String>,
}

/// Where a blob lives: which pack file, the on-disk slice, and whether
/// it is zstd-compressed.
struct BlobLoc {
    pack_id: String,
    offset: u64,
    length: u64,
    uncompressed_length: Option<u64>,
}

fn dec_err(ctx: &str, e: impl std::fmt::Display) -> MigrateError {
    MigrateError::Decode(format!("{ctx}: {e}"))
}

fn b64(s: &str) -> Result<Vec<u8>, MigrateError> {
    base64::engine::general_purpose::STANDARD
        .decode(s.trim())
        .map_err(|e| dec_err("base64", e))
}

/// Constant-time byte-slice equality for MAC verification.
fn ct_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b) {
        diff |= x ^ y;
    }
    diff == 0
}

/// restic object IDs are 64-char lowercase hex (SHA-256). Validate one
/// before using it to build a filesystem path — this rejects path
/// traversal (`..`, slashes) and any non-ASCII byte from a malicious
/// index (which would also panic the `[..2]` slice on a multi-byte
/// char boundary).
fn is_hex_id(s: &str) -> bool {
    s.len() == 64 && s.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
}

/// Decrypt one restic AEAD unit (`IV || ciphertext || MAC`), verifying
/// the Poly1305-AES tag before returning the AES-256-CTR plaintext.
fn restic_decrypt(key: &ResticKey, data: &[u8]) -> Result<Vec<u8>, MigrateError> {
    if data.len() < 32 {
        return Err(MigrateError::Decrypt(
            "restic blob shorter than 32 bytes".into(),
        ));
    }
    let iv = &data[..16];
    let ct = &data[16..data.len() - 16];
    let tag = &data[data.len() - 16..];

    // Poly1305-AES: s = AES-128(mac.k, IV); key = mac.r || s.
    let aes = Aes128::new(GenericArray::from_slice(&key.mac_k));
    let mut s = *GenericArray::from_slice(iv);
    aes.encrypt_block(&mut s);
    let mut poly_key = [0u8; 32];
    poly_key[..16].copy_from_slice(&key.mac_r);
    poly_key[16..].copy_from_slice(s.as_slice());
    let computed =
        poly1305::Poly1305::new(GenericArray::from_slice(&poly_key)).compute_unpadded(ct);
    if !ct_eq(computed.as_slice(), tag) {
        return Err(MigrateError::Decrypt(
            "restic MAC mismatch (wrong passphrase or corrupt data)".into(),
        ));
    }

    let mut pt = ct.to_vec();
    Aes256Ctr::new(
        GenericArray::from_slice(&key.encrypt),
        GenericArray::from_slice(iv),
    )
    .apply_keystream(&mut pt);
    Ok(pt)
}

/// Decrypt a standalone repo file (index / snapshot / config) and undo
/// its v2 version-byte compression framing.
fn decrypt_repo_file(key: &ResticKey, data: &[u8]) -> Result<Vec<u8>, MigrateError> {
    let pt = restic_decrypt(key, data)?;
    match pt.first() {
        Some(0x02) => zstd::stream::decode_all(&pt[1..]).map_err(|e| dec_err("zstd", e)),
        _ => Ok(pt),
    }
}

/// scrypt the passphrase against **every** key in `keys/` and return the
/// master key from the first that unwraps. restic repositories can hold
/// several keys (one per password added with `restic key add`), in
/// arbitrary filesystem order, so trying only the first would reject a
/// valid passphrase that happens to match a later key.
fn load_master_key(repo: &Path, password: &str) -> Result<ResticKey, MigrateError> {
    let keys_dir = repo.join("keys");
    let mut tried = 0u32;
    let mut last_err: Option<MigrateError> = None;
    for entry in std::fs::read_dir(&keys_dir).map_err(|e| MigrateError::Io {
        path: keys_dir.clone(),
        source: e,
    })? {
        let path = entry
            .map_err(|e| MigrateError::Io {
                path: keys_dir.clone(),
                source: e,
            })?
            .path();
        if !path.is_file() {
            continue;
        }
        tried += 1;
        match try_keyfile(&path, password) {
            Ok(key) => return Ok(key),
            Err(e) => last_err = Some(e),
        }
    }
    if tried == 0 {
        return Err(MigrateError::Format("no keys/* file in restic repo".into()));
    }
    Err(last_err
        .unwrap_or_else(|| MigrateError::Decrypt("no restic key matched the passphrase".into())))
}

/// Try to unwrap a single restic keyfile with `password`: scrypt → KEK →
/// decrypt the wrapped master key. A wrong passphrase fails the keyfile's
/// Poly1305-AES MAC and returns `Decrypt`.
fn try_keyfile(path: &Path, password: &str) -> Result<ResticKey, MigrateError> {
    let raw = std::fs::read(path).map_err(|e| MigrateError::Io {
        path: path.to_path_buf(),
        source: e,
    })?;
    let kf: KeyFile = serde_json::from_slice(&raw).map_err(|e| dec_err("keyfile json", e))?;
    if kf.kdf != "scrypt" {
        return Err(MigrateError::Format(format!(
            "unsupported restic kdf {:?} (only scrypt)",
            kf.kdf
        )));
    }
    if !kf.n.is_power_of_two() {
        return Err(MigrateError::Format(format!(
            "restic scrypt N {} is not a power of two",
            kf.n
        )));
    }
    let salt = b64(&kf.salt)?;
    let params = scrypt::Params::new(kf.n.trailing_zeros() as u8, kf.r, kf.p, 64)
        .map_err(|e| dec_err("scrypt params", e))?;
    let mut kek = [0u8; 64];
    scrypt::scrypt(password.as_bytes(), &salt, &params, &mut kek)
        .map_err(|e| dec_err("scrypt", e))?;
    let kek_key = ResticKey {
        encrypt: kek[0..32].try_into().expect("32"),
        mac_k: kek[32..48].try_into().expect("16"),
        mac_r: kek[48..64].try_into().expect("16"),
    };

    let data = b64(&kf.data)?;
    let master_json = restic_decrypt(&kek_key, &data)?;
    let mk: MasterKeyJson =
        serde_json::from_slice(&master_json).map_err(|e| dec_err("master key json", e))?;
    let encrypt = b64(&mk.encrypt)?;
    let mac_k = b64(&mk.mac.k)?;
    let mac_r = b64(&mk.mac.r)?;
    Ok(ResticKey {
        encrypt: encrypt
            .as_slice()
            .try_into()
            .map_err(|_| MigrateError::Format("master encrypt key not 32 bytes".into()))?,
        mac_k: mac_k
            .as_slice()
            .try_into()
            .map_err(|_| MigrateError::Format("master mac.k not 16 bytes".into()))?,
        mac_r: mac_r
            .as_slice()
            .try_into()
            .map_err(|_| MigrateError::Format("master mac.r not 16 bytes".into()))?,
    })
}

/// Build `blob id → location` from every **current** `index/*` file.
/// restic's prune/repack rewrites the index and lists the obsolete index
/// IDs in `supersedes`; honoring that avoids resolving a blob to a pack
/// that prune has already deleted (which would abort an otherwise-valid
/// migration).
fn load_index(repo: &Path, key: &ResticKey) -> Result<HashMap<String, BlobLoc>, MigrateError> {
    let dir = repo.join("index");
    let mut map = HashMap::new();
    if !dir.is_dir() {
        return Ok(map);
    }
    // First pass: parse every index, keyed by its own ID (the filename),
    // and union the set of index IDs they declare superseded.
    let mut parsed: Vec<(String, IndexJson)> = Vec::new();
    let mut superseded: HashSet<String> = HashSet::new();
    for e in std::fs::read_dir(&dir).map_err(|e| MigrateError::Io {
        path: dir.clone(),
        source: e,
    })? {
        let p = e
            .map_err(|e| MigrateError::Io {
                path: dir.clone(),
                source: e,
            })?
            .path();
        if !p.is_file() {
            continue;
        }
        let own_id = p
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
            .to_string();
        let raw = std::fs::read(&p).map_err(|e| MigrateError::Io {
            path: p.clone(),
            source: e,
        })?;
        let json = decrypt_repo_file(key, &raw)?;
        let idx: IndexJson = serde_json::from_slice(&json).map_err(|e| dec_err("index json", e))?;
        for s in &idx.supersedes {
            superseded.insert(s.clone());
        }
        parsed.push((own_id, idx));
    }
    // Second pass: only non-superseded indexes contribute blob locations.
    for (own_id, idx) in parsed {
        if superseded.contains(&own_id) {
            continue;
        }
        for pack in idx.packs {
            for b in pack.blobs {
                map.insert(
                    b.id,
                    BlobLoc {
                        pack_id: pack.id.clone(),
                        offset: b.offset,
                        length: b.length,
                        uncompressed_length: b.uncompressed_length,
                    },
                );
            }
        }
    }
    Ok(map)
}

/// Read + decrypt + (if compressed) decompress a single blob.
fn read_blob(
    repo: &Path,
    key: &ResticKey,
    index: &HashMap<String, BlobLoc>,
    id: &str,
) -> Result<Vec<u8>, MigrateError> {
    let loc = index
        .get(id)
        .ok_or_else(|| MigrateError::Format(format!("blob {id} not found in index")))?;
    if !is_hex_id(&loc.pack_id) {
        return Err(MigrateError::Format(format!(
            "invalid restic pack id {:?} (expected 64-char lowercase hex)",
            loc.pack_id
        )));
    }
    let pack_path = repo.join("data").join(&loc.pack_id[..2]).join(&loc.pack_id);
    let mut f = std::fs::File::open(&pack_path).map_err(|e| MigrateError::Io {
        path: pack_path.clone(),
        source: e,
    })?;
    f.seek(SeekFrom::Start(loc.offset))
        .map_err(|e| MigrateError::Io {
            path: pack_path.clone(),
            source: e,
        })?;
    let mut buf = vec![0u8; loc.length as usize];
    f.read_exact(&mut buf).map_err(|e| MigrateError::Io {
        path: pack_path.clone(),
        source: e,
    })?;
    let pt = restic_decrypt(key, &buf)?;
    let plaintext = if loc.uncompressed_length.is_some() {
        zstd::stream::decode_all(&pt[..]).map_err(|e| dec_err("blob zstd", e))?
    } else {
        pt
    };
    // A restic blob ID is the SHA-256 of its plaintext. Verify it (as
    // restic does on every load) so an index/pack inconsistency surfaces
    // as an error instead of silently migrating the wrong content.
    let digest: [u8; 32] = Sha256::digest(&plaintext).into();
    if crate::types::hex_of(&digest) != id {
        return Err(MigrateError::Decrypt(format!(
            "restic blob {id} failed its SHA-256 integrity check (index/pack mismatch)"
        )));
    }
    Ok(plaintext)
}

/// Maximum tree-recursion depth. Guards against a corrupt or malicious
/// repo whose tree blobs form a cycle, which would otherwise overflow
/// the stack; real filesystem trees never approach this.
const MAX_TREE_DEPTH: u32 = 512;

/// Read-only context shared across a single import.
struct ImportCtx<'a> {
    repo: &'a Path,
    key: &'a ResticKey,
    index: &'a HashMap<String, BlobLoc>,
    dest: &'a Repository,
    chunker: Chunker,
}

/// Recursively reconstruct every file under `tree_id`, **ingesting each
/// file into the destination store as it is reconstructed** (so only one
/// file is ever held in memory, never the whole snapshot) and pushing the
/// resulting [`FileEntry`] into `out`. `skipped` counts nodes that carry
/// no chunk content (symlinks, devices, FIFOs, empty dirs) rather than
/// dropping them silently.
fn walk_tree(
    ctx: &ImportCtx,
    tree_id: &str,
    prefix: &str,
    depth: u32,
    out: &mut Vec<FileEntry>,
    skipped: &mut u64,
) -> Result<(), MigrateError> {
    if depth > MAX_TREE_DEPTH {
        return Err(MigrateError::Format(format!(
            "restic tree nests deeper than {MAX_TREE_DEPTH} (cycle or corruption?)"
        )));
    }
    let tree_bytes = read_blob(ctx.repo, ctx.key, ctx.index, tree_id)?;
    let tree: TreeJson =
        serde_json::from_slice(&tree_bytes).map_err(|e| dec_err("tree json", e))?;
    for node in tree.nodes {
        let path = format!("{prefix}/{}", node.name);
        match node.typ.as_str() {
            "dir" => match node.subtree.as_deref() {
                Some(sub) => walk_tree(ctx, sub, &path, depth + 1, out, skipped)?,
                None => *skipped += 1, // empty directory — no content to store
            },
            "file" => {
                let mut bytes = Vec::new();
                for cid in node.content.iter().flatten() {
                    bytes.extend_from_slice(&read_blob(ctx.repo, ctx.key, ctx.index, cid)?);
                }
                let manifest =
                    crate::manifest::chunk_into_store(ctx.dest.store(), &ctx.chunker, &bytes)?.1;
                out.push(FileEntry { path, manifest });
            }
            // symlinks / devices / FIFOs carry no chunk content.
            _ => *skipped += 1,
        }
    }
    Ok(())
}

/// Import every snapshot of a restic repository into a CDR-0
/// [`Repository`] at `dst_root`.
pub(super) fn import_restic(
    repo: &Path,
    password: &str,
    dst_root: &Path,
) -> Result<MigrateReport, MigrateError> {
    let key = load_master_key(repo, password)?;
    let index = load_index(repo, &key)?;

    let dest = Repository::open(dst_root)?;
    super::write_cdr_descriptor(dst_root)?;
    let ctx = ImportCtx {
        repo,
        key: &key,
        index: &index,
        dest: &dest,
        chunker: Chunker::default(),
    };

    let snaps_dir = repo.join("snapshots");
    let mut report = MigrateReport::default();
    for e in std::fs::read_dir(&snaps_dir).map_err(|e| MigrateError::Io {
        path: snaps_dir.clone(),
        source: e,
    })? {
        let p = e
            .map_err(|e| MigrateError::Io {
                path: snaps_dir.clone(),
                source: e,
            })?
            .path();
        if !p.is_file() {
            continue;
        }
        let raw = std::fs::read(&p).map_err(|e| MigrateError::Io {
            path: p.clone(),
            source: e,
        })?;
        let json = decrypt_repo_file(&key, &raw)?;
        let snap: SnapshotJson =
            serde_json::from_slice(&json).map_err(|e| dec_err("snapshot json", e))?;

        let mut entries: Vec<FileEntry> = Vec::new();
        walk_tree(&ctx, &snap.tree, "", 0, &mut entries, &mut report.skipped)?;

        let created_at_ms = chrono::DateTime::parse_from_rfc3339(snap.time.trim())
            .map(|dt| dt.timestamp_millis())
            .unwrap_or(0);
        let label = if snap.paths.is_empty() {
            format!("restic snapshot ({})", snap.hostname)
        } else {
            format!("restic: {} ({})", snap.paths.join(", "), snap.hostname)
        };
        let n = entries.len() as u64;
        dest.record(SnapshotKind::Backup, &label, created_at_ms, entries)?;
        report.snapshots += 1;
        report.files += n;
    }
    Ok(report)
}
