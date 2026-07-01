//! Kopia repository importer (Phase 50d).
//!
//! Reads an encrypted Kopia **filesystem** repository (format v3, index
//! v2, BLAKE2B-256-128, AES256-GCM-HMAC-SHA256) and reconstructs each
//! file's bytes, which [`super::migrate`] re-ingests into a CDR-0
//! [`Repository`].
//!
//! # Two distinct AEAD schemes (verified against kopia 0.23.1 source)
//!
//! - **Format blob** (`kopia.repository`): scrypt(passphrase, salt =
//!   uniqueID) → masterKey; then `internal/crypto` AES-256-GCM where
//!   `aesKey = HKDF(masterKey, salt = uniqueID, "AES")` and the GCM AAD
//!   is `HKDF(masterKey, salt = uniqueID, "CHECKSUM")`. This yields the
//!   inner format JSON (the repository `masterKey` + `hmacSecret`).
//! - **Contents + index blobs** (`repo/encryption`): a one-time
//!   `keyDerivationSecret = HKDF(masterKey, "encryption")`, then per
//!   blob `aesKey = HMAC-SHA256(keyDerivationSecret, salt)`, nonce =
//!   `data[..12]`, AAD = `salt`, where **salt = the bare 16-byte BLAKE2b
//!   hash** (no content-prefix byte). The prefix byte is used only for
//!   the index lookup key.
//!
//! Index v2 → `contentID → {pack blob, offset, length, compression}`;
//! manifests live in `m`-prefixed contents (gzip JSON) → snapshots →
//! `rootEntry.obj`; objects resolve to content(s) (with `kopia:indirect`
//! seek-tables for large files) and directories are `kopia:directory`
//! JSON. All crypto + codecs (aes-gcm / hkdf / hmac / sha2 / scrypt /
//! zstd / flate2) are in-tree — no new crate.

use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use aes_gcm::aead::{Aead, Payload};
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use base64::Engine as _;
use hkdf::Hkdf;
use hmac::{Hmac, Mac};
use serde::Deserialize;
use sha2::Sha256;

use super::{MigrateError, MigrateReport};
use crate::chunker::Chunker;
use crate::repository::{FileEntry, Repository, SnapshotKind};

type HmacSha256 = Hmac<Sha256>;

fn dec(ctx: &str, e: impl std::fmt::Display) -> MigrateError {
    MigrateError::Decode(format!("kopia {ctx}: {e}"))
}

fn b64(s: &str) -> Result<Vec<u8>, MigrateError> {
    base64::engine::general_purpose::STANDARD
        .decode(s.trim())
        .map_err(|e| dec("base64", e))
}

fn hex_encode(b: &[u8]) -> String {
    use std::fmt::Write as _;
    let mut s = String::with_capacity(b.len() * 2);
    for x in b {
        let _ = write!(s, "{x:02x}");
    }
    s
}

/// Decode an ASCII-hex byte slice. Operates on bytes (never `&str`
/// slicing) so a non-hex / non-ASCII / multibyte input errors cleanly
/// instead of panicking on a char boundary.
fn hexdec(s: &[u8]) -> Result<Vec<u8>, MigrateError> {
    if s.len() % 2 != 0 {
        return Err(dec("hex", "odd length"));
    }
    s.chunks_exact(2)
        .map(|pair| {
            match (
                (pair[0] as char).to_digit(16),
                (pair[1] as char).to_digit(16),
            ) {
                (Some(h), Some(l)) => Ok((h * 16 + l) as u8),
                _ => Err(dec("hex", "non-hex digit")),
            }
        })
        .collect()
}

// ----------------------------------------------------------------------
// format blob
// ----------------------------------------------------------------------

#[derive(Deserialize)]
struct FormatBlob {
    #[serde(rename = "uniqueID")]
    unique_id: String,
    #[serde(rename = "keyAlgo")]
    key_algo: String,
    #[serde(rename = "encryptedBlockFormat")]
    encrypted_block_format: String,
}

#[derive(Deserialize)]
struct InnerFormatEnvelope {
    format: InnerFormat,
}

#[derive(Deserialize)]
struct InnerFormat {
    #[serde(rename = "masterKey")]
    master_key: String,
}

