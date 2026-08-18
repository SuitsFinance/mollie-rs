# Hostile payment / security review (static)

**Scope:** library safety invariants for `mollie-rs` on branch `floris-xlx-hardening` (RC hardening).  
**Mode:** static + automated proof review (no live credentials in this pass).  
**Not claimed:** marketplace 1.0 certification or credentialed live soak.

## Verdict

| Band | Result |
| --- | --- |
| Static hostile review | **PASS with residuals** |
| Live hostile soak | **NOT RUN** (requires `MOLLIE_*` secrets) |
| Live assurance paste | **OPEN** — harness ready; see [`live-assurance-evidence.md`](live-assurance-evidence.md) |
| Honest `1.0.0-rc` claim | **BLOCKED** until Tier 1 + Tier 2 evidence pasted |

## Attack / failure themes reviewed

| Theme | Expected fail-closed behavior | Evidence | Status |
| --- | --- | --- | --- |
| Auth header on cross-origin redirect | Redirects disabled (`Policy::none`) | `tests/http_contract.rs` `does_not_follow_redirect_to_foreign_host`; `docs/rc/hostile-transport-evidence.md` | PASS |
| Write retry without sticky idempotency | ≤1 attempt | `write_without_sticky_idempotency_is_not_auto_retried`; property INV-WRITE-02 | PASS |
| Ambiguous delivery without sticky key | Fail closed / Unknown | kernel + property tests | PASS |
| Pagination evil `next` host | Cursor host rejected | `page_cursor_rejects_evil_next_host` | PASS |
| Secret / credential leakage in Debug/logs | Redaction | `src/secret_leak_tests.rs` | PASS |
| Webhook HMAC forge | Signature required before trust | `webhook_verify` unit tests; `VerifiedWebhook` keeps raw on decode drift | PASS (decode-recover path present) |
| Custom `reqwest::Client` bypass of TLS/redirect policy | Safe builder must not skip last-apply | `configure_http` last-apply; builder `http_client` **removed**; low-level `from_generated` / `new_with_client` only | PASS |
| Silent high-risk write profile drift | CI blocks | `check_dangerous_profile_drift.py` 23/23 | PASS |
| New mutation ops without HR classification | Independent detector | `detect_high_risk_operations.py` | PASS |
| Tier-S request field smuggling via generated body | Prefer validated builders + allowlist registry | `tier-s-request-contracts.yaml` + `check_tier_s_request_contracts.py` | PASS (registry); raw `create_raw` still advanced escape |
| Tier-S public method renames/removals | Snapshot gate | `tier-s-public-api.snapshot` + `check_tier_s_public_api.py` | PASS |
| OpenAPI money/enum/auth drift silent merge | Semantic classifier blocking classes | 17 openapi-drift fixtures; `contract_diff --fail-on-blocking` | PASS |
| Provider history regressions | Corpus fixtures | `tests/provider_history_corpus.rs` | PASS (seed corpus) |
| Live key used for write smoke | Multi-gate + refuse `live_` | `tests/live_smoke.rs` unit gates | PASS (static); live run NOT RUN |
| Sandbox financial write accidents in CI | Ignored + multi-gate | never default CI | PASS |

## Residual risks (do not mark RC until closed or accepted)

1. **Live readonly matrix** not executed without env secrets — paste into [`live-assurance-evidence.md`](live-assurance-evidence.md).  
2. **Sandbox payment create/idempotency** not executed without secrets — same paste pad.  
3. **`create_raw` / generated write bodies** remain available for advanced callers — documented as non-preferred.  
4. **OpenEnum** on more provider statuses beyond payment status still partial (ENUM-prod residual).  
5. Refund/payout/transfer **live write** paths not automated (payment path only).  
6. Local fault-injection chaos server still **not built** (see transport evidence residual).

Closed since earlier draft: builder `http_client` removed; CI `cargo-semver-checks` fail-closed; Tier-S snapshot + workflow example matrix blocking; pagination streams on HAL facades; opt-in contract drift telemetry.

## Commands re-run for this review

```text
python scripts/check_dangerous_profile_drift.py
python scripts/detect_high_risk_operations.py
python scripts/run_openapi_drift_fixtures.py
python scripts/check_tier_s_request_contracts.py
python scripts/check_tier_s_public_api.py
python scripts/check_workflow_examples.py
cargo test --test live_smoke -- --exact write_gate_rejects_live_api_keys classify_auth_and_permission_errors
cargo test --test http_contract
```

## Sign-off

| Role | Status | Date |
| --- | --- | --- |
| Static hostile reviewer (agent) | Conditionally approved for merge-as-foundation | 2026-08-18 |
| Human release owner | Required before 0.8 / RC tag | pending |
| Live evidence owner | Required before RC | pending |

## See also

- [`hostile-transport-evidence.md`](hostile-transport-evidence.md)
- [`live-test-matrix.md`](live-test-matrix.md)
- [`../release-readiness.md`](../release-readiness.md)
