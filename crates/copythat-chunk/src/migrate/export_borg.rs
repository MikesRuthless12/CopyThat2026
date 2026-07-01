//! Borg (borgbackup 1.x) repository **exporter** (Phase 50e).
//!
//! The inverse of [`super::borg`]: it takes a CDR-0
//! [`Repository`](crate::repository::Repository) and writes a complete,
//! encrypted **repokey-mode** Borg repository under `dst` that our own
//! [`import_borg`](super::borg) reads back byte-for-byte.
//!
//! # What it writes
//!
//! Every primitive here is the exact inverse of the importer's decrypt
//! path, reusing the same crates (no new dependency):
//!
//! - **Key blob** ([`encode_outer_key`]): PBKDF2-HMAC-SHA256(passphrase,
//!   salt, 100000) → a 32-byte KEK that AES-256-CTR-encrypts (IV 0) the
//!   inner-key MessagePack (`enc_key`, `enc_hmac_key`, …); the passphrase
//!   check value is `HMAC-SHA256(derived, plaintext-inner-key)`. Emitted
//!   base64 into `config`'s `key =` line — the inverse of
//!   `borg::load_borg_key`.
//! - **Chunk sealing** ([`BorgExporter::seal_chunk`]): every chunk (file
//!   content, item-metadata stream, archive, manifest) is prefixed with a
//!   2-byte compression header (`00 00` = CNONE), AES-256-CTR-encrypted
//!   under a monotonic 64-bit nonce, then framed
//!   `0x03 || HMAC-SHA256(iv8||ct)(32) || iv8(8) || ct` — the inverse of
//!   `borg::decrypt_chunk`.
//! - **Segments** (`data/0/<n>`): a `BORG_SEG` log of
//!   `crc(4) size(4 LE) tag(1) key(32) data` PUT entries + a closing
//!   COMMIT, streamed to disk so a large migration never buffers a whole
//!   segment in RAM.
//! - **Manifest** = the chunk PUT under the fixed id `0*32` → MessagePack
//!   `{archives: {name: {id, time}}}`; each archive chunk →
//!   `{items: [chunk-id…], time}`; those item chunks concatenate to a
//!   MessagePack stream of file items (`{path, chunks: [[id, size,
//!   csize]…]}`).
//!
//! Borg chunk IDs aren't portable, so each file is re-chunked (reusing the
//! CDR manifest's own boundaries — each ≤ 4 MiB, pulled on demand) and
//! re-sealed; a chunk's content-address id is `HMAC-SHA256(id_key,
//! plaintext)` so identical content dedups exactly as Borg does.
//!
//! The correctness contract (CDR-0 §11) is a byte-identical round-trip
//! through [`import_borg`](super::borg). A real `borg` binary would
//! additionally require a manifest/archive TAM (tampering-authentication
//! message), which this exporter does not synthesise — reading with a
//! stock `borg` is out of scope.

// Staged ahead of its caller: the `migrate::export` dispatcher that
// invokes `export_borg` is wired separately, so every item here is
// momentarily unreachable from a crate-public root. The allowance turns
// into a no-op the instant that dispatcher lands (mirrors
// `export_restic`).
#![allow(dead_code)]

use std::collections::HashSet;
use std::io::Write as _;
use std::path::{Path, PathBuf};

use super::fill_random;
use aes::cipher::generic_array::GenericArray;
use aes::cipher::{KeyIvInit, StreamCipher};
use base64::Engine as _;
use ctr::Ctr128BE;
use hmac::{Hmac, Mac};
use sha2::Sha256;

use super::{MigrateError, MigrateReport};
use crate::manifest::materialise_range;
use crate::repository::{Repository, Snapshot};
use crate::types::hex_of;

type Aes256Ctr = Ctr128BE<aes::Aes256>;
type HmacSha256 = Hmac<Sha256>;

