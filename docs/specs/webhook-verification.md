# Spec: Next-gen webhook verification

## Scope

Verify Mollie Next-gen webhook authenticity using HMAC-SHA256 over the raw HTTP body.

## Invariants

1. Verification uses exact received bytes (no JSON reserialize).
2. Header name: `X-Mollie-Signature` (hex digest; optional `sha256=` prefix accepted).
3. Comparison is constant-time.
4. Empty body and oversized body fail closed.
5. Primary + previous secrets supported for rotation.
6. Classic `id=` webhooks are out of scope (separate API).

## API

- `WebhookVerifier::new(secret)`
- `with_previous_secret`, `with_max_body_bytes`, `with_max_skew` (app timestamps only)
- `verify(raw_body, signature)`
- `verify_and_decode<T>(raw_body, signature)`

## Errors

`MollieError::WebhookVerification { failure: WebhookVerifyFailure }`

## Tests

Known vector, tamper, rotation, oversize, JSON decode, secret redaction in Debug.
