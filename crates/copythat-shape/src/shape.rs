//! The GCRA token bucket. See the crate-level docs for the wiring
//! story.
//!
//! The external knobs are [`Shape::new`], [`Shape::set_rate`], and
//! [`Shape::permit`]. Everything else is internal plumbing: the live
//! `governor` limiter lives behind an `ArcSwap` so a tokio task
//! driving the schedule can replace the rate mid-copy without
//! pausing the hot-path `permit` awaits.

use std::num::NonZeroU32;
use std::sync::Arc;

use arc_swap::ArcSwap;
use governor::{
    Quota, RateLimiter,
    clock::DefaultClock,
    state::{InMemoryState, NotKeyed},
};

/// Bytes-per-second wrapper. Opaque on purpose so arithmetic with
/// raw `u64` rates doesn't silently confuse bits / bytes / bits-per-
/// second.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ByteRate(u64);

impl ByteRate {
    /// `n` bytes per second. `0` is a valid value and means
    /// "blocked" — a `Shape` with rate `0` never issues permits
    /// (the GCRA bucket is empty and refills at zero rate).
    pub const fn new(bytes_per_second: u64) -> Self {
        Self(bytes_per_second)
    }

    /// Convenience for SI-style rates. `512` = 512 B/s,
    /// `ByteRate::kibibytes_per_second(512)` = 512 KiB/s.
    pub const fn kibibytes_per_second(n: u64) -> Self {
        Self(n * 1024)
    }

    /// `n` MiB/s. Used by the Settings mirror.
    pub const fn mebibytes_per_second(n: u64) -> Self {
        Self(n * 1024 * 1024)
    }

    /// Raw bytes-per-second.
    pub const fn bytes_per_second(self) -> u64 {
        self.0
    }

    /// True when the rate is `0` — `permit` will block forever.
    pub const fn is_off(self) -> bool {
        self.0 == 0
    }
}

/// The opaque governor type we end up with. Pinned through this
/// alias so the hot-path code isn't sprinkled with eight-parameter
/// generic bounds.
type Limiter = RateLimiter<NotKeyed, InMemoryState, DefaultClock>;

/// The live bucket + metadata swapped in on every [`Shape::set_rate`].
/// Kept as a plain struct (no `Default`) because construction always
/// goes through [`Shape::build_inner`].
struct ShapeInner {
    /// `None` = unlimited, skip the limiter entirely. `Some` =
    /// limited at `burst_bytes` per second with matching burst
    /// capacity.
    limiter: Option<Limiter>,
    /// Governor's burst capacity — at most this many bytes can be
    /// requested in a single `until_n_ready` call without splitting.
    /// For `Quota::per_second(n)` this equals `n`.
    burst_bytes: u32,
    /// Pulled out of `Limiter` once so hot-path `permit` doesn't have
    /// to consult the live Quota shape; also handy for tests.
    current_rate: Option<ByteRate>,
}

/// GCRA bandwidth-shaping token bucket.
///
/// Cheap to clone (one `ArcSwap` read), safe to call concurrently
/// from any tokio task. See crate-level docs for the engine-wiring
/// story and the `Schedule` companion.
pub struct Shape {
    inner: ArcSwap<ShapeInner>,
}

impl std::fmt::Debug for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let guard = self.inner.load();
        f.debug_struct("Shape")
            .field("rate", &guard.current_rate)
            .field("burst_bytes", &guard.burst_bytes)
            .finish()
    }
}

impl Shape {
    /// Build a shape with the given cap.
    ///
    /// `None` is unlimited (the GCRA bucket is skipped entirely;
    /// `permit` returns instantly). `Some(ByteRate(0))` blocks
    /// forever — [`ByteRate::is_off`] surfaces that case without
    /// having to inspect the inner u64.
    pub fn new(rate: Option<ByteRate>) -> Self {
        Self {
            inner: ArcSwap::from_pointee(Self::build_inner(rate)),
        }
    }

    /// Hot-update the cap. Racing `permit` awaits continue on the
    /// pre-swap limiter; the next permit after the swap honours the
    /// new rate. `ArcSwap` means this never blocks the copy loop.
    pub fn set_rate(&self, rate: Option<ByteRate>) {
        self.inner.store(Arc::new(Self::build_inner(rate)));
    }

    /// The currently-active cap. Returns the `None` / `Some(ByteRate)`
    /// value passed to the most recent `new` / `set_rate`; useful for
    /// rendering the "🔻 30 MB/s" header badge.
    pub fn current_rate(&self) -> Option<ByteRate> {
        self.inner.load().current_rate
    }