/// HKDF-SHA256 → 32 bytes (kopia's `DeriveKeyFromMasterKey` / `deriveKey`).
fn hkdf32(ikm: &[u8], salt: &[u8], info: &[u8]) -> [u8; 32] {
    let hk = Hkdf::<Sha256>::new(Some(salt), ikm);
    let mut okm = [0u8; 32];
    hk.expand(info, &mut okm)
        .expect("32 is a valid HKDF length");
    okm
}

fn aes_gcm_open(key: &[u8; 32], nonce12: &[u8], ct_tag: &[u8], aad: &[u8]) -> Option<Vec<u8>> {
    let cipher = Aes256Gcm::new_from_slice(key).ok()?;
    cipher
        .decrypt(Nonce::from_slice(nonce12), Payload { msg: ct_tag, aad })
        .ok()
}

/// Decrypt `kopia.repository` → the inner repository-format JSON.
fn decrypt_format(repo: &Path, password: &str) -> Result<Vec<u8>, MigrateError> {
    let mut path = repo.join("kopia.repository");
    if !path.is_file() {
        path = repo.join("kopia.repository.f");
    }
    let raw = std::fs::read(&path).map_err(|e| MigrateError::Io {
        path: path.clone(),
        source: e,
    })?;
    let fb: FormatBlob = serde_json::from_slice(&raw).map_err(|e| dec("format json", e))?;
    if fb.key_algo != "scrypt-65536-8-1" {
        return Err(MigrateError::Format(format!(
            "unsupported kopia keyAlgo {:?}",
            fb.key_algo
        )));
    }
    let unique_id = b64(&fb.unique_id)?;
    let enc = b64(&fb.encrypted_block_format)?;
    if enc.len() < 12 + 16 {
        return Err(MigrateError::Format("kopia format blob too short".into()));
    }

    let params = scrypt::Params::new(16, 8, 1, 32).map_err(|e| dec("scrypt params", e))?;
    let mut master = [0u8; 32];
    scrypt::scrypt(password.as_bytes(), &unique_id, &params, &mut master)
        .map_err(|e| dec("scrypt", e))?;

    // Format-blob layer: HKDF "AES" key + "CHECKSUM" AAD, salt = uniqueID.
    let aes_key = hkdf32(&master, &unique_id, b"AES");
    let auth = hkdf32(&master, &unique_id, b"CHECKSUM");
    aes_gcm_open(&aes_key, &enc[..12], &enc[12..], &auth).ok_or_else(|| {
        MigrateError::Decrypt("kopia format decrypt failed (wrong passphrase?)".into())
    })
}

// ----------------------------------------------------------------------
// blob store (filesystem backend: blobs are sharded files with a `.f` suffix)
// ----------------------------------------------------------------------

struct BlobStore {
    /// blob name (e.g. `p9371785…-s…`) → on-disk file path.
    map: HashMap<String, PathBuf>,
}

impl BlobStore {
    fn build(repo: &Path) -> Result<Self, MigrateError> {
        let mut map = HashMap::new();
        let mut stack = vec![repo.to_path_buf()];
        while let Some(dir) = stack.pop() {
            let entries = std::fs::read_dir(&dir).map_err(|e| MigrateError::Io {
                path: dir.clone(),
                source: e,
            })?;
            for e in entries {
                let p = e
                    .map_err(|e| MigrateError::Io {
                        path: dir.clone(),
                        source: e,
                    })?
                    .path();
                if p.is_dir() {
                    stack.push(p);
                } else if let Some(name) = p.file_name().and_then(|s| s.to_str()) {
                    // Only `.f` blob files; reconstruct the blob name by
                    // joining the sharded path components (the backend
                    // inserts dir separators into the blob id).
                    if let Some(stripped) = name.strip_suffix(".f") {
                        let rel = p.strip_prefix(repo).unwrap_or(&p);
                        let mut blob = String::new();
                        for comp in rel.components() {
                            let c = comp.as_os_str().to_string_lossy();
                            blob.push_str(c.strip_suffix(".f").unwrap_or(&c));
                        }
                        // `blob` now == joined components incl. the final
                        // (already `.f`-stripped) piece.
                        let _ = stripped;
                        map.insert(blob, p);
                    }
                }
            }
        }
        Ok(Self { map })
    }

