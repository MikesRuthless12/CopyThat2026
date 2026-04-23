//! `HashAlgorithm` enum and the streaming `Hasher` trait.
//!
//! The trait is deliberately small: start, feed bytes in chunks, collect
//! the final digest as a `Vec<u8>`. Every algorithm in Phase 3 plugs into
//! the same pipeline — `copy_file` with verify and the streaming
//! `hash_file_async` both consume `Box<dyn Hasher>` and never special-case
//! an algorithm at a higher layer.

use std::fmt;
use std::str::FromStr;

pub use copythat_core::Hasher;

/// The set of verification algorithms Copy That v1.25.0 supports.
///
/// Order and stability of this enum matter — it is serialised in
/// `CopyOptions`, in sidecar filenames, and in the Phase 3 smoke test.
/// New algorithms go at the end.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HashAlgorithm {
    Crc32,
    Md5,
    Sha1,
    Sha256,
    Sha512,
    /// xxHash3 with a 64-bit digest. Not cryptographic — intended for
    /// fast "did anything change?" checks over big files.
    XxHash3_64,
    /// xxHash3 with a 128-bit digest. Same story, wider.
    XxHash3_128,
    Blake3,
}

impl HashAlgorithm {
    /// All supported algorithms in declaration order. Handy for tests
    /// and "run against each" iterators.
    pub const ALL: &'static [HashAlgorithm] = &[
        HashAlgorithm::Crc32,
        HashAlgorithm::Md5,
        HashAlgorithm::Sha1,
        HashAlgorithm::Sha256,
        HashAlgorithm::Sha512,
        HashAlgorithm::XxHash3_64,
        HashAlgorithm::XxHash3_128,
        HashAlgorithm::Blake3,
    ];

    /// Short stable name — used for sidecar filenames (e.g. `.sha256`)
    /// and UI labels. Never include version numbers here; the enum
    /// variant is the version.
    pub fn name(self) -> &'static str {
        match self {
            HashAlgorithm::Crc32 => "crc32",
            HashAlgorithm::Md5 => "md5",
            HashAlgorithm::Sha1 => "sha1",
            HashAlgorithm::Sha256 => "sha256",
            HashAlgorithm::Sha512 => "sha512",
            HashAlgorithm::XxHash3_64 => "xxh3-64",
            HashAlgorithm::XxHash3_128 => "xxh3-128",
            HashAlgorithm::Blake3 => "blake3",
        }
    }

    /// Output digest length in bytes.
    pub fn digest_len(self) -> usize {
        match self {
            HashAlgorithm::Crc32 => 4,
            HashAlgorithm::Md5 => 16,
            HashAlgorithm::Sha1 => 20,
            HashAlgorithm::Sha256 => 32,
            HashAlgorithm::Sha512 => 64,
            HashAlgorithm::XxHash3_64 => 8,
            HashAlgorithm::XxHash3_128 => 16,
            HashAlgorithm::Blake3 => 32,
        }
    }

    /// Whether this algorithm is exposed as a TeraCopy-compatible
    /// sidecar file. Only MD5 / SHA-1 / SHA-256 / SHA-512 / BLAKE3 get
    /// that treatment — CRC32 and xxHash3 are internal verify-only.
    pub fn sidecar_extension(self) -> Option<&'static str> {
        match self {
            HashAlgorithm::Md5 => Some("md5"),
            HashAlgorithm::Sha1 => Some("sha1"),
            HashAlgorithm::Sha256 => Some("sha256"),
            HashAlgorithm::Sha512 => Some("sha512"),
            HashAlgorithm::Blake3 => Some("blake3"),
            _ => None,
        }
    }

    /// Fresh boxed hasher ready to consume bytes.
    pub fn hasher(self) -> Box<dyn Hasher> {
        match self {
            HashAlgorithm::Crc32 => Box::new(crate::impls::Crc32Hasher::new()),
            HashAlgorithm::Md5 => Box::new(crate::impls::Md5Hasher::new()),
            HashAlgorithm::Sha1 => Box::new(crate::impls::Sha1Hasher::new()),
            HashAlgorithm::Sha256 => Box::new(crate::impls::Sha256Hasher::new()),
            HashAlgorithm::Sha512 => Box::new(crate::impls::Sha512Hasher::new()),
            HashAlgorithm::XxHash3_64 => Box::new(crate::impls::XxHash3_64Hasher::new()),
            HashAlgorithm::XxHash3_128 => Box::new(crate::impls::XxHash3_128Hasher::new()),
            HashAlgorithm::Blake3 => Box::new(crate::impls::Blake3Hasher::new()),
        }
    }

    /// Produce a `copythat_core::Verifier` for this algorithm. The
    /// returned verifier is cheap to clone (it wraps an `Arc`) and
    /// suitable for stashing in `CopyOptions::verify`.
    pub fn verifier(self) -> copythat_core::Verifier {
        copythat_core::Verifier::new(self.name(), move || self.hasher())
    }
}

