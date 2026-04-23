//! Content-defined chunking.
//!
//! `Chunker` wraps `fastcdc::v2020::FastCDC`, reads a source file in
//! streaming fashion, and yields `Chunk`s with their BLAKE3 digests.
//! The raw chunk bytes are *not* materialised on the iterator — only
//! offsets, lengths, and hashes — because the primary caller (the
//! engine's copy loop) already has the bytes in a buffer it's about
//! to write to the destination.
//!
//! # Buffer strategy
//!
//! FastCDC needs a contiguous window to find a cut point. We read
//! into a rolling `Vec<u8>` of `max_size` bytes + a drain region;
//! every time the cutter finds a cut, we hash the cut's bytes, yield
//! a `Chunk`, and slide the window forward. The final tail of the
//! file always emits as its own chunk (FastCDC returns the remaining
//! bytes with an "end" marker).
//!
//! # Hashing
//!
//! Each chunk is hashed with BLAKE3 in a single pass over the cut's
//! bytes. For the spec's defaults (1 MiB avg, 4 MiB max) that's
//! ~one BLAKE3 finalise per MiB read — negligible overhead compared
//! to the disk I/O cost of the read itself.

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use fastcdc::v2020::FastCDC;

use crate::error::{ChunkStoreError, Result};
use crate::types::{Blake3Hash, Chunk};

/// The spec-mandated FastCDC defaults. Match restic's tuning: 512 KiB
/// minimum chunk, 1 MiB average, 4 MiB maximum.
pub const DEFAULT_MIN: u32 = 512 * 1024;
pub const DEFAULT_AVG: u32 = 1024 * 1024;
pub const DEFAULT_MAX: u32 = 4 * 1024 * 1024;

/// A content-defined chunker.
///
/// Cheap to construct, cheap to clone. All state lives in the
/// FastCDC instance created per `chunk_file` call so the same
/// `Chunker` can serve many files concurrently.
#[derive(Debug, Clone, Copy)]
pub struct Chunker {
    min: u32,
    avg: u32,
    max: u32,
}

impl Default for Chunker {
    /// `Chunker::new(DEFAULT_MIN, DEFAULT_AVG, DEFAULT_MAX)`.
    fn default() -> Self {
        Self::new(DEFAULT_MIN, DEFAULT_AVG, DEFAULT_MAX)
    }
}

impl Chunker {
    /// Construct a chunker with explicit minimum / average / maximum
    /// chunk sizes in bytes.
    ///
    /// FastCDC documents min ≤ avg ≤ max. We don't enforce the
    /// ordering here — FastCDC panics on invalid input and any code
    /// path hitting that is a programmer bug, not a user input error.
    #[must_use]
    pub const fn new(min: u32, avg: u32, max: u32) -> Self {
        Self { min, avg, max }
    }

    /// Getter for testing + introspection.
    #[must_use]
    pub const fn min(&self) -> u32 {
        self.min
    }
    /// Getter for testing + introspection.
    #[must_use]
    pub const fn avg(&self) -> u32 {
        self.avg
    }
    /// Getter for testing + introspection.
    #[must_use]
    pub const fn max(&self) -> u32 {
        self.max
    }

    /// Chunk `file` in-memory.
    ///
    /// This is the most convenient API when the caller is happy to
    /// buffer the whole file (smoke tests, small-file paths). For
    /// the engine's streaming copy loop, prefer `chunk_reader` which
    /// accepts any `Read`.
    pub fn chunk_file(&self, file: &Path) -> Result<Vec<Chunk>> {
        let mut buf = Vec::new();
        let mut f = BufReader::new(File::open(file).map_err(|e| ChunkStoreError::Io {
            path: file.to_path_buf(),
            source: e,
        })?);
        f.read_to_end(&mut buf).map_err(|e| ChunkStoreError::Io {
            path: file.to_path_buf(),
            source: e,
        })?;
        Ok(self.chunk_bytes(&buf))
    }

    /// Chunk an in-memory byte slice. Useful for tests and small
    /// files; the engine uses `chunk_reader` for multi-gigabyte
    /// streams.
    #[must_use]
    pub fn chunk_bytes(&self, bytes: &[u8]) -> Vec<Chunk> {
        let cdc = FastCDC::new(bytes, self.min, self.avg, self.max);
        let mut out = Vec::new();
        for c in cdc {
            let end = c.offset + c.length;
            let slice = &bytes[c.offset..end];
            let hash = blake3::hash(slice);
            out.push(Chunk {
                offset: c.offset as u64,
                len: c.length as u32,
                hash: *hash.as_bytes(),
            });
        }
        out
    }

