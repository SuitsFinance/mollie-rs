# Route capabilities

`src/route_capabilities.rs` is generated/exported from the local OpenAPI pin and
describes every local operation:

* `operation_id`, `http_method`, `path`
* `supports_testmode`, `supports_idempotency`
* `safe_to_retry`, `retry_class` (`SafeRead` | `IdempotentWrite` | …)
* `paginated`
* `requires_profile_scope`
* `access` (`ValidatedFacade` | `GeneratedClient`)

Provider lifecycle maturity (`ga` | `beta` | `private_beta`) is **not** on
`RouteCapability`; see `docs/registries/provider-maturity.yaml` and the
`provider_maturity` field on each entry in `docs/registries/operation-registry.yaml`.

## Rules

1. Capabilities must match `specs-3.0.yaml` operation inventory (CI).
2. Transport uses `route_capability(operation_id).retry_class` before HTTP-method fallback.
3. Unknown / unlisted mutations must be treated as non-auto-retry.
4. `ValidatedFacade` is reserved for typed write builders that add validation.

## Commands

```text
python scripts/check_generation_reproducibility.py
python scripts/generate_route_capabilities.py   # when regenerating
python scripts/export_operation_registry.py
```
