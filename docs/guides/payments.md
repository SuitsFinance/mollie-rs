# Payments guide

Prefer Tier-S: `client.payments()`.

## Create

Use `CreatePaymentRequired` + optional sticky `IdempotencyKey`.

- Retries: `create_payment` is **IdempotentWrite** — multi-attempt only with caller-owned sticky key.
- Timeout after possible send → `error.delivery_outcome() == Some(Unknown)` — **do not** blind replay without sticky key + reconciliation.

## Cancel

`payments().cancel(id, body, key)` — IdempotentWrite. Persist sticky key if you may retry.

## Customer payments

`payments().create_for_customer(customer_id, required, key)`.

## List / stream

`list_page` / `list_all` / `stream_pages` / `stream_items` are **bounded** by `PaginationGuard` (never unbounded).

## Never log

API keys, Authorization headers, full card/PII payloads.
