//! `copythat-settings` — persistent user preferences + named profiles.
//!
//! Settings live in `settings.toml` at the OS config dir
//! (`%APPDATA%\CopyThat2026` / `~/Library/Application Support/CopyThat2026` /
//! `$XDG_CONFIG_HOME/copythat`) and are round-trippable via `serde`.
//! Every field uses `#[serde(default)]` so forward-compatibility is
//! free: an older `settings.toml` missing a field gets the default
//! on load; a newer binary writing a field that an older binary
//! doesn't know about just ignores the unknown keys.
//!
//! Profiles are named snapshots of `Settings`: the user saves a
//! current config as "Archive verify" or "Fast local" and flips
//! between them without touching individual knobs. Each profile is
//! one JSON file under `settings-profiles/` — JSON because users are
//! expected to share / commit / diff these, and TOML's lack of
//! a canonical stable ordering makes JSON the nicer portable format
//! despite TOML being the preferred main-file format.
//!
//! The crate intentionally does NOT depend on `copythat-core` or
//! `copythat-hash` — it's a pure preference + IO layer. The Tauri
//! shell translates the enum variants here to the engine types at
//! enqueue time. Keeping the split means the engine stays free of
//! UI-layer concerns (theme, telemetry flag, intercept-default-copy)
//! and this crate stays free of heavyweight engine dependencies.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

pub mod defaults;
pub mod error;
pub mod profiles;

pub use error::{Result, SettingsError};
pub use profiles::{ProfileInfo, ProfileStore};

// ---------------------------------------------------------------------
// Settings root
// ---------------------------------------------------------------------

/// The top-level preferences blob. Round-trips to TOML. Every nested
/// group carries `#[serde(default)]` so a new field added in a later
/// phase doesn't invalidate on-disk settings from an older run.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct Settings {
    pub general: GeneralSettings,
    pub transfer: TransferSettings,
    pub shell: ShellSettings,
    pub secure_delete: SecureDeleteSettings,
    pub advanced: AdvancedSettings,
    /// Phase 14a — enumeration-time filters (include/exclude globs,
    /// size range, date range, attribute bits). See [`FilterSettings`].
    pub filters: FilterSettings,
    /// Phase 15 — auto-update channel + throttle state. See
    /// [`UpdaterSettings`].
    pub updater: UpdaterSettings,
    /// Phase 19a — disk-backed scan database configuration. See
    /// [`ScanSettings`].
    pub scan: ScanSettings,
}

impl Settings {
    /// Load from `path`. Missing file → default settings (not an
    /// error); malformed TOML → `SettingsError::Parse`.
    pub fn load_from(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let raw = fs::read_to_string(path).map_err(|e| SettingsError::Read {
            path: path.to_path_buf(),
            source: e,
        })?;
        let settings: Settings = toml::from_str(&raw).map_err(|e| SettingsError::Parse {
            path: path.to_path_buf(),
            message: e.to_string(),
        })?;
        Ok(settings)
    }

    /// Write to `path` atomically — stage to a temp sibling then
    /// rename so a partial write can never leave the main file in a
    /// half-baked state. Creates parent directories on demand.
    pub fn save_to(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
        {
            fs::create_dir_all(parent).map_err(|e| SettingsError::Write {
                path: parent.to_path_buf(),
                source: e,
            })?;
        }
        let serialized = toml::to_string_pretty(self).map_err(|e| SettingsError::Serialize {
            message: e.to_string(),
        })?;
        let tmp = path.with_extension("toml.tmp");
        fs::write(&tmp, serialized).map_err(|e| SettingsError::Write {
            path: tmp.clone(),
            source: e,
        })?;
        match fs::rename(&tmp, path) {
            Ok(()) => Ok(()),
            Err(e) => {
                // Rename can fail cross-volume; best-effort cleanup so
                // a stale .tmp doesn't linger.
                let _ = fs::remove_file(&tmp);
                Err(SettingsError::Write {
                    path: path.to_path_buf(),
                    source: e,
                })
            }
        }
    }

    /// Resolve the default path for the main settings file under the
    /// OS config dir. Errors only if `directories` can't resolve a
    /// home directory at all (very rare — sandboxed cron-ish envs).
    pub fn default_path() -> Result<PathBuf> {
        let dirs = project_dirs()?;
        Ok(dirs.config_dir().join("settings.toml"))
    }

    /// Load from the default path (`Settings::default_path()`). Same
    /// "missing → defaults" semantics as `load_from`.
    pub fn load_default() -> Result<Self> {
        Self::load_from(&Self::default_path()?)
    }

