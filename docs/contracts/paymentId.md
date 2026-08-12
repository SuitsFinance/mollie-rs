# paymentId

## Summary
`PaymentId` validates Mollie payment resource ids (`tr_…`) before they are passed into route methods.

## Symbol
- Name: `PaymentId`
- Kind: `struct`
- Owner: `mollie_rs::ids`

## Signature
```rust
pub struct PaymentId(String)
```

## Location
- `src/ids.rs`

## Inputs
- `PaymentId::parse(value)` accepts strings with prefix `tr_` and a non-empty suffix.

## Returns
- `as_str()` / `Display` — raw id
- `into_token()` / `From` → `types::PaymentToken`
- `is_valid(value)` — boolean check

## Errors
- `MollieError::InvalidRequest` when empty, wrong prefix (e.g. `pfl_…`), empty suffix, or illegal characters.

## Guarantees
- Rejects profile ids (`pfl_`) so they cannot be passed as payment path params by mistake.

## Examples
```rust
use mollie_rs::PaymentId;

# fn main() -> Result<(), mollie_rs::MollieError> {
let id = PaymentId::parse("tr_WDqYK6vllg")?;
assert!(PaymentId::parse("pfl_QkEhN94Ba").is_err());
# let _ = id;
# Ok(())
# }
```
