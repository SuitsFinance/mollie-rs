# Implementation completion report — official SDK parity modernization

**Date:** 2026-08-04  
**Crate:** mollie-rs **0.6.1**  
**Branch:** main (working tree; changes in this session)  

## 1. What was already implemented

- Two-tier architecture: `MollieClient` / domain facades → generated `Client` → reqwest transport.
- 100 OpenAPI operations in `specs-3.0.yaml` with generated routes + examples.
- `RouteCapability` metadata + generation reproducibility scripts/CI.
- Validated money, resource IDs, credentials (redacted Debug, optional zeroize).
- Sticky testmode with live-only rejection for business routes.
- Idempotency keys; sticky-key-only write retries; retries default off.
- Bounded pagination (`PaginationGuard`, cycle detection, `AsyncPaginator` / `ItemStream`).
- Webhook HMAC verifier (rotation, bounds, missing vs malformed) + classic form parser.
- Domain facades: payments, refunds, captures, subscriptions, mandates, payment links, webhooks.
- Validated write builders for create payment / refund / subscription (plus additional builders in `write_requests`).
- Structured error catalog + response metadata.
- WireMock HTTP contract tests; cargo-deny blocking; MSRV/fmt/clippy/doc tests.

## 2. What was missing (audit)

- **24 operations** vs official Mollie OpenAPI (124 ops): business accounts, transfers, payouts, sessions, unmatched CT, verify payee, terminal pairing codes, OAuth token endpoints, payment-get-route.
- Sticky **profileId** client context (TS/Go globals).
- Scoped **with_credential** for multi-merchant Connect.
- Public **request hooks**.
- Route-aware retry classification at send-time (capabilities unused in `send`).
- Correct **retry budget** semantics (leftover request after deadline).
- Machine-readable **operation registry** + parity audit docs.
- PHP-class **guides/recipes** depth.
- Upstream drift as stronger release signal.
- Universal `stream_*` on all list facades; OAuth/payout/BA facades.

## 3. What was implemented in this tranche

### Documentation / audit (required first deliverable)

| File | Purpose |
| --- | --- |
| `docs/audits/official-sdk-parity-assessment.md` | Evidence-based assessment vs PHP/TS/Go/Java/C# + openapi |
| `docs/audits/official-sdk-parity-matrix.yaml` | Machine-readable status matrix + gap backlog |
| `docs/audits/official-sdk-feature-matrix.md` | Cross-SDK feature table |
| `docs/audits/implementation-completion-report.md` | This report |
| `docs/registries/operation-registry.yaml` | 100 local ops + 24 upstream gaps |
| `scripts/export_operation_registry.py` | Registry generator |
| `docs/production-checklist.md` | Production readiness levels + checklist |
| `docs/guides/safe-payment-retry.md` | Retry + idempotency guide |
| `docs/guides/handle-signed-webhook.md` | Signed webhook guide |

### Code

| Change | Detail |
| --- | --- |
| `src/hooks.rs` | `RequestHook`, `RequestContext`, `SharedRequestHook` |
| `src/integration.rs` | `WebhookEventStore`, `WebhookDispatcher`, `PaymentStateRefetcher` boundaries |
| `Client` | `profile_id`, `request_hook`; route-aware retry; no leftover send after budget; URL redaction for hooks |
| `MollieClient` / builder | `profile_id`, `user_agent_suffix`, `http_client`, hooks, `with_credential` |
| `RetryClass` / policy | Documented provider-idempotent write; deadline = retry budget |
| `MollieError` | `is_connection_failure`, `is_cancelled`, `attempt_count`, `operation` |

## 4. Intentionally unimplemented (this tranche)

- Full OpenAPI re-pin + regeneration of 24 missing ops (P0 next; high blast radius on Tier G types).
- Domain facades for payouts/accounts/OAuth/sessions/UCT/verify-payee (blocked on generation).
- Operation-level `RequestRetryConfig` struct.
- Universal `stream_pages` / `stream_items` on every facade.
- Fuzz/property suites; public API diff CI; release digest artifact.
- PHP-scale recipe library (starters only).
- Axum/Actix full recipe binaries.
- Blocking CI for every upstream schema change (registry + audit first; dangerous-change policy documented).

## 5. Files changed (summary)

- New: hooks, integration, audits, registries, guides, production checklist, export script.
- Modified: `src/lib.rs`, `src/client.rs`, `src/error.rs`, `src/transport/*`.

## 6. Public API changes (additive)

- `RequestHook` / `RequestContext` / `NoopHook` / `SharedRequestHook`
- `WebhookEventStore` / `WebhookDispatcher` / `PaymentStateRefetcher`
- `MollieClient::with_profile_id` / `clear_profile_id` / `profile_id`
- `MollieClient::with_request_hook` / `with_shared_request_hook` / `with_credential`
- `MollieClientBuilder::profile_id` / `user_agent_suffix` / `http_client` / `request_hook` / `shared_request_hook`
- `Client::with_profile_id` / hooks / profile accessors
- `MollieError::is_connection_failure` / `is_cancelled` / `attempt_count` / `operation`
- `RetryClass::provider_idempotent_write()`
- Manual `Debug` for `Client` / `MollieClientBuilder` (redacts credentials/hooks)

## 7. Compatibility impact

- Additive for applications; no intentional Tier S removals.
- Retry budget no longer issues a leftover HTTP request after deadline break (behavior fix; safer).
- `Debug` for `Client` no longer uses derive (still usable; fields may differ).

## 8. Security impact

- Positive: redacted hook URLs; redacted Debug for sticky keys; integration docs stress verify-before-parse and refetch.
- No weakening of sticky-key write retry rule.
- Custom `http_client` path requires caller to attach Authorization correctly (documented).

## 9. Test commands executed

```text
cargo check --lib --tests
cargo test --lib
```

## 10. Test results

- `cargo test --lib`: **161 passed**, 0 failed (2026-08-04 session).

## 11. Generated-contract changes

- None in this tranche (local pin remains 100 ops).
- Registry records 24 upstream gaps from `mollie/openapi` specs.yaml (and Speakeasy Go/TS mirrors).

## 12. Release risks

- Incomplete provider surface vs official SDKs until OpenAPI regen.
- `with_credential` rebuild uses default timeouts (may surprise advanced timeout configs).
- Injected `http_client` bypasses builder Authorization injection.

## 13. Remaining decisions

1. When to re-pin OpenAPI (0.7.0 recommendation) and how to handle `/oauth2/tokens` base URL.
2. Whether BA/payout/OAuth facades ship in same minor as generation or follow-up.
3. Whether upstream dangerous drift becomes hard-fail in CI or release-only gate.
4. MSRV policy if generation toolchain requires bumps.

## 14. Recommended next version

- **0.7.0** — OpenAPI re-pin (124 ops), regenerate Tier G, OAuth + payouts facades, registry CI gate, more guides.
- **0.8.0** — Remaining facades, stream helpers, property/fuzz, API freeze prep.
- **1.0.0-rc** — Stability freeze for Tier S.

## Readiness boundary

| Level | Status after this work |
| --- | --- |
| SDK production readiness | **Yes for core payments path** (with known contract gaps documented) |
| Payment-integration production readiness | **App-owned**; guides help but not automatic |
| Full payment-infrastructure readiness | **No** — ledger/queue/Connect BA incomplete |
