# SDD 00 — Baseline forensics (`mollie-rs` 0.7.0)

**Status:** Phase 0 freeze  
**Crate version:** `0.7.0` (`Cargo.toml`)  
**HEAD (session freeze):** `55187ee6cde3fdcee4b1a82ada4675362e472bab`  
**Branch:** `floris-xlx-mollie-rs-plan-review`  
**MSRV:** 1.88  
**Freeze date:** 2026-08-10  
**Supersedes:** `docs/audits/official-sdk-parity-assessment.md` (0.6.1 @ `d5a2527`) for operation-gap claims

## 1. Evidence sources

| Source | Path / command | Result |
| --- | --- | --- |
| Version | `Cargo.toml` | `0.7.0` |
| Local OpenAPI | `specs-3.0.yaml` / `specs.yaml` | **124** `operationId` |
| Generated routes | `rg "pub async fn" src/routes` | **124** |
| Capabilities | `src/route_capabilities.rs` `ROUTE_CAPABILITIES` | **124**; 0 missing vs pin |
| Registry | `docs/registries/operation-registry.yaml` | **124** |
| Drift report | `python scripts/report_api_drift.py` → `docs/api-drift-report.md` | Regenerated 2026-08-10; **124** |
| Re-pin report | `docs/audits/openapi-repin-0.7.0.md` | 100→124 closed |
| Production checklist | `docs/production-checklist.md` | Matches 124 + facade gaps |
| Tier-S modules | `src/domain/` | 7 facades |
| Transport | `src/transport/*`, `Client::send` in `src/lib.rs` | Sticky-key writes; deadline no leftover send |
| CI | `.github/workflows/ci.yml` | fmt/clippy/test/doc/MSRV/generation; upstream pin blocking; missing-ops advisory history |

## 2. Coverage language (precise)

| Layer | Count | Meaning |
| --- | ---: | --- |
| Official / local pin operations | **124** | OpenAPI operations in checked-in pin |
| Tier **G** typed route methods | **124** | `Client` async methods in `src/routes/*` |
| Route capabilities rows | **124** | Retry/testmode/idempotency/pagination metadata |
| Tier **S** domain facades | **7** | payments, refunds, captures, mandates, payment_links, subscriptions, webhooks |
| Validated write builders (`RouteAccess::ValidatedFacade`) | **3** | create_payment, create_refund, create_subscription (capability table) |
| Capability retry: SafeRead | **75** | |
| Capability retry: IdempotentWrite | **35** | Sticky key required for auto-retry |
| Capability retry: NonRetryableWrite | **14** | Never auto-retry (incl. OAuth tokens) |

**Do not say** “124 operations supported” without distinguishing Tier G vs Tier S.

## 3. Architecture (current)

```text
Application
  → Tier S: MollieClient + src/domain/* (7 modules)
  → Tier G: Client + src/routes/* + types (OpenAPI/progenitor)
  → Transport: reqwest, RetryPolicy (default off), sticky idempotency, hooks
  → Mollie API (HTTPS; loopback HTTP allowed for mocks)
```

`RouteCapability` is the closest existing SSOT for per-op policy. Plan v2 elevates it to full **`OperationSafetyProfile`** (auth class, mutation class, delivery-aware retry) without dual registries.

## 4. Area matrix

