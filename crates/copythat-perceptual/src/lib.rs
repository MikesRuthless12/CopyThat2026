//! Phase 42 — perceptual-hash + visual-similarity dedup.
//!
//! Pure-Rust integration of the [`image-hasher`] crate so Copy That
//! can warn the user before a copy clobbers a "visually identical"
//! file at the destination. The hash is a 64-bit fingerprint computed
//! over the image's perceptual content (not the byte stream); two
//! different JPEGs encoded from the same source picture, or a PNG and
//! a re-saved JPEG of the same scene, will hash to the same value
//! within a small Hamming-distance tolerance.
//!
//! # What's here today (Phase 42)
//!
//! - [`PerceptualKind`] — `Image` (default — works) and `Audio`
//!   (gated behind the `audio-perceptual` feature; returns
//!   [`PerceptualError::AudioNotImplemented`] in this revision until
//!   we settle the chromaprint / ffmpeg licensing question).
//! - [`perceptual_hash`] — opens the file, decodes via [`image`],
//!   runs the [`image_hasher::HasherConfig::new`] dHash pipeline, and
//!   returns the 64-bit hash as a `u64` (the upstream [`ImageHash`]
//!   `Vec<u8>` is folded into a `u64` for cheap storage in the chunk
//!   store + trivial wire-format).
//! - [`similarity`] — normalised Hamming distance between two hashes
//!   in the `[0.0, 1.0]` range. `0.0` = bit-identical;
//!   `0.5` = random; `1.0` = every bit flipped. The [`SIMILARITY_DEFAULT_THRESHOLD`]
//!   constant is the "warn on this or below" value the UI will use
//!   for the pre-copy "visually identical" prompt.
//!
//! # Threat model
//!
//! - The crate decodes user-supplied image files via the `image`
//!   crate. Decoder-side parsing bugs (`image` is broadly memory-safe
//!   Rust but bundles `weezl` / `gif` / `png` / `jpeg-decoder` etc.)
//!   surface as [`PerceptualError::DecodeFailed`] rather than panics
//!   so a malformed input doesn't take down the copy queue.
//! - Hash output is not a security primitive. It's perceptual — two
//!   different files SHOULD hash to the same value when they're
//!   visually similar. **Never use these hashes as a content-integrity
//!   check** — that's what `copythat-hash`'s BLAKE3 verifier is for.
//!
//! # Audio (deferred)
//!
//! `chromaprint-rs` (the obvious pick for audio fingerprinting) wraps
//! the GPL-incompatible Chromaprint C library; until we either find a
//! permissive pure-Rust alternative or ship a separate chromaprint-
//! linked helper binary, the audio path returns
//! [`PerceptualError::AudioNotImplemented`]. The feature gate keeps
//! the default `cargo build` light.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::path::Path;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// What kind of perceptual fingerprint the caller wants. Today only
/// [`PerceptualKind::Image`] is wired; [`PerceptualKind::Audio`] is
/// reserved for a future revision (see the module-level docstring).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PerceptualKind {
    /// JPEG / PNG / WebP / BMP / GIF — handled via the `image` crate
    /// + `image-hasher`'s dHash pipeline.
    Image,
    /// MP3 / FLAC / OGG / WAV — gated behind the `audio-perceptual`
    /// feature; returns [`PerceptualError::AudioNotImplemented`]
    /// until the chromaprint-vs-permissive question is resolved.
    Audio,
}

/// Failure modes for [`perceptual_hash`]. Every variant maps to a
/// stable Fluent message key the UI can render verbatim.
#[derive(Debug, Error)]
pub enum PerceptualError {
    /// Couldn't open the file at the given path.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    /// `image` couldn't decode the file (corrupt, unsupported format,
    /// not actually an image, etc.).
    #[error("image decode failed: {0}")]
    DecodeFailed(String),
    /// Caller passed [`PerceptualKind::Audio`]; deferred per the
    /// module-level note.
    #[error("audio perceptual hashing is not yet implemented")]
    AudioNotImplemented,
}

/// Default warn-threshold for the UI's "visually identical" prompt.
/// Hashes whose [`similarity`] returns `<= 0.10` are surfaced as
/// "looks identical" — that's roughly 6-bit-flip Hamming distance
/// across a 64-bit dHash, which empirically matches re-encodes of
/// the same source image. Tuneable later.
pub const SIMILARITY_DEFAULT_THRESHOLD: f32 = 0.10;

/// Compute a 64-bit perceptual fingerprint of the file at `path`.
///
/// `kind` selects the pipeline. For [`PerceptualKind::Image`] the
/// function decodes the file via the `image` crate (PNG / JPEG / BMP
/// / WebP / GIF — see this crate's `Cargo.toml` for the enabled
/// decoder set), runs the [`image_hasher::HasherConfig::new`] dHash
/// pipeline at the upstream default 8×8 resolution, and folds the
/// resulting `Vec<u8>` into a `u64`. Returns
/// [`PerceptualError::AudioNotImplemented`] for
/// [`PerceptualKind::Audio`] in this revision.
pub fn perceptual_hash(path: &Path, kind: PerceptualKind) -> Result<u64, PerceptualError> {
    match kind {
        PerceptualKind::Image => image_hash(path),
        PerceptualKind::Audio => Err(PerceptualError::AudioNotImplemented),
    }
}

