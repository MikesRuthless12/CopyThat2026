//! Phase 46.5 smoke: exercise the runtime against the compiled
//! sample plugins under `apps/copythat-ui/plugins/`.
//!
//! Filename note: `sample_plugins` (no `patch`/`setup`/`install`/
//! `update` substring) keeps the Windows test binary clear of the
//! UAC installer-detection heuristic.
//!
//! These tests **skip themselves** when the corresponding `.wasm`
//! artifact is missing, so a fresh checkout that hasn't run
//! `xtask build-sample-plugins` (or doesn't have the
//! `wasm32-unknown-unknown` target installed) still passes
//! `cargo test -p copythat-plugin`. CI runs `xtask
//! build-sample-plugins` first, so the artifacts are present and
//! the bodies execute.
//!
//! Each plugin gets:
//! - A round-trip test that runs the plugin against a representative
//!   `HookCtx::data` payload and asserts the right `HookOutcome`
//!   variant.
//! - A degenerate-input test that asserts the plugin falls back to
//!   `HookOutcome::Continue` rather than panicking when the input is
//!   missing the fields it expects.

use std::path::{Path, PathBuf};

use copythat_plugin::{Capability, CapabilityGrant, HookCtx, HookKind, HookOutcome, PluginHost};
use serde_json::json;

/// Resolve `<repo_root>/apps/copythat-ui/plugins/<dir>/target/wasm32-unknown-unknown/release/<crate_underscored>.wasm`.
/// Returns `None` if the artifact is missing — every test treats
/// `None` as "skip with eprintln" so machines without the wasm32
/// target stay green.
fn sample_wasm(dir_name: &str, crate_underscored: &str) -> Option<PathBuf> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    // CARGO_MANIFEST_DIR is `<repo>/crates/copythat-plugin`; back up
    // two levels to reach the workspace root.
    let repo_root = manifest_dir.parent()?.parent()?;
    let wasm = repo_root
        .join("apps")
        .join("copythat-ui")
        .join("plugins")
        .join(dir_name)
        .join("target")
        .join("wasm32-unknown-unknown")
        .join("release")
        .join(format!("{crate_underscored}.wasm"));
    if wasm.is_file() { Some(wasm) } else { None }
}

/// `eprintln` + early-return helper. The test name is the same as
/// the function so the skip message reads naturally in CI logs.
macro_rules! skip_unless_built {
    ($wasm:expr, $test_name:literal) => {
        match $wasm {
            Some(p) => p,
            None => {
                eprintln!(
                    "{}: skipped — sample plugin not built. Run `xtask build-sample-plugins` to enable.",
                    $test_name
                );
                return;
            }
        }
    };
}

// ---------------------------------------------------------------------------
// organize-by-exif
// ---------------------------------------------------------------------------

#[tokio::test]
async fn organize_by_exif_proposes_yyyy_mm_layout() {
    let wasm = skip_unless_built!(
        sample_wasm("organize-by-exif", "organize_by_exif"),
        "organize_by_exif_proposes_yyyy_mm_layout"
    );
    let host = PluginHost::new();
    let grant = CapabilityGrant::from_iter([
        Capability::ReadFs {
            scope: "source".into(),
        },
        Capability::WriteFs {
            scope: "dest".into(),
        },
    ]);
    let handle = host
        .load_plugin_with_grant(&wasm, grant)
        .expect("load_plugin_with_grant");

    let ctx = HookCtx {
        kind: None,
        data: json!({
            "src_path": "/source/IMG_0001.jpg",
            "dest_path": "/dest/IMG_0001.jpg",
            "dest_root": "/dest",
            "exif": { "date_taken_iso": "2024-03-15T10:30:00Z" }
        }),
    };
    let outcome = handle
        .call_hook(HookKind::AfterFile, ctx)
        .await
        .expect("call_hook");
    match outcome {
        HookOutcome::Notify { message } => {
            assert!(
                message.contains("/dest/2024/03/IMG_0001.jpg"),
                "expected proposed path in message, got: {message}"
            );
            assert!(
                message.contains("/source/IMG_0001.jpg"),
                "expected source path in message, got: {message}"
            );
        }
        other => panic!("expected Notify, got {other:?}"),
    }
}

