//! Phase 35 — policy types (recipients, encryption policy, compression
//! policy + level + default deny-extension list).
//!
//! These are the inputs the engine consumes at `copy_file` entry
//! time. Keep them serde-friendly so `copythat-settings` can round-
//! trip them without a second DTO layer.

use std::collections::HashSet;

use secrecy::SecretString;
use serde::{Deserialize, Serialize};

/// Who the encrypted stream is addressed to. One of three recipient
/// kinds mapped 1-to-1 onto [`age::Recipient`] impls.
#[derive(Debug, Clone)]
pub enum Recipient {
    /// Symmetric passphrase. `age` derives a KEK with scrypt.
    /// Wrapped in `SecretString` so accidental `Debug` / serde
    /// prints won't leak the passphrase.
    Passphrase(SecretString),
    /// An `age1…`-format X25519 recipient, encoded as the
    /// plain-text public-key string. Parsed by
    /// [`age::x25519::Recipient::from_str`] at pipeline build time.
    X25519(String),
    /// An SSH public key — `ssh-ed25519 AAAA…` or
    /// `ssh-rsa AAAA…`. age supports both natively via
    /// [`age::ssh::Recipient`].
    Ssh(String),
}

impl Recipient {
    /// Short tag for UI / telemetry. Not the user-visible label —
    /// Settings resolves that through Fluent.
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::Passphrase(_) => "passphrase",
            Self::X25519(_) => "x25519",
            Self::Ssh(_) => "ssh",
        }
    }
}

/// A complete encryption policy: non-empty list of recipients + the
/// minimum count that must be present for `encrypted_writer` to
/// proceed. Today this field is purely informational / a forward
/// hook — the age format encrypts the file key to each recipient,
/// and any *one* identity can unlock the file. A future n-of-m
/// policy (threshold cryptography) would surface here.
#[derive(Debug, Clone)]
pub struct EncryptionPolicy {
    pub recipients: Vec<Recipient>,
    /// Required recipient count. Today: must equal `recipients.len()`
    /// — all must be present at encrypt time. Leave at zero to
    /// disable the guard and pass through whatever's in
    /// `recipients`.
    pub require_recipient_count: usize,
}

impl EncryptionPolicy {
    /// Build with every recipient required (strict policy; the
    /// encoder will fail fast if the caller attempts to drop one).
    pub fn strict(recipients: Vec<Recipient>) -> Self {
        let n = recipients.len();
        Self {
            recipients,
            require_recipient_count: n,
        }
    }

    /// Convenience — encrypt with a single passphrase.
    pub fn passphrase(pw: SecretString) -> Self {
        Self::strict(vec![Recipient::Passphrase(pw)])
    }
}

/// Compression level — the zstd range is `1..=22` (low numbers are
/// faster, high numbers compress harder). A typed newtype so the
/// UI slider's clamped range stays enforced at the engine seam.
/// Defaults to `3`, which matches zstd's CLI default and the
/// typical "fast but useful" operating point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CompressionLevel(pub i32);

impl CompressionLevel {
    /// The permitted range; callers that receive an unclamped value
    /// (e.g. from a UI number input) should pipe through
    /// [`Self::clamp`].
    pub const MIN: i32 = 1;
    pub const MAX: i32 = 22;
    pub const DEFAULT: Self = Self(3);

    /// Clamp an arbitrary integer into the valid range. Returns the
    /// default when `raw` falls outside sensible bounds.
    pub fn clamp(raw: i32) -> Self {
        Self(raw.clamp(Self::MIN, Self::MAX))
    }

    /// The raw `i32` zstd wants.
    pub const fn as_i32(self) -> i32 {
        self.0
    }
}

