# Release readiness — mollie-rs 0.7.x → 1.0

**As of:** program v2 (kernel-first) implementation on branch work.  
**Crate:** `mollie-rs` **0.7.0** · MSRV **1.88** · Tier-G **124/124** ops.

## 1.0 band (honest)

| Band | Meaning |
| ---- | ------- |
| **NEAR READY** | Kernel + high-risk Tier-S + drift gates landed; residual assurance/docs polish and sandbox soak remain before calling RC/1.0 |

Not **1.0 READY**: live-write opt-in suite is thin; no crates.io soak of the new public surface; performance budgets not formally measured.

## Primary metric (v2)

```text
high_risk_ops_with_enforced_safety_invariants / total_high_risk_ops
```

| Set | Count | Enforced? |
| --- | ----- | --------- |
| High-risk writes (profile + ValidatedFacade + write retry class) | **16** | **16/16** via `scripts/check_dangerous_profile_drift.py` + unit `validated_write_operations_are_explicit` |
| Kernel proofs (≤1 attempt without sticky; no post-deadline attempt; Unknown sticky gate) | model tests in `src/property_tests.rs` | **Yes** |
| Mock HTTP money/OAuth | `tests/http_contract.rs` | **Yes** (subset) |

**Do not** use raw Tier-S method count as a success metric.

High-risk operation IDs (denominator):

`create_payment`, `create_refund`, `create_capture`, `create_subscription`, `create_payout`, `cancel_payout`, `create_transfer`, `verify_payee`, `oauth_generate_tokens`, `oauth_revoke_tokens`, `payment_create_route`, `create_session`, `terminals_request_pairing_code`, `terminals_revoke_pairing_code`, `match_unmatched_credit_transfer`, `return_unmatched_credit_transfer`.

## Architecture freeze

```text
App → Tier-S facades (justified) → Tier-G Client/routes → Transport kernel
                                         ↑
                              OperationSafetyProfile SSOT
                              (src/route_capabilities.rs + operation_safety.rs)
```

Invariants with automated proof (subset): INV-WRITE-01/02, INV-DEADLINE-01, INV-DELIV-01, INV-HOST-01/PAGE-01 (cursor host), INV-DRIFT-01, INV-PROFILE-01 (single table + CI).

## Test pyramid status

| Layer | Status | Evidence |
| ----- | ------ | -------- |
| Unit | Strong | `cargo test --lib` (~221) |
| Mock HTTP | Strong | `tests/http_contract.rs` |
| Retry model/property | Strong | `property_tests` + `simulate_retry_loop` |
| Fuzz parsers | Present | `fuzz/` targets (CI optional/nightly) |
| Live readonly | Env-gated matrix | `tests/live_smoke.rs` — 13 ops; `MOLLIE_LIVE_READONLY=1` (legacy `MOLLIE_LIVE_SMOKE=1`); see `docs/rc/live-test-matrix.md` |
| Live write | **Multi-gate opt-in** | `MOLLIE_TESTMODE_WRITE` + mutation phrase + `test_` key; payment create/idempotency present; not in default CI |
| Drift CI | Blocking | capabilities sync + `check_dangerous_profile_drift.py` + registry commit |

## CI gates (blocking)

Workflow: [`.github/workflows/ci.yml`](../.github/workflows/ci.yml)

1. `python scripts/check_generation_reproducibility.py`
2. `python scripts/check_dangerous_profile_drift.py`
3. `python scripts/export_operation_registry.py` + committed `docs/registries/operation-registry.yaml`
4. fmt / clippy `-D warnings` / test feature matrix / docs / examples compile
5. Upstream pin digest (`fetch_upstream_openapi.py`); inventory drift is advisory (exit 2)
6. `cargo deny` + `cargo package` dry-run

## Residual before RC / 1.0

| Item | Priority | Notes @ RC freeze work |
| ---- | -------- | ---- |
| Credentialed live readonly matrix run + evidence paste | P1 | Suite expanded; needs `MOLLIE_LIVE_READONLY=1` run |
| Sandbox payment smoke credentialed run | P1 | Multi-gate implemented |
| Refund/payout/transfer/OAuth sandbox + limitations | P1 | Payment path only so far |
| Hostile security review sign-off doc | P1 | Kernel proofs exist; formal `docs/rc/hostile-security-review.md` pending |
| Retry-After HTTP-date integration (currently ignored, non-panic) | P2 | Delta-seconds covered |
| Focused perf: pool reuse / pagination memory | P2 | |
| crates.io pre-release soak (`0.8.0` ladder) | P1 | |
| Examples compile gate | **Done** | CI `cargo check --examples --all-features` |

## Release ladder

| Line | Intent |
| ---- | ------ |
| **0.7.x** | Truth + kernel + drift (this line) |
| **0.8.x** | Connect + OAuth + money Tier-S publicized |
| **0.9 / rc** | Assurance + docs complete |
| **1.0** | Only if `docs/sdd/1.0-readiness/01-requirements.md` gates pass |

## Related docs

- `docs/API-STABILITY.md` — public API posture  
- `docs/guides/safe-payment-retry.md` — Unknown / cancel / sticky keys  
- `docs/sdd/1.0-readiness/*` — program SSOT  
- `docs/production-checklist.md` — operator checklist  
