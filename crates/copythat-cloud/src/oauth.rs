//! Phase 32f — OAuth 2.0 device-code flow.
//!
//! The OAuth 2.0 Device Authorization Grant (RFC 8628) is the
//! user-friendly authentication flow for desktop apps that don't
//! want to embed a web view: the app asks the provider for a
//! `user_code` + `verification_uri`, opens the URI in the user's
//! default browser, and polls the token endpoint until the user
//! completes the login step.
//!
//! # Providers wired
//!
//! - **Microsoft Graph** (OneDrive). Endpoint:
//!   `https://login.microsoftonline.com/consumers/oauth2/v2.0/devicecode`.
//!   Default scope: `Files.ReadWrite offline_access`.
//! - **Google** (Drive). Endpoint:
//!   `https://oauth2.googleapis.com/device/code`.
//!   Default scope: `https://www.googleapis.com/auth/drive.file`.
//! - **Dropbox**. Endpoint:
//!   `https://api.dropboxapi.com/oauth2/authorize` (Dropbox uses
//!   PKCE rather than pure device-code — Phase 32g swaps this for
//!   the browser-redirect flow). For now, Dropbox callers pass a
//!   pre-acquired access token through the keychain.
//!
//! # Flow
//!
//! 1. [`begin_device_code_flow`] POSTs the device-code endpoint,
//!    parses the response, and returns an [`OAuthDeviceCodeFlow`]
//!    holding `device_code`, `user_code`, `verification_uri`,
//!    `expires_in`, `interval`.
//! 2. The frontend shows the `user_code` and opens the
//!    `verification_uri` via `webbrowser::open`.
//! 3. [`poll_device_code_flow`] polls the token endpoint at
//!    `interval` seconds. Returns:
//!    - [`TokenPollOutcome::Pending`] — keep polling.
//!    - [`TokenPollOutcome::SlowDown`] — increase interval by 5s.
//!    - [`TokenPollOutcome::Granted`] — save the tokens to
//!      keychain via [`crate::Credentials`].
//!    - [`TokenPollOutcome::Denied`] / [`TokenPollOutcome::Expired`]
//!      / [`TokenPollOutcome::Error`] — terminal failure.
//!
//! # Keychain storage
//!
//! On success the caller stores
//! `<access_token>\n<refresh_token>\n<expires_at_unix>` under the
//! backend's keychain slot. Refresh is a Phase 32g follow-up —
//! today tokens live until they expire.

use std::time::Duration;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Which cloud provider the device-code flow targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OAuthProvider {
    MicrosoftGraph,
    Google,
    Dropbox,
}

impl OAuthProvider {
    pub fn device_code_endpoint(&self) -> &'static str {
        match self {
            Self::MicrosoftGraph => {
                "https://login.microsoftonline.com/consumers/oauth2/v2.0/devicecode"
            }
            Self::Google => "https://oauth2.googleapis.com/device/code",
            // Dropbox doesn't support pure device-code flow; Phase
            // 32g swaps to PKCE/browser-redirect. Today callers
            // paste a pre-acquired access token directly.
            Self::Dropbox => "",
        }
    }

    pub fn token_endpoint(&self) -> &'static str {
        match self {
            Self::MicrosoftGraph => "https://login.microsoftonline.com/consumers/oauth2/v2.0/token",
            Self::Google => "https://oauth2.googleapis.com/token",
            Self::Dropbox => "https://api.dropbox.com/oauth2/token",
        }
    }

    pub fn default_scope(&self) -> &'static str {
        match self {
            Self::MicrosoftGraph => "Files.ReadWrite offline_access",
            Self::Google => "https://www.googleapis.com/auth/drive.file",
            Self::Dropbox => "",
        }
    }

    pub fn wire(&self) -> &'static str {
        match self {
            Self::MicrosoftGraph => "microsoft-graph",
            Self::Google => "google",
            Self::Dropbox => "dropbox",
        }
    }
}

