# Route Examples

This file gives one call-shape example for every public async route method in `src/routes`. The API verb and path are copied from the Rustdoc comments, and each example calls the method exactly as it appears on `Client` and `MollieClient`.

Examples that need generated request bodies or typed token structs use concrete generated values. Required request payloads are created with `Default` when available, token newtypes use their generated `TryFrom<String>` implementations, and the few required non-default request payloads use JSON fixtures.

Every generated binary accepts shared environment variables and matching Clap options; unknown `--name value` options are also accepted as route/body fixture overrides. See [`docs/example-runtime-config.md`](example-runtime-config.md). CLI values override environment values, and `EXAMPLE_BODY_JSON` / `EXAMPLE_BODY_FILE` can replace a request body.

Optional pagination `from` cursors and optional `profile_id` filters are always omitted (`None`) so first-page list calls do not send placeholder IDs (Mollie rejects fake cursors as `INVALID_CURSOR`). With API-key credentials Mollie also rejects any `profileId` query param (`must not be sent`); only set `PROFILE_ID` / `--profile-id` when using organization-level OAuth and a real `pfl_*` id.

Run `powershell -ExecutionPolicy Bypass -File scripts/generate_route_examples.ps1` or `sh scripts/generate_route_examples.sh` after route changes, then run the matching `check_route_examples` script to verify this file and the Rust examples still cover every route method.

## Methods

### `list_balances`

