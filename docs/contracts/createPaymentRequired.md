# createPaymentRequired

## Summary
Validates the three create-payment body fields that are required for a normal hosted payment: `description`, `amount`, and `redirectUrl`.

## Symbols
- `PaymentDescription` — description string (required, max 255 chars)
- `RedirectUrl` — absolute `http`/`https` URL
- `CreatePaymentRequired` — combination of description + [`Money`](./money.md) + optional redirect
- Owner: `mollie_rs::create_payment`

## Location
- `src/create_payment.rs`

## Inputs
- `PaymentDescription::parse` — non-empty, ≤ 255 Unicode scalar values
- `Money::new` / existing amount validation for `currency` + `value`
- `RedirectUrl::parse` — absolute `http` or `https` URL
- `CreatePaymentRequired::new(description, amount, redirect_url)` — standard payment (redirect required)
- `CreatePaymentRequired::new_recurring(description, amount)` — omits redirect for recurring
- `CreatePaymentRequired::new_with_apple_pay_token(...)` — redirect optional when Apple Pay token is present

## Returns
- `into_payment_request()` → `types::CreatePaymentRequest` with the three fields set
- `apply(&mut PaymentRequest)` mutates an existing body
- `PaymentDescription` → `types::CreatePaymentRequestDescription`

## Errors
- Empty or over-long description
- Invalid amount/currency (via `Money`)
- Missing/invalid redirect URL on the standard path
- Empty Apple Pay token when used to skip redirect

## Preconditions
- Mollie: description required, max 255; amount required; redirectUrl normally required except recurring / Apple Pay token flows.

## Examples
```rust
use mollie_rs::{CreatePaymentRequired, Money, PaymentId};

# fn main() -> Result<(), mollie_rs::MollieError> {
let body = CreatePaymentRequired::new(
    "Order #12345",
    Money::new("EUR", "10.00")?,
    "https://example.com/return",
)?
.into_payment_request();

// Path params still use typed ids:
let payment_id = PaymentId::parse("tr_WDqYK6vllg")?;
# let _ = (body, payment_id);
# Ok(())
# }
```
