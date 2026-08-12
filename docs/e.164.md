# E.164 (phone numbers)

## Summary

[E.164](https://en.wikipedia.org/wiki/E.164) is the international public telecommunication numbering plan. Mollie requires phone numbers as **strings** in E.164 form.

**Example:** `+31208202070`

## Crate mapping

| Item | Location |
| --- | --- |
| Application type | `mollie_rs::PhoneNumber` (`src/phone_number.rs`) |
| Contract | [`docs/contracts/phoneNumber.md`](./contracts/phoneNumber.md) |
| Generated fields | plain `String` on address / profile types (e.g. `phone`) |

## Wire form

```text
+[country code][subscriber number]
```

| Rule | Detail |
| --- | --- |
| Leading `+` | Required |
| Digits after `+` | 1–15 total |
| First digit | `1`–`9` (country code must not start with `0`) |
| Characters | Digits only after `+` — **no** spaces, dashes, parentheses, or dots |

Valid:

- `+31208202070`
- `+12025550123`

Invalid:

- `0208202070` (national format, missing `+`)
- `31208202070` (missing `+`)
- `+31 20 820 2070` (spaces)
- `+31-20-8202070` (punctuation)
- `+031208202070` (leading zero after `+`)

## Validation in this crate

```rust
use mollie_rs::PhoneNumber;

let phone = PhoneNumber::parse("+31208202070")?;
assert_eq!(phone.as_str(), "+31208202070");

// Assign onto generated string fields:
// address.phone = Some(phone.into());
# Ok::<(), mollie_rs::MollieError>(())
```

`PhoneNumber::parse` returns `MollieError::InvalidRequest` for non-E.164 input so bad numbers fail locally instead of as API errors.

## Mollie usage examples

- Payment / customer **billing** and **shipping** address `phone`
- Any other documented `phone` field that requires E.164

## Related formats

| Format | Doc | Crate type |
| --- | --- | --- |
| Locale `xx_XX` | [iso/iso-15897.md](./iso/iso-15897.md) | `Locale` |
| Country `XX` | [iso/iso-3166-1-alpha-2.md](./iso/iso-3166-1-alpha-2.md) | `CountryCode` |
| Currency | [iso/iso-4217.md](./iso/iso-4217.md) | `Currency` / `Money` |
| Datetime | [iso/iso-8601.md](./iso/iso-8601.md) | string / `chrono` |

## External reference

- [E.164 (Wikipedia)](https://en.wikipedia.org/wiki/E.164)
- [ITU-T E.164](https://www.itu.int/rec/T-REC-E.164)
- Mollie address `phone` field documentation
