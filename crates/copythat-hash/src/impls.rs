//! Concrete `Hasher` implementations.
//!
//! Each implementation is a thin adapter over its upstream crate so the
//! pipeline in `crate::streaming` and the verify hook in `copythat-core`
//! only ever talks to the `Hasher` trait (which lives in
//! `copythat_core::verify`; `copythat-hash` re-exports it as
//! `copythat_hash::Hasher`). Every type here is constructed through
//! `HashAlgorithm::hasher`; consumers should not refer to the concrete
//! types directly.

use digest::Digest;

use crate::algorithm::{HashAlgorithm, Hasher};

// ---------- CRC32 ----------

pub(crate) struct Crc32Hasher {
    inner: crc32fast::Hasher,
}

impl Crc32Hasher {
    pub(crate) fn new() -> Self {
        Self {
            inner: crc32fast::Hasher::new(),
        }
    }
}

impl Hasher for Crc32Hasher {
    fn name(&self) -> &'static str {
        HashAlgorithm::Crc32.name()
    }
    fn update(&mut self, bytes: &[u8]) {
        self.inner.update(bytes);
    }
    fn finalize(self: Box<Self>) -> Vec<u8> {
        // Big-endian — matches the convention used by most CRC32 tools
        // and keeps our hex output stable across platforms.
        self.inner.finalize().to_be_bytes().to_vec()
    }
}

// ---------- MD5 / SHA-1 / SHA-256 / SHA-512 ----------
//
// All four are `digest::Digest` implementations, so one macro gives us
// four essentially-identical adapters.

macro_rules! digest_hasher {
    ($struct_name:ident, $inner_ty:ty, $algo:expr) => {
        pub(crate) struct $struct_name {
            inner: $inner_ty,
        }
        impl $struct_name {
            pub(crate) fn new() -> Self {
                Self {
                    inner: <$inner_ty>::new(),
                }
            }
        }
        impl Hasher for $struct_name {
            fn name(&self) -> &'static str {
                $algo.name()
            }
            fn update(&mut self, bytes: &[u8]) {
                self.inner.update(bytes);
            }
            fn finalize(self: Box<Self>) -> Vec<u8> {
                self.inner.finalize().to_vec()
            }
        }
    };
}

digest_hasher!(Md5Hasher, md5::Md5, HashAlgorithm::Md5);
digest_hasher!(Sha1Hasher, sha1::Sha1, HashAlgorithm::Sha1);
digest_hasher!(Sha256Hasher, sha2::Sha256, HashAlgorithm::Sha256);
digest_hasher!(Sha512Hasher, sha2::Sha512, HashAlgorithm::Sha512);

// ---------- xxHash3-64 ----------

pub(crate) struct XxHash3_64Hasher {
    inner: xxhash_rust::xxh3::Xxh3,
}

impl XxHash3_64Hasher {
    pub(crate) fn new() -> Self {
        Self {
            inner: xxhash_rust::xxh3::Xxh3::new(),
        }
    }
}

impl Hasher for XxHash3_64Hasher {
    fn name(&self) -> &'static str {
        HashAlgorithm::XxHash3_64.name()
    }
    fn update(&mut self, bytes: &[u8]) {
        self.inner.update(bytes);
    }
    fn finalize(self: Box<Self>) -> Vec<u8> {
        // xxHash is most commonly rendered big-endian for hex output
        // (matches the reference `xxhsum` CLI); we match that.
        self.inner.digest().to_be_bytes().to_vec()
    }
}

// ---------- xxHash3-128 ----------

#[allow(non_camel_case_types)]
pub(crate) struct XxHash3_128Hasher {
    inner: xxhash_rust::xxh3::Xxh3,
}

impl XxHash3_128Hasher {
    pub(crate) fn new() -> Self {
        Self {
            inner: xxhash_rust::xxh3::Xxh3::new(),
        }
    }
}

impl Hasher for XxHash3_128Hasher {
    fn name(&self) -> &'static str {
        HashAlgorithm::XxHash3_128.name()
    }
    fn update(&mut self, bytes: &[u8]) {
        self.inner.update(bytes);
    }
    fn finalize(self: Box<Self>) -> Vec<u8> {
        self.inner.digest128().to_be_bytes().to_vec()
    }
}

// ---------- BLAKE3 ----------

pub(crate) struct Blake3Hasher {
    inner: blake3::Hasher,
}

impl Blake3Hasher {
    pub(crate) fn new() -> Self {
        Self {
            inner: blake3::Hasher::new(),
        }
    }
}