    /// Save to the default path. Creates the config directory as
    /// needed.
    pub fn save_default(&self) -> Result<()> {
        self.save_to(&Self::default_path()?)
    }
}

fn project_dirs() -> Result<directories::ProjectDirs> {
    directories::ProjectDirs::from("dev", "copythat", "CopyThat2026")
        .ok_or(SettingsError::NoConfigDir)
}

// ---------------------------------------------------------------------
// General
// ---------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct GeneralSettings {
    /// BCP-47 locale code (`"en"`, `"fr"`, `"pt-BR"`, …). Empty string
    /// means "auto-detect from the OS" — the UI layer resolves via
    /// `navigator.languages` / `LC_ALL` / `LANG`.
    pub language: String,
    pub theme: ThemePreference,
    /// Launch on OS startup. Phase 12 stores this flag; actual OS
    /// registration (Windows Run key, macOS LoginItems, Linux
    /// autostart .desktop) lands in Phase 14.
    pub start_with_os: bool,
    pub single_instance: bool,
    /// Close to tray rather than exit. Stored flag; tray-icon wiring
    /// lands in Phase 14.
    pub minimize_to_tray: bool,
    /// How the UI surfaces per-file errors raised by the engine.
    /// `Modal` (default) is a blocking alert dialog — right for
    /// single-file copies where one failure means "stop and tell me".
    /// `Drawer` is a non-blocking corner panel — right for big trees
    /// with many expected errors (permission issues on a profile
    /// migration) where modal-every-error is punishing. The collision
    /// prompt is always modal; only error prompts honour this toggle.
    pub error_display_mode: ErrorDisplayMode,
    /// Register a system-wide hotkey that reads files from the OS
    /// clipboard and pipes them through Copy That's staging dialog.
    /// Enabled by default because it costs nothing until the user
    /// presses the combo — no pasteboard polling, no permission asks.
    pub paste_shortcut_enabled: bool,
    /// The hotkey combo for the paste-via-Copy-That feature. Tauri's
    /// `global-shortcut` plugin parses this string directly — see
    /// <https://v2.tauri.app/plugin/global-shortcut/> for the grammar.
    /// `CmdOrCtrl` resolves to Cmd on macOS, Ctrl on Windows / Linux.
    pub paste_shortcut: String,
    /// Background task that polls the OS clipboard every ~500 ms and
    /// surfaces a toast when files appear, hinting the user can press
    /// the paste hotkey. Opt-in — polling is cheap but not free, and
    /// users who prefer zero background work keep the default `false`.
    pub clipboard_watcher_enabled: bool,
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            language: String::new(),
            theme: ThemePreference::Auto,
            start_with_os: false,
            single_instance: true,
            minimize_to_tray: false,
            error_display_mode: ErrorDisplayMode::Modal,
            paste_shortcut_enabled: true,
            paste_shortcut: defaults::DEFAULT_PASTE_SHORTCUT.to_string(),
            clipboard_watcher_enabled: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ThemePreference {
    #[default]
    Auto,
    Light,
    Dark,
}

/// How error prompts appear. See `GeneralSettings::error_display_mode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ErrorDisplayMode {
    /// Full-screen alert dialog with a dark backdrop; blocks the app
    /// until the user picks an action.
    #[default]
    Modal,
    /// Corner panel; non-blocking, no backdrop. The queue keeps
    /// advancing while the user triages.
    Drawer,
}

impl ErrorDisplayMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Modal => "modal",
            Self::Drawer => "drawer",
        }
    }
}

// ---------------------------------------------------------------------
// Transfer
// ---------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct TransferSettings {
    /// Copy-loop buffer size in bytes. Clamped by the engine to
    /// `copythat-core`'s `[MIN_BUFFER_SIZE, MAX_BUFFER_SIZE]` window
    /// — we don't import that crate here just for the constants, so
    /// the clamp lives in `TransferSettings::effective_buffer_size`
    /// mirroring the engine's behaviour.
    pub buffer_size_bytes: usize,
    pub verify: VerifyChoice,
    pub concurrency: ConcurrencyChoice,
    pub reflink: ReflinkPreference,
    pub fsync_on_close: bool,
    pub preserve_timestamps: bool,
    pub preserve_permissions: bool,
    /// Preserve filesystem ACLs (NTFS SACL / POSIX.1e ACLs). Phase 12
    /// stores the flag; the `copy_file` engine does not yet consume
    /// it — landing alongside the Phase 14 "advanced features" work.
    pub preserve_acls: bool,
    /// Phase 14 — minimum free space to leave on the destination
    /// volume, in bytes. `0` disables the guard (engine never stops
    /// mid-tree on size grounds). When `> 0`, the engine re-checks
    /// the destination's free bytes before starting each file and
    /// halts cleanly (emitting a `TreeStopped` event with the count
    /// of files actually written) if completing the next file would
    /// push the volume below this reserve. A preflight check in the
    /// UI surfaces the shortfall before the engine even starts.
    pub reserve_free_space_bytes: u64,
    /// Phase 19b — what the engine does when a source file is
    /// exclusively locked by another process.
    ///
    /// Mirrors `copythat_core::LockedFilePolicy`. Kept as a separate
    /// enum here so this crate stays free of a `copythat-core`
    /// dependency; the Tauri bridge translates at enqueue time.
    pub on_locked: LockedFilePolicyChoice,
}

