//! Phase 46.6 — Settings → Plugins IPC layer.
//!
//! The runtime itself (`copythat-plugin`) is concerned with one thing:
//! dispatching a hook against a compiled WASM module under per-call
//! sandbox budgets + capability gating. *Where* plugins live on disk,
//! *how* the user installs them, and *which* capabilities they've
//! been granted is the Tauri shell's problem — that's what this
//! module owns.
//!
//! # Storage layout
//!
//! Plugins live under `<config_dir>/plugins/<plugin-name>/`:
//!
//! ```text
//! <config_dir>/plugins/
//!   organize-by-exif/
//!     plugin.toml      ← manifest, parsed by copythat_plugin::PluginManifest
//!     plugin.wasm      ← compiled module the runtime loads
//!     state.toml       ← per-plugin user state (enabled + granted caps)
//!   notify-discord/
//!     plugin.toml
//!     plugin.wasm
//!     state.toml
//! ```
//!
//! The directory name is the plugin's `name` field from `plugin.toml`;
//! that means installing two plugins with the same name overwrites
//! the older copy, which is the same semantics most extension stores
//! ship.
//!
//! # State file
//!
//! `state.toml` carries only what the manifest can't:
//!
//! ```toml
//! enabled = true
//! granted_capabilities = ["read_fs:source", "write_fs:dest"]
//! ```
//!
//! On a fresh install the file is created with `enabled = false`
//! and an empty grant list, so a freshly-installed plugin is inert
//! until the user explicitly enables it + ticks each capability the
//! manifest declares.
//!
//! # Install-from-URL contract
//!
//! `plugin_install_from_url` is two-phase:
//!
//! 1. **Preview** — caller passes `expected_hash = None`. The host
//!    downloads the .wasm + plugin.toml, parses + validates the
//!    manifest, computes the BLAKE3 *bundle* hash (a domain-separated
//!    digest covering the wasm bytes AND the manifest bytes — see
//!    [`bundle_hash`]), and returns a `PluginInstallPreviewDto`
//!    *without* writing to the plugin store. The frontend renders
//!    `name` / `version` / `hash` / `capabilities` so the user can
//!    decide whether to proceed.
//! 2. **Commit** — caller passes the same args plus
//!    `expected_hash = Some(<hex from preview>)`. The host
//!    re-downloads, recomputes the bundle hash, fails if the actual
//!    hash no longer matches the pinned value (fetch races / mutated
//!    upstream artefacts; **including manifest tampering** that
//!    would otherwise let an attacker swap the destination plugin
//!    name or escalate the declared capability set between phases)
//!    and on match copies the staged files into the plugin store.
//!
//! Hash-pinning across two distinct fetches gives the gate teeth —
//! the user's "yes, install this" decision applies to a specific
//! (wasm, manifest) pair, not to whatever content happens to be at
//! the URLs when the install command runs.
//!
//! Only `https://` URLs are accepted (HTTP would let a network MITM
//! defeat the integrity guarantee on first preview). Redirects are
//! capped at five hops.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use copythat_plugin::{Capability, PluginManifest};
use serde::{Deserialize, Serialize};

use crate::ipc_safety;

/// Subdirectory under the OS config dir where plugins live. Mirrors
/// the literal in [`default_plugins_root`] — kept as a named constant
/// so tests can reference it without a stringly-typed path.
pub const PLUGINS_SUBDIR: &str = "plugins";

/// Filename of the parsed manifest inside a plugin directory.
const MANIFEST_FILENAME: &str = "plugin.toml";

/// Filename the host writes the .wasm bytes to inside a plugin
/// directory. The filename is fixed (rather than mirroring the source
/// path's basename) so [`load_handle`] can find the module without
/// re-reading the manifest first.
const WASM_FILENAME: &str = "plugin.wasm";

/// Filename of the per-plugin user-state TOML.
const STATE_FILENAME: &str = "state.toml";

/// Hard cap on the size of a downloaded .wasm body in
/// [`plugin_install_from_url`]. 64 MiB is well past every sample
/// plugin shipped by 46.5 (the largest is ~250 KiB) but small enough
/// that a hostile URL can't OOM the host while we hash it. The
/// runtime's per-call linear-memory cap is independent of this — it
/// constrains how much memory a *running* plugin can grow into, not
/// how big the binary on disk is.
const MAX_WASM_BYTES: usize = 64 * 1024 * 1024;

/// Hard cap on the size of a downloaded `plugin.toml`. The TOML
/// parser handles arbitrarily large files, but a 64 KiB cap keeps a
/// hostile URL from streaming a multi-GB body that we'd then hand to
/// the parser.
const MAX_MANIFEST_BYTES: usize = 64 * 1024;

/// HTTP client timeout for `plugin_install_from_url`. 30 s is long
/// enough for slow links + large wasm bodies but short enough that a
/// hung URL doesn't permanently wedge the Settings → Plugins panel.
const HTTP_TIMEOUT_SECS: u64 = 30;

/// Hard cap on follow-redirect hops for the install fetch. Five is
/// generous for legitimate CDN routing and short enough that a
/// hostile chain can't fan out into arbitrary internal hosts.
const HTTP_REDIRECT_LIMIT: usize = 5;

/// Domain-separation prefix for [`bundle_hash`]. Bumping the version
/// suffix invalidates every previously-pinned hash; do not change
/// the byte string without coordinating with the Settings UI.
const BUNDLE_HASH_DOMAIN: &[u8] = b"copythat-plugin-bundle-v1\x00";

// ---------------------------------------------------------------------------
// Wire DTOs
// ---------------------------------------------------------------------------

