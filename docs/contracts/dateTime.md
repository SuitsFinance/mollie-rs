# dateTime

## Summary
`DateTime` and `Date` validate ISO 8601 timestamps and calendar dates for Mollie string / `NaiveDate` fields.

## Symbols
- `DateTime` — offset-aware instant (RFC 3339 / ISO 8601)
- `Date` — calendar date `YYYY-MM-DD`
- Owner: `mollie_rs::datetime`

## Signature
```rust
pub struct DateTime(chrono::DateTime<chrono::FixedOffset>);
pub struct Date(chrono::NaiveDate);
```

## Location
- `src/datetime.rs`
- ISO notes: `docs/iso/iso-8601.md`

## Inputs
- `DateTime::parse` — RFC 3339 strings with `Z` or numeric offset
- `DateTime::from_utc` / `from_chrono` / `now_utc`
- `Date::parse` — `YYYY-MM-DD`
- `Date::from_ymd` / `from_naive`

## Returns
- `DateTime::to_rfc3339()` → wire string (`2026-01-22T10:39:23+00:00`)
- `DateTime::to_utc` / `as_chrono` / `into_chrono`
- `Date::as_naive()` → `chrono::NaiveDate` for generated fields
- `Display` formats the canonical wire form

## Errors
- `MollieError::InvalidRequest` for empty values, naive datetimes without offset, invalid calendars, or non-ISO input

## Preconditions
- Mollie expects offset-aware instants on timestamp fields; do not send local-naive `YYYY-MM-DDThh:mm:ss` without zone.

## Examples
```rust
use mollie_rs::{Date, DateTime};

# fn main() -> Result<(), mollie_rs::MollieError> {
let expires = DateTime::parse("2026-07-13T12:00:00+00:00")?;
let field: String = expires.into();
assert_eq!(field, "2026-07-13T12:00:00+00:00");

let due = Date::parse("2026-07-13")?;
assert_eq!(due.to_string(), "2026-07-13");
# Ok(())
# }
```
