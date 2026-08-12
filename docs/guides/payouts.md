# Payouts guide

Tier-S: `client.payouts()`.

- Prefer `CreatePayoutRequired` (amount/balance/description validation).
- `create` / `cancel` are **IdempotentWrite** — sticky idempotency required for safe retry.
- After `Unknown` delivery, check payout status; never auto-duplicate without sticky key.
- `stream_pages` / `stream_items` honor `PaginationGuard`.
