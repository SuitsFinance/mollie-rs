# Official Mollie SDK parity assessment (`mollie-rs` 0.6.1)

> **STALE (2026-08-10).** This audit describes **0.6.1** @ `d5a2527`.  
> Do **not** treat the “24 missing operations”, missing `with_credential`, or missing request hooks claims as current.  
> **Current baseline:** crate **0.7.0**, Tier G **124/124** ops, see  
> [`docs/sdd/1.0-readiness/00-baseline.md`](../sdd/1.0-readiness/00-baseline.md) and  
> [`docs/audits/openapi-repin-0.7.0.md`](openapi-repin-0.7.0.md).  
> Remaining gaps are primarily **Tier-S facades** for high-risk domains and **transport safety kernel** hardening (delivery outcomes, host/redirect, profile SSOT)—not OpenAPI pin parity.

**Repository:** suitsfinance/mollie-rs  
**Crate version:** `0.6.1` (Cargo.toml) — *historical*  
**Branch audited:** `main` @ `d5a2527`  
**MSRV:** 1.88  
**Audit date:** 2026-08-04  

## Method

This assessment is **evidence-driven**. Sources:

| Source | Evidence |
| --- | --- |
| Local crate | Full tree under `src/`, `tests/`, `scripts/`, `.github/workflows/ci.yml`, `specs-3.0.yaml` |
| Official OpenAPI | `https://github.com/mollie/openapi` `specs.yaml` (downloaded 2026-08-04; 124 operations) |
| PHP SDK | `mollie/mollie-api-php` default `main` (hand-written endpoints, adapters, recipes) |
| TypeScript SDK | `mollie/mollie-api-typescript` default `main` (Speakeasy; 120 `src/funcs/*`) |
| Go SDK | `mollie/mollie-api-golang` default `main` (Speakeasy; 124-op `.speakeasy/out.openapi.yaml`) |
| Java / C# SDKs | Speakeasy-generated mirrors of the same contract surface |

Legend used throughout:

| Status | Meaning |
| --- | --- |
| **Implemented** | Present, tested, recommended for apps |
| **Partially implemented** | Present with gaps |
| **Generated only** | Tier G method exists; no validated Tier S facade |
| **Facade missing** | Operation or high-value workflow lacks Tier S API |
| **Not applicable to Rust** | Language/runtime-specific; do not copy |
| **Should not be copied** | Other SDK design that conflicts with payment safety / Rust ergonomics |
| **Requires provider-contract verification** | Seen in SDKs/OpenAPI; not yet in local pin / needs regen |

---

## Executive summary

`mollie-rs` is already a **payment-safe Rust SDK foundation**, not a thin generated client:

- Two-tier architecture (stable facade → generated client → transport).
- Validated money/IDs/credentials with redacted `Debug`.
- Sticky-key-only write retries (default retries **off**).
- Bounded pagination with cycle detection.
- Next-gen HMAC webhook verification + classic form body parser.
- Route capability metadata + generation reproducibility CI + cargo-deny.

The largest material gap versus the **current official Mollie contract** is **provider operation coverage**:

| Metric | Count |
| --- | --- |
| Local pin (`specs-3.0.yaml`) | **100** operations |
| Official OpenAPI / Speakeasy SDKs | **124** operations |
| **Missing from local pin** | **24** operations |

Missing areas (verified in `mollie/openapi` + TS/Go modules):

1. Business accounts + transactions  
2. Business-account transfers  
3. Payouts  
4. Sessions  
5. Unmatched credit transfers  
6. Verify payee  
7. Terminal pairing codes  
8. OAuth token generate/revoke (`/oauth2/tokens`, outside `/v2`)  
9. Payment route get (`payment-get-route`)

Secondary gaps (SDK ergonomics / ops):

- No client-level sticky `profileId` (TS/Go have globals; Rust only per-call params).  
- No scoped `with_credential` clone helper for multi-merchant Connect.  
- Retry classification is primarily **HTTP-method** based at send-time, not operation-id based.  
- `total_deadline` can still execute a leftover request after budget break (documented bug).  
- No public request/response/error hooks (TS Speakeasy hooks).  
- Domain facades cover core payments stack only (7 modules; 3 validated write builders).  
- Pagination helpers not universal across all list facades (`stream_*` missing).  
- Upstream OpenAPI drift CI is **advisory** (`continue-on-error: true`).  
- PHP-style recipe docs are stronger than Rust guides.  
- Version truth drift in some docs still cites `0.6.0` while crate is `0.6.1`.

