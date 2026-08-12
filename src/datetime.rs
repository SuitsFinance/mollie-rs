//! ISO 8601 date and datetime helpers for Mollie request/response fields.
//!
//! Mollie timestamps are ISO 8601 strings with an explicit offset, for example
//! `2026-01-22T10:39:23+00:00`. Calendar-only fields (e.g. `dueDate`) use
//! `YYYY-MM-DD`.
//!
//! Generated OpenAPI types keep many of these as plain `String` or
//! [`chrono::NaiveDate`]. This module validates and formats values before you
//! assign them.
//!
//! See `docs/iso/iso-8601.md`.
#![warn(missing_docs)]

use std::{fmt, str::FromStr};

use chrono::{
    DateTime as ChronoDateTime, Datelike, FixedOffset, NaiveDate, SecondsFormat, TimeZone, Utc,
};

use crate::{MollieError, MollieResult};

/// An ISO 8601 / RFC 3339 datetime with an explicit UTC offset.
///
/// Stored as [`chrono::DateTime<FixedOffset>`]. Wire form uses a complete date,
/// time, and offset (e.g. `2026-01-22T10:39:23+00:00` or `…Z`).
///
/// Naive local datetimes **without** an offset are rejected so API fields never
/// receive ambiguous values.
///
/// # Examples
///
/// ```rust
/// use mollie_rs::DateTime;
///
/// let dt = DateTime::parse("2026-01-22T10:39:23+00:00")?;
/// assert_eq!(dt.to_rfc3339(), "2026-01-22T10:39:23+00:00");
/// assert!(DateTime::parse("2026-01-22T10:39:23").is_err());
/// # Ok::<(), mollie_rs::MollieError>(())
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DateTime(ChronoDateTime<FixedOffset>);

impl DateTime {
    /// Parses an ISO 8601 datetime that includes a timezone offset (RFC 3339 profile).
    ///
    /// Accepted examples:
    /// - `2026-01-22T10:39:23+00:00`
    /// - `2026-01-22T10:39:23Z`
    /// - `2026-01-22T12:39:23+02:00`
    /// - fractional seconds: `2026-01-22T10:39:23.123+00:00`
    ///
    /// # Errors
    ///
    /// Returns [`MollieError::InvalidRequest`] when the value is empty, not a
    /// valid ISO 8601/RFC 3339 datetime, or lacks a timezone offset.
    pub fn parse(value: impl AsRef<str>) -> MollieResult<Self> {
        let value = value.as_ref().trim();
        if value.is_empty() {
            return Err(MollieError::invalid_request(
                "datetime cannot be empty; use ISO 8601 with offset (example: `2026-01-22T10:39:23+00:00`)",
            ));
        }
        // RFC 3339 is the interoperable ISO 8601 profile Mollie uses on the wire.
        match ChronoDateTime::parse_from_rfc3339(value) {
            Ok(dt) => Ok(Self(dt)),
            Err(err) => Err(MollieError::invalid_request(format!(
                "invalid ISO 8601 datetime `{value}`: {err} (require date, time, and offset; example: `2026-01-22T10:39:23+00:00`)"
            ))),
        }
    }

    /// Returns true when `value` is a valid offset-aware ISO 8601 datetime.
    pub fn is_valid(value: impl AsRef<str>) -> bool {
        Self::parse(value).is_ok()
    }

    /// Current UTC time as an offset-aware datetime (`+00:00`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::DateTime;
    ///
    /// let now = DateTime::now_utc();
    /// assert!(now.to_rfc3339().contains('+') || now.to_rfc3339().ends_with('Z') || now.to_rfc3339().contains("+00:00"));
    /// ```
    pub fn now_utc() -> Self {
        Self(Utc::now().fixed_offset())
    }

    /// Builds from a UTC [`chrono::DateTime`].
    pub fn from_utc(dt: ChronoDateTime<Utc>) -> Self {
        Self(dt.fixed_offset())
    }

    /// Builds from any chrono datetime by converting to a fixed offset.
    pub fn from_chrono<Tz: TimeZone>(dt: ChronoDateTime<Tz>) -> Self {
        Self(dt.fixed_offset())
    }

    /// Returns the inner chrono datetime.
    pub fn as_chrono(&self) -> &ChronoDateTime<FixedOffset> {
        &self.0
    }