    fn read(&self, name: &str) -> Result<Vec<u8>, MigrateError> {
        let path = self
            .map
            .get(name)
            .ok_or_else(|| MigrateError::Format(format!("kopia blob {name} not found")))?;
        std::fs::read(path).map_err(|e| MigrateError::Io {
            path: path.clone(),
            source: e,
        })
    }

    fn read_slice(&self, name: &str, offset: u32, len: u32) -> Result<Vec<u8>, MigrateError> {
        let path = self
            .map
            .get(name)
            .ok_or_else(|| MigrateError::Format(format!("kopia pack {name} not found")))?;
        let mut f = std::fs::File::open(path).map_err(|e| MigrateError::Io {
            path: path.clone(),
            source: e,
        })?;
        f.seek(SeekFrom::Start(u64::from(offset)))
            .map_err(|e| MigrateError::Io {
                path: path.clone(),
                source: e,
            })?;
        let mut buf = vec![0u8; len as usize];
        f.read_exact(&mut buf).map_err(|e| MigrateError::Io {
            path: path.clone(),
            source: e,
        })?;
        Ok(buf)
    }

    fn names_with_prefix(&self, prefix: &str) -> Vec<String> {
        self.map
            .keys()
            .filter(|k| k.starts_with(prefix))
            .cloned()
            .collect()
    }
}

// ----------------------------------------------------------------------
// content / index decryption (scheme #2)
// ----------------------------------------------------------------------

/// `aesKey = HMAC-SHA256(keyDerivationSecret, salt)`, then AES-256-GCM
/// open with nonce = `data[..12]`, AAD = `salt`. `salt` is the bare
/// 16-byte content hash.
fn content_decrypt(kds: &[u8], salt: &[u8], data: &[u8]) -> Result<Vec<u8>, MigrateError> {
    if data.len() < 12 + 16 {
        return Err(MigrateError::Decrypt(
            "kopia content shorter than 28 bytes".into(),
        ));
    }
    let mut mac = <HmacSha256 as Mac>::new_from_slice(kds).expect("HMAC key");
    mac.update(salt);
    let key: [u8; 32] = mac.finalize().into_bytes().into();
    aes_gcm_open(&key, &data[..12], &data[12..], salt)
        .ok_or_else(|| MigrateError::Decrypt("kopia content GCM authentication failed".into()))
}

fn decompress(header_id: u32, body: &[u8]) -> Result<Vec<u8>, MigrateError> {
    use std::io::Read as _;
    match header_id {
        // gzip + pgzip
        0x1000..=0x1002 | 0x1300..=0x1302 => {
            let mut out = Vec::new();
            flate2::read::GzDecoder::new(body)
                .read_to_end(&mut out)
                .map_err(|e| dec("gzip", e))?;
            Ok(out)
        }
        // zstd
        0x1100..=0x1103 => zstd::stream::decode_all(body).map_err(|e| dec("zstd", e)),
        // raw DEFLATE
        0x1500..=0x1502 => {
            let mut out = Vec::new();
            flate2::read::DeflateDecoder::new(body)
                .read_to_end(&mut out)
                .map_err(|e| dec("deflate", e))?;
            Ok(out)
        }
        other => Err(MigrateError::Format(format!(
            "unsupported kopia compression header {other:#06x} (s2/lz4 not supported)"
        ))),
    }
}

// ----------------------------------------------------------------------
// index v2
// ----------------------------------------------------------------------

#[derive(Clone)]
struct ContentInfo {
    pack_blob_id: String,
    pack_offset: u32,
    packed_length: u32,
    compression_header_id: u32,
    deleted: bool,
    timestamp: i64,
}

fn be16(b: &[u8]) -> u16 {
    u16::from_be_bytes([b[0], b[1]])
}
fn be24(b: &[u8]) -> u32 {
    u32::from_be_bytes([0, b[0], b[1], b[2]])
}
fn be32(b: &[u8]) -> u32 {
    u32::from_be_bytes([b[0], b[1], b[2], b[3]])
}

