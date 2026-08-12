# SDD 10 — Testing strategy

## Pyramid

| Layer | Focus | Default CI |
| ----- | ----- | ---------- |
| Unit | validators, money, outcomes, redaction, capabilities | **Yes** |
| Mock HTTP | path/method/headers/body (`tests/http_contract.rs`) | **Yes** |
| Model/property | retry engine sequences (`src/property_tests.rs`) | **Yes** |
| Fuzz | webhook/error/URL/money parsers (`fuzz/`) | Optional / nightly |
| trybuild | public secret types if valuable | Optional |
| Live readonly | `tests/live_smoke.rs` | **No** (env gate) |
| Live write | future opt-in only | **Never default** |

## Retry model sequences (required)

Simulated via `simulate_retry_loop` + `AttemptEvent`:

```text
connect_fail | timeout | 429 | 503 | success | deadline_exhausted
```

Proven invariants:

| ID | Property |
| -- | -------- |
| INV-WRITE-02 | financial/idempotent write without sticky key → attempts <= 1 |
| INV-DEADLINE-01 | no attempt begins after deadline marker |
| INV-DELIV-01 | timeout → Unknown; sticky gate for write retry |
| NonRetryable | never retries even with sticky |
| SafeRead | retries transient failures until success/deadline |

## Live policy

```text
MOLLIE_LIVE_SMOKE=1 MOLLIE_API_KEY=test_... cargo test --test live_smoke -- --ignored
```

Prefer test-mode keys. Default `cargo test` must not move real money.

## Acceptance

- [x] Matrix tracked in `docs/release-readiness.md`
- [x] Retry model tests present and extended
- [x] Drift gates in CI generation job
- [ ] Live-write opt-in suite expanded (residual for RC)
