# Open residuals (hardening program)

Freeze HEAD: `c4e909131a3797c69309c017661628b3a92700d5` · crate `0.7.1` · 2026-08-18

Only residuals that block an honest `1.0.0-rc` or are on the critical path are listed. Closed FINDINGS stay in `docs/sdd/1.0-readiness/FINDINGS.md`.

## P0 — must close before RC

| ID | Plan / Linear | Title | Current state | Exit criteria |
| --- | --- | --- | --- | --- |
| HTTP-001 | SUI-2329 | Unrestricted HTTP client escape hatch | **CLOSED on safe path:** builder `http_client` removed; `configure_http` runs before forced redirect-none / TLS 1.2+ / headers / timeouts; test `configure_http_cannot_reenable_redirects`. Unrestricted transport remains only via `MollieClient::from_generated` / `Client::new_with_client` (documented low-level). | Keep low-level path intentional; no reintroduction of builder inject |
| HTTP-002 | SUI-2329 | Response body / resource limits | **CLOSED:** `ResponseLimits` on `Client` + builder; `routes::response::json` / `read_body_limited` enforce success vs error ceilings; tests at-limit, +1, Content-Length oversize, error-body ceiling. Webhook default remains `DEFAULT_MAX_WEBHOOK_BODY_BYTES` (1 MiB). | Optional: pure chunked (no CL) overflow integration if wiremock allows |
| HTTP-003 | SUI-2329 | Compression / TLS / proxy / base-url policy completeness | **PARTIAL:** base URL HTTPS/loopback policy + TLS 1.2+ + rustls-only tree + `cargo deny` PASS; policy doc `docs/rc/transport-security-policy.md`; proxy credential isolation tests still thin | Proxy isolation / env-proxy tests; examples compile gate re-run |
| TIER-002 | SUI-2361 | Tier-S API stability enforcement incomplete | Tier-S snapshot **blocking**; `cargo-semver-checks` job is `continue-on-error` and swallows failure with `\|\| echo` | Explicit Tier-S vs Tier-G policy in `API-STABILITY.md`; Tier-S path structurally fail-closed |

## P1 — required for honest RC evidence

| ID | Plan / Linear | Title | Current state | Exit criteria |
| --- | --- | --- | --- | --- |
| ENUM-prod | SUI-2337 | Open enums on provider-controlled response fields | `OpenEnum` foundation landed; production enum migration incomplete | Classification + migrations + round-trip tests for provider-controlled enums |
| NULL-prod | SUI-2339 | Tri-state nullable request fields | `NullableField` foundation landed; priority fields not fully migrated | Exact-body tests omit vs null vs value for dueDate and peers |
| PM-unify | SUI-2348 | Payment method representation | Multiple surfaces may still diverge | Unified semantics where appropriate; Billink fixtures |
| SESS-PII | SUI-2350 | Sessions private-beta PII | Maturity-aware work partial | Tier-S does not over-promise; no PII in hooks |
| TERM-403 | SUI-2351 | Terminal pairing 403 structured error | Needs structured propagation proof | Distinct from 429/5xx/timeout/auth/decode |
| REQ-sep | SUI-2353 | High-risk request-model separation | Tier-S request contracts CI (6 ops) | No high-risk write reuses broad response entity |
| PAG-001 | SUI-2328 | Pagination consistency | Streams on key facades; matrix incomplete | Per-domain list_page/stream_*/guard/origin-safe matrix + cycle tests |
| EX-001 | SUI-2331 | Documented workflow compile-proof | Examples compile in CI; coverage script optional | Required workflows mapped to examples |
| REL-001 | SUI-2330 | Live assurance harness | Env-gated live readonly + multi-gate sandbox write exist; **not credentialed run** | Evidence paste for readonly + sandbox write; fail-closed credential gates |
| HOST-001 | SUI-2332 | Hostile RC soak | Static hostile review PASS; live soak NOT RUN | Transport/financial/credential adversarial evidence |

## P2 — valuable before 1.0

| ID | Plan / Linear | Title | Notes |
| --- | --- | --- | --- |
| TEL-001 | SUI-2366 | Runtime drift telemetry | Opt-in, redacted, non-panicking callbacks |
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
