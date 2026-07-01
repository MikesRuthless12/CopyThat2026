//! Phase 49k / 50i — repository passphrase ACCESS gate with multiple key slots.
//!
//! This is **not** at-rest encryption — Phase 51 owns that. It stores a
//! `repo-key.json` with one or more *key slots*, each a `scrypt(secret, salt)`
//! verifier, so the UI can do Kopia-style "connect with passphrase":
//! [`verify`] recomputes the KDF for the supplied secret and constant-time
//! compares against every slot — *any* slot grants access. Chunk bytes stay
//! plaintext on disk.
//!
//! Phase 50i adds **multiple slots** (several passphrases can unlock the same
//! repo — one per person / per device) plus a generated **recovery key**
//! (a high-entropy secret shown once; only its verifier is stored). scrypt is
//! reused (already a dep for the restic importer) rather than pulling argon2,
//! so no new crate enters the build.
//!
//! ## On-disk format
//!
//! v2 is `{ "slots": [ { label, kind, kdf, salt, verifier }, … ] }`. A v1
//! file (top-level `kdf`/`salt`/`verifier`, no `slots`) is transparently
//! migrated on read into a single `"primary"` password slot, so repos created
//! before 50i keep working with no rewrite.

use std::path::{Path, PathBuf};

use base64::Engine;
use serde::{Deserialize, Serialize};

use crate::error::{ChunkStoreError, Result};

const KEYFILE: &str = "repo-key.json";
const SALT_LEN: usize = 16;
const VERIFIER_LEN: usize = 32;
// scrypt log_n=15 (32 MiB), r=8, p=1 — a one-time cost on repo open/connect.
const SCRYPT_LOG_N: u8 = 15;
const SCRYPT_R: u32 = 8;
const SCRYPT_P: u32 = 1;

/// Slot kind for a password the user typed.
const KIND_PASSWORD: &str = "password";
/// Slot kind for a generated recovery key.
const KIND_RECOVERY: &str = "recovery";
/// Label of the slot created by [`write`] / the v1 migration.
const PRIMARY_LABEL: &str = "primary";

/// One access credential: a labelled `scrypt` verifier.
#[derive(Clone, Serialize, Deserialize)]
struct KeySlot {
    label: String,
    kind: String,
    kdf: String,
    salt: String,     // base64
    verifier: String, // base64
}

/// On-disk key file. v2 writes only `slots`; a v1 file (top-level
/// `kdf`/`salt`/`verifier`) is migrated on read (see [`load`]).
#[derive(Default, Serialize, Deserialize)]
struct RepoKeyFile {
    #[serde(default)]
    slots: Vec<KeySlot>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    kdf: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    salt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    verifier: Option<String>,
}

/// Public view of a key slot (no verifier bytes) for the settings UI.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeySlotInfo {
    /// Human label ("primary", "laptop", "recovery", …).
    pub label: String,
    /// `"password"` or `"recovery"`.
    pub kind: String,
}

fn keyfile_path(root: &Path) -> PathBuf {
    root.join(KEYFILE)
}

/// True if a passphrase verifier exists at `root`.
pub(crate) fn exists(root: &Path) -> bool {
    keyfile_path(root).is_file()
}

fn derive(secret: &str, salt: &[u8]) -> Result<[u8; VERIFIER_LEN]> {
    let params = scrypt::Params::new(SCRYPT_LOG_N, SCRYPT_R, SCRYPT_P, VERIFIER_LEN)
        .map_err(|e| ChunkStoreError::Redb(format!("scrypt params: {e}")))?;
    let mut out = [0u8; VERIFIER_LEN];
    scrypt::scrypt(secret.as_bytes(), salt, &params, &mut out)
        .map_err(|e| ChunkStoreError::Redb(format!("scrypt: {e}")))?;
    Ok(out)
}

fn ct_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b) {
        diff |= x ^ y;
    }
    diff == 0
}

