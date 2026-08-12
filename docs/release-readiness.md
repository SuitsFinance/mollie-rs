# Release readiness — mollie-rs 0.7.x → 1.0

**As of:** residual exceptional-1.0 program (public cleanup).  
**Crate:** `mollie-rs` **0.7.0** · MSRV **1.88** · Tier-G **124/124** ops.

## 1.0 band (honest)

| Band | Meaning |
| ---- | ------- |
| **NEAR READY** | Kernel + high-risk Tier-S + drift gates landed; residual live soak and formal hostile review remain before calling RC/1.0 |

Not **1.0 READY**: live-write opt-in suite is thin; no crates.io soak of the newest public surface; performance budgets not formally measured.

## Primary metric

```text
high_risk_ops_with_enforced_safety_invariants / total_high_risk_ops
```

| Set | Count | Enforced? |
| --- | ----- | --------- |
| High-risk writes (profile + ValidatedFacade + write retry class) | **23** | **23/23** via `check_dangerous_profile_drift.py` + `report_high_risk_coverage.py --require-full` + `high_risk_coverage` unit |
| Kernel proofs (<=1 attempt without sticky; no post-deadline attempt; Unknown sticky gate) | model tests in `src/property_tests.rs` | **Yes** |
| Mock HTTP money/OAuth | `tests/http_contract.rs` | **Yes** (subset) |

**Do not** use raw Tier-S method count as a success metric.

Frozen high-risk operation IDs: see `src/operation_safety.rs` (`HIGH_RISK_WRITE_OPERATION_IDS`) and `docs/registries/high-risk-coverage.md` (23 ops, including cancels, customer payment, and Connect balance transfer).

## Architecture freeze

```text
App -> Tier-S facades (justified) -> Tier-G Client/routes -> Transport kernel
                                         ^
                              OperationSafetyProfile SSOT
                              (src/route_capabilities.rs + operation_safety.rs)
```

Invariants with automated proof (subset): INV-WRITE-01/02, INV-DEADLINE-01, INV-DELIV-01, INV-HOST-01/PAGE-01 (cursor host), INV-DRIFT-01, INV-PROFILE-01 (single table + CI).

## Test pyramid status

| Layer | Status | Evidence |
| ----- | ------ | -------- |
| Unit | Strong | `cargo test --lib` |
| Mock HTTP | Strong | `tests/http_contract.rs` |
| Retry model/property | Strong | `property_tests` + `simulate_retry_loop` |
| Fuzz parsers | Present | `fuzz/` targets (CI optional/nightly) |
| Live readonly | Env-gated matrix | `tests/live_smoke.rs`; `MOLLIE_LIVE_READONLY=1`; see `docs/rc/live-test-matrix.md` |
| Live write | **Multi-gate opt-in** | `MOLLIE_TESTMODE_WRITE` + mutation phrase + `test_` key; not in default CI |
| Drift CI | Blocking | capabilities sync + dangerous profile drift + high-risk coverage + registry commit |

## CI gates (blocking)

Workflow: [`.github/workflows/ci.yml`](../.github/workflows/ci.yml)

1. `python scripts/check_generation_reproducibility.py`
2. `python scripts/check_dangerous_profile_drift.py`
3. `python scripts/report_high_risk_coverage.py --require-full`
4. `python scripts/export_operation_registry.py` + committed `docs/registries/operation-registry.yaml`
5. fmt / clippy `-D warnings` / test feature matrix / docs / examples compile
6. Upstream pin digest (`fetch_upstream_openapi.py`); inventory drift is advisory (exit 2)
7. `cargo deny` + `cargo package` dry-run

## Residual before RC / 1.0

| Item | Priority | Notes |
| ---- | -------- | ---- |
| Credentialed live readonly matrix run + evidence paste | P1 | Suite expanded; needs `MOLLIE_LIVE_READONLY=1` run |
| Sandbox payment smoke credentialed run | P1 | Multi-gate implemented |
| Refund/payout/transfer/OAuth sandbox + limitations | P1 | Payment path only so far |
| Hostile security review sign-off doc | P1 | Kernel proofs exist; formal `docs/rc/hostile-security-review.md` pending |
| Retry-After HTTP-date integration | **Done** | Delta-seconds + HTTP-date; budget-capped |
| Focused perf: pool reuse / pagination memory | P2 | |
| crates.io pre-release soak (`0.8.0` ladder) | P1 | |
| Examples compile gate | **Done** | CI `cargo check --examples --all-features` |

## Release ladder

See `docs/sdd/1.0-readiness/14-release-plan.md` and `docs/rc/rc-checklist.md`.

## See also

- [`API-STABILITY.md`](API-STABILITY.md)
- [`registries/high-risk-coverage.md`](registries/high-risk-coverage.md)
- [`guides/README.md`](guides/README.md)
- [`rc/1.0-scorecard.md`](rc/1.0-scorecard.md)
