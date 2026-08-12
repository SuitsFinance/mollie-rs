# Refunds

Tier-S: `client.refunds()`.

## Create and cancel

- Create with `CreateRefundRequired` + optional sticky `IdempotencyKey` (**IdempotentWrite**).
- Cancel with `refunds().cancel(payment_id, refund_id, key)` while Mollie still allows cancel.

```rust,no_run
use mollie_rs::{CreateRefundRequired, IdempotencyKey, Money, PaymentId};

# async fn example(client: mollie_rs::MollieClient) -> Result<(), mollie_rs::MollieError> {
let payment = PaymentId::parse("tr_WDqYK6vllg")?;
let required = CreateRefundRequired::new(Money::new("EUR", "1.00")?, "Partial refund")?;
let key = IdempotencyKey::new("order-12345-refund-1")?;
let _refund = client.refunds().create(&payment, required, Some(key)).await?;
# Ok(())
# }
```

## List and stream

Payment-scoped `list_page` / `list_all` / `stream_pages` / `stream_items` honor `PaginationGuard`.

## Unknown delivery

After a timeout or reset on create, treat the outcome as **Unknown**: GET the refund or list refunds for the payment before issuing a different body.

See also: [`safe-payment-retry.md`](safe-payment-retry.md).