/// One entry in the `plugin_list` response. Mirrors the on-disk
/// layout: every field is either parsed out of `plugin.toml` (name,
/// version, hooks, manifest_capabilities) or read from `state.toml`
/// (enabled, granted_capabilities).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginEntryDto {
    /// Plugin name from the manifest. Also the directory name under
    /// `<config_dir>/plugins/`.
    pub name: String,
    /// Semver-shaped version string from the manifest, surfaced
    /// verbatim — the runtime never compares versions, so we don't
    /// either.
    pub version: String,
    /// Lifecycle hooks the manifest declares. Renders next to the
    /// plugin name so the user sees "fires on after_file" without
    /// digging into the TOML.
    pub hooks: Vec<String>,
    /// Capabilities the manifest *requests*. The Settings UI renders
    /// one row per entry with a toggle bound to `granted_capabilities`.
    pub manifest_capabilities: Vec<String>,
    /// Capabilities the user has approved. Subset of
    /// `manifest_capabilities`; an entry not in the manifest is
    /// stripped on read so a stale state.toml can't smuggle in a
    /// permission the manifest never asked for.
    pub granted_capabilities: Vec<String>,
    /// `true` iff the user has enabled the plugin. Fresh installs
    /// land at `false` so a plugin doesn't go live without an
    /// explicit user gesture.
    pub enabled: bool,
    /// Absolute path to the directory the plugin lives in. Surfaced
    /// for the "open in file manager" affordance the Settings UI
    /// exposes.
    pub directory: String,
}

/// Argument bundle for [`plugin_install_from_file`]. Both fields are
/// absolute paths. `manifest_path` is optional — when omitted the
/// host reads `plugin.toml` from the directory next to the wasm
/// (matching `PluginHost::load_plugin`'s on-disk contract).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginInstallFromFileArgs {
    pub wasm_path: String,
    #[serde(default)]
    pub manifest_path: Option<String>,
}

/// Argument bundle for [`plugin_install_from_url`]. `manifest_url`
/// is required (we never *guess* one — a separate URL keeps the
/// staging path explicit + lets the user host the .wasm and the
/// .toml at different origins). `expected_hash` is the BLAKE3 hex
/// from a prior preview call; absent for the preview phase.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginInstallFromUrlArgs {
    pub wasm_url: String,
    pub manifest_url: String,
    #[serde(default)]
    pub expected_hash: Option<String>,
}

/// Response shape for both phases of [`plugin_install_from_url`].
/// `installed` is `false` on the preview call (when `expected_hash`
/// was `None`), `true` once the artefact has been written into the
/// plugin store.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginInstallPreviewDto {
    pub name: String,
    pub version: String,
    /// BLAKE3 hex digest of the downloaded .wasm bytes. The frontend
    /// echoes this back as `expected_hash` to commit; the host
    /// re-hashes and refuses the install if the value drifted.
    pub hash: String,
    pub hooks: Vec<String>,
    /// Capabilities the manifest declares — surfaced so the
    /// confirmation dialog can render them next to the hash.
    pub capabilities: Vec<String>,
    pub installed: bool,
}

// ---------------------------------------------------------------------------
// On-disk state TOML
// ---------------------------------------------------------------------------

/// Round-trip shape for `state.toml`. Kept private; the IPC layer
/// always converts to/from [`PluginEntryDto`] before crossing the
/// command boundary.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct PluginState {
    #[serde(default)]
    enabled: bool,
    #[serde(default)]
    granted_capabilities: Vec<String>,
}

impl PluginState {
    fn load(path: &Path) -> Self {
        match std::fs::read_to_string(path) {
            Ok(s) => toml::from_str(&s).unwrap_or_default(),
            Err(_) => PluginState::default(),
        }
    }

    fn save(&self, path: &Path) -> Result<(), String> {
        let s = toml::to_string(self).map_err(|e| format!("serialize state.toml: {e}"))?;
        atomic_write(path, s.as_bytes()).map_err(|e| format!("write state.toml: {e}"))
    }
}

// ---------------------------------------------------------------------------
// Path helpers
// ---------------------------------------------------------------------------

/// Resolve `<config_dir>/copythat/plugins`. Mirrors the resolver
/// used by [`crate::dropstack::default_dropstack_path`] so the user
/// finds plugins under the same `com.CopyThat.CopyThat2026` config
/// root as `settings.toml` and `dropstack.json`.
pub fn default_plugins_root() -> Option<PathBuf> {
    directories::ProjectDirs::from("com", "CopyThat", "CopyThat2026")
        .map(|d| d.config_dir().join(PLUGINS_SUBDIR))
}

/// Case-insensitive Windows reserved device names. A directory whose
/// last segment matches one of these (with or without an extension)
/// resolves to a console / printer / serial device on Windows
/// regardless of where it sits in the filesystem tree, so we reject
/// the manifest's `name` ever taking one of these values.
const WINDOWS_RESERVED_DEVICE_NAMES: &[&str] = &[
    "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
    "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
];

/// Validate a name as safe to use as a directory name on every
/// supported OS.
///
/// Rejection rules (any failure returns a descriptive error string):
/// - empty / over 128 chars
/// - leading or trailing whitespace (so `"  notify-discord"` can't
///   masquerade as `"notify-discord"` to the user)
/// - leading or trailing `.` (Windows trims trailing `.`; leading
///   `.` makes the directory hidden on Unix without the manifest
///   author saying so)
/// - reserved tokens `.` / `..`
/// - path separators + Windows-illegal characters
/// - control characters / NUL
/// - Windows reserved device names (CON / PRN / AUX / NUL / COM1-9
///   / LPT1-9), case-insensitive, with or without extension
///
/// The manifest's own `name` validation is permissive (it only
/// rejects empty + whitespace), so we tighten at the filesystem
/// boundary where the name becomes a directory under
/// `<config_dir>/plugins/`.
fn validate_plugin_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("plugin name must not be empty".into());
    }
    if name.len() > 128 {
        return Err("plugin name too long (max 128 chars)".into());
    }
    if name == "." || name == ".." {
        return Err("plugin name reserved".into());
    }
    if name.starts_with(char::is_whitespace) || name.ends_with(char::is_whitespace) {
        return Err("plugin name must not start or end with whitespace".into());
    }
    if name.starts_with('.') || name.ends_with('.') {
        return Err("plugin name must not start or end with `.`".into());
    }
    for c in name.chars() {
        // Reject path separators + characters Windows refuses in
        // filenames. The strict subset is intentional — friendly
        // names like "organize-by-exif" or "send_to.discord" still
        // pass, while `..` traversal or `:hidden:streams` are blocked.
        if matches!(
            c,
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' | '\0'
        ) {
            return Err(format!("plugin name contains illegal character: {c:?}"));
        }
        if c.is_control() {
            return Err("plugin name contains control character".into());
        }
    }
    let stem = name.split_once('.').map(|(s, _)| s).unwrap_or(name);
    let stem_upper = stem.to_ascii_uppercase();
    if WINDOWS_RESERVED_DEVICE_NAMES
        .iter()
        .any(|reserved| reserved.eq_ignore_ascii_case(&stem_upper))
    {
        return Err(format!(
            "plugin name `{name}` matches a Windows reserved device name"
        ));
    }
    Ok(())
}

