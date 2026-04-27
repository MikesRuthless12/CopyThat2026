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

use crate::error::{CryptError, CryptFinishError, Result};
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
/// [`EncryptionSink::finish`]-ed before the inner writer's bytes are
/// complete — age writes a final MAC on close. Dropping a sink
/// without calling [`EncryptionSink::finish`] (after any successful
/// write) panics: the on-disk ciphertext would be a truncated /
/// unauthenticated age stream and any verify-then-delete-source
/// workflow would silently delete the only intact copy.
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
        .map_err(|_| CryptError::AgeEncrypt(CryptFinishError::WrapOutput))?;

    Ok(EncryptionSink {
        inner: Some(writer),
    })
}

/// Owns the age `StreamWriter` and writes the MAC when explicitly
/// finished. The sink intentionally does **not** swallow errors on
/// drop — its [`Drop`] impl panics when the sink is dropped without
/// a successful [`EncryptionSink::finish`]. Earlier revisions
/// stashed the drop-time finalise error in an `Option<io::Error>`
/// the caller had to poll, but `?`-using call sites silently treated
/// a missing MAC as success — a verify-then-delete-source workflow
/// could then erase the source while the ciphertext on disk was
/// half-written. The explicit-finish contract makes that bug a
/// compile-time / panic-time signal.
pub struct EncryptionSink<W: Write> {
    inner: Option<age::stream::StreamWriter<W>>,
}

