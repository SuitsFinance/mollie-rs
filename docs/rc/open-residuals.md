# Open residuals (hardening program)

Freeze HEAD: `c4e909131a3797c69309c017661628b3a92700d5` · crate now `0.8.0` (freeze entry `0.7.1`) · 2026-08-18

Only residuals that block an honest `1.0.0-rc` or are on the critical path are listed. Closed FINDINGS stay in `docs/sdd/1.0-readiness/FINDINGS.md`.

## P0 — must close before RC

| ID | Plan / Linear | Title | Current state | Exit criteria |
| --- | --- | --- | --- | --- |
| HTTP-001 | SUI-2329 | Unrestricted HTTP client escape hatch | **CLOSED on safe path:** builder `http_client` removed; `configure_http` runs before forced redirect-none / TLS 1.2+ / headers / timeouts; test `configure_http_cannot_reenable_redirects`. Unrestricted transport remains only via `MollieClient::from_generated` / `Client::new_with_client` (documented low-level). | Keep low-level path intentional; no reintroduction of builder inject |
| HTTP-002 | SUI-2329 | Response body / resource limits | **CLOSED:** `ResponseLimits` on `Client` + builder; `routes::response::json` / `read_body_limited` enforce success vs error ceilings; tests at-limit, +1, Content-Length oversize, error-body ceiling. Webhook default remains `DEFAULT_MAX_WEBHOOK_BODY_BYTES` (1 MiB). | Optional: pure chunked (no CL) overflow integration if wiremock allows |
| HTTP-003 | SUI-2329 | Compression / TLS / proxy / base-url policy completeness | **PARTIAL:** base URL HTTPS/loopback policy + TLS 1.2+ + rustls-only tree + `cargo deny` PASS; policy doc `docs/rc/transport-security-policy.md`; proxy credential isolation tests still thin | Proxy isolation / env-proxy tests; examples compile gate re-run |
| TIER-002 | SUI-2361 | Tier-S API stability enforcement incomplete | **CLOSED:** Tier-S snapshot blocking; `API-STABILITY.md` Tier-S vs Tier-G vs Kernel policy; CI `semver` job fail-closed (`cargo semver-checks check-release`, no swallow). Crate version **0.8.0** for intentional builder break vs crates.io `0.7.1`. | Keep snapshot + semver green on every PR |

## P1 — required for honest RC evidence

| ID | Plan / Linear | Title | Current state | Exit criteria |
| --- | --- | --- | --- | --- |
| ENUM-prod | SUI-2337 | Open enums on provider-controlled response fields | **PARTIAL CLOSED:** `PaymentStatusValue` (`OpenEnum<PaymentStatusKnown>`) + round-trip tests; generated route bodies still use closed `types::PaymentStatus` until generator open-enum support | Expand to other provider-controlled statuses; optional decode adapters on routes |
| NULL-prod | SUI-2339 | Tri-state nullable request fields | **CLOSED for registry peers:** `CreatePaymentRequired.due_date`, `UpdatePaymentRequired.due_date`, `CreatePayoutRequired.description` + `to_write_json` omit/null/value tests | Keep registry in sync; do not blanket Option fields |
| PM-unify | SUI-2348 | Payment method representation | **CLOSED for request surface:** single `PaymentMethod` facade incl. Billink fixture/test; payment-link helpers | Watch OpenAPI pin for new methods |
| SESS-PII | SUI-2350 | Sessions private-beta PII | Maturity-aware work partial | Tier-S does not over-promise; no PII in hooks |
| TERM-403 | SUI-2351 | Terminal pairing 403 structured error | Needs structured propagation proof | Distinct from 429/5xx/timeout/auth/decode |
| REQ-sep | SUI-2353 | High-risk request-model separation | **PARTIAL CLOSED:** Tier-S `to_write_json` allowlisted bodies for payment/payout/update; contracts registry includes `update_payment` + `dueDate` | Generated `PayoutRequest`/`PaymentRequest` types remain dual-shaped; prefer Tier-S builders |
| PAG-001 | SUI-2328 | Pagination consistency | **CLOSED for HAL list facades:** captures/mandates/subscriptions/terminals gained `stream_pages`/`stream_items` (+ subscriptions `list_all`); matrix + intentional residuals in `docs/rc/pagination-matrix.md`; kernel origin/cycle/stream tests green | Keep matrix green; optional baseurl-threaded cursor parse |
| EX-001 | SUI-2331 | Documented workflow compile-proof | Examples compile in CI; coverage script optional | Required workflows mapped to examples |
| REL-001 | SUI-2330 | Live assurance harness | Env-gated live readonly + multi-gate sandbox write exist; **not credentialed run** | Evidence paste for readonly + sandbox write; fail-closed credential gates |
| HOST-001 | SUI-2332 | Hostile RC soak | Static hostile review PASS; live soak NOT RUN | Transport/financial/credential adversarial evidence |

## P2 — valuable before 1.0

| ID | Plan / Linear | Title | Notes |
| --- | --- | --- | --- |
| TEL-001 | SUI-2366 | Runtime drift telemetry | **CLOSED:** `ContractDriftObserver` + client/global attach; emit on unknown `OpenEnum` + off-origin pagination; panic isolation + redaction tests (`docs/rc/contract-drift-telemetry.md`) |
| PERF-001 | Phase 9 | Pool reuse / pagination memory / hook overhead | Record measurements; no nanosecond gates |
| REL-ladder | Phase 11 | `0.8.x` → `1.0.0-rc.1` ladder | Only after acceptance matrix green |

## Explicitly out of scope for this branch

- Redesign of `OperationSafetyProfile` / Tier-G generation architecture
- Expanding operation count as a success metric
- Unrelated feature work during RC branch

## Mapping to FINDINGS.md

| FINDINGS | Residual |
| --- | --- |
| HTTP-001 Partial | HTTP-001/002/003 |
| TIER-002 Partial | TIER-002 |
| HOST-001 Partial | HOST-001 |
| REL-001 Open | REL-001 |
