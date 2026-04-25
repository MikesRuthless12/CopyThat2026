//! TeraCopy-compatible sidecar files.
//!
//! TeraCopy writes `<hex>  <relpath>` — two spaces between the digest
//! and the path — which is also the output format of GNU coreutils
//! `sha256sum`, `md5sum`, and the BLAKE3 `b3sum` tool. That's the
//! format we produce and the format we accept.
//!
//! A tree sidecar is just the same format with one line per file, which
//! means `sha256sum -c foo.sha256` works end-to-end for the algorithms
//! that have a coreutils counterpart.

use std::fmt;
use std::path::{Path, PathBuf};

use tokio::io::AsyncWriteExt;

use crate::algorithm::HashAlgorithm;

/// A single entry in a sidecar file: hex digest + source-relative path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SidecarEntry {
    pub hex_digest: String,
    pub relative_path: PathBuf,
}

impl SidecarEntry {
    pub fn new(hex_digest: impl Into<String>, relative_path: impl Into<PathBuf>) -> Self {
        Self {
            hex_digest: hex_digest.into(),
            relative_path: relative_path.into(),
        }
    }
}

impl fmt::Display for SidecarEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Two spaces between the digest and the path is the coreutils
        // "binary mode" convention, which every matching tool groks.
        write!(
            f,
            "{}  {}",
            self.hex_digest,
            display_forward(&self.relative_path)
        )
    }
}

/// Format a path with forward slashes for portability. Sidecar files
/// generated on Windows should still verify against coreutils on
/// Linux; writing them with native separators breaks that contract.
fn display_forward(p: &Path) -> String {
    let s = p.to_string_lossy();
    if cfg!(windows) {
        s.replace('\\', "/")
    } else {
        s.into_owned()
    }
}

/// Phase 17f — refuse to write sidecar entries whose path component
/// is absolute or contains a parent-dir (`..`) segment. The
/// TeraCopy / coreutils sidecar convention is **job-root-relative**:
/// an entry with an absolute path silently leaks the user's
/// directory layout (`/home/alice/private/.git/...`) into a file
/// the `sha256sum -c` consumer expects to be portable. Reject at
/// write time so the on-disk format stays consistent.
///
/// Returns `Ok(())` for an acceptable relative path and an
/// `Err(io::Error)` with `ErrorKind::InvalidInput` on rejection.
pub fn validate_sidecar_relpath(p: &Path) -> std::io::Result<()> {
    use std::path::Component;
    if p.is_absolute() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!(
                "sidecar entry must use a job-root-relative path, got absolute: {}",
                p.display()
            ),
        ));
    }
    for c in p.components() {
        if matches!(
            c,
            Component::ParentDir | Component::Prefix(_) | Component::RootDir
        ) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!(
                    "sidecar entry contains forbidden path segment ({c:?}): {}",
                    p.display()
                ),
            ));
        }
    }
    Ok(())
}

/// Write a single-file sidecar alongside `dst`: `<dst>.<ext>`.
///
/// Returns the path of the sidecar that was created. `hex` must be the
/// lowercase hex representation of the digest. Fails if the algorithm
/// is not one that has a sidecar extension (CRC32 / xxHash3 variants).
///
/// Phase 17f — the entry written into the sidecar is `dst.file_name()`
/// only. If the destination has no file_name (degenerate input — a
/// path ending in `..`, a Windows drive root, etc.) we now fail with
/// `InvalidInput` rather than silently writing the absolute path.
pub async fn write_single_file_sidecar(
    dst: &Path,
    algorithm: HashAlgorithm,
    hex: &str,
) -> std::io::Result<PathBuf> {
    let ext = algorithm.sidecar_extension().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            format!("algorithm {algorithm} has no sidecar format"),
        )
    })?;
    let relative = dst.file_name().map(PathBuf::from).ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!(
                "sidecar relpath cannot be derived from {} (no file_name component)",
                dst.display()
            ),
        )
    })?;
    // Defence-in-depth: even file_name() should be a single component.
    // If a malicious dst ever produces something multi-component, the
    // validator surfaces it before we open the sidecar.
    validate_sidecar_relpath(&relative)?;
    let sidecar_path = append_extension(dst, ext);
    let line = SidecarEntry::new(hex.to_string(), relative).to_string();
    let mut file = tokio::fs::File::create(&sidecar_path).await?;
    file.write_all(line.as_bytes()).await?;
    file.write_all(b"\n").await?;
    file.flush().await?;
    Ok(sidecar_path)
}

