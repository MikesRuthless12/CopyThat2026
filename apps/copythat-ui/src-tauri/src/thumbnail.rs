//! Phase 22 — on-demand thumbnail generation for the aggregate
//! conflict dialog.
//!
//! `thumbnail_for(path, max_dim)` returns a [`ThumbnailDto`] the
//! Svelte frontend can drop into an `<img>` tag (for image files)
//! or hand to the existing `FileKindIcon` component (for PDFs,
//! videos, and anything else this pure-Rust pipeline can't decode).
//!
//! Pipeline:
//!
//! 1. Stat the file (size + mtime). Used for both the cache key and
//!    the early bail-out for huge-non-image blobs.
//! 2. If a cached PNG exists at `<config-dir>/thumb-cache/<key>.png`
//!    and its mtime is ≥ the source mtime, read + base64-encode +
//!    return. No decoding.
//! 3. Match on extension: image formats go through the `image`
//!    crate's `ImageReader::open().with_guessed_format().decode()`
//!    pipeline + `thumbnail(max, max)` downscale + PNG re-encode +
//!    disk cache write; everything else returns the file-kind icon
//!    variant (no `data_url`).
//!
//! Video (mp4/mov/webm) poster frames and PDF first-page rasters
//! were listed in the Phase 22 brief with explicit caveats: pure-
//! Rust decoders for both exist only as container parsers (not
//! frame extractors), and bundling a C codec library would break
//! the Phase 0 "$0 to build, commercially shippable" contract.
//! Falling back to the file-kind icon preserves feature parity
//! with the aggregate dialog's "thumbnail column exists" UX while
//! leaving the door open for a Phase 24+ privileged-helper-backed
//! OS thumbnailer bridge (Windows `IThumbnailProvider` / macOS
//! `QLThumbnailGenerator` / Linux `thumbnailer`).
//!
//! The cache directory is pruned elsewhere — this module only
//! writes. Lifetime management + GC happen in Phase 23's
//! `thumb-cache` sweeper (tracked in docs/ROADMAP.md).

use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use base64::Engine as _;
use base64::prelude::BASE64_STANDARD;

use crate::ipc::ThumbnailDto;

/// Default longest-edge pixel size the frontend asks for. Matches
/// the right-pane size the Phase 22 brief specified (240×240);
/// the left-rail uses 64 via the `max_dim` parameter.
pub const DEFAULT_MAX_DIM: u32 = 240;

/// Upper bound — refuse to decode huge blobs to avoid both OOM and
/// wall-time stalls on the first launch after a 50 GiB video drop.
/// 64 MiB covers every practical photo / small-video case; larger
/// files fall through to the icon variant.
const MAX_DECODE_BYTES: u64 = 64 * 1024 * 1024;

/// Generate a thumbnail for `path`. Errors only on unrecoverable
/// filesystem issues (parent-dir creation etc.); decode failures
/// return the icon fallback so a malformed image never breaks the
/// dialog.
pub fn thumbnail_for(path: &Path, max_dim: u32) -> ThumbnailDto {
    let max_dim = max_dim.clamp(16, 1024);
    let meta = match fs::metadata(path) {
        Ok(m) => m,
        Err(_) => return icon_fallback(path),
    };
    if !meta.is_file() {
        return icon_fallback(path);
    }
    let size = meta.len();
    let mtime_ms = meta
        .modified()
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);

    // Only image extensions go through the decode path. PDFs,
    // videos, archives, office docs → icon fallback.
    if !is_decodable_image(path) {
        return icon_fallback(path);
    }
    if size > MAX_DECODE_BYTES {
        return icon_fallback(path);
    }

    let cache_dir = cache_dir_for_thumbs();
    let cache_key = cache_key_for(path, size, mtime_ms, max_dim);
    let cache_path = cache_dir.as_ref().map(|d| d.join(&cache_key));
    if let Some(cp) = &cache_path
        && let Ok(cached_bytes) = fs::read(cp)
    {
        return ok_image(cached_bytes);
    }

    let bytes = match render_thumbnail(path, max_dim) {
        Ok(b) => b,
        Err(_) => return icon_fallback(path),
    };

    // Best-effort cache write — failure to persist does not fail
    // the request (worst case: next call re-decodes).
    if let (Some(dir), Some(cp)) = (cache_dir, cache_path) {
        let _ = fs::create_dir_all(&dir);
        let _ = fs::write(&cp, &bytes);
    }
    ok_image(bytes)
}

/// Best-effort cache-dir resolver. Returns `None` when the OS
/// config dir can't be resolved (sandboxed / cron-ish envs) — the
/// caller still renders, just without persistence.
fn cache_dir_for_thumbs() -> Option<PathBuf> {
    let dirs = directories::ProjectDirs::from("dev", "copythat", "CopyThat2026")?;
    Some(dirs.cache_dir().join("thumb-cache"))
}

