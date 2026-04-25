//! Phase 30 — cross-platform path translation.
//!
//! When the user copies a filename across operating-system boundaries,
//! the filename bytes that were valid on the source can become
//! unusable (or semantically different) on the destination. The
//! classic cases:
//!
//! - **Unicode normalization.** macOS HFS+ stored filenames in NFD
//!   (combining marks split off the base glyph). APFS is
//!   normalization-insensitive but accepts NFC. Windows and Linux
//!   expect NFC. A file literally named `re\u{0301}sume\u{0301}.pdf`
//!   (NFD) copied to Windows shows up as a visually correct but
//!   byte-different filename than the one the user sees in Explorer
//!   if someone else copies it back as `résumé.pdf` (NFC). This
//!   module renormalizes names into the form the destination expects.
//!
//! - **Reserved filenames on Windows.** `CON`, `PRN`, `AUX`, `NUL`,
//!   `COM1`..`COM9`, `LPT1`..`LPT9` — with or without an extension —
//!   are device names on Windows. Creating a file called `CON.txt`
//!   on Windows does not create a file; it opens the console device.
//!   [`ReservedNameStrategy::Suffix`] appends `_` (so `CON.txt`
//!   becomes `CON_.txt`); [`ReservedNameStrategy::Reject`] surfaces a
//!   typed error so the UI can flag the file for skip.
//!
//! - **Long paths.** The legacy Windows MAX_PATH is 260 characters.
//!   A path exceeding that needs the `\\?\` namespace prefix to be
//!   reachable through Win32 APIs. Windows 10 1607+ relaxes the
//!   limit for long-path-aware processes, which Tauri is, but belt-
//!   and-braces: destinations that would blow past 260 UTF-16 units
//!   get the explicit prefix so short-path-aware ancillary tools
//!   (zip archivers, older installers) keep working. UNC paths get
//!   the `\\?\UNC\server\share\…` variant instead.
//!
//! - **Line endings.** Opt-in per-extension allowlist. A `.md` copied
//!   from Windows to Linux can be CRLF → LF if the user asked for it
//!   in Settings; files outside the allowlist are byte-copied.
//!   Detection runs on the buffered contents (first 64 KiB is enough
//!   for every real text file), not on the extension alone.
//!
//! The module is **pure** — no filesystem access, no logging, no I/O.
//! Callers compose [`translate_path`] at enqueue time to decide what
//! the destination name should look like and [`translate_line_endings`]
//! at copy time to optionally rewrite text content. Engine wiring
//! lives in the Tauri runner; this module stays testable at the
//! value-type layer.

use std::path::{Path, PathBuf};

use thiserror::Error;

// ---------------------------------------------------------------------
// Policy
// ---------------------------------------------------------------------

/// Destination platform for a translation pass.
///
/// `Auto` resolves at translation time to the host OS — useful when
/// the user hasn't picked an explicit destination platform and the
/// engine is running on the same machine the destination filesystem
/// is mounted on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TargetOs {
    Windows,
    MacOs,
    Linux,
    /// Resolve to the host OS via `cfg!(target_os = …)`.
    #[default]
    Auto,
}

/// Unicode normalization form to apply to the filename component.
///
/// `Auto` picks NFC for Windows + Linux destinations and leaves
/// macOS names untouched (APFS handles both forms). `AsIs` is the
/// escape hatch — the caller wants the source name byte-for-byte,
/// even if the destination filesystem would rather see a different
/// form.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NormalizationMode {
    /// Canonical composed form.
    Nfc,
    /// Canonical decomposed form.
    Nfd,
    /// Do not normalize — copy the source name unchanged.
    AsIs,
    /// Pick NFC for Windows + Linux targets; leave macOS unchanged.
    #[default]
    Auto,
}

