//! Phase 35 — typed errors for the crypt + compress pipeline.

use std::io;
use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CryptError {
    /// Encryption policy has zero recipients. age refuses to encrypt
    /// without at least one, so we bail early with a clear reason.
    #[error("encryption policy has no recipients")]
    NoRecipients,

    /// Failed to parse an X25519 recipient string (`age1…`) or a
    /// loaded SSH key file into an age recipient. The contained
    /// string is a user-legible reason the Settings UI can show.
    #[error("invalid recipient: {0}")]
    InvalidRecipient(String),

    /// Failed to parse an X25519 / SSH identity string (`AGE-SECRET-
    /// KEY-…` or an SSH private key blob). Surfaces in the
    /// decryption path.
    #[error("invalid identity: {0}")]
    InvalidIdentity(String),

    /// `age::Encryptor::wrap_output` or the finishing `finish()` on
    /// the writer returned an error. age's own errors are generic
    /// `io::Error`s at this layer.
    #[error("age encryption failed: {0}")]
    AgeEncrypt(String),

    /// `age::Decryptor::new` or the underlying read returned an
    /// error. Wrong passphrase / mismatched recipient surfaces here
    /// too.
    #[error("age decryption failed: {0}")]
    AgeDecrypt(String),

    /// zstd encoder / decoder error. The upstream library emits
    /// generic messages; we pass them through.
    #[error("zstd {0}")]
    Zstd(String),

    /// IO error at the layer that owns the file handle (not the age
    /// / zstd primitives themselves).
    #[error("io error on {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    /// A passphrase-protected identity file was loaded without a
    /// passphrase — age refuses to decrypt until the UI prompts.
    #[error("identity requires passphrase to unlock")]
    IdentityLocked,
}

pub type Result<T> = std::result::Result<T, CryptError>;

impl From<CryptError> for io::Error {
    /// Engine seam — the pipeline wraps this error into an
    /// `io::Error` so the existing `CopyError::Io` mapping stays
    /// unchanged. The kind is `Other` because none of age's /
    /// zstd's failures map cleanly to a POSIX errno.
    fn from(err: CryptError) -> Self {
        io::Error::other(err.to_string())
    }
}