impl fmt::Display for HashAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// Parse an algorithm name as produced by [`HashAlgorithm::name`].
/// Case-insensitive and accepts a few common aliases (`sha-256`,
/// `md-5`, `xxh3`).
impl FromStr for HashAlgorithm {
    type Err = UnknownAlgorithm;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let canon: String = s
            .to_ascii_lowercase()
            .chars()
            .filter(|c| *c != '-' && *c != '_')
            .collect();
        Ok(match canon.as_str() {
            "crc32" | "crc" => HashAlgorithm::Crc32,
            "md5" => HashAlgorithm::Md5,
            "sha1" => HashAlgorithm::Sha1,
            "sha256" => HashAlgorithm::Sha256,
            "sha512" => HashAlgorithm::Sha512,
            "xxh364" | "xxhash364" | "xxh3" => HashAlgorithm::XxHash3_64,
            "xxh3128" | "xxhash3128" => HashAlgorithm::XxHash3_128,
            "blake3" | "b3" => HashAlgorithm::Blake3,
            _ => return Err(UnknownAlgorithm(s.to_string())),
        })
    }
}

/// Error returned by `HashAlgorithm::from_str` when the name is not
/// recognised.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownAlgorithm(pub String);

impl fmt::Display for UnknownAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown hash algorithm: {:?}", self.0)
    }
}

impl std::error::Error for UnknownAlgorithm {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_roundtrip_through_fromstr() {
        for algo in HashAlgorithm::ALL {
            let name = algo.name();
            assert_eq!(HashAlgorithm::from_str(name).unwrap(), *algo, "{name}");
        }
    }

    #[test]
    fn fromstr_accepts_aliases() {
        assert_eq!(
            HashAlgorithm::from_str("SHA-256").unwrap(),
            HashAlgorithm::Sha256
        );
        assert_eq!(HashAlgorithm::from_str("MD-5").unwrap(), HashAlgorithm::Md5);
        assert_eq!(
            HashAlgorithm::from_str("xxh3").unwrap(),
            HashAlgorithm::XxHash3_64
        );
        assert_eq!(
            HashAlgorithm::from_str("b3").unwrap(),
            HashAlgorithm::Blake3
        );
    }

    #[test]
    fn digest_lengths_sensible() {
        for algo in HashAlgorithm::ALL {
            let mut h = algo.hasher();
            h.update(b"");
            let bytes = h.finalize();
            assert_eq!(bytes.len(), algo.digest_len(), "{}", algo);
        }
    }

    #[test]
    fn sidecars_present_only_for_user_facing_algorithms() {
        assert_eq!(HashAlgorithm::Crc32.sidecar_extension(), None);
        assert_eq!(HashAlgorithm::XxHash3_64.sidecar_extension(), None);
        assert_eq!(HashAlgorithm::XxHash3_128.sidecar_extension(), None);
        assert_eq!(HashAlgorithm::Md5.sidecar_extension(), Some("md5"));
        assert_eq!(HashAlgorithm::Sha1.sidecar_extension(), Some("sha1"));
        assert_eq!(HashAlgorithm::Sha256.sidecar_extension(), Some("sha256"));
        assert_eq!(HashAlgorithm::Sha512.sidecar_extension(), Some("sha512"));
        assert_eq!(HashAlgorithm::Blake3.sidecar_extension(), Some("blake3"));
    }
}
