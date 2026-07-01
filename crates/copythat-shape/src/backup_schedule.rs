//! Phase 49f — in-process schedule evaluator for per-source backups.
//!
//! The CLI's `copythat schedule` (Phase 14d) *renders* OS scheduler stanzas
//! (schtasks / launchd / systemd) from an `@hourly` / `@daily` / `@weekly` /
//! `M H * * *` grammar. This module is the *other* side of the same grammar:
//! an in-process evaluator the GUI daemon (and `copythat backup run --due`)
//! ticks itself — "is this source due to run now, and when does it next
//! fire?".
//!
//! Anchors match the CLI renderer: `@daily` fires at 03:00 local, `@weekly`
//! at Sunday 03:00 local, `@hourly` at minute 0, and `M H * * *` at HH:MM
//! local every day. Richer cron (ranges / lists / steps / day-of-month /
//! month / weekday filters) is refused with
//! [`BackupScheduleError::UnsupportedCron`], mirroring the CLI's
//! `err-schedule-unsupported-cron`.
//!
//! All wall-clock reasoning is in `Local` so the anchors mean what the user
//! expects ("03:00" is 03:00 *their* time). `is_due` takes the last
//! successful run as epoch-ms so it composes with the `last_run_at_ms`
//! persisted in settings.

use chrono::{DateTime, Datelike, Duration, Local, NaiveDate, TimeZone, Timelike};
use thiserror::Error;

/// Local hour the `@daily` / `@weekly` anchors fire at (mirrors the CLI
/// renderer's 03:00 default).
const ANCHOR_HOUR: u32 = 3;

/// A parsed per-source backup schedule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupSchedule {
    /// Never auto-runs (still runnable on demand). The empty string and
    /// `"manual"` parse to this.
    Manual,
    /// Fires at minute 0 of every hour.
    Hourly,
    /// Fires at 03:00 local every day.
    Daily,
    /// Fires Sunday 03:00 local.
    Weekly,
    /// Fires at `hour:minute` local every day — the supported
    /// `M H * * *` cron subset.
    Cron {
        /// Minute of the hour (0–59).
        minute: u32,
        /// Hour of the day (0–23).
        hour: u32,
    },
}

/// Why a schedule spec didn't parse.
#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum BackupScheduleError {
    /// The spec was neither a recognised shortcut (`@hourly`/`@daily`/
    /// `@weekly`/empty) nor the supported `M H * * *` cron subset
    /// (numeric minute+hour, `*` for day-of-month / month / weekday).
    #[error("unsupported backup schedule: {0}")]
    UnsupportedCron(String),
}

impl BackupScheduleError {
    /// Stable Fluent key, shared with the Phase 14d CLI renderer so the
    /// GUI + CLI surface the same message.
    #[must_use]
    pub fn localized_key(&self) -> &'static str {
        "err-schedule-unsupported-cron"
    }
}

impl BackupSchedule {
    /// Parse a schedule spec. `""` / `"manual"` → [`Self::Manual`];
    /// `@hourly` / `@daily` / `@weekly`; or a `M H * * *` cron with a
    /// numeric minute + hour and `*` for the remaining three fields.
    pub fn parse(spec: &str) -> Result<Self, BackupScheduleError> {
        // Reject control chars (newline-smuggling) up front, like the CLI
        // parser — a schedule string is never multi-line.
        if spec.chars().any(|c| c.is_control() && c != '\t') {
            return Err(BackupScheduleError::UnsupportedCron(spec.to_string()));
        }
        match spec.trim() {
            "" | "manual" => Ok(Self::Manual),
            "@hourly" => Ok(Self::Hourly),
            "@daily" => Ok(Self::Daily),
            "@weekly" => Ok(Self::Weekly),
            other => Self::parse_cron(other),
        }
    }

    fn parse_cron(spec: &str) -> Result<Self, BackupScheduleError> {
        let unsupported = || BackupScheduleError::UnsupportedCron(spec.to_string());
        let parts: Vec<&str> = spec.split_whitespace().collect();
        if parts.len() != 5 || parts[2] != "*" || parts[3] != "*" || parts[4] != "*" {
            return Err(unsupported());
        }
        let minute: u32 = parts[0].parse().map_err(|_| unsupported())?;
        let hour: u32 = parts[1].parse().map_err(|_| unsupported())?;
        if minute > 59 || hour > 23 {
            return Err(unsupported());
        }
        Ok(Self::Cron { minute, hour })
    }

