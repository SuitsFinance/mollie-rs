# Multi-merchant clients

Scope credentials and profile per request path with cheap clones — do not mutate a shared client.

```rust,no_run
# fn example(client: mollie_rs::MollieClient, cred_a: mollie_rs::Credential, cred_b: mollie_rs::Credential) -> Result<(), mollie_rs::MollieError> {
let merchant_a = client.with_credential(cred_a)?;
let merchant_b = client.with_credential(cred_b)?;
let _ = (merchant_a, merchant_b);
# Ok(())
# }
```

## Rules

- Combine with `with_profile_id` / `with_idempotency` for request-scoped context.
- Concurrent scoped clients must not cross-wire credentials (covered by HTTP contract tests).
- Prefer one sticky idempotency key **per merchant business operation**, not a global key.
