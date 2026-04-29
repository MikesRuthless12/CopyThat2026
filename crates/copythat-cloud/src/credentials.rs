//! Phase 32 — OS keychain wrapper for backend secrets.
//!
//! Each backend's secret bundle is stored under the service name
//! `copythat-cloud/<backend-name>` with a fixed account label
//! ([`CREDENTIAL_USER`]). Backends that need multiple separate
//! secrets (e.g., S3's access key + secret access key) pack them into
//! a single newline-delimited blob the operator builder splits at
//! load time — keeps the keychain entry count one-per-backend.

use std::sync::Mutex;

use thiserror::Error;

/// Account label used inside the keychain entry. Matches the value
/// the Add-backend wizard writes; changing it would orphan stored
/// credentials, so it lives in code as a constant.
pub const CREDENTIAL_USER: &str = "copythat-cloud";

/// Service name prefix the backend's name slot is appended to. The
/// final keychain key is `copythat-cloud/<backend-name>`.
pub const SERVICE_PREFIX: &str = "copythat-cloud";

#[derive(Debug, Error)]
pub enum CredentialsError {
    #[error("keychain rejected the operation: {0}")]
    Keyring(#[from] keyring::Error),

    #[error("backend name `{0}` is empty or contains an illegal character")]
    InvalidBackendName(String),

    /// The supplied secret blob was empty. Empty secrets are rejected
    /// at store-time to prevent silently writing a sentinel that
    /// would later be indistinguishable from a missing-entry case.
    #[error("refusing to store an empty secret for backend `{0}`")]
    EmptySecret(String),
}

/// Stateless façade — every method opens a fresh `keyring::Entry`
/// keyed on the backend name so the registry can be re-entered
/// from multiple threads.
///
/// **Phase 32h TOCTOU hardening:** the `keyring` crate's `Entry`
/// is not internally synchronised; back-to-back `store` /
/// `load` / `delete` calls from different threads can interleave
/// at the OS-keystore level, so a load that races a concurrent
/// store can return either the previous or the new value
/// non-deterministically (and on macOS, can spuriously surface
/// `NoEntry` between the underlying SecItem operations). We
/// serialise all access through a process-wide [`Mutex`] —
/// minimal contention because credential reads happen only at
/// operator-build time, but enough to give callers a coherent
/// view. The mutex covers the whole `entry()` + read/write pair
/// so the critical section matches what an atomic keystore op
/// would have given us.
#[derive(Debug, Default)]
pub struct Credentials;

/// Process-wide keyring serialisation lock. Held across each of
/// the four public operations (`store` / `load` / `delete` +
/// `entry()` builds). Poison recovery: an unwrap on a poisoned
/// mutex would fail loudly, but we'd rather degrade to "treat as
/// busy" than panic — the `lock_keyring` helper recovers the
/// guard via `into_inner` so a previously-panicking thread
/// doesn't permanently brick credential access.
static KEYRING_LOCK: Mutex<()> = Mutex::new(());

fn lock_keyring() -> std::sync::MutexGuard<'static, ()> {
    match KEYRING_LOCK.lock() {
        Ok(g) => g,
        Err(poisoned) => poisoned.into_inner(),
    }
}

impl Credentials {
    /// Persist the secret blob for `backend_name` in the OS
    /// keychain. Overwrites any existing entry under the same name.
    /// Empty `secret` values are rejected with
    /// [`CredentialsError::EmptySecret`] to prevent storing a
    /// sentinel that's indistinguishable from a missing entry on
    /// the next read (which could flip a configured backend into
    /// "anonymous" mode silently).
    pub fn store(&self, backend_name: &str, secret: &str) -> Result<(), CredentialsError> {
        if secret.is_empty() {
            return Err(CredentialsError::EmptySecret(backend_name.to_owned()));
        }
        let _guard = lock_keyring();
        let entry = self.entry(backend_name)?;
        entry.set_password(secret)?;
        Ok(())
    }

