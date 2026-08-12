# fromEnv

## Summary
`MollieClient::from_env` loads `.env` when present and builds an authenticated client from environment variables.

## Symbol
- Name: `from_env`
- Kind: `associated function`
- Owner: `MollieClient`

## Signature
```rust
pub fn from_env() -> MollieResult<MollieClient>
```

## Location
- `src/client.rs`

## Inputs
- Loads `.env` from the current working directory via `load_dotenv` (missing file is success; existing process vars are not overwritten).
- `MOLLIE_API_KEY` (preferred) or `MOLLIE_OAUTH_ACCESS_TOKEN`.
- Optional `MOLLIE_BASE_URL` overrides the default Mollie API base URL.

## Returns
- `Ok(MollieClient)` configured with bearer auth and default HTTP transport.

## Errors
- Returns `MollieError::InvalidConfiguration` when dotenv fails, neither credential variable is set, a value fails validation, or client construction fails.
- Propagates header / HTTP client construction errors from the builder.

## Related
- Credential resolution lives in `Credential::from_env` (API key, then OAuth).
- Missing both credentials uses `MollieError::missing_mollie_credentials()`.
- `ApiKey::from_env` and `OAuthAccessToken::from_env` also call `load_dotenv` before reading their variable.

## Source of Truth
- Implementation: `src/client.rs`
- Env helpers: `src/env.rs`