impl Default for TransferSettings {
    fn default() -> Self {
        Self {
            buffer_size_bytes: defaults::DEFAULT_BUFFER_SIZE,
            verify: VerifyChoice::Off,
            concurrency: ConcurrencyChoice::Auto,
            reflink: ReflinkPreference::Prefer,
            fsync_on_close: false,
            preserve_timestamps: true,
            preserve_permissions: true,
            preserve_acls: false,
            reserve_free_space_bytes: 0,
            on_locked: LockedFilePolicyChoice::default(),
        }
    }
}

/// Phase 19b — mirror of `copythat_core::LockedFilePolicy`.
///
/// `Ask` is the default so first-run users see the prompt once and can
/// opt into per-volume remember; subsequent runs honour the user's
/// choice without re-asking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LockedFilePolicyChoice {
    /// Ask the user the first time per volume. Remembered choice
    /// (Skip / Retry / Snapshot) stored under
    /// [`TransferSettings::volume_snapshot_prefs`] (added in the
    /// per-volume remember follow-up).
    #[default]
    Ask,
    /// Short exponential backoff inside the engine; on exhaustion,
    /// surface the sharing-violation error. Pre-Phase-19b behaviour.
    Retry,
    /// Skip locked files after the retry loop exhausts.
    Skip,
    /// Fall through to a filesystem snapshot (VSS / ZFS / Btrfs / APFS).
    Snapshot,
}

impl LockedFilePolicyChoice {
    /// Stable wire string the Tauri bridge uses to round-trip the
    /// setting across the IPC boundary.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ask => "ask",
            Self::Retry => "retry",
            Self::Skip => "skip",
            Self::Snapshot => "snapshot",
        }
    }

    /// Parse the wire string. Unknown values fall back to `Ask` to
    /// keep forward-compatibility: an older binary reading a settings
    /// file written by a newer one won't panic on a variant it
    /// doesn't know.
    pub fn from_wire(s: &str) -> Self {
        match s {
            "retry" => Self::Retry,
            "skip" => Self::Skip,
            "snapshot" => Self::Snapshot,
            _ => Self::Ask,
        }
    }
}

impl TransferSettings {
    /// Clamp `buffer_size_bytes` to the engine's `[MIN, MAX]` range.
    /// Mirrors `copythat_core::options::{MIN_BUFFER_SIZE, MAX_BUFFER_SIZE}`.
    /// See `defaults.rs` for the constants; keep in sync if the
    /// engine relaxes its bounds.
    pub fn effective_buffer_size(&self) -> usize {
        self.buffer_size_bytes
            .clamp(defaults::MIN_BUFFER_SIZE, defaults::MAX_BUFFER_SIZE)
    }
}

/// Verify algorithm selection. Mirrors `copythat_hash::HashAlgorithm`
/// plus an explicit `Off` variant. We keep a separate enum here so
/// this crate doesn't depend on `copythat-hash`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum VerifyChoice {
    #[default]
    Off,
    Crc32,
    Md5,
    Sha1,
    Sha256,
    Sha512,
    XxHash3_64,
    XxHash3_128,
    Blake3,
}

/// Concurrency strategy for `copy_tree` / `move_tree`. `Auto` hands
/// control to `copythat_platform::recommend_concurrency`; `Manual`
/// caps at 16 (the Phase 6 heuristic's observed ceiling before
/// diminishing returns on SSDs).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConcurrencyChoice {
    #[default]
    Auto,
    Manual(u8),
}

impl ConcurrencyChoice {
    /// Resolved worker count given the platform recommendation.
    /// `Auto` returns `auto_hint`; `Manual(n)` clamps to `[1, 16]`.
    pub fn resolved(&self, auto_hint: u8) -> u8 {
        match self {
            Self::Auto => auto_hint,
            Self::Manual(n) => (*n).clamp(1, 16),
        }
    }
}

