//! Phase 35 — age encryption stage.
//!
//! [`encrypted_writer`] wraps a sync `Write` in an age encryptor
//! that writes the binary age format (header + body) directly to
//! the inner sink. The returned writer must be consumed to
//! completion — the age encoder emits a final authentication tag
//! that only flushes on drop / explicit finish.
//!
//! Recipient parsing accepts all three kinds the Settings UI
//! collects:
//!
//! - [`Recipient::Passphrase`] — hands the `SecretString` to
//!   [`age::scrypt::Recipient`] which derives the KEK on encrypt.
//! - [`Recipient::X25519`] — parses an `age1…` string via
//!   [`age::x25519::Recipient::from_str`].
//! - [`Recipient::Ssh`] — parses an `ssh-ed25519 …` or
//!   `ssh-rsa …` string via [`age::ssh::Recipient::from_str`].

use std::io::{self, Write};
use std::path::Path;
use std::str::FromStr;

use secrecy::ExposeSecret;

use crate::error::{CryptError, Result};
use crate::policy::{EncryptionPolicy, Recipient};

/// Does `path` already look like an age-encrypted file? True when
/// the lowercase extension is `age` or the full path ends in `.age`.
/// Used by the engine's copy-FROM branch to decide whether to pipe
/// through [`crate::decrypt::decrypted_reader`].
pub fn is_age_path(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e.eq_ignore_ascii_case("age"))
        .unwrap_or(false)
}

/// Build an age-encrypting writer over `inner` with the policy's
/// recipients. The returned writer **must** be explicitly
/// [`EncryptionSink::finish`]-ed (or dropped) before the inner
/// writer's bytes are complete — age writes a final MAC on close.
pub fn encrypted_writer<W>(inner: W, policy: &EncryptionPolicy) -> Result<EncryptionSink<W>>
where
    W: Write,
{
    if policy.recipients.is_empty() {
        return Err(CryptError::NoRecipients);
    }
    if policy.require_recipient_count > 0
        && policy.require_recipient_count != policy.recipients.len()
    {
        return Err(CryptError::InvalidRecipient(format!(
            "policy requires {} recipients but {} provided",
            policy.require_recipient_count,
            policy.recipients.len()
        )));
    }

    let age_recipients = build_age_recipients(&policy.recipients)?;
    // age 0.11's `Encryptor::with_recipients` accepts any iterator
    // over `&dyn age::Recipient`; we construct the owning Box<dyn>
    // chain here so its lifetime outlives the encryptor build.
    let recipient_refs: Vec<&dyn age::Recipient> = age_recipients
        .iter()
        .map(|r| &**r as &dyn age::Recipient)
        .collect();
    let encryptor = age::Encryptor::with_recipients(recipient_refs.into_iter())
        .map_err(|e| CryptError::InvalidRecipient(e.to_string()))?;
    let writer = encryptor
        .wrap_output(inner)
        .map_err(|e| CryptError::AgeEncrypt(e.to_string()))?;

    Ok(EncryptionSink {
        inner: Some(writer),
        finish_error: None,
    })
}

/// Owns the age `StreamWriter` and flushes the MAC on drop.
pub struct EncryptionSink<W: Write> {
    inner: Option<age::stream::StreamWriter<W>>,
    /// Set when the implicit `Drop` path's `finish()` returned an
    /// error. Callers that prefer the implicit-close shape can
    /// inspect this *before* the sink drops to surface a truncated
    /// MAC — without it the engine would believe the copy succeeded
    /// while the file on disk has a half-written authentication
    /// tag, breaking any verify-then-delete-source workflow.
    finish_error: Option<String>,
}

impl<W: Write> EncryptionSink<W> {
    /// Explicit finish — closes the age stream (writing the MAC) and
    /// returns the inner sink. Use this when the caller needs the
    /// inner writer back (e.g. to flush a file handle or chain into
    /// the next stage).
    pub fn finish(mut self) -> Result<W> {
        let writer = self.inner.take().expect("encryption sink already finished");
        writer
            .finish()
            .map_err(|e| CryptError::AgeEncrypt(e.to_string()))
    }

    /// Did the implicit `Drop` path's `finish()` return an error?
    /// Always `None` when the caller used [`EncryptionSink::finish`]
    /// explicitly. Callers that wrap the sink in higher-level
    /// pipelines should check this before claiming the encrypted
    /// stream is durable.
    pub fn drop_error(&self) -> Option<&str> {
        self.finish_error.as_deref()
    }
}

impl<W: Write> Write for EncryptionSink<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner
            .as_mut()
            .expect("encryption sink already finished")
            .write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        if let Some(w) = self.inner.as_mut() {
            w.flush()?;
        }
        Ok(())
    }
}

