//! Named profile management.
//!
//! A profile is a named snapshot of `Settings`. The user saves the
//! current in-memory state as "Archive verify" or "Fast local", then
//! flips between snapshots without touching individual knobs. Per
//! the Phase 12 spec, profiles round-trip through **JSON** (not
//! TOML) so users can share / commit / diff them cleanly; TOML is
//! still the format for the main `settings.toml`.
//!
//! Each profile is one JSON file under
//! `<config_dir>/settings-profiles/<name>.json`. Profile names are
//! validated: non-empty, no path separators, no leading `.`, capped
//! at 64 chars. Anything richer (tags, nested folders, cloud sync)
//! is deferred — users who need structure can always hand-roll.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{Result, SettingsError};
use crate::{Settings, project_dirs};

const PROFILES_DIRNAME: &str = "settings-profiles";
const PROFILE_EXT: &str = "json";
const MAX_PROFILE_NAME_LEN: usize = 64;

/// Lightweight directory-listing wrapper. Each entry describes one
/// on-disk profile — name + filesystem path. Callers load the full
/// `Settings` body with `store.load(name)`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileInfo {
    pub name: String,
    pub path: PathBuf,
}

/// Handle onto the profile directory. Clone-cheap; each operation is
/// synchronous filesystem IO. Tauri handlers wrap `.map_err(...)`
/// onto the string surface the frontend expects.
#[derive(Debug, Clone)]
pub struct ProfileStore {
    root: PathBuf,
}

impl ProfileStore {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    /// Construct a store rooted under the OS config dir's
    /// `settings-profiles/` subfolder. Creates the directory on
    /// first write — no IO yet at construction.
    pub fn default_store() -> Result<Self> {
        let dirs = project_dirs()?;
        Ok(Self::new(dirs.config_dir().join(PROFILES_DIRNAME)))
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    /// List profile names + paths, lexicographic. Non-JSON files are
    /// ignored (makes it safe to drop a `.gitignore` or a `README.md`
    /// into the folder). A missing directory is treated as "no
    /// profiles" rather than an error.
    pub fn list(&self) -> Result<Vec<ProfileInfo>> {
        if !self.root.exists() {
            return Ok(Vec::new());
        }
        let mut out = BTreeSet::new();
        let entries = fs::read_dir(&self.root).map_err(|e| SettingsError::Read {
            path: self.root.clone(),
            source: e,
        })?;
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) != Some(PROFILE_EXT) {
                continue;
            }
            let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
                continue;
            };
            out.insert((stem.to_string(), path));
        }
        Ok(out
            .into_iter()
            .map(|(name, path)| ProfileInfo { name, path })
            .collect())
    }

    /// Write `settings` to `<root>/<name>.json`. Errors if the name
    /// is invalid or the target already exists — force-overwrite is
    /// explicit via [`save_replacing`](Self::save_replacing).
    pub fn save(&self, name: &str, settings: &Settings) -> Result<ProfileInfo> {
        validate_name(name)?;
        fs::create_dir_all(&self.root).map_err(|e| SettingsError::Write {
            path: self.root.clone(),
            source: e,
        })?;
        let path = self.path_for(name);
        if path.exists() {
            return Err(SettingsError::ProfileExists {
                name: name.to_string(),
            });
        }
        self.write_profile(&path, settings)?;
        Ok(ProfileInfo {
            name: name.to_string(),
            path,
        })
    }

    /// Save, overwriting any existing profile with the same name.
    pub fn save_replacing(&self, name: &str, settings: &Settings) -> Result<ProfileInfo> {
        validate_name(name)?;
        fs::create_dir_all(&self.root).map_err(|e| SettingsError::Write {
            path: self.root.clone(),
            source: e,
        })?;
        let path = self.path_for(name);
        self.write_profile(&path, settings)?;
        Ok(ProfileInfo {
            name: name.to_string(),
            path,
        })
    }

    pub fn load(&self, name: &str) -> Result<Settings> {
        validate_name(name)?;
        let path = self.path_for(name);
        if !path.exists() {
            return Err(SettingsError::ProfileNotFound {
                name: name.to_string(),
            });
        }
        let raw = fs::read_to_string(&path).map_err(|e| SettingsError::Read {
            path: path.clone(),
            source: e,
        })?;
        let settings: Settings = serde_json::from_str(&raw).map_err(|e| SettingsError::Parse {
            path,
            message: e.to_string(),
        })?;
        Ok(settings)
    }

    pub fn delete(&self, name: &str) -> Result<()> {
        validate_name(name)?;
        let path = self.path_for(name);
        if !path.exists() {
            return Err(SettingsError::ProfileNotFound {
                name: name.to_string(),
            });
        }
        fs::remove_file(&path).map_err(|e| SettingsError::Write { path, source: e })
    }

    /// Export an existing profile to an arbitrary location (e.g. the
    /// user's Desktop, a shared drive, etc.). Overwrites the target
    /// if it exists — mirrors `fs::copy` / `fs::write` semantics.
    pub fn export(&self, name: &str, dest: &Path) -> Result<()> {
        let settings = self.load(name)?;
        if let Some(parent) = dest.parent()
            && !parent.as_os_str().is_empty()
        {
            fs::create_dir_all(parent).map_err(|e| SettingsError::Write {
                path: parent.to_path_buf(),
                source: e,
            })?;
        }
        let serialized =
            serde_json::to_string_pretty(&settings).map_err(|e| SettingsError::Serialize {
                message: e.to_string(),
            })?;
        fs::write(dest, serialized).map_err(|e| SettingsError::Write {
            path: dest.to_path_buf(),
            source: e,
        })
    }

    /// Import a profile from an arbitrary JSON file. The profile is
    /// saved under `name`; fails if `name` is already in use.
    pub fn import(&self, name: &str, src: &Path) -> Result<ProfileInfo> {
        validate_name(name)?;
        let raw = fs::read_to_string(src).map_err(|e| SettingsError::Read {
            path: src.to_path_buf(),
            source: e,
        })?;
        let settings: Settings = serde_json::from_str(&raw).map_err(|e| SettingsError::Parse {
            path: src.to_path_buf(),
            message: e.to_string(),
        })?;
        self.save(name, &settings)
    }

    fn path_for(&self, name: &str) -> PathBuf {
        self.root.join(format!("{name}.{PROFILE_EXT}"))
    }

    fn write_profile(&self, path: &Path, settings: &Settings) -> Result<()> {
        let serialized =
            serde_json::to_string_pretty(settings).map_err(|e| SettingsError::Serialize {
                message: e.to_string(),
            })?;
        let tmp = path.with_extension(format!("{PROFILE_EXT}.tmp"));
        fs::write(&tmp, serialized).map_err(|e| SettingsError::Write {
            path: tmp.clone(),
            source: e,
        })?;
        if let Err(e) = fs::rename(&tmp, path) {
            let _ = fs::remove_file(&tmp);
            return Err(SettingsError::Write {
                path: path.to_path_buf(),
                source: e,
            });
        }
        Ok(())
    }
}

