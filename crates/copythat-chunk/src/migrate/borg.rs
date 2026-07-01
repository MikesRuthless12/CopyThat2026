//! Borg (borgbackup 1.x) repository importer (Phase 50).
//!
//! Reads an encrypted **repokey-mode** Borg repository and reconstructs
//! each file's bytes, which [`super::migrate`] re-ingests into a CDR-0
//! [`Repository`] (re-chunked with FastCDC + BLAKE3 — Borg's buzhash
//! chunk IDs aren't portable).
//!
//! # Format (validated against `borgbackup 1.4.4`)
//!
//! - **Key** (`repokey`): the `config` INI's `key =` field is a base64
//!   msgpack blob `{algorithm, salt, iterations, data, hash}`. PBKDF2-
//!   HMAC-SHA256(passphrase, salt, iterations) → a 32-byte key that
//!   AES-256-CTR-decrypts `data` (IV 0) into the inner key msgpack
//!   (`enc_key`, `enc_hmac_key`); the passphrase is verified by
//!   HMAC-SHA256(derived, plaintext) == `hash`.
//! - **Segments** (`data/<n>/<seg>`): a `BORG_SEG` log of
//!   `crc(4) size(4 LE) tag(1) [key(32)] [data]` entries (PUT=0,
//!   DELETE=1, COMMIT=2; last PUT of a key wins).
//! - **Chunk** (`data` of a PUT): `0x03 || HMAC-SHA256(32) || IV(8) ||
//!   ciphertext`; AES-256-CTR (IV = `IV(8) || 0*8`) then a 2-byte
//!   compression header (`00`=none, `01`=LZ4, `03`=zstd, zlib magic).
//! - **Manifest** = the chunk with id `0*32` → msgpack `{archives:
//!   {name: {id, time}}}`. Each archive chunk → `{items: [chunk-id…]}`;
//!   those chunks concatenate to a msgpack **stream** of item maps
//!   (`{path, mode, chunks: [[id, size, csize]…]}`); a file's content is
//!   its chunks concatenated + decompressed.
//!
//! MessagePack and the LZ4-block decompressor are hand-rolled here, and
//! every cipher (HMAC/SHA-256/AES-CTR/zstd/flate2) is already in the
//! workspace — so Borg support adds **no new third-party crate**.

use std::collections::HashMap;
use std::path::Path;

use aes::cipher::generic_array::GenericArray;
use aes::cipher::{KeyIvInit, StreamCipher};
use base64::Engine as _;
use ctr::Ctr128BE;
use hmac::{Hmac, Mac};
use sha2::Sha256;

use super::{MigrateError, MigrateReport};
use crate::chunker::Chunker;
use crate::repository::{FileEntry, Repository, SnapshotKind};

type Aes256Ctr = Ctr128BE<aes::Aes256>;
type HmacSha256 = Hmac<Sha256>;

const MANIFEST_ID: [u8; 32] = [0u8; 32];

// ----------------------------------------------------------------------
// minimal MessagePack reader
//
// Borg packs with the *legacy* spec (`use_bin_type=False`), so raw byte
// strings arrive as msgpack `str`. We treat `str` and `bin` identically
// as opaque bytes and never assume UTF-8.
// ----------------------------------------------------------------------

// A complete msgpack value. The Borg importer only reads bytes / uints /
// arrays / maps, but the reader still parses bool / float / int / nil so
// it advances the cursor correctly across map values it skips — hence the
// allow on those carried-but-unread payloads.
#[derive(Debug, Clone)]
#[allow(dead_code)]
enum Mp {
    Nil,
    Bool(bool),
    Uint(u64),
    Int(i64),
    Bytes(Vec<u8>),
    Array(Vec<Mp>),
    Map(Vec<(Mp, Mp)>),
    F64(f64),
}

