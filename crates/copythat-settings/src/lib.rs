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

pub mod conflict_profile;
pub mod defaults;
pub mod error;
pub mod profiles;

pub use conflict_profile::{
    ConflictMatch, ConflictProfile, ConflictProfileSettings, ConflictRule, ConflictRuleResolution,
};
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
    /// Phase 21 — bandwidth shaping (global cap + schedule +
    /// auto-throttle rules). See [`NetworkSettings`].
    pub network: NetworkSettings,
    /// Phase 22 — named conflict profiles + the currently-active
    /// selection. Each profile is an ordered list of
    /// `(glob → resolution)` rules + an optional fallback. The
    /// runner consults the active profile before raising a
    /// collision prompt so that previously-answered patterns
    /// auto-resolve. See [`ConflictProfileSettings`].
    pub conflict_profiles: ConflictProfileSettings,
    /// Phase 25 — two-way sync pair definitions + global defaults.
    /// See [`SyncSettings`].
    pub sync: SyncSettings,
    /// Phase 27 — content-defined chunk store. Gates delta-resume +
    /// same-job dedup + the moonshot phases that layer on top. See
    /// [`ChunkStoreSettings`].
    pub chunk_store: ChunkStoreSettings,
    /// Phase 28 — tray-resident Drop Stack. See
    /// [`DropStackSettings`].
    pub drop_stack: DropStackSettings,
    /// Phase 29 — drag-and-drop polish (spring-load, drag thumbnails,
    /// invalid-target highlight). See [`DndSettings`].
    pub dnd: DndSettings,
    /// Phase 30 — cross-platform path translation (Unicode NFC/NFD,
    /// opt-in line-ending rewrite for text files, Windows reserved-
    /// name handling, `\\?\` long-path prefix). See
    /// [`PathTranslationSettings`].
    pub path_translation: PathTranslationSettings,
    /// Phase 31 — power-aware copying (pause on battery, slow-down
    /// on metered networks, pause during Zoom calls / fullscreen,
    /// thermal-throttle cap). See [`PowerPoliciesSettings`].
    pub power: PowerPoliciesSettings,
    /// Phase 32 — cloud backend matrix (S3, R2, B2, Azure Blob, GCS,
    /// OneDrive, Drive, Dropbox, WebDAV, SFTP, FTP, LocalFs). Stores
    /// the list of configured remotes; secrets live in the OS
    /// keychain, not here. See [`RemoteSettings`].
    pub remotes: RemoteSettings,
    /// Phase 33 — mount-as-filesystem (FUSE / WinFsp). Persists the
    /// "mount latest snapshot on launch" toggle + the target
    /// mountpoint. Active mounts themselves are runtime-only.
    pub mount: MountSettings,
    /// Phase 34 — audit log export + WORM mode. See [`AuditSettings`].
    /// Off by default so a fresh install ships no log file until the
    /// user explicitly enables the toggle in Settings → Advanced →
    /// Audit log.
    pub audit: AuditSettings,
    /// Phase 35 — destination encryption + on-the-fly compression.
    /// Both stages are off by default so existing workflows see no
    /// behaviour change until the user explicitly opts in via
    /// Settings → Transfer. See [`CryptSettings`].
    pub crypt: CryptSettings,
    /// Phase 37 — mobile-companion pairing + push-notification
    /// settings. Off by default; the runner only spins up the
    /// pair-server when the user toggles `pair_enabled` in Settings
    /// → Mobile. See [`MobileSettings`]. Provider credentials
    /// (`apns_p8_pem` / `fcm_service_account_json`) are stored
    /// here for now; the Phase 37 follow-up that wires the
    /// keychain-backed APNs / FCM signers will move them to the OS
    /// keychain.
    pub mobile: MobileSettings,
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
    /// Phase 20 — when `true`, the resume modal is skipped and any
    /// unfinished jobs detected at startup are silently re-enqueued.
    /// Default `false`: the first launch with unfinished work shows
    /// the prompt; the user's choice on the prompt can flip this on
    /// per-volume in a follow-up phase. For now it's a global flag.
    pub auto_resume_interrupted: bool,
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
            auto_resume_interrupted: false,
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
    /// Phase 23 — preserve source sparseness on the destination.
    ///
    /// When `true` (the default), `CopyOptions::preserve_sparseness`
    /// is set at enqueue time and the engine's sparse pathway copies
    /// only the allocated extents. `false` forces a dense copy even
    /// when the source has holes — useful when the user needs the
    /// destination to be fully allocated (e.g. latency-sensitive VM
    /// disks where on-demand allocation hurts first-write cost).
    pub preserve_sparseness: bool,
    /// Phase 24 — security-metadata preservation. Master toggle for
    /// the entire metadata pass; when `false`, none of the per-stream
    /// flags below have any effect because the engine never enters
    /// the `meta_ops.transfer` path.
    pub preserve_security_metadata: bool,
    /// Phase 24 — preserve the Windows Mark-of-the-Web stream
    /// (`Zone.Identifier` ADS) that SmartScreen / Office Protected
    /// View key off. Security-sensitive: turning this off lets a
    /// downloaded executable shed its origin marker on copy.
    pub preserve_motw: bool,
    /// Phase 24 — preserve POSIX ACLs and the broader xattr surface
    /// (`user.*` / `system.posix_acl_*` / `trusted.*`). The
    /// `system.posix_acl_*` entries also flow through
    /// `MetaSnapshot::posix_acl` so the UI can render them.
    pub preserve_posix_acls: bool,
    /// Phase 24 — preserve the `security.selinux` xattr (Mandatory
    /// Access Control label). Required for daemons running under
    /// SELinux confined contexts to keep accessing the file after
    /// the copy.
    pub preserve_selinux_contexts: bool,
    /// Phase 24 — preserve macOS resource forks
    /// (`<file>/..namedfork/rsrc` + `com.apple.ResourceFork` xattr)
    /// and Finder info (`com.apple.FinderInfo` — color tags, Carbon
    /// metadata).
    pub preserve_resource_forks: bool,
    /// Phase 24 — when the destination filesystem can't accept the
    /// foreign metadata streams (e.g. macOS resource fork landing on
    /// a Linux ext4 share, Windows ADS on a FAT32 USB), serialise
    /// them into an `._<filename>` AppleDouble sidecar so the data
    /// survives the trip. Default ON; only meaningful when
    /// `preserve_security_metadata` is also ON.
    pub appledouble_fallback: bool,
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
            preserve_sparseness: true,
            preserve_security_metadata: true,
            preserve_motw: true,
            preserve_posix_acls: true,
            preserve_selinux_contexts: true,
            preserve_resource_forks: true,
            appledouble_fallback: true,
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

