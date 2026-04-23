//! Per-file vector clocks.
//!
//! A [`VersionVector`] maps each participating [`DeviceId`] to a
//! monotonic counter of "how many times did this device write this
//! file?". The counter is incremented on every local write; a
//! propagate copies the source's vector verbatim; a conflict-merge
//! component-wise `max`es the two vectors then bumps the resolving
//! device's entry.
//!
//! Three comparisons matter:
//!
//! - `V_a == V_b` — component-wise equality. If content also matches,
//!   no-op. If content differs, the file is corrupt on one side
//!   (disk bit flip, manual edit behind sync's back, etc.).
//! - `V_a` dominates `V_b` (`Descends`) — for every component
//!   `V_a[i] >= V_b[i]`, and at least one is strictly greater. `V_b`
//!   is an ancestor of `V_a` → propagate `V_a`'s content.
//! - Neither dominates (`Concurrent`) — both sides added writes the
//!   other hasn't seen → conflict.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Stable per-device identifier. Generated on a device's first sync
/// run, persisted in the pair DB's `device` table for life.
pub type DeviceId = Uuid;

/// Per-file vector clock — a map from [`DeviceId`] to monotonic
/// write counter.
///
/// A fresh file on a brand-new device has `vv = {device: 1}` after
/// the first write. A propagate from device A to device B leaves
/// `vv = {A: N}` on B (B didn't author this file). A concurrent
/// edit on B produces `vv = {A: N, B: 1}` on B. When A sees B's
/// version next round, it detects `Concurrent` (A's vector is
/// `{A: N}`, B's is `{A: N, B: 1}` — B dominates, so A propagates
/// B) — but only if A hasn't also edited again. If A edited
/// locally between rounds, A's vector becomes `{A: N+1}`, and
/// now `{A: N+1}` vs `{A: N, B: 1}` is concurrent (neither
/// dominates) → conflict.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct VersionVector(pub HashMap<DeviceId, u64>);

impl VersionVector {
    /// Empty vector clock.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Vector clock with one device set to counter 1 — "device just
    /// authored this file once, nobody else has seen it".
    pub fn first_write(device: DeviceId) -> Self {
        let mut m = HashMap::new();
        m.insert(device, 1);
        Self(m)
    }

    /// Increment `device`'s entry by one. Used on a local write that
    /// the engine detected by content comparison against the
    /// baseline. Creates the entry if missing.
    pub fn increment(&mut self, device: DeviceId) {
        *self.0.entry(device).or_insert(0) += 1;
    }

    /// Component-wise merge: the resulting vector's entry for each
    /// device is `max(self[d], other[d])`. Used when resolving a
    /// conflict or when replaying a `propagate` side-effect.
    pub fn merge_from(&mut self, other: &VersionVector) {
        for (d, v) in &other.0 {
            let cell = self.0.entry(*d).or_insert(0);
            if *v > *cell {
                *cell = *v;
            }
        }
    }

    /// `true` if every component of `self` is `>=` the matching
    /// component of `other`.
    fn dominates(&self, other: &VersionVector) -> bool {
        for (d, v_other) in &other.0 {
            let v_self = self.0.get(d).copied().unwrap_or(0);
            if v_self < *v_other {
                return false;
            }
        }
        true
    }

    /// Compare two vectors.
    ///
    /// - `Equal` — every component matches.
    /// - `Descends` — `self` dominates `other` with at least one
    ///   strictly-greater component.
    /// - `Ancestor` — `other` dominates `self` with at least one
    ///   strictly-greater component (the mirror of `Descends`).
    /// - `Concurrent` — neither dominates.
    pub fn compare(&self, other: &VersionVector) -> VvCompare {
        let self_dominates = self.dominates(other);
        let other_dominates = other.dominates(self);
        match (self_dominates, other_dominates) {
            (true, true) => VvCompare::Equal,
            (true, false) => VvCompare::Descends,
            (false, true) => VvCompare::Ancestor,
            (false, false) => VvCompare::Concurrent,
        }
    }
}

