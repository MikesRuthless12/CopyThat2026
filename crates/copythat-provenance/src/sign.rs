//! Ed25519 detached signing helpers.
//!
//! `ed25519-dalek` 2.x is the underlying crate; we re-export
//! [`SigningKey`] / [`VerifyingKey`] under our own names so the
//! Tauri command surface and the CLI import the public API without
//! pulling `ed25519_dalek` into their import lists.
//!
//! Public-key distribution is out-of-band by design (the user's
//! responsibility). We provide PKCS#8 PEM import/export so the
//! Settings UI can stash keys in the OS keyring or hand them to a
//! file picker; raw 32-byte / 64-byte access is also exposed for
//! callers who already have the bytes.

use ed25519_dalek::pkcs8::DecodePrivateKey;
use ed25519_dalek::pkcs8::DecodePublicKey;
use ed25519_dalek::pkcs8::EncodePrivateKey;
use ed25519_dalek::pkcs8::EncodePublicKey;
use ed25519_dalek::pkcs8::spki::der::pem::LineEnding;

pub use ed25519_dalek::SigningKey;
pub use ed25519_dalek::VerifyingKey;

use crate::error::ProvenanceError;

/// Generate a fresh ed25519 signing key using the OS RNG.
pub fn generate_signing_key() -> SigningKey {
    let mut csprng = rand::rngs::OsRng;
    SigningKey::generate(&mut csprng)
}

/// Serialise a [`SigningKey`] to PKCS#8 PEM. The output is a
/// standard `-----BEGIN PRIVATE KEY-----` block consumable by any
/// PKCS#8-aware tool (openssl, Vault, the OS keyring).
pub fn signing_key_to_pem(key: &SigningKey) -> Result<String, ProvenanceError> {
    let pem = key
        .to_pkcs8_pem(LineEnding::LF)
        .map_err(|e| ProvenanceError::Ed25519Key(format!("pkcs8 encode failed: {e}")))?;
    Ok(pem.to_string())
}

/// Parse a PKCS#8 PEM string into a [`SigningKey`].
pub fn signing_key_from_pem(pem: &str) -> Result<SigningKey, ProvenanceError> {
    SigningKey::from_pkcs8_pem(pem)
        .map_err(|e| ProvenanceError::Ed25519Key(format!("pkcs8 decode failed: {e}")))
}

/// Serialise a [`VerifyingKey`] to SubjectPublicKeyInfo PEM.
pub fn verifying_key_to_pem(key: &VerifyingKey) -> Result<String, ProvenanceError> {
    let pem = key
        .to_public_key_pem(LineEnding::LF)
        .map_err(|e| ProvenanceError::Ed25519Key(format!("spki encode failed: {e}")))?;
    Ok(pem)
}

/// Parse a SubjectPublicKeyInfo PEM string into a [`VerifyingKey`].
pub fn verifying_key_from_pem(pem: &str) -> Result<VerifyingKey, ProvenanceError> {
    VerifyingKey::from_public_key_pem(pem)
        .map_err(|e| ProvenanceError::Ed25519Key(format!("spki decode failed: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signing_key_roundtrips_through_pem() {
        let key = generate_signing_key();
        let pem = signing_key_to_pem(&key).unwrap();
        let parsed = signing_key_from_pem(&pem).unwrap();
        assert_eq!(parsed.to_bytes(), key.to_bytes());
    }

    #[test]
    fn verifying_key_roundtrips_through_pem() {
        let sk = generate_signing_key();
        let vk = sk.verifying_key();
        let pem = verifying_key_to_pem(&vk).unwrap();
        let parsed = verifying_key_from_pem(&pem).unwrap();
        assert_eq!(parsed.to_bytes(), vk.to_bytes());
    }

    #[test]
    fn signing_key_from_pem_rejects_garbage() {
        let err = signing_key_from_pem("-----BEGIN GARBAGE-----\nAAAA\n-----END GARBAGE-----\n")
            .unwrap_err();
        assert!(matches!(err, ProvenanceError::Ed25519Key(_)));
    }
}