/// Phase 21 — bandwidth shaping preferences.
///
/// `mode` decides the shape's source of truth:
/// - `Off` → unlimited (engine runs at storage native speed).
/// - `Fixed { bytes_per_second }` → static cap.
/// - `Schedule` → time-of-day rules in `schedule_spec` drive
///   `Shape::set_rate` once per minute.
///
/// `auto_*` rules layer on top of the chosen mode: when the OS
/// reports the matching state, `Shape::set_rate` is called with the
/// rule's value (overriding the schedule for that minute). Phase
/// 21's `current_network_class` / `current_power_state` are stubbed
/// to "Unmetered / PluggedIn", so the auto rules persist but do
/// nothing on disk yet — the per-OS bridges land in Phase 31.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct NetworkSettings {
    pub mode: BandwidthMode,
    /// Used when `mode == Fixed`. Bytes per second; `0` is rejected
    /// upstream (the wire boundary) so it can't degrade silently
    /// into "off".
    pub fixed_bytes_per_second: u64,
    /// rclone-style schedule string, used when `mode == Schedule`.
    /// Empty = "no rules" (effectively unlimited).
    pub schedule_spec: String,
    pub auto_on_metered: AutoThrottleRule,
    pub auto_on_battery: AutoThrottleRule,
    pub auto_on_cellular: AutoThrottleRule,
}

impl Default for NetworkSettings {
    fn default() -> Self {
        Self {
            mode: BandwidthMode::Off,
            fixed_bytes_per_second: 0,
            schedule_spec: String::new(),
            auto_on_metered: AutoThrottleRule::Unchanged,
            auto_on_battery: AutoThrottleRule::Unchanged,
            auto_on_cellular: AutoThrottleRule::Unchanged,
        }
    }
}

/// `BandwidthMode` mirrors the three-way Settings-tab selector
/// "Off / fixed value / use schedule". Stored as a kebab-case
/// string in TOML so an older binary (no idea what `Schedule` is)
/// silently falls back to `Off`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BandwidthMode {
    /// Unlimited (engine ignores `Shape`). Default.
    #[default]
    Off,
    /// Fixed cap from `fixed_bytes_per_second`.
    Fixed,
    /// Time-of-day schedule from `schedule_spec`.
    Schedule,
}

impl BandwidthMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::Fixed => "fixed",
            Self::Schedule => "schedule",
        }
    }
}

/// One row of the auto-throttle table — what the engine should do
/// when the OS reports the matching condition. `Unchanged` lets the
/// schedule / fixed cap apply unmodified; `Pause` makes Shape
/// effectively block (rate = 0); `Cap` overrides with a fixed cap.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "kind", content = "value")]
pub enum AutoThrottleRule {
    /// Don't override — the global mode (`Off` / `Fixed` / `Schedule`)
    /// stays authoritative for this condition. Default.
    #[default]
    Unchanged,
    /// Block the copy entirely while the condition holds. Translates
    /// to `Shape::set_rate(Some(ByteRate(0)))`.
    Pause,
    /// Cap to N bytes/s while the condition holds. Translates to
    /// `Shape::set_rate(Some(ByteRate(value)))`.
    Cap { bytes_per_second: u64 },
}

impl AutoThrottleRule {
    /// Stable wire string — `unchanged` / `pause` / `cap-NN`. The
    /// IPC layer uses a tagged DTO so cap values round-trip cleanly;
    /// this short form is the user-visible label.
    pub fn label(self) -> &'static str {
        match self {
            Self::Unchanged => "unchanged",
            Self::Pause => "pause",
            Self::Cap { .. } => "cap",
        }
    }
}

// ---------------------------------------------------------------------
// Sync (Phase 25)
// ---------------------------------------------------------------------

/// Top-level sync state: the list of configured pairs + the defaults
/// a new pair inherits when the user clicks "Add".
///
/// Mirrors `copythat_sync::SyncPair` / `SyncMode` at the settings
/// boundary so this crate stays free of a `copythat-sync` dependency;
/// the Tauri bridge translates the DTO at enqueue time.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct SyncSettings {
    /// All configured sync pairs. Each pair has its own state DB at
    /// `.copythat-sync.db` under its left root by default; the path
    /// override is available when the DB needs to live elsewhere
    /// (read-only left root, shared NAS left root, etc).
    pub pairs: Vec<SyncPairConfig>,
    /// Default mode for a freshly-added pair.
    pub default_mode: SyncModeChoice,
    /// Default conflict-preservation suffix format. Reserved for a
    /// future phase that lets the user customise this; the engine
    /// reads it but the UI does not yet expose the knob.
    pub conflict_suffix_format: ConflictSuffixFormat,
    /// Host identifier for this device. Empty = "use the OS hostname
    /// at runtime". Override for fleet deployments where the OS
    /// hostname isn't a friendly label.
    pub host_label_override: String,
}