**Recommendation:** treat this workstream as a path to **0.7.x** (contract + ergonomics) and **0.8/1.0-rc** (facade freeze), not a claim of 1.0 readiness.

---

## Architecture comparison

```
Application
    │
    ├─ Tier S: MollieClient, domain facades, Money/IDs, errors, webhooks
    │       ↓
    ├─ Tier G: Client + routes/* + types (OpenAPI-generated)
    │       ↓
    └─ Transport: reqwest, RetryPolicy (opt-in), idempotency headers
```

| Concern | PHP | TS/Go/Java/C# (Speakeasy) | mollie-rs |
| --- | --- | --- | --- |
| Contract source | Hand + docs | Speakeasy OpenAPI | Checked-in OpenAPI + progenitor |
| Client entry | `MollieApiClient` | SDK + options | `MollieClient` + `Client` |
| HTTP customization | Adapters (Guzzle/PSR-18/Curl) | `httpClient` option | `Client::new_with_client`, builder timeouts |
| Globals | testmode traits | `profileId`, `testmode`, custom UA | sticky `testmode` only |
| Idempotency | Middleware + generators | SDK retry + headers | sticky key + UUID default |
| Retry default | Configurable strategy | Speakeasy retry config | **Disabled** by default (safer) |
| Webhooks | SignatureValidator + event map | Models | HMAC verifier + classic parser |
| Recipes | Strong (`docs/recipes/*`) | USAGE/docs models | Examples matrix; fewer guides |

**Should not be copied:** Speakeasy’s default willingness to retry writes without forcing caller-owned idempotency keys; unbounded lazy collections without explicit budgets; mixing application ledger logic into the client.

---

## Operation coverage

### Local (Implemented / Generated only)

100 operations across Payments, Refunds, Captures, Chargebacks, Customers, Mandates, Subscriptions, Methods, Payment Links, Profiles, Onboarding, Organizations, Permissions, Clients, Client Links, Connect balance transfers, Balances, Settlements, Invoices, Sales invoices, Terminals (get/list only), Wallets (Apple Pay session), Webhooks + webhook events, Delayed routing (list/create only).

See `docs/route-coverage.md` and `docs/registries/operation-registry.yaml`.

### Missing (Requires provider-contract verification → then Generated + Facade)

| Operation | Method | Path (under API base unless noted) | Priority |
| --- | --- | --- | --- |
| `list_business_accounts` | GET | `/business-accounts/accounts` | P1 Connect/BA |
| `get_business_account` | GET | `/business-accounts/accounts/{id}` | P1 |
| `list_business_account_transactions` | GET | `.../transactions` | P1 |
| `get_business_account_transaction` | GET | `.../transactions/{id}` | P1 |
| `create_transfer` | POST | `/business-accounts/transfers` | P1 |
| `get_transfer` | GET | `/business-accounts/transfers/{id}` | P1 |
| `list_payouts` | GET | `/payouts` | P1 |
| `create_payout` | POST | `/payouts` | P1 |
| `get_payout` | GET | `/payouts/{id}` | P1 |
| `cancel_payout` | DELETE | `/payouts/{id}` | P1 |
| `create_session` / `get_session` | POST/GET | `/sessions` | P2 |
| unmatched CT list/get/match/return | * | `/unmatched-credit-transfers*` | P1 |
| `verify_payee` | POST | `/business-accounts/payee-verifications` | P1 |
| terminal pairing codes (4) | * | `/terminals/pairing-codes*` | P2 |
| `oauth_generate_tokens` / `oauth_revoke_tokens` | POST/DELETE | `/oauth2/tokens` (absolute, not `/v2`) | P0 Connect |
| `payment_get_route` | GET | `/payments/{id}/routes/{routeId}` | P2 |

**Proposed Rust approach:**

1. Re-pin `specs-3.0.yaml` from official openapi with path adaptation (`/v2` strip; special-case oauth base).  
2. Regenerate via `scripts/generate_openapi_client.*`.  
3. Extend `ROUTE_CAPABILITIES` / registry.  
4. Add Tier S facades only where validation adds payment safety (payouts, OAuth, verify payee).  

**Compatibility:** additive for consumers; Tier G may introduce many new types (pre-1.0 minor OK with changelog).

---

## Authentication, OAuth, profile, testmode

