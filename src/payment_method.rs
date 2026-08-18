//! Typed payment method helpers for request payloads.
//!
//! The generated OpenAPI wire types remain in [`crate::types`]:
//! - [`types::Method`] / [`types::MethodInner`] for payment create/update `method`
//! - [`types::PaymentLinkMethods`] for payment-link `allowedMethods` (string list)
//!
//! This module is a small validated facade so applications can pick a method
//! with a typed enum, parse/validate strings before sending, and convert into
//! the generated request fields.
#![warn(missing_docs)]

use std::{fmt, str::FromStr};

use crate::{types, MollieError, MollieResult};

/// A Mollie payment method accepted when selecting or restricting methods on
/// requests (payments, payment links, etc.).
///
/// Wire value is the lowercase identifier Mollie expects (for example `ideal`,
/// `creditcard`). Deprecated or response-only identifiers such as `googlepay`
/// are intentionally **not** accepted here; use this type for outbound requests
/// so invalid variants fail locally instead of as API/serde errors.
///
/// Wraps the generated [`types::MethodInner`] enum from the checked-in OpenAPI
/// spec.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PaymentMethod(types::MethodInner);

impl PaymentMethod {
    /// `alma`
    pub const ALMA: Self = Self(types::MethodInner::Alma);
    /// `applepay`
    pub const APPLEPAY: Self = Self(types::MethodInner::Applepay);
    /// `bacs`
    pub const BACS: Self = Self(types::MethodInner::Bacs);
    /// `bancomatpay`
    pub const BANCOMATPAY: Self = Self(types::MethodInner::Bancomatpay);
    /// `bancontact`
    pub const BANCONTACT: Self = Self(types::MethodInner::Bancontact);
    /// `banktransfer`
    pub const BANKTRANSFER: Self = Self(types::MethodInner::Banktransfer);
    /// `belfius`
    pub const BELFIUS: Self = Self(types::MethodInner::Belfius);
    /// `billie`
    pub const BILLIE: Self = Self(types::MethodInner::Billie);
    /// `billink`
    pub const BILLINK: Self = Self(types::MethodInner::Billink);
    /// `bizum`
    pub const BIZUM: Self = Self(types::MethodInner::Bizum);
    /// `blik`
    pub const BLIK: Self = Self(types::MethodInner::Blik);
    /// `creditcard`
    pub const CREDITCARD: Self = Self(types::MethodInner::Creditcard);
    /// `directdebit`
    pub const DIRECTDEBIT: Self = Self(types::MethodInner::Directdebit);
    /// `eps`
    pub const EPS: Self = Self(types::MethodInner::Eps);
    /// `giftcard`
    pub const GIFTCARD: Self = Self(types::MethodInner::Giftcard);
    /// `ideal`
    pub const IDEAL: Self = Self(types::MethodInner::Ideal);
    /// `in3`
    pub const IN3: Self = Self(types::MethodInner::In3);
    /// `kbc`
    pub const KBC: Self = Self(types::MethodInner::Kbc);
    /// `klarna`
    pub const KLARNA: Self = Self(types::MethodInner::Klarna);
    /// `mbway`
    pub const MBWAY: Self = Self(types::MethodInner::Mbway);
    /// `mobilepay`
    pub const MOBILEPAY: Self = Self(types::MethodInner::Mobilepay);
    /// `multibanco`
    pub const MULTIBANCO: Self = Self(types::MethodInner::Multibanco);
    /// `mybank`
    pub const MYBANK: Self = Self(types::MethodInner::Mybank);
    /// `paybybank`
    pub const PAYBYBANK: Self = Self(types::MethodInner::Paybybank);
    /// `paypal`
    pub const PAYPAL: Self = Self(types::MethodInner::Paypal);
    /// `paysafecard`
    pub const PAYSAFECARD: Self = Self(types::MethodInner::Paysafecard);
    /// `pointofsale`
    pub const POINTOFSALE: Self = Self(types::MethodInner::Pointofsale);
    /// `przelewy24`
    pub const PRZELEWY24: Self = Self(types::MethodInner::Przelewy24);
    /// `riverty`
    pub const RIVERTY: Self = Self(types::MethodInner::Riverty);
    /// `satispay`
    pub const SATISPAY: Self = Self(types::MethodInner::Satispay);
    /// `swish`
    pub const SWISH: Self = Self(types::MethodInner::Swish);
    /// `trustly`
    pub const TRUSTLY: Self = Self(types::MethodInner::Trustly);
    /// `twint`
    pub const TWINT: Self = Self(types::MethodInner::Twint);
    /// `vipps`
    pub const VIPPS: Self = Self(types::MethodInner::Vipps);
    /// `voucher`
    pub const VOUCHER: Self = Self(types::MethodInner::Voucher);

