//! Validated Mollie resource identifiers.
//!
//! Generated OpenAPI path/body tokens (`types::PaymentToken`,
//! `types::ProfileToken`, …) are plain strings. This module checks the
//! documented id **prefix** before you pass a value into a route, so swapping
//! a profile id into a payment path (or the reverse) fails locally.
//!
//! | Prefix | Resource | Facade type |
//! | --- | --- | --- |
//! | `tr_` | payment | [`PaymentId`] |
//! | `pfl_` | profile | [`ProfileId`] |
#![warn(missing_docs)]

use std::{fmt, str::FromStr};

use crate::{types, MollieError, MollieResult};

/// A validated Mollie **payment** id (`tr_…`).
///
/// # Examples
///
/// ```rust
/// use mollie_rs::PaymentId;
///
/// let id = PaymentId::parse("tr_WDqYK6vllg")?;
/// assert_eq!(id.as_str(), "tr_WDqYK6vllg");
/// assert!(PaymentId::parse("pfl_QkEhN94Ba").is_err());
/// # Ok::<(), mollie_rs::MollieError>(())
/// ```
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PaymentId(String);

impl PaymentId {
    /// Resource id prefix for payments.
    pub const PREFIX: &'static str = "tr_";

    /// Parses a payment id (`tr_` + non-empty suffix).
    ///
    /// # Errors
    ///
    /// Returns [`MollieError::InvalidRequest`] when the value is blank, has the
    /// wrong prefix (for example a `pfl_` profile id), or has an empty suffix.
    pub fn parse(value: impl AsRef<str>) -> MollieResult<Self> {
        let value = value.as_ref().trim();
        validate_prefixed_id("payment", Self::PREFIX, value)?;
        Ok(Self(value.to_string()))
    }

    /// Returns the raw payment id string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the id and returns the owned string.
    pub fn into_string(self) -> String {
        self.0
    }

    /// Converts into the generated path/body [`types::PaymentToken`].
    pub fn into_token(self) -> types::PaymentToken {
        types::PaymentToken(self.0)
    }

    /// Returns true when `value` looks like a payment id.
    pub fn is_valid(value: impl AsRef<str>) -> bool {
        Self::parse(value).is_ok()
    }
}

