//! Phase 48 — server mode + observability.
//!
//! Runs CopyThat headless as a file-serving endpoint with a Prometheus
//! `/metrics` surface and webhook notifications:
//!
//! - [`serve`] binds an axum/hyper server on [`ServerConfig::bind_addr`]
//!   and exposes the configured HTTP-family protocols (WebDAV today; HTTP
//!   and S3 share the same listener) plus `/metrics`. WebDAV is backed by
//!   `dav-server`'s local filesystem over [`ServerConfig::root`], honouring
//!   [`ServerConfig::readonly`] and [`ServerConfig::auth`].
//! - [`MetricsRegistry`] is the live counter set the `/metrics` endpoint
//!   renders via [`Metrics::render_prometheus`]; protocol handlers bump it
//!   as files move.
//! - [`format_webhook_payload`] / [`send_webhook`] build and deliver the
//!   Slack / Discord / ntfy / Pushover notification bodies.
//!
//! The loopback-server shape (synchronous-feeling bind that surfaces a
//! port-in-use error from `serve`, a [`ServerHandle`] with `local_addr`
//! + graceful shutdown) mirrors the Phase 39 recovery server.

#![forbid(unsafe_code)]

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::oneshot;

mod http;
mod otel;
mod s3;
mod sftp;
pub mod webhook;

pub use otel::{OtelError, OtelGuard, install_otel};
pub use webhook::{PushoverCreds, WebhookSink, send_webhook};

/// Protocols the server can expose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Protocol {
    WebDav,
    Sftp,
    Http,
    S3,
}

impl Protocol {
    /// Human-facing label (canonical capitalisation).
    pub fn label(self) -> &'static str {
        match self {
            Self::WebDav => "WebDAV",
            Self::Sftp => "SFTP",
            Self::Http => "HTTP",
            Self::S3 => "S3",
        }
    }

    /// Whether this protocol is served over the shared WebDAV/HTTP file
    /// handler on the axum/hyper listener. WebDAV and plain HTTP map onto the
    /// same handler; SFTP runs on its own SSH transport and S3 (a distinct
    /// REST/XML API) runs on its own axum router — both are served by
    /// [`serve`], but neither shares the WebDAV/HTTP handler, so both are
    /// *not* HTTP-family.
    pub fn is_http_family(self) -> bool {
        matches!(self, Self::WebDav | Self::Http)
    }
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.label())
    }
}

/// How the server authenticates clients. mTLS is intentionally omitted —
/// the loopback / homelab deployment terminates TLS at a reverse proxy;
/// the server itself speaks plaintext to that proxy or to localhost.
#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum AuthMode {
    /// Open access on the bind address (loopback-only by default).
    #[default]
    None,
    /// `Authorization: Bearer <token>`.
    Bearer { token: String },
    /// HTTP Basic — `Authorization: Basic base64(user:password)`.
    Basic { user: String, password: String },
}

// Manual `Debug` that redacts the secret material, so it never leaks
// through `ServerConfig` / `ServerHandle`'s derived `Debug` (a stray
// `tracing::debug!(?config)` or a panic backtrace). The username is not a
// secret; the token / password are.
impl std::fmt::Debug for AuthMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::Bearer { .. } => f.write_str(r#"Bearer { token: "<redacted>" }"#),
            Self::Basic { user, .. } => {
                write!(f, r#"Basic {{ user: {user:?}, password: "<redacted>" }}"#)
            }
        }
    }
}

/// Server configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Bind address, e.g. `"127.0.0.1:8080"`.
    pub bind_addr: String,
    /// Which protocols to expose.
    pub protocols: Vec<Protocol>,
    /// How clients authenticate.
    #[serde(default)]
    pub auth: AuthMode,
    /// Filesystem root the server exposes.
    #[serde(default)]
    pub root: PathBuf,
    /// Refuse write methods (PUT / DELETE / MKCOL / MOVE / COPY / ...).
    #[serde(default)]
    pub readonly: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_addr: "127.0.0.1:8080".to_string(),
            protocols: Vec::new(),
            auth: AuthMode::None,
            root: PathBuf::from("."),
            readonly: false,
        }
    }
}

/// OpenTelemetry export configuration, consumed by [`install_otel`].
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct OtelConfig {
    /// OTLP/HTTP traces endpoint, used verbatim — pass the full traces path,
    /// e.g. `"http://localhost:4318/v1/traces"`.
    pub endpoint: String,
    /// Whether trace export is enabled.
    pub enabled: bool,
}

