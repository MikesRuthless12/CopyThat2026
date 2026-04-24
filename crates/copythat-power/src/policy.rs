//! [`PowerPolicies`] + [`apply_event`] — the pure state-machine
//! that decides what the engine should do in response to each
//! power transition.
//!
//! The model: maintain a small [`PowerState`] tracking the last-known
//! value on each dimension; on every [`PowerEvent`], update the
//! relevant field and recompute a [`PowerAction`] from the policies.
//! Actions combine via a strictness order — `Pause` dominates `Cap`
//! dominates `Continue` — so the runner gets a single directive even
//! when multiple dimensions would act.
//!
//! This module is **pure** — no tokio, no I/O, no OS calls. The
//! smoke test drives it directly; the tokio poller feeds it observed
//! events. Trivially fuzzable.

use serde::{Deserialize, Serialize};

use crate::event::{NetworkClass, PowerEvent, ThermalKind};

// ---------------------------------------------------------------------
// Policy rule shapes
// ---------------------------------------------------------------------

/// What to do when the system is running on battery.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BatteryPolicy {
    /// Continue copying at full speed. Default to preserve the
    /// pre-Phase-31 behaviour for users who don't touch Settings.
    #[default]
    Continue,
    /// Pause all active jobs until the host returns to AC power.
    Pause,
    /// Cap the global shape rate to `bytes_per_second` while on
    /// battery. Restores the previous rate on AC return.
    Cap { bytes_per_second: u64 },
}

/// What to do when the active connection is metered or cellular.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NetworkPolicy {
    #[default]
    Continue,
    Pause,
    Cap {
        bytes_per_second: u64,
    },
}

/// What to do when the host is in a known presentation mode (Zoom /
/// Teams / Keynote / PowerPoint / Meet active).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PresentationPolicy {
    Continue,
    /// Pause. Default per the brief — a copy churning disk or network
    /// during a Zoom call is exactly what the user is trying to avoid.
    #[default]
    Pause,
    Cap {
        bytes_per_second: u64,
    },
}

/// What to do when any app is fullscreen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FullscreenPolicy {
    /// Default per brief: fullscreen alone is ambiguous (games,
    /// movies, IDEs) and shouldn't suspend a copy.
    #[default]
    Continue,
    Pause,
    Cap {
        bytes_per_second: u64,
    },
}

/// What to do when the CPU is thermal-throttling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ThermalPolicy {
    Continue,
    Pause,
    /// Cap to `percent_of_current / 100.0` of the current shape rate.
    /// `50` = half speed; `10` = tenth speed. `100` + `Continue`
    /// differ only in that `CapPercent` kicks in whenever thermal
    /// throttling is active, even at 100 %, so the runner can
    /// restore the prior rate cleanly on resolution.
    CapPercent {
        percent: u8,
    },
}

impl Default for ThermalPolicy {
    // Default per brief: "cap to 50%" when CPU is throttling.
    // A `#[default]` attribute can't carry the percent field, so this
    // hand-rolled impl stays.
    #[allow(clippy::derivable_impls)]
    fn default() -> Self {
        Self::CapPercent { percent: 50 }
    }
}

/// The full bundle of user-configurable rules. Round-trippable via
/// `serde`; persisted by `copythat-settings::PowerPoliciesSettings`.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct PowerPolicies {
    pub battery: BatteryPolicy,
    pub metered: NetworkPolicy,
    pub cellular: NetworkPolicy,
    pub presentation: PresentationPolicy,
    pub fullscreen: FullscreenPolicy,
    pub thermal: ThermalPolicy,
}

// ---------------------------------------------------------------------
// Observed state
// ---------------------------------------------------------------------

/// Latest-known value on each dimension. Starts at the "nothing
/// adverse detected" defaults; the poller updates in place on every
/// received event via [`apply_event`].
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct PowerState {
    pub on_battery: bool,
    pub battery_percent: f32,
    pub network_class: NetworkClass,
    pub presenting: bool,
    pub fullscreen: bool,
    pub throttling: bool,
    pub thermal_kind: ThermalKind,
}