/// The Borg manifest is always stored under this fixed all-zero id.
const MANIFEST_ID: [u8; 32] = [0u8; 32];

/// PBKDF2 iteration count for the exported key blob (Borg's own default;
/// the value is self-describing in the blob, so the importer reads it
/// back).
const KDF_ITERATIONS: u32 = 100_000;

/// Roll to a fresh segment file once the current one reaches this size
/// (Borg's default `max_segment_size`). Only affects on-disk layout — the
/// importer re-scans + re-orders segments regardless.
const MAX_SEGMENT_SIZE: usize = 500 * 1024 * 1024;

/// Split the per-archive item-metadata stream into chunks no larger than
/// this. Import concatenates the item chunks, so any boundary is correct;
/// a cap just bounds a single sealed blob's size.
const MAX_ITEM_CHUNK: usize = 8 * 1024 * 1024;

// ----------------------------------------------------------------------
// crypto (the ENCRYPT direction of `borg`'s decrypt primitives — same
// crates, so no new dependency)
// ----------------------------------------------------------------------

fn hmac_sha256(key: &[u8], msg: &[u8]) -> [u8; 32] {
    let mut m = <HmacSha256 as Mac>::new_from_slice(key).expect("HMAC accepts any key length");
    m.update(msg);
    m.finalize().into_bytes().into()
}

/// PBKDF2-HMAC-SHA256 for a single 32-byte output block — byte-identical
/// to `borg::pbkdf2_sha256_32` so the derived KEK matches on import.
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

fn io_err(path: &Path, source: std::io::Error) -> MigrateError {
    MigrateError::Io {
        path: path.to_path_buf(),
        source,
    }
}

// ----------------------------------------------------------------------
// minimal MessagePack writer
//
// The inverse of `borg`'s reader: it emits the exact shapes the importer
// looks up — maps keyed by ASCII `str`, arrays, `bin` byte strings (32-
// byte ids as `bin8`), and unsigned ints. Legacy Borg treats `str`/`bin`
// identically, so either is read back as opaque bytes.
// ----------------------------------------------------------------------

fn mp_map_header(out: &mut Vec<u8>, n: usize) {
    if n <= 15 {
        out.push(0x80 | n as u8);
    } else if n <= 0xffff {
        out.push(0xde);
        out.extend_from_slice(&(n as u16).to_be_bytes());
    } else {
        out.push(0xdf);
        out.extend_from_slice(&(n as u32).to_be_bytes());
    }
}

fn mp_array_header(out: &mut Vec<u8>, n: usize) {
    if n <= 15 {
        out.push(0x90 | n as u8);
    } else if n <= 0xffff {
        out.push(0xdc);
        out.extend_from_slice(&(n as u16).to_be_bytes());
    } else {
        out.push(0xdd);
        out.extend_from_slice(&(n as u32).to_be_bytes());
    }
}

/// Encode a UTF-8 string as MessagePack `str` (used for map keys, paths,
/// archive names, and ISO timestamps).
fn mp_str(out: &mut Vec<u8>, s: &str) {
    let b = s.as_bytes();
    let n = b.len();
    if n <= 31 {
        out.push(0xa0 | n as u8);
    } else if n <= 0xff {
        out.push(0xd9);
        out.push(n as u8);
    } else if n <= 0xffff {
        out.push(0xda);
        out.extend_from_slice(&(n as u16).to_be_bytes());
    } else {
        out.push(0xdb);
        out.extend_from_slice(&(n as u32).to_be_bytes());
    }
    out.extend_from_slice(b);
}

/// Encode raw bytes as MessagePack `bin` (used for 32-byte chunk ids, the
/// salt, and the wrapped key ciphertext). A 32-byte id becomes `c4 20 …`,
/// read back by `Mp::id32`.
fn mp_bin(out: &mut Vec<u8>, b: &[u8]) {
    let n = b.len();
    if n <= 0xff {
        out.push(0xc4);
        out.push(n as u8);
    } else if n <= 0xffff {
        out.push(0xc5);
        out.extend_from_slice(&(n as u16).to_be_bytes());
    } else {
        out.push(0xc6);
        out.extend_from_slice(&(n as u32).to_be_bytes());
    }
    out.extend_from_slice(b);
}