/// Reflink / OS-native fast-path preference. Maps 1-to-1 onto
/// `copythat_core::CopyStrategy`: `Prefer → Auto`, `Avoid → NoReflink`,
/// `Disabled → AlwaysAsync`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReflinkPreference {
    /// Try reflink first, then OS-native copy, then async loop.
    #[default]
    Prefer,
    /// Skip reflink but still use OS-native fast paths.
    Avoid,
    /// Disable every fast path — always use the async byte-by-byte engine.
    Disabled,
}

// ---------------------------------------------------------------------
// Shell
// ---------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct ShellSettings {
    /// Register the OS context-menu entries (right-click → "Copy with
    /// Copy That"). Phase 7 ships the actual COM / Finder Sync / GIO
    /// wiring; Phase 12 adds a user-level on/off flag that the
    /// installer respects on first run and subsequent launches honour.
    pub context_menu_enabled: bool,
    /// Windows-only. When on, COM copy-hook intercepts Explorer's
    /// default Ctrl+C/Ctrl+V handler. Stored flag; the registration
    /// toggle lands in Phase 14.
    pub intercept_default_copy: bool,
    /// Show an OS notification when a job completes.
    pub notify_on_completion: bool,
}

impl Default for ShellSettings {
    fn default() -> Self {
        Self {
            context_menu_enabled: true,
            intercept_default_copy: false,
            notify_on_completion: true,
        }
    }
}

// ---------------------------------------------------------------------
// Secure delete
// ---------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct SecureDeleteSettings {
    pub method: ShredMethodChoice,
    /// Require a second confirmation before shred on non-empty
    /// selections. Defaults to ON — irreversible action deserves the
    /// two-step.
    pub confirm_twice: bool,
}

impl Default for SecureDeleteSettings {
    fn default() -> Self {
        Self {
            method: ShredMethodChoice::DoD3Pass,
            confirm_twice: true,
        }
    }
}

/// Mirrors `copythat_secure_delete::ShredMethod`. Kept separate so
/// this crate doesn't depend on the secure-delete crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ShredMethodChoice {
    Zero,
    Random,
    #[default]
    DoD3Pass,
    DoD7Pass,
    Gutmann,
    Nist80088,
}

// ---------------------------------------------------------------------
// Advanced
// ---------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct AdvancedSettings {
    pub log_level: LogLevel,
    /// Always `false`. The field exists so the TOML surface carries
    /// it explicitly ("yes we thought about telemetry, and no, it
    /// never phones home") but there is no UI toggle and the Tauri
    /// layer asserts this stays off.
    #[serde(skip_deserializing)]
    pub telemetry: bool,
    pub error_policy: ErrorPolicyChoice,
    /// Days of history retained before auto-purge. `0` = never purge.
    /// The `history_purge` IPC call honours the existing 30-day
    /// default when the user hasn't customised this.
    pub history_retention_days: u32,
    /// Override the SQLite database location. `None` = use
    /// `copythat_history::History::default_path()` (OS data dir).
    pub database_path: Option<PathBuf>,
}

