# Mollie-rs production assessment (current)

**Repository:** SuitsFinance/mollie-rs  
**Version:** **0.7.0**  
**Authoritative readiness:** [`docs/release-readiness.md`](release-readiness.md)  
**MSRV:** 1.88  

## Scores (/10)

| Axis | Score |
| --- | --- |
| Architecture | 9.0 |
| Payment-safety model | 9.0 |
| Transport | 8.8 |
| Domain API quality | 8.5 |
| Test / CI | 8.5 |
| Release governance | 8.5 |
| Provider parity | 9.0 |
| **Overall** | **8.7** |

## What is implemented (do not list as missing)

- Generated OpenAPI client (**124/124** ops) + route capabilities + operation registry  
- Typed money, IDs, locale, phone, country codes  
- ResponseMetadata + bounded error context  
- IdempotencyKey; sticky-key write retries only; retries default off  
- Route-aware retry classification; `DeliveryOutcome` (NotSent / Rejected / Succeeded / Unknown)  
- Client profile context, request hooks, `with_credential`  
- HMAC Next-gen webhooks; missing vs malformed signatures  
- AsyncPaginator / ItemStream; set-based cursor cycle detection; pagination origin allowlist  
- Domain facades: payments, refunds, captures, subscriptions, mandates, payment links, webhooks, payouts, transfers, OAuth, sessions, terminals, verify-payee, unmatched CT  
- Validated create builders on high-risk writes  
- Wiremock: auth, idempotency, 400/401/429/502/503/504  
- CI: fmt, clippy `-D`, tests, MSRV, generation reproducibility, dangerous-profile drift, cargo-deny  

## Remaining gaps (honest)

| Gap | Severity |
| --- | --- |
| Live Mollie e2e not in default CI (opt-in only) | Medium (intentional) |
| Sandbox write matrix beyond payment create/idempotency | Medium |
| crates.io soak of full 0.7 public surface | Medium until first publish |
| Formal hostile security review sign-off doc | Low–medium |
| 1.0 stability freeze | Not claimed (see SDD 1.0-readiness) |

## Release recommendation

**0.7.x** is the production SDK line for typed Mollie API access with payment-safe transport defaults. **Not 1.0 ready** until assurance gates in `docs/sdd/1.0-readiness/` pass.

## Architecture (current)

```
App → MollieClient / domain facades (Tier S)
       → Client generated routes (Tier G)
         → reqwest + optional RetryPolicy / DeliveryOutcome
       → WebhookVerifier / classic WebhookNotification
```

Historical phase logs from earlier assessments remain under `docs/audits/` for provenance; treat this file and `docs/release-readiness.md` as current.
