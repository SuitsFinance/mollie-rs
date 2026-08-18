//! Env-gated live Mollie smoke tests (readonly + optional testmode writes).
//!
//! **Default `cargo test` never hits the network and never mutates Mollie.**
//!
//! ## Tier 1 — readonly
//!
//! ```text
//! MOLLIE_LIVE_READONLY=1 MOLLIE_API_KEY=test_... \
//!   cargo test --test live_smoke -- --ignored --nocapture
//! ```
//!
//! `MOLLIE_LIVE_SMOKE=1` remains accepted as an alias for readonly (legacy).
//!
//! ## Tier 2 — testmode writes (multi-gate)
//!
//! ```text
//! MOLLIE_TESTMODE_WRITE=1 \
//! MOLLIE_ALLOW_MUTATION=I_UNDERSTAND_THIS_MUTATES_MOLLIE \
//! MOLLIE_API_KEY=test_... \
//!   cargo test --test live_smoke sandbox_ -- --ignored --nocapture
//! ```
//!
//! `live_` API keys are refused for write suites. Never commit secrets.

use mollie_rs::{IntoMollieFuture, MollieClient, MollieError, PaginationGuard};
use std::num::NonZeroU64;

// ---------------------------------------------------------------------------
// Gates (pure helpers + env wrappers)
// ---------------------------------------------------------------------------

fn env_truthy(name: &str) -> bool {
    matches!(
        std::env::var(name).as_deref(),
        Ok("1") | Ok("true") | Ok("yes")
    )
}

/// Readonly live suite: explicit flag or legacy smoke alias.
fn live_readonly_enabled() -> bool {
    env_truthy("MOLLIE_LIVE_READONLY") || env_truthy("MOLLIE_LIVE_SMOKE")
}

const MUTATION_PHRASE: &str = "I_UNDERSTAND_THIS_MUTATES_MOLLIE";

/// Whether an API key string is eligible for testmode write smoke.
///
/// - `test_…` → allowed
/// - `live_…` → never
/// - missing / other → not allowed unless separate OAuth override is used at env layer
fn api_key_allows_testmode_write(api_key: Option<&str>) -> bool {
    match api_key {
        Some(k) if k.starts_with("test_") => true,
        Some(k) if k.starts_with("live_") => false,
        _ => false,
    }
}

fn using_test_api_key() -> bool {
    api_key_allows_testmode_write(std::env::var("MOLLIE_API_KEY").ok().as_deref())
}

/// Multi-gate write enablement. All conditions required.
fn live_write_enabled() -> bool {
    env_truthy("MOLLIE_TESTMODE_WRITE")
        && std::env::var("MOLLIE_ALLOW_MUTATION").as_deref() == Ok(MUTATION_PHRASE)
        && (using_test_api_key() || env_truthy("MOLLIE_TESTMODE_WRITE_ALLOW_OAUTH"))
}

fn destructive_smoke_enabled() -> bool {
    live_write_enabled() && env_truthy("MOLLIE_DESTRUCTIVE_SMOKE")
}

fn assert_readonly_gate() {
    assert!(
        live_readonly_enabled(),
        "set MOLLIE_LIVE_READONLY=1 (or legacy MOLLIE_LIVE_SMOKE=1) to run live readonly smoke"
    );
}

fn assert_write_gates() {
    assert!(
        live_write_enabled(),
        "write smoke requires MOLLIE_TESTMODE_WRITE=1, \
         MOLLIE_ALLOW_MUTATION={MUTATION_PHRASE}, and a test_ API key \
         (or MOLLIE_TESTMODE_WRITE_ALLOW_OAUTH=1 with disposable OAuth)"
    );
    if let Ok(key) = std::env::var("MOLLIE_API_KEY") {
        assert!(
            !key.starts_with("live_"),
            "live_ API keys must not run write smoke; use test_ credentials only for RC"
        );
    }
}

fn live_client() -> MollieClient {
    MollieClient::from_env().expect("MOLLIE_API_KEY (or OAuth) required for live smoke")
}

fn page_limit() -> Option<NonZeroU64> {
    NonZeroU64::new(5)
}

// ---------------------------------------------------------------------------
// Outcome classification (account limits ≠ SDK failure)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LiveCallClass {
    /// Credential cannot call this route (403).
    PermissionDenied,
    /// Bad/missing auth (401).
    AuthenticationFailed,
    /// Route/resource not available to this account shape (404/410).
    UnsupportedByAccount,
    /// Provider rejected the shape for this account (422) — treat as env limit.
    ProviderRejected,
    /// Unexpected SDK/transport/5xx — fail the test.
    SdkOrTransportFailure,
}

