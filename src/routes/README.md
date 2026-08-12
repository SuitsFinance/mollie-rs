# Generated Route Modules

Machine-generated from `/specs-3.0.yaml` by
`scripts/generate_openapi_client.sh`. **Do not edit by hand** — the contract gate
in CI regenerates this directory and fails the build on any diff.

## Shape

Route methods are **inherent methods on `crate::Client`**, not on per-group
structs. Each file here contributes one `impl Client` block for one OpenAPI tag:

```rust
let response = client.create_payment(None, &body).await?;
```

Files map 1:1 to route groups — `payments.rs`, `refunds.rs`, `customers.rs`,
`settlements.rs`, `oauth.rs`, and so on (29 groups covering 124 operations).

Two files are not route groups:

| File | Role |
| --- | --- |
| `operations.rs` | Per-operation metadata used by transport and safety policy. |
| `response.rs` | Shared response decoding for every generated call. |

## What this layer does and does not do

This layer is deliberately thin. It shapes the request, sends it, and decodes
the response.

It does **not**:

- validate inputs beyond what the types enforce,
- choose idempotency keys,
- decide whether a call may be retried,
- apply business defaults.

Those belong to [`../domain/`](../domain) and [`../transport/`](../transport). If
you find yourself wanting to add one here, you are in the wrong layer — and the
next regeneration would delete it anyway.

## Adding or changing a route

A route exists here because it exists in the pinned spec. To add one:

1. Confirm it is present in the upstream snapshot (see [`/specs/`](../../specs)).
2. Re-pin and re-adapt if needed.
3. Run `sh scripts/generate_openapi_client.sh`.
4. Run `sh scripts/check_route_examples.sh` — every route needs an example.

The operation set is cross-checked against `../route_capabilities.rs` and
`docs/registries/operation-registry.yaml`; all three must agree.

## Using a route directly

Generated methods are the escape hatch when a domain facade does not cover your
case. That is supported — but you then own the concerns the facade would have
handled: input validation, idempotency keys on writes, and retry safety. Read
[`../transport/README.md`](../transport/README.md) before retrying anything that
moves money.