#[tokio::test]
async fn organize_by_exif_falls_back_to_continue_without_exif() {
    let wasm = skip_unless_built!(
        sample_wasm("organize-by-exif", "organize_by_exif"),
        "organize_by_exif_falls_back_to_continue_without_exif"
    );
    let host = PluginHost::new();
    let grant = CapabilityGrant::from_iter([
        Capability::ReadFs {
            scope: "source".into(),
        },
        Capability::WriteFs {
            scope: "dest".into(),
        },
    ]);
    let handle = host
        .load_plugin_with_grant(&wasm, grant)
        .expect("load_plugin_with_grant");

    let ctx = HookCtx {
        kind: None,
        // src_path + dest_path supplied, but no exif block → plugin
        // has no DateTaken to anchor the YYYY/MM proposal on, so the
        // documented fallback kicks in.
        data: json!({
            "src_path": "/source/IMG_0002.jpg",
            "dest_path": "/dest/IMG_0002.jpg",
            "dest_root": "/dest"
        }),
    };
    let outcome = handle
        .call_hook(HookKind::AfterFile, ctx)
        .await
        .expect("call_hook");
    assert_eq!(outcome, HookOutcome::Continue);
}

// ---------------------------------------------------------------------------
// notify-discord
// ---------------------------------------------------------------------------

#[tokio::test]
async fn notify_discord_emits_webhook_envelope() {
    let wasm = skip_unless_built!(
        sample_wasm("notify-discord", "notify_discord"),
        "notify_discord_emits_webhook_envelope"
    );
    let host = PluginHost::new();
    let grant = CapabilityGrant::from_iter([Capability::Network]);
    let handle = host
        .load_plugin_with_grant(&wasm, grant)
        .expect("load_plugin_with_grant");

    let ctx = HookCtx {
        kind: None,
        data: json!({
            "job_id": "test-job-001",
            "files_total": 1234u64,
            "files_failed": 0u64,
            "bytes_total": 9_876_543_210u64,
            "elapsed_ms": 42_000u64,
            "webhook_url": "https://discord.com/api/webhooks/123/abc"
        }),
    };
    let outcome = handle
        .call_hook(HookKind::AfterJob, ctx)
        .await
        .expect("call_hook");

    let message = match outcome {
        HookOutcome::Notify { message } => message,
        other => panic!("expected Notify, got {other:?}"),
    };

    // The plugin's `message` is itself a JSON envelope the engine
    // decodes — re-parse it here and assert the fields the engine
    // depends on (target / url / payload shape).
    let envelope: serde_json::Value =
        serde_json::from_str(&message).expect("envelope must be valid JSON");
    assert_eq!(envelope["target"], "discord");
    assert_eq!(envelope["url"], "https://discord.com/api/webhooks/123/abc");
    let payload = &envelope["payload"];
    assert!(
        payload["content"].as_str().unwrap().contains("1234 files"),
        "content must summarise files copied, got: {}",
        payload["content"]
    );
    assert_eq!(
        payload["embeds"][0]["title"],
        "Freally File Manager — job complete"
    );
}

#[tokio::test]
async fn notify_discord_skips_without_webhook_url() {
    let wasm = skip_unless_built!(
        sample_wasm("notify-discord", "notify_discord"),
        "notify_discord_skips_without_webhook_url"
    );
    let host = PluginHost::new();
    let grant = CapabilityGrant::from_iter([Capability::Network]);
    let handle = host
        .load_plugin_with_grant(&wasm, grant)
        .expect("load_plugin_with_grant");

    let ctx = HookCtx {
        kind: None,
        data: json!({
            "job_id": "test-job-002",
            "files_total": 10u64,
        }),
    };
    let outcome = handle
        .call_hook(HookKind::AfterJob, ctx)
        .await
        .expect("call_hook");
    assert_eq!(outcome, HookOutcome::Continue);
}

// ---------------------------------------------------------------------------
// notify-ntfy
// ---------------------------------------------------------------------------

#[tokio::test]
async fn notify_ntfy_emits_topic_envelope() {
    let wasm = skip_unless_built!(
        sample_wasm("notify-ntfy", "notify_ntfy"),
        "notify_ntfy_emits_topic_envelope"
    );
    let host = PluginHost::new();
    let grant = CapabilityGrant::from_iter([Capability::Network]);
    let handle = host
        .load_plugin_with_grant(&wasm, grant)
        .expect("load_plugin_with_grant");

    let ctx = HookCtx {
        kind: None,
        data: json!({
            "job_id": "test-job-003",
            "files_total": 7u64,
            "files_failed": 1u64,
            "bytes_total": 1_500_000u64,
            "elapsed_ms": 4_500u64,
            "ntfy_topic_url": "https://ntfy.sh/copythat-mike",
            "priority": "high"
        }),
    };
    let outcome = handle
        .call_hook(HookKind::AfterJob, ctx)
        .await
        .expect("call_hook");
    let message = match outcome {
        HookOutcome::Notify { message } => message,
        other => panic!("expected Notify, got {other:?}"),
    };
    let envelope: serde_json::Value =
        serde_json::from_str(&message).expect("envelope must be valid JSON");
    assert_eq!(envelope["target"], "ntfy");
    assert_eq!(envelope["url"], "https://ntfy.sh/copythat-mike");
    assert_eq!(envelope["headers"]["Priority"], "high");
    assert_eq!(
        envelope["headers"]["Title"],
        "Freally File Manager — job complete"
    );
    let body = envelope["body"].as_str().expect("body is string");
    assert!(body.contains("7 files"), "body summarises files: {body}");
    assert!(body.contains("test-job-003"), "body carries job id: {body}");
}