/// Line-ending conversion mode.
///
/// The engine reads the file, sniffs its current line ending, and
/// rewrites only if the detected form disagrees with the target.
/// Files whose extension isn't on the allowlist are byte-copied
/// regardless of this field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LineEndingMode {
    /// Force `\r\n` line endings.
    Crlf,
    /// Force `\n` line endings.
    Lf,
    /// Preserve whatever the source has.
    #[default]
    AsIs,
}

/// What to do when the source filename is a Windows reserved device
/// name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ReservedNameStrategy {
    /// Append `_` between the stem and extension — `CON.txt` →
    /// `CON_.txt`. Default because it keeps the copy moving without
    /// a user prompt.
    #[default]
    Suffix,
    /// Surface [`TranslateError::ReservedName`] so the UI can flag
    /// the file. The caller (typically the aggregate conflict
    /// dialog) decides whether to skip or have the user rename.
    Reject,
}

/// What to do when a destination path would exceed Windows MAX_PATH.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LongPathStrategy {
    /// Prefix with `\\?\` (or `\\?\UNC\` for SMB shares). Windows 10
    /// 1607+ handles both transparently; older tools that open the
    /// resulting path may need extended-length awareness.
    #[default]
    Win32LongPath,
    /// Truncate the filename stem until the composed path fits
    /// within MAX_PATH. Extension preserved; a `-N` discriminator is
    /// appended to keep the truncation reversible-enough for sync
    /// identity. Last resort — most users prefer the long-path path.
    Truncate,
    /// Surface [`TranslateError::PathTooLong`]. Caller decides.
    Reject,
}

/// End-to-end translation policy driving [`translate_path`].
///
/// Each field has a sensible default; the typical caller instantiates
/// with `PathPolicy::default()` and flips one or two knobs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathPolicy {
    pub target_os: TargetOs,
    pub unicode_normalization: NormalizationMode,
    pub line_endings: LineEndingMode,
    pub reserved_name_strategy: ReservedNameStrategy,
    pub long_path_strategy: LongPathStrategy,
    /// Lowercase extensions (no leading dot) eligible for line-ending
    /// conversion. Only consulted when `line_endings != AsIs`.
    pub line_ending_allowlist: Vec<String>,
}

impl Default for PathPolicy {
    fn default() -> Self {
        Self {
            target_os: TargetOs::Auto,
            unicode_normalization: NormalizationMode::Auto,
            line_endings: LineEndingMode::AsIs,
            reserved_name_strategy: ReservedNameStrategy::Suffix,
            long_path_strategy: LongPathStrategy::Win32LongPath,
            line_ending_allowlist: default_text_extensions(),
        }
    }
}

/// The extension allowlist the brief calls out. Lowercase; no leading
/// dot. Callers can extend or replace this at the settings layer.
pub fn default_text_extensions() -> Vec<String> {
    [
        "txt", "md", "csv", "json", "xml", "yaml", "yml", "ini", "conf", "sh", "py", "rs", "ts",
        "js", "css", "html",
    ]
    .iter()
    .map(|s| (*s).to_string())
    .collect()
}

// ---------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------

#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum TranslateError {
    /// Filename is a Windows reserved device name (`CON`, `LPT1`, …)
    /// and the caller selected [`ReservedNameStrategy::Reject`].
    #[error("filename `{name}` is reserved on Windows (device name)")]
    ReservedName { name: String },

    /// Destination path would exceed Windows MAX_PATH (260 chars)
    /// and the caller selected [`LongPathStrategy::Reject`].
    #[error("destination path length {len} exceeds Windows MAX_PATH (260)")]
    PathTooLong { len: usize },

    /// Source path has no filename component (e.g. `"/"` or empty).
    #[error("source path has no filename component")]
    NoFileName,

    /// Destination root must be absolute when the `\\?\` long-path
    /// prefix is applied — the namespace only accepts fully-qualified
    /// paths, not relative ones.
    #[error("destination root must be absolute to apply the Windows long-path prefix")]
    RelativeDstRoot,
}

