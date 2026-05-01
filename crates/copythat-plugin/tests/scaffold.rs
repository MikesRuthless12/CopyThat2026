//! Phase 46.1 scaffold smoke: confirms the public type surface
//! compiles and serde derives round-trip the JSON shapes the WASM
//! ABI plugins compile against. Real dispatch is exercised by
//! `tests/dispatch.rs` (Phase 46.2).

use copythat_plugin::{HookCtx, HookKind, HookOutcome};

#[test]
fn hook_kind_round_trips_as_snake_case() {
    let json = serde_json::to_string(&HookKind::BeforeJob).unwrap();
    assert_eq!(json, "\"before_job\"");
    let back: HookKind = serde_json::from_str(&json).unwrap();
    assert_eq!(back, HookKind::BeforeJob);
}

#[test]
fn hook_outcome_round_trips_with_kind_tag() {
    let cases = [
        (HookOutcome::Continue, r#"{"kind":"continue"}"#),
        (HookOutcome::SkipFile, r#"{"kind":"skip_file"}"#),
        (HookOutcome::AbortJob, r#"{"kind":"abort_job"}"#),
        (
            HookOutcome::Notify {
                message: "ok".into(),
            },
            r#"{"kind":"notify","message":"ok"}"#,
        ),
    ];
    for (variant, expected) in cases {
        let actual = serde_json::to_string(&variant).unwrap();
        assert_eq!(actual, expected, "serialize {variant:?}");
        let back: HookOutcome = serde_json::from_str(expected).unwrap();
        assert_eq!(back, variant, "round-trip {variant:?}");
    }
}

#[test]
fn hook_ctx_default_serialises_to_empty_object() {
    let ctx = HookCtx::default();
    assert_eq!(serde_json::to_string(&ctx).unwrap(), "{}");
}

#[test]
fn hook_outcome_default_is_continue() {
    assert_eq!(HookOutcome::default(), HookOutcome::Continue);
}
