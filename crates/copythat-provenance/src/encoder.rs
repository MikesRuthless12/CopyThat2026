//! [`OutboardEncoder`] implementations — the per-file streaming
//! consumers the engine feeds source bytes into.
//!
//! Two flavours ship today:
//!
//! - [`BaoOutboardEncoder`] — wraps `bao::encode::Encoder` in
//!   outboard mode; writes the verified-streaming Merkle tree to an
//!   in-memory `Vec<u8>` and finalises to the BLAKE3 root.
//! - [`RootOnlyEncoder`] — drives a plain `blake3::Hasher` and
//!   returns an empty outboard. Useful when the caller wants only
//!   integrity (full re-hash on verify) and not partial-byte-range
//!   verification.

use std::io::{Cursor, Write};

use copythat_core::OutboardEncoder;

/// Verified-streaming outboard encoder. Wraps `bao::encode::Encoder`
/// in outboard mode (data flows in via `Write`; the Merkle tree is
/// written to a separate `Cursor<Vec<u8>>` buffer the encoder
/// borrows for its full lifetime) so the per-file outboard ends up
/// in the manifest's `bao_outboard` field.
///
/// `Cursor<Vec<u8>>` is required because Bao's outboard mode writes
/// the tree non-sequentially (the file-length header lands at the
/// front after the rest of the tree is built); a plain `Vec<u8>`
/// doesn't implement `Seek`, so the encoder won't accept it.
///
/// Buffer growth is in-memory: a 100 GiB file produces a ~1.5 GiB
/// outboard. Future work: stream outboards to per-file sidecars
/// rather than buffering. Tracked as a follow-up; the manifest
/// schema already allows an empty `bao_outboard` (verify falls back
/// to a full re-hash via [`RootOnlyEncoder`]).
pub struct BaoOutboardEncoder {
    inner: bao::encode::Encoder<Cursor<Vec<u8>>>,
}

impl BaoOutboardEncoder {
    /// Construct a fresh outboard encoder with an empty in-memory
    /// outboard buffer.
    pub fn new() -> Self {
        Self {
            inner: bao::encode::Encoder::new_outboard(Cursor::new(Vec::new())),
        }
    }
}

impl Default for BaoOutboardEncoder {
    fn default() -> Self {
        Self::new()
    }
}

impl OutboardEncoder for BaoOutboardEncoder {
    fn update(&mut self, bytes: &[u8]) {
        // `Encoder::write_all` only fails when the inner sink fails;
        // our sink is a `Cursor<Vec<u8>>` whose `Write` impl is
        // infallible outside of OOM, which would already crash the
        // process. Swallowing the result here keeps the engine's hot
        // path fallible-error-free — provenance is a best-effort
        // feature and the caller's verify pass also detects mismatch.
        let _ = self.inner.write_all(bytes);
    }

    fn finalize(mut self: Box<Self>) -> ([u8; 32], Vec<u8>) {
        let hash = self
            .inner
            .finalize()
            .expect("Cursor<Vec<u8>> writer is infallible");
        let outboard = self.inner.into_inner().into_inner();
        (*hash.as_bytes(), outboard)
    }
}

/// Streaming hasher that produces just the BLAKE3 root with no Bao
/// tree. Use when you don't need partial-byte-range verification —
/// the full file is re-hashed on verify in this mode.
///
/// Cheaper than [`BaoOutboardEncoder`] (no tree allocation, no
/// per-chunk hashing overhead) and produces a manifest whose
/// `bao_outboard` is `Vec::new()` for every file.
pub struct RootOnlyEncoder {
    hasher: blake3::Hasher,
}

impl RootOnlyEncoder {
    /// Fresh hasher.
    pub fn new() -> Self {
        Self {
            hasher: blake3::Hasher::new(),
        }
    }
}

impl Default for RootOnlyEncoder {
    fn default() -> Self {
        Self::new()
    }
}

impl OutboardEncoder for RootOnlyEncoder {
    fn update(&mut self, bytes: &[u8]) {
        self.hasher.update(bytes);
    }

    fn finalize(self: Box<Self>) -> ([u8; 32], Vec<u8>) {
        (*self.hasher.finalize().as_bytes(), Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_only_encoder_matches_blake3_direct() {
        let bytes = b"the quick brown fox jumps over the lazy dog";
        let mut enc: Box<dyn OutboardEncoder> = Box::new(RootOnlyEncoder::new());
        enc.update(bytes);
        let (root, outboard) = enc.finalize();
        assert!(
            outboard.is_empty(),
            "root-only mode should produce no outboard"
        );
        let direct = blake3::hash(bytes);
        assert_eq!(root, *direct.as_bytes());
    }

    #[test]
    fn bao_encoder_root_matches_blake3_for_single_chunk() {
        // BLAKE3 single-chunk input (< 1024 bytes) hashes to the same
        // value whether or not Bao tree construction runs.
        let bytes = b"hello provenance";
        let mut enc: Box<dyn OutboardEncoder> = Box::new(BaoOutboardEncoder::new());
        enc.update(bytes);
        let (root, _outboard) = enc.finalize();
        let direct = blake3::hash(bytes);
        assert_eq!(root, *direct.as_bytes());
    }

    #[test]
    fn bao_encoder_produces_nontrivial_outboard_for_multi_chunk() {
        // > 1024 bytes triggers tree construction; outboard should
        // be non-empty.
        let bytes = vec![0xAB; 4096];
        let mut enc: Box<dyn OutboardEncoder> = Box::new(BaoOutboardEncoder::new());
        enc.update(&bytes);
        let (_root, outboard) = enc.finalize();
        assert!(
            !outboard.is_empty(),
            "multi-chunk input should yield a non-empty outboard"
        );
    }

    #[test]
    fn streaming_update_matches_one_shot() {
        let bytes = vec![0x42; 8192];
        let one_shot = blake3::hash(&bytes);

        let mut enc_a: Box<dyn OutboardEncoder> = Box::new(RootOnlyEncoder::new());
        for chunk in bytes.chunks(63) {
            enc_a.update(chunk);
        }
        let (root_a, _) = enc_a.finalize();
        assert_eq!(root_a, *one_shot.as_bytes());

        let mut enc_b: Box<dyn OutboardEncoder> = Box::new(BaoOutboardEncoder::new());
        for chunk in bytes.chunks(63) {
            enc_b.update(chunk);
        }
        let (root_b, _) = enc_b.finalize();
        assert_eq!(root_b, *one_shot.as_bytes());
    }
}
