# RC checklist (exceptional 1.0 residual)

| Gate | Command / proof | Status |
| --- | --- | --- |
| High-risk drift | `python scripts/check_dangerous_profile_drift.py` | PASS (23 ops) |
| High-risk coverage | `python scripts/report_high_risk_coverage.py --require-full` | PASS 23/23 |
| Generation inventory | `python scripts/check_generation_reproducibility.py` | PASS 124/124 |
| Unit + contract tests | `cargo test --all-features` | PASS |
| Clippy | `cargo clippy --all-targets --all-features -- -D warnings` | PASS (run in CI) |
| Format | `cargo fmt --all -- --check` | PASS (run in CI) |
| Live smoke | `MOLLIE_LIVE_READONLY=1` ignored tests | Env-gated (not required for library RC) |
| Sandbox writes | multi-gate ignored tests | Env-gated |
| Package | `cargo package --allow-dirty` optional pre-publish | Operator |

## Release band

**NEAR READY / library RC candidate** when CI on PR is green. Not a claim of Mollie production certification.