/// Reject install URLs that aren't `https://`. HTTP would let a
/// network MITM swap the plugin payload before the user ever sees
/// the preview hash, defeating the bundle-hash gate on the very
/// first round-trip.
fn validate_install_url(raw: &str) -> Result<(), String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err("install URL must not be empty".into());
    }
    if !trimmed.starts_with("https://") {
        return Err(format!(
            "install URL must use `https://` scheme (got `{trimmed}`)"
        ));
    }
    Ok(())
}

/// Compose `<root>/<name>` after running [`validate_plugin_name`].
/// Returned path is absolute iff `root` is.
fn plugin_dir(root: &Path, name: &str) -> Result<PathBuf, String> {
    validate_plugin_name(name)?;
    Ok(root.join(name))
}

fn manifest_path(plugin_dir: &Path) -> PathBuf {
    plugin_dir.join(MANIFEST_FILENAME)
}

fn wasm_path(plugin_dir: &Path) -> PathBuf {
    plugin_dir.join(WASM_FILENAME)
}

fn state_path(plugin_dir: &Path) -> PathBuf {
    plugin_dir.join(STATE_FILENAME)
}

/// Atomic file write — stage to `<path>.tmp`, then rename. Avoids the
/// half-written-state.toml class of corruption if the host crashes
/// mid-save.
fn atomic_write(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let tmp = path.with_extension(match path.extension() {
        Some(e) => format!("{}.tmp", e.to_string_lossy()),
        None => "tmp".into(),
    });
    std::fs::write(&tmp, bytes)?;
    std::fs::rename(&tmp, path)
}

// ---------------------------------------------------------------------------
// Core helpers (sync — used by both IPC commands and the smoke test)
// ---------------------------------------------------------------------------

/// Read one plugin directory into a `PluginEntryDto`. Returns `Err`
/// when the manifest is missing, malformed, or the directory layout
/// is incomplete (no `.wasm`); the IPC layer surfaces those as
/// per-entry skips so one bad plugin doesn't blank the whole list.
pub fn read_entry(plugin_dir: &Path) -> Result<PluginEntryDto, String> {
    let manifest_src = std::fs::read_to_string(manifest_path(plugin_dir))
        .map_err(|e| format!("read manifest: {e}"))?;
    let manifest =
        PluginManifest::parse(&manifest_src).map_err(|e| format!("parse manifest: {e}"))?;
    let wasm = wasm_path(plugin_dir);
    if !wasm.is_file() {
        return Err(format!(
            "plugin.wasm missing at {}",
            wasm.display()
        ));
    }
    let state = PluginState::load(&state_path(plugin_dir));

    let manifest_capabilities: Vec<String> =
        manifest.capabilities.iter().map(Capability::as_str).collect();
    // Strip stale grants — if the manifest no longer requests a
    // capability, scrub it from the active grant so a downgrade can't
    // leave a phantom permission live.
    let granted_capabilities: Vec<String> = state
        .granted_capabilities
        .into_iter()
        .filter(|cap| manifest_capabilities.contains(cap))
        .collect();

    Ok(PluginEntryDto {
        name: manifest.name,
        version: manifest.version,
        hooks: manifest
            .hooks
            .iter()
            .map(|h| hook_kind_str(*h).to_owned())
            .collect(),
        manifest_capabilities,
        granted_capabilities,
        enabled: state.enabled,
        directory: plugin_dir.to_string_lossy().into_owned(),
    })
}

fn hook_kind_str(h: copythat_plugin::HookKind) -> &'static str {
    use copythat_plugin::HookKind::*;
    match h {
        BeforeJob => "before_job",
        BeforeFile => "before_file",
        AfterFile => "after_file",
        AfterJob => "after_job",
        OnError => "on_error",
    }
}

/// Enumerate every plugin under `root`. Missing root → empty list
/// (a fresh install has nothing to enumerate); per-entry parse
/// failures are skipped with a `tracing::warn!` so a single malformed
/// plugin doesn't make the whole panel error out.
pub fn list_entries(root: &Path) -> Vec<PluginEntryDto> {
    let mut out = Vec::new();
    let dir = match std::fs::read_dir(root) {
        Ok(d) => d,
        Err(_) => return out,
    };
    for entry in dir.flatten() {
        if !entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
            continue;
        }
        match read_entry(&entry.path()) {
            Ok(dto) => out.push(dto),
            Err(e) => {
                tracing::warn!(
                    target: "copythat::plugins",
                    path = %entry.path().display(),
                    error = %e,
                    "skipping malformed plugin directory"
                );
            }
        }
    }
    // Stable order so the UI doesn't re-shuffle on every refresh.
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}

/// Toggle the `enabled` flag for one plugin. Returns the refreshed
/// entry so the frontend doesn't need a follow-up `plugin_list`.
pub fn set_enabled(root: &Path, name: &str, enabled: bool) -> Result<PluginEntryDto, String> {
    let dir = plugin_dir(root, name)?;
    let mut state = PluginState::load(&state_path(&dir));
    state.enabled = enabled;
    state.save(&state_path(&dir))?;
    read_entry(&dir)
}

