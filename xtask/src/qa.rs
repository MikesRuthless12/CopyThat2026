//! `xtask qa-automate` — drive every checkbox in
//! `QualityAssuranceChecklist.md` that the existing toolchain can
//! execute without a human in the loop, and emit one pass/fail report.
//!
//! Sections covered (matches the checklist's appendix):
//! - §0 pre-flight: working tree clean, `Cargo.lock` present,
//!   `xtask i18n-lint`.
//! - §1 static analysis: `cargo fmt --all -- --check`,
//!   `cargo clippy --workspace --all-targets -- -D warnings`,
//!   `cargo deny check`. The workspace lint
//!   `unsafe_code = "warn"` plus clippy's `-D warnings` already covers
//!   the "no unsafe outside copythat-platform" rule, so no separate
//!   gate.
//! - §1 frontend: `pnpm exec svelte-check` and
//!   `pnpm exec tsc --noEmit` in `apps/copythat-ui`.
//! - §2 per-crate tests: `cargo test -p <crate>` for every workspace
//!   crate. We deliberately skip the all-workspace
//!   `cargo test --workspace` — the user's iteration profile prefers
//!   per-crate runs to keep total wall-clock down. The smoke matrix
//!   (`tests/smoke/phase_*.rs`) is registered against each crate's
//!   `[[test]]` block, so per-crate runs still hit every smoke.
//! - §3 security: `cargo audit` with the same advisory-ignore list
//!   that lives in `deny.toml` (parsed at runtime so the two never
//!   drift), `cargo vet --locked` (non-blocking, mirrors `ci.yml`).
//! - §5 perf: `xtask bench-ci` (Criterion suite, CI-scaled).
//!
//! Out of scope:
//! - §5 `bench-vs`: needs competitor binaries on PATH + a synthetic
//!   workload. Run `xtask bench-vs` directly when the host has them.
//! - §9 packaging: lives in `release.yml` on tag (free-tier signing).
//! - §4 / §6 / §7 manual UI / cross-platform / edge cases: physical
//!   state. The Playwright + tauri-driver harness slots in here once
//!   it lands; nothing in the checklist's "Already fully automated"
//!   set requires it.
//!
//! Output:
//! - Live progress on stderr as each step starts/finishes (subprocess
//!   stdout/stderr inherit through so the user sees clippy /
//!   cargo-test output as it streams).
//! - Final pass/fail summary on stdout.
//! - Markdown report at `target/qa-report.md` (override with
//!   `--report <path>`) for CI artifact upload.
//! - Exit code: 0 on all-pass, 1 on any fail. `cargo vet` is reported
//!   non-blocking so a vet drift won't red-X qa-automate while the
//!   audit backlog catches up.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use crate::repo_root;

