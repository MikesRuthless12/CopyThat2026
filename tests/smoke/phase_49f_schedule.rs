//! Phase 49f smoke — the in-process backup schedule evaluator.
//!
//! Validates the public `BackupSchedule` surface the GUI daemon + the
//! `copythat backup` CLI tick on: spec parse + round-trip, `is_due` across
//! the cadences, `next_after` anchoring in the future, and Manual being
//! inert. (The DST + bucket-edge math is unit-tested inside the module.)

use chrono::{DateTime, Local, TimeZone};
use copythat_shape::{BackupSchedule, BackupScheduleError};

fn at(y: i32, m: u32, d: u32, h: u32, min: u32) -> DateTime<Local> {
    Local.with_ymd_and_hms(y, m, d, h, min, 0).single().unwrap()
}

#[test]
fn parse_round_trips_and_rejects_rich_cron() {
    for spec in ["", "@hourly", "@daily", "@weekly", "30 2 * * *"] {
        let s = BackupSchedule::parse(spec).unwrap();
        assert_eq!(BackupSchedule::parse(&s.as_spec()).unwrap(), s);
    }
    assert!(matches!(
        BackupSchedule::parse("*/5 * * * *"),
        Err(BackupScheduleError::UnsupportedCron(_))
    ));
}

#[test]
fn hourly_and_daily_due_windows() {
    let last = at(2026, 6, 30, 10, 0);
    let h = BackupSchedule::Hourly;
    assert!(!h.is_due(last.timestamp_millis(), at(2026, 6, 30, 10, 30)));
    assert!(h.is_due(last.timestamp_millis(), at(2026, 6, 30, 11, 1)));

    let d = BackupSchedule::Daily;
    assert!(!d.is_due(last.timestamp_millis(), at(2026, 6, 30, 23, 0)));
    assert!(d.is_due(last.timestamp_millis(), at(2026, 7, 1, 4, 0)));
}

#[test]
fn manual_is_inert() {
    assert_eq!(BackupSchedule::parse("").unwrap(), BackupSchedule::Manual);
    assert!(!BackupSchedule::Manual.is_due(0, at(2030, 1, 1, 0, 0)));
    assert!(
        BackupSchedule::Manual
            .next_after(at(2026, 6, 30, 10, 0))
            .is_none()
    );
}

#[test]
fn next_after_is_strictly_future() {
    let now = at(2026, 6, 30, 10, 0);
    let next = BackupSchedule::Daily.next_after(now).unwrap();
    assert!(next > now.timestamp_millis());
}