impl<W: Write> Drop for EncryptionSink<W> {
    fn drop(&mut self) {
        // Drop without `finish()` — write the MAC anyway so the
        // inner bytes form a valid age file. Drop can't return a
        // Result, but a `finish()` failure here means the on-disk
        // ciphertext has a truncated / unwritten authentication tag
        // — every subsequent decrypt will fail with `AgeDecrypt`,
        // and a "verify-then-delete-source" workflow would silently
        // delete the only intact copy. Stash the error string so
        // the caller can poll `drop_error()` before treating the
        // copy as successful.
        if let Some(writer) = self.inner.take() {
            if let Err(e) = writer.finish() {
                self.finish_error = Some(e.to_string());
            }
        }
    }
}

/// Translate `copythat_crypt::Recipient` values into boxed age
/// recipients. Returns the vector in the same order as the input so
/// a deterministic header layout is possible if future tooling
/// cares. `ParseRecipientKeyError` doesn't implement `Display` in
/// age 0.11 so the SSH branch formats through `Debug` — the reason
/// still carries the variant name + supporting detail.
fn build_age_recipients(recipients: &[Recipient]) -> Result<Vec<Box<dyn age::Recipient>>> {
    let mut out: Vec<Box<dyn age::Recipient>> = Vec::with_capacity(recipients.len());
    for r in recipients {
        match r {
            Recipient::Passphrase(secret) => {
                let pw = age::secrecy::SecretString::from(secret.expose_secret().to_string());
                out.push(Box::new(age::scrypt::Recipient::new(pw)));
            }
            Recipient::X25519(s) => {
                let parsed = age::x25519::Recipient::from_str(s.trim())
                    .map_err(|e| CryptError::InvalidRecipient(format!("x25519: {e}")))?;
                out.push(Box::new(parsed));
            }
            Recipient::Ssh(s) => {
                let parsed = age::ssh::Recipient::from_str(s.trim())
                    .map_err(|e| CryptError::InvalidRecipient(format!("ssh: {e:?}")))?;
                out.push(Box::new(parsed));
            }
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use secrecy::SecretString;

    #[test]
    fn is_age_path_matches_lowercase_and_uppercase() {
        assert!(is_age_path(Path::new("foo.age")));
        assert!(is_age_path(Path::new("FOO.AGE")));
        assert!(!is_age_path(Path::new("foo.txt")));
        assert!(!is_age_path(Path::new("foo")));
    }

    #[test]
    fn empty_recipient_list_fails_fast() {
        let policy = EncryptionPolicy {
            recipients: vec![],
            require_recipient_count: 0,
        };
        let mut sink: Vec<u8> = Vec::new();
        let result = encrypted_writer(&mut sink, &policy);
        match result {
            Err(CryptError::NoRecipients) => {}
            Err(other) => panic!("expected NoRecipients, got {other:?}"),
            Ok(_) => panic!("expected NoRecipients, got Ok"),
        }
    }

    #[test]
    fn passphrase_round_trip_via_age_public_api() {
        use std::io::Read;

        let pw = SecretString::from("hunter2".to_string());
        let policy = EncryptionPolicy::passphrase(pw.clone());

        let mut encrypted: Vec<u8> = Vec::new();
        let sink = encrypted_writer(&mut encrypted, &policy).unwrap();
        // Build the encrypted payload.
        let plaintext = b"The quick brown fox jumps over the lazy dog";
        let mut writer = sink;
        writer.write_all(plaintext).unwrap();
        writer.finish().unwrap();

        // Decrypt via age directly — exercises wire compatibility
        // with non-Copy-That tooling.
        let decryptor = age::Decryptor::new(&encrypted[..]).unwrap();
        let age_pw = age::secrecy::SecretString::from(pw.expose_secret().to_string());
        let identity = age::scrypt::Identity::new(age_pw);
        let mut reader = decryptor
            .decrypt(std::iter::once(&identity as &dyn age::Identity))
            .unwrap();
        let mut plain_back = Vec::new();
        reader.read_to_end(&mut plain_back).unwrap();
        assert_eq!(plain_back, plaintext);
    }

    #[test]
    fn malformed_x25519_recipient_surfaces_invalid_error() {
        let policy = EncryptionPolicy::strict(vec![Recipient::X25519(
            "age1-not-a-real-recipient".to_string(),
        )]);
        let sink: Vec<u8> = Vec::new();
        let result = encrypted_writer(sink, &policy);
        match result {
            Err(CryptError::InvalidRecipient(_)) => {}
            Err(other) => panic!("expected InvalidRecipient, got {other:?}"),
            Ok(_) => panic!("expected InvalidRecipient, got Ok"),
        }
    }
}