/// Workspace member crates as declared in the root `Cargo.toml`. Kept
/// in this fixed order so the qa-automate run always covers crates
/// in the same sequence (helpful when comparing two runs side by
/// side). The `workspace_crates_match_cargo_toml` test trips on
/// drift so a forgotten new crate fails the build.
///
/// `copythat-ui` (the Tauri shell crate at
/// `apps/copythat-ui/src-tauri`) is in the list because it
/// exclusively registers several Phase 17e / 28 / 29 smoke tests
/// the QA checklist explicitly calls out. Its first build pulls in
/// the Tauri toolchain (libwebkit2gtk on Linux, codesign hooks on
/// macOS) so the slot lands at the bottom of the list. Reach for
/// `--skip-tests` if a fast inner loop matters.
const WORKSPACE_CRATES: &[&str] = &[
    "copythat-core",
    "copythat-hash",
    "copythat-secure-delete",
    "copythat-history",
    "copythat-platform",
    "copythat-shellext",
    "copythat-i18n",
    "copythat-settings",
    "copythat-snapshot",
    "copythat-journal",
    "copythat-shape",
    "copythat-sync",
    "copythat-watch",
    "copythat-chunk",
    "copythat-power",
    "copythat-cloud",
    "copythat-mount",
    "copythat-audit",
    "copythat-crypt",
    "copythat-cli",
    "copythat-helper",
    "copythat-mobile",
    "xtask",
    "copythat-ui",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Section {
    PreFlight,
    Static,
    Frontend,
    Security,
    Tests,
    Performance,
}

impl Section {
    fn label(&self) -> &'static str {
        match self {
            Section::PreFlight => "§0 pre-flight",
            Section::Static => "§1 static",
            Section::Frontend => "§1 frontend",
            Section::Security => "§3 security",
            Section::Tests => "§2 tests",
            Section::Performance => "§5 perf",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Outcome {
    Pass,
    Fail,
    Skipped,
}

impl Outcome {
    fn glyph(&self) -> &'static str {
        match self {
            Outcome::Pass => "PASS",
            Outcome::Fail => "FAIL",
            Outcome::Skipped => "SKIP",
        }
    }
}

struct Step {
    section: Section,
    name: String,
    outcome: Outcome,
    duration: Duration,
    note: String,
}

#[derive(Debug, Default)]
struct Args {
    skip_frontend: bool,
    skip_audit: bool,
    skip_vet: bool,
    skip_deny: bool,
    skip_tests: bool,
    skip_bench: bool,
    skip_clippy: bool,
    skip_git_check: bool,
    fail_fast: bool,
    report: Option<PathBuf>,
}

pub(crate) fn run(argv: Vec<String>) -> Result<(), String> {
    let args = parse_args(argv)?;
    let root = repo_root().ok_or("could not locate repo root (Cargo.toml + locales/)")?;
    let started = Instant::now();
    let mut steps: Vec<Step> = Vec::new();

    eprintln!("xtask qa-automate: starting in {}", root.display());

    // §0 pre-flight ----------------------------------------------------
    if args.skip_git_check {
        steps.push(skipped(
            Section::PreFlight,
            "working tree clean",
            "--skip-git-check",
        ));
    } else {
        steps.push(check_git_clean(&root));
    }
    log_last(&steps);
    if args.fail_fast && any_failed(&steps) {
        return finalize(&root, &steps, started, &args);
    }

    steps.push(check_cargo_lock(&root));
    log_last(&steps);

    steps.push(run_i18n_lint());
    log_last(&steps);
    if args.fail_fast && any_failed(&steps) {
        return finalize(&root, &steps, started, &args);
    }

    // §1 static analysis (Rust side) -----------------------------------
    steps.push(run_cmd(
        Section::Static,
        "cargo fmt --check",
        &root,
        "cargo",
        &["fmt", "--all", "--", "--check"],
    ));
    log_last(&steps);
    if args.fail_fast && any_failed(&steps) {
        return finalize(&root, &steps, started, &args);
    }

    if args.skip_clippy {
        steps.push(skipped(Section::Static, "cargo clippy", "--skip-clippy"));
    } else {
        steps.push(run_cmd(
            Section::Static,
            "cargo clippy -D warnings",
            &root,
            "cargo",
            &[
                "clippy",
                "--workspace",
                "--all-targets",
                "--",
                "-D",
                "warnings",
            ],
        ));
        log_last(&steps);
        if args.fail_fast && any_failed(&steps) {
            return finalize(&root, &steps, started, &args);
        }
    }

    if args.skip_deny {
        steps.push(skipped(Section::Static, "cargo deny check", "--skip-deny"));
    } else {
        steps.push(maybe_run(
            Section::Static,
            "cargo deny check",
            &root,
            "cargo-deny",
            "cargo",
            &["deny", "check"],
        ));
        log_last(&steps);
    }

    // §1 frontend ------------------------------------------------------
    let pnpm_available = which("pnpm");
    if args.skip_frontend || !pnpm_available {
        let why = if args.skip_frontend {
            "--skip-frontend"
        } else {
            "pnpm not on PATH"
        };
        steps.push(skipped(Section::Frontend, "pnpm svelte-check", why));
        steps.push(skipped(Section::Frontend, "pnpm tsc --noEmit", why));
    } else {
        let ui = root.join("apps").join("copythat-ui");
        steps.push(run_cmd(
            Section::Frontend,
            "pnpm svelte-check",
            &ui,
            "pnpm",
            &["exec", "svelte-check", "--tsconfig", "./tsconfig.json"],
        ));
        log_last(&steps);
        steps.push(run_cmd(
            Section::Frontend,
            "pnpm tsc --noEmit",
            &ui,
            "pnpm",
            &["exec", "tsc", "--noEmit"],
        ));
        log_last(&steps);
    }

    // §3 security ------------------------------------------------------
    if args.skip_audit {
        steps.push(skipped(Section::Security, "cargo audit", "--skip-audit"));
    } else {
        steps.push(run_cargo_audit(&root));
        log_last(&steps);
    }

    if args.skip_vet {
        steps.push(skipped(Section::Security, "cargo vet", "--skip-vet"));
    } else {
        // Mirror ci.yml: cargo-vet is non-blocking. We run it for
        // visibility but never fail qa-automate on a vet drift while
        // the audit backlog catches up. Tagged releases enforce vet
        // through release.yml.
        let mut s = maybe_run(
            Section::Security,
            "cargo vet (non-blocking)",
            &root,
            "cargo-vet",
            "cargo",
            &["vet", "--locked"],
        );
        if s.outcome == Outcome::Fail {
            s.outcome = Outcome::Pass;
            s.note = format!("non-blocking — reported as pass; underlying: {}", s.note);
        }
        steps.push(s);
        log_last(&steps);
    }

    // §2 per-crate tests ----------------------------------------------
    if args.skip_tests {
        steps.push(skipped(
            Section::Tests,
            "cargo test (per-crate)",
            "--skip-tests",
        ));
    } else {
        for crate_name in WORKSPACE_CRATES {
            steps.push(run_cmd(
                Section::Tests,
                &format!("cargo test -p {crate_name}"),
                &root,
                "cargo",
                &["test", "-p", crate_name],
            ));
            log_last(&steps);
            if args.fail_fast && any_failed(&steps) {
                return finalize(&root, &steps, started, &args);
            }
        }
    }

    // §5 performance --------------------------------------------------
    if args.skip_bench {
        steps.push(skipped(Section::Performance, "xtask bench-ci", "--skip-bench"));
    } else {
        let t0 = Instant::now();
        let (outcome, note) = match crate::bench::run(true) {
            Ok(()) => (Outcome::Pass, "Criterion suite passed".to_string()),
            Err(e) => (Outcome::Fail, e),
        };
        steps.push(Step {
            section: Section::Performance,
            name: "xtask bench-ci".into(),
            outcome,
            duration: t0.elapsed(),
            note,
        });
        log_last(&steps);
    }

    finalize(&root, &steps, started, &args)
}

// ---------------------------------------------------------------------
// Argument parsing
// ---------------------------------------------------------------------

fn parse_args(argv: Vec<String>) -> Result<Args, String> {
    let mut a = Args::default();
    let mut iter = argv.into_iter();
    while let Some(s) = iter.next() {
        match s.as_str() {
            "--skip-frontend" => a.skip_frontend = true,
            "--skip-audit" => a.skip_audit = true,
            "--skip-vet" => a.skip_vet = true,
            "--skip-deny" => a.skip_deny = true,
            "--skip-tests" => a.skip_tests = true,
            "--skip-bench" => a.skip_bench = true,
            "--skip-clippy" => a.skip_clippy = true,
            "--skip-git-check" => a.skip_git_check = true,
            "--fail-fast" => a.fail_fast = true,
            "--report" => {
                let p = iter
                    .next()
                    .ok_or_else(|| "--report requires a path".to_string())?;
                a.report = Some(PathBuf::from(p));
            }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            other => return Err(format!("unknown flag: `{other}` (try --help)")),
        }
    }
    Ok(a)
}

fn print_help() {
    println!(
        "Usage: xtask qa-automate [options]\n\
         \n\
         Drives every automatable checkbox in QualityAssuranceChecklist.md\n\
         (§0 pre-flight, §1 static + frontend, §2 per-crate tests,\n\
         §3 security, §5 bench-ci) and emits a pass/fail report.\n\
         \n\
         Options:\n  \
         --skip-frontend   Don't run pnpm svelte-check / tsc\n  \
         --skip-audit      Don't run cargo-audit\n  \
         --skip-vet        Don't run cargo-vet\n  \
         --skip-deny       Don't run cargo-deny\n  \
         --skip-tests      Don't run per-crate cargo test (§2)\n  \
         --skip-bench      Don't run xtask bench-ci (§5)\n  \
         --skip-clippy     Don't run cargo clippy (faster local iter)\n  \
         --skip-git-check  Don't fail on a dirty working tree\n  \
         --fail-fast       Stop at the first failed step\n  \
         --report <path>   Override report path (default: target/qa-report.md)\n  \
         -h, --help        Show this help and exit\n"
    );
}

// ---------------------------------------------------------------------
// Step runners
// ---------------------------------------------------------------------

fn check_git_clean(root: &Path) -> Step {
    let t0 = Instant::now();
    let result = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .current_dir(root)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();
    match result {
        Ok(out) if out.status.success() => {
            let s = String::from_utf8_lossy(&out.stdout);
            if s.trim().is_empty() {
                Step {
                    section: Section::PreFlight,
                    name: "working tree clean".into(),
                    outcome: Outcome::Pass,
                    duration: t0.elapsed(),
                    note: "git status --porcelain empty".into(),
                }
            } else {
                let n = s.lines().count();
                Step {
                    section: Section::PreFlight,
                    name: "working tree clean".into(),
                    outcome: Outcome::Fail,
                    duration: t0.elapsed(),
                    note: format!(
                        "{n} pending file(s) per `git status --porcelain` \
                         (use --skip-git-check to override)"
                    ),
                }
            }
        }
        Ok(out) => Step {
            section: Section::PreFlight,
            name: "working tree clean".into(),
            outcome: Outcome::Fail,
            duration: t0.elapsed(),
            note: format!("git status exited with {}", out.status),
        },
        Err(e) => Step {
            section: Section::PreFlight,
            name: "working tree clean".into(),
            outcome: Outcome::Skipped,
            duration: t0.elapsed(),
            note: format!("git not on PATH ({e})"),
        },
    }
}

fn check_cargo_lock(root: &Path) -> Step {
    let t0 = Instant::now();
    let path = root.join("Cargo.lock");
    if path.is_file() {
        Step {
            section: Section::PreFlight,
            name: "Cargo.lock present".into(),
            outcome: Outcome::Pass,
            duration: t0.elapsed(),
            note: "Cargo.lock at workspace root".into(),
        }
    } else {
        Step {
            section: Section::PreFlight,
            name: "Cargo.lock present".into(),
            outcome: Outcome::Fail,
            duration: t0.elapsed(),
            note: "Cargo.lock missing — run `cargo generate-lockfile`".into(),
        }
    }
}

fn run_i18n_lint() -> Step {
    let t0 = Instant::now();
    match crate::i18n_lint() {
        Ok(()) => Step {
            section: Section::PreFlight,
            name: "xtask i18n-lint".into(),
            outcome: Outcome::Pass,
            duration: t0.elapsed(),
            note: "all locales parse + key parity OK".into(),
        },
        Err(e) => Step {
            section: Section::PreFlight,
            name: "xtask i18n-lint".into(),
            outcome: Outcome::Fail,
            duration: t0.elapsed(),
            note: e,
        },
    }
}

fn run_cargo_audit(root: &Path) -> Step {
    let t0 = Instant::now();
    let ignores = match read_advisory_ignores(root) {
        Ok(ids) => ids,
        Err(e) => {
            return Step {
                section: Section::Security,
                name: "cargo audit".into(),
                outcome: Outcome::Fail,
                duration: t0.elapsed(),
                note: format!("read deny.toml advisories: {e}"),
            };
        }
    };
    if !is_cargo_subcommand_installed("audit") {
        return Step {
            section: Section::Security,
            name: "cargo audit".into(),
            outcome: Outcome::Skipped,
            duration: t0.elapsed(),
            note: "cargo-audit not installed (cargo install --locked cargo-audit)"
                .into(),
        };
    }

    // Build argv: `cargo audit --ignore <id> --ignore <id> ...`
    let mut argv: Vec<String> = vec!["audit".into()];
    for id in &ignores {
        argv.push("--ignore".into());
        argv.push(id.clone());
    }
    let argv_ref: Vec<&str> = argv.iter().map(|s| s.as_str()).collect();

    let mut step = run_cmd(
        Section::Security,
        &format!("cargo audit ({} ignores)", ignores.len()),
        root,
        "cargo",
        &argv_ref,
    );
    if step.outcome == Outcome::Pass {
        step.note = format!("clean against {} ignored advisories", ignores.len());
    }
    step.duration = t0.elapsed();
    step
}

/// Generic subprocess runner. Inherits stdout/stderr so the user sees
/// the underlying tool's output streamed live, then captures the
/// exit status into a `Step`.
fn run_cmd(section: Section, name: &str, dir: &Path, exe: &str, args: &[&str]) -> Step {
    let t0 = Instant::now();
    let status = Command::new(exe)
        .args(args)
        .current_dir(dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();
    let duration = t0.elapsed();
    match status {
        Ok(s) if s.success() => Step {
            section,
            name: name.into(),
            outcome: Outcome::Pass,
            duration,
            note: format!("{exe} ok"),
        },
        Ok(s) => Step {
            section,
            name: name.into(),
            outcome: Outcome::Fail,
            duration,
            note: format!("{exe} exited with {s}"),
        },
        Err(e) => Step {
            section,
            name: name.into(),
            outcome: Outcome::Fail,
            duration,
            note: format!("spawn {exe}: {e}"),
        },
    }
}

/// Run a cargo subcommand only if it's actually installed. Returns a
/// `Skipped` step (not `Fail`) when the probe fails — qa-automate
/// should be runnable on a fresh clone before the developer has
/// `cargo install`ed every auxiliary tool. Subcommand name is the
/// `cargo install` form (e.g. `cargo-audit`) which we strip the
/// `cargo-` prefix off when probing via `cargo <name> --version`.
fn maybe_run(
    section: Section,
    name: &str,
    dir: &Path,
    probe: &str,
    exe: &str,
    args: &[&str],
) -> Step {
    let probe_name = probe.strip_prefix("cargo-").unwrap_or(probe);
    if !is_cargo_subcommand_installed(probe_name) {
        return Step {
            section,
            name: name.into(),
            outcome: Outcome::Skipped,
            duration: Duration::default(),
            note: format!("{probe} not installed (cargo install --locked {probe})"),
        };
    }
    run_cmd(section, name, dir, exe, args)
}

fn skipped(section: Section, name: &str, why: &str) -> Step {
    Step {
        section,
        name: name.into(),
        outcome: Outcome::Skipped,
        duration: Duration::default(),
        note: why.into(),
    }
}

fn is_cargo_subcommand_installed(subcommand: &str) -> bool {
    Command::new("cargo")
        .arg(subcommand)
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn which(exe: &str) -> bool {
    let lookup = if cfg!(windows) { "where" } else { "which" };
    Command::new(lookup)
        .arg(exe)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn any_failed(steps: &[Step]) -> bool {
    steps.iter().any(|s| s.outcome == Outcome::Fail)
}

fn log_last(steps: &[Step]) {
    if let Some(s) = steps.last() {
        eprintln!(
            "  [{}]  {:>7}  {:6.1}s  {}  — {}",
            s.outcome.glyph(),
            short_section(s.section),
            s.duration.as_secs_f64(),
            s.name,
            s.note
        );
    }
}

fn short_section(s: Section) -> &'static str {
    match s {
        Section::PreFlight => "preflt",
        Section::Static => "static",
        Section::Frontend => "frontnd",
        Section::Security => "sec",
        Section::Tests => "tests",
        Section::Performance => "perf",
    }
}

// ---------------------------------------------------------------------
// deny.toml parsing
// ---------------------------------------------------------------------

/// Parse the `[advisories] ignore = [...]` block out of `deny.toml`.
/// Reading at runtime keeps `cargo audit --ignore` in lockstep with
/// the deny config — same tripwire the Phase 17b smoke test enforces
/// at filesystem level. We don't pull a TOML parser dependency: each
/// id sits on its own line as `"RUSTSEC-YYYY-NNNN",`, which a string
/// scan handles unambiguously.
fn read_advisory_ignores(root: &Path) -> Result<Vec<String>, String> {
    let body = fs::read_to_string(root.join("deny.toml"))
        .map_err(|e| format!("read deny.toml: {e}"))?;
    let mut ids: Vec<String> = Vec::new();
    for line in body.lines() {
        let line = line.trim();
        if !line.starts_with('"') {
            continue;
        }
        let rest = &line[1..];
        if !rest.starts_with("RUSTSEC-") {
            continue;
        }
        let Some(close) = rest.find('"') else {
            continue;
        };
        ids.push(rest[..close].to_string());
    }
    if ids.is_empty() {
        return Err("deny.toml has no RUSTSEC-* ignores".into());
    }
    Ok(ids)
}

// ---------------------------------------------------------------------
// Reporting
// ---------------------------------------------------------------------

fn finalize(
    root: &Path,
    steps: &[Step],
    started: Instant,
    args: &Args,
) -> Result<(), String> {
    let total = started.elapsed();
    let pass = steps.iter().filter(|s| s.outcome == Outcome::Pass).count();
    let fail = steps.iter().filter(|s| s.outcome == Outcome::Fail).count();
    let skip = steps.iter().filter(|s| s.outcome == Outcome::Skipped).count();

    println!();
    println!("===== xtask qa-automate summary =====");
    for s in steps {
        println!(
            "  [{}]  {:>7}  {:6.1}s  {:<40}  — {}",
            s.outcome.glyph(),
            short_section(s.section),
            s.duration.as_secs_f64(),
            s.name,
            s.note
        );
    }
    println!("=====================================");
    println!(
        "total: {pass} pass / {fail} fail / {skip} skip in {:.1}s",
        total.as_secs_f64()
    );

    let report_path = args
        .report
        .clone()
        .unwrap_or_else(|| root.join("target").join("qa-report.md"));
    if let Some(parent) = report_path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let md = render_markdown(steps, total);
    match fs::write(&report_path, md) {
        Ok(()) => eprintln!("xtask qa-automate: report → {}", report_path.display()),
        Err(e) => eprintln!(
            "xtask qa-automate: warning — could not write {}: {e}",
            report_path.display()
        ),
    }

    if fail > 0 {
        Err(format!("{fail} step(s) failed"))
    } else {
        Ok(())
    }
}

fn render_markdown(steps: &[Step], total: Duration) -> String {
    let pass = steps.iter().filter(|s| s.outcome == Outcome::Pass).count();
    let fail = steps.iter().filter(|s| s.outcome == Outcome::Fail).count();
    let skip = steps.iter().filter(|s| s.outcome == Outcome::Skipped).count();
    let mut out = String::new();
    out.push_str("# `xtask qa-automate` report\n\n");
    out.push_str(&format!(
        "**Result:** {pass} pass / {fail} fail / {skip} skip in {:.1}s\n\n",
        total.as_secs_f64()
    ));
    out.push_str("| Section | Status | Duration | Name | Note |\n");
    out.push_str("| --- | --- | ---: | --- | --- |\n");
    for s in steps {
        out.push_str(&format!(
            "| {} | {} | {:.1}s | {} | {} |\n",
            s.section.label(),
            s.outcome.glyph(),
            s.duration.as_secs_f64(),
            s.name,
            s.note.replace('|', "\\|")
        ));
    }
    out
}

// ---------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_default_args() {
        let a = parse_args(vec![]).unwrap();
        assert!(!a.skip_frontend);
        assert!(!a.skip_audit);
        assert!(!a.skip_vet);
        assert!(!a.skip_deny);
        assert!(!a.skip_tests);
        assert!(!a.skip_bench);
        assert!(!a.skip_clippy);
        assert!(!a.skip_git_check);
        assert!(!a.fail_fast);
        assert!(a.report.is_none());
    }

    #[test]
    fn parses_skip_flags() {
        let a = parse_args(vec![
            "--skip-frontend".into(),
            "--skip-audit".into(),
            "--skip-vet".into(),
            "--skip-deny".into(),
            "--skip-tests".into(),
            "--skip-bench".into(),
            "--skip-clippy".into(),
            "--skip-git-check".into(),
            "--fail-fast".into(),
        ])
        .unwrap();
        assert!(a.skip_frontend);
        assert!(a.skip_audit);
        assert!(a.skip_vet);
        assert!(a.skip_deny);
        assert!(a.skip_tests);
        assert!(a.skip_bench);
        assert!(a.skip_clippy);
        assert!(a.skip_git_check);
        assert!(a.fail_fast);
    }

    #[test]
    fn parses_report_path() {
        let a = parse_args(vec!["--report".into(), "out.md".into()]).unwrap();
        assert_eq!(a.report.as_deref(), Some(Path::new("out.md")));
    }

    #[test]
    fn rejects_unknown_flag() {
        let err = parse_args(vec!["--no-such".into()]).unwrap_err();
        assert!(err.contains("unknown flag"), "got {err:?}");
    }

    #[test]
    fn rejects_report_without_value() {
        let err = parse_args(vec!["--report".into()]).unwrap_err();
        assert!(err.contains("requires a path"), "got {err:?}");
    }

    /// Tripwire: when a new crate joins the workspace, `WORKSPACE_CRATES`
    /// has to grow alongside it. Reads `Cargo.toml` at runtime so a
    /// drift between the two surfaces fails the build before merge.
    /// Mirrors the Phase 17b deny.toml ↔ ci.yml drift test in spirit.
    /// The `apps/copythat-ui/src-tauri` workspace member maps to the
    /// `copythat-ui` package name; we hard-code that translation here
    /// because the package name doesn't appear in the workspace
    /// `members = [...]` block, only the path.
    #[test]
    fn workspace_crates_match_cargo_toml() {
        let root = repo_root().expect("repo root");
        let body = fs::read_to_string(root.join("Cargo.toml")).expect("read Cargo.toml");
        let mut declared: Vec<String> = body
            .lines()
            .filter_map(|l| {
                let l = l.trim();
                let inner = l.strip_prefix('"')?.strip_suffix("\",")?;
                if let Some(name) = inner.strip_prefix("crates/") {
                    Some(name.to_string())
                } else if inner == "xtask" {
                    Some(inner.to_string())
                } else if inner == "apps/copythat-ui/src-tauri" {
                    Some("copythat-ui".to_string())
                } else {
                    None
                }
            })
            .collect();
        declared.sort();
        let mut listed: Vec<String> =
            WORKSPACE_CRATES.iter().map(|s| (*s).to_string()).collect();
        listed.sort();
        assert_eq!(
            listed, declared,
            "WORKSPACE_CRATES drifted from Cargo.toml workspace.members"
        );
    }

    #[test]
    fn parses_advisory_ignores_from_deny_toml() {
        let root = repo_root().expect("repo root");
        let ids = read_advisory_ignores(&root).expect("read advisory ignores");
        assert!(!ids.is_empty(), "expected RUSTSEC ignores in deny.toml");
        for id in &ids {
            assert!(id.starts_with("RUSTSEC-"), "unexpected id `{id}`");
        }
    }

    #[test]
    fn renders_markdown_table() {
        let steps = vec![
            Step {
                section: Section::Static,
                name: "fmt".into(),
                outcome: Outcome::Pass,
                duration: Duration::from_secs(1),
                note: "ok".into(),
            },
            Step {
                section: Section::Tests,
                name: "cargo test -p foo".into(),
                outcome: Outcome::Fail,
                duration: Duration::from_secs(3),
                note: "1 failure".into(),
            },
            Step {
                section: Section::Security,
                name: "cargo vet".into(),
                outcome: Outcome::Skipped,
                duration: Duration::default(),
                note: "not installed".into(),
            },
        ];
        let md = render_markdown(&steps, Duration::from_secs(4));
        assert!(md.contains("PASS"), "missing PASS: {md}");
        assert!(md.contains("FAIL"), "missing FAIL: {md}");
        assert!(md.contains("SKIP"), "missing SKIP: {md}");
        assert!(md.contains("cargo test -p foo"));
        assert!(md.contains("1 pass / 1 fail / 1 skip"));
    }

    #[test]
    fn render_markdown_escapes_pipe_chars() {
        let steps = vec![Step {
            section: Section::Static,
            name: "x".into(),
            outcome: Outcome::Pass,
            duration: Duration::default(),
            note: "a | b".into(),
        }];
        let md = render_markdown(&steps, Duration::default());
        assert!(md.contains("a \\| b"), "pipe not escaped: {md}");
    }

    #[test]
    fn outcome_glyphs_are_distinct() {
        assert_eq!(Outcome::Pass.glyph(), "PASS");
        assert_eq!(Outcome::Fail.glyph(), "FAIL");
        assert_eq!(Outcome::Skipped.glyph(), "SKIP");
    }
}