/// Build a fresh slot for `secret` with a random 16-byte salt drawn from the
/// OS CSPRNG (`getrandom`).
fn new_slot(label: &str, kind: &str, secret: &str) -> Result<KeySlot> {
    let mut salt = [0u8; SALT_LEN];
    getrandom::fill(&mut salt).expect("OS CSPRNG (getrandom) unavailable");
    let verifier = derive(secret, &salt)?;
    let b64 = base64::engine::general_purpose::STANDARD;
    Ok(KeySlot {
        label: label.to_string(),
        kind: kind.to_string(),
        kdf: "scrypt".to_string(),
        salt: b64.encode(salt),
        verifier: b64.encode(verifier),
    })
}

/// Whether `secret` unlocks `slot` (constant-time verifier compare).
fn slot_matches(slot: &KeySlot, secret: &str) -> Result<bool> {
    let b64 = base64::engine::general_purpose::STANDARD;
    let salt = b64
        .decode(&slot.salt)
        .map_err(|e| ChunkStoreError::Redb(format!("keyfile salt: {e}")))?;
    let want = b64
        .decode(&slot.verifier)
        .map_err(|e| ChunkStoreError::Redb(format!("keyfile verifier: {e}")))?;
    let got = derive(secret, &salt)?;
    Ok(ct_eq(&got, &want))
}

/// Index of the first slot `secret` unlocks, if any.
fn find_matching(slots: &[KeySlot], secret: &str) -> Result<Option<usize>> {
    for (i, s) in slots.iter().enumerate() {
        if slot_matches(s, secret)? {
            return Ok(Some(i));
        }
    }
    Ok(None)
}

/// Read the slots at `root`, migrating a v1 single-verifier file into one
/// `"primary"` slot. An absent file is a caller error (guard with [`exists`]).
fn load(root: &Path) -> Result<Vec<KeySlot>> {
    let path = keyfile_path(root);
    let raw = std::fs::read(&path).map_err(|e| ChunkStoreError::Io {
        path: path.clone(),
        source: e,
    })?;
    let kf: RepoKeyFile = serde_json::from_slice(&raw)?;
    if !kf.slots.is_empty() {
        return Ok(kf.slots);
    }
    // v1 migration: top-level kdf/salt/verifier → one primary slot.
    if let (Some(kdf), Some(salt), Some(verifier)) = (kf.kdf, kf.salt, kf.verifier) {
        return Ok(vec![KeySlot {
            label: PRIMARY_LABEL.to_string(),
            kind: KIND_PASSWORD.to_string(),
            kdf,
            salt,
            verifier,
        }]);
    }
    Ok(Vec::new())
}

/// The current slots, or an empty vec when no key file exists yet.
fn load_or_empty(root: &Path) -> Result<Vec<KeySlot>> {
    if exists(root) {
        load(root)
    } else {
        Ok(Vec::new())
    }
}

/// Persist `slots` (v2 shape — only the `slots` array).
fn save(root: &Path, slots: &[KeySlot]) -> Result<()> {
    let kf = RepoKeyFile {
        slots: slots.to_vec(),
        kdf: None,
        salt: None,
        verifier: None,
    };
    let json = serde_json::to_vec_pretty(&kf)?;
    let path = keyfile_path(root);
    std::fs::write(&path, json).map_err(|e| ChunkStoreError::Io { path, source: e })
}

/// A high-entropy recovery secret (256 bits from the OS CSPRNG), grouped into
/// eight 8-char hex blocks for legibility, e.g. `1A2B3C4D-…`.
fn generate_recovery_secret() -> String {
    let mut raw = [0u8; 32];
    getrandom::fill(&mut raw).expect("OS CSPRNG (getrandom) unavailable");
    let mut hex = String::with_capacity(64);
    for byte in &raw {
        hex.push_str(&format!("{byte:02X}"));
    }
    hex.as_bytes()
        .chunks(8)
        .map(|c| std::str::from_utf8(c).unwrap_or(""))
        .collect::<Vec<_>>()
        .join("-")
}

/// Initialise the key file at `root` with a single `"primary"` password slot
/// (used by `Repository::create`).
pub(crate) fn write(root: &Path, password: &str) -> Result<()> {
    save(root, &[new_slot(PRIMARY_LABEL, KIND_PASSWORD, password)?])
}

