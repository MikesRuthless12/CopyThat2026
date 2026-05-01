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

use std::io::Write;

use copythat_plugin::{HookCtx, HookKind, HookOutcome, PluginError, PluginHost};

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
    let tmp = write_wat(SKIP_FILE_WAT);
    let host = PluginHost::new();
    let handle = host.load_plugin(tmp.path()).expect("load_plugin");

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
    let tmp = write_wat(MISSING_HOOK_WAT);
    let host = PluginHost::new();
    let handle = host.load_plugin(tmp.path()).expect("load_plugin");

    let err = handle
        .call_hook(HookKind::BeforeFile, HookCtx::default())
        .await
        .expect_err("missing `hook` export must surface");
    assert!(
        matches!(err, PluginError::MissingExport("hook")),
        "{err:?}"
    );
}
