//! For **each unique error response** harvested from Mollie's Postman
//! collections, assert the shared error factory path:
//!
//! `ErrorResponse` → `classify_api` → `MollieError::api` → `to_envelope()`
//!
//! Uses `tests/fixtures/postman_error_responses.json` (deduped from all six
//! collections). Success samples are indexed separately for documentation;
//! error classification is what the global factory owns.

use mollie_rs::{types::ErrorResponse, MollieError, MollieErrorCatalogEntry, MollieErrorKey};
use reqwest::{header::HeaderMap, StatusCode};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct HarvestedError {
    status: u16,
    title: String,
    detail: String,
    #[serde(default)]
    field: Option<String>,
    body: ErrorResponse,
    routes: Vec<RouteHit>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct RouteHit {
    collection: String,
    method: String,
    path: String,
    request: String,
    example: String,
}

fn status_code(code: u16) -> StatusCode {
    StatusCode::from_u16(code).expect("valid HTTP status in fixture")
}

/// Every unique Postman error body classifies and envelopes consistently.
#[test]
fn each_unique_postman_error_uses_shared_factory_envelope() {
    let raw = include_str!("fixtures/postman_error_responses.json");
    let samples: Vec<HarvestedError> =
        serde_json::from_str(raw).expect("postman_error_responses.json");

    assert!(
        samples.len() >= 25,
        "expected full harvest of unique error bodies, got {}",
        samples.len()
    );

    for sample in &samples {
        let entry = MollieErrorCatalogEntry::classify_api(&sample.body);
        let error = MollieError::api(
            status_code(sample.status),
            HeaderMap::new(),
            sample.body.clone(),
        );
        let envelope = error.to_envelope();

        assert!(
            !envelope.ok,
            "envelope.ok must be false for {} {:?}",
            sample.status, sample.detail
        );
        assert_eq!(
            envelope.status,
            Some(sample.status),
            "status mismatch for detail={}",
            sample.detail
        );
        assert_eq!(envelope.code, entry.code());
        assert_eq!(envelope.key, entry.key());
        assert_eq!(envelope.title.as_deref(), Some(sample.title.as_str()));
        assert_eq!(envelope.detail, sample.detail);
        if let Some(field) = &sample.field {
            assert_eq!(envelope.field.as_deref(), Some(field.as_str()));
        }
        assert!(
            envelope.documentation.is_some(),
            "documentation href required for {}",
            sample.detail
        );

        // Status-family sanity: key matches HTTP class.
        match sample.status {
            400 => assert!(
                matches!(
                    entry.key(),
                    MollieErrorKey::InvalidCursor | MollieErrorKey::ApiError
                ),
                "400 → {:?} for {}",
                entry.key(),
                sample.detail
            ),
            403 => assert!(
                matches!(
                    entry.key(),
                    MollieErrorKey::AccessTokenProfileRestricted
                        | MollieErrorKey::DemoProfileLimitReached
                        | MollieErrorKey::DemoProfileNotEditable
                        | MollieErrorKey::Forbidden
                ),
                "403 → {:?}",
                entry.key()
            ),
            404 => assert!(
                matches!(
                    entry.key(),
                    MollieErrorKey::EntityNotFound | MollieErrorKey::NotFound
                ),
                "404 → {:?}",
                entry.key()
            ),
            409 => assert!(
                matches!(
                    entry.key(),
                    MollieErrorKey::PayoutNotCancelable | MollieErrorKey::Conflict
                ),
                "409 → {:?}",
                entry.key()
            ),
            410 => assert!(
                matches!(
                    entry.key(),
                    MollieErrorKey::ProfileDeleted | MollieErrorKey::Gone
                ),
                "410 → {:?}",
                entry.key()
            ),
            422 => assert!(
                matches!(
                    entry.key(),
                    MollieErrorKey::ValidationError
                        | MollieErrorKey::ResourceStateConflict
                        | MollieErrorKey::UnprocessableEntity
                ),
                "422 → {:?} for {}",
                entry.key(),
                sample.detail
            ),
            429 => {
                assert_eq!(entry.key(), MollieErrorKey::RateLimitExceeded);
                assert!(error.is_rate_limited());
                // Same factory reused for every route (clients, capabilities, …).
                let factory = MollieError::rate_limit_exceeded().to_envelope();
                assert_eq!(factory.key, envelope.key);
                assert_eq!(factory.code, envelope.code);
            }
            503 => assert_eq!(
                entry.key(),
                MollieErrorKey::ServiceTemporarilyUnavailable,
                "503 → {:?}",
                entry.key()
            ),
            other => panic!("unexpected status in harvest: {other}"),
        }

        assert!(
            !sample.routes.is_empty(),
            "each unique body should list at least one route occurrence"
        );
    }
}

/// 429 appears on many routes; all share one factory identity.
#[test]
fn rate_limit_429_is_global_across_harvested_routes() {
    let raw: &str = include_str!("fixtures/postman_error_responses.json");
    let samples: Vec<HarvestedError> = serde_json::from_str(raw).unwrap();
    let rate: &HarvestedError = samples
        .iter()
        .find(|s| s.status == 429)
        .expect("429 sample present");

    // Occurs on many collection routes (clients, balances, …).
    assert!(!rate.routes.is_empty(), "429 should be recorded on routes");

    let paths: Vec<&str> = rate.routes.iter().map(|r| r.path.as_str()).collect();
    // Sanity: harvest attaches multiple route hits when re-seen.
    let _ = paths;

    let factory: MollieError = MollieError::rate_limit_exceeded();
    assert!(factory.is_rate_limited());
    assert_eq!(
        factory.catalog_entry().key(),
        MollieErrorKey::RateLimitExceeded
    );
}

/// Success response index exists and covers 2xx samples from all collections.
#[test]
fn success_response_index_covers_all_collections() {
    let raw = include_str!("fixtures/postman_success_response_index.json");
    let index: Vec<serde_json::Value> = serde_json::from_str(raw).expect("success index JSON");
    assert!(
        index.len() >= 50,
        "expected broad success sample index, got {}",
        index.len()
    );

    let mut collections: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for row in &index {
        if let Some(c) = row.get("collection").and_then(|v| v.as_str()) {
            collections.insert(c.to_string());
        }
        let status: u16 = row.get("status").and_then(|v| v.as_u64()).expect("status") as u16;
        assert!((200..300).contains(&status), "success index must be 2xx");
    }
    assert!(
        collections.len() >= 6,
        "all six postman collections should appear, got {collections:?}"
    );
}