impl TranslateError {
    /// Fluent key mirroring the existing `localized_key` convention.
    pub const fn localized_key(&self) -> &'static str {
        match self {
            Self::ReservedName { .. } => "err-translate-reserved-name",
            Self::PathTooLong { .. } => "err-translate-path-too-long",
            Self::NoFileName => "err-translate-no-filename",
            Self::RelativeDstRoot => "err-translate-relative-dst",
        }
    }
}

// ---------------------------------------------------------------------
// Reserved Windows device names
// ---------------------------------------------------------------------

/// Uppercased Windows reserved device names. Case-insensitive match
/// applied to the filename stem (everything before the first `.`).
pub const WINDOWS_RESERVED_STEMS: &[&str] = &[
    "CON", "PRN", "AUX", "NUL", "COM0", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7",
    "COM8", "COM9", "LPT0", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
];

/// Returns `true` if `name` (with or without an extension) is a
/// Windows reserved device name.
///
/// Match is case-insensitive and applies to the portion of `name`
/// before the first `.`, so `Con.TXT`, `con`, and `CON.log.old` all
/// classify as reserved.
pub fn is_reserved_windows_name(name: &str) -> bool {
    let stem = name.split('.').next().unwrap_or(name);
    let upper = stem.to_ascii_uppercase();
    WINDOWS_RESERVED_STEMS.iter().any(|r| *r == upper)
}

/// Append `_` between the stem and extension. Idempotent — applying
/// twice produces `CON__.txt`; callers should only invoke after
/// [`is_reserved_windows_name`] returns `true`.
pub fn apply_reserved_suffix(name: &str) -> String {
    if let Some(dot) = name.find('.') {
        let (stem, ext) = name.split_at(dot);
        format!("{stem}_{ext}")
    } else {
        format!("{name}_")
    }
}

// ---------------------------------------------------------------------
// Line-ending detection & rewrite
// ---------------------------------------------------------------------

/// Dominant line ending detected in a byte buffer.
///
/// `Mixed` fires when both CRLF and bare LF occur. The translator
/// treats `Mixed` the same as the source-majority form — callers can
/// branch on it to log a warning if they prefer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineEnding {
    /// `\r\n` — Windows-style.
    Crlf,
    /// `\n` — Unix-style.
    Lf,
    /// Both appeared in the sample.
    Mixed,
    /// Neither appeared in the sample (no newlines at all).
    None,
}

/// Probe the first `sample_limit` bytes (or the whole buffer if
/// shorter) and classify the dominant line ending. Free of
/// allocation; safe on non-UTF-8 bytes.
pub fn detect_line_ending_in(sample: &[u8]) -> LineEnding {
    let mut crlf = 0usize;
    let mut bare_lf = 0usize;
    let mut i = 0;
    while i < sample.len() {
        if sample[i] == b'\n' {
            if i > 0 && sample[i - 1] == b'\r' {
                crlf += 1;
            } else {
                bare_lf += 1;
            }
        }
        i += 1;
    }
    match (crlf, bare_lf) {
        (0, 0) => LineEnding::None,
        (c, 0) if c > 0 => LineEnding::Crlf,
        (0, l) if l > 0 => LineEnding::Lf,
        _ => LineEnding::Mixed,
    }
}

/// Convenience wrapper with the default 64 KiB sample cap. Use
/// [`detect_line_ending_in`] directly when a custom cap is useful.
pub fn detect_line_ending(bytes: &[u8]) -> LineEnding {
    const SAMPLE_CAP: usize = 64 * 1024;
    let slice = if bytes.len() > SAMPLE_CAP {
        &bytes[..SAMPLE_CAP]
    } else {
        bytes
    };
    detect_line_ending_in(slice)
}

/// Rewrite `bytes` so every CRLF / bare-LF line terminator matches
/// `target`. Byte-for-byte copy when the source already matches
/// (no allocation in the fast path).
///
/// `target == LineEndingMode::AsIs` is a no-op — the function
/// returns a cloned buffer unchanged so callers don't branch on the
/// mode.
pub fn translate_content_line_endings(bytes: &[u8], target: LineEndingMode) -> Vec<u8> {
    match target {
        LineEndingMode::AsIs => bytes.to_vec(),
        LineEndingMode::Crlf => to_crlf(bytes),
        LineEndingMode::Lf => to_lf(bytes),
    }
}

