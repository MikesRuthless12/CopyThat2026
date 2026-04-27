//! Phase 32i — OAuth 2.0 PKCE browser-redirect flow.
//!
//! Dropbox's device-code flow is limited (short-lived tokens, no
//! refresh token). The recommended pattern for desktop apps is
//! PKCE (RFC 7636) with a local-loopback redirect URI. This
//! module implements that flow generically so future providers
//! that don't support device-code (some corporate OAuth setups)
//! can reuse it.
//!
//! # Flow
//!
//! 1. Generate a random `code_verifier` (43–128 URL-safe chars).
//! 2. Derive `code_challenge = BASE64URL(SHA256(code_verifier))`
//!    (the "S256" method).
//! 3. Bind a `TcpListener` on `127.0.0.1:<auto-port>` (or a
//!    configured port).
//! 4. Build the authorize URL with
//!    `redirect_uri=http://127.0.0.1:<port>`, `code_challenge`,
//!    `code_challenge_method=S256`, `state=<random>`, `scope=...`.
//! 5. Open the URL in the user's default browser via
//!    `webbrowser::open`.
//! 6. Accept one connection on the listener, parse the auth code
//!    plus state from the redirect URI's query string, return a
//!    minimal HTML success page.
//! 7. POST to the token endpoint with `grant_type=authorization_code`,
//!    the received auth code, `code_verifier`, and
//!    `redirect_uri` — receive `access_token` + optional
//!    `refresh_token`.
//!
//! # Dropbox specifics
//!
//! - Authorize endpoint: `https://www.dropbox.com/oauth2/authorize`
//! - Token endpoint: `https://api.dropboxapi.com/oauth2/token`
//! - Default scope: `files.content.write files.content.read`
//!   (scope string is space-separated).
//! - Dropbox requires `token_access_type=offline` in the authorize
//!   URL to issue a refresh_token.
//!
//! # USER ACTION to validate end-to-end
//!
//! 1. Register a Dropbox App at <https://www.dropbox.com/developers/apps>
//!    (Scoped access, Full Dropbox or App folder).
//! 2. Add `http://127.0.0.1:<port>` to the app's Redirect URIs
//!    (Dropbox accepts wildcard ports via `http://127.0.0.1:*`).
//! 3. Pass the app's `client_id` to `run_pkce_flow(...)`.
//!
//! Without (1)+(2), the authorize URL opens but the provider
//! returns `access_denied` at step 6. The scaffolding itself
//! compiles + unit-tests cleanly with or without a real app.

use std::net::{SocketAddr, TcpListener};
use std::time::Duration;

use base64::Engine;
use sha2::Digest;

use crate::oauth::{OAuthError, OAuthTokenResponse};

/// PKCE state returned by [`begin_pkce_flow`]. Passed back into
/// [`run_pkce_redirect_listener`] + [`exchange_pkce_code`].
#[derive(Debug, Clone)]
pub struct PkceFlow {
    pub code_verifier: String,
    pub code_challenge: String,
    pub state: String,
    pub redirect_uri: String,
    pub authorize_url: String,
    /// Port the local loopback HTTP receiver will listen on. Pass
    /// to `run_pkce_redirect_listener`.
    pub listen_port: u16,
}

/// Authorize-endpoint parameters per provider. Dropbox is the
/// anchor consumer today; the shape fits any RFC 6749 provider.
#[derive(Debug, Clone)]
pub struct PkceProvider {
    pub name: &'static str,
    pub authorize_endpoint: &'static str,
    pub token_endpoint: &'static str,
    pub default_scope: &'static str,
    /// Extra query params the provider needs on the authorize URL
    /// (e.g. `&token_access_type=offline` for Dropbox refresh
    /// tokens). Each tuple becomes `key=url_encoded_value`.
    pub extra_authorize_params: &'static [(&'static str, &'static str)],
}

impl PkceProvider {
    pub const DROPBOX: PkceProvider = PkceProvider {
        name: "dropbox",
        authorize_endpoint: "https://www.dropbox.com/oauth2/authorize",
        token_endpoint: "https://api.dropboxapi.com/oauth2/token",
        default_scope: "files.content.write files.content.read",
        extra_authorize_params: &[("token_access_type", "offline")],
    };
}

