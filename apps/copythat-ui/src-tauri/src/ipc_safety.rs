//! Phase 17e — IPC argument audit + canonicalisation.
//!
//! Every `#[tauri::command]` that takes a path-typed argument must
//! gate the input through this module. The single source of truth
//! for the gate is [`validate_ipc_path`]; multi-path commands route
//! through [`validate_ipc_paths`]. Both surface a typed [`IpcError`]
//! that the frontend can translate via Fluent — so a rejected path
//! never reaches the engine and never lands as an opaque ad-hoc
//! `String` error message.
//!
//! # Threat model the gate addresses
//!
//! 1. **Traversal escape.** Phase 17a's
//!    [`copythat_core::safety::validate_path_no_traversal`] guard
//!    rejects any path with a `..` segment or NUL byte. The IPC
//!    layer is the first place an attacker can deliver such a path
//!    (a forged WebView script, a malicious shell-extension caller,
//!    or a TOML jobspec hand-edited to point at `/etc/passwd`).
//!    Reject up front, before any history row writes.
//!
//! 2. **Trimmed-empty paths.** A user copy-pasting whitespace into
//!    the destination field is a bug, not an attacker — but the
//!    engine's metadata pre-flight crashes on `""`, so reject early
//!    with a Fluent-keyed error rather than letting the engine
//!    surface a low-level `NotFound`.
//!
//! 3. **Non-UTF-8 / lossy strings.** IPC arrives as `String`; on
//!    Windows, Tauri's serde layer already coerces the WTF-16
//!    payload to UTF-8 via lossy replacement. We check for the
//!    U+FFFD replacement character and reject — a path containing a
//!    real `?` is fine, but a path the OS would render with U+FFFD
//!    fragments would silently drift between what the user typed
//!    and what the engine opens.
//!
//! # What the gate does NOT address
//!
//! - **Symlink races (TOCTOU).** That's Phase 17c's `O_NOFOLLOW` /
//!   `FILE_FLAG_OPEN_REPARSE_POINT` bar; the IPC layer can't see
//!   into the engine's open call. The two layers compose: the IPC
//!   gate rejects shaped-bad input and the engine gate rejects
//!   raced symlinks.
//! - **Per-command capability checks.** Each command should still
//!   call its domain-specific validator (e.g. "destination must
//!   exist as a directory") on top of this lexical gate. The gate
//!   is the floor, not the ceiling.

use std::path::{Path, PathBuf};

use copythat_core::{PathSafetyError, validate_path_no_traversal};
use serde::{Deserialize, Serialize};

/// Typed IPC failure. Encodes the rejection class so the frontend
/// can translate via Fluent rather than parsing a free-text message.
///
/// This is a stable wire enum: the discriminants are namespaced by
/// the Fluent key, never numeric. Adding a variant is a non-breaking
/// change as long as the frontend tolerates unknown keys.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "key", rename_all = "kebab-case")]
pub enum IpcError {
    /// Phase 17a — `..` or NUL-byte rejection. Maps to Fluent key
    /// `err-path-escape` to align with `CopyErrorKind::PathEscape`.
    PathEscape,
    /// The string was empty after trim. Maps to Fluent key
    /// `err-destination-empty`.
    EmptyPath,
    /// The Tauri serde layer delivered a string with one or more
    /// U+FFFD replacement characters; the original payload was not
    /// valid UTF-8 (or was lossily coerced from WTF-16). Reject
    /// rather than process a path that drifts between the
    /// frontend and the engine.
    InvalidEncoding,
    /// Path-list arg was empty. Specific to `Vec<String>`-typed
    /// commands; maps to `err-source-required`.
    EmptyList,
}

impl IpcError {
    /// Stable Fluent key the UI renders this error as. Mirrors the
    /// `CopyErrorKind::localized_key` pattern so the engine + IPC
    /// layers share the same i18n surface.
    pub const fn localized_key(&self) -> &'static str {
        match self {
            Self::PathEscape => "err-path-escape",
            Self::EmptyPath => "err-destination-empty",
            Self::InvalidEncoding => "err-path-invalid-encoding",
            Self::EmptyList => "err-source-required",
        }
    }
}

impl std::fmt::Display for IpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.localized_key())
    }
}

impl std::error::Error for IpcError {}