fn to_lf(bytes: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\r' && i + 1 < bytes.len() && bytes[i + 1] == b'\n' {
            out.push(b'\n');
            i += 2;
        } else if bytes[i] == b'\r' {
            // Lone CR (classic Mac). Treat as LF — the user asked
            // for Unix endings and an isolated CR has no better
            // interpretation.
            out.push(b'\n');
            i += 1;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    out
}

fn to_crlf(bytes: &[u8]) -> Vec<u8> {
    // First normalize to LF, then expand every LF to CRLF. Two-pass
    // so a mixed-ending source ends up uniform.
    let lf_form = to_lf(bytes);
    let mut out = Vec::with_capacity(lf_form.len() + lf_form.len() / 32);
    for b in lf_form {
        if b == b'\n' {
            out.push(b'\r');
            out.push(b'\n');
        } else {
            out.push(b);
        }
    }
    out
}

/// Returns `true` if `name`'s extension is on `allowlist`.
/// Case-insensitive; a file with no extension always returns `false`.
pub fn should_translate_extension(name: &str, allowlist: &[String]) -> bool {
    let Some(dot) = name.rfind('.') else {
        return false;
    };
    let ext_lower = name[dot + 1..].to_ascii_lowercase();
    allowlist.iter().any(|a| a.eq_ignore_ascii_case(&ext_lower))
}

// ---------------------------------------------------------------------
// Target-OS resolution + Unicode normalization
// ---------------------------------------------------------------------

/// Resolve [`TargetOs::Auto`] to a concrete platform using `cfg!`.
pub fn resolve_target_os(target: TargetOs) -> TargetOs {
    match target {
        TargetOs::Auto => {
            if cfg!(target_os = "windows") {
                TargetOs::Windows
            } else if cfg!(target_os = "macos") {
                TargetOs::MacOs
            } else {
                TargetOs::Linux
            }
        }
        other => other,
    }
}

/// Normalize a filename string according to the policy's
/// `unicode_normalization` knob. Returns the (possibly rewritten)
/// name plus a `changed` flag so callers can decide whether to emit
/// `CopyEvent::UnicodeRenormalized`.
pub fn normalize_name(
    name: &str,
    policy: &PathPolicy,
    effective_target: TargetOs,
) -> (String, bool) {
    use unicode_normalization::UnicodeNormalization;

    let mode = match policy.unicode_normalization {
        NormalizationMode::AsIs => return (name.to_string(), false),
        NormalizationMode::Nfc => NormalizationMode::Nfc,
        NormalizationMode::Nfd => NormalizationMode::Nfd,
        NormalizationMode::Auto => match effective_target {
            TargetOs::Windows | TargetOs::Linux => NormalizationMode::Nfc,
            TargetOs::MacOs => return (name.to_string(), false),
            TargetOs::Auto => NormalizationMode::Nfc,
        },
    };
    let out: String = match mode {
        NormalizationMode::Nfc => name.nfc().collect(),
        NormalizationMode::Nfd => name.nfd().collect(),
        // AsIs + Auto are already handled above.
        _ => name.to_string(),
    };
    let changed = out != name;
    (out, changed)
}

// ---------------------------------------------------------------------
// Long-path prefixing
// ---------------------------------------------------------------------

/// Windows MAX_PATH including the trailing NUL.
pub const WINDOWS_MAX_PATH: usize = 260;

