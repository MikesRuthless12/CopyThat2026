//! Phase 17a — path-safety guards.
//!
//! Every path that crosses a trust boundary (IPC, CLI, scheduled-job
//! TOML, shell-extension `--enqueue`) must pass through
//! [`validate_path_no_traversal`] before the engine sees it. The
//! check is lexical — it walks `Path::components` and rejects any
//! `ParentDir` (`..`) segment — so it's fast, deterministic, and
//! never touches the filesystem. That matters because filesystem-
//! based checks have TOCTOU races (Time-of-Check-to-Time-of-Use):
//! a `canonicalize()` result can disagree with what the engine
//! actually opens milliseconds later if an attacker swaps a
//! symlink in between.
//!
//! ## Threat model this blocks
//!
//! A malicious IPC caller (e.g. a compromised browser renderer
//! embedded in the webview, or a shell-extension forgery) crafting
//! a destination like `foo/../../../etc/passwd` to escape the
//! user's chosen staging root. The engine would happily follow the
//! `..` chain because POSIX and NTFS path resolution is sequential
//! — the safeguards are up to us.
//!
//! ## What this does NOT block (yet)
//!
//! - **Symlink races (TOCTOU).** If the user's chosen destination
//!   already contains a symlink that points outside, the engine
//!   follows it. Phase 17c (deferred — see `docs/SECURITY_BACKLOG.md`)
//!   switches to `openat(O_NOFOLLOW)` on Linux/macOS and
//!   `CreateFileW(FILE_FLAG_OPEN_REPARSE_POINT)` on Windows to close
//!   this gap.
//! - **Absolute-path policy.** A caller can still specify
//!   `/etc/passwd` directly; we treat that as an explicit user
//!   choice because the UI's drop dialog is the one resolving it,
//!   and forbidding absolute paths would break every real use case.
//!   Callers that want an absolute-root jail (e.g. a scheduled-job
//!   runner) should call [`is_within_root`] after canonicalisation.
//! - **Non-UTF-8 on Windows.** Windows paths are WTF-16 and the
//!   engine handles `OsStr` throughout; the IPC boundary rejects
//!   non-UTF-8 before it reaches the engine.
//!
//! ## Why lexical, not filesystem-based
//!
//! `std::fs::canonicalize` resolves symlinks, but doing so requires
//! the path to exist *right now*, which is the opposite of what we
//! want for a *destination* path (which we're about to create).
//! Walking `Path::components` is O(segments) and works equally well
//! for "foo/../bar" on a path that doesn't yet exist.

use std::path::{Component, Path, PathBuf};

use thiserror::Error;

/// Every way a path can fail the Phase 17a safety bar. Kept narrow
/// so callers can render it directly (each variant maps to one
/// Fluent key) without having to flatten into a string first.
#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum PathSafetyError {
    /// The path contains one or more `..` components. Even a single
    /// parent-dir segment is enough to reject — we don't try to
    /// "simplify" `foo/../bar` back to `bar` because an attacker
    /// who slipped the `..` in once will slip in two more.
    #[error("path `{}` contains parent-directory (`..`) component(s) — rejected to prevent directory traversal", offending.display())]
    ParentTraversal { offending: PathBuf },

    /// The path is empty after trimming. Engine callers that want
    /// to distinguish "no source" from "bad source" already do so
    /// before the safety bar; this variant exists so
    /// [`validate_path_no_traversal`] never silently accepts `""`.
    #[error("path is empty")]
    Empty,

    /// The path's bytes contain a NUL (`\0`). On POSIX this is
    /// never legal; on Windows the Rust stdlib would also reject
    /// this at `OsStr::new_unchecked` level, but the IPC layer
    /// hands us `String`, so we guard explicitly.
    #[error("path contains a NUL byte")]
    NulByte,
}

impl PathSafetyError {
    /// Fluent key the UI renders. Keeps error UX consistent with
    /// the existing `CopyErrorKind::localized_key` convention.
    pub const fn localized_key(&self) -> &'static str {
        match self {
            Self::ParentTraversal { .. } => "err-path-escape",
            Self::Empty => "err-destination-empty",
            Self::NulByte => "err-path-escape",
        }
    }
}

/// Lexical guard: reject any path containing a parent-dir (`..`)
/// component, a NUL byte, or no content at all. The check is
/// filesystem-free — safe to call on a destination that doesn't
/// yet exist, and free from TOCTOU windows.
pub fn validate_path_no_traversal(path: &Path) -> Result<(), PathSafetyError> {
    if path.as_os_str().is_empty() {
        return Err(PathSafetyError::Empty);
    }
    // NUL-byte check via the platform-specific bytes view. On
    // Windows OsStr is WTF-16 so we inspect the u16 units; on
    // Unix the raw bytes include potential embedded NULs.
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt;
        if path.as_os_str().as_bytes().contains(&0) {
            return Err(PathSafetyError::NulByte);
        }
    }
    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStrExt;
        if path.as_os_str().encode_wide().any(|u| u == 0) {
            return Err(PathSafetyError::NulByte);
        }
    }
    for comp in path.components() {
        if matches!(comp, Component::ParentDir) {
            return Err(PathSafetyError::ParentTraversal {
                offending: path.to_path_buf(),
            });
        }
    }
    Ok(())
}

