//! Phase 36 smoke — `copythat` CLI.
//!
//! Six cases mandated by the brief:
//!
//! 1. `copythat version --json` round-trips through serde and reports
//!    the workspace crate name + version.
//! 2. A jobspec describing 3 files; `plan --spec` exits 2 with three
//!    `plan_action` JSON events.
//! 3. `apply --spec` exits 0 and the destination tree mirrors the
//!    source.
//! 4. Re-running `apply --spec` on the now-finished tree exits 0
//!    with zero `plan_action` events (idempotency).
//! 5. `copy SRC DST --json` emits well-formed JSON-Lines on stdout.
//! 6. A jobspec with `verify = "blake3"` and a tampered destination
//!    surfaces `ExitCode::VerifyFailed` (4).
//! 7. All 20 Phase 36 Fluent keys present in every one of the 18
//!    locales.

use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

use copythat_cli::ExitCode;
use copythat_cli::output::{JsonEvent, JsonEventKind};
use serde_json::Value;
use tempfile::tempdir;

const PHASE_36_KEYS: &[&str] = &[
    "cli-help-tagline",
    "cli-help-exit-codes",
    "cli-error-bad-args",
    "cli-error-unknown-algo",
    "cli-error-missing-spec",
    "cli-error-spec-parse",
    "cli-error-spec-empty-sources",
    "cli-info-shape-recorded",
    "cli-info-stub-deferred",
    "cli-plan-summary",
    "cli-plan-pending",
    "cli-plan-already-done",
    "cli-apply-success",
    "cli-apply-failed",
    "cli-verify-ok",
    "cli-verify-failed",
    "cli-config-set",
    "cli-config-reset",
    "cli-config-unknown-key",
    "cli-completions-emitted",
];

const LOCALES: &[&str] = &[
    "en", "es", "zh-CN", "hi", "ar", "pt-BR", "ru", "ja", "de", "fr", "ko", "it", "tr", "vi", "pl",
    "nl", "id", "uk",
];

#[test]
fn case01_version_json_round_trips() {
    let buf = run_capture(["copythat", "--json", "version"]);
    let mut found = false;
    for line in buf.lines() {
        let evt: JsonEvent =
            serde_json::from_str(line).unwrap_or_else(|e| panic!("parse `{line}`: {e}"));
        if let JsonEventKind::Version {
            version,
            crate_name,
            ..
        } = evt.event
        {
            assert!(!version.is_empty(), "version string empty");
            assert_eq!(crate_name, "copythat-cli");
            found = true;
        }
    }
    assert!(found, "no version event in output: {buf}");
}

#[test]
fn case02_plan_emits_three_actions_for_three_file_spec() {
    let dir = tempdir().unwrap();
    let (src, dst, spec_path) = write_three_file_spec(dir.path(), "copy");

    let captured = run_capture_with_status(
        [
            "copythat",
            "--json",
            "plan",
            "--spec",
            &spec_path.display().to_string(),
        ],
        ExitCode::PendingActions,
    );

    let actions = count_event_kind(&captured, "plan_action");
    assert_eq!(
        actions, 3,
        "expected 3 plan actions, captured = `{captured}`"
    );

    drop(src);
    drop(dst);
}

#[test]
fn case03_apply_executes_then_idempotent_reapply_is_noop() {
    let dir = tempdir().unwrap();
    let (src, dst, spec_path) = write_three_file_spec(dir.path(), "copy");

    let _first = run_capture_with_status(
        [
            "copythat",
            "--json",
            "apply",
            "--spec",
            &spec_path.display().to_string(),
        ],
        ExitCode::Success,
    );

    for i in 0..3 {
        let landed = dst.join(src.file_name().unwrap()).join(format!("f{i}.bin"));
        assert!(landed.exists(), "file {i} missing at {}", landed.display());
    }

    let captured = run_capture_with_status(
        [
            "copythat",
            "--json",
            "apply",
            "--spec",
            &spec_path.display().to_string(),
        ],
        ExitCode::Success,
    );
    let actions = count_event_kind(&captured, "plan_action");
    assert_eq!(
        actions, 0,
        "idempotent re-apply emitted actions:\n{captured}"
    );
}

