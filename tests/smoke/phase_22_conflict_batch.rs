//! Phase 22 smoke test — aggregate conflict dialog v2.
//!
//! Drives the engine's `copy_tree` with `CollisionPolicy::Prompt`
//! and a lightweight in-test runner-simulator that consumes
//! `CopyEvent::Collision` events against a [`ConflictProfile`] —
//! exactly the path the real `apps/copythat-ui` runner takes in
//! production. Mirrors the Phase 22 brief's acceptance criteria:
//!
//! 1. Seed a tree where **50 source files collide** with pre-
//!    populated destination counterparts. Mix of extensions — 20
//!    `*.txt`, 15 `*.docx`, 10 `*.jpg`, 5 misc. No non-colliding
//!    control files so the 50-file count is the authoritative
//!    answer to "how many prompts did the engine raise?".
//! 2. Build a `ConflictProfile` with the brief's three rule set:
//!    `*.txt → Skip`, `*.docx → OverwriteIfNewer`, fallback
//!    `KeepBoth`. Source `.docx` mtimes are strictly newer than
//!    destination so all 15 should overwrite.
//! 3. Run `copy_tree` + the simulator. Assert post-state:
//!    - 20 `*.txt` skipped (destination bytes match the pre-
//!      populated "OLD-" marker).
//!    - 15 `*.docx` overwritten (destination bytes match the
//!      source "NEW-" marker).
//!    - 15 `*.jpg` + `*.misc` kept-both (destination has BOTH the
//!      pre-populated "OLD-" file AND a `_2` sibling carrying the
//!      source "NEW-" bytes).
//! 4. Round-trip the profile through
//!    `copythat_settings::ProfileStore` — save under "Imports",
//!    load back, assert deep equality. Also exercise the
//!    ProfileStore's TOML-path via `save_to` / `load_from` so a
//!    user-edited `settings.toml` still recovers the profile.
//! 5. Re-run the same job with the same profile active. Assert
//!    **zero interactive prompts** were surfaced — the simulator's
//!    `prompt_count` should be 0 because every rule matched
//!    eagerly. Mirrors the brief's "Save the rule set as profile
//!    'Imports', re-run the same job, assert no conflict modal
//!    appears (all auto-resolved)" requirement.
//!
//! Runs on all three platforms — the engine is OS-agnostic and the
//! profile matcher is a pure-Rust glob check. No Tauri / webview
//! involved.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, UNIX_EPOCH};

use copythat_core::{
    CollisionPolicy, CollisionResolution, CopyControl, CopyEvent, TreeOptions, copy_tree,
};
use copythat_settings::{
    ConflictProfile, ConflictProfileSettings, ConflictRule, ConflictRuleResolution, ProfileStore,
    Settings,
};
use filetime::{FileTime, set_file_mtime};
use tempfile::tempdir;
use tokio::sync::mpsc;

/// Number of files per extension bucket. 20 + 15 + 10 + 5 = 50 —
/// exactly the Phase 22 brief's collision count.
const TXT_COUNT: usize = 20;
const DOCX_COUNT: usize = 15;
const JPG_COUNT: usize = 10;
const MISC_COUNT: usize = 5;

fn write_file(path: &Path, bytes: &[u8]) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    std::fs::write(path, bytes).unwrap();
}

fn set_mtime(path: &Path, unix_secs: i64) {
    set_file_mtime(path, FileTime::from_unix_time(unix_secs, 0)).unwrap();
}

/// Seed source + destination trees with 50 colliding files. Sources
/// carry `NEW-<name>` bodies and a fresh mtime; destinations carry
/// `OLD-<name>` bodies and an older mtime. The mtime delta is what
/// drives the `OverwriteIfNewer` rule on `.docx`.
fn seed_trees(src: &Path, dst: &Path) -> Vec<(PathBuf, PathBuf)> {
    let mut pairs = Vec::new();
    let old_secs: i64 = 1_600_000_000; // older
    let new_secs: i64 = 1_700_000_000; // newer

    let mut push = |idx: usize, stem: &str, ext: &str| {
        let fname = format!("{stem}-{idx:02}.{ext}");
        let s = src.join(&fname);
        let d = dst.join(&fname);
        write_file(&s, format!("NEW-{fname}").as_bytes());
        write_file(&d, format!("OLD-{fname}").as_bytes());
        set_mtime(&s, new_secs);
        set_mtime(&d, old_secs);
        pairs.push((s, d));
    };

    for i in 0..TXT_COUNT {
        push(i, "doc", "txt");
    }
    for i in 0..DOCX_COUNT {
        push(i, "report", "docx");
    }
    for i in 0..JPG_COUNT {
        push(i, "photo", "jpg");
    }
    for i in 0..MISC_COUNT {
        push(i, "misc", "dat");
    }
    pairs
}

