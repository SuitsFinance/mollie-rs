# Safe payment retry

## Rule

> Never automatically retry a financial write without a **stable, caller-owned** idempotency key.

`mollie-rs` enforces this in transport policy:

- Default `RetryPolicy` is **disabled**.
- `RetryPolicy::default_safe` retries safe reads freely.
- Writes classified as `RetryClass::IdempotentWrite` retry **only** when a sticky key is bound on the client (not merely an auto-generated per-request UUID).
- Classification is **operation-registry-driven** (`route_capability` / `retry_class_for_operation`), not HTTP-method-primary.

| `RetryClass` | Auto-retry under `default_safe` |
| --- | --- |
| `SafeRead` | Yes |
| `IdempotentWrite` | Sticky caller-owned key only |
| `NonRetryableWrite` | Never |
| `ProviderDefined` | Never (safe default) |
| `Unknown` | Never (missing registry entry) |

## Recommended pattern

```rust,no_run
use mollie_rs::{CreatePaymentRequired, IdempotencyKey, MollieClient, Money, RetryPolicy};

# async fn example() -> Result<(), mollie_rs::MollieError> {
let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?
    .with_retry_policy(RetryPolicy::default_safe());

// Persist this key with your order id before the first attempt.
let key = IdempotencyKey::new("order-12345-payment-create")?;
let required = CreatePaymentRequired::new(
    "Order #12345",
    Money::new("EUR", "10.00")?,
    "https://example.com/return",
)?;

let payment = client
    .payments()
    .create(required, Some(key))
    .await?
    .into_inner();
let _ = payment;
# Ok(())
# }
```

## Retry budget

`RetryPolicy::total_deadline` (alias `retry_budget()`) is a wall-clock **retry budget** from the first attempt: when exhausted, the SDK returns the last attempt (or a budget error) and does **not** send an extra leftover request.

## Delivery outcomes (ambiguous failure)

Transport classifies attempts as:

| Outcome | Meaning | Auto-retry (with `default_safe`) |
| --- | --- | --- |
| **NotSent** | Request not known to leave the client (connect/DNS) | Yes if class allows |
| **Rejected** | Definitive provider rejection (typical 4xx) | No |
| **Succeeded** | Definitive success | n/a |
| **Unknown** | May have been processed (timeout after transmit, reset, cancel mid-flight) | Reads: yes. Writes: **only** with sticky key |

Inspect via `MollieError::delivery_outcome()` / `is_outcome_unknown()`.

**Cancellation:** dropping an in-flight Rust future after the request may have been written is **Unknown**. Always bind a sticky idempotency key for writes you might cancel and retry.

## What not to do

- Do not reuse one sticky key across unrelated payments.
- Do not enable write retries globally without understanding Mollie idempotency semantics.
- Do not treat webhook delivery as a substitute for an authoritative GET after a write.
- Do not assume `is_timeout()` means the write did not happen — treat it as **Unknown**.