    /// The canonical spec string (round-trips through [`Self::parse`]).
    #[must_use]
    pub fn as_spec(&self) -> String {
        match self {
            Self::Manual => String::new(),
            Self::Hourly => "@hourly".to_string(),
            Self::Daily => "@daily".to_string(),
            Self::Weekly => "@weekly".to_string(),
            Self::Cron { minute, hour } => format!("{minute} {hour} * * *"),
        }
    }

    /// Is the source due to run at `now`, given its last successful run
    /// (`last_run_ms`, milliseconds since epoch; `0` = never)? True when a
    /// scheduled fire falls in the half-open interval `(last_run, now]`.
    /// A never-run schedule (`last_run_ms == 0`) is due on the next tick.
    /// [`Self::Manual`] is never due.
    #[must_use]
    pub fn is_due(&self, last_run_ms: i64, now: DateTime<Local>) -> bool {
        if matches!(self, Self::Manual) {
            return false;
        }
        let Some(last_run) = Local.timestamp_millis_opt(last_run_ms).single() else {
            return false;
        };
        // First fire strictly after the last run; due once `now` reaches it.
        match self.next_fire_at_or_after(last_run + Duration::milliseconds(1)) {
            Some(next) => next <= now,
            None => false,
        }
    }

    /// Next fire at or after `now`, as milliseconds since the Unix epoch.
    /// `None` for [`Self::Manual`].
    #[must_use]
    pub fn next_after(&self, now: DateTime<Local>) -> Option<i64> {
        self.next_fire_at_or_after(now)
            .map(|dt| dt.timestamp_millis())
    }

    /// Earliest scheduled fire at or after `t`.
    fn next_fire_at_or_after(&self, t: DateTime<Local>) -> Option<DateTime<Local>> {
        match self {
            Self::Manual => None,
            Self::Hourly => {
                let trunc = t.with_minute(0)?.with_second(0)?.with_nanosecond(0)?;
                Some(if trunc >= t {
                    trunc
                } else {
                    trunc + Duration::hours(1)
                })
            }
            Self::Daily => Some(next_daily(t, ANCHOR_HOUR, 0)),
            Self::Weekly => Some(next_weekly_sunday(t, ANCHOR_HOUR, 0)),
            Self::Cron { minute, hour } => Some(next_daily(t, *hour, *minute)),
        }
    }
}

/// Next `hour:minute` local fire at or after `t`.
fn next_daily(t: DateTime<Local>, hour: u32, minute: u32) -> DateTime<Local> {
    let today = local_at(t.date_naive(), hour, minute);
    if today >= t {
        today
    } else {
        local_at(t.date_naive() + Duration::days(1), hour, minute)
    }
}

/// Next Sunday `hour:minute` local fire at or after `t`.
fn next_weekly_sunday(t: DateTime<Local>, hour: u32, minute: u32) -> DateTime<Local> {
    let days_to_sunday = i64::from((7 - t.weekday().num_days_from_sunday()) % 7);
    let candidate_date = t.date_naive() + Duration::days(days_to_sunday);
    let candidate = local_at(candidate_date, hour, minute);
    if candidate >= t {
        candidate
    } else {
        local_at(candidate_date + Duration::days(7), hour, minute)
    }
}

