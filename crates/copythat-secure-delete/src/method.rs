//! The nine named shred methods and the pass sequences they expand to.
//!
//! References:
//! - DoD 5220.22-M (1995 ECE variant of the 3-pass and 7-pass
//!   overwrites — the public sanitization guide published before NIST
//!   800-88 deprecated it for modern drives).
//! - Peter Gutmann, "Secure Deletion of Data from Magnetic and
//!   Solid-State Memory" (USENIX Security '96): the 35-pass sequence.
//! - Bruce Schneier, *Applied Cryptography*, 2nd ed., §10.9: the
//!   7-pass 0x00 / 0xFF / 5×Random scheme.
//! - BSI VSITR (Germany, 1999): 7 passes alternating 0x00 / 0xFF
//!   three times then 0xAA.
//! - NIST SP 800-88 rev. 1 (2014): modern media-sanitization taxonomy,
//!   splitting "Clear" (single logical overwrite) and "Purge" (hardware
//!   secure-erase or cryptographic erase).

use crate::pattern::{PassPattern, fixed, fixed_verify, random, random_verify, tiled3};

/// Chunk size used for each overwrite pass. Matches the copy engine's
/// default so the shred loop shares cache / tuning with `copy_file`.
pub const CHUNK_SIZE: usize = 1024 * 1024; // 1 MiB

/// A named multi-pass shred method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShredMethod {
    /// Single pass of all-zero bytes (0x00).
    Zero,
    /// Single pass of cryptographically-random bytes.
    Random,
    /// DoD 5220.22-M ECE 3-pass: 0x00, 0xFF, Random + verify.
    DoD3Pass,
    /// DoD 5220.22-M ECE 7-pass: 0x00, 0xFF, Random, 0x00, 0xFF,
    /// Random, Random + verify.
    DoD7Pass,
    /// Peter Gutmann's 35-pass sequence: 4 Random + 27 fixed Gutmann
    /// patterns + 4 Random.
    Gutmann35,
    /// Bruce Schneier's 7-pass: 0x00, 0xFF, five Random.
    Schneier7,
    /// BSI VSITR 7-pass: alternating 0x00 / 0xFF three times, then 0xAA.
    Vsitr7,
    /// NIST 800-88 Clear: single zero overwrite — modern default for
    /// magnetic media.
    Nist80088Clear,
    /// NIST 800-88 Purge: prefer hardware secure-erase. Refuses with
    /// [`ShredErrorKind::PurgeNotSupported`] when no hardware path is
    /// available.
    ///
    /// [`ShredErrorKind::PurgeNotSupported`]: crate::error::ShredErrorKind::PurgeNotSupported
    Nist80088Purge,
}

impl ShredMethod {
    /// Stable short name for logs / telemetry / sidecar records.
    pub fn name(&self) -> &'static str {
        match self {
            ShredMethod::Zero => "zero",
            ShredMethod::Random => "random",
            ShredMethod::DoD3Pass => "dod3",
            ShredMethod::DoD7Pass => "dod7",
            ShredMethod::Gutmann35 => "gutmann35",
            ShredMethod::Schneier7 => "schneier7",
            ShredMethod::Vsitr7 => "vsitr7",
            ShredMethod::Nist80088Clear => "nist-clear",
            ShredMethod::Nist80088Purge => "nist-purge",
        }
    }

    /// Number of software passes this method performs.
    ///
    /// `Nist80088Purge` returns 0 — the hardware path is not a sequence
    /// of passes; when no hardware path is available the method fails
    /// before any passes run.
    pub fn pass_count(&self) -> usize {
        match self {
            ShredMethod::Zero | ShredMethod::Random | ShredMethod::Nist80088Clear => 1,
            ShredMethod::DoD3Pass => 3,
            ShredMethod::DoD7Pass => 7,
            ShredMethod::Schneier7 | ShredMethod::Vsitr7 => 7,
            ShredMethod::Gutmann35 => 35,
            ShredMethod::Nist80088Purge => 0,
        }
    }

    /// Expand this method into its pass sequence.
    ///
    /// Always returns the full list — the engine decides how to fill
    /// buffers for each pattern kind.
    pub fn passes(&self) -> Vec<PassPattern> {
        match self {
            ShredMethod::Zero | ShredMethod::Nist80088Clear => vec![fixed(0x00)],
            ShredMethod::Random => vec![random()],
            ShredMethod::DoD3Pass => vec![fixed(0x00), fixed(0xFF), random_verify()],
            ShredMethod::DoD7Pass => vec![
                fixed(0x00),
                fixed(0xFF),
                random(),
                fixed(0x00),
                fixed(0xFF),
                random(),
                random_verify(),
            ],
            ShredMethod::Schneier7 => vec![
                fixed(0x00),
                fixed(0xFF),
                random(),
                random(),
                random(),
                random(),
                random(),
            ],
            ShredMethod::Vsitr7 => vec![
                fixed(0x00),
                fixed(0xFF),
                fixed(0x00),
                fixed(0xFF),
                fixed(0x00),
                fixed(0xFF),
                fixed_verify(0xAA),
            ],
            ShredMethod::Gutmann35 => gutmann_sequence(),
            ShredMethod::Nist80088Purge => Vec::new(),
        }
    }
}

