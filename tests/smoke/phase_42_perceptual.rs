//! Phase 42 smoke — perceptual-hash dedup over real synthetic
//! images.
//!
//! Five cases:
//!
//! 1. The same picture saved as PNG + JPEG hashes to nearby values
//!    (similarity below the default warn threshold).
//! 2. Two visually different pictures hash far apart (similarity well
//!    above the default warn threshold).
//! 3. The hash function is stable — running it twice on the same
//!    file yields the same `u64`.
//! 4. Audio kind reports `AudioNotImplemented` per the deferred-
//!    feature contract.
//! 5. All Phase 42 Fluent keys (`perceptual-*`) appear in every one
//!    of the 18 locale files.

use std::fs;
use std::path::Path;

use copythat_perceptual::{
    PerceptualKind, SIMILARITY_DEFAULT_THRESHOLD, perceptual_hash, similarity,
};
use image::ImageBuffer;

const PHASE_42_KEYS: &[&str] = &[
    "perceptual-warn-title",
    "perceptual-warn-body",
    "perceptual-warn-keep-both",
    "perceptual-warn-skip",
    "perceptual-warn-overwrite",
    "perceptual-settings-heading",
    "perceptual-settings-hint",
    "perceptual-settings-threshold-label",
];

const LOCALES: &[&str] = &[
    "en", "es", "zh-CN", "hi", "ar", "pt-BR", "ru", "ja", "de", "fr", "ko", "it", "tr", "vi", "pl",
    "nl", "id", "uk",
];

/// Generate a deterministic 128×128 RGB picture parameterised by
/// `seed`. The hash pipeline downsamples to a small grid internally,
/// so this resolution is plenty to exercise the full encode → decode
/// → hash round-trip. Three structurally-distinct shapes pick on the
/// low byte of `seed` so two seeds at different points in the table
/// produce visually different pictures (a smooth-gradient seed scheme
/// alone fooled DoubleGradient — every gradient shares the same
/// neighbour-comparison structure).
fn synthesize_image(seed: u8) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    ImageBuffer::from_fn(128, 128, |x, y| match seed % 3 {
        0 => {
            // Horizontal stripes — sharp transitions every 16 px.
            let band = (y / 16) as u8;
            image::Rgb([(band * 32) ^ seed, 32, 192])
        }
        1 => {
            // Checkerboard — strongest possible signal for any
            // perceptual hash that compares neighbours.
            let on = ((x / 16) + (y / 16)) % 2 == 0;
            if on {
                image::Rgb([240, 240, 240])
            } else {
                image::Rgb([16, 16, 16])
            }
        }
        _ => {
            // Diagonal gradient — same shape but very different
            // intensity profile from the stripes / checkerboard.
            let v = ((x + y) as u8).wrapping_mul(seed.wrapping_add(1));
            image::Rgb([v, v.wrapping_add(40), v.wrapping_add(80)])
        }
    })
}

fn save_png(path: &Path, img: &ImageBuffer<image::Rgb<u8>, Vec<u8>>) {
    img.save_with_format(path, image::ImageFormat::Png)
        .expect("png save");
}

fn save_jpeg(path: &Path, img: &ImageBuffer<image::Rgb<u8>, Vec<u8>>) {
    img.save_with_format(path, image::ImageFormat::Jpeg)
        .expect("jpeg save");
}

#[test]
fn case01_same_picture_two_formats_hash_close() {
    let dir = tempfile::tempdir().unwrap();
    let img = synthesize_image(0x42);
    let png = dir.path().join("a.png");
    let jpg = dir.path().join("a.jpg");
    save_png(&png, &img);
    save_jpeg(&jpg, &img);

    let h_png = perceptual_hash(&png, PerceptualKind::Image).expect("png hash");
    let h_jpg = perceptual_hash(&jpg, PerceptualKind::Image).expect("jpg hash");
    let s = similarity(h_png, h_jpg);
    assert!(
        s <= 0.25,
        "PNG/JPEG of the same picture should be visually similar; \
         similarity = {s}"
    );
}

#[test]
fn case02_two_different_pictures_hash_far_apart() {
    let dir = tempfile::tempdir().unwrap();
    // Pick seeds that hit *different* shape branches (stripes vs
    // checkerboard) so the test exercises the dedup-warn boundary.
    let a = synthesize_image(0); // stripes
    let b = synthesize_image(1); // checkerboard
    let p_a = dir.path().join("a.png");
    let p_b = dir.path().join("b.png");
    save_png(&p_a, &a);
    save_png(&p_b, &b);

    let h_a = perceptual_hash(&p_a, PerceptualKind::Image).expect("a hash");
    let h_b = perceptual_hash(&p_b, PerceptualKind::Image).expect("b hash");
    let s = similarity(h_a, h_b);
    assert!(
        s > SIMILARITY_DEFAULT_THRESHOLD,
        "Different pictures should NOT trip the default warn \
         threshold; similarity = {s}, threshold = \
         {SIMILARITY_DEFAULT_THRESHOLD}"
    );
}

#[test]
fn case03_hash_is_stable_across_runs() {
    let dir = tempfile::tempdir().unwrap();
    let img = synthesize_image(0x77);
    let path = dir.path().join("stable.png");
    save_png(&path, &img);

    let h1 = perceptual_hash(&path, PerceptualKind::Image).expect("h1");
    let h2 = perceptual_hash(&path, PerceptualKind::Image).expect("h2");
    let h3 = perceptual_hash(&path, PerceptualKind::Image).expect("h3");
    assert_eq!(h1, h2);
    assert_eq!(h2, h3);
}

#[test]
fn case04_audio_kind_returns_not_implemented() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("dummy.bin");
    fs::write(&path, b"not actually audio").unwrap();
    let err = perceptual_hash(&path, PerceptualKind::Audio).unwrap_err();
    assert!(matches!(
        err,
        copythat_perceptual::PerceptualError::AudioNotImplemented
    ));
}

#[test]
fn case05_all_phase_42_keys_present_in_every_locale() {
    let workspace_root = workspace_root();
    let mut missing: Vec<String> = Vec::new();
    for locale in LOCALES {
        let path = workspace_root
            .join("locales")
            .join(locale)
            .join("copythat.ftl");
        let body = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("could not read {}", path.display()));
        for key in PHASE_42_KEYS {
            let needle = format!("{key} =");
            if !body.contains(&needle) {
                missing.push(format!("{locale}/{key}"));
            }
        }
    }
    assert!(missing.is_empty(), "missing Phase 42 keys: {missing:?}");
}

fn workspace_root() -> std::path::PathBuf {
    let crate_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    crate_dir
        .parent()
        .and_then(|p| p.parent())
        .expect("workspace has crates/<name>/Cargo.toml layout")
        .to_path_buf()
}
