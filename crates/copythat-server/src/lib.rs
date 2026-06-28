//! Phase 48 — server mode + observability (SCAFFOLD).
//!
//! Ships the stable public surface for running CopyThat as a server and
//! emitting observability data, plus the pure, network-free first-slice
//! logic that needs no heavy deps:
//!
//! - [`format_webhook_payload`] builds each sink's documented JSON body
//!   (Slack / Discord / ntfy / Pushover).
//! - [`Metrics::render_prometheus`] emits valid Prometheus text exposition.
//!
//! Deferred to a follow-up increment (so the API is stable before the
//! heavy server/network crates land): the actual protocol listeners
//! ([`serve`] returns [`ServerError::NotImplemented`]), the live `/metrics`
//! HTTP endpoint, OpenTelemetry export ([`OtelConfig`] is config-only), and
//! webhook delivery ([`send_webhook`] returns `NotImplemented`).

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Protocols the server can expose. The listeners are deferred; the enum
/// and config are stable now.
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
}

impl std::fmt::Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.label())
    }
}

/// Server configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Bind address, e.g. `"127.0.0.1:8080"`.
    pub bind_addr: String,
    /// Which protocols to expose.
    pub protocols: Vec<Protocol>,
    /// Optional bearer token gating access (None = open on the bind addr).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_addr: "127.0.0.1:8080".to_string(),
            protocols: Vec::new(),
            auth_token: None,
        }
    }
}

/// OpenTelemetry export configuration. The export pipeline itself is
/// deferred — this carries the knobs the future exporter will read.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct OtelConfig {
    /// OTLP collector endpoint, e.g. `"http://localhost:4317"`.
    pub endpoint: String,
    /// Whether trace export is enabled.
    pub enabled: bool,
}

/// Handle to a running server. Today a thin placeholder carrying the
/// effective config; the real handle (task join handles + a shutdown
/// signal) lands with the listeners.
#[derive(Debug)]
pub struct ServerHandle {
    pub config: ServerConfig,
}

/// Server / observability errors.
#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum ServerError {
    /// The protocol listener isn't implemented yet (Phase 48 follow-up).
    #[error("server protocol {protocol} not yet implemented")]
    NotImplemented { protocol: Protocol },
    /// Webhook delivery isn't implemented yet (the HTTP client lands with
    /// the server).
    #[error("webhook delivery not yet implemented")]
    WebhookNotImplemented,
}

impl ServerError {
    /// Stable Fluent key, matching the engine's `localized_key` convention.
    pub fn localized_key(&self) -> &'static str {
        match self {
            Self::NotImplemented { .. } => "err-server-not-implemented",
            Self::WebhookNotImplemented => "err-webhook-not-implemented",
        }
    }
}

/// Start the server. **Deferred** — returns
/// [`ServerError::NotImplemented`] for the first configured protocol (or
/// `Http` when none is set) until the listeners land. The signature is the
/// stable contract the real implementation will keep.
pub fn serve(config: ServerConfig) -> Result<ServerHandle, ServerError> {
    let protocol = config.protocols.first().copied().unwrap_or(Protocol::Http);
    Err(ServerError::NotImplemented { protocol })
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

/// Webhook destinations. Delivery is deferred; payload formatting is real.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WebhookTarget {
    Slack,
    Discord,
    Ntfy,
    Pushover,
}

/// Build the service-specific JSON body for `target` from `event`. Pure —
/// the actual HTTP POST is [`send_webhook`] (deferred). `token`/`user` for
/// Pushover are placeholders here; delivery fills them from config.
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

/// Deliver a formatted webhook payload. **Deferred** — the HTTP client
/// lands with the server; returns [`ServerError::WebhookNotImplemented`].
pub fn send_webhook(
    _target: WebhookTarget,
    _url: &str,
    _payload: &serde_json::Value,
) -> Result<(), ServerError> {
    Err(ServerError::WebhookNotImplemented)
}

/// Engine metrics surfaced by the (deferred) `/metrics` endpoint. The
/// counters/gauges and the exposition rendering are real now.
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
