//! Phase 35 — Tauri-side encryption + compression glue.
//!
//! The runner builds a [`copythat_crypt::CopyThatCryptHook`] from
//! `Settings::crypt` ahead of each job enqueue and attaches it to
//! the job's `CopyOptions::transform`. The engine short-circuits to
//! the sink's transform path whenever the hook is present; verify,
//! fast paths, sparse + chunk-store are skipped by construction (see
//! `copythat_core::copy_file` for the exclusion rationale).
//!
//! This module is intentionally slim: the passphrase flow is
//! **deferred** to Phase 35 follow-up. When `encryption_mode ==
//! "passphrase"` the runner logs to stderr and treats the mode as
//! off; the Settings → Encryption UI accepts the passphrase and
//! stores it in the keychain as a Phase 36-ish follow-up.
//! Everything else (compression, recipients-from-file encryption)
//! works end-to-end today.

use std::collections::HashSet;
use std::path::Path;
use std::sync::Arc;

use copythat_crypt::{
    CompressionLevel, CompressionPolicy, CopyThatCryptHook, DEFAULT_DENY_EXTENSIONS,
    EncryptionPolicy, Recipient,
};
use copythat_settings::CryptSettings;

use serde::Serialize;

/// Snapshot of the effective crypt configuration surfaced through
/// IPC. The Settings UI reads this to populate its panel + show a
/// live "active / disabled" badge.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CryptStatusDto {
    /// Encryption will run for new copies (at least one valid
    /// recipient parsed + the mode is not "off").
    pub encryption_active: bool,
    /// Compression will run for new copies (mode != "off").
    pub compression_active: bool,
    /// Parsed recipients list — kinds only, never the key material.
    pub recipient_kinds: Vec<String>,
    /// Size of the effective compression deny-extension set
    /// (built-in + user-supplied extras). Informational, shown next
    /// to the slider so the user sees the blast radius.
    pub deny_extension_count: usize,
    /// Clamped compression level that will run when the policy is
    /// active. Always 1..=22.
    pub effective_level: i32,
}

/// Parse a recipients file — one recipient per non-blank,
/// non-comment line. Lines starting with `#` are ignored.
fn parse_recipients_file(path: &Path) -> Result<Vec<Recipient>, String> {
    let raw = std::fs::read_to_string(path).map_err(|e| format!("read {path:?}: {e}"))?;
    let mut out = Vec::new();
    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if trimmed.starts_with("age1") {
            out.push(Recipient::X25519(trimmed.to_string()));
        } else if trimmed.starts_with("ssh-") {
            out.push(Recipient::Ssh(trimmed.to_string()));
        } else {
            return Err(format!(
                "unrecognised recipient (expected `age1…` or `ssh-…`): `{trimmed}`"
            ));
        }
    }
    Ok(out)
}

/// Build the engine-facing hook from live settings. Returns `None`
/// when neither stage is active. Errors surface as user-legible
/// strings that the Settings UI / runner pipe into a toast.
pub fn build_hook(settings: &CryptSettings) -> Result<Option<Arc<CopyThatCryptHook>>, String> {
    let compression = build_compression_policy(settings);
    let encryption = build_encryption_policy(settings)?;
    if matches!(compression, CompressionPolicy::Off) && encryption.is_none() {
        return Ok(None);
    }
    Ok(Some(Arc::new(CopyThatCryptHook::new(
        compression,
        encryption,
    ))))
}

fn build_compression_policy(settings: &CryptSettings) -> CompressionPolicy {
    let level = CompressionLevel::clamp(settings.compression_level);
    match settings.compression_mode.to_ascii_lowercase().as_str() {
        "always" => CompressionPolicy::Always { level },
        "smart" => {
            // Built-in defaults + any extras the user configured.
            let mut deny: HashSet<String> = DEFAULT_DENY_EXTENSIONS
                .iter()
                .map(|s| (*s).to_string())
                .collect();
            for ext in &settings.compression_extra_deny {
                let trimmed = ext.trim_start_matches('.').to_ascii_lowercase();
                if !trimmed.is_empty() {
                    deny.insert(trimmed);
                }
            }
            CompressionPolicy::SmartByExtension {
                default_level: level,
                deny_extensions: deny,
            }
        }
        _ => CompressionPolicy::Off,
    }
}

