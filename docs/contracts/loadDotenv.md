# loadDotenv

## Summary
`load_dotenv` loads variables from a `.env` file in the current directory into the process environment via `dotenvy`. A missing file is success.

## Symbol
- Name: `load_dotenv`
- Kind: `function`
- Owner: `mollie_rs::env`

## Signature
```rust
pub fn load_dotenv() -> MollieResult<()>
```

## Location
- `src/env.rs`

## Inputs
- None. Reads `.env` from the process current working directory.

## Returns
- `Ok(())` when no file exists, or when the file is loaded.
- Existing process variables are not overwritten (`dotenvy` default).

## Errors
- Returns `MollieError::InvalidConfiguration` when `.env` exists but cannot be read or parsed.

## Side Effects
- May insert environment variables into the process environment.

## Related
- `load_dotenv_from(path)` loads a specific path and errors if the file is missing.
- `ApiKey::from_env`, `OAuthAccessToken::from_env`, and `MollieClient::from_env` call `load_dotenv` internally; most apps do not need to call this helper directly.

## Source of Truth
- Implementation: `src/env.rs`
- Public exports: `src/lib.rs`
