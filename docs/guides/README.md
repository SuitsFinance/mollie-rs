# Guides

Task-oriented walkthroughs for production use of `mollie-rs`.

## Start here

| Guide | Topic |
| --- | --- |
| [`safe-payment-retry.md`](safe-payment-retry.md) | Sticky idempotency, retries, `DeliveryOutcome`, cancellation |
| [`payments.md`](payments.md) | Create / cancel / customer payments / streams |
| [`refunds.md`](refunds.md) | Refund create / cancel / list |
| [`handle-signed-webhook.md`](handle-signed-webhook.md) | Next-gen webhook HMAC verify + app ownership |
| [`payouts-and-transfers.md`](payouts-and-transfers.md) | Payouts, BA transfers, verify-payee |
| [`oauth-connect.md`](oauth-connect.md) | OAuth tokens + Connect balance transfers |
| [`pagination.md`](pagination.md) | Cursors, budgets, streams |
| [`multi-merchant.md`](multi-merchant.md) | Credential / profile scoping |
| [`error-handling.md`](error-handling.md) | `MollieError` helpers and logging |
| [`testing.md`](testing.md) | Unit, contract, live, fuzz |

Also see [`../API-STABILITY.md`](../API-STABILITY.md) and [`../release-readiness.md`](../release-readiness.md).
