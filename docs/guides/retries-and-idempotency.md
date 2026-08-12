# Retries and idempotency

- Default `RetryPolicy` does **not** auto-retry financial writes.
- `IdempotentWrite` multi-attempt requires **caller-owned sticky** `IdempotencyKey` on a scoped client.
- Auto UUID per request is **not** sticky and must not be treated as safe replay.
- `Retry-After` supports delta-seconds and HTTP-date; never sleep past remaining deadline.