/// Prefix `path` with the Windows extended-length namespace.
///
/// - `\\server\share\foo` becomes `\\?\UNC\server\share\foo`.
/// - `C:\foo\bar` becomes `\\?\C:\foo\bar`.
/// - Already-prefixed paths are returned unchanged.
/// - Relative paths are returned unchanged with no error — the gate
///   in [`translate_path`] is responsible for rejecting them.
pub fn apply_long_path_prefix(path: &Path) -> PathBuf {
    let s = path.to_string_lossy();
    // Refuse to wrap a `..`-laden path: the `\\?\` namespace turns
    // off Win32 path normalisation, so `..` would survive into the
    // kernel-resolved path and escape the lexical-traversal guard
    // that fires before this prefix is applied. Caller sees the
    // unprefixed path and the engine's metadata pre-flight will
    // reject it with the normal Phase 17a error.
    if s.contains("..") {
        return path.to_path_buf();
    }
    if s.starts_with(r"\\?\") {
        return path.to_path_buf();
    }
    if let Some(rest) = s.strip_prefix(r"\\") {
        return PathBuf::from(format!(r"\\?\UNC\{rest}"));
    }
    // Bare drive path (`C:\…`) or any other absolute-looking form.
    PathBuf::from(format!(r"\\?\{s}"))
}

/// Approximate length of the path as the Windows API would see it
/// (UTF-16 code units). ASCII code points count as 1; combining-mark
/// code points count as 1 (one `u16` each); supplementary-plane
/// characters count as 2 (surrogate pair). Accurate enough for the
/// `> MAX_PATH` gate.
pub fn path_utf16_len(path: &Path) -> usize {
    path.to_string_lossy()
        .chars()
        .map(|c| if c as u32 > 0xFFFF { 2 } else { 1 })
        .sum()
}

// ---------------------------------------------------------------------
// translate_path — the main entry point
// ---------------------------------------------------------------------

/// Produce the destination path for `src` under `dst_root` that
/// satisfies `policy` on the target platform.
///
/// The return value is the fully composed destination (directory +
/// translated filename), with reserved-name suffix applied, Windows
/// long-path prefix applied when the policy calls for it, and
/// Unicode normalization folded in.
///
/// The source path is consumed only for its filename component; no
/// filesystem access occurs. Callers that already know the source
/// is a directory (and therefore has no "filename" in the usual
/// sense) can pass the trailing path segment directly wrapped in a
/// `PathBuf`.
pub fn translate_path(
    src: &Path,
    dst_root: &Path,
    policy: &PathPolicy,
) -> Result<PathBuf, TranslateError> {
    let filename_os = src.file_name().ok_or(TranslateError::NoFileName)?;
    let filename_str = filename_os.to_string_lossy();
    let effective_target = resolve_target_os(policy.target_os);

    // 1. Unicode normalization on the filename component.
    let (normalized, _changed) = normalize_name(&filename_str, policy, effective_target);

    // 2. Reserved-name handling. Apply unconditionally regardless
    // of `effective_target` — a Linux-generated tree containing
    // `CON.txt` / `LPT1.log` that is later mounted on Windows
    // (zip archive, network share, removable drive) would
    // otherwise resolve those names to console / serial-port
    // devices when the Windows-side reader opens them. The cost
    // on a non-Windows-native dst is one underscore per pathological
    // filename and zero behaviour change for the typical case.
    let adjusted = if is_reserved_windows_name(&normalized) {
        match policy.reserved_name_strategy {
            ReservedNameStrategy::Suffix => apply_reserved_suffix(&normalized),
            ReservedNameStrategy::Reject => {
                return Err(TranslateError::ReservedName { name: normalized });
            }
        }
    } else {
        normalized
    };

    // 3. Compose destination. Simplify any incoming `\\?\` so our
    //    length gate sees the user-visible form.
    let simplified_root = dunce::simplified(dst_root).to_path_buf();
    let composed = simplified_root.join(&adjusted);

    // 4. Long-path handling (Windows only).
    if effective_target == TargetOs::Windows {
        let utf16_len = path_utf16_len(&composed);
        if utf16_len > WINDOWS_MAX_PATH {
            match policy.long_path_strategy {
                LongPathStrategy::Win32LongPath => {
                    if !composed.is_absolute() {
                        return Err(TranslateError::RelativeDstRoot);
                    }
                    return Ok(apply_long_path_prefix(&composed));
                }
                LongPathStrategy::Truncate => {
                    return Ok(truncate_to_max_path(&composed));
                }
                LongPathStrategy::Reject => {
                    return Err(TranslateError::PathTooLong { len: utf16_len });
                }
            }
        }
    }

    Ok(composed)
}