/// One configured pair.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct SyncPairConfig {
    /// Stable pair ID (UUID v4 string, generated when the pair is
    /// created). Used as the key in the IPC surface so a rename of
    /// `label` doesn't lose the on-disk DB association.
    pub id: String,
    /// User-visible label — "Documents ↔ NAS", "Photos ↔ Backup".
    pub label: String,
    /// Left-hand side absolute path.
    pub left: String,
    /// Right-hand side absolute path.
    pub right: String,
    /// Sync mode. Default is `TwoWay`.
    pub mode: SyncModeChoice,
    /// Optional override for the pair DB path. Empty → default
    /// `<left>/.copythat-sync.db`.
    pub db_path_override: String,
    /// ISO-8601 last-run timestamp (UTC). Empty = "never run".
    pub last_run_at: String,
    /// Summary of the last run, for the pair row in the UI:
    /// `"+3 / −1 / !2"` = 3 copies, 1 delete, 2 conflicts.
    pub last_run_summary: String,
    /// Phase 26 — when `true`, enabling this pair starts a live
    /// filesystem watcher on `left` and re-syncs on every debounced
    /// change. Default `false` — live mirror is an explicit opt-in
    /// because it runs a background thread per watched pair.
    pub live_mirror: bool,
}

/// Mirror of `copythat_sync::SyncMode`. Kept as a separate enum so
/// this crate has no engine dependency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SyncModeChoice {
    #[default]
    TwoWay,
    MirrorLeftToRight,
    MirrorRightToLeft,
    ContributeLeftToRight,
}

impl SyncModeChoice {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TwoWay => "two-way",
            Self::MirrorLeftToRight => "mirror-left-to-right",
            Self::MirrorRightToLeft => "mirror-right-to-left",
            Self::ContributeLeftToRight => "contribute-left-to-right",
        }
    }

    pub fn from_wire(s: &str) -> Self {
        match s {
            "mirror-left-to-right" => Self::MirrorLeftToRight,
            "mirror-right-to-left" => Self::MirrorRightToLeft,
            "contribute-left-to-right" => Self::ContributeLeftToRight,
            _ => Self::TwoWay,
        }
    }
}

/// Placeholder for a future per-pair conflict-suffix format
/// (`syncthing-style`, `timestamp-only`, `host-only`, etc).
/// Currently only the Syncthing-style default is implemented.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConflictSuffixFormat {
    /// `name.sync-conflict-YYYYMMDD-HHMMSS-<host>.ext`. Default.
    #[default]
    Syncthing,
}

// ---------------------------------------------------------------------
// Phase 27 — chunk store
// ---------------------------------------------------------------------

/// Content-defined chunk store configuration.
///
/// The chunk store is always optional — Copy That works identically
/// with it disabled. When enabled, every copy ingests its source
/// files into the store so that (a) a retry of the same copy only
/// re-writes the chunks that actually changed, and (b) within one
/// job, files that share content blocks store those blocks once.
/// The moonshot Phases 49–51 extend the same store for backup + P2P
/// sync + encrypted collaboration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct ChunkStoreSettings {
    /// Master toggle. Default `false` — opt-in so new users don't
    /// pay the disk cost of a chunk store they might not need.
    pub enabled: bool,
    /// Absolute path to the chunk-store root. Empty string = "use
    /// [`copythat_chunk::default_chunk_store_path`] at runtime", which
    /// resolves to `<data-dir>/chunks/` under the Copy That project
    /// dir.
    pub location_override: String,
    /// Maximum store size in bytes. `0` disables the cap (unbounded).
    /// The default of 20 GiB is generous enough to hold a few
    /// full-disk backups' worth of chunks on a typical workstation.
    pub max_size_bytes: u64,
    /// Prune chunks that haven't been referenced for this many days.
    /// `0` disables pruning. Default 60 days.
    pub prune_older_than_days: u32,
}

impl Default for ChunkStoreSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            location_override: String::new(),
            // 20 GiB.
            max_size_bytes: 20 * 1024 * 1024 * 1024,
            prune_older_than_days: 60,
        }
    }
}

// ---------------------------------------------------------------------
// Phase 28 — tray-resident Drop Stack
// ---------------------------------------------------------------------

/// Drop Stack preferences.
///
/// The Drop Stack itself (the list of staged paths) is persisted in
/// a separate `dropstack.json` file under the same config dir —
/// this struct holds only the UI knobs that govern the tray icon +
/// window behaviour, which belong with every other Settings
/// category.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct DropStackSettings {
    /// Show the tray icon on app start. Default `true` — Copy That
    /// has had a tray icon since Phase 16; the Drop Stack-specific
    /// menu items extend the existing tray surface.
    pub show_tray_icon: bool,
    /// Pin the Drop Stack window always-on-top. Default `false`;
    /// power users flip this on so the window floats over their
    /// source applications while they drag.
    pub always_on_top: bool,
    /// Open the Drop Stack window automatically on app start.
    /// Default `false` — the window is lazy until the user clicks
    /// the tray icon or the "Drop Stack" menu entry.
    pub open_on_start: bool,
    /// Last-known window geometry. `None` = "let Tauri pick"
    /// (720×480 default). Phase 28 persists whatever Tauri reports
    /// on `CloseRequested` so the user's preferred size + position
    /// survive restarts.
    pub window_bounds: Option<DropStackBounds>,
}

