# Mollie API drift report

Generated: `2026-08-10`

This file is produced by `scripts/report_api_drift.py`.
It does **not** regenerate client sources.

## Local pinned contract (`specs-3.0.yaml`)

- Operations: **124**
- Route capabilities: **124**
- Missing from capabilities: **0**
- Extra in capabilities: **0**

Local capabilities match the pinned OpenAPI operation inventory.

### Operations by tag

- `Accounts API`: 4
- `Balance Transfers API`: 3
- `Balances API`: 5
- `Capabilities API`: 1
- `Captures API`: 3
- `Chargebacks API`: 3
- `Client Links API`: 1
- `Clients API`: 2
- `Customers API`: 7
- `Delayed Routing API`: 3
- `Invoices API`: 2
- `Mandates API`: 4
- `Methods API`: 7
- `OAuth API`: 2
- `Onboarding API`: 2
- `Organizations API`: 3
- `Payment Links API`: 6
- `Payments API`: 6
- `Payouts API`: 4
- `Permissions API`: 2
- `Profiles API`: 6
- `Refunds API`: 5
- `Sales Invoices API`: 5
- `Sessions API`: 2
- `Settlements API`: 8
- `Subscriptions API`: 7
- `Terminals API`: 6
- `Transfers API`: 2
- `Unmatched Credit Transfers API`: 4
- `Verify Payee API`: 1
- `Wallets API`: 1
- `Webhook Events API`: 1
- `Webhooks API`: 6

### Full local operation inventory

