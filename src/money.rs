//! Typed money helpers for constructing Mollie amount payloads.
//!
//! The generated OpenAPI `types::Amount` remains the wire type. This module
//! provides a small validated layer for application code that wants to check
//! the `currency` and `value` pair before sending a request.
#![warn(missing_docs)]

use std::{fmt, str::FromStr};

use crate::{types, MollieError, MollieResult};

/// A Mollie-supported currency.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Currency(types::Currencies);

impl Currency {
    /// All currencies accepted by the checked-in Mollie OpenAPI spec.
    pub const SUPPORTED: [Self; 13] = [
        Self(types::Currencies::Eur),
        Self(types::Currencies::Gbp),
        Self(types::Currencies::Chf),
        Self(types::Currencies::Dkk),
        Self(types::Currencies::Nok),
        Self(types::Currencies::Pln),
        Self(types::Currencies::Sek),
        Self(types::Currencies::Usd),
        Self(types::Currencies::Czk),
        Self(types::Currencies::Huf),
        Self(types::Currencies::Aud),
        Self(types::Currencies::Cad),
        Self(types::Currencies::Ron),
    ];

    /// Parses a supported currency code.
    ///
    /// # Errors
    ///
    /// Returns [`MollieError::InvalidRequest`] when the code is not supported
    /// by the checked-in Mollie OpenAPI spec.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::Currency;
    ///
    /// let currency = Currency::parse("EUR")?;
    /// assert_eq!(currency.code(), "EUR");
    /// # Ok::<(), mollie_rs::MollieError>(())
    /// ```
    pub fn parse(code: impl AsRef<str>) -> MollieResult<Self> {
        code.as_ref().parse()
    }

    /// Returns the ISO 4217 currency code.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::Currency;
    ///
    /// assert_eq!(Currency::parse("USD")?.code(), "USD");
    /// # Ok::<(), mollie_rs::MollieError>(())
    /// ```
    pub fn code(self) -> &'static str {
        match self.0 {
            types::Currencies::Eur => "EUR",
            types::Currencies::Gbp => "GBP",
            types::Currencies::Chf => "CHF",
            types::Currencies::Dkk => "DKK",
            types::Currencies::Nok => "NOK",
            types::Currencies::Pln => "PLN",
            types::Currencies::Sek => "SEK",
            types::Currencies::Usd => "USD",
            types::Currencies::Czk => "CZK",
            types::Currencies::Huf => "HUF",
            types::Currencies::Aud => "AUD",
            types::Currencies::Cad => "CAD",
            types::Currencies::Ron => "RON",
        }
    }

    /// Returns the number of fractional digits accepted for amount values.
    ///
    /// Values are taken from Mollie's wire format for currencies listed in the
    /// pinned OpenAPI `Currencies` enum. Every currency currently in that set
    /// uses **two** decimal places on the wire (including `HUF`). This table is
    /// the single source of truth so future zero-decimal currencies can be
    /// added without guessing.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::Currency;
    ///
    /// assert_eq!(Currency::parse("EUR")?.minor_units(), 2);
    /// assert_eq!(Currency::parse("HUF")?.minor_units(), 2);
    /// # Ok::<(), mollie_rs::MollieError>(())
    /// ```
    pub const fn minor_units(self) -> u8 {
        match self.0 {
            // All currencies in the pinned Mollie OpenAPI set use 2 places.
            types::Currencies::Eur
            | types::Currencies::Gbp
            | types::Currencies::Chf
            | types::Currencies::Dkk
            | types::Currencies::Nok
            | types::Currencies::Pln
            | types::Currencies::Sek
            | types::Currencies::Usd
            | types::Currencies::Czk
            | types::Currencies::Huf
            | types::Currencies::Aud
            | types::Currencies::Cad
            | types::Currencies::Ron => 2,
        }
    }

    /// Returns the generated OpenAPI currency enum.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::{types, Currency};
    ///
    /// let generated: types::Currencies = Currency::parse("EUR")?.into_generated();
    /// assert_eq!(generated.to_string(), "EUR");
    /// # Ok::<(), mollie_rs::MollieError>(())
    /// ```
    pub const fn into_generated(self) -> types::Currencies {
        self.0
    }

    /// Returns true when a currency code is supported by the checked-in spec.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::Currency;
    ///
    /// assert!(Currency::is_supported("EUR"));
    /// assert!(!Currency::is_supported("ISK"));
    /// ```
    pub fn is_supported(code: impl AsRef<str>) -> bool {
        Self::parse(code).is_ok()
    }
}

