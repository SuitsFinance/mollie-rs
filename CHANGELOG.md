# Changelog

## [Unreleased]

## [0.8.0] (unreleased)

### Breaking

- **Removed `MollieClientBuilder::http_client`.** Callers can no longer inject a
  prebuilt `reqwest::Client` on the safe builder path (it bypassed redirect-none
  / TLS floor last-apply). Use `configure_http` for limited customization, or
  `MollieClient::from_generated` / `Client::new_with_client` for full transport
  ownership.
- **`CreatePayoutRequired.description`** is now `NullableField<String>` (was
  `Option<String>`) so omit / null / value match Mollie write semantics. Prefer
  `with_description` / `clear_description` / `omit_description` and
  `to_write_json` when exact null encoding is required.

### Security / hardening

- **`ResponseLimits`:** configurable ceilings for success JSON (default 8 MiB)
  and provider error bodies (default 64 KiB) on all generated route decode paths.
- Builder always last-applies redirect-none, TLS 1.2+, auth/UA headers, and
  timeouts after `configure_http`.
- `with_credential` preserves response body limits.
- CI `cargo-semver-checks` job is **fail-closed** (no `continue-on-error` / no
  swallowed exit). Tier-S snapshot remains blocking in the contract job.
- **Provider models (Phase 3):** `CreatePaymentRequired.due_date` and
  `UpdatePaymentRequired` use `NullableField` with `to_write_json` omit/null/value
  bodies; `PaymentStatusValue` (`OpenEnum`) preserves unknown payment statuses;
  Billink covered as first-class `PaymentMethod`.
- **Pagination consistency (Phase 4):** `stream_pages` / `stream_items` on
  captures, mandates, subscriptions, and terminals; subscriptions `list_all`;
  matrix in `docs/rc/pagination-matrix.md`.
- **Contract drift telemetry (Phase 5 / TEL-001):** opt-in
  `ContractDriftObserver` (client or global) with redacted
  `ContractDriftSignal`s for unknown open-enum values and off-origin
  pagination links; observer panics are isolated from the request path.
- **Workflow example matrix (Phase 6 / EX-001):** Tier-S money-path workflows
  mapped to compile-checked `examples/*.rs` via
  `docs/registries/tier-s-workflow-examples.yaml` and CI gate
  `scripts/check_workflow_examples.py` (see `docs/rc/workflow-example-matrix.md`).
- **Tier-S snapshot:** refreshed for captures/mandates/subscriptions/terminals
  stream + `SubscriptionsApi.list_all` surface from Phase 4.
- **Live assurance docs (Phase 7 / REL-001):** runbook + evidence paste pad in
  `docs/rc/live-assurance-evidence.md`; hostile review residuals refreshed for
  builder `http_client` removal and fail-closed semver. Credentialed live tiers
  remain operator-run (not default CI).
- **Terminal pairing 403 (TERM-403):** catalog key
  `TERMINAL_PAIRING_FORBIDDEN` (40304) and `MollieError::terminal_pairing_forbidden`,
  classified from pairing-denied provider bodies (fixture-backed).
- **Transport proxy policy (HTTP-003):** builder Debug does not leak
  `configure_http` proxy userinfo; `no_proxy` configure path covered by tests.

### Docs

- RC baseline pack under `docs/rc/` (baseline, residuals, acceptance matrix,
  transport security policy).
- `docs/API-STABILITY.md`: explicit Tier-S vs Tier-G vs Kernel policy and gate map.
- **Sales Invoices provider maturity (SUI-2369):** record `sales_invoices_api` as
  GA in `docs/registries/provider-maturity.yaml`; project `provider_maturity`
  onto `docs/registries/operation-registry.yaml`. Tier coverage stays Generated
  only (no API shape change).

## [0.7.1] (2026-08-12)

### Features

- **High-risk coverage metric (23/23):** frozen denominator in `operation_safety` + `check_dangerous_profile_drift.py`; CI report via `scripts/report_high_risk_coverage.py --require-full`.
- **Connect balance transfers Tier-S:** `client.connect_balance_transfers()` with `CreateConnectBalanceTransferRequired`.
- **Pagination streams:** `stream_pages` / `stream_items` on key list facades (payments, refunds, payouts, payment links, Connect, UCT).
- **Retry-After HTTP-date:** IMF-fix / RFC 2822 in addition to delta-seconds.
- **Production guides:** expanded `docs/guides/` index (payments, refunds, OAuth/Connect, pagination, multi-merchant, errors, testing).
- **Transport safety kernel:** first-class `DeliveryOutcome` (NotSent / Rejected / Succeeded / Unknown), operation-aware retry simulation, pagination origin checks, redirect auth hardening.
- **`OperationSafetyProfile` SSOT** via `route_capabilities` + `operation_safety`; dangerous-profile CI gate (`scripts/check_dangerous_profile_drift.py`).
- **Tier-S facades:** OAuth, payouts, transfers (client signature), verify-payee, unmatched credit transfers, sessions, terminal pairing, Connect balance transfers; delayed routes on payments.
- **Docs:** `docs/API-STABILITY.md`, `docs/release-readiness.md`, `docs/sdd/1.0-readiness/*` program pack; expanded safe-retry guide (Unknown / cancellation / Retry-After).