/// Parse a decrypted v2 packindex into `(key bytes, ContentInfo)` pairs.
fn parse_v2(data: &[u8]) -> Result<Vec<(Vec<u8>, ContentInfo)>, MigrateError> {
    if data.len() < 17 || data[0] != 0x02 {
        return Err(MigrateError::Format("kopia index not v2".into()));
    }
    let key_size = data[1] as usize;
    let entry_size = be16(&data[2..4]) as usize;
    let entry_count = be32(&data[4..8]) as usize;
    let pack_count = be32(&data[8..12]) as usize;
    let format_count = data[12] as usize;
    let base_ts = i64::from(be32(&data[13..17]));
    if key_size < 2 || !(16..=19).contains(&entry_size) {
        return Err(MigrateError::Format("kopia index header invalid".into()));
    }
    let stride = key_size + entry_size;
    let entries_off = 17usize;
    let overflow = || MigrateError::Format("kopia index size overflow".into());
    let packs_off = entry_count
        .checked_mul(stride)
        .and_then(|x| x.checked_add(entries_off))
        .ok_or_else(overflow)?;
    let formats_off = pack_count
        .checked_mul(5)
        .and_then(|x| x.checked_add(packs_off))
        .ok_or_else(overflow)?;
    let extra_off = format_count
        .checked_mul(6)
        .and_then(|x| x.checked_add(formats_off))
        .ok_or_else(overflow)?;
    if data.len() < extra_off {
        return Err(MigrateError::Format("kopia index truncated".into()));
    }

    // Pack-id table → names.
    let mut pack_ids = Vec::with_capacity(pack_count);
    for i in 0..pack_count {
        let off = packs_off + i * 5;
        let name_len = data[off] as usize;
        let name_off = be32(&data[off + 1..off + 5]) as usize;
        let name = data
            .get(name_off..name_off + name_len)
            .ok_or_else(|| MigrateError::Format("kopia pack name out of range".into()))?;
        pack_ids.push(String::from_utf8_lossy(name).into_owned());
    }

    // Format table → (compressionHeaderID, encryptionKeyID).
    let mut formats = Vec::with_capacity(format_count);
    for i in 0..format_count {
        let off = formats_off + i * 6;
        formats.push((be32(&data[off..off + 4]), data[off + 5]));
    }

    let mut out = Vec::with_capacity(entry_count);
    for i in 0..entry_count {
        let rec = entries_off + i * stride;
        let key = data[rec..rec + key_size].to_vec();
        let v = &data[rec + key_size..rec + stride];
        let timestamp = i64::from(be32(&v[0..4])) + base_ts;
        let deleted = v[4] & 0x80 != 0;
        let pack_offset = be32(&v[4..8]) & 0x7fff_ffff;
        let mut packed_length = be24(&v[11..14]);
        let mut pack_idx = u32::from(be16(&v[14..16]));
        let fid = if entry_size > 16 { v[16] as usize } else { 0 };
        if entry_size > 17 {
            pack_idx |= u32::from(v[17]) << 16;
        }
        if entry_size > 18 {
            packed_length |= u32::from(v[18] & 0x0f) << 24;
        }
        let (compression_header_id, enc_key_id) = match formats.get(fid) {
            Some(f) => *f,
            // formatID 0 with no format table = uncompressed (valid);
            // any other out-of-range id is a corrupt index, not a guess.
            None if fid == 0 => (0, 0),
            None => {
                return Err(MigrateError::Format(format!(
                    "kopia index formatID {fid} out of range ({} formats)",
                    formats.len()
                )));
            }
        };
        if enc_key_id != 0 {
            return Err(MigrateError::Format(format!(
                "unsupported kopia encryptionKeyID {enc_key_id}"
            )));
        }
        let pack_blob_id = pack_ids
            .get(pack_idx as usize)
            .cloned()
            .ok_or_else(|| MigrateError::Format("kopia pack index out of range".into()))?;
        out.push((
            key,
            ContentInfo {
                pack_blob_id,
                pack_offset,
                packed_length,
                compression_header_id,
                deleted,
                timestamp,
            },
        ));
    }
    Ok(out)
}

