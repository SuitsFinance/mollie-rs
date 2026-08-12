//! Typed helpers for classic Mollie webhook callbacks.
//!
//! Classic Mollie callbacks contain only the updated resource ID. The receiver
//! must acknowledge the callback and refetch the resource through the
//! authenticated API client before deciding whether state changed.
#![warn(missing_docs)]

use std::{fmt, net::IpAddr, str::FromStr};

use serde::Deserialize;

use crate::{MollieError, MollieResult};

/// A validated URL that Mollie can use as a webhook destination.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WebhookUrl(String);

impl WebhookUrl {
    /// Parses an absolute HTTP(S) webhook URL.
    ///
    /// Localhost and loopback destinations are rejected because Mollie cannot
    /// reach them. Public reachability and endpoint behavior must still be
    /// verified by the application or deployment environment.
    pub fn parse(value: impl Into<String>) -> MollieResult<Self> {
        let value = value.into();
        if value
            .chars()
            .any(|character| character.is_whitespace() || character.is_control())
        {
            return Err(MollieError::invalid_request(
                "invalid webhookUrl: whitespace and control characters are not allowed",
            ));
        }

        let url = reqwest::Url::parse(&value).map_err(|error| {
            MollieError::invalid_request(format!("invalid webhookUrl `{value}`: {error}"))
        })?;
        if url.scheme() != "http" && url.scheme() != "https" {
            return Err(MollieError::invalid_request(
                "invalid webhookUrl: scheme must be `http` or `https`",
            ));
        }

        let host = url.host_str().ok_or_else(|| {
            MollieError::invalid_request("invalid webhookUrl: an absolute host is required")
        })?;
        let normalized_host = host.trim_end_matches('.');
        let is_loopback = IpAddr::from_str(normalized_host)
            .map(|address| address.is_loopback())
            .unwrap_or(false);
        if normalized_host.eq_ignore_ascii_case("localhost") || is_loopback {
            return Err(MollieError::invalid_request(
                "invalid webhookUrl: localhost and loopback destinations are not reachable by Mollie",
            ));
        }

        Ok(Self(value))
    }

    /// Returns the validated webhook URL as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the URL and returns its owned string.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for WebhookUrl {
    /// Formats the validated URL.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl AsRef<str> for WebhookUrl {
    /// Returns the URL as a string reference.
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl FromStr for WebhookUrl {
    type Err = MollieError;

    /// Parses a webhook URL from a string.
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl TryFrom<&str> for WebhookUrl {
    type Error = MollieError;

    /// Parses a webhook URL from a string slice.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl TryFrom<String> for WebhookUrl {
    type Error = MollieError;

    /// Parses a webhook URL from an owned string.
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

/// The resource identifier delivered by a classic Mollie webhook callback.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WebhookNotification {
    id: String,
}

impl WebhookNotification {
    /// Creates a notification from a non-empty resource ID.
    pub fn new(id: impl Into<String>) -> MollieResult<Self> {
        let id = id.into();
        if id.trim().is_empty() {
            return Err(MollieError::invalid_request(
                "webhook notification id cannot be empty",
            ));
        }
        Ok(Self { id })
    }

    /// Parses Mollie’s classic `id=<resource-id>` form-encoded callback body.
    pub fn parse_form_urlencoded(body: impl AsRef<[u8]>) -> MollieResult<Self> {
        #[derive(Deserialize)]
        struct Payload {
            id: Option<String>,
        }

        let payload: Payload = serde_urlencoded::from_bytes(body.as_ref()).map_err(|error| {
            MollieError::invalid_request(format!("invalid Mollie webhook body: {error}"))
        })?;
        Self::new(payload.id.unwrap_or_default())
    }

    /// Returns the resource ID that must be refetched from Mollie.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Consumes the notification and returns its resource ID.
    pub fn into_id(self) -> String {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::{WebhookNotification, WebhookUrl};

    /// Parses the classic form body used by Mollie callbacks.
    #[test]
    fn parses_classic_notification() {
        let notification =
            WebhookNotification::parse_form_urlencoded("id=tr_d0b0E3EA3v").expect("valid body");

        assert_eq!(notification.id(), "tr_d0b0E3EA3v");
    }

    /// Rejects callbacks without a resource ID.
    #[test]
    fn rejects_missing_notification_id() {
        assert!(WebhookNotification::parse_form_urlencoded("foo=bar").is_err());
    }

    /// Rejects destinations that Mollie cannot reach.
    #[test]
    fn rejects_loopback_webhook_urls() {
        assert!(WebhookUrl::parse("http://localhost/webhook").is_err());
        assert!(WebhookUrl::parse("http://127.0.0.1/webhook").is_err());
        assert_eq!(
            WebhookUrl::parse("https://example.com/webhook")
                .expect("public URL")
                .as_str(),
            "https://example.com/webhook"
        );
    }
}
