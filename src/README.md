# Crate Source Layout

`mollie-rs` is a typed Mollie API client. The source is split into a **generated
lower half** and a **handwritten upper half**, and the boundary matters: the
generated half is overwritten wholesale by `scripts/generate_openapi_client.sh`,
so anything you add there is lost.

## Generated — do not edit

| Path | Produced from |
| --- | --- |
| `types.rs` | `/specs-3.0.yaml` (very large; ~10 MB) |
| `routes/` | `/specs-3.0.yaml` — one module per route group |
| `route_capabilities.rs` | `/specs-3.0.yaml` via `scripts/generate_route_capabilities.py` |

CI enforces this: the contract gate regenerates and fails on any diff. To change
generated code, change the spec or the generator.

## Subdirectories

| Path | Role |
| --- | --- |
| [`routes/`](routes) | Generated route methods — thin, exhaustive, 1:1 with the OpenAPI operations. |
| [`domain/`](domain) | Handwritten facades over those routes, adding validation and safe defaults. |
| [`transport/`](transport) | Retry, rate-limit, deadline, and delivery-outcome policy. |

## Top-level modules

**Entry points and client construction**

| Module | Purpose |
| --- | --- |
| `lib.rs` | Crate root, public re-exports, `prelude`. |
| `client.rs` | Ergonomic client construction. |
| `auth.rs` | Auth helpers (API key and OAuth access token). |
| `env.rs` | Process environment access and optional dotenv loading. |
| `tracing_config.rs` | Tracing subscriber setup for applications and examples. |

**Errors and envelopes**

| Module | Purpose |
| --- | --- |
| `error.rs` | The facade error type. |
| `error_catalog.rs` | Stable codes, keys, and message keys for success and error envelopes. |
| `factory.rs` | Shared success/error factories for application-facing envelopes. |
| `envelope.rs` | Response-envelope helpers for generated route calls. |
| `metadata.rs` | Operational metadata lifted out of HTTP headers and status. |
| `empty.rs` | Explicit modelling of 204 / empty-body responses. |

**Validated value types** — parse-don't-validate wrappers that keep malformed
values out of request bodies

`money.rs` · `ids.rs` · `country_code.rs` · `locale.rs` · `datetime.rs` ·
`phone_number.rs` · `address.rs` · `payment_method.rs` · `pagination.rs`

**Request construction and safety**

| Module | Purpose |
| --- | --- |
| `write_requests.rs` | Validated builders for write bodies. |
| `create_payment.rs` | Local validation of the three required create-payment fields. |
| `idempotency.rs` | Request-scoped idempotency keys for writes. |
| `operation_safety.rs` | Single source of truth for per-operation safety, shared by transport and facades. |
| `hooks.rs` | Narrow request lifecycle hooks for observability and test doubles. |

**Webhooks**

| Module | Purpose |
| --- | --- |
| `webhook.rs` | Classic form-urlencoded callbacks. |
| `webhook_verify.rs` | Next-generation HMAC-SHA256 signature verification. |
| `integration.rs` | Application integration boundaries for webhook processing. |

**Tests compiled into the crate**

`property_tests.rs` · `secret_leak_tests.rs` · `capabilities_fixture.rs` ·
`postman_error_fixtures.rs`

`secret_leak_tests.rs` is a standing regression suite: API keys and tokens must
never surface in `Debug`, `Display`, serialization, or error output.

## Layering rule

```text
domain/  →  routes/  →  transport/  →  reqwest
```

Dependencies point downward only. `domain/` never reimplements HTTP, and
`routes/` never encodes business policy. See the per-directory READMEs for the
specific contracts each layer owns.