/// The salt used to derive the GCM AAD/key for an index blob: the 32 hex
/// chars immediately before the first `-` in the blob name, hex-decoded.
fn index_blob_salt(name: &str) -> Result<Vec<u8>, MigrateError> {
    let head = name.split('-').next().unwrap_or(name).as_bytes();
    if head.len() < 32 {
        return Err(MigrateError::Format(format!(
            "kopia index name {name} too short"
        )));
    }
    hexdec(&head[head.len() - 32..])
}

/// Load + merge every epoch index blob into `key → ContentInfo`
/// (newest-timestamp wins; live beats tombstone; deleted suppressed).
fn load_index(
    store: &BlobStore,
    kds: &[u8],
) -> Result<HashMap<Vec<u8>, ContentInfo>, MigrateError> {
    let mut merged: HashMap<Vec<u8>, ContentInfo> = HashMap::new();
    let mut names = store.names_with_prefix("xn");
    names.extend(store.names_with_prefix("xs"));
    names.extend(store.names_with_prefix("xr"));
    // Legacy (non-epoch) single-index layout uses bare `n…` index blobs.
    names.extend(store.names_with_prefix("n"));
    for name in names {
        let salt = index_blob_salt(&name)?;
        let raw = store.read(&name)?;
        let plain = content_decrypt(kds, &salt, &raw)?;
        for (key, info) in parse_v2(&plain)? {
            let win = match merged.get(&key) {
                None => true,
                Some(prev) => {
                    info.timestamp > prev.timestamp
                        || (info.timestamp == prev.timestamp && prev.deleted && !info.deleted)
                        || (info.timestamp == prev.timestamp
                            && info.deleted == prev.deleted
                            && info.pack_blob_id > prev.pack_blob_id)
                }
            };
            if win {
                merged.insert(key, info);
            }
        }
    }
    merged.retain(|_, v| !v.deleted);
    Ok(merged)
}

// ----------------------------------------------------------------------
// content + object resolution
// ----------------------------------------------------------------------

/// Index lookup key for a content-ID string: `[prefix byte][hash]`
/// (prefix `0x00` when the id is pure hex).
fn content_key(id: &str) -> Result<Vec<u8>, MigrateError> {
    let bytes = id.as_bytes();
    let (prefix, hex) = if bytes.len() % 2 == 1 {
        (bytes[0], &bytes[1..])
    } else {
        (0u8, bytes)
    };
    let mut key = Vec::with_capacity(1 + hex.len() / 2);
    key.push(prefix);
    key.extend_from_slice(&hexdec(hex)?);
    Ok(key)
}

fn read_content(
    store: &BlobStore,
    index: &HashMap<Vec<u8>, ContentInfo>,
    kds: &[u8],
    id: &str,
) -> Result<Vec<u8>, MigrateError> {
    let key = content_key(id)?;
    let info = index
        .get(&key)
        .ok_or_else(|| MigrateError::Format(format!("kopia content {id} not in index")))?;
    let raw = store.read_slice(&info.pack_blob_id, info.pack_offset, info.packed_length)?;
    // salt = the bare 16-byte hash (the key without its prefix byte).
    let plain = content_decrypt(kds, &key[1..], &raw)?;
    if info.compression_header_id == 0 {
        return Ok(plain);
    }
    if plain.len() < 4 || be32(&plain[..4]) != info.compression_header_id {
        return Err(MigrateError::Format(
            "kopia content compression header mismatch".into(),
        ));
    }
    decompress(info.compression_header_id, &plain[4..])
}

#[derive(Deserialize)]
struct IndirectObject {
    #[serde(default)]
    entries: Vec<IndirectEntry>,
}
#[derive(Deserialize)]
struct IndirectEntry {
    o: String,
}

