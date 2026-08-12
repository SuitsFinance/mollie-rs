# responseEnvelope

## Summary
`ResponseEnvelope<T>` stores a typed Mollie response body with its HTTP status, headers, and resolved `idempotency_key` (when known). `MollieEnvelope<T>` is the shared alias for the same type, and `MollieResponse<T>` is the shared `MollieResult<MollieEnvelope<T>>` alias.

## Symbol
- Name: `ResponseEnvelope`
- Kind: `struct`
- Owner: `mollie_rs::envelope`

## Signature
```rust
pub struct ResponseEnvelope<T>
```

```rust
pub type MollieEnvelope<T> = ResponseEnvelope<T>;
pub type MollieResponse<T> = MollieResult<MollieEnvelope<T>>;
```

## Location
- `src/envelope.rs`

## Inputs
- `from_parts(data, status, headers)` accepts a typed body, `StatusCode`, and `HeaderMap` (reads `idempotency-key` from headers when present).
- `from_parts_with_idempotency(data, status, headers, idempotency_key)` sets the key explicitly.
- `from_response_value(response)` accepts a generated `ResponseValue<T>` (including the key attached by generated routes).
- `ok(data)` creates a local successful envelope for tests and factories.

## Returns
- `data()` returns `&T`.
- `status()` returns `StatusCode`.
- `headers()` returns `&HeaderMap`.
- `idempotency_key()` returns `Option<&str>` for the key sent with the request.
- `into_inner()` returns `T`.
- `map()` transforms `T` while preserving status, headers, and idempotency key.
- `MollieEnvelope<T>` can be used anywhere `ResponseEnvelope<T>` is expected.
- `MollieResponse<T>` represents a converted successful route envelope or a crate-owned `MollieError`.
- `ResponseValueExt::idempotency_key()` reads the same key from generated `ResponseValue` headers.

## Errors
- The envelope constructors are infallible.

## Preconditions
- `from_response_value` expects a generated successful response.

## Side Effects
- None.

## Guarantees
- Preserves status, headers, and idempotency key when converting from `ResponseValue<T>`.
- Generated routes always resolve an idempotency key from client state (sticky key via `Client::with_idempotency_key`, or UUID v4) and attach it to the response.
- `map()` does not modify status, headers, or idempotency key.
- `success_catalog()` / `to_success_envelope()` map HTTP status to stable success codes/keys (see `mollieSuccessEnvelope.md`).

## Examples
```rust
use mollie_rs::{ResponseEnvelope, ResponseValue};
use reqwest::StatusCode;

let generated = ResponseValue::new("ok", StatusCode::OK, Default::default());
let envelope = ResponseEnvelope::from_response_value(generated);
assert_eq!(envelope.into_inner(), "ok");
```

## Source of Truth
- Implementation: `src/envelope.rs`