    /// All payment methods accepted by the current OpenAPI pin.
    pub const SUPPORTED: [Self; 35] = [
        Self::ALMA,
        Self::APPLEPAY,
        Self::BACS,
        Self::BANCOMATPAY,
        Self::BANCONTACT,
        Self::BANKTRANSFER,
        Self::BELFIUS,
        Self::BILLIE,
        Self::BILLINK,
        Self::BIZUM,
        Self::BLIK,
        Self::CREDITCARD,
        Self::DIRECTDEBIT,
        Self::EPS,
        Self::GIFTCARD,
        Self::IDEAL,
        Self::IN3,
        Self::KBC,
        Self::KLARNA,
        Self::MBWAY,
        Self::MOBILEPAY,
        Self::MULTIBANCO,
        Self::MYBANK,
        Self::PAYBYBANK,
        Self::PAYPAL,
        Self::PAYSAFECARD,
        Self::POINTOFSALE,
        Self::PRZELEWY24,
        Self::RIVERTY,
        Self::SATISPAY,
        Self::SWISH,
        Self::TRUSTLY,
        Self::TWINT,
        Self::VIPPS,
        Self::VOUCHER,
    ];

    /// Parses a supported payment method identifier.
    ///
    /// Comparison is case-sensitive and matches Mollie's wire values
    /// (`ideal`, not `iDEAL` or `googlepay`).
    ///
    /// # Errors
    ///
    /// Returns [`MollieError::InvalidRequest`] when the value is not a
    /// request-supported method in the checked-in OpenAPI spec.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::PaymentMethod;
    ///
    /// let method = PaymentMethod::parse("ideal")?;
    /// assert_eq!(method.as_str(), "ideal");
    /// assert!(PaymentMethod::parse("googlepay").is_err());
    /// # Ok::<(), mollie_rs::MollieError>(())
    /// ```
    pub fn parse(value: impl AsRef<str>) -> MollieResult<Self> {
        let raw = value.as_ref();
        let inner = raw.parse::<types::MethodInner>().map_err(|_| {
            MollieError::invalid_request(format!(
                "unsupported payment method `{raw}` (not in OpenAPI Method enum)"
            ))
        })?;
        Ok(Self(inner))
    }

