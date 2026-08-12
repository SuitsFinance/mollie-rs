# Public API inventory (Tier-S + kernel)

**HEAD:** `e3358d2e49cb065d690deea8b43cdf2c9ed93a8a`  
**Stability policy:** `docs/API-STABILITY.md`  
**Crate root exports:** `src/lib.rs`

---

## 1. Tier model

| Tier | Surface | App guidance |
| --- | --- | --- |
| **S** | `MollieClient` domain accessors + validated builders | Preferred application API |
| **G** | Generated `Client` routes + `types::*` | Complete contract escape hatch; pin-churn expected |
| **Kernel** | Retry, delivery, redirects, pagination origin, profiles, credentials | Behavioral safety; fail-closed rules must not loosen |

---

## 2. Tier-S facades (`src/domain/mod.rs`)

| Accessor (expected) | Module | API type | list_page | list_all | stream_* | Notes |
| --- | --- | --- | --- | --- | --- | --- |
| `payments()` | payments | PaymentsApi | Y | Y | N | create/get + routes |
| `refunds()` | refunds | RefundsApi | Y | Y | N | cancel |
| `captures()` | captures | CapturesApi | Y | Y | N | |
| `mandates()` | mandates | MandatesApi | Y | Y | N | SEPA helper |
| `payment_links()` | payment_links | PaymentLinksApi | Y | Y | N | |
| `subscriptions()` | subscriptions | SubscriptionsApi | Y | — | N | update/cancel |
| `webhooks()` | webhooks | WebhooksApi | — | — | — | verify/parse |
| `payouts()` | payouts | PayoutsApi | Y | Y | N | create/cancel |
| `transfers()` | transfers | TransfersApi | — | — | — | signature required |
| `oauth()` | oauth | OAuthApi | — | — | — | token ops |
| `sessions()` | sessions | SessionsApi | — | — | — | |
| `terminals()` | terminals | TerminalsApi | Y | Y | N | pairing |
| `verify_payee()` | verify_payee | VerifyPayeeApi | — | — | — | |
| unmatched CT | unmatched_credit_transfers | UnmatchedCreditTransfersApi | Y | Y | N | match/return |
| **connect balance** | **missing** | — | — | — | — | Tier-G `routes/connect.rs` only |

Grammar target (mission): `create|get|update|cancel|delete|list_page|list_all|stream_pages|stream_items` where Mollie supports the verb. **Do not invent verbs.**

---

## 3. Kernel / safety exports (sample)

From `src/lib.rs` public surface (non-exhaustive):

- Client: `MollieClient`, `MollieClientBuilder`, `with_credential`, `with_profile_id`, `with_idempotency`
- Auth: `ApiKey`, `BasicAuth`, `Credential`, `OAuthAccessToken`
- Money/IDs: `money::*`, `ids::*`, `IdempotencyKey`
- Safety: `OperationSafetyProfile`, `AuthClass`, `MutationClass`, `IdempotencyClass`, `RouteCapability`, `ROUTE_CAPABILITIES`
- Transport: `DeliveryOutcome`, `RetryClass`, `RetryPolicy`, `compute_backoff`
- Errors: `MollieError`, `MollieResult`, delivery/retry helpers
- Webhooks: verify + notification types
- Hooks: `RequestHook`, redaction-aware context

---

## 4. Consistency gaps (P17/P12)

| Gap | Evidence | Priority |
| --- | --- | --- |
| No `stream_pages` / `stream_items` | `rg fn stream_` empty | P2 |
| Connect facade missing | no domain module | P1 |
| Subscriptions lack `list_all` | domain API | P3 |
| Tier-G discoverability | no `raw()`; docs must state Tier-G lower-level | P2 docs |
| Guide coverage | 3 guides vs 12 mission | P1 |

---

## 5. Examples

`examples/` count **126** including high-risk: `create_payout`, `cancel_payout`, `create_transfer`, `create_connect_balance_transfer`, payment/refund/capture, oauth-adjacent, webhooks. Prefer Tier-S in new examples; keep one Tier-G teaching example max.

---

## 6. Semver / stability

- Pre-1.0: additive preferred; breaking needs changelog  
- CI: `cargo-semver-checks` vs crates.io  
- Generated models: **not** full semver-stable (pin-driven) — already stated in `API-STABILITY.md`