impl Default for CompressionLevel {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// Strategy for compressing file contents before they reach disk /
/// cloud / mount. `Off` is the engine default; callers opt into
/// `Always` for "always compress everything" (archive destinations
/// on fast links) or `SmartByExtension` for "compress except
/// already-compressed media".
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "kebab-case")]
pub enum CompressionPolicy {
    /// Pass every byte through untouched. No `.zst` extension append.
    Off,
    /// zstd-compress every file at the given level.
    Always { level: CompressionLevel },
    /// zstd-compress every file whose extension (case-insensitive,
    /// leading dot stripped) is *not* in `deny_extensions`. Typical
    /// deployment: the [`crate::DEFAULT_DENY_EXTENSIONS`] set, which
    /// covers already-compressed media (jpg / mp4 / zip / …).
    SmartByExtension {
        default_level: CompressionLevel,
        deny_extensions: HashSet<String>,
    },
}

impl CompressionPolicy {
    /// Active level for a file, or `None` when the policy elects to
    /// skip compression for the given extension. `file_ext` is
    /// matched case-insensitively with any leading dot stripped.
    pub fn effective_level(&self, file_ext: &str) -> Option<CompressionLevel> {
        match self {
            Self::Off => None,
            Self::Always { level } => Some(*level),
            Self::SmartByExtension {
                default_level,
                deny_extensions,
            } => {
                let normalized = file_ext.trim_start_matches('.').to_ascii_lowercase();
                if normalized.is_empty() || deny_extensions.contains(&normalized) {
                    None
                } else {
                    Some(*default_level)
                }
            }
        }
    }

    /// Convenience constructor for the standard smart policy: the
    /// default level + the default deny-extension set.
    pub fn smart() -> Self {
        Self::SmartByExtension {
            default_level: CompressionLevel::default(),
            deny_extensions: crate::DEFAULT_DENY_EXTENSIONS
                .iter()
                .map(|s| (*s).to_string())
                .collect(),
        }
    }
}

impl Default for CompressionPolicy {
    /// Compression is *off* by default. Turning it on is a deliberate
    /// user-visible toggle in Settings → Transfer.
    fn default() -> Self {
        Self::Off
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compression_level_clamps_to_range() {
        assert_eq!(CompressionLevel::clamp(0).as_i32(), 1);
        assert_eq!(CompressionLevel::clamp(99).as_i32(), 22);
        assert_eq!(CompressionLevel::clamp(5).as_i32(), 5);
    }

    #[test]
    fn smart_policy_denies_configured_extensions() {
        let policy = CompressionPolicy::smart();
        assert!(policy.effective_level("jpg").is_none());
        assert!(policy.effective_level(".JPG").is_none());
        assert!(policy.effective_level("mp4").is_none());
        assert!(policy.effective_level("txt").is_some());
    }

    #[test]
    fn smart_policy_skips_files_without_extension() {
        let policy = CompressionPolicy::smart();
        // No extension → Smart policy has no signal, err on the
        // side of compressing would risk double-compressing
        // archives named `backup` (no suffix). Our rule: an empty
        // extension also skips — only compress when we know the
        // type.
        assert!(policy.effective_level("").is_none());
    }

    #[test]
    fn always_policy_returns_level_unconditionally() {
        let policy = CompressionPolicy::Always {
            level: CompressionLevel(9),
        };
        assert_eq!(policy.effective_level("jpg"), Some(CompressionLevel(9)));
    }

    #[test]
    fn off_policy_never_compresses() {
        assert!(CompressionPolicy::Off.effective_level("txt").is_none());
        assert!(CompressionPolicy::Off.effective_level("bin").is_none());
    }

    #[test]
    fn recipient_kind_matches_variant() {
        let pw = Recipient::Passphrase(SecretString::from("hunter2".to_string()));
        assert_eq!(pw.kind(), "passphrase");
        let x = Recipient::X25519("age1abc".into());
        assert_eq!(x.kind(), "x25519");
        let s = Recipient::Ssh("ssh-ed25519 AAAA...".into());
        assert_eq!(s.kind(), "ssh");
    }
}
