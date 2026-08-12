# SDD 06 — Retries, idempotency, delivery outcomes (KERNEL)

## Context
Payment SDK correctness hinges on retry + idempotency + ambiguous delivery.

## Problem
Without explicit Unknown outcome, timeouts after send look like “connection failures” and may be retried unsafely. Multiple metadata sources can diverge.

## Existing behavior
- `RetryClass` + `RetryPolicy::allows(class, sticky)` (`transport/`)
- `Client::send`: registry class; sticky from client key; max_attempts 1 if !may_retry; budget checks; backoff; retries timeout/connect and transient HTTP
- Idempotency header always set (sticky or UUID)
- `IdempotencyKey` type exists
- Deadline leftover send fixed

## Desired behavior

### OperationSafetyProfile SSOT
Evolve `RouteCapability` (or wrapper type alias) to include:
auth, mutation, retry, idempotency, testmode, profile_scope, pagination.
Transport, facades, drift tools, tests consume **only** this table.

### DeliveryOutcome
| Variant | Meaning |
| --- | --- |
| NotSent | Not known to leave client |
| Rejected | Definitive provider rejection |
| Succeeded | Definitive success |
| Unknown | May have been processed (post-transmit timeout/reset; cancel after send) |

Rules:
- Financial + Unknown + !sticky → **no** auto-retry; surface outcome
- Financial + Unknown + sticky → retry only per profile (same key)
- NotSent → may retry if profile allows and budget remains
- Drop after transmit → document as Unknown; apps must use sticky keys

### Idempotency taxonomy
Request correlation id ≠ auto UUID marker ≠ caller-owned logical key (`IdempotencyKey`).

### Retry engine proofs
Property/model:
- financial write without sticky → attempts ≤ 1
- no attempt.begin after total_deadline
- sequences: connect_fail, timeout, 429, 503, success, deadline

### Retry-After
Support delta-seconds (exists) + HTTP-date if practical.

## Non-goals
Application-level outbox; automatic GET-reconcile for all resources.

## Invariants
INV-WRITE-01/02, INV-DEADLINE-01, INV-DELIV-01, INV-CANCEL-01, INV-IDEM-01, INV-PROFILE-01.

## API design
- Public: keep `RetryPolicy`, `RetryClass`; add `DeliveryOutcome` on errors/metadata as needed
- Internal: classify_reqwest_error(before_transmit: bool) → outcome
- Prefer additive public API

## Testing
Unit + http_contract mock + property_tests state machine.

## Acceptance
- [ ] Profile SSOT 124 ops
- [ ] DeliveryOutcome used in send loop decisions
- [ ] Property proofs green
- [ ] Docs: safe-payment-retry guide updated for Unknown/cancel
