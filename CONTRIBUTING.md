# Contributing to mollie-rs

Thank you for improving the Mollie Rust SDK.

## Development setup

1. Install Rust **1.88+** (see `rust-toolchain.toml` and `docs/compatibility.md`).
2. Clone the repository and run:

```sh
cargo test --all-targets
cargo test --doc
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all
```

Optional Python tooling for generation/drift:

```sh
pip install pyyaml
python scripts/check_generation_reproducibility.py
python scripts/report_api_drift.py --write docs/api-drift-report.md
```

## Architecture expectations

- Prefer the **facade** (`MollieClient`, validators, envelopes) for application-facing APIs.
- Keep **generated** code reproducible via `scripts/generate_openapi_client.*`.
- Do **not** enable automatic retries for non-idempotent writes.
- Webhook signature verification must use **raw request bytes**, never re-serialized JSON.
- Do not log secrets. Credentials must remain redacted in `Debug`.
- Do not use floating-point for money.

Read `docs/assessment-production-sdk.md` and `docs/compatibility.md` before large changes.

## Pull requests

- Add or update tests for every behavior change.
- Update docs for every public API change.
- Keep commits focused; avoid drive-by refactors.
- Run the generation checks if you touch `specs-3.0.yaml` or the generator.

## Public API checks

CI runs `cargo-semver-checks` against the last crates.io release to catch
accidental public API breakage. Intentional 0.x breaks must be called out in
`CHANGELOG.md` and the PR description.

## Fuzzing

Parser / signature targets live under `fuzz/` (nightly + `cargo-fuzz`). CI only
**builds** them; for local runs:

```sh
rustup install nightly
cargo install cargo-fuzz
cd fuzz
cargo +nightly fuzz run webhook_signature -- -runs=10000
```

See `fuzz/README.md`.

## Code of conduct

Participation is governed by [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md).

## Security

Report vulnerabilities privately per `SECURITY.md` (GitHub Security Advisories preferred). Do not open public issues for undisclosed security bugs.

## Public repository hygiene

- Never commit `.env`, live keys, OAuth tokens, or webhook secrets.
- Keep examples and tests on placeholder `test_xxxx…` / fixture IDs only.
- Prefer focused PRs; run the checklist in the pull request template.