impl Default for AdvancedSettings {
    fn default() -> Self {
        Self {
            log_level: LogLevel::Info,
            telemetry: false,
            error_policy: ErrorPolicyChoice::Ask,
            history_retention_days: 0,
            database_path: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LogLevel {
    Off,
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

/// Mirrors `copythat_core::ErrorPolicy`. Flattened for TOML round-
/// trip (the enum-with-fields variant serialises cleanly as a
/// nested table).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ErrorPolicyChoice {
    /// Stop on the first failure and ask the user.
    #[default]
    Ask,
    /// Record the failure and continue.
    Skip,
    /// Retry N times with a fixed backoff before falling through to Skip.
    RetryN { max_attempts: u8, backoff_ms: u64 },
    /// Cancel the whole tree on first failure.
    Abort,
}

// ---------------------------------------------------------------------
// Filters (Phase 14a)
// ---------------------------------------------------------------------

/// TOML-friendly mirror of `copythat_core::FilterSet`.
///
/// Kept in this crate so `Settings` can round-trip cleanly without
/// pulling in a `copythat-core` dependency; the Tauri bridge is
/// responsible for translating into `FilterSet` at enqueue time.
///
/// Date fields use Unix epoch seconds (signed — so pre-1970 mtimes
/// don't silently wrap); `None` means "no bound on that end of the
/// range". Size fields use `u64` bytes; `None` is unbounded.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct FilterSettings {
    /// Master toggle. When `false`, the whole filter set is treated
    /// as empty regardless of the lists below — this lets the user
    /// temporarily disable filtering without losing the configured
    /// patterns.
    pub enabled: bool,
    /// Include whitelist — if non-empty, a file must match at least
    /// one pattern to survive. Directories are not gated.
    pub include_globs: Vec<String>,
    /// Exclude blacklist — any match (file or directory) is pruned.
    pub exclude_globs: Vec<String>,
    pub min_size_bytes: Option<u64>,
    pub max_size_bytes: Option<u64>,
    pub min_mtime_unix_secs: Option<i64>,
    pub max_mtime_unix_secs: Option<i64>,
    pub skip_hidden: bool,
    pub skip_system: bool,
    pub skip_readonly: bool,
}

impl FilterSettings {
    /// True when there is nothing the engine needs to do — either
    /// the master switch is off or every field is at its default.
    /// The Tauri bridge skips compile/attach when this holds.
    pub fn is_effectively_empty(&self) -> bool {
        !self.enabled
            || (self.include_globs.is_empty()
                && self.exclude_globs.is_empty()
                && self.min_size_bytes.is_none()
                && self.max_size_bytes.is_none()
                && self.min_mtime_unix_secs.is_none()
                && self.max_mtime_unix_secs.is_none()
                && !self.skip_hidden
                && !self.skip_system
                && !self.skip_readonly)
    }
}

// ---------------------------------------------------------------------
// Updater (Phase 15)
// ---------------------------------------------------------------------

/// Release channel the updater consumes manifests from. The channel
/// name is substituted into the endpoint URL (`{{channel}}`) so the
/// server can serve separate `stable.json` / `beta.json` manifests
/// from a single deployment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum UpdateChannel {
    #[default]
    Stable,
    Beta,
}

impl UpdateChannel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
        }
    }
}

/// Phase 15 — auto-update preferences and throttle state.
///
/// The updater hits `endpoints` once per launch (gated by the 24 h
/// `last_check_unix_secs` throttle) unless `auto_check` is off. When
/// a newer version is announced, the UI surfaces a notification and
/// offers to install-on-quit via Tauri's updater plugin. Signing
/// material lives in `tauri.conf.json` → `plugins.updater.pubkey`;
/// this struct intentionally carries no crypto material.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct UpdaterSettings {
    /// Master switch. `true` by default — silent check-on-launch is
    /// an expected part of a modern desktop app. `false` disables
    /// both the automatic launch check and the 24 h throttle; the
    /// user can still trigger a manual check from Settings.
    pub auto_check: bool,
    /// Release channel. `stable` by default; `beta` opts into
    /// pre-release manifests.
    pub channel: UpdateChannel,
    /// Unix epoch seconds of the last successful check. `0` means
    /// "never checked" and always lets the next launch check fire.
    /// Signed so a clock skew back to pre-1970 doesn't pin the
    /// throttle forever.
    pub last_check_unix_secs: i64,
    /// Version string the user actively dismissed ("skip this
    /// release"). While non-empty, the updater surfaces no banner
    /// for exactly that version — a newer version flips this back
    /// to empty on its own.
    pub dismissed_version: String,
    /// Minimum seconds between two automatic checks. Default is
    /// 86_400 (24 h). `0` disables the throttle — only useful for
    /// tests; the UI pins the setting at 24 h and offers no knob.
    pub check_interval_secs: u32,
}

impl Default for UpdaterSettings {
    fn default() -> Self {
        Self {
            auto_check: true,
            channel: UpdateChannel::Stable,
            last_check_unix_secs: 0,
            dismissed_version: String::new(),
            check_interval_secs: 86_400,
        }
    }
}

impl UpdaterSettings {
    /// True when `now` is far enough past `last_check_unix_secs` for
    /// the next automatic check to fire. `check_interval_secs == 0`
    /// degenerates to "always fire" (the test knob). A negative /
    /// zero `last_check` is treated as "never checked".
    pub fn due_for_check(&self, now_unix_secs: i64) -> bool {
        if self.check_interval_secs == 0 {
            return true;
        }
        if self.last_check_unix_secs <= 0 {
            return true;
        }
        let elapsed = now_unix_secs.saturating_sub(self.last_check_unix_secs);
        elapsed >= self.check_interval_secs as i64
    }
}