impl Mp {
    fn as_bytes(&self) -> Option<&[u8]> {
        if let Mp::Bytes(b) = self {
            Some(b)
        } else {
            None
        }
    }
    fn as_u64(&self) -> Option<u64> {
        match self {
            Mp::Uint(u) => Some(*u),
            Mp::Int(i) if *i >= 0 => Some(*i as u64),
            _ => None,
        }
    }
    fn as_array(&self) -> Option<&[Mp]> {
        if let Mp::Array(a) = self {
            Some(a)
        } else {
            None
        }
    }
    fn id32(&self) -> Option<[u8; 32]> {
        self.as_bytes().and_then(|b| <[u8; 32]>::try_from(b).ok())
    }
    /// Map lookup by ASCII string key (Borg map keys are `str`).
    fn get(&self, key: &str) -> Option<&Mp> {
        if let Mp::Map(m) = self {
            m.iter()
                .find(|(k, _)| k.as_bytes() == Some(key.as_bytes()))
                .map(|(_, v)| v)
        } else {
            None
        }
    }
}

fn mp_err(s: impl std::fmt::Display) -> MigrateError {
    MigrateError::Decode(format!("borg msgpack: {s}"))
}

fn rd_u8(b: &[u8], p: &mut usize) -> Result<u8, MigrateError> {
    let v = *b.get(*p).ok_or_else(|| mp_err("eof"))?;
    *p += 1;
    Ok(v)
}

fn rd_n(b: &[u8], p: &mut usize, n: usize) -> Result<Vec<u8>, MigrateError> {
    let end = p.checked_add(n).ok_or_else(|| mp_err("len overflow"))?;
    let s = b.get(*p..end).ok_or_else(|| mp_err("slice eof"))?.to_vec();
    *p = end;
    Ok(s)
}

fn rd_be(b: &[u8], p: &mut usize, n: usize) -> Result<u64, MigrateError> {
    let mut v = 0u64;
    for _ in 0..n {
        v = (v << 8) | u64::from(rd_u8(b, p)?);
    }
    Ok(v)
}

/// Maximum msgpack nesting depth — guards the hand-rolled reader against
/// a malicious (even pre-authentication) blob of deeply nested containers
/// that would otherwise overflow the stack.
const MAX_MP_DEPTH: u32 = 100;

fn read_mp(b: &[u8], p: &mut usize) -> Result<Mp, MigrateError> {
    read_mp_d(b, p, 0)
}

fn read_mp_d(b: &[u8], p: &mut usize, depth: u32) -> Result<Mp, MigrateError> {
    if depth > MAX_MP_DEPTH {
        return Err(mp_err("nesting too deep"));
    }
    let t = rd_u8(b, p)?;
    Ok(match t {
        0x00..=0x7f => Mp::Uint(u64::from(t)),
        0xe0..=0xff => Mp::Int(i64::from(t as i8)),
        0x80..=0x8f => read_map(b, p, (t & 0x0f) as usize, depth)?,
        0x90..=0x9f => read_arr(b, p, (t & 0x0f) as usize, depth)?,
        0xa0..=0xbf => Mp::Bytes(rd_n(b, p, (t & 0x1f) as usize)?),
        0xc0 => Mp::Nil,
        0xc2 => Mp::Bool(false),
        0xc3 => Mp::Bool(true),
        0xc4 | 0xd9 => {
            let n = rd_be(b, p, 1)? as usize;
            Mp::Bytes(rd_n(b, p, n)?)
        }
        0xc5 | 0xda => {
            let n = rd_be(b, p, 2)? as usize;
            Mp::Bytes(rd_n(b, p, n)?)
        }
        0xc6 | 0xdb => {
            let n = rd_be(b, p, 4)? as usize;
            Mp::Bytes(rd_n(b, p, n)?)
        }
        0xca => Mp::F64(f64::from(f32::from_bits(rd_be(b, p, 4)? as u32))),
        0xcb => Mp::F64(f64::from_bits(rd_be(b, p, 8)?)),
        0xcc => Mp::Uint(rd_be(b, p, 1)?),
        0xcd => Mp::Uint(rd_be(b, p, 2)?),
        0xce => Mp::Uint(rd_be(b, p, 4)?),
        0xcf => Mp::Uint(rd_be(b, p, 8)?),
        0xd0 => Mp::Int(i64::from(rd_be(b, p, 1)? as u8 as i8)),
        0xd1 => Mp::Int(i64::from(rd_be(b, p, 2)? as u16 as i16)),
        0xd2 => Mp::Int(i64::from(rd_be(b, p, 4)? as u32 as i32)),
        0xd3 => Mp::Int(rd_be(b, p, 8)? as i64),
        0xdc => {
            let n = rd_be(b, p, 2)? as usize;
            read_arr(b, p, n, depth)?
        }
        0xdd => {
            let n = rd_be(b, p, 4)? as usize;
            read_arr(b, p, n, depth)?
        }
        0xde => {
            let n = rd_be(b, p, 2)? as usize;
            read_map(b, p, n, depth)?
        }
        0xdf => {
            let n = rd_be(b, p, 4)? as usize;
            read_map(b, p, n, depth)?
        }
        other => return Err(mp_err(format!("unsupported tag {other:#04x}"))),
    })
}

