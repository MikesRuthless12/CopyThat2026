//! Bearer-token authentication.
//!
//! Two accepted forms, in this order:
//!
//! 1. `Authorization: Bearer <token>` — the canonical, terminal-
//!    friendly form a desktop browser sends after the user pasted the
//!    token into a "Token" prompt.
//! 2. `?t=<token>` query parameter — the form the Tauri "Open
//!    recovery UI" button uses so a single click opens the browser
//!    already logged in. Only accepted because the recovery surface
//!    is loopback-by-default; opening it to a non-loopback bind
//!    forces the user to acknowledge the same warning that gates the
//!    setting.
//!
//! Both forms are compared in constant time via `subtle::ConstantTimeEq`
//! so a 401-loop attacker cannot recover the token byte-by-byte by
//! timing the response.

use axum::extract::Request;
use axum::http::{StatusCode, header};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use secrecy::ExposeSecret;
use subtle::ConstantTimeEq;

use crate::server::ServerState;

/// Parsed credential. The token bytes are borrowed from the request
/// so the parser is allocation-free on the hot path.
#[derive(Debug)]
struct Presented<'a> {
    bytes: &'a [u8],
}

impl<'a> Presented<'a> {
    fn from_request(req: &'a Request) -> Option<Self> {
        if let Some(value) = req.headers().get(header::AUTHORIZATION)
            && let Ok(s) = value.to_str()
            && let Some(rest) = s.strip_prefix("Bearer ")
        {
            return Some(Self {
                bytes: rest.as_bytes(),
            });
        }
        // Query string: scan for `t=` without pulling in a percent-
        // decode dep. Token bytes are ASCII (`base32` Crockford), no
        // decoding needed.
        if let Some(q) = req.uri().query() {
            for kv in q.split('&') {
                if let Some(v) = kv.strip_prefix("t=") {
                    return Some(Self {
                        bytes: v.as_bytes(),
                    });
                }
            }
        }
        None
    }
}

/// Tower middleware enforcing the bearer-token check. Any request that
/// doesn't present the right token gets a 401 + `WWW-Authenticate`
/// challenge. Static assets share the same gate so an unauthenticated
/// browser can't probe the route surface.
pub(crate) async fn require_token(
    axum::extract::State(state): axum::extract::State<ServerState>,
    req: Request,
    next: Next,
) -> Response {
    let presented = Presented::from_request(&req);
    let expected = state.token.expose_secret().as_bytes();
    let ok = match presented {
        Some(p) => p.bytes.ct_eq(expected).into(),
        None => false,
    };
    if !ok {
        return unauthorized();
    }
    next.run(req).await
}

fn unauthorized() -> Response {
    let body = "401 — recovery token required";
    (
        StatusCode::UNAUTHORIZED,
        [
            (
                header::WWW_AUTHENTICATE,
                "Bearer realm=\"copythat-recovery\"",
            ),
            (header::CONTENT_TYPE, "text/plain; charset=utf-8"),
        ],
        body,
    )
        .into_response()
}

/// Generate a 20-byte random token, render as Crockford-base32 (32
/// chars). Same alphabet as the mobile pairing token (Phase 37) so the
/// token reads cleanly when typed by hand.
pub fn generate_token() -> String {
    let mut bytes = [0u8; 20];
    // getrandom is the same crate Phase 37 uses for the SAS seed; on
    // Windows it backs onto BCryptGenRandom, on Linux getrandom(2),
    // on macOS arc4random_buf.
    getrandom::fill(&mut bytes).expect("system RNG must be available");
    base32::encode(base32::Alphabet::Crockford, &bytes)
}