/// Phase 19a — disk-backed scan database preferences.
///
/// Every field is serialized with `#[serde(default)]` so an older
/// `settings.toml` missing the `[scan]` table loads with defaults
/// and a newer on-disk file with extra keys decodes cleanly on
/// pre-Phase-19a binaries (they drop the unknown keys).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct ScanSettings {
    /// Compute BLAKE3 content hashes while the enumerator streams
    /// entries into the DB. Off by default — hashing roughly doubles
    /// scan time on HDD storage; leave opt-in for users who need
    /// deterministic source hashes for the Phase 3 verify pipeline.
    pub hash_during_scan: bool,
    /// Override the on-disk location of `scan-<uuid>.db` and
    /// `main.db`. `None` keeps the default
    /// `<config-dir>/scans/`.
    pub database_path: Option<PathBuf>,
    /// Days a completed scan DB is retained before the background
    /// sweeper deletes it. `0` disables auto-cleanup. The default
    /// of 7 matches TeraCopy's retention window.
    pub auto_delete_after_days: u32,
    /// Hard cap on the number of scan DBs the sweeper keeps on
    /// disk. Older DBs are pruned past this count regardless of the
    /// age window. Default 50.
    pub max_scans_to_keep: u32,
}

impl Default for ScanSettings {
    fn default() -> Self {
        Self {
            hash_during_scan: false,
            database_path: None,
            auto_delete_after_days: 7,
            max_scans_to_keep: 50,
        }
    }
}

// ---------------------------------------------------------------------
// Convenience surface
// ---------------------------------------------------------------------

/// Map our flat `ShredMethodChoice` into whatever enum downstream
/// crates use. Kept here so consumers don't redo it in every IPC
/// handler.
impl ShredMethodChoice {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Zero => "zero",
            Self::Random => "random",
            Self::DoD3Pass => "dod-3-pass",
            Self::DoD7Pass => "dod-7-pass",
            Self::Gutmann => "gutmann",
            Self::Nist80088 => "nist-800-88",
        }
    }
}

impl VerifyChoice {
    /// The wire-format string consumed by `HashAlgorithm::from_str`
    /// in `copythat-hash`. `Off` is mapped to `None` because the
    /// engine interprets "no verify" as `CopyOptions::verify = None`.
    pub fn as_algorithm(&self) -> Option<&'static str> {
        match self {
            Self::Off => None,
            Self::Crc32 => Some("crc32"),
            Self::Md5 => Some("md5"),
            Self::Sha1 => Some("sha1"),
            Self::Sha256 => Some("sha256"),
            Self::Sha512 => Some("sha512"),
            Self::XxHash3_64 => Some("xxhash3-64"),
            Self::XxHash3_128 => Some("xxhash3-128"),
            Self::Blake3 => Some("blake3"),
        }
    }
}

impl ThemePreference {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::Trace => "trace",
            Self::Debug => "debug",
            Self::Info => "info",
            Self::Warn => "warn",
            Self::Error => "error",
        }
    }
}