fn read_arr(b: &[u8], p: &mut usize, n: usize, depth: u32) -> Result<Mp, MigrateError> {
    let mut v = Vec::with_capacity(n.min(4096));
    for _ in 0..n {
        v.push(read_mp_d(b, p, depth + 1)?);
    }
    Ok(Mp::Array(v))
}

fn read_map(b: &[u8], p: &mut usize, n: usize, depth: u32) -> Result<Mp, MigrateError> {
    let mut v = Vec::with_capacity(n.min(4096));
    for _ in 0..n {
        let k = read_mp_d(b, p, depth + 1)?;
        let val = read_mp_d(b, p, depth + 1)?;
        v.push((k, val));
    }
    Ok(Mp::Map(v))
}

// ----------------------------------------------------------------------
// LZ4 block decompressor (input-driven; output size not needed)
// ----------------------------------------------------------------------

fn lz4_block_decompress(src: &[u8]) -> Result<Vec<u8>, MigrateError> {
    let mut out: Vec<u8> = Vec::with_capacity(src.len().saturating_mul(3));
    let mut i = 0usize;
    let lz = |s: &str| MigrateError::Decode(format!("borg lz4: {s}"));
    while i < src.len() {
        let token = src[i];
        i += 1;
        // literal run
        let mut lit = (token >> 4) as usize;
        if lit == 15 {
            loop {
                let x = *src.get(i).ok_or_else(|| lz("literal length eof"))?;
                i += 1;
                lit += x as usize;
                if x != 0xff {
                    break;
                }
            }
        }
        let end = i.checked_add(lit).ok_or_else(|| lz("overflow"))?;
        out.extend_from_slice(src.get(i..end).ok_or_else(|| lz("literal eof"))?);
        i = end;
        if i >= src.len() {
            break; // final literal run carries no match
        }
        // match
        let lo = *src.get(i).ok_or_else(|| lz("offset eof"))?;
        let hi = *src.get(i + 1).ok_or_else(|| lz("offset eof"))?;
        i += 2;
        let off = u16::from_le_bytes([lo, hi]) as usize;
        if off == 0 || off > out.len() {
            return Err(lz("bad match offset"));
        }
        let mut mlen = (token & 0x0f) as usize;
        if mlen == 15 {
            loop {
                let x = *src.get(i).ok_or_else(|| lz("match length eof"))?;
                i += 1;
                mlen += x as usize;
                if x != 0xff {
                    break;
                }
            }
        }
        mlen += 4; // minimum match
        let start = out.len() - off;
        for j in 0..mlen {
            // byte-by-byte for overlapping (RLE) matches
            let byte = out[start + j];
            out.push(byte);
        }
    }
    Ok(out)
}

// ----------------------------------------------------------------------
// crypto
// ----------------------------------------------------------------------

struct BorgKey {
    enc_key: [u8; 32],
    mac_key: [u8; 32],
}

fn hmac_sha256(key: &[u8], msg: &[u8]) -> [u8; 32] {
    let mut m = <HmacSha256 as Mac>::new_from_slice(key).expect("HMAC accepts any key length");
    m.update(msg);
    m.finalize().into_bytes().into()
}