fn validate_name(name: &str) -> Result<()> {
    if name.is_empty()
        || name.len() > MAX_PROFILE_NAME_LEN
        || name.starts_with('.')
        || name.contains('/')
        || name.contains('\\')
        || name.contains('\0')
    {
        return Err(SettingsError::InvalidProfileName);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn fresh_store() -> (ProfileStore, tempfile::TempDir) {
        let d = tempdir().unwrap();
        let store = ProfileStore::new(d.path().join("profiles"));
        (store, d)
    }

    #[test]
    fn save_then_load_round_trip() {
        let (store, _tmp) = fresh_store();
        let mut s = Settings::default();
        s.general.language = "fr".to_string();
        s.transfer.buffer_size_bytes = 4 * 1024 * 1024;
        let info = store.save("fast-local", &s).unwrap();
        assert_eq!(info.name, "fast-local");
        let loaded = store.load("fast-local").unwrap();
        assert_eq!(loaded, s);
    }

    #[test]
    fn save_twice_without_replace_errors() {
        let (store, _tmp) = fresh_store();
        store.save("x", &Settings::default()).unwrap();
        let err = store.save("x", &Settings::default()).unwrap_err();
        assert!(matches!(err, SettingsError::ProfileExists { .. }));
    }

    #[test]
    fn save_replacing_overwrites() {
        let (store, _tmp) = fresh_store();
        let mut a = Settings::default();
        a.general.language = "en".into();
        let mut b = Settings::default();
        b.general.language = "fr".into();

        store.save("x", &a).unwrap();
        store.save_replacing("x", &b).unwrap();
        assert_eq!(store.load("x").unwrap().general.language, "fr");
    }

    #[test]
    fn list_returns_sorted_names_only() {
        let (store, _tmp) = fresh_store();
        store.save("zebra", &Settings::default()).unwrap();
        store.save("alpha", &Settings::default()).unwrap();
        store.save("middle", &Settings::default()).unwrap();
        // Drop a non-JSON file — list() should ignore it.
        fs::write(store.root().join("readme.txt"), b"hi").unwrap();
        let names: Vec<String> = store.list().unwrap().into_iter().map(|p| p.name).collect();
        assert_eq!(names, vec!["alpha", "middle", "zebra"]);
    }

    #[test]
    fn list_on_missing_dir_is_empty() {
        let (store, _tmp) = fresh_store();
        assert!(store.list().unwrap().is_empty());
    }

    #[test]
    fn delete_known_name_ok_unknown_errors() {
        let (store, _tmp) = fresh_store();
        store.save("x", &Settings::default()).unwrap();
        store.delete("x").unwrap();
        assert!(matches!(
            store.delete("x"),
            Err(SettingsError::ProfileNotFound { .. })
        ));
    }

    #[test]
    fn invalid_names_rejected() {
        let (store, _tmp) = fresh_store();
        for bad in [
            "",
            ".hidden",
            "with/slash",
            "with\\backslash",
            "null\0byte",
            &"X".repeat(MAX_PROFILE_NAME_LEN + 1),
        ] {
            let err = store.save(bad, &Settings::default()).unwrap_err();
            assert!(
                matches!(err, SettingsError::InvalidProfileName),
                "name `{bad}` should have been rejected"
            );
        }
    }

    #[test]
    fn export_then_import_round_trip() {
        let (store, tmp) = fresh_store();
        let mut s = Settings::default();
        s.general.language = "ja".into();
        store.save("src", &s).unwrap();

        let dest = tmp.path().join("exported.json");
        store.export("src", &dest).unwrap();
        assert!(dest.exists());

        store.import("dst", &dest).unwrap();
        let imported = store.load("dst").unwrap();
        assert_eq!(imported, s);
    }

    #[test]
    fn load_unknown_profile_errors() {
        let (store, _tmp) = fresh_store();
        let err = store.load("missing").unwrap_err();
        assert!(matches!(err, SettingsError::ProfileNotFound { .. }));
    }
}
