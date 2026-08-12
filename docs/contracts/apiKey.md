# apiKey

## Summary
`ApiKey` is a validated wrapper for Mollie API keys used by the SDK facade.

## Symbol
- Name: `ApiKey`
- Kind: `struct`
- Owner: `mollie_rs::auth`

## Signature
```rust
pub struct ApiKey(String)
```

## Location
- `src/auth.rs`

## Inputs
- `ApiKey::new(api_key)` accepts an owned or convertible string.
- `ApiKey::from_env()` loads `.env` when present, then reads `MOLLIE_API_KEY`.
- `TryFrom<&str>` and `TryFrom<String>` delegate to `ApiKey::new`.

## Returns
- `Ok(ApiKey)` when the key is non-blank and header-safe.
- `as_str()` returns the raw key as `&str`.

## Errors
- Returns `MollieError::InvalidConfiguration` for blank values, leading or trailing whitespace, control characters, or a missing / non-UTF-8 `MOLLIE_API_KEY`.

## Preconditions
- The SDK only validates local shape. It does not verify the key with Mollie during construction.

## Side Effects
- None.

## Guarantees
- `Debug` output is redacted.
- `AsRef<str>` returns the raw key for internal header construction.

## Examples
```rust
use mollie_rs::ApiKey;

# fn main() -> Result<(), mollie_rs::MollieError> {
let key = ApiKey::new("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;
assert_eq!(key.as_str(), "test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
# Ok(())
# }
```

## Source of Truth
- Implementation: `src/auth.rs`
- Tests: `src/auth.rs`
