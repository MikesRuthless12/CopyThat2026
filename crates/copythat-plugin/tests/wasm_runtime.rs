//! Phase 46.2 wasm-runtime smoke: builds a tiny WAT plugin that
//! always returns `HookOutcome::SkipFile`, loads it through
//! `PluginHost::load_plugin`, dispatches via
//! `PluginHandle::call_hook`, and asserts the outcome decodes back
//! correctly through the JSON-over-linear-memory ABI.
//!
//! Filename note: the binary is `wasm_runtime.exe` (not the more
//! obvious `dispatch.exe`) because Windows UAC installer detection
//! treats any PE whose name *contains* `patch` / `setup` / `install`
//! / `update` etc. as an installer and refuses to launch it without
//! elevation. "dis**patch**" trips that heuristic; "wasm_runtime"
//! doesn't.
//!
//! The plugin exports `memory`, `alloc`, `dealloc` (no-op for the
//! smoke), and `hook`. The data segment at offset 1024 holds the
//! pre-baked 20-byte response `{"kind":"skip_file"}`; `hook` always
//! returns the packed `(1024, 20)` pair regardless of input.
//!
//! 46.4 added the `plugin.toml` manifest contract. Every test that
//! exercises a successful load now uses [`write_plugin`] to drop a
//! matching `plugin.toml` next to the WAT file; the
//! invalid-module / missing-file tests keep using bare
//! `NamedTempFile` because they fail before the manifest is read.

use std::io::Write;
use std::path::PathBuf;

use copythat_plugin::{
    HookCtx, HookKind, HookOutcome, PluginConfig, PluginError, PluginHost,
};

const DEFAULT_MANIFEST: &str = r#"
name = "wasm_runtime_smoke"
version = "0.1.0"
hooks = ["before_file", "after_file", "before_job", "after_job", "on_error"]
"#;

/// Drop a `plugin.wat` + `plugin.toml` pair into a fresh temp dir
/// and hand back the directory (kept alive by the caller for
/// cleanup) and the path to the WAT file. Hosts the new 46.4
/// manifest contract: `load_plugin` reads `<wasm.parent>/plugin.toml`,
/// so co-locating both files in one temp dir is the simplest setup.
fn write_plugin(wat: &str, manifest: &str) -> (tempfile::TempDir, PathBuf) {
    let dir = tempfile::tempdir().expect("tempdir");
    let wat_path = dir.path().join("plugin.wat");
    let toml_path = dir.path().join("plugin.toml");
    std::fs::write(&wat_path, wat).expect("write wat");
    std::fs::write(&toml_path, manifest).expect("write toml");
    (dir, wat_path)
}

const SKIP_FILE_WAT: &str = r#"
(module
  (memory (export "memory") 1)

  ;; Pre-baked response JSON at offset 1024. After WAT escape
  ;; processing this is exactly 20 bytes: {"kind":"skip_file"}
  (data (i32.const 1024) "{\"kind\":\"skip_file\"}")

  ;; Bump allocator starting at offset 4096 so it never collides
  ;; with the pre-baked response at offset 1024.
  (global $bump (mut i32) (i32.const 4096))

  (func (export "alloc") (param $size i32) (result i32)
    (local $ptr i32)
    (local.set $ptr (global.get $bump))
    (global.set $bump (i32.add (global.get $bump) (local.get $size)))
    (local.get $ptr))

  (func (export "dealloc") (param $ptr i32) (param $size i32))

  ;; hook ignores its input and returns packed (ptr=1024, len=20)
  ;; as i64: (1024 << 32) | 20.
  (func (export "hook") (param $ctx_ptr i32) (param $ctx_len i32) (result i64)
    (i64.or
      (i64.shl (i64.const 1024) (i64.const 32))
      (i64.const 20)))
)
"#;

const MISSING_HOOK_WAT: &str = r#"
(module
  (memory (export "memory") 1)
  (func (export "alloc") (param $size i32) (result i32) (i32.const 0))
)
"#;