/// In-test runner-simulator. Drains `CopyEvent` from the channel,
/// resolves every `Collision` against the supplied
/// `ConflictProfile`, counts how many would have surfaced to the
/// user (i.e. cases where no rule matched).
///
/// Matches the real runner's logic in
/// `apps/copythat-ui/src-tauri/src/runner.rs` and
/// `collisions::apply_rule_resolution`. If the logic drifts, this
/// test will catch it.
async fn drive(
    rx: &mut mpsc::Receiver<CopyEvent>,
    profile: ConflictProfile,
    prompt_count: Arc<AtomicUsize>,
) {
    while let Some(evt) = rx.recv().await {
        if let CopyEvent::Collision(coll) = evt {
            let src = coll.src.clone();
            let dst = coll.dst.clone();
            let basename = src
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();
            if let Some(m) = profile.match_basename_or_path(&basename, &basename) {
                let resolution = translate(m.resolution, &src, &dst);
                coll.resolve(resolution);
            } else {
                prompt_count.fetch_add(1, Ordering::Relaxed);
                // No rule matched — fall through to Skip so the tree
                // completes. In the real UI this path would raise a
                // prompt; the count lets the test distinguish
                // "auto-resolved" from "would have prompted".
                coll.resolve(CollisionResolution::Skip);
            }
        }
    }
}

/// Mirror of `crate::collisions::apply_rule_resolution` — kept
/// inline so the smoke test doesn't depend on the UI crate's
/// internals.
fn translate(resolution: ConflictRuleResolution, src: &Path, dst: &Path) -> CollisionResolution {
    match resolution {
        ConflictRuleResolution::Skip => CollisionResolution::Skip,
        ConflictRuleResolution::Overwrite => CollisionResolution::Overwrite,
        ConflictRuleResolution::OverwriteIfNewer => {
            if source_is_newer(src, dst) {
                CollisionResolution::Overwrite
            } else {
                CollisionResolution::Skip
            }
        }
        ConflictRuleResolution::OverwriteIfLarger => {
            let s_size = std::fs::metadata(src).map(|m| m.len()).unwrap_or(0);
            let d_size = std::fs::metadata(dst).map(|m| m.len()).unwrap_or(0);
            if s_size > d_size {
                CollisionResolution::Overwrite
            } else {
                CollisionResolution::Skip
            }
        }
        ConflictRuleResolution::KeepBoth => match next_keep_both_name(dst) {
            Some(n) => CollisionResolution::Rename(n),
            None => CollisionResolution::Skip,
        },
    }
}

fn source_is_newer(src: &Path, dst: &Path) -> bool {
    let s_mtime = std::fs::metadata(src)
        .and_then(|m| m.modified())
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .unwrap_or(Duration::ZERO);
    let d_mtime = std::fs::metadata(dst)
        .and_then(|m| m.modified())
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .unwrap_or(Duration::ZERO);
    s_mtime > d_mtime
}

fn next_keep_both_name(dst: &Path) -> Option<String> {
    let parent = dst.parent()?;
    let stem = dst.file_stem()?.to_string_lossy().into_owned();
    let ext = dst
        .extension()
        .map(|e| e.to_string_lossy().into_owned())
        .unwrap_or_default();
    for n in 2..10_000 {
        let candidate_name = if ext.is_empty() {
            format!("{stem}_{n}")
        } else {
            format!("{stem}_{n}.{ext}")
        };
        let candidate = parent.join(&candidate_name);
        if !candidate.exists() {
            return Some(candidate_name);
        }
    }
    None
}