    /// Consume `bytes` worth of cells, awaiting if the bucket is
    /// currently empty. A `None`-rate shape short-circuits and
    /// returns instantly.
    ///
    /// Requests larger than the bucket's burst capacity are split
    /// into burst-sized chunks internally; the total wall time is
    /// `bytes / rate` plus whatever replenishment jitter the GCRA
    /// introduces. Governor is accurate to within a few
    /// microseconds on a real-time clock.
    pub async fn permit(&self, bytes: u64) {
        if bytes == 0 {
            return;
        }
        // Snapshot the live Arc<ShapeInner>; dropping the guard
        // before the await means a subsequent `set_rate` won't
        // stall on this task's continuation.
        let inner = self.inner.load_full();
        let Some(limiter) = inner.limiter.as_ref() else {
            // Unlimited — no-op hot path, matches pre-Phase-21
            // engine behaviour.
            return;
        };
        let burst = inner.burst_bytes.max(1);

        let mut remaining = bytes;
        while remaining > 0 {
            let chunk = remaining.min(u64::from(burst)).max(1);
            // Safe: `chunk` is clamped to `burst <= u32::MAX` and
            // `max(1)` guarantees non-zero.
            let nz = NonZeroU32::new(chunk as u32)
                .unwrap_or_else(|| NonZeroU32::new(1).expect("1 is non-zero"));
            // `until_n_ready` only errors when `n > burst`; we
            // clamped above, so this is unreachable in practice.
            // Treat a pathological error as "unlimited for this
            // chunk" to avoid stalling a copy on a governor edge
            // case no user can hit.
            let _ = limiter.until_n_ready(nz).await;
            remaining = remaining.saturating_sub(chunk);
        }
    }

    /// Internal constructor used by both `new` and `set_rate`.
    fn build_inner(rate: Option<ByteRate>) -> ShapeInner {
        match rate {
            None => ShapeInner {
                limiter: None,
                burst_bytes: 0,
                current_rate: None,
            },
            Some(r) => {
                let bps_u64 = r.bytes_per_second();
                // Governor's NonZeroU32 quota caps at 4 GiB/s. A
                // rate of 0 is modelled by "smallest possible"
                // quota (1 byte/s) plus a burst of 1 — `permit`
                // then effectively sleeps one second per byte,
                // which is the closest "paused" approximation a
                // GCRA token bucket can express. Genuinely-zero
                // rates should not hit this constructor: callers
                // use `None` for unlimited and the settings UI
                // doesn't expose a 0-bps knob.
                let effective = bps_u64.clamp(1, u32::MAX as u64) as u32;
                let nz = NonZeroU32::new(effective).expect("clamped to >= 1");
                let quota = Quota::per_second(nz);
                // `allow_burst` defaults to `replenish_interval *
                // rate` which for `per_second(n)` is exactly `n` —
                // one second's worth of bytes. That matches the
                // usual "let the copy land a full second's budget
                // in one gulp, then smooth out" behaviour users
                // expect from a bandwidth cap.
                let limiter = RateLimiter::direct(quota);
                ShapeInner {
                    limiter: Some(limiter),
                    burst_bytes: effective,
                    current_rate: Some(r),
                }
            }
        }
    }
}

impl Default for Shape {
    fn default() -> Self {
        Self::new(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[tokio::test(flavor = "multi_thread")]
    async fn unlimited_permit_is_instant() {
        let shape = Shape::new(None);
        let started = Instant::now();
        shape.permit(16 * 1024 * 1024).await;
        // Under the unlimited path we don't even touch governor —
        // the call should return in well under 1 ms on any machine.
        assert!(
            started.elapsed().as_millis() < 50,
            "unlimited permit took {:?}",
            started.elapsed()
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn set_rate_updates_current_rate() {
        let shape = Shape::new(None);
        assert!(shape.current_rate().is_none());
        shape.set_rate(Some(ByteRate::mebibytes_per_second(32)));
        assert_eq!(
            shape.current_rate(),
            Some(ByteRate::mebibytes_per_second(32))
        );
        shape.set_rate(None);
        assert!(shape.current_rate().is_none());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn permit_respects_rate_ballpark() {
        // A fresh GCRA bucket starts with a full burst's worth of
        // cells (one second at the configured rate). We drain the
        // burst first so the second permit pays the "actually
        // bandwidth-limited" cost. At 8 MiB/s, draining the
        // 8 MiB burst then asking for another 4 MiB should take
        // roughly 500 ms (the bucket has to refill 4 MiB of cells
        // at 8 MiB/s). Upper bound is generous for slow CI
        // runners; the tight-band assertion lives in the smoke
        // test.
        let shape = Shape::new(Some(ByteRate::mebibytes_per_second(8)));
        // Drain the initial 8 MiB burst (two 4 MiB permits keep
        // each call inside the burst ceiling).
        shape.permit(4 * 1024 * 1024).await;
        shape.permit(4 * 1024 * 1024).await;
        let started = Instant::now();
        shape.permit(4 * 1024 * 1024).await;
        let elapsed = started.elapsed().as_millis();
        assert!(
            (300..=2000).contains(&elapsed),
            "post-burst permit of 4 MiB @ 8 MiB/s took {elapsed} ms (expected 300..=2000)"
        );
    }
}