fn classify_error(err: &MollieError) -> LiveCallClass {
    // Prefer HTTP status (works for UnexpectedStatus and API envelopes).
    match err.status().map(|s| s.as_u16()) {
        Some(401) => return LiveCallClass::AuthenticationFailed,
        Some(403) => return LiveCallClass::PermissionDenied,
        Some(404) | Some(410) => return LiveCallClass::UnsupportedByAccount,
        Some(422) => return LiveCallClass::ProviderRejected,
        Some(code) if (500..600).contains(&code) => return LiveCallClass::SdkOrTransportFailure,
        _ => {}
    }
    if err.is_authentication_failure() {
        return LiveCallClass::AuthenticationFailed;
    }
    if err.is_authorization_failure() {
        return LiveCallClass::PermissionDenied;
    }
    if err.is_not_found() {
        return LiveCallClass::UnsupportedByAccount;
    }
    LiveCallClass::SdkOrTransportFailure
}

/// Accept success or documented account/environment limitations; fail hard otherwise.
fn accept_readonly_result(operation: &str, result: Result<(), MollieError>) {
    match result {
        Ok(()) => {
            eprintln!("live readonly {operation}: ok");
        }
        Err(err) => {
            let class = classify_error(&err);
            eprintln!("live readonly {operation}: {class:?} — {err}");
            match class {
                LiveCallClass::PermissionDenied
                | LiveCallClass::UnsupportedByAccount
                | LiveCallClass::ProviderRejected => {}
                LiveCallClass::AuthenticationFailed => {
                    panic!(
                        "{operation}: authentication failed — check MOLLIE_API_KEY/OAuth: {err}"
                    );
                }
                LiveCallClass::SdkOrTransportFailure => {
                    panic!("{operation}: unexpected SDK/transport failure: {err}");
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Tier 1 — readonly matrix
// ---------------------------------------------------------------------------

/// Lists payment methods (Tier-G + serialization path).
#[tokio::test]
#[ignore = "live network; set MOLLIE_LIVE_READONLY=1 and MOLLIE_API_KEY"]
async fn live_methods_readonly() {
    assert_readonly_gate();
    let client = live_client();
    let result = client
        .list_methods(None, None, None, None, None, None, None, None, None)
        .into_mollie_result()
        .await
        .map(|_| ());
    accept_readonly_result("list_methods", result);
}

/// Payments facade list_page (Tier-S pagination decode).
#[tokio::test]
#[ignore = "live network; set MOLLIE_LIVE_READONLY=1 and MOLLIE_API_KEY"]
async fn live_payments_readonly() {
    assert_readonly_gate();
    let client = live_client();
    let result = client.payments().list_page(None, Some(5)).await;
    if let Ok(page) = &result {
        assert!(page.items.len() <= 5);
        let _ = PaginationGuard::default_safe();
    }
    accept_readonly_result("payments.list_page", result.map(|_| ()));
}

/// Profiles list (org/profile route group).
#[tokio::test]
#[ignore = "live network; set MOLLIE_LIVE_READONLY=1 and MOLLIE_API_KEY"]
async fn live_profiles_readonly() {
    assert_readonly_gate();
    let client = live_client();
    let result = client
        .list_profiles(None, page_limit())
        .into_mollie_result()
        .await
        .map(|_| ());
    accept_readonly_result("list_profiles", result);
}

/// Current profile shortcut when API key scoped.
#[tokio::test]
#[ignore = "live network; set MOLLIE_LIVE_READONLY=1 and MOLLIE_API_KEY"]
async fn live_current_profile_readonly() {
    assert_readonly_gate();
    let client = live_client();
    let result = client
        .get_current_profile()
        .into_mollie_result()
        .await
        .map(|_| ());
    accept_readonly_result("get_current_profile", result);
}

/// Balances list (Connect/balance surface).
#[tokio::test]
#[ignore = "live network; set MOLLIE_LIVE_READONLY=1 and MOLLIE_API_KEY"]
async fn live_balances_readonly() {
    assert_readonly_gate();
    let client = live_client();
    let result = client
        .list_balances(None, None, page_limit())
        .into_mollie_result()
        .await
        .map(|_| ());
    accept_readonly_result("list_balances", result);
}

/// Settlements list.
#[tokio::test]
#[ignore = "live network; set MOLLIE_LIVE_READONLY=1 and MOLLIE_API_KEY"]
async fn live_settlements_readonly() {
    assert_readonly_gate();
    let client = live_client();
    let result = client
        .list_settlements(None, None, None, page_limit(), None, None)
        .into_mollie_result()
        .await
        .map(|_| ());
    accept_readonly_result("list_settlements", result);
}

/// Organization “me” (often OAuth-oriented).
#[tokio::test]
#[ignore = "live network; set MOLLIE_LIVE_READONLY=1 and MOLLIE_API_KEY"]
async fn live_organizations_readonly() {
    assert_readonly_gate();
    let client = live_client();
    let result = client
        .get_current_organization()
        .into_mollie_result()
        .await
        .map(|_| ());
    accept_readonly_result("get_current_organization", result);
}

/// Permissions list (OAuth permission surface).
#[tokio::test]
#[ignore = "live network; set MOLLIE_LIVE_READONLY=1 and MOLLIE_API_KEY"]
async fn live_permissions_readonly() {
    assert_readonly_gate();
    let client = live_client();
    let result = client
        .list_permissions()
        .into_mollie_result()
        .await
        .map(|_| ());
    accept_readonly_result("list_permissions", result);
}

/// Global refunds list (not payment-scoped).
#[tokio::test]
#[ignore = "live network; set MOLLIE_LIVE_READONLY=1 and MOLLIE_API_KEY"]
async fn live_refunds_readonly() {
    assert_readonly_gate();
    let client = live_client();
    let result = client
        .list_all_refunds(None, None, page_limit(), None, None)
        .into_mollie_result()
        .await
        .map(|_| ());
    accept_readonly_result("list_all_refunds", result);
}

/// Tier-S refunds facade list (payment-scoped list needs a payment id; uses all-refunds path via facade when available).
#[tokio::test]
#[ignore = "live network; set MOLLIE_LIVE_READONLY=1 and MOLLIE_API_KEY"]
async fn live_refunds_facade_readonly() {
    assert_readonly_gate();
    let client = live_client();
    // Facade entry must construct; list without payment id is account-global via generated route.
    let _ = client.refunds();
    let result = client
        .list_all_refunds(None, None, page_limit(), None, None)
        .into_mollie_result()
        .await
        .map(|_| ());
    accept_readonly_result("refunds_facade+list_all_refunds", result);
}

/// Captures are payment-scoped; exercise facade construction + payments list as precondition surface.
#[tokio::test]
#[ignore = "live network; set MOLLIE_LIVE_READONLY=1 and MOLLIE_API_KEY"]
async fn live_captures_facade_readonly() {
    assert_readonly_gate();
    let client = live_client();
    let _ = client.captures();
    let result = client.payments().list_page(None, Some(5)).await;
    accept_readonly_result("captures_facade+payments.list_page", result.map(|_| ()));
}

/// Payouts facade list (Tier-S money read).
#[tokio::test]
#[ignore = "live network; set MOLLIE_LIVE_READONLY=1 and MOLLIE_API_KEY"]
async fn live_payouts_readonly() {
    assert_readonly_gate();
    let client = live_client();
    let result = client.payouts().list_page(None, None, Some(5)).await;
    accept_readonly_result("payouts.list_page", result.map(|_| ()));
}

/// Business accounts (often entitlement-gated).
#[tokio::test]
#[ignore = "live network; set MOLLIE_LIVE_READONLY=1 and MOLLIE_API_KEY"]
async fn live_business_accounts_readonly() {
    assert_readonly_gate();
    let client = live_client();
    let result = client
        .list_business_accounts(None, page_limit(), None)
        .into_mollie_result()
        .await
        .map(|_| ());
    accept_readonly_result("list_business_accounts", result);
}

/// Terminals facade list.
#[tokio::test]
#[ignore = "live network; set MOLLIE_LIVE_READONLY=1 and MOLLIE_API_KEY"]
async fn live_terminals_readonly() {
    assert_readonly_gate();
    let client = live_client();
    let result = client.terminals().list_page(None, Some(5)).await;
    accept_readonly_result("terminals.list_page", result.map(|_| ()));
}

/// Webhooks list when API supports it for the credential.
#[tokio::test]
#[ignore = "live network; set MOLLIE_LIVE_READONLY=1 and MOLLIE_API_KEY"]
async fn live_webhooks_readonly() {
    assert_readonly_gate();
    let client = live_client();
    let result = client
        .list_webhooks(None, None, page_limit(), None)
        .into_mollie_result()
        .await
        .map(|_| ());
    accept_readonly_result("list_webhooks", result);
}

// ---------------------------------------------------------------------------
// Tier 2 — sandbox write smoke (multi-gate; payment path)
// ---------------------------------------------------------------------------

/// Canonical payment create → get smoke under test credentials only.
#[tokio::test]
#[ignore = "sandbox write; multi-gate MOLLIE_TESTMODE_WRITE + mutation phrase + test_ key"]
async fn sandbox_payment_create() {
    assert_write_gates();
    let client = live_client();

    use mollie_rs::{CreatePaymentRequired, IdempotencyKey, Money, PaymentId};

    let amount = Money::new("EUR", "1.00").expect("EUR 1.00");
    let required = CreatePaymentRequired::new(
        "mollie-rs RC sandbox smoke",
        amount,
        "https://example.com/redirect",
    )
    .expect("payment required");
    let key = IdempotencyKey::new(format!("rc-smoke-{}", uuid_like())).expect("idempotency key");

    let created = client
        .payments()
        .create(required, Some(key))
        .await
        .expect("create test payment");
    let payment = created.into_inner();
    let id = PaymentId::parse(payment.id.as_str()).expect("payment id");
    eprintln!(
        "sandbox_payment_create: id={} status={:?}",
        id.as_str(),
        payment.status
    );

    let fetched = client
        .payments()
        .get(&id)
        .await
        .expect("retrieve created payment");
    let body = fetched.into_inner();
    assert_eq!(body.id.as_str(), id.as_str());
    assert_eq!(body.amount.currency, "EUR");

    if destructive_smoke_enabled() {
        let token = mollie_rs::types::PaymentToken(id.as_str().to_string());
        let body = mollie_rs::types::CancelPaymentBody::default();
        match client
            .cancel_payment(&token, &body)
            .into_mollie_result()
            .await
        {
            Ok(_) => eprintln!("sandbox_payment_create: canceled {}", id.as_str()),
            Err(err) => eprintln!("sandbox_payment_create: cancel skipped: {err}"),
        }
    }
}

/// Same logical key twice should not create two independent movements when provider supports it.
#[tokio::test]
#[ignore = "sandbox write; multi-gate + records provider idempotency behavior"]
async fn sandbox_payment_idempotency() {
    assert_write_gates();
    let client = live_client();

    use mollie_rs::{CreatePaymentRequired, IdempotencyKey, Money};

    let amount = Money::new("EUR", "1.00").expect("EUR 1.00");
    let required = CreatePaymentRequired::new(
        "mollie-rs RC idempotency smoke",
        amount,
        "https://example.com/redirect",
    )
    .expect("payment required");
    let key = IdempotencyKey::new(format!("rc-idem-{}", uuid_like())).expect("idempotency key");

    let first = client
        .payments()
        .create(required.clone(), Some(key.clone()))
        .await
        .expect("first create");
    let second = client
        .payments()
        .create(required, Some(key))
        .await
        .expect("second create same key");

    let id1 = first.into_inner().id;
    let id2 = second.into_inner().id;
    eprintln!(
        "sandbox_payment_idempotency: first={} second={} same={}",
        id1.as_str(),
        id2.as_str(),
        id1.as_str() == id2.as_str()
    );
    assert_eq!(
        id1.as_str(),
        id2.as_str(),
        "provider returned different payment ids for the same sticky idempotency key"
    );
}

fn uuid_like() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("{nanos:x}")
}

// ---------------------------------------------------------------------------
// Gate unit tests (no network)
// ---------------------------------------------------------------------------

#[test]
fn write_gate_rejects_live_api_keys() {
    assert!(!api_key_allows_testmode_write(Some(
        "live_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
    )));
    assert!(api_key_allows_testmode_write(Some(
        "test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
    )));
    assert!(!api_key_allows_testmode_write(None));
    assert!(!api_key_allows_testmode_write(Some("access_token_abc")));
}

#[test]
fn classify_auth_and_permission_errors() {
    use reqwest::StatusCode;

    let unauth = MollieError::unexpected_status(StatusCode::UNAUTHORIZED);
    assert_eq!(classify_error(&unauth), LiveCallClass::AuthenticationFailed);

    let forbid = MollieError::unexpected_status(StatusCode::FORBIDDEN);
    assert_eq!(classify_error(&forbid), LiveCallClass::PermissionDenied);

    let missing = MollieError::unexpected_status(StatusCode::NOT_FOUND);
    assert_eq!(
        classify_error(&missing),
        LiveCallClass::UnsupportedByAccount
    );

    let rejected = MollieError::unexpected_status(StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(classify_error(&rejected), LiveCallClass::ProviderRejected);

    let boom = MollieError::unexpected_status(StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(classify_error(&boom), LiveCallClass::SdkOrTransportFailure);
}