/// Handle to a running server: the bound address, the serving task, and a
/// graceful-shutdown trigger. Dropping it leaves the server running until
/// the process exits; call [`shutdown`](Self::shutdown) to drain cleanly.
#[derive(Debug)]
pub struct ServerHandle {
    config: ServerConfig,
    local_addr: SocketAddr,
    task: tokio::task::JoinHandle<()>,
    shutdown: Option<oneshot::Sender<()>>,
    metrics: Arc<MetricsRegistry>,
}

impl ServerHandle {
    /// The effective config the server is running with.
    pub fn config(&self) -> &ServerConfig {
        &self.config
    }

    /// The socket the HTTP listener actually bound to (reflects an
    /// OS-assigned port when the config used `:0`).
    pub fn local_addr(&self) -> SocketAddr {
        self.local_addr
    }

    /// The live metrics registry the protocol handlers update.
    pub fn metrics(&self) -> Arc<MetricsRegistry> {
        self.metrics.clone()
    }

    /// Trigger a graceful shutdown and wait for the server task to drain.
    pub async fn shutdown(mut self) {
        if let Some(tx) = self.shutdown.take() {
            let _ = tx.send(());
        }
        let _ = self.task.await;
    }

    /// Force-cancel the server task without draining.
    pub fn abort(self) {
        self.task.abort();
    }
}

/// Server / observability errors.
#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum ServerError {
    /// No protocols were configured, so there's nothing to serve.
    #[error("no protocols configured")]
    NoProtocols,
    /// The bind address was invalid or already in use.
    #[error("failed to bind {addr}: {message}")]
    Bind { addr: String, message: String },
    /// A configured protocol isn't served yet (e.g. SFTP in the HTTP-only
    /// build increment).
    #[error("server protocol {protocol} not yet implemented")]
    NotImplemented { protocol: Protocol },
    /// Webhook delivery failed.
    #[error("webhook delivery failed: {0}")]
    Webhook(String),
}

impl ServerError {
    /// Stable Fluent key, matching the engine's `localized_key` convention.
    pub fn localized_key(&self) -> &'static str {
        match self {
            Self::NoProtocols => "err-server-no-protocols",
            Self::Bind { .. } => "err-server-bind",
            Self::NotImplemented { .. } => "err-server-not-implemented",
            Self::Webhook(_) => "err-webhook-failed",
        }
    }
}

/// Whether a server bound to `addr` with `auth` exposes the served
/// directory to the network with no authentication — a non-loopback bind
/// plus [`AuthMode::None`]. Used to emit a startup security warning; shared
/// so the CLI and the library agree on the exact condition.
pub fn exposes_unauthenticated(addr: &SocketAddr, auth: &AuthMode) -> bool {
    !addr.ip().is_loopback() && matches!(auth, AuthMode::None)
}