fn mp_uint(out: &mut Vec<u8>, v: u64) {
    if v <= 0x7f {
        out.push(v as u8);
    } else if v <= 0xff {
        out.push(0xcc);
        out.push(v as u8);
    } else if v <= 0xffff {
        out.push(0xcd);
        out.extend_from_slice(&(v as u16).to_be_bytes());
    } else if v <= 0xffff_ffff {
        out.push(0xce);
        out.extend_from_slice(&(v as u32).to_be_bytes());
    } else {
        out.push(0xcf);
        out.extend_from_slice(&v.to_be_bytes());
    }
}

fn mp_bool(out: &mut Vec<u8>, b: bool) {
    out.push(if b { 0xc3 } else { 0xc2 });
}

// ----------------------------------------------------------------------
// key blob (inverse of `borg::load_borg_key`)
// ----------------------------------------------------------------------

/// The inner key MessagePack `{enc_key, enc_hmac_key, …}`. Only `enc_key`
/// and `enc_hmac_key` are consumed by the importer; the rest are written
/// for real-Borg faithfulness.
fn encode_inner_key(
    enc_key: &[u8; 32],
    enc_hmac_key: &[u8; 32],
    id_key: &[u8; 32],
    chunk_seed: u64,
    repository_id: &[u8; 32],
) -> Vec<u8> {
    let mut o = Vec::new();
    mp_map_header(&mut o, 7);
    mp_str(&mut o, "version");
    mp_uint(&mut o, 1);
    mp_str(&mut o, "repository_id");
    mp_bin(&mut o, repository_id);
    mp_str(&mut o, "enc_key");
    mp_bin(&mut o, enc_key);
    mp_str(&mut o, "enc_hmac_key");
    mp_bin(&mut o, enc_hmac_key);
    mp_str(&mut o, "id_key");
    mp_bin(&mut o, id_key);
    mp_str(&mut o, "chunk_seed");
    mp_uint(&mut o, chunk_seed);
    mp_str(&mut o, "tam_required");
    mp_bool(&mut o, true);
    o
}

/// The outer key blob `{version, algorithm, iterations, salt, data,
/// hash}` — base64 of this becomes `config`'s `key =` value.
fn encode_outer_key(salt: &[u8; 32], iterations: u32, cdata: &[u8], hash: &[u8; 32]) -> Vec<u8> {
    let mut o = Vec::new();
    mp_map_header(&mut o, 6);
    mp_str(&mut o, "version");
    mp_uint(&mut o, 1);
    mp_str(&mut o, "algorithm");
    mp_str(&mut o, "sha256");
    mp_str(&mut o, "iterations");
    mp_uint(&mut o, u64::from(iterations));
    mp_str(&mut o, "salt");
    mp_bin(&mut o, salt);
    mp_str(&mut o, "data");
    mp_bin(&mut o, cdata);
    mp_str(&mut o, "hash");
    mp_bin(&mut o, hash);
    o
}

// ----------------------------------------------------------------------
// archive / manifest MessagePack (inverse of `borg`'s import parse)
// ----------------------------------------------------------------------

/// One file item: `{path, mode, chunks: [[id, size, csize]…]}`. The
/// importer keys files off the presence of `chunks`; `mode` is ignored.
fn encode_item(out: &mut Vec<u8>, path: &str, chunks: &[([u8; 32], u64, u64)]) {
    mp_map_header(out, 3);
    mp_str(out, "path");
    mp_str(out, path);
    mp_str(out, "mode");
    mp_uint(out, 0o100644); // regular file, rw-r--r-- (faithful; import-ignored)
    mp_str(out, "chunks");
    mp_array_header(out, chunks.len());
    for (id, size, csize) in chunks {
        mp_array_header(out, 3);
        mp_bin(out, id);
        mp_uint(out, *size);
        mp_uint(out, *csize);
    }
}

