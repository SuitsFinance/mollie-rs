# Finding register (1.0-readiness)

| ID | Sev | Title | Status | Evidence | Target phase |
| --- | --- | --- | --- | --- | --- |
| KER-001 | P0 | Delivery outcome Unknown missing | **Closed** | `transport/delivery.rs` | — |
| KER-002 | P0 | Redirect may forward Authorization | **Closed** | `Policy::none()` | — |
| KER-003 | P0 | Pagination next URL no origin allowlist | **Closed** | `PageCursor` | — |
| KER-004 | P0 | Incomplete OperationSafetyProfile SSOT | **Closed v1** | `operation_safety.rs` + risk/exposure methods | — |
| FAC-001 | P1 | High-risk Tier-G only | **Closed** | `domain/*` 23/23 | — |
| HR-001 | P1 | High-risk denominator incomplete | **Closed** | 23/23 + `detect_high_risk_operations.py` | — |
| DRIFT-001 | P0 | Semantic OpenAPI classifier missing | **Closed** | `scripts/contract_diff` + fixtures | drift-1 |
| DRIFT-002 | P0 | Miniature drift fixtures missing | **Closed** | `tests/fixtures/openapi-drift/*` | drift-2 |
| DRIFT-003 | P1 | Contract graph / replacement | **Closed** | `build_contract_graph.py`, SchemaReplacement | drift-3 |
| DRIFT-004 | P1 | Approved deltas registry | **Closed** | `docs/registries/approved-contract-deltas.yaml` | drift-2 |
| ENUM-001 | P0 | OpenEnum missing | **Closed** | `src/open_enum.rs` | drift-4 |
| NULL-001 | P1 | Tri-state field helper | **Closed** | `src/nullable_field.rs` + field-semantics.yaml | drift-4 |
| PROF-001 | P0 | OperationRisk/Exposure | **Closed** | derived methods on `RouteCapability` | drift-5 |
| PROF-002 | P0 | Independent mutation discovery | **Closed** | `detect_high_risk_operations.py` | drift-5 |
| PROF-003 | P1 | Method/issuer capability risk | **Closed** | `PaymentCapabilityMutation` | drift-5 |
| WHK-001 | P1 | VerifiedWebhook recover path | **Closed** | `webhook_verify::VerifiedWebhook` | drift-7 |
| HTTP-001 | P0 | Safe builder http_client footgun | **Partial** | `configure_http` + deprecated `http_client` | drift-7 |
| TIER-001 | P0 | Tier-S request allowlists | **Partial** | `tier-s-request-contracts.yaml` seed | drift-6 |
| TIER-002 | P0 | Blocking semver + API snapshot | **Open** | CI still advisory semver | drift-6 |
| PROV-001 | P1 | Provenance / repin | **Partial** | `upstream-baseline.yaml` + `repin_upstream_openapi.py` | drift-8 |
| CANARY-001 | P3 | Official SDK canary | **Open** | — | drift-8 |
| CORPUS-001 | P1 | provider_history | **Partial** | `tests/fixtures/provider_history` + corpus test | drift-8 |
| REL-001 | P1 | Live/sandbox/hostile/RC | **Open** | residual | drift-9 |
| SDD-001 | P1 | Baseline honesty @ 98025b9 | **Closed** | `docs/audits/2026-08-mollie-contract-delta.md` | drift-0 |

**STALE:** Earlier “residual exceptional 1.0 phases 2–7 closed ⇒ drift-ready” claims. HR facades landed; drift stack landed starting 2026-08-18 program.
