use crate::{HookCtx, HookKind, HookOutcome, PluginError};

/// Plugin must export its linear memory under this name.
const ABI_MEMORY: &str = "memory";
/// Plugin must export `fn alloc(size: i32) -> i32` returning a
/// pointer to a fresh `size`-byte buffer in plugin memory.
const ABI_ALLOC: &str = "alloc";
/// Plugin must export
/// `fn hook(ctx_ptr: i32, ctx_len: i32) -> i64` returning a packed
/// `(out_ptr << 32) | out_len` value pointing at the JSON
/// [`HookOutcome`] response.
const ABI_HOOK: &str = "hook";

/// Handle to a compiled plugin module.
///
/// Each [`PluginHandle::call_hook`] dispatch builds a fresh
/// `wasmtime::Store` and `Instance` so plugins are stateless across
/// calls — that keeps the sandbox surface narrow at the cost of
/// re-running module initialisers per dispatch. 46.4 revisits the
/// store-per-call vs. store-per-job tradeoff once CPU/memory budgets
/// and per-instance state are on the table.
pub struct PluginHandle {
    engine: wasmtime::Engine,
    module: wasmtime::Module,
}

impl std::fmt::Debug for PluginHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // `wasmtime::Module` doesn't carry a useful `Debug` impl, so
        // print only the type marker — module bytes / engine state
        // would just be noise in panic backtraces.
        f.debug_struct("PluginHandle").finish_non_exhaustive()
    }
}

impl PluginHandle {
    pub(crate) fn from_parts(engine: wasmtime::Engine, module: wasmtime::Module) -> Self {
        Self { engine, module }
    }

    /// Dispatch a hook to this plugin and decode the outcome.
    ///
    /// # ABI
    ///
    /// The plugin must export a `memory` plus the two ABI functions
    /// described on the [`ABI_ALLOC`] / [`ABI_HOOK`] constants. The
    /// host:
    ///
    /// 1. Stamps `HookCtx::kind` with the requested [`HookKind`] so
    ///    the plugin can branch without a separate argument.
    /// 2. JSON-encodes the [`HookCtx`].
    /// 3. Calls `alloc(ctx_len)` and writes the JSON bytes to the
    ///    returned pointer.
    /// 4. Calls `hook(ctx_ptr, ctx_len)`, unpacks the returned
    ///    `(out_ptr, out_len)`, reads the JSON [`HookOutcome`] back
    ///    out of plugin memory, and decodes it.
    ///
    /// The flow is currently synchronous internally — the function
    /// is `async` to keep the public signature stable when 46.3
    /// flips on `wasmtime`'s async support for CPU-budget yielding.
    pub async fn call_hook(
        &self,
        hook: HookKind,
        mut ctx: HookCtx,
    ) -> Result<HookOutcome, PluginError> {
        ctx.kind = Some(hook);
        let ctx_json = serde_json::to_vec(&ctx)?;
        let ctx_len: i32 = ctx_json
            .len()
            .try_into()
            .map_err(|_| PluginError::PayloadTooLarge(ctx_json.len()))?;

        let mut store = wasmtime::Store::new(&self.engine, ());
        let instance = wasmtime::Instance::new(&mut store, &self.module, &[])?;

        let memory = instance
            .get_memory(&mut store, ABI_MEMORY)
            .ok_or(PluginError::MissingExport(ABI_MEMORY))?;
        let alloc = instance
            .get_typed_func::<i32, i32>(&mut store, ABI_ALLOC)
            .map_err(|_| PluginError::MissingExport(ABI_ALLOC))?;
        let hook_fn = instance
            .get_typed_func::<(i32, i32), i64>(&mut store, ABI_HOOK)
            .map_err(|_| PluginError::MissingExport(ABI_HOOK))?;

        let ctx_ptr = alloc.call(&mut store, ctx_len)?;
        write_memory(&memory, &mut store, ctx_ptr, &ctx_json)?;

        let packed = hook_fn.call(&mut store, (ctx_ptr, ctx_len))? as u64;
        let out_ptr = (packed >> 32) as u32;
        let out_len = (packed & 0xffff_ffff) as u32;

        let mut out_buf = vec![0u8; out_len as usize];
        read_memory(&memory, &store, out_ptr, &mut out_buf)?;

        let outcome: HookOutcome = serde_json::from_slice(&out_buf)?;
        Ok(outcome)
    }
}

fn write_memory(
    memory: &wasmtime::Memory,
    store: &mut wasmtime::Store<()>,
    ptr: i32,
    data: &[u8],
) -> Result<(), PluginError> {
    let offset = (ptr as u32) as usize;
    memory
        .write(store, offset, data)
        .map_err(|_| PluginError::OutOfBounds {
            ptr: ptr as u32,
            len: data.len() as u32,
        })
}

fn read_memory(
    memory: &wasmtime::Memory,
    store: &wasmtime::Store<()>,
    ptr: u32,
    buf: &mut [u8],
) -> Result<(), PluginError> {
    memory
        .read(store, ptr as usize, buf)
        .map_err(|_| PluginError::OutOfBounds {
            ptr,
            len: buf.len() as u32,
        })
}