/// Archive metadata `{version, name, items: [id…], time, time_end}`. The
/// importer reads `items` (bare 32-byte ids) + `time`.
fn encode_archive(name: &str, item_ids: &[[u8; 32]], time: &str) -> Vec<u8> {
    let mut o = Vec::new();
    mp_map_header(&mut o, 5);
    mp_str(&mut o, "version");
    mp_uint(&mut o, 1);
    mp_str(&mut o, "name");
    mp_str(&mut o, name);
    mp_str(&mut o, "items");
    mp_array_header(&mut o, item_ids.len());
    for id in item_ids {
        mp_bin(&mut o, id);
    }
    mp_str(&mut o, "time");
    mp_str(&mut o, time);
    mp_str(&mut o, "time_end");
    mp_str(&mut o, time);
    o
}

/// Repository manifest `{version, archives: {name: {id, time}},
/// timestamp}`. The importer reads only `archives`.
fn encode_manifest(archives: &[(String, [u8; 32], String)], timestamp: &str) -> Vec<u8> {
    let mut o = Vec::new();
    mp_map_header(&mut o, 3);
    mp_str(&mut o, "version");
    mp_uint(&mut o, 1);
    mp_str(&mut o, "archives");
    mp_map_header(&mut o, archives.len());
    for (name, id, time) in archives {
        mp_str(&mut o, name);
        mp_map_header(&mut o, 2);
        mp_str(&mut o, "id");
        mp_bin(&mut o, id);
        mp_str(&mut o, "time");
        mp_str(&mut o, time);
    }
    mp_str(&mut o, "timestamp");
    mp_str(&mut o, timestamp);
    o
}

/// Format `ms` since the Unix epoch as the Borg archive-time string
/// (`%Y-%m-%dT%H:%M:%S%.6f`, UTC, no offset) that `borg::parse_borg_time`
/// parses back to the same millisecond.
fn ms_to_borg_time(ms: i64) -> String {
    chrono::DateTime::<chrono::Utc>::from_timestamp_millis(ms)
        .unwrap_or_else(|| {
            chrono::DateTime::<chrono::Utc>::from_timestamp_millis(0)
                .expect("epoch is representable")
        })
        .format("%Y-%m-%dT%H:%M:%S%.6f")
        .to_string()
}

/// Derive a unique, filesystem-safe archive name from a snapshot. The
/// monotonic snapshot id suffix guarantees no two archives collide as
/// `manifest.archives` map keys (Borg would silently drop a duplicate).
fn unique_archive_name(snap: &Snapshot) -> String {
    let mut base: String = snap
        .label
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.') {
                c
            } else {
                '_'
            }
        })
        .collect();
    base.truncate(200);
    if base.is_empty() {
        base = "archive".to_string();
    }
    format!("{base}-{}", snap.id)
}

// ----------------------------------------------------------------------
// segment log writer (streamed — never buffers a whole segment in RAM)
// ----------------------------------------------------------------------

/// Streams `BORG_SEG` PUT/COMMIT entries into `data/<n/1000>/<n>` files,
/// rolling to a new segment at [`MAX_SEGMENT_SIZE`].
struct SegmentWriter {
    data_dir: PathBuf,
    idx: usize,
    cur_path: PathBuf,
    file: Option<std::io::BufWriter<std::fs::File>>,
    bytes: usize,
    max: usize,
}

impl SegmentWriter {
    fn new(data_dir: PathBuf, max: usize) -> Self {
        Self {
            data_dir,
            idx: 0,
            cur_path: PathBuf::new(),
            file: None,
            bytes: 0,
            max,
        }
    }