/// Shrink `path`'s final segment until the composed path fits inside
/// [`WINDOWS_MAX_PATH`], preserving the extension and appending a
/// trailing `-t` to distinguish the truncated form. Last-ditch
/// fallback for [`LongPathStrategy::Truncate`].
fn truncate_to_max_path(path: &Path) -> PathBuf {
    const SUFFIX: &str = "-t";
    let Some(parent) = path.parent() else {
        return path.to_path_buf();
    };
    let Some(filename) = path.file_name().map(|f| f.to_string_lossy().into_owned()) else {
        return path.to_path_buf();
    };
    let (stem, ext) = match filename.rfind('.') {
        Some(dot) => (filename[..dot].to_string(), filename[dot..].to_string()),
        None => (filename.clone(), String::new()),
    };
    let parent_len = path_utf16_len(parent);
    // `parent + sep + stem' + SUFFIX + ext` must fit in MAX_PATH.
    let sep_len = 1;
    let budget =
        WINDOWS_MAX_PATH.saturating_sub(parent_len + sep_len + SUFFIX.len() + ext.chars().count());
    let mut truncated_stem: String = stem.chars().take(budget).collect();
    if truncated_stem.is_empty() {
        truncated_stem.push('x');
    }
    parent.join(format!("{truncated_stem}{SUFFIX}{ext}"))
}