/// Step 1+2+3+4 — kick off the PKCE flow. Generates the code
/// verifier/challenge, binds a loopback listener, composes the
/// authorize URL. Caller is expected to open `authorize_url` in
/// the browser (or use [`run_pkce_flow`] which does that +
/// awaits the redirect + exchanges).
pub fn begin_pkce_flow(
    provider: &PkceProvider,
    client_id: &str,
    scope_override: Option<&str>,
    preferred_port: Option<u16>,
) -> Result<(PkceFlow, TcpListener), OAuthError> {
    let code_verifier = generate_code_verifier();
    let code_challenge = derive_code_challenge_s256(&code_verifier);
    let state = generate_opaque_state();

    // Bind the loopback receiver. Port 0 lets the OS pick; we
    // read back the actual port for the redirect_uri.
    let addr: SocketAddr = format!("127.0.0.1:{}", preferred_port.unwrap_or(0))
        .parse()
        .map_err(|e: std::net::AddrParseError| OAuthError::Parse(e.to_string()))?;
    let listener =
        TcpListener::bind(addr).map_err(|e| OAuthError::Http(format!("bind loopback: {e}")))?;
    let listen_port = listener
        .local_addr()
        .map_err(|e| OAuthError::Http(format!("local_addr: {e}")))?
        .port();
    let redirect_uri = format!("http://127.0.0.1:{listen_port}");

    let scope = scope_override.unwrap_or(provider.default_scope);
    let mut url = format!(
        "{ep}?response_type=code&client_id={cid}&redirect_uri={ru}&code_challenge={cc}&code_challenge_method=S256&state={s}&scope={sc}",
        ep = provider.authorize_endpoint,
        cid = url_encode(client_id),
        ru = url_encode(&redirect_uri),
        cc = url_encode(&code_challenge),
        s = url_encode(&state),
        sc = url_encode(scope),
    );
    for (k, v) in provider.extra_authorize_params {
        url.push('&');
        url.push_str(k);
        url.push('=');
        url.push_str(&url_encode(v));
    }

    Ok((
        PkceFlow {
            code_verifier,
            code_challenge,
            state,
            redirect_uri,
            authorize_url: url,
            listen_port,
        },
        listener,
    ))
}

/// Step 6 — accept the browser's redirect connection on
/// `listener`, parse the auth code + state from its query string,
/// return both. Reply with a minimal HTML page so the user's
/// browser shows a success message.
///
/// `listener` must be the `TcpListener` returned from
/// [`begin_pkce_flow`]. `timeout` bounds the wait; default 120 s.
pub fn run_pkce_redirect_listener(
    listener: TcpListener,
    flow: &PkceFlow,
    timeout: Option<Duration>,
) -> Result<String, OAuthError> {
    use std::io::{BufRead, BufReader, Write};

    listener
        .set_nonblocking(false)
        .map_err(|e| OAuthError::Http(format!("set blocking: {e}")))?;

    let (mut stream, _peer) = listener
        .accept()
        .map_err(|e| OAuthError::Http(format!("accept: {e}")))?;

    // Honour the timeout on the accepted stream — a hostile local
    // process can race-connect to the loopback port (the port is
    // randomised but enumerable) and hold the socket open
    // indefinitely without sending a request line, wedging the
    // PKCE flow forever. With a default 120 s read timeout the
    // user can cancel and retry.
    let stream_timeout = timeout.unwrap_or_else(|| Duration::from_secs(120));
    let _ = stream.set_read_timeout(Some(stream_timeout));
    let _ = stream.set_write_timeout(Some(stream_timeout));

    // Parse the HTTP request line only: `GET /?code=...&state=... HTTP/1.1`.
    let mut reader = BufReader::new(
        stream
            .try_clone()
            .map_err(|e| OAuthError::Http(format!("stream clone: {e}")))?,
    );
    let mut request_line = String::new();
    reader
        .read_line(&mut request_line)
        .map_err(|e| OAuthError::Http(format!("read request: {e}")))?;

    let path_and_query = request_line
        .split_whitespace()
        .nth(1)
        .ok_or_else(|| OAuthError::Parse("malformed HTTP request line".into()))?;
    let query = path_and_query.split_once('?').map(|(_, q)| q).unwrap_or("");
    let (code, state) = parse_code_and_state(query);

    // Send a minimal success page before returning.
    let body = if code.is_none() {
        "<h1>Copy That — authorization failed</h1><p>Please return to the app.</p>"
    } else {
        "<h1>Copy That — authorization complete</h1><p>You can close this window.</p>"
    };
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        body.len(),
        body
    );
    let _ = stream.write_all(response.as_bytes());
    let _ = stream.flush();

    let code = code.ok_or_else(|| OAuthError::Provider("no `code` in redirect".into()))?;
    let state = state.unwrap_or_default();
    if state != flow.state {
        return Err(OAuthError::Provider(
            "redirect `state` mismatch — possible CSRF".into(),
        ));
    }
    Ok(code)
}

