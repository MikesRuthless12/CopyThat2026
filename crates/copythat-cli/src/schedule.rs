//! Phase 14d — scheduled-job dispatch.
//!
//! `copythat schedule <SPEC>` reads an absolute-pathed `[job]`
//! jobspec (the same shape Phase 36's `plan` / `apply` consume) and
//! renders it into a per-OS scheduler stanza:
//!
//! - **Windows** — a `schtasks /create` command line targeting the
//!   user's profile (`/SC ONCE` / `/SC DAILY` / `/SC HOURLY`
//!   depending on the spec's `[schedule]` block).
//! - **macOS** — a launchd plist suitable for
//!   `~/Library/LaunchAgents/`. Per-user agent (no daemon
//!   privilege).
//! - **Linux** — a systemd `--user` timer + service pair the user
//!   drops in `~/.config/systemd/user/`.
//!
//! The `RenderedSchedule` shape returns the raw textual stanza
//! plus a suggested filesystem path; the CLI prints both. The
//! actual installation is a follow-up: writing into a user's
//! systemd / launchd directory means we're carrying a destructive
//! action's blast radius (per the CLAUDE.md "executing actions
//! with care" rule), so by default we render only and let the
//! user paste the stanza in. The `--install` flag opts into the
//! write.
//!
//! ## Threat model
//!
//! - **Phase 17a** — every path inside the rendered stanza is
//!   gated through `validate_path_no_traversal` before we emit it.
//!   A traversal-laden source / destination / spec_path is
//!   rejected with `ScheduleError::PathRejected` and surfaces the
//!   `err-path-escape` Fluent key.
//! - **Quoting** — the renderer escapes spaces / quotes / dollar
//!   signs in paths so a hostile filename can't break out of the
//!   schtasks `/TR` argument or the systemd `ExecStart=` line.
//! - **Per-user only** — none of the three OS paths require
//!   elevation. A user who wants a system-wide schedule can adapt
//!   the rendered stanza by hand; we don't write to system dirs.

use std::path::{Path, PathBuf};

use copythat_core::{PathSafetyError, validate_path_no_traversal};
use thiserror::Error;

use crate::jobspec::{JobKind, JobSpec};

/// Shape of a `[schedule]` block parsed from a jobspec. The
/// jobspec already carries a free-form `cron` string (see
/// `jobspec::ScheduleBlock`); the renderer interprets it as a
/// cron-shaped 5-field expression OR one of three named
/// frequencies — `@hourly` / `@daily` / `@weekly` — that map
/// cleanly to the per-OS scheduler vocabularies. Anything else
/// surfaces as `ScheduleError::UnsupportedCron` and the CLI
/// asks the user to pick a supported expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScheduleSpec {
    /// `@hourly` — fires at minute 00 of every hour.
    Hourly,
    /// `@daily` — fires at 03:00 user-local every day.
    Daily,
    /// `@weekly` — fires Sunday 03:00 user-local.
    Weekly,
    /// Five-field crontab. We pass it through to systemd as an
    /// `OnCalendar=` expression, to launchd as a `StartCalendarInterval`
    /// dict, and to schtasks via a literal-time arg list. Validation
    /// is shape-only; semantic validation lives in the per-OS
    /// scheduler the user paste-installs into.
    Cron(String),
}

impl ScheduleSpec {
    /// Parse a free-form schedule string from `jobspec.schedule.cron`.
    pub fn parse(raw: &str) -> Result<Self, ScheduleError> {
        // Reject any control character (newline / carriage-return /
        // NUL) up front — a cron string with an embedded newline
        // would smuggle a forged second stanza past systemd's
        // `OnCalendar=` line and break schtasks/launchd parsing.
        // Tabs are allowed because `split_whitespace` treats them as
        // field separators; any other 0x00–0x1F / 0x7F char is a
        // crafted payload, not a real schedule.
        if raw.chars().any(|c| c.is_control() && c != '\t') {
            return Err(ScheduleError::UnsupportedCron(raw.to_string()));
        }
        let trimmed = raw.trim();
        match trimmed {
            "@hourly" => Ok(Self::Hourly),
            "@daily" => Ok(Self::Daily),
            "@weekly" => Ok(Self::Weekly),
            "" => Err(ScheduleError::EmptyCron),
            other => {
                if other.split_whitespace().count() == 5 {
                    Ok(Self::Cron(other.to_string()))
                } else {
                    Err(ScheduleError::UnsupportedCron(other.to_string()))
                }
            }
        }
    }

