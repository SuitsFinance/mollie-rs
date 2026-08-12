# mollieClient

## Summary
`MollieClient` is the recommended application-facing client that configures HTTP transport and authentication while preserving access to every generated typed route method.

## Symbol
- Name: `MollieClient`
- Kind: `struct`
- Owner: `mollie_rs::client`

## Signature
```rust
pub struct MollieClient
```

## Location
- `src/client.rs`

## Inputs
- Construct with `MollieClient::from_api_key(api_key)`, `MollieClient::from_oauth_access_token(token)`, `MollieClient::from_env()`, `MollieClient::builder()`, or `MollieClient::from_generated(client)`.
- `api_key` and `token` must parse into validated `ApiKey` or `OAuthAccessToken` values when built through the facade helpers.
- `from_env` loads `.env` when present, then reads `MOLLIE_API_KEY` or `MOLLIE_OAUTH_ACCESS_TOKEN`. Optional `MOLLIE_BASE_URL` overrides the default base URL.
- The builder defaults to `https://api.mollie.com/v2`.

## Returns
- Dereferences to the generated `Client`.
- `raw()` returns `&Client`.
- `into_raw()` returns the owned generated `Client`.

## Errors
- `from_api_key`, `from_oauth_access_token`, `from_env`, and `builder().build()` return `MollieError` for invalid configuration, invalid headers, or HTTP client construction failures.

## Preconditions
- Callers need a valid Mollie API key or OAuth access token for authenticated requests.
- Network route calls must run inside an async runtime.

## Side Effects
- Construction creates a `reqwest::Client`.
- Route calls send HTTP requests to the configured Mollie base URL.

## Guarantees
- The facade does not hide generated route methods.
- Built clients include a bearer `Authorization` header.
- Built clients use a TLS-capable `reqwest` client for the default HTTPS base URL.
- Generated route request and response bodies remain typed from the OpenAPI spec.

## Examples
```rust,no_run
use mollie_rs::MollieClient;

# fn main() -> Result<(), mollie_rs::MollieError> {
let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;
let _generated = client.raw();
# Ok(())
# }
```

## Source of Truth
- Implementation: `src/client.rs`
- Public exports: `src/lib.rs`
- Route coverage: `docs/route-coverage.md`
