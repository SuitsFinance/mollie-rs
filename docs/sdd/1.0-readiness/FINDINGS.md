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

See `00-baseline.md` for full matrix.