### Safety

- High-risk writes marked `RouteAccess::ValidatedFacade` with profile invariants (**23/23** metric + coverage report). Coverage gate fails closed below 100%.
- `EntityBalanceTransfer` create bodies omit response-only fields on the wire (same pattern as payouts).
- Model tests: write ≤1 attempt without sticky key; no post-deadline attempt; NonRetryable never retries.

### Chores

- Public-repo production hygiene: GitHub Actions CI (fmt/clippy/test/docs/MSRV/cargo-deny/drift/package), Dependabot, issue/PR templates, Code of Conduct.
- crates.io metadata polish (`categories`, `homepage`, package `exclude` for non-crate artifacts).
- Refreshed production assessment, RC checklist/scorecard/SBOM notes, and docs index for the 0.7.x line.

## [0.7.0] (2026-08-09)

### Features

- Expanded the generated API surface with payout, session, and transfer operations, plus new account and number-format types.
- Added an operation registry, profile-scoped client context, request hooks, route-aware retry classification, and safer retry-budget handling.
- Added atomic webhook replay claims, transport-annotated errors, secret-leak regression tests, property-style parser tests, and fuzz targets.
- Added public API compatibility checks and upstream OpenAPI drift tooling to CI.

### Bug fixes

- Prevented HTTP sends after the retry budget or total deadline has been exhausted.
- Made non-retryable write operations explicit so unsafe writes are not retried automatically.
- Ensured credential rebuilding preserves configured timeouts and user-agent values.
- Zeroized webhook signing secrets when the `zeroize` feature is enabled.

### Chores

- Pinned and digest-verified the upstream Mollie OpenAPI contract, and documented the contract-generation pipeline.
- Added production-readiness, SDK-parity, implementation-baseline, and route-coverage documentation.
- Added architecture and dependency diagrams to the documentation.
- Recorded release scope, timing, and release ledger metadata.

### Breaking changes

- Regenerated the public API against the expanded upstream contract; several generated helpers and types were removed or reshaped, including locale helpers and address-related types.

## [0.6.1](https://github.com/suitsfinance/mollie-rs/releases/tag/mollie-api-rust-0.6.1) (2026-08-03)

### Added

- **Validated write builders:** `CreateCaptureRequired`, `CreateSepaMandateRequired`, `CreatePaymentLinkRequired`.
- **PaymentLinksApi** facade (`client.payment_links()`) with create/get/delete/list_page/list_all.
- **EmptyResponse** for cancel/revoke/delete empty provider bodies (refunds cancel, mandates revoke, payment-link delete).
- **list_all** helpers on payments, refunds, captures, mandates, payment links (budgeted via `PaginationGuard`).
- **CapturesApi::create** / **MandatesApi::create_sepa** validated entry points (keep `create_raw` / `create` for generated bodies).
- Optional **`zeroize`** feature: zero credential secrets on drop.
- Env-gated **live smoke** tests (`tests/live_smoke.rs`, ignored by default).
- Advisory CI job **upstream OpenAPI drift** (non-blocking fetch + compare artifact).
- CI no-default-features checks limited to lib+tests (examples require `app-helpers`).

### Notes

- First crates.io publish on the **0.6** line (registry was still **0.5.0**). Git history already carried a 0.6.0 development cut; **0.6.1** is the published package.

## [0.6.0](https://github.com/suitsfinance/mollie-rs/releases/tag/v0.6.0) (2026-08-03)

> Development / git cut. Prefer **0.6.1** on crates.io.

### Breaking (facade)

- `PaymentsApi::create` / `RefundsApi::create` / `SubscriptionsApi::create` now take **validated builders** (`CreatePaymentRequired`, `CreateRefundRequired`, `CreateSubscriptionRequired`). Use `create_raw` for generated request bodies.
- Write auto-retries require a **sticky** idempotency key (auto UUID alone is not enough).

### Added

- MandatesApi and WebhooksApi (classic parse, Next-gen verify/decode, `get_event`).
- PaginationGuard arbitrary cursor cycle detection (bounded set).
- Specs: `docs/specs/current-state-audit.md`, `domain-facade-contract.md`, `release-0.6-contract.md`.

### Prior 0.5.x foundations (rolled into 0.6 narrative)