/// Start the server.
///
/// Binds the HTTP listener on [`ServerConfig::bind_addr`] (a port-in-use
/// error surfaces here, not in the background task) and spawns the axum
/// server on the current tokio runtime, serving every configured
/// HTTP-family protocol ([`Protocol::is_http_family`]) plus `/metrics`.
/// Returns a [`ServerHandle`] whose [`local_addr`](ServerHandle::local_addr)
/// reflects the OS-assigned port when the config used `:0`.
///
/// SFTP ([`Protocol::Sftp`]) is served over its own SSH transport (russh +
/// russh-sftp) on the same bound socket; a config with *only* SFTP spawns
/// that server instead of the axum stack. S3 ([`Protocol::S3`]) is served
/// over its own axum router (path-style, single implicit bucket = the served
/// root). Neither SFTP nor S3 can share one listener with the WebDAV/HTTP
/// handler (or with each other), so any mixed config yields
/// [`ServerError::Bind`]. S3 has no bearer concept, so an S3 +
/// [`AuthMode::Bearer`] config is likewise rejected with [`ServerError::Bind`].
pub async fn serve(config: ServerConfig) -> Result<ServerHandle, ServerError> {
    if config.protocols.is_empty() {
        return Err(ServerError::NoProtocols);
    }

    // SFTP speaks SSH and S3 has its own REST/XML semantics, so neither can
    // share the single bound listener with the WebDAV/HTTP handler — keep
    // each exclusive to one transport per `serve`.
    let wants_sftp = config.protocols.contains(&Protocol::Sftp);
    let wants_s3 = config.protocols.contains(&Protocol::S3);
    let wants_http = config.protocols.iter().any(|p| p.is_http_family());
    if wants_sftp && wants_http {
        return Err(ServerError::Bind {
            addr: config.bind_addr.clone(),
            message: "SFTP and HTTP-family protocols cannot share one bind address; \
                      run them as separate servers"
                .to_string(),
        });
    }
    if wants_s3 && (wants_http || wants_sftp) {
        return Err(ServerError::Bind {
            addr: config.bind_addr.clone(),
            message: "S3 is a distinct transport and cannot share one bind address with \
                      WebDAV/HTTP or SFTP; run it as a separate server"
                .to_string(),
        });
    }
    // S3 authenticates via AWS SigV4 (access-key-id / secret = Basic's
    // user / password); a bearer token has no place in the S3 request
    // signing model, so reject the combination rather than serve it open.
    if wants_s3 && matches!(config.auth, AuthMode::Bearer { .. }) {
        return Err(ServerError::Bind {
            addr: config.bind_addr.clone(),
            message: "S3 does not support bearer auth; use basic auth (access-key-id / \
                      secret) for SigV4, or no auth"
                .to_string(),
        });
    }

    let addr: SocketAddr = config.bind_addr.parse().map_err(|e| ServerError::Bind {
        addr: config.bind_addr.clone(),
        message: format!("invalid address: {e}"),
    })?;

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| ServerError::Bind {
            addr: config.bind_addr.clone(),
            message: e.to_string(),
        })?;
    let local_addr = listener.local_addr().map_err(|e| ServerError::Bind {
        addr: config.bind_addr.clone(),
        message: e.to_string(),
    })?;

    // Security advisory: a non-loopback bind with no authentication exposes
    // the served root to every host that can reach this address (read/write
    // unless `readonly`). Loud-warn rather than refuse — a trusted LAN or a
    // reverse proxy terminating auth in front is a legitimate deployment.
    if exposes_unauthenticated(&local_addr, &config.auth) {
        let access = if config.readonly {
            "read-only"
        } else {
            "read/write"
        };
        tracing::warn!(
            %local_addr,
            "serving with NO authentication on a non-loopback address: any host that \
             can reach it has {access} access to the served directory — set bearer or \
             basic auth, or bind a loopback address"
        );
    }

    let metrics = Arc::new(MetricsRegistry::default());
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    let task = if wants_sftp {
        // SFTP over its own SSH transport on the same bound socket. The
        // metrics registry isn't wired into the SFTP handler in this
        // increment, so `/metrics` would read zero — there's no scrape
        // surface on an SSH listener anyway.
        sftp::spawn(&config, listener, shutdown_rx)?
    } else {
        // S3 and WebDAV/HTTP both ride the axum stack but build different
        // routers; S3 is a distinct REST/XML surface with its own path jail.
        let router = if wants_s3 {
            s3::build_router(&config, metrics.clone())?
        } else {
            http::build_router(&config, metrics.clone())
        };
        tokio::spawn(async move {
            let server = axum::serve(listener, router.into_make_service()).with_graceful_shutdown(
                async move {
                    let _ = shutdown_rx.await;
                },
            );
            if let Err(e) = server.await {
                tracing::warn!(error = ?e, "copythat server task ended with error");
            }
        })
    };

    Ok(ServerHandle {
        config,
        local_addr,
        task,
        shutdown: Some(shutdown_tx),
        metrics,
    })
}

/// Live, atomically-updated engine counters the `/metrics` endpoint
/// renders. Cloneable by `Arc`; every protocol handler shares one.
#[derive(Debug, Default)]
pub struct MetricsRegistry {
    jobs_total: AtomicU64,
    files_copied_total: AtomicU64,
    bytes_copied_total: AtomicU64,
    errors_total: AtomicU64,
    active_jobs: AtomicU64,
}

impl MetricsRegistry {
    /// Record a completed file write (a WebDAV/HTTP/S3 PUT): one job, one
    /// file, `bytes` bytes.
    pub fn record_copy(&self, bytes: u64) {
        self.jobs_total.fetch_add(1, Ordering::Relaxed);
        self.files_copied_total.fetch_add(1, Ordering::Relaxed);
        self.bytes_copied_total.fetch_add(bytes, Ordering::Relaxed);
    }

    /// Record a server-side error response.
    pub fn record_error(&self) {
        self.errors_total.fetch_add(1, Ordering::Relaxed);
    }

    /// Mark an in-flight request started.
    pub fn inc_active(&self) {
        self.active_jobs.fetch_add(1, Ordering::Relaxed);
    }

    /// Mark an in-flight request finished.
    pub fn dec_active(&self) {
        // Saturating: never wrap below zero on a double-dec.
        let _ = self
            .active_jobs
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |v| {
                Some(v.saturating_sub(1))
            });
    }

    /// Snapshot the counters into a renderable [`Metrics`].
    pub fn snapshot(&self) -> Metrics {
        Metrics {
            jobs_total: self.jobs_total.load(Ordering::Relaxed),
            files_copied_total: self.files_copied_total.load(Ordering::Relaxed),
            bytes_copied_total: self.bytes_copied_total.load(Ordering::Relaxed),
            errors_total: self.errors_total.load(Ordering::Relaxed),
            active_jobs: self.active_jobs.load(Ordering::Relaxed),
        }
    }
}