fn build_encryption_policy(settings: &CryptSettings) -> Result<Option<EncryptionPolicy>, String> {
    match settings.encryption_mode.to_ascii_lowercase().as_str() {
        "off" | "" => Ok(None),
        "passphrase" => {
            // Passphrase entry is deferred — the UI's future Phase
            // 35-follow-up will prompt the user + stash the
            // secret in the OS keychain. Today we refuse to enable
            // passphrase mode silently; the runner logs and treats
            // encryption as off.
            eprintln!(
                "[crypt] passphrase encryption requires the Settings → Encryption modal to \
                 collect the passphrase at copy-start; treating as off for now"
            );
            Ok(None)
        }
        "recipients" => {
            if settings.recipients_file.trim().is_empty() {
                return Err("recipients mode selected but no recipients file configured".into());
            }
            let recipients = parse_recipients_file(Path::new(&settings.recipients_file))?;
            if recipients.is_empty() {
                return Err("recipients file is empty".into());
            }
            Ok(Some(EncryptionPolicy::strict(recipients)))
        }
        other => Err(format!("unknown encryption mode: `{other}`")),
    }
}

#[tauri::command]
pub fn crypt_status(state: tauri::State<'_, crate::state::AppState>) -> CryptStatusDto {
    let settings = state.settings_snapshot();
    let cfg = &settings.crypt;

    let compression = build_compression_policy(cfg);
    let compression_active = !matches!(compression, CompressionPolicy::Off);

    let (encryption_active, recipient_kinds) = match build_encryption_policy(cfg) {
        Ok(Some(policy)) => {
            let kinds = policy
                .recipients
                .iter()
                .map(|r| r.kind().to_string())
                .collect();
            (true, kinds)
        }
        _ => (false, Vec::new()),
    };

    // Deny-extension count: only meaningful under Smart mode.
    let deny_extension_count = match &compression {
        CompressionPolicy::SmartByExtension {
            deny_extensions, ..
        } => deny_extensions.len(),
        _ => 0,
    };

    CryptStatusDto {
        encryption_active,
        compression_active,
        recipient_kinds,
        deny_extension_count,
        effective_level: CompressionLevel::clamp(cfg.compression_level).as_i32(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_settings_produce_no_hook() {
        let hook = build_hook(&CryptSettings::default()).unwrap();
        assert!(hook.is_none());
    }

    #[test]
    fn always_mode_activates_compression() {
        let cfg = CryptSettings {
            compression_mode: "always".into(),
            compression_level: 5,
            ..CryptSettings::default()
        };
        let hook = build_hook(&cfg).unwrap().expect("hook");
        let plan = hook.will_transform("txt");
        assert_eq!(plan.compression_level_i32, Some(5));
        assert!(!plan.encrypted);
    }

    #[test]
    fn smart_mode_adds_user_extras_to_deny_list() {
        let cfg = CryptSettings {
            compression_mode: "smart".into(),
            compression_extra_deny: vec!["log".into()],
            ..CryptSettings::default()
        };
        let hook = build_hook(&cfg).unwrap().expect("hook");
        let plan = hook.will_transform("log");
        assert!(plan.compression_level_i32.is_none());
    }

    #[test]
    fn unknown_encryption_mode_fails() {
        let cfg = CryptSettings {
            encryption_mode: "magic".into(),
            ..CryptSettings::default()
        };
        let err = build_hook(&cfg).unwrap_err();
        assert!(err.contains("unknown encryption mode"));
    }
}
