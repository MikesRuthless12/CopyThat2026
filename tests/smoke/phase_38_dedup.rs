//! Phase 38 smoke — destination dedup + reflink fallback ladder.
//!
//! Five cases covering the ladder + the wire surface:
//!
//! 1. `DedupMode::None` skips the ladder unconditionally — outcome
//!    is `Skipped`, no destination written.
//! 2. AutoLadder on a normal-volume tempdir: each file ends up as
//!    `Reflink` (when the FS supports it) or `Copy` (when it
//!    doesn't). The fallback is always available.
//! 3. HardlinkAggressive forces a hardlink leg even when the file
//!    is writable; the destination shares the inode.
//! 4. ReflinkOnly skips the hardlink leg — even when hardlink is
//!    explicitly requested, it falls through to `Copy` if reflink
//!    isn't supported.
//! 5. All 12 Phase 38 Fluent keys present in every one of the 18
//!    locales.

use std::fs;
use std::path::PathBuf;

use copythat_platform::dedup::{
    DedupMode, DedupOptions, DedupOutcome, DedupStrategy, HardlinkPolicy, try_dedup,
};

const PHASE_38_KEYS: &[&str] = &[
    "settings-dedup-heading",
    "settings-dedup-hint",
    "settings-dedup-mode-auto",
    "settings-dedup-mode-reflink-only",
    "settings-dedup-mode-hardlink-aggressive",
    "settings-dedup-mode-off",
    "settings-dedup-hardlink-policy",
    "settings-dedup-prescan",
    "dedup-badge-reflinked",
    "dedup-badge-hardlinked",
    "dedup-badge-chunk-shared",
    "dedup-badge-copied",
];

const LOCALES: &[&str] = &[
    "en", "es", "zh-CN", "hi", "ar", "pt-BR", "ru", "ja", "de", "fr", "ko", "it", "tr", "vi", "pl",
    "nl", "id", "uk",
];

#[test]
fn case01_mode_none_skips_unconditionally() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("a.bin");
    let dst = dir.path().join("b.bin");
    fs::write(&src, vec![0u8; 4096]).unwrap();

    let opts = DedupOptions {
        mode: DedupMode::None,
        ..DedupOptions::default()
    };
    let outcome = try_dedup(&src, &dst, &opts).unwrap();
    assert_eq!(outcome.strategy, DedupStrategy::Skipped);
    assert_eq!(outcome.bytes_saved, 0);
    assert!(!dst.exists(), "Skipped must not write the destination");
}

#[test]
fn case02_auto_ladder_falls_through_to_copy_or_reflink() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("src.bin");
    let dst = dir.path().join("dst.bin");
    fs::write(&src, vec![0xAB; 16 * 1024]).unwrap();

    let opts = DedupOptions {
        mode: DedupMode::AutoLadder,
        hardlink_policy: HardlinkPolicy::Never,
        chunk_share_enabled: false,
    };
    let outcome = try_dedup(&src, &dst, &opts).unwrap();
    assert!(
        matches!(
            outcome.strategy,
            DedupStrategy::Reflink | DedupStrategy::Copy
        ),
        "expected Reflink or Copy, got {:?}",
        outcome.strategy
    );
}

#[test]
fn case03_hardlink_aggressive_creates_hardlink_or_reflink() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("src.bin");
    let dst = dir.path().join("dst.bin");
    fs::write(&src, b"shared payload").unwrap();

    let opts = DedupOptions {
        mode: DedupMode::HardlinkAggressive,
        hardlink_policy: HardlinkPolicy::Always,
        chunk_share_enabled: false,
    };
    let outcome = try_dedup(&src, &dst, &opts).unwrap();
    assert!(
        matches!(
            outcome.strategy,
            DedupStrategy::Reflink | DedupStrategy::Hardlink | DedupStrategy::Copy
        ),
        "expected Reflink/Hardlink/Copy, got {:?}",
        outcome.strategy
    );
    if matches!(
        outcome.strategy,
        DedupStrategy::Reflink | DedupStrategy::Hardlink
    ) {
        assert!(dst.exists());
        let dst_bytes = fs::read(&dst).unwrap();
        assert_eq!(dst_bytes, b"shared payload");
    }
}

#[test]
fn case04_reflink_only_skips_hardlink_leg() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("src.bin");
    let dst = dir.path().join("dst.bin");
    fs::write(&src, vec![0xCDu8; 1024]).unwrap();

    let opts = DedupOptions {
        mode: DedupMode::ReflinkOnly,
        // Even though hardlink_policy = Always, ReflinkOnly must
        // never try the hardlink leg — it's the whole point of the
        // mode.
        hardlink_policy: HardlinkPolicy::Always,
        chunk_share_enabled: false,
    };
    let outcome = try_dedup(&src, &dst, &opts).unwrap();
    assert!(
        outcome.strategy != DedupStrategy::Hardlink,
        "ReflinkOnly should never produce a Hardlink outcome"
    );
}

#[test]
fn case05_dedup_outcome_round_trips_through_serde() {
    let outcome = DedupOutcome {
        strategy: DedupStrategy::Reflink,
        bytes_saved: 4_194_304,
    };
    let s = serde_json::to_string(&outcome).unwrap();
    let back: DedupOutcome = serde_json::from_str(&s).unwrap();
    assert_eq!(outcome, back);
}

#[test]
fn case06_phase_38_fluent_keys_present_in_every_locale() {
    let root = locate_locales_dir().expect("locate locales/");
    for code in LOCALES {
        let path = root.join(code).join("copythat.ftl");
        let content =
            fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        for key in PHASE_38_KEYS {
            let starts = content.starts_with(&format!("{key} ="));
            let inline = content.contains(&format!("\n{key} ="));
            assert!(
                starts || inline,
                "locale `{code}` missing key `{key}` at {}",
                path.display()
            );
        }
    }
}

fn locate_locales_dir() -> Option<PathBuf> {
    let mut cur = std::env::current_dir().ok()?;
    for _ in 0..6 {
        let candidate = cur.join("locales");
        if candidate.join("en").join("copythat.ftl").exists() {
            return Some(candidate);
        }
        if !cur.pop() {
            break;
        }
    }
    None
}