/// Outcome of comparing two version vectors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VvCompare {
    /// Every component matches.
    Equal,
    /// `self` strictly dominates `other` — `self` is the newer
    /// version.
    Descends,
    /// `other` strictly dominates `self` — `other` is the newer
    /// version.
    Ancestor,
    /// Neither dominates — the two vectors diverged from a common
    /// ancestor. This is a conflict.
    Concurrent,
}

/// Convenience for the engine: given two vectors known to be
/// concurrent, produce a merged vector that descends both. The
/// resolving device's component is bumped one past the component-wise
/// max.
pub fn resolve_concurrent(
    a: &VersionVector,
    b: &VersionVector,
    resolver: DeviceId,
) -> VersionVector {
    let mut out = VersionVector::new();
    out.merge_from(a);
    out.merge_from(b);
    out.increment(resolver);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dev(byte: u8) -> DeviceId {
        // Deterministic UUIDs for tests — just repeat a byte across
        // the v4 slot. Not a real device id, but it round-trips.
        let mut bytes = [0u8; 16];
        bytes[0] = byte;
        Uuid::from_bytes(bytes)
    }

    #[test]
    fn empty_vectors_are_equal() {
        let a = VersionVector::new();
        let b = VersionVector::new();
        assert_eq!(a.compare(&b), VvCompare::Equal);
    }

    #[test]
    fn first_write_descends_empty() {
        let a = VersionVector::first_write(dev(1));
        let b = VersionVector::new();
        assert_eq!(a.compare(&b), VvCompare::Descends);
        assert_eq!(b.compare(&a), VvCompare::Ancestor);
    }

    #[test]
    fn two_writes_same_device_still_descends() {
        let mut a = VersionVector::first_write(dev(1));
        a.increment(dev(1));
        let b = VersionVector::first_write(dev(1));
        assert_eq!(a.compare(&b), VvCompare::Descends);
    }

    #[test]
    fn concurrent_writes_detected() {
        // A writes; B gets A's copy (vv = {A:1}); A writes again
        // (vv = {A:2}); B writes locally (vv = {A:1, B:1}). Neither
        // dominates → Concurrent.
        let mut a = VersionVector::first_write(dev(1));
        a.increment(dev(1)); // {A: 2}
        let mut b = VersionVector::first_write(dev(1));
        b.increment(dev(2)); // {A: 1, B: 1}
        assert_eq!(a.compare(&b), VvCompare::Concurrent);
        assert_eq!(b.compare(&a), VvCompare::Concurrent);
    }

    #[test]
    fn propagated_copy_is_ancestor_of_local_edit() {
        // B has A's propagated copy; B then edits locally.
        let propagated = VersionVector::first_write(dev(1)); // {A: 1}
        let mut after_local = propagated.clone();
        after_local.increment(dev(2)); // {A: 1, B: 1}
        assert_eq!(after_local.compare(&propagated), VvCompare::Descends);
        assert_eq!(propagated.compare(&after_local), VvCompare::Ancestor);
    }

    #[test]
    fn resolve_concurrent_dominates_both() {
        let mut a = VersionVector::first_write(dev(1));
        a.increment(dev(1));
        let mut b = VersionVector::first_write(dev(1));
        b.increment(dev(2));
        let c = resolve_concurrent(&a, &b, dev(3));
        assert_eq!(c.compare(&a), VvCompare::Descends);
        assert_eq!(c.compare(&b), VvCompare::Descends);
    }

    #[test]
    fn merge_from_component_wise_max() {
        let mut a = VersionVector::new();
        a.0.insert(dev(1), 3);
        a.0.insert(dev(2), 1);
        let mut b = VersionVector::new();
        b.0.insert(dev(1), 2);
        b.0.insert(dev(2), 4);
        b.0.insert(dev(3), 7);
        a.merge_from(&b);
        assert_eq!(a.0.get(&dev(1)).copied(), Some(3));
        assert_eq!(a.0.get(&dev(2)).copied(), Some(4));
        assert_eq!(a.0.get(&dev(3)).copied(), Some(7));
    }

    #[test]
    fn equal_vectors_compare_equal_even_when_nonempty() {
        let mut a = VersionVector::new();
        a.0.insert(dev(1), 5);
        a.0.insert(dev(2), 7);
        let b = a.clone();
        assert_eq!(a.compare(&b), VvCompare::Equal);
    }
}
