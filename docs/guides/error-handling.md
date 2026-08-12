# Error handling

Prefer structured helpers on `MollieError` over string matching.

## Ask the error

- Validation vs authentication vs rate limit vs provider vs transport
- `delivery_outcome()` — `NotSent` / `Rejected` / `Succeeded` / `Unknown`
- `is_timeout()` / `is_connection_failure()` / `is_retryable()`
- `retry_after()` — delta-seconds or HTTP-date when present
- `request_id()` / `provider_code()` for support tickets

## Logging

Never log Authorization headers, API keys, OAuth client secrets, webhook secrets, or raw PAN/PII. Credential and secret types redact in `Debug`.

## Unknown after a write

Treat as “may have succeeded.” Reconcile with GET/list using the same business identifiers and sticky key policy — see [`safe-payment-retry.md`](safe-payment-retry.md).
