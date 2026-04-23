//! Typed failure for schedule parsing + shape construction.
//!
//! Both error kinds are informational — `Shape` construction is
//! infallible once a valid [`crate::ByteRate`] is provided, and
//! [`Schedule::parse`](crate::Schedule::parse) accumulates every
//! bad token's position so the Settings UI can highlight the exact
//! offset the user typed wrong.

use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum ScheduleError {
    /// A token didn't have the required `key,rate` comma split.
    #[error("schedule token `{token}` is missing the `,<rate>` suffix")]
    MissingComma { token: String },
    /// The key half (`HH:MM` or `DAY-DAY`) didn't parse.
    #[error("schedule key `{key}` is neither HH:MM nor a day range")]
    InvalidKey { key: String },
    /// The rate half was neither `off` / `unlimited` / a number
    /// followed by an optional k / M / G / T suffix.
    #[error("schedule rate `{rate}` is not `off`, `unlimited`, or `NNNN[kMGT]`")]
    InvalidRate { rate: String },
    /// `HH:MM` was syntactically sensible but `HH >= 24` or `MM >= 60`.
    #[error("schedule time `{time}` is out of range (HH<24, MM<60)")]
    TimeOutOfRange { time: String },
    /// A day abbreviation wasn't one of Mon / Tue / Wed / Thu / Fri /
    /// Sat / Sun (case-insensitive).
    #[error("schedule day `{day}` is not a three-letter weekday")]
    UnknownDay { day: String },
}

#[derive(Debug, Clone, Error)]
pub enum ShapeError {
    /// `Shape::new` received a [`crate::ByteRate`] that overflows
    /// the `u32` domain governor's GCRA bucket uses internally.
    /// Anything under 4 GiB/s is safe; the cap is there so a
    /// pathological Settings blob (`"9999999M"`) can't panic.
    #[error("shape rate `{bytes_per_second}` bytes/s exceeds the 4 GiB/s ceiling")]
    RateExceedsCeiling { bytes_per_second: u64 },
}
