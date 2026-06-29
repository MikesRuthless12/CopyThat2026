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
pub mod webhook;

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

    /// Whether this protocol is served over the shared axum/hyper HTTP
    /// listener (vs. its own transport, like SFTP's SSH channel).
    pub fn is_http_family(self) -> bool {
        matches!(self, Self::WebDav | Self::Http | Self::S3)
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
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
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

/// OpenTelemetry export configuration. The export pipeline is wired in the
/// observability increment; this carries the knobs it reads.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct OtelConfig {
    /// OTLP collector endpoint, e.g. `"http://localhost:4317"`.
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

/// Start the server.
///
/// Binds the HTTP listener on [`ServerConfig::bind_addr`] (a port-in-use
/// error surfaces here, not in the background task) and spawns the axum
/// server on the current tokio runtime, serving every configured
/// HTTP-family protocol ([`Protocol::is_http_family`]) plus `/metrics`.
/// Returns a [`ServerHandle`] whose [`local_addr`](ServerHandle::local_addr)
/// reflects the OS-assigned port when the config used `:0`.
///
/// SFTP ([`Protocol::Sftp`]) is served over its own SSH transport in a
/// later increment; a config with *only* SFTP currently yields
/// [`ServerError::NotImplemented`].
pub async fn serve(config: ServerConfig) -> Result<ServerHandle, ServerError> {
    if config.protocols.is_empty() {
        return Err(ServerError::NoProtocols);
    }
    if !config.protocols.iter().any(|p| p.is_http_family()) {
        // The only configured protocols are non-HTTP (SFTP) — not wired yet.
        let protocol = config.protocols[0];
        return Err(ServerError::NotImplemented { protocol });
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

    let metrics = Arc::new(MetricsRegistry::default());
    let router = http::build_router(&config, metrics.clone());

    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
    let task = tokio::spawn(async move {
        let server =
            axum::serve(listener, router.into_make_service()).with_graceful_shutdown(async move {
                let _ = shutdown_rx.await;
            });
        if let Err(e) = server.await {
            tracing::warn!(error = ?e, "copythat server task ended with error");
        }
    });

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
