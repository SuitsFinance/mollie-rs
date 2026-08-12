# ISO 3166-1 alpha-2 (country codes)

## Summary

ISO 3166-1 alpha-2 defines two-letter country codes such as `NL`, `DE`, and `US`. Mollie uses this form for address `country`, `billingCountry`, card country metadata, and related fields.

This crate’s table has **three columns** for each entry:

| Column | API | Content |
| --- | --- | --- |
| **Entry** | `as_str()` / `Display` | ISO 3166-1 alpha-2 code (`NL`) |
| **Country name** | `name()` | English short name officially used by ISO 3166/MA (title case) |
| **Subdivisions** | `subdivisions()` | Number/category of subdivisions assigned codes in **ISO 3166-2** (summary only; `None` if none) |

## Crate mapping

| Item | Location |
| --- | --- |
| Application enum | `mollie_rs::CountryCode` (`src/country_code.rs`) |
| Contract | [`docs/contracts/countryCode.md`](../contracts/countryCode.md) |
| Generator | `scripts/generate_country_code.py` |

## Wire form

- Exactly **two** ASCII letters
- **Uppercase** only (`NL`, not `nl`)
- Not ISO 3166-1 alpha-3 (`NLD`) and not numeric (`528`)

## Validation in this crate

```rust
use mollie_rs::CountryCode;

let nl = CountryCode::parse("NL")?;
assert_eq!(nl.as_str(), "NL");
assert_eq!(nl.name(), "Netherlands, Kingdom of the");
assert!(nl.subdivisions().is_some());

// Format-only vs assigned code
assert!(CountryCode::is_valid_format("XX"));
assert!(!CountryCode::is_valid("XX"));
# Ok::<(), mollie_rs::MollieError>(())
```

`CountryCode::ALL` lists every code recognized by the crate (**249** currently assigned entries plus historically assigned **`AN`** for Netherlands Antilles). Unknown codes fail at parse time with `MollieError::InvalidRequest`.

## Examples of official short names

| Code | Name (ISO 3166/MA) | Subdivisions (summary) |
| --- | --- | --- |
| `AD` | Andorra | 7 parishes |
| `BO` | Bolivia, Plurinational State of | departments |
| `GB` | United Kingdom of Great Britain and Northern Ireland | countries, counties, … |
| `NL` | Netherlands, Kingdom of the | provinces, countries, special municipalities |
| `US` | United States of America | states, 1 district, 6 outlying areas |
| `AQ` | Antarctica | *(none)* |

Full list is encoded in `CountryCode` / the generator script.

## Mollie usage examples

- Payment / customer address `country`
- Methods list query `billingCountry` (`list_methods(..., billing_country: Some(CountryCode::DE.as_str()), ...)`)
- Payment method restriction / eligibility by country where documented

Generated OpenAPI types often keep these fields as `String`; prefer constructing with `CountryCode` then passing `.as_str()` or `String::from(code)`.

## Related standards

- **ISO 3166-1 alpha-3** — three-letter codes (`NLD`); not used by Mollie country fields
- **ISO 3166-2** — country subdivision codes (`NL-NH`); see [iso-3166-1-alpha-1.md](./iso-3166-1-alpha-1.md)
- **ISO 15897 locales** — `language_TERRITORY` where territory is often alpha-2 (`nl_NL`); see [iso-15897.md](./iso-15897.md)

## External reference

- [ISO 3166 country codes](https://www.iso.org/iso-3166-country-codes.html)
- Mollie API docs for address and method parameters (billing country)
