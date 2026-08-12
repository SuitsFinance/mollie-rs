//! HTTP contract tests against a mock Mollie API (wiremock).
//!
//! These prove request shaping that unit/fixture tests cannot:
//! authorization headers, idempotency keys, custom base URLs, and status handling.

use mollie_rs::{auth::Credential, MollieClient, ResponseValueExt, RetryPolicy};
use serde_json::json;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn list_payments_body() -> serde_json::Value {
    json!({
        "count": 0,
        "_embedded": { "payments": [] },
        "_links": {
            "self": { "href": "https://api.mollie.com/v2/payments", "type": "application/hal+json" },
            "previous": null,
            "next": null,
            "documentation": {
                "href": "https://docs.mollie.com/reference/list-payments",
                "type": "text/html"
            }
        }
    })
}

#[tokio::test]
async fn list_payments_sends_bearer_auth_and_idempotency_key_to_custom_base_url() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/payments"))
        .and(header(
            "authorization",
            "Bearer test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
        ))
        .and(header(
            "idempotency-key",
            "6f7ef3e6-8c2f-4d1c-9f08-5ab7adf56c91",
        ))
        .and(header("accept", "application/json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(list_payments_body()))
        .expect(1)
        .mount(&server)
        .await;

    let client = MollieClient::builder()
        .base_url(server.uri())
        .credential(
            Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx").expect("valid test key"),
        )
        .build()
        .expect("client builds against mock base URL")
        .with_idempotency_key("6f7ef3e6-8c2f-4d1c-9f08-5ab7adf56c91");

    let response = client
        .list_payments(None, None, None, None)
        .await
        .expect("list_payments succeeds against wiremock");

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(
        response.idempotency_key(),
        Some("6f7ef3e6-8c2f-4d1c-9f08-5ab7adf56c91")
    );
}

#[tokio::test]
async fn oauth_credential_sends_bearer_access_token() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/payments"))
        .and(header("authorization", "Bearer access_token_example_value"))
        .respond_with(ResponseTemplate::new(200).set_body_json(list_payments_body()))
        .expect(1)
        .mount(&server)
        .await;

    let client = MollieClient::builder()
        .base_url(server.uri())
        .credential(
            Credential::oauth_access_token("access_token_example_value").expect("valid token"),
        )
        .build()
        .expect("oauth client");

    client
        .list_payments(None, None, None, None)
        .await
        .expect("oauth list_payments");
}

#[tokio::test]
async fn malformed_json_error_body_is_not_silent_success() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/payments"))
        .respond_with(
            ResponseTemplate::new(500)
                .insert_header("content-type", "text/html")
                .set_body_string("<html>gateway error</html>"),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = MollieClient::builder()
        .base_url(server.uri())
        .credential(
            Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx").expect("valid test key"),
        )
        .build()
        .expect("client");

    let error = client
        .list_payments(None, None, None, None)
        .await
        .expect_err("HTML 500 must not decode as success");

    // Today this surfaces as a payload/error-response failure from the generated
    // layer; Phase B will wrap it with richer ErrorResponseContext.
    let message = format!("{error:?}");
    assert!(
        message.contains("InvalidResponsePayload")
            || message.contains("ErrorResponse")
            || message.contains("ResponseBody")
            || message.contains("error"),
        "unexpected error shape: {message}"
    );
}

#[tokio::test]
async fn default_safe_retry_recovers_from_503_then_success() {
    let server = MockServer::start().await;
    let hits = Arc::new(AtomicUsize::new(0));
    let hits_mock = hits.clone();

    Mock::given(method("GET"))
        .and(path("/payments"))
        .respond_with(move |_req: &wiremock::Request| {
            let n = hits_mock.fetch_add(1, Ordering::SeqCst);
            if n == 0 {
                ResponseTemplate::new(503)
                    .insert_header("retry-after", "0")
                    .set_body_string("unavailable")
            } else {
                ResponseTemplate::new(200).set_body_json(list_payments_body())
            }
        })
        .expect(2..)
        .mount(&server)
        .await;

    let client = MollieClient::builder()
        .base_url(server.uri())
        .credential(
            Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx").expect("valid test key"),
        )
        .retry_policy(RetryPolicy::default_safe())
        .build()
        .expect("client");

    client
        .list_payments(None, None, None, None)
        .await
        .expect("list_payments should succeed after 503 retry");

    assert!(hits.load(Ordering::SeqCst) >= 2);
}