| Capability | Status in mollie-rs | Official SDK evidence | Gap action |
| --- | --- | --- | --- |
| API key Bearer | **Implemented** | all | — |
| OAuth access token Bearer | **Implemented** | all | — |
| Basic Auth (client credentials) | **Implemented** | PHP/TS OAuth | Use for token endpoints after contract land |
| Env credential loading | **Implemented** | PHP | — |
| Secret redaction in Debug | **Implemented** | varies | keep; expand tests |
| Optional zeroize | **Implemented** (feature) | rare | extend to webhook secret |
| Sticky testmode | **Implemented** | TS/Go globals | keep live-only reject |
| Sticky profileId | **Facade missing** | TS `profileId`, Go `WithProfileID` | add `ClientContext` |
| Operation override precedence | **Partially** | TS/Go | formalize operation > client > omit |
| Scoped credential clone | **Facade missing** | multi-merchant apps | `with_credential` rebuild |
| OAuth generate/revoke API | **Requires contract** | TS `oauthGenerate`/`oauthRevoke` | after OpenAPI pin |
| Profile restricted token errors | **Implemented** catalog | Mollie errors | — |

**Ownership:** client context stays on `Client`/`MollieClient` (Tier S). Do not inject profile into routes that lack the parameter (capabilities already track `requires_profile_scope` / params).

---

## Idempotency and retry

| Capability | Status | Notes |
| --- | --- | --- |
| Idempotency-Key always sent | **Implemented** | UUID if unset |
| Sticky key for retries | **Implemented** | `with_idempotency` / `with_idempotency_key` |
| Write auto-retry without sticky key | **Blocked (good)** | policy requires sticky key |
| Default retries disabled | **Implemented** | safer than Speakeasy defaults |
| Safe-read retry | **Implemented** | opt-in `default_safe` |
| 429 Retry-After | **Partially** | integer seconds only; not HTTP-date |
| Route-specific retry class in send() | **Partially** | capabilities have classes; send uses method |
| total_deadline semantics | **Partially / bug** | leftover request after break (lib.rs ~633) |
| Operation-level retry config | **Facade missing** | TS per-call retry | RequestRetryConfig |
| Connection failure retry flag | **Partial** | retries connect/timeout when allowed |

**Payment-safety rule to preserve forever:** never auto-retry financial writes without a **caller-owned** stable idempotency key.

---

## Pagination

| Capability | Status |
| --- | --- |
| `Page` / `PageCursor` / `PaginationGuard` | **Implemented** |
| Cursor cycle detection (set) | **Implemented** |
| max pages / max items budgets | **Implemented** |
| `list_page` / `list_all` on major facades | **Partially** (payments, refunds, captures, mandates, payment links, subscriptions list_page) |
| `stream_pages` / `stream_items` on every list facade | **Facade missing** (primitives exist: `AsyncPaginator`, `ItemStream`) |
| Universal registry-driven list surface | **Partially** (`paginated` flag in capabilities) |

---

## Webhooks

| Capability | Status |
| --- | --- |
| Classic form `id=` parser | **Implemented** |
| Next-gen raw HMAC SHA256 hex | **Implemented** |
| Constant-time compare | **Implemented** |
| Current + previous secret | **Implemented** |
| Bounded body size | **Implemented** |
| Missing vs malformed signature | **Implemented** |
| Verify-before-decode | **Implemented** |
| Replay prevention ownership | **Documented as app-owned** (correct) |
| Event type map (PHP-style) | **Should not be copied** wholesale; models may grow with contract |
| Integration traits (store/dispatcher/refetch) | **Facade missing** (docs/traits only) |
| Framework recipes (Axum/Actix) | **Partially** (docs; not full recipes) |
| Zeroize webhook secret | **Partially** (credential feature; secret type gap) |

---

## Errors and metadata

| Capability | Status |
| --- | --- |
| Structured `MollieError` | **Implemented** |
| Catalog keys / envelopes | **Implemented** |
| `status` / `request_id` / `retry_after` / `provider_code` | **Implemented** |
| `is_timeout` / authz / retryable | **Implemented** |
| `is_connection_failure` / `is_cancelled` | **Partially / missing** |
| operation / method / attempt on errors | **Partial** (metadata fields exist; not always filled) |
| Response validation distinct errors | **Partial** (`InvalidResponsePayload`, `MalformedProviderResponse`) |
| Redacted URL without secrets | **Missing** helper |

---

## HTTP customization and observability

| Capability | Status |
| --- | --- |
| Custom base URL | **Implemented** |
| Timeouts / connect timeout | **Implemented** |
| Default headers | **Implemented** |
| Custom user-agent | **Implemented** |
| User-agent suffix | **Missing** |
| Inject pre-built reqwest client | **Partially** (`from_generated` / `new_with_client`; not builder-first) |
| Request/response/error hooks | **Missing** public Tier S hooks |
| Proxy / custom TLS | **App-owned via reqwest** (document) |
| Structured tracing fields | **Partially** |
| Secret-leak tests for logs/Debug | **Partially** |

