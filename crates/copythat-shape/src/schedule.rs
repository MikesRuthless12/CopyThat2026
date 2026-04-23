//! rclone-style time-of-day + day-of-week schedule.
//!
//! Grammar: a whitespace-separated list of `key,rate` tokens.
//!
//! - `key` is `HH:MM` for a time-of-day boundary OR a 3-letter
//!   weekday range like `Sat-Sun` (or a single day like `Mon`).
//! - `rate` is `off` (paused), `unlimited`, or `NNNN[kMGT]`
//!   (e.g. `512k`, `10M`, `2G`). Suffixes are powers of 1024.
//!
//! Day-range tokens take precedence over time-of-day boundaries;
//! they apply for the whole UTC-local day. Time-of-day boundaries
//! sort by minute-of-day at parse time and the boundary whose
//! minute-of-day is the largest value `<= now` wins.
//!
//! Example (mirrors the rclone docs):
//!
//! ```text
//! 08:00,512k 12:00,off 13:00,512k 18:00,10M Sat-Sun,unlimited
//! ```
//!
//! On a Tuesday at 09:00 → `512 KiB/s`; at 12:30 → off; at 14:00 →
//! `512 KiB/s`; at 19:00 → `10 MiB/s`. On Saturday at 13:00 →
//! unlimited (the day-range rule shadows the time-of-day chain).

use chrono::{DateTime, Datelike, Local, Timelike};

use crate::error::ScheduleError;
use crate::shape::ByteRate;

/// Bitmask of weekdays — bit 0 = Monday, bit 6 = Sunday.
/// Matches `chrono::Weekday::num_days_from_monday`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DayMask(pub u8);

impl DayMask {
    pub const fn empty() -> Self {
        Self(0)
    }

    pub const fn all() -> Self {
        Self(0b0111_1111)
    }

    /// True when `day` (Monday-zero-indexed) is set.
    pub const fn contains(self, day: u8) -> bool {
        self.0 & (1 << day) != 0
    }

    pub const fn bits(self) -> u8 {
        self.0
    }
}

/// A `[start, end)` minute-of-day window. `end` of `1440` means
/// "until midnight"; ranges that wrap past midnight are not
/// representable (rare in user-typed schedules — split into two
/// rules).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeWindow {
    pub start_minute_of_day: u32,
    pub end_minute_of_day: u32,
}

impl TimeWindow {
    /// 00:00..24:00 — the whole day. Day-range schedule rules
    /// always carry a full-day window.
    pub const FULL_DAY: TimeWindow = TimeWindow {
        start_minute_of_day: 0,
        end_minute_of_day: 1440,
    };

    /// True when `minute_of_day` falls inside the half-open
    /// `[start, end)` range.
    pub fn contains(&self, minute_of_day: u32) -> bool {
        minute_of_day >= self.start_minute_of_day && minute_of_day < self.end_minute_of_day
    }
}

/// One rule produced by [`Schedule::parse`]. Day-range and
/// time-of-day rules are stored uniformly; the difference is
/// whether `day_mask` is the full set (time-of-day) or a subset
/// (explicit day range).
#[derive(Debug, Clone)]
pub struct ScheduleRule {
    pub day_mask: DayMask,
    pub time_window: TimeWindow,
    /// `None` = unlimited during this window; `Some(ByteRate(0))`
    /// = paused; otherwise the cap.
    pub limit: Option<ByteRate>,
    /// `true` for day-range rules so [`Schedule::current_limit`]
    /// can prefer them over the time-of-day chain.
    is_day_range: bool,
}

#[derive(Debug, Clone, Default)]
pub struct Schedule {
    rules: Vec<ScheduleRule>,
}

impl Schedule {
    /// Empty schedule — `current_limit` always returns `None`.
    /// Used as a placeholder when the user opts out of scheduling.
    pub fn empty() -> Self {
        Self { rules: Vec::new() }
    }