/// Update `state` in place to reflect `event`. Pure, infallible,
/// idempotent — applying the same event twice leaves state unchanged.
pub fn apply_event(state: &mut PowerState, event: &PowerEvent) {
    match event {
        PowerEvent::BatteryStateChanged {
            on_battery,
            percent,
        } => {
            state.on_battery = *on_battery;
            state.battery_percent = *percent;
        }
        PowerEvent::NetworkClassChanged { class } => {
            state.network_class = *class;
        }
        PowerEvent::PresentationStateChanged { presenting } => {
            state.presenting = *presenting;
        }
        PowerEvent::FullscreenChanged { fullscreen } => {
            state.fullscreen = *fullscreen;
        }
        PowerEvent::ThermalChanged { throttling, kind } => {
            state.throttling = *throttling;
            state.thermal_kind = *kind;
        }
    }
}

// ---------------------------------------------------------------------
// Action computation
// ---------------------------------------------------------------------

/// Why the runner is pausing or capping. Carried on [`PowerAction`]
/// so the UI badge can render "⏸ Paused — Zoom call detected"
/// instead of a generic "power policy active".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PowerReason {
    OnBattery,
    MeteredNetwork,
    CellularNetwork,
    Presenting,
    Fullscreen,
    ThermalThrottling,
}

impl PowerReason {
    /// Stable Fluent-key suffix — the UI concatenates
    /// `"power-reason-"` + this value to get the localized label.
    pub const fn fluent_suffix(&self) -> &'static str {
        match self {
            Self::OnBattery => "on-battery",
            Self::MeteredNetwork => "metered-network",
            Self::CellularNetwork => "cellular-network",
            Self::Presenting => "presenting",
            Self::Fullscreen => "fullscreen",
            Self::ThermalThrottling => "thermal-throttling",
        }
    }
}

/// What the runner should do right now. `Continue` means "no power-
/// driven override — let the user's manual pause/shape state stand";
/// `Pause` and `Cap` carry the triggering [`PowerReason`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "kind")]
pub enum PowerAction {
    /// No power-driven action. Runner releases any power-owned pause
    /// / cap and returns to the user's manual state.
    Continue,
    /// Pause all active jobs. The runner tracks that *it* paused
    /// (vs the user manually pausing) so a later `Continue` can
    /// resume cleanly without stepping on a user-initiated pause.
    Pause { reason: PowerReason },
    /// Cap the global shape rate. Same runner bookkeeping as `Pause`
    /// — the runner restores the prior rate on `Continue`.
    Cap {
        bytes_per_second: u64,
        reason: PowerReason,
    },
}

impl PowerAction {
    /// `true` when this action implies an immediate pause of all
    /// active jobs (as opposed to a rate cap or pass-through).
    pub const fn is_pause(&self) -> bool {
        matches!(self, Self::Pause { .. })
    }

    /// The [`PowerReason`] that triggered this action, or `None` for
    /// [`PowerAction::Continue`]. Consumed by the UI badge to render
    /// "⏸ Paused — Zoom call detected" style text.
    pub const fn reason(&self) -> Option<PowerReason> {
        match self {
            Self::Continue => None,
            Self::Pause { reason } | Self::Cap { reason, .. } => Some(*reason),
        }
    }
}

/// Take the stricter of two actions. `Pause` dominates `Cap` (which
/// takes the lower cap when both are caps) which dominates
/// `Continue`.
fn combine(a: PowerAction, b: PowerAction) -> PowerAction {
    match (a, b) {
        (PowerAction::Pause { .. }, _) => a,
        (_, PowerAction::Pause { .. }) => b,
        (
            PowerAction::Cap {
                bytes_per_second: ba,
                reason: ra,
            },
            PowerAction::Cap {
                bytes_per_second: bb,
                reason: rb,
            },
        ) => {
            if ba <= bb {
                PowerAction::Cap {
                    bytes_per_second: ba,
                    reason: ra,
                }
            } else {
                PowerAction::Cap {
                    bytes_per_second: bb,
                    reason: rb,
                }
            }
        }
        (PowerAction::Cap { .. }, PowerAction::Continue) => a,
        (PowerAction::Continue, PowerAction::Cap { .. }) => b,
        (PowerAction::Continue, PowerAction::Continue) => PowerAction::Continue,
    }
}

