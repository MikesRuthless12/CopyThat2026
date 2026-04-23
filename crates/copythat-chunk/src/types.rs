//! Public plain-data types shared across the crate.
//!
//! These are deliberately small, `Copy`-where-possible structs with
//! no behaviour — behaviour lives on `Chunker` / `ChunkStore`. Keeping
//! the types separate lets the engine pass a `Vec<ChunkRef>` across
//! the trait boundary without pulling in redb or fastcdc.

use serde::{Deserialize, Serialize};

/// Raw 32-byte BLAKE3 digest. Used as the primary key for every
/// chunk in the store.
pub type Blake3Hash = [u8; 32];

/// A single content-defined chunk pulled out of a source file.
///
/// `Chunker::chunk_file` yields these. `offset` + `len` describe
/// where this chunk lives in the source; `hash` is its BLAKE3 digest.
/// The raw bytes are not carried on the struct — callers read them
/// back from the file when they want to write to the destination or
/// persist to the store.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Chunk {
    /// Byte offset in the source file where this chunk starts.
    pub offset: u64,
    /// Length of the chunk in bytes. FastCDC's contract says this is
    /// bounded above by `max` from `Chunker::new`.
    pub len: u32,
    /// BLAKE3 of the chunk's bytes.
    pub hash: Blake3Hash,
}

/// A chunk reference inside a file manifest.
///
/// `Manifest::chunks` is ordered: walking `chunks` in order and
/// emitting `ChunkRef` bytes reconstructs the original file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChunkRef {
    /// BLAKE3 of the referenced chunk. Look up in the chunk store.
    #[serde(with = "serde_hash")]
    pub hash: Blake3Hash,
    /// Byte offset in the reconstructed file where this chunk starts.
    pub offset: u64,
    /// Byte length of the chunk.
    pub len: u32,
}

/// A file-level manifest. One manifest per distinct source file that
/// passed through the chunk store.
///
/// The manifest lets the engine answer the delta-resume question: "I
/// have an old manifest for this (src, dst) pair — which chunks have
/// changed?" Only the differing chunks need re-reading and
/// re-writing on retry, which is the entire point of Phase 27.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Manifest {
    /// BLAKE3 of the full source file's bytes. Authoritative — can be
    /// re-computed from the chunks but is stored to avoid a second
    /// pass on read-back.
    #[serde(with = "serde_hash")]
    pub file_hash: Blake3Hash,
    /// Total size in bytes. Equals `chunks.iter().map(|c| c.len as
    /// u64).sum()`.
    pub size: u64,
    /// Ordered chunks. Concatenating their bytes reconstructs the
    /// file byte-for-byte.
    pub chunks: Vec<ChunkRef>,
}

impl Manifest {
    /// Count of distinct chunks by hash. A file that fully dedups
    /// against itself (rare but possible with tiny files) will have
    /// `unique_chunks < chunks.len()`.
    #[must_use]
    pub fn unique_chunks(&self) -> usize {
        let mut seen = std::collections::HashSet::new();
        for c in &self.chunks {
            seen.insert(c.hash);
        }
        seen.len()
    }

    /// The set of chunk hashes in `new` that are *not* already
    /// present in `self`. Used by the delta-resume fast path: when a
    /// retry sees an old manifest, only the returned hashes need
    /// re-reading from the source.
    #[must_use]
    pub fn new_chunks_relative_to(&self, new: &Manifest) -> Vec<Blake3Hash> {
        let old: std::collections::HashSet<Blake3Hash> =
            self.chunks.iter().map(|c| c.hash).collect();
        let mut out = Vec::new();
        let mut added = std::collections::HashSet::new();
        for c in &new.chunks {
            if !old.contains(&c.hash) && added.insert(c.hash) {
                out.push(c.hash);
            }
        }
        out
    }
}

/// Serde helper — BLAKE3 digests live as lowercase hex strings in
/// the on-disk JSON (for eyeball debuggability) but as raw `[u8; 32]`
/// in memory. Rolling our own visitor keeps the serde dep weightless.
mod serde_hash {
    use serde::{Deserialize, Deserializer, Serializer};

    use super::Blake3Hash;

    pub fn serialize<S: Serializer>(h: &Blake3Hash, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&hex_encode(h))
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Blake3Hash, D::Error> {
        let s = String::deserialize(d)?;
        hex_decode(&s).map_err(serde::de::Error::custom)
    }

    fn hex_encode(bytes: &[u8; 32]) -> String {
        let mut s = String::with_capacity(64);
        for b in bytes {
            s.push(HEX[((*b >> 4) & 0xf) as usize]);
            s.push(HEX[(*b & 0xf) as usize]);
        }
        s
    }

    fn hex_decode(s: &str) -> Result<[u8; 32], String> {
        if s.len() != 64 {
            return Err(format!("expected 64 hex chars, got {}", s.len()));
        }
        let bytes = s.as_bytes();
        let mut out = [0u8; 32];
        for i in 0..32 {
            let hi = from_hex(bytes[2 * i])?;
            let lo = from_hex(bytes[2 * i + 1])?;
            out[i] = (hi << 4) | lo;
        }
        Ok(out)
    }

    fn from_hex(c: u8) -> Result<u8, String> {
        match c {
            b'0'..=b'9' => Ok(c - b'0'),
            b'a'..=b'f' => Ok(c - b'a' + 10),
            b'A'..=b'F' => Ok(c - b'A' + 10),
            _ => Err(format!("invalid hex char: {c:?}")),
        }
    }

    const HEX: &[char; 16] = &[
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
    ];
}

/// Render a BLAKE3 digest as lowercase hex.
#[must_use]
pub fn hex_of(h: &Blake3Hash) -> String {
    let mut s = String::with_capacity(64);
    for b in h {
        s.push_str(&format!("{b:02x}"));
    }
    s
}
