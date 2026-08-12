# intoMollieResult

## Summary
`IntoMollieFuture` converts route futures into `MollieResponse<T>` or `MollieResult<T>` when awaited. `IntoMollieResult` remains available for already-awaited route results.

## Symbol
- Name: `IntoMollieFuture`
- Kind: `trait`
- Owner: `mollie_rs::envelope`

## Signature
```rust
pub trait IntoMollieFuture<T>
```

## Location
- `src/envelope.rs`

## Inputs
- Implemented for route futures that return `ResponseValue<T>` on success.
- `IntoMollieResult` is implemented for already-awaited route results.
- `GeneratedMollieResult<T>` aliases the already-awaited route result shape.

## Returns
- `into_mollie_result().await` returns `MollieResponse<T>`.
- `into_mollie_data().await` returns `MollieResult<T>`.

## Errors
- Converts request, communication, API, payload, unexpected-status, and hook errors into `MollieError`.

## Preconditions
- The input should be a future returned by a Mollie route method.

## Side Effects
- None.

## Guarantees
- Successful responses keep their typed body, status, and headers as `ResponseEnvelope<T>` / `MollieEnvelope<T>`.
- Mollie HAL error bodies (including global statuses such as `403` and `429` that may be omitted from per-operation OpenAPI responses) are preserved as `MollieError::Api` and classify via `catalog_entry()` / `to_envelope()`.
- App boundary: success → `envelope.to_success_envelope()`; error → `err.to_envelope()` (both share `ok` + `code` + `key` + `message_key`).

## Examples
```rust
use mollie_rs::{IntoMollieFuture, ResponseValue};
use reqwest::StatusCode;

# async fn example() -> Result<(), mollie_rs::MollieError> {
let generated = async {
    Ok::<_, mollie_rs::Error<mollie_rs::types::ErrorResponse>>(ResponseValue::new(
        "ok",
        StatusCode::OK,
        Default::default(),
    ))
};
assert_eq!(generated.into_mollie_data().await?, "ok");
# Ok(())
# }
```

## Source of Truth
- Implementation: `src/envelope.rs`