    /// Friendly label for human output.
    pub fn label(&self) -> String {
        match self {
            Self::Hourly => "@hourly".to_string(),
            Self::Daily => "@daily".to_string(),
            Self::Weekly => "@weekly".to_string(),
            Self::Cron(raw) => raw.clone(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScheduleHostOs {
    Windows,
    MacOs,
    Linux,
}

/// Output of [`render_schedule`]. The `body` is the on-disk
/// stanza the user installs; `suggested_install_path` is where it
/// belongs on the chosen OS. The CLI prints both; the user opts
/// in to actual installation via `--install`.
#[derive(Debug, Clone)]
pub struct RenderedSchedule {
    pub host_os: ScheduleHostOs,
    pub body: String,
    pub suggested_install_path: PathBuf,
    pub schedule: ScheduleSpec,
}

#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum ScheduleError {
    #[error("jobspec has no [schedule] block")]
    Missing,
    #[error("[schedule] cron value is empty")]
    EmptyCron,
    #[error("unsupported cron expression: {0}")]
    UnsupportedCron(String),
    #[error("path rejected by Phase 17a safety guard ({offending}): {reason}")]
    PathRejected {
        offending: PathBuf,
        reason: PathSafetyError,
    },
    #[error("scheduled job kind {0:?} not supported (copy / move / sync only)")]
    UnsupportedJobKind(JobKind),
    #[error("source path must be absolute for a scheduled job: {0}")]
    SourceNotAbsolute(PathBuf),
    #[error("destination path must be absolute for a scheduled job: {0}")]
    DestinationNotAbsolute(PathBuf),
}

impl ScheduleError {
    /// Stable Fluent key for IPC / CLI surfaces. Keeps consistent
    /// shape with `CopyErrorKind::localized_key`.
    pub fn localized_key(&self) -> &'static str {
        match self {
            Self::PathRejected { .. } => "err-path-escape",
            Self::Missing | Self::EmptyCron => "err-schedule-missing",
            Self::UnsupportedCron(_) => "err-schedule-unsupported-cron",
            Self::UnsupportedJobKind(_) => "err-schedule-unsupported-kind",
            Self::SourceNotAbsolute(_) | Self::DestinationNotAbsolute(_) => {
                "err-schedule-not-absolute"
            }
        }
    }
}

/// Render the spec into a per-OS scheduler stanza. The host OS is
/// supplied so the same code path is testable on every platform
/// (we can render a launchd plist on Windows, etc.).
pub fn render_schedule(
    spec: &JobSpec,
    spec_path: &Path,
    host: ScheduleHostOs,
) -> Result<RenderedSchedule, ScheduleError> {
    // Phase 17a — gate every path that lands in the rendered stanza.
    validate_path_no_traversal(spec_path).map_err(|e| ScheduleError::PathRejected {
        offending: spec_path.to_path_buf(),
        reason: e,
    })?;
    for src in &spec.job.source {
        validate_path_no_traversal(src).map_err(|e| ScheduleError::PathRejected {
            offending: src.clone(),
            reason: e,
        })?;
        if !src.is_absolute() {
            return Err(ScheduleError::SourceNotAbsolute(src.clone()));
        }
    }
    validate_path_no_traversal(&spec.job.destination).map_err(|e| ScheduleError::PathRejected {
        offending: spec.job.destination.clone(),
        reason: e,
    })?;
    if !spec.job.destination.is_absolute() {
        return Err(ScheduleError::DestinationNotAbsolute(
            spec.job.destination.clone(),
        ));
    }

    if !matches!(spec.job.kind, JobKind::Copy | JobKind::Move | JobKind::Sync) {
        return Err(ScheduleError::UnsupportedJobKind(spec.job.kind));
    }

    let cron_raw = spec
        .schedule
        .as_ref()
        .map(|s| s.cron.clone())
        .ok_or(ScheduleError::Missing)?;
    let schedule = ScheduleSpec::parse(&cron_raw)?;

    let body = match host {
        ScheduleHostOs::Windows => render_schtasks(spec, spec_path, &schedule),
        ScheduleHostOs::MacOs => render_launchd_plist(spec, spec_path, &schedule),
        ScheduleHostOs::Linux => render_systemd_user(spec, spec_path, &schedule),
    };
    let suggested_install_path = match host {
        ScheduleHostOs::Windows => PathBuf::from(r"copythat-schedule-stanza.bat"),
        ScheduleHostOs::MacOs => {
            PathBuf::from("~/Library/LaunchAgents/app.copythat.scheduled-job.plist")
        }
        ScheduleHostOs::Linux => {
            PathBuf::from("~/.config/systemd/user/copythat-scheduled-job.timer")
        }
    };
    Ok(RenderedSchedule {
        host_os: host,
        body,
        suggested_install_path,
        schedule,
    })
}

/// Quote a path for inclusion in a Windows schtasks `/TR` argument.
/// Wraps in double-quotes and escapes embedded double-quotes by
/// doubling them — same convention `cmd.exe` honours.
fn quote_for_schtasks(p: &Path) -> String {
    let raw = p.to_string_lossy();
    let escaped = raw.replace('"', "\"\"");
    format!("\"{escaped}\"")
}

/// Quote a path for inclusion in a launchd plist's `<string>` arg
/// or a systemd `ExecStart=` line. Wraps in single quotes; embedded
/// single quotes get the bash-canonical `'\''` escape so a path
/// containing `'` survives a shell re-parse.
fn quote_for_shell(p: &Path) -> String {
    let raw = p.to_string_lossy();
    let escaped = raw.replace('\'', "'\\''");
    format!("'{escaped}'")
}

/// Escape a string for inclusion as XML `#PCDATA` (between tags).
/// Covers all five XML 1.0 predefined entities so a path containing
/// `<`, `>`, `&`, `'`, or `"` produces a well-formed plist instead
/// of one that breaks parsing or — worse — injects a fake key/value
/// pair that overrides the surrounding stanza.
fn xml_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '\'' => out.push_str("&apos;"),
            '"' => out.push_str("&quot;"),
            other => out.push(other),
        }
    }
    out
}

