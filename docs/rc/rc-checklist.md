# RC checklist (exceptional 1.0 residual)

Library release-candidate gates for the **safety-invariant** band. This is not Mollie product certification.

| Gate | Command / proof | Status |
| --- | --- | --- |
| High-risk drift | `python scripts/check_dangerous_profile_drift.py` | PASS (23 ops) |
| High-risk coverage | `python scripts/report_high_risk_coverage.py --require-full` | PASS 23/23 |
| Generation inventory | `python scripts/check_generation_reproducibility.py` | PASS 124/124 |
| Unit + contract tests | `cargo test --all-features` | PASS (local) |
| Clippy | `cargo clippy --all-targets --all-features -- -D warnings` | PASS (local) |
| Format | `cargo fmt --all -- --check` | CI |
| Docs build | `cargo doc --no-deps --all-features` | CI |
| Live smoke | `MOLLIE_LIVE_READONLY=1` ignored tests | Env-gated |
| Sandbox writes | multi-gate ignored tests | Env-gated |
| Package dry-run | `cargo package` | CI |

## Band

**NEAR READY / library RC candidate** when GitHub Actions on the PR is green. Remaining P1 items are listed in `docs/release-readiness.md`.

## Artifacts

- [`1.0-scorecard.md`](1.0-scorecard.md)
- [`sbom-notes.md`](sbom-notes.md)
- [`live-test-matrix.md`](live-test-matrix.md)
- [`../registries/high-risk-coverage.md`](../registries/high-risk-coverage.md)