fn cache_key_for(path: &Path, size: u64, mtime_ms: u64, max_dim: u32) -> String {
    let mut hasher = DefaultHasher::new();
    path.to_string_lossy().hash(&mut hasher);
    size.hash(&mut hasher);
    mtime_ms.hash(&mut hasher);
    max_dim.hash(&mut hasher);
    format!("{:016x}.png", hasher.finish())
}

fn render_thumbnail(path: &Path, max_dim: u32) -> Result<Vec<u8>, image::ImageError> {
    // `with_guessed_format()` reads the first few bytes to detect
    // the actual container (so a `.jpg` that's actually a PNG still
    // decodes). The stream is rewound internally.
    let reader = image::ImageReader::open(path)?.with_guessed_format()?;
    let img = reader.decode()?;
    let thumb = img.thumbnail(max_dim, max_dim);
    let mut out = Vec::with_capacity(16 * 1024);
    thumb.write_to(&mut Cursor::new(&mut out), image::ImageFormat::Png)?;
    Ok(out)
}

fn is_decodable_image(path: &Path) -> bool {
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_ascii_lowercase())
        .unwrap_or_default();
    matches!(
        ext.as_str(),
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp"
    )
}

/// Build the `kind: "image"` response from raw PNG bytes.
fn ok_image(png_bytes: Vec<u8>) -> ThumbnailDto {
    let encoded = BASE64_STANDARD.encode(&png_bytes);
    ThumbnailDto {
        kind: "image",
        data_url: Some(format!("data:image/png;base64,{encoded}")),
        icon_kind: None,
        extension: None,
    }
}

/// Build the `kind: "icon"` fallback — reuses the Phase 5
/// [`crate::icon::classify`] pipeline so the left-rail and right-
/// pane show the same Lucide glyph the rest of the UI uses.
fn icon_fallback(path: &Path) -> ThumbnailDto {
    let icon = crate::icon::classify(path);
    ThumbnailDto {
        kind: "icon",
        data_url: None,
        icon_kind: Some(icon.kind),
        extension: icon.extension,
    }
}

/// Best-effort age-based cache sweep. Deletes any `*.png` older
/// than `max_age_ms`. Called from Phase 23's periodic sweeper.
/// Never errors — a locked / missing cache dir is a no-op.
#[allow(dead_code)]
pub fn sweep_cache(max_age_ms: u64) {
    let Some(dir) = cache_dir_for_thumbs() else {
        return;
    };
    let Ok(entries) = fs::read_dir(&dir) else {
        return;
    };
    let now_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);
    for entry in entries.flatten() {
        let p = entry.path();
        if p.extension().and_then(|s| s.to_str()) != Some("png") {
            continue;
        }
        let Ok(meta) = entry.metadata() else { continue };
        let modified_ms = meta
            .modified()
            .ok()
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);
        if now_ms.saturating_sub(modified_ms) > max_age_ms {
            let _ = fs::remove_file(&p);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{Rgb, RgbImage};
    use tempfile::tempdir;

    fn write_tiny_png(path: &Path) {
        let mut img = RgbImage::new(8, 8);
        for (_, _, px) in img.enumerate_pixels_mut() {
            *px = Rgb([128, 64, 192]);
        }
        img.save(path).unwrap();
    }

    #[test]
    fn image_extension_gets_image_kind() {
        let tmp = tempdir().unwrap();
        let p = tmp.path().join("ok.png");
        write_tiny_png(&p);
        let dto = thumbnail_for(&p, 64);
        assert_eq!(dto.kind, "image");
        assert!(
            dto.data_url
                .as_ref()
                .is_some_and(|u| u.starts_with("data:image/png;base64,"))
        );
    }

    #[test]
    fn non_image_extension_falls_back_to_icon() {
        let tmp = tempdir().unwrap();
        let p = tmp.path().join("readme.pdf");
        fs::write(&p, b"%PDF-dummy").unwrap();
        let dto = thumbnail_for(&p, 64);
        assert_eq!(dto.kind, "icon");
        assert!(dto.data_url.is_none());
        assert!(dto.icon_kind.is_some());
    }

    #[test]
    fn missing_file_returns_icon_fallback() {
        let tmp = tempdir().unwrap();
        let p = tmp.path().join("nope.png");
        let dto = thumbnail_for(&p, 64);
        assert_eq!(dto.kind, "icon");
    }

    #[test]
    fn cache_key_differs_on_mtime_change() {
        let k1 = cache_key_for(Path::new("/a/b.png"), 100, 1000, 64);
        let k2 = cache_key_for(Path::new("/a/b.png"), 100, 2000, 64);
        assert_ne!(k1, k2);
        let k3 = cache_key_for(Path::new("/a/b.png"), 100, 1000, 128);
        assert_ne!(k1, k3);
    }

    #[test]
    fn directory_argument_returns_icon() {
        let tmp = tempdir().unwrap();
        let dto = thumbnail_for(tmp.path(), 64);
        assert_eq!(dto.kind, "icon");
    }
}
