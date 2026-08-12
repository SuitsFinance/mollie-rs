# fromApiKey

## Summary
`MollieClient::from_api_key` creates an authenticated Mollie client without requiring callers to build a `reqwest::Client`.

## Symbol
- Name: `from_api_key`
- Kind: `associated function`
- Owner: `MollieClient`

## Signature
```rust
pub fn from_api_key(api_key: impl Into<String>) -> MollieResult<MollieClient>
```

## Location
- `src/client.rs`

## Inputs
- `api_key`: Mollie API key, required, non-blank, no leading or trailing whitespace, and no control characters.

## Returns
- `Ok(MollieClient)` with default base URL, authorization header, timeouts, user agent, and HTTPS transport support.

## Errors
- Returns `MollieError::InvalidConfiguration` when `api_key` is blank.
- Returns `MollieError::InvalidConfiguration` when `api_key` includes leading or trailing whitespace or control characters.
- Returns `MollieError::Communication` if `reqwest` cannot build the HTTP client.
- Returns `MollieError::InvalidHeaderValue` if the authorization header cannot be encoded.

## Preconditions
- The API key should be valid for the Mollie account and mode the caller wants to access.

## Side Effects
- Builds a reusable `reqwest::Client`.

## Guarantees
- Uses bearer authentication.
- Does not send a network request during construction.
- API keys are represented by the typed `ApiKey` wrapper internally and are redacted in debug output.

## Examples
```rust,no_run
use mollie_rs::MollieClient;

# fn main() -> Result<(), mollie_rs::MollieError> {
let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;
let _ = client.raw();
# Ok(())
# }
```

## Source of Truth
- Implementation: `src/client.rs`
- Tests: `src/client.rs`, `src/auth.rs`