impl<W: Write> EncryptionSink<W> {
    /// Explicit finish — closes the age stream (writing the MAC) and
    /// returns the inner sink. **Must** be called on every sink
    /// before it goes out of scope; dropping without a successful
    /// finish panics in [`Drop`]. Returns the inner writer so the
    /// caller can chain into the next pipeline stage (e.g. flush a
    /// file handle, hand it back to the runner for the next file).
    pub fn finish(mut self) -> Result<W> {
        let writer = self
            .inner
            .take()
            .expect("encryption sink already finished");
        // Translate the age `io::Error` to our bounded enum: only the
        // `ErrorKind` propagates — never the verbatim `Display` of
        // the inner error, which on some pipeline configurations can
        // include adapter-side buffers (zstd residual frame, inner
        // writer path-aware messages) that risk leaking
        // ciphertext-adjacent bytes into telemetry.
        writer
            .finish()
            .map_err(|e| CryptError::AgeEncrypt(CryptFinishError::Finalise(e.kind())))
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
        // The contract is: every `EncryptionSink` must be consumed
        // by `finish()` before going out of scope. If `inner` is
        // still `Some` here, the caller forgot — the on-disk
        // ciphertext would be a truncated / unauthenticated age
        // stream, and any verify-then-delete-source workflow would
        // silently delete the only intact copy. Panic loudly so the
        // bug is caught in dev / CI rather than at restore time.
        if self.inner.is_some() {
            // During a panic unwind we can be the second panic if
            // the caller dropped us *because* of an earlier panic.
            // Suppress the secondary panic — leaving the file
            // truncated in that case is the lesser evil; the
            // primary panic surfaces the original failure.
            if std::thread::panicking() {
                return;
            }
            panic!(
                "EncryptionSink dropped without calling .finish() — \
                 the age MAC was never written and the on-disk \
                 ciphertext is truncated. Call EncryptionSink::finish() \
                 before letting the sink go out of scope."
            );
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
            allow_compression_before_encrypt: true,
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
    fn explicit_finish_returns_writer() {
        let pw = SecretString::from("finish-test".to_string());
        let policy = EncryptionPolicy::passphrase(pw);
        let mut buf: Vec<u8> = Vec::new();
        let mut sink = encrypted_writer(&mut buf, &policy).unwrap();
        sink.write_all(b"payload").unwrap();
        // finish() must consume the sink and return the inner writer
        // without panicking on drop.
        let _writer = sink.finish().expect("explicit finish");
        // Buffer now holds a complete age stream.
        assert!(!buf.is_empty());
    }

    #[test]
    #[should_panic(expected = "EncryptionSink dropped without calling .finish()")]
    fn drop_without_finish_panics_to_signal_truncated_mac() {
        // Build a sink, write a byte, then drop it without
        // calling finish(). The new contract says this is a bug —
        // the on-disk file would have a truncated age MAC and any
        // verify-then-delete-source workflow would silently delete
        // the only intact copy. Panic at drop-time so the bug is
        // caught early.
        let pw = SecretString::from("drop-test".to_string());
        let policy = EncryptionPolicy::passphrase(pw);
        let buf: Vec<u8> = Vec::new();
        let mut sink = encrypted_writer(buf, &policy).unwrap();
        sink.write_all(b"oops").unwrap();
        // Sink dropped here at end of scope without finish() — must
        // panic.
        drop(sink);
    }

    #[test]
    fn finish_error_carries_bounded_kind_only() {
        // The CryptFinishError variants are a bounded shape — the
        // Display impl must never include verbatim io::Error
        // messages or path strings. Smoke-check the Display surface.
        use crate::error::CryptFinishError;
        let e = CryptFinishError::Finalise(io::ErrorKind::BrokenPipe);
        let rendered = format!("{e}");
        assert!(rendered.contains("BrokenPipe"));
        assert!(!rendered.contains("/"));
        let wrap = CryptFinishError::WrapOutput;
        assert_eq!(format!("{wrap}"), "wrap-output rejected the recipient chain");
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

    /// Phase 42 wave-2 regression — pin the wave-1 critical fix that
    /// makes dropping an [`EncryptionSink`] without explicit `finish()`
    /// a hard panic. Earlier revisions stashed the drop-time finalise
    /// error in an `Option<io::Error>` the caller had to poll, but
    /// `?`-using call sites silently treated a missing MAC as success
    /// — a verify-then-delete-source workflow could then erase the
    /// source while the ciphertext on disk was half-written.
    ///
    /// `#[should_panic]` (above) covers the unwound-panic-message
    /// surface; this test uses `catch_unwind` so we can additionally
    /// assert the panic payload's structure WITHOUT relying on the
    /// panic-message-substring matcher. That keeps the regression
    /// signal robust if the panic message ever gets reformatted.
    #[test]
    fn drop_without_finish_panics_caught_via_catch_unwind() {
        use std::panic;

        let pw = SecretString::from("catch-unwind-test".to_string());
        let policy = EncryptionPolicy::passphrase(pw);
        // `catch_unwind` requires `UnwindSafe` for closures that move
        // captured state; the closure builds the sink internally so we
        // don't capture across the unwind boundary.
        let result = panic::catch_unwind(|| {
            let buf: Vec<u8> = Vec::new();
            let mut sink = encrypted_writer(buf, &policy).expect("build sink");
            sink.write_all(b"some-bytes").expect("write to sink");
            // Deliberately drop without `finish()` — must panic.
            drop(sink);
        });
        let payload = result.expect_err("Drop without finish() must panic");
        // Confirm the payload is the &str / String produced by the
        // explicit `panic!` macro in `Drop` and that it carries the
        // documented contract message (substring check).
        let msg = payload
            .downcast_ref::<String>()
            .map(String::as_str)
            .or_else(|| payload.downcast_ref::<&'static str>().copied())
            .unwrap_or("");
        assert!(
            msg.contains("EncryptionSink dropped without calling .finish()"),
            "unexpected panic payload: {msg:?}"
        );
    }

    /// Phase 42 wave-2 — Drop suppresses its own panic when the
    /// thread is already unwinding. The contract preserves the
    /// primary panic and avoids a double-panic abort. Without this
    /// guard, a write error inside the user's pipeline + the sink's
    /// drop-time panic would produce a confusing abort instead of
    /// surfacing the original failure.
    #[test]
    fn drop_suppresses_secondary_panic_during_unwind() {
        use std::panic;

        let pw = SecretString::from("nested-panic".to_string());
        let policy = EncryptionPolicy::passphrase(pw);
        let result = panic::catch_unwind(|| {
            let buf: Vec<u8> = Vec::new();
            let mut sink = encrypted_writer(buf, &policy).expect("build sink");
            sink.write_all(b"payload").unwrap();
            // First panic — the sink will be dropped during the
            // unwind. The Drop impl checks `thread::panicking()` and
            // suppresses its own panic so the primary signal makes
            // it out cleanly.
            panic!("primary panic");
        });
        let payload = result.expect_err("primary panic must propagate");
        let msg = payload
            .downcast_ref::<String>()
            .map(String::as_str)
            .or_else(|| payload.downcast_ref::<&'static str>().copied())
            .unwrap_or("");
        assert!(
            msg.contains("primary panic"),
            "the primary panic must dominate; got: {msg:?}"
        );
        assert!(
            !msg.contains("EncryptionSink dropped"),
            "the drop-time panic must be suppressed when already unwinding"
        );
    }
}
