# responseValueExt

## Summary
`ResponseValueExt` exposes the resolved idempotency key on generated `ResponseValue<T>` responses. Generated route methods always send an `Idempotency-Key` (from client sticky state or a generated UUID v4) and echo it onto the response headers.

## Symbol
- Name: `ResponseValueExt`
- Kind: `trait`
- Owner: `mollie_rs::envelope`

## Signature
```rust
pub trait ResponseValueExt {
    fn idempotency_key(&self) -> Option<&str>;
}
```

## Location
- `src/envelope.rs`

## Inputs
- Implemented for `ResponseValue<T>` from `progenitor_client`.

## Returns
- `idempotency_key()` returns `Some` when the `idempotency-key` header is present and valid UTF-8.

## Errors
- Infallible (returns `None` when the header is missing or not valid UTF-8).

## Guarantees
- Generated routes attach the same key that was sent on the request.
- `ResponseEnvelope::from_response_value` copies the key into `ResponseEnvelope::idempotency_key()`.

## Examples
```rust
use mollie_rs::{ResponseValue, ResponseValueExt};
use reqwest::StatusCode;

let mut headers = reqwest::header::HeaderMap::new();
headers.insert(
    "idempotency-key",
    "6f7ef3e6-8c2f-4d1c-9f08-5ab7adf56c91"
        .parse()
        .expect("static header value"),
);
let response = ResponseValue::new("ok", StatusCode::OK, headers);
assert_eq!(
    response.idempotency_key(),
    Some("6f7ef3e6-8c2f-4d1c-9f08-5ab7adf56c91")
);
```

## Source of Truth
- Implementation: `src/envelope.rs`
- Request resolution: `Client::request` in `src/lib.rs`
- Header attachment: `src/routes/response.rs`
