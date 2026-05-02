use crate::{
    CapabilityGrant, HookCtx, HookKind, HookOutcome, PluginConfig, PluginError, PluginManifest,
};

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

/// Per-Store data the runtime threads through `wasmtime`.
///
/// 46.3 carries just the [`PluginLimiter`] (memory cap); 46.4 will
/// add the WASI context for capability-based file/network access.
struct StoreData {
    limiter: PluginLimiter,
}

/// `wasmtime::ResourceLimiter` impl that caps total linear-memory
/// growth at [`PluginConfig::max_memory_bytes`]. Returns `Err`
/// (rather than `Ok(false)`) on rejection so the trap propagates
/// out of the call instead of letting the plugin observe a
/// soft-failed `memory.grow` and continue.
struct PluginLimiter {
    max_memory_bytes: usize,
}

/// Marker error used by [`PluginLimiter::memory_growing`] to
/// downgrade a memory-cap rejection into a `wasmtime::Error`. The
/// host downcasts this in [`map_engine_error`] to surface the rich
/// [`PluginError::MemoryExceeded`] diagnostic instead of a
/// generic engine error.
#[derive(Debug)]
struct MemoryRejectedMarker {
    wanted_bytes: usize,
    max_bytes: usize,
}

impl std::fmt::Display for MemoryRejectedMarker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "plugin memory growth rejected: wanted {} bytes, cap {}",
            self.wanted_bytes, self.max_bytes
        )
    }
}

impl std::error::Error for MemoryRejectedMarker {}

impl wasmtime::ResourceLimiter for PluginLimiter {
    fn memory_growing(
        &mut self,
        _current: usize,
        desired: usize,
        _max: Option<usize>,
    ) -> wasmtime::Result<bool> {
        if desired <= self.max_memory_bytes {
            Ok(true)
        } else {
            Err(MemoryRejectedMarker {
                wanted_bytes: desired,
                max_bytes: self.max_memory_bytes,
            }
            .into())
        }
    }

    fn table_growing(
        &mut self,
        _current: usize,
        _desired: usize,
        _max: Option<usize>,
    ) -> wasmtime::Result<bool> {
        // Tables are out-of-scope for the post-copy-hook ABI; let
        // the engine's own defaults handle them.
        Ok(true)
    }
}

/// Handle to a compiled plugin module.
///
/// Each [`PluginHandle::call_hook`] dispatch builds a fresh
/// `wasmtime::Store` so plugins are stateless across calls — that
/// keeps the sandbox surface narrow at the cost of re-running
/// module initialisers per dispatch. Per-call CPU fuel, memory,
/// and wall-clock caps come from the [`PluginConfig`] cloned out
/// of the host at [`crate::PluginHost::load_plugin`] time. The
/// [`PluginManifest`] + [`CapabilityGrant`] gate the call before
/// the engine even instantiates the module — every capability the
/// manifest declares must be in the grant or the dispatch fails
/// with [`PluginError::CapabilityDenied`].
pub struct PluginHandle {
    engine: wasmtime::Engine,
    module: wasmtime::Module,
    config: PluginConfig,
    manifest: PluginManifest,
    grant: CapabilityGrant,
}

impl std::fmt::Debug for PluginHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // `wasmtime::Module` doesn't carry a useful `Debug` impl, so
        // print only the type marker, config, and manifest — module
        // bytes / engine state would be noise in panic backtraces.
        f.debug_struct("PluginHandle")
            .field("config", &self.config)
            .field("manifest", &self.manifest)
            .field("grant", &self.grant)
            .finish_non_exhaustive()
    }
}

impl PluginHandle {
    pub(crate) fn from_parts(
        engine: wasmtime::Engine,
        module: wasmtime::Module,
        config: PluginConfig,
        manifest: PluginManifest,
        grant: CapabilityGrant,
    ) -> Self {
        Self {
            engine,
            module,
            config,
            manifest,
            grant,
        }
    }

    /// Borrow the [`PluginConfig`] this handle dispatches under.
    pub fn config(&self) -> &PluginConfig {
        &self.config
    }

    /// Borrow the parsed [`PluginManifest`] for this plugin.
    pub fn manifest(&self) -> &PluginManifest {
        &self.manifest
    }

    /// Borrow the [`CapabilityGrant`] applied to this plugin.
    pub fn grant(&self) -> &CapabilityGrant {
        &self.grant
    }