    /// Parse a whitespace-separated rclone-style schedule.
    ///
    /// Returns the first error encountered. Tokens following an
    /// error are not parsed (the UI surfaces a single message
    /// rather than a noisy list); a follow-up phase can iterate
    /// the error set if the textarea grows beyond a one-line cap.
    pub fn parse(spec: &str) -> Result<Self, ScheduleError> {
        let mut time_boundaries: Vec<(u32, Option<ByteRate>)> = Vec::new();
        let mut day_rules: Vec<(DayMask, Option<ByteRate>)> = Vec::new();

        for token in spec.split_whitespace() {
            let (key, val) = token
                .split_once(',')
                .ok_or_else(|| ScheduleError::MissingComma {
                    token: token.to_string(),
                })?;
            let limit = parse_rate(val.trim())?;
            let key = key.trim();
            if let Some(start_minute) = parse_hh_mm(key)? {
                time_boundaries.push((start_minute, limit));
            } else if let Some(mask) = parse_day_range(key)? {
                day_rules.push((mask, limit));
            } else {
                return Err(ScheduleError::InvalidKey {
                    key: key.to_string(),
                });
            }
        }

        // Sort time boundaries so adjacent ones can pair into
        // `[start, next_start)` windows.
        time_boundaries.sort_by_key(|(m, _)| *m);

        let mut rules: Vec<ScheduleRule> = Vec::new();

        // Day-range rules become full-day windows on their respective
        // days. Stored first so `current_limit` can short-circuit on
        // a day-range hit before walking the time-of-day chain.
        for (mask, limit) in &day_rules {
            rules.push(ScheduleRule {
                day_mask: *mask,
                time_window: TimeWindow::FULL_DAY,
                limit: *limit,
                is_day_range: true,
            });
        }

        // Time-of-day boundaries — pair adjacent entries and
        // close out the last one at midnight.
        for (i, (start, limit)) in time_boundaries.iter().enumerate() {
            let end = time_boundaries.get(i + 1).map(|(s, _)| *s).unwrap_or(1440);
            rules.push(ScheduleRule {
                day_mask: DayMask::all(),
                time_window: TimeWindow {
                    start_minute_of_day: *start,
                    end_minute_of_day: end,
                },
                limit: *limit,
                is_day_range: false,
            });
        }

        Ok(Self { rules })
    }

    /// Pick the active cap for `now`.
    ///
    /// Day-range rules win over time-of-day boundaries. If multiple
    /// day-range rules cover `now`, the first-seen one wins (matches
    /// rclone's "first matching wins" semantic).
    ///
    /// `None` returned means either no rule matched (caller falls
    /// back to its global cap) or the matching rule says
    /// "unlimited" — these collapse for the engine because both
    /// translate to "don't shape".
    pub fn current_limit(&self, now: DateTime<Local>) -> Option<ByteRate> {
        let weekday_idx = now.weekday().num_days_from_monday() as u8;
        let minute_of_day = now.hour() * 60 + now.minute();

        // Day-range pass first.
        for rule in self.rules.iter().filter(|r| r.is_day_range) {
            if rule.day_mask.contains(weekday_idx) {
                return rule.limit;
            }
        }

        // Time-of-day chain.
        for rule in self.rules.iter().filter(|r| !r.is_day_range) {
            if rule.day_mask.contains(weekday_idx) && rule.time_window.contains(minute_of_day) {
                return rule.limit;
            }
        }

        None
    }

    /// True when `parse` produced no rules — the caller falls back
    /// to the global cap.
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }

    /// Direct access for tests + the Settings UI's "show me what
    /// you parsed" preview row.
    pub fn rules(&self) -> &[ScheduleRule] {
        &self.rules
    }
}

