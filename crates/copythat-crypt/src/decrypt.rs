//! Phase 35 — age decryption stage.
//!
//! The engine's copy-FROM branch (when the source is a `.age` file
//! and the user has loaded identities) pipes the source reader
//! through [`decrypted_reader`] before the bytes hit the destination
//! writer.
//!
//! [`Identity`] wraps whatever identities the caller has loaded —
//! passphrase(s), X25519 private keys, SSH private keys. Each of
//! these maps onto one `age::Identity` impl.

use std::io::{self, Read};
use std::path::Path;
use std::str::FromStr;

use secrecy::{ExposeSecret, SecretString};

use crate::error::{CryptError, Result};

/// One or more unlockers the decryption pipeline will try in order
/// against an encrypted stream. age's internal matching picks the
/// identity that successfully unwraps the file key.
#[derive(Default)]
pub struct Identity {
    entries: Vec<Box<dyn age::Identity + Send + Sync>>,
}

impl Identity {
    /// Fresh, empty bag. Chain `add_*` calls to populate.
    pub fn new() -> Self {
        Self::default()
    }

    /// Append a passphrase. Uses age's scrypt identity which matches
    /// [`crate::Recipient::Passphrase`] at encrypt time.
    pub fn with_passphrase(mut self, pw: SecretString) -> Self {
        self.add_passphrase(pw);
        self
    }

    /// Same as [`Self::with_passphrase`] but mutates in place —
    /// the looper in [`load_identities_from`] prefers the &mut form.
    pub fn add_passphrase(&mut self, pw: SecretString) {
        let age_pw = age::secrecy::SecretString::from(pw.expose_secret().to_string());
        self.entries
            .push(Box::new(age::scrypt::Identity::new(age_pw)));
    }

    /// Append an X25519 identity from an `AGE-SECRET-KEY-1…` string.
    pub fn with_x25519(mut self, secret: &str) -> Result<Self> {
        self.add_x25519(secret)?;
        Ok(self)
    }

    /// &mut-form of [`Self::with_x25519`].
    pub fn add_x25519(&mut self, secret: &str) -> Result<()> {
        let parsed = age::x25519::Identity::from_str(secret.trim())
            .map_err(|e| CryptError::InvalidIdentity(format!("x25519: {e}")))?;
        self.entries.push(Box::new(parsed));
        Ok(())
    }

    /// Append an SSH identity from a raw OpenSSH private-key blob
    /// (plain-text PEM). Encrypted SSH keys surface
    /// [`CryptError::IdentityLocked`] — Copy That's Settings UI
    /// prompts the user for the passphrase + decrypts to an
    /// unencrypted blob before calling this helper. A future
    /// revision may plumb a passphrase callback through age's
    /// `EncryptedKey::decrypt` API directly.
    pub fn with_ssh(mut self, pem: &str, passphrase: Option<SecretString>) -> Result<Self> {
        self.add_ssh(pem, passphrase)?;
        Ok(self)
    }

    /// &mut-form of [`Self::with_ssh`].
    pub fn add_ssh(&mut self, pem: &str, _passphrase: Option<SecretString>) -> Result<()> {
        let identity = age::ssh::Identity::from_buffer(pem.as_bytes(), None)
            .map_err(|e| CryptError::InvalidIdentity(format!("ssh: {e}")))?;
        match identity {
            age::ssh::Identity::Unencrypted(_) => {
                self.entries.push(Box::new(identity));
                Ok(())
            }
            age::ssh::Identity::Encrypted(_) => Err(CryptError::IdentityLocked),
            age::ssh::Identity::Unsupported(_) => Err(CryptError::InvalidIdentity(
                "SSH key type not supported by age".into(),
            )),
        }
    }

    /// Number of stored identities.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Iterate over the stored identities as age trait objects.
    fn iter_age(&self) -> impl Iterator<Item = &(dyn age::Identity + Send + Sync)> {
        self.entries.iter().map(|b| b.as_ref())
    }
}

/// Wrap an age-encrypted source reader in a decrypting reader. The
/// caller must pass the inner reader positioned at the start of the
/// age file (not seeked past the header).
///
/// Returns a boxed trait object because age's concrete
/// `StreamReader<R>` parametrises on `R` and we want the engine's
/// pipeline generic in the reader it reads from.
pub fn decrypted_reader<R>(inner: R, identity: &Identity) -> Result<Box<dyn Read + '_>>
where
    R: Read + 'static,
{
    if identity.is_empty() {
        return Err(CryptError::InvalidIdentity("no identities loaded".into()));
    }
    let decryptor =
        age::Decryptor::new(inner).map_err(|e| CryptError::AgeDecrypt(e.to_string()))?;
    let reader = decryptor
        .decrypt(identity.iter_age().map(|r| r as &dyn age::Identity))
        .map_err(|e| CryptError::AgeDecrypt(e.to_string()))?;
    Ok(Box::new(reader))
}

