//! Phase 35 — destination encryption + on-the-fly compression.
//!
//! # Scope
//!
//! Two independent-but-composable pipeline stages the engine can
//! chain before the final write lands on disk / cloud / mount:
//!
//! - [`encrypt::encrypted_writer`] — wraps an inner `Write` in the
//!   age format (X25519 / passphrase / SSH recipients;
//!   ChaCha20-Poly1305 body, scrypt-derived KEK for passphrases).
//! - [`compress::compressed_writer`] — wraps an inner `Write` in
//!   zstd, either unconditionally or driven by a per-extension
//!   deny heuristic that matches what a Copy That user actually
//!   wants (skip `.jpg`, `.mp4`, `.zip`, … because they're already
//!   compressed).
//!
//! Both helpers are sync-Write-first by design. The `age` crate's
//! async wrappers pull an extra dep graph (`futures-util` etc.)
//! and its own tokio shims; we'd rather the engine bridge a
//! sync stage via `spawn_blocking` at the pipeline seam. Phase 35's
//! engine integration does exactly that — one blocking task per
//! encrypted / compressed file, chain-hashed into the existing
//! progress + journal callbacks.
//!
//! # CRIME side-channel — compress-before-encrypt
//!
//! When both stages run, the pipeline order is
//! `src → compress → encrypt → file`. zstd compression therefore
//! happens *inside* the age encryption boundary. This is the same
//! arrangement TLS used until 2012, and it inherits the same risk:
//! when the plaintext is partially attacker-influenced (HTTP
//! cookies, structured records mixing user input with secrets),
//! observed ciphertext length leaks information about plaintext
//! repetition even though the bytes themselves stay confidential.
//! The [original CRIME paper](https://en.wikipedia.org/wiki/CRIME)
//! describes the technique in detail.
//!
//! Most Copy That deployments are not vulnerable — the plaintext is
//! a whole file the user owns end-to-end, not a mix of attacker-
//! supplied and secret bytes. We keep `compress-then-encrypt` as
//! the default for the compression-effectiveness win on backups +
//! archive workloads. Security-sensitive deployments that *do*
//! handle partially-attacker-influenced plaintext should set
//! [`EncryptionPolicy::allow_compression_before_encrypt`] to
//! `false`, which forces the pipeline to skip compression for any
//! file whose destination is encrypted (regardless of what the
//! [`CompressionPolicy`] asks for). The default stays `true` to
//! preserve historical behaviour.
//!
//! # Wire compatibility
//!
//! The on-disk format written by [`encrypt::encrypted_writer`] is
//! bit-for-bit identical to `rage -r <recipient> <file>` or
//! `rage -p <file>`; decryption with the matching `.age` identity
//! file from <https://age-encryption.org> or the `rage` CLI
//! round-trips without Copy That in the loop. Compressed files are
//! plain `.zst` streams with the magic number `0xFD2FB528` — any
//! zstd decoder round-trips.
//!
//! # What this crate is not
//!
//! - Not a key-management service. Generating / storing / rotating
//!   X25519 + SSH keys happens outside this crate (the Settings UI
//!   pickers read files; [`decrypt::Identity`] wraps what's been
//!   loaded).
//! - Not a secure-delete primitive. Shredding a source after
//!   encryption is a `copythat-secure-delete` concern the engine
//!   wires up separately.
//! - Not a format-agnostic abstraction. We ship age + zstd
//!   specifically; a future swap would be a major version bump.

#![forbid(unsafe_code)]

pub mod compress;
pub mod decrypt;
pub mod encrypt;
pub mod error;
pub mod policy;
pub mod sink;

pub use compress::{
    CompressionMetrics, CompressionSinkStats, DEFAULT_DENY_EXTENSIONS, compressed_writer,
    should_compress,
};
pub use decrypt::{Identity, decrypted_reader, load_identities_from};
pub use encrypt::{encrypted_writer, is_age_path};
pub use error::{CryptError, Result};
pub use policy::{CompressionLevel, CompressionPolicy, EncryptionPolicy, Recipient};
pub use sink::{CopyThatCryptHook, TransformPlan, dst_is_age};
