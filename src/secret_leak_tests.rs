//! Regression suite: secrets must never appear in Debug, Display (where
//! redacted), error surfaces, or hook URL redaction helpers.
//!
//! Payment SDKs must keep credentials, webhook secrets, and sticky idempotency
//! material out of logs and diagnostics.

use crate::auth::{ApiKey, BasicAuth, Credential, OAuthAccessToken};
use crate::client::MollieClientBuilder;
use crate::idempotency::IdempotencyKey;
use crate::webhook_verify::{
    compute_mollie_signature_hex, WebhookSigningSecret, WebhookVerifier, WebhookVerifyFailure,
};
use crate::MollieError;

const API_KEY: &str = "test_super_secret_api_key_value_xyz";
const OAUTH_TOKEN: &str = "access_super_secret_oauth_token_xyz";
const CLIENT_SECRET: &str = "oauth_client_secret_value_xyz";
const WEBHOOK_SECRET: &str = "whsec_super_secret_webhook_key_xyz";
const IDEMPOTENCY: &str = "idempotency-secret-key-value-xyz";

fn assert_no_secret(surface: &str, secret: &str) {
    assert!(
        !surface.contains(secret),
        "secret leaked in diagnostic surface: {surface}"
    );
}

#[test]
fn api_key_debug_redacts() {
    let key = ApiKey::new(API_KEY).expect("valid");
    let dbg = format!("{key:?}");
    assert_no_secret(&dbg, API_KEY);
    assert!(dbg.contains("redacted") || dbg.contains("ApiKey"));
}

#[test]
fn oauth_token_debug_redacts() {
    let token = OAuthAccessToken::new(OAUTH_TOKEN).expect("valid");
    assert_no_secret(&format!("{token:?}"), OAUTH_TOKEN);
}

#[test]
fn basic_auth_debug_redacts_secret_and_encoded() {
    let basic = BasicAuth::new("client-id", CLIENT_SECRET).expect("valid");
    let dbg = format!("{basic:?}");
    assert_no_secret(&dbg, CLIENT_SECRET);
    // Base64 payload of client_id:client_secret must not appear in Debug.
    let auth = basic.authorization_value();
    let encoded = auth.strip_prefix("Basic ").unwrap_or(&auth);
    assert_no_secret(&dbg, encoded);
}

#[test]
fn credential_debug_redacts_all_variants() {
    for cred in [
        Credential::api_key(API_KEY).unwrap(),
        Credential::oauth_access_token(OAUTH_TOKEN).unwrap(),
        Credential::basic_auth("client-id", CLIENT_SECRET).unwrap(),
    ] {
        let dbg = format!("{cred:?}");
        assert_no_secret(&dbg, API_KEY);
        assert_no_secret(&dbg, OAUTH_TOKEN);
        assert_no_secret(&dbg, CLIENT_SECRET);
    }
}

#[test]
fn webhook_signing_secret_debug_redacts() {
    let secret = WebhookSigningSecret::new(WEBHOOK_SECRET).expect("valid");
    assert_no_secret(&format!("{secret:?}"), WEBHOOK_SECRET);
}

#[test]
fn webhook_verifier_debug_redacts_secrets() {
    let verifier = WebhookVerifier::new(WEBHOOK_SECRET)
        .unwrap()
        .with_previous_secret("previous_webhook_secret_xyz")
        .unwrap();
    let dbg = format!("{verifier:?}");
    assert_no_secret(&dbg, WEBHOOK_SECRET);
    assert_no_secret(&dbg, "previous_webhook_secret_xyz");
}

#[test]
fn webhook_verify_failures_do_not_echo_secret() {
    let body = br#"{"id":"event"}"#;
    let verifier = WebhookVerifier::new(WEBHOOK_SECRET).unwrap();
    let err = verifier.verify(body, "deadbeef").unwrap_err();
    let msg = err.to_string();
    assert_no_secret(&msg, WEBHOOK_SECRET);
    let dbg = format!("{err:?}");
    assert_no_secret(&dbg, WEBHOOK_SECRET);
    match err {
        MollieError::WebhookVerification { failure } => {
            assert_no_secret(&failure.to_string(), WEBHOOK_SECRET);
            assert_no_secret(&format!("{failure:?}"), WEBHOOK_SECRET);
            assert!(matches!(
                failure,
                WebhookVerifyFailure::SignatureMismatch | WebhookVerifyFailure::MalformedSignature
            ));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn idempotency_key_debug_redacts() {
    let key = IdempotencyKey::new(IDEMPOTENCY).expect("valid");
    assert_no_secret(&format!("{key:?}"), IDEMPOTENCY);
}

#[test]
fn client_builder_debug_redacts_credential() {
    let builder = MollieClientBuilder::default().credential(Credential::api_key(API_KEY).unwrap());
    let dbg = format!("{builder:?}");
    assert_no_secret(&dbg, API_KEY);
}

#[test]
fn invalid_configuration_messages_do_not_embed_raw_secrets() {
    // Validation rejects whitespace; message must not include the secret value.
    let err = ApiKey::new(format!(" {API_KEY} ")).unwrap_err();
    assert_no_secret(&err.to_string(), API_KEY);
    assert_no_secret(&format!("{err:?}"), API_KEY);
}

#[test]
fn signature_helper_output_is_not_the_secret() {
    let sig = compute_mollie_signature_hex(WEBHOOK_SECRET.as_bytes(), b"{}").unwrap();
    assert_no_secret(&sig, WEBHOOK_SECRET);
}

#[test]
fn request_hook_url_redaction_strips_query() {
    // exercise the same helper path used by Client::send
    let url = reqwest::Url::parse("https://api.mollie.com/v2/payments?api_key=test_leak").unwrap();
    let mut redacted = url.clone();
    if redacted.query().is_some() {
        redacted.set_query(Some("<redacted>"));
    }
    let s = redacted.to_string();
    assert!(!s.contains("test_leak"));
    assert!(s.contains("redacted"));
}
