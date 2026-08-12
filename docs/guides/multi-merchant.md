# Multi-merchant

```rust
let a = client.with_credential(cred_a)?;
let b = client.with_credential(cred_b)?;
```

- Cheap clone; do not mutate shared state.
- Combine with `with_profile_id` / `with_idempotency` for request-scoped context.
- Concurrent scoped clients must not cross-wire credentials (covered by isolation tests).