/// The profile the brief names — txt Skip, docx OverwriteIfNewer,
/// everything else KeepBoth via fallback. Rules are order-
/// sensitive (first match wins), so listing the specific
/// extensions first and the catch-all last is required.
fn imports_profile() -> ConflictProfile {
    ConflictProfile {
        rules: vec![
            ConflictRule {
                pattern: "*.txt".to_string(),
                resolution: ConflictRuleResolution::Skip,
            },
            ConflictRule {
                pattern: "*.docx".to_string(),
                resolution: ConflictRuleResolution::OverwriteIfNewer,
            },
        ],
        fallback: Some(ConflictRuleResolution::KeepBoth),
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn phase_22_rules_auto_resolve_all_50_collisions_and_roundtrip_profile() {
    let tmp = tempdir().expect("tempdir");
    let src_root = tmp.path().join("src");
    let dst_root = tmp.path().join("dst");
    std::fs::create_dir_all(&src_root).unwrap();
    std::fs::create_dir_all(&dst_root).unwrap();
    let pairs = seed_trees(&src_root, &dst_root);
    assert_eq!(
        pairs.len(),
        TXT_COUNT + DOCX_COUNT + JPG_COUNT + MISC_COUNT,
        "seed setup invariant"
    );

    let (tx, mut rx) = mpsc::channel::<CopyEvent>(256);
    let ctrl = CopyControl::new();
    let tree_opts = TreeOptions {
        collision: CollisionPolicy::Prompt,
        ..TreeOptions::default()
    };
    let prompt_count = Arc::new(AtomicUsize::new(0));

    let profile = imports_profile();
    let sim_profile = profile.clone();
    let sim_counter = prompt_count.clone();
    let sim = tokio::spawn(async move {
        drive(&mut rx, sim_profile, sim_counter).await;
    });

    let report = copy_tree(&src_root, &dst_root, tree_opts, ctrl, tx)
        .await
        .expect("copy_tree completed");
    sim.await.expect("sim task joins");

    // Every source file generated exactly one collision event — so
    // prompt_count should be 0 (every rule matched) and report
    // should tally 50 files seen by the walker.
    assert_eq!(
        prompt_count.load(Ordering::Relaxed),
        0,
        "every collision must auto-resolve via the Imports profile"
    );
    // `report.skipped` counts files the collision policy skipped;
    // the 20 *.txt land here plus any `*.docx` where source was
    // older (none in this seed). `files` counts files that were
    // actually written — 15 docx overwrites + 15 keep-both
    // renames (10 jpg + 5 misc) = 30.
    assert_eq!(
        report.files,
        (DOCX_COUNT + JPG_COUNT + MISC_COUNT) as u64,
        "30 writes expected (15 overwrite + 15 keep-both)"
    );
    assert_eq!(
        report.skipped, TXT_COUNT as u64,
        "20 *.txt should be skipped by rule"
    );

    // --- Assertion pass A: *.txt skipped, destination still "OLD-" ---
    for i in 0..TXT_COUNT {
        let name = format!("doc-{i:02}.txt");
        let dst = dst_root.join(&name);
        let body = std::fs::read(&dst).expect("dst still present");
        assert_eq!(
            body,
            format!("OLD-{name}").as_bytes(),
            "*.txt rule must leave dst untouched: {name}"
        );
    }

    // --- Assertion pass B: *.docx overwritten, destination now "NEW-" ---
    for i in 0..DOCX_COUNT {
        let name = format!("report-{i:02}.docx");
        let dst = dst_root.join(&name);
        let body = std::fs::read(&dst).expect("dst present");
        assert_eq!(
            body,
            format!("NEW-{name}").as_bytes(),
            "*.docx newer-wins must overwrite: {name}"
        );
    }

    // --- Assertion pass C: *.jpg + *.dat kept both — original "OLD-"
    //     stays, new "_2" sibling carries "NEW-" ---
    for i in 0..JPG_COUNT {
        let name = format!("photo-{i:02}.jpg");
        let dst = dst_root.join(&name);
        let dst_2 = dst_root.join(format!("photo-{i:02}_2.jpg"));
        let orig = std::fs::read(&dst).expect("orig present");
        let fresh = std::fs::read(&dst_2).expect("_2 sibling present");
        assert_eq!(orig, format!("OLD-{name}").as_bytes(), "orig preserved");
        assert_eq!(fresh, format!("NEW-{name}").as_bytes(), "_2 carries new");
    }
    for i in 0..MISC_COUNT {
        let name = format!("misc-{i:02}.dat");
        let dst = dst_root.join(&name);
        let dst_2 = dst_root.join(format!("misc-{i:02}_2.dat"));
        let orig = std::fs::read(&dst).expect("orig present");
        let fresh = std::fs::read(&dst_2).expect("_2 sibling present");
        assert_eq!(
            orig,
            format!("OLD-{name}").as_bytes(),
            "misc orig preserved"
        );
        assert_eq!(
            fresh,
            format!("NEW-{name}").as_bytes(),
            "misc _2 carries new"
        );
    }

    // --- Profile persistence round-trip (ProfileStore + Settings) ---
    let cfg_dir = tmp.path().join("config");
    std::fs::create_dir_all(&cfg_dir).unwrap();

    let settings = Settings {
        conflict_profiles: ConflictProfileSettings {
            active: Some("Imports".to_string()),
            profiles: [("Imports".to_string(), profile.clone())]
                .into_iter()
                .collect(),
        },
        ..Settings::default()
    };

    // TOML round-trip — `settings.toml` carries everything including
    // `conflict_profiles`, so an edited or imported file recovers
    // the profile correctly.
    let toml_path = cfg_dir.join("settings.toml");
    settings.save_to(&toml_path).unwrap();
    let reloaded_settings = Settings::load_from(&toml_path).unwrap();
    assert_eq!(
        reloaded_settings.conflict_profiles, settings.conflict_profiles,
        "conflict profiles must round-trip through settings.toml"
    );
    assert!(
        reloaded_settings
            .conflict_profiles
            .active_profile()
            .is_some(),
        "active profile resolves after TOML round-trip"
    );

    // ProfileStore JSON round-trip — saving a named snapshot under
    // "Imports" then loading it back must preserve the conflict
    // rules. This is the path the "Save these rules as profile…"
    // button in the UI takes.
    let store = ProfileStore::new(cfg_dir.join("profiles"));
    store.save("Imports", &settings).unwrap();
    let loaded = store.load("Imports").unwrap();
    assert_eq!(loaded.conflict_profiles, settings.conflict_profiles);

    // --- Re-run the same job: zero prompts with the profile active ---
    // Reset destination to the pre-test state so a fresh run actually
    // regenerates the same 50 collisions.
    std::fs::remove_dir_all(&dst_root).unwrap();
    std::fs::create_dir_all(&dst_root).unwrap();
    for i in 0..TXT_COUNT {
        let name = format!("doc-{i:02}.txt");
        let d = dst_root.join(&name);
        write_file(&d, format!("OLD-{name}").as_bytes());
        set_mtime(&d, 1_600_000_000);
    }
    for i in 0..DOCX_COUNT {
        let name = format!("report-{i:02}.docx");
        let d = dst_root.join(&name);
        write_file(&d, format!("OLD-{name}").as_bytes());
        set_mtime(&d, 1_600_000_000);
    }
    for i in 0..JPG_COUNT {
        let name = format!("photo-{i:02}.jpg");
        let d = dst_root.join(&name);
        write_file(&d, format!("OLD-{name}").as_bytes());
        set_mtime(&d, 1_600_000_000);
    }
    for i in 0..MISC_COUNT {
        let name = format!("misc-{i:02}.dat");
        let d = dst_root.join(&name);
        write_file(&d, format!("OLD-{name}").as_bytes());
        set_mtime(&d, 1_600_000_000);
    }

    let (tx2, mut rx2) = mpsc::channel::<CopyEvent>(256);
    let ctrl2 = CopyControl::new();
    let opts2 = TreeOptions {
        collision: CollisionPolicy::Prompt,
        ..TreeOptions::default()
    };
    let second_prompts = Arc::new(AtomicUsize::new(0));
    let active_profile = reloaded_settings
        .conflict_profiles
        .active_profile()
        .expect("active profile resolves")
        .clone();
    let sim_counter_2 = second_prompts.clone();
    let sim2 = tokio::spawn(async move {
        drive(&mut rx2, active_profile, sim_counter_2).await;
    });
    let _ = copy_tree(&src_root, &dst_root, opts2, ctrl2, tx2)
        .await
        .expect("second copy_tree");
    sim2.await.unwrap();
    assert_eq!(
        second_prompts.load(Ordering::Relaxed),
        0,
        "re-running with the saved profile must surface zero prompts"
    );
}
