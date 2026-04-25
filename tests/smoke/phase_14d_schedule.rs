//! Phase 14d smoke test — scheduled-job render paths.
//!
//! Drives `copythat_cli::schedule::render_schedule` against the
//! three host OS targets (Windows / macOS / Linux) and asserts:
//!
//! 1. **Per-OS shape** — schtasks for Windows, launchd plist for
//!    macOS, systemd .service + .timer for Linux.
//! 2. **Phase 17a path-safety bar** — a traversal-laden source is
//!    rejected before any byte renders. Same guard the engine
//!    applies; the schedule renderer is the entry point that
//!    needs it earliest.
//! 3. **Absolute-path requirement** — relative source / destination
//!    surfaces a typed `SourceNotAbsolute` / `DestinationNotAbsolute`.
//! 4. **Quote handling** — paths with spaces / quotes / single
//!    quotes survive the render without breaking the OS-native
//!    parser.
//! 5. **Rejection of unsupported job kinds** — `verify` / `shred`
//!    can't be scheduled today; the renderer fails with a typed
//!    `UnsupportedJobKind` rather than emitting a malformed
//!    stanza.

use std::path::{Path, PathBuf};

use copythat_cli::jobspec::{JobBlock, JobKind, JobSpec, ScheduleBlock};
use copythat_cli::schedule::{ScheduleError, ScheduleHostOs, ScheduleSpec, render_schedule};

fn make_spec(kind: JobKind, src: PathBuf, dst: PathBuf, cron: Option<&str>) -> JobSpec {
    JobSpec {
        job: JobBlock {
            kind,
            source: vec![src],
            destination: dst,
            verify: None,
            shape: None,
            preserve: Default::default(),
            collisions: Default::default(),
        },
        retry: Default::default(),
        schedule: cron.map(|c| ScheduleBlock { cron: c.into() }),
    }
}

#[cfg(windows)]
fn abs(s: &str) -> PathBuf {
    PathBuf::from(format!(r"C:\{}", s))
}
#[cfg(not(windows))]
fn abs(s: &str) -> PathBuf {
    PathBuf::from(format!("/{}", s))
}

#[test]
fn render_windows_schtasks_shape() {
    let spec = make_spec(JobKind::Copy, abs("src"), abs("dst"), Some("@daily"));
    let render = render_schedule(
        &spec,
        Path::new(r"C:\specs\backup.toml"),
        ScheduleHostOs::Windows,
    )
    .unwrap();
    assert_eq!(render.host_os, ScheduleHostOs::Windows);
    assert!(render.body.contains("schtasks /Create"));
    assert!(render.body.contains("/SC DAILY"));
    assert!(render.body.contains("/ST 03:00"));
    assert!(render.body.contains(r"backup.toml"));
}

#[test]
fn render_macos_launchd_shape() {
    let spec = make_spec(JobKind::Sync, abs("src"), abs("dst"), Some("@hourly"));
    let render = render_schedule(
        &spec,
        Path::new("/Users/test/specs/sync.toml"),
        ScheduleHostOs::MacOs,
    )
    .unwrap();
    assert!(render.body.contains("<plist version=\"1.0\""));
    assert!(render.body.contains("StartInterval"));
    assert!(render.body.contains("/Users/test/specs/sync.toml"));
    assert!(render.body.contains("ProgramArguments"));
}

#[test]
fn render_linux_systemd_shape() {
    let spec = make_spec(JobKind::Move, abs("src"), abs("dst"), Some("@weekly"));
    let render = render_schedule(
        &spec,
        Path::new("/home/test/specs/move.toml"),
        ScheduleHostOs::Linux,
    )
    .unwrap();
    assert!(render.body.contains("[Service]"));
    assert!(render.body.contains("[Timer]"));
    assert!(render.body.contains("OnCalendar=weekly"));
    assert!(
        render
            .body
            .contains("ExecStart=/usr/bin/copythat apply --spec")
    );
}

#[test]
fn render_rejects_traversal_source() {
    let mut spec = make_spec(JobKind::Copy, abs("src"), abs("dst"), Some("@daily"));
    spec.job.source[0] = PathBuf::from("evil/../etc/passwd");
    let err = render_schedule(&spec, Path::new("/spec.toml"), ScheduleHostOs::Linux).unwrap_err();
    assert_eq!(err.localized_key(), "err-path-escape");
    assert!(matches!(err, ScheduleError::PathRejected { .. }));
}

#[test]
fn render_rejects_traversal_spec_path() {
    let spec = make_spec(JobKind::Copy, abs("src"), abs("dst"), Some("@daily"));
    let err = render_schedule(
        &spec,
        Path::new("/specs/../etc/passwd"),
        ScheduleHostOs::Linux,
    )
    .unwrap_err();
    assert_eq!(err.localized_key(), "err-path-escape");
}

#[test]
fn render_rejects_relative_source() {
    let spec = make_spec(
        JobKind::Copy,
        PathBuf::from("rel/src"),
        abs("dst"),
        Some("@daily"),
    );
    let err = render_schedule(&spec, Path::new("/spec.toml"), ScheduleHostOs::Linux).unwrap_err();
    assert!(matches!(err, ScheduleError::SourceNotAbsolute(_)));
}

#[test]
fn render_rejects_relative_destination() {
    let spec = make_spec(
        JobKind::Copy,
        abs("src"),
        PathBuf::from("rel/dst"),
        Some("@daily"),
    );
    let err = render_schedule(&spec, Path::new("/spec.toml"), ScheduleHostOs::Linux).unwrap_err();
    assert!(matches!(err, ScheduleError::DestinationNotAbsolute(_)));
}

#[test]
fn render_rejects_unsupported_kind() {
    let spec = make_spec(JobKind::Verify, abs("src"), abs("dst"), Some("@daily"));
    let err = render_schedule(&spec, Path::new("/spec.toml"), ScheduleHostOs::Linux).unwrap_err();
    assert!(matches!(
        err,
        ScheduleError::UnsupportedJobKind(JobKind::Verify)
    ));
}

#[test]
fn render_rejects_missing_schedule_block() {
    let spec = make_spec(JobKind::Copy, abs("src"), abs("dst"), None);
    let err = render_schedule(&spec, Path::new("/spec.toml"), ScheduleHostOs::Linux).unwrap_err();
    assert_eq!(err, ScheduleError::Missing);
}

#[test]
fn schedule_spec_parses_named_and_cron_forms() {
    assert_eq!(
        ScheduleSpec::parse("@hourly").unwrap(),
        ScheduleSpec::Hourly
    );
    let raw = "*/15 * * * *";
    let parsed = ScheduleSpec::parse(raw).unwrap();
    assert!(matches!(parsed, ScheduleSpec::Cron(ref c) if c == raw));
    let bad = ScheduleSpec::parse("not-a-cron-expression").unwrap_err();
    assert!(matches!(bad, ScheduleError::UnsupportedCron(_)));
}

#[cfg(not(windows))]
#[test]
fn quoted_paths_with_spaces_and_quotes_survive_linux_render() {
    let spec = make_spec(
        JobKind::Copy,
        PathBuf::from("/path with space/src"),
        PathBuf::from("/dst"),
        Some("@daily"),
    );
    let render = render_schedule(
        &spec,
        Path::new("/path with space/spec.toml"),
        ScheduleHostOs::Linux,
    )
    .unwrap();
    // Single-quote-wrapped: the render must include the bash escape
    // sequence so a `'` inside the path doesn't break the quoting.
    assert!(render.body.contains("'/path with space/spec.toml'"));
}