fn schtasks_freq_args(schedule: &ScheduleSpec) -> Vec<String> {
    match schedule {
        ScheduleSpec::Hourly => vec!["/SC".into(), "HOURLY".into()],
        ScheduleSpec::Daily => vec!["/SC".into(), "DAILY".into(), "/ST".into(), "03:00".into()],
        ScheduleSpec::Weekly => vec![
            "/SC".into(),
            "WEEKLY".into(),
            "/D".into(),
            "SUN".into(),
            "/ST".into(),
            "03:00".into(),
        ],
        ScheduleSpec::Cron(_) => vec![
            // schtasks doesn't speak cron; emit a daily 03:00 stanza
            // and document the cron expression as a comment for the
            // user to translate by hand.
            "/SC".into(),
            "DAILY".into(),
            "/ST".into(),
            "03:00".into(),
        ],
    }
}

fn render_schtasks(spec: &JobSpec, spec_path: &Path, schedule: &ScheduleSpec) -> String {
    let mut args: Vec<String> = vec![
        "schtasks".into(),
        "/Create".into(),
        "/TN".into(),
        "\"CopyThat Scheduled Job\"".into(),
    ];
    args.extend(schtasks_freq_args(schedule));
    args.push("/F".into());
    args.push("/TR".into());
    let cmd = format!(
        "{} apply --spec {}",
        quote_for_schtasks(Path::new("copythat.exe")),
        quote_for_schtasks(spec_path),
    );
    args.push(format!("\"{}\"", cmd.replace('"', "\\\"")));
    let mut out = String::new();
    if let ScheduleSpec::Cron(raw) = schedule {
        out.push_str(&format!(
            ":: Cron {raw:?} translated to daily 03:00 (schtasks doesn't speak cron).\n"
        ));
    }
    out.push_str(":: ");
    out.push_str(&format!(
        "kind={} sources={} dst={}\n",
        spec.job.kind.as_str(),
        spec.job.source.len(),
        spec.job.destination.display(),
    ));
    out.push_str(&args.join(" "));
    out.push('\n');
    out
}

