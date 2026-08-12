# Postman response matrix

Generated from six Mollie Postman collections. The collections themselves are
copyright Mollie B.V. and are **not** redistributed in this repository (see
[`NOTICE`](../NOTICE)); only the deduplicated response fixtures below are kept.

- **Unique error bodies:** 30 (full HAL in `tests/fixtures/postman_error_responses.json`)
- **Unique success shapes:** 112 (index in `tests/fixtures/postman_success_response_index.json`)

Every unique **error** body is exercised by `tests/postman_all_responses.rs` through the shared
error factory / catalog / envelope (`ok: false`, code, key, message_key, title, detail, documentation).

Global **429** uses a single factory: `factory::rate_limit_exceeded()` / `MollieError::rate_limit_exceeded()`,
including when returned from `list_clients` (`GET /clients`), `list_capabilities`, and every other route.

## Unique error bodies

Full `detail` text is kept (**not truncated**). Prefer the JSON fixture for the complete HAL `_links` object.

| Status | Title | Detail | Example routes | Catalog / factory |
| --- | --- | --- | --- | --- |
| 400 | Bad Request | Invalid cursor value | `GET /business-accounts/accounts/:businessAccountId/transactions`<br>`GET /business-accounts/accounts`<br>`GET /balances/:balanceId/transactions`<br>`GET /balances` | INVALID_CURSOR / factory::invalid_cursor |
| 403 | Forbidden | Profile limit has been reached for demo accounts. | `POST /v2/profiles` | DEMO_PROFILE_LIMIT_REACHED |
| 403 | Forbidden | This profile cannot be edited because it belongs to a demo account. | `PATCH /v2/profiles/:profileId` | DEMO_PROFILE_NOT_EDITABLE |
| 404 | Not Found | No entity exists with token 'uct_abcDEFghij123456789' | `GET /business-accounts/accounts/:businessAccountId/transactions/:transactionId`<br>`GET /business-accounts/accounts/:businessAccountId`<br>`GET /business-accounts/transfers/:businessAccountsTransferId`<br>`GET /balances/:balanceId/report` | ENTITY_NOT_FOUND / factory::entity_not_found |
| 409 | Conflict | The payout cannot be canceled in its current state. | `DELETE /payouts/:payoutId` | PAYOUT_NOT_CANCELABLE / factory::payout_not_cancelable |
| 410 | Gone | Profile with token pfl_QkEhN94Ba has been deleted. | `GET /v2/profiles/:profileId`<br>`PATCH /v2/profiles/:profileId`<br>`DELETE /v2/profiles/:profileId` | PROFILE_DELETED / factory::profile_deleted |
| 422 | Unprocessable Entity | At least the 'name', 'website', 'email', 'phone', 'categoryCode', 'businessCategory', 'description', 'countriesOfActivity' or 'mode' field has to be provided | `PATCH /v2/profiles/:profileId` | VALIDATION_ERROR / factory::validation_error |
| 422 | Unprocessable Entity | Cannot create a shipment or cancelation for these lines, as none of the lines can be shipped or canceled. | `POST /orders/:orderId/shipments` | VALIDATION_ERROR / factory::validation_error |
| 422 | Unprocessable Entity | Field 'paymentTerm' must be one of the following: ... | `PATCH /sales-invoices/:salesInvoiceId` | VALIDATION_ERROR / factory::validation_error |
| 422 | Unprocessable Entity | Field 'status' must be provided | `POST /sales-invoices` | VALIDATION_ERROR / factory::validation_error |
| 422 | Unprocessable entity | Invalid URL provided | `PATCH /v2/webhooks/:webhookId`<br>`POST /v2/webhooks` | VALIDATION_ERROR / factory::validation_error |
| 422 | Unprocessable Entity | Invoice cannot be deleted unless it is draft status. | `DELETE /sales-invoices/:salesInvoiceId` | RESOURCE_STATE_CONFLICT / factory::resource_state_conflict |
| 422 | Unprocessable Entity | The 'amount' field is missing | `POST /orders/:orderId/refunds` | VALIDATION_ERROR / factory::validation_error |
| 422 | Unprocessable Entity | The 'balanceId' field is invalid. | `POST /payouts` | VALIDATION_ERROR / factory::validation_error |
| 422 | Unprocessable Entity | The 'creditor.account.iban' field is invalid | `POST /business-accounts/transfers` | VALIDATION_ERROR / factory::validation_error |
| 422 | Unprocessable Entity | The 'creditorBankAccount.accountNumber' field is missing | `POST /business-accounts/payee-verifications` | VALIDATION_ERROR / factory::validation_error |
| 422 | Unprocessable Entity | The 'description' field is missing | `POST /customers/:customerId/payments` | VALIDATION_ERROR / factory::validation_error |
| 422 | Unprocessable Entity | The 'from' field is after 'until' field | `GET /balances/:balanceId/report` | VALIDATION_ERROR / factory::validation_error |
| 422 | Unprocessable Entity | The 'orderNumber' field is missing | `POST /orders` | VALIDATION_ERROR / factory::validation_error |
| 422 | Unprocessable Entity | The 'owner' field is missing | `POST /v2/client-links` | VALIDATION_ERROR / factory::validation_error |
| 422 | Unprocessable Entity | The 'website' field is missing | `POST /v2/profiles` | VALIDATION_ERROR / factory::validation_error |
| 422 | Unprocessable Entity | The amount contains an invalid value | `POST /v2/connect/balance-transfers` | VALIDATION_ERROR / factory::validation_error |
| 422 | Unprocessable Entity | The order cannot be cancelled | `DELETE /orders/:orderId` | RESOURCE_STATE_CONFLICT / factory::resource_state_conflict |
| 422 | Unprocessable Entity | The redirect URL cannot be updated when the order is finalized | `PATCH /orders/:orderId` | RESOURCE_STATE_CONFLICT / factory::resource_state_conflict |
| 422 | Unprocessable entity | This subscription was already deleted. | `POST /v2/webhooks/:webhookId/ping`<br>`GET /v2/webhooks/:webhookId`<br>`DELETE /v2/webhooks/:webhookId` | RESOURCE_STATE_CONFLICT / factory::resource_state_conflict |
| 422 | Unprocessable Entity | Update authorization not allowed | `PATCH /orders/:orderId/lines/:orderlineId`<br>`DELETE /orders/:orderId/lines` | RESOURCE_STATE_CONFLICT / factory::resource_state_conflict |
| 429 | Too Many Requests | You have exceeded the rate limit. Please slow down your requests. | `GET /business-accounts/accounts/:businessAccountId/transactions/:transactionId`<br>`GET /business-accounts/accounts/:businessAccountId/transactions`<br>`GET /business-accounts/accounts/:businessAccountId`<br>`GET /business-accounts/accounts` | RATE_LIMIT_EXCEEDED / factory::rate_limit_exceeded (global) |
| 503 | Service Unavailable | An unexpected error occurred while processing the transfer. Please try again later. | `POST /business-accounts/transfers` | SERVICE_TEMPORARILY_UNAVAILABLE / factory::service_temporarily_unavailable |
| 503 | Service Unavailable | An unexpected error occurred while processing the verification request. Please try again later. | `POST /business-accounts/payee-verifications` | SERVICE_TEMPORARILY_UNAVAILABLE / factory::service_temporarily_unavailable |
| 503 | Service Unavailable | Payment platform for this payment method temporarily not available | `POST /customers/:customerId/payments` | SERVICE_TEMPORARILY_UNAVAILABLE / factory::service_temporarily_unavailable |

## Success responses (index)

Success samples are indexed by method/path/status/top-level keys (not full bodies).
Typed success uses `ResponseEnvelope<T>` + `to_success_envelope()` on existing route methods.

| Collection | Unique success shapes |
| --- | ---: |
| Business Accounts | 7 |
| Business operations | 19 |
| Mollie Connect | 30 |
| Receiving orders | 22 |
| Recurring | 29 |
| Revenue Collection | 5 |

### Success shapes by status

| HTTP | Count |
| ---: | ---: |
| 200 | 74 |
| 201 | 29 |
| 202 | 1 |
| 204 | 8 |

