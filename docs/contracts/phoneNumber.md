# phoneNumber

## Summary
`PhoneNumber` validates E.164 phone number strings for Mollie request fields.

## Symbol
- Name: `PhoneNumber`
- Kind: `struct`
- Owner: `mollie_rs::phone_number`

## Signature
```rust
pub struct PhoneNumber(String)
```

## Location
- `src/phone_number.rs`
- Format notes: `docs/e.164.md`

## Inputs
- `PhoneNumber::parse(value)` accepts a full E.164 string including `+`.
- `TryFrom<&str>` / `TryFrom<String>` / `FromStr` delegate to `parse`.

## Returns
- `as_str()` / `Display` — E.164 string (`+31208202070`)
- `into_string()` / `From<PhoneNumber> for String` — owned wire value
- `digits()` — digits without `+`
- `is_valid(value)` — boolean check

## Errors
- `parse` returns `MollieError::InvalidRequest` when the value is not E.164
  (missing `+`, spaces/punctuation, leading `0` after `+`, empty, or more than 15 digits).

## Preconditions
- Mollie documents: all phone numbers must be passed as strings in E.164 format.

## Side Effects
- None.

## Guarantees
- A successfully parsed value always starts with `+` followed by 1–15 digits, first digit `1`–`9`.

## Examples
```rust
use mollie_rs::PhoneNumber;

# fn main() -> Result<(), mollie_rs::MollieError> {
let phone = PhoneNumber::parse("+31208202070")?;
assert_eq!(phone.as_str(), "+31208202070");
let field: String = phone.into();
assert_eq!(field, "+31208202070");
# Ok(())
# }
```

## Source of Truth
- Implementation: `src/phone_number.rs`
- Docs: `docs/e.164.md`
