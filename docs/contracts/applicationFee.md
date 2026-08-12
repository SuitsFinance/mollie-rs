# applicationFee

## Summary
`ApplicationFee` and `ApplicationFeeDescription` validate Mollie Connect application-fee payloads (`amount` + `description`) before converting into generated request types.

## Symbols
- `ApplicationFeeDescription` — non-empty description, max 255 characters
- `ApplicationFee` — validated [`Money`](./money.md) + description
- `APPLICATION_FEE_DESCRIPTION_MAX_LEN` — `255`
- Owner: `mollie_rs::money`

## Location
- `src/money.rs`

## Inputs
- `ApplicationFeeDescription::parse(description)` — non-empty, ≤ 255 Unicode scalar values
- `ApplicationFee::new(amount, description)` — existing [`Money`](./money.md) + description
- `ApplicationFee::parse(currency, value, description)` — constructs `Money` and description together

## Returns
- `into_payment_request_fee()` → `types::CreatePaymentRequestApplicationFee`
- `into_payment_link_fee()` → `types::CreatePaymentLinkBodyApplicationFee`
- `into_subscription_request_fee()` → `types::CreateSubscriptionRequestApplicationFee`
- `From` conversions also cover entity payment / payment-link / subscription / response application-fee types

## Errors
- Empty or over-long description → `MollieError::InvalidRequest`
- Invalid currency/value via [`Money`](./money.md) → `MollieError::InvalidRequest`

## Preconditions
- Mollie Connect OAuth: fee is deducted from the connected merchant balance and credited to the platform when the payment succeeds.
- Currency must be in the supported OpenAPI set (see [`currency.md`](./currency.md)).

## Examples
```rust
use mollie_rs::{types::CreatePaymentRequest, ApplicationFee, Money};

# fn main() -> Result<(), mollie_rs::MollieError> {
let fee = ApplicationFee::new(Money::new("EUR", "1.00")?, "Platform fee")?;
let mut body = PaymentRequest::default();
body.application_fee = Some(fee.into());
# let _ = body;
# Ok(())
# }
```

## Source of Truth
- Implementation: `src/money.rs`
- Generated wire types: `src/types.rs` `*ApplicationFee*`
