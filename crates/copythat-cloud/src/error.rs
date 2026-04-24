//! Phase 32 — typed backend errors.

use thiserror::Error;

use crate::credentials::CredentialsError;

/// Errors raised by [`crate::make_operator`], [`crate::CopyTarget`]
/// implementations, and the [`crate::registry::BackendRegistry`].
#[derive(Debug, Error)]
pub enum BackendError {
    /// The configured backend kind isn't enabled in this build. Phase
    /// 32a only lights `LocalFs` and `S3`; other kinds parse + persist
    /// but `make_operator` returns this until Phase 32b.
    #[error("backend kind not enabled in this build: {kind}")]
    BackendNotEnabled { kind: &'static str },

    /// The backend config is missing a required field, has an empty
    /// value where one is required, or contains an unparseable URL.
    #[error("invalid backend config: {0}")]
    InvalidConfig(String),

    /// A backend with this name was not registered.
    #[error("backend `{0}` not found in registry")]
    NotFound(String),

    /// A backend with this name already exists in the registry.
    #[error("backend `{0}` already registered")]
    AlreadyExists(String),

    /// The OS keychain layer failed to read or write a credential.
    #[error("keychain access failed: {0}")]
    Credentials(#[from] CredentialsError),

    /// Errors surfaced by the underlying OpenDAL operator (network,
    /// 404, permission, etc.).
    #[error("opendal error: {0}")]
    OpenDal(#[from] opendal::Error),
}

impl BackendError {
    /// Stable Fluent key for end-user surface. Maps every variant to
    /// one of the five `cloud-error-*` keys in
    /// `locales/*/copythat.ftl`.
    pub fn fluent_key(&self) -> &'static str {
        match self {
            BackendError::Credentials(_) => "cloud-error-keychain",
            BackendError::InvalidConfig(_) => "cloud-error-invalid-config",
            BackendError::NotFound(_) => "cloud-error-not-found",
            BackendError::OpenDal(e) => match e.kind() {
                opendal::ErrorKind::NotFound => "cloud-error-not-found",
                opendal::ErrorKind::PermissionDenied => "cloud-error-permission",
                _ => "cloud-error-network",
            },
            BackendError::BackendNotEnabled { .. } => "cloud-error-invalid-config",
            BackendError::AlreadyExists(_) => "cloud-error-invalid-config",
        }
    }
}
