# createRefundRequired

`CreateRefundRequired` validates the writable refund fields before a request is sent.

```rust
use mollie_rs::{CreateRefundRequired, Money};

let body = CreateRefundRequired::new(
    Money::new("EUR", "10.00")?,
    "Order refund",
)?
.into_request()?;
# Ok::<(), mollie_rs::MollieError>(())
```

The generated `CreateRefundRequest` contains only writable refund fields. Response-owned values such as `id`, `status`, `createdAt`, `mode`, and `_links` are represented by `EntityRefundResponse`.
