//! Phase 31 smoke — power-aware copying.
//!
//! The brief asks for:
//! 1. Start a copy.
//! 2. Inject `BatteryStateChanged { on_battery: true }` → engine pauses.
//! 3. Inject `BatteryStateChanged { on_battery: false }` → engine resumes.
//! 4. Inject `PresentationStateChanged { presenting: true }` → engine pauses.
//!
//! The runner-level side of the pause path (queue.pause_job in a
//! loop, shape.set_rate for caps) lives in `copythat-ui-lib`; wiring
//! a full Tauri runtime into a smoke would drag in the entire UI
//! graph. Instead this smoke exercises the pure-library layer where
//! the Phase 31 logic actually decides things:
//!
//! - `apply_event` + `compute_action` for the four brief cases,
//!   under the user-configured default policies and an explicit
//!   "pause on battery" override.
//! - `PowerBus::spawn_poller` + `SyntheticProbes` to verify that a
//!   probe flip actually surfaces on the broadcast channel (the
//!   real end-to-end from OS probe → subscriber → pause_all).
//! - `PowerPoliciesSettings` TOML round-trip.
//! - Fluent key parity for the 18 Phase 31 keys across all 18
//!   locales.

use std::time::Duration;

use copythat_power::{
    BatteryPolicy, NetworkClass, PowerAction, PowerBus, PowerEvent, PowerPolicies, PowerReason,
    PowerState, PresentationPolicy, SyntheticProbes, ThermalKind, ThermalPolicy, apply_event,
    compute_action,
};
use copythat_settings::{PowerPoliciesSettings, PowerRuleChoice, Settings, ThermalRuleChoice};

const PHASE_31_KEYS: &[&str] = &[
    "power-heading",
    "power-enabled",
    "power-battery-label",
    "power-metered-label",
    "power-cellular-label",
    "power-presentation-label",
    "power-fullscreen-label",
    "power-thermal-label",
    "power-rule-continue",
    "power-rule-pause",
    "power-rule-cap",
    "power-rule-cap-percent",
    "power-reason-on-battery",
    "power-reason-metered-network",
    "power-reason-cellular-network",
    "power-reason-presenting",
    "power-reason-fullscreen",
    "power-reason-thermal-throttling",
];

const LOCALES: &[&str] = &[
    "en", "es", "zh-CN", "hi", "ar", "pt-BR", "ru", "ja", "de", "fr", "ko", "it", "tr", "vi", "pl",
    "nl", "id", "uk",
];

#[test]
fn case1_battery_on_pauses_under_pause_policy() {
    let policies = PowerPolicies {
        battery: BatteryPolicy::Pause,
        ..Default::default()
    };
    let mut s = PowerState::default();
    apply_event(
        &mut s,
        &PowerEvent::BatteryStateChanged {
            on_battery: true,
            percent: 42.0,
        },
    );
    let action = compute_action(&s, &policies);
    assert!(
        matches!(
            action,
            PowerAction::Pause {
                reason: PowerReason::OnBattery
            }
        ),
        "expected Pause(OnBattery), got {action:?}"
    );
}

#[test]
fn case2_battery_off_resumes() {
    let policies = PowerPolicies {
        battery: BatteryPolicy::Pause,
        ..Default::default()
    };
    let mut s = PowerState::default();
    apply_event(
        &mut s,
        &PowerEvent::BatteryStateChanged {
            on_battery: true,
            percent: 42.0,
        },
    );
    apply_event(
        &mut s,
        &PowerEvent::BatteryStateChanged {
            on_battery: false,
            percent: 100.0,
        },
    );
    assert_eq!(compute_action(&s, &policies), PowerAction::Continue);
}

#[test]
fn case3_presentation_pauses_under_default_policy() {
    // Default presentation policy is `Pause` per the Phase 31 brief.
    let policies = PowerPolicies::default();
    let mut s = PowerState::default();
    apply_event(
        &mut s,
        &PowerEvent::PresentationStateChanged { presenting: true },
    );
    let action = compute_action(&s, &policies);
    assert!(
        matches!(
            action,
            PowerAction::Pause {
                reason: PowerReason::Presenting
            }
        ),
        "expected Pause(Presenting), got {action:?}"
    );
}

#[test]
fn case4_thermal_default_surfaces_cap_sentinel() {
    // Default thermal policy is CapPercent(50); compute_action
    // encodes that as a `Cap { bytes_per_second: 0, … }` sentinel
    // so the runner can resolve "percent of current" against its
    // live shape state. This test locks the sentinel contract.
    let policies = PowerPolicies::default();
    assert!(matches!(
        policies.thermal,
        ThermalPolicy::CapPercent { .. }
    ));
    let mut s = PowerState::default();
    apply_event(
        &mut s,
        &PowerEvent::ThermalChanged {
            throttling: true,
            kind: ThermalKind::X86Cpuid,
        },
    );
    let action = compute_action(&s, &policies);
    match action {
        PowerAction::Cap {
            bytes_per_second,
            reason,
        } => {
            assert_eq!(bytes_per_second, 0, "sentinel cap");
            assert_eq!(reason, PowerReason::ThermalThrottling);
        }
        other => panic!("expected thermal Cap sentinel, got {other:?}"),
    }
}

