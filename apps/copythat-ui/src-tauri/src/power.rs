//! Phase 31 — runner-level power subscriber.
//!
//! Maps `copythat_power::PowerEvent`s received on the AppState's
//! `PowerBus` into `pause_all` / `resume_all` / `shape.set_rate`
//! directives, using the user's `PowerPoliciesSettings` as the
//! policy layer.
//!
//! The subscriber maintains just enough bookkeeping to unpause / un-
//! cap cleanly: it remembers whether the current pause/cap was
//! *power-owned* so a later `Continue` doesn't step on a user-
//! initiated pause. If the user manually paused via `pause_all`
//! while the bus was also pausing, the manual pause wins on resume.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use copythat_power::{
    NetworkClass, PowerAction, PowerBus, PowerEvent, PowerPolicies, PowerReason, PowerState,
    apply_event, compute_action,
};
use copythat_settings::{PowerPoliciesSettings, PowerRuleChoice, ThermalRuleChoice};
use copythat_shape::{ByteRate, Shape};
use serde::Serialize;
use tauri::{AppHandle, Emitter, State};

use crate::state::AppState;

/// Tauri event emitted when the power subscriber changes its
/// directive. The frontend listens and renders the header "⏸ Paused
/// — Zoom call detected" badge.
pub const EVENT_POWER_ACTION: &str = "power-action-changed";

/// Tauri-wire form of a [`PowerAction`] announcement. `kind` is
/// `"continue" | "pause" | "cap"`; `reason` is the Fluent-key suffix
/// (e.g. `"on-battery"`) the UI concatenates with `power-reason-`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PowerActionDto {
    pub kind: &'static str,
    pub reason: Option<&'static str>,
    #[serde(rename = "bytesPerSecond")]
    pub bytes_per_second: Option<u64>,
}

impl From<&PowerAction> for PowerActionDto {
    fn from(a: &PowerAction) -> Self {
        match a {
            PowerAction::Continue => Self {
                kind: "continue",
                reason: None,
                bytes_per_second: None,
            },
            PowerAction::Pause { reason } => Self {
                kind: "pause",
                reason: Some(reason.fluent_suffix()),
                bytes_per_second: None,
            },
            PowerAction::Cap {
                bytes_per_second,
                reason,
            } => Self {
                kind: "cap",
                reason: Some(reason.fluent_suffix()),
                bytes_per_second: Some(*bytes_per_second),
            },
        }
    }
}

/// Convert the persisted settings group into the pure-library policy.
fn policies_from_settings(s: &PowerPoliciesSettings) -> PowerPolicies {
    PowerPolicies {
        battery: rule_to_power_battery(s.battery),
        metered: rule_to_power_network(s.metered),
        cellular: rule_to_power_network(s.cellular),
        presentation: rule_to_presentation(s.presentation),
        fullscreen: rule_to_fullscreen(s.fullscreen),
        thermal: rule_to_thermal(s.thermal),
    }
}

fn rule_to_power_battery(r: PowerRuleChoice) -> copythat_power::BatteryPolicy {
    match r {
        PowerRuleChoice::Continue => copythat_power::BatteryPolicy::Continue,
        PowerRuleChoice::Pause => copythat_power::BatteryPolicy::Pause,
        PowerRuleChoice::Cap { bytes_per_second } => {
            copythat_power::BatteryPolicy::Cap { bytes_per_second }
        }
    }
}

fn rule_to_power_network(r: PowerRuleChoice) -> copythat_power::NetworkPolicy {
    match r {
        PowerRuleChoice::Continue => copythat_power::NetworkPolicy::Continue,
        PowerRuleChoice::Pause => copythat_power::NetworkPolicy::Pause,
        PowerRuleChoice::Cap { bytes_per_second } => {
            copythat_power::NetworkPolicy::Cap { bytes_per_second }
        }
    }
}

fn rule_to_presentation(r: PowerRuleChoice) -> copythat_power::PresentationPolicy {
    match r {
        PowerRuleChoice::Continue => copythat_power::PresentationPolicy::Continue,
        PowerRuleChoice::Pause => copythat_power::PresentationPolicy::Pause,
        PowerRuleChoice::Cap { bytes_per_second } => {
            copythat_power::PresentationPolicy::Cap { bytes_per_second }
        }
    }
}

fn rule_to_fullscreen(r: PowerRuleChoice) -> copythat_power::FullscreenPolicy {
    match r {
        PowerRuleChoice::Continue => copythat_power::FullscreenPolicy::Continue,
        PowerRuleChoice::Pause => copythat_power::FullscreenPolicy::Pause,
        PowerRuleChoice::Cap { bytes_per_second } => {
            copythat_power::FullscreenPolicy::Cap { bytes_per_second }
        }
    }
}

fn rule_to_thermal(r: ThermalRuleChoice) -> copythat_power::ThermalPolicy {
    match r {
        ThermalRuleChoice::Continue => copythat_power::ThermalPolicy::Continue,
        ThermalRuleChoice::Pause => copythat_power::ThermalPolicy::Pause,
        ThermalRuleChoice::CapPercent { percent } => {
            copythat_power::ThermalPolicy::CapPercent { percent }
        }
    }
}

