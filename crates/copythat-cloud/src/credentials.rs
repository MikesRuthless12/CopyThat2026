//! Phase 32 — OS keychain wrapper for backend secrets.
//!
//! Each backend's secret bundle is stored under the service name
//! `copythat-cloud/<backend-name>` with a fixed account label
//! ([`CREDENTIAL_USER`]). Backends that need multiple separate
//! secrets (e.g., S3's access key + secret access key) pack them into
//! a single newline-delimited blob the operator builder splits at
//! load time — keeps the keychain entry count one-per-backend.

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
}

/// Stateless façade — every method opens a fresh `keyring::Entry`
/// keyed on the backend name so the registry can be re-entered
/// from multiple threads.
#[derive(Debug, Default, Clone, Copy)]
pub struct Credentials;

impl Credentials {
    /// Persist the secret blob for `backend_name` in the OS
    /// keychain. Overwrites any existing entry under the same name.
    pub fn store(&self, backend_name: &str, secret: &str) -> Result<(), CredentialsError> {
        let entry = self.entry(backend_name)?;
        entry.set_password(secret)?;
        Ok(())
    }

    /// Read the secret blob for `backend_name`. Returns
    /// `Ok(None)` when no entry exists; raises [`CredentialsError`]
    /// for everything else.
    pub fn load(&self, backend_name: &str) -> Result<Option<String>, CredentialsError> {
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
        let entry = self.entry(backend_name)?;
        match entry.delete_credential() {
            Ok(()) => Ok(()),
            Err(keyring::Error::NoEntry) => Ok(()),
            Err(e) => Err(CredentialsError::Keyring(e)),
        }
    }

    /// Compose the per-backend service string + open the entry.
    /// Rejects empty / slash-bearing backend names so the
    /// `<prefix>/<name>` join is unambiguous.
    fn entry(&self, backend_name: &str) -> Result<keyring::Entry, CredentialsError> {
        if backend_name.is_empty() || backend_name.contains('/') || backend_name.contains('\0') {
            return Err(CredentialsError::InvalidBackendName(backend_name.to_owned()));
        }
        let service = format!("{SERVICE_PREFIX}/{backend_name}");
        Ok(keyring::Entry::new(&service, CREDENTIAL_USER)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_or_slashed_name() {
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
    }

    #[test]
    fn entry_uses_prefixed_service_name() {
        let creds = Credentials;
        // Constructing the entry doesn't touch the live keychain; we
        // can sanity-check the path without depending on a daemon.
        let entry = creds.entry("my-bucket");
        assert!(entry.is_ok(), "entry build should succeed for legal names");
    }
}
