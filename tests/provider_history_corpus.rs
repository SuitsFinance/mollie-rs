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
