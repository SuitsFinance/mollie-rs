# money

## Summary
`Currency`, `AmountValue`, and `Money` validate Mollie amount payloads before converting into generated amount types. `ApplicationFee` validates Connect fee payloads on top of `Money`.

## Symbols
- `Currency`
- `AmountValue`
- `Money`
- `ApplicationFee` / `ApplicationFeeDescription` — see [`applicationFee.md`](./applicationFee.md)

## Location
- `src/money.rs`

## Inputs
- `Currency::parse(code)` accepts a currency code supported by the checked-in Mollie spec.
- `AmountValue::parse(currency, value)` accepts a plain decimal amount string for that currency.
- `Money::new(currency, value)` accepts both and validates the pair.
- `Money::try_from(types::Amount)` re-validates wire amounts from responses (balances, settlements, etc.).

## Supported Currencies
`EUR`, `GBP`, `CHF`, `DKK`, `NOK`, `PLN`, `SEK`, `USD`, `CZK`, `HUF`, `AUD`, and `CAD`.

## Guarantees
- Unknown currencies return `MollieError::InvalidRequest`.
- Values must have exactly two decimal places for the supported currency set.
- Values must not include signs, leading zeroes, missing decimals, or non-digit characters.
- `Money` converts into generated `types::Amount`, `types::AmountNullableInner`, `types::AmountNullable`, and `Option<types::Amount>`.
- `Money` can re-validate those wire types via `TryFrom` for response-side amounts.

## Mollie usage
| Use case | Typical wire type | Facade |
| --- | --- | --- |
| Payment / refund / capture amount | `types::Amount` | `Money` → `Amount` |
| Payment-link amount | `types::AmountNullable` | `Option<Money>` → `AmountNullable` |
| Balance / settlement amounts | `types::Amount` | `Money::try_from(amount)` |
| Application fee amount | nested `applicationFee` | [`ApplicationFee`](./applicationFee.md) |

## Examples
```rust
use mollie_rs::{types, ApplicationFee, Money};

# fn main() -> Result<(), mollie_rs::MollieError> {
// Payment / refund / capture / settlement-style amount
let amount: types::Amount = Money::new("EUR", "10.00")?.into();
assert_eq!(amount.currency, "EUR");
assert_eq!(amount.value, "10.00");

// Application fee on a payment
let fee = ApplicationFee::parse("EUR", "1.00", "Platform fee")?;
let _: types::CreatePaymentRequestApplicationFee = fee.into();

// Re-validate a balance amount from the API
let money = Money::try_from(amount)?;
assert_eq!(money.currency().code(), "EUR");
# Ok(())
# }
```

## Source of Truth
- Implementation: `src/money.rs`
- Generated wire type: `src/types.rs` `types::Amount`
