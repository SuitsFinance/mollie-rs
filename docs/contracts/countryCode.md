# countryCode

## Summary
`CountryCode` validates ISO 3166-1 alpha-2 country codes and exposes ISO 3166/MA English short names plus ISO 3166-2 subdivision summaries.

## Symbol
- Name: `CountryCode`
- Kind: `enum`
- Owner: `mollie_rs::country_code`

## Signature
```rust
pub enum CountryCode {
    Ad, Ae, /* … 250 variants … */ Zw,
}
```

## Location
- `src/country_code.rs`
- Generator: `scripts/generate_country_code.py`
- ISO notes: `docs/iso/iso-3166-1-alpha-2.md`

## Columns

| Method | Column |
| --- | --- |
| `as_str()` | Entry (alpha-2), e.g. `"NL"` |
| `name()` | Country name (ISO 3166/MA English short name) |
| `subdivisions()` | ISO 3166-2 subdivision categories summary, or `None` |

## Inputs
- Named constants such as `CountryCode::NL`, `CountryCode::DE`.
- `CountryCode::parse(value)` accepts uppercase alpha-2 strings only.
- `TryFrom<&str>` / `TryFrom<String>` delegate to `parse`.

## Returns
- `as_str()` / `Display` → `"NL"`, `"DE"`, …
- `name()` → e.g. `"Netherlands, Kingdom of the"`
- `subdivisions()` → e.g. `Some("7 parishes")`
- `is_valid` / `is_valid_format`
- `ALL` — full recognized set (250)

## Errors
- `parse` returns `MollieError::InvalidRequest` for lowercase, alpha-3, empty, or unassigned codes.

## Preconditions
- Wire form is ISO 3166-1 **alpha-2**, not alpha-3 or ISO 3166-2 subdivisions.

## Examples
```rust
use mollie_rs::CountryCode;

# fn main() -> Result<(), mollie_rs::MollieError> {
let nl = CountryCode::parse("NL")?;
assert_eq!(nl.as_str(), "NL");
assert_eq!(nl.name(), "Netherlands, Kingdom of the");
assert!(nl.subdivisions().is_some());
# Ok(())
# }
```

## Source of Truth
- Implementation: `src/country_code.rs` (generated)
- Data: `scripts/generate_country_code.py` (`COUNTRIES` table)
- Docs: `docs/iso/iso-3166-1-alpha-2.md`