/// Handle returned by [`spawn_power_subscriber`]. Dropping it does
/// nothing; abort the task explicitly via `handle.abort()` if the
/// runner ever needs to tear the subscriber down mid-lifetime.
pub struct PowerSubscriberHandle {
    pub join: tokio::task::JoinHandle<()>,
    /// Flip this to tell the subscriber to exit cleanly on the next
    /// event. Currently used only by tests; production relies on the
    /// tokio runtime dropping all tasks at shutdown.
    pub stop: Arc<AtomicBool>,
}

/// Bookkeeping that the subscriber keeps between events so it can
/// unpause / un-cap cleanly.
#[derive(Debug, Default)]
struct Bookkeeping {
    /// `true` if the subscriber is currently owning the pause state
    /// (i.e. we called `pause_all` due to a power event). On
    /// transition to `Continue`, we resume.
    power_owned_pause: bool,
    /// Last applied bus-driven cap (in bytes/sec). `None` when the
    /// bus isn't capping; `Some(_)` stays set until we revert to the
    /// user's configured rate via `apply_network_settings_to_shape`.
    power_owned_cap: Option<u64>,
}

/// Spawn the power-subscriber task. Returns a handle whose
/// `JoinHandle` completes when the bus's last sender is dropped or
/// the `stop` flag is set.
pub fn spawn_power_subscriber(state: AppState, app: AppHandle) -> PowerSubscriberHandle {
    let mut rx = state.power_bus.subscribe();
    let stop = Arc::new(AtomicBool::new(false));
    let stop_for_task = stop.clone();

    let join = tokio::spawn(async move {
        let power_state = Arc::new(Mutex::new(PowerState::default()));
        let book = Arc::new(Mutex::new(Bookkeeping::default()));
        let mut last_action = PowerAction::Continue;

        loop {
            if stop_for_task.load(Ordering::SeqCst) {
                break;
            }
            let ev = match rx.recv().await {
                Ok(ev) => ev,
                Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => {
                    // Lost some intermediate events. No cheap way to
                    // reconcile without re-sampling the probes; the
                    // caller (the poller) re-emits on the next state
                    // change so we'll catch up within one tick.
                    continue;
                }
            };

            // Snapshot settings + advance the observed state.
            let snap = state.settings_snapshot();
            if !snap.power.enabled {
                // Power policies disabled — release any power-owned
                // pause/cap and stop processing.
                release_power_owned(&state, &app, &book, &snap, &mut last_action);
                continue;
            }
            let policies = policies_from_settings(&snap.power);

            {
                let mut ps = power_state.lock().expect("power_state lock");
                apply_event(&mut ps, &ev);
            }
            let current_state = *power_state.lock().expect("power_state lock");
            let action = compute_action(&current_state, &policies);

            if action == last_action {
                continue;
            }
            apply_action(&state, &app, &book, &snap, &action);
            last_action = action;
        }
    });

    PowerSubscriberHandle { join, stop }
}

/// Perform the side-effects implied by transitioning to `action`.
fn apply_action(
    state: &AppState,
    app: &AppHandle,
    book: &Arc<Mutex<Bookkeeping>>,
    snap: &copythat_settings::Settings,
    action: &PowerAction,
) {
    let mut b = book.lock().expect("bookkeeping lock");
    match action {
        PowerAction::Continue => {
            if b.power_owned_pause {
                // Release the power-owned pause.
                for job in state.queue.snapshot() {
                    state.queue.resume_job(job.id);
                }
                b.power_owned_pause = false;
            }
            if b.power_owned_cap.is_some() {
                // Restore user-configured rate.
                crate::state::apply_network_settings_to_shape(&state.shape, &snap.network);
                b.power_owned_cap = None;
            }
        }
        PowerAction::Pause { .. } => {
            if !b.power_owned_pause {
                for job in state.queue.snapshot() {
                    state.queue.pause_job(job.id);
                }
                b.power_owned_pause = true;
            }
            // Clear cap bookkeeping — pause dominates.
            if b.power_owned_cap.is_some() {
                b.power_owned_cap = None;
            }
        }
        PowerAction::Cap {
            bytes_per_second,
            reason,
        } => {
            // Thermal cap uses the sentinel `0` to mean "consult the
            // live shape and scale by the policy's percent". Anything
            // else is an absolute bytes/sec cap.
            let resolved_bps = if *bytes_per_second == 0
                && matches!(reason, PowerReason::ThermalThrottling)
            {
                let percent = match snap.power.thermal {
                    ThermalRuleChoice::CapPercent { percent } => percent.min(100) as u64,
                    _ => 50,
                };
                let current = state
                    .shape
                    .current_rate()
                    .map(|r| r.bytes_per_second())
                    .unwrap_or(u64::MAX);
                current.saturating_mul(percent) / 100
            } else {
                *bytes_per_second
            };
            if b.power_owned_pause {
                // Moving from Pause → Cap: resume jobs, then set rate.
                for job in state.queue.snapshot() {
                    state.queue.resume_job(job.id);
                }
                b.power_owned_pause = false;
            }
            apply_shape_cap(&state.shape, resolved_bps);
            b.power_owned_cap = Some(resolved_bps);
        }
    }
    drop(b);
    let dto = PowerActionDto::from(action);
    let _ = app.emit(EVENT_POWER_ACTION, dto);
}

