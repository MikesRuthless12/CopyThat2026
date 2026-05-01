use thiserror::Error;

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
}
