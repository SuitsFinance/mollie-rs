# currency

## Summary
`Currency` validates currency codes against the currencies supported by the checked-in Mollie OpenAPI spec.

## Symbol
- Name: `Currency`
- Kind: `struct`
- Owner: `mollie_rs::money`

## Signature
```rust
pub struct Currency(types::Currencies)
```

## Location
- `src/money.rs`

## Inputs
- `Currency::parse(code)` accepts a currency code string.
- `TryFrom<&str>` and `TryFrom<String>` delegate to `Currency::parse`.
- `From<types::Currencies>` wraps the generated currency enum.

## Returns
- `code()` returns the ISO 4217 code.
- `minor_units()` returns `2` for the currently supported currency set.
- `into_generated()` returns `types::Currencies`.
- `is_supported(code)` returns whether parsing would succeed.

## Errors
- `Currency::parse` returns `MollieError::InvalidRequest` for unsupported currency codes.

## Preconditions
- The supported set is intentionally tied to the checked-in spec: `EUR`, `GBP`, `CHF`, `DKK`, `NOK`, `PLN`, `SEK`, `USD`, `CZK`, `HUF`, `AUD`, and `CAD`.

## Side Effects
- None.

## Guarantees
- `Currency::SUPPORTED` lists the full supported set in code.
- `Display` prints the ISO 4217 code.
- Used by [`Money`](./money.md) for payment, payment-link, refund, capture, balance, settlement, and [`ApplicationFee`](./applicationFee.md) amounts.

## Examples
```rust
use mollie_rs::Currency;

# fn main() -> Result<(), mollie_rs::MollieError> {
let currency = Currency::parse("EUR")?;
assert_eq!(currency.code(), "EUR");
assert_eq!(currency.minor_units(), 2);
# Ok(())
# }
```

## Source of Truth
- Implementation: `src/money.rs`
- Generated enum: `src/types.rs` `types::Currencies`