/// Walk a directory and load every `*.age`-format identity file
/// into a fresh [`Identity`]. Convenience for the Settings UI's
/// *Import keys from folder* action. Passphrase-protected identity
/// files surface as [`CryptError::IdentityLocked`] — the caller
/// prompts the user and calls [`Identity::with_ssh`] /
/// [`Identity::with_x25519`] explicitly for those.
pub fn load_identities_from(dir: &Path) -> io::Result<Identity> {
    /// Per-file size cap for the identity loader. age secret keys
    /// are ~62 chars; SSH private keys peak around ~14 KiB for
    /// 4096-bit RSA in PEM. 64 KiB gives generous headroom while
    /// keeping a malicious `~/Downloads`-pointed import from
    /// reading huge attacker files into memory.
    const MAX_IDENTITY_FILE_BYTES: u64 = 64 * 1024;
    /// Allowed identity-file extensions. Anything else is skipped
    /// silently — pointing the picker at a generic dir like
    /// `~/Downloads` would otherwise silently slurp every `*.txt`
    /// the user has, including potentially attacker-planted
    /// secrets that happen to parse as X25519 / SSH keys.
    const IDENTITY_EXTENSIONS: &[&str] = &["age", "key", "txt", "pem", "id_ed25519", "id_rsa"];

    let mut id = Identity::new();
    let entries = std::fs::read_dir(dir)?;
    for ent in entries {
        let ent = ent?;
        if !ent.file_type()?.is_file() {
            continue;
        }
        let path = ent.path();
        let ext_ok = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| {
                let lower = e.to_ascii_lowercase();
                IDENTITY_EXTENSIONS.iter().any(|w| *w == lower)
            })
            .unwrap_or_else(|| {
                // Files with no extension (e.g. `id_ed25519`) match by
                // their full file_name when it's in the allowlist.
                path.file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| {
                        let lower = n.to_ascii_lowercase();
                        IDENTITY_EXTENSIONS.iter().any(|w| *w == lower)
                    })
                    .unwrap_or(false)
            });
        if !ext_ok {
            continue;
        }
        let md = ent.metadata()?;
        if md.len() > MAX_IDENTITY_FILE_BYTES {
            continue;
        }
        let raw = std::fs::read_to_string(&path)?;
        // Try each parse order: X25519 secret key string, then SSH
        // private key. Failures are non-fatal — we only want to
        // ingest recognisable identities.
        if id.add_x25519(raw.trim()).is_ok() {
            continue;
        }
        let _ = id.add_ssh(raw.trim(), None);
    }
    Ok(id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encrypt::encrypted_writer;
    use crate::policy::EncryptionPolicy;

    #[test]
    fn empty_identity_refuses_to_decrypt() {
        let encrypted = b"not-actually-age-bytes";
        let identity = Identity::new();
        let result = decrypted_reader(&encrypted[..], &identity);
        match result {
            Err(CryptError::InvalidIdentity(_)) => {}
            Err(other) => panic!("expected InvalidIdentity, got {other:?}"),
            Ok(_) => panic!("expected InvalidIdentity, got Ok"),
        }
    }

    #[test]
    fn passphrase_encrypt_then_decrypt_round_trips() {
        use std::io::Write;

        let pw = SecretString::from("correct horse battery staple".to_string());
        let policy = EncryptionPolicy::passphrase(pw.clone());

        let mut buf: Vec<u8> = Vec::new();
        let mut sink = encrypted_writer(&mut buf, &policy).unwrap();
        let plaintext = b"Phase 35 smoke payload";
        sink.write_all(plaintext).unwrap();
        sink.finish().unwrap();

        let identity = Identity::new().with_passphrase(pw);
        let mut reader = decrypted_reader(std::io::Cursor::new(buf), &identity).unwrap();
        let mut round_trip = Vec::new();
        reader.read_to_end(&mut round_trip).unwrap();
        assert_eq!(round_trip, plaintext);
    }

    #[test]
    fn wrong_passphrase_fails_decryption() {
        use std::io::Write;

        let correct = SecretString::from("correct".to_string());
        let wrong = SecretString::from("wrong".to_string());

        let mut buf: Vec<u8> = Vec::new();
        let mut sink = encrypted_writer(&mut buf, &EncryptionPolicy::passphrase(correct)).unwrap();
        sink.write_all(b"secret").unwrap();
        sink.finish().unwrap();

        let bad_identity = Identity::new().with_passphrase(wrong);
        let result = decrypted_reader(std::io::Cursor::new(buf), &bad_identity);
        match result {
            Err(CryptError::AgeDecrypt(_)) => {}
            Err(other) => panic!("expected AgeDecrypt, got {other:?}"),
            Ok(_) => panic!("expected AgeDecrypt, got Ok"),
        }
    }
}
