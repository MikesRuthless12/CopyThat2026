//! Phase 46.4 smoke: manifest round-trip + capability gate +
//! wall-clock budget enforcement.
//!
//! Filename note: `manifest` (no `patch`/`setup`/`install`/`update`
//! substring) keeps the Windows test binary clear of the UAC
//! installer-detection heuristic.
//!
//! Three smoke groups:
//! 1. Manifest round-trip — valid `plugin.toml` parses, malformed
//!    versions / empty hooks / unknown capabilities reject.
//! 2. `read_fs:source` denied → `PluginError::CapabilityDenied`
//!    surfaces before the WASM instance is built.
//! 3. Wall-time exceeded → `PluginError::WallTimeExceeded`
//!    surfaces from a tiny-budget config when the plugin runs an
//!    unbounded loop.

use std::path::PathBuf;
use std::time::Duration;

use copythat_plugin::{
    Capability, CapabilityGrant, HookCtx, HookKind, HookOutcome, PluginConfig, PluginError,
    PluginHost, PluginManifest,
};

// ---------------------------------------------------------------------------
// Manifest round-trip
// ---------------------------------------------------------------------------

#[test]
fn manifest_round_trip_parses_capabilities_and_hooks() {
    let src = r#"
name = "exif-rename"
version = "1.2.3"
hooks = ["after_file"]
capabilities = ["read_fs:source", "write_fs:dest"]
"#;
    let m = PluginManifest::parse(src).expect("must parse");
    assert_eq!(m.name, "exif-rename");
    assert_eq!(m.version, "1.2.3");
    assert_eq!(m.hooks, vec![HookKind::AfterFile]);
    assert_eq!(
        m.capabilities,
        vec![
            Capability::ReadFs {
                scope: "source".into()
            },
            Capability::WriteFs {
                scope: "dest".into()
            },
        ]
    );
}

#[test]
fn manifest_rejects_malformed_version() {
    let src = r#"
name = "x"
version = "1.0"
hooks = ["before_job"]
"#;
    let err = PluginManifest::parse(src).expect_err("must reject");
    assert!(matches!(err, PluginError::Manifest(_)), "{err:?}");
}

#[test]
fn manifest_rejects_empty_hooks() {
    let src = r#"
name = "x"
version = "0.1.0"
hooks = []
"#;
    let err = PluginManifest::parse(src).expect_err("must reject");
    assert!(matches!(err, PluginError::Manifest(_)), "{err:?}");
}

#[test]
fn manifest_rejects_unknown_capability() {
    let src = r#"
name = "x"
version = "0.1.0"
hooks = ["before_job"]
capabilities = ["filesystem:everywhere"]
"#;
    let err = PluginManifest::parse(src).expect_err("must reject");
    assert!(matches!(err, PluginError::Manifest(_)), "{err:?}");
}

// ---------------------------------------------------------------------------
// Capability gate
// ---------------------------------------------------------------------------

/// Tiny plugin that would happily return `HookOutcome::Continue` if
/// it ever ran. The capability-denied test never reaches it — the
/// host trips on the manifest's declared `read_fs:source` before
/// building an instance — but a working module is still required so
/// `load_plugin` itself succeeds.
const CONTINUE_WAT: &str = r#"
(module
  (memory (export "memory") 1)
  (data (i32.const 1024) "{\"kind\":\"continue\"}")
  (global $bump (mut i32) (i32.const 4096))

  (func (export "alloc") (param $size i32) (result i32)
    (local $ptr i32)
    (local.set $ptr (global.get $bump))
    (global.set $bump (i32.add (global.get $bump) (local.get $size)))
    (local.get $ptr))

  (func (export "hook") (param $ctx_ptr i32) (param $ctx_len i32) (result i64)
    (i64.or
      (i64.shl (i64.const 1024) (i64.const 32))
      (i64.const 19)))
)
"#;

const FILE_READER_MANIFEST: &str = r#"
name = "file-reader"
version = "0.1.0"
hooks = ["before_file"]
capabilities = ["read_fs:source"]
"#;