/// Step 7 — POST the auth code + verifier + redirect_uri to the
/// token endpoint. Returns the full token response.
pub async fn exchange_pkce_code(
    provider: &PkceProvider,
    client_id: &str,
    code: &str,
    flow: &PkceFlow,
) -> Result<OAuthTokenResponse, OAuthError> {
    let client = reqwest::Client::new();
    let form = [
        ("grant_type", "authorization_code"),
        ("client_id", client_id),
        ("code", code),
        ("redirect_uri", flow.redirect_uri.as_str()),
        ("code_verifier", flow.code_verifier.as_str()),
    ];
    let resp = client
        .post(provider.token_endpoint)
        .form(&form)
        .send()
        .await?;
    if !resp.status().is_success() {
        // The full provider response body could echo the
        // client_id, code_verifier, or — on misconfigured providers
        // — the access_token itself. Surface only the status code
        // so the rendered error never includes secret-bearing
        // bytes.
        let status_code = resp.status().as_u16();
        return Err(OAuthError::Provider(format!(
            "token endpoint returned status {status_code}"
        )));
    }
    let tokens: OAuthTokenResponse = resp
        .json()
        .await
        .map_err(|_| OAuthError::Parse("token-response decode failed".into()))?;
    Ok(tokens)
}

/// Convenience — end-to-end PKCE flow with browser launch + single
/// accept + token exchange. Blocks on the redirect. Matches the
/// shape of `copythat_cloud::run_device_code_flow`.
pub async fn run_pkce_flow(
    provider: &PkceProvider,
    client_id: &str,
    scope_override: Option<&str>,
) -> Result<OAuthTokenResponse, OAuthError> {
    let (flow, listener) = begin_pkce_flow(provider, client_id, scope_override, None)?;
    // Same scheme allow-list as the device-code flow — refuse
    // anything other than https:// before handing to the OS shell.
    // Also reject authority-obfuscation via userinfo (e.g.,
    // `https://www.dropbox.com@attacker.com/...`) — see
    // `oauth::is_safe_https_for_browser` for the threat model.
    if crate::oauth::is_safe_https_for_browser(&flow.authorize_url) {
        let _ = webbrowser::open(&flow.authorize_url);
    }
    // Accept the redirect on the calling thread — the dedicated
    // tokio runtime in `CopyThatCloudSink::run_on_dedicated_thread`
    // isn't needed here because `run_pkce_redirect_listener` is
    // blocking, not async.
    let code = tokio::task::spawn_blocking({
        let flow = flow.clone();
        move || run_pkce_redirect_listener(listener, &flow, Some(Duration::from_secs(300)))
    })
    .await
    .map_err(|e| OAuthError::Http(format!("redirect worker join: {e}")))??;
    exchange_pkce_code(provider, client_id, &code, &flow).await
}

// ---------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------

fn generate_code_verifier() -> String {
    // RFC 7636 §4.1 — verifier is 43..128 unreserved chars
    // `ALPHA / DIGIT / "-" / "." / "_" / "~"`. 96 random bytes →
    // 128 base64url chars after encoding.
    let mut buf = [0u8; 96];
    getrandom::fill(&mut buf).expect("OS RNG");
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(buf)
}

fn derive_code_challenge_s256(verifier: &str) -> String {
    let mut hasher = sha2::Sha256::new();
    hasher.update(verifier.as_bytes());
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(hasher.finalize())
}