impl From<types::Currencies> for Currency {
    /// Wraps a generated currency enum in the validated facade type.
    fn from(value: types::Currencies) -> Self {
        Self(value)
    }
}

impl TryFrom<&str> for Currency {
    type Error = MollieError;

    /// Parses a supported currency code from a string slice.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl TryFrom<String> for Currency {
    type Error = MollieError;

    /// Parses a supported currency code from an owned string.
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<Currency> for types::Currencies {
    /// Converts the facade currency into the generated currency enum.
    fn from(value: Currency) -> Self {
        value.into_generated()
    }
}

impl FromStr for Currency {
    type Err = MollieError;

    /// Parses a supported currency code from a string slice.
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let parsed = types::Currencies::from_str(value).map_err(|_| {
            MollieError::invalid_request(format!("unsupported Mollie currency `{value}`"))
        })?;
        Ok(Self(parsed))
    }
}

impl fmt::Display for Currency {
    /// Formats the currency as its ISO 4217 code.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.code())
    }
}

/// A string amount value validated for a specific currency.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AmountValue(String);

impl AmountValue {
    /// Parses an amount value for the provided currency.
    ///
    /// The value must use a plain decimal representation, contain at least one
    /// digit before the decimal point, and contain exactly the number of
    /// fractional digits required by the currency.
    ///
    /// # Errors
    ///
    /// Returns [`MollieError::InvalidRequest`] when the value is empty,
    /// signed, missing a decimal separator, has leading zeroes, contains
    /// non-digits, or does not match the currency's minor-unit scale.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::{AmountValue, Currency};
    ///
    /// let value = AmountValue::parse(Currency::parse("EUR")?, "10.00")?;
    /// assert_eq!(value.as_str(), "10.00");
    /// # Ok::<(), mollie_rs::MollieError>(())
    /// ```
    pub fn parse(currency: Currency, value: impl Into<String>) -> MollieResult<Self> {
        let value = value.into();
        validate_amount_value(currency, &value)?;
        Ok(Self(value))
    }

