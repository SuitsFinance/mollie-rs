# Hostile transport evidence (kernel already present)

Recorded against RC baseline HEAD in [`baseline.md`](baseline.md).

These are **not** new architecture — existing automated proofs. Formal hostile **security review** sign-off remains `docs/rc/hostile-security-review.md`.

| Invariant | Test / gate | Status |
| --- | --- | --- |
| Write without sticky ≤1 attempt | `tests/http_contract.rs` `write_without_sticky_idempotency_is_not_auto_retried`; `property_tests` INV-WRITE-02 | Present |
| Sticky write may retry 503 | `write_with_sticky_idempotency_retries_503` | Present |
| Deadline no leftover send | `retry_budget_does_not_send_leftover_attempt_after_deadline` | Present |
| Cross-origin redirect not followed w/ Auth | `does_not_follow_redirect_to_foreign_host` (`redirect::Policy::none`) | Present |
| Pagination evil next host rejected | `page_cursor_rejects_evil_next_host` | Present |
| Connect scoped credentials | `concurrent_scoped_credentials_do_not_cross_wire` | Present |
| Retry-After delta-seconds | `retries_429_with_retry_after_then_succeeds`; metadata unit | Present |
| Retry-After HTTP-date | `ignores_non_numeric_retry_after` — **ignored, no panic** | Residual: optional parse |
| Local fault-injection drop/reset server | Full chaos matrix from master plan §13 | **Not built** as dedicated server |
| Secret leak | `src/secret_leak_tests.rs` | Present |

## Command

```bash
cargo test --test http_contract
cargo test --lib property_tests
```
