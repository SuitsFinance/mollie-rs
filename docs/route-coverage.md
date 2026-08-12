# Mollie Route Coverage

The checked-in `specs.yaml` / `specs-3.0.yaml` currently declare **124** OpenAPI operations. The `src/routes` modules expose **124** public async route methods on `Client`, so every operation in the local pin has a typed Rust method (**Tier G**).

This is **not** the same as Tier-S facade coverage. Handwritten domain APIs live under `src/domain/` (payments, refunds, captures, mandates, payment links, subscriptions, webhooks). See `docs/sdd/1.0-readiness/00-baseline.md` and `docs/contracts/operation-coverage.md`.

The ergonomic `MollieClient` facade dereferences to `Client`; call generated methods directly on either type. Facade-owned credential types validate authentication inputs without replacing typed request/response models.

See `docs/route-examples.md` for call-shape examples. Test-mode query support is operation-specific (`contracts/test-mode.md`).

## Example Pattern

```rust,no_run
use mollie_rs::{IntoMollieFuture, MollieClient};

# async fn list() -> Result<(), mollie_rs::MollieError> {
let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;
let balances = client
    .list_balances(None, None, None, Some(true), None)
    .into_mollie_data()
    .await?;
# let _ = balances;
# Ok(())
# }
```

## Operation Groups

| Area | Generated methods (Tier G) |
| --- | --- |
| OAuth | `oauth_generate_tokens`, `oauth_revoke_tokens` |
| Balances | `list_balances`, `get_balance`, `get_primary_balance`, `get_balance_report`, `list_balance_transactions` |
| Settlements | `list_settlements`, `get_settlement`, `get_open_settlement`, `get_next_settlement`, `list_settlement_payments`, `list_settlement_captures`, `list_settlement_refunds`, `list_settlement_chargebacks` |
| Invoices | `list_invoices`, `get_invoice` |
| Permissions | `list_permissions`, `get_permission` |
| Organizations | `get_organization`, `get_current_organization`, `get_partner_status` |
| Profiles and onboarding | `list_profiles`, `create_profile`, `get_profile`, `delete_profile`, `update_profile`, `get_current_profile`, `get_onboarding_status`, `submit_onboarding_data`, `list_capabilities` |
| Clients | `list_clients`, `get_client`, `create_client_link` |
| Webhooks and events | `list_webhooks`, `create_webhook`, `get_webhook`, `delete_webhook`, `update_webhook`, `test_webhook`, `get_webhook_event` |
| Connect balance transfers | `list_connect_balance_transfers`, `create_connect_balance_transfer`, `get_connect_balance_transfer` |
| Business accounts | account list/get + transaction list/get (see `routes/accounts`) |
| Transfers | `create_transfer`, `get_transfer` |
| Payouts | `list_payouts`, `create_payout`, `get_payout`, `cancel_payout` |
| Verify payee | `verify_payee` |
| Unmatched credit transfers | list/get/match/return (see `routes/unmatched_credit_transfers`) |
| Sessions | `create_session`, `get_session` |
| Payments | `list_payments`, `create_payment`, `get_payment`, `cancel_payment`, `update_payment`, `release_authorization` |
| Methods | `list_methods`, `list_all_methods`, `get_method`, `enable_method`, `disable_method`, `enable_method_issuer`, `disable_method_issuer` |
| Refunds | `list_refunds`, `create_refund`, `get_refund`, `cancel_refund`, `list_all_refunds` |
| Chargebacks | `list_chargebacks`, `get_chargeback`, `list_all_chargebacks` |
| Captures | `list_captures`, `create_capture`, `get_capture` |
| Wallets | `request_apple_pay_payment_session` |
| Payment links | `list_payment_links`, `create_payment_link`, `get_payment_link`, `delete_payment_link`, `update_payment_link`, `get_payment_link_payments` |
| Terminals | list/get + pairing-code operations (see `routes/terminals`) |
| Payment routes | `payment_list_routes`, `payment_create_route`, `payment_get_route` |
| Customers | `list_customers`, `create_customer`, `get_customer`, `delete_customer`, `update_customer`, `list_customer_payments`, `create_customer_payment` |
| Mandates | `list_mandates`, `create_mandate`, `get_mandate`, `revoke_mandate` |
| Subscriptions | `list_subscriptions`, `create_subscription`, `get_subscription`, `cancel_subscription`, `update_subscription`, `list_all_subscriptions`, `list_subscription_payments` |
| Sales invoices | `list_sales_invoices`, `create_sales_invoice`, `get_sales_invoice`, `delete_sales_invoice`, `update_sales_invoice` |

Full machine-readable inventory: `docs/api-drift-report.md`, `docs/registries/operation-registry.yaml`.

## Drift Check

Use these commands after route surface changes:

```powershell
rg "operationId:" specs.yaml | Measure-Object -Line
rg "pub async fn" src/routes | Measure-Object -Line
powershell -ExecutionPolicy Bypass -File scripts/generate_openapi_client.ps1
powershell -ExecutionPolicy Bypass -File scripts/check_route_examples.ps1
```

```sh
grep -c "operationId:" specs.yaml
grep -R "pub async fn" src/routes | wc -l
sh scripts/generate_openapi_client.sh
sh scripts/check_route_examples.sh
```

Both counts should match, and the route example check should report markdown and Rust example coverage for every route method.
