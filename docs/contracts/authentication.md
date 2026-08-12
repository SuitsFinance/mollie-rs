# Authentication contracts

`MollieClient` supports the authentication forms used by Mollie’s API:

| Credential | Facade | Authorization header | Use |
| --- | --- | --- | --- |
| API key | `ApiKey` / `Credential::ApiKey` | `Bearer <key>` | Profile-scoped API calls |
| Advanced or app access token | `OAuthAccessToken` / `Credential::OAuthAccessToken` | `Bearer <token>` | Organization or connected-account API calls |
| OAuth client credentials | `BasicAuth` / `Credential::BasicAuth` | `Basic <base64(client_id:secret)>` | OAuth token generation and revocation |

```rust
use mollie_rs::{BasicAuth, Credential, MollieClient};

let basic = BasicAuth::new("client-id", "client-secret")?;
let credential = Credential::from(basic);
let client = MollieClient::builder().credential(credential).build()?;
# Ok::<(), mollie_rs::MollieError>(())
```

Basic Auth is intended for the OAuth token-management endpoints. Ordinary
resource requests should use an API key or access token.

## Environment variables

`Credential::from_env` checks credentials in this order:

1. `MOLLIE_API_KEY`
2. `MOLLIE_OAUTH_ACCESS_TOKEN`
3. The pair `MOLLIE_OAUTH_CLIENT_ID` and `MOLLIE_OAUTH_CLIENT_SECRET`

The Basic Auth variables must be supplied together. Secret-bearing types redact
their values in `Debug` output.

## Transport and test mode

The ergonomic client requires HTTPS for remote Mollie endpoints and configures
Rustls with TLS 1.2 as the minimum. HTTP is accepted only for loopback mocked
servers. API keys select test or live mode themselves; OAuth credentials can
use `with_testmode(true)` or the builder's `testmode(true)` for operations that
explicitly support the `testmode` query parameter.

Test-mode support is route-specific. Some operations support the sticky query,
some request bodies contain their own `testmode` field, and the two mechanisms
are independent. Balances, Settlements, and Invoices are live-only reporting
routes in the supplied Mollie documentation and reject configured sticky
`testmode` before HTTP. See [`test-mode.md`](test-mode.md).

Store credentials securely, rotate API keys by introducing the replacement
before revoking the old key, and never include secrets in logs or committed
configuration.