# paymentMethod

## Summary
`PaymentMethod` validates Mollie payment method identifiers used on request bodies (payment `method`, payment-link `allowedMethods`, and other method-selection fields).

## Symbol
- Name: `PaymentMethod`
- Kind: `struct`
- Owner: `mollie_rs::payment_method`

## Signature
```rust
pub struct PaymentMethod(types::MethodInner)
```

## Location
- `src/payment_method.rs`

## Inputs
- Named constants such as `PaymentMethod::IDEAL` and `PaymentMethod::CREDITCARD`.
- `PaymentMethod::parse(value)` accepts a lowercase Mollie method string.
- `TryFrom<&str>` and `TryFrom<String>` delegate to `PaymentMethod::parse`.
- `From<types::MethodInner>` wraps the generated request-method enum.

## Returns
- `as_str()` returns the wire identifier (`"ideal"`, `"creditcard"`, …).
- `into_generated()` returns `types::MethodInner`.
- `into_method()` / `From<PaymentMethod> for types::Method` produce the nullable generated wrapper used on `PaymentRequest.method`.
- `From<PaymentMethod> for Option<types::Method>` produces `Some(...)` for assignment into optional method fields.
- `payment_link_methods([...])` and `parse_payment_link_methods([...])` build validated `types::PaymentLinkMethods` for `CreatePaymentLinkBody.allowed_methods`.
- `is_supported(value)` returns whether parsing would succeed.
- `SUPPORTED` lists every accepted request method.

## Errors
- `PaymentMethod::parse` returns `MollieError::InvalidRequest` for identifiers outside the request-method set (for example `googlepay`, deprecated response-only values, or typos).

## Preconditions
- The supported set matches generated `types::MethodInner` from the checked-in OpenAPI spec used for payment create/update `method` (not the broader historical `types::PaymentMethodInner` response enum).

## Side Effects
- None.

## Guarantees
- Request construction can fail locally before HTTP when using `parse` / `parse_payment_link_methods`.
- `Display` prints the lowercase wire identifier.

## Examples
```rust
use mollie_rs::{types, PaymentMethod};

# fn main() -> Result<(), mollie_rs::MollieError> {
let method = PaymentMethod::parse("ideal")?;
let payment_method: types::Method = method.into();
assert_eq!(payment_method.0, Some(types::MethodInner::Ideal));

let allowed = PaymentMethod::payment_link_methods([
    PaymentMethod::IDEAL,
    PaymentMethod::BANCONTACT,
])?;
assert_eq!(
    allowed.0,
    Some(vec!["ideal".to_string(), "bancontact".to_string()])
);
assert!(PaymentMethod::parse("googlepay").is_err());
# Ok(())
# }
```

## Source of Truth
- Implementation: `src/payment_method.rs`
- Generated request enum: `src/types.rs` `types::MethodInner` / `types::Method`
- Generated payment-link field: `src/types.rs` `types::PaymentLinkMethods`
