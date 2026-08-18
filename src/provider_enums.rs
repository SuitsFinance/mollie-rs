//! Forward-compatible provider status enums (INV-ENUM-01).
//!
//! Generated OpenAPI enums remain closed for typed route decode. These Tier-S
//! wrappers preserve unknown provider strings via [`OpenEnum`] so applications
//! can round-trip new statuses without failing deserialize when used at the
//! facade boundary (webhooks, manual JSON, drift-tolerant views).

use std::fmt;
use std::str::FromStr;

use crate::open_enum::{OpenEnum, OpenEnumError};
use crate::types;

/// Known Mollie payment status values from the OpenAPI pin.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PaymentStatusKnown {
    /// Payment is open / checkout not finished.
    Open,
    /// Payment is pending provider confirmation.
    Pending,
    /// Payment is authorized but not captured.
    Authorized,
    /// Payment is paid.
    Paid,
    /// Payment was canceled.
    Canceled,
    /// Payment expired.
    Expired,
    /// Payment failed.
    Failed,
}

impl PaymentStatusKnown {
    /// Wire value used by Mollie.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Pending => "pending",
            Self::Authorized => "authorized",
            Self::Paid => "paid",
            Self::Canceled => "canceled",
            Self::Expired => "expired",
            Self::Failed => "failed",
        }
    }

    /// Builds an [`OpenEnum`] for this known status.
    pub fn open(self) -> PaymentStatusValue {
        OpenEnum::from_known(self, self.as_str()).expect("known status wire value is valid")
    }
}

impl fmt::Display for PaymentStatusKnown {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for PaymentStatusKnown {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "open" => Ok(Self::Open),
            "pending" => Ok(Self::Pending),
            "authorized" => Ok(Self::Authorized),
            "paid" => Ok(Self::Paid),
            "canceled" => Ok(Self::Canceled),
            "expired" => Ok(Self::Expired),
            "failed" => Ok(Self::Failed),
            _ => Err(()),
        }
    }
}

impl From<types::PaymentStatus> for PaymentStatusKnown {
    fn from(value: types::PaymentStatus) -> Self {
        match value {
            types::PaymentStatus::Open => Self::Open,
            types::PaymentStatus::Pending => Self::Pending,
            types::PaymentStatus::Authorized => Self::Authorized,
            types::PaymentStatus::Paid => Self::Paid,
            types::PaymentStatus::Canceled => Self::Canceled,
            types::PaymentStatus::Expired => Self::Expired,
            types::PaymentStatus::Failed => Self::Failed,
        }
    }
}

impl From<PaymentStatusKnown> for types::PaymentStatus {
    fn from(value: PaymentStatusKnown) -> Self {
        match value {
            PaymentStatusKnown::Open => Self::Open,
            PaymentStatusKnown::Pending => Self::Pending,
            PaymentStatusKnown::Authorized => Self::Authorized,
            PaymentStatusKnown::Paid => Self::Paid,
            PaymentStatusKnown::Canceled => Self::Canceled,
            PaymentStatusKnown::Expired => Self::Expired,
            PaymentStatusKnown::Failed => Self::Failed,
        }
    }
}

/// Open payment status that preserves unrecognized provider values.
pub type PaymentStatusValue = OpenEnum<PaymentStatusKnown>;

/// Parses a payment status string into an open enum.
///
/// # Errors
///
/// Returns [`OpenEnumError`] when the raw value is empty or too long.
pub fn parse_payment_status(raw: impl Into<String>) -> Result<PaymentStatusValue, OpenEnumError> {
    OpenEnum::parse_str(raw)
}

/// Converts a generated closed status into an open enum.
pub fn payment_status_from_generated(status: types::PaymentStatus) -> PaymentStatusValue {
    PaymentStatusKnown::from(status).open()
}

/// Converts an open status back to the generated enum when the value is known.
///
/// Returns `None` for unknown provider values (do not invent mappings).
pub fn payment_status_to_generated(status: &PaymentStatusValue) -> Option<types::PaymentStatus> {
    status.known().copied().map(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_status_round_trip_json() {
        let value: PaymentStatusValue = serde_json::from_str("\"paid\"").unwrap();
        assert_eq!(value.known(), Some(&PaymentStatusKnown::Paid));
        assert_eq!(serde_json::to_string(&value).unwrap(), "\"paid\"");
        assert_eq!(
            payment_status_to_generated(&value),
            Some(types::PaymentStatus::Paid)
        );
    }

    #[test]
    fn unknown_status_preserved() {
        let value: PaymentStatusValue = serde_json::from_str("\"partially_paid\"").unwrap();
        assert!(value.is_unknown());
        assert_eq!(value.as_str(), "partially_paid");
        assert_eq!(serde_json::to_string(&value).unwrap(), "\"partially_paid\"");
        assert_eq!(payment_status_to_generated(&value), None);
    }

    #[test]
    fn generated_conversion_covers_all_variants() {
        for status in [
            types::PaymentStatus::Open,
            types::PaymentStatus::Pending,
            types::PaymentStatus::Authorized,
            types::PaymentStatus::Paid,
            types::PaymentStatus::Canceled,
            types::PaymentStatus::Expired,
            types::PaymentStatus::Failed,
        ] {
            let open = payment_status_from_generated(status);
            assert_eq!(payment_status_to_generated(&open), Some(status));
        }
    }
}