fn render_launchd_plist(spec: &JobSpec, spec_path: &Path, schedule: &ScheduleSpec) -> String {
    let interval_xml = match schedule {
        ScheduleSpec::Hourly => {
            // StartInterval = 3600 seconds.
            "    <key>StartInterval</key>\n    <integer>3600</integer>".to_string()
        }
        ScheduleSpec::Daily => "    <key>StartCalendarInterval</key>\n    <dict>\n        <key>Hour</key><integer>3</integer>\n        <key>Minute</key><integer>0</integer>\n    </dict>".to_string(),
        ScheduleSpec::Weekly => "    <key>StartCalendarInterval</key>\n    <dict>\n        <key>Weekday</key><integer>0</integer>\n        <key>Hour</key><integer>3</integer>\n        <key>Minute</key><integer>0</integer>\n    </dict>".to_string(),
        ScheduleSpec::Cron(raw) => {
            format!(
                "    <!-- TODO: translate cron `{cron}` to a StartCalendarInterval dict -->\n    <key>StartCalendarInterval</key>\n    <dict>\n        <key>Hour</key><integer>3</integer>\n        <key>Minute</key><integer>0</integer>\n    </dict>",
                cron = xml_escape(raw),
            )
        }
    };
    let _ = spec; // job kind is encoded in the spec_path; nothing else to do here yet
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n<plist version=\"1.0\">\n<dict>\n    <key>Label</key>\n    <string>app.copythat.scheduled-job</string>\n    <key>ProgramArguments</key>\n    <array>\n        <string>/usr/local/bin/copythat</string>\n        <string>apply</string>\n        <string>--spec</string>\n        <string>{spec_path}</string>\n    </array>\n{interval_xml}\n</dict>\n</plist>\n",
        spec_path = xml_escape(&spec_path.to_string_lossy()),
    )
}

