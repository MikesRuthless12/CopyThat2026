use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use crate::{CapabilityGrant, PluginConfig, PluginError, PluginHandle, PluginManifest};


/// Owns the `wasmtime::Engine` shared across plugin instances, the
/// [`PluginConfig`] applied to every `call_hook` dispatch, and the
/// engine-tied epoch ticker that drives wall-clock budget
/// enforcement.
///
/// 46.2 introduced the engine; 46.3 layered the per-call sandbox
/// (`async_support` + `consume_fuel` flipped on in `Config`, fuel
/// and memory budgets dialed via [`PluginConfig`]). 46.4 added
/// `epoch_interruption` + a 1ms ticker thread that calls
/// `Engine::increment_epoch()`. The ticker is owned by the host and
/// stops when the host drops, so loaded handles cannot outlive the
/// host that minted them (handles clone the engine, not the
/// ticker, but a stale handle running after the host drops will
/// still trap on its first epoch check because nothing is
/// advancing the epoch — the deadline never resets).
pub struct PluginHost {
    engine: wasmtime::Engine,
    config: PluginConfig,
    // `_ticker` is held only for its `Drop` side-effect (signal the
    // ticker thread to stop and join it). Prefixed with `_` to
    // suppress the dead-code lint without disabling it crate-wide.
    _ticker: EpochTicker,
}

impl std::fmt::Debug for PluginHost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PluginHost")
            .field("config", &self.config)
            .finish_non_exhaustive()
    }
}

/// Background thread that drives `Engine::increment_epoch()` on a
/// 1ms cadence so per-call `Store::set_epoch_deadline(N)` deadlines
/// fire after N milliseconds.
struct EpochTicker {
    stop: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
}

impl EpochTicker {
    fn spawn(engine: wasmtime::Engine) -> Self {
        let stop = Arc::new(AtomicBool::new(false));
        let stop_clone = Arc::clone(&stop);
        let handle = thread::Builder::new()
            .name("copythat-plugin-epoch-ticker".to_owned())
            .spawn(move || {
                // 1ms tick: tight enough for 50ms wall budgets to
                // fire within ±1ms, loose enough that the syscall
                // overhead on Windows + Linux + macOS stays
                // negligible.
                let tick = Duration::from_millis(1);
                while !stop_clone.load(Ordering::Relaxed) {
                    thread::sleep(tick);
                    engine.increment_epoch();
                }
            })
            .expect("OS must allow spawning a daemon thread");
        EpochTicker {
            stop,
            handle: Some(handle),
        }
    }
}

impl Drop for EpochTicker {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            // Best-effort join. If the ticker thread has already
            // exited (e.g. panic'd) join returns Err — we don't
            // care; the host is dropping anyway.
            let _ = handle.join();
        }
    }
}

impl PluginHost {
    /// Build a fresh host with [`PluginConfig::default`] budgets.
    /// See [`PluginHost::with_config`] for the configurable
    /// constructor.
    pub fn new() -> Self {
        Self::with_config(PluginConfig::default())
    }

    /// Build a fresh host with the given [`PluginConfig`].
    ///
    /// `wasmtime::Config` is built with `consume_fuel(true)` (CPU
    /// budget) and `epoch_interruption(true)` (wall-clock budget,
    /// driven by [`EpochTicker`]). Async support is always-on in
    /// wasmtime 44+ — the explicit `async_support` toggle was
    /// deprecated as a no-op, so we don't call it.
    pub fn with_config(config: PluginConfig) -> Self {
        let mut wt_config = wasmtime::Config::new();
        wt_config.consume_fuel(true);
        wt_config.epoch_interruption(true);
        let engine = wasmtime::Engine::new(&wt_config)
            .expect("default wasmtime config + consume_fuel + epoch_interruption must construct");
        let _ticker = EpochTicker::spawn(engine.clone());
        Self {
            engine,
            config,
            _ticker,
        }
    }

    /// Borrow the underlying `wasmtime::Engine`.
    ///
    /// Surfaced so the rest of the runtime (and 46.5's WASI wiring)
    /// can layer a `Linker` against the same engine without forcing
    /// every helper to thread a `&PluginHost` around.
    pub fn engine(&self) -> &wasmtime::Engine {
        &self.engine
    }

    /// Borrow the [`PluginConfig`] in effect for new plugins
    /// loaded from this host.
    pub fn config(&self) -> &PluginConfig {
        &self.config
    }

    /// Compile a plugin from a `.wasm` (or `.wat`) file on disk and
    /// load the adjacent `plugin.toml` manifest with an
    /// **empty** [`CapabilityGrant`].
    ///
    /// Equivalent to [`PluginHost::load_plugin_with_grant`] with
    /// `CapabilityGrant::empty()`. Useful for plugins that declare
    /// no capabilities (so an empty grant trivially covers them);
    /// any plugin that *does* declare capabilities will fail every
    /// `call_hook` with [`PluginError::CapabilityDenied`] until the
    /// caller swaps in a populated grant via
    /// [`PluginHost::load_plugin_with_grant`].
    pub fn load_plugin(&self, wasm_path: &Path) -> Result<PluginHandle, PluginError> {
        self.load_plugin_with_grant(wasm_path, CapabilityGrant::empty())
    }

    /// Compile a plugin and apply the given [`CapabilityGrant`] for
    /// the per-call gate.
    ///
    /// Order of operations:
    /// 1. Read the WASM/WAT bytes (`Io` errors propagate).
    /// 2. Compile via `wasmtime::Module::new` (`Wasmtime` errors
    ///    propagate).
    /// 3. Read and parse the adjacent `plugin.toml` (the file
    ///    named `plugin.toml` in the same directory as
    ///    `wasm_path`); `Manifest` errors propagate.
    ///
    /// The compile-before-manifest order means an invalid-WASM
    /// module surfaces as `Wasmtime`, not `Manifest` (preserves
    /// 46.2's diagnostic shape) — and a missing-WASM file
    /// surfaces as `Io` for the same reason.
    pub fn load_plugin_with_grant(
        &self,
        wasm_path: &Path,
        grant: CapabilityGrant,
    ) -> Result<PluginHandle, PluginError> {
        let bytes = std::fs::read(wasm_path)?;
        let module = wasmtime::Module::new(&self.engine, &bytes)?;
        let manifest = read_adjacent_manifest(wasm_path)?;
        Ok(PluginHandle::from_parts(
            self.engine.clone(),
            module,
            self.config.clone(),
            manifest,
            grant,
        ))
    }
}

impl Default for PluginHost {
    fn default() -> Self {
        Self::new()
    }
}

/// Read the `plugin.toml` adjacent to `wasm_path` (same parent
/// directory). The file is required by the 46.4 contract; absence
/// surfaces as [`PluginError::Manifest`] so the caller gets a
/// uniform diagnostic regardless of whether the manifest was
/// missing or malformed.
fn read_adjacent_manifest(wasm_path: &Path) -> Result<PluginManifest, PluginError> {
    let manifest_path = wasm_path
        .parent()
        .map(|p| p.join("plugin.toml"))
        .ok_or_else(|| {
            PluginError::Manifest(format!(
                "wasm path {} has no parent directory to look for plugin.toml in",
                wasm_path.display()
            ))
        })?;
    let toml_src = std::fs::read_to_string(&manifest_path).map_err(|e| {
        PluginError::Manifest(format!(
            "failed to read {}: {e}",
            manifest_path.display()
        ))
    })?;
    PluginManifest::parse(&toml_src)
}

