# OAuth and Connect

## OAuth token endpoints

`client.oauth()` exposes token generate/revoke as **NonRetryableWrite**.

- Use the OAuth credential types; secrets are redacted in `Debug`.
- Do not pass an API key where client basic auth is required — the facade rejects the wrong credential class.
- Transport never auto-retries these calls.

## Connect balance transfers

`client.connect_balance_transfers()` is the Tier-S path for merchant-to-merchant balance movement.

- Build with `CreateConnectBalanceTransferRequired` (distinct `org_…` parties, amount, descriptions).
- Create is **IdempotentWrite** — bind a sticky `IdempotencyKey` for safe retries.
- `get` / `list_page` / `list_all` / streams are available for reconciliation.
- Isolate merchants with `with_credential` / `with_profile_id` under concurrency.

```rust,no_run
use mollie_rs::{CreateConnectBalanceTransferRequired, IdempotencyKey, Money};

# async fn example(client: mollie_rs::MollieClient) -> Result<(), mollie_rs::MollieError> {
let key = IdempotencyKey::new("platform-fee-991")?;
let required = CreateConnectBalanceTransferRequired::new(
    Money::new("EUR", "12.50")?,
    "Invoice fee",
    "org_source",
    "Platform fee",
    "org_dest",
    "Merchant share",
)?;
let _ = client.connect_balance_transfers().create(required, Some(key)).await?;
# Ok(())
# }
```

See also: [`payouts-and-transfers.md`](payouts-and-transfers.md), [`multi-merchant.md`](multi-merchant.md).
