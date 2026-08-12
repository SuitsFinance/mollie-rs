# High-risk operation coverage

**Fully protected:** 23 / 23

| operation_id | access | retry | Tier-S | status |
| --- | --- | --- | --- | --- |
| `cancel_payment` | ValidatedFacade | IdempotentWrite | yes (payments) | **full** |
| `cancel_payout` | ValidatedFacade | IdempotentWrite | yes (payouts) | **full** |
| `cancel_refund` | ValidatedFacade | IdempotentWrite | yes (refunds) | **full** |
| `cancel_subscription` | ValidatedFacade | IdempotentWrite | yes (subscriptions) | **full** |
| `create_capture` | ValidatedFacade | IdempotentWrite | yes (captures) | **full** |
| `create_connect_balance_transfer` | ValidatedFacade | IdempotentWrite | yes (connect_balance_transfers) | **full** |
| `create_customer_payment` | ValidatedFacade | IdempotentWrite | yes (payments) | **full** |
| `create_mandate` | ValidatedFacade | IdempotentWrite | yes (mandates) | **full** |
| `create_payment` | ValidatedFacade | IdempotentWrite | yes (payments) | **full** |
| `create_payment_link` | ValidatedFacade | IdempotentWrite | yes (payment_links) | **full** |
| `create_payout` | ValidatedFacade | IdempotentWrite | yes (payouts) | **full** |
| `create_refund` | ValidatedFacade | IdempotentWrite | yes (refunds) | **full** |
| `create_session` | ValidatedFacade | IdempotentWrite | yes (sessions) | **full** |
| `create_subscription` | ValidatedFacade | IdempotentWrite | yes (subscriptions) | **full** |
| `create_transfer` | ValidatedFacade | IdempotentWrite | yes (transfers) | **full** |
| `match_unmatched_credit_transfer` | ValidatedFacade | NonRetryableWrite | yes (unmatched_credit_transfers) | **full** |
| `oauth_generate_tokens` | ValidatedFacade | NonRetryableWrite | yes (oauth) | **full** |
| `oauth_revoke_tokens` | ValidatedFacade | NonRetryableWrite | yes (oauth) | **full** |
| `payment_create_route` | ValidatedFacade | IdempotentWrite | yes (payments) | **full** |
| `return_unmatched_credit_transfer` | ValidatedFacade | NonRetryableWrite | yes (unmatched_credit_transfers) | **full** |
| `terminals_request_pairing_code` | ValidatedFacade | NonRetryableWrite | yes (terminals) | **full** |
| `terminals_revoke_pairing_code` | ValidatedFacade | NonRetryableWrite | yes (terminals) | **full** |
| `verify_payee` | ValidatedFacade | NonRetryableWrite | yes (verify_payee) | **full** |

Fully protected = `ValidatedFacade` + Tier-S module present + write retry class.

