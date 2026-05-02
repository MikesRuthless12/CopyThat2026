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
//! real `wasmtime` engine ([`PluginHost::load_plugin`] compiles a
//! WASM/WAT module from disk and [`PluginHandle::call_hook`]
//! dispatches via a JSON-over-linear-memory ABI documented on
//! [`PluginHandle::call_hook`]). Sub-phase 46.3 added the per-call
//! sandbox: a [`PluginConfig`] knobs `fuel_per_call` (CPU budget
//! enforced via wasmtime fuel; trips [`PluginError::FuelExhausted`])
//! and `max_memory_bytes` (linear-memory growth cap enforced via
//! `wasmtime::ResourceLimiter`; trips [`PluginError::MemoryExceeded`]).
//! Sub-phase 46.4 layered the manifest + capability gate
//! ([`PluginManifest`] parsed from an adjacent `plugin.toml`, with
//! a [`CapabilityGrant`] consulted before every dispatch — failing
//! with [`PluginError::CapabilityDenied`]) and the wall-clock
//! budget (`PluginConfig::wall_time_budget`, default 50 ms,
//! enforced via `Config::epoch_interruption` + a 1ms ticker thread
//! tied to the `PluginHost`; trips [`PluginError::WallTimeExceeded`]).
//! Sub-phase 46.5 shipped four sample plugins exercising the host
//! ABI (organize-by-exif, notify-discord, notify-ntfy, dedup-warning)
//! plus `xtask build-sample-plugins`. Sub-phase 46.6 layered the
//! per-user plugin store + Settings UI in the Tauri shell. Sub-phase
//! 46.7 wraps the round with a security audit, the cumulative end-
//! to-end smoke under [`tests/smoke/phase_46_plugin.rs`], and the
//! out-of-bounds pre-allocation clamp in [`PluginHandle::call_hook`]
//! that bounds plugin-controlled response lengths to the per-call
//! memory cap before the host allocates a buffer.
//!
//! `wasmtime-wasi` wiring for capability-gated file/network imports
//! (so a `read_fs:source` grant materialises as a preopened directory
//! handle inside the sandbox) is **deferred** — the sample plugins
//! emit declarative envelopes instead, and the engine performs the
//! gated I/O on their behalf.

#![forbid(unsafe_code)]

mod capability;
mod config;
mod error;
mod handle;
mod hook;
mod host;
mod manifest;

pub use capability::{Capability, CapabilityGrant};
pub use config::PluginConfig;
pub use error::PluginError;
pub use handle::PluginHandle;
pub use hook::{HookCtx, HookKind, HookOutcome};
pub use host::PluginHost;
pub use manifest::PluginManifest;