fn write_wat(wat: &str) -> tempfile::NamedTempFile {
    let mut tmp = tempfile::Builder::new()
        .suffix(".wat")
        .tempfile()
        .expect("tempfile");
    tmp.write_all(wat.as_bytes()).expect("write WAT");
    tmp.flush().expect("flush");
    tmp
}

#[tokio::test]
async fn wat_plugin_returns_skip_file() {
    let (_dir, wasm) = write_plugin(SKIP_FILE_WAT, DEFAULT_MANIFEST);
    let host = PluginHost::new();
    let handle = host.load_plugin(&wasm).expect("load_plugin");

    let outcome = handle
        .call_hook(HookKind::BeforeFile, HookCtx::default())
        .await
        .expect("call_hook");

    assert_eq!(outcome, HookOutcome::SkipFile);
}

#[test]
fn loading_invalid_module_returns_wasmtime_error() {
    let tmp = write_wat("this is not valid wat");
    let host = PluginHost::new();
    let err = host
        .load_plugin(tmp.path())
        .expect_err("invalid WAT must error");
    assert!(matches!(err, PluginError::Wasmtime(_)), "{err:?}");
}

#[test]
fn loading_missing_file_returns_io_error() {
    let host = PluginHost::new();
    let err = host
        .load_plugin(std::path::Path::new(
            "this_file_definitely_does_not_exist.wasm",
        ))
        .expect_err("missing file must error");
    assert!(matches!(err, PluginError::Io(_)), "{err:?}");
}

#[tokio::test]
async fn missing_hook_export_is_diagnosed() {
    let (_dir, wasm) = write_plugin(MISSING_HOOK_WAT, DEFAULT_MANIFEST);
    let host = PluginHost::new();
    let handle = host.load_plugin(&wasm).expect("load_plugin");

    let err = handle
        .call_hook(HookKind::BeforeFile, HookCtx::default())
        .await
        .expect_err("missing `hook` export must surface");
    assert!(
        matches!(err, PluginError::MissingExport("hook")),
        "{err:?}"
    );
}

// ---------------------------------------------------------------------------
// Phase 46.3 — sandbox budget tests
// ---------------------------------------------------------------------------

/// `hook` is an unconditional infinite loop. With a small fuel
/// budget the engine traps with `wasmtime::Trap::OutOfFuel`, which
/// the host converts to `PluginError::FuelExhausted`. The
/// `(unreachable)` after the loop satisfies the i64 return-type
/// checker — the loop body diverges via `br`, so control never
/// reaches the unreachable instruction at runtime.
const INFINITE_LOOP_WAT: &str = r#"
(module
  (memory (export "memory") 1)
  (global $bump (mut i32) (i32.const 4096))

  (func (export "alloc") (param $size i32) (result i32)
    (local $ptr i32)
    (local.set $ptr (global.get $bump))
    (global.set $bump (i32.add (global.get $bump) (local.get $size)))
    (local.get $ptr))

  (func (export "hook") (param $ctx_ptr i32) (param $ctx_len i32) (result i64)
    (loop $burn (br $burn))
    (unreachable))
)
"#;

/// `hook` repeatedly tries to grow linear memory by one page each
/// iteration. With a low `max_memory_bytes` cap the limiter
/// returns `Err(MemoryRejectedMarker)` on the first growth that
/// would exceed it, the engine surfaces that as a trap, and the
/// host converts it to `PluginError::MemoryExceeded`.
const MEMORY_GROW_LOOP_WAT: &str = r#"
(module
  (memory (export "memory") 1)
  (global $bump (mut i32) (i32.const 4096))

  (func (export "alloc") (param $size i32) (result i32)
    (local $ptr i32)
    (local.set $ptr (global.get $bump))
    (global.set $bump (i32.add (global.get $bump) (local.get $size)))
    (local.get $ptr))

  (func (export "hook") (param $ctx_ptr i32) (param $ctx_len i32) (result i64)
    (loop $grow
      (drop (memory.grow (i32.const 1)))
      (br $grow))
    (unreachable))
)
"#;

