use std::time::Duration;

/// Per-call sandbox budgets installed on every `wasmtime::Store`
/// the runtime allocates for a [`crate::PluginHandle::call_hook`]
/// dispatch.
///
/// Defaults match the spec's targets (64 MiB memory, 50 ms wall
/// clock) plus an empirically-chosen fuel cap that lets a small
/// synchronous hook finish without tripping while still bounding a
/// misbehaving plugin (1,000,000 fuel ≈ tens of milliseconds on
/// typical hardware for the kind of work post-copy hooks do —
/// read EXIF, format a JSON notification body, decide on
/// `HookOutcome::Continue` / `SkipFile`). Callers needing tighter
/// or looser budgets construct their own and hand it to
/// [`crate::PluginHost::with_config`].
#[derive(Debug, Clone)]
pub struct PluginConfig {
    /// Maximum fuel the WASM engine consumes per `call_hook`
    /// dispatch. Each WASM instruction burns a small amount of
    /// fuel (instruction-class dependent — see
    /// `wasmtime::Config::consume_fuel`); when fuel reaches zero
    /// the next instruction traps with `wasmtime::Trap::OutOfFuel`,
    /// which the host converts to [`crate::PluginError::FuelExhausted`].
    /// 1,000,000 is roughly tens-of-milliseconds for a small
    /// hook; a tight loop will burn through it in microseconds,
    /// so this is a CPU-cost cliff, not a wall-clock cap (the
    /// 50ms wall-clock target landed in 46.4 — see
    /// [`PluginConfig::wall_time_budget`]).
    pub fuel_per_call: u64,

    /// Maximum total linear-memory growth the plugin is allowed
    /// to reach during a single `call_hook` dispatch, in bytes.
    /// Enforced via a `wasmtime::ResourceLimiter` whose
    /// `memory_growing` callback returns `Err` once a `memory.grow`
    /// would push the plugin past this cap; the resulting trap
    /// surfaces to the host as
    /// [`crate::PluginError::MemoryExceeded`]. WASM memory is
    /// page-quantised (64 KiB pages), so the effective cap rounds
    /// down to the nearest page multiple.
    pub max_memory_bytes: usize,

    /// Maximum wall-clock duration a single `call_hook` dispatch is
    /// allowed to spend executing inside the WASM engine. Enforced
    /// via `wasmtime::Config::epoch_interruption(true)` plus a
    /// 1ms-tick `Engine::increment_epoch()` ticker spawned by
    /// [`crate::PluginHost`]; per call,
    /// `Store::set_epoch_deadline(N)` is called with `N` =
    /// `wall_time_budget / 1ms`. When the deadline fires the engine
    /// traps with `wasmtime::Trap::Interrupt`, which the host
    /// converts to [`crate::PluginError::WallTimeExceeded`].
    ///
    /// Default is 50 ms (the Phase 46 spec target). Sub-millisecond
    /// granularity rounds up to one tick because the ticker fires
    /// at 1 kHz.
    pub wall_time_budget: Duration,
}

impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            fuel_per_call: 1_000_000,
            // 64 MiB matches the spec's "64 MiB max" target.
            max_memory_bytes: 64 * 1024 * 1024,
            // 50 ms matches the spec's wall-clock target.
            wall_time_budget: Duration::from_millis(50),
        }
    }
}
