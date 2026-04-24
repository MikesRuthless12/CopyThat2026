//! Phase 32 — cloud backend matrix via OpenDAL.
//!
//! Wraps [`opendal::Operator`] in a typed [`Backend`] / [`BackendKind`]
//! / [`BackendConfig`] surface, plus a [`CopyTarget`] async trait that
//! the Phase 32 engine integration calls into.
//!
//! # Phase 32a scope (this crate, today)
//!
//! - Backend types + config DTOs for all 12 brief-listed
//!   [`BackendKind`]s, with TOML-friendly `serde` plumbing.
//! - `make_operator` factory wired up for [`BackendKind::LocalFs`] and
//!   [`BackendKind::S3`] (the latter covers Amazon S3, Cloudflare R2,
//!   Backblaze B2, and any S3-compatible endpoint via the same
//!   driver). The remaining ten kinds parse their config and resolve
//!   to [`BackendError::BackendNotEnabled`] until 32b enables their
//!   opendal feature flags.
//! - [`CopyTarget`] trait + [`OperatorTarget`] adapter exposing
//!   `put / get / list / stat / delete` over any backend.
//! - [`Credentials`] keychain layer (`store / load / delete`) backed
//!   by the cross-platform `keyring` crate; service name is
//!   `copythat-cloud/<backend-name>`.
//! - In-memory [`BackendRegistry`] with deterministic add / remove /
//!   get and a `to_toml_value` round-trip suitable for persistence
//!   from `copythat-settings`.
//!
//! Phase 32b lights up the remaining backends, the engine
//! `CopySource` / `CopySink` integration, and the Svelte "Remotes"
//! tab + Add-backend wizard.

#![forbid(unsafe_code)]

pub mod backend;
pub mod credentials;
pub mod error;
pub mod registry;
pub mod target;

pub use backend::{
    AzureBlobConfig, Backend, BackendConfig, BackendKind, FtpConfig, GcsConfig, LocalFsConfig,
    OAuthConfig, S3Config, SftpConfig, WebdavConfig, make_operator,
};
pub use credentials::{Credentials, CredentialsError};
pub use error::BackendError;
pub use registry::BackendRegistry;
pub use target::{CopyTarget, EntryMeta, OperatorTarget};

/// Re-export of `opendal` for callers that don't want to depend on it
/// directly. The Phase 32 IPC layer leans on this to map OpenDAL's
/// `ErrorKind` into Fluent suffix strings.
pub use opendal;
