//! Lightweight path-to-icon classifier.
//!
//! The frontend picks a matching Lucide glyph from the returned
//! `kind`. Phase 7 will extend this with platform-native file-type
//! icon lookups (SHGetFileInfo / NSWorkspace / GIO) that return a
//! raster image the UI can display as-is.

use std::path::Path;

use crate::ipc::FileIconDto;

pub fn classify(path: &Path) -> FileIconDto {
    // Missing paths degrade gracefully to "file" — callers sometimes
    // ask about a destination that doesn't exist yet (e.g. a
    // pending job), and the UI still wants a glyph.
    let kind = match std::fs::symlink_metadata(path) {
        Ok(meta) if meta.file_type().is_dir() => "folder",
        Ok(meta) if meta.file_type().is_symlink() => "symlink",
        _ => bucket(path),
    };
    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_ascii_lowercase());
    FileIconDto { kind, extension }
}

fn bucket(path: &Path) -> &'static str {
    let Some(ext) = path.extension().and_then(|e| e.to_str()) else {
        return "file";
    };
    match ext.to_ascii_lowercase().as_str() {
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "svg" | "tif" | "tiff" | "heic"
        | "avif" => "image",
        "mp3" | "wav" | "flac" | "ogg" | "m4a" | "aac" | "opus" => "audio",
        "mp4" | "m4v" | "mkv" | "mov" | "webm" | "avi" | "wmv" => "video",
        "zip" | "tar" | "gz" | "bz2" | "xz" | "7z" | "rar" | "zst" | "tgz" | "tbz2" => "archive",
        "txt" | "md" | "rst" | "log" | "csv" | "tsv" | "json" | "yaml" | "yml" | "toml" | "xml"
        | "ini" => "text",
        "rs" | "py" | "js" | "ts" | "jsx" | "tsx" | "c" | "h" | "cpp" | "hpp" | "cs" | "go"
        | "java" | "kt" | "rb" | "swift" | "php" | "sh" | "ps1" | "bash" | "zsh" | "lua" => "code",
        "pdf" => "pdf",
        "exe" | "msi" | "dmg" | "app" | "deb" | "rpm" | "appimage" | "apk" => "binary",
        _ => "file",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_known_buckets() {
        assert_eq!(bucket(Path::new("photo.jpg")), "image");
        assert_eq!(bucket(Path::new("song.flac")), "audio");
        assert_eq!(bucket(Path::new("clip.mp4")), "video");
        assert_eq!(bucket(Path::new("release.tar.gz")), "archive");
        assert_eq!(bucket(Path::new("README.md")), "text");
        assert_eq!(bucket(Path::new("lib.rs")), "code");
        assert_eq!(bucket(Path::new("installer.msi")), "binary");
        assert_eq!(bucket(Path::new("doc.pdf")), "pdf");
    }

    #[test]
    fn unknown_extension_falls_back_to_file() {
        assert_eq!(bucket(Path::new("mystery.xyz")), "file");
        assert_eq!(bucket(Path::new("noext")), "file");
    }

    #[test]
    fn classify_picks_folder_for_current_dir() {
        let dto = classify(Path::new("."));
        assert_eq!(dto.kind, "folder");
    }
}