#[derive(Debug, Error)]
pub enum OAuthError {
    #[error("provider `{0}` does not support the device-code flow (use PKCE/browser-redirect)")]
    Unsupported(&'static str),
    #[error("http error: {0}")]
    Http(String),
    #[error("malformed response: {0}")]
    Parse(String),
    #[error("provider error: {0}")]
    Provider(String),
}

impl From<reqwest::Error> for OAuthError {
    fn from(e: reqwest::Error) -> Self {
        Self::Http(e.to_string())
    }
}

/// Live device-code flow state. Created by
/// [`begin_device_code_flow`], consumed by
/// [`poll_device_code_flow`].
#[derive(Debug, Clone)]
pub struct OAuthDeviceCodeFlow {
    pub provider: OAuthProvider,
    pub client_id: String,
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    /// Seconds until the code expires, as reported by the provider.
    pub expires_in: u64,
    /// Recommended poll interval in seconds.
    pub interval: u64,
}

#[derive(Debug, Deserialize)]
struct DeviceCodeResponse {
    device_code: String,
    user_code: String,
    verification_uri: Option<String>,
    verification_url: Option<String>,
    expires_in: u64,
    interval: Option<u64>,
}

/// Shape returned by the provider's token-endpoint success. Google
/// uses `refresh_token`; MS uses `refresh_token` too; both use
/// `access_token` and `expires_in`.
///
/// `Debug` is hand-implemented to redact `access_token` and
/// `refresh_token`. The previous derived-`Debug` impl would dump
/// both secrets if any future contributor added a
/// `tracing::debug!("got tokens: {tokens:?}")` line — and the
/// audit-layer field-name scrubber only fires when the secrets
/// appear as JSON object keys, not when they're inlined into a
/// debug-string. Same risk for `Serialize`: keep it (the keychain
/// blob path needs it) but rely on the JSON consumer to redact.
#[derive(Clone, Deserialize, Serialize)]
pub struct OAuthTokenResponse {
    pub access_token: String,
    #[serde(default)]
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

impl std::fmt::Debug for OAuthTokenResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OAuthTokenResponse")
            .field("access_token", &"<redacted>")
            .field("refresh_token", &"<redacted>")
            .field("token_type", &self.token_type)
            .field("expires_in", &self.expires_in)
            .finish()
    }
}

impl OAuthTokenResponse {
    /// Encode as the `<access>\n<refresh>\n<expires_at_unix>` blob
    /// the keychain persistence convention uses.
    pub fn as_keychain_blob(&self, now_unix: i64) -> String {
        let expires_at = now_unix + self.expires_in as i64;
        format!(
            "{}\n{}\n{}",
            self.access_token, self.refresh_token, expires_at
        )
    }
}

/// Result of one poll cycle against the token endpoint.
#[derive(Debug, Clone)]
pub enum TokenPollOutcome {
    /// Keep polling at the configured interval.
    Pending,
    /// Increase poll interval by 5 seconds per RFC 8628.
    SlowDown,
    /// User authorized — tokens are ready.
    Granted(OAuthTokenResponse),
    /// User denied the request.
    Denied,
    /// Device code expired before the user authorized.
    Expired,
    /// Other provider error — access_denied with a reason, etc.
    Error(String),
}

/// Begin the device-code flow. POSTs the provider's device-code
/// endpoint with `client_id` + `scope`. Returns state the caller
/// holds while they show the user the `user_code` and
/// `verification_uri`.
pub async fn begin_device_code_flow(
    provider: OAuthProvider,
    client_id: &str,
    scope_override: Option<&str>,
) -> Result<OAuthDeviceCodeFlow, OAuthError> {
    if matches!(provider, OAuthProvider::Dropbox) {
        return Err(OAuthError::Unsupported(provider.wire()));
    }
    let scope = scope_override.unwrap_or(provider.default_scope());
    let client = reqwest::Client::new();
    let form = [("client_id", client_id), ("scope", scope)];
    let resp: DeviceCodeResponse = client
        .post(provider.device_code_endpoint())
        .form(&form)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    let verification_uri = resp
        .verification_uri
        .or(resp.verification_url)
        .ok_or_else(|| OAuthError::Parse("missing verification_uri".into()))?;
    Ok(OAuthDeviceCodeFlow {
        provider,
        client_id: client_id.to_owned(),
        device_code: resp.device_code,
        user_code: resp.user_code,
        verification_uri,
        expires_in: resp.expires_in,
        interval: resp.interval.unwrap_or(5),
    })
}