    /// Open the current segment file (creating `data/<n/1000>/`) and write
    /// its `BORG_SEG` magic, unless one is already open.
    fn ensure_open(&mut self) -> Result<(), MigrateError> {
        if self.file.is_some() {
            return Ok(());
        }
        let sub = self.data_dir.join((self.idx / 1000).to_string());
        std::fs::create_dir_all(&sub).map_err(|e| io_err(&sub, e))?;
        let path = sub.join(self.idx.to_string());
        let f = std::fs::File::create(&path).map_err(|e| io_err(&path, e))?;
        let mut w = std::io::BufWriter::new(f);
        w.write_all(b"BORG_SEG").map_err(|e| io_err(&path, e))?;
        self.file = Some(w);
        self.cur_path = path;
        self.bytes = 8;
        Ok(())
    }

    /// Flush + close the current segment and advance to the next index.
    fn roll(&mut self) -> Result<(), MigrateError> {
        if let Some(mut w) = self.file.take() {
            w.flush().map_err(|e| io_err(&self.cur_path, e))?;
        }
        self.idx += 1;
        self.bytes = 0;
        Ok(())
    }

    /// Append a PUT entry: `crc(4)=0 || size(4 LE) || tag(1)=0 || key(32)
    /// || sealed`, where `size` is the total on-disk entry length.
    fn put(&mut self, key: &[u8; 32], sealed: &[u8]) -> Result<(), MigrateError> {
        self.ensure_open()?;
        // Total entry length = crc(4) + size(4) + tag(1) + key(32) + data.
        let size = 41 + sealed.len();
        if self.bytes > 8 && self.bytes + size > self.max {
            self.roll()?;
            self.ensure_open()?;
        }
        let path = self.cur_path.clone();
        let w = self.file.as_mut().expect("segment open");
        let mut hdr = [0u8; 9]; // crc(4)=0, size(4 LE), tag(1)=PUT(0)
        hdr[4..8].copy_from_slice(&(size as u32).to_le_bytes());
        w.write_all(&hdr).map_err(|e| io_err(&path, e))?;
        w.write_all(key).map_err(|e| io_err(&path, e))?;
        w.write_all(sealed).map_err(|e| io_err(&path, e))?;
        self.bytes += size;
        Ok(())
    }

    /// Append a COMMIT entry (`size=9, tag=2`) and flush + close the log.
    fn commit_and_close(&mut self) -> Result<(), MigrateError> {
        self.ensure_open()?;
        let path = self.cur_path.clone();
        let w = self.file.as_mut().expect("segment open");
        let mut entry = [0u8; 9]; // crc(4)=0, size=9 LE, tag=COMMIT(2)
        entry[4..8].copy_from_slice(&9u32.to_le_bytes());
        entry[8] = 0x02;
        w.write_all(&entry).map_err(|e| io_err(&path, e))?;
        if let Some(mut w) = self.file.take() {
            w.flush().map_err(|e| io_err(&path, e))?;
        }
        Ok(())
    }
}

// ----------------------------------------------------------------------
// exporter
// ----------------------------------------------------------------------

/// Holds the master secrets, dedup set, and monotonic nonce while sealing
/// chunks into the segment log.
struct BorgExporter {
    enc_key: [u8; 32],
    enc_hmac_key: [u8; 32],
    id_key: [u8; 32],
    /// One monotonic counter for the whole export: each sealed chunk
    /// reserves `ceil(len/16)` blocks so AES-CTR keystreams never overlap
    /// under `enc_key` (catastrophic if they did).
    next_nonce: u64,
    /// Chunk ids already PUT — content dedup (identical plaintext → same
    /// `HMAC(id_key, …)` id → sealed + stored once).
    seen: HashSet<[u8; 32]>,
    seg: SegmentWriter,
}