/// Add a capability to the per-plugin grant. Idempotent — granting an
/// already-granted capability is a no-op. Refuses capabilities the
/// manifest doesn't declare so the UI can't smuggle a permission in
/// past the user's review.
pub fn grant(root: &Path, name: &str, capability: &str) -> Result<PluginEntryDto, String> {
    let dir = plugin_dir(root, name)?;
    let manifest_src = std::fs::read_to_string(manifest_path(&dir))
        .map_err(|e| format!("read manifest: {e}"))?;
    let manifest =
        PluginManifest::parse(&manifest_src).map_err(|e| format!("parse manifest: {e}"))?;
    let parsed_cap =
        Capability::parse(capability).map_err(|e| format!("invalid capability: {e}"))?;
    if !manifest.capabilities.contains(&parsed_cap) {
        return Err(format!(
            "capability `{capability}` is not declared in the manifest"
        ));
    }
    let canonical = parsed_cap.as_str();
    let mut state = PluginState::load(&state_path(&dir));
    if !state.granted_capabilities.iter().any(|c| c == &canonical) {
        state.granted_capabilities.push(canonical);
    }
    state.save(&state_path(&dir))?;
    read_entry(&dir)
}

/// Remove a capability from the per-plugin grant. Idempotent — a
/// missing capability is a no-op.
pub fn revoke(root: &Path, name: &str, capability: &str) -> Result<PluginEntryDto, String> {
    let dir = plugin_dir(root, name)?;
    let canonical = Capability::parse(capability)
        .map(|c| c.as_str())
        .unwrap_or_else(|_| capability.to_owned());
    let mut state = PluginState::load(&state_path(&dir));
    state.granted_capabilities.retain(|c| c != &canonical);
    state.save(&state_path(&dir))?;
    read_entry(&dir)
}

/// Copy a wasm + manifest pair into the plugin store. Either source
/// path may live on a different volume than `<config_dir>` so we
/// always read into memory then write fresh rather than `rename`.
/// Existing entries with the same name get *overwritten* — installing
/// a new version of an already-installed plugin is the dominant
/// upgrade path. State-toml handling matches [`install_from_bytes`]:
/// the enable bit survives a same-capabilities reinstall, the grant
/// list is reset on a capability-set change so the user must
/// re-approve.
pub fn install_from_disk(
    root: &Path,
    wasm_src: &Path,
    manifest_src: &Path,
) -> Result<PluginEntryDto, String> {
    let manifest_text = std::fs::read_to_string(manifest_src)
        .map_err(|e| format!("read manifest from disk: {e}"))?;
    let wasm_bytes = std::fs::read(wasm_src).map_err(|e| format!("read wasm from disk: {e}"))?;
    install_from_bytes(root, &wasm_bytes, &manifest_text)
}

/// Install a plugin straight from in-memory bytes — used by both
/// [`install_from_disk`] (after reading from disk) and
/// [`plugin_install_from_url`] (after the bundle-hash check).
///
/// State-toml policy on overwrite:
/// - Existing dir + identical manifest capability set → preserve
///   `state.toml` (legitimate same-version reinstall keeps the
///   user's prior grants + enable bit).
/// - Existing dir + different manifest capability set → reset
///   `granted_capabilities` to the empty list so the user must
///   re-approve under the new manifest. The `enabled` bit is
///   preserved; a deliberate disable-on-update would surprise
///   users running auto-update flows.
/// - Fresh install → write a default `state.toml` (disabled, no
///   grants).
pub fn install_from_bytes(
    root: &Path,
    wasm_bytes: &[u8],
    manifest_text: &str,
) -> Result<PluginEntryDto, String> {
    if wasm_bytes.len() > MAX_WASM_BYTES {
        return Err(format!(
            "wasm too large ({} bytes; cap is {})",
            wasm_bytes.len(),
            MAX_WASM_BYTES
        ));
    }
    let new_manifest = PluginManifest::parse(manifest_text)
        .map_err(|e| format!("validate manifest: {e}"))?;
    let dir = plugin_dir(root, &new_manifest.name)?;

    // Capture the existing manifest's capability set before we write
    // the new one, so we can decide whether the user's existing
    // grants are still valid under the new manifest.
    let existing_capabilities: Option<BTreeSet<String>> = if dir.exists() {
        std::fs::read_to_string(manifest_path(&dir))
            .ok()
            .and_then(|s| PluginManifest::parse(&s).ok())
            .map(|m| {
                m.capabilities
                    .iter()
                    .map(Capability::as_str)
                    .collect::<BTreeSet<String>>()
            })
    } else {
        None
    };

    std::fs::create_dir_all(&dir).map_err(|e| format!("create plugin dir: {e}"))?;
    atomic_write(&wasm_path(&dir), wasm_bytes)
        .map_err(|e| format!("write wasm into store: {e}"))?;
    atomic_write(&manifest_path(&dir), manifest_text.as_bytes())
        .map_err(|e| format!("write manifest into store: {e}"))?;

    let st_path = state_path(&dir);
    let new_capabilities: BTreeSet<String> = new_manifest
        .capabilities
        .iter()
        .map(Capability::as_str)
        .collect();
    let identity_changed = existing_capabilities
        .as_ref()
        .map(|prev| prev != &new_capabilities)
        .unwrap_or(false);

    if identity_changed {
        // Preserve the enable bit but force the user to re-approve
        // capabilities under the new manifest. `read_entry`'s strip
        // pass already filters grants outside the new manifest set,
        // but explicitly resetting here means the on-disk state
        // matches the wire response and a subsequent legitimate
        // grant won't trip on a stale entry.
        let mut state = PluginState::load(&st_path);
        state.granted_capabilities.clear();
        state.save(&st_path)?;
    } else if !st_path.exists() {
        PluginState::default().save(&st_path)?;
    }

    read_entry(&dir)
}

/// Compute the BLAKE3 hex digest of `bytes`. Used for the legacy
/// per-bytes digest exposed via the smoke harness; the install
/// pipeline pins on [`bundle_hash`] (which folds the manifest into
/// the same digest) so manifest tampering between preview and commit
/// can't smuggle a different plugin past the user's confirmation.
pub fn blake3_hex(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}

