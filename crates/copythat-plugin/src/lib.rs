//! `copythat-plugin` — sandboxed WASM plugin runtime for post-copy
//! hooks (Phase 46 / changelog Phase 48).
//!
//! Plugins receive lifecycle callbacks ([`HookKind::BeforeJob`],
//! [`HookKind::BeforeFile`], [`HookKind::AfterFile`],
//! [`HookKind::AfterJob`], [`HookKind::OnError`]) and return a
//! [`HookOutcome`] that the engine respects: [`HookOutcome::Continue`]
//! to proceed normally, [`HookOutcome::SkipFile`] to skip the current
//! file, [`HookOutcome::AbortJob`] to stop the job, or
//! [`HookOutcome::Notify`] to emit a tray/log notification while
//! continuing.
//!
//! # Status
//!
//! Sub-phase 46.1 laid down the public type surface; 46.2 wired the
//! real `wasmtime` engine: [`PluginHost::load_plugin`] compiles a
//! WASM (or WAT) module from disk and [`PluginHandle::call_hook`]
//! dispatches hooks via a JSON-over-linear-memory ABI documented on
//! [`PluginHandle::call_hook`]. Sandbox budgets (CPU + memory),
//! manifest parsing, and capability gating land in 46.3 → 46.X.

#![forbid(unsafe_code)]

mod error;
mod handle;
mod hook;
mod host;

pub use error::PluginError;
pub use handle::PluginHandle;
pub use hook::{HookCtx, HookKind, HookOutcome};
pub use host::PluginHost;
