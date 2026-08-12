# Handle signed (Next-gen) webhooks

## Flow

```text
raw body + signature header
  → WebhookVerifier::verify (constant-time HMAC)
  → derive event identity
  → dedupe (app store)
  → enqueue
  → HTTP 2xx ack
  → worker: refetch resource → reconcile → mark processed
```

## Verify before parse

```rust,no_run
use mollie_rs::{WebhookVerifier, MOLLIE_SIGNATURE_HEADER};

# fn handle(raw: &[u8], signature: &str) -> Result<(), mollie_rs::MollieError> {
let verifier = WebhookVerifier::new(std::env::var("MOLLIE_WEBHOOK_SECRET").unwrap())?
    .with_previous_secret(std::env::var("MOLLIE_WEBHOOK_SECRET_PREV").unwrap_or_default())
    .ok()
    .unwrap_or_else(|| WebhookVerifier::new(std::env::var("MOLLIE_WEBHOOK_SECRET").unwrap()).unwrap());

verifier.verify(raw, signature)?;
// Only now decode JSON / map event types.
let _ = signature;
let _ = MOLLIE_SIGNATURE_HEADER;
# Ok(())
# }
```

## Application ownership

Implement `WebhookEventStore`, `WebhookDispatcher`, and `PaymentStateRefetcher` (see `mollie_rs::integration`) in your app. The crate will not ship a database.

HMAC verification does **not** prevent replay — dedupe does.
