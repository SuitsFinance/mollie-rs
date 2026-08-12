# Domain facade contract (Tier S)

## Principles

1. Facades **validate** before HTTP when a builder exists.
2. Facades **scope** idempotency per logical write via short-lived sticky clones.
3. Facades **do not** reimplement HTTP; they call generated routes.
4. Generated types remain available via `*_raw` / advanced methods.
5. List pages use opaque `from` cursors and shared limit validation.

## Preferred create signatures (0.6)

| Facade | Preferred | Advanced |
| --- | --- | --- |
| payments | `create(CreatePaymentRequired, key?)` | `create_raw(&CreatePaymentRequest, key?)` |
| refunds | `create(PaymentId, CreateRefundRequired, key?)` | `create_raw(..., &CreateRefundRequest, key?)` |
| subscriptions | `create(CustomerId, CreateSubscriptionRequired, key?)` | `create_raw(..., &CreateSubscriptionRequest, key?)` |
| captures | `create(PaymentId, &EntityCapture, key?)` | (builder TBD) |
| mandates | `create(CustomerId, &MandateRequest, key?)` | (builder TBD) |

## Breaking relative to 0.5.x facades

If early consumers called `payments().create(&CreatePaymentRequest, …)`, migrate to:

- `create(CreatePaymentRequired::…, …)` or
- `create_raw(&request, …)`.

This is intentional for 0.6 payment safety.

## Webhooks

| Method | Guarantee |
| --- | --- |
| `parse_classic` | ID only; **not** authenticated |
| `verify_next_gen` | HMAC over raw bytes |
| `verify_and_decode_next_gen` | verify then JSON |
| `get_event` | server-side authenticity via API |