/// PBKDF2-HMAC-SHA256 for a 32-byte key (a single output block, which is
/// all Borg's `repokey` needs).
fn pbkdf2_sha256_32(password: &[u8], salt: &[u8], iterations: u32) -> [u8; 32] {
    let mut block = salt.to_vec();
    block.extend_from_slice(&1u32.to_be_bytes()); // INT(i=1)
    let mut u = hmac_sha256(password, &block);
    let mut out = u;
    for _ in 1..iterations {
        u = hmac_sha256(password, &u);
        for k in 0..32 {
            out[k] ^= u[k];
        }
    }
    out
}

fn aes_ctr(key: &[u8; 32], iv16: &[u8; 16], data: &[u8]) -> Vec<u8> {
    let mut buf = data.to_vec();
    Aes256Ctr::new(
        GenericArray::from_slice(key),
        GenericArray::from_slice(iv16),
    )
    .apply_keystream(&mut buf);
    buf
}

/// Read the repokey `key =` value (a base64 blob spanning indented INI
/// continuation lines) from the `config` file.
fn parse_config_key(cfg: &str) -> Option<String> {
    let mut out = String::new();
    let mut collecting = false;
    for line in cfg.lines() {
        if collecting {
            if line.starts_with([' ', '\t']) {
                out.push_str(line.trim());
                continue;
            }
            break;
        }
        if let Some(rest) = line
            .strip_prefix("key =")
            .or_else(|| line.strip_prefix("key="))
        {
            out.push_str(rest.trim());
            collecting = true;
        }
    }
    if out.is_empty() { None } else { Some(out) }
}

fn load_borg_key(repo: &Path, password: &str) -> Result<BorgKey, MigrateError> {
    let cfg_path = repo.join("config");
    let cfg = std::fs::read_to_string(&cfg_path).map_err(|e| MigrateError::Io {
        path: cfg_path.clone(),
        source: e,
    })?;
    let key_b64 = parse_config_key(&cfg).ok_or_else(|| {
        MigrateError::Format(
            "no repokey `key` in borg config (keyfile-mode repos store the key outside the \
             repository and are not supported)"
                .into(),
        )
    })?;
    let blob = base64::engine::general_purpose::STANDARD
        .decode(key_b64)
        .map_err(|e| MigrateError::Decode(format!("borg key base64: {e}")))?;

    let mut p = 0;
    let m = read_mp(&blob, &mut p)?;
    let algorithm = m
        .get("algorithm")
        .and_then(Mp::as_bytes)
        .unwrap_or(b"sha256");
    if algorithm != b"sha256" {
        return Err(MigrateError::Format(format!(
            "unsupported borg key KDF algorithm {:?} (only sha256/PBKDF2)",
            String::from_utf8_lossy(algorithm)
        )));
    }
    let salt = m
        .get("salt")
        .and_then(Mp::as_bytes)
        .ok_or_else(|| mp_err("key salt"))?
        .to_vec();
    let iterations = m
        .get("iterations")
        .and_then(Mp::as_u64)
        .ok_or_else(|| mp_err("key iterations"))? as u32;
    let data = m
        .get("data")
        .and_then(Mp::as_bytes)
        .ok_or_else(|| mp_err("key data"))?
        .to_vec();
    let hash = m
        .get("hash")
        .and_then(Mp::as_bytes)
        .ok_or_else(|| mp_err("key hash"))?
        .to_vec();

    let derived = pbkdf2_sha256_32(password.as_bytes(), &salt, iterations);
    let plain = aes_ctr(&derived, &[0u8; 16], &data);
    if !super::ct_eq(&hmac_sha256(&derived, &plain), &hash) {
        return Err(MigrateError::Decrypt(
            "borg passphrase incorrect (key HMAC mismatch)".into(),
        ));
    }

    let mut q = 0;
    let inner = read_mp(&plain, &mut q)?;
    let enc_key = inner
        .get("enc_key")
        .and_then(Mp::id32)
        .ok_or_else(|| mp_err("inner enc_key not 32 bytes"))?;
    let mac_key = inner
        .get("enc_hmac_key")
        .and_then(Mp::id32)
        .ok_or_else(|| mp_err("inner enc_hmac_key not 32 bytes"))?;
    Ok(BorgKey { enc_key, mac_key })
}

