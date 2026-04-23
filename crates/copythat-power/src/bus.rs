//! [`PowerBus`] — the broadcast channel + background poller.
//!
//! One bus owns one tokio broadcast sender; subscribers (the runner's
//! action task, debug tools, the smoke test) each hold a `Receiver`.
//! The bus also owns the tokio interval task that reads [`ProbeSet`]
//! each tick and emits events on transitions. Tests skip the poller
//! entirely and drive the bus with [`PowerBus::inject`].

use std::sync::Arc;
use std::time::Duration;

use thiserror::Error;
use tokio::sync::broadcast;
use tokio::task::JoinHandle;

use crate::event::{NetworkClass, PowerEvent, ThermalKind};
use crate::source::{BatterySnapshot, ProbeSet};

/// Default tokio-interval period between probe reads. 5 s matches
/// the brief's "poll every 5 s" spec for Windows presentation mode.
pub const DEFAULT_POLL_PERIOD: Duration = Duration::from_secs(5);

/// Broadcast-channel capacity. Subscribers that fall behind by more
/// than this many messages see `RecvError::Lagged` and can choose to
/// reconcile; the power-driven runner subscriber doesn't care about
/// missed intermediate events because the poller only fires on state
/// transitions and the runner re-queries [`crate::compute_action`]
/// on the new state.
pub const BROADCAST_CAPACITY: usize = 64;

/// Broadcast hub for [`PowerEvent`]s.
#[derive(Clone)]
pub struct PowerBus {
    tx: broadcast::Sender<PowerEvent>,
}

pub type PowerSubscriber = broadcast::Receiver<PowerEvent>;

#[derive(Debug, Error)]
pub enum PowerBusError {
    #[error("broadcast send failed — every subscriber has dropped")]
    AllReceiversGone,
}

impl PowerBus {
    /// Create an idle bus with no poller attached. Call
    /// [`Self::spawn_poller`] to begin observing the host.
    pub fn new() -> Self {
        let (tx, _rx) = broadcast::channel(BROADCAST_CAPACITY);
        Self { tx }
    }

    /// New subscriber. Receives every event emitted after this call;
    /// dropped subscribers don't block the bus.
    pub fn subscribe(&self) -> PowerSubscriber {
        self.tx.subscribe()
    }

    /// Fire an event on the bus. Never fails in production (the
    /// internal `_rx` keeps at least one receiver alive until the bus
    /// itself is dropped); `send` returning zero subscribers is fine.
    pub fn inject(&self, event: PowerEvent) {
        let _ = self.tx.send(event);
    }

    /// Spawn the tokio interval task that reads `probes` every
    /// `period` and emits [`PowerEvent`]s on detected transitions.
    /// Returns the `JoinHandle` so the caller can abort the task on
    /// shutdown. Dropping the handle lets the task run forever —
    /// typical for a long-lived runner.
    pub fn spawn_poller(&self, probes: ProbeSet, period: Duration) -> JoinHandle<()> {
        let tx = self.tx.clone();
        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(period);
            // Burn the immediate-fire tick; we want the first "real"
            // poll to happen one period from now so initial UI state
            // stays consistent with "we just started, no adverse
            // signal yet".
            ticker.tick().await;

            let mut last = Snapshot::default();
            // Bootstrap with a first read so the initial emitted
            // events reflect the host's actual state.
            let first = sample(&probes);
            emit_delta(&tx, &last, &first);
            last = first;

            loop {
                ticker.tick().await;
                let now = sample(&probes);
                emit_delta(&tx, &last, &now);
                last = now;
            }
        })
    }

    /// One-shot snapshot read of the host state, without starting a
    /// poller. Useful for UIs that want an initial-read to render
    /// the power badge before the first poll tick arrives.
    pub fn sample_once(&self, probes: &ProbeSet) {
        let first = sample(probes);
        // Treat "nothing known" as the prior snapshot; the delta
        // emission will fire BatteryStateChanged etc. for anything
        // observed.
        emit_delta(&self.tx, &Snapshot::default(), &first);
    }
}