/// Parse `HH:MM`. Returns `Ok(Some(minute_of_day))` on a hit,
/// `Ok(None)` when the key has no colon (so the caller can try
/// `parse_day_range`), `Err` on a malformed time.
fn parse_hh_mm(key: &str) -> Result<Option<u32>, ScheduleError> {
    let Some((hh, mm)) = key.split_once(':') else {
        return Ok(None);
    };
    let h: u32 = hh.parse().map_err(|_| ScheduleError::InvalidKey {
        key: key.to_string(),
    })?;
    let m: u32 = mm.parse().map_err(|_| ScheduleError::InvalidKey {
        key: key.to_string(),
    })?;
    if h >= 24 || m >= 60 {
        return Err(ScheduleError::TimeOutOfRange {
            time: key.to_string(),
        });
    }
    Ok(Some(h * 60 + m))
}

/// Parse `Mon-Fri` / `Sat-Sun` / `Sat`. Returns `Ok(Some(mask))`
/// on a hit, `Ok(None)` when the key isn't a day-range pattern,
/// `Err` on a recognisable but malformed token.
fn parse_day_range(key: &str) -> Result<Option<DayMask>, ScheduleError> {
    if let Some((a, b)) = key.split_once('-') {
        let start = parse_weekday(a)?;
        let end = parse_weekday(b)?;
        let mut mask = 0u8;
        let mut i = start;
        loop {
            mask |= 1 << i;
            if i == end {
                break;
            }
            i = (i + 1) % 7;
            if i == start {
                // Walked the whole week (e.g. "Mon-Sun" sweeps all
                // days; the loop terminates when i wraps back to
                // start without ever hitting end). Treat as "all
                // days" rather than diverge.
                break;
            }
        }
        Ok(Some(DayMask(mask)))
    } else if let Ok(d) = parse_weekday(key) {
        Ok(Some(DayMask(1 << d)))
    } else {
        Ok(None)
    }
}

fn parse_weekday(s: &str) -> Result<u8, ScheduleError> {
    match s.to_ascii_lowercase().as_str() {
        "mon" | "monday" => Ok(0),
        "tue" | "tues" | "tuesday" => Ok(1),
        "wed" | "weds" | "wednesday" => Ok(2),
        "thu" | "thur" | "thurs" | "thursday" => Ok(3),
        "fri" | "friday" => Ok(4),
        "sat" | "saturday" => Ok(5),
        "sun" | "sunday" => Ok(6),
        _ => Err(ScheduleError::UnknownDay { day: s.to_string() }),
    }
}

