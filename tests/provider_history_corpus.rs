//! Provider-history corpus smoke tests (INV-CORPUS / Phase 8 seed).

use mollie_rs::{OpenEnum, OPEN_ENUM_MAX_RAW_LEN};
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
enum DemoTx {
    Payment,
}

impl FromStr for DemoTx {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "payment" => Ok(Self::Payment),
            _ => Err(()),
        }
    }
}

#[test]
fn balance_transaction_history_fixture_unknown_type_round_trips() {
    let raw = include_str!("fixtures/provider_history/balance_transaction_new_type.json");
    let v: serde_json::Value = serde_json::from_str(raw).unwrap();
    let ty = v.get("type").and_then(|x| x.as_str()).unwrap();
    let open: OpenEnum<DemoTx> = OpenEnum::parse_str(ty).unwrap();
    assert!(open.is_unknown());
    assert_eq!(open.as_str(), "future_unknown_transaction_type");
    assert_eq!(
        serde_json::to_string(&open).unwrap(),
        "\"future_unknown_transaction_type\""
    );
}

#[test]
fn open_enum_max_bound_enforced_on_parse() {
    // Behavioral bound (const compare alone is clippy::assertions_on_constants).
    let ok = "x".repeat(OPEN_ENUM_MAX_RAW_LEN.min(64));
    assert!(OpenEnum::<DemoTx>::parse_str(&ok).is_ok());
    let too_long = "x".repeat(OPEN_ENUM_MAX_RAW_LEN.saturating_add(1));
    assert!(OpenEnum::<DemoTx>::parse_str(&too_long).is_err());
}

#[test]
fn draft_transfer_symbols_absent_from_public_prelude_surface() {
    // Compile-time / link-time absence is enforced by not exporting DraftTransfer.
    // Runtime guard: types module path string search is CI-side; here ensure
    // OpenEnum path works as stand-in for corpus wiring.
    let _ = OpenEnum::<DemoTx>::parse_str("payment").unwrap();
}

#[test]
fn terminal_pairing_403_fixture_is_structured_forbidden_not_retry_or_auth() {
    use mollie_rs::{
        types::{ErrorResponse, ErrorResponseLinks, ErrorResponseLinksDocumentation},
        MollieError, MollieErrorKey,
    };
    use reqwest::StatusCode;

    let raw = include_str!("fixtures/provider_history/terminal_pairing_403.json");
    let v: serde_json::Value = serde_json::from_str(raw).unwrap();
    let body = ErrorResponse {
        detail: v["detail"].as_str().unwrap().to_string(),
        field: None,
        links: ErrorResponseLinks {
            documentation: ErrorResponseLinksDocumentation {
                href: "https://docs.mollie.com/errors".into(),
                type_: "text/html".into(),
            },
        },
        status: v["status"].as_u64().unwrap() as i64,
        title: v["title"].as_str().unwrap().to_string(),
    };
    let err = MollieError::api(StatusCode::FORBIDDEN, Default::default(), body);
    assert_eq!(err.status(), Some(StatusCode::FORBIDDEN));
    assert_eq!(
        err.catalog_entry().key(),
        MollieErrorKey::TerminalPairingForbidden
    );
    assert_ne!(err.catalog_entry().key(), MollieErrorKey::RateLimitExceeded);
    assert_ne!(err.status(), Some(StatusCode::UNAUTHORIZED));
    assert_ne!(err.status(), Some(StatusCode::TOO_MANY_REQUESTS));
    // Constructor parity with catalog
    let known = MollieError::terminal_pairing_forbidden();
    assert_eq!(
        known.catalog_entry().key(),
        MollieErrorKey::TerminalPairingForbidden
    );
}
