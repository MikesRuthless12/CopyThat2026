use thiserror::Error;

use crate::Capability;

/// Errors raised by [`crate::PluginHost`] and [`crate::PluginHandle`].
#[derive(Debug, Error)]
pub enum PluginError {
    /// Failed to read the WASM module from disk.
    #[error("plugin runtime: I/O error reading module: {0}")]
    Io(#[from] std::io::Error),

    /// `wasmtime` rejected the module (compile / instantiate / link
    /// / typed-func call). Wraps the upstream `wasmtime::Error` so
    /// the engine's diagnostic chain reaches the caller intact.
    #[error("plugin runtime: wasmtime engine error: {0}")]
    Wasmtime(#[from] wasmtime::Error),

    /// Hook returned malformed JSON we couldn't decode into
    /// [`crate::HookOutcome`].
    #[error("plugin runtime: hook returned malformed outcome: {0}")]
    Outcome(#[from] serde_json::Error),

    /// Plugin module is missing one of the required ABI exports
    /// (`memory`, `alloc`, or `hook`).
    #[error("plugin runtime: plugin missing required export `{0}`")]
    MissingExport(&'static str),

    /// Plugin returned a `(ptr, len)` pair that walks past the end
    /// of its linear memory. Caught before the engine reads bytes.
    #[error("plugin runtime: plugin returned out-of-bounds pointer (ptr={ptr}, len={len})")]
    OutOfBounds { ptr: u32, len: u32 },

    /// Caller-supplied [`crate::HookCtx`] serialised to more than
    /// `i32::MAX` bytes — wasmtime's typed-func ABI uses signed
    /// 32-bit integers for sizes, so payloads larger than that
    /// would silently truncate.
    #[error("plugin runtime: hook payload too large ({0} bytes; ABI cap is i32::MAX)")]
    PayloadTooLarge(usize),

    /// Plugin burned through the per-call CPU budget
    /// (`PluginConfig::fuel_per_call`). The engine traps with
    /// `wasmtime::Trap::OutOfFuel`; the host converts that to
    /// this variant.
    #[error("plugin runtime: plugin exhausted CPU fuel budget")]
    FuelExhausted,

    /// Plugin tried to grow linear memory past
    /// `PluginConfig::max_memory_bytes`. The
    /// `wasmtime::ResourceLimiter` rejects the growth, the engine
    /// traps, and the host converts that trap to this variant
    /// carrying the requested-vs-cap diagnostic.
    #[error(
        "plugin runtime: plugin tried to grow memory to {wanted_bytes} bytes (cap {max_bytes})"
    )]
    MemoryExceeded {
        wanted_bytes: usize,
        max_bytes: usize,
    },

    /// `plugin.toml` failed to parse or violated the schema
    /// (non-empty `name`, semver-shaped `version`, non-empty
    /// `hooks`, valid `capabilities` grammar). The wrapped string
    /// is a human-readable diagnostic suitable for surfacing in the
    /// Settings UI.
    #[error("plugin runtime: invalid manifest: {0}")]
    Manifest(String),

    /// Plugin's manifest declared a capability that the user has
    /// not granted. Surfaced by [`crate::PluginHandle::call_hook`]
    /// before the WASM instance is constructed, so the plugin
    /// observes neither a partially-built sandbox nor any
    /// host-side state.
    #[error("plugin runtime: plugin `{plugin}` requested ungranted capability `{capability}`")]
    CapabilityDenied {
        /// Plugin name as declared in `plugin.toml`.
        plugin: String,
        /// String form of the denied capability, e.g.
        /// `read_fs:source`. Stored as a string (not the
        /// [`Capability`] enum) so the error renders with the same
        /// grammar a user wrote in the manifest.
        capability: String,
    },

    /// Plugin ran past `PluginConfig::wall_time_budget`. The
    /// engine's epoch-interruption ticker (driven by
    /// [`crate::PluginHost`]) trips a `wasmtime::Trap::Interrupt`,
    /// which the host converts to this variant.
    #[error("plugin runtime: plugin exceeded wall-clock budget")]
    WallTimeExceeded,
}

impl PluginError {
    /// Build a [`PluginError::CapabilityDenied`] from the typed
    /// [`Capability`] so the error string matches the manifest
    /// grammar exactly.
    pub(crate) fn capability_denied(plugin: impl Into<String>, capability: &Capability) -> Self {
        PluginError::CapabilityDenied {
            plugin: plugin.into(),
            capability: capability.as_str(),
        }
    }
}
