# Route Examples

One runnable binary per generated SDK route method — 126 in total, plus a shared
`support/` module.

> **Generated — do not edit by hand.** Every `examples/<method>.rs` starts with
> `// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.` and is produced by
> `scripts/route_examples.py`. Hand edits are overwritten on the next
> regeneration. Change the generator, or `examples/support/mod.rs`, instead.

## What they are for

These serve two purposes:

1. **Compile-checked documentation** — CI runs `cargo check --examples --all-features`,
   so an example that stops compiling is a signal that a route's request or
   response types changed.
2. **Live probes** — run against Mollie with real credentials to record what a
   given key, token, or profile actually returns for each route.

## Running one

```sh
cargo run --example create_payment
```

Credentials come from the environment (a `.env` file is loaded automatically):

| Variable | Purpose |
| --- | --- |
| `MOLLIE_API_KEY` | API key auth (`test_…` / `live_…`) |
| `MOLLIE_OAUTH_ACCESS_TOKEN` | OAuth access token auth |
| `MOLLIE_TESTMODE` | Force `testmode` on/off for token auth |

Missing credentials are treated as a **local skip**, not a failure, so
`cargo run --example …` is safe on a fresh checkout.

## Fixture overrides

Most routes need a resource id. Every example shares one CLI/env surface
(defined once in `support/mod.rs`), so the same value works everywhere:

```sh
PAYMENT_ID=tr_xxx cargo run --example get_payment
cargo run --example get_payment -- --payment-id tr_xxx
```

CLI flags take precedence over environment variables. Run any example with
`--help` to list the fixtures it accepts.

## Logs and the support matrix

Each run appends a timestamped entry to `logs/<example>.log` at the crate root,
then rebuilds [`docs/example-support-matrix.md`](../docs/example-support-matrix.md)
from the latest entry of every log. Examples with no log yet are reported as
`untested`.

To rebuild the matrix offline, without calling Mollie:

```sh
python scripts/rebuild_example_support_matrix.py
```

## Regenerating

```sh
sh scripts/generate_openapi_client.sh   # regenerate client + examples
sh scripts/check_route_examples.sh      # verify every route has an example
```

See [Regenerating the Client](../README.md#regenerating-the-client) for the full
pipeline.