#[tokio::test]
async fn fuel_exhausted_when_plugin_burns_more_than_budget() {
    let (_dir, wasm) = write_plugin(INFINITE_LOOP_WAT, DEFAULT_MANIFEST);
    // 10,000 fuel is enough to instantiate the module + run a
    // handful of loop iterations, but nowhere near enough to keep
    // an unconditional loop running. The engine traps with
    // `OutOfFuel` long before the test could time out. The wall
    // budget is left at the default 50ms; a 10k-fuel cap trips
    // first, so the test asserts `FuelExhausted` rather than
    // racing against `WallTimeExceeded`.
    let host = PluginHost::with_config(PluginConfig {
        fuel_per_call: 10_000,
        ..PluginConfig::default()
    });
    let handle = host.load_plugin(&wasm).expect("load_plugin");

    let err = handle
        .call_hook(HookKind::BeforeFile, HookCtx::default())
        .await
        .expect_err("infinite loop must trip the fuel cap");
    assert!(matches!(err, PluginError::FuelExhausted), "{err:?}");
}

#[tokio::test]
async fn memory_exceeded_when_plugin_grows_past_cap() {
    let (_dir, wasm) = write_plugin(MEMORY_GROW_LOOP_WAT, DEFAULT_MANIFEST);
    // WASM pages are 64 KiB. Initial memory in the WAT is 1 page
    // (64 KiB); the cap is 128 KiB so the first `memory.grow(1)`
    // succeeds (1→2 pages) and the second is rejected (2→3 pages
    // = 192 KiB > 128 KiB). Fuel is left high so the memory cap
    // is what trips, not fuel exhaustion.
    let host = PluginHost::with_config(PluginConfig {
        fuel_per_call: 1_000_000,
        max_memory_bytes: 128 * 1024,
        ..PluginConfig::default()
    });
    let handle = host.load_plugin(&wasm).expect("load_plugin");

    let err = handle
        .call_hook(HookKind::BeforeFile, HookCtx::default())
        .await
        .expect_err("growth past cap must surface as MemoryExceeded");
    match err {
        PluginError::MemoryExceeded {
            wanted_bytes,
            max_bytes,
        } => {
            assert_eq!(max_bytes, 128 * 1024, "max_bytes echoes the cap");
            assert!(
                wanted_bytes > max_bytes,
                "wanted_bytes ({wanted_bytes}) must exceed the cap ({max_bytes})"
            );
        }
        other => panic!("expected MemoryExceeded, got {other:?}"),
    }
}

/// `hook` returns a packed pointer with `out_len = 0xFFFFFFFF`. Pre-46.7
/// the host blindly trusted the plugin-controlled length and would
/// `vec![0u8; 0xFFFFFFFF]` (≈4 GiB) before the subsequent
/// `Memory::read` could fail safely — a free DoS for any plugin the
/// user installed. 46.7's wrap pass added a clamp against
/// `PluginConfig::max_memory_bytes`; this test pins that the clamp
/// fires with `OutOfBounds` rather than panicking the host.
///
/// `alloc` is a real bump allocator (not the `(i32.const 0)` shim of
/// the negative-/null-alloc tests) so the new alloc-not-zero guard
/// in `handle.rs` doesn't trip first — this test is specifically
/// about the post-call `out_len` clamp.
const HUGE_OUT_LEN_WAT: &str = r#"
(module
  (memory (export "memory") 1)
  (global $bump (mut i32) (i32.const 4096))

  (func (export "alloc") (param $size i32) (result i32)
    (local $ptr i32)
    (local.set $ptr (global.get $bump))
    (global.set $bump (i32.add (global.get $bump) (local.get $size)))
    (local.get $ptr))

  ;; Always return packed (out_ptr=0, out_len=0xFFFFFFFF). The high
  ;; 32 bits encode out_ptr; we leave them zero so the failure is
  ;; unambiguously the length clamp tripping, not the pointer
  ;; walking past memory.
  (func (export "hook") (param $ctx_ptr i32) (param $ctx_len i32) (result i64)
    (i64.const 0xFFFFFFFF))
)
"#;