    /// Read the secret blob for `backend_name`. Returns
    /// `Ok(None)` when no entry exists; raises [`CredentialsError`]
    /// for everything else. The `entry()` build + read live inside
    /// the same critical section so a concurrent `store` /
    /// `delete` cannot race between the two halves.
    pub fn load(&self, backend_name: &str) -> Result<Option<String>, CredentialsError> {
        let _guard = lock_keyring();
        let entry = self.entry(backend_name)?;
        match entry.get_password() {
            Ok(p) => Ok(Some(p)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(CredentialsError::Keyring(e)),
        }
    }

    /// Remove the secret entry for `backend_name`. No-op if there
    /// was no entry to begin with.
    pub fn delete(&self, backend_name: &str) -> Result<(), CredentialsError> {
        let _guard = lock_keyring();
        let entry = self.entry(backend_name)?;
        match entry.delete_credential() {
            Ok(()) => Ok(()),
            Err(keyring::Error::NoEntry) => Ok(()),
            Err(e) => Err(CredentialsError::Keyring(e)),
        }
    }

    /// Compose the per-backend service string + open the entry.
    /// Rejects backend names that are empty or contain any byte
    /// outside `[A-Za-z0-9_-]`. The tighter allowlist (vs. the
    /// previous `/` + NUL exclusion) defends against names like
    /// `..` / `.` / `name with spaces` / unicode lookalikes that
    /// would otherwise round-trip through the `<prefix>/<name>`
    /// service-string join unchecked. Per-OS keychain
    /// implementations may further restrict allowed bytes — the
    /// allowlist matches the most conservative set
    /// (Windows Credential Manager treats `/` as a separator,
    /// macOS Keychain accepts most printable ASCII, Linux
    /// Secret Service is a free-form string) so the same name
    /// is portable across platforms.
    fn entry(&self, backend_name: &str) -> Result<keyring::Entry, CredentialsError> {
        if backend_name.is_empty() {
            return Err(CredentialsError::InvalidBackendName(
                backend_name.to_owned(),
            ));
        }
        if !backend_name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
        {
            return Err(CredentialsError::InvalidBackendName(
                backend_name.to_owned(),
            ));
        }
        let service = format!("{SERVICE_PREFIX}/{backend_name}");
        Ok(keyring::Entry::new(&service, CREDENTIAL_USER)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_or_illegal_name() {
        let creds = Credentials;
        assert!(matches!(
            creds.entry("").unwrap_err(),
            CredentialsError::InvalidBackendName(_)
        ));
        assert!(matches!(
            creds.entry("a/b").unwrap_err(),
            CredentialsError::InvalidBackendName(_)
        ));
        assert!(matches!(
            creds.entry("a\0b").unwrap_err(),
            CredentialsError::InvalidBackendName(_)
        ));
        // Phase 32h — `..` / `.` / spaces / dots / colons all
        // refused under the tighter `[A-Za-z0-9_-]` allowlist.
        for bad in [
            "..", ".", "a.b", "a b", "a:b", "a@b", "a\\b", "ünicode", "name?",
        ] {
            assert!(
                matches!(
                    creds.entry(bad).unwrap_err(),
                    CredentialsError::InvalidBackendName(_)
                ),
                "expected `{bad}` to be rejected"
            );
        }
    }

    #[test]
    fn entry_uses_prefixed_service_name() {
        let creds = Credentials;
        // Constructing the entry doesn't touch the live keychain; we
        // can sanity-check the path without depending on a daemon.
        let entry = creds.entry("my-bucket");
        assert!(entry.is_ok(), "entry build should succeed for legal names");
    }

    #[test]
    fn allowlist_accepts_alphanumeric_underscore_hyphen() {
        let creds = Credentials;
        for ok in ["abc", "ABC123", "alpha_beta", "alpha-beta", "X9_y-Z"] {
            assert!(
                creds.entry(ok).is_ok(),
                "expected `{ok}` to pass the allowlist"
            );
        }
    }

    #[test]
    fn store_rejects_empty_secret() {
        let creds = Credentials;
        // Skip touching the real keychain by relying on the empty-
        // secret short-circuit at the top of `store` — the guard
        // returns before any keyring call. (Some CI runners don't
        // have a Secret Service / Credential Manager available.)
        let err = creds.store("legal-name", "").unwrap_err();
        assert!(
            matches!(err, CredentialsError::EmptySecret(_)),
            "expected EmptySecret, got {err:?}"
        );
    }
}