impl Default for DropStackSettings {
    fn default() -> Self {
        Self {
            show_tray_icon: true,
            always_on_top: false,
            open_on_start: false,
            window_bounds: None,
        }
    }
}

/// Drop Stack window geometry. Coordinates are logical (Tauri's
/// default). The `monitor` field is a stable identifier so moving
/// the window across monitors doesn't make the next open show up
/// off-screen.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct DropStackBounds {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    /// Monitor label (OS-provided) the window was last positioned
    /// on. Empty string = "unknown"; the app falls back to the
    /// primary monitor.
    pub monitor: String,
}

impl Default for DropStackBounds {
    fn default() -> Self {
        Self {
            x: 100,
            y: 100,
            width: 380,
            height: 520,
            monitor: String::new(),
        }
    }
}

// ---------------------------------------------------------------------
// Phase 29 — drag-and-drop polish
// ---------------------------------------------------------------------

/// Drag-and-drop UX preferences.
///
/// Governs Phase 29's spring-loaded folders, drag thumbnails, and the
/// error-border treatment on invalid drop targets. Changes take effect
/// the next time the DestinationPicker or DropTarget component is
/// mounted — no restart required.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct DndSettings {
    /// Master toggle for spring-loaded folders. Default `true`.
    pub spring_load_enabled: bool,
    /// Spring-load delay in milliseconds. Clamped to
    /// [`DND_MIN_SPRING_MS`] .. [`DND_MAX_SPRING_MS`] on both read and
    /// write so a hand-edited TOML file can't push the value outside
    /// the spec'd 200..2000 ms band. Default `650` — matches macOS
    /// Finder's well-known figure.
    pub spring_load_delay_ms: u32,
    /// Render a drag thumbnail (canvas composited via `setDragImage`)
    /// when dragging rows out of in-app surfaces like the Drop Stack.
    /// Default `true`; users on low-end GPUs or with prefers-reduced-
    /// motion can turn it off.
    pub show_drag_thumbnails: bool,
    /// Paint the red error border + tooltip on drop targets that
    /// aren't writable (read-only FS, insufficient permission).
    /// Default `true`. Off falls back to the plain hover border so
    /// the target visually looks droppable; the actual copy will
    /// still fail with the underlying permission error.
    pub highlight_invalid_targets: bool,
}

/// Minimum spring-load delay (50 ms). Anything shorter opens folders
/// the moment the cursor nicks the edge, which defeats the "deliberate
/// hover" intent.
pub const DND_MIN_SPRING_MS: u32 = 50;

/// Maximum spring-load delay (5 000 ms). Guard against accidental huge
/// values in hand-edited TOML — the UI caps its slider at 2 000 ms but
/// the clamp gives us one more safety net.
pub const DND_MAX_SPRING_MS: u32 = 5_000;

impl Default for DndSettings {
    fn default() -> Self {
        Self {
            spring_load_enabled: true,
            spring_load_delay_ms: 650,
            show_drag_thumbnails: true,
            highlight_invalid_targets: true,
        }
    }
}

impl DndSettings {
    /// Returns the effective spring-load delay, clamped into the
    /// valid band. Callers should prefer this over raw field access
    /// so out-of-range TOML can't reach the UI.
    pub fn effective_spring_ms(&self) -> u32 {
        self.spring_load_delay_ms
            .clamp(DND_MIN_SPRING_MS, DND_MAX_SPRING_MS)
    }
}

// ---------------------------------------------------------------------
// Phase 30 — cross-platform path translation
// ---------------------------------------------------------------------

/// Destination platform selection for path translation. Mirrors
/// `copythat_core::translate::TargetOs` but kept local so this crate
/// stays independent of `copythat-core`; the Tauri bridge translates
/// at enqueue time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TargetOsChoice {
    /// Resolve to the host OS at enqueue time. Right default for the
    /// common "copy from one local drive to another" case; only flip
    /// when the destination is a networked / mounted filesystem from
    /// a different platform.
    #[default]
    Auto,
    Windows,
    MacOs,
    Linux,
}

impl TargetOsChoice {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Windows => "windows",
            Self::MacOs => "macos",
            Self::Linux => "linux",
        }
    }

    /// Parse the wire string. Unknown values fall back to `Auto` so
    /// an older binary reading a settings file written by a newer
    /// one never panics on a variant it doesn't know.
    pub fn from_wire(s: &str) -> Self {
        match s {
            "windows" => Self::Windows,
            "macos" => Self::MacOs,
            "linux" => Self::Linux,
            _ => Self::Auto,
        }
    }
}

/// Unicode normalization mode selector. Mirrors
/// `copythat_core::translate::NormalizationMode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NormalizationModeChoice {
    /// NFC for Windows + Linux destinations; leave macOS alone.
    /// Default because it's the "do the right thing" answer for
    /// cross-platform sync.
    #[default]
    Auto,
    /// Force composed form (NFC).
    Nfc,
    /// Force decomposed form (NFD).
    Nfd,
    /// Don't renormalize — copy the source name unchanged.
    AsIs,
}

impl NormalizationModeChoice {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Nfc => "nfc",
            Self::Nfd => "nfd",
            Self::AsIs => "as-is",
        }
    }

    pub fn from_wire(s: &str) -> Self {
        match s {
            "nfc" => Self::Nfc,
            "nfd" => Self::Nfd,
            "as-is" => Self::AsIs,
            _ => Self::Auto,
        }
    }
}