/// Verify `secret` against the slots at `root`. `Ok(())` on a match against
/// *any* slot (or when there is no key file — an unlocked repo);
/// [`ChunkStoreError::Locked`] if slots exist but `secret` is `None`;
/// [`ChunkStoreError::BadPassphrase`] on no match.
pub(crate) fn verify(root: &Path, secret: Option<&str>) -> Result<()> {
    if !exists(root) {
        return Ok(()); // no gate
    }
    let slots = load(root)?;
    if slots.is_empty() {
        return Ok(()); // gate file with no slots — treat as unlocked
    }
    let Some(secret) = secret else {
        return Err(ChunkStoreError::Locked);
    };
    if find_matching(&slots, secret)?.is_some() {
        Ok(())
    } else {
        Err(ChunkStoreError::BadPassphrase)
    }
}

/// Rotate the slot that `old` authenticates to `new`, leaving every other
/// slot intact. When there is no matching slot (an unlocked repo, `old` =
/// `None`), a `"primary"` slot is created. Backs `Repository::change_password`.
pub(crate) fn rotate(root: &Path, old: Option<&str>, new: &str) -> Result<()> {
    verify(root, old)?;
    let mut slots = load_or_empty(root)?;
    if let Some(secret) = old {
        if let Some(idx) = find_matching(&slots, secret)? {
            let (label, kind) = (slots[idx].label.clone(), slots[idx].kind.clone());
            slots[idx] = new_slot(&label, &kind, new)?;
            return save(root, &slots);
        }
    }
    slots.push(new_slot(PRIMARY_LABEL, KIND_PASSWORD, new)?);
    save(root, &slots)
}

/// Add a new password slot `label` unlocking with `new_password`. `auth` must
/// already grant access. Errors if `label` is already taken.
pub(crate) fn add_slot(
    root: &Path,
    auth: Option<&str>,
    new_password: &str,
    label: &str,
) -> Result<()> {
    verify(root, auth)?;
    // `recovery` is RESERVED for the recovery slot. Without this, a password
    // slot could be labelled "recovery" too, and remove_slot (which deletes
    // every slot matching a label) would then revoke the user's password slot
    // when they meant to drop only the recovery key — a lock-out risk.
    if label == KIND_RECOVERY {
        return Err(ChunkStoreError::Redb(format!(
            "{KIND_RECOVERY:?} is a reserved slot label; use the recovery-key generator"
        )));
    }
    let mut slots = load_or_empty(root)?;
    if slots.iter().any(|s| s.label == label) {
        return Err(ChunkStoreError::Redb(format!(
            "key slot {label:?} already exists"
        )));
    }
    slots.push(new_slot(label, KIND_PASSWORD, new_password)?);
    save(root, &slots)
}

/// Remove the slot named `label`. Returns `false` if no such slot existed.
/// Refuses to remove the *last* slot (that would silently unlock the repo).
pub(crate) fn remove_slot(root: &Path, label: &str) -> Result<bool> {
    if !exists(root) {
        return Ok(false);
    }
    let mut slots = load(root)?;
    let before = slots.len();
    slots.retain(|s| s.label != label);
    if slots.len() == before {
        return Ok(false);
    }
    if slots.is_empty() {
        return Err(ChunkStoreError::Redb(
            "refusing to remove the last key slot (would unlock the repository)".to_string(),
        ));
    }
    save(root, &slots)?;
    Ok(true)
}

/// Generate a recovery key (returned ONCE — only its verifier is stored),
/// replacing any existing recovery slot. `auth` must already grant access.
pub(crate) fn generate_recovery(root: &Path, auth: Option<&str>) -> Result<String> {
    verify(root, auth)?;
    let mut slots = load_or_empty(root)?;
    slots.retain(|s| s.kind != KIND_RECOVERY); // one recovery slot at a time
    let secret = generate_recovery_secret();
    slots.push(new_slot(KIND_RECOVERY, KIND_RECOVERY, &secret)?);
    save(root, &slots)?;
    Ok(secret)
}

/// The slots at `root` (labels + kinds; no verifier bytes).
pub(crate) fn list_slots(root: &Path) -> Result<Vec<KeySlotInfo>> {
    if !exists(root) {
        return Ok(Vec::new());
    }
    Ok(load(root)?
        .into_iter()
        .map(|s| KeySlotInfo {
            label: s.label,
            kind: s.kind,
        })
        .collect())
}