    /// Returns the lowercase Mollie method identifier.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::PaymentMethod;
    ///
    /// assert_eq!(PaymentMethod::IDEAL.as_str(), "ideal");
    /// ```
    pub fn as_str(self) -> &'static str {
        match self.0 {
            types::MethodInner::Alma => "alma",
            types::MethodInner::Applepay => "applepay",
            types::MethodInner::Bacs => "bacs",
            types::MethodInner::Bancomatpay => "bancomatpay",
            types::MethodInner::Bancontact => "bancontact",
            types::MethodInner::Banktransfer => "banktransfer",
            types::MethodInner::Belfius => "belfius",
            types::MethodInner::Billie => "billie",
            types::MethodInner::Billink => "billink",
            types::MethodInner::Bizum => "bizum",
            types::MethodInner::Blik => "blik",
            types::MethodInner::Creditcard => "creditcard",
            types::MethodInner::Directdebit => "directdebit",
            types::MethodInner::Eps => "eps",
            types::MethodInner::Giftcard => "giftcard",
            types::MethodInner::Ideal => "ideal",
            types::MethodInner::In3 => "in3",
            types::MethodInner::Kbc => "kbc",
            types::MethodInner::Klarna => "klarna",
            types::MethodInner::Mbway => "mbway",
            types::MethodInner::Mobilepay => "mobilepay",
            types::MethodInner::Multibanco => "multibanco",
            types::MethodInner::Mybank => "mybank",
            types::MethodInner::Paybybank => "paybybank",
            types::MethodInner::Paypal => "paypal",
            types::MethodInner::Paysafecard => "paysafecard",
            types::MethodInner::Pointofsale => "pointofsale",
            types::MethodInner::Przelewy24 => "przelewy24",
            types::MethodInner::Riverty => "riverty",
            types::MethodInner::Satispay => "satispay",
            types::MethodInner::Swish => "swish",
            types::MethodInner::Trustly => "trustly",
            types::MethodInner::Twint => "twint",
            types::MethodInner::Vipps => "vipps",
            types::MethodInner::Voucher => "voucher",
        }
    }

    /// Returns the generated OpenAPI method enum variant.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::{types, PaymentMethod};
    ///
    /// let generated: types::MethodInner = PaymentMethod::IDEAL.into_generated();
    /// assert_eq!(generated.to_string(), "ideal");
    /// ```
    pub const fn into_generated(self) -> types::MethodInner {
        self.0
    }

    /// Converts into the generated nullable [`types::Method`] wire wrapper used
    /// on payment create/update bodies.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::{types, PaymentMethod};
    ///
    /// let method: types::Method = PaymentMethod::IDEAL.into_method();
    /// ```
    pub fn into_method(self) -> types::Method {
        types::Method(Some(self.0))
    }

    /// Returns true when a method identifier is supported for request payloads.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::PaymentMethod;
    ///
    /// assert!(PaymentMethod::is_supported("ideal"));
    /// assert!(!PaymentMethod::is_supported("googlepay"));
    /// ```
    pub fn is_supported(value: impl AsRef<str>) -> bool {
        Self::parse(value).is_ok()
    }

    /// Validates a list of method identifiers and builds
    /// [`types::PaymentLinkMethods`] for `CreatePaymentLinkBody.allowed_methods`.
    ///
    /// Empty iterators produce `PaymentLinkMethods(Some(vec![]))` (same meaning
    /// as an empty allow-list on the wire: no methods restricted / all enabled
    /// depending on Mollie's empty-array semantics).
    ///
    /// # Errors
    ///
    /// Returns [`MollieError::InvalidRequest`] on the first unsupported method.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::{types, PaymentMethod};
    ///
    /// let allowed: types::PaymentLinkMethods =
    ///     PaymentMethod::payment_link_methods([PaymentMethod::IDEAL, PaymentMethod::CREDITCARD])?;
    /// assert_eq!(
    ///     allowed.0,
    ///     Some(vec![types::PaymentLinkMethod::Ideal, types::PaymentLinkMethod::Creditcard])
    /// );
    /// assert!(PaymentMethod::parse_payment_link_methods(["googlepay"]).is_err());
    /// # Ok::<(), mollie_rs::MollieError>(())
    /// ```
    pub fn payment_link_methods<I>(methods: I) -> MollieResult<types::PaymentLinkMethods>
    where
        I: IntoIterator<Item = Self>,
    {
        let out: Vec<types::PaymentLinkMethod> = methods
            .into_iter()
            .map(|method| method.as_str().parse::<types::PaymentLinkMethod>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| MollieError::invalid_request(error.to_string()))?;
        Ok(types::PaymentLinkMethods(Some(out)))
    }

    /// Parses and validates string method identifiers into
    /// [`types::PaymentLinkMethods`].
    ///
    /// # Errors
    ///
    /// Returns [`MollieError::InvalidRequest`] when any identifier is not
    /// supported.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::PaymentMethod;
    ///
    /// let allowed = PaymentMethod::parse_payment_link_methods(["ideal", "paypal"])?;
    /// assert!(allowed.as_ref().is_some_and(|m| m.len() == 2));
    /// # Ok::<(), mollie_rs::MollieError>(())
    /// ```
    pub fn parse_payment_link_methods<I, S>(methods: I) -> MollieResult<types::PaymentLinkMethods>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut out: Vec<types::PaymentLinkMethod> = Vec::new();
        for item in methods {
            let method = Self::parse(item.as_ref())?;
            out.push(
                method
                    .as_str()
                    .parse::<types::PaymentLinkMethod>()
                    .map_err(|e| MollieError::invalid_request(e.to_string()))?,
            );
        }
        Ok(types::PaymentLinkMethods(Some(out)))
    }
}

