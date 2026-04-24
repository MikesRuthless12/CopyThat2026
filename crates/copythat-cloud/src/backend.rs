//! Phase 32 — backend descriptors + the `make_operator` factory.

use serde::{Deserialize, Serialize};

use crate::error::BackendError;

/// One configured remote: a stable user-facing name + the typed
/// [`BackendKind`] + the kind-specific [`BackendConfig`]. Secrets do
/// not live here — they're stored in the OS keychain under
/// `copythat-cloud/<name>` via [`crate::credentials::Credentials`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Backend {
    pub name: String,
    pub kind: BackendKind,
    #[serde(default)]
    pub config: BackendConfig,
}

/// Twelve top-level backends Copy That surfaces in the Add-backend
/// wizard. The string identifiers in the `wire` attribute are stable
/// — they round-trip through TOML and the IPC `BackendDto`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum BackendKind {
    S3,
    R2,
    B2,
    AzureBlob,
    Gcs,
    Onedrive,
    GoogleDrive,
    Dropbox,
    Webdav,
    Sftp,
    Ftp,
    LocalFs,
}

impl BackendKind {
    /// Stable wire string used by IPC + the Phase 32 Fluent
    /// `backend-*` keys. Mirrored exactly by the `serde(rename_all =
    /// "kebab-case")` derive — kept as a separate `&'static str`
    /// helper because the enum's `Display` would otherwise carry the
    /// debug formatter.
    pub fn wire(&self) -> &'static str {
        match self {
            BackendKind::S3 => "s3",
            BackendKind::R2 => "r2",
            BackendKind::B2 => "b2",
            BackendKind::AzureBlob => "azure-blob",
            BackendKind::Gcs => "gcs",
            BackendKind::Onedrive => "onedrive",
            BackendKind::GoogleDrive => "google-drive",
            BackendKind::Dropbox => "dropbox",
            BackendKind::Webdav => "webdav",
            BackendKind::Sftp => "sftp",
            BackendKind::Ftp => "ftp",
            BackendKind::LocalFs => "local-fs",
        }
    }

    /// Fluent display key for the picker (`backend-s3`, etc).
    pub fn fluent_key(&self) -> &'static str {
        match self {
            BackendKind::S3 => "backend-s3",
            BackendKind::R2 => "backend-r2",
            BackendKind::B2 => "backend-b2",
            BackendKind::AzureBlob => "backend-azure-blob",
            BackendKind::Gcs => "backend-gcs",
            BackendKind::Onedrive => "backend-onedrive",
            BackendKind::GoogleDrive => "backend-google-drive",
            BackendKind::Dropbox => "backend-dropbox",
            BackendKind::Webdav => "backend-webdav",
            BackendKind::Sftp => "backend-sftp",
            BackendKind::Ftp => "backend-ftp",
            BackendKind::LocalFs => "backend-local-fs",
        }
    }

    /// All twelve kinds in a stable display order. Used by the
    /// Add-backend wizard's combo box and by the smoke test's parity
    /// scan.
    pub const fn all() -> &'static [BackendKind] {
        &[
            BackendKind::S3,
            BackendKind::R2,
            BackendKind::B2,
            BackendKind::AzureBlob,
            BackendKind::Gcs,
            BackendKind::Onedrive,
            BackendKind::GoogleDrive,
            BackendKind::Dropbox,
            BackendKind::Webdav,
            BackendKind::Sftp,
            BackendKind::Ftp,
            BackendKind::LocalFs,
        ]
    }

    /// Whether [`make_operator`] can successfully build an operator
    /// for this kind in the current build. Phase 32a returns true
    /// only for `LocalFs` and `S3` (which covers R2 + B2 +
    /// S3-compatible). Phase 32b flips the rest.
    pub fn is_enabled(&self) -> bool {
        matches!(self, BackendKind::LocalFs | BackendKind::S3 | BackendKind::R2 | BackendKind::B2)
    }
}