/// Line-ending rewrite mode selector. Mirrors
/// `copythat_core::translate::LineEndingMode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LineEndingModeChoice {
    /// Preserve whatever the source has. Default.
    #[default]
    AsIs,
    /// Force CRLF (`\r\n`).
    Crlf,
    /// Force LF (`\n`).
    Lf,
}

impl LineEndingModeChoice {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AsIs => "as-is",
            Self::Crlf => "crlf",
            Self::Lf => "lf",
        }
    }

    pub fn from_wire(s: &str) -> Self {
        match s {
            "crlf" => Self::Crlf,
            "lf" => Self::Lf,
            _ => Self::AsIs,
        }
    }
}

/// Reserved-Windows-name strategy selector. Mirrors
/// `copythat_core::translate::ReservedNameStrategy`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReservedNameChoice {
    /// Append `_` between the stem and extension. Default — keeps
    /// the copy moving without a user prompt.
    #[default]
    Suffix,
    /// Reject with a typed error so the UI can surface it in the
    /// aggregate conflict dialog.
    Reject,
}

impl ReservedNameChoice {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Suffix => "suffix",
            Self::Reject => "reject",
        }
    }

    pub fn from_wire(s: &str) -> Self {
        match s {
            "reject" => Self::Reject,
            _ => Self::Suffix,
        }
    }
}

/// Long-path strategy selector. Mirrors
/// `copythat_core::translate::LongPathStrategy`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LongPathChoice {
    /// Prefix with `\\?\` (or `\\?\UNC\`). Default — Windows 10
    /// 1607+ handles both transparently, and legacy tools can still
    /// round-trip the long-path form through the Win32 API.
    #[default]
    Win32LongPath,
    /// Shrink the filename stem to fit inside MAX_PATH.
    Truncate,
    /// Reject with a typed error.
    Reject,
}

impl LongPathChoice {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Win32LongPath => "win32-long-path",
            Self::Truncate => "truncate",
            Self::Reject => "reject",
        }
    }

    pub fn from_wire(s: &str) -> Self {
        match s {
            "truncate" => Self::Truncate,
            "reject" => Self::Reject,
            _ => Self::Win32LongPath,
        }
    }
}

/// Persistent cross-platform translation preferences. TOML-round-
/// trippable; the Tauri bridge builds
/// `copythat_core::translate::PathPolicy` at enqueue time.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct PathTranslationSettings {
    /// Master toggle. When `false`, the runner skips the translator
    /// entirely and the engine sees the source path unchanged —
    /// matching pre-Phase-30 behaviour. Default `true`.
    pub enabled: bool,
    pub target_os: TargetOsChoice,
    pub unicode_normalization: NormalizationModeChoice,
    pub line_endings: LineEndingModeChoice,
    pub reserved_name_strategy: ReservedNameChoice,
    pub long_path_strategy: LongPathChoice,
    /// Lowercase extensions (no leading dot) eligible for
    /// line-ending conversion. Default matches
    /// `copythat_core::translate::default_text_extensions()`.
    pub line_ending_allowlist: Vec<String>,
}

impl Default for PathTranslationSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            target_os: TargetOsChoice::Auto,
            unicode_normalization: NormalizationModeChoice::Auto,
            line_endings: LineEndingModeChoice::AsIs,
            reserved_name_strategy: ReservedNameChoice::Suffix,
            long_path_strategy: LongPathChoice::Win32LongPath,
            line_ending_allowlist: default_text_extensions_for_settings(),
        }
    }
}

/// Mirror of `copythat_core::translate::default_text_extensions()`
/// kept inline so this crate doesn't pull in `copythat-core`. Sync
/// by hand if the engine's allowlist changes — the Phase-30 smoke
/// test asserts parity between the two lists.
pub fn default_text_extensions_for_settings() -> Vec<String> {
    [
        "txt", "md", "csv", "json", "xml", "yaml", "yml", "ini", "conf", "sh", "py", "rs", "ts",
        "js", "css", "html",
    ]
    .iter()
    .map(|s| (*s).to_string())
    .collect()
}

// ---------------------------------------------------------------------
// Phase 31 — power-aware copying
// ---------------------------------------------------------------------

/// Per-dimension policy choice with a uniform `Continue / Pause /
/// Cap` shape. Mirrors `copythat_power::BatteryPolicy` and
/// `copythat_power::NetworkPolicy` — kept local so this crate stays
/// independent of `copythat-power`. The Tauri bridge translates at
/// enqueue time.
///
/// `Cap` carries the rate as bytes-per-second (matches the
/// `NetworkSettings::fixed_bytes_per_second` wire format from Phase
/// 21).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "kind")]
pub enum PowerRuleChoice {
    #[default]
    Continue,
    Pause,
    Cap {
        bytes_per_second: u64,
    },
}

impl PowerRuleChoice {
    /// Stable short wire string — used by the IPC DTO so older
    /// frontends that expect string enums round-trip cleanly.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Continue => "continue",
            Self::Pause => "pause",
            Self::Cap { .. } => "cap",
        }
    }
}

/// Thermal-specific policy — caps as a *percent of current shape
/// rate* rather than an absolute bytes-per-second, so cooling down
/// works across bandwidth settings. Default matches the brief's
/// "cap to 50 %" when throttling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "kind")]
pub enum ThermalRuleChoice {
    Continue,
    Pause,
    CapPercent { percent: u8 },
}

impl Default for ThermalRuleChoice {
    fn default() -> Self {
        Self::CapPercent { percent: 50 }
    }
}

