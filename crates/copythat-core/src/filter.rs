//! Phase 14a — enumeration-time filters.
//!
//! `FilterSet` is the user-facing, cheap-to-clone configuration that
//! travels with `TreeOptions`; `CompiledFilters` is the
//! engine-internal compiled form the walker actually consults. The
//! split lets the walker avoid re-parsing glob patterns for every
//! entry — `globset::GlobSet` compiles once and matches without
//! per-path allocation.
//!
//! Filter semantics:
//! - Include globs act as a whitelist for *files* only. If any
//!   include glob is configured, a file must match at least one to
//!   survive. Directories are never gated by include globs — that
//!   would prune every subtree (e.g. `*.txt` would reject `photos/`
//!   and we'd never discover the .txt files inside).
//! - Exclude globs apply to both files and directories. A match on a
//!   directory prunes the whole subtree, which is how
//!   `**/node_modules` works.
//! - Size / date / attribute filters apply to *files* only.
//! - Glob matching is done on the path relative to the src root, so
//!   `docs/draft.md` is what `**/*.md` is asked to match against, not
//!   the absolute path.

use std::path::Path;
use std::time::SystemTime;

use globset::{Glob, GlobSet, GlobSetBuilder};
use thiserror::Error;

/// Caller-supplied include/exclude rules applied during a tree walk.
/// All fields are optional; an all-default `FilterSet` matches every
/// file. Compile via [`FilterSet::compile`] before passing to the
/// engine.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FilterSet {
    /// Whitelist globs. Empty = no whitelist.
    pub include_globs: Vec<String>,
    /// Blacklist globs. Empty = no blacklist.
    pub exclude_globs: Vec<String>,
    /// Skip files smaller than this many bytes.
    pub min_size_bytes: Option<u64>,
    /// Skip files larger than this many bytes.
    pub max_size_bytes: Option<u64>,
    /// Skip files modified before this instant.
    pub min_mtime: Option<SystemTime>,
    /// Skip files modified after this instant.
    pub max_mtime: Option<SystemTime>,
    /// Skip dotfiles / Windows hidden-attribute files.
    pub skip_hidden: bool,
    /// Skip Windows system-attribute files.
    pub skip_system: bool,
    /// Skip read-only files.
    pub skip_readonly: bool,
}

impl FilterSet {
    /// `true` when no filter is active (i.e. every file passes).
    pub fn is_empty(&self) -> bool {
        self.include_globs.is_empty()
            && self.exclude_globs.is_empty()
            && self.min_size_bytes.is_none()
            && self.max_size_bytes.is_none()
            && self.min_mtime.is_none()
            && self.max_mtime.is_none()
            && !self.skip_hidden
            && !self.skip_system
            && !self.skip_readonly
    }

    /// Validate every glob pattern and return the engine-ready
    /// [`CompiledFilters`] form.
    pub fn compile(&self) -> Result<CompiledFilters, FilterError> {
        let include = build_glob_set("include", &self.include_globs)?;
        let exclude = build_glob_set("exclude", &self.exclude_globs)?;
        Ok(CompiledFilters {
            include: (!self.include_globs.is_empty()).then_some(include),
            exclude: (!self.exclude_globs.is_empty()).then_some(exclude),
            min_size_bytes: self.min_size_bytes,
            max_size_bytes: self.max_size_bytes,
            min_mtime: self.min_mtime,
            max_mtime: self.max_mtime,
            skip_hidden: self.skip_hidden,
            skip_system: self.skip_system,
            skip_readonly: self.skip_readonly,
        })
    }
}

fn build_glob_set(kind: &'static str, patterns: &[String]) -> Result<GlobSet, FilterError> {
    let mut b = GlobSetBuilder::new();
    for p in patterns {
        let g = Glob::new(p).map_err(|source| FilterError::InvalidGlob {
            kind,
            pattern: p.clone(),
            source,
        })?;
        b.add(g);
    }
    b.build().map_err(|source| FilterError::InvalidGlob {
        kind,
        pattern: String::new(),
        source,
    })
}

/// Errors raised by [`FilterSet::compile`].
#[derive(Debug, Error)]
pub enum FilterError {
    /// One of the include/exclude glob patterns failed to parse.
    #[error("invalid {kind} glob pattern `{pattern}`: {source}")]
    InvalidGlob {
        /// `"include"` or `"exclude"` — which list the bad pattern came from.
        kind: &'static str,
        /// The user-supplied glob string.
        pattern: String,
        /// Underlying parser error.
        #[source]
        source: globset::Error,
    },
}

/// Pre-compiled, cheap-to-match form of [`FilterSet`]. Not
/// `Serialize`: it owns a compiled `GlobSet` state machine.
#[derive(Debug, Clone)]
pub struct CompiledFilters {
    include: Option<GlobSet>,
    exclude: Option<GlobSet>,
    min_size_bytes: Option<u64>,
    max_size_bytes: Option<u64>,
    min_mtime: Option<SystemTime>,
    max_mtime: Option<SystemTime>,
    skip_hidden: bool,
    skip_system: bool,
    skip_readonly: bool,
}