#[test]
fn case5_bus_end_to_end_presentation_flip() {
    // Full poller → subscriber round-trip: flipping a synthetic
    // presentation probe must surface a `PresentationStateChanged`
    // on the broadcast within two tick windows.
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let bus = PowerBus::new();
        let mut rx = bus.subscribe();
        let probes = SyntheticProbes::new();
        let handle = bus.spawn_poller(probes.as_set(), Duration::from_millis(15));

        // Let the bootstrap tick flush.
        tokio::time::sleep(Duration::from_millis(30)).await;
        probes.set_presenting(true);

        let ev = tokio::time::timeout(Duration::from_millis(300), rx.recv())
            .await
            .expect("bus tick in window")
            .expect("recv");
        assert!(
            matches!(
                ev,
                PowerEvent::PresentationStateChanged { presenting: true }
            ),
            "expected presenting-true, got {ev:?}"
        );
        handle.abort();
    });
}

#[test]
fn case6_settings_toml_round_trip() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let path = tmp.path().join("settings.toml");

    let s = Settings {
        power: PowerPoliciesSettings {
            enabled: true,
            battery: PowerRuleChoice::Pause,
            metered: PowerRuleChoice::Cap {
                bytes_per_second: 2_000_000,
            },
            cellular: PowerRuleChoice::Pause,
            presentation: PowerRuleChoice::Pause,
            fullscreen: PowerRuleChoice::Continue,
            thermal: ThermalRuleChoice::CapPercent { percent: 40 },
        },
        ..Default::default()
    };
    s.save_to(&path).expect("save");
    let loaded = Settings::load_from(&path).expect("load");
    assert_eq!(loaded.power.battery, PowerRuleChoice::Pause);
    assert_eq!(
        loaded.power.metered,
        PowerRuleChoice::Cap {
            bytes_per_second: 2_000_000
        }
    );
    assert_eq!(
        loaded.power.thermal,
        ThermalRuleChoice::CapPercent { percent: 40 }
    );
    assert!(loaded.power.enabled);
}

#[test]
fn case7_fullscreen_default_does_not_pause() {
    // Regression guard on the documented default — fullscreen alone
    // (games, movies, IDEs) shouldn't interrupt a copy.
    let policies = PowerPolicies::default();
    let mut s = PowerState::default();
    apply_event(&mut s, &PowerEvent::FullscreenChanged { fullscreen: true });
    assert_eq!(compute_action(&s, &policies), PowerAction::Continue);
}

#[test]
fn case8_metered_cap_resolves_at_configured_rate() {
    let policies = PowerPolicies {
        metered: copythat_power::NetworkPolicy::Cap {
            bytes_per_second: 512 * 1024,
        },
        ..Default::default()
    };
    let mut s = PowerState::default();
    apply_event(
        &mut s,
        &PowerEvent::NetworkClassChanged {
            class: NetworkClass::Metered,
        },
    );
    match compute_action(&s, &policies) {
        PowerAction::Cap {
            bytes_per_second, ..
        } => assert_eq!(bytes_per_second, 512 * 1024),
        other => panic!("expected Cap, got {other:?}"),
    }
}

#[test]
fn case9_pause_dominates_cap_across_dimensions() {
    // Battery pause + metered cap → Pause wins regardless of order
    // the poller observes the transitions.
    let policies = PowerPolicies {
        battery: BatteryPolicy::Pause,
        metered: copythat_power::NetworkPolicy::Cap {
            bytes_per_second: 1_000_000,
        },
        presentation: PresentationPolicy::Continue,
        ..Default::default()
    };
    let mut s = PowerState::default();
    apply_event(
        &mut s,
        &PowerEvent::NetworkClassChanged {
            class: NetworkClass::Metered,
        },
    );
    apply_event(
        &mut s,
        &PowerEvent::BatteryStateChanged {
            on_battery: true,
            percent: 10.0,
        },
    );
    assert!(matches!(
        compute_action(&s, &policies),
        PowerAction::Pause {
            reason: PowerReason::OnBattery
        }
    ));
}

#[test]
fn case10_fluent_keys_present_in_en() {
    let en =
        std::fs::read_to_string(repo_root().join("locales/en/copythat.ftl")).expect("read en");
    for key in PHASE_31_KEYS {
        let needle = format!("\n{key} = ");
        assert!(
            en.contains(&needle),
            "Phase 31 key `{key}` missing from locales/en/copythat.ftl"
        );
    }
}

#[test]
fn case11_fluent_parity_across_all_locales() {
    for loc in LOCALES {
        let path = repo_root().join("locales").join(loc).join("copythat.ftl");
        let body = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        for key in PHASE_31_KEYS {
            let needle = format!("\n{key} = ");
            assert!(
                body.contains(&needle),
                "Phase 31 key `{key}` missing from locale {loc}"
            );
        }
    }
}

fn repo_root() -> std::path::PathBuf {
    // Smoke runs from `crates/copythat-power`. Walk up until we find
    // the `locales/en/…` directory.
    let manifest = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut cur: &std::path::Path = manifest.as_path();
    for _ in 0..5 {
        if cur.join("locales").join("en").join("copythat.ftl").exists() {
            return cur.to_path_buf();
        }
        cur = cur.parent().unwrap_or(cur);
    }
    panic!("could not locate repo root from {manifest:?}");
}