impl ThermalRuleChoice {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Continue => "continue",
            Self::Pause => "pause",
            Self::CapPercent { .. } => "cap-percent",
        }
    }
}

/// Full power-policy settings group. Round-trips via TOML; the Tauri
/// runner subscribes to `copythat_power::PowerBus` and maps each
/// event through these rules.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct PowerPoliciesSettings {
    /// Master toggle. When `false`, the runner skips the power
    /// subscriber entirely — pre-Phase-31 behaviour. Default `true`
    /// so the default policies take effect immediately after upgrade.
    pub enabled: bool,
    /// "On battery" rule. Default `Continue` — existing behaviour
    /// for any user who hasn't changed their Settings.
    pub battery: PowerRuleChoice,
    /// "On metered network" rule. Default `Continue`.
    pub metered: PowerRuleChoice,
    /// "On cellular network" rule. Default `Continue`.
    pub cellular: PowerRuleChoice,
    /// "When presenting (Zoom / Teams / Keynote / PowerPoint / Meet)"
    /// rule. Default `Pause` per the Phase 31 brief.
    pub presentation: PowerRuleChoice,
    /// "When any app is fullscreen" rule. Default `Continue`.
    pub fullscreen: PowerRuleChoice,
    /// "When CPU is thermal-throttling" rule. Default
    /// `CapPercent(50)` per the Phase 31 brief.
    pub thermal: ThermalRuleChoice,
}

impl Default for PowerPoliciesSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            battery: PowerRuleChoice::Continue,
            metered: PowerRuleChoice::Continue,
            cellular: PowerRuleChoice::Continue,
            presentation: PowerRuleChoice::Pause,
            fullscreen: PowerRuleChoice::Continue,
            thermal: ThermalRuleChoice::default(),
        }
    }
}

// ---------------------------------------------------------------------
// Phase 32 — cloud backend matrix (RemoteSettings)
// ---------------------------------------------------------------------
//
// The settings layer owns *persistence*; the `copythat-cloud` crate
// owns *operators + credentials*. These types are the on-disk mirror
// of `copythat_cloud::Backend`. They stay here (rather than in
// `copythat-cloud`) to keep this crate free of the OpenDAL dep — the
// Tauri shell translates `BackendConfigEntry` into
// `copythat_cloud::Backend` when it builds operators.

/// Phase 32 — list of user-configured remote backends + a per-backend
/// default-root override. Secrets never live here; they're in the OS
/// keychain under `copythat-cloud/<name>` via the
/// `copythat-cloud::Credentials` API. The entries round-trip through
/// TOML so older builds without Phase 32 support just ignore the
/// `remotes` table.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct RemoteSettings {
    /// Configured backends, in display order. Duplicate names are
    /// collapsed at registry-load time (see
    /// `copythat_cloud::BackendRegistry::from_snapshot`).
    pub backends: Vec<BackendConfigEntry>,
    /// Per-backend name that the runner defaults to when a user
    /// picks "Copy to remote…" without naming one. Empty = no
    /// default; the picker opens unfiltered.
    pub default_backend: String,
}

/// One TOML-persisted backend descriptor. The kind-specific config
/// is stored as a sibling `Option<...>` rather than a tagged enum so
/// `#[serde(default)]`-driven forward compat still works when a
/// future build adds a new kind — older readers see `kind = "..."`
/// plus one or more unrecognised sub-tables and keep the entry
/// intact without panicking.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct BackendConfigEntry {
    pub name: String,
    pub kind: BackendKindChoice,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_fs: Option<LocalFsBackendConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3: Option<S3BackendConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r2: Option<S3BackendConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b2: Option<S3BackendConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_blob: Option<AzureBlobBackendConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gcs: Option<GcsBackendConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onedrive: Option<OAuthBackendConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_drive: Option<OAuthBackendConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropbox: Option<OAuthBackendConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webdav: Option<WebdavBackendConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sftp: Option<SftpBackendConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ftp: Option<FtpBackendConfig>,
}

/// Which of the 12 backend kinds this entry configures. Wire string
/// mirrors `copythat_cloud::BackendKind::wire`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BackendKindChoice {
    S3,
    R2,
    B2,
    AzureBlob,
    Gcs,
    Onedrive,
    GoogleDrive,
    Dropbox,
    Webdav,
    Sftp,
    Ftp,
    #[default]
    LocalFs,
}

