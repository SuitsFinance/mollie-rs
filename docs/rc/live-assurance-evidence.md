# Live assurance evidence (REL-001)

**Harness:** [`tests/live_smoke.rs`](../../tests/live_smoke.rs)  
**Matrix:** [`live-test-matrix.md`](live-test-matrix.md)  
**Hostile static:** [`hostile-security-review.md`](hostile-security-review.md) · [`hostile-transport-evidence.md`](hostile-transport-evidence.md)

This document is the **runbook + evidence paste pad** for credentialed live tiers.
Default CI never enables these tiers. Secrets must never be committed.

## Gate design (always fail-closed)

| Control | Behavior | Automated proof (no network) |
| --- | --- | --- |
| Readonly opt-in | Requires `MOLLIE_LIVE_READONLY=1` or legacy `MOLLIE_LIVE_SMOKE=1` | Ignored tests; gate assert message |
| Write multi-gate | Requires `MOLLIE_TESTMODE_WRITE=1` **and** exact phrase `MOLLIE_ALLOW_MUTATION=I_UNDERSTAND_THIS_MUTATES_MOLLIE` **and** `test_` key (or explicit OAuth override) | `write_gate_rejects_live_api_keys` |
| Live keys on writes | `live_` API keys refused | Same unit test |
| Destructive | Tier 2 **and** `MOLLIE_DESTRUCTIVE_SMOKE=1` | Gate helper only; cancel path behind flag |
| Auth failure class | 401 fails the suite (not “account limit”) | `classify_auth_and_permission_errors` |
| Account limits | 403 / 404 / 410 / 422 accepted as env limits on readonly | Same unit test + matrix table |

```bash
# Always-on (CI-safe)
cargo test --test live_smoke -- --exact \
  write_gate_rejects_live_api_keys \
  classify_auth_and_permission_errors
```

## How to run (operator)

### Tier 1 — readonly

```bash
export MOLLIE_LIVE_READONLY=1
export MOLLIE_API_KEY=test_...   # or OAuth env per MollieClient::from_env
cargo test --test live_smoke -- --ignored --nocapture
```

Expect: each `live_*_readonly` either **Succeeded** or an accepted account-limit class.
**Fail** on 401 or transport/5xx/`SdkOrTransportFailure`.

### Tier 2 — sandbox payment write

```bash
export MOLLIE_TESTMODE_WRITE=1
export MOLLIE_ALLOW_MUTATION=I_UNDERSTAND_THIS_MUTATES_MOLLIE
export MOLLIE_API_KEY=test_...
cargo test --test live_smoke sandbox_payment -- --ignored --nocapture
```

Expect:

- `sandbox_payment_create` — create + get (+ optional cancel only if Tier 3)
- `sandbox_payment_idempotency` — two creates with same sticky key → same payment id

### Tier 3 — destructive (manual only)

```bash
export MOLLIE_DESTRUCTIVE_SMOKE=1
# plus all Tier 2 vars
cargo test --test live_smoke sandbox_payment_create -- --ignored --nocapture
```

## Evidence paste template

Copy a block per run. Do **not** paste API keys, tokens, or full payment payloads with PII.

### Session status (this branch)

| Field | Value |
| --- | --- |
| Branch | `floris-xlx-hardening` |
| Date (UTC) | 2026-03-26 |
| Operator | agent (autopilot) |
| Credentials available | **No** |
| Tier 1 readonly | **NOT RUN** |
| Tier 2 sandbox write | **NOT RUN** |
| Tier 3 destructive | **NOT RUN** |
| Always-on gate tests | **RUN** — see command output below |
| Hostile transport suite | **RUN** — `cargo test --test http_contract` (agent session) |

### Always-on gate proof (fill when re-run)

```text
cargo test --test live_smoke -- --exact write_gate_rejects_live_api_keys classify_auth_and_permission_errors
running 2 tests
test write_gate_rejects_live_api_keys ... ok
test classify_auth_and_permission_errors ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 17 filtered out

cargo test --test http_contract
running 23 tests
test result: ok. 23 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Tier 1 paste (fill when credentialed)

```text
HEAD:
Date (UTC):
Operator:
Key kind: test_ | oauth (never paste the secret)
Command:
Summary table:
  live_methods_readonly: Succeeded | PermissionDenied | ...
  live_payments_readonly:
  live_profiles_readonly:
  live_current_profile_readonly:
  live_balances_readonly:
  live_settlements_readonly:
  live_organizations_readonly:
  live_permissions_readonly:
  live_refunds_readonly:
  live_refunds_facade_readonly:
  live_captures_facade_readonly:
  live_payouts_readonly:
  live_business_accounts_readonly:
  live_terminals_readonly:
  live_webhooks_readonly:
Overall: PASS | FAIL
Notes (no secrets):
```

### Tier 2 paste (fill when credentialed)

```text
HEAD:
Date (UTC):
Operator:
Key kind: test_ only
Command:
sandbox_payment_create: PASS | FAIL — payment id prefix only (e.g. tr_…)
sandbox_payment_idempotency: PASS | FAIL — same id on replay: yes/no
Overall: PASS | FAIL
Notes (no secrets):
```

## Residual (honest RC)

| ID | State | Blocks RC? |
| --- | --- | --- |
| Harness + multi-gate + matrix docs | **CLOSED** | No |
| Credentialed Tier 1 paste | **OPEN** until human/operator run | **Yes** for honest RC claim |
| Credentialed Tier 2 paste | **OPEN** until human/operator run | **Yes** for honest RC claim |
| Refund/payout/transfer live write automation | **Deferred** (matrix “Not yet automated”) | No for payment-path RC if Tier 2 payment passes |

## See also

- [`../guides/testing.md`](../guides/testing.md)
- [`../production-checklist.md`](../production-checklist.md)