/// Poll the token endpoint once. Returns [`TokenPollOutcome`];
/// callers sleep `flow.interval` seconds between calls and handle
/// [`TokenPollOutcome::SlowDown`] by bumping that interval by 5s.
pub async fn poll_device_code_flow(
    flow: &OAuthDeviceCodeFlow,
) -> Result<TokenPollOutcome, OAuthError> {
    let client = reqwest::Client::new();
    let grant_type = "urn:ietf:params:oauth:grant-type:device_code";
    let form = [
        ("client_id", flow.client_id.as_str()),
        ("device_code", flow.device_code.as_str()),
        ("grant_type", grant_type),
    ];
    let resp = client
        .post(flow.provider.token_endpoint())
        .form(&form)
        .send()
        .await?;
    let status = resp.status();
    let body: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| OAuthError::Parse(format!("token body: {e}")))?;

    if status.is_success() {
        // Suppress serde_json's input-bearing Display message — on
        // a malformed-but-200 response it can include excerpts of
        // the surrounding bytes (and thus token characters) in the
        // error string that propagates up to the renderer log.
        let tokens: OAuthTokenResponse = serde_json::from_value(body)
            .map_err(|_| OAuthError::Parse("token-response decode failed".into()))?;
        return Ok(TokenPollOutcome::Granted(tokens));
    }

    // Failures come back as 4xx with a JSON body carrying an
    // `error` string. RFC 8628 defines the set: authorization_pending,
    // slow_down, access_denied, expired_token.
    let err = body
        .get("error")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_owned();
    Ok(match err.as_str() {
        "authorization_pending" => TokenPollOutcome::Pending,
        "slow_down" => TokenPollOutcome::SlowDown,
        "access_denied" => TokenPollOutcome::Denied,
        "expired_token" => TokenPollOutcome::Expired,
        other => TokenPollOutcome::Error(other.to_owned()),
    })
}

/// Phase 32g — exchange a long-lived refresh token for a fresh
/// access token. Call whenever a stored access token is about to
/// expire (see `OAuthTokenResponse::as_keychain_blob` for the
/// expiry timestamp format).
///
/// Matches the OAuth 2.0 `refresh_token` grant-type from RFC 6749
/// §6. Providers that don't support refresh tokens (Dropbox's
/// device-code flow issues short-lived access tokens without a
/// refresh_token) will 400 — the caller should re-run
/// `begin_device_code_flow` in that case.
pub async fn refresh_oauth_token(
    provider: OAuthProvider,
    client_id: &str,
    refresh_token: &str,
) -> Result<OAuthTokenResponse, OAuthError> {
    if refresh_token.is_empty() {
        return Err(OAuthError::Provider(
            "refresh_token is empty — re-run the device-code flow".into(),
        ));
    }
    let client = reqwest::Client::new();
    let form = [
        ("client_id", client_id),
        ("refresh_token", refresh_token),
        ("grant_type", "refresh_token"),
    ];
    let resp = client
        .post(provider.token_endpoint())
        .form(&form)
        .send()
        .await?;
    if !resp.status().is_success() {
        // Sanitised: suppress serde_json's input-bearing error
        // messages and provider-side body content (which on
        // misconfigured endpoints can echo the access_token).
        let status_code = resp.status().as_u16();
        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|_| OAuthError::Parse("refresh-response body decode failed".into()))?;
        let err = body
            .get("error")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_owned();
        // Allow only RFC 6749 short error codes through; anything
        // else collapses to a generic label.
        let safe_err = match err.as_str() {
            "invalid_grant"
            | "invalid_client"
            | "invalid_request"
            | "invalid_scope"
            | "unauthorized_client"
            | "unsupported_grant_type"
            | "access_denied" => err,
            _ => format!("token endpoint returned status {status_code}"),
        };
        return Err(OAuthError::Provider(safe_err));
    }
    // Sanitised — same reason as the failure-path decode above.
    let mut tokens: OAuthTokenResponse = resp
        .json()
        .await
        .map_err(|_| OAuthError::Parse("refresh-response decode failed".into()))?;
    // Some providers (Microsoft Graph) omit the refresh_token
    // in the refresh-response — in that case the existing
    // refresh_token stays valid. Preserve it so callers can keep
    // refreshing.
    if tokens.refresh_token.is_empty() {
        tokens.refresh_token = refresh_token.to_owned();
    }
    Ok(tokens)
}