---

## Domain facades

| Facade | Status | Invariant added |
| --- | --- | --- |
| `PaymentsApi` | **Implemented** | validated create |
| `RefundsApi` | **Implemented** | validated create |
| `CapturesApi` | **Partially** | list helpers; create builder present in write_requests |
| `SubscriptionsApi` | **Implemented** | validated create |
| `MandatesApi` | **Partially** | SEPA builder |
| `PaymentLinksApi` | **Partially** | create builder |
| `WebhooksApi` | **Implemented** | classic + next-gen glue |
| Accounts / payouts / transfers / sessions / UCT / verify / oauth / connect / settlements / orgs / invoices | **Facade missing** (generated routes for some exist) | |

Validated write builders in capabilities: **3** (`create_payment`, `create_refund`, `create_subscription`) — capture/mandate/payment-link builders exist in code but capabilities may lag (see `VALIDATED_OPERATIONS` in generator).

---

## Documentation, examples, tests, release

| Area | Status |
| --- | --- |
| Route examples + example binaries | **Implemented** (strong) |
| Contracts under `docs/contracts/` | **Implemented** |
| Task guides / recipes (PHP parity) | **Partially / missing** |
| Migration docs per break | **Partially** (CHANGELOG) |
| WireMock HTTP contract tests | **Implemented** core paths |
| Property / fuzz targets | **Missing** |
| Live e2e default CI | **Should not enable** without secrets; opt-in exists |
| Generation reproducibility CI | **Implemented** |
| Upstream drift CI | **Partial** (advisory) |
| cargo-deny blocking | **Implemented** |
| Public API diff CI | **Missing** |
| Version truth enforcement | **Partial** (some docs stale at 0.6.0) |

---

## Gap backlog (prioritized)

### P0 — contract + safety truth

1. Re-pin OpenAPI to official 124-op contract; regenerate Tier G.  
2. Machine-readable operation registry in CI (local vs upstream counts, missing/removed).  
3. Dangerous upstream drift as release-blocking review signal.  
4. Fix `total_deadline` leftover-request semantics.  
5. Route-aware retry classification in `Client::send`.  
6. Version truth: all docs match `Cargo.toml`.

### P1 — Connect / multi-merchant / BA

7. Client context: `profile_id`, `user_agent_suffix`, precedence.  
8. `with_credential` scoped clients.  
9. OAuth facade after contract.  
10. Payouts / transfers / BA / UCT / verify-payee facades + tests.  

### P2 — ergonomics

11. Request hooks.  
12. Universal `stream_*` on paginated facades.  
13. Operation-level retry config.  
14. Guides + recipes (Axum/Actix webhooks, reconciliation).  
15. Error helper completeness + redaction tests.  

### P3 — 1.0 readiness

16. Public API freeze plan.  
17. Property/fuzz suites.  
18. Release artifact (digests, parity report).  
19. Facade gap registry empty for high-value writes.

---

## What already must not be reimplemented

- Progenitor-generated route methods (use regeneration).  
- Money/ID validation.  
- Webhook HMAC raw-byte verifier.  
- Sticky-key write retry policy.  
- Pagination cycle guards.  
- Error catalog mapping from Postman fixtures.  

## Intentionally not in this crate

- Application ledger / accounting.  
- Durable webhook queue / DB.  
- Generic multi-PSP abstraction.  
- Unbounded auto-pagination defaults.  
- Live Mollie credentials in default CI.

---

## Scores (honest, post-audit)

| Axis | Score (/10) | Rationale |
| --- | --- | --- |
| Architecture | 8.8 | Clear tiers; missing context/hooks |
| Payment safety | 8.9 | Strong defaults; deadline edge case |
| Provider parity | 6.5 | 100/124 ops |
| Domain API quality | 7.8 | Core strong; Connect/BA thin |
| Transport | 8.0 | Good policy; classification incomplete |
| Docs / recipes | 7.0 | Contracts strong; guides thin |
| CI / governance | 8.0 | Deny+gen solid; upstream advisory |
| **Overall** | **7.9** | Best-in-class Rust safety path; not full Mollie surface yet |

## Next recommended version

- **0.7.0** — OpenAPI re-pin + 24 ops + client context + credential scope + deadline fix + registry CI.  
- **0.8.0** — BA/payout/OAuth facades + hooks + guides.  
- **1.0.0-rc** — facade freeze, no silent Tier S breaks.