impl Default for PowerBus {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------
// Internal delta tracking
// ---------------------------------------------------------------------

#[derive(Clone, Copy, Default, PartialEq)]
struct Snapshot {
    battery: Option<BatterySnapshot>,
    presenting: bool,
    fullscreen: bool,
    thermal: (bool, ThermalKind),
    network: NetworkClass,
}

fn sample(probes: &ProbeSet) -> Snapshot {
    Snapshot {
        battery: probes.battery.snapshot(),
        presenting: probes.presentation.is_presenting(),
        fullscreen: probes.fullscreen.is_fullscreen(),
        thermal: probes.thermal.is_throttling(),
        network: probes.network.class(),
    }
}

/// Smallest-percent delta that still emits a new BatteryStateChanged.
/// `0.5 %` matches what the Windows power indicator renders at
/// (half-ticks on the fine-grained display); finer resolutions spam
/// subscribers without helping the UI.
const BATTERY_PERCENT_EPSILON: f32 = 0.5;

fn emit_delta(tx: &broadcast::Sender<PowerEvent>, prev: &Snapshot, now: &Snapshot) {
    if battery_changed(prev.battery, now.battery) {
        if let Some(snap) = now.battery {
            let _ = tx.send(PowerEvent::BatteryStateChanged {
                on_battery: snap.on_battery,
                percent: snap.percent,
            });
        }
    }
    if prev.network != now.network {
        let _ = tx.send(PowerEvent::NetworkClassChanged {
            class: now.network,
        });
    }
    if prev.presenting != now.presenting {
        let _ = tx.send(PowerEvent::PresentationStateChanged {
            presenting: now.presenting,
        });
    }
    if prev.fullscreen != now.fullscreen {
        let _ = tx.send(PowerEvent::FullscreenChanged {
            fullscreen: now.fullscreen,
        });
    }
    if prev.thermal != now.thermal {
        let (throttling, kind) = now.thermal;
        let _ = tx.send(PowerEvent::ThermalChanged { throttling, kind });
    }
}

fn battery_changed(prev: Option<BatterySnapshot>, now: Option<BatterySnapshot>) -> bool {
    match (prev, now) {
        (None, None) => false,
        (None, Some(_)) | (Some(_), None) => true,
        (Some(a), Some(b)) => {
            a.on_battery != b.on_battery || (a.percent - b.percent).abs() >= BATTERY_PERCENT_EPSILON
        }
    }
}

/// Type-erasing alias so the `ProbeSet`'s internals stay opaque.
/// `Arc<dyn Probe>` is already shared; this just hides the `Send +
/// Sync + 'static` bound from callers.
pub type SharedBus = Arc<PowerBus>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::NetworkClass;
    use crate::source::SyntheticProbes;

    #[tokio::test]
    async fn inject_then_receive() {
        let bus = PowerBus::new();
        let mut rx = bus.subscribe();
        bus.inject(PowerEvent::PresentationStateChanged { presenting: true });
        let ev = rx.recv().await.expect("recv");
        assert!(matches!(
            ev,
            PowerEvent::PresentationStateChanged { presenting: true }
        ));
    }

    #[tokio::test]
    async fn poller_emits_on_synthetic_change() {
        let bus = PowerBus::new();
        let mut rx = bus.subscribe();
        let probes = SyntheticProbes::new();
        let handle = bus.spawn_poller(probes.as_set(), Duration::from_millis(20));
        // Give the poller the bootstrap tick, then flip presenting.
        tokio::time::sleep(Duration::from_millis(40)).await;
        probes.set_presenting(true);
        // Next tick emits; allow up to ~60 ms for it.
        let ev = tokio::time::timeout(Duration::from_millis(200), rx.recv())
            .await
            .expect("poller tick")
            .expect("recv");
        assert!(matches!(
            ev,
            PowerEvent::PresentationStateChanged { presenting: true }
        ));
        handle.abort();
    }

    #[tokio::test]
    async fn network_class_transition_emits() {
        let bus = PowerBus::new();
        let mut rx = bus.subscribe();
        let probes = SyntheticProbes::new();
        let handle = bus.spawn_poller(probes.as_set(), Duration::from_millis(15));
        tokio::time::sleep(Duration::from_millis(30)).await;
        probes.set_network(NetworkClass::Metered);
        let ev = tokio::time::timeout(Duration::from_millis(200), rx.recv())
            .await
            .expect("poller tick")
            .expect("recv");
        assert!(matches!(
            ev,
            PowerEvent::NetworkClassChanged {
                class: NetworkClass::Metered
            }
        ));
        handle.abort();
    }
}