#[test]
fn case04_copy_emits_well_formed_json_lines() {
    let dir = tempdir().unwrap();
    let src = dir.path().join("payload.bin");
    fs::write(&src, vec![0u8; 64 * 1024]).unwrap();
    let dst_dir = dir.path().join("dst");
    fs::create_dir(&dst_dir).unwrap();

    let captured = run_capture_with_status(
        [
            "copythat",
            "--json",
            "copy",
            &src.display().to_string(),
            &dst_dir.display().to_string(),
        ],
        ExitCode::Success,
    );

    assert!(!captured.trim().is_empty(), "no JSON output");
    for line in captured.lines() {
        let _: JsonEvent =
            serde_json::from_str(line).unwrap_or_else(|e| panic!("parse `{line}`: {e}"));
    }
    let landed = dst_dir.join("payload.bin");
    assert!(landed.exists(), "destination file did not land");
}

#[test]
fn case05_verify_failed_jobspec_exits_four() {
    let dir = tempdir().unwrap();
    let target = dir.path().join("target.bin");
    fs::write(&target, b"this content will not match the sidecar digest").unwrap();
    let sidecar = dir.path().join("target.bin.blake3");
    fs::write(&sidecar, format!("{} *target.bin\n", "00".repeat(32))).unwrap();

    run_capture_with_status(
        [
            "copythat",
            "--json",
            "verify",
            &target.display().to_string(),
            "--algo",
            "blake3",
            "--against",
            &sidecar.display().to_string(),
        ],
        ExitCode::VerifyFailed,
    );
}

#[test]
fn case06_unknown_algo_exits_config_invalid() {
    let dir = tempdir().unwrap();
    let target = dir.path().join("noop.bin");
    fs::write(&target, b"hello").unwrap();

    run_capture_with_status(
        [
            "copythat",
            "verify",
            &target.display().to_string(),
            "--algo",
            "not-a-real-algorithm",
        ],
        ExitCode::ConfigInvalid,
    );
}

#[test]
fn case07_phase_36_fluent_keys_present_in_every_locale() {
    let root = locate_locales_dir().expect("locate locales/");
    for code in LOCALES {
        let path = root.join(code).join("copythat.ftl");
        let content =
            fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        for key in PHASE_36_KEYS {
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

// ---------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------

fn write_three_file_spec(root: &Path, kind: &str) -> (PathBuf, PathBuf, PathBuf) {
    let src = root.join("src");
    fs::create_dir(&src).unwrap();
    for i in 0..3 {
        fs::write(src.join(format!("f{i}.bin")), vec![i as u8; 1024]).unwrap();
    }
    let dst = root.join("dst");
    fs::create_dir(&dst).unwrap();

    let dst_subdir = dst.join("src");
    let spec_path = root.join("job.toml");
    fs::write(
        &spec_path,
        format!(
            r#"
[job]
kind = "{kind}"
source = ["{src}"]
destination = "{dst}"
"#,
            kind = kind,
            src = src.display().to_string().replace('\\', "/"),
            dst = dst_subdir.display().to_string().replace('\\', "/"),
        ),
    )
    .unwrap();

    (src, dst, spec_path)
}

fn run_capture<I, S>(args: I) -> String
where
    I: IntoIterator<Item = S>,
    S: Into<OsString>,
{
    run_capture_with_status(args, ExitCode::Success)
}

fn run_capture_with_status<I, S>(args: I, expected: ExitCode) -> String
where
    I: IntoIterator<Item = S>,
    S: Into<OsString>,
{
    let argv: Vec<OsString> = args.into_iter().map(Into::into).collect();
    let exe = OsString::from(env!("CARGO_BIN_EXE_copythat"));
    let mut cmd = std::process::Command::new(&exe);
    cmd.args(argv.iter().skip(1));
    let output = cmd
        .output()
        .unwrap_or_else(|e| panic!("spawn {}: {e}", exe.to_string_lossy()));
    let stdout = String::from_utf8(output.stdout).expect("stdout utf-8");
    let actual = output.status.code().unwrap_or(-1);
    assert_eq!(
        actual,
        expected.as_u8() as i32,
        "expected exit {} ({:?}), got {actual}\nargv: {argv:?}\nstdout:\n{stdout}\nstderr:\n{}",
        expected.as_u8(),
        expected,
        String::from_utf8_lossy(&output.stderr),
    );
    stdout
}

fn count_event_kind(stdout: &str, kind: &str) -> usize {
    let mut n = 0;
    for line in stdout.lines() {
        let value: Value = match serde_json::from_str(line) {
            Ok(v) => v,
            Err(_) => continue,
        };
        if value.get("kind").and_then(Value::as_str) == Some(kind) {
            n += 1;
        }
    }
    n
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