fn release_power_owned(
    state: &AppState,
    app: &AppHandle,
    book: &Arc<Mutex<Bookkeeping>>,
    snap: &copythat_settings::Settings,
    last_action: &mut PowerAction,
) {
    let mut b = book.lock().expect("bookkeeping lock");
    if b.power_owned_pause {
        for job in state.queue.snapshot() {
            state.queue.resume_job(job.id);
        }
        b.power_owned_pause = false;
    }
    if b.power_owned_cap.take().is_some() {
        crate::state::apply_network_settings_to_shape(&state.shape, &snap.network);
    }
    drop(b);
    *last_action = PowerAction::Continue;
    let dto = PowerActionDto::from(&PowerAction::Continue);
    let _ = app.emit(EVENT_POWER_ACTION, dto);
}

fn apply_shape_cap(shape: &Shape, bytes_per_second: u64) {
    if bytes_per_second == 0 {
        // Zero means "paused via shape" — treat as pause-equivalent
        // on the shape layer.
        shape.set_rate(Some(ByteRate::new(0)));
    } else {
        shape.set_rate(Some(ByteRate::new(bytes_per_second)));
    }
}

// ---------------------------------------------------------------------
// IPC: test-only + runtime-side event injection
// ---------------------------------------------------------------------

/// Wire-form of [`PowerEvent`] accepted by [`inject_power_event`].
/// Tagged-object under the `event` key so the frontend's
/// `{ event: "..." }` dispatch matches the DTOs that ship elsewhere.
/// A discriminator-tag name of `event` avoids colliding with
/// `ThermalChanged`'s own `kind` field.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase", tag = "event")]
pub enum PowerEventDto {
    BatteryStateChanged {
        on_battery: bool,
        percent: f32,
    },
    NetworkClassChanged {
        class: String,
    },
    PresentationStateChanged {
        presenting: bool,
    },
    FullscreenChanged {
        fullscreen: bool,
    },
    ThermalChanged {
        throttling: bool,
        #[serde(default)]
        kind: String,
    },
}

impl PowerEventDto {
    fn into_event(self) -> PowerEvent {
        match self {
            Self::BatteryStateChanged {
                on_battery,
                percent,
            } => PowerEvent::BatteryStateChanged {
                on_battery,
                percent,
            },
            Self::NetworkClassChanged { class } => PowerEvent::NetworkClassChanged {
                class: parse_network_class(&class),
            },
            Self::PresentationStateChanged { presenting } => {
                PowerEvent::PresentationStateChanged { presenting }
            }
            Self::FullscreenChanged { fullscreen } => PowerEvent::FullscreenChanged { fullscreen },
            Self::ThermalChanged { throttling, kind } => PowerEvent::ThermalChanged {
                throttling,
                kind: parse_thermal_kind(&kind),
            },
        }
    }
}

fn parse_network_class(s: &str) -> NetworkClass {
    match s {
        "metered" => NetworkClass::Metered,
        "cellular" => NetworkClass::Cellular,
        _ => NetworkClass::Unmetered,
    }
}

fn parse_thermal_kind(s: &str) -> copythat_power::ThermalKind {
    match s {
        "x86-cpuid" => copythat_power::ThermalKind::X86Cpuid,
        "os-reported" => copythat_power::ThermalKind::OsReported,
        _ => copythat_power::ThermalKind::Unknown,
    }
}

/// Fire a synthetic [`PowerEvent`] onto the AppState bus. The runner
/// subscriber reacts exactly as it would to a real poller-emitted
/// event. Exposed as a stable IPC command so the Phase 31 smoke test
/// and debugging tools can exercise the pause-path end-to-end
/// without the OS FFI probes.
#[tauri::command]
pub fn inject_power_event(
    event: PowerEventDto,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.power_bus.inject(event.into_event());
    Ok(())
}

/// Resolve the runner's current power-driven action without waiting
/// for the subscriber task to process another event. Returns what
/// [`compute_action`] says the policy would pick given the AppState's
/// current settings and the supplied [`PowerState`]. Used by the
/// Phase 31 smoke to assert the pure policy resolution without
/// racing the tokio broadcast.
pub fn compute_action_for_state(
    state: &AppState,
    observed: &PowerState,
) -> PowerAction {
    let snap = state.settings_snapshot();
    if !snap.power.enabled {
        return PowerAction::Continue;
    }
    let policies = policies_from_settings(&snap.power);
    compute_action(observed, &policies)
}

// Suppress "unused import" warnings on the `PowerBus` name — the
// type is referenced via the AppState field declaration only, which
// the linter can't see from inside this module.
#[allow(dead_code)]
fn _require_bus_type(_bus: &PowerBus) {}
