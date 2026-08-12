# Security Policy

## Supported Versions

| Version | Channel | Supported |
| ------- | ------- | --------- |
| 0.7.x | stable | :white_check_mark: |
| 0.6.x | security fixes only | :warning: |
| before 0.5 | unsupported | :x: |

Only the latest patch on the current minor (`0.7.x`) receives routine fixes.

See [`docs/compatibility.md`](docs/compatibility.md) for generated-vs-facade stability tiers and MSRV.

## Reporting a Vulnerability

**Do not open a public GitHub issue for undisclosed security bugs.**

Prefer [GitHub Security Advisories](https://github.com/SuitsFinance/mollie-rs/security/advisories/new) for private coordinated disclosure. Include reproduction steps, impact, and affected versions. You will receive an acknowledgement and triage update as soon as possible.

## Webhook security guidance

### Classic callbacks (`id=…`)

Classic Mollie webhooks only deliver a resource id. **Never** trust the callback alone:

1. Parse with `WebhookNotification::parse_form_urlencoded`.
2. Authenticate the caller at the network edge if possible.
3. **Refetch** the resource with the Mollie API using your server credentials.
4. Reconcile state idempotently (dedupe by resource id + status).

### Next-generation webhooks (HMAC)

Next-gen events include `X-Mollie-Signature`: HMAC-SHA256 of the **raw body**, hex-encoded.

Use `WebhookVerifier` on the exact request bytes:

```rust
use mollie_rs::WebhookVerifier;

let verifier = WebhookVerifier::new(secret)?.with_previous_secret(previous)?;
verifier.verify(raw_body, signature_header)?;
// only then decode JSON
```

Invariants:

- Verify **before** trusting parsed JSON.
- Never re-serialize JSON and HMAC the result.
- Support secret rotation via `with_previous_secret`.
- Bound body size (`with_max_body_bytes`).
- Prefer constant-time comparison (implemented by the verifier).
- Optionally re-fetch the event via Mollie Webhook Events API when the signing secret may be compromised.
- HMAC does **not** by itself prevent replay; persist processed event IDs (or refetch the event) for deduplication. `with_max_skew` is an **application-level** timestamp check, not a Mollie-signed field.

Do not log signing secrets or full payment/customer payloads by default.

## Transport retries

Automatic retries are **disabled by default**. `RetryPolicy::default_safe` may retry safe reads and writes that already carry an `Idempotency-Key`. Never enable blind write retries without operation-scoped idempotency.

## Credentials

- Prefer environment-based loading (`MOLLIE_API_KEY` / `MOLLIE_OAUTH_ACCESS_TOKEN`).
- Never commit live or testmode secrets.
- Enable the optional `zeroize` feature when process memory retention of secrets is in scope for your threat model.
- Credential types redact secret material in `Debug` output; do not log `Authorization` headers.