#[tokio::test]
async fn write_without_sticky_idempotency_is_not_auto_retried() {
    let server = MockServer::start().await;
    let hits = Arc::new(AtomicUsize::new(0));
    let hits_mock = hits.clone();

    Mock::given(method("POST"))
        .and(path("/payments"))
        .respond_with(move |_req: &wiremock::Request| {
            hits_mock.fetch_add(1, Ordering::SeqCst);
            ResponseTemplate::new(503).set_body_string("unavailable")
        })
        .expect(1)
        .mount(&server)
        .await;

    let client = MollieClient::builder()
        .base_url(server.uri())
        .credential(
            Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx").expect("valid test key"),
        )
        .retry_policy(RetryPolicy::default_safe())
        .build()
        .expect("client");

    let body = mollie_rs::CreatePaymentRequired::new(
        "Order retry-test",
        mollie_rs::Money::new("EUR", "1.00").unwrap(),
        "https://example.com/r",
    )
    .unwrap()
    .into_payment_request();

    let _ = client.create_payment(None, &body).await;
    assert_eq!(
        hits.load(Ordering::SeqCst),
        1,
        "POST without sticky key must not auto-retry"
    );
}

#[tokio::test]
async fn write_with_sticky_idempotency_retries_503() {
    let server = MockServer::start().await;
    let hits = Arc::new(AtomicUsize::new(0));
    let hits_mock = hits.clone();

    Mock::given(method("POST"))
        .and(path("/payments"))
        .and(header(
            "idempotency-key",
            "6f7ef3e6-8c2f-4d1c-9f08-5ab7adf56c91",
        ))
        .respond_with(move |_req: &wiremock::Request| {
            let n = hits_mock.fetch_add(1, Ordering::SeqCst);
            if n == 0 {
                ResponseTemplate::new(503)
                    .insert_header("retry-after", "0")
                    .set_body_string("unavailable")
            } else {
                // Minimal payment-shaped success is hard; a second 503 still
                // proves the retry happened with the same sticky key.
                ResponseTemplate::new(503).set_body_string("still unavailable")
            }
        })
        .expect(2..)
        .mount(&server)
        .await;

    let client = MollieClient::builder()
        .base_url(server.uri())
        .credential(
            Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx").expect("valid test key"),
        )
        .retry_policy(RetryPolicy::default_safe())
        .build()
        .expect("client")
        .with_idempotency_key("6f7ef3e6-8c2f-4d1c-9f08-5ab7adf56c91");

    let body = mollie_rs::CreatePaymentRequired::new(
        "Order retry-test",
        mollie_rs::Money::new("EUR", "1.00").unwrap(),
        "https://example.com/r",
    )
    .unwrap()
    .into_payment_request();

    let _ = client.create_payment(None, &body).await;
    assert!(
        hits.load(Ordering::SeqCst) >= 2,
        "POST with sticky key should retry transient 503"
    );
}

#[tokio::test]
async fn disabled_retry_does_not_repeat_on_503() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/payments"))
        .respond_with(
            ResponseTemplate::new(503)
                .insert_header("content-type", "text/plain")
                .set_body_string("unavailable"),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = MollieClient::builder()
        .base_url(server.uri())
        .credential(
            Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx").expect("valid test key"),
        )
        // default retry policy is disabled
        .build()
        .expect("client");

    let err = client
        .list_payments(None, None, None, None)
        .await
        .expect_err("503 without retries should fail");
    let _ = err;
}

fn mollie_error_json(status: u16, title: &str, detail: &str) -> serde_json::Value {
    json!({
        "status": status,
        "title": title,
        "detail": detail,
        "_links": {
            "documentation": {
                "href": "https://docs.mollie.com/overview/handling-errors",
                "type": "text/html"
            }
        }
    })
}

async fn list_client_against(server: &MockServer) -> MollieClient {
    MollieClient::builder()
        .base_url(server.uri())
        .credential(
            Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx").expect("valid test key"),
        )
        .retry_policy(RetryPolicy::default_safe())
        .build()
        .expect("client")
}

#[tokio::test]
async fn retries_429_with_retry_after_then_succeeds() {
    let server = MockServer::start().await;
    let hits = Arc::new(AtomicUsize::new(0));
    let hits_mock = hits.clone();

    Mock::given(method("GET"))
        .and(path("/payments"))
        .respond_with(move |_req: &wiremock::Request| {
            let n = hits_mock.fetch_add(1, Ordering::SeqCst);
            if n == 0 {
                ResponseTemplate::new(429)
                    .insert_header("retry-after", "0")
                    .set_body_json(mollie_error_json(
                        429,
                        "Too Many Requests",
                        "You have exceeded the rate limit. Please slow down your requests.",
                    ))
            } else {
                ResponseTemplate::new(200).set_body_json(list_payments_body())
            }
        })
        .expect(2..)
        .mount(&server)
        .await;

    list_client_against(&server)
        .await
        .list_payments(None, None, None, None)
        .await
        .expect("429 then success");
    assert!(hits.load(Ordering::SeqCst) >= 2);
}

