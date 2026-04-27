//! Push-notification dispatch primitives.
//!
//! `PushTarget` is the typed wrapper around an APNs device token
//! (iOS) or an FCM registration token (Android). The runner hands a
//! `PushPayload` to a [`NotifyDispatcher`] and the dispatcher
//! issues an HTTP POST against the provider endpoint configured in
//! `MobileSettings`.
//!
//! **Provider-token signing (Phase 37 follow-up).** APNs uses
//! ES256 JWTs minted from a `.p8` team key; FCM HTTP v1 uses RS256
//! JWTs minted from a Google service-account JSON. Both signers
//! live in this module behind the [`PushSigner`] trait so the
//! [`HttpDispatcher`] can plug either in interchangeably; the
//! follow-up Tauri runner reads the signer credentials out of the
//! OS keychain and constructs the right signer for each push.

use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use jsonwebtoken::{Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};

/// Where the desktop pushes a notification to. Variants cover the
/// two production providers (Apple, Google) plus a `Stub` flavour
/// the smoke test points at a localhost mock so we don't burn real
/// device tokens during CI.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum PushTarget {
    /// Apple Push Notification Service token (iOS / iPadOS).
    Apns {
        /// 64-character hex-encoded device token issued by APNs.
        token: String,
    },
    /// Firebase Cloud Messaging registration token (Android).
    Fcm {
        /// Long alphanumeric registration token issued by FCM.
        token: String,
    },
    /// Smoke-test only: POSTs the payload to an arbitrary HTTP URL
    /// so the call surface is exercised without external side
    /// effects. The runner never selects this in production code.
    #[serde(rename = "stub_endpoint")]
    StubEndpoint {
        /// Full URL the dispatcher POSTs to. Useful in tests with
        /// a localhost mock listening on a random port.
        url: String,
    },
}

/// Body the desktop wants to surface on the phone's lock screen.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushPayload {
    pub title: String,
    pub body: String,
    /// Optional small icon hint. iOS / Android clients map this to
    /// the appropriate native asset.
    pub icon: Option<String>,
    /// Optional deep-link URL the phone navigates to when the user
    /// taps the notification (`copythat-mobile://job/<row-id>`).
    pub deep_link: Option<String>,
}

/// What the dispatcher hands back after a successful POST. Stripped
/// of any provider-specific fields so callers can record the
/// outcome generically.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushReceipt {
    pub provider: String,
    pub status: u16,
}

#[derive(Debug, thiserror::Error)]
pub enum PushSendError {
    #[error("http {provider}: {source}")]
    Http {
        provider: &'static str,
        #[source]
        source: reqwest::Error,
    },
    #[error("http {provider} returned {status} (body: {body})")]
    BadStatus {
        provider: &'static str,
        status: u16,
        body: String,
    },
    #[error("sign {provider}: {reason}")]
    Sign {
        provider: &'static str,
        reason: String,
    },
}

/// Boundary the runner targets. Real APNs / FCM dispatchers
/// implement this in the Phase 37 follow-up; today the default
/// implementation is `HttpDispatcher`, an unsigned reqwest-backed
/// POST.
#[async_trait::async_trait]
pub trait NotifyDispatcher: Send + Sync {
    async fn send(
        &self,
        target: &PushTarget,
        payload: &PushPayload,
    ) -> Result<PushReceipt, PushSendError>;
}

/// Default dispatcher: POSTs the JSON payload against the target
/// URL. When an [`Arc<dyn PushSigner>`] is wired via
/// [`HttpDispatcher::with_signer`], the dispatcher attaches the
/// `Authorization: bearer <jwt>` header before sending. Without a
/// signer, requests go out unsigned — useful for the smoke test +
/// localhost mocks (`PushTarget::StubEndpoint`) but rejected by the
/// real APNs / FCM endpoints with a 401.
pub struct HttpDispatcher {
    client: reqwest::Client,
    signer: Option<Arc<dyn PushSigner>>,
}

impl HttpDispatcher {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .unwrap_or_else(|_| reqwest::Client::new()),
            signer: None,
        }
    }

    /// Attach a signer that mints the per-request bearer token.
    pub fn with_signer(mut self, signer: Arc<dyn PushSigner>) -> Self {
        self.signer = Some(signer);
        self
    }

    /// Resolve the per-target URL the dispatcher hits.
    fn url_for(target: &PushTarget) -> (&'static str, String) {
        match target {
            PushTarget::Apns { token } => (
                "apns",
                format!("https://api.push.apple.com/3/device/{token}"),
            ),
            PushTarget::Fcm { token: _ } => (
                "fcm",
                "https://fcm.googleapis.com/v1/projects/copythat/messages:send".to_string(),
            ),
            PushTarget::StubEndpoint { url } => ("stub", url.clone()),
        }
    }
}