impl BackendKindChoice {
    /// Stable wire string — kept in lockstep with
    /// `copythat_cloud::BackendKind::wire`.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::S3 => "s3",
            Self::R2 => "r2",
            Self::B2 => "b2",
            Self::AzureBlob => "azure-blob",
            Self::Gcs => "gcs",
            Self::Onedrive => "onedrive",
            Self::GoogleDrive => "google-drive",
            Self::Dropbox => "dropbox",
            Self::Webdav => "webdav",
            Self::Sftp => "sftp",
            Self::Ftp => "ftp",
            Self::LocalFs => "local-fs",
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct LocalFsBackendConfig {
    pub root: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct S3BackendConfig {
    pub bucket: String,
    pub region: String,
    pub endpoint: String,
    pub root: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct AzureBlobBackendConfig {
    pub container: String,
    pub account_name: String,
    pub endpoint: String,
    pub root: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct GcsBackendConfig {
    pub bucket: String,
    pub service_account: String,
    pub root: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct OAuthBackendConfig {
    pub client_id: String,
    pub root: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct WebdavBackendConfig {
    pub endpoint: String,
    pub username: String,
    pub root: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct SftpBackendConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub root: String,
}

impl Default for SftpBackendConfig {
    fn default() -> Self {
        Self {
            host: String::new(),
            port: 22,
            username: String::new(),
            root: String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct FtpBackendConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub root: String,
}

impl Default for FtpBackendConfig {
    fn default() -> Self {
        Self {
            host: String::new(),
            port: 21,
            username: String::new(),
            root: String::new(),
        }
    }
}

// ---------------------------------------------------------------------
// Phase 33 — mount-as-filesystem (MountSettings)
// ---------------------------------------------------------------------

/// Phase 33 — persisted preferences for the `copythat-mount` integration.
/// Active mount handles + their mountpoints live at runtime in the
/// Tauri `AppState.mounts` registry; this struct only captures the
/// "start with the latest snapshot mounted" toggle + its target path.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct MountSettings {
    /// When `true`, the Tauri runner's startup hook (see
    /// `apps/copythat-ui/src-tauri/src/mount_commands.rs::mount_latest_on_launch`)
    /// picks the most recent successful history row and mounts it at
    /// [`Self::mount_on_launch_path`]. No-op when the path is empty
    /// or history has no rows.
    pub mount_on_launch: bool,
    /// Absolute path the auto-mount lands at. Empty = disabled (the
    /// toggle can be on without a configured path if the user is
    /// between a toggle-on + path-picker step).
    pub mount_on_launch_path: String,
}

// ---------------------------------------------------------------------
// Phase 34 — audit log export + WORM mode (AuditSettings)
// ---------------------------------------------------------------------

/// Enterprise-grade audit-log preferences. Off by default. When on,
/// the Tauri runner opens a [`copythat_audit::AuditSink`] at startup
/// (or on the first `update_settings` flip) and records the
/// brief's eight [`copythat_audit::AuditEvent`] variants.
///
/// `format` persists the kebab-case
/// [`copythat_audit::AuditFormat`] identifier (`csv`, `json-lines`,
/// `syslog`, `cef`, `leef`) — the audit crate's enum is mirrored as a
/// string here so [`Settings`] can round-trip without taking a hard
/// dep on `copythat-audit` (keeps the settings crate a pure preference
/// layer, consistent with the Phase 9 / 27 pattern).
///
/// `worm` stores the kebab-case [`copythat_audit::WormMode`]
/// identifier (`off` / `on`) for the same reason.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct AuditSettings {
    /// Master toggle. Off = no sink, no log file, no tracing fan-out.
    pub enabled: bool,
    /// Kebab-case format identifier. Default `"json-lines"`.
    pub format: String,
    /// Absolute path to the audit log file. Empty = derive a default
    /// under `<config-dir>/audit/copythat-audit.log` at sink open
    /// time.
    pub file_path: String,
    /// Kebab-case WORM state (`"off"` / `"on"`). Default `"off"`.
    /// When `on`, Copy That applies the platform's append-only flag
    /// after every create / rotation.
    pub worm: String,
    /// Rotation threshold in bytes. Zero = no rotation. Default
    /// 10 MiB.
    pub max_size_bytes: u64,
    /// Optional remote syslog destination (`host:port`). Phase 34
    /// writes the file path sink only; the remote bridge is a
    /// Phase 36 CLI follow-up. Persisted here so the Settings UI can
    /// collect the value ahead of time.
    pub syslog_destination: String,
}

impl Default for AuditSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            format: "json-lines".into(),
            file_path: String::new(),
            worm: "off".into(),
            max_size_bytes: 10 * 1024 * 1024,
            syslog_destination: String::new(),
        }
    }
}

// ---------------------------------------------------------------------
// Phase 35 — encryption + compression (CryptSettings)
// ---------------------------------------------------------------------

/// Persisted encryption + compression preferences. The live
/// [`copythat_crypt::EncryptionPolicy`] / `CompressionPolicy` are
/// built from these strings at copy time by the Tauri runner.
///
/// The enum-shaped fields (`encryption_mode`, `compression_mode`)
/// are kebab-case strings for the same reason `audit.format` is:
/// keeps the settings crate a pure preference layer with no
/// `copythat-crypt` dep.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct CryptSettings {
    /// `"off" | "passphrase" | "recipients"`. When `"passphrase"`,
    /// the Settings UI pops a password modal at copy-start time —
    /// the passphrase never touches disk. When `"recipients"`, the
    /// `recipients_file` path is read for `age1…` / SSH keys.
    pub encryption_mode: String,
    /// Absolute path to a file with one recipient per line (an
    /// `age1…` string or an SSH public key). Ignored unless
    /// `encryption_mode == "recipients"`.
    pub recipients_file: String,
    /// `"off" | "always" | "smart"`. Default `"off"`.
    pub compression_mode: String,
    /// zstd level 1–22. UI slider clamps; defaults to 3 (zstd's
    /// own CLI default and the workspace's "useful + fast" pick).
    pub compression_level: i32,
    /// Extra file extensions the Smart policy should deny on top
    /// of the built-in defaults (`jpg`, `mp4`, `zip`, …). Lowercase,
    /// no leading dot.
    pub compression_extra_deny: Vec<String>,
}

impl Default for CryptSettings {
    fn default() -> Self {
        Self {
            encryption_mode: "off".into(),
            recipients_file: String::new(),
            compression_mode: "off".into(),
            compression_level: 3,
            compression_extra_deny: Vec::new(),
        }
    }
}

// ---------------------------------------------------------------------
// Phase 37 — mobile companion (MobileSettings)
// ---------------------------------------------------------------------

/// Persisted mobile-companion preferences. Mirrors
/// `copythat_mobile::MobileSettings` but with stringly-typed fields
/// so this crate stays free of the reqwest / x25519 dep tree the
/// mobile crate carries. The Tauri runner converts on demand.
///
/// Off by default — a fresh install ships with `pair_enabled =
/// false` and the runner skips registering the desktop peer-id
/// with PeerJS until the user flips the toggle on in Settings →
/// Mobile.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct MobileSettings {
    /// Master toggle for new-device enrolment. While `true`, the
    /// Settings → Mobile panel shows the pairing QR + accepts new
    /// pairing handshakes.
    pub pair_enabled: bool,
    /// "Always connect to mobile app" — when `true`, the runner
    /// auto-registers the persisted `desktop_peer_id` with the
    /// PeerJS broker on every launch so already-paired phones can
    /// connect anytime the desktop is running.
    pub auto_connect: bool,
    /// PeerJS broker URL. Empty string means the public default
    /// (`0.peerjs.com`); production deployments override with a
    /// self-hosted broker.
    pub peerjs_broker: String,
    /// Stable PeerJS peer-id the desktop registers under. Minted
    /// once via `copythat_mobile::mint_peer_id` and persisted so
    /// already-paired phones can reconnect without re-pairing.
    pub desktop_peer_id: String,
    /// Persisted records of every device that has completed
    /// pairing.
    pub pairings: Vec<MobilePairingEntry>,
    /// PEM-encoded `.p8` ECDSA P-256 private key Apple issues for
    /// APNs token-based authentication. Empty string disables APNs
    /// pushes. The Phase 37 follow-up moves this to the OS keychain;
    /// holding it in TOML in the meantime keeps the runner wiring
    /// simple while the per-platform keychain helpers ship.
    pub apns_p8_pem: String,
    /// Apple-issued team identifier (10 chars).
    pub apns_team_id: String,
    /// Apple-issued key identifier (10 chars).
    pub apns_key_id: String,
    /// Google service-account JSON for FCM HTTP v1. Empty string
    /// disables FCM pushes.
    pub fcm_service_account_json: String,
}

/// One persisted pairing record. Mirrors
/// `copythat_mobile::PairingRecord`; the binary X25519 public key is
/// hex-encoded so settings TOML stays human-readable + cross-tool
/// compatible.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct MobilePairingEntry {
    pub label: String,
    /// Lowercase hex-encoded 32-byte X25519 public key.
    pub phone_public_key_hex: String,
    /// Unix-epoch seconds when the pairing was committed.
    pub paired_at: i64,
    /// Optional push target. Schema mirrors
    /// `copythat_mobile::PushTarget`'s tagged-enum wire form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_target: Option<MobilePushTarget>,
}