impl BorgExporter {
    /// Seal one chunk: `0x03 || HMAC(enc_hmac_key, iv8||ct)(32) || iv8(8)
    /// || ct`, where `ct = AES-256-CTR(enc_key, 0^8||iv8, 00||00||raw)`.
    /// The exact byte sequence `borg::decrypt_chunk` expects.
    fn seal_chunk(&mut self, raw: &[u8]) -> Vec<u8> {
        // CNONE compression header (method 0x00, level byte ignored) — the
        // importer always runs `decompress_borg`, which strips these two
        // bytes.
        let mut plain = Vec::with_capacity(2 + raw.len());
        plain.push(0x00);
        plain.push(0x00);
        plain.extend_from_slice(raw);

        let blocks = plain.len().div_ceil(16) as u64;
        let iv_val = self.next_nonce;
        self.next_nonce = self.next_nonce.wrapping_add(blocks);

        let iv8 = iv_val.to_be_bytes();
        // The 8-byte nonce is the LOW 64 bits of a 128-bit big-endian CTR.
        let mut iv16 = [0u8; 16];
        iv16[8..].copy_from_slice(&iv8);
        let ct = aes_ctr(&self.enc_key, &iv16, &plain);

        // HMAC covers iv8 || ct and EXCLUDES the 0x03 type byte.
        let mut mac_msg = Vec::with_capacity(8 + ct.len());
        mac_msg.extend_from_slice(&iv8);
        mac_msg.extend_from_slice(&ct);
        let mac = hmac_sha256(&self.enc_hmac_key, &mac_msg);

        let mut sealed = Vec::with_capacity(1 + 32 + 8 + ct.len());
        sealed.push(0x03);
        sealed.extend_from_slice(&mac);
        sealed.extend_from_slice(&iv8);
        sealed.extend_from_slice(&ct);
        sealed
    }

    /// Content-address, seal, and PUT `raw` (unless already stored),
    /// returning its id and stored (sealed) size. `manifest` forces the
    /// fixed [`MANIFEST_ID`] key.
    fn put(&mut self, raw: &[u8], manifest: bool) -> Result<([u8; 32], u64), MigrateError> {
        let id = if manifest {
            MANIFEST_ID
        } else {
            hmac_sha256(&self.id_key, raw)
        };
        // CNONE sealed size is a pure function of the plaintext length:
        // 0x03(1) + mac(32) + iv8(8) + [00 00 || raw](2 + raw.len()).
        let csize = (43 + raw.len()) as u64;
        if self.seen.insert(id) {
            let sealed = self.seal_chunk(raw);
            self.seg.put(&id, &sealed)?;
        }
        Ok((id, csize))
    }
}

