//! Path-based filtering.
//!
//! Drops the noise classes that every real "live mirror" tool has to
//! handle: vim swap files, emacs lock files, Office lock files, and
//! (optionally) editor backup files. The filter runs before any
//! debounce or atomic-save logic — filtered paths never enter the
//! state machine, so they can't trigger false synthetic events.
//!
//! Filters are defined as functions from path → decision so the
//! compiled matcher stays allocation-free; no glob expansion or
//! regex on the hot path.

use std::path::Path;

/// Categorisation of a path. `Filtered` kinds never produce a
/// [`crate::FsEvent`]; `Passthrough` continues into the debouncer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathFilter {
    /// Regular user file — pass through to the debouncer.
    Passthrough,
    /// Vim swap file — drop.
    VimSwap,
    /// Emacs lock file — drop.
    EmacsLock,
    /// Microsoft Office owner-lock file (`~$file.docx`) — drop.
    OfficeLock,
    /// Editor backup (`file~`, `file.bak`) — drop when
    /// `filter_editor_backups` is true, pass otherwise.
    EditorBackup,
    /// Known temp name (`~tmpXXXX.tmp`, `*.crdownload`,
    /// `*.part`) — pass so the atomic-save state machine can see it
    /// and coalesce with the later RENAME.
    KnownTemp,
}

impl PathFilter {
    /// `true` when this categorisation should drop the event
    /// outright (before debounce).
    pub fn is_dropped(self, filter_editor_backups: bool) -> bool {
        match self {
            Self::Passthrough | Self::KnownTemp => false,
            Self::VimSwap | Self::EmacsLock | Self::OfficeLock => true,
            Self::EditorBackup => filter_editor_backups,
        }
    }
}

/// Default classifier. Inspects the final path segment only — no FS
/// calls, no allocation beyond a transient `&str` of the file name.
///
/// Rules:
/// - `*.swp` / `*.swpx` / `*.swx` → VimSwap
/// - `.#*` → EmacsLock
/// - `~$*` → OfficeLock
/// - `*.tmp` / `*.crdownload` / `*.part` → KnownTemp (reaches the
///   atomic-save state machine; never leaks as an event on its own)
/// - `*~` / `*.bak` → EditorBackup (caller decides)
/// - Everything else → Passthrough
pub fn default_filter(path: &Path) -> PathFilter {
    let Some(name_os) = path.file_name() else {
        return PathFilter::Passthrough;
    };
    let Some(name) = name_os.to_str() else {
        return PathFilter::Passthrough;
    };

    // Emacs lock: `.#foo`
    if let Some(stripped) = name.strip_prefix(".#") {
        if !stripped.is_empty() {
            return PathFilter::EmacsLock;
        }
    }

    // Office owner-lock: `~$file.docx`
    if let Some(stripped) = name.strip_prefix("~$") {
        if !stripped.is_empty() {
            return PathFilter::OfficeLock;
        }
    }

    // Vim swap: `.<name>.swp` / `.<name>.swpx` / `.<name>.swx`
    if name.starts_with('.')
        && (name.ends_with(".swp") || name.ends_with(".swpx") || name.ends_with(".swx"))
    {
        return PathFilter::VimSwap;
    }

    // Known temps — atomic-save candidates.
    let name_lower = name.to_ascii_lowercase();
    if name_lower.ends_with(".tmp")
        || name_lower.ends_with(".crdownload")
        || name_lower.ends_with(".part")
        || name_lower.ends_with(".partial")
    {
        return PathFilter::KnownTemp;
    }
    // Office save-dance temps: `~tmpFE12.tmp` is already caught by
    // `.tmp`, but the bare-number variant without extension
    // (`~WRL1234.tmp` etc) is too — always `.tmp`.

    // Editor backups.
    if name.ends_with('~') || name_lower.ends_with(".bak") {
        return PathFilter::EditorBackup;
    }

    PathFilter::Passthrough
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn classify(name: &str) -> PathFilter {
        default_filter(&PathBuf::from(name))
    }

    #[test]
    fn vim_swap_variants() {
        assert_eq!(classify(".file.txt.swp"), PathFilter::VimSwap);
        assert_eq!(classify(".file.txt.swpx"), PathFilter::VimSwap);
        assert_eq!(classify(".file.txt.swx"), PathFilter::VimSwap);
    }

    #[test]
    fn emacs_lock_prefix() {
        assert_eq!(classify(".#foo"), PathFilter::EmacsLock);
        assert_eq!(classify(".#budget.numbers"), PathFilter::EmacsLock);
    }

    #[test]
    fn office_owner_lock_prefix() {
        assert_eq!(classify("~$budget.xlsx"), PathFilter::OfficeLock);
        assert_eq!(classify("~$Report.docx"), PathFilter::OfficeLock);
    }

    #[test]
    fn known_temp_extensions() {
        assert_eq!(classify("save.tmp"), PathFilter::KnownTemp);
        assert_eq!(classify("download.crdownload"), PathFilter::KnownTemp);
        assert_eq!(classify("upload.part"), PathFilter::KnownTemp);
        assert_eq!(classify("chunk.partial"), PathFilter::KnownTemp);
    }

    #[test]
    fn editor_backup_suffix() {
        assert_eq!(classify("file.txt~"), PathFilter::EditorBackup);
        assert_eq!(classify("file.bak"), PathFilter::EditorBackup);
    }

    #[test]
    fn regular_user_files_pass_through() {
        assert_eq!(classify("report.docx"), PathFilter::Passthrough);
        assert_eq!(classify(".gitignore"), PathFilter::Passthrough);
        assert_eq!(classify("image.png"), PathFilter::Passthrough);
    }

    #[test]
    fn is_dropped_respects_editor_backup_toggle() {
        assert!(PathFilter::VimSwap.is_dropped(false));
        assert!(PathFilter::EmacsLock.is_dropped(false));
        assert!(PathFilter::OfficeLock.is_dropped(false));
        assert!(!PathFilter::Passthrough.is_dropped(false));
        assert!(!PathFilter::KnownTemp.is_dropped(false));
        // Editor backup is the only one whose decision depends on the flag.
        assert!(!PathFilter::EditorBackup.is_dropped(false));
        assert!(PathFilter::EditorBackup.is_dropped(true));
    }

    #[test]
    fn bare_tilde_dollar_not_flagged() {
        // `~$` alone (no suffix) isn't an Office lock.
        assert_eq!(classify("~$"), PathFilter::Passthrough);
    }

    #[test]
    fn bare_hash_dot_not_flagged() {
        assert_eq!(classify(".#"), PathFilter::Passthrough);
    }
}