    /// Consumes the value and returns the inner chrono datetime.
    pub fn into_chrono(self) -> ChronoDateTime<FixedOffset> {
        self.0
    }

    /// Returns the instant in UTC.
    pub fn to_utc(self) -> ChronoDateTime<Utc> {
        self.0.with_timezone(&Utc)
    }

    /// Formats as RFC 3339 / ISO 8601 with second precision and numeric offset
    /// (e.g. `2026-01-22T10:39:23+00:00`). Prefer this for Mollie string fields.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::DateTime;
    ///
    /// let dt = DateTime::parse("2026-01-22T10:39:23Z")?;
    /// assert_eq!(dt.to_rfc3339(), "2026-01-22T10:39:23+00:00");
    /// # Ok::<(), mollie_rs::MollieError>(())
    /// ```
    pub fn to_rfc3339(&self) -> String {
        self.0.to_rfc3339_opts(SecondsFormat::Secs, false)
    }

    /// Formats as RFC 3339, preserving sub-second precision when present.
    pub fn to_rfc3339_opts(&self, secform: SecondsFormat, use_z: bool) -> String {
        self.0.to_rfc3339_opts(secform, use_z)
    }

    /// Returns the calendar date (UTC date of the instant).
    pub fn date_utc(self) -> Date {
        Date(self.to_utc().date_naive())
    }
}

impl FromStr for DateTime {
    type Err = MollieError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl fmt::Display for DateTime {
    /// Formats as RFC 3339 with second precision and numeric offset.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_rfc3339())
    }
}

impl From<ChronoDateTime<FixedOffset>> for DateTime {
    fn from(value: ChronoDateTime<FixedOffset>) -> Self {
        Self(value)
    }
}

impl From<ChronoDateTime<Utc>> for DateTime {
    fn from(value: ChronoDateTime<Utc>) -> Self {
        Self::from_utc(value)
    }
}

impl From<DateTime> for ChronoDateTime<FixedOffset> {
    fn from(value: DateTime) -> Self {
        value.into_chrono()
    }
}

impl From<DateTime> for ChronoDateTime<Utc> {
    fn from(value: DateTime) -> Self {
        value.to_utc()
    }
}

impl From<DateTime> for String {
    fn from(value: DateTime) -> Self {
        value.to_rfc3339()
    }
}