/// Compute the BLAKE3 hex digest of the (wasm, manifest) install
/// bundle. Domain-separated and length-prefixed so neither component
/// can be shifted into the other:
///
/// ```text
/// H = BLAKE3("copythat-plugin-bundle-v1\0"
///         || u64_le(wasm_len)     || wasm_bytes
///         || u64_le(manifest_len) || manifest_bytes)
/// ```
///
/// The preview phase of [`plugin_install_from_url`] returns this
/// digest to the user; the commit phase recomputes it and refuses
/// the install on mismatch. Pinning both bytes streams together
/// closes the Phase 46.7 audit's manifest-tampering hole — pre-fix,
/// only the wasm hash was pinned and an attacker could swap the
/// manifest's `name` field between phases to overwrite a different
/// already-installed plugin's directory while inheriting its grants.
pub fn bundle_hash(wasm_bytes: &[u8], manifest_bytes: &[u8]) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(BUNDLE_HASH_DOMAIN);
    hasher.update(&(wasm_bytes.len() as u64).to_le_bytes());
    hasher.update(wasm_bytes);
    hasher.update(&(manifest_bytes.len() as u64).to_le_bytes());
    hasher.update(manifest_bytes);
    hasher.finalize().to_hex().to_string()
}

// ---------------------------------------------------------------------------
// HTTP fetch helper (used only by the URL install command)
// ---------------------------------------------------------------------------

async fn fetch_with_cap(client: &reqwest::Client, url: &str, cap: usize) -> Result<Vec<u8>, String> {
    let resp = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("GET {url}: {e}"))?;
    if !resp.status().is_success() {
        return Err(format!("GET {url}: HTTP {}", resp.status()));
    }
    if let Some(len) = resp.content_length() {
        if len as usize > cap {
            return Err(format!(
                "GET {url}: body too large ({len} bytes; cap is {cap})"
            ));
        }
    }
    let bytes = resp.bytes().await.map_err(|e| format!("read body: {e}"))?;
    if bytes.len() > cap {
        return Err(format!(
            "GET {url}: body too large ({} bytes; cap is {cap})",
            bytes.len()
        ));
    }
    Ok(bytes.to_vec())
}

fn build_client() -> Result<reqwest::Client, String> {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(HTTP_TIMEOUT_SECS))
        .redirect(reqwest::redirect::Policy::limited(HTTP_REDIRECT_LIMIT))
        // `https_only` rejects any request whose final URL (after
        // redirects) is HTTP. Combined with `validate_install_url`
        // checking the *initial* URL is HTTPS, this closes the
        // chain: a hostile redirector can't downgrade to plaintext.
        .https_only(true)
        .build()
        .map_err(|e| format!("build http client: {e}"))
}

// ---------------------------------------------------------------------------
// IPC commands
// ---------------------------------------------------------------------------

/// `plugin_list` — enumerate every plugin in the per-user store.
#[tauri::command]
pub async fn plugin_list() -> Result<Vec<PluginEntryDto>, String> {
    let root = default_plugins_root().ok_or_else(|| "config dir unavailable".to_string())?;
    // `read_dir` + per-entry parse is cheap (a handful of TOML files)
    // but is still synchronous I/O; spawn_blocking keeps the runtime
    // responsive on slow disks.
    tokio::task::spawn_blocking(move || Ok(list_entries(&root)))
        .await
        .map_err(|e| format!("plugin_list join: {e}"))?
}

/// `plugin_enable` — flip a plugin's `enabled` bit on.
#[tauri::command]
pub async fn plugin_enable(name: String) -> Result<PluginEntryDto, String> {
    let root = default_plugins_root().ok_or_else(|| "config dir unavailable".to_string())?;
    tokio::task::spawn_blocking(move || set_enabled(&root, &name, true))
        .await
        .map_err(|e| format!("plugin_enable join: {e}"))?
}

/// `plugin_disable` — flip a plugin's `enabled` bit off.
#[tauri::command]
pub async fn plugin_disable(name: String) -> Result<PluginEntryDto, String> {
    let root = default_plugins_root().ok_or_else(|| "config dir unavailable".to_string())?;
    tokio::task::spawn_blocking(move || set_enabled(&root, &name, false))
        .await
        .map_err(|e| format!("plugin_disable join: {e}"))?
}

/// `plugin_grant_capability` — add a capability to the per-plugin
/// grant. Refuses capabilities the manifest doesn't declare.
#[tauri::command]
pub async fn plugin_grant_capability(
    name: String,
    capability: String,
) -> Result<PluginEntryDto, String> {
    let root = default_plugins_root().ok_or_else(|| "config dir unavailable".to_string())?;
    tokio::task::spawn_blocking(move || grant(&root, &name, &capability))
        .await
        .map_err(|e| format!("plugin_grant_capability join: {e}"))?
}

/// `plugin_revoke_capability` — remove a capability from the
/// per-plugin grant. Idempotent.
#[tauri::command]
pub async fn plugin_revoke_capability(
    name: String,
    capability: String,
) -> Result<PluginEntryDto, String> {
    let root = default_plugins_root().ok_or_else(|| "config dir unavailable".to_string())?;
    tokio::task::spawn_blocking(move || revoke(&root, &name, &capability))
        .await
        .map_err(|e| format!("plugin_revoke_capability join: {e}"))?
}

/// `plugin_install_from_file` — copy a wasm + manifest pair from
/// arbitrary on-disk paths into the plugin store. When
/// `manifest_path` is omitted the host reads `plugin.toml` from the
/// directory next to the wasm (the same on-disk shape the runtime's
/// `PluginHost::load_plugin` expects).
///
/// Both paths flow through [`ipc_safety::validate_ipc_path`] (the
/// same lexical gate every other path-typed IPC command uses) so a
/// frontend XSS / forged invocation can't smuggle a `..`-traversal
/// or NUL-byte path past the install code.
#[tauri::command]
pub async fn plugin_install_from_file(
    args: PluginInstallFromFileArgs,
) -> Result<PluginEntryDto, String> {
    let root = default_plugins_root().ok_or_else(|| "config dir unavailable".to_string())?;
    let wasm = ipc_safety::validate_ipc_path(&args.wasm_path).map_err(ipc_safety::err_string)?;
    let manifest = match args.manifest_path {
        Some(p) => ipc_safety::validate_ipc_path(&p).map_err(ipc_safety::err_string)?,
        None => wasm
            .parent()
            .map(|p| p.join(MANIFEST_FILENAME))
            .ok_or_else(|| "wasm path has no parent directory".to_string())?,
    };
    tokio::task::spawn_blocking(move || install_from_disk(&root, &wasm, &manifest))
        .await
        .map_err(|e| format!("plugin_install_from_file join: {e}"))?
}

