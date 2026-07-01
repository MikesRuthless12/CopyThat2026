//! Phase 49h — chunk compression policy.
//!
//! Chunks are content-addressed by `BLAKE3(plaintext)` — that hash is the
//! dedup key, the manifest key, and the CDR-0 identity, so it MUST NOT
//! change. Compression therefore only changes the *bytes on disk*: the
//! chunk hash + the manifest's logical length are always of the
//! plaintext, and [`crate::ChunkStore::get`] always returns plaintext.
//!
//! This is deliberately a tiny, age-free mirror of
//! `copythat_crypt::CompressionLevel` (same `1..=22` range + default 3) so
//! `copythat-chunk` stays self-contained — exactly as it re-implements hex
//! instead of pulling a crate. (`copythat_crypt::CompressionPolicy` is a
//! file-extension deny-list at the file layer; it does not map onto
//! chunks, which have no extension.)

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// How a stored chunk's bytes are encoded in its pack. The chunk hash is
/// always BLAKE3 of the *plaintext*, independent of codec.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ChunkCodec {
    /// Raw bytes (on-disk == plaintext). On-disk/wire tag `0`.
    #[default]
    None,
    /// A single zstd frame. On-disk/wire tag `1`.
    Zstd,
}

impl ChunkCodec {
    /// The 1-byte on-disk tag.
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        match self {
            Self::None => 0,
            Self::Zstd => 1,
        }
    }

    /// Decode a 1-byte tag; `None` for an unrecognised value.
    #[must_use]
    pub const fn from_u8(b: u8) -> Option<Self> {
        match b {
            0 => Some(Self::None),
            1 => Some(Self::Zstd),
            _ => None,
        }
    }

    /// Stable lowercase tag (matches the kebab-case serde form).
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Zstd => "zstd",
        }
    }
}

/// zstd compression level. Mirrors `copythat_crypt::CompressionLevel`
/// (`1..=22`, default 3) so the chunk crate stays age-free.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CompressionLevel(pub i32);

impl CompressionLevel {
    /// Lowest zstd level (fastest, least compression).
    pub const MIN: i32 = 1;
    /// Highest zstd level (slowest, most compression).
    pub const MAX: i32 = 22;
    /// Default level (zstd's own default).
    pub const DEFAULT: i32 = 3;

    /// Clamp into the valid `1..=22` range.
    #[must_use]
    pub fn clamped(self) -> i32 {
        self.0.clamp(Self::MIN, Self::MAX)
    }
}

impl Default for CompressionLevel {
    fn default() -> Self {
        Self(Self::DEFAULT)
    }
}

/// Repository-level compression strategy. Distinct from the file-level
/// `copythat_crypt::CompressionPolicy` (an extension deny-list): chunks
/// have no extension, so "skip incompressible" is decided by *trying*.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "kebab-case")]
pub enum RepoCompression {
    /// Never compress (the pre-Phase-49h behaviour).
    #[default]
    Off,
    /// Try zstd; keep it only when it shrinks the chunk by the margin —
    /// already-compressed media falls back to raw automatically.
    Auto {
        /// zstd level to try.
        level: CompressionLevel,
    },
    /// Compress every chunk, but still store raw (recording `None`) if the
    /// "compressed" form ended up no smaller (incompressible data).
    Always {
        /// zstd level.
        level: CompressionLevel,
    },
}

/// The fixed margin restic/borg use for "auto": keep the compressed form
/// only when it saves at least ~3% (avoids storing a zstd frame that's
/// barely smaller — or larger — than the plaintext).
const AUTO_KEEP_NUM: u64 = 97;
const AUTO_KEEP_DEN: u64 = 100;

impl RepoCompression {
    /// Decide the codec + the bytes to actually store for `plaintext`.
    /// `Off` (or a compression that doesn't win) borrows `plaintext`
    /// unchanged with [`ChunkCodec::None`]; a win returns the owned zstd
    /// frame with [`ChunkCodec::Zstd`].
    #[must_use]
    pub fn encode<'a>(&self, plaintext: &'a [u8]) -> (ChunkCodec, Cow<'a, [u8]>) {
        let level = match self {
            Self::Off => return (ChunkCodec::None, Cow::Borrowed(plaintext)),
            Self::Auto { level } | Self::Always { level } => level.clamped(),
        };
        let Ok(compressed) = zstd::bulk::compress(plaintext, level) else {
            return (ChunkCodec::None, Cow::Borrowed(plaintext));
        };
        let keep = match self {
            // Always: keep iff it's strictly smaller (else raw).
            Self::Always { .. } => compressed.len() < plaintext.len(),
            // Auto: keep iff it wins by the ~3% margin.
            _ => {
                (compressed.len() as u64) * AUTO_KEEP_DEN < (plaintext.len() as u64) * AUTO_KEEP_NUM
            }
        };
        if keep {
            (ChunkCodec::Zstd, Cow::Owned(compressed))
        } else {
            (ChunkCodec::None, Cow::Borrowed(plaintext))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codec_tag_round_trips() {
        for c in [ChunkCodec::None, ChunkCodec::Zstd] {
            assert_eq!(ChunkCodec::from_u8(c.as_u8()), Some(c));
        }
        assert_eq!(ChunkCodec::from_u8(2), None);
    }

    #[test]
    fn level_clamps() {
        assert_eq!(CompressionLevel(0).clamped(), 1);
        assert_eq!(CompressionLevel(99).clamped(), 22);
        assert_eq!(CompressionLevel(7).clamped(), 7);
        assert_eq!(CompressionLevel::default().0, 3);
    }

    #[test]
    fn off_never_compresses() {
        let (codec, bytes) = RepoCompression::Off.encode(&[0u8; 4096]);
        assert_eq!(codec, ChunkCodec::None);
        assert_eq!(bytes.len(), 4096);
    }

    #[test]
    fn compressible_data_shrinks_under_zstd() {
        // Highly repetitive → zstd wins big.
        let data = vec![7u8; 64 * 1024];
        let (codec, bytes) = RepoCompression::Auto {
            level: CompressionLevel::default(),
        }
        .encode(&data);
        assert_eq!(codec, ChunkCodec::Zstd);
        assert!(bytes.len() < data.len());
    }

    #[test]
    fn incompressible_data_stays_raw_under_auto() {
        // Pseudo-random → zstd can't beat the 3% margin → store raw.
        let mut data = vec![0u8; 64 * 1024];
        let mut s = 1234567u64;
        for b in &mut data {
            s = s.wrapping_mul(6364136223846793005).wrapping_add(1);
            *b = (s >> 33) as u8;
        }
        let (codec, bytes) = RepoCompression::Auto {
            level: CompressionLevel::default(),
        }
        .encode(&data);
        assert_eq!(codec, ChunkCodec::None);
        assert_eq!(bytes.len(), data.len());
    }
}
