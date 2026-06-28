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

use std::collections::HashSet;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use copythat_core::{Job, JobId};
use copythat_power::{
    NetworkClass, PowerAction, PowerBus, PowerEvent, PowerPolicies, PowerReason, PowerState,
    ScopedActions, apply_event, compute_action, compute_scoped_actions,
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
    /// The set of jobs *we* (the power policy) paused. We resume
    /// exactly these on a later transition, so we never step on a job
    /// the user paused manually. Global rules (battery / presentation /
    /// fullscreen / thermal) add every job; the metered/cellular rule
    /// adds only network-bound jobs — a local disk-to-disk copy never
    /// crosses the metered link, so it keeps running.
    power_paused_jobs: HashSet<JobId>,
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

    // Enter Tauri's tokio runtime context so the bare `tokio::spawn`
    // below resolves to a `tokio::task::JoinHandle` (matches
    // `PowerSubscriberHandle.join`'s type) AND has a reactor to
    // attach to. From the setup-hook path the call site is OUTSIDE
    // any tokio context, so a bare `tokio::spawn` here would panic
    // with "no reactor running". See shell.rs:89.
    //
    // `clippy::async_yields_async`: yielding the `JoinHandle` is the
    // entire point — we store it in `PowerSubscriberHandle.join` so
    // the caller can `.abort()` or `.await` on shutdown. Awaiting it
    // here would block `block_on` until the subscriber loop exits,
    // which never happens during normal app run.
    #[allow(clippy::async_yields_async)]
    let join = tauri::async_runtime::block_on(async {
        tokio::spawn(async move {
            let power_state = Arc::new(Mutex::new(PowerState::default()));
            let book = Arc::new(Mutex::new(Bookkeeping::default()));
            let mut last_scoped = ScopedActions {
                global: PowerAction::Continue,
                network: PowerAction::Continue,
            };

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
                    release_power_owned(&state, &app, &book, &snap, &mut last_scoped);
                    continue;
                }
                let policies = policies_from_settings(&snap.power);

                {
                    let mut ps = power_state.lock().expect("power_state lock");
                    apply_event(&mut ps, &ev);
                }
                let current_state = *power_state.lock().expect("power_state lock");
                let scoped = compute_scoped_actions(&current_state, &policies);

                if scoped == last_scoped {
                    continue;
                }
                apply_scoped(&state, &app, &book, &snap, &scoped);
                last_scoped = scoped;
            }
        })
    });

    PowerSubscriberHandle { join, stop }
}

/// Apply the scoped power actions: `global` (battery / presentation /
/// fullscreen / thermal) to every job, `network` (metered / cellular)
/// to network-bound jobs only. Reconciles the set of jobs *we* paused
/// so a transition resumes exactly those — never a job the user paused
/// manually.
fn apply_scoped(
    state: &AppState,
    app: &AppHandle,
    book: &Arc<Mutex<Bookkeeping>>,
    snap: &copythat_settings::Settings,
    scoped: &ScopedActions,
) {
    let mut b = book.lock().expect("bookkeeping lock");

    // The shape (rate cap) is a single global limiter, so a network
    // *Cap* can't be applied per-job — fold a (rare, hand-edited)
    // network cap into the global action for the shape. A network
    // *Pause* stays scoped to network-bound jobs in the loop below.
    let global = match scoped.network {
        PowerAction::Cap { .. } => scoped.global.stricter(scoped.network),
        _ => scoped.global,
    };
    let global_pauses = global.is_pause();
    let network_pauses = scoped.network.is_pause();

    // Per-job pause reconciliation: a job is power-paused when a global
    // rule pauses, OR a network rule pauses and the job is network-bound.
    let mut desired: HashSet<JobId> = HashSet::new();
    for job in state.queue.snapshot() {
        if global_pauses || (network_pauses && is_network_bound(&job)) {
            desired.insert(job.id);
        }
    }
    for &id in &desired {
        // `insert` returns true the first time we take this job's pause.
        if b.power_paused_jobs.insert(id) {
            state.queue.pause_job(id);
        }
    }
    let stale: Vec<JobId> = b
        .power_paused_jobs
        .iter()
        .copied()
        .filter(|id| !desired.contains(id))
        .collect();
    for id in stale {
        state.queue.resume_job(id);
        b.power_paused_jobs.remove(&id);
    }

    // Global shape cap (driven by the global/folded action only).
    match global {
        PowerAction::Cap {
            bytes_per_second,
            reason,
        } => {
            let resolved = resolve_cap_bps(state, snap, bytes_per_second, reason);
            apply_shape_cap(&state.shape, resolved);
            b.power_owned_cap = Some(resolved);
        }
        PowerAction::Continue | PowerAction::Pause { .. } => {
            if b.power_owned_cap.take().is_some() {
                crate::state::apply_network_settings_to_shape(&state.shape, &snap.network);
            }
        }
    }
    drop(b);

    // UI badge: the single most-restrictive action across both scopes.
    let badge = scoped.global.stricter(scoped.network);
    let dto = PowerActionDto::from(&badge);
    let _ = app.emit(EVENT_POWER_ACTION, dto);
}