impl FromStr for PaymentId {
    type Err = MollieError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl fmt::Display for PaymentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for PaymentId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl TryFrom<&str> for PaymentId {
    type Error = MollieError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl TryFrom<String> for PaymentId {
    type Error = MollieError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<PaymentId> for types::PaymentToken {
    fn from(value: PaymentId) -> Self {
        value.into_token()
    }
}

impl From<PaymentId> for String {
    fn from(value: PaymentId) -> Self {
        value.into_string()
    }
}

/// A validated Mollie **profile** id (`pfl_…`).
///
/// # Examples
///
/// ```rust
/// use mollie_rs::ProfileId;
///
/// let id = ProfileId::parse("pfl_QkEhN94Ba")?;
/// assert_eq!(id.as_str(), "pfl_QkEhN94Ba");
/// assert!(ProfileId::parse("tr_WDqYK6vllg").is_err());
/// # Ok::<(), mollie_rs::MollieError>(())
/// ```
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProfileId(String);

impl ProfileId {
    /// Resource id prefix for profiles.
    pub const PREFIX: &'static str = "pfl_";

    /// Parses a profile id (`pfl_` + non-empty suffix).
    ///
    /// # Errors
    ///
    /// Returns [`MollieError::InvalidRequest`] when the value is blank, has the
    /// wrong prefix (for example a `tr_` payment id), or has an empty suffix.
    pub fn parse(value: impl AsRef<str>) -> MollieResult<Self> {
        let value = value.as_ref().trim();
        validate_prefixed_id("profile", Self::PREFIX, value)?;
        Ok(Self(value.to_string()))
    }

    /// Returns the raw profile id string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the id and returns the owned string.
    pub fn into_string(self) -> String {
        self.0
    }

    /// Converts into the generated path/body [`types::ProfileToken`].
    pub fn into_token(self) -> types::ProfileToken {
        types::ProfileToken(self.0)
    }

    /// Returns true when `value` looks like a profile id.
    pub fn is_valid(value: impl AsRef<str>) -> bool {
        Self::parse(value).is_ok()
    }
}

impl FromStr for ProfileId {
    type Err = MollieError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl fmt::Display for ProfileId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for ProfileId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl TryFrom<&str> for ProfileId {
    type Error = MollieError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl TryFrom<String> for ProfileId {
    type Error = MollieError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<ProfileId> for types::ProfileToken {
    fn from(value: ProfileId) -> Self {
        value.into_token()
    }
}

impl From<ProfileId> for String {
    fn from(value: ProfileId) -> Self {
        value.into_string()
    }
}

fn validate_prefixed_id(kind: &str, prefix: &str, value: &str) -> MollieResult<()> {
    if value.is_empty() {
        return Err(MollieError::invalid_request(format!(
            "{kind} id cannot be empty (expected prefix `{prefix}`)"
        )));
    }
    if value.chars().any(|c| c.is_whitespace() || c.is_control()) {
        return Err(MollieError::invalid_request(format!(
            "invalid {kind} id `{value}`: whitespace and control characters are not allowed"
        )));
    }
    if value.chars().count() > 32 {
        return Err(MollieError::invalid_request(format!(
            "invalid {kind} id `{value}`: identifiers may not exceed 32 characters"
        )));
    }
    if !value.starts_with(prefix) {
        let hint = if value.starts_with(PaymentId::PREFIX) {
            "this looks like a payment id (`tr_`)"
        } else if value.starts_with(ProfileId::PREFIX) {
            "this looks like a profile id (`pfl_`)"
        } else {
            "check the resource prefix"
        };
        return Err(MollieError::invalid_request(format!(
            "invalid {kind} id `{value}`: expected prefix `{prefix}` ({hint})"
        )));
    }
    let suffix = &value[prefix.len()..];
    if suffix.is_empty() {
        return Err(MollieError::invalid_request(format!(
            "invalid {kind} id `{value}`: suffix after `{prefix}` cannot be empty"
        )));
    }
    if !suffix
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        return Err(MollieError::invalid_request(format!(
            "invalid {kind} id `{value}`: suffix may only contain ASCII alphanumeric characters, `-`, or `_`"
        )));
    }
    Ok(())
}

macro_rules! validated_id {
    ($name:ident, $token:ident, $resource:literal, $prefix:literal) => {
        #[doc = concat!("A validated Mollie ", $resource, " id (`", $prefix, "...`).")]
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            /// The documented resource id prefix.
            pub const PREFIX: &'static str = $prefix;

            /// Parses and validates a resource id.
            pub fn parse(value: impl AsRef<str>) -> MollieResult<Self> {
                let value = value.as_ref().trim();
                validate_prefixed_id($resource, Self::PREFIX, value)?;
                Ok(Self(value.to_string()))
            }

            /// Returns the raw resource id string.
            pub fn as_str(&self) -> &str {
                &self.0
            }

            /// Consumes the id and returns the owned string.
            pub fn into_string(self) -> String {
                self.0
            }

            /// Converts into the generated route token.
            pub fn into_token(self) -> types::$token {
                types::$token(self.0)
            }

            /// Returns true when the value has the expected resource prefix.
            pub fn is_valid(value: impl AsRef<str>) -> bool {
                Self::parse(value).is_ok()
            }
        }

        impl FromStr for $name {
            type Err = MollieError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::parse(value)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl TryFrom<&str> for $name {
            type Error = MollieError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::parse(value)
            }
        }

        impl TryFrom<String> for $name {
            type Error = MollieError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::parse(value)
            }
        }

        impl From<$name> for types::$token {
            fn from(value: $name) -> Self {
                value.into_token()
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.into_string()
            }
        }
    };
}

validated_id!(CustomerId, CustomerToken, "customer", "cst_");
validated_id!(RefundId, RefundToken, "refund", "re_");
validated_id!(SubscriptionId, SubscriptionToken, "subscription", "sub_");
validated_id!(MandateId, MandateToken, "mandate", "mdt_");
validated_id!(PaymentLinkId, PaymentLinkToken, "payment link", "pl_");
validated_id!(
    SalesInvoiceId,
    SalesInvoiceToken,
    "sales invoice",
    "invoice_"
);
validated_id!(SettlementId, SettlementToken, "settlement", "stl_");
validated_id!(CaptureId, CaptureToken, "capture", "cpt_");
validated_id!(ChargebackId, ChargebackToken, "chargeback", "chb_");
validated_id!(BalanceId, BalanceToken, "balance", "bal_");
validated_id!(TerminalId, TerminalToken, "terminal", "term_");

#[cfg(test)]
mod tests {
    use super::{CustomerId, PaymentId, ProfileId, RefundId, SubscriptionId};

    #[test]
    fn payment_id_accepts_tr_prefix_only() {
        let id = PaymentId::parse("tr_WDqYK6vllg").unwrap();
        assert_eq!(id.as_str(), "tr_WDqYK6vllg");
        assert_eq!(id.into_token().0, "tr_WDqYK6vllg");

        assert!(PaymentId::parse("pfl_QkEhN94Ba").is_err());
        assert!(PaymentId::parse("tr_").is_err());
        assert!(PaymentId::parse("").is_err());
        assert!(PaymentId::parse(" tr_x ").is_ok()); // trim
        assert!(PaymentId::parse("tr_x y").is_err());
    }

    #[test]
    fn profile_id_accepts_pfl_prefix_only() {
        let id = ProfileId::parse("pfl_QkEhN94Ba").unwrap();
        assert_eq!(id.as_str(), "pfl_QkEhN94Ba");

        assert!(ProfileId::parse("tr_WDqYK6vllg").is_err());
        assert!(ProfileId::parse("pfl_").is_err());
        assert!(!ProfileId::is_valid("payment_id"));
    }

    #[test]
    /// Ensures route-specific prefixes reject ids from another Mollie resource.
    fn resource_ids_reject_wrong_route_tokens() {
        assert!(CustomerId::parse("cst_123").is_ok());
        assert!(RefundId::parse("re_123").is_ok());
        assert!(SubscriptionId::parse("sub_123").is_ok());
        assert!(CustomerId::parse("re_123").is_err());
        assert!(RefundId::parse("sub_123").is_err());
        assert!(SubscriptionId::parse("cst_123").is_err());
    }

    #[test]
    fn resource_ids_enforce_mollie_maximum_length() {
        assert!(PaymentId::parse(format!("tr_{}", "a".repeat(29))).is_ok());
        assert!(PaymentId::parse(format!("tr_{}", "a".repeat(30))).is_err());
    }
}