/// Construct the local `DateTime` at `date` `hour:minute:00`, resolving DST
/// ambiguity to the earliest instant and stepping past a spring-forward gap
/// by an hour (the once-a-year edge a backup scheduler can tolerate).
fn local_at(date: NaiveDate, hour: u32, minute: u32) -> DateTime<Local> {
    let naive = date
        .and_hms_opt(hour, minute, 0)
        .unwrap_or_else(|| date.and_hms_opt(0, 0, 0).expect("midnight is always valid"));
    Local
        .from_local_datetime(&naive)
        .earliest()
        .unwrap_or_else(|| {
            Local
                .from_local_datetime(&(naive + Duration::hours(1)))
                .earliest()
                .unwrap_or_else(Local::now)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(y: i32, m: u32, d: u32, h: u32, min: u32) -> DateTime<Local> {
        Local.with_ymd_and_hms(y, m, d, h, min, 0).single().unwrap()
    }

    #[test]
    fn parse_shortcuts_and_manual() {
        assert_eq!(BackupSchedule::parse("").unwrap(), BackupSchedule::Manual);
        assert_eq!(
            BackupSchedule::parse("manual").unwrap(),
            BackupSchedule::Manual
        );
        assert_eq!(
            BackupSchedule::parse("@hourly").unwrap(),
            BackupSchedule::Hourly
        );
        assert_eq!(
            BackupSchedule::parse("@daily").unwrap(),
            BackupSchedule::Daily
        );
        assert_eq!(
            BackupSchedule::parse("@weekly").unwrap(),
            BackupSchedule::Weekly
        );
    }

    #[test]
    fn parse_cron_subset_and_rejections() {
        assert_eq!(
            BackupSchedule::parse("30 2 * * *").unwrap(),
            BackupSchedule::Cron {
                minute: 30,
                hour: 2
            }
        );
        for bad in [
            "*/5 * * * *",
            "0 0 1 * *",
            "60 3 * * *",
            "0 24 * * *",
            "nonsense",
        ] {
            assert!(
                matches!(
                    BackupSchedule::parse(bad),
                    Err(BackupScheduleError::UnsupportedCron(_))
                ),
                "expected {bad:?} to be rejected"
            );
        }
    }

    #[test]
    fn spec_round_trips() {
        for spec in ["", "@hourly", "@daily", "@weekly", "30 2 * * *"] {
            let parsed = BackupSchedule::parse(spec).unwrap();
            assert_eq!(BackupSchedule::parse(&parsed.as_spec()).unwrap(), parsed);
        }
    }

    #[test]
    fn hourly_is_due_after_the_hour_boundary() {
        let last = at(2026, 6, 30, 10, 0);
        // 30 min later: the 11:00 fire hasn't arrived.
        assert!(!BackupSchedule::Hourly.is_due(last.timestamp_millis(), at(2026, 6, 30, 10, 30)));
        // 61 min later: past 11:00 → due.
        assert!(BackupSchedule::Hourly.is_due(last.timestamp_millis(), at(2026, 6, 30, 11, 1)));
    }

    #[test]
    fn daily_not_due_same_day_then_due_next_day() {
        // Ran at 10:00, after today's 03:00 anchor.
        let last = at(2026, 6, 30, 10, 0);
        assert!(!BackupSchedule::Daily.is_due(last.timestamp_millis(), at(2026, 6, 30, 23, 0)));
        // Next day past 03:00 → due.
        assert!(BackupSchedule::Daily.is_due(last.timestamp_millis(), at(2026, 7, 1, 4, 0)));
    }

    #[test]
    fn manual_never_due_and_has_no_next() {
        let last = at(2026, 6, 30, 10, 0);
        assert!(!BackupSchedule::Manual.is_due(last.timestamp_millis(), at(2030, 1, 1, 0, 0)));
        assert!(!BackupSchedule::Manual.is_due(0, at(2030, 1, 1, 0, 0)));
        assert!(
            BackupSchedule::Manual
                .next_after(at(2026, 6, 30, 10, 0))
                .is_none()
        );
    }

    #[test]
    fn never_run_is_due_immediately() {
        // last_run_ms == 0 (never) → due on the next tick for any cadence.
        assert!(BackupSchedule::Daily.is_due(0, at(2026, 6, 30, 12, 0)));
    }

    #[test]
    fn next_after_is_in_the_future_at_the_anchor() {
        let now = at(2026, 6, 30, 10, 0); // after 03:00
        let next = BackupSchedule::Daily.next_after(now).unwrap();
        assert!(next > now.timestamp_millis());
        let next_dt = Local.timestamp_millis_opt(next).single().unwrap();
        assert_eq!(next_dt.hour(), 3);
        assert_eq!(next_dt.minute(), 0);
        // The day after, since today's 03:00 already passed.
        assert_eq!(next_dt.date_naive(), at(2026, 7, 1, 3, 0).date_naive());
    }

    #[test]
    fn weekly_anchors_on_sunday() {
        // 2026-06-30 is a Tuesday; the next Sunday is 2026-07-05.
        let now = at(2026, 6, 30, 10, 0);
        let next = BackupSchedule::Weekly.next_after(now).unwrap();
        let next_dt = Local.timestamp_millis_opt(next).single().unwrap();
        assert_eq!(next_dt.weekday(), chrono::Weekday::Sun);
        assert_eq!(next_dt.hour(), 3);
    }
}