#[tokio::test]
async fn out_len_above_memory_cap_rejected_before_allocation() {
    let (_dir, wasm) = write_plugin(HUGE_OUT_LEN_WAT, DEFAULT_MANIFEST);
    let host = PluginHost::new();
    let handle = host.load_plugin(&wasm).expect("load_plugin");

    let err = handle
        .call_hook(HookKind::BeforeFile, HookCtx::default())
        .await
        .expect_err("4 GiB out_len must reject without OOM");
    match err {
        PluginError::OutOfBounds { ptr, len } => {
            assert_eq!(ptr, 0, "out_ptr echoed verbatim from packed return");
            assert_eq!(len, 0xFFFFFFFF, "out_len echoed verbatim from packed return");
        }
        other => panic!("expected OutOfBounds clamp, got {other:?}"),
    }
}

#[tokio::test]
async fn happy_path_still_works_with_default_sandbox_budgets() {
    // Sanity check: the default `PluginConfig` (1M fuel, 64 MiB
    // memory, 50 ms wall) doesn't accidentally trip on the
    // well-behaved skip-file plugin. This guards against future
    // regressions where someone tightens the defaults below the
    // cost of a trivial hook.
    let (_dir, wasm) = write_plugin(SKIP_FILE_WAT, DEFAULT_MANIFEST);
    let host = PluginHost::new();
    let handle = host.load_plugin(&wasm).expect("load_plugin");

    let outcome = handle
        .call_hook(HookKind::BeforeFile, HookCtx::default())
        .await
        .expect("call_hook under default budgets");
    assert_eq!(outcome, HookOutcome::SkipFile);
}

/// `alloc` always returns 0 (a literal null pointer). Without the
/// 46.7-followup zero-pointer guard the host would happily write
/// `ctx_json` at offset 0 — inside the WASM module's data segments
/// where a malicious plugin can stage a forged HookOutcome at the
/// same offset that `hook` later returns. The guard turns this into
/// a clean `OutOfBounds` before any host-side write.
const NULL_ALLOC_WAT: &str = r#"
(module
  (memory (export "memory") 1)
  (data (i32.const 1024) "{\"kind\":\"skip_file\"}")

  (func (export "alloc") (param $size i32) (result i32) (i32.const 0))

  ;; Returns packed (1024, 20). Without the zero-pointer guard this
  ;; would succeed and the host would decode the SkipFile outcome
  ;; even though `ctx_json` was just written over the data segment.
  (func (export "hook") (param $ctx_ptr i32) (param $ctx_len i32) (result i64)
    (i64.or
      (i64.shl (i64.const 1024) (i64.const 32))
      (i64.const 20)))
)
"#;

#[tokio::test]
async fn alloc_returning_null_pointer_is_rejected() {
    let (_dir, wasm) = write_plugin(NULL_ALLOC_WAT, DEFAULT_MANIFEST);
    let host = PluginHost::new();
    let handle = host.load_plugin(&wasm).expect("load_plugin");

    let err = handle
        .call_hook(HookKind::BeforeFile, HookCtx::default())
        .await
        .expect_err("alloc returning 0 must reject before write_memory");
    assert!(matches!(err, PluginError::OutOfBounds { ptr: 0, .. }), "{err:?}");
}

/// `alloc` returns -1 (`0xFFFFFFFF` reinterpreted as i32). Same
/// guard catches it — a negative pointer would otherwise widen to
/// `0xFFFFFFFF` and walk past every page of plugin memory before
/// the engine's own bounds check fires.
const NEGATIVE_ALLOC_WAT: &str = r#"
(module
  (memory (export "memory") 1)

  (func (export "alloc") (param $size i32) (result i32) (i32.const -1))

  (func (export "hook") (param $ctx_ptr i32) (param $ctx_len i32) (result i64)
    (i64.const 0))
)
"#;