/// Decrypt + decompress one Borg chunk (`0x03 || HMAC || IV || ct`).
fn decrypt_chunk(key: &BorgKey, raw: &[u8]) -> Result<Vec<u8>, MigrateError> {
    if raw.len() < 41 {
        return Err(MigrateError::Decrypt(
            "borg chunk shorter than 41 bytes".into(),
        ));
    }
    let typ = raw[0];
    if typ != 0x03 {
        return Err(MigrateError::Format(format!(
            "unsupported borg chunk type {typ:#04x} (only 0x03 = repokey AES-256-CTR + HMAC-SHA256)"
        )));
    }
    let mac = &raw[1..33];
    let iv8 = &raw[33..41];
    let ct = &raw[41..];
    // HMAC-SHA256 covers `IV(8) || ciphertext` (everything after the
    // type byte and the MAC itself).
    let mut h = <HmacSha256 as Mac>::new_from_slice(&key.mac_key).expect("HMAC key");
    h.update(&raw[33..]);
    if !super::ct_eq(&h.finalize().into_bytes(), mac) {
        return Err(MigrateError::Decrypt(
            "borg chunk HMAC mismatch (corrupt repo or wrong key)".into(),
        ));
    }
    // Borg's AES-CTR counter is the 64-bit block nonce as a 16-byte
    // big-endian integer, so the stored 8-byte IV is the *low* half.
    let mut iv16 = [0u8; 16];
    iv16[8..].copy_from_slice(iv8);
    let plain = aes_ctr(&key.enc_key, &iv16, ct);
    decompress_borg(&plain)
}

fn decompress_borg(plain: &[u8]) -> Result<Vec<u8>, MigrateError> {
    if plain.len() < 2 {
        return Err(MigrateError::Decode(
            "borg chunk plaintext < 2 bytes".into(),
        ));
    }
    // Legacy zlib has no Borg header — detect by the zlib magic.
    if plain[0] & 0x0f == 0x08 && ((u16::from(plain[0]) << 8) | u16::from(plain[1])) % 31 == 0 {
        return inflate_zlib(plain);
    }
    let body = &plain[2..];
    match plain[0] {
        0x00 => Ok(body.to_vec()),          // CNONE
        0x01 => lz4_block_decompress(body), // LZ4
        0x02 => Err(MigrateError::Format(
            "borg LZMA compression not supported".into(),
        )),
        0x03 => zstd::stream::decode_all(body)
            .map_err(|e| MigrateError::Decode(format!("borg zstd: {e}"))),
        other => Err(MigrateError::Format(format!(
            "unknown borg compression id {other:#04x}"
        ))),
    }
}

fn inflate_zlib(data: &[u8]) -> Result<Vec<u8>, MigrateError> {
    use std::io::Read;
    let mut out = Vec::new();
    flate2::read::ZlibDecoder::new(data)
        .read_to_end(&mut out)
        .map_err(|e| MigrateError::Decode(format!("borg zlib: {e}")))?;
    Ok(out)
}

// ----------------------------------------------------------------------
// segment log
// ----------------------------------------------------------------------

/// A live chunk's on-disk location: which segment file + the byte range
/// of its (still-encrypted) data.
struct ChunkLoc {
    seg: usize,
    offset: u64,
    len: usize,
}

/// The repository's live chunks as segment files + `chunk-id → location`,
/// so chunk data is read on demand (seek) rather than held in memory.
struct Segments {
    paths: Vec<std::path::PathBuf>,
    locs: HashMap<[u8; 32], ChunkLoc>,
}

