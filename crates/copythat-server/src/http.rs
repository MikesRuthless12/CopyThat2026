//! Phase 48 — the shared axum/hyper listener for the file-serving
//! protocols (WebDAV and plain HTTP GET/PUT) plus `/metrics`.
//!
//! File access is delegated to `dav-server`'s [`DavHandler`] over a
//! [`LocalFs`] rooted at [`ServerConfig::root`]. The handler is mounted as
//! the router *fallback* so every method/path (`PROPFIND`, `PUT`, `GET`,
//! `MKCOL`, …) reaches it, while `/metrics` keeps a dedicated `GET` route.
//! Writes bump the shared [`MetricsRegistry`]; `readonly` rejects write
//! methods before they touch the filesystem; `auth` gates everything
//! except `/metrics` (left open for scrapers). S3 (a distinct REST/XML
//! API) and SFTP are rejected by [`crate::serve`], not served here.

use std::path::Path;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::task::{Context, Poll};

use axum::Router;
use axum::extract::{Request, State};
use axum::http::{HeaderMap, Method, StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use bytes::Bytes;
use dav_server::DavHandler;
use dav_server::fakels::FakeLs;
use dav_server::localfs::LocalFs;
use http_body::{Body as HttpBody, Frame};
use subtle::ConstantTimeEq;

use crate::{AuthMode, MetricsRegistry, Protocol, ServerConfig};

/// State shared with every request handler.
#[derive(Clone)]
struct HttpState {
    metrics: Arc<MetricsRegistry>,
    auth: Arc<AuthMode>,
    readonly: bool,
    /// `Some` when WebDAV or plain-HTTP file access is enabled.
    dav: Option<Arc<DavHandler>>,
}

/// Build the axum router for the file-serving protocols.
pub(crate) fn build_router(config: &ServerConfig, metrics: Arc<MetricsRegistry>) -> Router {
    // WebDAV and plain HTTP expose the same local filesystem (HTTP GET/PUT
    // is a subset of WebDAV), so one dav-backed handler serves both. S3 and
    // SFTP are rejected earlier in `serve()`, never reaching here.
    let serves_files = config
        .protocols
        .iter()
        .any(|p| matches!(p, Protocol::WebDav | Protocol::Http));
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
/// writes (by bytes actually streamed) into the metrics registry.
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

    // Count the bytes dav-server actually reads from the request body — works
    // for both Content-Length and chunked-transfer uploads, unlike trusting
    // the header.
    let counter = Arc::new(AtomicU64::new(0));
    let (parts, body) = req.into_parts();
    let req = Request::from_parts(
        parts,
        CountingBody {
            inner: body,
            counter: counter.clone(),
        },
    );

    // RAII so `active_jobs` is balanced even if this future is dropped
    // (client disconnects) before `dav.handle` returns.
    let _active = ActiveGuard::new(state.metrics.clone());
    let dav_resp = dav.handle(req).await;

    let status = dav_resp.status();
    if status.is_success() && method == Method::PUT {
        state.metrics.record_copy(counter.load(Ordering::Relaxed));
    } else if status.is_server_error() {
        state.metrics.record_error();
    }

    // dav-server's `Body` is `http_body::Body<Data = Bytes, Error = io::Error>`,
    // which axum's `Body::new` wraps directly.
    let (parts, body) = dav_resp.into_parts();
    Response::from_parts(parts, axum::body::Body::new(body))
}

/// RAII active-request guard: increments `active_jobs` on construction and
/// decrements on drop, so the gauge stays balanced even when the request
/// future is cancelled (client disconnect) before the handler completes.
struct ActiveGuard(Arc<MetricsRegistry>);

impl ActiveGuard {
    fn new(metrics: Arc<MetricsRegistry>) -> Self {
        metrics.inc_active();
        Self(metrics)
    }
}

impl Drop for ActiveGuard {
    fn drop(&mut self) {
        self.0.dec_active();
    }
}

/// An `http_body::Body` wrapper that tallies the bytes streamed through it
/// into a shared counter, so a PUT's recorded size is the bytes actually
/// written (no Content-Length required).
struct CountingBody<B> {
    inner: B,
    counter: Arc<AtomicU64>,
}

impl<B> HttpBody for CountingBody<B>
where
    B: HttpBody<Data = Bytes> + Unpin,
{
    type Data = Bytes;
    type Error = B::Error;

    fn poll_frame(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Bytes>, Self::Error>>> {
        let this = &mut *self;
        match Pin::new(&mut this.inner).poll_frame(cx) {
            Poll::Ready(Some(Ok(frame))) => {
                if let Some(data) = frame.data_ref() {
                    this.counter.fetch_add(data.len() as u64, Ordering::Relaxed);
                }
                Poll::Ready(Some(Ok(frame)))
            }
            other => other,
        }
    }
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

/// Constant-time string compare via the workspace's `subtle` primitive
/// (also used by copythat-audit / -recovery / -mobile). The byte length is
/// not secret; the contents are — `ct_eq` doesn't early-exit on the first
/// differing byte.
fn ct_eq(a: &str, b: &str) -> bool {
    a.as_bytes().ct_eq(b.as_bytes()).into()
}
