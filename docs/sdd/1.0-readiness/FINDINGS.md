# Finding register (1.0-readiness)

| ID | Sev | Title | Status | Evidence | Target phase |
| --- | --- | --- | --- | --- | --- |
| KER-001 | P0 | Delivery outcome Unknown missing | **Closed** | `transport/delivery.rs`, `MollieError::delivery_outcome` | Phase 2 |
| KER-002 | P0 | Redirect may forward Authorization | **Closed** | `Policy::none()` + http_contract | Phase 2 |
| KER-003 | P0 | Pagination next URL no origin allowlist | **Closed** | `PageCursor::from_list_link*` | Phase 2 |
| KER-004 | P0 | Incomplete OperationSafetyProfile SSOT | **Closed v1** | `operation_safety.rs` | Phase 2 |
| KER-005 | P0 | Retry state-machine proofs incomplete | **Closed** | `simulate_retry_loop` + property_tests | Phase 2 |
| KER-006 | P1 | Cancel-after-transmit docs | **Closed** | guide + error docs | Phase 2 |
| KER-007 | P0 | Connect concurrency stress | **Closed** | `concurrent_scoped_credentials_do_not_cross_wire` | Phase 2/3 |
| FAC-001 | P1 | High-risk Tier-G only (payouts/transfers/oauth/…) | **Closed** | `domain/payouts.rs`, `transfers.rs`, `oauth.rs`, … | Phase 4–5 |
| DOC-001 | P0 | Stale 100-op docs | Closed | route-coverage + drift regen + parity STALE | Phase 0 |
| CI-001 | P1 | Dangerous semantic drift hard-fail | **Closed** | `.github/workflows/ci.yml` + `check_dangerous_profile_drift.py` | Phase 7 |
| HR-001 | P1 | High-risk denominator incomplete vs mission set | **Closed** | CI `HIGH_RISK_WRITES`=23; `high_risk_coverage` 23/23; `docs/registries/high-risk-coverage.*` | Prog P2 |
| HR-002 | P1 | Connect balance transfer Tier-G only | **Closed** | `domain/connect_balance_transfers.rs` + ValidatedFacade | Prog P3 |
| KER-RA-01 | P2 | Retry-After HTTP-date ignored | **Closed** | `metadata.rs` RFC2822 via chrono | Prog P4 |
| PAG-001 | P2 | No stream_pages/stream_items | **Closed** | payments/payouts/refunds/payment_links/connect/UCT + `domain/common` | Prog P5 |
| DOC-GUIDE-01 | P1 | Production guides 3/12 mission | **Closed** | `docs/guides/*` (12 guides) | Prog P6 |
| REL-001 | P1 | RC checklist open (hostile/package/live) | **Closed (local gates)** | cargo test/clippy + drift/coverage; live smoke still env-gated | Prog P7 |
| REL-002 | P2 | SBOM/provenance absent | **Closed (notes)** | `docs/rc/sbom-notes.md` cargo-based SBOM path | Prog P7 |

See `00-current-baseline.md` (freeze SHA `e3358d2`) and `16-exceptional-1.0-program.md`. Residual exceptional-1.0 program phases 2–7 implemented on branch.