/// Expand a cap's bytes/sec, resolving the thermal `0` sentinel
/// ("percent of the live shape rate") against the current shape state.
fn resolve_cap_bps(
    state: &AppState,
    snap: &copythat_settings::Settings,
    bytes_per_second: u64,
    reason: PowerReason,
) -> u64 {
    if bytes_per_second == 0 && matches!(reason, PowerReason::ThermalThrottling) {
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
        bytes_per_second
    }
}

/// A queue job is "network-bound" when its source or destination is a
/// network share (UNC path). The metered/cellular power rule pauses
/// these; a local disk-to-disk copy is left running since it never
/// crosses the metered link. Cloud transfers (S3 / SFTP / WebDAV) run
/// on a separate path — see `cloud_commands` — and are handled there.
fn is_network_bound(job: &Job) -> bool {
    is_network_path(&job.src) || job.dst.as_deref().is_some_and(is_network_path)
}

/// `true` when `p` is a UNC network-share path (`\\server\share` or
/// the verbatim `\\?\UNC\…` form). Local drive paths (`C:\…`,
/// `\\?\C:\…`) and relative paths are not network shares. On non-
/// Windows hosts no path carries a UNC prefix, so this is always
/// `false` there (network mounts live at arbitrary paths the runner
/// can't classify; cloud transfers are handled separately).
fn is_network_path(p: &Path) -> bool {
    use std::path::{Component, Prefix};
    matches!(
        p.components().next(),
        Some(Component::Prefix(pre))
            if matches!(pre.kind(), Prefix::UNC(..) | Prefix::VerbatimUNC(..))
    )
}

fn release_power_owned(
    state: &AppState,
    app: &AppHandle,
    book: &Arc<Mutex<Bookkeeping>>,
    snap: &copythat_settings::Settings,
    last_scoped: &mut ScopedActions,
) {
    let mut b = book.lock().expect("bookkeeping lock");
    let paused: Vec<JobId> = b.power_paused_jobs.drain().collect();
    for id in paused {
        state.queue.resume_job(id);
    }
    if b.power_owned_cap.take().is_some() {
        crate::state::apply_network_settings_to_shape(&state.shape, &snap.network);
    }
    drop(b);
    *last_scoped = ScopedActions {
        global: PowerAction::Continue,
        network: PowerAction::Continue,
    };
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
pub fn inject_power_event(event: PowerEventDto, state: State<'_, AppState>) -> Result<(), String> {
    state.power_bus.inject(event.into_event());
    Ok(())
}

/// Resolve the runner's current power-driven action without waiting
/// for the subscriber task to process another event. Returns what
/// [`compute_action`] says the policy would pick given the AppState's
/// current settings and the supplied [`PowerState`]. Used by the
/// Phase 31 smoke to assert the pure policy resolution without
/// racing the tokio broadcast.
pub fn compute_action_for_state(state: &AppState, observed: &PowerState) -> PowerAction {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(windows)]
    #[test]
    fn unc_shares_are_network_bound() {
        // A UNC share — and its verbatim form — is a network path, so
        // the metered/cellular rule pauses copies to/from it.
        assert!(is_network_path(Path::new(r"\\server\share\file.bin")));
        assert!(is_network_path(Path::new(r"\\?\UNC\server\share\file.bin")));
    }

    #[test]
    fn local_and_relative_paths_are_not_network_bound() {
        // Local drives + relative paths are never a network share, so a
        // local disk-to-disk copy keeps running on a metered connection.
        // (On non-Windows these parse as non-prefixed paths — also
        // correctly `false`.)
        assert!(!is_network_path(Path::new("relative/file.bin")));
        assert!(!is_network_path(Path::new(r"C:\Users\me\file.bin")));
        assert!(!is_network_path(Path::new(r"\\?\C:\Users\me\file.bin")));
    }
}