/// Resolve an object ID to its full bytes (handles `I…` indirection +
/// `Z…` object-level compression; `D` is a dropped legacy prefix).
fn resolve_object(
    store: &BlobStore,
    index: &HashMap<Vec<u8>, ContentInfo>,
    kds: &[u8],
    obj_id: &str,
    depth: u32,
) -> Result<Vec<u8>, MigrateError> {
    if depth > 64 {
        return Err(MigrateError::Format(
            "kopia object indirection too deep".into(),
        ));
    }
    let mut s = obj_id;
    let mut indirection = 0u32;
    while let Some(rest) = s.strip_prefix('I') {
        indirection += 1;
        s = rest;
    }
    let compressed = s.starts_with('Z');
    s = s.strip_prefix('Z').unwrap_or(s);
    s = s.strip_prefix('D').unwrap_or(s);

    if indirection > 0 {
        // `s` is the content id of a seek-table; read it (one level down).
        let inner_id = format!("{}{}", "I".repeat((indirection - 1) as usize), s);
        let table_bytes = resolve_object(store, index, kds, &inner_id, depth + 1)?;
        let table: IndirectObject =
            serde_json::from_slice(&table_bytes).map_err(|e| dec("indirect json", e))?;
        let mut out = Vec::new();
        for e in &table.entries {
            out.extend_from_slice(&resolve_object(store, index, kds, &e.o, depth + 1)?);
        }
        return Ok(out);
    }

    let mut data = read_content(store, index, kds, s)?;
    if compressed {
        // object-level `Z`: a 4-byte BE compression header + body.
        if data.len() < 4 {
            return Err(MigrateError::Format("kopia Z object too short".into()));
        }
        let hid = be32(&data[..4]);
        data = decompress(hid, &data[4..])?;
    }
    Ok(data)
}

// ----------------------------------------------------------------------
// manifests → snapshots
// ----------------------------------------------------------------------

#[derive(Deserialize)]
struct ManifestEnvelope {
    entries: Vec<ManifestEntry>,
}
#[derive(Deserialize)]
struct ManifestEntry {
    id: String,
    #[serde(default)]
    labels: HashMap<String, String>,
    #[serde(default)]
    modified: String,
    #[serde(default)]
    deleted: bool,
    #[serde(default)]
    data: serde_json::Value,
}

#[derive(Deserialize)]
struct SnapshotManifest {
    #[serde(rename = "rootEntry")]
    root_entry: Option<DirEntry>,
    #[serde(default)]
    source: SnapshotSource,
    #[serde(rename = "startTime", default)]
    start_time: String,
}
#[derive(Deserialize, Default)]
struct SnapshotSource {
    #[serde(default)]
    path: String,
    #[serde(default)]
    host: String,
}
#[derive(Deserialize)]
struct DirEntry {
    #[serde(default)]
    name: String,
    #[serde(rename = "type", default)]
    typ: String,
    obj: String,
}
#[derive(Deserialize)]
struct DirManifest {
    #[serde(rename = "stream")]
    stream: String,
    #[serde(default)]
    entries: Vec<DirEntry>,
}

struct Snapshot {
    root_obj: String,
    path: String,
    host: String,
    created_at_ms: i64,
}

fn gunzip(data: &[u8]) -> Result<Vec<u8>, MigrateError> {
    let mut out = Vec::new();
    flate2::read::GzDecoder::new(data)
        .read_to_end(&mut out)
        .map_err(|e| dec("manifest gzip", e))?;
    Ok(out)
}

/// Parse a manifest entry's RFC3339 `modified` into nanoseconds for
/// correct chronological dedup — a plain string compare misorders the
/// variable fractional-second precision Kopia/Go emit.
fn manifest_ts(s: &str) -> i64 {
    chrono::DateTime::parse_from_rfc3339(s.trim())
        .ok()
        .and_then(|d| d.timestamp_nanos_opt())
        .unwrap_or(i64::MIN)
}

fn load_snapshots(
    store: &BlobStore,
    index: &HashMap<Vec<u8>, ContentInfo>,
    kds: &[u8],
) -> Result<Vec<Snapshot>, MigrateError> {
    // Collect manifest entries from every `m`-prefixed content.
    let mut entries: HashMap<String, ManifestEntry> = HashMap::new();
    for (key, _) in index.iter().filter(|(k, _)| k.first() == Some(&b'm')) {
        // rebuild the content-id string ('m' + hex of the hash).
        let id = format!("m{}", hex_encode(&key[1..]));
        let content = read_content(store, index, kds, &id)?;
        let json = gunzip(&content)?;
        let env: ManifestEnvelope =
            serde_json::from_slice(&json).map_err(|e| dec("manifest json", e))?;
        for e in env.entries {
            match entries.get(&e.id) {
                Some(prev) if manifest_ts(&prev.modified) >= manifest_ts(&e.modified) => {}
                _ => {
                    entries.insert(e.id.clone(), e);
                }
            }
        }
    }

    let mut snaps = Vec::new();
    for e in entries.values() {
        if e.deleted || e.labels.get("type").map(String::as_str) != Some("snapshot") {
            continue;
        }
        let manifest: SnapshotManifest =
            serde_json::from_value(e.data.clone()).map_err(|err| dec("snapshot manifest", err))?;
        let Some(root) = manifest.root_entry else {
            continue;
        };
        let created_at_ms = chrono::DateTime::parse_from_rfc3339(manifest.start_time.trim())
            .map(|dt| dt.timestamp_millis())
            .unwrap_or(0);
        snaps.push(Snapshot {
            root_obj: root.obj,
            path: manifest.source.path,
            host: manifest.source.host,
            created_at_ms,
        });
    }
    Ok(snaps)
}

