# SDD 03 — Payouts (Tier-S after kernel)

## Context
Payouts move money. Tier G routes exist (`list/create/get/cancel_payout`).

## Problem
Raw generated create is easy to misuse (retries, money, credentials).

## Existing behavior
Generated-only; IdempotentWrite/NonRetryable per capabilities; no domain facade.

## Desired behavior
`mollie.payouts()` with list/get/create/cancel where contract supports.
Create/cancel: sticky idempotency required for any retry path; Money validation; inherit kernel DeliveryOutcome.
**Blocked on Phase 2 kernel freeze.**

## Non-goals
Dashboard-only payout schedule management.

## Invariants
INV-WRITE-*, INV-MONEY-01, INV-IDEM-01, INV-DELIV-01.

## API design
```rust
mollie.payouts().create(req).idempotency_key(key).await?;
```
Justify each method vs Tier G.

## Testing
Mock path/method/headers; malformed amount; wrong credential class; 409/422/429/5xx; Unknown without key → single attempt.

## Acceptance
- [ ] Facade justification filled
- [ ] Mock contract tests
- [ ] No duplicate retry logic in facade