// ---------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn reserved_name_matches_case_insensitive() {
        assert!(is_reserved_windows_name("CON"));
        assert!(is_reserved_windows_name("con"));
        assert!(is_reserved_windows_name("Con.txt"));
        assert!(is_reserved_windows_name("LPT1"));
        assert!(is_reserved_windows_name("lpt9.log.old"));
        assert!(!is_reserved_windows_name("COMS"));
        assert!(!is_reserved_windows_name("CONS.txt"));
        assert!(!is_reserved_windows_name("readme.txt"));
    }

    #[test]
    fn reserved_suffix_inserts_underscore() {
        assert_eq!(apply_reserved_suffix("CON"), "CON_");
        assert_eq!(apply_reserved_suffix("CON.txt"), "CON_.txt");
        assert_eq!(apply_reserved_suffix("LPT1.log.old"), "LPT1_.log.old");
    }

    #[test]
    fn nfc_changes_nfd_resume() {
        let nfd = "re\u{0301}sume\u{0301}.pdf";
        let policy = PathPolicy {
            target_os: TargetOs::Windows,
            unicode_normalization: NormalizationMode::Auto,
            ..PathPolicy::default()
        };
        let (out, changed) = normalize_name(nfd, &policy, TargetOs::Windows);
        assert_eq!(out, "résumé.pdf");
        assert!(changed);
    }

    #[test]
    fn auto_leaves_macos_unchanged() {
        let nfd = "re\u{0301}sume\u{0301}.pdf";
        let policy = PathPolicy {
            target_os: TargetOs::MacOs,
            unicode_normalization: NormalizationMode::Auto,
            ..PathPolicy::default()
        };
        let (out, changed) = normalize_name(nfd, &policy, TargetOs::MacOs);
        assert_eq!(out, nfd);
        assert!(!changed);
    }

    #[test]
    fn translate_path_nfd_to_windows() {
        let src: PathBuf = "/tmp/re\u{0301}sume\u{0301}.pdf".into();
        let dst_root = if cfg!(windows) {
            PathBuf::from(r"C:\out")
        } else {
            PathBuf::from("/out")
        };
        let policy = PathPolicy {
            target_os: TargetOs::Windows,
            unicode_normalization: NormalizationMode::Auto,
            ..PathPolicy::default()
        };
        let out = translate_path(&src, &dst_root, &policy).unwrap();
        assert!(out.to_string_lossy().contains("résumé.pdf"));
    }

    #[test]
    fn translate_path_reserved_suffix() {
        let src = PathBuf::from("/src/CON.txt");
        let dst_root = if cfg!(windows) {
            PathBuf::from(r"C:\out")
        } else {
            PathBuf::from("/out")
        };
        let policy = PathPolicy {
            target_os: TargetOs::Windows,
            reserved_name_strategy: ReservedNameStrategy::Suffix,
            ..PathPolicy::default()
        };
        let out = translate_path(&src, &dst_root, &policy).unwrap();
        assert!(out.file_name().unwrap().to_string_lossy() == "CON_.txt");
    }

    #[test]
    fn translate_path_reserved_reject() {
        let src = PathBuf::from("/src/CON.txt");
        let dst_root = if cfg!(windows) {
            PathBuf::from(r"C:\out")
        } else {
            PathBuf::from("/out")
        };
        let policy = PathPolicy {
            target_os: TargetOs::Windows,
            reserved_name_strategy: ReservedNameStrategy::Reject,
            ..PathPolicy::default()
        };
        let err = translate_path(&src, &dst_root, &policy).unwrap_err();
        assert!(matches!(err, TranslateError::ReservedName { .. }));
    }

    #[test]
    fn translate_path_reserved_applied_regardless_of_target_os() {
        // Phase 17 follow-up — reserved-name handling now fires
        // even when target_os is Linux, because a Linux-side dst
        // can later be mounted on Windows (zip / network share /
        // removable drive) and a literal `CON.txt` would resolve
        // to the console device. The Reject strategy therefore
        // refuses on Linux too.
        let src = PathBuf::from("/src/CON.txt");
        let dst_root = PathBuf::from("/out");
        let policy = PathPolicy {
            target_os: TargetOs::Linux,
            reserved_name_strategy: ReservedNameStrategy::Reject,
            ..PathPolicy::default()
        };
        let err = translate_path(&src, &dst_root, &policy).unwrap_err();
        assert!(matches!(err, TranslateError::ReservedName { .. }));
    }

    #[test]
    fn translate_path_reserved_suffix_on_linux_target() {
        // Default strategy is Suffix — verify the Linux target
        // also gets the underscore appended.
        let src = PathBuf::from("/src/LPT1.txt");
        let dst_root = PathBuf::from("/out");
        let policy = PathPolicy {
            target_os: TargetOs::Linux,
            reserved_name_strategy: ReservedNameStrategy::Suffix,
            ..PathPolicy::default()
        };
        let out = translate_path(&src, &dst_root, &policy).unwrap();
        let name = out.file_name().unwrap().to_string_lossy().into_owned();
        assert_ne!(name, "LPT1.txt");
        assert!(name.starts_with("LPT1") || name.starts_with("LPT1_"));
    }

    #[test]
    fn apply_long_path_prefix_drive() {
        let out = apply_long_path_prefix(Path::new(r"C:\very\long\path"));
        assert_eq!(out.to_string_lossy(), r"\\?\C:\very\long\path");
    }

    #[test]
    fn apply_long_path_prefix_unc() {
        let out = apply_long_path_prefix(Path::new(r"\\server\share\folder"));
        assert_eq!(out.to_string_lossy(), r"\\?\UNC\server\share\folder");
    }

    #[test]
    fn apply_long_path_prefix_idempotent() {
        let already = Path::new(r"\\?\C:\path");
        let out = apply_long_path_prefix(already);
        assert_eq!(out, already);
    }

    #[test]
    fn translate_path_long_path_prefix_applied() {
        // Compose a >260-char destination path on Windows.
        let long = "x".repeat(250);
        let src = PathBuf::from(format!("/src/{long}.bin"));
        let dst_root = PathBuf::from(r"C:\out");
        let policy = PathPolicy {
            target_os: TargetOs::Windows,
            long_path_strategy: LongPathStrategy::Win32LongPath,
            ..PathPolicy::default()
        };
        let out = translate_path(&src, &dst_root, &policy).unwrap();
        let s = out.to_string_lossy();
        assert!(s.starts_with(r"\\?\"), "expected long-path prefix, got {s}");
    }

    #[test]
    fn translate_path_long_path_reject() {
        let long = "y".repeat(260);
        let src = PathBuf::from(format!("/src/{long}.bin"));
        let dst_root = PathBuf::from(r"C:\out");
        let policy = PathPolicy {
            target_os: TargetOs::Windows,
            long_path_strategy: LongPathStrategy::Reject,
            ..PathPolicy::default()
        };
        let err = translate_path(&src, &dst_root, &policy).unwrap_err();
        assert!(matches!(err, TranslateError::PathTooLong { .. }));
    }

    #[test]
    fn translate_path_no_filename_errors() {
        let src = PathBuf::from("/");
        let dst_root = PathBuf::from("/dst");
        let err = translate_path(&src, &dst_root, &PathPolicy::default()).unwrap_err();
        assert_eq!(err, TranslateError::NoFileName);
    }

    #[test]
    fn detect_line_ending_classifies_crlf() {
        let crlf = b"line1\r\nline2\r\nline3";
        assert_eq!(detect_line_ending(crlf), LineEnding::Crlf);
    }

    #[test]
    fn detect_line_ending_classifies_lf() {
        let lf = b"line1\nline2\nline3";
        assert_eq!(detect_line_ending(lf), LineEnding::Lf);
    }

    #[test]
    fn detect_line_ending_classifies_mixed() {
        let mixed = b"line1\r\nline2\nline3";
        assert_eq!(detect_line_ending(mixed), LineEnding::Mixed);
    }

    #[test]
    fn detect_line_ending_classifies_empty() {
        assert_eq!(detect_line_ending(b""), LineEnding::None);
        assert_eq!(detect_line_ending(b"no newlines here"), LineEnding::None);
    }

    #[test]
    fn translate_content_crlf_to_lf() {
        let crlf = b"a\r\nb\r\nc";
        let out = translate_content_line_endings(crlf, LineEndingMode::Lf);
        assert_eq!(out, b"a\nb\nc");
    }

    #[test]
    fn translate_content_lf_to_crlf() {
        let lf = b"a\nb\nc";
        let out = translate_content_line_endings(lf, LineEndingMode::Crlf);
        assert_eq!(out, b"a\r\nb\r\nc");
    }

    #[test]
    fn translate_content_mixed_to_lf() {
        let mixed = b"a\r\nb\nc\rd";
        let out = translate_content_line_endings(mixed, LineEndingMode::Lf);
        assert_eq!(out, b"a\nb\nc\nd");
    }

    #[test]
    fn translate_content_as_is_is_identity() {
        let mixed = b"a\r\nb\nc";
        let out = translate_content_line_endings(mixed, LineEndingMode::AsIs);
        assert_eq!(out, mixed);
    }

    #[test]
    fn extension_allowlist_matches_case_insensitively() {
        let allow = default_text_extensions();
        assert!(should_translate_extension("readme.md", &allow));
        assert!(should_translate_extension("README.MD", &allow));
        assert!(should_translate_extension("main.rs", &allow));
        assert!(!should_translate_extension("photo.jpg", &allow));
        assert!(!should_translate_extension("no-extension", &allow));
    }

    #[test]
    fn path_utf16_len_counts_supplementary_as_two() {
        // ASCII: 1 per char.
        assert_eq!(path_utf16_len(Path::new("abc")), 3);
        // Supplementary-plane glyph (musical symbol 𝄞, U+1D11E) is
        // encoded as a UTF-16 surrogate pair (2 code units).
        assert_eq!(path_utf16_len(Path::new("a𝄞b")), 4);
    }
}
