# amountValue

## Summary
`AmountValue` validates the string value part of a Mollie amount for a known currency.

## Symbol
- Name: `AmountValue`
- Kind: `struct`
- Owner: `mollie_rs::money`

## Signature
```rust
pub struct AmountValue(String)
```

## Location
- `src/money.rs`

## Inputs
- `AmountValue::parse(currency, value)` accepts a validated `Currency` and a value string.

## Returns
- `Ok(AmountValue)` when the value matches the currency scale.
- `as_str()` returns the validated value as `&str`.
- `into_string()` returns the owned validated value.

## Errors
- Returns `MollieError::InvalidRequest` for empty values, signed values, missing decimals, leading zeroes, non-digit characters, or a fractional scale different from the currency's minor units.

## Preconditions
- The provided `Currency` must already be valid.

## Side Effects
- None.

## Guarantees
- The current supported currency set requires exactly two decimal places.
- The value is stored unchanged after validation.

## Examples
```rust
use mollie_rs::{AmountValue, Currency};

# fn main() -> Result<(), mollie_rs::MollieError> {
let value = AmountValue::parse(Currency::parse("EUR")?, "10.00")?;
assert_eq!(value.as_str(), "10.00");
# Ok(())
# }
```

## Source of Truth
- Implementation: `src/money.rs`