impl Default for HttpDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl NotifyDispatcher for HttpDispatcher {
    async fn send(
        &self,
        target: &PushTarget,
        payload: &PushPayload,
    ) -> Result<PushReceipt, PushSendError> {
        let (provider, url) = Self::url_for(target);
        let mut req = self.client.post(&url).json(payload);
        if let Some(signer) = &self.signer {
            let token = signer.sign_for(target).map_err(|e| PushSendError::Sign {
                provider,
                reason: e,
            })?;
            req = req.bearer_auth(token);
        }
        let resp = req
            .send()
            .await
            .map_err(|source| PushSendError::Http { provider, source })?;
        let status = resp.status().as_u16();
        if !resp.status().is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(PushSendError::BadStatus {
                provider,
                status,
                body,
            });
        }
        Ok(PushReceipt {
            provider: provider.to_string(),
            status,
        })
    }
}

// ---------------------------------------------------------------------
// Push signers (APNs ES256 + FCM RS256)
// ---------------------------------------------------------------------

/// Signs a per-request bearer token. The dispatcher wires this in
/// behind a `Send + Sync` `Arc` so a single signer is shared across
/// every concurrent push.
pub trait PushSigner: Send + Sync + std::fmt::Debug {
    fn sign_for(&self, target: &PushTarget) -> Result<String, String>;
}

/// APNs token-based authentication.
///
/// Apple issues a `.p8` PEM-encoded ECDSA P-256 private key alongside
/// a 10-character `key_id` and a 10-character `team_id`. The runner
/// stores the PEM in the OS keychain (under
/// `copythat-mobile/apns_p8`), reads it back at startup, and hands
/// the bytes to [`ApnsSigner::new`]. Each `sign_for` call mints a
/// fresh ES256 JWT with `iat = now`, `exp = now + 1h`, `iss = team_id`,
/// and `kid = key_id` in the header — the format documented at
/// <https://developer.apple.com/documentation/usernotifications/establishing_a_token-based_connection_to_apns>.
#[derive(Debug, Clone)]
pub struct ApnsSigner {
    pub team_id: String,
    pub key_id: String,
    /// PEM-encoded ECDSA P-256 private key (`-----BEGIN PRIVATE KEY-----`
    /// … `-----END PRIVATE KEY-----`). Kept as bytes so the signer
    /// can be created from the keychain without round-tripping
    /// through `String`.
    pub p8_pem: Vec<u8>,
}

impl ApnsSigner {
    /// Build a signer from the team + key identifiers and the raw
    /// PEM bytes of the p8 file. Returns an error if the PEM does
    /// not parse as an ECDSA P-256 key.
    pub fn new(
        team_id: impl Into<String>,
        key_id: impl Into<String>,
        p8_pem: Vec<u8>,
    ) -> Result<Self, String> {
        // Validate the key parses now so a misconfigured signer fails
        // at construction rather than at first push.
        EncodingKey::from_ec_pem(&p8_pem).map_err(|e| format!("apns p8: {e}"))?;
        Ok(Self {
            team_id: team_id.into(),
            key_id: key_id.into(),
            p8_pem,
        })
    }
}

#[derive(Debug, Serialize)]
struct ApnsClaims<'a> {
    iss: &'a str,
    iat: u64,
    exp: u64,
}

impl PushSigner for ApnsSigner {
    fn sign_for(&self, _target: &PushTarget) -> Result<String, String> {
        let key = EncodingKey::from_ec_pem(&self.p8_pem).map_err(|e| format!("apns p8: {e}"))?;
        let mut header = Header::new(Algorithm::ES256);
        header.kid = Some(self.key_id.clone());
        let now = unix_now();
        let claims = ApnsClaims {
            iss: &self.team_id,
            iat: now,
            exp: now + 60 * 60, // 1 hour — APNs caps tokens at ~1h.
        };
        jsonwebtoken::encode(&header, &claims, &key).map_err(|e| format!("apns sign: {e}"))
    }
}