impl From<types::MethodInner> for PaymentMethod {
    /// Wraps a generated method enum in the validated facade type.
    fn from(value: types::MethodInner) -> Self {
        Self(value)
    }
}

impl From<PaymentMethod> for types::MethodInner {
    /// Converts the facade method into the generated enum.
    fn from(value: PaymentMethod) -> Self {
        value.into_generated()
    }
}

impl From<PaymentMethod> for types::Method {
    /// Converts into the nullable generated method wrapper for payment bodies.
    fn from(value: PaymentMethod) -> Self {
        value.into_method()
    }
}

impl From<PaymentMethod> for Option<types::Method> {
    /// Wraps the method as `Some(...)` for `PaymentRequest.method` fields.
    fn from(value: PaymentMethod) -> Self {
        Some(value.into_method())
    }
}

impl TryFrom<&str> for PaymentMethod {
    type Error = MollieError;

    /// Parses a supported payment method from a string slice.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl TryFrom<String> for PaymentMethod {
    type Error = MollieError;

    /// Parses a supported payment method from an owned string.
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl FromStr for PaymentMethod {
    type Err = MollieError;

    /// Parses a supported payment method from a string slice.
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl fmt::Display for PaymentMethod {
    /// Formats the method as its lowercase Mollie identifier.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for PaymentMethod {
    /// Returns the method identifier as a string slice.
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::PaymentMethod;
    use crate::types;

    #[test]
    fn parse_accepts_supported_methods_and_rejects_googlepay() {
        assert_eq!(PaymentMethod::parse("ideal").unwrap(), PaymentMethod::IDEAL);
        assert_eq!(
            PaymentMethod::parse("creditcard").unwrap(),
            PaymentMethod::CREDITCARD
        );
        let err = PaymentMethod::parse("googlepay").unwrap_err();
        assert!(err.to_string().contains("googlepay"));
        assert!(!PaymentMethod::is_supported("googlepay"));
    }

    #[test]
    fn billink_is_first_class_supported_method() {
        assert_eq!(PaymentMethod::parse("billink").unwrap(), PaymentMethod::BILLINK);
        assert_eq!(PaymentMethod::BILLINK.as_str(), "billink");
        assert!(PaymentMethod::SUPPORTED.contains(&PaymentMethod::BILLINK));
        let method: types::Method = PaymentMethod::BILLINK.into_method();
        assert_eq!(method.0, Some(types::MethodInner::Billink));
    }

    #[test]
    fn supported_list_matches_parse_round_trip() {
        assert_eq!(PaymentMethod::SUPPORTED.len(), 35);
        for method in PaymentMethod::SUPPORTED {
            let again = PaymentMethod::parse(method.as_str()).expect("round-trip");
            assert_eq!(again, method);
            assert_eq!(again.into_generated(), method.into_generated());
        }
    }

    #[test]
    fn converts_into_generated_method_wrappers() {
        let method: types::Method = PaymentMethod::PAYPAL.into();
        assert_eq!(method.0, Some(types::MethodInner::Paypal));

        let optional: Option<types::Method> = PaymentMethod::IDEAL.into();
        assert_eq!(optional.unwrap().0, Some(types::MethodInner::Ideal));
    }

    #[test]
    fn payment_link_methods_validates_each_entry() {
        let allowed =
            PaymentMethod::payment_link_methods([PaymentMethod::IDEAL, PaymentMethod::BANCONTACT])
                .expect("valid methods");
        assert_eq!(
            allowed.0.as_deref(),
            Some(
                [
                    types::PaymentLinkMethod::Ideal,
                    types::PaymentLinkMethod::Bancontact
                ]
                .as_slice()
            )
        );

        let err = PaymentMethod::parse_payment_link_methods(["ideal", "googlepay"]).unwrap_err();
        assert!(err.to_string().contains("googlepay"));
    }
}