/// Scan every segment, recording each live chunk's LOCATION (last PUT of
/// a key wins; DELETE removes it). Chunk bytes are read later on demand,
/// so a multi-GB Borg repo never loads wholesale into RAM.
fn scan_segments(repo: &Path) -> Result<Segments, MigrateError> {
    let data_dir = repo.join("data");
    let io = |path: &Path, e| MigrateError::Io {
        path: path.to_path_buf(),
        source: e,
    };
    let mut files: Vec<(u64, std::path::PathBuf)> = Vec::new();
    for d in std::fs::read_dir(&data_dir).map_err(|e| io(&data_dir, e))? {
        let dp = d.map_err(|e| io(&data_dir, e))?.path();
        if !dp.is_dir() {
            continue;
        }
        for f in std::fs::read_dir(&dp).map_err(|e| io(&dp, e))? {
            let fp = f.map_err(|e| io(&dp, e))?.path();
            if let Some(n) = fp
                .file_name()
                .and_then(|s| s.to_str())
                .and_then(|s| s.parse::<u64>().ok())
            {
                files.push((n, fp));
            }
        }
    }
    files.sort_by_key(|(n, _)| *n);

    let mut paths = Vec::with_capacity(files.len());
    let mut locs: HashMap<[u8; 32], ChunkLoc> = HashMap::new();
    for (_, path) in files {
        let bytes = std::fs::read(&path).map_err(|e| io(&path, e))?;
        let seg = paths.len();
        parse_segment(seg, &bytes, &mut locs)?;
        paths.push(path);
    }
    Ok(Segments { paths, locs })
}

fn parse_segment(
    seg: usize,
    b: &[u8],
    locs: &mut HashMap<[u8; 32], ChunkLoc>,
) -> Result<(), MigrateError> {
    const MAGIC: &[u8; 8] = b"BORG_SEG";
    if b.len() < 8 || &b[..8] != MAGIC {
        return Err(MigrateError::Format("bad borg segment magic".into()));
    }
    let mut p = 8usize;
    while p + 9 <= b.len() {
        p += 4; // skip crc
        let size = u32::from_le_bytes([b[p], b[p + 1], b[p + 2], b[p + 3]]) as usize;
        p += 4;
        let tag = b[p];
        p += 1;
        if size < 9 {
            return Err(MigrateError::Format("borg segment entry size < 9".into()));
        }
        let payload = size - 9;
        let entry_end = p
            .checked_add(payload)
            .ok_or_else(|| mp_err("segment overflow"))?;
        if entry_end > b.len() {
            return Err(MigrateError::Format("borg segment entry past end".into()));
        }
        match tag {
            // PUT (0): key(32) || data.  PUT2 (3): xxh64(8) || key(32) || data.
            0 | 3 => {
                let key_at = if tag == 3 { p + 8 } else { p };
                if entry_end < key_at + 32 {
                    return Err(MigrateError::Format("borg PUT entry too small".into()));
                }
                let mut key = [0u8; 32];
                key.copy_from_slice(&b[key_at..key_at + 32]);
                let data_at = key_at + 32;
                locs.insert(
                    key,
                    ChunkLoc {
                        seg,
                        offset: data_at as u64,
                        len: entry_end - data_at,
                    },
                );
            }
            1 => {
                // DELETE
                if payload >= 32 {
                    let mut key = [0u8; 32];
                    key.copy_from_slice(&b[p..p + 32]);
                    locs.remove(&key);
                }
            }
            2 => {} // COMMIT
            other => {
                return Err(MigrateError::Format(format!(
                    "unknown borg segment tag {other}"
                )));
            }
        }
        p = entry_end;
    }
    Ok(())
}

/// Read + decrypt one chunk by id, reading only its slice from the
/// segment file (memory-bounded).
fn read_chunk(key: &BorgKey, segs: &Segments, id: &[u8; 32]) -> Result<Vec<u8>, MigrateError> {
    use std::io::{Read as _, Seek as _, SeekFrom};
    let loc = segs.locs.get(id).ok_or_else(|| {
        MigrateError::Format(format!("borg chunk {} not found", crate::types::hex_of(id)))
    })?;
    let path = &segs.paths[loc.seg];
    let mut f = std::fs::File::open(path).map_err(|e| MigrateError::Io {
        path: path.clone(),
        source: e,
    })?;
    f.seek(SeekFrom::Start(loc.offset))
        .map_err(|e| MigrateError::Io {
            path: path.clone(),
            source: e,
        })?;
    let mut buf = vec![0u8; loc.len];
    f.read_exact(&mut buf).map_err(|e| MigrateError::Io {
        path: path.clone(),
        source: e,
    })?;
    decrypt_chunk(key, &buf)
}

// ----------------------------------------------------------------------
// import
// ----------------------------------------------------------------------

