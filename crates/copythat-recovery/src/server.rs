//! `serve()` — entry point. Binds the listener synchronously (so a
//! port-in-use error surfaces inline) then spawns the axum runner on
//! the current tokio runtime. The returned [`JoinHandle`] gives the
//! caller the bound `local_addr`, the underlying task's join handle,
//! and a graceful-shutdown trigger.

use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use axum::Router;
use axum::middleware::from_fn_with_state;
use axum::routing::{get, post};
use copythat_chunk::ChunkStore;
use copythat_history::History;
use secrecy::SecretString;
use tokio::sync::oneshot;

use crate::auth::require_token;
use crate::error::ServeError;
use crate::handlers;

/// State threaded through every handler. The `Arc<History>` is what
/// the public `serve()` signature receives; the wrapper `History`
/// already holds an `Arc<Inner>` internally so this is double-wrapped
/// — the outer `Arc` exists so the `Tauri AppState` (which clones
/// `Arc<History>` for its other consumers) can hand the same handle
/// in without cloning the underlying SQLite connection.
#[derive(Clone)]
pub(crate) struct ServerState {
    pub(crate) db: Arc<History>,
    pub(crate) chunk: Arc<ChunkStore>,
    pub(crate) token: Arc<SecretString>,
}

/// Handle to a running recovery server.
///
/// `local_addr()` reflects the OS-assigned port when `serve()` was
/// called with port 0, which is how the Settings → Advanced toggle
/// remembers a "stable random" port. `shutdown().await` triggers a
/// graceful drain — outstanding requests finish, then the task
/// returns. `abort()` kills the task without waiting.
///
/// Phase 40 review-fix — `is_live()` reflects whether the underlying
/// axum task is still serving requests. Flips `false` if the task
/// returned with an error (port reclaimed by another process,
/// listener closed unexpectedly, panic in the runtime). The Tauri
/// shell's `recovery_status` IPC reads this flag so the Settings
/// panel surfaces a crashed server instead of silently lying about
/// the bound URL.
pub struct JoinHandle {
    local_addr: SocketAddr,
    task: tokio::task::JoinHandle<()>,
    shutdown: Option<oneshot::Sender<()>>,
    live: Arc<AtomicBool>,
}

impl JoinHandle {
    /// The socket the server is actually bound to. Reflects an OS-
    /// assigned port if the caller passed `0`.
    #[must_use]
    pub fn local_addr(&self) -> SocketAddr {
        self.local_addr
    }

    /// Phase 40 review-fix — `true` while the server task is still
    /// accepting requests. The IPC layer polls this so a Tauri shell
    /// that's been running for hours can detect a server task that
    /// died after a port-reclaim race or a runtime panic, instead of
    /// rendering a stale "active" status. Cheap (`Relaxed` load on a
    /// single atomic bool) — safe to read from any thread.
    #[must_use]
    pub fn is_live(&self) -> bool {
        self.live.load(Ordering::Relaxed)
    }

    /// Trigger a graceful shutdown. Outstanding requests are
    /// allowed to finish, then the task returns. Returns once the
    /// task joins.
    pub async fn shutdown(mut self) {
        if let Some(tx) = self.shutdown.take() {
            let _ = tx.send(());
        }
        let _ = self.task.await;
    }

    /// Force-cancel the server task.
    pub fn abort(self) {
        self.task.abort();
    }
}

/// Spin up the recovery HTTP server on `addr`.
///
/// The function binds synchronously so a port-in-use error surfaces
/// at the call site. After the bind succeeds, the axum runner is
/// spawned on the current tokio runtime; the caller MUST be inside
/// a runtime (`#[tokio::main]` or a test marked `#[tokio::test]`).
///
/// `addr` may use port `0` for "OS-assigned"; the actual bound port
/// is available via [`JoinHandle::local_addr`].
///
/// `db` is the shared history handle (Phase 9 SQLite store);
/// `chunk` is the Phase 27 content-defined chunk store. The recovery
/// UI reads from both — it never writes to the chunk store, and only
/// records new `restore` rows in History via `POST /restore`.
///
/// `token` is the bearer token presented as either `Authorization:
/// Bearer <token>` or `?t=<token>` query parameter. Requests without
/// the right token get 401.
pub fn serve(
    addr: SocketAddr,
    db: Arc<History>,
    chunk: Arc<ChunkStore>,
    token: SecretString,
) -> Result<JoinHandle, ServeError> {
    // `Handle::try_current` so we surface a clean error rather than
    // panicking when the caller forgot to start a runtime.
    let handle = tokio::runtime::Handle::try_current().map_err(|_| ServeError::NoRuntime)?;

    // Synchronous bind keeps port-in-use errors at the call site.
    let std_listener =
        std::net::TcpListener::bind(addr).map_err(|e| ServeError::Bind { addr, source: e })?;
    let local_addr = std_listener
        .local_addr()
        .map_err(|e| ServeError::Listener { addr, source: e })?;
    std_listener
        .set_nonblocking(true)
        .map_err(|e| ServeError::Listener { addr, source: e })?;
    let listener = tokio::net::TcpListener::from_std(std_listener)
        .map_err(|e| ServeError::Listener { addr, source: e })?;

    let state = ServerState {
        db,
        chunk,
        token: Arc::new(token),
    };

    let router = build_router(state);

    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
    let live = Arc::new(AtomicBool::new(true));
    let live_for_task = Arc::clone(&live);
    let task = handle.spawn(async move {
        let server =
            axum::serve(listener, router.into_make_service()).with_graceful_shutdown(async move {
                let _ = shutdown_rx.await;
            });
        if let Err(e) = server.await {
            tracing::warn!(error = ?e, "recovery server task ended with error");
        }
        // Phase 40 review-fix — flip the liveness flag whether the
        // task ended cleanly (graceful shutdown) or with an error.
        // The Tauri shell's `recovery_status` IPC reads this to tell
        // the Settings panel that the server it thought was running
        // has actually stopped. Relaxed ordering is sufficient — the
        // worst-case stale read is one IPC poll cycle of out-of-date.
        live_for_task.store(false, Ordering::Relaxed);
    });

    Ok(JoinHandle {
        local_addr,
        task,
        shutdown: Some(shutdown_tx),
        live,
    })
}

fn build_router(state: ServerState) -> Router {
    Router::new()
        .route("/", get(handlers::landing))
        .route("/jobs", get(handlers::jobs_list))
        .route("/jobs/{id}", get(handlers::job_detail))
        .route("/jobs/{id}/files/{*path}", get(handlers::job_file_download))
        .route("/restore", get(handlers::restore_form))
        .route("/restore", post(handlers::restore_submit))
        .route("/sessions", get(handlers::sessions_stub))
        .route("/metrics", get(handlers::metrics_stub))
        .layer(from_fn_with_state(state.clone(), require_token))
        .with_state(state)
}