impl TryFrom<&str> for DateTime {
    type Error = MollieError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl TryFrom<String> for DateTime {
    type Error = MollieError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

/// An ISO 8601 calendar date (`YYYY-MM-DD`) without a time component.
///
/// Used for Mollie fields such as `dueDate` that map to
/// [`chrono::NaiveDate`] in generated types.
///
/// # Examples
///
/// ```rust
/// use mollie_rs::Date;
///
/// let date = Date::parse("2026-07-13")?;
/// assert_eq!(date.to_string(), "2026-07-13");
/// assert!(Date::parse("13-07-2026").is_err());
/// # Ok::<(), mollie_rs::MollieError>(())
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Date(NaiveDate);

impl Date {
    /// Parses a calendar date in extended ISO 8601 form `YYYY-MM-DD`.
    ///
    /// # Errors
    ///
    /// Returns [`MollieError::InvalidRequest`] when the value is not a valid
    /// `YYYY-MM-DD` date.
    pub fn parse(value: impl AsRef<str>) -> MollieResult<Self> {
        let value = value.as_ref().trim();
        if value.is_empty() {
            return Err(MollieError::invalid_request(
                "date cannot be empty; use ISO 8601 calendar form `YYYY-MM-DD`",
            ));
        }
        NaiveDate::parse_from_str(value, "%Y-%m-%d")
            .map(Self)
            .map_err(|err| {
                MollieError::invalid_request(format!(
                    "invalid ISO 8601 date `{value}`: {err} (expected `YYYY-MM-DD`)"
                ))
            })
    }

    /// Returns true when `value` is a valid `YYYY-MM-DD` date.
    pub fn is_valid(value: impl AsRef<str>) -> bool {
        Self::parse(value).is_ok()
    }

    /// Builds from year, month, and day.
    ///
    /// # Errors
    ///
    /// Returns [`MollieError::InvalidRequest`] when the combination is not a
    /// real calendar date.
    pub fn from_ymd(year: i32, month: u32, day: u32) -> MollieResult<Self> {
        NaiveDate::from_ymd_opt(year, month, day)
            .map(Self)
            .ok_or_else(|| {
                MollieError::invalid_request(format!(
                    "invalid calendar date {year:04}-{month:02}-{day:02}"
                ))
            })
    }

    /// Builds from a [`chrono::NaiveDate`].
    pub const fn from_naive(date: NaiveDate) -> Self {
        Self(date)
    }

    /// Returns the inner [`chrono::NaiveDate`].
    pub const fn as_naive(self) -> NaiveDate {
        self.0
    }

    /// Year component.
    pub fn year(self) -> i32 {
        self.0.year()
    }

    /// Month component (`1..=12`).
    pub fn month(self) -> u32 {
        self.0.month()
    }

    /// Day component (`1..=31`).
    pub fn day(self) -> u32 {
        self.0.day()
    }
}

impl FromStr for Date {
    type Err = MollieError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%d"))
    }
}

impl From<NaiveDate> for Date {
    fn from(value: NaiveDate) -> Self {
        Self(value)
    }
}

impl From<Date> for NaiveDate {
    fn from(value: Date) -> Self {
        value.as_naive()
    }
}

impl From<Date> for String {
    fn from(value: Date) -> Self {
        value.to_string()
    }
}

impl TryFrom<&str> for Date {
    type Error = MollieError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl TryFrom<String> for Date {
    type Error = MollieError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{Date, DateTime};
    use chrono::{FixedOffset, TimeZone, Utc};

    #[test]
    fn parses_mollie_style_timestamps() {
        let dt = DateTime::parse("2026-01-22T10:39:23+00:00").unwrap();
        assert_eq!(dt.to_rfc3339(), "2026-01-22T10:39:23+00:00");

        let z = DateTime::parse("2026-01-22T10:39:23Z").unwrap();
        assert_eq!(z.to_rfc3339(), "2026-01-22T10:39:23+00:00");

        let offset = DateTime::parse("2026-01-22T12:39:23+02:00").unwrap();
        assert_eq!(
            offset
                .to_utc()
                .to_rfc3339_opts(chrono::SecondsFormat::Secs, false),
            "2026-01-22T10:39:23+00:00"
        );
    }

    #[test]
    fn rejects_naive_and_invalid() {
        assert!(DateTime::parse("").is_err());
        assert!(DateTime::parse("2026-01-22T10:39:23").is_err());
        assert!(DateTime::parse("2026-01-22").is_err());
        assert!(DateTime::parse("not-a-date").is_err());
        assert!(!DateTime::is_valid("10:39:23"));
    }

    #[test]
    fn from_utc_and_display() {
        let utc = Utc.with_ymd_and_hms(2026, 7, 13, 12, 0, 0).unwrap();
        let dt = DateTime::from_utc(utc);
        assert_eq!(dt.to_rfc3339(), "2026-07-13T12:00:00+00:00");
        assert_eq!(dt.to_string(), "2026-07-13T12:00:00+00:00");

        let fixed = FixedOffset::east_opt(3600)
            .unwrap()
            .with_ymd_and_hms(2026, 1, 1, 0, 0, 0)
            .unwrap();
        let dt = DateTime::from(fixed);
        assert!(dt.to_rfc3339().ends_with("+01:00"));
    }

    #[test]
    fn date_parses_yyyy_mm_dd() {
        let date = Date::parse("2026-07-13").unwrap();
        assert_eq!(date.to_string(), "2026-07-13");
        assert_eq!(date.year(), 2026);
        assert_eq!(date.month(), 7);
        assert_eq!(date.day(), 13);
        assert_eq!(Date::from_ymd(2026, 7, 13).unwrap(), date);

        assert!(Date::parse("2026/07/13").is_err());
        assert!(Date::parse("13-07-2026").is_err());
        assert!(Date::parse("2026-02-30").is_err());
    }

    #[test]
    fn datetime_date_utc_truncates_to_calendar_date() {
        let dt = DateTime::parse("2026-01-22T23:30:00-05:00").unwrap();
        // 23:30 -05:00 → 04:30 UTC next day
        assert_eq!(dt.date_utc().to_string(), "2026-01-23");
    }
}
