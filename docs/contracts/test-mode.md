# Test mode contract

Mollie test mode is route-specific. It is not a global switch that every API
operation accepts.

## Sticky query mode

`Client::with_testmode(true)` and the `MollieClientBuilder::testmode(true)`
option configure client state. The generator adds that value to a request only
when the operation declares the OpenAPI `testmode` query parameter.

The current OpenAPI contract includes the query parameter on these operation
IDs:

- Access and platform: `get-permission`, `get-organization`, `get-profile`,
  `list-webhooks`, `get-webhook`, `get-webhook-event`,
  `list-connect-balance-transfers`, `get-connect-balance-transfer`.
- Payments: `list-payments`, `get-payment`, `list-methods`,
  `list-all-methods`, `get-method`, `list-refunds`, `get-refund`,
  `cancel-refund`, `list-all-refunds`, `list-chargebacks`,
  `get-chargeback`, `list-all-chargebacks`, `list-captures`, `get-capture`,
  `list-payment-links`, `get-payment-link`, `get-payment-link-payments`,
  `list-terminals`, `get-terminal`, `payment-list-routes`.
- Recurring: `list-customers`, `get-customer`, `list-customer-payments`,
  `list-mandates`, `get-mandate`, `list-subscriptions`,
  `get-subscription`, `list-all-subscriptions`,
  `list-subscription-payments`.
- Revenue collection: `list-sales-invoices`, `get-sales-invoice`.

This is an operation-level list, not a promise that every route in one API
family accepts the query. The generated route method and the OpenAPI source are
the authority. See [`../route-coverage.md`](../route-coverage.md).

## Live-only reporting routes

The Mollie documentation supplied with this integration identifies the
following Business Operations APIs as not supporting `testmode`:

- Balances: list, get, primary, report, and transactions.
- Settlements: list, get, open, next, payments, captures, refunds, and chargebacks.
- Invoices: list and get.

For these routes, `mollie-rs` rejects any configured sticky `testmode` value
before sending an HTTP request. This prevents a caller from silently asking
for test mode and receiving a live-mode reporting response.

Mollie documents Payouts as the Business Operations exception. The current
checked-in OpenAPI contract does not contain a Payouts operation, so there is
no generated Payouts method in this release to exercise that exception.

## Request-body test mode

Some create and update request bodies contain their own `testmode` field. That
field is independent from the client-owned query setting and must be set on
the typed request body when the operation supports it. Do not infer body
support from sticky-query support, or the reverse.

## Runtime configuration

`MOLLIE_TESTMODE=true` and `--testmode true` configure the same sticky client
value. The value is omitted from routes without the query parameter. For
Business Operations routes listed above, the SDK returns a local invalid
request error instead of making a request.
