# Mollie API overview in `mollie-rs`

`mollie-rs` is the native Rust client for Mollie’s REST API. The generated
`Client` exposes the complete route surface from the checked-in OpenAPI
contract, while `MollieClient` adds validated credentials, HTTPS transport,
test mode, idempotency, and response/error envelopes.

## API basics

All generated operations are grouped by API area under `Client` route modules.
Generated request and response models remain available through `mollie_rs::types`.
Use the facade builders where cross-route validation is required, then pass the
result to the generated route method.

```rust
use mollie_rs::{CreatePaymentRequired, IntoMollieFuture, MollieClient, Money};

let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;
let payment = CreatePaymentRequired::new(
    "Order #12345",
    Money::new("EUR", "10.00")?,
    "https://example.com/return",
)?.into_payment_request();

let response = client
    .create_payment(None, &payment)
    .into_mollie_result()
    .await?;
# Ok::<(), mollie_rs::MollieError>(())
```

## Native coverage

| Mollie area | Rust surface |
| --- | --- |
| Payments, methods, refunds, captures, chargebacks | Generated routes plus validated payment/refund builders |
| Webhooks | Generated webhook-management routes, `WebhookUrl`, and classic `WebhookNotification` parsing |
| Recurring payments and mandates | Customers, mandates, subscriptions, and payment routes |
| Mollie Connect | OAuth bearer credentials, Basic Auth for token-management calls, organizations, profiles, permissions, and onboarding routes |
| Business operations | Balances, settlements, invoices, transactions, and related generated routes; test-mode support is route-specific and documented in `contracts/test-mode.md` |
| Payment links and terminals | Generated payment-link and terminal routes |

The generated route client is the native escape hatch for operations that are
not part of the provider-neutral Athena billing facade.

## Test mode support

Mollie test mode works on many operations, but not every route. The sticky
`testmode` query is attached only where the OpenAPI operation declares it.
Some request bodies also expose an independent `testmode` field.

Machine-readable operation metadata is available through
`mollie_rs::ROUTE_CAPABILITIES` and `mollie_rs::route_capability()`. Each entry
contains the normalized operation id, OpenAPI route group, HTTP method/path,
whether the `testmode` query is supported, and whether a dedicated validated
write builder exists or the generated client is the direct route surface.

Balances, Settlements, and Invoices are the live-only Business Operations APIs
identified in the supplied Mollie documentation; the SDK rejects sticky test
mode for those routes before HTTP. Payouts are Mollie's documented exception,
but no Payouts operation exists in the current checked-in OpenAPI contract.
See [`contracts/test-mode.md`](contracts/test-mode.md) for the route-level
matrix and request-body distinction.
## Webhooks

Classic Mollie webhooks contain an updated resource ID, not a trusted status.
Parse the form body with `WebhookNotification`, acknowledge the request, then
refetch the typed resource through the authenticated client before processing
state changes. See [`contracts/webhooks.md`](contracts/webhooks.md).

## Out of scope for this Rust SDK

- Mollie Components and Mollie.js are frontend integrations, not Rust API-client routes.
- The Mollie MCP Server is a separate integration surface and is not bundled into `mollie-rs`.
- Prebuilt integrations are deployment/application templates rather than SDK types.

See [`route-coverage.md`](route-coverage.md) for the generated operation matrix
and [`contracts/`](contracts/) for validated facade contracts.
