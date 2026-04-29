//! Typed errors. `ServeError` is what `serve()` returns; everything
//! else funnels into `RouteError` so a per-handler failure can render
//! a stable HTML page instead of a tower default.

use std::io;

use thiserror::Error;

/// Failure modes for [`crate::serve`].
#[derive(Debug, Error)]
pub enum ServeError {
    /// Bind to the requested socket address failed (port in use,
    /// permission denied for a privileged port, etc.).
    #[error("bind to {addr}: {source}")]
    Bind {
        addr: std::net::SocketAddr,
        #[source]
        source: io::Error,
    },
    /// `serve()` was called outside a tokio runtime context.
    #[error("serve() must be called inside a tokio runtime")]
    NoRuntime,
    /// A bind succeeded but switching the socket to non-blocking mode
    /// or wrapping it in tokio failed.
    #[error("prepare listener for {addr}: {source}")]
    Listener {
        addr: std::net::SocketAddr,
        #[source]
        source: io::Error,
    },
}

/// Per-handler failure surface. Every handler returns
/// `Result<T, RouteError>`; the `IntoResponse` impl picks the right
/// HTTP status and renders an HTML or JSON body.
#[derive(Debug, Error)]
pub(crate) enum RouteError {
    /// Caller asked for a job rowid we couldn't find.
    #[error("not found")]
    NotFound,
    /// Caller's POST body was malformed.
    #[error("bad request: {0}")]
    BadRequest(String),
    /// The history database failed to read or write.
    #[error("history error: {0}")]
    History(#[from] copythat_history::HistoryError),
    /// The chunk store failed to read.
    #[error("chunk store error: {0}")]
    Chunk(#[from] copythat_chunk::ChunkStoreError),
    /// askama failed to render a template — this is always a bug,
    /// rendered as 500.
    #[error("template render: {0}")]
    Render(#[from] askama::Error),
    /// A route surfaced a feature that hasn't shipped yet
    /// (`/sessions`, `/metrics`).
    #[error("not implemented")]
    NotImplemented,
    /// Phase 40 review-fix — the requested file's manifest exceeds the
    /// `MAX_FILE_DOWNLOAD_BYTES` cap that bounds in-RAM buffering. The
    /// caller can retry by configuring a follow-up streaming endpoint
    /// once it lands; for now this surfaces as `413 Payload Too Large`
    /// with the size + limit in the body.
    #[error("payload too large: file is {size} bytes, max {limit}")]
    PayloadTooLarge { size: u64, limit: u64 },
}
