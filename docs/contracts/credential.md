# credential

## Summary
`Credential` is the authentication enum used by `MollieClientBuilder` to build the bearer `Authorization` header.

## Symbol
- Name: `Credential`
- Kind: `enum`
- Owner: `mollie_rs::auth`

## Signature
```rust
pub enum Credential {
    ApiKey(ApiKey),
    OAuthAccessToken(OAuthAccessToken),
}
```

## Location
- `src/auth.rs`

## Inputs
- `Credential::api_key(api_key)` validates and wraps an API key.
- `Credential::oauth_access_token(token)` validates and wraps an OAuth access token.
- `Credential::from_api_key(api_key)` accepts an already validated `ApiKey`.
- `Credential::from_oauth_access_token(token)` accepts an already validated `OAuthAccessToken`.

## Returns
- `scheme()` returns `Bearer`.
- `secret()` returns the raw credential secret.
- `authorization_value()` returns `Bearer <secret>`.
- `is_blank()` reports whether the contained secret is blank; normal constructors reject blank values.

## Errors
- The validating constructors return `MollieError::InvalidConfiguration` for invalid secret shape.

## Preconditions
- Callers should prefer validated constructors unless they already hold an `ApiKey` or `OAuthAccessToken`.

## Side Effects
- None.

## Guarantees
- Both credential variants use bearer authentication.
- The enum can be passed directly to `MollieClientBuilder::credential`.

## Examples
```rust
use mollie_rs::Credential;

# fn main() -> Result<(), mollie_rs::MollieError> {
let credential = Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;
assert!(credential.authorization_value().starts_with("Bearer "));
# Ok(())
# }
```

## Source of Truth
- Implementation: `src/auth.rs`
- Client usage: `src/client.rs`
