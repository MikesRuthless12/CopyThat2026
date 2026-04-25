//! Push-notification dispatch primitives.
//!
//! `PushTarget` is the typed wrapper around an APNs device token
//! (iOS) or an FCM registration token (Android). The runner hands a
//! `PushPayload` to a [`NotifyDispatcher`] and the dispatcher
//! issues an HTTP POST against the provider endpoint configured in
//! [`MobileSettings`].
//!
//! Real provider-token signing (APNs ed25519 JWT, FCM Google service
//! account JWT) is intentionally deferred to a Phase 37 follow-up.
//! The [`NotifyDispatcher`] surface is wired here so the runner can
//! plug a real signer in without a Cargo.toml diff. Today the
//! default dispatcher emits an unsigned POST against the configured
//! endpoint — sufficient to exercise the call site in the smoke
//! test against a localhost mock.

use std::time::Duration;

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
/// URL with no authentication. Suitable for the smoke test +
/// localhost mocks; production swaps in a signer-equipped variant.
pub struct HttpDispatcher {
    client: reqwest::Client,
}

impl HttpDispatcher {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .unwrap_or_else(|_| reqwest::Client::new()),
        }
    }

    /// Resolve the per-target URL the dispatcher hits. Production
    /// APNs / FCM URLs are placeholders today; the follow-up wires
    /// the real provider endpoints + signing.
    fn url_for(target: &PushTarget) -> (&'static str, String) {
        match target {
            PushTarget::Apns { token } => (
                "apns",
                format!("https://api.push.apple.com/3/device/{token}"),
            ),
            PushTarget::Fcm { token } => (
                "fcm",
                format!(
                    "https://fcm.googleapis.com/v1/projects/copythat/messages:send?dryRun=true&token={token}"
                ),
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
        let resp = self
            .client
            .post(&url)
            .json(payload)
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