/// A job-lifecycle notification fanned out to webhook sinks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JobNotification {
    /// Event kind, e.g. `"job_completed"` (also used as the ntfy topic).
    pub kind: String,
    pub title: String,
    pub body: String,
    /// Whether the job succeeded (drives the status glyph).
    pub ok: bool,
}

/// Webhook destinations. Payload formatting is [`format_webhook_payload`];
/// delivery is [`send_webhook`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WebhookTarget {
    Slack,
    Discord,
    Ntfy,
    Pushover,
}

/// Build the service-specific JSON body for `target` from `event`. Pure —
/// the actual HTTP POST is [`send_webhook`]. `token`/`user` for Pushover
/// are placeholders here; delivery fills them from config.
pub fn format_webhook_payload(target: WebhookTarget, event: &JobNotification) -> serde_json::Value {
    let status = if event.ok { "OK" } else { "FAILED" };
    let text = format!("[{status}] {} — {}", event.title, event.body);
    match target {
        WebhookTarget::Slack => serde_json::json!({ "text": text }),
        WebhookTarget::Discord => serde_json::json!({ "content": text }),
        WebhookTarget::Ntfy => serde_json::json!({
            "topic": event.kind,
            "title": event.title,
            "message": event.body,
        }),
        WebhookTarget::Pushover => serde_json::json!({
            "token": "",
            "user": "",
            "title": event.title,
            "message": event.body,
        }),
    }
}

/// Engine metrics surfaced by the `/metrics` endpoint.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Metrics {
    pub jobs_total: u64,
    pub files_copied_total: u64,
    pub bytes_copied_total: u64,
    pub errors_total: u64,
    pub active_jobs: u64,
}

impl Metrics {
    /// Render the Prometheus text-exposition format: a `# HELP` + `# TYPE`
    /// line precede each sample, and every series carries the `copythat_`
    /// prefix.
    pub fn render_prometheus(&self) -> String {
        let counters: [(&str, &str, u64); 4] = [
            (
                "copythat_jobs_total",
                "Total copy/move jobs run.",
                self.jobs_total,
            ),
            (
                "copythat_files_copied_total",
                "Total files copied.",
                self.files_copied_total,
            ),
            (
                "copythat_bytes_copied_total",
                "Total bytes copied.",
                self.bytes_copied_total,
            ),
            (
                "copythat_errors_total",
                "Total errors surfaced.",
                self.errors_total,
            ),
        ];
        let mut out = String::new();
        for (name, help, val) in counters {
            out.push_str(&format!(
                "# HELP {name} {help}\n# TYPE {name} counter\n{name} {val}\n"
            ));
        }
        out.push_str(&format!(
            "# HELP copythat_active_jobs Jobs currently running.\n\
             # TYPE copythat_active_jobs gauge\n\
             copythat_active_jobs {}\n",
            self.active_jobs
        ));
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unauthenticated_exposure_needs_nonloopback_and_no_auth() {
        let loopback: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        let any: SocketAddr = "0.0.0.0:8080".parse().unwrap();
        let lan: SocketAddr = "192.168.1.10:8080".parse().unwrap();
        let bearer = AuthMode::Bearer { token: "t".into() };

        // No auth + reachable from the network → exposed.
        assert!(exposes_unauthenticated(&any, &AuthMode::None));
        assert!(exposes_unauthenticated(&lan, &AuthMode::None));
        // Loopback is safe even with no auth.
        assert!(!exposes_unauthenticated(&loopback, &AuthMode::None));
        // Non-loopback WITH auth is fine.
        assert!(!exposes_unauthenticated(&any, &bearer));
        assert!(!exposes_unauthenticated(&lan, &bearer));
    }

    #[test]
    fn metrics_registry_counts_writes() {
        let m = MetricsRegistry::default();
        m.record_copy(100);
        m.record_copy(50);
        m.record_error();
        m.inc_active();
        m.inc_active();
        m.dec_active();
        m.dec_active();
        m.dec_active(); // saturating — must not underflow
        let snap = m.snapshot();
        assert_eq!(snap.jobs_total, 2);
        assert_eq!(snap.files_copied_total, 2);
        assert_eq!(snap.bytes_copied_total, 150);
        assert_eq!(snap.errors_total, 1);
        assert_eq!(snap.active_jobs, 0);
    }
}