impl Hasher for Blake3Hasher {
    fn name(&self) -> &'static str {
        HashAlgorithm::Blake3.name()
    }
    fn update(&mut self, bytes: &[u8]) {
        self.inner.update(bytes);
    }
    fn finalize(self: Box<Self>) -> Vec<u8> {
        self.inner.finalize().as_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    //! Known-answer tests. Vectors are cross-checked against:
    //! - `echo -n "" | <tool>` output (empty string).
    //! - `echo -n "abc" | <tool>` output.
    //! - RFC / spec-published vectors where applicable.
    //!
    //! Sources:
    //! - MD5 RFC 1321 appendix.
    //! - SHA-1 FIPS 180-2 appendix A.
    //! - SHA-256 FIPS 180-2 appendix B.
    //! - SHA-512 FIPS 180-2 appendix C.
    //! - BLAKE3 <https://github.com/BLAKE3-team/BLAKE3/blob/master/test_vectors/test_vectors.json>
    //! - xxHash3 `xxhsum -H3 --little-endian ...` spot-checks, converted
    //!   to the big-endian encoding our `finalize` uses.

    use crate::algorithm::HashAlgorithm;

    fn run(algo: HashAlgorithm, input: &[u8]) -> String {
        let mut h = algo.hasher();
        h.update(input);
        hex::encode(h.finalize())
    }

    #[test]
    fn crc32_known_vectors() {
        assert_eq!(run(HashAlgorithm::Crc32, b""), "00000000");
        assert_eq!(run(HashAlgorithm::Crc32, b"abc"), "352441c2");
        // Classic "123456789" vector for CRC32/ISO-HDLC.
        assert_eq!(run(HashAlgorithm::Crc32, b"123456789"), "cbf43926");
    }

    #[test]
    fn md5_known_vectors() {
        // RFC 1321 test suite.
        assert_eq!(
            run(HashAlgorithm::Md5, b""),
            "d41d8cd98f00b204e9800998ecf8427e"
        );
        assert_eq!(
            run(HashAlgorithm::Md5, b"abc"),
            "900150983cd24fb0d6963f7d28e17f72"
        );
        assert_eq!(
            run(HashAlgorithm::Md5, b"message digest"),
            "f96b697d7cb7938d525a2f31aaf161d0"
        );
    }

    #[test]
    fn sha1_known_vectors() {
        assert_eq!(
            run(HashAlgorithm::Sha1, b""),
            "da39a3ee5e6b4b0d3255bfef95601890afd80709"
        );
        assert_eq!(
            run(HashAlgorithm::Sha1, b"abc"),
            "a9993e364706816aba3e25717850c26c9cd0d89d"
        );
    }

    #[test]
    fn sha256_known_vectors() {
        assert_eq!(
            run(HashAlgorithm::Sha256, b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            run(HashAlgorithm::Sha256, b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn sha512_known_vectors() {
        assert_eq!(
            run(HashAlgorithm::Sha512, b""),
            "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce\
             47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e"
        );
        assert_eq!(
            run(HashAlgorithm::Sha512, b"abc"),
            "ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a\
             2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f"
        );
    }

    #[test]
    fn blake3_known_vectors() {
        // From the BLAKE3 reference test vectors.
        assert_eq!(
            run(HashAlgorithm::Blake3, b""),
            "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262"
        );
        // The reference vector for the byte sequence 0..=250 (len = 1)
        // → "0x00" produces the documented digest below.
        assert_eq!(
            run(HashAlgorithm::Blake3, &[0u8]),
            "2d3adedff11b61f14c886e35afa036736dcd87a74d27b5c1510225d0f592e213"
        );
    }

    #[test]
    fn xxh3_64_known_vectors() {
        // Reference: `xxhsum -H3`, converted from its native little-endian
        // display to the big-endian byte sequence our finalize emits.
        // Empty input canonical value: 0x2d06800538d394c2.
        assert_eq!(run(HashAlgorithm::XxHash3_64, b""), "2d06800538d394c2");
        // "abc" canonical value: 0x78af5f94892f3950.
        assert_eq!(run(HashAlgorithm::XxHash3_64, b"abc"), "78af5f94892f3950");
    }

    #[test]
    fn xxh3_128_known_vectors() {
        // Empty input: 0x99aa06d3014798d86001c324468d497f (big-endian).
        assert_eq!(
            run(HashAlgorithm::XxHash3_128, b""),
            "99aa06d3014798d86001c324468d497f"
        );
        // "abc": 0x06b05ab6733a618578af5f94892f3950 (big-endian).
        assert_eq!(
            run(HashAlgorithm::XxHash3_128, b"abc"),
            "06b05ab6733a618578af5f94892f3950"
        );
    }

    #[test]
    fn streaming_matches_single_shot() {
        // Feed the same bytes in two different chunk shapes and assert
        // equal digests — this is the invariant the verify pipeline
        // relies on when a 1 MiB buffer splits a message.
        let payload: Vec<u8> = (0u8..=255).collect();
        for algo in HashAlgorithm::ALL {
            let mut one = algo.hasher();
            one.update(&payload);
            let one_shot = one.finalize();

            let mut chunked = algo.hasher();
            for chunk in payload.chunks(7) {
                chunked.update(chunk);
            }
            let streamed = chunked.finalize();

            assert_eq!(one_shot, streamed, "streaming mismatch for {algo}");
        }
    }
}