// ----------------------------------------------------------------------
// import
// ----------------------------------------------------------------------

struct Ctx<'a> {
    store: &'a BlobStore,
    index: &'a HashMap<Vec<u8>, ContentInfo>,
    kds: &'a [u8],
    dest: &'a Repository,
    chunker: Chunker,
}

fn walk_dir(
    ctx: &Ctx,
    dir_obj: &str,
    prefix: &str,
    depth: u32,
    out: &mut Vec<FileEntry>,
    skipped: &mut u64,
) -> Result<(), MigrateError> {
    if depth > 512 {
        return Err(MigrateError::Format(
            "kopia directory nesting too deep".into(),
        ));
    }
    let raw = resolve_object(ctx.store, ctx.index, ctx.kds, dir_obj, 0)?;
    let manifest: DirManifest =
        serde_json::from_slice(&raw).map_err(|e| dec("directory json", e))?;
    if manifest.stream != "kopia:directory" {
        return Err(MigrateError::Format("kopia: not a directory stream".into()));
    }
    for entry in &manifest.entries {
        let path = format!("{prefix}/{}", entry.name);
        match entry.typ.as_str() {
            "d" => walk_dir(ctx, &entry.obj, &path, depth + 1, out, skipped)?,
            "f" => {
                let bytes = resolve_object(ctx.store, ctx.index, ctx.kds, &entry.obj, 0)?;
                let manifest = crate::manifest::chunk_into_store(
                    ctx.dest.store(),
                    &ctx.chunker,
                    &bytes,
                    ctx.dest.compression(),
                )?
                .1;
                out.push(FileEntry {
                    path: super::safe_path(&path),
                    manifest,
                });
            }
            // symlinks / unknown carry no chunk content.
            _ => *skipped += 1,
        }
    }
    Ok(())
}

/// Import every snapshot of a Kopia repository into a CDR-0 repository.
pub(super) fn import_kopia(
    repo: &Path,
    password: &str,
    dst_root: &Path,
) -> Result<MigrateReport, MigrateError> {
    let format_json = decrypt_format(repo, password)?;
    let inner: InnerFormatEnvelope =
        serde_json::from_slice(&format_json).map_err(|e| dec("inner format json", e))?;
    let master_key = b64(&inner.format.master_key)?;
    let kds = hkdf32(&master_key, b"encryption", b"");

    let store = BlobStore::build(repo)?;
    let index = load_index(&store, &kds)?;
    let snapshots = load_snapshots(&store, &index, &kds)?;

    let dest = Repository::open(dst_root)?;
    super::write_cdr_descriptor(dst_root)?;
    let ctx = Ctx {
        store: &store,
        index: &index,
        kds: &kds,
        dest: &dest,
        chunker: Chunker::default(),
    };

    let mut report = MigrateReport::default();
    for snap in &snapshots {
        let mut entries: Vec<FileEntry> = Vec::new();
        walk_dir(
            &ctx,
            &snap.root_obj,
            "",
            0,
            &mut entries,
            &mut report.skipped,
        )?;
        let label = format!("kopia: {} ({})", snap.path, snap.host);
        let n = entries.len() as u64;
        dest.record(SnapshotKind::Backup, &label, snap.created_at_ms, entries)?;
        report.snapshots += 1;
        report.files += n;
    }
    Ok(report)
}