fn parse_borg_time(b: &[u8]) -> i64 {
    let s = String::from_utf8_lossy(b);
    chrono::NaiveDateTime::parse_from_str(s.trim(), "%Y-%m-%dT%H:%M:%S%.f")
        .map(|dt| dt.and_utc().timestamp_millis())
        .unwrap_or(0)
}

/// Import every archive of a Borg repository into a CDR-0 [`Repository`].
pub(super) fn import_borg(
    repo: &Path,
    password: &str,
    dst_root: &Path,
) -> Result<MigrateReport, MigrateError> {
    let key = load_borg_key(repo, password)?;
    let segs = scan_segments(repo)?;

    let dest = Repository::open(dst_root)?;
    super::write_cdr_descriptor(dst_root)?;
    let chunker = Chunker::default();

    let manifest_bytes = read_chunk(&key, &segs, &MANIFEST_ID)?;
    let mut p = 0;
    let manifest = read_mp(&manifest_bytes, &mut p)?;
    let archives = manifest
        .get("archives")
        .ok_or_else(|| mp_err("manifest has no archives"))?;
    let Mp::Map(arch_map) = archives else {
        return Err(mp_err("manifest archives is not a map"));
    };

    let mut report = MigrateReport::default();
    for (name_v, meta_v) in arch_map {
        let name = String::from_utf8_lossy(name_v.as_bytes().unwrap_or(b"archive")).into_owned();
        let archive_id = meta_v
            .get("id")
            .and_then(Mp::id32)
            .ok_or_else(|| mp_err("archive id not 32 bytes"))?;
        import_archive(
            &key,
            &segs,
            &dest,
            &chunker,
            &name,
            &archive_id,
            &mut report,
        )?;
    }
    Ok(report)
}

fn import_archive(
    key: &BorgKey,
    segs: &Segments,
    dest: &Repository,
    chunker: &Chunker,
    name: &str,
    archive_id: &[u8; 32],
    report: &mut MigrateReport,
) -> Result<(), MigrateError> {
    let arch_bytes = read_chunk(key, segs, archive_id)?;
    let mut p = 0;
    let arch = read_mp(&arch_bytes, &mut p)?;

    // The archive's `items` chunks concatenate to a msgpack stream.
    let item_ids = arch
        .get("items")
        .and_then(Mp::as_array)
        .ok_or_else(|| mp_err("archive has no items list"))?;
    let mut stream = Vec::new();
    for id_v in item_ids {
        let id = id_v.id32().ok_or_else(|| mp_err("item-stream chunk id"))?;
        stream.extend_from_slice(&read_chunk(key, segs, &id)?);
    }

    let created_at_ms = arch
        .get("time")
        .and_then(Mp::as_bytes)
        .map_or(0, parse_borg_time);
    let label = format!("borg: {name}");

    let mut entries: Vec<FileEntry> = Vec::new();
    let mut q = 0;
    while q < stream.len() {
        let item = read_mp(&stream, &mut q)?;
        let path = item
            .get("path")
            .and_then(Mp::as_bytes)
            .map(|b| String::from_utf8_lossy(b).into_owned())
            .unwrap_or_default();
        match item.get("chunks").and_then(Mp::as_array) {
            Some(file_chunks) => {
                let mut bytes = Vec::new();
                for fc in file_chunks {
                    // each entry is [id, size, csize]
                    let id = fc
                        .as_array()
                        .and_then(<[Mp]>::first)
                        .and_then(Mp::id32)
                        .ok_or_else(|| mp_err("file chunk id"))?;
                    bytes.extend_from_slice(&read_chunk(key, segs, &id)?);
                }
                let manifest = crate::manifest::chunk_into_store(
                    dest.store(),
                    chunker,
                    &bytes,
                    dest.compression(),
                )?
                .1;
                entries.push(FileEntry {
                    path: super::safe_path(&path),
                    manifest,
                });
            }
            // directories / symlinks / devices carry no chunk content.
            None => report.skipped += 1,
        }
    }

    let n = entries.len() as u64;
    dest.record(SnapshotKind::Backup, &label, created_at_ms, entries)?;
    report.snapshots += 1;
    report.files += n;
    Ok(())
}