/// FCM HTTP v1 token-based authentication via a Google service-
/// account JSON. The runner stores the JSON in the keychain (under
/// `copythat-mobile/fcm_service_account`); the signer parses out the
/// PEM private key, the client_email, and the token_uri at
/// construction time. Each `sign_for` call mints a fresh RS256 JWT
/// scoped for `https://www.googleapis.com/auth/firebase.messaging`.
#[derive(Debug, Clone)]
pub struct FcmSigner {
    pub client_email: String,
    pub project_id: String,
    /// PEM-encoded RSA private key, extracted from the service
    /// account JSON's `private_key` field at construction time.
    pub rsa_pem: Vec<u8>,
}

impl FcmSigner {
    /// Build a signer from a Google service-account JSON blob (raw
    /// bytes, exactly as the file ships). Returns an error if the
    /// JSON is malformed or the key does not parse as RSA.
    pub fn from_service_account_json(json_bytes: &[u8]) -> Result<Self, String> {
        #[derive(Deserialize)]
        struct ServiceAccount {
            project_id: String,
            client_email: String,
            private_key: String,
        }
        let sa: ServiceAccount =
            serde_json::from_slice(json_bytes).map_err(|e| format!("fcm json: {e}"))?;
        let pem_bytes = sa.private_key.into_bytes();
        EncodingKey::from_rsa_pem(&pem_bytes).map_err(|e| format!("fcm rsa: {e}"))?;
        Ok(Self {
            client_email: sa.client_email,
            project_id: sa.project_id,
            rsa_pem: pem_bytes,
        })
    }
}

#[derive(Debug, Serialize)]
struct FcmClaims<'a> {
    iss: &'a str,
    scope: &'a str,
    aud: &'a str,
    iat: u64,
    exp: u64,
}

impl PushSigner for FcmSigner {
    fn sign_for(&self, _target: &PushTarget) -> Result<String, String> {
        let key = EncodingKey::from_rsa_pem(&self.rsa_pem).map_err(|e| format!("fcm rsa: {e}"))?;
        let header = Header::new(Algorithm::RS256);
        let now = unix_now();
        let claims = FcmClaims {
            iss: &self.client_email,
            scope: "https://www.googleapis.com/auth/firebase.messaging",
            aud: "https://oauth2.googleapis.com/token",
            iat: now,
            exp: now + 60 * 60,
        };
        jsonwebtoken::encode(&header, &claims, &key).map_err(|e| format!("fcm sign: {e}"))
    }
}

fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::{DecodingKey, Validation};

    #[test]
    fn push_target_round_trips_through_serde() {
        let t = PushTarget::Apns {
            token: "ab".repeat(32),
        };
        let s = serde_json::to_string(&t).unwrap();
        let back: PushTarget = serde_json::from_str(&s).unwrap();
        assert_eq!(t, back);
    }

    #[test]
    fn url_for_smoke_endpoints() {
        let stub = PushTarget::StubEndpoint {
            url: "http://127.0.0.1:9000/push".into(),
        };
        let (provider, url) = HttpDispatcher::url_for(&stub);
        assert_eq!(provider, "stub");
        assert!(url.starts_with("http://"));
    }

    /// Mint a test ECDSA P-256 keypair as a PEM PKCS#8 + matching
    /// public-key PEM. Reuses rcgen's primitives (already a dep for
    /// `EphemeralCert`) so the test doesn't grow the dep tree.
    fn make_p256_pem_pair() -> (Vec<u8>, Vec<u8>) {
        let key_pair = rcgen::KeyPair::generate_for(&rcgen::PKCS_ECDSA_P256_SHA256).unwrap();
        let priv_pem = key_pair.serialize_pem();
        let pub_pem = key_pair.public_key_pem();
        (priv_pem.into_bytes(), pub_pem.into_bytes())
    }

    fn make_rsa_pem_pair() -> (Vec<u8>, Vec<u8>) {
        // rcgen 0.14 supports PKCS_RSA_SHA256 only when paired with a
        // user-supplied RSA key (rcgen itself doesn't ship an RSA
        // generator). For the FCM signer test we generate an RSA key
        // via the `rsa` crate (already in the workspace transitively
        // via `age`) and serialize to PKCS#8 PEM.
        use rsa::pkcs1::EncodeRsaPublicKey;
        use rsa::pkcs8::{EncodePrivateKey, LineEnding};
        use rsa::{RsaPrivateKey, RsaPublicKey};
        let mut rng = rand_compat::SystemRng;
        let priv_key = RsaPrivateKey::new(&mut rng, 2048).unwrap();
        let pub_key = RsaPublicKey::from(&priv_key);
        let priv_pem = priv_key.to_pkcs8_pem(LineEnding::LF).unwrap().to_string();
        let pub_pem = pub_key.to_pkcs1_pem(LineEnding::LF).unwrap();
        (priv_pem.into_bytes(), pub_pem.into_bytes())
    }

    /// Minimal CryptoRng + RngCore shim so `rsa::RsaPrivateKey::new`
    /// can pull entropy from `getrandom` without dragging the full
    /// `rand` ecosystem in as a dev-dep.
    mod rand_compat {
        pub struct SystemRng;
        impl rand_core::RngCore for SystemRng {
            fn next_u32(&mut self) -> u32 {
                let mut buf = [0u8; 4];
                getrandom::fill(&mut buf).unwrap();
                u32::from_le_bytes(buf)
            }
            fn next_u64(&mut self) -> u64 {
                let mut buf = [0u8; 8];
                getrandom::fill(&mut buf).unwrap();
                u64::from_le_bytes(buf)
            }
            fn fill_bytes(&mut self, dst: &mut [u8]) {
                getrandom::fill(dst).unwrap();
            }
            fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), rand_core::Error> {
                getrandom::fill(dst)
                    .map_err(|e| rand_core::Error::new(std::io::Error::other(e.to_string())))
            }
        }
        impl rand_core::CryptoRng for SystemRng {}
    }

    #[test]
    fn apns_signer_mints_es256_jwt_with_team_iss_and_kid() {
        let (priv_pem, pub_pem) = make_p256_pem_pair();
        let signer = ApnsSigner::new("ABCDE12345", "KEY1234567", priv_pem).expect("apns signer");
        let token = signer
            .sign_for(&PushTarget::Apns {
                token: "deadbeef".into(),
            })
            .expect("sign");
        assert_eq!(token.matches('.').count(), 2, "expected three JWT segments");

        let decoding = DecodingKey::from_ec_pem(&pub_pem).expect("decoding key");
        let mut v = Validation::new(Algorithm::ES256);
        v.validate_exp = true;
        v.validate_aud = false;
        v.required_spec_claims.clear();
        let data =
            jsonwebtoken::decode::<serde_json::Value>(&token, &decoding, &v).expect("verify");
        assert_eq!(data.claims["iss"], "ABCDE12345");
        let kid = data.header.kid.as_deref().unwrap_or_default();
        assert_eq!(kid, "KEY1234567");
    }

    #[test]
    fn fcm_signer_parses_service_account_and_signs_rs256() {
        let (priv_pem, pub_pem) = make_rsa_pem_pair();
        let priv_pem_str = String::from_utf8(priv_pem).unwrap();
        let json = serde_json::json!({
            "type": "service_account",
            "project_id": "copythat-test",
            "client_email": "robot@copythat-test.iam.gserviceaccount.com",
            "private_key": priv_pem_str,
        });
        let signer =
            FcmSigner::from_service_account_json(json.to_string().as_bytes()).expect("fcm signer");
        assert_eq!(signer.project_id, "copythat-test");
        assert_eq!(
            signer.client_email,
            "robot@copythat-test.iam.gserviceaccount.com"
        );

        let token = signer
            .sign_for(&PushTarget::Fcm {
                token: "fcm-device-token".into(),
            })
            .expect("sign");
        let decoding = DecodingKey::from_rsa_pem(&pub_pem).expect("decoding key");
        let mut v = Validation::new(Algorithm::RS256);
        v.validate_exp = true;
        v.validate_aud = false;
        v.required_spec_claims.clear();
        let data =
            jsonwebtoken::decode::<serde_json::Value>(&token, &decoding, &v).expect("verify");
        assert_eq!(
            data.claims["iss"],
            "robot@copythat-test.iam.gserviceaccount.com"
        );
        assert_eq!(
            data.claims["scope"],
            "https://www.googleapis.com/auth/firebase.messaging"
        );
    }

    #[test]
    fn apns_signer_rejects_malformed_p8() {
        let err = ApnsSigner::new("X", "Y", b"not a pem".to_vec()).unwrap_err();
        assert!(err.contains("apns p8"), "{err}");
    }

    #[test]
    fn fcm_signer_rejects_non_json_bytes() {
        let err = FcmSigner::from_service_account_json(b"{not json").unwrap_err();
        assert!(err.contains("fcm json"), "{err}");
    }
}