/// Tagged push-target sub-record persisted in the `pairings` array.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum MobilePushTarget {
    Apns {
        token: String,
    },
    Fcm {
        token: String,
    },
    /// Smoke-test only.
    StubEndpoint {
        url: String,
    },
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
    fn network_defaults_off() {
        let n = NetworkSettings::default();
        assert_eq!(n.mode, BandwidthMode::Off);
        assert_eq!(n.fixed_bytes_per_second, 0);
        assert_eq!(n.schedule_spec, "");
        assert_eq!(n.auto_on_metered, AutoThrottleRule::Unchanged);
        assert_eq!(n.auto_on_battery, AutoThrottleRule::Unchanged);
        assert_eq!(n.auto_on_cellular, AutoThrottleRule::Unchanged);
    }

    #[test]
    fn network_round_trips_via_toml() {
        let d = tempdir().unwrap();
        let path = d.path().join("settings.toml");
        let s = Settings {
            network: NetworkSettings {
                mode: BandwidthMode::Schedule,
                fixed_bytes_per_second: 5 * 1024 * 1024,
                schedule_spec: "08:00,512k 18:00,10M Sat-Sun,unlimited".to_string(),
                auto_on_metered: AutoThrottleRule::Cap {
                    bytes_per_second: 1024 * 1024,
                },
                auto_on_battery: AutoThrottleRule::Pause,
                auto_on_cellular: AutoThrottleRule::Cap {
                    bytes_per_second: 100 * 1024,
                },
            },
            ..Settings::default()
        };
        s.save_to(&path).unwrap();
        let back = Settings::load_from(&path).unwrap();
        assert_eq!(back.network, s.network);
    }

    #[test]
    fn network_toml_uses_kebab_case_keys() {
        let mut s = Settings::default();
        s.network.mode = BandwidthMode::Fixed;
        s.network.fixed_bytes_per_second = 4 * 1024 * 1024;
        let dumped = toml::to_string(&s).unwrap();
        assert!(dumped.contains(r#"mode = "fixed""#), "{dumped}");
        assert!(
            dumped.contains("fixed-bytes-per-second = 4194304"),
            "{dumped}"
        );
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

    #[test]
    fn remotes_round_trip_preserves_config() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let path = tmp.path().join("settings.toml");
        let mut s = Settings::default();
        s.remotes.backends.push(BackendConfigEntry {
            name: "prod-s3".into(),
            kind: BackendKindChoice::S3,
            s3: Some(S3BackendConfig {
                bucket: "my-bucket".into(),
                region: "us-east-1".into(),
                endpoint: String::new(),
                root: "archive/".into(),
            }),
            ..Default::default()
        });
        s.save_to(&path).unwrap();
        let back = Settings::load_from(&path).unwrap();
        assert_eq!(back.remotes, s.remotes);
    }
}
