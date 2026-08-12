# createSubscriptionRequired

`CreateSubscriptionRequired` validates writable subscription fields, including Mollie's interval syntax and optional ISO date and URL fields.

```rust
use mollie_rs::{CreateSubscriptionRequired, Money};

let body = CreateSubscriptionRequired::new(
    Money::new("EUR", "10.00")?,
    "Monthly plan",
    "1 month",
)?
.with_start_date("2026-01-31")?
.with_webhook_url("https://example.com/mollie")?
.into_request()?;
# Ok::<(), mollie_rs::MollieError>(())
```

The customer ID remains a route argument to `Client::create_subscription`; it is not serialized into the request body.
