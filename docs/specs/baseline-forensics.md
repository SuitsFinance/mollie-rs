# Phase 0 baseline forensics

## Test matrix (pre/post hardening session)

| Command | Result |
| --- | --- |
| `cargo +1.88.0 test --lib --tests` | 136 + 5 + 3 = 144 pass at session start |
| Generation reproducibility | 100 ops match |

## Architecture (actual)

```
Application
  → MollieClient (auth, base URL, retry_policy, sticky idempotency/testmode)
    → Client (generated routes, request(), send())
      → reqwest
  → PaymentsApi facade (domain)
  → WebhookVerifier / WebhookNotification
  → types::* / routes::* (generated)
```

## Non-regression invariants

1. `Client::request` always attaches `Idempotency-Key` (sticky or UUID v4).
2. Credentials redact in `Debug`.
3. Floating money rejected.
4. Classic webhooks parse `id=` only.
5. `app-helpers` optional; process env vars always work.

## Retry decision (post-fix)

| Class | Policy `default_safe` | Condition |
| --- | --- | --- |
| SafeRead (GET/HEAD) | may retry | transient status / connect / timeout |
| IdempotentWrite | may retry | **sticky** client idempotency key set |
| NeverAutoRetry / Unknown | never | — |

## Gaps remaining after this tranche

- Not all list routes have domain facades / paginators wired.
- Refunds/captures/subscriptions facades incomplete.
- Full wiremock matrix for every status still expanding.
- Live Mollie tests not in default CI.