impl From<io::ErrorKind> for SettingsError {
    fn from(kind: io::ErrorKind) -> Self {
        SettingsError::Io { kind }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn defaults_round_trip_via_toml() {
        let d = tempdir().unwrap();
        let path = d.path().join("settings.toml");
        let s0 = Settings::default();
        s0.save_to(&path).unwrap();
        let s1 = Settings::load_from(&path).unwrap();
        assert_eq!(s0, s1);
    }

    #[test]
    fn missing_file_returns_defaults() {
        let d = tempdir().unwrap();
        let s = Settings::load_from(&d.path().join("nope.toml")).unwrap();
        assert_eq!(s, Settings::default());
    }

    #[test]
    fn partial_toml_fills_missing_fields_with_defaults() {
        // Older binaries' on-disk toml might only carry a subset of
        // the current fields. Serde's `#[serde(default)]` on each
        // group means the load never fails on missing keys.
        let d = tempdir().unwrap();
        let path = d.path().join("settings.toml");
        fs::write(
            &path,
            r#"
[general]
language = "fr"
theme = "dark"

[transfer]
buffer-size-bytes = 4194304
"#,
        )
        .unwrap();

        let s = Settings::load_from(&path).unwrap();
        assert_eq!(s.general.language, "fr");
        assert_eq!(s.general.theme, ThemePreference::Dark);
        assert_eq!(s.transfer.buffer_size_bytes, 4_194_304);
        // Fields not in the TOML fall back to defaults.
        assert_eq!(s.shell, ShellSettings::default());
        assert_eq!(s.advanced, AdvancedSettings::default());
    }

    #[test]
    fn telemetry_is_never_deserialized() {
        // Even if a malicious / old settings.toml has `telemetry = true`,
        // the field is `#[serde(skip_deserializing)]` so the in-memory
        // value stays the Default::default() (= false).
        let d = tempdir().unwrap();
        let path = d.path().join("settings.toml");
        fs::write(
            &path,
            r#"
[advanced]
telemetry = true
log-level = "debug"
"#,
        )
        .unwrap();
        let s = Settings::load_from(&path).unwrap();
        assert!(
            !s.advanced.telemetry,
            "telemetry must never deserialize to true"
        );
        assert_eq!(s.advanced.log_level, LogLevel::Debug);
    }

    #[test]
    fn buffer_size_clamp() {
        let t = TransferSettings {
            buffer_size_bytes: 1, // below MIN
            ..Default::default()
        };
        assert_eq!(t.effective_buffer_size(), defaults::MIN_BUFFER_SIZE);

        let t = TransferSettings {
            buffer_size_bytes: usize::MAX, // above MAX
            ..Default::default()
        };
        assert_eq!(t.effective_buffer_size(), defaults::MAX_BUFFER_SIZE);

        let t = TransferSettings {
            buffer_size_bytes: 4 * 1024 * 1024,
            ..Default::default()
        };
        assert_eq!(t.effective_buffer_size(), 4 * 1024 * 1024);
    }

    #[test]
    fn concurrency_manual_clamps_to_16() {
        assert_eq!(ConcurrencyChoice::Manual(0).resolved(8), 1);
        assert_eq!(ConcurrencyChoice::Manual(1).resolved(8), 1);
        assert_eq!(ConcurrencyChoice::Manual(16).resolved(8), 16);
        assert_eq!(ConcurrencyChoice::Manual(200).resolved(8), 16);
    }

    #[test]
    fn concurrency_auto_uses_hint() {
        assert_eq!(ConcurrencyChoice::Auto.resolved(4), 4);
    }

    #[test]
    fn verify_as_algorithm_strings() {
        assert_eq!(VerifyChoice::Off.as_algorithm(), None);
        assert_eq!(VerifyChoice::Sha256.as_algorithm(), Some("sha256"));
        assert_eq!(VerifyChoice::Blake3.as_algorithm(), Some("blake3"));
    }

    #[test]
    fn error_display_mode_defaults_to_modal() {
        // Default is Modal — safe conservative choice so an unpatched
        // older binary that ignores this field still ships the
        // blocking dialog shape every user is used to.
        assert_eq!(
            GeneralSettings::default().error_display_mode,
            ErrorDisplayMode::Modal
        );
        assert_eq!(ErrorDisplayMode::default(), ErrorDisplayMode::Modal);
        assert_eq!(ErrorDisplayMode::Modal.as_str(), "modal");
        assert_eq!(ErrorDisplayMode::Drawer.as_str(), "drawer");
    }

    #[test]
    fn error_display_mode_round_trips_via_toml() {
        let d = tempdir().unwrap();
        let path = d.path().join("settings.toml");
        let mut s = Settings::default();
        s.general.error_display_mode = ErrorDisplayMode::Drawer;
        s.save_to(&path).unwrap();
        let back = Settings::load_from(&path).unwrap();
        assert_eq!(back.general.error_display_mode, ErrorDisplayMode::Drawer);
    }

    #[test]
    fn error_display_mode_serialises_as_kebab_case() {
        // On-disk TOML carries the wire string directly — verify so a
        // future enum-name change can't silently break saved files.
        let mut s = Settings::default();
        s.general.error_display_mode = ErrorDisplayMode::Drawer;
        let dumped = toml::to_string(&s).unwrap();
        assert!(
            dumped.contains(r#"error-display-mode = "drawer""#),
            "TOML dump missing kebab-case field:\n{dumped}"
        );
    }

    #[test]
    fn paste_shortcut_defaults() {
        let g = GeneralSettings::default();
        assert!(g.paste_shortcut_enabled, "paste hotkey is on by default");
        assert_eq!(g.paste_shortcut, "CmdOrCtrl+Shift+V");
        assert!(!g.clipboard_watcher_enabled, "clipboard polling is opt-in");
    }

    #[test]
    fn paste_shortcut_round_trips_via_toml() {
        let d = tempdir().unwrap();
        let path = d.path().join("settings.toml");
        let mut s = Settings::default();
        s.general.paste_shortcut_enabled = false;
        s.general.paste_shortcut = "Alt+Shift+V".to_string();
        s.general.clipboard_watcher_enabled = true;
        s.save_to(&path).unwrap();
        let back = Settings::load_from(&path).unwrap();
        assert!(!back.general.paste_shortcut_enabled);
        assert_eq!(back.general.paste_shortcut, "Alt+Shift+V");
        assert!(back.general.clipboard_watcher_enabled);
    }

    #[test]
    fn atomic_save_leaves_no_stale_tmp() {
        let d = tempdir().unwrap();
        let path = d.path().join("settings.toml");
        Settings::default().save_to(&path).unwrap();
        assert!(path.exists());
        assert!(!d.path().join("settings.toml.tmp").exists());
    }

    #[test]
    fn filters_default_is_effectively_empty() {
        assert!(FilterSettings::default().is_effectively_empty());
        assert!(Settings::default().filters.is_effectively_empty());
    }

    #[test]
    fn filters_disabled_master_treats_everything_as_empty() {
        // Even with populated lists, enabled=false must short-circuit.
        let f = FilterSettings {
            enabled: false,
            include_globs: vec!["**/*.txt".into()],
            min_size_bytes: Some(100),
            ..Default::default()
        };
        assert!(f.is_effectively_empty());
    }

    #[test]
    fn filters_round_trip_via_toml() {
        let d = tempdir().unwrap();
        let path = d.path().join("settings.toml");
        let s = Settings {
            filters: FilterSettings {
                enabled: true,
                include_globs: vec!["**/*.rs".into(), "**/*.md".into()],
                exclude_globs: vec!["**/target/**".into()],
                min_size_bytes: Some(1024),
                max_size_bytes: Some(10 * 1024 * 1024),
                min_mtime_unix_secs: Some(1_700_000_000),
                max_mtime_unix_secs: None,
                skip_hidden: true,
                skip_system: false,
                skip_readonly: true,
            },
            ..Settings::default()
        };
        s.save_to(&path).unwrap();
        let back = Settings::load_from(&path).unwrap();
        assert_eq!(back.filters, s.filters);
    }

    #[test]
    fn updater_defaults_are_auto_check_stable_24h() {
        let u = UpdaterSettings::default();
        assert!(u.auto_check);
        assert_eq!(u.channel, UpdateChannel::Stable);
        assert_eq!(u.check_interval_secs, 86_400);
        assert_eq!(u.last_check_unix_secs, 0);
        assert!(u.dismissed_version.is_empty());
    }

    #[test]
    fn updater_due_for_check_honors_throttle() {
        // Fresh defaults — never checked → always due.
        let u = UpdaterSettings::default();
        assert!(u.due_for_check(1_748_736_000));

        // Checked one second ago → not due yet.
        let u = UpdaterSettings {
            last_check_unix_secs: 1_748_735_999,
            ..Default::default()
        };
        assert!(!u.due_for_check(1_748_736_000));

        // Checked exactly 24 h ago → due.
        let u = UpdaterSettings {
            last_check_unix_secs: 1_748_736_000 - 86_400,
            ..Default::default()
        };
        assert!(u.due_for_check(1_748_736_000));

        // Interval = 0 (test knob) → always due.
        let u = UpdaterSettings {
            check_interval_secs: 0,
            last_check_unix_secs: 1_748_735_999,
            ..Default::default()
        };
        assert!(u.due_for_check(1_748_736_000));
    }

    #[test]
    fn updater_round_trips_via_toml() {
        let d = tempdir().unwrap();
        let path = d.path().join("settings.toml");
        let s = Settings {
            updater: UpdaterSettings {
                auto_check: false,
                channel: UpdateChannel::Beta,
                last_check_unix_secs: 1_748_736_000,
                dismissed_version: "1.2.3".to_string(),
                check_interval_secs: 3_600,
            },
            ..Settings::default()
        };
        s.save_to(&path).unwrap();
        let back = Settings::load_from(&path).unwrap();
        assert_eq!(back.updater, s.updater);
    }

    #[test]
    fn updater_toml_uses_kebab_case_keys() {
        let mut s = Settings::default();
        s.updater.channel = UpdateChannel::Beta;
        s.updater.dismissed_version = "0.9.9".to_string();
        let dumped = toml::to_string(&s).unwrap();
        assert!(dumped.contains("auto-check = true"), "{dumped}");
        assert!(dumped.contains(r#"channel = "beta""#), "{dumped}");
        assert!(
            dumped.contains(r#"dismissed-version = "0.9.9""#),
            "{dumped}"
        );
        assert!(dumped.contains("check-interval-secs = 86400"), "{dumped}");
    }

    #[test]
    fn filters_toml_uses_kebab_case_keys() {
        let mut s = Settings::default();
        s.filters.enabled = true;
        s.filters.skip_hidden = true;
        s.filters.min_mtime_unix_secs = Some(1_700_000_000);
        let dumped = toml::to_string(&s).unwrap();
        assert!(dumped.contains("skip-hidden = true"), "{dumped}");
        assert!(
            dumped.contains("min-mtime-unix-secs = 1700000000"),
            "{dumped}"
        );
    }
}