/// Per-kind config. The variants intentionally don't share fields
/// because S3's `region` has no meaning to a `WebDAV` endpoint and
/// vice versa — keeping the data model tagged stops the UI from
/// rendering the wrong inputs.
///
/// Secrets (S3 secret access key, SFTP password, etc.) live in the
/// keychain, never in `BackendConfig`. The wizard collects them once
/// and stores them via [`crate::credentials::Credentials::store`];
/// `make_operator` reads them back at operator-build time.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum BackendConfig {
    LocalFs(LocalFsConfig),
    S3(S3Config),
    R2(S3Config),
    B2(S3Config),
    AzureBlob(AzureBlobConfig),
    Gcs(GcsConfig),
    Onedrive(OAuthConfig),
    GoogleDrive(OAuthConfig),
    Dropbox(OAuthConfig),
    Webdav(WebdavConfig),
    Sftp(SftpConfig),
    Ftp(FtpConfig),
    /// Sentinel used when a `Backend` is constructed without a config
    /// — only legal during a TOML round-trip on a partially-populated
    /// row from older builds. `make_operator` rejects it.
    #[default]
    Empty,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LocalFsConfig {
    /// Absolute path the operator's `/` resolves to.
    pub root: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct S3Config {
    pub bucket: String,
    /// Region name — required by the AWS S3 driver, ignored by
    /// custom-endpoint backends like R2 / B2 but still stored for
    /// display.
    pub region: String,
    /// Optional custom endpoint URL; set for R2 / B2 / MinIO /
    /// LocalStack. Empty string = use the AWS default endpoint for
    /// the configured `region`.
    #[serde(default)]
    pub endpoint: String,
    /// Optional prefix prepended to every key. Empty = bucket root.
    #[serde(default)]
    pub root: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AzureBlobConfig {
    pub container: String,
    pub account_name: String,
    /// Optional custom endpoint URL; defaults to
    /// `https://<account>.blob.core.windows.net`.
    #[serde(default)]
    pub endpoint: String,
    #[serde(default)]
    pub root: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GcsConfig {
    pub bucket: String,
    /// Optional service-account email; the OAuth token itself sits in
    /// the keychain under the backend's secret slot.
    #[serde(default)]
    pub service_account: String,
    #[serde(default)]
    pub root: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OAuthConfig {
    /// OAuth client identifier — the secret half lives in the
    /// keychain.
    pub client_id: String,
    #[serde(default)]
    pub root: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WebdavConfig {
    pub endpoint: String,
    pub username: String,
    #[serde(default)]
    pub root: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SftpConfig {
    pub host: String,
    #[serde(default = "default_sftp_port")]
    pub port: u16,
    pub username: String,
    #[serde(default)]
    pub root: String,
}

fn default_sftp_port() -> u16 {
    22
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FtpConfig {
    pub host: String,
    #[serde(default = "default_ftp_port")]
    pub port: u16,
    pub username: String,
    #[serde(default)]
    pub root: String,
}

fn default_ftp_port() -> u16 {
    21
}

/// Build a configured [`opendal::Operator`] from a [`Backend`] +
/// (optionally) a previously-loaded credential bundle.
///
/// `secret` semantics:
/// - `Some` — apply the supplied secret to the relevant operator
///   builder fields (`access_key_id` / `secret_access_key` for S3,
///   etc.). The caller usually retrieves it from
///   [`crate::credentials::Credentials::load`] keyed on the
///   backend's name.
/// - `None` — let opendal pull defaults from the environment
///   (`AWS_ACCESS_KEY_ID` / instance metadata) where supported.
pub fn make_operator(
    backend: &Backend,
    secret: Option<&str>,
) -> Result<opendal::Operator, BackendError> {
    match &backend.config {
        BackendConfig::LocalFs(cfg) => {
            if cfg.root.is_empty() {
                return Err(BackendError::InvalidConfig(
                    "local-fs backend requires a non-empty root path".into(),
                ));
            }
            let builder = opendal::services::Fs::default().root(&cfg.root);
            Ok(opendal::Operator::new(builder)?.finish())
        }
        BackendConfig::S3(cfg) | BackendConfig::R2(cfg) | BackendConfig::B2(cfg) => {
            if cfg.bucket.is_empty() {
                return Err(BackendError::InvalidConfig(
                    "s3-class backend requires a non-empty bucket".into(),
                ));
            }
            let mut builder = opendal::services::S3::default().bucket(&cfg.bucket);
            if !cfg.region.is_empty() {
                builder = builder.region(&cfg.region);
            }
            if !cfg.endpoint.is_empty() {
                builder = builder.endpoint(&cfg.endpoint);
            }
            if !cfg.root.is_empty() {
                builder = builder.root(&cfg.root);
            }
            // Secret format: "<access_key>\n<secret_key>" — the
            // wizard packs both halves into the keychain entry under
            // the same backend name. Phase 32b adds session-token
            // support for STS-issued credentials.
            if let Some(s) = secret {
                if let Some((ak, sk)) = s.split_once('\n') {
                    builder = builder.access_key_id(ak.trim()).secret_access_key(sk.trim());
                } else {
                    return Err(BackendError::InvalidConfig(
                        "s3 secret bundle must be `<access_key>\\n<secret_key>`".into(),
                    ));
                }
            }
            Ok(opendal::Operator::new(builder)?.finish())
        }
        BackendConfig::AzureBlob(_) => {
            Err(BackendError::BackendNotEnabled { kind: "azure-blob" })
        }
        BackendConfig::Gcs(_) => Err(BackendError::BackendNotEnabled { kind: "gcs" }),
        BackendConfig::Onedrive(_) => Err(BackendError::BackendNotEnabled { kind: "onedrive" }),
        BackendConfig::GoogleDrive(_) => {
            Err(BackendError::BackendNotEnabled { kind: "google-drive" })
        }
        BackendConfig::Dropbox(_) => Err(BackendError::BackendNotEnabled { kind: "dropbox" }),
        BackendConfig::Webdav(_) => Err(BackendError::BackendNotEnabled { kind: "webdav" }),
        BackendConfig::Sftp(_) => Err(BackendError::BackendNotEnabled { kind: "sftp" }),
        BackendConfig::Ftp(_) => Err(BackendError::BackendNotEnabled { kind: "ftp" }),
        BackendConfig::Empty => Err(BackendError::InvalidConfig(
            "backend has no config — re-add through the wizard".into(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wire_string_roundtrip_matches_serde() {
        // We can't pull serde_json just for tests; round-trip through
        // a TOML-compatible single-field struct instead. The kebab-
        // case rename hits whichever serde format we use, so toml
        // catches the same drift.
        for kind in BackendKind::all() {
            #[derive(serde::Serialize, serde::Deserialize)]
            struct W {
                k: BackendKind,
            }
            let s = toml::to_string(&W { k: *kind }).expect("serialize");
            // toml writes `k = "kebab-case"`.
            assert!(
                s.contains(&format!("k = \"{}\"", kind.wire())),
                "wire helper drifted from serde rename: {s}"
            );
            let back: W = toml::from_str(&s).expect("roundtrip");
            assert_eq!(back.k, *kind);
        }
    }

    #[test]
    fn fluent_key_unique_per_kind() {
        let keys: Vec<_> = BackendKind::all().iter().map(|k| k.fluent_key()).collect();
        let mut sorted = keys.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), keys.len(), "duplicate fluent key across kinds");
    }

    #[test]
    fn make_operator_local_fs_round_trip() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let backend = Backend {
            name: "local-test".into(),
            kind: BackendKind::LocalFs,
            config: BackendConfig::LocalFs(LocalFsConfig {
                root: tmp.path().to_string_lossy().into_owned(),
            }),
        };
        let op = make_operator(&backend, None).expect("operator");
        // We don't `await` here — just confirm the operator built.
        // The async smoke test exercises put/get round-trip.
        let info = op.info();
        assert_eq!(info.scheme(), opendal::Scheme::Fs);
    }

    #[test]
    fn make_operator_local_fs_rejects_empty_root() {
        let backend = Backend {
            name: "broken".into(),
            kind: BackendKind::LocalFs,
            config: BackendConfig::LocalFs(LocalFsConfig { root: String::new() }),
        };
        let err = make_operator(&backend, None).expect_err("must reject");
        assert!(matches!(err, BackendError::InvalidConfig(_)));
    }

    #[test]
    fn make_operator_s3_rejects_bad_secret_shape() {
        let backend = Backend {
            name: "s3".into(),
            kind: BackendKind::S3,
            config: BackendConfig::S3(S3Config {
                bucket: "b".into(),
                region: "us-east-1".into(),
                endpoint: String::new(),
                root: String::new(),
            }),
        };
        // Single-line secret has no `\n` separator.
        let err = make_operator(&backend, Some("only-access-key"))
            .expect_err("must reject single-line secret");
        assert!(matches!(err, BackendError::InvalidConfig(_)));
    }

    #[test]
    fn make_operator_disabled_kinds_surface_typed_error() {
        let backend = Backend {
            name: "azure".into(),
            kind: BackendKind::AzureBlob,
            config: BackendConfig::AzureBlob(AzureBlobConfig {
                container: "c".into(),
                account_name: "a".into(),
                endpoint: String::new(),
                root: String::new(),
            }),
        };
        match make_operator(&backend, None) {
            Err(BackendError::BackendNotEnabled { kind }) => assert_eq!(kind, "azure-blob"),
            other => panic!("expected BackendNotEnabled, got {other:?}"),
        }
    }

    #[test]
    fn make_operator_empty_config_rejected() {
        let backend = Backend {
            name: "stub".into(),
            kind: BackendKind::S3,
            config: BackendConfig::Empty,
        };
        let err = make_operator(&backend, None).expect_err("empty rejected");
        assert!(matches!(err, BackendError::InvalidConfig(_)));
    }

    #[test]
    fn is_enabled_matches_phase_32a_scope() {
        for kind in BackendKind::all() {
            let expected = matches!(
                kind,
                BackendKind::LocalFs
                    | BackendKind::S3
                    | BackendKind::R2
                    | BackendKind::B2
            );
            assert_eq!(kind.is_enabled(), expected, "is_enabled drift on {kind:?}");
        }
    }
}

