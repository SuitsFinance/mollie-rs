# oauthAccessToken

## Summary
`OAuthAccessToken` is a validated wrapper for Mollie OAuth access tokens used by the SDK facade.

## Symbol
- Name: `OAuthAccessToken`
- Kind: `struct`
- Owner: `mollie_rs::auth`

## Signature
```rust
pub struct OAuthAccessToken(String)
```

## Location
- `src/auth.rs`

## Inputs
- `OAuthAccessToken::new(token)` accepts an owned or convertible string.
- `OAuthAccessToken::from_env()` loads `.env` when present, then reads `MOLLIE_OAUTH_ACCESS_TOKEN`.
- `TryFrom<&str>` and `TryFrom<String>` delegate to `OAuthAccessToken::new`.

## Returns
- `Ok(OAuthAccessToken)` when the token is non-blank and header-safe.
- `as_str()` returns the raw token as `&str`.

## Errors
- Returns `MollieError::InvalidConfiguration` for blank values, leading or trailing whitespace, control characters, or a missing / non-UTF-8 `MOLLIE_OAUTH_ACCESS_TOKEN`.

## Preconditions
- The SDK only validates local shape. It does not verify token validity with Mollie during construction.

## Side Effects
- None.

## Guarantees
- `Debug` output is redacted.
- `AsRef<str>` returns the raw token for internal header construction.

## Examples
```rust
use mollie_rs::OAuthAccessToken;

# fn main() -> Result<(), mollie_rs::MollieError> {
let token = OAuthAccessToken::new("access-token")?;
assert_eq!(token.as_str(), "access-token");
# Ok(())
# }
```

## Source of Truth
- Implementation: `src/auth.rs`
