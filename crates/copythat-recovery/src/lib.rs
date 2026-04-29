//! `copythat-recovery` — Phase 39 browser-accessible recovery UI.
//!
//! A loopback HTTP server, served by `axum` and rendered with
//! `askama` templates, that exposes the Phase 9 history archive +
//! the Phase 27 content-defined chunk store through a small,
//! authenticated browser UI. Lets the user browse recent jobs,
//! download a file as it was when the job ran, and initiate a
//! restore at an arbitrary timestamp.
//!
//! # Threat model
//!
//! - **Default bind:** `127.0.0.1`. Loopback-only — every byte the
//!   recovery surface emits is on the user's own machine and the
//!   token check is belt-and-suspenders for "another user account
//!   on the same machine".
//! - **Non-loopback bind:** opt-in via Settings → Advanced →
//!   "Recovery web UI" → "Allow non-loopback bind", with an
//!   acknowledged warning. The token then becomes the only
//!   authentication factor; the deployer is responsible for fronting
//!   the bind with TLS or a reverse-proxy if the LAN is untrusted.
//! - **Bearer token.** 20 random bytes (`getrandom`) rendered as 32-
//!   character Crockford base32. Compared in constant time. Rotation
//!   is a Settings button — clicking it generates a fresh token,
//!   persists it, and restarts the server.
//!
//! # Public surface
//!
//! ```no_run
//! use std::net::SocketAddr;
//! use std::sync::Arc;
//! use copythat_recovery::{serve, generate_token};
//! use secrecy::SecretString;
//!
//! # async fn demo() -> Result<(), Box<dyn std::error::Error>> {
//! let history = Arc::new(copythat_history::History::open_in_memory().await?);
//! let chunks = Arc::new(copythat_chunk::ChunkStore::open(
//!     std::path::Path::new("/tmp/cthat-chunks"),
//! )?);
//! let token: SecretString = generate_token().into();
//! let addr: SocketAddr = "127.0.0.1:0".parse()?;
//! let handle = serve(addr, history, chunks, token)?;
//! println!("listening on http://{}", handle.local_addr());
//! handle.shutdown().await;
//! # Ok(()) }
//! ```

#![forbid(unsafe_code)]

mod auth;
mod error;
mod handlers;
mod server;
mod templates;

pub use auth::generate_token;
pub use error::ServeError;
pub use server::{JoinHandle, serve};