/// Run [`validate_path_no_traversal`] on each path in `paths`.
/// Returns the first offender's error; on success echoes the input
/// back as an owned [`Vec<PathBuf>`] so the caller doesn't have to
/// duplicate an iteration.
pub fn validate_all<I, P>(paths: I) -> Result<Vec<PathBuf>, PathSafetyError>
where
    I: IntoIterator<Item = P>,
    P: AsRef<Path>,
{
    let mut out = Vec::new();
    for p in paths {
        let pb = p.as_ref().to_path_buf();
        validate_path_no_traversal(&pb)?;
        out.push(pb);
    }
    Ok(out)
}

/// Check whether `candidate` — *after* symlink-free canonicalisation
/// — is lexically contained inside `root`. Useful for scheduled-job
/// runners that want to jail the engine to a fixed directory. Not
/// used by `copy_file` / `copy_tree` today; kept here so callers
/// have one place to find the helper.
///
/// This DOES touch the filesystem (both arms get canonicalised),
/// so it's TOCTOU-susceptible — only call it once, and have the
/// engine open with `O_NOFOLLOW` afterwards (Phase 17c).
pub fn is_within_root(candidate: &Path, root: &Path) -> std::io::Result<bool> {
    let c = candidate.canonicalize()?;
    let r = root.canonicalize()?;
    Ok(c.starts_with(r))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_simple_parent_traversal() {
        let err = validate_path_no_traversal(Path::new("foo/../../../etc/passwd")).unwrap_err();
        assert!(matches!(err, PathSafetyError::ParentTraversal { .. }));
        assert_eq!(err.localized_key(), "err-path-escape");
    }

    #[test]
    fn rejects_single_parent_component() {
        let err = validate_path_no_traversal(Path::new("..")).unwrap_err();
        assert!(matches!(err, PathSafetyError::ParentTraversal { .. }));
    }

    #[test]
    fn rejects_absolute_traversal() {
        // Even on an absolute path, a `..` component is suspicious:
        // the caller is either mistaken or attempting to obfuscate.
        let err =
            validate_path_no_traversal(Path::new("/home/user/../../../etc/passwd")).unwrap_err();
        assert!(matches!(err, PathSafetyError::ParentTraversal { .. }));
    }

    #[cfg(windows)]
    #[test]
    fn rejects_windows_backslash_traversal() {
        // Gated to Windows: on POSIX, `Path::components` does not
        // treat `\` as a path separator, so `C:\a\..` parses as a
        // single component string and the `..` never surfaces as a
        // `ParentDir`. That's not a safety gap — a POSIX caller
        // hands POSIX paths — but it means the test only makes
        // sense where `\` is the real separator.
        let err = validate_path_no_traversal(Path::new(r"C:\Users\user\..\..\..\Windows\System32"))
            .unwrap_err();
        assert!(matches!(err, PathSafetyError::ParentTraversal { .. }));
    }

    #[test]
    fn accepts_normal_relative_path() {
        validate_path_no_traversal(Path::new("subdir/file.txt")).unwrap();
    }

    #[test]
    fn accepts_absolute_path_without_parent_components() {
        #[cfg(unix)]
        validate_path_no_traversal(Path::new("/tmp/dst")).unwrap();
        #[cfg(windows)]
        validate_path_no_traversal(Path::new(r"C:\Users\me\dst")).unwrap();
    }

    #[test]
    fn accepts_current_dir_and_plain_filenames() {
        // `Component::CurDir` (a literal `.`) is not a traversal
        // threat and is harmless — it resolves back to the parent
        // during normal path processing.
        validate_path_no_traversal(Path::new("./foo.txt")).unwrap();
        validate_path_no_traversal(Path::new("foo.txt")).unwrap();
    }

    #[test]
    fn rejects_empty_path() {
        let err = validate_path_no_traversal(Path::new("")).unwrap_err();
        assert_eq!(err, PathSafetyError::Empty);
    }

    #[cfg(unix)]
    #[test]
    fn rejects_nul_byte_on_unix() {
        use std::os::unix::ffi::OsStringExt;
        use std::path::PathBuf;
        let bad = PathBuf::from(std::ffi::OsString::from_vec(b"foo\0bar".to_vec()));
        let err = validate_path_no_traversal(&bad).unwrap_err();
        assert_eq!(err, PathSafetyError::NulByte);
    }

    #[test]
    fn validate_all_flags_first_offender() {
        let err = validate_all([
            Path::new("ok.txt"),
            Path::new("bad/../../etc/passwd"),
            Path::new("also-ok"),
        ])
        .unwrap_err();
        assert!(matches!(err, PathSafetyError::ParentTraversal { .. }));
    }

    #[test]
    fn validate_all_returns_pathbufs_on_success() {
        let out =
            validate_all([Path::new("a/b.txt"), Path::new("c/d.txt")]).expect("both are safe");
        assert_eq!(out.len(), 2);
    }
}
