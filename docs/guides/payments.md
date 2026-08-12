# Payments

Prefer the Tier-S facade: `client.payments()`.

## Create

Use `CreatePaymentRequired` so amount, description, and redirect URL are validated before any HTTP call. Bind a **caller-owned sticky** `IdempotencyKey` when the application may retry.

```rust,no_run
use mollie_rs::{CreatePaymentRequired, IdempotencyKey, MollieClient, Money, RetryPolicy};

# async fn example() -> Result<(), mollie_rs::MollieError> {
let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?
    .with_retry_policy(RetryPolicy::default_safe());

let key = IdempotencyKey::new("order-12345-payment-create")?;
let required = CreatePaymentRequired::new(
    "Order #12345",
    Money::new("EUR", "10.00")?,
    "https://example.com/return",
)?;

let payment = client.payments().create(required, Some(key)).await?.into_inner();
let _ = payment;
# Ok(())
# }
```

## Cancel

`payments().cancel(id, body, key)` is an **IdempotentWrite**. Persist the sticky key if cancel may be retried.

## Customer payments

`payments().create_for_customer(customer_id, required, key)` covers the customer-scoped create path with the same sticky-key rules.

## Delayed Connect routes

`payments().create_delayed_route(...)` is Tier-S for delayed routing (IdempotentWrite).

## List and stream

- `list_page` — one cursor page for UIs
- `list_all` — bounded by `PaginationGuard`
- `stream_pages` / `stream_items` — async streams, also budget-guarded

Pagination never follows foreign-origin `next` links and rejects cursor cycles.

## Ambiguous failures

If `error.delivery_outcome()` is `Some(Unknown)`, the request may have reached Mollie. Reconcile with `get` / list before creating another payment without the **same** sticky key.

See also: [`safe-payment-retry.md`](safe-payment-retry.md), [`pagination.md`](pagination.md).
