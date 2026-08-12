//! Local validation for the three required create-payment body fields.
//!
//! Mollie `POST /payments` requires (in the common case):
//!
//! 1. **`description`** — non-empty string, max 255 characters  
//! 2. **`amount`** — `{ currency, value }` (validated via [`Money`])  
//! 3. **`redirectUrl`** — absolute `http`/`https` URL (required unless
//!    `sequenceType` is `recurring` or an Apple Pay payment token is set)
//!
//! Generated [`types::PaymentRequest`] still accepts partial structs; use this
//! module so bad payloads fail before the HTTP call.
#![warn(missing_docs)]

use std::{fmt, str::FromStr};

use serde_json::{json, Value};

use crate::{
    types, CustomerId, Locale, MollieError, MollieResult, Money, PaymentMethod, WebhookUrl,
};

/// Absolute maximum length of a payment description (Mollie API).
pub const PAYMENT_DESCRIPTION_MAX_LEN: usize = 255;

/// A validated payment description for create-payment.
///
/// Shown on the customer's card or bank statement when possible. Must be
/// non-empty and at most [`PAYMENT_DESCRIPTION_MAX_LEN`] characters. Mollie may
/// still truncate further per payment method; this crate enforces the absolute
/// API maximum and rejects empty values locally.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PaymentDescription(String);

impl PaymentDescription {
    /// Maximum character length (Unicode scalar values).
    pub const MAX_LEN: usize = PAYMENT_DESCRIPTION_MAX_LEN;