| Area | Status | Risk | Existing implementation | Missing work | Priority |
| --- | --- | ---: | --- | --- | --- |
| Payments | Tier-S + G | Med | `domain/payments`, validated create | Stream helpers consistency; delivery Unknown | P1 |
| Refunds | Tier-S + G | Med | `domain/refunds`, validated create | Same | P1 |
| Captures | Tier-S + G | Med | `domain/captures` | — | P2 |
| Chargebacks | G only | Low | routes | Optional thin facade | P3 |
| Customers | G only | Low | routes | Optional facade | P3 |
| Mandates | Tier-S + G | Med | SEPA helper | — | P2 |
| Subscriptions | Tier-S + G | Med | validated create | — | P2 |
| Payment Links | Tier-S + G | Med | domain | — | P2 |
| Profiles / Orgs / Clients | G only | Low | routes | — | P3 |
| Connect balance transfers | G only | High | routes | Tier-S + Connect isolation tests | P1 |
| Business accounts | G only | High | `routes/accounts` | Tier-S reads after kernel | P1 |
| Transfers | G only | **Crit** | `routes/transfers` + signing headers | Tier-S + kernel Unknown | P0 after kernel |
| Payouts | G only | **Crit** | `routes/payouts` | Tier-S create/cancel | P0 after kernel |
| Settlements / Invoices / Sales inv | G only | Low–Med | routes | — | P3 |
| Sessions | G only | Med | routes | Justified facade? | P2 |
| Terminals + pairing | G only | Med | routes | Pairing Tier-S if safety adds value | P2 |
| Wallets | G only | Low | Apple Pay session | — | P3 |
| OAuth tokens | G only | **Crit** | `routes/oauth` `/oauth2/tokens` | Tier-S + secret types | P0 after kernel |
| Verify payee / UCT | G only | High | routes | Tier-S after kernel | P1 |
| Webhooks | Tier-S + G | High | verify + classic parse | Adversarial/fuzz expand | P1 |
| Credentials | Present | High | `auth`, `with_credential` | Concurrency stress; no dual mutate | P0 |
| Idempotency | Present | High | sticky vs UUID; `IdempotencyKey` | Taxonomy docs; property proofs | P0 kernel |
| Retry | Present | **Crit** | op `retry_class` + sticky gate; budget | Delivery Unknown; model tests; Retry-After HTTP-date | P0 kernel |
| Errors | Strong | Med | many `is_*` helpers | Explicit delivery outcome | P0 kernel |
| Pagination | Present | High | `PaginationGuard` cycle/budgets | **No next-URL origin allowlist** | P0 kernel |
| Observability | Present | Med | hooks + redacted URL | Metadata allowlist audit | P1 |
| Contract generation | Present | Med | scripts + CI | Profile-driven manifest | P1 |
| Drift detection | Partial | High | local inventory CI; upstream pin blocking | Dangerous semantic drift hard-fail | P1 |
| CI | Strong | Med | MSRV 1.88 enforced | semver-checks optional | P2 |
| Public API stability | Weak docs | Med | pre-1.0 | `API-STABILITY.md` | P1 |
| Documentation | Partial | Med | checklist, 2 guides | Recipe guides; supersede stale audits | P0/P2 |
| Redirect / host safety | **Gap** | **Crit** | default reqwest redirects; Auth in default_headers | Explicit policy + regression test | P0 kernel |
| Delivery outcome | **Missing** | **Crit** | timeout/connect treated as retry signal | NotSent/Rejected/Succeeded/Unknown | P0 kernel |
| Cancellation | Partial | High | `is_cancelled` exists | Post-transmit Unknown docs | P0 kernel |

## 5. Kernel evidence (re-verified on HEAD)

### 5.1 Already closed (do not re-open as 0.6.1 bugs)

| Claim | Evidence |
| --- | --- |
| 24 missing OpenAPI ops | **Closed** — 124/124 pin (`openapi-repin-0.7.0.md`, drift report) |
| Sticky-key-only write retries | `RetryPolicy::allows` + `Client::send` uses `retry_class_for_operation` + sticky (`src/lib.rs` ~700–715) |
| Auto UUID ≠ sticky | Docs + `has_sticky` only from client-bound key |
| Deadline leftover request | **Fixed:** budget check before attempt; backoff that would exceed budget returns last result without new send (`src/lib.rs` 719–790; `policy.rs` docs) |
| `with_credential` | Present; rebuilds client; preserves timeouts/UA/retry/hooks (`client.rs` 376–407) |
| Request hooks | `src/hooks.rs` + builder |
| Secret redaction Debug | credentials/idempotency redacted on Client/Builder |

### 5.2 Kernel findings (status after Phase 2 kernel slice)