/// Resolve the current power-driven action across all policy
/// dimensions, given the observed [`PowerState`].
///
/// Walks the active conditions in priority order and combines each
/// dimension's would-be action via [`combine`]. Order of evaluation
/// doesn't affect the final answer (combine is commutative-enough
/// under the strictness ordering) but it's stable so tie-breaks are
/// deterministic.
pub fn compute_action(state: &PowerState, policies: &PowerPolicies) -> PowerAction {
    let mut out = PowerAction::Continue;

    if state.on_battery {
        out = combine(
            out,
            policy_to_action(policies.battery, PowerReason::OnBattery),
        );
    }
    match state.network_class {
        NetworkClass::Metered => {
            out = combine(
                out,
                policy_to_action(policies.metered, PowerReason::MeteredNetwork),
            );
        }
        NetworkClass::Cellular => {
            out = combine(
                out,
                policy_to_action(policies.cellular, PowerReason::CellularNetwork),
            );
        }
        NetworkClass::Unmetered => {}
    }
    if state.presenting {
        out = combine(
            out,
            presentation_to_action(policies.presentation, PowerReason::Presenting),
        );
    }
    if state.fullscreen {
        out = combine(
            out,
            fullscreen_to_action(policies.fullscreen, PowerReason::Fullscreen),
        );
    }
    if state.throttling {
        // Thermal cap is specified as a percent of current; runners
        // translate `CapPercent` against their live shape state. We
        // don't know that here — surface a sentinel `0` cap and let
        // the runner interpret it (documented in Shape::set_rate
        // docs). `Pause` / `Continue` pass through cleanly.
        out = combine(out, thermal_to_action(policies.thermal));
    }

    out
}

fn policy_to_action<P: Copy + Into<PolicyAction>>(policy: P, reason: PowerReason) -> PowerAction {
    match policy.into() {
        PolicyAction::Continue => PowerAction::Continue,
        PolicyAction::Pause => PowerAction::Pause { reason },
        PolicyAction::Cap(bps) => PowerAction::Cap {
            bytes_per_second: bps,
            reason,
        },
    }
}

fn presentation_to_action(policy: PresentationPolicy, reason: PowerReason) -> PowerAction {
    match policy {
        PresentationPolicy::Continue => PowerAction::Continue,
        PresentationPolicy::Pause => PowerAction::Pause { reason },
        PresentationPolicy::Cap { bytes_per_second } => PowerAction::Cap {
            bytes_per_second,
            reason,
        },
    }
}

fn fullscreen_to_action(policy: FullscreenPolicy, reason: PowerReason) -> PowerAction {
    match policy {
        FullscreenPolicy::Continue => PowerAction::Continue,
        FullscreenPolicy::Pause => PowerAction::Pause { reason },
        FullscreenPolicy::Cap { bytes_per_second } => PowerAction::Cap {
            bytes_per_second,
            reason,
        },
    }
}

fn thermal_to_action(policy: ThermalPolicy) -> PowerAction {
    match policy {
        ThermalPolicy::Continue => PowerAction::Continue,
        ThermalPolicy::Pause => PowerAction::Pause {
            reason: PowerReason::ThermalThrottling,
        },
        ThermalPolicy::CapPercent { percent: _ } => PowerAction::Cap {
            // Sentinel — the runner interprets `0` as "consult my
            // current shape rate and scale by the policy's percent".
            // Kept out of the pure layer because percent-of-current
            // needs the runner's shape state.
            bytes_per_second: 0,
            reason: PowerReason::ThermalThrottling,
        },
    }
}

/// Internal normalization — each *uniform* policy shape
/// (BatteryPolicy / NetworkPolicy) maps here so we don't duplicate
/// the match arms.
enum PolicyAction {
    Continue,
    Pause,
    Cap(u64),
}

impl From<BatteryPolicy> for PolicyAction {
    fn from(p: BatteryPolicy) -> Self {
        match p {
            BatteryPolicy::Continue => Self::Continue,
            BatteryPolicy::Pause => Self::Pause,
            BatteryPolicy::Cap { bytes_per_second } => Self::Cap(bytes_per_second),
        }
    }
}

impl From<NetworkPolicy> for PolicyAction {
    fn from(p: NetworkPolicy) -> Self {
        match p {
            NetworkPolicy::Continue => Self::Continue,
            NetworkPolicy::Pause => Self::Pause,
            NetworkPolicy::Cap { bytes_per_second } => Self::Cap(bytes_per_second),
        }
    }
}

