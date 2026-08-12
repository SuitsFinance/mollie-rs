# profileId

## Summary
`ProfileId` validates Mollie profile resource ids (`pfl_…`) before they are passed into route methods.

## Symbol
- Name: `ProfileId`
- Kind: `struct`
- Owner: `mollie_rs::ids`

## Signature
```rust
pub struct ProfileId(String)
```

## Location
- `src/ids.rs`

## Inputs
- `ProfileId::parse(value)` accepts strings with prefix `pfl_` and a non-empty suffix.

## Returns
- `as_str()` / `Display` — raw id
- `into_token()` / `From` → `types::ProfileToken`
- `is_valid(value)` — boolean check

## Errors
- `MollieError::InvalidRequest` when empty, wrong prefix (e.g. `tr_…`), empty suffix, or illegal characters.

## Guarantees
- Rejects payment ids (`tr_`) so they cannot be passed as profile path params by mistake.

## Examples
```rust
use mollie_rs::ProfileId;

# fn main() -> Result<(), mollie_rs::MollieError> {
let id = ProfileId::parse("pfl_QkEhN94Ba")?;
assert!(ProfileId::parse("tr_WDqYK6vllg").is_err());
# let _ = id;
# Ok(())
# }
```