fn write_plugin(wat: &str, manifest: &str) -> (tempfile::TempDir, PathBuf) {
    let dir = tempfile::tempdir().expect("tempdir");
    let wasm = dir.path().join("plugin.wat");
    let toml = dir.path().join("plugin.toml");
    std::fs::write(&wasm, wat).expect("write wat");
    std::fs::write(&toml, manifest).expect("write toml");
    (dir, wasm)
}

#[tokio::test]
async fn read_fs_source_denied_blocks_the_call() {
    // A plugin whose manifest declares `read_fs:source` but whose
    // grant is empty — so the pre-call gate trips on the missing
    // capability and the dispatch returns `CapabilityDenied`
    // before the engine builds an instance. The smoke test
    // matches the spec's "read_fs:source denied → plugin can't
    // open files" assertion: no host-mediated file IO can happen
    // because `call_hook` never gets that far.
    let (_dir, wasm) = write_plugin(CONTINUE_WAT, FILE_READER_MANIFEST);
    let host = PluginHost::new();
    let handle = host
        .load_plugin_with_grant(&wasm, CapabilityGrant::empty())
        .expect("plugin loads even when caps are denied");

    let err = handle
        .call_hook(HookKind::BeforeFile, HookCtx::default())
        .await
        .expect_err("denied capability must trip the pre-call gate");
    match err {
        PluginError::CapabilityDenied { plugin, capability } => {
            assert_eq!(plugin, "file-reader");
            assert_eq!(capability, "read_fs:source");
        }
        other => panic!("expected CapabilityDenied, got {other:?}"),
    }
}

#[tokio::test]
async fn read_fs_source_granted_passes_the_gate() {
    // Same plugin + manifest, but the grant now includes
    // `read_fs:source`. The pre-call gate passes, the engine
    // instantiates, and the plugin returns `HookOutcome::Continue`.
    let (_dir, wasm) = write_plugin(CONTINUE_WAT, FILE_READER_MANIFEST);
    let host = PluginHost::new();
    let grant = CapabilityGrant::from_iter([Capability::ReadFs {
        scope: "source".into(),
    }]);
    let handle = host
        .load_plugin_with_grant(&wasm, grant)
        .expect("load_plugin");

    let outcome = handle
        .call_hook(HookKind::BeforeFile, HookCtx::default())
        .await
        .expect("granted capability must let the call through");
    assert_eq!(outcome, HookOutcome::Continue);
}

// ---------------------------------------------------------------------------
// Wall-clock budget
// ---------------------------------------------------------------------------

/// Hook is an unconditional infinite loop that does **not** burn
/// fuel-equivalent units fast enough to reach an `OutOfFuel` trap
/// before the wall budget fires. With a 1ms wall budget and
/// `fuel_per_call` set high (1B), the wall ticker trips first.
const WALL_LOOP_WAT: &str = r#"
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

const WALL_LOOP_MANIFEST: &str = r#"
name = "wall-loop"
version = "0.1.0"
hooks = ["before_file"]
"#;

#[tokio::test]
async fn wall_time_exceeded_when_plugin_runs_past_budget() {
    // 1 ms wall budget against the 1-tick-per-ms ticker should
    // fire well before the 1-billion fuel cap could ever burn out
    // on an empty `(loop (br $burn))`. Result: the host catches
    // `Trap::Interrupt` and surfaces `WallTimeExceeded`.
    let (_dir, wasm) = write_plugin(WALL_LOOP_WAT, WALL_LOOP_MANIFEST);
    let host = PluginHost::with_config(PluginConfig {
        fuel_per_call: 1_000_000_000,
        max_memory_bytes: 64 * 1024 * 1024,
        wall_time_budget: Duration::from_millis(1),
    });
    let handle = host.load_plugin(&wasm).expect("load_plugin");

    let err = handle
        .call_hook(HookKind::BeforeFile, HookCtx::default())
        .await
        .expect_err("infinite loop with 1ms wall budget must trip");
    assert!(matches!(err, PluginError::WallTimeExceeded), "{err:?}");
}
