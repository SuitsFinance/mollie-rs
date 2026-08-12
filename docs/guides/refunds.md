# Refunds guide

Tier-S: `client.refunds()`.

- Create with `CreateRefundRequired` + sticky key for retries (`IdempotentWrite`).
- Cancel with `refunds().cancel` while cancelable.
- `DeliveryOutcome::Unknown` after ambiguous transport failure — reconcile via get/list before creating another refund.
- List helpers are budget-guarded.
