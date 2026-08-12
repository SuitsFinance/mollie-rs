# mollieError

## Summary
`MollieError` is the crate-owned error family for configuration, request, transport, payload, and Mollie API errors. HAL API failures classify into a stable catalog (numeric code, uppercase snake-case key, i18n message key) and serialize via `to_envelope()` with a **single global envelope shape**.

Catalog patterns were harvested from Mollie's Postman collections (not redistributed here; see [`NOTICE`](../../NOTICE)) and deduplicated by HTTP status family.

## Symbol
- Name: `MollieError`
- Kind: `enum`
- Owner: `mollie_rs::error`

## Location
- `src/error.rs`
- Catalog: `src/error_catalog.rs`
- Factories: `mollie_rs::factory`
- Fixtures: `src/postman_error_fixtures.rs` (tests)

## Global catalog (Postman-deduped)

| HTTP | code | key | Factory | Notes |
| --- | --- | --- | --- | --- |
| 400 | 40001 | `INVALID_CURSOR` | `invalid_cursor()` | list pagination |
| 403 | 40301 | `ACCESS_TOKEN_PROFILE_RESTRICTED` | `access_token_profile_restricted()` | OAuth/org |
| 403 | 40302 | `DEMO_PROFILE_LIMIT_REACHED` | `demo_profile_limit_reached()` | demo accounts |
| 403 | 40303 | `DEMO_PROFILE_NOT_EDITABLE` | `demo_profile_not_editable()` | demo accounts |
| 403 | 40300 | `FORBIDDEN` | — | fallback |
| 404 | 40401 | `ENTITY_NOT_FOUND` | `entity_not_found(token)` | global not-found |
| 404 | 40400 | `NOT_FOUND` | — | fallback |
| 409 | 40901 | `PAYOUT_NOT_CANCELABLE` | `payout_not_cancelable()` | |
| 409 | 40900 | `CONFLICT` | `conflict(detail)` | fallback |
| 410 | 41001 | `PROFILE_DELETED` | `profile_deleted(token)` | |
| 410 | 41000 | `GONE` | `gone(detail)` | fallback |
| 422 | 42201 | `VALIDATION_ERROR` | `validation_error(detail, field?)` | field missing/invalid (many Postman wordings) |
| 422 | 42202 | `RESOURCE_STATE_CONFLICT` | `resource_state_conflict(detail)` | already deleted / cannot cancel / not allowed |
| 422 | 42200 | `UNPROCESSABLE_ENTITY` | — | fallback |
| 429 | 42901 | `RATE_LIMIT_EXCEEDED` | `rate_limit_exceeded()` | **global** — every route, including `list_clients` (`GET /clients`), `list_capabilities`, etc. Same factory; no per-route 429 type. |
| 503 | 50301 | `SERVICE_TEMPORARILY_UNAVAILABLE` | `service_temporarily_unavailable(detail)` | transfer / platform |

All use the same envelope: `ok: false`, `status`, `code`, `key`, `message_key`, `title`, `detail`, `field?`, `documentation?`.

## Match helpers
- `is_rate_limited()`, `is_validation_error()`, `is_not_found()`, `is_api_error()`, `as_api_body()`, `catalog_entry()`, `to_envelope()`

## Preferred call path
```rust
use mollie_rs::{factory, IntoMollieFuture, MollieClient};

# async fn example(client: &MollieClient) -> Result<(), mollie_rs::MollieError> {
match client.list_customers(None, None, None, None, None).into_mollie_result().await {
    Ok(env) => {
        let _success = env.to_success_envelope();
    }
    Err(e) => {
        let envelope = e.to_envelope(); // same shape for every status
        let _ = factory::rate_limit_exceeded(); // fixtures / local simulation
        return Err(e);
    }
}
# Ok(())
# }
```

## Source of Truth
- `src/error.rs`, `src/error_catalog.rs`, `src/factory.rs`, `src/postman_error_fixtures.rs`
- Full harvest (every unique error body from all collections): `tests/fixtures/postman_error_responses.json`
- Integration coverage: `tests/postman_all_responses.rs`
- Matrix: `docs/postman-response-matrix.md`
