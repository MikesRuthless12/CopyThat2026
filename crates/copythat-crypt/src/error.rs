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
    /// the writer returned an error. The bounded variant carries the
    /// kind only — never the wrapped `io::Error`'s `Display` payload,
    /// which on some platforms / pipeline configurations would
    /// include adapter-side buffers (e.g. zstd's residual frame) that
    /// could leak ciphertext-adjacent bytes if surfaced verbatim into
    /// telemetry.
    #[error("age encryption failed: {0}")]
    AgeEncrypt(CryptFinishError),

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

/// Bounded reason for an age-side encryption failure. Each variant is
/// a finite shape — by construction the `Display` impl can never
/// echo bytes from the in-flight plaintext or its surrounding paths.
/// The pipeline's age adapter raised IO errors verbatim before this
/// type existed; surfacing those `io::Error::Display` strings risked
/// leaking adapter-side buffer contents (zstd's residual frame, the
/// inner writer's path-aware error message) into telemetry / audit
/// trails. The variants below carry only enum tags + an `io::ErrorKind`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum CryptFinishError {
    /// `age::stream::StreamWriter::wrap_output` failed — the recipient
    /// chain rejected the build before any plaintext flowed.
    #[error("wrap-output rejected the recipient chain")]
    WrapOutput,
    /// The age stream's finalising `finish()` (which writes the MAC)
    /// returned an error. The contained `io::ErrorKind` is the only
    /// information we propagate — never the inner `io::Error::Display`
    /// string, which can include adapter-side payload buffers.
    #[error("MAC finalisation failed (io kind: {0:?})")]
    Finalise(io::ErrorKind),
    /// The sink was dropped without a successful explicit `.finish()`.
    /// Surfaced when an outer pipeline forced an early drop without
    /// ever calling `finish()`; in well-formed code this never
    /// reaches a caller because [`crate::encrypt::EncryptionSink`]'s
    /// Drop panics first.
    #[error("encryption sink dropped without explicit finish")]
    DropWithoutFinish,
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