/// Parse `off` / `unlimited` / `NNNN[kMGT]`. Returns
/// `Ok(None)` for unlimited, `Ok(Some(ByteRate))` for everything
/// else (including `off` → `Some(ByteRate(0))`).
fn parse_rate(rate: &str) -> Result<Option<ByteRate>, ScheduleError> {
    let lower = rate.to_ascii_lowercase();
    match lower.as_str() {
        "unlimited" | "inf" | "infinite" | "" => Ok(None),
        "off" | "0" => Ok(Some(ByteRate::new(0))),
        _ => {
            let (num_str, mult) = match lower.chars().last().unwrap_or('\0') {
                'k' => (&lower[..lower.len() - 1], 1024u64),
                'm' => (&lower[..lower.len() - 1], 1024 * 1024),
                'g' => (&lower[..lower.len() - 1], 1024 * 1024 * 1024),
                't' => (&lower[..lower.len() - 1], 1024 * 1024 * 1024 * 1024),
                _ => (lower.as_str(), 1u64),
            };
            let n: u64 = num_str.parse().map_err(|_| ScheduleError::InvalidRate {
                rate: rate.to_string(),
            })?;
            Ok(Some(ByteRate::new(n.saturating_mul(mult))))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn local(year: i32, month: u32, day: u32, hour: u32, minute: u32) -> DateTime<Local> {
        Local
            .with_ymd_and_hms(year, month, day, hour, minute, 0)
            .single()
            .expect("constructable local time")
    }

    #[test]
    fn parses_rclone_canonical_example() {
        let s = Schedule::parse("08:00,512k 12:00,off 13:00,512k 18:00,10M Sat-Sun,unlimited")
            .expect("valid schedule");
        // 4 time-of-day boundaries + 1 day-range rule.
        assert_eq!(s.rules().len(), 5);
    }

    #[test]
    fn empty_schedule_returns_none() {
        let s = Schedule::empty();
        assert!(s.is_empty());
        assert!(s.current_limit(local(2026, 4, 22, 9, 0)).is_none());
    }

    #[test]
    fn time_of_day_boundaries_track_correctly() {
        let s = Schedule::parse("08:00,512k 12:00,off 13:00,512k 18:00,10M").unwrap();
        // 2026-04-22 is a Wednesday.
        let kib = ByteRate::kibibytes_per_second(512);
        let mib = ByteRate::mebibytes_per_second(10);
        // Before 08:00 → no rule applies → None.
        assert!(s.current_limit(local(2026, 4, 22, 7, 30)).is_none());
        // 09:00 → 512 KiB/s.
        assert_eq!(s.current_limit(local(2026, 4, 22, 9, 0)), Some(kib));
        // 12:30 → off (ByteRate(0)).
        assert_eq!(
            s.current_limit(local(2026, 4, 22, 12, 30)),
            Some(ByteRate::new(0))
        );
        // 14:00 → 512 KiB/s.
        assert_eq!(s.current_limit(local(2026, 4, 22, 14, 0)), Some(kib));
        // 19:00 → 10 MiB/s.
        assert_eq!(s.current_limit(local(2026, 4, 22, 19, 0)), Some(mib));
    }

    #[test]
    fn day_range_rule_overrides_time_of_day() {
        let s = Schedule::parse("08:00,512k 18:00,10M Sat-Sun,unlimited").unwrap();
        // 2026-04-25 is a Saturday → unlimited regardless of hour.
        assert!(s.current_limit(local(2026, 4, 25, 13, 0)).is_none());
        assert!(s.current_limit(local(2026, 4, 25, 19, 0)).is_none());
        // 2026-04-22 (Wed) still honours the time-of-day chain.
        assert_eq!(
            s.current_limit(local(2026, 4, 22, 9, 0)),
            Some(ByteRate::kibibytes_per_second(512))
        );
    }

    #[test]
    fn rate_parser_handles_units() {
        assert_eq!(parse_rate("512").unwrap(), Some(ByteRate::new(512)));
        assert_eq!(parse_rate("512k").unwrap(), Some(ByteRate::new(512 * 1024)));
        assert_eq!(
            parse_rate("10M").unwrap(),
            Some(ByteRate::new(10 * 1024 * 1024))
        );
        assert_eq!(
            parse_rate("2G").unwrap(),
            Some(ByteRate::new(2 * 1024 * 1024 * 1024))
        );
        assert_eq!(parse_rate("off").unwrap(), Some(ByteRate::new(0)));
        assert_eq!(parse_rate("unlimited").unwrap(), None);
    }

    #[test]
    fn invalid_tokens_surface_typed_errors() {
        assert!(matches!(
            Schedule::parse("nope"),
            Err(ScheduleError::MissingComma { .. })
        ));
        assert!(matches!(
            Schedule::parse("25:00,512k"),
            Err(ScheduleError::TimeOutOfRange { .. })
        ));
        assert!(matches!(
            Schedule::parse("Foo-Sun,512k"),
            Err(ScheduleError::UnknownDay { .. })
        ));
        assert!(matches!(
            Schedule::parse("08:00,wat"),
            Err(ScheduleError::InvalidRate { .. })
        ));
    }

    #[test]
    fn day_mask_helpers() {
        let mut m = DayMask::empty();
        assert_eq!(m.bits(), 0);
        m = DayMask(0b0000_0001);
        assert!(m.contains(0));
        assert!(!m.contains(1));
        let all = DayMask::all();
        for d in 0..7 {
            assert!(all.contains(d));
        }
    }
}