/// Convenience — run the full flow to completion with an
/// `on_poll` callback fired after each wait. Used by tests and
/// scripted callers; the UI layer wants finer control so it calls
/// `begin_device_code_flow` + `poll_device_code_flow` directly.
pub async fn run_device_code_flow(
    provider: OAuthProvider,
    client_id: &str,
    scope_override: Option<&str>,
    on_poll: &(dyn Fn(&OAuthDeviceCodeFlow, &TokenPollOutcome) + Send + Sync),
) -> Result<OAuthTokenResponse, OAuthError> {
    let flow = begin_device_code_flow(provider, client_id, scope_override).await?;

    // Open the verification URI in the user's default browser —
    // best-effort, most users expect this. Reject anything other
    // than `https://` before handing to `webbrowser::open`: an
    // attacker who MITMs the device-code POST (CA misissuance,
    // forced corporate root) can return `verification_uri =
    // "ms-cxh-full://...arg"` or `"javascript:..."` which on
    // Windows surface as a `ShellExecuteW` against the registered
    // protocol handler — RCE on the unprivileged user.
    if let Ok(parsed) = url::Url::parse(&flow.verification_uri) {
        if parsed.scheme() == "https" {
            let _ = webbrowser::open(&flow.verification_uri);
        }
    }

    let mut interval = flow.interval.max(1);
    let deadline = std::time::Instant::now() + Duration::from_secs(flow.expires_in.max(60));
    loop {
        if std::time::Instant::now() > deadline {
            return Err(OAuthError::Provider("device code expired".into()));
        }
        tokio::time::sleep(Duration::from_secs(interval)).await;
        let outcome = poll_device_code_flow(&flow).await?;
        on_poll(&flow, &outcome);
        match outcome {
            TokenPollOutcome::Pending => {}
            TokenPollOutcome::SlowDown => interval += 5,
            TokenPollOutcome::Granted(tokens) => return Ok(tokens),
            TokenPollOutcome::Denied => return Err(OAuthError::Provider("access denied".into())),
            TokenPollOutcome::Expired => {
                return Err(OAuthError::Provider("device code expired".into()));
            }
            TokenPollOutcome::Error(e) => return Err(OAuthError::Provider(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn endpoints_are_configured_per_provider() {
        assert!(
            OAuthProvider::MicrosoftGraph
                .device_code_endpoint()
                .starts_with("https://login.microsoftonline.com")
        );
        assert!(
            OAuthProvider::Google
                .device_code_endpoint()
                .starts_with("https://oauth2.googleapis.com")
        );
        assert_eq!(OAuthProvider::Dropbox.device_code_endpoint(), "");
    }

    #[test]
    fn wire_strings_stable() {
        assert_eq!(OAuthProvider::MicrosoftGraph.wire(), "microsoft-graph");
        assert_eq!(OAuthProvider::Google.wire(), "google");
        assert_eq!(OAuthProvider::Dropbox.wire(), "dropbox");
    }

    #[test]
    fn keychain_blob_round_trip_includes_expiry() {
        let tokens = OAuthTokenResponse {
            access_token: "at".into(),
            refresh_token: "rt".into(),
            token_type: "Bearer".into(),
            expires_in: 3600,
        };
        let blob = tokens.as_keychain_blob(1_000_000);
        assert_eq!(blob, "at\nrt\n1003600");
    }

    #[tokio::test]
    async fn dropbox_is_rejected_for_device_code() {
        let err = begin_device_code_flow(OAuthProvider::Dropbox, "cid", None)
            .await
            .expect_err("must reject");
        assert!(matches!(err, OAuthError::Unsupported("dropbox")));
    }
}