/// `plugin_install_from_url` — preview-or-commit URL install.
///
/// See the module docstring for the two-phase contract: a
/// `expected_hash = None` call returns the computed bundle hash +
/// parsed manifest *without* writing to the plugin store; a
/// follow-up call with the same URLs and `expected_hash = Some(<hex>)`
/// re-fetches, verifies the hash, and installs.
#[tauri::command]
pub async fn plugin_install_from_url(
    args: PluginInstallFromUrlArgs,
) -> Result<PluginInstallPreviewDto, String> {
    let root = default_plugins_root().ok_or_else(|| "config dir unavailable".to_string())?;
    validate_install_url(&args.wasm_url)?;
    validate_install_url(&args.manifest_url)?;
    let client = build_client()?;
    let wasm_bytes = fetch_with_cap(&client, &args.wasm_url, MAX_WASM_BYTES).await?;
    let manifest_bytes = fetch_with_cap(&client, &args.manifest_url, MAX_MANIFEST_BYTES).await?;
    let manifest_text = String::from_utf8(manifest_bytes)
        .map_err(|_| "plugin.toml is not valid UTF-8".to_string())?;
    let manifest =
        PluginManifest::parse(&manifest_text).map_err(|e| format!("validate manifest: {e}"))?;

    // Bundle hash covers BOTH the wasm bytes AND the manifest bytes
    // (length-prefixed + domain-separated). The user's preview-phase
    // confirmation pins this digest; the commit-phase recompute
    // catches any drift in either component.
    let actual_hash = bundle_hash(&wasm_bytes, manifest_text.as_bytes());
    let capabilities: Vec<String> =
        manifest.capabilities.iter().map(Capability::as_str).collect();
    let hooks: Vec<String> = manifest
        .hooks
        .iter()
        .map(|h| hook_kind_str(*h).to_owned())
        .collect();

    if let Some(expected) = args.expected_hash.as_deref() {
        // The hash comes back from the user's own confirmation
        // dialog (not an adversary-controlled channel), so a naive
        // `==` is fine — but we lowercase both sides so the UI can
        // echo our hex in any case without a spurious mismatch.
        let actual_lc = actual_hash.to_ascii_lowercase();
        let expected_lc = expected.to_ascii_lowercase();
        if actual_lc != expected_lc {
            return Err(format!(
                "hash mismatch: expected `{expected_lc}`, got `{actual_lc}`"
            ));
        }
        // Commit phase — write into the plugin store. Sync I/O
        // (atomic writes + state.toml read/write) goes through
        // `spawn_blocking` so the runtime stays responsive while
        // the (up to 64 MiB) wasm hits disk.
        let install_root = root.clone();
        let install_text = manifest_text.clone();
        let install_bytes = wasm_bytes.clone();
        let entry = tokio::task::spawn_blocking(move || {
            install_from_bytes(&install_root, &install_bytes, &install_text)
        })
        .await
        .map_err(|e| format!("plugin_install_from_url commit join: {e}"))??;
        return Ok(PluginInstallPreviewDto {
            name: entry.name,
            version: entry.version,
            hash: actual_hash,
            hooks,
            capabilities,
            installed: true,
        });
    }

    // Preview phase — return the parsed metadata without touching the
    // plugin store. The frontend renders this in the confirmation
    // dialog before deciding whether to commit.
    Ok(PluginInstallPreviewDto {
        name: manifest.name,
        version: manifest.version,
        hash: actual_hash,
        hooks,
        capabilities,
        installed: false,
    })
}