#[tokio::test]
async fn notify_ntfy_clamps_unknown_priority_to_default() {
    let wasm = skip_unless_built!(
        sample_wasm("notify-ntfy", "notify_ntfy"),
        "notify_ntfy_clamps_unknown_priority_to_default"
    );
    let host = PluginHost::new();
    let grant = CapabilityGrant::from_iter([Capability::Network]);
    let handle = host
        .load_plugin_with_grant(&wasm, grant)
        .expect("load_plugin_with_grant");

    let ctx = HookCtx {
        kind: None,
        data: json!({
            "ntfy_topic_url": "https://ntfy.sh/topic",
            // hostile / mistyped value — the plugin must collapse it
            // to "default" so the eventual HTTP POST cannot smuggle a
            // header value the user didn't intend.
            "priority": "X-Header-Injection: 1\r\nX-Other: yes"
        }),
    };
    let outcome = handle
        .call_hook(HookKind::AfterJob, ctx)
        .await
        .expect("call_hook");
    let message = match outcome {
        HookOutcome::Notify { message } => message,
        other => panic!("expected Notify, got {other:?}"),
    };
    let envelope: serde_json::Value = serde_json::from_str(&message).expect("envelope JSON");
    assert_eq!(envelope["headers"]["Priority"], "default");
}

// ---------------------------------------------------------------------------
// dedup-warning
// ---------------------------------------------------------------------------

#[tokio::test]
async fn dedup_warning_fires_above_threshold() {
    let wasm = skip_unless_built!(
        sample_wasm("dedup-warning", "dedup_warning"),
        "dedup_warning_fires_above_threshold"
    );
    let host = PluginHost::new();
    // dedup-warning declares no capabilities; an empty grant
    // suffices.
    let handle = host.load_plugin(&wasm).expect("load_plugin");

    let ctx = HookCtx {
        kind: None,
        data: json!({
            "job_id": "dedup-job",
            "duplicates_found": 47u64,
            "files_total": 100u64,
            "threshold": 10u64,
        }),
    };
    let outcome = handle
        .call_hook(HookKind::AfterJob, ctx)
        .await
        .expect("call_hook");
    match outcome {
        HookOutcome::Notify { message } => {
            assert!(message.contains("47 duplicates"), "{message}");
            assert!(message.contains("threshold 10"), "{message}");
            assert!(message.contains("47.0%"), "{message}");
        }
        other => panic!("expected Notify, got {other:?}"),
    }
}

#[tokio::test]
async fn dedup_warning_quiet_below_threshold() {
    let wasm = skip_unless_built!(
        sample_wasm("dedup-warning", "dedup_warning"),
        "dedup_warning_quiet_below_threshold"
    );
    let host = PluginHost::new();
    let handle = host.load_plugin(&wasm).expect("load_plugin");

    let ctx = HookCtx {
        kind: None,
        data: json!({
            "duplicates_found": 3u64,
            "threshold": 10u64,
        }),
    };
    let outcome = handle
        .call_hook(HookKind::AfterJob, ctx)
        .await
        .expect("call_hook");
    assert_eq!(outcome, HookOutcome::Continue);
}

#[tokio::test]
async fn dedup_warning_uses_default_threshold_when_absent() {
    let wasm = skip_unless_built!(
        sample_wasm("dedup-warning", "dedup_warning"),
        "dedup_warning_uses_default_threshold_when_absent"
    );
    let host = PluginHost::new();
    let handle = host.load_plugin(&wasm).expect("load_plugin");

    // 11 > default threshold (10) → must fire.
    let ctx = HookCtx {
        kind: None,
        data: json!({
            "duplicates_found": 11u64,
        }),
    };
    let outcome = handle
        .call_hook(HookKind::AfterJob, ctx)
        .await
        .expect("call_hook");
    assert!(matches!(outcome, HookOutcome::Notify { .. }), "{outcome:?}");
}
