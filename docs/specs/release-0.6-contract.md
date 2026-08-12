# Release 0.6.0 contract

## Decision

Ship **0.6.0** as a real minor: payment-domain facades + transport/webhook foundations.

## Included

- Domain facades: payments, refunds, captures, subscriptions, mandates, webhooks
- Validated create builders on payments/refunds/subscriptions
- Sticky-key-only write retries; default retries off
- HMAC webhooks; MissingSignature vs MalformedSignature
- AsyncPaginator with set-based cycle detection
- Blocking cargo-deny; MSRV 1.88; generation checks

## Explicitly not included

- Full live Mollie e2e in CI
- Upstream OpenAPI auto-merge
- 1.0 stability freeze
- Zeroizing secret storage (optional future feature)
- Complete Connect / balance product facades

## Gates (must pass before crates.io publish)

| Gate | Command / check |
| --- | --- |
| A | `cargo test --all-targets` (+ no-default-features) |
| B | `cargo clippy … -D warnings`, `fmt --check` |
| C | MSRV 1.88 |
| D | `python scripts/check_generation_reproducibility.py` |
| E | `cargo deny check` |
| F | HTTP contract suite |
| G | version truth Cargo/README/SECURITY/CHANGELOG |
| H | `cargo package --allow-dirty --no-verify` |
| I | live smoke optional / credential-blocked |

## Semver notes for 0.6

- Facade `create` methods now prefer validated builders (`create_raw` for generated bodies).
- Write auto-retry requires sticky idempotency keys.