impl From<PathSafetyError> for IpcError {
    fn from(value: PathSafetyError) -> Self {
        match value {
            PathSafetyError::ParentTraversal { .. } | PathSafetyError::NulByte => Self::PathEscape,
            PathSafetyError::Empty => Self::EmptyPath,
        }
    }
}

/// Validate a single path-typed argument received over IPC. Returns
/// the trimmed `PathBuf` on success; an [`IpcError`] keyed to the
/// rejection class on failure.
///
/// Use shape:
///
/// ```ignore
/// #[tauri::command]
/// pub async fn destination_free_bytes(path: String) -> Result<u64, String> {
///     let p = ipc_safety::validate_ipc_path(&path)?;
///     // …engine call with `p`…
/// }
/// ```
///
/// The `Result<_, String>` shape Tauri's IPC layer expects is
/// preserved by `IpcError::Display` rendering the Fluent key.
pub fn validate_ipc_path(raw: &str) -> Result<PathBuf, IpcError> {
    if raw.contains('\u{FFFD}') {
        return Err(IpcError::InvalidEncoding);
    }
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(IpcError::EmptyPath);
    }
    let pb = PathBuf::from(trimmed);
    validate_path_no_traversal(&pb).map_err(IpcError::from)?;
    Ok(pb)
}

/// Multi-path companion to [`validate_ipc_path`]. Rejects an empty
/// list with [`IpcError::EmptyList`] so commands like
/// `path_total_bytes` surface a specific Fluent key rather than the
/// generic empty-path error.
pub fn validate_ipc_paths<I, S>(raws: I) -> Result<Vec<PathBuf>, IpcError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut out = Vec::new();
    for raw in raws {
        out.push(validate_ipc_path(raw.as_ref())?);
    }
    if out.is_empty() {
        return Err(IpcError::EmptyList);
    }
    Ok(out)
}

/// Convenience for commands that already hold a `&Path` (e.g.
/// because a sibling command parsed a DTO). Performs the lexical
/// gate without the trim/encoding pass — those have already run
/// earlier in the pipeline.
pub fn validate_ipc_path_ref(p: &Path) -> Result<(), IpcError> {
    validate_path_no_traversal(p).map_err(IpcError::from)
}

/// Render an [`IpcError`] into the `String` shape Tauri expects from
/// `Result<_, String>`-returning commands. Equivalent to `e.to_string()`
/// but threading the Fluent key directly is faster than `Display`'s
/// `Formatter` allocation; kept here so the call sites stay tight.
pub fn err_string(e: IpcError) -> String {
    e.localized_key().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_traversal() {
        assert_eq!(
            validate_ipc_path("foo/../etc/passwd"),
            Err(IpcError::PathEscape)
        );
    }

    #[test]
    fn rejects_empty_after_trim() {
        assert_eq!(validate_ipc_path("   "), Err(IpcError::EmptyPath));
        assert_eq!(validate_ipc_path(""), Err(IpcError::EmptyPath));
    }

    #[test]
    fn rejects_replacement_character() {
        let bad = format!("good{}/path", '\u{FFFD}');
        assert_eq!(validate_ipc_path(&bad), Err(IpcError::InvalidEncoding));
    }

    #[test]
    fn accepts_normal_path() {
        let p = validate_ipc_path("/tmp/dst").unwrap();
        assert_eq!(p, PathBuf::from("/tmp/dst"));
    }

    #[test]
    fn empty_path_list_errors_distinctly() {
        let raws: Vec<String> = vec![];
        assert_eq!(validate_ipc_paths(raws), Err(IpcError::EmptyList));
    }

    #[test]
    fn list_passes_through_each_validation() {
        let ok = vec!["a", "b/c", "/d"];
        let v = validate_ipc_paths(ok).unwrap();
        assert_eq!(v.len(), 3);
        let bad = vec!["a", "b/../c"];
        assert_eq!(validate_ipc_paths(bad), Err(IpcError::PathEscape));
    }

    #[test]
    fn keys_match_engine_convention() {
        assert_eq!(IpcError::PathEscape.localized_key(), "err-path-escape");
        assert_eq!(IpcError::EmptyPath.localized_key(), "err-destination-empty");
        assert_eq!(
            IpcError::InvalidEncoding.localized_key(),
            "err-path-invalid-encoding"
        );
        assert_eq!(IpcError::EmptyList.localized_key(), "err-source-required");
    }
}