// ---------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn pause_on_battery() -> PowerPolicies {
        PowerPolicies {
            battery: BatteryPolicy::Pause,
            ..Default::default()
        }
    }

    #[test]
    fn apply_event_updates_battery_fields() {
        let mut s = PowerState::default();
        apply_event(
            &mut s,
            &PowerEvent::BatteryStateChanged {
                on_battery: true,
                percent: 73.5,
            },
        );
        assert!(s.on_battery);
        assert_eq!(s.battery_percent, 73.5);
    }

    #[test]
    fn apply_event_is_idempotent() {
        let mut s = PowerState::default();
        let e = PowerEvent::PresentationStateChanged { presenting: true };
        apply_event(&mut s, &e);
        let snapshot = s;
        apply_event(&mut s, &e);
        assert_eq!(s, snapshot);
    }

    #[test]
    fn battery_on_with_pause_policy_yields_pause() {
        let mut s = PowerState::default();
        apply_event(
            &mut s,
            &PowerEvent::BatteryStateChanged {
                on_battery: true,
                percent: 50.0,
            },
        );
        let action = compute_action(&s, &pause_on_battery());
        assert_eq!(
            action,
            PowerAction::Pause {
                reason: PowerReason::OnBattery
            }
        );
    }

    #[test]
    fn battery_off_yields_continue() {
        let mut s = PowerState::default();
        apply_event(
            &mut s,
            &PowerEvent::BatteryStateChanged {
                on_battery: true,
                percent: 50.0,
            },
        );
        apply_event(
            &mut s,
            &PowerEvent::BatteryStateChanged {
                on_battery: false,
                percent: 100.0,
            },
        );
        let action = compute_action(&s, &pause_on_battery());
        assert_eq!(action, PowerAction::Continue);
    }

    #[test]
    fn presentation_default_policy_is_pause() {
        let policies = PowerPolicies::default();
        assert_eq!(policies.presentation, PresentationPolicy::Pause);

        let mut s = PowerState::default();
        apply_event(
            &mut s,
            &PowerEvent::PresentationStateChanged { presenting: true },
        );
        assert_eq!(
            compute_action(&s, &policies),
            PowerAction::Pause {
                reason: PowerReason::Presenting
            }
        );
    }

    #[test]
    fn fullscreen_default_policy_is_continue() {
        let policies = PowerPolicies::default();
        assert_eq!(policies.fullscreen, FullscreenPolicy::Continue);

        let mut s = PowerState::default();
        apply_event(&mut s, &PowerEvent::FullscreenChanged { fullscreen: true });
        assert_eq!(compute_action(&s, &policies), PowerAction::Continue);
    }

    #[test]
    fn thermal_default_is_cap_percent_50() {
        let policies = PowerPolicies::default();
        match policies.thermal {
            ThermalPolicy::CapPercent { percent } => assert_eq!(percent, 50),
            other => panic!("expected CapPercent(50), got {other:?}"),
        }
    }

    #[test]
    fn pause_dominates_cap() {
        let a = PowerAction::Cap {
            bytes_per_second: 1_000_000,
            reason: PowerReason::MeteredNetwork,
        };
        let b = PowerAction::Pause {
            reason: PowerReason::OnBattery,
        };
        assert!(combine(a, b).is_pause());
        assert!(combine(b, a).is_pause());
    }

    #[test]
    fn cap_combine_picks_lower() {
        let a = PowerAction::Cap {
            bytes_per_second: 10_000_000,
            reason: PowerReason::MeteredNetwork,
        };
        let b = PowerAction::Cap {
            bytes_per_second: 2_000_000,
            reason: PowerReason::OnBattery,
        };
        let combined = combine(a, b);
        match combined {
            PowerAction::Cap {
                bytes_per_second, ..
            } => assert_eq!(bytes_per_second, 2_000_000),
            _ => panic!("expected Cap, got {combined:?}"),
        }
    }

    #[test]
    fn metered_network_triggers_configured_cap() {
        let policies = PowerPolicies {
            metered: NetworkPolicy::Cap {
                bytes_per_second: 5_000_000,
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
        assert_eq!(
            compute_action(&s, &policies),
            PowerAction::Cap {
                bytes_per_second: 5_000_000,
                reason: PowerReason::MeteredNetwork
            }
        );
    }

    #[test]
    fn reason_fluent_suffix_stable() {
        assert_eq!(PowerReason::OnBattery.fluent_suffix(), "on-battery");
        assert_eq!(PowerReason::Presenting.fluent_suffix(), "presenting");
        assert_eq!(
            PowerReason::ThermalThrottling.fluent_suffix(),
            "thermal-throttling"
        );
    }
}