    /// Stream-chunk a `Read`. Accumulates bytes into a rolling buffer
    /// and yields chunks as FastCDC cuts them. The callback receives
    /// each `(Chunk, &[u8])` in order so the engine can write the
    /// bytes to both the destination and the chunk store without a
    /// second read of the source.
    ///
    /// Returns the full-file BLAKE3 digest so the caller doesn't need
    /// a third pass over the bytes to build the manifest.
    pub fn chunk_reader<R: Read, F: FnMut(&Chunk, &[u8]) -> Result<()>>(
        &self,
        mut reader: R,
        mut on_chunk: F,
    ) -> Result<Blake3Hash> {
        // Rolling buffer sized at 2× max so the cutter always has
        // enough lookahead to find its next cut and we never
        // reallocate mid-hash.
        let buffer_cap = (self.max as usize) * 2;
        let mut buf: Vec<u8> = Vec::with_capacity(buffer_cap);
        let mut hasher = blake3::Hasher::new();

        // Read loop: top up the buffer, cut off as many chunks as
        // FastCDC will give us with the current buffer, slide the
        // remainder to the front, repeat.
        let mut eof = false;
        while !eof || !buf.is_empty() {
            if !eof && buf.len() < buffer_cap {
                let want = buffer_cap - buf.len();
                let start = buf.len();
                buf.resize(buf.len() + want, 0);
                let read = reader
                    .read(&mut buf[start..])
                    .map_err(|e| ChunkStoreError::Io {
                        path: std::path::PathBuf::new(),
                        source: e,
                    })?;
                buf.truncate(start + read);
                if read == 0 {
                    eof = true;
                }
            }

            // When the buffer is too small for a max-size chunk and
            // we haven't hit EOF yet, loop back and read more. This
            // avoids yielding a short chunk just because we ran low
            // on data.
            if !eof && buf.len() < self.max as usize {
                continue;
            }

            // At EOF, FastCDC cuts everything remaining. Before EOF,
            // we must leave a `max`-byte tail so FastCDC can still
            // find a cut within it on the next iteration.
            let window: &[u8] = &buf;
            let cutter = FastCDC::new(window, self.min, self.avg, self.max);
            let mut consumed = 0usize;
            for c in cutter {
                // Before EOF, only emit chunks that sit fully within
                // the left half of the window — the tail might extend
                // across the next read and change FastCDC's decision.
                if !eof && c.offset + c.length > self.max as usize {
                    break;
                }
                let slice = &window[c.offset..c.offset + c.length];
                let hash = blake3::hash(slice);
                hasher.update(slice);
                let chunk = Chunk {
                    // Absolute offset fixed up below by the caller's
                    // accumulator — here the offsets are relative to
                    // the current window.
                    offset: c.offset as u64,
                    len: c.length as u32,
                    hash: *hash.as_bytes(),
                };
                on_chunk(&chunk, slice)?;
                consumed = c.offset + c.length;
            }

            if consumed == 0 {
                // Nothing cut this iteration; either we need more
                // data (handled by the top-of-loop top-up) or we're
                // at EOF with a short residual. At EOF a short
                // residual means no cut was made above — force one.
                if eof && !buf.is_empty() {
                    let slice = &buf;
                    let hash = blake3::hash(slice);
                    hasher.update(slice);
                    let chunk = Chunk {
                        offset: 0,
                        len: slice.len() as u32,
                        hash: *hash.as_bytes(),
                    };
                    on_chunk(&chunk, slice)?;
                    buf.clear();
                }
                if !eof {
                    continue;
                } else {
                    break;
                }
            }
            buf.drain(..consumed);
        }

        Ok(*hasher.finalize().as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunker_new_stores_sizes() {
        let c = Chunker::new(1024, 2048, 4096);
        assert_eq!(c.min(), 1024);
        assert_eq!(c.avg(), 2048);
        assert_eq!(c.max(), 4096);
    }

    #[test]
    fn chunker_default_matches_spec() {
        let c = Chunker::default();
        assert_eq!(c.min(), 512 * 1024);
        assert_eq!(c.avg(), 1024 * 1024);
        assert_eq!(c.max(), 4 * 1024 * 1024);
    }

    #[test]
    fn chunk_bytes_covers_all_input() {
        // Deterministic pseudo-random input. Use a simple LCG so the
        // test doesn't depend on a PRNG crate.
        let mut bytes = vec![0u8; 3 * 1024 * 1024];
        let mut s: u64 = 0xdeadbeef;
        for b in &mut bytes {
            s = s
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            *b = (s >> 33) as u8;
        }
        let chunker = Chunker::default();
        let chunks = chunker.chunk_bytes(&bytes);
        assert!(!chunks.is_empty(), "at least one chunk");
        // Full coverage: chunk offsets/lengths tile [0, size) exactly.
        let mut cursor = 0u64;
        for c in &chunks {
            assert_eq!(c.offset, cursor, "chunks must tile");
            cursor += u64::from(c.len);
        }
        assert_eq!(cursor as usize, bytes.len(), "total coverage");
    }

    #[test]
    fn chunk_bytes_is_deterministic() {
        let mut bytes = vec![0u8; 2 * 1024 * 1024];
        let mut s: u64 = 12345;
        for b in &mut bytes {
            s = s
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            *b = (s >> 33) as u8;
        }
        let chunker = Chunker::default();
        let a = chunker.chunk_bytes(&bytes);
        let b = chunker.chunk_bytes(&bytes);
        assert_eq!(a, b, "same input → same cuts + hashes");
    }

    #[test]
    fn single_byte_change_affects_one_chunk_region() {
        // Use a 4 MiB buffer so FastCDC produces multiple chunks.
        let mut bytes = vec![0u8; 4 * 1024 * 1024];
        let mut s: u64 = 98765;
        for b in &mut bytes {
            s = s
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            *b = (s >> 33) as u8;
        }
        let chunker = Chunker::default();
        let base = chunker.chunk_bytes(&bytes);
        let mut mutated = bytes.clone();
        // Flip one byte near the middle.
        mutated[2 * 1024 * 1024] ^= 0xff;
        let after = chunker.chunk_bytes(&mutated);

        // Most hashes should be unchanged. FastCDC's small-shift
        // resilience means a one-byte mutation touches at most 2–3
        // chunks in practice.
        let base_hashes: std::collections::HashSet<_> = base.iter().map(|c| c.hash).collect();
        let shared = after
            .iter()
            .filter(|c| base_hashes.contains(&c.hash))
            .count();
        let ratio = shared as f64 / after.len() as f64;
        assert!(
            ratio >= 0.50,
            "expected ≥50% shared chunks after 1-byte mutation, got {ratio:.2} ({shared}/{})",
            after.len()
        );
    }
}
