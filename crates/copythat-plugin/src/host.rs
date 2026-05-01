use std::path::Path;

use crate::{PluginError, PluginHandle};

/// Owns the `wasmtime::Engine` shared across plugin instances.
///
/// 46.2 holds the engine and compiles modules from disk; 46.3 grows
/// the host to also carry a `wasmtime::Linker` (so plugins can
/// import host-provided helpers) and the WASI context factory + per-
/// plugin capability registry.
#[derive(Debug)]
pub struct PluginHost {
    engine: wasmtime::Engine,
}

impl PluginHost {
    /// Build a fresh host with default `wasmtime` configuration
    /// (cranelift JIT, parallel module compilation).
    ///
    /// CPU/memory budgets and capability gating land in 46.3 → 46.4
    /// and will grow `Config` rather than this constructor's
    /// signature.
    pub fn new() -> Self {
        Self {
            engine: wasmtime::Engine::default(),
        }
    }

    /// Borrow the underlying `wasmtime::Engine`.
    ///
    /// Surfaced so 46.3 can layer a `Linker` + WASI context against
    /// the same engine without forcing every helper to thread a
    /// `&PluginHost` around.
    pub fn engine(&self) -> &wasmtime::Engine {
        &self.engine
    }

    /// Compile a plugin from a `.wasm` (or `.wat`) file on disk.
    ///
    /// The file is read into memory, compiled by `wasmtime`, and
    /// returned as a [`PluginHandle`] that can be dispatched via
    /// [`PluginHandle::call_hook`]. Each `call_hook` builds a fresh
    /// `wasmtime::Store` so plugins are stateless across calls;
    /// 46.4 revisits this when per-instance budgets and capability
    /// grants are introduced.
    pub fn load_plugin(&self, wasm_path: &Path) -> Result<PluginHandle, PluginError> {
        let bytes = std::fs::read(wasm_path)?;
        let module = wasmtime::Module::new(&self.engine, &bytes)?;
        Ok(PluginHandle::from_parts(self.engine.clone(), module))
    }
}

impl Default for PluginHost {
    fn default() -> Self {
        Self::new()
    }
}
