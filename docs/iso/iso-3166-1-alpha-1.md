# ISO 3166 naming note (and country subdivision codes)

## Summary

There is **no** “ISO 3166-1 alpha-1” code set. ISO 3166-1 defines:

| Form | Example | Used by Mollie country fields? |
| --- | --- | --- |
| **alpha-2** | `NL` | **Yes** — see [iso-3166-1-alpha-2.md](./iso-3166-1-alpha-2.md) and `CountryCode` |
| **alpha-3** | `NLD` | No |
| **numeric** | `528` | No |

**Country subdivision codes** are specified by a different part of the family:

| Standard | Example | Meaning |
| --- | --- | --- |
| **ISO 3166-2** | `NL-NH`, `US-CA`, `DE-BY` | Country + subdivision (province, state, region) |

## Country codes vs subdivision codes

```
NL          → ISO 3166-1 alpha-2 country (Netherlands)
NL-NH       → ISO 3166-2 subdivision (Noord-Holland)
```

Mollie address objects typically take **country** as ISO 3166-1 alpha-2. Subdivision / state / region fields, when present, are product-specific strings and are **not** validated by `CountryCode`.

## Crate mapping

| Need | Type / doc |
| --- | --- |
| Country (`NL`) | [`CountryCode`](../contracts/countryCode.md), [iso-3166-1-alpha-2.md](./iso-3166-1-alpha-2.md) |
| Locale region part (`nl_NL`) | [`Locale`](../contracts/locale.md), [iso-15897.md](./iso-15897.md) |
| Subdivision (`NL-NH`) | Not a dedicated crate enum today; pass through as plain string if an API field requires it |

## Validation guidance

- Prefer `CountryCode::parse("NL")` for country fields.
- Do **not** put a 3166-2 code (`NL-NH`) into a country field; Mollie expects alpha-2 only.
- For subdivision-like fields, validate format in application code if required (often `CC-…` where `CC` is alpha-2).

## Related

- [iso-3166-1-alpha-2.md](./iso-3166-1-alpha-2.md) — country codes and `CountryCode`
- [iso-15897.md](./iso-15897.md) — locales (`language_TERRITORY`)
- [ISO 3166 overview](https://www.iso.org/iso-3166-country-codes.html)
