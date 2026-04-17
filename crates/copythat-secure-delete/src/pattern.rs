//! A single overwrite pass descriptor.
//!
//! Each [`ShredMethod`](crate::ShredMethod) expands into an ordered list
//! of `PassPattern`s. The engine walks that list, writing the file's
//! length worth of data per pass and optionally re-reading to verify.

/// One overwrite pass.
///
/// `Fixed(byte)`   — fill every byte of the file with `byte`.
/// `Bytes(slice)`  — tile the slice over the file length (Gutmann's
///                    fixed patterns are 3 bytes wide; "tile" means the
///                    pattern repeats to the file's size).
/// `Random`        — cryptographically-random bytes from `OsRng`.
/// The `verify` flag, when `true`, re-reads the file after writing and
/// asserts the written pattern is what's on disk. Used as the last pass
/// of DoD 3/7 variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PassPattern {
    /// Write a single constant byte across the whole file.
    Fixed { byte: u8, verify: bool },
    /// Write cryptographically-random bytes across the whole file.
    Random { verify: bool },
    /// Tile a short fixed pattern (up to 8 bytes) across the whole file.
    /// Used for Gutmann's 3-byte-wide patterns.
    Tiled {
        pattern: [u8; 8],
        len: u8,
        verify: bool,
    },
}

impl PassPattern {
    pub(crate) fn verify(&self) -> bool {
        match self {
            PassPattern::Fixed { verify, .. } => *verify,
            PassPattern::Random { verify } => *verify,
            PassPattern::Tiled { verify, .. } => *verify,
        }
    }
}

pub(crate) const fn fixed(byte: u8) -> PassPattern {
    PassPattern::Fixed {
        byte,
        verify: false,
    }
}

pub(crate) const fn fixed_verify(byte: u8) -> PassPattern {
    PassPattern::Fixed { byte, verify: true }
}

pub(crate) const fn random() -> PassPattern {
    PassPattern::Random { verify: false }
}

pub(crate) const fn random_verify() -> PassPattern {
    PassPattern::Random { verify: true }
}

pub(crate) const fn tiled3(a: u8, b: u8, c: u8) -> PassPattern {
    // Store three-byte pattern into an 8-byte buffer; `len` records how
    // much of the buffer is meaningful.
    PassPattern::Tiled {
        pattern: [a, b, c, 0, 0, 0, 0, 0],
        len: 3,
        verify: false,
    }
}
