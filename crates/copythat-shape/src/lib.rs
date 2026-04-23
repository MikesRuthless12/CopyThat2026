//! `copythat-shape` — bandwidth shaping with GCRA token bucket,
//! time-of-day schedule, and OS-aware auto-throttle.
//!
//! The engine calls [`Shape::permit`] after every buffered read; the
//! GCRA token bucket (via the `governor` crate) blocks the async
//! loop until enough virtual cells are available to admit the
//! requested byte count, producing a smooth cap that tracks the
//! configured rate within a handful of milliseconds.
//!
//! # Architecture
//!
//! Three layers:
//!
//! 1. [`Shape`] — the hot-path object. A single instance is attached
//!    to every [`copythat_core::CopyOptions`] via the [`ShapeSink`]
//!    bridge in [`CopyThatShapeSink`]. Its [`Shape::set_rate`] method
//!    is safe to call from any task: the internal `ArcSwap` swaps
//!    the live limiter without blocking concurrent `permit` calls.
//! 2. [`Schedule`] — parsed, rclone-compatible calendar of rate
//!    boundaries. [`Schedule::current_limit`] asks "what's the cap
//!    right now?" given a wall-clock; a tokio interval task in the
//!    Tauri runner polls this every minute and feeds the answer
//!    into [`Shape::set_rate`].
//! 3. [`auto`] — OS-level probes for network class (metered /
//!    cellular / unmetered) and power state (plugged-in / on-battery).
//!    Stubbed to `Unmetered` / `PluggedIn` in Phase 21; Windows
//!    `INetworkCostManager`, macOS `NWPathMonitor`, and Linux
//!    NetworkManager DBus land in Phase 31-adjacent work.
//!
//! # Engine wiring
//!
//! [`CopyThatShapeSink`] wraps an `Arc<Shape>` and implements the
//! `copythat_core::ShapeSink` trait-object boundary. The trait object
//! sits on `CopyOptions::shape`; with `shape = None`, the engine's
//! existing code path runs untouched.
//!
//! [`ShapeSink`]: copythat_core::ShapeSink

#![forbid(unsafe_code)]

pub mod auto;
mod error;
mod schedule;
mod shape;
mod sink;

pub use auto::{NetworkClass, PowerState, current_network_class, current_power_state};
pub use error::{ScheduleError, ShapeError};
pub use schedule::{DayMask, Schedule, ScheduleRule, TimeWindow};
pub use shape::{ByteRate, Shape};
pub use sink::CopyThatShapeSink;