| ID | Finding | Status | Evidence |
| --- | --- | --- | --- |
| KER-001 | Delivery outcome Unknown | **Closed** | `DeliveryOutcome` + `Client::send` + `MollieError::delivery_outcome` |
| KER-002 | Redirect / Authorization leakage | **Closed** | `redirect::Policy::none()` + `does_not_follow_redirect_to_foreign_host` |
| KER-003 | Pagination next host | **Closed** | `from_list_link` origin allowlist + off-origin tests |
| KER-004 | OperationSafetyProfile SSOT | **Closed (v1)** | `operation_safety` aliases `RouteCapability` + derived classes |
| KER-005 | Retry model/property proofs | **Closed** | `simulate_retry_loop` + property_tests INV-WRITE-02/DEADLINE/DELIV |
| KER-006 | Cancellation after transmit | **Closed (docs)** | guide + `delivery_outcome` / `is_cancelled` docs |
| KER-007 | Connect concurrent isolation | **Closed** | `concurrent_scoped_credentials_do_not_cross_wire` |
| FAC-001 | High-risk money ops Tier-G only | **Open** | Facades after kernel — Phase 4–5 |

### 5.3 High-risk operation set (metric denominator seed)

Financial / credential-sensitive mutations (non-exhaustive seed; refine in profile freeze):

`create_payment`, `cancel_payment`, `create_refund`, `cancel_refund`, `create_capture`, `create_subscription`, `cancel_subscription`, `create_mandate`, `create_payment_link`, `create_customer_payment`, `create_payout`, `cancel_payout`, `create_transfer`, `create_connect_balance_transfer`, `oauth_generate_tokens`, `oauth_revoke_tokens`, `verify_payee`, `match_unmatched_credit_transfer`, `return_unmatched_credit_transfer`, `create_session`, `payment_create_route`, plus webhook verification path (not OpenAPI op).

**Primary metric (plan v2):**

```text
high_risk_ops_with_enforced_safety_invariants / total_high_risk_ops
```

Enforced = profile fields + transport honors + automated proof — **not** “has thin facade.”

## 6. Tier-S inventory (methods)

| Facade | Accessor | Methods (summary) |
| --- | --- | --- |
| Payments | `payments()` | create, create_raw, get, list_page, list_all |
| Refunds | `refunds()` | create, create_raw, get, cancel, list_page, list_all |
| Captures | `captures()` | create, create_raw, get, list_page, list_all |
| Mandates | `mandates()` | create_sepa, create, get, revoke, list_page, list_all |
| Payment links | `payment_links()` | create, create_raw, get, delete, list_page, list_all |
| Subscriptions | `subscriptions()` | create, create_raw, get, update, cancel, list_page |
| Webhooks | `webhooks()` | parse_classic, verify_next_gen, verify_and_decode_next_gen, get_event |

## 7. Stale documentation actions (this phase)

| Artifact | Action |
| --- | --- |
| `docs/api-drift-report.md` | **Regenerated** 2026-08-10 → 124 ops |
| `docs/route-coverage.md` | **Update** 100→124 + new groups |
| `docs/audits/official-sdk-parity-assessment.md` | **STALE banner** + pointer to this baseline |
| `docs/contracts/operation-coverage.md` | Already 124 — keep |

## 8. Invariants draft (freeze candidates)

See program plan v2: INV-WRITE-01/02, INV-DEADLINE-01, INV-DELIV-01, INV-CANCEL-01, INV-HOST-01, INV-PAGE-01, INV-CONN-01, INV-PROFILE-01, INV-IDEM-01, INV-SEC-01, INV-MONEY-01, INV-WH-01/02, INV-DRIFT-01, INV-TIER-01.

## 9. Baseline gate commands

```text
cargo fmt --check
cargo check --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo test --lib --tests --no-default-features
cargo doc --no-deps
python scripts/report_api_drift.py
python scripts/export_operation_registry.py
python scripts/check_generation_reproducibility.py
```

## 10. Phase 0 acceptance

- [x] HEAD + version recorded
- [x] Tier-G 124 vs Tier-S 7 precise
- [x] Drift report refreshed to 124
- [x] route-coverage + parity STALE banner applied
- [x] High-risk set seeded
- [x] P0 kernel findings listed
- [x] Baseline cargo gates green (`cargo test` lib+doctest; `cargo clippy -D warnings`; `cargo fmt`)

## 11. Next phase

Phase 2 transport safety kernel **landed** (this branch). Next: OAuth Tier-S → payouts/transfers facades → drift/semver → assurance.