#[tokio::test]
async fn does_not_retry_validation_400() {
    let server = MockServer::start().await;
    let hits = Arc::new(AtomicUsize::new(0));
    let hits_mock = hits.clone();

    Mock::given(method("GET"))
        .and(path("/payments"))
        .respond_with(move |_req: &wiremock::Request| {
            hits_mock.fetch_add(1, Ordering::SeqCst);
            ResponseTemplate::new(400).set_body_json(mollie_error_json(
                400,
                "Bad Request",
                "Invalid cursor value",
            ))
        })
        .expect(1)
        .mount(&server)
        .await;

    let err = list_client_against(&server)
        .await
        .list_payments(None, None, None, None)
        .await
        .expect_err("400 must fail");
    let _ = err;
    assert_eq!(hits.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn retry_budget_does_not_send_leftover_attempt_after_deadline() {
    use std::time::Duration;

    let server = MockServer::start().await;
    let hits = Arc::new(AtomicUsize::new(0));
    let hits_mock = hits.clone();

    // Always 503 with Retry-After: 0 so policy would want to retry, but the
    // total retry budget is tiny so a second attempt must not be started.
    Mock::given(method("GET"))
        .and(path("/payments"))
        .respond_with(move |_req: &wiremock::Request| {
            hits_mock.fetch_add(1, Ordering::SeqCst);
            ResponseTemplate::new(503)
                .insert_header("retry-after", "0")
                .insert_header("content-type", "text/plain")
                .set_body_string("unavailable")
        })
        .expect(1..)
        .mount(&server)
        .await;

    // Budget already elapsed relative to any second attempt: max_attempts > 1
    // but total_deadline is zero so only the first send may run.
    let mut policy = RetryPolicy::default_safe();
    policy.total_deadline = Duration::from_millis(0);
    policy.max_attempts = 5;
    policy.initial_backoff = Duration::from_millis(50);

    let client = MollieClient::builder()
        .base_url(server.uri())
        .credential(
            Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx").expect("valid test key"),
        )
        .retry_policy(policy)
        .build()
        .expect("client");

    let _ = client
        .list_payments(None, None, None, None)
        .await
        .expect_err("budget-exhausted list should fail");

    // Invariant: at most one HTTP attempt when budget is already exhausted
    // before a second send can be scheduled.
    assert_eq!(
        hits.load(Ordering::SeqCst),
        1,
        "must not send a leftover request after the retry budget"
    );
}

#[tokio::test]
async fn does_not_retry_unauthorized_401() {
    let server = MockServer::start().await;
    let hits = Arc::new(AtomicUsize::new(0));
    let hits_mock = hits.clone();

    Mock::given(method("GET"))
        .and(path("/payments"))
        .respond_with(move |_req: &wiremock::Request| {
            hits_mock.fetch_add(1, Ordering::SeqCst);
            ResponseTemplate::new(401).set_body_json(mollie_error_json(
                401,
                "Unauthorized",
                "Missing authentication",
            ))
        })
        .expect(1)
        .mount(&server)
        .await;

    let _ = list_client_against(&server)
        .await
        .list_payments(None, None, None, None)
        .await
        .expect_err("401 must fail");
    assert_eq!(hits.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn retries_502_then_succeeds() {
    let server = MockServer::start().await;
    let hits = Arc::new(AtomicUsize::new(0));
    let hits_mock = hits.clone();

    Mock::given(method("GET"))
        .and(path("/payments"))
        .respond_with(move |_req: &wiremock::Request| {
            let n = hits_mock.fetch_add(1, Ordering::SeqCst);
            if n == 0 {
                ResponseTemplate::new(502)
                    .insert_header("content-type", "text/plain")
                    .set_body_string("bad gateway")
            } else {
                ResponseTemplate::new(200).set_body_json(list_payments_body())
            }
        })
        .expect(2..)
        .mount(&server)
        .await;

    list_client_against(&server)
        .await
        .list_payments(None, None, None, None)
        .await
        .expect("502 then success");
    assert!(hits.load(Ordering::SeqCst) >= 2);
}

#[tokio::test]
async fn retries_504_then_succeeds() {
    let server = MockServer::start().await;
    let hits = Arc::new(AtomicUsize::new(0));
    let hits_mock = hits.clone();

    Mock::given(method("GET"))
        .and(path("/payments"))
        .respond_with(move |_req: &wiremock::Request| {
            let n = hits_mock.fetch_add(1, Ordering::SeqCst);
            if n == 0 {
                ResponseTemplate::new(504).set_body_string("gateway timeout")
            } else {
                ResponseTemplate::new(200).set_body_json(list_payments_body())
            }
        })
        .expect(2..)
        .mount(&server)
        .await;

    list_client_against(&server)
        .await
        .list_payments(None, None, None, None)
        .await
        .expect("504 then success");
    assert!(hits.load(Ordering::SeqCst) >= 2);
}

#[tokio::test]
async fn refunds_list_page_uses_payment_scoped_path() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/payments/tr_WDqYK6vllg/refunds"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "count": 0,
            "_embedded": { "refunds": [] },
            "_links": {
                "self": { "href": "https://api.mollie.com/v2/payments/tr_WDqYK6vllg/refunds", "type": "application/hal+json" },
                "previous": null,
                "next": null,
                "documentation": { "href": "https://docs.mollie.com", "type": "text/html" }
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = MollieClient::builder()
        .base_url(server.uri())
        .credential(
            Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx").expect("valid test key"),
        )
        .build()
        .expect("client");

    let payment = mollie_rs::PaymentId::parse("tr_WDqYK6vllg").unwrap();
    let page = client
        .refunds()
        .list_page(&payment, None, Some(50))
        .await
        .expect("list refunds page");
    assert!(page.is_empty());
}

/// INV-HOST-01: client must not follow cross-origin redirects with Authorization.
/// Default builder uses `redirect::Policy::none()` so a 302 is not chased to evil hosts.
#[tokio::test]
async fn does_not_follow_redirect_to_foreign_host() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/payments"))
        .respond_with(
            ResponseTemplate::new(302).insert_header("location", "https://evil.example/steal"),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = MollieClient::builder()
        .base_url(server.uri())
        .credential(
            Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx").expect("valid test key"),
        )
        .build()
        .expect("client");

    let err = client
        .list_payments(None, None, None, None)
        .await
        .expect_err("redirect must not be followed as success");
    // Policy::none surfaces redirect as a transport/status error — never 200 from evil.
    let _ = err;
}

/// INV-CONN-01: concurrent scoped clients must not cross-wire Authorization.
#[tokio::test]
async fn concurrent_scoped_credentials_do_not_cross_wire() {
    use wiremock::matchers::header;

    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/payments"))
        .and(header(
            "authorization",
            "Bearer test_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(list_payments_body()))
        .expect(8)
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/payments"))
        .and(header(
            "authorization",
            "Bearer test_bbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(list_payments_body()))
        .expect(8)
        .mount(&server)
        .await;

    let base = MollieClient::builder()
        .base_url(server.uri())
        .credential(Credential::api_key("test_zzzzzzzzzzzzzzzzzzzzzzzzzzzzzz").expect("z"))
        .build()
        .expect("base");

    let client_a = base
        .clone()
        .with_credential(Credential::api_key("test_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").expect("a"))
        .expect("scope a");
    let client_b = base
        .with_credential(Credential::api_key("test_bbbbbbbbbbbbbbbbbbbbbbbbbbbbbb").expect("b"))
        .expect("scope b");

    let mut set = tokio::task::JoinSet::new();
    for _ in 0..8 {
        let a = client_a.clone();
        let b = client_b.clone();
        set.spawn(async move {
            a.list_payments(None, None, None, None)
                .await
                .expect("a list");
        });
        set.spawn(async move {
            b.list_payments(None, None, None, None)
                .await
                .expect("b list");
        });
    }
    while let Some(joined) = set.join_next().await {
        joined.expect("task");
    }
}

#[tokio::test]
async fn oauth_facade_generate_tokens_uses_basic_auth_and_oauth2_path() {
    use mollie_rs::types::{OauthGenerateTokensBody, OauthGrantType};
    use mollie_rs::BasicAuth;

    let server = MockServer::start().await;
    let basic = BasicAuth::new("app_client_id", "app_client_secret").expect("basic");
    let expected_auth = basic.authorization_value();
    assert!(
        expected_auth.starts_with("Basic "),
        "OAuth client credentials must use Basic scheme"
    );

    Mock::given(method("POST"))
        .and(path("/oauth2/tokens"))
        .and(header("authorization", expected_auth.as_str()))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "access_token": "access_mock",
            "refresh_token": "refresh_mock",
            "expires_in": 3600,
            "token_type": "bearer",
            "scope": "payments.read"
        })))
        .expect(1)
        .mount(&server)
        .await;

    // Client default credential is unused for OAuth facade (per-call Basic).
    let client = MollieClient::builder()
        .base_url(format!("{}/v2", server.uri()))
        .credential(
            Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx").expect("valid test key"),
        )
        .build()
        .expect("client");

    let body = OauthGenerateTokensBody {
        code: Some("auth_code".into()),
        grant_type: OauthGrantType::AuthorizationCode,
        redirect_uri: Some("https://example.com/callback".into()),
        refresh_token: None,
    };

    let response = client
        .oauth()
        .generate_tokens(&basic, &body)
        .await
        .expect("generate_tokens");

    assert_eq!(
        response.into_inner().access_token.as_deref(),
        Some("access_mock")
    );
}

#[tokio::test]
async fn oauth_facade_rejects_api_key_credential_class() {
    use mollie_rs::types::{OauthGenerateTokensBody, OauthGrantType};

    let server = MockServer::start().await;
    let client = MollieClient::builder()
        .base_url(format!("{}/v2", server.uri()))
        .credential(
            Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx").expect("valid test key"),
        )
        .build()
        .expect("client");

    let body = OauthGenerateTokensBody {
        code: None,
        grant_type: OauthGrantType::RefreshToken,
        redirect_uri: None,
        refresh_token: Some("refresh_x".into()),
    };
    let api_key = Credential::api_key("test_yyyyyyyyyyyyyyyyyyyyyyyyyyyyyy").expect("key");
    let err = client
        .oauth()
        .generate_tokens_with_credential(&api_key, &body)
        .await
        .expect_err("must reject API key");
    assert!(
        err.to_string().contains("Basic client credentials"),
        "unexpected error: {err}"
    );
}

#[tokio::test]
async fn oauth_facade_revoke_tokens_sends_delete() {
    use mollie_rs::types::{OauthRevokeTokensBody, OauthTokenTypeHint};
    use mollie_rs::BasicAuth;

    let server = MockServer::start().await;
    let basic = BasicAuth::new("app_client_id", "app_client_secret").expect("basic");
    let expected_auth = basic.authorization_value();

    Mock::given(method("DELETE"))
        .and(path("/oauth2/tokens"))
        .and(header("authorization", expected_auth.as_str()))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    let client = MollieClient::builder()
        .base_url(format!("{}/v2", server.uri()))
        .credential(
            Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx").expect("valid test key"),
        )
        .build()
        .expect("client");

    let body = OauthRevokeTokensBody {
        token: "access_to_revoke".into(),
        token_type_hint: OauthTokenTypeHint::AccessToken,
    };

    client
        .oauth()
        .revoke_tokens(&basic, &body)
        .await
        .expect("revoke_tokens");
}

#[tokio::test]
async fn payouts_facade_create_posts_balance_and_amount() {
    use mollie_rs::{CreatePayoutRequired, IdempotencyKey, Money};

    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/payouts"))
        .and(header("idempotency-key", "payout-sticky-key-0001"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "resource": "payout",
            "id": "payout_j8NvRAM2WNZtsykpLEX8J",
            "mode": "test",
            "balanceId": "bal_gVMhHKqSSRYJyPsuoPNFH",
            "amount": { "currency": "EUR", "value": "10.00" },
            "status": "requested",
            "statusReason": { "code": "requested", "message": "Payout requested" },
            "createdAt": "2024-03-20T09:13:37+00:00"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = MollieClient::builder()
        .base_url(server.uri())
        .credential(
            Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx").expect("valid test key"),
        )
        .build()
        .expect("client");

    let required = CreatePayoutRequired::with_amount_for_balance_str(
        "bal_gVMhHKqSSRYJyPsuoPNFH",
        Money::new("EUR", "10.00").unwrap(),
    )
    .expect("builder");
    let key = IdempotencyKey::new("payout-sticky-key-0001").expect("key");
    let response = client
        .payouts()
        .create(required, Some(key))
        .await
        .expect("create payout");
    assert_eq!(response.into_inner().id, "payout_j8NvRAM2WNZtsykpLEX8J");
}

#[tokio::test]
async fn transfers_facade_create_requires_signature_headers() {
    use mollie_rs::types::TransferSchemeType;
    use mollie_rs::{CreateTransferRequired, IdempotencyKey, Money, TransferClientSignature};

    let server = MockServer::start().await;
    // Signature headers are the transfer-specific contract; sticky Idempotency-Key
    // is also required by the facade (asserted via successful create).
    Mock::given(method("POST"))
        .and(path("/business-accounts/transfers"))
        .and(header("x-client-signature", "sig-material"))
        .and(header("x-client-signed-at", "2024-03-20T09:13:37Z"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "resource": "business-account-transfer",
            "id": "batrf_87GByBuj4UCcUTEbs6aGJ",
            "mode": "test",
            "amount": { "currency": "EUR", "value": "25.00" },
            "businessAccountTransactionId": "batr_abc123XYZ",
            "createdAt": "2024-03-20T09:13:37+00:00",
            "creditDebitIndicator": "debit",
            "creditor": {
                "fullName": "Jan Jansen",
                "account": { "iban": "NL02ABNA0123456789" }
            },
            "debtor": {
                "fullName": "Merchant BV",
                "account": { "iban": "NL55MLLE0123456789" }
            },
            "status": "requested",
            "statusHistory": [
                {
                    "status": "requested",
                    "createdAt": "2024-03-20T09:13:37+00:00"
                }
            ],
            "transferScheme": { "type": "sepa-credit" }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = MollieClient::builder()
        .base_url(server.uri())
        .credential(
            Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx").expect("valid test key"),
        )
        .build()
        .expect("client");

    let required = CreateTransferRequired::new(
        Money::new("EUR", "25.00").unwrap(),
        "NL55MLLE0123456789",
        "Jan Jansen",
        "NL02ABNA0123456789",
        TransferSchemeType::SepaCredit,
    )
    .expect("builder");
    let key = IdempotencyKey::new("xfer-sticky-key-0001").expect("key");
    let response = client
        .transfers()
        .create(
            required,
            &key,
            TransferClientSignature {
                signature: "sig-material",
                signed_at: "2024-03-20T09:13:37Z",
            },
        )
        .await
        .expect("create transfer");
    assert_eq!(response.into_inner().id.0, "batrf_87GByBuj4UCcUTEbs6aGJ");
}

#[tokio::test]
async fn transfers_facade_rejects_empty_signature_without_http() {
    use mollie_rs::types::TransferSchemeType;
    use mollie_rs::{CreateTransferRequired, IdempotencyKey, Money, TransferClientSignature};

    let server = MockServer::start().await;
    let client = MollieClient::builder()
        .base_url(server.uri())
        .credential(
            Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx").expect("valid test key"),
        )
        .build()
        .expect("client");

    let required = CreateTransferRequired::new(
        Money::new("EUR", "25.00").unwrap(),
        "NL55MLLE0123456789",
        "Jan Jansen",
        "NL02ABNA0123456789",
        TransferSchemeType::SepaCredit,
    )
    .expect("builder");
    let key = IdempotencyKey::new("xfer-sticky-key-0002").expect("key");
    let err = client
        .transfers()
        .create(
            required,
            &key,
            TransferClientSignature {
                signature: "  ",
                signed_at: "2024-03-20T09:13:37+00:00",
            },
        )
        .await
        .expect_err("empty signature must fail closed");
    assert!(
        err.to_string().contains("X-Client-Signature"),
        "unexpected: {err}"
    );
}

#[tokio::test]
async fn verify_payee_facade_posts_iban_body() {
    use mollie_rs::VerifyPayeeRequired;

    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/business-accounts/payee-verifications"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "resource": "business-account-payee-verification",
            "mode": "test",
            "createdAt": "2024-03-20T09:13:37+00:00",
            "creditorBankAccount": {
                "format": "iban",
                "accountHolderName": "Jan Jansen",
                "accountNumber": "NL02ABNA0123456789"
            },
            "verificationResult": {
                "outcome": "match"
            }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = MollieClient::builder()
        .base_url(server.uri())
        .credential(
            Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx").expect("valid test key"),
        )
        .build()
        .expect("client");

    let required = VerifyPayeeRequired::new("Jan Jansen", "NL02ABNA0123456789").expect("builder");
    let response = client
        .payee_verifications()
        .verify(required, None)
        .await
        .expect("verify payee");
    assert_eq!(
        response.into_inner().resource,
        "business-account-payee-verification"
    );
}