- Summary: List balances
- Route: `GET /balances`
- Response: `ResponseValue<ListBalancesResponse>`
- Rust example: `examples/list_balances.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::ListBalancesResponse;

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let response: ResponseValue<ListBalancesResponse> = client
        .list_balances(
            Some("EUR"),
            None,
            ::std::num::NonZeroU64::new(50),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `get_balance`

- Summary: Get balance
- Route: `GET /balances/{balanceId}`
- Response: `ResponseValue<EntityBalance>`
- Rust example: `examples/get_balance.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{BalanceToken, EntityBalance};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let balance_id: BalanceToken = BalanceToken::try_from("bal_1234567890".to_owned()).expect("valid generated token fixture");

    let response: ResponseValue<EntityBalance> = client
        .get_balance(
            &balance_id,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `get_primary_balance`

- Summary: Get primary balance
- Route: `GET /balances/primary`
- Response: `ResponseValue<EntityBalance>`
- Rust example: `examples/get_primary_balance.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::EntityBalance;

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let response: ResponseValue<EntityBalance> = client
        .get_primary_balance()
        .await;

    let _ = response;
    Ok(())
}
```

### `get_balance_report`

- Summary: Get balance report
- Route: `GET /balances/{balanceId}/report`
- Response: `ResponseValue<EntityBalanceReport>`
- Rust example: `examples/get_balance_report.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{BalanceReportGrouping, BalanceToken, EntityBalanceReport};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let balance_id: BalanceToken = BalanceToken::try_from("bal_1234567890".to_owned()).expect("valid generated token fixture");

    let response: ResponseValue<EntityBalanceReport> = client
        .get_balance_report(
            &balance_id,
            "2026-01-01",
            Some(BalanceReportGrouping::StatusBalances),
            "2026-01-01",
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `list_balance_transactions`

- Summary: List balance transactions
- Route: `GET /balances/{balanceId}/transactions`
- Response: `ResponseValue<ListBalanceTransactionsResponse>`
- Rust example: `examples/list_balance_transactions.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{BalanceToken, ListBalanceTransactionsResponse};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let balance_id: BalanceToken = BalanceToken::try_from("bal_1234567890".to_owned()).expect("valid generated token fixture");

    let response: ResponseValue<ListBalanceTransactionsResponse> = client
        .list_balance_transactions(
            &balance_id,
            None,
            ::std::num::NonZeroU64::new(50),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `list_settlements`

- Summary: List settlements
- Route: `GET /settlements`
- Response: `ResponseValue<ListSettlementsResponse>`
- Rust example: `examples/list_settlements.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{BalanceToken, Currencies, ListSettlementsResponse};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let balance_id: BalanceToken = BalanceToken::try_from("bal_1234567890".to_owned()).expect("valid generated token fixture");

    let response: ResponseValue<ListSettlementsResponse> = client
        .list_settlements(
            Some(&balance_id),
            Some(Currencies::Eur),
            None,
            ::std::num::NonZeroU64::new(50),
            Some("2026-01"),
            Some("2026"),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `get_settlement`

- Summary: Get settlement
- Route: `GET /settlements/{settlementId}`
- Response: `ResponseValue<EntitySettlement>`
- Rust example: `examples/get_settlement.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{EntitySettlement, SettlementToken};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let settlement_id: SettlementToken = from_value::<SettlementToken>(json!({}))?;

    let response: ResponseValue<EntitySettlement> = client
        .get_settlement(
            &settlement_id,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `get_open_settlement`

- Summary: Get open settlement
- Route: `GET /settlements/open`
- Response: `ResponseValue<EntitySettlement>`
- Rust example: `examples/get_open_settlement.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::EntitySettlement;

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let response: ResponseValue<EntitySettlement> = client
        .get_open_settlement()
        .await;

    let _ = response;
    Ok(())
}
```

### `get_next_settlement`

- Summary: Get next settlement
- Route: `GET /settlements/next`
- Response: `ResponseValue<EntitySettlement>`
- Rust example: `examples/get_next_settlement.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::EntitySettlement;

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let response: ResponseValue<EntitySettlement> = client
        .get_next_settlement()
        .await;

    let _ = response;
    Ok(())
}
```

### `list_settlement_payments`

- Summary: List settlement payments
- Route: `GET /settlements/{settlementId}/payments`
- Response: `ResponseValue<ListSettlementPaymentsResponse>`
- Rust example: `examples/list_settlement_payments.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{ListSettlementPaymentsResponse, SettlementToken, Sorting};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let settlement_id: SettlementToken = from_value::<SettlementToken>(json!({}))?;

    let response: ResponseValue<ListSettlementPaymentsResponse> = client
        .list_settlement_payments(
            &settlement_id,
            None,
            ::std::num::NonZeroU64::new(50),
            None,
            Some(Sorting::Desc),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `list_settlement_captures`

- Summary: List settlement captures
- Route: `GET /settlements/{settlementId}/captures`
- Response: `ResponseValue<ListSettlementCapturesResponse>`
- Rust example: `examples/list_settlement_captures.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{ListSettlementCapturesResponse, SettlementToken};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let settlement_id: SettlementToken = from_value::<SettlementToken>(json!({}))?;

    let response: ResponseValue<ListSettlementCapturesResponse> = client
        .list_settlement_captures(
            &settlement_id,
            Some("payments"),
            None,
            ::std::num::NonZeroU64::new(50),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `list_settlement_refunds`

- Summary: List settlement refunds
- Route: `GET /settlements/{settlementId}/refunds`
- Response: `ResponseValue<ListSettlementRefundsResponse>`
- Rust example: `examples/list_settlement_refunds.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{ListSettlementRefundsResponse, SettlementToken};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let settlement_id: SettlementToken = from_value::<SettlementToken>(json!({}))?;

    let response: ResponseValue<ListSettlementRefundsResponse> = client
        .list_settlement_refunds(
            &settlement_id,
            Some("payments"),
            None,
            ::std::num::NonZeroU64::new(50),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `list_settlement_chargebacks`

- Summary: List settlement chargebacks
- Route: `GET /settlements/{settlementId}/chargebacks`
- Response: `ResponseValue<ListSettlementChargebacksResponse>`
- Rust example: `examples/list_settlement_chargebacks.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{ListSettlementChargebacksResponse, SettlementToken};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let settlement_id: SettlementToken = from_value::<SettlementToken>(json!({}))?;

    let response: ResponseValue<ListSettlementChargebacksResponse> = client
        .list_settlement_chargebacks(
            &settlement_id,
            Some("payments"),
            None,
            ::std::num::NonZeroU64::new(50),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `list_invoices`

- Summary: List invoices
- Route: `GET /invoices`
- Response: `ResponseValue<ListInvoicesResponse>`
- Rust example: `examples/list_invoices.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{ListInvoicesResponse, Sorting};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let response: ResponseValue<ListInvoicesResponse> = client
        .list_invoices(
            None,
            ::std::num::NonZeroU64::new(50),
            Some("INV-12345"),
            Some(Sorting::Desc),
            Some("2026"),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `get_invoice`

- Summary: Get invoice
- Route: `GET /invoices/{invoiceId}`
- Response: `ResponseValue<EntityInvoice>`
- Rust example: `examples/get_invoice.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{EntityInvoice, InvoiceToken};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let invoice_id: InvoiceToken = from_value::<InvoiceToken>(json!({}))?;

    let response: ResponseValue<EntityInvoice> = client
        .get_invoice(
            &invoice_id,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `list_permissions`

- Summary: List permissions
- Route: `GET /permissions`
- Response: `ResponseValue<ListPermissionsResponse>`
- Rust example: `examples/list_permissions.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::ListPermissionsResponse;

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let response: ResponseValue<ListPermissionsResponse> = client
        .list_permissions()
        .await;

    let _ = response;
    Ok(())
}
```

### `get_permission`

- Summary: Get permission
- Route: `GET /permissions/{permissionId}`
- Response: `ResponseValue<EntityPermission>`
- Rust example: `examples/get_permission.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{EntityPermission, PermissionToken};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let permission_id: PermissionToken = PermissionToken::try_from("payments.read".to_owned()).expect("valid generated token fixture");

    let response: ResponseValue<EntityPermission> = client
        .get_permission(
            &permission_id,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `get_organization`

- Summary: Get organization
- Route: `GET /organizations/{organizationId}`
- Response: `ResponseValue<EntityOrganization>`
- Rust example: `examples/get_organization.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{EntityOrganization, OrganizationToken};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let organization_id: OrganizationToken = from_value::<OrganizationToken>(json!({}))?;

    let response: ResponseValue<EntityOrganization> = client
        .get_organization(
            &organization_id,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `get_current_organization`

- Summary: Get current organization
- Route: `GET /organizations/me`
- Response: `ResponseValue<EntityOrganization>`
- Rust example: `examples/get_current_organization.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::EntityOrganization;

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let response: ResponseValue<EntityOrganization> = client
        .get_current_organization()
        .await;

    let _ = response;
    Ok(())
}
```

### `get_partner_status`

- Summary: Get partner status
- Route: `GET /organizations/me/partner`
- Response: `ResponseValue<GetPartnerStatusResponse>`
- Rust example: `examples/get_partner_status.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::GetPartnerStatusResponse;

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let response: ResponseValue<GetPartnerStatusResponse> = client
        .get_partner_status()
        .await;

    let _ = response;
    Ok(())
}
```

### `list_profiles`

- Summary: List profiles
- Route: `GET /profiles`
- Response: `ResponseValue<ListProfilesResponse>`
- Rust example: `examples/list_profiles.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::ListProfilesResponse;

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let response: ResponseValue<ListProfilesResponse> = client
        .list_profiles(
            None,
            ::std::num::NonZeroU64::new(50),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `create_profile`

- Summary: Create profile
- Route: `POST /profiles`
- Response: `ResponseValue<ProfileResponse>`
- Rust example: `examples/create_profile.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{ProfileRequest, ProfileResponse};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let body: ProfileRequest = from_value::<ProfileRequest>(json!({}))?;

    let response: ResponseValue<ProfileResponse> = client
        .create_profile(
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `get_profile`

- Summary: Get profile
- Route: `GET /profiles/{profileId}`
- Response: `ResponseValue<ProfileResponse>`
- Rust example: `examples/get_profile.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{ProfileResponse, ProfileToken};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let profile_id: ProfileToken = ProfileToken::try_from("pfl_1234567890".to_owned()).expect("valid generated token fixture");

    let response: ResponseValue<ProfileResponse> = client
        .get_profile(
            &profile_id,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `delete_profile`

- Summary: Delete profile
- Route: `DELETE /profiles/{profileId}`
- Response: `ResponseValue<()>`
- Rust example: `examples/delete_profile.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::ProfileToken;

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let profile_id: ProfileToken = ProfileToken::try_from("pfl_1234567890".to_owned()).expect("valid generated token fixture");

    let response: ResponseValue<()> = client
        .delete_profile(
            &profile_id,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `update_profile`

- Summary: Update profile
- Route: `PATCH /profiles/{profileId}`
- Response: `ResponseValue<ProfileResponse>`
- Rust example: `examples/update_profile.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{ProfileResponse, ProfileToken, UpdateProfileBody};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let profile_id: ProfileToken = ProfileToken::try_from("pfl_1234567890".to_owned()).expect("valid generated token fixture");
    let body: UpdateProfileBody = UpdateProfileBody::default();

    let response: ResponseValue<ProfileResponse> = client
        .update_profile(
            &profile_id,
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `get_current_profile`

- Summary: Get current profile
- Route: `GET /profiles/me`
- Response: `ResponseValue<ProfileResponse>`
- Rust example: `examples/get_current_profile.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::ProfileResponse;

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let response: ResponseValue<ProfileResponse> = client
        .get_current_profile()
        .await;

    let _ = response;
    Ok(())
}
```

### `get_onboarding_status`

- Summary: Get onboarding status
- Route: `GET /onboarding/me`
- Response: `ResponseValue<EntityOnboardingStatus>`
- Rust example: `examples/get_onboarding_status.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::EntityOnboardingStatus;

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let response: ResponseValue<EntityOnboardingStatus> = client
        .get_onboarding_status()
        .await;

    let _ = response;
    Ok(())
}
```

### `submit_onboarding_data`

- Summary: Submit onboarding data
- Route: `POST /onboarding/me`
- Response: `ResponseValue<()>`
- Rust example: `examples/submit_onboarding_data.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::SubmitOnboardingDataBody;

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let body: SubmitOnboardingDataBody = SubmitOnboardingDataBody::default();

    let response: ResponseValue<()> = client
        .submit_onboarding_data(
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `list_capabilities`

- Summary: List capabilities
- Route: `GET /capabilities`
- Response: `ResponseValue<ListCapabilitiesResponse>`
- Rust example: `examples/list_capabilities.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::ListCapabilitiesResponse;

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let response: ResponseValue<ListCapabilitiesResponse> = client
        .list_capabilities()
        .await;

    let _ = response;
    Ok(())
}
```

### `list_clients`

- Summary: List clients
- Route: `GET /clients`
- Response: `ResponseValue<ListClientsResponse>`
- Rust example: `examples/list_clients.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::ListClientsResponse;

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let response: ResponseValue<ListClientsResponse> = client
        .list_clients(
            Some("payments"),
            None,
            ::std::num::NonZeroU64::new(50),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `get_client`

- Summary: Get client
- Route: `GET /clients/{organizationId}`
- Response: `ResponseValue<GetClientResponse>`
- Rust example: `examples/get_client.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{GetClientResponse, OrganizationToken};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let organization_id: OrganizationToken = from_value::<OrganizationToken>(json!({}))?;

    let response: ResponseValue<GetClientResponse> = client
        .get_client(
            &organization_id,
            Some("payments"),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `create_client_link`

- Summary: Create client link
- Route: `POST /client-links`
- Response: `ResponseValue<ClientLinkResponse>`
- Rust example: `examples/create_client_link.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{ClientLinkRequest, ClientLinkResponse};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let body: ClientLinkRequest = from_value::<ClientLinkRequest>(json!({}))?;

    let response: ResponseValue<ClientLinkResponse> = client
        .create_client_link(
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `list_webhooks`

- Summary: List all webhooks
- Route: `GET /webhooks`
- Response: `ResponseValue<ListWebhooksResponse>`
- Rust example: `examples/list_webhooks.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{ListWebhooksResponse, Sorting, WebhookEventTypes};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let response: ResponseValue<ListWebhooksResponse> = client
        .list_webhooks(
            Some(WebhookEventTypes::PaymentPaid),
            None,
            ::std::num::NonZeroU64::new(50),
            Some(Sorting::Desc),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `create_webhook`

- Summary: Create a webhook
- Route: `POST /webhooks`
- Response: `ResponseValue<CreateWebhook>`
- Rust example: `examples/create_webhook.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{CreateWebhook, CreateWebhookBody};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let body: CreateWebhookBody = from_value::<CreateWebhookBody>(json!({
        "eventTypes": [
            "payment-link.paid"
        ],
        "name": "Payment links webhook",
        "url": "https://example.com/webhooks/mollie"
    }))?;

    let response: ResponseValue<CreateWebhook> = client
        .create_webhook(
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `get_webhook`

- Summary: Get a webhook
- Route: `GET /webhooks/{webhookId}`
- Response: `ResponseValue<EntityWebhook>`
- Rust example: `examples/get_webhook.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{EntityWebhook, WebhookToken};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let webhook_id: WebhookToken = from_value::<WebhookToken>(json!({}))?;

    let response: ResponseValue<EntityWebhook> = client
        .get_webhook(
            &webhook_id,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `delete_webhook`

- Summary: Delete a webhook
- Route: `DELETE /webhooks/{webhookId}`
- Response: `ResponseValue<()>`
- Rust example: `examples/delete_webhook.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{DeleteWebhookBody, WebhookToken};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let webhook_id: WebhookToken = from_value::<WebhookToken>(json!({}))?;
    let body: DeleteWebhookBody = DeleteWebhookBody::default();

    let response: ResponseValue<()> = client
        .delete_webhook(
            &webhook_id,
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `update_webhook`

- Summary: Update a webhook
- Route: `PATCH /webhooks/{webhookId}`
- Response: `ResponseValue<EntityWebhook>`
- Rust example: `examples/update_webhook.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{EntityWebhook, UpdateWebhookBody, WebhookToken};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let webhook_id: WebhookToken = from_value::<WebhookToken>(json!({}))?;
    let body: UpdateWebhookBody = UpdateWebhookBody::default();

    let response: ResponseValue<EntityWebhook> = client
        .update_webhook(
            &webhook_id,
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `test_webhook`

- Summary: Test a webhook
- Route: `POST /webhooks/{webhookId}/ping`
- Response: `ResponseValue<()>`
- Rust example: `examples/test_webhook.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{TestWebhookBody, WebhookToken};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let webhook_id: WebhookToken = from_value::<WebhookToken>(json!({}))?;
    let body: TestWebhookBody = TestWebhookBody::default();

    let response: ResponseValue<()> = client
        .test_webhook(
            &webhook_id,
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `get_webhook_event`

- Summary: Get a Webhook Event
- Route: `GET /events/{webhookEventId}`
- Response: `ResponseValue<EntityWebhookEvent>`
- Rust example: `examples/get_webhook_event.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{EntityWebhookEvent, WebhookEventToken};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let webhook_event_id: WebhookEventToken = from_value::<WebhookEventToken>(json!({}))?;

    let response: ResponseValue<EntityWebhookEvent> = client
        .get_webhook_event(
            &webhook_event_id,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `list_connect_balance_transfers`

- Summary: List all Connect balance transfers
- Route: `GET /connect/balance-transfers`
- Response: `ResponseValue<ListConnectBalanceTransfersResponse>`
- Rust example: `examples/list_connect_balance_transfers.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{ListConnectBalanceTransfersResponse, Sorting};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let response: ResponseValue<ListConnectBalanceTransfersResponse> = client
        .list_connect_balance_transfers(
            None,
            ::std::num::NonZeroU64::new(50),
            Some(Sorting::Desc),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `create_connect_balance_transfer`

- Summary: Create a Connect balance transfer
- Route: `POST /connect/balance-transfers`
- Response: `ResponseValue<EntityBalanceTransferResponse>`
- Rust example: `examples/create_connect_balance_transfer.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{EntityBalanceTransfer, EntityBalanceTransferResponse};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let body: EntityBalanceTransfer = from_value::<EntityBalanceTransfer>(json!({}))?;

    let response: ResponseValue<EntityBalanceTransferResponse> = client
        .create_connect_balance_transfer(
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `get_connect_balance_transfer`

- Summary: Get a Connect balance transfer
- Route: `GET /connect/balance-transfers/{balanceTransferId}`
- Response: `ResponseValue<EntityBalanceTransferResponse>`
- Rust example: `examples/get_connect_balance_transfer.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{ConnectBalanceTransferToken, EntityBalanceTransferResponse};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let balance_transfer_id: ConnectBalanceTransferToken = from_value::<ConnectBalanceTransferToken>(json!({}))?;

    let response: ResponseValue<EntityBalanceTransferResponse> = client
        .get_connect_balance_transfer(
            &balance_transfer_id,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `list_payments`

- Summary: List payments
- Route: `GET /payments`
- Response: `ResponseValue<ListPaymentsResponse>`
- Rust example: `examples/list_payments.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{ListPaymentsResponse, Sorting};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let response: ResponseValue<ListPaymentsResponse> = client
        .list_payments(
            None,
            ::std::num::NonZeroU64::new(50),
            None,
            Some(Sorting::Desc),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `create_payment`

- Summary: Create payment
- Route: `POST /payments`
- Response: `ResponseValue<PaymentResponse>`
- Rust example: `examples/create_payment.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{PaymentRequest, PaymentResponse};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let body: PaymentRequest = from_value::<PaymentRequest>(json!({}))?;

    let response: ResponseValue<PaymentResponse> = client
        .create_payment(
            Some("issuers"),
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `get_payment`

- Summary: Get payment
- Route: `GET /payments/{paymentId}`
- Response: `ResponseValue<PaymentResponse>`
- Rust example: `examples/get_payment.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{PaymentResponse, PaymentToken};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let payment_id: PaymentToken = PaymentToken::try_from("tr_1234567890".to_owned()).expect("valid generated token fixture");

    let response: ResponseValue<PaymentResponse> = client
        .get_payment(
            &payment_id,
            Some("payments"),
            Some("issuers"),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `cancel_payment`

- Summary: Cancel payment
- Route: `DELETE /payments/{paymentId}`
- Response: `ResponseValue<PaymentResponse>`
- Rust example: `examples/cancel_payment.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{CancelPaymentBody, PaymentResponse, PaymentToken};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let payment_id: PaymentToken = PaymentToken::try_from("tr_1234567890".to_owned()).expect("valid generated token fixture");
    let body: CancelPaymentBody = CancelPaymentBody::default();

    let response: ResponseValue<PaymentResponse> = client
        .cancel_payment(
            &payment_id,
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `update_payment`

- Summary: Update payment
- Route: `PATCH /payments/{paymentId}`
- Response: `ResponseValue<PaymentResponse>`
- Rust example: `examples/update_payment.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{PaymentResponse, PaymentToken, UpdatePaymentBody};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let payment_id: PaymentToken = PaymentToken::try_from("tr_1234567890".to_owned()).expect("valid generated token fixture");
    let body: UpdatePaymentBody = UpdatePaymentBody::default();

    let response: ResponseValue<PaymentResponse> = client
        .update_payment(
            &payment_id,
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `release_authorization`

- Summary: Release payment authorization
- Route: `POST /payments/{paymentId}/release-authorization`
- Response: `ResponseValue<()>`
- Rust example: `examples/release_authorization.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{PaymentToken, ReleaseAuthorizationBody};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let payment_id: PaymentToken = PaymentToken::try_from("tr_1234567890".to_owned()).expect("valid generated token fixture");
    let body: ReleaseAuthorizationBody = ReleaseAuthorizationBody::default();

    let response: ResponseValue<()> = client
        .release_authorization(
            &payment_id,
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `payment_list_routes`

- Summary: List payment routes
- Route: `GET /payments/{paymentId}/routes`
- Response: `ResponseValue<PaymentListRoutesResponse>`
- Rust example: `examples/payment_list_routes.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{PaymentListRoutesResponse, PaymentToken};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let payment_id: PaymentToken = PaymentToken::try_from("tr_1234567890".to_owned()).expect("valid generated token fixture");

    let response: ResponseValue<PaymentListRoutesResponse> = client
        .payment_list_routes(
            &payment_id,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `payment_create_route`

- Summary: Create a delayed route
- Route: `POST /payments/{paymentId}/routes`
- Response: `ResponseValue<RouteCreateResponse>`
- Rust example: `examples/payment_create_route.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{PaymentToken, RouteCreateRequest, RouteCreateResponse};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let payment_id: PaymentToken = PaymentToken::try_from("tr_1234567890".to_owned()).expect("valid generated token fixture");
    let body: RouteCreateRequest = from_value::<RouteCreateRequest>(json!({}))?;

    let response: ResponseValue<RouteCreateResponse> = client
        .payment_create_route(
            &payment_id,
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `payment_get_route`

- Summary: Get a delayed route
- Route: `GET /payments/{paymentId}/routes/{routeId}`
- Response: `ResponseValue<RouteGetResponse>`
- Rust example: `examples/payment_get_route.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{ConnectRouteToken, PaymentToken, RouteGetResponse};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let payment_id: PaymentToken = PaymentToken::try_from("tr_1234567890".to_owned()).expect("valid generated token fixture");
    let route_id: ConnectRouteToken = from_value::<ConnectRouteToken>(json!({}))?;

    let response: ResponseValue<RouteGetResponse> = client
        .payment_get_route(
            &payment_id,
            &route_id,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `list_methods`

- Summary: List payment methods
- Route: `GET /methods`
- Response: `ResponseValue<ListMethodsResponse>`
- Rust example: `examples/list_methods.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{Amount, LineCategories, ListMethodsResponse, Locale, MethodIncludeWalletsParameter, MethodResourceParameter, SequenceType};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let amount: Amount = from_value::<Amount>(json!({
        "currency": "EUR",
        "value": "10.00"
    }))?;
    let locale: Locale = from_value::<Locale>(json!({}))?;

    let response: ResponseValue<ListMethodsResponse> = client
        .list_methods(
            Some(&amount),
            Some("NL"),
            Some("issuers"),
            Some(MethodIncludeWalletsParameter::Applepay),
            Some(&locale),
            Some(LineCategories::Eco),
            None,
            Some(MethodResourceParameter::Payments),
            Some(SequenceType::Oneoff),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `list_all_methods`

- Summary: List all payment methods
- Route: `GET /methods/all`
- Response: `ResponseValue<ListAllMethodsResponse>`
- Rust example: `examples/list_all_methods.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{Amount, ListAllMethodsResponse, Locale, SequenceType};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let amount: Amount = from_value::<Amount>(json!({
        "currency": "EUR",
        "value": "10.00"
    }))?;
    let locale: Locale = from_value::<Locale>(json!({}))?;

    let response: ResponseValue<ListAllMethodsResponse> = client
        .list_all_methods(
            Some(&amount),
            Some("issuers"),
            Some(&locale),
            None,
            Some(SequenceType::Oneoff),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `get_method`

- Summary: Get payment method
- Route: `GET /methods/{methodId}`
- Response: `ResponseValue<EntityMethodGet>`
- Rust example: `examples/get_method.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{EntityMethodGet, Locale, Method, SequenceType};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let method_id: Method = from_value::<Method>(json!({}))?;
    let locale: Locale = from_value::<Locale>(json!({}))?;

    let response: ResponseValue<EntityMethodGet> = client
        .get_method(
            &method_id,
            Some("EUR"),
            Some("issuers"),
            Some(&locale),
            None,
            Some(SequenceType::Oneoff),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `enable_method`

- Summary: Enable payment method
- Route: `POST /profiles/{profileId}/methods/{methodId}`
- Response: `ResponseValue<EntityMethodGet>`
- Rust example: `examples/enable_method.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{EnableMethodProfileId, EntityMethodGet, Method, ProfileToken};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let profile_id: EnableMethodProfileId = EnableMethodProfileId::from(ProfileToken::try_from("pfl_1234567890".to_owned()).expect("valid profile token fixture"));
    let method_id: Method = from_value::<Method>(json!({}))?;

    let response: ResponseValue<EntityMethodGet> = client
        .enable_method(
            &profile_id,
            &method_id,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `disable_method`

- Summary: Disable payment method
- Route: `DELETE /profiles/{profileId}/methods/{methodId}`
- Response: `ResponseValue<()>`
- Rust example: `examples/disable_method.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{DisableMethodProfileId, Method, ProfileToken};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let profile_id: DisableMethodProfileId = DisableMethodProfileId::from(ProfileToken::try_from("pfl_1234567890".to_owned()).expect("valid profile token fixture"));
    let method_id: Method = from_value::<Method>(json!({}))?;

    let response: ResponseValue<()> = client
        .disable_method(
            &profile_id,
            &method_id,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `enable_method_issuer`

- Summary: Enable payment method issuer
- Route: `POST /profiles/{profileId}/methods/{methodId}/issuers/{issuerId}`
- Response: `ResponseValue<EnableMethodIssuerResponse>`
- Rust example: `examples/enable_method_issuer.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{EnableMethodIssuerBody, EnableMethodIssuerIssuerId, EnableMethodIssuerProfileId, EnableMethodIssuerResponse, MethodIdWithIssuer, ProfileToken};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let profile_id: EnableMethodIssuerProfileId = EnableMethodIssuerProfileId::from(ProfileToken::try_from("pfl_1234567890".to_owned()).expect("valid profile token fixture"));
    let issuer_id: EnableMethodIssuerIssuerId = from_value::<EnableMethodIssuerIssuerId>(json!({}))?;
    let body: EnableMethodIssuerBody = EnableMethodIssuerBody::default();

    let response: ResponseValue<EnableMethodIssuerResponse> = client
        .enable_method_issuer(
            &profile_id,
            MethodIdWithIssuer::Voucher,
            &issuer_id,
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `disable_method_issuer`

- Summary: Disable payment method issuer
- Route: `DELETE /profiles/{profileId}/methods/{methodId}/issuers/{issuerId}`
- Response: `ResponseValue<()>`
- Rust example: `examples/disable_method_issuer.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{DisableMethodIssuerIssuerId, DisableMethodIssuerProfileId, MethodIdWithIssuer, ProfileToken};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let profile_id: DisableMethodIssuerProfileId = DisableMethodIssuerProfileId::from(ProfileToken::try_from("pfl_1234567890".to_owned()).expect("valid profile token fixture"));
    let issuer_id: DisableMethodIssuerIssuerId = from_value::<DisableMethodIssuerIssuerId>(json!({}))?;

    let response: ResponseValue<()> = client
        .disable_method_issuer(
            &profile_id,
            MethodIdWithIssuer::Voucher,
            &issuer_id,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `list_refunds`

- Summary: List payment refunds
- Route: `GET /payments/{paymentId}/refunds`
- Response: `ResponseValue<ListRefundsResponse>`
- Rust example: `examples/list_refunds.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{ListRefundsResponse, PaymentToken};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let payment_id: PaymentToken = PaymentToken::try_from("tr_1234567890".to_owned()).expect("valid generated token fixture");

    let response: ResponseValue<ListRefundsResponse> = client
        .list_refunds(
            &payment_id,
            Some("payments"),
            None,
            ::std::num::NonZeroU64::new(50),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `create_refund`

- Summary: Create payment refund
- Route: `POST /payments/{paymentId}/refunds`
- Response: `ResponseValue<EntityRefundResponse>`
- Rust example: `examples/create_refund.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{EntityRefundResponse, PaymentToken, RefundRequest};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let payment_id: PaymentToken = PaymentToken::try_from("tr_1234567890".to_owned()).expect("valid generated token fixture");
    let body: RefundRequest = from_value::<RefundRequest>(json!({}))?;

    let response: ResponseValue<EntityRefundResponse> = client
        .create_refund(
            &payment_id,
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `get_refund`

- Summary: Get payment refund
- Route: `GET /payments/{paymentId}/refunds/{refundId}`
- Response: `ResponseValue<EntityRefundResponse>`
- Rust example: `examples/get_refund.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{EntityRefundResponse, PaymentToken, RefundToken};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let payment_id: PaymentToken = PaymentToken::try_from("tr_1234567890".to_owned()).expect("valid generated token fixture");
    let refund_id: RefundToken = RefundToken::try_from("re_1234567890".to_owned()).expect("valid generated token fixture");

    let response: ResponseValue<EntityRefundResponse> = client
        .get_refund(
            &payment_id,
            &refund_id,
            Some("payments"),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `cancel_refund`

- Summary: Cancel payment refund
- Route: `DELETE /payments/{paymentId}/refunds/{refundId}`
- Response: `ResponseValue<()>`
- Rust example: `examples/cancel_refund.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{PaymentToken, RefundToken};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let payment_id: PaymentToken = PaymentToken::try_from("tr_1234567890".to_owned()).expect("valid generated token fixture");
    let refund_id: RefundToken = RefundToken::try_from("re_1234567890".to_owned()).expect("valid generated token fixture");

    let response: ResponseValue<()> = client
        .cancel_refund(
            &payment_id,
            &refund_id,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `list_all_refunds`

- Summary: List all refunds
- Route: `GET /refunds`
- Response: `ResponseValue<ListAllRefundsResponse>`
- Rust example: `examples/list_all_refunds.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{ListAllRefundsResponse, Sorting};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let response: ResponseValue<ListAllRefundsResponse> = client
        .list_all_refunds(
            Some("payments"),
            None,
            ::std::num::NonZeroU64::new(50),
            None,
            Some(Sorting::Desc),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `list_chargebacks`

- Summary: List payment chargebacks
- Route: `GET /payments/{paymentId}/chargebacks`
- Response: `ResponseValue<ListChargebacksResponse>`
- Rust example: `examples/list_chargebacks.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{ListChargebacksResponse, PaymentToken};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let payment_id: PaymentToken = PaymentToken::try_from("tr_1234567890".to_owned()).expect("valid generated token fixture");

    let response: ResponseValue<ListChargebacksResponse> = client
        .list_chargebacks(
            &payment_id,
            Some("payments"),
            None,
            ::std::num::NonZeroU64::new(50),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `get_chargeback`

- Summary: Get payment chargeback
- Route: `GET /payments/{paymentId}/chargebacks/{chargebackId}`
- Response: `ResponseValue<EntityChargeback>`
- Rust example: `examples/get_chargeback.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{ChargebackToken, EntityChargeback, PaymentToken};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let payment_id: PaymentToken = PaymentToken::try_from("tr_1234567890".to_owned()).expect("valid generated token fixture");
    let chargeback_id: ChargebackToken = ChargebackToken::try_from("chb_1234567890".to_owned()).expect("valid generated token fixture");

    let response: ResponseValue<EntityChargeback> = client
        .get_chargeback(
            &payment_id,
            &chargeback_id,
            Some("payments"),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `list_all_chargebacks`

- Summary: List all chargebacks
- Route: `GET /chargebacks`
- Response: `ResponseValue<ListAllChargebacksResponse>`
- Rust example: `examples/list_all_chargebacks.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{ListAllChargebacksResponse, Sorting};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let response: ResponseValue<ListAllChargebacksResponse> = client
        .list_all_chargebacks(
            Some("payments"),
            None,
            ::std::num::NonZeroU64::new(50),
            None,
            Some(Sorting::Desc),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `list_captures`

- Summary: List captures
- Route: `GET /payments/{paymentId}/captures`
- Response: `ResponseValue<ListCapturesResponse>`
- Rust example: `examples/list_captures.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{ListCapturesResponse, PaymentToken};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let payment_id: PaymentToken = PaymentToken::try_from("tr_1234567890".to_owned()).expect("valid generated token fixture");

    let response: ResponseValue<ListCapturesResponse> = client
        .list_captures(
            &payment_id,
            Some("payments"),
            None,
            ::std::num::NonZeroU64::new(50),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `create_capture`

- Summary: Create capture
- Route: `POST /payments/{paymentId}/captures`
- Response: `ResponseValue<CaptureResponse>`
- Rust example: `examples/create_capture.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{CaptureResponse, EntityCapture, PaymentToken};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let payment_id: PaymentToken = PaymentToken::try_from("tr_1234567890".to_owned()).expect("valid generated token fixture");
    let body: EntityCapture = EntityCapture::default();

    let response: ResponseValue<CaptureResponse> = client
        .create_capture(
            &payment_id,
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `get_capture`

- Summary: Get capture
- Route: `GET /payments/{paymentId}/captures/{captureId}`
- Response: `ResponseValue<CaptureResponse>`
- Rust example: `examples/get_capture.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{CaptureResponse, CaptureToken, PaymentToken};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let payment_id: PaymentToken = PaymentToken::try_from("tr_1234567890".to_owned()).expect("valid generated token fixture");
    let capture_id: CaptureToken = CaptureToken::try_from("cpt_1234567890".to_owned()).expect("valid generated token fixture");

    let response: ResponseValue<CaptureResponse> = client
        .get_capture(
            &payment_id,
            &capture_id,
            Some("payments"),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `request_apple_pay_payment_session`

- Summary: Request Apple Pay payment session
- Route: `POST /wallets/applepay/sessions`
- Response: `ResponseValue<EntitySession2>`
- Rust example: `examples/request_apple_pay_payment_session.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{EntitySession2, RequestApplePayPaymentSessionBody};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let body: RequestApplePayPaymentSessionBody = from_value::<RequestApplePayPaymentSessionBody>(json!({
        "domain": "pay.example.com",
        "validationUrl": "https://apple-pay-gateway-cert.apple.com/paymentservices/paymentSession"
    }))?;

    let response: ResponseValue<EntitySession2> = client
        .request_apple_pay_payment_session(
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `list_payment_links`

- Summary: List payment links
- Route: `GET /payment-links`
- Response: `ResponseValue<ListPaymentLinksResponse>`
- Rust example: `examples/list_payment_links.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::ListPaymentLinksResponse;

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let response: ResponseValue<ListPaymentLinksResponse> = client
        .list_payment_links(
            None,
            ::std::num::NonZeroU64::new(50),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `create_payment_link`

- Summary: Create payment link
- Route: `POST /payment-links`
- Response: `ResponseValue<PaymentLinkResponse>`
- Rust example: `examples/create_payment_link.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{CreatePaymentLinkBody, PaymentLinkResponse};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let body: CreatePaymentLinkBody = from_value::<CreatePaymentLinkBody>(json!({
        "amount": {
            "currency": "EUR",
            "value": "10.00"
        },
        "description": "Order #12345"
    }))?;

    let response: ResponseValue<PaymentLinkResponse> = client
        .create_payment_link(
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `get_payment_link`

- Summary: Get payment link
- Route: `GET /payment-links/{paymentLinkId}`
- Response: `ResponseValue<PaymentLinkResponse>`
- Rust example: `examples/get_payment_link.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{PaymentLinkResponse, PaymentLinkToken};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let payment_link_id: PaymentLinkToken = PaymentLinkToken::try_from("pl_1234567890".to_owned()).expect("valid generated token fixture");

    let response: ResponseValue<PaymentLinkResponse> = client
        .get_payment_link(
            &payment_link_id,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `delete_payment_link`

- Summary: Delete payment link
- Route: `DELETE /payment-links/{paymentLinkId}`
- Response: `ResponseValue<()>`
- Rust example: `examples/delete_payment_link.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{DeletePaymentLinkBody, PaymentLinkToken};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let payment_link_id: PaymentLinkToken = PaymentLinkToken::try_from("pl_1234567890".to_owned()).expect("valid generated token fixture");
    let body: DeletePaymentLinkBody = DeletePaymentLinkBody::default();

    let response: ResponseValue<()> = client
        .delete_payment_link(
            &payment_link_id,
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `update_payment_link`

- Summary: Update payment link
- Route: `PATCH /payment-links/{paymentLinkId}`
- Response: `ResponseValue<PaymentLinkResponse>`
- Rust example: `examples/update_payment_link.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{PaymentLinkResponse, PaymentLinkToken, UpdatePaymentLinkBody};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let payment_link_id: PaymentLinkToken = PaymentLinkToken::try_from("pl_1234567890".to_owned()).expect("valid generated token fixture");
    let body: UpdatePaymentLinkBody = UpdatePaymentLinkBody::default();

    let response: ResponseValue<PaymentLinkResponse> = client
        .update_payment_link(
            &payment_link_id,
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `get_payment_link_payments`

- Summary: Get payment link payments
- Route: `GET /payment-links/{paymentLinkId}/payments`
- Response: `ResponseValue<GetPaymentLinkPaymentsResponse>`
- Rust example: `examples/get_payment_link_payments.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{GetPaymentLinkPaymentsResponse, PaymentLinkToken, Sorting};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let payment_link_id: PaymentLinkToken = PaymentLinkToken::try_from("pl_1234567890".to_owned()).expect("valid generated token fixture");

    let response: ResponseValue<GetPaymentLinkPaymentsResponse> = client
        .get_payment_link_payments(
            &payment_link_id,
            None,
            ::std::num::NonZeroU64::new(50),
            Some(Sorting::Desc),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `list_terminals`

- Summary: List terminals
- Route: `GET /terminals`
- Response: `ResponseValue<ListTerminalsResponse>`
- Rust example: `examples/list_terminals.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{ListTerminalsResponse, Sorting};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let response: ResponseValue<ListTerminalsResponse> = client
        .list_terminals(
            None,
            ::std::num::NonZeroU64::new(50),
            Some(Sorting::Desc),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `get_terminal`

- Summary: Get terminal
- Route: `GET /terminals/{terminalId}`
- Response: `ResponseValue<EntityTerminal>`
- Rust example: `examples/get_terminal.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{EntityTerminal, TerminalToken};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let terminal_id: TerminalToken = TerminalToken::try_from("term_1234567890".to_owned()).expect("valid generated token fixture");

    let response: ResponseValue<EntityTerminal> = client
        .get_terminal(
            &terminal_id,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `terminals_list_pairing_codes`

- Summary: List terminal pairing codes
- Route: `GET /terminals/pairing-codes`
- Response: `ResponseValue<TerminalsListPairingCodesResponse>`
- Rust example: `examples/terminals_list_pairing_codes.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{Sorting, TerminalsListPairingCodesResponse};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let response: ResponseValue<TerminalsListPairingCodesResponse> = client
        .terminals_list_pairing_codes(
            None,
            ::std::num::NonZeroU64::new(50),
            None,
            Some(Sorting::Desc),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `terminals_request_pairing_code`

- Summary: Request terminal pairing code
- Route: `POST /terminals/pairing-codes`
- Response: `ResponseValue<EntityPairingCode>`
- Rust example: `examples/terminals_request_pairing_code.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{EntityPairingCode, TerminalsRequestPairingCodeBody};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let body: TerminalsRequestPairingCodeBody = from_value::<TerminalsRequestPairingCodeBody>(json!({}))?;

    let response: ResponseValue<EntityPairingCode> = client
        .terminals_request_pairing_code(
            Some("issuers"),
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `terminals_get_pairing_code`

- Summary: Get terminal pairing code
- Route: `GET /terminals/pairing-codes/{pairingCodeId}`
- Response: `ResponseValue<EntityPairingCode>`
- Rust example: `examples/terminals_get_pairing_code.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{EntityPairingCode, TerminalPairingCodeToken};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let pairing_code_id: TerminalPairingCodeToken = from_value::<TerminalPairingCodeToken>(json!({}))?;

    let response: ResponseValue<EntityPairingCode> = client
        .terminals_get_pairing_code(
            &pairing_code_id,
            Some("issuers"),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `terminals_revoke_pairing_code`

- Summary: Revoke terminal pairing code
- Route: `DELETE /terminals/pairing-codes/{pairingCodeId}`
- Response: `ResponseValue<EntityPairingCode>`
- Rust example: `examples/terminals_revoke_pairing_code.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{EntityPairingCode, TerminalPairingCodeToken};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let pairing_code_id: TerminalPairingCodeToken = from_value::<TerminalPairingCodeToken>(json!({}))?;

    let response: ResponseValue<EntityPairingCode> = client
        .terminals_revoke_pairing_code(
            &pairing_code_id,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `list_business_accounts`

- Summary: List business accounts
- Route: `GET /business-accounts/accounts`
- Response: `ResponseValue<ListBusinessAccountsResponse>`
- Rust example: `examples/list_business_accounts.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{ListBusinessAccountsResponse, Sorting};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let response: ResponseValue<ListBusinessAccountsResponse> = client
        .list_business_accounts(
            None,
            ::std::num::NonZeroU64::new(50),
            Some(Sorting::Desc),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `get_business_account`

- Summary: Get business account
- Route: `GET /business-accounts/accounts/{businessAccountId}`
- Response: `ResponseValue<BusinessAccountResponse>`
- Rust example: `examples/get_business_account.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{BusinessAccountResponse, BusinessAccountToken};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let business_account_id: BusinessAccountToken = from_value::<BusinessAccountToken>(json!({}))?;

    let response: ResponseValue<BusinessAccountResponse> = client
        .get_business_account(
            &business_account_id,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `list_business_account_transactions`

- Summary: List transactions
- Route: `GET /business-accounts/accounts/{businessAccountId}/transactions`
- Response: `ResponseValue<ListBusinessAccountTransactionsResponse>`
- Rust example: `examples/list_business_account_transactions.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{BusinessAccountToken, ListBusinessAccountTransactionsResponse, Sorting};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let business_account_id: BusinessAccountToken = from_value::<BusinessAccountToken>(json!({}))?;

    let response: ResponseValue<ListBusinessAccountTransactionsResponse> = client
        .list_business_account_transactions(
            &business_account_id,
            None,
            ::std::num::NonZeroU64::new(50),
            Some(Sorting::Desc),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `get_business_account_transaction`

- Summary: Get transaction
- Route: `GET /business-accounts/accounts/{businessAccountId}/transactions/{transactionId}`
- Response: `ResponseValue<TransactionResponse>`
- Rust example: `examples/get_business_account_transaction.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{BusinessAccountToken, BusinessAccountTransactionToken, TransactionResponse};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let business_account_id: BusinessAccountToken = from_value::<BusinessAccountToken>(json!({}))?;
    let transaction_id: BusinessAccountTransactionToken = from_value::<BusinessAccountTransactionToken>(json!({}))?;

    let response: ResponseValue<TransactionResponse> = client
        .get_business_account_transaction(
            &business_account_id,
            &transaction_id,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `list_payouts`

- Summary: List payouts
- Route: `GET /payouts`
- Response: `ResponseValue<ListPayoutsResponse>`
- Rust example: `examples/list_payouts.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{ListPayoutsBalanceId, ListPayoutsResponse, Sorting};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let balance_id: ListPayoutsBalanceId = from_value::<ListPayoutsBalanceId>(json!({}))?;

    let response: ResponseValue<ListPayoutsResponse> = client
        .list_payouts(
            Some(&balance_id),
            None,
            ::std::num::NonZeroU64::new(50),
            Some(Sorting::Desc),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `create_payout`

- Summary: Create payout
- Route: `POST /payouts`
- Response: `ResponseValue<EntityPayoutResponse>`
- Rust example: `examples/create_payout.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{EntityPayoutResponse, PayoutRequest};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let body: PayoutRequest = from_value::<PayoutRequest>(json!({}))?;

    let response: ResponseValue<EntityPayoutResponse> = client
        .create_payout(
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `get_payout`

- Summary: Get payout
- Route: `GET /payouts/{payoutId}`
- Response: `ResponseValue<EntityPayoutResponse>`
- Rust example: `examples/get_payout.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::EntityPayoutResponse;

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let response: ResponseValue<EntityPayoutResponse> = client
        .get_payout(
            "example-id",
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `cancel_payout`

- Summary: Cancel payout
- Route: `DELETE /payouts/{payoutId}`
- Response: `ResponseValue<EntityPayoutResponse>`
- Rust example: `examples/cancel_payout.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::EntityPayoutResponse;

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let response: ResponseValue<EntityPayoutResponse> = client
        .cancel_payout(
            "example-id",
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `create_transfer`

- Summary: Create transfer
- Route: `POST /business-accounts/transfers`
- Response: `ResponseValue<TransferResponse>`
- Rust example: `examples/create_transfer.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{TransferRequest, TransferResponse};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let body: TransferRequest = from_value::<TransferRequest>(json!({}))?;

    let response: ResponseValue<TransferResponse> = client
        .create_transfer(
            "example-id",
            "example-id",
            "example-id",
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `get_transfer`

- Summary: Get transfer
- Route: `GET /business-accounts/transfers/{businessAccountsTransferId}`
- Response: `ResponseValue<TransferResponse>`
- Rust example: `examples/get_transfer.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{BusinessAccountTransferToken, TransferResponse};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let business_accounts_transfer_id: BusinessAccountTransferToken = from_value::<BusinessAccountTransferToken>(json!({}))?;

    let response: ResponseValue<TransferResponse> = client
        .get_transfer(
            &business_accounts_transfer_id,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `create_session`

- Summary: Create session
- Route: `POST /sessions`
- Response: `ResponseValue<SessionResponse>`
- Rust example: `examples/create_session.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{SessionRequest, SessionResponse};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let body: SessionRequest = from_value::<SessionRequest>(json!({}))?;

    let response: ResponseValue<SessionResponse> = client
        .create_session(
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `get_session`

- Summary: Get session
- Route: `GET /sessions/{sessionId}`
- Response: `ResponseValue<SessionResponse>`
- Rust example: `examples/get_session.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{SessionResponse, SessionToken};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let session_id: SessionToken = from_value::<SessionToken>(json!({}))?;

    let response: ResponseValue<SessionResponse> = client
        .get_session(
            &session_id,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `list_unmatched_credit_transfers`

- Summary: List unmatched credit transfers
- Route: `GET /unmatched-credit-transfers`
- Response: `ResponseValue<ListUnmatchedCreditTransfersResponse>`
- Rust example: `examples/list_unmatched_credit_transfers.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::ListUnmatchedCreditTransfersResponse;

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let response: ResponseValue<ListUnmatchedCreditTransfersResponse> = client
        .list_unmatched_credit_transfers(
            None,
            ::std::num::NonZeroU64::new(50),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `get_unmatched_credit_transfer`

- Summary: Get unmatched credit transfer
- Route: `GET /unmatched-credit-transfers/{unmatchedCreditTransferId}`
- Response: `ResponseValue<EntityUnmatchedCreditTransfer>`
- Rust example: `examples/get_unmatched_credit_transfer.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{EntityUnmatchedCreditTransfer, UnmatchedCreditTransferToken};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let unmatched_credit_transfer_id: UnmatchedCreditTransferToken = from_value::<UnmatchedCreditTransferToken>(json!({}))?;

    let response: ResponseValue<EntityUnmatchedCreditTransfer> = client
        .get_unmatched_credit_transfer(
            &unmatched_credit_transfer_id,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `match_unmatched_credit_transfer`

- Summary: Match unmatched credit transfer
- Route: `POST /unmatched-credit-transfers/{unmatchedCreditTransferId}/match`
- Response: `ResponseValue<UnmatchedCreditTransferActionResponse>`
- Rust example: `examples/match_unmatched_credit_transfer.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{UnmatchedCreditTransferActionResponse, UnmatchedCreditTransferMatchRequest, UnmatchedCreditTransferToken};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let unmatched_credit_transfer_id: UnmatchedCreditTransferToken = from_value::<UnmatchedCreditTransferToken>(json!({}))?;
    let body: UnmatchedCreditTransferMatchRequest = from_value::<UnmatchedCreditTransferMatchRequest>(json!({}))?;

    let response: ResponseValue<UnmatchedCreditTransferActionResponse> = client
        .match_unmatched_credit_transfer(
            &unmatched_credit_transfer_id,
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `return_unmatched_credit_transfer`

- Summary: Return unmatched credit transfer
- Route: `POST /unmatched-credit-transfers/{unmatchedCreditTransferId}/return`
- Response: `ResponseValue<UnmatchedCreditTransferActionResponse>`
- Rust example: `examples/return_unmatched_credit_transfer.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{UnmatchedCreditTransferActionResponse, UnmatchedCreditTransferToken};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let unmatched_credit_transfer_id: UnmatchedCreditTransferToken = from_value::<UnmatchedCreditTransferToken>(json!({}))?;

    let response: ResponseValue<UnmatchedCreditTransferActionResponse> = client
        .return_unmatched_credit_transfer(
            &unmatched_credit_transfer_id,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `verify_payee`

- Summary: Verify Payee
- Route: `POST /business-accounts/payee-verifications`
- Response: `ResponseValue<VerificationOfPayeeResponse>`
- Rust example: `examples/verify_payee.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{VerificationOfPayeeRequest, VerificationOfPayeeResponse};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let body: VerificationOfPayeeRequest = from_value::<VerificationOfPayeeRequest>(json!({}))?;

    let response: ResponseValue<VerificationOfPayeeResponse> = client
        .verify_payee(
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `oauth_generate_tokens`

- Summary: Generate OAuth access / refresh tokens using client credentials (Basic auth).
- Route: `POST /oauth2/tokens`
- Response: `ResponseValue<OauthGenerateTokensResponse>`
- Rust example: `examples/oauth_generate_tokens.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{OauthGenerateTokensBody, OauthGenerateTokensResponse};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let body: OauthGenerateTokensBody = from_value::<OauthGenerateTokensBody>(json!({}))?;

    let response: ResponseValue<OauthGenerateTokensResponse> = client
        .oauth_generate_tokens(
            "example-id",
            Some("example-id"),
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `oauth_revoke_tokens`

- Summary: Revoke an OAuth access or refresh token using client credentials (Basic auth).
- Route: `DELETE /oauth2/tokens`
- Response: `ResponseValue<()>`
- Rust example: `examples/oauth_revoke_tokens.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::OauthRevokeTokensBody;
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let body: OauthRevokeTokensBody = from_value::<OauthRevokeTokensBody>(json!({}))?;

    let response: ResponseValue<()> = client
        .oauth_revoke_tokens(
            "example-id",
            Some("example-id"),
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `list_customers`

- Summary: List customers
- Route: `GET /customers`
- Response: `ResponseValue<ListCustomersResponse>`
- Rust example: `examples/list_customers.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{ListCustomersResponse, Sorting};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let response: ResponseValue<ListCustomersResponse> = client
        .list_customers(
            None,
            ::std::num::NonZeroU64::new(50),
            Some(Sorting::Desc),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `create_customer`

- Summary: Create customer
- Route: `POST /customers`
- Response: `ResponseValue<CustomerResponse>`
- Rust example: `examples/create_customer.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{CustomerResponse, EntityCustomer};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let body: EntityCustomer = EntityCustomer::default();

    let response: ResponseValue<CustomerResponse> = client
        .create_customer(
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `get_customer`

- Summary: Get customer
- Route: `GET /customers/{customerId}`
- Response: `ResponseValue<CustomerResponse>`
- Rust example: `examples/get_customer.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{CustomerResponse, CustomerToken};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let customer_id: CustomerToken = CustomerToken::try_from("cst_1234567890".to_owned()).expect("valid generated token fixture");

    let response: ResponseValue<CustomerResponse> = client
        .get_customer(
            &customer_id,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `delete_customer`

- Summary: Delete customer
- Route: `DELETE /customers/{customerId}`
- Response: `ResponseValue<()>`
- Rust example: `examples/delete_customer.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{CustomerToken, DeleteCustomerBody};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let customer_id: CustomerToken = CustomerToken::try_from("cst_1234567890".to_owned()).expect("valid generated token fixture");
    let body: DeleteCustomerBody = DeleteCustomerBody::default();

    let response: ResponseValue<()> = client
        .delete_customer(
            &customer_id,
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `update_customer`

- Summary: Update customer
- Route: `PATCH /customers/{customerId}`
- Response: `ResponseValue<CustomerResponse>`
- Rust example: `examples/update_customer.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{CustomerResponse, CustomerToken, UpdateCustomerBody};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let customer_id: CustomerToken = CustomerToken::try_from("cst_1234567890".to_owned()).expect("valid generated token fixture");
    let body: UpdateCustomerBody = UpdateCustomerBody::default();

    let response: ResponseValue<CustomerResponse> = client
        .update_customer(
            &customer_id,
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `list_customer_payments`

- Summary: List customer payments
- Route: `GET /customers/{customerId}/payments`
- Response: `ResponseValue<ListCustomerPaymentsResponse>`
- Rust example: `examples/list_customer_payments.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{CustomerToken, ListCustomerPaymentsResponse, Sorting};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let customer_id: CustomerToken = CustomerToken::try_from("cst_1234567890".to_owned()).expect("valid generated token fixture");

    let response: ResponseValue<ListCustomerPaymentsResponse> = client
        .list_customer_payments(
            &customer_id,
            None,
            ::std::num::NonZeroU64::new(50),
            None,
            Some(Sorting::Desc),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `create_customer_payment`

- Summary: Create customer payment
- Route: `POST /customers/{customerId}/payments`
- Response: `ResponseValue<PaymentResponse>`
- Rust example: `examples/create_customer_payment.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{CustomerToken, PaymentRequest, PaymentResponse};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let customer_id: CustomerToken = CustomerToken::try_from("cst_1234567890".to_owned()).expect("valid generated token fixture");
    let body: PaymentRequest = from_value::<PaymentRequest>(json!({}))?;

    let response: ResponseValue<PaymentResponse> = client
        .create_customer_payment(
            &customer_id,
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `list_mandates`

- Summary: List mandates
- Route: `GET /customers/{customerId}/mandates`
- Response: `ResponseValue<ListMandatesResponse>`
- Rust example: `examples/list_mandates.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{CustomerToken, ListMandatesResponse, MandateScopes, Sorting};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let customer_id: CustomerToken = CustomerToken::try_from("cst_1234567890".to_owned()).expect("valid generated token fixture");
    let scopes: ::std::vec::Vec<MandateScopes> = ::std::vec::Vec::new();

    let response: ResponseValue<ListMandatesResponse> = client
        .list_mandates(
            &customer_id,
            None,
            ::std::num::NonZeroU64::new(50),
            Some(&scopes),
            Some(Sorting::Desc),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `create_mandate`

- Summary: Create mandate
- Route: `POST /customers/{customerId}/mandates`
- Response: `ResponseValue<MandateResponse>`
- Rust example: `examples/create_mandate.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{CustomerToken, MandateRequest, MandateResponse};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let customer_id: CustomerToken = CustomerToken::try_from("cst_1234567890".to_owned()).expect("valid generated token fixture");
    let body: MandateRequest = from_value::<MandateRequest>(json!({
        "consumerAccount": "NL55INGB0000000000",
        "consumerName": "Jane Doe",
        "method": "directdebit",
        "signatureDate": "2026-01-01"
    }))?;

    let response: ResponseValue<MandateResponse> = client
        .create_mandate(
            &customer_id,
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `get_mandate`

- Summary: Get mandate
- Route: `GET /customers/{customerId}/mandates/{mandateId}`
- Response: `ResponseValue<MandateResponse>`
- Rust example: `examples/get_mandate.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{CustomerToken, MandateResponse, MandateToken};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let customer_id: CustomerToken = CustomerToken::try_from("cst_1234567890".to_owned()).expect("valid generated token fixture");
    let mandate_id: MandateToken = MandateToken::try_from("mdt_1234567890".to_owned()).expect("valid generated token fixture");

    let response: ResponseValue<MandateResponse> = client
        .get_mandate(
            &customer_id,
            &mandate_id,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `revoke_mandate`

- Summary: Revoke mandate
- Route: `DELETE /customers/{customerId}/mandates/{mandateId}`
- Response: `ResponseValue<()>`
- Rust example: `examples/revoke_mandate.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{CustomerToken, MandateToken, RevokeMandateBody};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let customer_id: CustomerToken = CustomerToken::try_from("cst_1234567890".to_owned()).expect("valid generated token fixture");
    let mandate_id: MandateToken = MandateToken::try_from("mdt_1234567890".to_owned()).expect("valid generated token fixture");
    let body: RevokeMandateBody = RevokeMandateBody::default();

    let response: ResponseValue<()> = client
        .revoke_mandate(
            &customer_id,
            &mandate_id,
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `list_subscriptions`

- Summary: List customer subscriptions
- Route: `GET /customers/{customerId}/subscriptions`
- Response: `ResponseValue<ListSubscriptionsResponse>`
- Rust example: `examples/list_subscriptions.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{CustomerToken, ListSubscriptionsResponse, Sorting};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let customer_id: CustomerToken = CustomerToken::try_from("cst_1234567890".to_owned()).expect("valid generated token fixture");

    let response: ResponseValue<ListSubscriptionsResponse> = client
        .list_subscriptions(
            &customer_id,
            None,
            ::std::num::NonZeroU64::new(50),
            Some(Sorting::Desc),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `create_subscription`

- Summary: Create subscription
- Route: `POST /customers/{customerId}/subscriptions`
- Response: `ResponseValue<SubscriptionResponse>`
- Rust example: `examples/create_subscription.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{CustomerToken, SubscriptionRequest, SubscriptionResponse};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let customer_id: CustomerToken = CustomerToken::try_from("cst_1234567890".to_owned()).expect("valid generated token fixture");
    let body: SubscriptionRequest = SubscriptionRequest::default();

    let response: ResponseValue<SubscriptionResponse> = client
        .create_subscription(
            &customer_id,
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `get_subscription`

- Summary: Get subscription
- Route: `GET /customers/{customerId}/subscriptions/{subscriptionId}`
- Response: `ResponseValue<SubscriptionResponse>`
- Rust example: `examples/get_subscription.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{CustomerToken, SubscriptionResponse, SubscriptionToken};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let customer_id: CustomerToken = CustomerToken::try_from("cst_1234567890".to_owned()).expect("valid generated token fixture");
    let subscription_id: SubscriptionToken = SubscriptionToken::try_from("sub_1234567890".to_owned()).expect("valid generated token fixture");

    let response: ResponseValue<SubscriptionResponse> = client
        .get_subscription(
            &customer_id,
            &subscription_id,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `cancel_subscription`

- Summary: Cancel subscription
- Route: `DELETE /customers/{customerId}/subscriptions/{subscriptionId}`
- Response: `ResponseValue<SubscriptionResponse>`
- Rust example: `examples/cancel_subscription.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{CancelSubscriptionBody, CustomerToken, SubscriptionResponse, SubscriptionToken};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let customer_id: CustomerToken = CustomerToken::try_from("cst_1234567890".to_owned()).expect("valid generated token fixture");
    let subscription_id: SubscriptionToken = SubscriptionToken::try_from("sub_1234567890".to_owned()).expect("valid generated token fixture");
    let body: CancelSubscriptionBody = CancelSubscriptionBody::default();

    let response: ResponseValue<SubscriptionResponse> = client
        .cancel_subscription(
            &customer_id,
            &subscription_id,
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `update_subscription`

- Summary: Update subscription
- Route: `PATCH /customers/{customerId}/subscriptions/{subscriptionId}`
- Response: `ResponseValue<SubscriptionResponse>`
- Rust example: `examples/update_subscription.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{CustomerToken, SubscriptionResponse, SubscriptionToken, UpdateSubscriptionBody};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let customer_id: CustomerToken = CustomerToken::try_from("cst_1234567890".to_owned()).expect("valid generated token fixture");
    let subscription_id: SubscriptionToken = SubscriptionToken::try_from("sub_1234567890".to_owned()).expect("valid generated token fixture");
    let body: UpdateSubscriptionBody = UpdateSubscriptionBody::default();

    let response: ResponseValue<SubscriptionResponse> = client
        .update_subscription(
            &customer_id,
            &subscription_id,
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `list_all_subscriptions`

- Summary: List all subscriptions
- Route: `GET /subscriptions`
- Response: `ResponseValue<ListAllSubscriptionsResponse>`
- Rust example: `examples/list_all_subscriptions.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::ListAllSubscriptionsResponse;

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let response: ResponseValue<ListAllSubscriptionsResponse> = client
        .list_all_subscriptions(
            None,
            ::std::num::NonZeroU64::new(50),
            None,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `list_subscription_payments`

- Summary: List subscription payments
- Route: `GET /customers/{customerId}/subscriptions/{subscriptionId}/payments`
- Response: `ResponseValue<ListSubscriptionPaymentsResponse>`
- Rust example: `examples/list_subscription_payments.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{CustomerToken, ListSubscriptionPaymentsResponse, Sorting, SubscriptionToken};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let customer_id: CustomerToken = CustomerToken::try_from("cst_1234567890".to_owned()).expect("valid generated token fixture");
    let subscription_id: SubscriptionToken = SubscriptionToken::try_from("sub_1234567890".to_owned()).expect("valid generated token fixture");

    let response: ResponseValue<ListSubscriptionPaymentsResponse> = client
        .list_subscription_payments(
            &customer_id,
            &subscription_id,
            None,
            ::std::num::NonZeroU64::new(50),
            None,
            Some(Sorting::Desc),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `list_sales_invoices`

- Summary: List sales invoices
- Route: `GET /sales-invoices`
- Response: `ResponseValue<ListSalesInvoicesResponse>`
- Rust example: `examples/list_sales_invoices.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::ListSalesInvoicesResponse;

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let response: ResponseValue<ListSalesInvoicesResponse> = client
        .list_sales_invoices(
            None,
            ::std::num::NonZeroU64::new(50),
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `create_sales_invoice`

- Summary: Create sales invoice
- Route: `POST /sales-invoices`
- Response: `ResponseValue<SalesInvoiceResponse>`
- Rust example: `examples/create_sales_invoice.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{SalesInvoiceRequest, SalesInvoiceResponse};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let body: SalesInvoiceRequest = from_value::<SalesInvoiceRequest>(json!({}))?;

    let response: ResponseValue<SalesInvoiceResponse> = client
        .create_sales_invoice(
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `get_sales_invoice`

- Summary: Get sales invoice
- Route: `GET /sales-invoices/{salesInvoiceId}`
- Response: `ResponseValue<SalesInvoiceResponse>`
- Rust example: `examples/get_sales_invoice.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{SalesInvoiceResponse, SalesInvoiceToken};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let sales_invoice_id: SalesInvoiceToken = from_value::<SalesInvoiceToken>(json!({}))?;

    let response: ResponseValue<SalesInvoiceResponse> = client
        .get_sales_invoice(
            &sales_invoice_id,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `delete_sales_invoice`

- Summary: Delete sales invoice
- Route: `DELETE /sales-invoices/{salesInvoiceId}`
- Response: `ResponseValue<()>`
- Rust example: `examples/delete_sales_invoice.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{DeleteValuesSalesInvoice, SalesInvoiceToken};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let sales_invoice_id: SalesInvoiceToken = from_value::<SalesInvoiceToken>(json!({}))?;
    let body: DeleteValuesSalesInvoice = DeleteValuesSalesInvoice::default();

    let response: ResponseValue<()> = client
        .delete_sales_invoice(
            &sales_invoice_id,
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

### `update_sales_invoice`

- Summary: Update sales invoice
- Route: `PATCH /sales-invoices/{salesInvoiceId}`
- Response: `ResponseValue<SalesInvoiceResponse>`
- Rust example: `examples/update_sales_invoice.rs`

```rust
use mollie_rs::{MollieClient, ResponseValue};
use mollie_rs::types::{SalesInvoiceResponse, SalesInvoiceToken, UpdateSalesInvoiceBody};
use serde_json::{from_value, json};

async fn example() -> Result<(), mollie_rs::MollieError> {
    let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;

    let sales_invoice_id: SalesInvoiceToken = from_value::<SalesInvoiceToken>(json!({}))?;
    let body: UpdateSalesInvoiceBody = UpdateSalesInvoiceBody::default();

    let response: ResponseValue<SalesInvoiceResponse> = client
        .update_sales_invoice(
            &sales_invoice_id,
            &body,
        )
        .await;

    let _ = response;
    Ok(())
}
```