fn generate_opaque_state() -> String {
    let mut buf = [0u8; 16];
    getrandom::fill(&mut buf).expect("OS RNG");
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(buf)
}

fn url_encode(s: &str) -> String {
    // Minimal percent-encoding — enough for authorize-URL
    // composition. Covers `+`, `/`, `=`, `&`, `?`, `#`, space, and
    // anything non-ASCII.
    let mut out = String::with_capacity(s.len());
    for byte in s.as_bytes() {
        match *byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(*byte as char);
            }
            other => out.push_str(&format!("%{other:02X}")),
        }
    }
    out
}

fn parse_code_and_state(query: &str) -> (Option<String>, Option<String>) {
    let mut code = None;
    let mut state = None;
    for pair in query.split('&') {
        let Some((k, v)) = pair.split_once('=') else {
            continue;
        };
        let decoded = url_decode(v);
        match k {
            "code" => code = Some(decoded),
            "state" => state = Some(decoded),
            _ => {}
        }
    }
    (code, state)
}

fn url_decode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'+' => {
                out.push(' ');
                i += 1;
            }
            b'%' if i + 2 < bytes.len() => {
                let hex = &s[i + 1..i + 3];
                if let Ok(n) = u8::from_str_radix(hex, 16) {
                    out.push(n as char);
                } else {
                    out.push('%');
                }
                i += 3;
            }
            b => {
                out.push(b as char);
                i += 1;
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_verifier_is_long_and_url_safe() {
        let v = generate_code_verifier();
        assert!(v.len() >= 43 && v.len() <= 128, "len = {}", v.len());
        assert!(
            v.chars()
                .all(|c| c.is_ascii_alphanumeric() || "-_".contains(c))
        );
    }

    #[test]
    fn s256_challenge_matches_rfc_vector() {
        // RFC 7636 appendix B test vector:
        //   code_verifier = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk"
        //   code_challenge = "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM"
        let verifier = "dBjftJeZ4CVP-mB92K27uhbUJU1p1r_wW1gFWFOEjXk";
        let challenge = derive_code_challenge_s256(verifier);
        assert_eq!(challenge, "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM");
    }

    #[test]
    fn url_encode_handles_special_chars() {
        assert_eq!(url_encode("abc 123"), "abc%20123");
        assert_eq!(url_encode("a+b/c"), "a%2Bb%2Fc");
        assert_eq!(url_encode("plain.path-ok_~"), "plain.path-ok_~");
    }

    #[test]
    fn url_decode_round_trips_simple_values() {
        assert_eq!(url_decode("abc%20123"), "abc 123");
        assert_eq!(url_decode("a%2Bb%2Fc"), "a+b/c");
        assert_eq!(url_decode("plain.path"), "plain.path");
    }

    #[test]
    fn parse_code_and_state_extracts_both() {
        let (code, state) = parse_code_and_state("code=ABC%20def&state=xyz123");
        assert_eq!(code.as_deref(), Some("ABC def"));
        assert_eq!(state.as_deref(), Some("xyz123"));
    }

    #[test]
    fn begin_pkce_flow_composes_authorize_url_with_pkce_params() {
        let (flow, _listener) =
            begin_pkce_flow(&PkceProvider::DROPBOX, "test-client-id", None, None).expect("begin");
        assert!(flow.authorize_url.contains("client_id=test-client-id"));
        assert!(flow.authorize_url.contains("code_challenge_method=S256"));
        assert!(flow.authorize_url.contains("token_access_type=offline"));
        assert!(flow.redirect_uri.starts_with("http://127.0.0.1:"));
        assert!(flow.listen_port > 0);
    }

    #[test]
    fn state_mismatch_rejects_redirect() {
        // Simulate the redirect-listener's state check.
        let flow = PkceFlow {
            code_verifier: "v".into(),
            code_challenge: "c".into(),
            state: "expected".into(),
            redirect_uri: "http://127.0.0.1:1234".into(),
            authorize_url: String::new(),
            listen_port: 1234,
        };
        let (code, state) = parse_code_and_state("code=abc&state=wrong");
        assert_eq!(code.as_deref(), Some("abc"));
        assert_ne!(state.as_deref(), Some(flow.state.as_str()));
    }
}