#[tokio::test]
async fn alloc_returning_negative_pointer_is_rejected() {
    let (_dir, wasm) = write_plugin(NEGATIVE_ALLOC_WAT, DEFAULT_MANIFEST);
    let host = PluginHost::new();
    let handle = host.load_plugin(&wasm).expect("load_plugin");

    let err = handle
        .call_hook(HookKind::BeforeFile, HookCtx::default())
        .await
        .expect_err("negative alloc return must reject before write_memory");
    assert!(matches!(err, PluginError::OutOfBounds { .. }), "{err:?}");
}

/// `hook` returns an `out_ptr` that walks past the end of linear
/// memory while keeping `out_len` *under* the per-call memory cap
/// (so the 46.7 pre-allocation length clamp doesn't fire). The
/// `read_memory` call must catch the OOB read and surface
/// `OutOfBounds` rather than panicking.
const OOB_OUT_PTR_WAT: &str = r#"
(module
  (memory (export "memory") 1)
  (global $bump (mut i32) (i32.const 4096))

  (func (export "alloc") (param $size i32) (result i32)
    (local $ptr i32)
    (local.set $ptr (global.get $bump))
    (global.set $bump (i32.add (global.get $bump) (local.get $size)))
    (local.get $ptr))

  ;; Returns packed (out_ptr=0xFFFF0000, out_len=64) — a 64-byte read
  ;; starting near the 4 GiB mark, well past the single page of
  ;; linear memory. `out_len` is below max_memory_bytes so the clamp
  ;; doesn't fire; `Memory::read` must reject the read.
  (func (export "hook") (param $ctx_ptr i32) (param $ctx_len i32) (result i64)
    (i64.or
      (i64.shl (i64.const 0xFFFF0000) (i64.const 32))
      (i64.const 64)))
)
"#;

#[tokio::test]
async fn out_ptr_past_memory_with_small_len_rejects_via_read_clamp() {
    let (_dir, wasm) = write_plugin(OOB_OUT_PTR_WAT, DEFAULT_MANIFEST);
    let host = PluginHost::new();
    let handle = host.load_plugin(&wasm).expect("load_plugin");

    let err = handle
        .call_hook(HookKind::BeforeFile, HookCtx::default())
        .await
        .expect_err("OOB out_ptr must reject");
    match err {
        PluginError::OutOfBounds { ptr, len } => {
            assert_eq!(ptr, 0xFFFF_0000, "ptr echoed verbatim from packed return");
            assert_eq!(len, 64, "len echoed verbatim from packed return");
        }
        other => panic!("expected OutOfBounds from read_memory, got {other:?}"),
    }
}

/// `hook` returns a valid `(ptr, len)` pair pointing at non-JSON
/// bytes. The host should surface `Outcome` (the serde_json wrapper),
/// not panic.
const NON_JSON_OUTCOME_WAT: &str = r#"
(module
  (memory (export "memory") 1)
  ;; 8 bytes of garbage at offset 1024 — definitely not JSON.
  (data (i32.const 1024) "garbage!")
  (global $bump (mut i32) (i32.const 4096))

  (func (export "alloc") (param $size i32) (result i32)
    (local $ptr i32)
    (local.set $ptr (global.get $bump))
    (global.set $bump (i32.add (global.get $bump) (local.get $size)))
    (local.get $ptr))

  (func (export "hook") (param $ctx_ptr i32) (param $ctx_len i32) (result i64)
    (i64.or
      (i64.shl (i64.const 1024) (i64.const 32))
      (i64.const 8)))
)
"#;

#[tokio::test]
async fn malformed_outcome_json_surfaces_as_outcome_error() {
    let (_dir, wasm) = write_plugin(NON_JSON_OUTCOME_WAT, DEFAULT_MANIFEST);
    let host = PluginHost::new();
    let handle = host.load_plugin(&wasm).expect("load_plugin");

    let err = handle
        .call_hook(HookKind::BeforeFile, HookCtx::default())
        .await
        .expect_err("non-JSON outcome must surface as Outcome");
    assert!(matches!(err, PluginError::Outcome(_)), "{err:?}");
}