    /// Returns the validated amount string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::{AmountValue, Currency};
    ///
    /// let value = AmountValue::parse(Currency::parse("EUR")?, "10.00")?;
    /// assert_eq!(value.as_str(), "10.00");
    /// # Ok::<(), mollie_rs::MollieError>(())
    /// ```
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the value and returns the owned string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::{AmountValue, Currency};
    ///
    /// let value = AmountValue::parse(Currency::parse("EUR")?, "10.00")?;
    /// assert_eq!(value.into_string(), "10.00");
    /// # Ok::<(), mollie_rs::MollieError>(())
    /// ```
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for AmountValue {
    /// Returns the amount value as a string slice.
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for AmountValue {
    /// Formats the amount value as the validated decimal string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A validated Mollie amount.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Money {
    currency: Currency,
    value: AmountValue,
}

impl Money {
    /// Creates a validated currency/value pair.
    ///
    /// # Errors
    ///
    /// Returns [`MollieError::InvalidRequest`] when the currency is not
    /// supported or the amount value is not valid for that currency.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::{types, Money};
    ///
    /// let amount: types::Amount = Money::new("EUR", "10.00")?.into();
    /// assert_eq!(amount.currency, "EUR");
    /// assert_eq!(amount.value, "10.00");
    /// # Ok::<(), mollie_rs::MollieError>(())
    /// ```
    pub fn new(currency: impl TryInto<Currency>, value: impl Into<String>) -> MollieResult<Self> {
        let currency = currency
            .try_into()
            .map_err(|_| MollieError::invalid_request("invalid Mollie currency"))?;
        let value = AmountValue::parse(currency, value)?;
        Ok(Self { currency, value })
    }

    /// Returns the validated currency.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::Money;
    ///
    /// let money = Money::new("EUR", "10.00")?;
    /// assert_eq!(money.currency().code(), "EUR");
    /// # Ok::<(), mollie_rs::MollieError>(())
    /// ```
    pub const fn currency(&self) -> Currency {
        self.currency
    }

    /// Returns the validated amount value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::Money;
    ///
    /// let money = Money::new("EUR", "10.00")?;
    /// assert_eq!(money.value().as_str(), "10.00");
    /// # Ok::<(), mollie_rs::MollieError>(())
    /// ```
    pub const fn value(&self) -> &AmountValue {
        &self.value
    }

    /// Converts into the generated OpenAPI amount type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::{types, Money};
    ///
    /// let amount: types::Amount = Money::new("EUR", "10.00")?.into_amount();
    /// assert_eq!(amount.currency, "EUR");
    /// # Ok::<(), mollie_rs::MollieError>(())
    /// ```
    pub fn into_amount(self) -> types::Amount {
        self.into()
    }
}

impl TryFrom<(Currency, &str)> for Money {
    type Error = MollieError;

    /// Builds a validated amount from a currency and decimal value.
    fn try_from(value: (Currency, &str)) -> Result<Self, Self::Error> {
        Self::new(value.0, value.1)
    }
}

impl TryFrom<(&str, &str)> for Money {
    type Error = MollieError;

    /// Builds a validated amount from currency and value string slices.
    fn try_from(value: (&str, &str)) -> Result<Self, Self::Error> {
        Self::new(Currency::parse(value.0)?, value.1)
    }
}

impl From<Money> for types::Amount {
    /// Converts the facade amount into the generated non-null amount type.
    ///
    /// Use this for payment, payment-link, refund, capture, balance, settlement,
    /// and application-fee amount fields that take [`types::Amount`].
    fn from(value: Money) -> Self {
        Self {
            currency: value.currency.code().to_string(),
            value: value.value.into_string(),
        }
    }
}

impl From<Money> for types::AmountNullableInner {
    /// Converts the facade amount into the generated nullable inner amount.
    fn from(value: Money) -> Self {
        Self {
            currency: value.currency.code().to_string(),
            value: value.value.into_string(),
        }
    }
}

impl From<Option<Money>> for types::AmountNullable {
    /// Converts an optional facade amount into the generated nullable amount.
    ///
    /// Payment-link and similar bodies use [`types::AmountNullable`] for optional
    /// amount fields.
    fn from(value: Option<Money>) -> Self {
        Self(value.map(types::AmountNullableInner::from))
    }
}

impl From<Money> for Option<types::Amount> {
    /// Wraps a validated amount as `Some(types::Amount)`.
    fn from(value: Money) -> Self {
        Some(value.into())
    }
}

impl TryFrom<types::Amount> for Money {
    type Error = MollieError;

    /// Re-validates a generated amount (for example balance or settlement
    /// amounts from API responses) against the supported currency set and
    /// minor-unit scale.
    fn try_from(value: types::Amount) -> Result<Self, Self::Error> {
        Self::new(value.currency, value.value)
    }
}

impl TryFrom<&types::Amount> for Money {
    type Error = MollieError;

    /// Re-validates a generated amount by reference.
    fn try_from(value: &types::Amount) -> Result<Self, Self::Error> {
        Self::new(value.currency.as_str(), value.value.as_str())
    }
}

impl TryFrom<types::AmountNullableInner> for Money {
    type Error = MollieError;

    /// Re-validates a generated nullable-inner amount.
    fn try_from(value: types::AmountNullableInner) -> Result<Self, Self::Error> {
        Self::new(value.currency, value.value)
    }
}

impl TryFrom<&types::AmountNullableInner> for Money {
    type Error = MollieError;

    /// Re-validates a generated nullable-inner amount by reference.
    fn try_from(value: &types::AmountNullableInner) -> Result<Self, Self::Error> {
        Self::new(value.currency.as_str(), value.value.as_str())
    }
}

impl TryFrom<types::AmountNullable> for Option<Money> {
    type Error = MollieError;

    /// Re-validates a nullable amount envelope; `None` stays `None`.
    fn try_from(value: types::AmountNullable) -> Result<Self, Self::Error> {
        value.0.map(Money::try_from).transpose()
    }
}

/// Maximum length of an application-fee description (Mollie API).
pub const APPLICATION_FEE_DESCRIPTION_MAX_LEN: usize = 255;

/// A validated application-fee description (non-empty, max 255 characters).
///
/// Appears on settlement reports for both the platform and the connected
/// merchant when using Mollie Connect `applicationFee`.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ApplicationFeeDescription(String);

impl ApplicationFeeDescription {
    /// Absolute maximum character length (Unicode scalar values).
    pub const MAX_LEN: usize = APPLICATION_FEE_DESCRIPTION_MAX_LEN;

    /// Parses an application-fee description.
    ///
    /// # Errors
    ///
    /// Returns [`MollieError::InvalidRequest`] when the value is empty or
    /// longer than 255 characters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::ApplicationFeeDescription;
    ///
    /// let description = ApplicationFeeDescription::parse("Platform fee")?;
    /// assert_eq!(description.as_str(), "Platform fee");
    /// # Ok::<(), mollie_rs::MollieError>(())
    /// ```
    pub fn parse(value: impl Into<String>) -> MollieResult<Self> {
        let value = value.into();
        if value.is_empty() {
            return Err(MollieError::invalid_request(
                "application fee description is required and cannot be empty",
            ));
        }
        let len = value.chars().count();
        if len > Self::MAX_LEN {
            return Err(MollieError::invalid_request(format!(
                "application fee description is {len} characters; maximum is {}",
                Self::MAX_LEN
            )));
        }
        Ok(Self(value))
    }

    /// Returns the description string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the value and returns the owned string.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl FromStr for ApplicationFeeDescription {
    type Err = MollieError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl fmt::Display for ApplicationFeeDescription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for ApplicationFeeDescription {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl TryFrom<&str> for ApplicationFeeDescription {
    type Error = MollieError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl TryFrom<String> for ApplicationFeeDescription {
    type Error = MollieError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

/// A validated Mollie Connect application fee (`amount` + `description`).
///
/// Use this for payment, payment-link, and subscription `applicationFee`
/// fields so currency/value and description length are checked before the
/// HTTP call.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ApplicationFee {
    amount: Money,
    description: ApplicationFeeDescription,
}

impl ApplicationFee {
    /// Creates a validated application fee.
    ///
    /// # Errors
    ///
    /// Returns [`MollieError::InvalidRequest`] when the description is empty or
    /// too long. Amount validation is performed when constructing [`Money`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::{types, ApplicationFee, Money};
    ///
    /// let fee = ApplicationFee::new(Money::new("EUR", "1.00")?, "Platform fee")?;
    /// let wire: types::CreatePaymentRequestApplicationFee = fee.into();
    /// assert_eq!(wire.amount.as_ref().map(|a| a.value.as_str()), Some("1.00"));
    /// # Ok::<(), mollie_rs::MollieError>(())
    /// ```
    pub fn new(amount: Money, description: impl Into<String>) -> MollieResult<Self> {
        Ok(Self {
            amount,
            description: ApplicationFeeDescription::parse(description)?,
        })
    }

    /// Builds from currency code, value, and description strings.
    ///
    /// # Errors
    ///
    /// Propagates failures from [`Money::new`] and
    /// [`ApplicationFeeDescription::parse`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::ApplicationFee;
    ///
    /// let fee = ApplicationFee::parse("EUR", "0.50", "Connect fee")?;
    /// assert_eq!(fee.amount().currency().code(), "EUR");
    /// assert_eq!(fee.description().as_str(), "Connect fee");
    /// # Ok::<(), mollie_rs::MollieError>(())
    /// ```
    pub fn parse(
        currency: impl TryInto<Currency>,
        value: impl Into<String>,
        description: impl Into<String>,
    ) -> MollieResult<Self> {
        Self::new(Money::new(currency, value)?, description)
    }

    /// Returns the validated fee amount.
    pub const fn amount(&self) -> &Money {
        &self.amount
    }

    /// Returns the validated fee description.
    pub const fn description(&self) -> &ApplicationFeeDescription {
        &self.description
    }

    /// Converts into the payment-request application-fee body field.
    pub fn into_payment_request_fee(self) -> types::CreatePaymentRequestApplicationFee {
        self.into()
    }

    /// Converts into the create-payment-link application-fee body field.
    pub fn into_payment_link_fee(self) -> types::CreatePaymentLinkBodyApplicationFee {
        self.into()
    }

    /// Converts into the subscription-request application-fee body field.
    pub fn into_subscription_request_fee(self) -> types::CreateSubscriptionRequestApplicationFee {
        self.into()
    }
}

impl From<ApplicationFee> for types::CreatePaymentRequestApplicationFee {
    fn from(value: ApplicationFee) -> Self {
        Self {
            amount: Some(value.amount.into_amount()),
            description: Some(
                value
                    .description
                    .as_str()
                    .parse()
                    .expect("ApplicationFeeDescription length was validated to fit generated max"),
            ),
        }
    }
}

impl From<ApplicationFee> for types::EntityPaymentApplicationFee {
    fn from(value: ApplicationFee) -> Self {
        Self {
            amount: Some(value.amount.into_amount()),
            description: Some(
                value
                    .description
                    .as_str()
                    .parse()
                    .expect("ApplicationFeeDescription length was validated to fit generated max"),
            ),
        }
    }
}

impl From<ApplicationFee> for types::CreatePaymentLinkBodyApplicationFee {
    fn from(value: ApplicationFee) -> Self {
        Self {
            amount: value.amount.into_amount(),
            description: value
                .description
                .as_str()
                .parse()
                .expect("ApplicationFeeDescription length was validated to fit generated max"),
        }
    }
}

impl From<ApplicationFee> for types::EntityPaymentLinkApplicationFee {
    fn from(value: ApplicationFee) -> Self {
        Self {
            amount: value.amount.into_amount(),
            description: value
                .description
                .as_str()
                .parse()
                .expect("ApplicationFeeDescription length was validated to fit generated max"),
        }
    }
}

impl From<ApplicationFee> for types::CreateSubscriptionRequestApplicationFee {
    fn from(value: ApplicationFee) -> Self {
        Self {
            amount: value.amount.into_amount(),
            description: value.description.into_string(),
        }
    }
}

impl From<ApplicationFee> for types::EntitySubscriptionApplicationFee {
    fn from(value: ApplicationFee) -> Self {
        Self {
            amount: value.amount.into_amount(),
            description: value.description.into_string(),
        }
    }
}

impl From<ApplicationFee> for types::EntityPaymentResponseApplicationFee {
    fn from(value: ApplicationFee) -> Self {
        Self {
            amount: Some(value.amount.into_amount()),
            description: Some(
                value
                    .description
                    .as_str()
                    .parse()
                    .expect("ApplicationFeeDescription length was validated to fit generated max"),
            ),
        }
    }
}

/// Validates the decimal value for the selected Mollie currency.
fn validate_amount_value(currency: Currency, value: &str) -> MollieResult<()> {
    if value.is_empty() {
        return Err(MollieError::invalid_request(
            "Mollie amount value cannot be empty",
        ));
    }

    if value.starts_with('+') || value.starts_with('-') {
        return Err(MollieError::invalid_request(
            "Mollie amount value must not include a sign",
        ));
    }

    let Some((major, minor)) = value.split_once('.') else {
        return Err(MollieError::invalid_request(format!(
            "Mollie amount value for {} must contain {} decimal places",
            currency,
            currency.minor_units()
        )));
    };

    if major.is_empty() || !major.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(MollieError::invalid_request(
            "Mollie amount value must contain digits before the decimal point",
        ));
    }

    if major.len() > 1 && major.starts_with('0') {
        return Err(MollieError::invalid_request(
            "Mollie amount value must not contain leading zeroes",
        ));
    }

    let expected_minor_units = usize::from(currency.minor_units());
    if minor.len() != expected_minor_units || !minor.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(MollieError::invalid_request(format!(
            "Mollie amount value for {currency} must contain exactly {expected_minor_units} decimal places"
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    mod currency {
        use super::*;

        #[test]
        fn parse_accepts_supported_spec_currency() {
            let currency = Currency::parse("EUR").expect("currency should parse");

            assert_eq!(currency.code(), "EUR");
            assert_eq!(currency.minor_units(), 2);
        }

        #[test]
        fn all_pinned_currencies_use_two_minor_units_on_wire() {
            // Mollie OpenAPI currently lists these with two decimal places,
            // including HUF. Do not invent zero-decimal rules without a pin change.
            for code in Currency::SUPPORTED.map(Currency::code) {
                let c = Currency::parse(code).expect("supported");
                assert_eq!(c.minor_units(), 2, "{code}");
            }
        }

        #[test]
        fn parse_rejects_unknown_currency() {
            let error = Currency::parse("ISK").unwrap_err();

            assert!(matches!(error, MollieError::InvalidRequest(_)));
        }

        #[test]
        fn supported_contains_all_spec_currencies() {
            let codes = Currency::SUPPORTED.map(Currency::code);

            assert_eq!(
                codes,
                [
                    "EUR", "GBP", "CHF", "DKK", "NOK", "PLN", "SEK", "USD", "CZK", "HUF", "AUD",
                    "CAD", "RON",
                ],
            );
        }
    }

    mod amount_value {
        use super::*;

        #[test]
        fn parse_accepts_exact_minor_units() {
            let value: AmountValue = AmountValue::parse(Currency::parse("USD").unwrap(), "10.00")
                .expect("value should parse");

            assert_eq!(value.as_str(), "10.00");
        }

        #[test]
        fn parse_rejects_too_few_minor_units() {
            let error: MollieError =
                AmountValue::parse(Currency::parse("EUR").unwrap(), "10.0").unwrap_err();

            assert!(matches!(error, MollieError::InvalidRequest(_)));
        }

        #[test]
        fn parse_rejects_too_many_minor_units() {
            let error: MollieError =
                AmountValue::parse(Currency::parse("EUR").unwrap(), "10.000").unwrap_err();

            assert!(matches!(error, MollieError::InvalidRequest(_)));
        }

        #[test]
        fn parse_rejects_signs() {
            let error: MollieError =
                AmountValue::parse(Currency::parse("EUR").unwrap(), "-10.00").unwrap_err();

            assert!(matches!(error, MollieError::InvalidRequest(_)));
        }
    }

    mod money {
        use super::*;

        #[test]
        fn new_converts_to_generated_amount() {
            let amount: types::Amount = Money::new(Currency::parse("EUR").unwrap(), "10.00")
                .expect("money should parse")
                .into();

            assert_eq!(amount.currency, "EUR");
            assert_eq!(amount.value, "10.00");
        }

        #[test]
        fn tuple_try_from_accepts_currency_code_and_value() {
            let amount = Money::try_from(("CAD", "25.50")).expect("money should parse");

            assert_eq!(amount.currency().code(), "CAD");
            assert_eq!(amount.value().as_str(), "25.50");
        }

        #[test]
        fn try_from_amount_revalidates_wire_amounts() {
            let wire = types::Amount {
                currency: "GBP".to_owned(),
                value: "3.50".to_owned(),
            };
            let money = Money::try_from(wire).expect("amount should revalidate");
            assert_eq!(money.currency().code(), "GBP");
            assert_eq!(money.value().as_str(), "3.50");
        }

        #[test]
        fn try_from_amount_rejects_unsupported_currency() {
            let wire = types::Amount {
                currency: "ISK".to_owned(),
                value: "10.00".to_owned(),
            };
            assert!(matches!(
                Money::try_from(wire),
                Err(MollieError::InvalidRequest(_))
            ));
        }

        #[test]
        fn optional_amount_nullable_round_trip() {
            let money = Money::new("USD", "1.25").unwrap();
            let nullable: types::AmountNullable = Some(money.clone()).into();
            let back: Option<Money> = nullable.try_into().expect("nullable should parse");
            assert_eq!(back.unwrap().value().as_str(), "1.25");
        }
    }

    mod application_fee {
        use super::*;

        #[test]
        fn new_converts_to_payment_request_fee() {
            let fee = ApplicationFee::new(Money::new("EUR", "1.00").unwrap(), "Platform fee")
                .expect("fee should parse");
            let wire: types::CreatePaymentRequestApplicationFee = fee.into();
            assert_eq!(
                wire.amount.as_ref().map(|a| a.currency.as_str()),
                Some("EUR")
            );
            assert_eq!(wire.amount.as_ref().map(|a| a.value.as_str()), Some("1.00"));
            assert_eq!(
                wire.description.as_ref().map(|d| d.as_str()),
                Some("Platform fee")
            );
        }

        #[test]
        fn parse_converts_to_payment_link_and_subscription_fees() {
            let fee = ApplicationFee::parse("CHF", "2.50", "Connect fee").unwrap();

            let link: types::CreatePaymentLinkBodyApplicationFee = fee.clone().into();
            assert_eq!(link.amount.currency, "CHF");
            assert_eq!(link.amount.value, "2.50");
            assert_eq!(link.description.as_str(), "Connect fee");

            let sub: types::CreateSubscriptionRequestApplicationFee = fee.into();
            assert_eq!(sub.amount.currency, "CHF");
            assert_eq!(sub.description, "Connect fee");
        }

        #[test]
        fn rejects_empty_description() {
            let error = ApplicationFee::new(Money::new("EUR", "1.00").unwrap(), "").unwrap_err();
            assert!(matches!(error, MollieError::InvalidRequest(_)));
        }
    }
}