| operation_id | method | path | deprecated |
| --- | --- | --- | --- |
| `cancel_payment` | `DELETE` | `/payments/{paymentId}` |  |
| `cancel_payout` | `DELETE` | `/payouts/{payoutId}` |  |
| `cancel_refund` | `DELETE` | `/payments/{paymentId}/refunds/{refundId}` |  |
| `cancel_subscription` | `DELETE` | `/customers/{customerId}/subscriptions/{subscriptionId}` |  |
| `create_capture` | `POST` | `/payments/{paymentId}/captures` |  |
| `create_client_link` | `POST` | `/client-links` |  |
| `create_connect_balance_transfer` | `POST` | `/connect/balance-transfers` |  |
| `create_customer` | `POST` | `/customers` |  |
| `create_customer_payment` | `POST` | `/customers/{customerId}/payments` |  |
| `create_mandate` | `POST` | `/customers/{customerId}/mandates` |  |
| `create_payment` | `POST` | `/payments` |  |
| `create_payment_link` | `POST` | `/payment-links` |  |
| `create_payout` | `POST` | `/payouts` |  |
| `create_profile` | `POST` | `/profiles` |  |
| `create_refund` | `POST` | `/payments/{paymentId}/refunds` |  |
| `create_sales_invoice` | `POST` | `/sales-invoices` |  |
| `create_session` | `POST` | `/sessions` |  |
| `create_subscription` | `POST` | `/customers/{customerId}/subscriptions` |  |
| `create_transfer` | `POST` | `/business-accounts/transfers` |  |
| `create_webhook` | `POST` | `/webhooks` |  |
| `delete_customer` | `DELETE` | `/customers/{customerId}` |  |
| `delete_payment_link` | `DELETE` | `/payment-links/{paymentLinkId}` |  |
| `delete_profile` | `DELETE` | `/profiles/{profileId}` |  |
| `delete_sales_invoice` | `DELETE` | `/sales-invoices/{salesInvoiceId}` |  |
| `delete_webhook` | `DELETE` | `/webhooks/{webhookId}` |  |
| `disable_method` | `DELETE` | `/profiles/{profileId}/methods/{methodId}` |  |
| `disable_method_issuer` | `DELETE` | `/profiles/{profileId}/methods/{methodId}/issuers/{issuerId}` |  |
| `enable_method` | `POST` | `/profiles/{profileId}/methods/{methodId}` |  |
| `enable_method_issuer` | `POST` | `/profiles/{profileId}/methods/{methodId}/issuers/{issuerId}` |  |
| `get_balance` | `GET` | `/balances/{balanceId}` |  |
| `get_balance_report` | `GET` | `/balances/{balanceId}/report` |  |
| `get_business_account` | `GET` | `/business-accounts/accounts/{businessAccountId}` |  |
| `get_business_account_transaction` | `GET` | `/business-accounts/accounts/{businessAccountId}/transactions/{transactionId}` |  |
| `get_capture` | `GET` | `/payments/{paymentId}/captures/{captureId}` |  |
| `get_chargeback` | `GET` | `/payments/{paymentId}/chargebacks/{chargebackId}` |  |
| `get_client` | `GET` | `/clients/{organizationId}` |  |
| `get_connect_balance_transfer` | `GET` | `/connect/balance-transfers/{balanceTransferId}` |  |
| `get_current_organization` | `GET` | `/organizations/me` |  |
| `get_current_profile` | `GET` | `/profiles/me` |  |
| `get_customer` | `GET` | `/customers/{customerId}` |  |
| `get_invoice` | `GET` | `/invoices/{invoiceId}` |  |
| `get_mandate` | `GET` | `/customers/{customerId}/mandates/{mandateId}` |  |
| `get_method` | `GET` | `/methods/{methodId}` |  |
| `get_next_settlement` | `GET` | `/settlements/next` |  |
| `get_onboarding_status` | `GET` | `/onboarding/me` |  |
| `get_open_settlement` | `GET` | `/settlements/open` |  |
| `get_organization` | `GET` | `/organizations/{organizationId}` |  |
| `get_partner_status` | `GET` | `/organizations/me/partner` |  |
| `get_payment` | `GET` | `/payments/{paymentId}` |  |
| `get_payment_link` | `GET` | `/payment-links/{paymentLinkId}` |  |
| `get_payment_link_payments` | `GET` | `/payment-links/{paymentLinkId}/payments` |  |
| `get_payout` | `GET` | `/payouts/{payoutId}` |  |
| `get_permission` | `GET` | `/permissions/{permissionId}` |  |
| `get_primary_balance` | `GET` | `/balances/primary` |  |
| `get_profile` | `GET` | `/profiles/{profileId}` |  |
| `get_refund` | `GET` | `/payments/{paymentId}/refunds/{refundId}` |  |
| `get_sales_invoice` | `GET` | `/sales-invoices/{salesInvoiceId}` |  |
| `get_session` | `GET` | `/sessions/{sessionId}` |  |
| `get_settlement` | `GET` | `/settlements/{settlementId}` |  |
| `get_subscription` | `GET` | `/customers/{customerId}/subscriptions/{subscriptionId}` |  |
| `get_terminal` | `GET` | `/terminals/{terminalId}` |  |
| `get_transfer` | `GET` | `/business-accounts/transfers/{businessAccountsTransferId}` |  |
| `get_unmatched_credit_transfer` | `GET` | `/unmatched-credit-transfers/{unmatchedCreditTransferId}` |  |
| `get_webhook` | `GET` | `/webhooks/{webhookId}` |  |
| `get_webhook_event` | `GET` | `/events/{webhookEventId}` |  |
| `list_all_chargebacks` | `GET` | `/chargebacks` |  |
| `list_all_methods` | `GET` | `/methods/all` |  |
| `list_all_refunds` | `GET` | `/refunds` |  |
| `list_all_subscriptions` | `GET` | `/subscriptions` |  |
| `list_balance_transactions` | `GET` | `/balances/{balanceId}/transactions` |  |
| `list_balances` | `GET` | `/balances` |  |
| `list_business_account_transactions` | `GET` | `/business-accounts/accounts/{businessAccountId}/transactions` |  |
| `list_business_accounts` | `GET` | `/business-accounts/accounts` |  |
| `list_capabilities` | `GET` | `/capabilities` |  |
| `list_captures` | `GET` | `/payments/{paymentId}/captures` |  |
| `list_chargebacks` | `GET` | `/payments/{paymentId}/chargebacks` |  |
| `list_clients` | `GET` | `/clients` |  |
| `list_connect_balance_transfers` | `GET` | `/connect/balance-transfers` |  |
| `list_customer_payments` | `GET` | `/customers/{customerId}/payments` |  |
| `list_customers` | `GET` | `/customers` |  |
| `list_invoices` | `GET` | `/invoices` |  |
| `list_mandates` | `GET` | `/customers/{customerId}/mandates` |  |
| `list_methods` | `GET` | `/methods` |  |
| `list_payment_links` | `GET` | `/payment-links` |  |
| `list_payments` | `GET` | `/payments` |  |
| `list_payouts` | `GET` | `/payouts` |  |
| `list_permissions` | `GET` | `/permissions` |  |
| `list_profiles` | `GET` | `/profiles` |  |
| `list_refunds` | `GET` | `/payments/{paymentId}/refunds` |  |
| `list_sales_invoices` | `GET` | `/sales-invoices` |  |
| `list_settlement_captures` | `GET` | `/settlements/{settlementId}/captures` |  |
| `list_settlement_chargebacks` | `GET` | `/settlements/{settlementId}/chargebacks` |  |
| `list_settlement_payments` | `GET` | `/settlements/{settlementId}/payments` |  |
| `list_settlement_refunds` | `GET` | `/settlements/{settlementId}/refunds` |  |
| `list_settlements` | `GET` | `/settlements` |  |
| `list_subscription_payments` | `GET` | `/customers/{customerId}/subscriptions/{subscriptionId}/payments` |  |
| `list_subscriptions` | `GET` | `/customers/{customerId}/subscriptions` |  |
| `list_terminals` | `GET` | `/terminals` |  |
| `list_unmatched_credit_transfers` | `GET` | `/unmatched-credit-transfers` |  |
| `list_webhooks` | `GET` | `/webhooks` |  |
| `match_unmatched_credit_transfer` | `POST` | `/unmatched-credit-transfers/{unmatchedCreditTransferId}/match` |  |
| `oauth_generate_tokens` | `POST` | `/oauth2/tokens` |  |
| `oauth_revoke_tokens` | `DELETE` | `/oauth2/tokens` |  |
| `payment_create_route` | `POST` | `/payments/{paymentId}/routes` |  |
| `payment_get_route` | `GET` | `/payments/{paymentId}/routes/{routeId}` |  |
| `payment_list_routes` | `GET` | `/payments/{paymentId}/routes` |  |
| `release_authorization` | `POST` | `/payments/{paymentId}/release-authorization` |  |
| `request_apple_pay_payment_session` | `POST` | `/wallets/applepay/sessions` |  |
| `return_unmatched_credit_transfer` | `POST` | `/unmatched-credit-transfers/{unmatchedCreditTransferId}/return` |  |
| `revoke_mandate` | `DELETE` | `/customers/{customerId}/mandates/{mandateId}` |  |
| `submit_onboarding_data` | `POST` | `/onboarding/me` |  |
| `terminals_get_pairing_code` | `GET` | `/terminals/pairing-codes/{pairingCodeId}` |  |
| `terminals_list_pairing_codes` | `GET` | `/terminals/pairing-codes` |  |
| `terminals_request_pairing_code` | `POST` | `/terminals/pairing-codes` |  |
| `terminals_revoke_pairing_code` | `DELETE` | `/terminals/pairing-codes/{pairingCodeId}` |  |
| `test_webhook` | `POST` | `/webhooks/{webhookId}/ping` |  |
| `update_customer` | `PATCH` | `/customers/{customerId}` |  |
| `update_payment` | `PATCH` | `/payments/{paymentId}` |  |
| `update_payment_link` | `PATCH` | `/payment-links/{paymentLinkId}` |  |
| `update_profile` | `PATCH` | `/profiles/{profileId}` |  |
| `update_sales_invoice` | `PATCH` | `/sales-invoices/{salesInvoiceId}` |  |
| `update_subscription` | `PATCH` | `/customers/{customerId}/subscriptions/{subscriptionId}` |  |
| `update_webhook` | `PATCH` | `/webhooks/{webhookId}` |  |
| `verify_payee` | `POST` | `/business-accounts/payee-verifications` |  |

## Upstream comparison

No `--upstream` snapshot provided. CI records the local inventory only.
To compare against an authoritative Mollie OpenAPI document:

```sh
python scripts/report_api_drift.py --upstream path/to/upstream.yaml --write docs/api-drift-report.md
```

## Policy

- Do **not** auto-publish a regeneration from upstream drift.
- Review Tier G (generated) fallout before merging OpenAPI updates.
- Intentional exclusions should be documented in `docs/route-coverage.md`.
- See `docs/compatibility.md` for stability tiers.
