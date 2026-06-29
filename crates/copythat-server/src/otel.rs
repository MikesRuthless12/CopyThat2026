//! Phase 48 follow-up — OpenTelemetry (OTLP/HTTP) trace export.
//!
//! [`install_otel`] bridges the engine's `tracing` spans into an OTLP span
//! pipeline so a collector (Jaeger / Tempo / an OpenTelemetry Collector) can
//! ingest them. The transport is OTLP over **HTTP/protobuf** on the in-tree
//! async `reqwest` + `rustls` stack — no gRPC/tonic, no blocking client.
//!
//! ## Why a dedicated runtime
//!
//! The async `reqwest` exporter and the batch span processor both need a
//! tokio reactor to run on. Borrowing the caller's runtime is unsafe here:
//! the CLI drives `serve` on a single-threaded `current_thread` runtime, and
//! the batch processor's `shutdown()` *synchronously* blocks the calling
//! thread until the batch task drains — on a current-thread runtime that is
//! the same thread, so it would deadlock (and dropping a tokio `Runtime` from
//! inside an async context panics outright). [`install_otel`] therefore spins
//! up a small dedicated multi-thread runtime (one worker) that owns all OTel
//! background work; [`OtelGuard`] holds it and tears it down on drop. Trace
//! export stays fully decoupled from the request-serving runtime.

use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::{Protocol, SpanExporter, WithExportConfig};
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::runtime;
use opentelemetry_sdk::trace::SdkTracerProvider;
use opentelemetry_sdk::trace::span_processor_with_async_runtime::BatchSpanProcessor;
use thiserror::Error;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::OtelConfig;

/// Errors raised while wiring up the OpenTelemetry export pipeline.
#[derive(Debug, Error)]
pub enum OtelError {
    /// The dedicated tokio runtime backing the export pipeline could not be
    /// built.
    #[error("failed to build OpenTelemetry export runtime: {0}")]
    Runtime(String),
    /// The OTLP span exporter could not be constructed — e.g. the endpoint
    /// URL was malformed, or the HTTP client failed to build.
    #[error("failed to build OTLP span exporter: {0}")]
    Exporter(String),
}

impl OtelError {
    /// Stable Fluent key, matching the engine's `localized_key` convention.
    pub fn localized_key(&self) -> &'static str {
        match self {
            Self::Runtime(_) => "err-otel-runtime",
            Self::Exporter(_) => "err-otel-exporter",
        }
    }
}

/// Keeps the OpenTelemetry export pipeline alive. Dropping it flushes any
/// buffered spans (a best-effort `shutdown`) and tears down the dedicated
/// export runtime. A disabled [`OtelConfig`] yields an inert guard whose drop
/// does nothing.
#[derive(Debug, Default)]
pub struct OtelGuard {
    provider: Option<SdkTracerProvider>,
    runtime: Option<tokio::runtime::Runtime>,
}

impl Drop for OtelGuard {
    fn drop(&mut self) {
        // Flush buffered spans first: `shutdown()` blocks the calling thread
        // until the batch task (running on a worker of `runtime`) drains, so
        // the runtime must still be alive at this point.
        if let Some(provider) = self.provider.take() {
            let _ = provider.shutdown();
        }
        // Detach the dedicated runtime without a blocking join — a blocking
        // `Runtime` drop from inside the caller's async context would panic.
        if let Some(rt) = self.runtime.take() {
            rt.shutdown_background();
        }
    }
}

/// Install the OpenTelemetry trace-export pipeline described by `config`.
///
/// When `config.enabled` is false this is a no-op returning an inert
/// [`OtelGuard`]. When enabled it builds an OTLP HTTP/protobuf span exporter
/// targeting `config.endpoint` — which must be the full OTLP *traces* URL
/// (e.g. `http://localhost:4318/v1/traces`; the path is used verbatim, not
/// auto-appended) — attaches a batch span processor on a dedicated tokio
/// runtime, and installs a `tracing-opentelemetry` layer into the process
/// `tracing` subscriber via `try_init`. `try_init` (rather than `init`) means
/// a subscriber installed earlier in the process does not panic the server: a
/// warning is logged and the existing subscriber is left in place.
///
/// Export is best-effort. A successful return means the pipeline was built and
/// the dedicated runtime is live; it does not guarantee the collector is
/// reachable (delivery failures surface only as dropped spans, never as a
/// serving error).
pub fn install_otel(config: &OtelConfig) -> Result<OtelGuard, OtelError> {
    if !config.enabled {
        return Ok(OtelGuard::default());
    }

    // One worker thread is ample for span batching, and it keeps OTel's async
    // work off the caller's (possibly current-thread) runtime so the
    // synchronous shutdown on drop cannot deadlock.
    let otel_rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .thread_name("copythat-otel")
        .build()
        .map_err(|e| OtelError::Runtime(e.to_string()))?;

    let provider = {
        // Enter the dedicated runtime so the batch processor's `tokio::spawn`
        // and the exporter's reqwest client bind to it, not the caller's.
        let _enter = otel_rt.enter();

        let exporter = SpanExporter::builder()
            .with_http()
            .with_endpoint(config.endpoint.as_str())
            .with_protocol(Protocol::HttpBinary)
            .build()
            .map_err(|e| OtelError::Exporter(e.to_string()))?;

        let processor = BatchSpanProcessor::builder(exporter, runtime::Tokio).build();

        SdkTracerProvider::builder()
            .with_span_processor(processor)
            .with_resource(Resource::builder().with_service_name("copythat").build())
            .build()
    };

    let tracer = provider.tracer("copythat-server");
    let otel_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    if let Err(e) = tracing_subscriber::registry().with(otel_layer).try_init() {
        // A subscriber is already installed (e.g. the host process set one
        // up). Export is best-effort, so warn and keep serving rather than
        // abort — the spans simply won't reach this OTel layer.
        tracing::warn!(
            error = %e,
            "OpenTelemetry layer not installed: a tracing subscriber is already active"
        );
    }

    Ok(OtelGuard {
        provider: Some(provider),
        runtime: Some(otel_rt),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disabled_config_returns_inert_guard() {
        let cfg = OtelConfig {
            endpoint: String::new(),
            enabled: false,
        };
        // No runtime, no provider, and dropping it must not panic.
        let guard = install_otel(&cfg).expect("disabled install is infallible");
        assert!(guard.provider.is_none());
        assert!(guard.runtime.is_none());
        drop(guard);
    }

    #[test]
    fn enabled_with_unreachable_endpoint_builds_and_drops_cleanly() {
        // A syntactically valid but unreachable endpoint: the exporter builds
        // (no connection is made until export), and dropping the guard flushes
        // + tears down without panicking or hanging.
        let cfg = OtelConfig {
            endpoint: "http://127.0.0.1:4318/v1/traces".to_string(),
            enabled: true,
        };
        let guard = install_otel(&cfg).expect("exporter build should succeed");
        assert!(guard.provider.is_some());
        assert!(guard.runtime.is_some());
        drop(guard);
    }
}