// ---------------------------------------------------------------------------
// Tests — exercise the sync core helpers without booting Tauri.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const SAMPLE_MANIFEST: &str = r#"
name = "exif-rename"
version = "0.1.0"
hooks = ["after_file"]
capabilities = ["read_fs:source", "write_fs:dest"]
"#;

    /// Write a minimal valid pair (manifest + 4-byte stub wasm) into
    /// `<root>/<name>/`. Tests use this to seed the store without
    /// crossing the IPC / HTTP boundary.
    fn seed(root: &Path, name: &str, manifest_src: &str) {
        let dir = root.join(name);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join(MANIFEST_FILENAME), manifest_src).unwrap();
        // 4-byte WASM magic. The IPC layer never instantiates the
        // module — only the runtime does — so the bytes need only be
        // *present*, not *valid*, for these tests.
        fs::write(dir.join(WASM_FILENAME), b"\0asm").unwrap();
    }

    #[test]
    fn list_entries_is_empty_when_root_missing() {
        let tmp = tempfile::tempdir().unwrap();
        let entries = list_entries(&tmp.path().join("does-not-exist"));
        assert!(entries.is_empty());
    }

    #[test]
    fn read_entry_round_trips_manifest_fields() {
        let tmp = tempfile::tempdir().unwrap();
        seed(tmp.path(), "exif-rename", SAMPLE_MANIFEST);
        let dto = read_entry(&tmp.path().join("exif-rename")).unwrap();
        assert_eq!(dto.name, "exif-rename");
        assert_eq!(dto.version, "0.1.0");
        assert_eq!(dto.hooks, vec!["after_file"]);
        assert_eq!(
            dto.manifest_capabilities,
            vec!["read_fs:source", "write_fs:dest"]
        );
        assert!(dto.granted_capabilities.is_empty());
        assert!(!dto.enabled);
    }

    #[test]
    fn enable_disable_round_trips_through_state_toml() {
        let tmp = tempfile::tempdir().unwrap();
        seed(tmp.path(), "exif-rename", SAMPLE_MANIFEST);
        let dto = set_enabled(tmp.path(), "exif-rename", true).unwrap();
        assert!(dto.enabled);
        let dto = set_enabled(tmp.path(), "exif-rename", false).unwrap();
        assert!(!dto.enabled);
    }

    #[test]
    fn grant_then_revoke_clears_capability() {
        let tmp = tempfile::tempdir().unwrap();
        seed(tmp.path(), "exif-rename", SAMPLE_MANIFEST);
        let dto = grant(tmp.path(), "exif-rename", "read_fs:source").unwrap();
        assert_eq!(dto.granted_capabilities, vec!["read_fs:source"]);
        // Idempotent — granting the same cap twice is a no-op.
        let dto = grant(tmp.path(), "exif-rename", "read_fs:source").unwrap();
        assert_eq!(dto.granted_capabilities, vec!["read_fs:source"]);
        let dto = revoke(tmp.path(), "exif-rename", "read_fs:source").unwrap();
        assert!(dto.granted_capabilities.is_empty());
    }

    #[test]
    fn grant_refuses_capability_not_in_manifest() {
        let tmp = tempfile::tempdir().unwrap();
        seed(tmp.path(), "exif-rename", SAMPLE_MANIFEST);
        let err = grant(tmp.path(), "exif-rename", "network").unwrap_err();
        assert!(err.contains("not declared in the manifest"), "{err}");
    }

    #[test]
    fn grant_refuses_unparseable_capability() {
        let tmp = tempfile::tempdir().unwrap();
        seed(tmp.path(), "exif-rename", SAMPLE_MANIFEST);
        let err = grant(tmp.path(), "exif-rename", "bogus_cap").unwrap_err();
        assert!(err.contains("invalid capability"), "{err}");
    }

    #[test]
    fn read_entry_strips_stale_grants_outside_manifest() {
        let tmp = tempfile::tempdir().unwrap();
        seed(tmp.path(), "exif-rename", SAMPLE_MANIFEST);
        // Hand-write a state.toml that grants a capability the manifest
        // never declared (simulates a manifest downgrade between runs).
        let state = PluginState {
            enabled: true,
            granted_capabilities: vec![
                "read_fs:source".into(),
                "network".into(), // not in the sample manifest
            ],
        };
        let st_path = tmp.path().join("exif-rename").join(STATE_FILENAME);
        state.save(&st_path).unwrap();
        let dto = read_entry(&tmp.path().join("exif-rename")).unwrap();
        assert_eq!(dto.granted_capabilities, vec!["read_fs:source"]);
    }

    #[test]
    fn install_from_disk_copies_files_and_creates_state() {
        let tmp = tempfile::tempdir().unwrap();
        let src_dir = tmp.path().join("src");
        let store = tmp.path().join("store");
        fs::create_dir_all(&src_dir).unwrap();
        fs::write(src_dir.join("plugin.toml"), SAMPLE_MANIFEST).unwrap();
        fs::write(src_dir.join("plugin.wasm"), b"\0asm").unwrap();

        let dto = install_from_disk(
            &store,
            &src_dir.join("plugin.wasm"),
            &src_dir.join("plugin.toml"),
        )
        .unwrap();
        assert_eq!(dto.name, "exif-rename");
        assert!(!dto.enabled, "fresh install must default to disabled");
        assert!(store.join("exif-rename").join("plugin.wasm").is_file());
        assert!(store.join("exif-rename").join("plugin.toml").is_file());
        assert!(store.join("exif-rename").join("state.toml").is_file());
    }

    #[test]
    fn install_from_disk_preserves_state_on_reinstall() {
        let tmp = tempfile::tempdir().unwrap();
        let src_dir = tmp.path().join("src");
        let store = tmp.path().join("store");
        fs::create_dir_all(&src_dir).unwrap();
        fs::write(src_dir.join("plugin.toml"), SAMPLE_MANIFEST).unwrap();
        fs::write(src_dir.join("plugin.wasm"), b"\0asm").unwrap();
        install_from_disk(
            &store,
            &src_dir.join("plugin.wasm"),
            &src_dir.join("plugin.toml"),
        )
        .unwrap();
        // Enable + grant, then reinstall — state must survive.
        set_enabled(&store, "exif-rename", true).unwrap();
        grant(&store, "exif-rename", "read_fs:source").unwrap();
        install_from_disk(
            &store,
            &src_dir.join("plugin.wasm"),
            &src_dir.join("plugin.toml"),
        )
        .unwrap();
        let dto = read_entry(&store.join("exif-rename")).unwrap();
        assert!(dto.enabled, "reinstall must preserve enable bit");
        assert_eq!(dto.granted_capabilities, vec!["read_fs:source"]);
    }

    #[test]
    fn validate_plugin_name_rejects_traversal() {
        assert!(validate_plugin_name("..").is_err());
        assert!(validate_plugin_name(".").is_err());
        assert!(validate_plugin_name("a/b").is_err());
        assert!(validate_plugin_name("a\\b").is_err());
        assert!(validate_plugin_name("").is_err());
        assert!(validate_plugin_name("ok-name_v2").is_ok());
    }

    #[test]
    fn validate_plugin_name_rejects_oversize() {
        let too_long = "x".repeat(129);
        assert!(validate_plugin_name(&too_long).is_err());
        let just_long = "x".repeat(128);
        assert!(validate_plugin_name(&just_long).is_ok());
    }

    #[test]
    fn validate_plugin_name_rejects_nul_and_controls() {
        assert!(validate_plugin_name("a\0b").is_err());
        assert!(validate_plugin_name("a\nb").is_err());
        assert!(validate_plugin_name("a\tb").is_err());
    }

    #[test]
    fn validate_plugin_name_rejects_windows_illegal_chars() {
        for c in ['*', '?', '"', '<', '>', '|', ':'] {
            let n = format!("a{c}b");
            assert!(
                validate_plugin_name(&n).is_err(),
                "expected reject for {c:?}"
            );
        }
    }

    #[test]
    fn validate_plugin_name_rejects_windows_reserved_names() {
        // Bare reserved names — case-insensitive, with or without
        // extension. Pre-46.7 these slipped past `validate_plugin_name`
        // because the rule set only blocked path separators + `..`.
        for name in ["CON", "con", "PRN", "AUX", "NUL", "COM1", "lpt9"] {
            assert!(
                validate_plugin_name(name).is_err(),
                "bare `{name}` must reject"
            );
        }
        // With extension — Windows resolves these to the same device.
        for name in ["CON.wasm", "lpt1.txt"] {
            assert!(
                validate_plugin_name(name).is_err(),
                "`{name}` (with extension) must reject"
            );
        }
        // Names that *contain* a reserved token but aren't equal to
        // it should still pass.
        assert!(validate_plugin_name("connector").is_ok());
        assert!(validate_plugin_name("not-con").is_ok());
        assert!(validate_plugin_name("auxiliary").is_ok());
    }

    #[test]
    fn validate_plugin_name_rejects_leading_trailing_whitespace_and_dot() {
        assert!(validate_plugin_name(" notify-discord").is_err());
        assert!(validate_plugin_name("notify-discord ").is_err());
        assert!(validate_plugin_name(".notify").is_err());
        assert!(validate_plugin_name("notify.").is_err());
    }

    #[test]
    fn validate_install_url_rejects_non_https() {
        assert!(validate_install_url("http://example.com/p.wasm").is_err());
        assert!(validate_install_url("file:///etc/passwd").is_err());
        assert!(validate_install_url("ftp://example.com/p.wasm").is_err());
        assert!(validate_install_url("").is_err());
        assert!(validate_install_url("   ").is_err());
        assert!(validate_install_url("https://example.com/p.wasm").is_ok());
    }

    #[test]
    fn bundle_hash_is_deterministic_and_domain_separated() {
        let h1 = bundle_hash(b"\0asm", b"name = \"a\"");
        let h2 = bundle_hash(b"\0asm", b"name = \"a\"");
        assert_eq!(h1, h2, "bundle hash must be deterministic");
        assert_eq!(h1.len(), 64);
        // A different manifest yields a different digest — this is
        // the entire point of folding the manifest into the hash.
        let h3 = bundle_hash(b"\0asm", b"name = \"b\"");
        assert_ne!(h1, h3);
        // A different wasm yields a different digest.
        let h4 = bundle_hash(b"\0wasm-other", b"name = \"a\"");
        assert_ne!(h1, h4);
        // Domain separation: the bundle hash is NOT equal to the raw
        // BLAKE3 of the concatenated bytes, so a future change of
        // protocol can't be confused with a legacy raw hash.
        let mut concat = Vec::new();
        concat.extend_from_slice(b"\0asm");
        concat.extend_from_slice(b"name = \"a\"");
        let raw = blake3_hex(&concat);
        assert_ne!(h1, raw, "bundle hash must be domain-separated from raw");
    }

    #[test]
    fn bundle_hash_is_unambiguous_under_byte_shift() {
        // Without length-prefixing, splitting the same byte stream
        // at a different point would yield the same hash. With
        // length-prefixing, ("ab", "c") != ("a", "bc").
        let h1 = bundle_hash(b"ab", b"c");
        let h2 = bundle_hash(b"a", b"bc");
        assert_ne!(h1, h2, "length-prefixing must distinguish split points");
    }

    #[test]
    fn install_from_bytes_resets_grants_on_capability_change() {
        let tmp = tempfile::tempdir().unwrap();
        let store = tmp.path();
        // First install with two capabilities, both granted.
        let v1 = SAMPLE_MANIFEST;
        install_from_bytes(store, b"\0asm", v1).unwrap();
        set_enabled(store, "exif-rename", true).unwrap();
        grant(store, "exif-rename", "read_fs:source").unwrap();
        grant(store, "exif-rename", "write_fs:dest").unwrap();
        let pre = read_entry(&store.join("exif-rename")).unwrap();
        assert_eq!(pre.granted_capabilities.len(), 2);
        assert!(pre.enabled);

        // Reinstall with a manifest that drops one capability and
        // adds a new one. The grant set must reset (user must
        // re-approve under the new manifest); the enable bit must
        // survive (deliberate disable-on-update would surprise
        // auto-update flows).
        let v2 = r#"
name = "exif-rename"
version = "0.2.0"
hooks = ["after_file"]
capabilities = ["network", "read_fs:source"]
"#;
        let dto = install_from_bytes(store, b"\0asm-v2", v2).unwrap();
        assert!(dto.enabled, "enable bit must survive capability change");
        assert!(
            dto.granted_capabilities.is_empty(),
            "grants must reset on capability set change, got: {:?}",
            dto.granted_capabilities
        );
    }

    #[test]
    fn install_from_bytes_preserves_grants_when_capabilities_unchanged() {
        let tmp = tempfile::tempdir().unwrap();
        let store = tmp.path();
        install_from_bytes(store, b"\0asm", SAMPLE_MANIFEST).unwrap();
        set_enabled(store, "exif-rename", true).unwrap();
        grant(store, "exif-rename", "read_fs:source").unwrap();

        // Same capability set, new wasm bytes — a vanilla
        // version-bump-with-no-cap-changes upgrade. The grant must
        // survive.
        let v2 = r#"
name = "exif-rename"
version = "0.2.0"
hooks = ["after_file"]
capabilities = ["read_fs:source", "write_fs:dest"]
"#;
        let dto = install_from_bytes(store, b"\0asm-v2", v2).unwrap();
        assert!(dto.enabled);
        assert_eq!(dto.granted_capabilities, vec!["read_fs:source"]);
    }

    #[test]
    fn list_entries_skips_malformed_plugin_directory() {
        let tmp = tempfile::tempdir().unwrap();
        seed(tmp.path(), "good", SAMPLE_MANIFEST);
        // Create a "plugin dir" with no manifest at all.
        fs::create_dir_all(tmp.path().join("bad")).unwrap();
        let entries = list_entries(tmp.path());
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].name, "exif-rename");
    }

    #[test]
    fn blake3_hex_is_lowercase_hex() {
        let hex = blake3_hex(b"\0asm");
        assert_eq!(hex.len(), 64, "BLAKE3 hex is 32 bytes = 64 hex chars");
        assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(hex.chars().all(|c| !c.is_ascii_uppercase()));
    }
}