    /// Dispatch a hook to this plugin and decode the outcome.
    ///
    /// # ABI
    ///
    /// The plugin must export a `memory` plus the two ABI
    /// functions described on the [`ABI_ALLOC`] / [`ABI_HOOK`]
    /// constants. The host:
    ///
    /// 1. Stamps `HookCtx::kind` with the requested [`HookKind`]
    ///    so the plugin can branch without a separate argument.
    /// 2. JSON-encodes the [`HookCtx`].
    /// 3. Builds a fresh `wasmtime::Store`, installs the
    ///    `PluginConfig::fuel_per_call` budget via
    ///    `Store::set_fuel`, and the
    ///    `PluginConfig::max_memory_bytes` cap via a
    ///    `wasmtime::ResourceLimiter`.
    /// 4. Calls `alloc(ctx_len)` and writes the JSON bytes to
    ///    the returned pointer.
    /// 5. Calls `hook(ctx_ptr, ctx_len)`, unpacks the returned
    ///    `(out_ptr, out_len)`, reads the JSON [`HookOutcome`]
    ///    back out of plugin memory, and decodes it.
    ///
    /// All wasmtime calls go through the `*_async` variants so
    /// fuel exhaustion and memory-growth rejection trap
    /// cooperatively rather than blocking a runtime thread.
    pub async fn call_hook(
        &self,
        hook: HookKind,
        mut ctx: HookCtx,
    ) -> Result<HookOutcome, PluginError> {
        // 46.4 — pre-call capability gate. Any manifest-declared
        // capability the user has not granted blocks the dispatch
        // before the engine builds an instance, so the plugin can
        // observe nothing of the partially-built sandbox.
        if let Some(missing) = self.grant.first_missing(self.manifest.capabilities.iter()) {
            return Err(PluginError::capability_denied(
                self.manifest.name.clone(),
                missing,
            ));
        }

        ctx.kind = Some(hook);
        let ctx_json = serde_json::to_vec(&ctx)?;
        let ctx_len: i32 = ctx_json
            .len()
            .try_into()
            .map_err(|_| PluginError::PayloadTooLarge(ctx_json.len()))?;

        let store_data = StoreData {
            limiter: PluginLimiter {
                max_memory_bytes: self.config.max_memory_bytes,
            },
        };
        let mut store = wasmtime::Store::new(&self.engine, store_data);
        store
            .set_fuel(self.config.fuel_per_call)
            .map_err(map_engine_error)?;
        store.limiter(|data| &mut data.limiter);

        // 46.4 — wall-clock deadline. The host's `EpochTicker`
        // increments the engine's epoch every 1ms, so an N-tick
        // deadline fires after N milliseconds. `wall_time_budget`
        // gets clamped to >= 1 ms because a 0-tick deadline would
        // trap on the first epoch check before the plugin runs any
        // real instructions. `epoch_deadline_trap()` (the default
        // for stores under an epoch-interrupted engine) means
        // expiry surfaces as `Trap::Interrupt`, which
        // `map_engine_error` converts to `WallTimeExceeded`.
        let ticks = wall_budget_ticks(self.config.wall_time_budget);
        store.set_epoch_deadline(ticks);

        let instance = wasmtime::Instance::new_async(&mut store, &self.module, &[])
            .await
            .map_err(map_engine_error)?;

        let memory = instance
            .get_memory(&mut store, ABI_MEMORY)
            .ok_or(PluginError::MissingExport(ABI_MEMORY))?;
        let alloc = instance
            .get_typed_func::<i32, i32>(&mut store, ABI_ALLOC)
            .map_err(|_| PluginError::MissingExport(ABI_ALLOC))?;
        let hook_fn = instance
            .get_typed_func::<(i32, i32), i64>(&mut store, ABI_HOOK)
            .map_err(|_| PluginError::MissingExport(ABI_HOOK))?;

        let ctx_ptr = alloc
            .call_async(&mut store, ctx_len)
            .await
            .map_err(map_engine_error)?;
        write_memory(&memory, &mut store, ctx_ptr, &ctx_json)?;

        let packed = hook_fn
            .call_async(&mut store, (ctx_ptr, ctx_len))
            .await
            .map_err(map_engine_error)? as u64;
        let out_ptr = (packed >> 32) as u32;
        let out_len = (packed & 0xffff_ffff) as u32;

        // Bound `out_len` to the per-call memory cap before allocating
        // the host-side response buffer. Without this clamp a malicious
        // plugin can return e.g. `out_len = 0xFFFFFFFF` and force the
        // host to attempt a ~4 GiB allocation. The subsequent
        // `Memory::read` would catch the out-of-bounds read, but the
        // pre-read `vec![0u8; out_len]` would already have OOM-panicked
        // the host or starved the system. Plugin memory is itself
        // capped at `max_memory_bytes`, so a legitimate `(out_ptr,
        // out_len)` pair always satisfies this bound.
        if (out_len as usize) > self.config.max_memory_bytes {
            return Err(PluginError::OutOfBounds {
                ptr: out_ptr,
                len: out_len,
            });
        }

        let mut out_buf = vec![0u8; out_len as usize];
        read_memory(&memory, &store, out_ptr, &mut out_buf)?;

        let outcome: HookOutcome = serde_json::from_slice(&out_buf)?;
        Ok(outcome)
    }
}

/// Convert a `wasmtime::Error` returned by an async engine call
/// into the richest [`PluginError`] variant available — fuel
/// exhaustion, memory-cap rejection, and wall-time-deadline trips
/// get their own diagnostics before falling back to the generic
/// `Wasmtime` wrapper.
fn map_engine_error(e: wasmtime::Error) -> PluginError {
    if let Some(marker) = e.downcast_ref::<MemoryRejectedMarker>() {
        return PluginError::MemoryExceeded {
            wanted_bytes: marker.wanted_bytes,
            max_bytes: marker.max_bytes,
        };
    }
    if let Some(trap) = e.downcast_ref::<wasmtime::Trap>() {
        match trap {
            wasmtime::Trap::OutOfFuel => return PluginError::FuelExhausted,
            wasmtime::Trap::Interrupt => return PluginError::WallTimeExceeded,
            _ => {}
        }
    }
    PluginError::Wasmtime(e)
}

/// Convert [`PluginConfig::wall_time_budget`] into the integer-tick
/// argument `Store::set_epoch_deadline` wants. The host's ticker
/// fires every 1 ms, so one tick == one millisecond. We clamp to a
/// minimum of one tick so a sub-millisecond budget does not become
/// "0 ticks beyond current" (which is "already expired" — every
/// dispatch would trap on its first epoch check).
fn wall_budget_ticks(budget: std::time::Duration) -> u64 {
    let millis = budget.as_millis();
    let ticks: u64 = millis.try_into().unwrap_or(u64::MAX);
    ticks.max(1)
}

fn write_memory(
    memory: &wasmtime::Memory,
    store: &mut wasmtime::Store<StoreData>,
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
    store: &wasmtime::Store<StoreData>,
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