impl CompiledFilters {
    /// `true` when the file at `rel_path` (with the given metadata)
    /// passes every active filter and should be copied.
    pub fn passes_file(&self, rel_path: &Path, meta: &std::fs::Metadata) -> bool {
        if let Some(inc) = &self.include
            && !inc.is_match(rel_path)
        {
            return false;
        }
        if let Some(exc) = &self.exclude
            && exc.is_match(rel_path)
        {
            return false;
        }
        let size = meta.len();
        if let Some(min) = self.min_size_bytes
            && size < min
        {
            return false;
        }
        if let Some(max) = self.max_size_bytes
            && size > max
        {
            return false;
        }
        if let Ok(mtime) = meta.modified() {
            if let Some(min) = self.min_mtime
                && mtime < min
            {
                return false;
            }
            if let Some(max) = self.max_mtime
                && mtime > max
            {
                return false;
            }
        }
        if self.skip_hidden && is_hidden(rel_path, meta) {
            return false;
        }
        if self.skip_system && is_system(meta) {
            return false;
        }
        if self.skip_readonly && is_readonly(meta) {
            return false;
        }
        true
    }

    /// Directory gate. Returning false prunes the whole subtree —
    /// that's how `**/node_modules` pruning works.
    pub fn passes_dir(&self, rel_path: &Path, meta: &std::fs::Metadata) -> bool {
        if let Some(exc) = &self.exclude
            && exc.is_match(rel_path)
        {
            return false;
        }
        if self.skip_hidden && is_hidden(rel_path, meta) {
            return false;
        }
        if self.skip_system && is_system(meta) {
            return false;
        }
        true
    }
}

#[cfg(unix)]
fn is_hidden(rel_path: &Path, _meta: &std::fs::Metadata) -> bool {
    rel_path
        .file_name()
        .and_then(|n| n.to_str())
        .map(|n| n.starts_with('.'))
        .unwrap_or(false)
}

#[cfg(windows)]
fn is_hidden(_rel_path: &Path, meta: &std::fs::Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;
    const FILE_ATTRIBUTE_HIDDEN: u32 = 0x2;
    meta.file_attributes() & FILE_ATTRIBUTE_HIDDEN != 0
}

#[cfg(not(any(unix, windows)))]
fn is_hidden(_rel_path: &Path, _meta: &std::fs::Metadata) -> bool {
    false
}

#[cfg(windows)]
fn is_system(meta: &std::fs::Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;
    const FILE_ATTRIBUTE_SYSTEM: u32 = 0x4;
    meta.file_attributes() & FILE_ATTRIBUTE_SYSTEM != 0
}

#[cfg(not(windows))]
fn is_system(_meta: &std::fs::Metadata) -> bool {
    false
}

fn is_readonly(meta: &std::fs::Metadata) -> bool {
    meta.permissions().readonly()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn meta_of(path: &Path) -> std::fs::Metadata {
        std::fs::metadata(path).expect("metadata")
    }

    #[test]
    fn empty_filter_passes_everything() {
        let fs = FilterSet::default();
        assert!(fs.is_empty());
        let c = fs.compile().unwrap();
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let p = tmp.path();
        let m = meta_of(p);
        assert!(c.passes_file(&PathBuf::from("anything.txt"), &m));
    }

    #[test]
    fn include_glob_rejects_non_matching_files() {
        let fs = FilterSet {
            include_globs: vec!["**/*.txt".into()],
            ..Default::default()
        };
        let c = fs.compile().unwrap();
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let m = meta_of(tmp.path());
        assert!(c.passes_file(&PathBuf::from("notes/a.txt"), &m));
        assert!(!c.passes_file(&PathBuf::from("notes/a.md"), &m));
    }

    #[test]
    fn exclude_glob_wins_over_include() {
        let fs = FilterSet {
            include_globs: vec!["**/*.txt".into()],
            exclude_globs: vec!["**/draft-*.txt".into()],
            ..Default::default()
        };
        let c = fs.compile().unwrap();
        let tmp = tempfile::NamedTempFile::new().unwrap();
        let m = meta_of(tmp.path());
        assert!(c.passes_file(&PathBuf::from("notes/final.txt"), &m));
        assert!(!c.passes_file(&PathBuf::from("notes/draft-1.txt"), &m));
    }

    #[test]
    fn include_glob_does_not_gate_directories() {
        let fs = FilterSet {
            include_globs: vec!["**/*.txt".into()],
            ..Default::default()
        };
        let c = fs.compile().unwrap();
        let tmp = tempfile::tempdir().unwrap();
        let m = meta_of(tmp.path());
        // Dir must pass so the walker descends and discovers .txt
        // children — otherwise the include pattern would starve.
        assert!(c.passes_dir(&PathBuf::from("photos"), &m));
    }

    #[test]
    fn exclude_glob_prunes_directory() {
        let fs = FilterSet {
            exclude_globs: vec!["**/node_modules".into()],
            ..Default::default()
        };
        let c = fs.compile().unwrap();
        let tmp = tempfile::tempdir().unwrap();
        let m = meta_of(tmp.path());
        assert!(!c.passes_dir(&PathBuf::from("pkg/node_modules"), &m));
        assert!(c.passes_dir(&PathBuf::from("pkg/src"), &m));
    }

    #[test]
    fn invalid_glob_surfaces_error() {
        let fs = FilterSet {
            include_globs: vec!["[broken".into()],
            ..Default::default()
        };
        let err = fs.compile().unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("include"), "{msg}");
        assert!(msg.contains("[broken"), "{msg}");
    }
}