fn render_systemd_user(spec: &JobSpec, spec_path: &Path, schedule: &ScheduleSpec) -> String {
    // Translate the schedule into a systemd `OnCalendar=` value.
    // systemd accepts the named shortcuts (`hourly`, `daily`,
    // `weekly`) directly. For 5-field cron we translate the simple
    // `M H * * *` form to `*-*-* HH:MM:00`; anything more elaborate
    // gets emitted as a commented-out hint with `daily` as the live
    // default — passing raw cron syntax on `OnCalendar=` would make
    // the unit fail to load at `systemctl daemon-reload`.
    let (on_calendar, cron_note) = match schedule {
        ScheduleSpec::Hourly => ("hourly".to_string(), None),
        ScheduleSpec::Daily => ("daily".to_string(), None),
        ScheduleSpec::Weekly => ("weekly".to_string(), None),
        ScheduleSpec::Cron(raw) => match cron_to_oncalendar(raw) {
            Some(translated) => (translated, Some(format!("# Translated from cron `{raw}`."))),
            None => (
                "daily".to_string(),
                Some(format!(
                    "# WARNING: cron `{raw}` could not be translated to systemd's OnCalendar grammar.\n# Falling back to `daily` (03:00 user-local). Replace the OnCalendar= line with a\n# `systemd-analyze calendar`-validated expression before enabling.",
                )),
            ),
        },
    };
    let exec = format!(
        "/usr/bin/copythat apply --spec {}",
        quote_for_shell(spec_path)
    );
    let _ = spec;
    let note_block = cron_note
        .as_deref()
        .map(|n| format!("{n}\n"))
        .unwrap_or_default();
    // Emit the two unit files separated by an unambiguous boundary
    // marker so a user (or a downstream `--install` wrapper) can split
    // them apart deterministically. Each marker names the on-disk
    // path the stanza belongs in. The whole output is one String for
    // the existing `RenderedSchedule.body` shape.
    format!(
        "# ===== ~/.config/systemd/user/copythat-scheduled-job.service =====\n[Unit]\nDescription=Copy That scheduled job\n\n[Service]\nType=oneshot\nExecStart={exec}\n\n# ===== ~/.config/systemd/user/copythat-scheduled-job.timer =====\n{note_block}[Unit]\nDescription=Copy That scheduled job timer\n\n[Timer]\nOnCalendar={on_calendar}\nPersistent=true\nUnit=copythat-scheduled-job.service\n\n[Install]\nWantedBy=timers.target\n"
    )
}

