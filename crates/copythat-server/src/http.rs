//! Phase 48 — the shared axum/hyper listener for the HTTP-family
//! protocols (WebDAV today; HTTP and S3 share it) plus `/metrics`.
//!
//! WebDAV is delegated to `dav-server`'s [`DavHandler`] over a
//! [`LocalFs`] rooted at [`ServerConfig::root`]. The handler is mounted as
//! the router *fallback* so every WebDAV method/path (`PROPFIND`, `PUT`,
//! `MKCOL`, …) reaches it, while `/metrics` keeps a dedicated `GET` route.
//! Writes bump the shared [`MetricsRegistry`]; `readonly` rejects write
//! methods before they touch the filesystem; `auth` gates everything
//! except `/metrics` (left open for scrapers).

use std::path::Path;
use std::sync::Arc;

use axum::Router;
use axum::extract::{Request, State};
use axum::http::{HeaderMap, Method, StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use dav_server::DavHandler;
use dav_server::fakels::FakeLs;
use dav_server::localfs::LocalFs;

use crate::{AuthMode, MetricsRegistry, Protocol, ServerConfig};

/// State shared with every request handler.
#[derive(Clone)]
struct HttpState {
    metrics: Arc<MetricsRegistry>,
    auth: Arc<AuthMode>,
    readonly: bool,
    /// `Some` when WebDAV (or, later, plain HTTP/S3 file access) is enabled.
    dav: Option<Arc<DavHandler>>,
}

/// Build the axum router for the configured HTTP-family protocols.
pub(crate) fn build_router(config: &ServerConfig, metrics: Arc<MetricsRegistry>) -> Router {
    // WebDAV, plain HTTP, and S3 all expose the same local filesystem, so a
    // single dav-backed handler serves any of them today.
    let serves_files = config
        .protocols
        .iter()
        .any(|p| matches!(p, Protocol::WebDav | Protocol::Http | Protocol::S3));
    let dav = serves_files.then(|| Arc::new(build_dav(&config.root)));

    let state = HttpState {
        metrics,
        auth: Arc::new(config.auth.clone()),
        readonly: config.readonly,
        dav,
    };

    Router::new()
        .route("/metrics", get(metrics_handler))
        .fallback(dav_fallback)
        .with_state(state)
}

/// A `DavHandler` over the local filesystem rooted at `root`, with a fake
/// lock system so Windows / macOS WebDAV clients see locking support.
fn build_dav(root: &Path) -> DavHandler {
    DavHandler::builder()
        .filesystem(LocalFs::new(root, false, false, cfg!(target_os = "macos")))
        .locksystem(FakeLs::new())
        .build_handler()
}

/// `GET /metrics` — Prometheus text exposition. Intentionally unauthenticated
/// so a scraper can read it without the file-access credential.
async fn metrics_handler(State(state): State<HttpState>) -> Response {
    let body = state.metrics.snapshot().render_prometheus();
    (
        [(
            header::CONTENT_TYPE,
            "text/plain; version=0.0.4; charset=utf-8",
        )],
        body,
    )
        .into_response()
}

/// Fallback — every non-`/metrics` request. Authenticates, enforces
/// `readonly`, then delegates to the WebDAV handler, counting successful
/// writes into the metrics registry.
async fn dav_fallback(State(state): State<HttpState>, req: Request) -> Response {
    if let Some(resp) = check_auth(&state.auth, req.headers()) {
        return resp;
    }
    let Some(dav) = state.dav.clone() else {
        return (StatusCode::NOT_FOUND, "no file protocol enabled").into_response();
    };

    let method = req.method().clone();
    if state.readonly && is_write_method(&method) {
        return (StatusCode::FORBIDDEN, "server is read-only").into_response();
    }
    let content_len = req
        .headers()
        .get(header::CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<u64>().ok());

    state.metrics.inc_active();
    let dav_resp = dav.handle(req).await;
    state.metrics.dec_active();

    let status = dav_resp.status();
    if status.is_success() && method == Method::PUT {
        state.metrics.record_copy(content_len.unwrap_or(0));
    } else if status.is_server_error() {
        state.metrics.record_error();
    }

    // dav-server's `Body` is `http_body::Body<Data = Bytes, Error = io::Error>`,
    // which axum's `Body::new` wraps directly.
    let (parts, body) = dav_resp.into_parts();
    Response::from_parts(parts, axum::body::Body::new(body))
}

/// WebDAV / HTTP methods that mutate the filesystem.
fn is_write_method(method: &Method) -> bool {
    matches!(
        method.as_str(),
        "PUT" | "DELETE" | "MKCOL" | "MOVE" | "COPY" | "PROPPATCH" | "LOCK" | "UNLOCK" | "POST"
    )
}

/// Enforce the configured auth mode against a request's headers. Returns
/// `Some(401)` when the request should be rejected, `None` when it passes.
fn check_auth(auth: &AuthMode, headers: &HeaderMap) -> Option<Response> {
    let authorized = match auth {
        AuthMode::None => true,
        AuthMode::Bearer { token } => headers
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "))
            .map(|t| ct_eq(t, token))
            .unwrap_or(false),
        AuthMode::Basic { user, password } => headers
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Basic "))
            .and_then(|b64| BASE64.decode(b64.trim()).ok())
            .and_then(|bytes| String::from_utf8(bytes).ok())
            .and_then(|creds| {
                creds
                    .split_once(':')
                    .map(|(u, p)| ct_eq(u, user) & ct_eq(p, password))
            })
            .unwrap_or(false),
    };
    let scheme = match auth {
        AuthMode::Basic { .. } => "Basic",
        _ => "Bearer",
    };
    (!authorized).then(|| unauthorized(scheme))
}

/// A `401` carrying the `WWW-Authenticate` challenge for `scheme`.
fn unauthorized(scheme: &str) -> Response {
    (
        StatusCode::UNAUTHORIZED,
        [(
            header::WWW_AUTHENTICATE,
            format!("{scheme} realm=\"CopyThat\""),
        )],
        "unauthorized",
    )
        .into_response()
}

/// Length-independent-content constant-time string compare (the length is
/// not secret; the contents are). Avoids leaking the token via early-exit
/// timing on the first mismatching byte.
fn ct_eq(a: &str, b: &str) -> bool {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b) {
        diff |= x ^ y;
    }
    diff == 0
}
