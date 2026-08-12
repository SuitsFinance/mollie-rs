# OAuth and Connect guide

## OAuth

`client.oauth()` — token generate/revoke are **NonRetryableWrite**. Use secret types; Debug is redacted.

## Connect balance transfers

`client.connect_balance_transfers()`:

- Validated `CreateConnectBalanceTransferRequired` (distinct `org_` parties, amount, descriptions).
- Sticky idempotency for retries.
- Isolate merchants with `with_credential` / `with_profile_id` clones under concurrency.
