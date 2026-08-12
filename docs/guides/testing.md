# Testing with mollie-rs

| Layer | What to use |
| --- | --- |
| Unit | Builders, `OperationSafetyProfile`, `simulate_retry_loop`, secret-leak suite |
| HTTP contract | WireMock tests under `tests/http_contract.rs` |
| Live readonly | `tests/live_smoke.rs` with `MOLLIE_LIVE_READONLY=1` |
| Sandbox writes | Multi-gate ignored tests (`MOLLIE_TESTMODE_WRITE` + phrase + `test_` key) |
| Fuzz | Targets under `fuzz/` (webhook, money, page cursor, retry-after) |

## Guidance

- Prefer Tier-S facades in application tests.
- Use Tier-G only for contract edge cases that facades intentionally hide.
- Do not disable safety tests to “make green” — fix the invariant instead.
- Live credentials belong in CI secrets or local env, never in the tree.

See `docs/rc/live-test-matrix.md` for the env-gated matrix.