- Domain facades for payments/refunds/captures/subscriptions, transport policy, HMAC webhooks, HTTP contracts, blocking cargo-deny, ResponseMetadata.

### Added ΓÇö domain facades and transport matrix (historical 0.5.2 unreleased)

- **Refunds / captures / subscriptions facades:** `client.refunds()`, `client.captures()`, `client.subscriptions()` with create/get/list_page (+ cancel/update where applicable) and request-scoped idempotency.
- **Shared domain helpers:** list limit validation and list-link cursor extraction.
- **HTTP contracts:** 429+Retry-After recovery, no-retry on 400/401, 502/504 recovery, payment-scoped refunds list path.

### Added ΓÇö workflow completeness (post-6a10172)

- **Safer write retries:** auto-retry writes only with a **sticky** caller-bound idempotency key (not auto UUID alone).
- **Backoff:** OS entropy jitter via `getrandom` (deterministic fallback if entropy fails).
- **Pagination:** `AsyncPaginator` / `ItemStream` + two-page cycle detection; `PageCursor::from_list_link`.
- **Payments facade:** `MollieClient::payments()` create/get/list_page with request-scoped keys.
- **Webhooks:** `verify_header` distinguishes missing vs malformed signatures; whitespace-body tests.
- **CI:** `cargo-deny` is blocking (no `continue-on-error`).
- **Specs:** `docs/specs/contract-audit.md`, `baseline-forensics.md`.

### Added ΓÇö production hardening (Phases BΓÇôE foundations)

- **Response metadata:** expanded `ResponseMetadata` (rate-limit limit/remaining/reset, elapsed/attempt/operation hooks, provider error fields) plus `ErrorResponseContext` with bounded body retention.
- **Errors:** `MollieError::MalformedProviderResponse`, `MollieError::WebhookVerification`; helpers `metadata`, `request_id`, `retry_after`, `is_timeout`, `is_authentication_failure`, `provider_code` / `provider_key`; capped invalid payload bytes.
- **Idempotency:** `IdempotencyKey` validated type + `MollieClient::with_idempotency` for request-scoped keys (client-global sticky keys retained, discouraged for unrelated ops).
- **Transport:** `RetryPolicy` (disabled by default; `default_safe` for conservative retries), backoff with `Retry-After`, classification via `RetryClass`; wired into `Client::send` / builder.
- **Webhooks:** `WebhookVerifier` HMAC-SHA256 over raw body (`X-Mollie-Signature`), secret rotation, body limits, `verify_and_decode`; SECURITY.md guidance.
- **Pagination:** `Page`, `PageCursor`, `PaginationGuard` (loop/budget protection).
- **Route capabilities:** `supports_idempotency`, `safe_to_retry`, `retry_class`, `paginated`, `requires_profile_scope`.
- **HTTP contracts:** 503ΓåÆsuccess retry, disabled-retry single attempt, prior auth/idempotency tests.

### Documentation and governance (Phase A)

- README install `0.5`, compatibility tiers, MSRV **1.88**, CI, generation/drift scripts, `app-helpers` feature gate.

### Notes

- Retries remain **opt-in**; payment writes only auto-retry when policy allows and an `Idempotency-Key` is present on the request.
- Classic webhooks still require API refetch; Next-gen requires HMAC verification before trust.
- Domain facades for all payment flows and full async paginators remain follow-ups.
- Target release line: **0.6.0** once this tranche is published.

## [0.5.2](https://github.com/suitsfinance/mollie-rs/releases/tag/v0.5.2)

- Current crates.io / repository package version at time of assessment.
- Changelog history below still records earlier published tags; intermediate `0.4`/`0.5` notes will be backfilled as release notes are formalized.

## [0.3.1](https://github.com/suitsfinance/mollie-rs/compare/v0.2.0...v0.3.1) (2026-07-13)

- Release channel: stable
- Tag: `v0.3.1`

## [0.2.0](https://github.com/suitsfinance/mollie-rs/compare/v0.1.5...v0.2.0) (2026-07-08)

- Release channel: stable
- Tag: `v0.2.0`

## [0.1.5](https://github.com/suitsfinance/mollie-rs/compare/v0.1.4...v0.1.5) (2026-04-15)

- Release channel: stable
- Tag: `v0.1.5`

## [0.1.4](https://github.com/suitsfinance/mollie-rs/compare/v0.1.1...v0.1.4) (2026-03-27)

- Release channel: stable
- Tag: `v0.1.4`

## [0.1.1](https://github.com/suitsfinance/mollie-rs/compare/v0.1.0...v0.1.1) (2025-12-03)

- Release channel: stable
- Tag: `v0.1.1`

## [0.1.0](https://github.com/suitsfinance/mollie-rs/releases/tag/v0.1.0) (2025-10-14)

- Release channel: stable
- Tag: `v0.1.0`