/// Write a batch sidecar at `sidecar_path` covering every entry in
/// `entries`. Caller supplies the entries — this module doesn't walk a
/// tree itself so the higher-level verify pipeline stays in charge of
/// path layout.
///
/// Phase 17f — every entry's `relative_path` is validated through
/// [`validate_sidecar_relpath`] before any byte is written. The
/// first absolute or `..`-laden entry fails the whole write, so
/// callers can't accidentally publish a sidecar mixing relative
/// and absolute paths.
pub async fn write_tree_sidecar(
    sidecar_path: &Path,
    entries: &[SidecarEntry],
) -> std::io::Result<()> {
    for entry in entries {
        validate_sidecar_relpath(&entry.relative_path)?;
    }
    let mut file = tokio::fs::File::create(sidecar_path).await?;
    for entry in entries {
        let line = entry.to_string();
        file.write_all(line.as_bytes()).await?;
        file.write_all(b"\n").await?;
    }
    file.flush().await?;
    Ok(())
}

/// Parse a single coreutils-style line into a `SidecarEntry`. Returns
/// `None` for blank lines and comment lines (prefixed with `#`). The
/// shape accepts both `<hex>  <path>` (two spaces, binary mode) and
/// `<hex>  *<path>` (coreutils binary-mode "asterisk" prefix), plus the
/// single-space "text mode" fallback.
pub fn parse_line(line: &str) -> Option<SidecarEntry> {
    let trimmed = line.trim_end_matches(['\n', '\r']);
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return None;
    }
    // Split on the first run of whitespace.
    let mut iter = trimmed.splitn(2, char::is_whitespace);
    let hex = iter.next()?.trim();
    let rest = iter.next()?.trim_start();
    // coreutils binary-mode tag (`*`). Strip it; we don't track mode.
    let path_str = rest.strip_prefix('*').unwrap_or(rest);
    if hex.is_empty() || path_str.is_empty() {
        return None;
    }
    Some(SidecarEntry::new(hex.to_string(), PathBuf::from(path_str)))
}

/// Parse a whole sidecar file's contents.
pub fn parse_sidecar(contents: &str) -> Vec<SidecarEntry> {
    contents.lines().filter_map(parse_line).collect()
}

fn append_extension(p: &Path, ext: &str) -> PathBuf {
    // We intentionally append (`file.bin` → `file.bin.sha256`) rather
    // than replace; this matches TeraCopy and ensures two sidecars on
    // the same file can coexist (`.sha256` and `.blake3`).
    let mut s = p.as_os_str().to_owned();
    s.push(".");
    s.push(ext);
    PathBuf::from(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn entry_round_trips() {
        let entry = SidecarEntry::new(
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string(),
            PathBuf::from("sub/empty.bin"),
        );
        let line = entry.to_string();
        let parsed = parse_line(&line).unwrap();
        assert_eq!(parsed, entry);
    }

    #[test]
    fn parse_accepts_coreutils_asterisk_mode() {
        let parsed = parse_line("abcd  *path/with space.bin").unwrap();
        assert_eq!(parsed.hex_digest, "abcd");
        assert_eq!(parsed.relative_path, PathBuf::from("path/with space.bin"));
    }

    #[test]
    fn parse_skips_blank_and_comment_lines() {
        assert!(parse_line("").is_none());
        assert!(parse_line("   ").is_none());
        assert!(parse_line("# a comment").is_none());
    }

    #[test]
    fn forward_slash_in_display() {
        let p: PathBuf = ["sub", "nested", "file.bin"].iter().collect();
        let s = display_forward(&p);
        assert_eq!(s, "sub/nested/file.bin");
    }

    #[tokio::test]
    async fn write_and_parse_tree_sidecar_roundtrip() {
        let dir = tempdir().unwrap();
        let sidecar = dir.path().join("all.sha256");

        let entries = vec![
            SidecarEntry::new("aa".repeat(32), PathBuf::from("one.bin")),
            SidecarEntry::new("bb".repeat(32), PathBuf::from("sub/two.bin")),
        ];
        write_tree_sidecar(&sidecar, &entries).await.unwrap();

        let contents = tokio::fs::read_to_string(&sidecar).await.unwrap();
        let parsed = parse_sidecar(&contents);
        assert_eq!(parsed, entries);
    }

    #[tokio::test]
    async fn write_single_file_sidecar_adds_extension() {
        let dir = tempdir().unwrap();
        let dst = dir.path().join("file.bin");
        tokio::fs::write(&dst, b"payload").await.unwrap();

        let sidecar = write_single_file_sidecar(&dst, HashAlgorithm::Sha256, "deadbeef")
            .await
            .unwrap();
        assert!(
            sidecar.to_string_lossy().ends_with(".sha256"),
            "unexpected sidecar path: {}",
            sidecar.display()
        );
        let contents = tokio::fs::read_to_string(&sidecar).await.unwrap();
        assert_eq!(contents, "deadbeef  file.bin\n");
    }

    #[tokio::test]
    async fn write_single_file_sidecar_rejects_non_sidecar_algorithm() {
        let dir = tempdir().unwrap();
        let dst = dir.path().join("file.bin");
        tokio::fs::write(&dst, b"payload").await.unwrap();

        let err = write_single_file_sidecar(&dst, HashAlgorithm::Crc32, "deadbeef")
            .await
            .unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::Unsupported);
    }
}