fn image_hash(path: &Path) -> Result<u64, PerceptualError> {
    let img = image::open(path).map_err(|e| PerceptualError::DecodeFailed(e.to_string()))?;
    let hasher = image_hasher::HasherConfig::new()
        .hash_alg(image_hasher::HashAlg::DoubleGradient)
        .to_hasher();
    let hash = hasher.hash_image(&img);
    let bytes = hash.as_bytes();
    // Phase 42 post-review (M1) — defend the discrimination floor.
    // Pre-fix, `fold_to_u64` XOR-collapsed bytes 8..16 onto bytes
    // 0..8 in the same byte lanes; for the default 8×8
    // DoubleGradient config (8-byte output) this is fine, but a
    // future `hash_size(16, 16)` bump would silently halve
    // discriminating power without any compile-time signal. Assert
    // the size at hash construction so a misconfig fails loudly at
    // the caller's first hash, not silently in user data.
    debug_assert_eq!(
        bytes.len(),
        8,
        "copythat-perceptual: DoubleGradient at 8×8 should produce 8-byte hashes; \
         a config change to hash_size(16,16) would require a real mix function in fold_to_u64"
    );
    Ok(fold_to_u64(bytes))
}

/// Fold the `ImageHash` byte buffer into a `u64`. The upstream
/// `ImageHash` for the default DoubleGradient `8×8` config lands at
/// exactly 8 bytes (= one `u64`); the byte-aligned XOR fold below is
/// loss-free in that case. For configs that exceed 8 bytes the fold
/// is lossy by construction — see the `debug_assert_eq!` in
/// `image_hash` above for the loud-failure signal a misconfigured
/// hash size triggers in dev builds.
fn fold_to_u64(bytes: &[u8]) -> u64 {
    let mut acc: u64 = 0;
    for (i, b) in bytes.iter().enumerate() {
        acc ^= (*b as u64) << ((i % 8) * 8);
    }
    acc
}

/// Normalised Hamming distance between two perceptual hashes. Returns
/// a `f32` in `[0.0, 1.0]`:
///
/// - `0.0` — bit-identical (all 64 bits match)
/// - `0.5` — random (32 bits flipped on average)
/// - `1.0` — every bit inverted (extremely rare in practice)
///
/// The UI compares this against [`SIMILARITY_DEFAULT_THRESHOLD`] when
/// rendering the "looks visually identical" warning.
pub fn similarity(a: u64, b: u64) -> f32 {
    let differing_bits = (a ^ b).count_ones() as f32;
    differing_bits / 64.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn similarity_of_identical_hashes_is_zero() {
        assert_eq!(similarity(0xDEADBEEF, 0xDEADBEEF), 0.0);
        assert_eq!(similarity(0, 0), 0.0);
        assert_eq!(similarity(u64::MAX, u64::MAX), 0.0);
    }

    #[test]
    fn similarity_of_inverse_is_one() {
        assert_eq!(similarity(0, u64::MAX), 1.0);
        assert_eq!(similarity(0xFF, !0xFFu64), 1.0);
    }

    #[test]
    fn similarity_one_bit_difference_is_one_over_sixty_four() {
        let s = similarity(0, 1);
        assert!((s - (1.0 / 64.0)).abs() < f32::EPSILON);
    }

    #[test]
    fn similarity_is_symmetric() {
        let a = 0x123456789ABCDEF0;
        let b = 0xFEDCBA9876543210;
        assert_eq!(similarity(a, b), similarity(b, a));
    }

    #[test]
    fn audio_kind_returns_not_implemented() {
        let path = std::path::Path::new("dummy.mp3");
        let err = perceptual_hash(path, PerceptualKind::Audio).unwrap_err();
        assert!(matches!(err, PerceptualError::AudioNotImplemented));
    }

    #[test]
    fn nonexistent_file_yields_decode_failed_or_io_error() {
        let path = std::path::Path::new("nope-this-doesnt-exist-anywhere-12345.png");
        let err = perceptual_hash(path, PerceptualKind::Image).unwrap_err();
        // `image::open` returns its own error for missing files, so
        // the path here is `DecodeFailed`. Either is acceptable —
        // both are caught + classified rather than panicking.
        assert!(matches!(
            err,
            PerceptualError::DecodeFailed(_) | PerceptualError::Io(_)
        ));
    }

    #[test]
    fn perceptual_kind_serde_round_trip() {
        for &kind in &[PerceptualKind::Image, PerceptualKind::Audio] {
            let json = serde_json::to_string(&kind).unwrap();
            let back: PerceptualKind = serde_json::from_str(&json).unwrap();
            assert_eq!(kind, back);
        }
    }

    #[test]
    fn fold_to_u64_handles_short_and_long_buffers() {
        // 8-byte buffer (the dHash 8x8 default) round-trips verbatim.
        let bytes_8 = [0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xBA, 0xBE];
        let folded = fold_to_u64(&bytes_8);
        // Bytes are LE-shifted into the u64.
        let expected = 0xBEBAFECAEFBEADDE_u64;
        assert_eq!(folded, expected);

        // 1-byte buffer doesn't underflow.
        assert_eq!(fold_to_u64(&[0x42]), 0x42);

        // Empty buffer is zero.
        assert_eq!(fold_to_u64(&[]), 0);
    }

    #[test]
    fn default_threshold_is_in_range() {
        // Belt-and-suspenders gate against a future tweak that nudges
        // the constant outside the (0, 0.5) sane range. Clippy
        // auto-fires `assertions-on-constants` on raw `assert!(K > 0)`,
        // so we channel through a non-const helper to keep the lint
        // happy while still surfacing the invariant in a unit test.
        fn in_range(t: f32) -> bool {
            t > 0.0 && t < 0.5
        }
        assert!(in_range(SIMILARITY_DEFAULT_THRESHOLD));
    }
}
