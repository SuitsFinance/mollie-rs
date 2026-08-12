# Example support matrix

Auto-generated from the **latest** entry in each `logs/<example>.log` file whenever a route example runs (`examples/support/mod.rs`).

Do not edit by hand - re-run examples (or delete a log and re-run) to refresh a row.

Offline rebuild (no API calls):

```sh
python scripts/rebuild_example_support_matrix.py
```

## How to read this

| Support | Meaning |
| --- | --- |
| `supported` | Last run logged `OK response` / `OK envelope` (HTTP success decoded). |
| `failed` | Last run logged `ERROR ...` (API error, decode error, or client failure). |
| `skipped` | Missing credentials; example did not call Mollie. |
| `untested` | No `logs/<example>.log` yet (or unparseable). |

| Label | Meaning |
| --- | --- |
| `access-token-not-profile-restricted` | The endpoint requires an access token that is not restricted to a specific profile. |

**Totals:** 100 examples - **13** supported, **7** failed, **0** skipped, **80** untested.

Detail and full bodies stay in the per-example log; this table is the roll-up.

## Matrix

| Example | Route | Support | HTTP | Code | Key | Label | Summary | Log | Updated |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `list_balances` | `GET /balances` | `failed` | Some(403) | 40301 | ACCESS_TOKEN_PROFILE_RESTRICTED | access-token-not-profile-restricted | Forbidden: This API endpoint is only available with an access token not restricted to a specific profile. (code 40301, key ACCESS_TOKEN_PROFILE_RESTRICTED) | `logs/list_balances.log` | 1783900198s |
| `list_captures` | `GET /payments/{paymentId}/captures` | `failed` | Some(404) | 40400 | NOT_FOUND | - | Not Found: No payment exists with token tr_1234567890. (code 40400, key NOT_FOUND) | `logs/list_captures.log` | 1783903483s |
| `list_invoices` | `GET /invoices` | `failed` | Some(403) | 40301 | ACCESS_TOKEN_PROFILE_RESTRICTED | access-token-not-profile-restricted | Forbidden: This API endpoint is only available with an access token not restricted to a specific profile. (code 40301, key ACCESS_TOKEN_PROFILE_RESTRICTED) | `logs/list_invoices.log` | 1783930319s |
| `list_methods` | `GET /methods` | `failed` | Some(400) | 40000 | API_ERROR | - | Bad Request: The billingCountry is invalid (code 40000, key API_ERROR) | `logs/list_methods.log` | 1783908208s |
| `list_refunds` | `GET /payments/{paymentId}/refunds` | `failed` | Some(404) | 40400 | NOT_FOUND | - | Not Found: No payment exists with token tr_1234567890. (code 40400, key NOT_FOUND) | `logs/list_refunds.log` | 1783908239s |
| `list_settlements` | `GET /settlements` | `failed` | Some(403) | 40301 | ACCESS_TOKEN_PROFILE_RESTRICTED | access-token-not-profile-restricted | Forbidden: This API endpoint is only available with an access token not restricted to a specific profile. (code 40301, key ACCESS_TOKEN_PROFILE_RESTRICTED) | `logs/list_settlements.log` | 1783908135s |
| `list_webhooks` | `GET /webhooks` | `failed` | Some(403) | 40301 | ACCESS_TOKEN_PROFILE_RESTRICTED | access-token-not-profile-restricted | Forbidden: This API endpoint is only available with an access token not restricted to a specific profile. (code 40301, key ACCESS_TOKEN_PROFILE_RESTRICTED) | `logs/list_webhooks.log` | 1783903451s |
| `create_customer` | `POST /customers` | `supported` | 201 Created | 20100 | CREATED | - | OK response | `logs/create_customer.log` | 1783930282s |
| `list_all_chargebacks` | `GET /chargebacks` | `supported` | 200 OK | 20000 | OK | - | OK response | `logs/list_all_chargebacks.log` | 1783903579s |
| `list_all_methods` | `GET /methods/all` | `supported` | 200 OK | 20000 | OK | - | OK response | `logs/list_all_methods.log` | 1783932674s |
| `list_all_refunds` | `GET /refunds` | `supported` | 200 OK | 20000 | OK | - | OK response | `logs/list_all_refunds.log` | 1783937634s |
| `list_all_subscriptions` | `GET /subscriptions` | `supported` | 200 OK | 20000 | OK | - | OK response | `logs/list_all_subscriptions.log` | 1783903486s |
| `list_chargebacks` | `GET /payments/{paymentId}/chargebacks` | `supported` | 200 OK | 20000 | OK | - | OK response | `logs/list_chargebacks.log` | 1783930424s |
| `list_customers` | `GET /customers` | `supported` | 200 OK | 20000 | OK | - | OK response | `logs/list_customers.log` | 1783919947s |
| `list_mandates` | `GET /customers/{customerId}/mandates` | `supported` | 200 OK | 20000 | OK | - | OK response | `logs/list_mandates.log` | 1783908125s |
| `list_payment_links` | `GET /payment-links` | `supported` | 200 OK | 20000 | OK | - | OK response | `logs/list_payment_links.log` | 1783900191s |
| `list_payments` | `GET /payments` | `supported` | 200 OK | 20000 | OK | - | OK response | `logs/list_payments.log` | 1783900220s |
| `list_sales_invoices` | `GET /sales-invoices` | `supported` | 200 OK | 20000 | OK | - | OK response | `logs/list_sales_invoices.log` | 1783908229s |
| `list_subscriptions` | `GET /customers/{customerId}/subscriptions` | `supported` | 200 OK | 20000 | OK | - | OK response | `logs/list_subscriptions.log` | 1783901960s |
| `list_terminals` | `GET /terminals` | `supported` | 200 OK | 20000 | OK | - | OK response | `logs/list_terminals.log` | 1783903454s |
| `cancel_payment` | `DELETE /payments/{paymentId}` | `untested` | - | - | - | - | no log yet | `-` | - |
| `cancel_refund` | `DELETE /payments/{paymentId}/refunds/{refundId}` | `untested` | - | - | - | - | no log yet | `-` | - |
| `cancel_subscription` | `DELETE /customers/{customerId}/subscriptions/{subscriptionId}` | `untested` | - | - | - | - | no log yet | `-` | - |
| `create_capture` | `POST /payments/{paymentId}/captures` | `untested` | - | - | - | - | no log yet | `-` | - |
| `create_client_link` | `POST /client-links` | `untested` | - | - | - | - | no log yet | `-` | - |
| `create_connect_balance_transfer` | `POST /connect/balance-transfers` | `untested` | - | - | - | - | no log yet | `-` | - |
| `create_customer_payment` | `POST /customers/{customerId}/payments` | `untested` | - | - | - | - | no log yet | `-` | - |
| `create_mandate` | `POST /customers/{customerId}/mandates` | `untested` | - | - | - | - | no log yet | `-` | - |
| `create_payment` | `POST /payments` | `untested` | - | - | - | - | no log yet | `-` | - |
| `create_payment_link` | `POST /payment-links` | `untested` | - | - | - | - | no log yet | `-` | - |
| `create_profile` | `POST /profiles` | `untested` | - | - | - | - | no log yet | `-` | - |
| `create_refund` | `POST /payments/{paymentId}/refunds` | `untested` | - | - | - | - | no log yet | `-` | - |
| `create_sales_invoice` | `POST /sales-invoices` | `untested` | - | - | - | - | no log yet | `-` | - |
| `create_subscription` | `POST /customers/{customerId}/subscriptions` | `untested` | - | - | - | - | no log yet | `-` | - |
| `create_webhook` | `POST /webhooks` | `untested` | - | - | - | - | no log yet | `-` | - |
| `delete_customer` | `DELETE /customers/{customerId}` | `untested` | - | - | - | - | no log yet | `-` | - |
| `delete_payment_link` | `DELETE /payment-links/{paymentLinkId}` | `untested` | - | - | - | - | no log yet | `-` | - |
| `delete_profile` | `DELETE /profiles/{id}` | `untested` | - | - | - | - | no log yet | `-` | - |
| `delete_sales_invoice` | `DELETE /sales-invoices/{id}` | `untested` | - | - | - | - | no log yet | `-` | - |
| `delete_webhook` | `DELETE /webhooks/{id}` | `untested` | - | - | - | - | no log yet | `-` | - |
| `disable_method` | `DELETE /profiles/{profileId}/methods/{id}` | `untested` | - | - | - | - | no log yet | `-` | - |
| `disable_method_issuer` | `DELETE /profiles/{profileId}/methods/{methodId}/issuers/{id}` | `untested` | - | - | - | - | no log yet | `-` | - |
| `enable_method` | `POST /profiles/{profileId}/methods/{id}` | `untested` | - | - | - | - | no log yet | `-` | - |
| `enable_method_issuer` | `POST /profiles/{profileId}/methods/{methodId}/issuers/{id}` | `untested` | - | - | - | - | no log yet | `-` | - |
| `get_balance` | `GET /balances/{id}` | `untested` | - | - | - | - | no log yet | `-` | - |
| `get_balance_report` | `GET /balances/{balanceId}/report` | `untested` | - | - | - | - | no log yet | `-` | - |
| `get_capture` | `GET /payments/{paymentId}/captures/{captureId}` | `untested` | - | - | - | - | no log yet | `-` | - |
| `get_chargeback` | `GET /payments/{paymentId}/chargebacks/{chargebackId}` | `untested` | - | - | - | - | no log yet | `-` | - |
| `get_client` | `GET /clients/{id}` | `untested` | - | - | - | - | no log yet | `-` | - |
| `get_connect_balance_transfer` | `GET /connect/balance-transfers/{id}` | `untested` | - | - | - | - | no log yet | `-` | - |
| `get_current_organization` | `GET /organizations/me` | `untested` | - | - | - | - | no log yet | `-` | - |
| `get_current_profile` | `GET /profiles/me` | `untested` | - | - | - | - | no log yet | `-` | - |
| `get_customer` | `GET /customers/{customerId}` | `untested` | - | - | - | - | no log yet | `-` | - |
| `get_invoice` | `GET /invoices/{id}` | `untested` | - | - | - | - | no log yet | `-` | - |
| `get_mandate` | `GET /customers/{customerId}/mandates/{mandateId}` | `untested` | - | - | - | - | no log yet | `-` | - |
| `get_method` | `GET /methods/{id}` | `untested` | - | - | - | - | no log yet | `-` | - |
| `get_next_settlement` | `GET /settlements/next` | `untested` | - | - | - | - | no log yet | `-` | - |
| `get_onboarding_status` | `GET /onboarding/me` | `untested` | - | - | - | - | no log yet | `-` | - |
| `get_open_settlement` | `GET /settlements/open` | `untested` | - | - | - | - | no log yet | `-` | - |
| `get_organization` | `GET /organizations/{id}` | `untested` | - | - | - | - | no log yet | `-` | - |
| `get_partner_status` | `GET /organizations/me/partner` | `untested` | - | - | - | - | no log yet | `-` | - |
| `get_payment` | `GET /payments/{paymentId}` | `untested` | - | - | - | - | no log yet | `-` | - |
| `get_payment_link` | `GET /payment-links/{paymentLinkId}` | `untested` | - | - | - | - | no log yet | `-` | - |
| `get_payment_link_payments` | `GET /payment-links/{paymentLinkId}/payments` | `untested` | - | - | - | - | no log yet | `-` | - |
| `get_permission` | `GET /permissions/{permissionId}` | `untested` | - | - | - | - | no log yet | `-` | - |
| `get_primary_balance` | `GET /balances/primary` | `untested` | - | - | - | - | no log yet | `-` | - |
| `get_profile` | `GET /profiles/{id}` | `untested` | - | - | - | - | no log yet | `-` | - |
| `get_refund` | `GET /payments/{paymentId}/refunds/{refundId}` | `untested` | - | - | - | - | no log yet | `-` | - |
| `get_sales_invoice` | `GET /sales-invoices/{id}` | `untested` | - | - | - | - | no log yet | `-` | - |
| `get_settlement` | `GET /settlements/{id}` | `untested` | - | - | - | - | no log yet | `-` | - |
| `get_subscription` | `GET /customers/{customerId}/subscriptions/{subscriptionId}` | `untested` | - | - | - | - | no log yet | `-` | - |
| `get_terminal` | `GET /terminals/{terminalId}` | `untested` | - | - | - | - | no log yet | `-` | - |
| `get_webhook` | `GET /webhooks/{id}` | `untested` | - | - | - | - | no log yet | `-` | - |
| `get_webhook_event` | `GET /events/{id}` | `untested` | - | - | - | - | no log yet | `-` | - |
| `list_balance_transactions` | `GET /balances/{balanceId}/transactions` | `untested` | - | - | - | - | no log yet | `-` | - |
| `list_capabilities` | `GET /capabilities` | `untested` | - | - | - | - | no log yet | `-` | - |
| `list_clients` | `GET /clients` | `untested` | - | - | - | - | no log yet | `-` | - |
| `list_connect_balance_transfers` | `GET /connect/balance-transfers` | `untested` | - | - | - | - | no log yet | `-` | - |
| `list_customer_payments` | `GET /customers/{customerId}/payments` | `untested` | - | - | - | - | no log yet | `-` | - |
| `list_permissions` | `GET /permissions` | `untested` | - | - | - | - | no log yet | `-` | - |
| `list_profiles` | `GET /profiles` | `untested` | - | - | - | - | no log yet | `-` | - |
| `list_settlement_captures` | `GET /settlements/{settlementId}/captures` | `untested` | - | - | - | - | no log yet | `-` | - |
| `list_settlement_chargebacks` | `GET /settlements/{settlementId}/chargebacks` | `untested` | - | - | - | - | no log yet | `-` | - |
| `list_settlement_payments` | `GET /settlements/{settlementId}/payments` | `untested` | - | - | - | - | no log yet | `-` | - |
| `list_settlement_refunds` | `GET /settlements/{settlementId}/refunds` | `untested` | - | - | - | - | no log yet | `-` | - |
| `list_subscription_payments` | `GET /customers/{customerId}/subscriptions/{subscriptionId}/payments` | `untested` | - | - | - | - | no log yet | `-` | - |
| `payment_create_route` | `POST /payments/{paymentId}/routes` | `untested` | - | - | - | - | no log yet | `-` | - |
| `payment_list_routes` | `GET /payments/{paymentId}/routes` | `untested` | - | - | - | - | no log yet | `-` | - |
| `release_authorization` | `POST /payments/{paymentId}/release-authorization` | `untested` | - | - | - | - | no log yet | `-` | - |
| `request_apple_pay_payment_session` | `POST /wallets/applepay/sessions` | `untested` | - | - | - | - | no log yet | `-` | - |
| `revoke_mandate` | `DELETE /customers/{customerId}/mandates/{mandateId}` | `untested` | - | - | - | - | no log yet | `-` | - |
| `submit_onboarding_data` | `POST /onboarding/me` | `untested` | - | - | - | - | no log yet | `-` | - |
| `test_webhook` | `POST /webhooks/{id}/ping` | `untested` | - | - | - | - | no log yet | `-` | - |
| `update_customer` | `PATCH /customers/{customerId}` | `untested` | - | - | - | - | no log yet | `-` | - |
| `update_payment` | `PATCH /payments/{paymentId}` | `untested` | - | - | - | - | no log yet | `-` | - |
| `update_payment_link` | `PATCH /payment-links/{paymentLinkId}` | `untested` | - | - | - | - | no log yet | `-` | - |
| `update_profile` | `PATCH /profiles/{id}` | `untested` | - | - | - | - | no log yet | `-` | - |
| `update_sales_invoice` | `PATCH /sales-invoices/{id}` | `untested` | - | - | - | - | no log yet | `-` | - |
| `update_subscription` | `PATCH /customers/{customerId}/subscriptions/{subscriptionId}` | `untested` | - | - | - | - | no log yet | `-` | - |
| `update_webhook` | `PATCH /webhooks/{id}` | `untested` | - | - | - | - | no log yet | `-` | - |