/// Export every snapshot of the CDR-0 repository `src` into a fresh,
/// encrypted **Borg** (repokey) repository under `dst`, unlockable with
/// `password`. The result round-trips through
/// [`import_borg`](super::borg) to byte-identical files.
pub(super) fn export_borg(
    src: &Repository,
    dst: &Path,
    password: &str,
) -> Result<MigrateReport, MigrateError> {
    std::fs::create_dir_all(dst).map_err(|e| io_err(dst, e))?;
    let data_dir = dst.join("data");
    std::fs::create_dir_all(&data_dir).map_err(|e| io_err(&data_dir, e))?;

    // Master secrets — generated once, reused for the whole run.
    let mut enc_key = [0u8; 32];
    let mut enc_hmac_key = [0u8; 32];
    let mut id_key = [0u8; 32];
    let mut repository_id = [0u8; 32];
    fill_random(&mut enc_key);
    fill_random(&mut enc_hmac_key);
    fill_random(&mut id_key);
    fill_random(&mut repository_id);
    let chunk_seed: u64 = 0;

    // Wrap the inner key with a scrypt-free PBKDF2 KEK (Borg repokey).
    let inner = encode_inner_key(&enc_key, &enc_hmac_key, &id_key, chunk_seed, &repository_id);
    let mut salt = [0u8; 32];
    fill_random(&mut salt);
    let derived = pbkdf2_sha256_32(password.as_bytes(), &salt, KDF_ITERATIONS);
    let hash = hmac_sha256(&derived, &inner); // over the PLAINTEXT inner key
    let cdata = aes_ctr(&derived, &[0u8; 16], &inner); // all-zero IV
    let outer = encode_outer_key(&salt, KDF_ITERATIONS, &cdata, &hash);
    let key_b64 = base64::engine::general_purpose::STANDARD.encode(&outer);

    // README + config. `detect()` needs README + config + data/ and no
    // snapshots/ dir, so we write the raw Borg files by hand (never open a
    // Repository on the destination).
    let readme_path = dst.join("README");
    std::fs::write(
        &readme_path,
        "This is a Borg Backup repository.\nSee https://borgbackup.readthedocs.io/\n",
    )
    .map_err(|e| io_err(&readme_path, e))?;
    let config = format!(
        "[repository]\nversion = 1\nsegments_per_dir = 1000\nmax_segment_size = {}\nappend_only = 0\nstorage_quota = 0\nadditional_free_space = 0\nid = {}\nkey = {}\n",
        MAX_SEGMENT_SIZE,
        hex_of(&repository_id),
        key_b64,
    );
    let config_path = dst.join("config");
    std::fs::write(&config_path, config).map_err(|e| io_err(&config_path, e))?;

    let mut ex = BorgExporter {
        enc_key,
        enc_hmac_key,
        id_key,
        next_nonce: 0,
        seen: HashSet::new(),
        seg: SegmentWriter::new(data_dir, MAX_SEGMENT_SIZE),
    };
    let mut report = MigrateReport::default();
    let mut archives: Vec<(String, [u8; 32], String)> = Vec::new();
    let mut latest_ms = 0i64;

    for snap in src.load_all_snapshots()? {
        latest_ms = latest_ms.max(snap.created_at_ms);

        // Build the item-metadata stream, sealing each file's chunks.
        let mut item_stream: Vec<u8> = Vec::new();
        for fe in &snap.files {
            let mut chunks: Vec<([u8; 32], u64, u64)> =
                Vec::with_capacity(fe.manifest.chunks.len());
            for cr in &fe.manifest.chunks {
                // Pull only this chunk's plaintext (≤ 4 MiB) — never the
                // whole file — matching the importer's per-chunk streaming.
                let raw = materialise_range(src.store(), &fe.manifest, cr.offset, cr.len as usize)?;
                let (id, csize) = ex.put(&raw, false)?;
                chunks.push((id, u64::from(cr.len), csize));
            }
            encode_item(&mut item_stream, &fe.path, &chunks);
            report.files += 1;
        }

        // Seal the item stream in bounded pieces; their ids form the
        // archive's item list.
        let mut item_ids: Vec<[u8; 32]> = Vec::new();
        for piece in item_stream.chunks(MAX_ITEM_CHUNK) {
            let (id, _) = ex.put(piece, false)?;
            item_ids.push(id);
        }

        let name = unique_archive_name(&snap);
        let time = ms_to_borg_time(snap.created_at_ms);
        let archive_bytes = encode_archive(&name, &item_ids, &time);
        let (archive_id, _) = ex.put(&archive_bytes, false)?;
        archives.push((name, archive_id, time));
        report.snapshots += 1;
    }

    // Seal the manifest under the fixed MANIFEST_ID and close the log.
    let manifest_bytes = encode_manifest(&archives, &ms_to_borg_time(latest_ms));
    ex.put(&manifest_bytes, true)?;
    ex.seg.commit_and_close()?;

    // Faithful (import-ignored) next-free-nonce marker.
    let nonce_path = dst.join("nonce");
    std::fs::write(&nonce_path, format!("{:016X}", ex.next_nonce))
        .map_err(|e| io_err(&nonce_path, e))?;

    Ok(report)
}
