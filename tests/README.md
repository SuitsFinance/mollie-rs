# Integration Tests

Tests that need a real HTTP boundary, real provider payloads, or a real network
call. Unit and property tests live next to the code they cover in
[`../src/`](../src) (`src/property_tests.rs`, `src/secret_leak_tests.rs`,
`src/capabilities_fixture.rs`, and per-module `#[cfg(test)]` blocks).

**`cargo test` never hits the network and never mutates anything in Mollie.**

| File | Boundary | Network |
| --- | --- | --- |
| `http_contract.rs` | Mock Mollie API via `wiremock` | Local only |
| `postman_all_responses.rs` | Recorded fixtures from Mollie's Postman collections | None |
| `live_smoke.rs` | The real Mollie API | Yes — env-gated and `#[ignore]`d |

## `http_contract.rs`

Proves request *shaping* that fixture-based tests structurally cannot see:
authorization headers, idempotency keys, custom base URLs, and status handling.
This is where you assert "the client actually sent what we think it sent."

## `postman_all_responses.rs`

For every unique error response harvested from Mollie's six Postman collections,
asserts the shared error path end to end:

```text
ErrorResponse → classify_api → MollieError::api → to_envelope()
```

Backed by `fixtures/postman_error_responses.json` (deduped across all
collections). `fixtures/postman_success_response_index.json` indexes success
samples for documentation; error classification is what the global factory owns
and what this suite guards.

Regenerate the fixtures with `python scripts/generate_postman_matrix.py`.

## `live_smoke.rs`

Skipped by default. Two tiers, both requiring an explicit opt-in:

```sh
# Tier 1 — readonly
MOLLIE_LIVE_READONLY=1 MOLLIE_API_KEY=test_... \
  cargo test --test live_smoke -- --ignored --nocapture
```

`MOLLIE_LIVE_SMOKE=1` is still accepted as a legacy alias for readonly. Tier 2
adds testmode writes and requires its own opt-in variable — see the module docs
at the top of `live_smoke.rs` for the current matrix and
[`../docs/rc/live-test-matrix.md`](../docs/rc/live-test-matrix.md) for what a
release run is expected to cover.

Use a `test_` key. Never point these at a live key.

## Fixtures

`fixtures/` holds recorded provider payloads only. Keep them **verbatim** — the
point is that they are Mollie's bytes, not ours. If a fixture needs changing,
re-harvest it rather than editing it by hand, and confirm no real credentials,
customer data, or ids leaked into the recording.