impl std::fmt::Display for ShredMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// Peter Gutmann's 35-pass overwrite sequence from
/// "Secure Deletion of Data from Magnetic and Solid-State Memory"
/// (USENIX Security '96), Table 1:
///
/// Passes 1-4:   Random
/// Passes 5-31:  27 fixed patterns targeting historical MFM/RLL
///               encodings
/// Passes 32-35: Random
fn gutmann_sequence() -> Vec<PassPattern> {
    vec![
        // 1-4: Random
        random(),
        random(),
        random(),
        random(),
        // 5: 0x55 0x55 0x55
        tiled3(0x55, 0x55, 0x55),
        // 6: 0xAA 0xAA 0xAA
        tiled3(0xAA, 0xAA, 0xAA),
        // 7: 0x92 0x49 0x24
        tiled3(0x92, 0x49, 0x24),
        // 8: 0x49 0x24 0x92
        tiled3(0x49, 0x24, 0x92),
        // 9: 0x24 0x92 0x49
        tiled3(0x24, 0x92, 0x49),
        // 10: 0x00 0x00 0x00
        tiled3(0x00, 0x00, 0x00),
        // 11: 0x11 0x11 0x11
        tiled3(0x11, 0x11, 0x11),
        // 12: 0x22 0x22 0x22
        tiled3(0x22, 0x22, 0x22),
        // 13: 0x33 0x33 0x33
        tiled3(0x33, 0x33, 0x33),
        // 14: 0x44 0x44 0x44
        tiled3(0x44, 0x44, 0x44),
        // 15: 0x55 0x55 0x55
        tiled3(0x55, 0x55, 0x55),
        // 16: 0x66 0x66 0x66
        tiled3(0x66, 0x66, 0x66),
        // 17: 0x77 0x77 0x77
        tiled3(0x77, 0x77, 0x77),
        // 18: 0x88 0x88 0x88
        tiled3(0x88, 0x88, 0x88),
        // 19: 0x99 0x99 0x99
        tiled3(0x99, 0x99, 0x99),
        // 20: 0xAA 0xAA 0xAA
        tiled3(0xAA, 0xAA, 0xAA),
        // 21: 0xBB 0xBB 0xBB
        tiled3(0xBB, 0xBB, 0xBB),
        // 22: 0xCC 0xCC 0xCC
        tiled3(0xCC, 0xCC, 0xCC),
        // 23: 0xDD 0xDD 0xDD
        tiled3(0xDD, 0xDD, 0xDD),
        // 24: 0xEE 0xEE 0xEE
        tiled3(0xEE, 0xEE, 0xEE),
        // 25: 0xFF 0xFF 0xFF
        tiled3(0xFF, 0xFF, 0xFF),
        // 26: 0x92 0x49 0x24
        tiled3(0x92, 0x49, 0x24),
        // 27: 0x49 0x24 0x92
        tiled3(0x49, 0x24, 0x92),
        // 28: 0x24 0x92 0x49
        tiled3(0x24, 0x92, 0x49),
        // 29: 0x6D 0xB6 0xDB
        tiled3(0x6D, 0xB6, 0xDB),
        // 30: 0xB6 0xDB 0x6D
        tiled3(0xB6, 0xDB, 0x6D),
        // 31: 0xDB 0x6D 0xB6
        tiled3(0xDB, 0x6D, 0xB6),
        // 32-35: Random
        random(),
        random(),
        random(),
        random(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pass_counts_match_expansion() {
        for m in [
            ShredMethod::Zero,
            ShredMethod::Random,
            ShredMethod::DoD3Pass,
            ShredMethod::DoD7Pass,
            ShredMethod::Gutmann35,
            ShredMethod::Schneier7,
            ShredMethod::Vsitr7,
            ShredMethod::Nist80088Clear,
        ] {
            assert_eq!(
                m.passes().len(),
                m.pass_count(),
                "{m}: pass_count vs passes().len() disagree"
            );
        }
        // Purge returns zero software passes.
        assert_eq!(ShredMethod::Nist80088Purge.passes().len(), 0);
        assert_eq!(ShredMethod::Nist80088Purge.pass_count(), 0);
    }

    #[test]
    fn dod3_ends_in_random_verify() {
        let passes = ShredMethod::DoD3Pass.passes();
        assert_eq!(passes.len(), 3);
        assert!(matches!(passes[0], PassPattern::Fixed { byte: 0x00, .. }));
        assert!(matches!(passes[1], PassPattern::Fixed { byte: 0xFF, .. }));
        assert!(matches!(passes[2], PassPattern::Random { verify: true }));
    }

    #[test]
    fn dod7_ends_in_random_verify() {
        let passes = ShredMethod::DoD7Pass.passes();
        assert_eq!(passes.len(), 7);
        assert!(matches!(passes[6], PassPattern::Random { verify: true }));
    }

    #[test]
    fn vsitr_alternates_then_ends_in_aa_verify() {
        let passes = ShredMethod::Vsitr7.passes();
        assert_eq!(passes.len(), 7);
        for (i, p) in passes.iter().take(6).enumerate() {
            let want = if i % 2 == 0 { 0x00 } else { 0xFF };
            match p {
                PassPattern::Fixed { byte, .. } => assert_eq!(
                    *byte,
                    want,
                    "VSITR pass {} should be 0x{want:02X} not 0x{byte:02X}",
                    i + 1
                ),
                other => panic!("VSITR pass {} not Fixed: {other:?}", i + 1),
            }
        }
        assert!(matches!(
            passes[6],
            PassPattern::Fixed {
                byte: 0xAA,
                verify: true
            }
        ));
    }

    #[test]
    fn gutmann_shape_4_random_27_fixed_4_random() {
        let passes = ShredMethod::Gutmann35.passes();
        assert_eq!(passes.len(), 35);
        // First 4 Random.
        for (i, p) in passes.iter().take(4).enumerate() {
            assert!(
                matches!(p, PassPattern::Random { .. }),
                "Gutmann pass {} expected Random, got {p:?}",
                i + 1
            );
        }
        // Last 4 Random.
        for (i, p) in passes.iter().skip(31).enumerate() {
            assert!(
                matches!(p, PassPattern::Random { .. }),
                "Gutmann pass {} expected Random, got {p:?}",
                32 + i
            );
        }
        // Middle 27 Tiled.
        for (i, p) in passes.iter().skip(4).take(27).enumerate() {
            assert!(
                matches!(p, PassPattern::Tiled { .. }),
                "Gutmann pass {} expected Tiled, got {p:?}",
                5 + i
            );
        }
    }

    #[test]
    fn schneier_shape_zero_ff_five_random() {
        let passes = ShredMethod::Schneier7.passes();
        assert_eq!(passes.len(), 7);
        assert!(matches!(passes[0], PassPattern::Fixed { byte: 0x00, .. }));
        assert!(matches!(passes[1], PassPattern::Fixed { byte: 0xFF, .. }));
        for p in &passes[2..] {
            assert!(matches!(p, PassPattern::Random { .. }));
        }
    }

    #[test]
    fn names_are_stable_and_unique() {
        let all = [
            ShredMethod::Zero,
            ShredMethod::Random,
            ShredMethod::DoD3Pass,
            ShredMethod::DoD7Pass,
            ShredMethod::Gutmann35,
            ShredMethod::Schneier7,
            ShredMethod::Vsitr7,
            ShredMethod::Nist80088Clear,
            ShredMethod::Nist80088Purge,
        ];
        let mut names: Vec<&'static str> = all.iter().map(|m| m.name()).collect();
        names.sort();
        let mut dedup = names.clone();
        dedup.dedup();
        assert_eq!(names, dedup, "ShredMethod::name() collided");
    }
}