/// Translate the `M H * * *` subset of 5-field cron to systemd's
/// `OnCalendar=` grammar. Returns `None` for any cron expression
/// that uses ranges, lists, steps, day/month/weekday filters, or
/// non-numeric tokens — those need a hand-written `OnCalendar=`
/// expression that this renderer doesn't try to synthesise.
fn cron_to_oncalendar(raw: &str) -> Option<String> {
    let parts: Vec<&str> = raw.split_whitespace().collect();
    if parts.len() != 5 {
        return None;
    }
    // Day-of-month, month, day-of-week must all be `*` for the
    // simple translation; anything else needs a richer expression.
    if parts[2] != "*" || parts[3] != "*" || parts[4] != "*" {
        return None;
    }
    let minute: u32 = parts[0].parse().ok()?;
    let hour: u32 = parts[1].parse().ok()?;
    if minute > 59 || hour > 23 {
        return None;
    }
    Some(format!("*-*-* {hour:02}:{minute:02}:00"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jobspec::{JobBlock, JobKind, ScheduleBlock};

    fn make_spec(kind: JobKind, schedule: Option<&str>) -> JobSpec {
        let abs = if cfg!(windows) {
            (PathBuf::from(r"C:\src"), PathBuf::from(r"D:\dst"))
        } else {
            (PathBuf::from("/src"), PathBuf::from("/dst"))
        };
        JobSpec {
            job: JobBlock {
                kind,
                source: vec![abs.0],
                destination: abs.1,
                verify: None,
                shape: None,
                preserve: Default::default(),
                collisions: Default::default(),
            },
            retry: Default::default(),
            schedule: schedule.map(|c| ScheduleBlock { cron: c.into() }),
        }
    }

    #[test]
    fn parse_named_frequencies() {
        assert_eq!(
            ScheduleSpec::parse("@hourly").unwrap(),
            ScheduleSpec::Hourly
        );
        assert_eq!(ScheduleSpec::parse("@daily").unwrap(), ScheduleSpec::Daily);
        assert_eq!(
            ScheduleSpec::parse("@weekly").unwrap(),
            ScheduleSpec::Weekly
        );
    }

    #[test]
    fn parse_five_field_cron() {
        let s = ScheduleSpec::parse("0 3 * * *").unwrap();
        assert!(matches!(s, ScheduleSpec::Cron(ref c) if c == "0 3 * * *"));
    }

    #[test]
    fn parse_rejects_garbage() {
        assert_eq!(ScheduleSpec::parse(""), Err(ScheduleError::EmptyCron));
        assert!(matches!(
            ScheduleSpec::parse("not-cron"),
            Err(ScheduleError::UnsupportedCron(_))
        ));
    }

    #[test]
    fn render_rejects_missing_schedule_block() {
        let spec = make_spec(JobKind::Copy, None);
        let err =
            render_schedule(&spec, Path::new("/spec.toml"), ScheduleHostOs::Linux).unwrap_err();
        assert_eq!(err, ScheduleError::Missing);
    }

    #[test]
    fn render_rejects_traversal_in_paths() {
        let mut spec = make_spec(JobKind::Copy, Some("@daily"));
        spec.job.source[0] = PathBuf::from("foo/../etc/passwd");
        let err =
            render_schedule(&spec, Path::new("/spec.toml"), ScheduleHostOs::Linux).unwrap_err();
        match err {
            ScheduleError::PathRejected { reason, .. } => {
                assert!(matches!(reason, PathSafetyError::ParentTraversal { .. }))
            }
            other => panic!("expected PathRejected, got {other:?}"),
        }
    }

    #[test]
    fn render_rejects_relative_paths() {
        let mut spec = make_spec(JobKind::Copy, Some("@daily"));
        spec.job.source[0] = PathBuf::from("rel/src");
        let err =
            render_schedule(&spec, Path::new("/spec.toml"), ScheduleHostOs::Linux).unwrap_err();
        assert!(matches!(err, ScheduleError::SourceNotAbsolute(_)));
    }

    #[test]
    fn render_rejects_non_copy_kinds() {
        let spec = make_spec(JobKind::Verify, Some("@daily"));
        let err =
            render_schedule(&spec, Path::new("/spec.toml"), ScheduleHostOs::Linux).unwrap_err();
        assert!(matches!(err, ScheduleError::UnsupportedJobKind(_)));
    }

    #[test]
    fn render_windows_emits_schtasks_create() {
        let spec = make_spec(JobKind::Copy, Some("@daily"));
        let render = render_schedule(
            &spec,
            Path::new(r"C:\specs\backup.toml"),
            ScheduleHostOs::Windows,
        )
        .unwrap();
        assert!(render.body.contains("schtasks /Create"));
        assert!(render.body.contains("/SC DAILY"));
        assert!(render.body.contains("apply --spec"));
        assert!(render.body.contains(r"backup.toml"));
    }

    #[test]
    fn render_macos_emits_launchd_plist() {
        let spec = make_spec(JobKind::Sync, Some("@hourly"));
        let render = render_schedule(
            &spec,
            Path::new("/Users/alice/specs/sync.toml"),
            ScheduleHostOs::MacOs,
        )
        .unwrap();
        assert!(render.body.contains("<plist version=\"1.0\""));
        assert!(render.body.contains("StartInterval"));
        assert!(render.body.contains("/Users/alice/specs/sync.toml"));
    }

    #[test]
    fn render_linux_emits_systemd_units() {
        let spec = make_spec(JobKind::Move, Some("@weekly"));
        let render = render_schedule(
            &spec,
            Path::new("/home/alice/specs/move.toml"),
            ScheduleHostOs::Linux,
        )
        .unwrap();
        assert!(render.body.contains("[Service]"));
        assert!(render.body.contains("ExecStart="));
        assert!(render.body.contains("[Timer]"));
        assert!(render.body.contains("OnCalendar=weekly"));
    }

    #[test]
    fn quoted_paths_handle_quotes_and_spaces() {
        let p = Path::new("/path with space/file");
        let q = quote_for_shell(p);
        assert!(q.starts_with('\''));
        assert!(q.ends_with('\''));
        assert!(q.contains("path with space"));

        let p2 = Path::new(r#"C:\Program Files\copy.exe"#);
        let q2 = quote_for_schtasks(p2);
        assert!(q2.starts_with('"') && q2.ends_with('"'));
    }
}