    /// Parses a payment description.
    ///
    /// # Errors
    ///
    /// Returns [`MollieError::InvalidRequest`] when the value is empty or
    /// longer than 255 characters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::PaymentDescription;
    ///
    /// let description = PaymentDescription::parse("Order #12345")?;
    /// assert_eq!(description.as_str(), "Order #12345");
    /// assert!(PaymentDescription::parse("").is_err());
    /// # Ok::<(), mollie_rs::MollieError>(())
    /// ```
    pub fn parse(value: impl Into<String>) -> MollieResult<Self> {
        let value = value.into();
        if value.is_empty() {
            return Err(MollieError::invalid_request(
                "payment description is required and cannot be empty",
            ));
        }
        let len = value.chars().count();
        if len > Self::MAX_LEN {
            return Err(MollieError::invalid_request(format!(
                "payment description is {len} characters; maximum is {}",
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

    /// Converts into the generated [`types::CreatePaymentRequestDescription`].
    ///
    /// # Panics
    ///
    /// Does not panic: length was already validated against the same 255-char
    /// limit enforced by the generated type's `FromStr`.
    pub fn into_generated(self) -> types::CreatePaymentRequestDescription {
        self.0
            .parse()
            .expect("PaymentDescription length was validated to fit PaymentRequestDescription")
    }
}

impl FromStr for PaymentDescription {
    type Err = MollieError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl fmt::Display for PaymentDescription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for PaymentDescription {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl TryFrom<&str> for PaymentDescription {
    type Error = MollieError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl TryFrom<String> for PaymentDescription {
    type Error = MollieError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<PaymentDescription> for types::CreatePaymentRequestDescription {
    fn from(value: PaymentDescription) -> Self {
        value.into_generated()
    }
}

impl From<PaymentDescription> for Option<types::CreatePaymentRequestDescription> {
    fn from(value: PaymentDescription) -> Self {
        Some(value.into_generated())
    }
}

/// A validated payment redirect URL (`redirectUrl`).
///
/// Must be an absolute `http` or `https` URL. The field is normally required on
/// create-payment; it may be omitted for `sequenceType: recurring` and for
/// Apple Pay with an `applePayPaymentToken` (see
/// [`CreatePaymentRequired::new_recurring`] /
/// [`CreatePaymentRequired::new_with_apple_pay_token`]).
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RedirectUrl(String);

impl RedirectUrl {
    /// Parses a redirect URL.
    ///
    /// # Errors
    ///
    /// Returns [`MollieError::InvalidRequest`] when the value is empty, not
    /// absolute, or uses a scheme other than `http` / `https`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::RedirectUrl;
    ///
    /// let url = RedirectUrl::parse("https://example.com/return")?;
    /// assert_eq!(url.as_str(), "https://example.com/return");
    /// assert!(RedirectUrl::parse("example.com/return").is_err());
    /// # Ok::<(), mollie_rs::MollieError>(())
    /// ```
    pub fn parse(value: impl Into<String>) -> MollieResult<Self> {
        let value = value.into();
        validate_redirect_url(&value)?;
        Ok(Self(value))
    }

    /// Returns the URL string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the value and returns the owned string.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl FromStr for RedirectUrl {
    type Err = MollieError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl fmt::Display for RedirectUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for RedirectUrl {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl TryFrom<&str> for RedirectUrl {
    type Error = MollieError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl TryFrom<String> for RedirectUrl {
    type Error = MollieError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<RedirectUrl> for String {
    fn from(value: RedirectUrl) -> Self {
        value.into_string()
    }
}

impl From<RedirectUrl> for Option<String> {
    fn from(value: RedirectUrl) -> Self {
        Some(value.into_string())
    }
}

fn validate_redirect_url(value: &str) -> MollieResult<()> {
    if value.is_empty() {
        return Err(MollieError::invalid_request(
            "redirectUrl is required and cannot be empty (use https://…)",
        ));
    }
    if value.chars().any(|c| c.is_whitespace() || c.is_control()) {
        return Err(MollieError::invalid_request(format!(
            "invalid redirectUrl `{value}`: whitespace and control characters are not allowed"
        )));
    }
    // Minimal absolute URL check without pulling in a URL crate: scheme + :// + host-ish.
    let Some((scheme, rest)) = value.split_once("://") else {
        return Err(MollieError::invalid_request(format!(
            "invalid redirectUrl `{value}`: must be an absolute http(s) URL (example: `https://example.com/return`)"
        )));
    };
    let scheme = scheme.to_ascii_lowercase();
    if scheme != "https" && scheme != "http" {
        return Err(MollieError::invalid_request(format!(
            "invalid redirectUrl `{value}`: scheme must be `http` or `https`"
        )));
    }
    if rest.is_empty() || rest.starts_with('/') {
        return Err(MollieError::invalid_request(format!(
            "invalid redirectUrl `{value}`: missing host after `{scheme}://`"
        )));
    }
    Ok(())
}

/// The three create-payment fields that are required for a normal (one-off)
/// hosted payment, after local validation.
///
/// Use [`Self::apply`] / [`Self::into_payment_request`] to populate a
/// [`types::PaymentRequest`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreatePaymentRequired {
    /// Payment description (max 255 characters).
    pub description: PaymentDescription,
    /// Charge amount (`currency` + `value`).
    pub amount: Money,
    /// Customer return URL, when required for this payment shape.
    pub redirect_url: Option<RedirectUrl>,
    /// Optional Apple Pay payment token.
    pub apple_pay_payment_token: Option<String>,
    /// Optional webhook URL.
    pub webhook_url: Option<RedirectUrl>,
    /// Optional cancellation URL.
    pub cancel_url: Option<RedirectUrl>,
    /// Optional Mollie payment method.
    pub method: Option<String>,
    /// Optional hosted checkout locale.
    pub locale: Option<String>,
    /// Optional payment sequence type.
    pub sequence_type: Option<String>,
    /// Optional customer id for recurring payments.
    pub customer_id: Option<String>,
    /// Optional provider metadata.
    pub metadata: Option<Value>,
}

impl CreatePaymentRequired {
    /// Validates the three fields for a **standard** payment (redirect required).
    ///
    /// # Errors
    ///
    /// Propagates validation failures from description, amount, or redirect URL.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::{types::CreatePaymentRequest, CreatePaymentRequired, Money};
    ///
    /// let required = CreatePaymentRequired::new(
    ///     "Order #12345",
    ///     Money::new("EUR", "10.00")?,
    ///     "https://example.com/return",
    /// )?;
    /// let body: CreatePaymentRequest = required.into_payment_request();
    /// assert!(body.redirect_url.is_some());
    /// # Ok::<(), mollie_rs::MollieError>(())
    /// ```
    pub fn new(
        description: impl Into<String>,
        amount: Money,
        redirect_url: impl Into<String>,
    ) -> MollieResult<Self> {
        Ok(Self {
            description: PaymentDescription::parse(description)?,
            amount,
            redirect_url: Some(RedirectUrl::parse(redirect_url)?),
            apple_pay_payment_token: None,
            webhook_url: None,
            cancel_url: None,
            method: None,
            locale: None,
            sequence_type: None,
            customer_id: None,
            metadata: None,
        })
    }

    /// Like [`Self::new`], but omits `redirectUrl` for **recurring** payments
    /// (`sequenceType: recurring`).
    ///
    /// Callers must still set `sequence_type` on the full [`types::PaymentRequest`].
    pub fn new_recurring(description: impl Into<String>, amount: Money) -> MollieResult<Self> {
        Ok(Self {
            description: PaymentDescription::parse(description)?,
            amount,
            redirect_url: None,
            apple_pay_payment_token: None,
            webhook_url: None,
            cancel_url: None,
            method: None,
            locale: None,
            sequence_type: Some("recurring".to_string()),
            customer_id: None,
            metadata: None,
        })
    }

    /// Like [`Self::new`], but allows omitting `redirectUrl` when an Apple Pay
    /// payment token is supplied on the full request body.
    ///
    /// # Errors
    ///
    /// Returns [`MollieError::InvalidRequest`] when the Apple Pay token is empty.
    pub fn new_with_apple_pay_token(
        description: impl Into<String>,
        amount: Money,
        apple_pay_payment_token: impl AsRef<str>,
        redirect_url: Option<&str>,
    ) -> MollieResult<Self> {
        if apple_pay_payment_token.as_ref().trim().is_empty() {
            return Err(MollieError::invalid_request(
                "applePayPaymentToken cannot be empty when used to omit redirectUrl",
            ));
        }
        let redirect_url = match redirect_url {
            Some(url) => Some(RedirectUrl::parse(url)?),
            None => None,
        };
        Ok(Self {
            description: PaymentDescription::parse(description)?,
            amount,
            redirect_url,
            apple_pay_payment_token: Some(apple_pay_payment_token.as_ref().to_string()),
            webhook_url: None,
            cancel_url: None,
            method: None,
            locale: None,
            sequence_type: None,
            customer_id: None,
            metadata: None,
        })
    }

    /// Sets and validates the webhook URL.
    pub fn with_webhook_url(mut self, value: impl Into<String>) -> MollieResult<Self> {
        let webhook_url = WebhookUrl::parse(value)?.into_string();
        self.webhook_url = Some(RedirectUrl::parse(webhook_url)?);
        Ok(self)
    }

    /// Sets and validates the cancellation URL.
    pub fn with_cancel_url(mut self, value: impl Into<String>) -> MollieResult<Self> {
        self.cancel_url = Some(RedirectUrl::parse(value)?);
        Ok(self)
    }

    /// Sets and validates a Mollie payment method.
    pub fn with_method(mut self, value: impl AsRef<str>) -> MollieResult<Self> {
        PaymentMethod::parse(value.as_ref())?;
        self.method = Some(value.as_ref().to_string());
        Ok(self)
    }

    /// Sets and validates an ISO 15897 checkout locale.
    pub fn with_locale(mut self, value: impl AsRef<str>) -> MollieResult<Self> {
        Locale::parse(value.as_ref())?.into_generated()?;
        self.locale = Some(value.as_ref().to_string());
        Ok(self)
    }

    /// Sets a supported payment sequence type.
    pub fn with_sequence_type(mut self, value: impl Into<String>) -> MollieResult<Self> {
        let value = value.into();
        if !matches!(value.as_str(), "oneoff" | "first" | "recurring") {
            return Err(MollieError::invalid_request(format!(
                "invalid sequenceType `{value}`"
            )));
        }
        self.sequence_type = Some(value);
        Ok(self)
    }

    /// Sets and validates a Mollie customer id.
    pub fn with_customer_id(mut self, value: impl AsRef<str>) -> MollieResult<Self> {
        self.customer_id = Some(CustomerId::parse(value.as_ref())?.into_string());
        Ok(self)
    }

    /// Sets and validates an Apple Pay payment token.
    pub fn with_apple_pay_payment_token(mut self, value: impl Into<String>) -> MollieResult<Self> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(MollieError::invalid_request(
                "applePayPaymentToken cannot be empty",
            ));
        }
        self.apple_pay_payment_token = Some(value);
        Ok(self)
    }

    /// Sets provider metadata for the payment.
    pub fn with_metadata(mut self, value: Value) -> Self {
        self.metadata = Some(value);
        self
    }

    /// Writes the validated fields onto an existing writable create-payment request.
    pub fn apply(self, request: &mut types::CreatePaymentRequest) {
        request.description = self.description.into_generated();
        request.amount = self.amount.into_amount();
        request.redirect_url = self.redirect_url.map(RedirectUrl::into_string);
        request.apple_pay_payment_token = self.apple_pay_payment_token;
    }

    /// Builds a writable generated create-payment request with the required fields set.
    pub fn into_payment_request(self) -> types::CreatePaymentRequest {
        let value = json!({
            "amount": self.amount.into_amount(),
            "description": self.description.into_generated(),
            "redirectUrl": self.redirect_url.map(RedirectUrl::into_string),
            "applePayPaymentToken": self.apple_pay_payment_token,
        });
        serde_json::from_value(value).expect("validated create-payment fields must deserialize")
    }

    /// Builds the writable generated create-payment request with optional fields.
    pub fn into_create_payment_request(self) -> MollieResult<types::CreatePaymentRequest> {
        let optional = self.clone();
        let mut value = serde_json::to_value(self.into_payment_request())
            .map_err(|error| MollieError::invalid_request(error.to_string()))?;
        let object = value
            .as_object_mut()
            .ok_or_else(|| MollieError::invalid_request("payment request is not an object"))?;
        if let Some(value) = optional.cancel_url.map(RedirectUrl::into_string) {
            object.insert("cancelUrl".to_string(), Value::String(value));
        }
        if let Some(value) = optional.customer_id {
            object.insert("customerId".to_string(), Value::String(value));
        }
        if let Some(value) = optional.locale {
            object.insert("locale".to_string(), Value::String(value));
        }
        if let Some(value) = optional.method {
            object.insert("method".to_string(), Value::String(value));
        }
        if let Some(value) = optional.metadata {
            object.insert("metadata".to_string(), value);
        }
        if let Some(value) = optional.sequence_type {
            object.insert("sequenceType".to_string(), Value::String(value));
        }
        if let Some(value) = optional.webhook_url.map(RedirectUrl::into_string) {
            object.insert("webhookUrl".to_string(), Value::String(value));
        }
        serde_json::from_value(value)
            .map_err(|error| MollieError::invalid_request(error.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::{CreatePaymentRequired, PaymentDescription, RedirectUrl};
    use crate::Money;

    #[test]
    fn description_enforces_non_empty_and_max_len() {
        assert!(PaymentDescription::parse("Order #1").is_ok());
        assert!(PaymentDescription::parse("").is_err());
        let long: String = "x".repeat(256);
        assert!(PaymentDescription::parse(long).is_err());
        let ok: String = "y".repeat(255);
        assert!(PaymentDescription::parse(ok).is_ok());
    }

    #[test]
    fn redirect_url_requires_http_s() {
        assert!(RedirectUrl::parse("https://example.com/return").is_ok());
        assert!(RedirectUrl::parse("http://localhost:3000/ok").is_ok());
        assert!(RedirectUrl::parse("ftp://example.com").is_err());
        assert!(RedirectUrl::parse("example.com").is_err());
        assert!(RedirectUrl::parse("").is_err());
    }

    #[test]
    fn create_payment_required_populates_request() {
        let required = CreatePaymentRequired::new(
            "Order #12345",
            Money::new("EUR", "10.00").unwrap(),
            "https://example.com/return",
        )
        .unwrap();
        let body = required.into_payment_request();
        assert_eq!(body.description.as_str(), "Order #12345");
        assert_eq!(body.amount.currency.as_str(), "EUR");
        assert_eq!(
            body.redirect_url.as_deref(),
            Some("https://example.com/return")
        );
    }

    #[test]
    fn recurring_may_omit_redirect() {
        let required =
            CreatePaymentRequired::new_recurring("Sub", Money::new("EUR", "5.00").unwrap())
                .unwrap();
        assert!(required.redirect_url.is_none());
    }
}
