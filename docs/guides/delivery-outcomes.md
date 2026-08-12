# Delivery outcomes

`DeliveryOutcome`: `NotSent` | `Rejected` | `Succeeded` | `Unknown`.

- **Unknown** means the request may have reached Mollie — never silently downgrade to NotSent.
- Use `error.delivery_outcome()`, `is_retryable()`, `request_id()`.
- Application must reconcile after Unknown (GET by id / list / provider dashboard).
