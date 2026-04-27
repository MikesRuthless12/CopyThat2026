//! Phase 32 — in-memory registry of configured backends.
//!
//! The registry is the runner's single source of truth for *which*
//! remote backends exist. It intentionally doesn't own the live
//! [`opendal::Operator`]s — those get built on-demand by
//! [`crate::make_operator`] when a copy enqueues — because
//! operators carry a pool of connections and bundling them into a
//! long-lived registry would keep sockets open to every configured
//! remote.
//!
//! Persistence lives outside: `copythat-settings`'
//! `RemoteSettings::backends` is the disk-backed mirror of this
//! registry. [`BackendRegistry::snapshot`] and
//! [`BackendRegistry::from_snapshot`] move a plain `Vec<Backend>` between
//! the two.

use std::sync::RwLock;

use crate::backend::Backend;
use crate::error::BackendError;

/// Thread-safe registry keyed on a backend's display name.
#[derive(Debug, Default)]
pub struct BackendRegistry {
    inner: RwLock<Vec<Backend>>,
}

impl BackendRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Build a registry from a persisted list. Duplicate names in
    /// the input collapse to the first occurrence — the second
    /// (and any later) entries are silently dropped so a corrupt
    /// `settings.toml` can't panic the app at startup.
    pub fn from_snapshot(list: Vec<Backend>) -> Self {
        let mut seen: Vec<Backend> = Vec::with_capacity(list.len());
        for b in list {
            if !seen.iter().any(|existing| existing.name == b.name) {
                seen.push(b);
            }
        }
        Self {
            inner: RwLock::new(seen),
        }
    }

    /// Current registered backends. The returned Vec is a clone —
    /// mutating it doesn't affect the registry.
    pub fn snapshot(&self) -> Vec<Backend> {
        self.inner.read().expect("registry poisoned").clone()
    }

    /// Insert a new backend. Fails with
    /// [`BackendError::AlreadyExists`] when a backend with the same
    /// name is already registered.
    pub fn add(&self, backend: Backend) -> Result<(), BackendError> {
        let mut guard = self.inner.write().expect("registry poisoned");
        if guard.iter().any(|b| b.name == backend.name) {
            return Err(BackendError::AlreadyExists(backend.name));
        }
        guard.push(backend);
        Ok(())
    }

    /// Overwrite the existing entry with the same name, or append
    /// when the name isn't present. Useful for the Add-backend
    /// wizard's "Save" button which handles both new + edit paths.
    pub fn upsert(&self, backend: Backend) {
        let mut guard = self.inner.write().expect("registry poisoned");
        if let Some(existing) = guard.iter_mut().find(|b| b.name == backend.name) {
            *existing = backend;
        } else {
            guard.push(backend);
        }
    }

    /// Remove by name. Fails with [`BackendError::NotFound`] when the
    /// name isn't registered.
    pub fn remove(&self, name: &str) -> Result<Backend, BackendError> {
        let mut guard = self.inner.write().expect("registry poisoned");
        let idx = guard
            .iter()
            .position(|b| b.name == name)
            .ok_or_else(|| BackendError::NotFound(name.to_owned()))?;
        Ok(guard.remove(idx))
    }

    /// Look up a single backend by name.
    pub fn get(&self, name: &str) -> Option<Backend> {
        self.inner
            .read()
            .expect("registry poisoned")
            .iter()
            .find(|b| b.name == name)
            .cloned()
    }

    /// Number of registered backends.
    pub fn len(&self) -> usize {
        self.inner.read().expect("registry poisoned").len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::{BackendConfig, BackendKind, LocalFsConfig};

    fn local(name: &str) -> Backend {
        Backend {
            name: name.into(),
            kind: BackendKind::LocalFs,
            config: BackendConfig::LocalFs(LocalFsConfig {
                root: "/tmp".into(),
            }),
        }
    }

    #[test]
    fn add_and_get_round_trip() {
        let reg = BackendRegistry::new();
        reg.add(local("prod")).expect("add");
        assert_eq!(reg.len(), 1);
        assert!(reg.get("prod").is_some());
        assert!(reg.get("staging").is_none());
    }

    #[test]
    fn add_rejects_duplicate_name() {
        let reg = BackendRegistry::new();
        reg.add(local("prod")).expect("first");
        let err = reg.add(local("prod")).expect_err("second");
        assert!(matches!(err, BackendError::AlreadyExists(_)));
    }

    #[test]
    fn upsert_overwrites() {
        let reg = BackendRegistry::new();
        let mut b = local("edit");
        reg.upsert(b.clone());
        // Flip the config and re-upsert.
        b.config = BackendConfig::LocalFs(LocalFsConfig {
            root: "/new-root".into(),
        });
        reg.upsert(b.clone());
        assert_eq!(reg.len(), 1);
        match reg.get("edit").expect("present").config {
            BackendConfig::LocalFs(cfg) => assert_eq!(cfg.root, "/new-root"),
            _ => panic!("wrong variant after upsert"),
        }
    }

    #[test]
    fn remove_missing_is_typed_error() {
        let reg = BackendRegistry::new();
        let err = reg.remove("ghost").expect_err("must fail");
        assert!(matches!(err, BackendError::NotFound(_)));
    }

    #[test]
    fn from_snapshot_dedupes_corrupt_input() {
        let list = vec![local("prod"), local("prod"), local("staging")];
        let reg = BackendRegistry::from_snapshot(list);
        assert_eq!(reg.len(), 2);
        assert!(reg.get("prod").is_some());
        assert!(reg.get("staging").is_some());
    }

    #[test]
    fn snapshot_round_trip_preserves_order() {
        let reg = BackendRegistry::new();
        reg.add(local("one")).expect("one");
        reg.add(local("two")).expect("two");
        reg.add(local("three")).expect("three");
        let snap = reg.snapshot();
        assert_eq!(
            snap.iter().map(|b| b.name.as_str()).collect::<Vec<_>>(),
            ["one", "two", "three"]
        );
    }
}
