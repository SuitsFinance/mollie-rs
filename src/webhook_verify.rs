//! Next-generation Mollie webhook signature verification.
//!
//! Mollie Next-gen webhooks send `X-Mollie-Signature`: HMAC-SHA256 of the **raw
//! request body**, hex-encoded. Verification must use the exact bytes received
//! — never reserialize parsed JSON.
//!
//! Classic form `id=` callbacks remain in [`crate::webhook`] and do **not**
//! carry this signature; applications must still refetch the resource.
//!
//! Spec reference: <https://docs.mollie.com/reference/webhooks-best-practices>
#![warn(missing_docs)]

use std::fmt;
use std::time::Duration;

use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::{MollieError, MollieResult};

type HmacSha256 = Hmac<Sha256>;

/// Default maximum webhook body size (1 MiB).
pub const DEFAULT_MAX_WEBHOOK_BODY_BYTES: usize = 1024 * 1024;

/// HTTP header carrying the Mollie Next-gen webhook HMAC.
pub const MOLLIE_SIGNATURE_HEADER: &str = "x-mollie-signature";

/// Shared secret used to verify Next-gen webhook signatures.
///
/// Secrets are never printed by [`Debug`]. With the `zeroize` feature, secret
/// buffers are wiped on drop (same policy as [`crate::ApiKey`] / credentials).
#[derive(Clone, Eq, PartialEq)]
pub struct WebhookSigningSecret(String);

impl WebhookSigningSecret {
    /// Creates a signing secret from a non-empty string.
    pub fn new(secret: impl Into<String>) -> MollieResult<Self> {
        let secret = secret.into();
        if secret.is_empty() {
            return Err(MollieError::invalid_configuration(
                "webhook signing secret cannot be empty",
            ));
        }
        Ok(Self(secret))
    }

    /// Returns the raw secret bytes.
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl fmt::Debug for WebhookSigningSecret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("WebhookSigningSecret(<redacted>)")
    }
}

#[cfg(feature = "zeroize")]
impl Drop for WebhookSigningSecret {
    fn drop(&mut self) {
        use zeroize::Zeroize;
        self.0.zeroize();
    }
}

/// Verifies Mollie Next-gen webhook signatures over raw request bodies.
#[derive(Clone, Debug)]
pub struct WebhookVerifier {
    primary: WebhookSigningSecret,
    previous: Option<WebhookSigningSecret>,
    max_body_bytes: usize,
    /// Optional max skew for application-provided event timestamps (not required by Mollie header).
    max_skew: Option<Duration>,
}

impl WebhookVerifier {
    /// Creates a verifier with a single active secret.
    pub fn new(secret: impl Into<String>) -> MollieResult<Self> {
        Ok(Self {
            primary: WebhookSigningSecret::new(secret)?,
            previous: None,
            max_body_bytes: DEFAULT_MAX_WEBHOOK_BODY_BYTES,
            max_skew: None,
        })
    }

    /// Accepts a previous secret during rotation (Mollie keeps old keys briefly).
    pub fn with_previous_secret(mut self, secret: impl Into<String>) -> MollieResult<Self> {
        self.previous = Some(WebhookSigningSecret::new(secret)?);
        Ok(self)
    }

    /// Sets the maximum accepted raw body size.
    pub fn with_max_body_bytes(mut self, max: usize) -> Self {
        self.max_body_bytes = max;
        self
    }

    /// Optional application-level timestamp skew check (not part of Mollie header).
    ///
    /// Callers that extract an event timestamp from the JSON body after
    /// signature verification can use [`Self::check_timestamp_skew`].
    pub fn with_max_skew(mut self, max_skew: Duration) -> Self {
        self.max_skew = Some(max_skew);
        self
    }

    /// Verifies an optional signature header value against the raw body.
    ///
    /// Distinguishes missing vs malformed signatures. Prefer this when reading
    /// headers from an HTTP framework.
    pub fn verify_header(
        &self,
        raw_body: &[u8],
        signature_header: Option<&str>,
    ) -> MollieResult<()> {
        match signature_header.map(str::trim).filter(|s| !s.is_empty()) {
            None => Err(MollieError::webhook_verification(
                WebhookVerifyFailure::MissingSignature,
            )),
            Some(signature) => self.verify(raw_body, signature),
        }
    }

    /// Verifies `X-Mollie-Signature` against the raw body.
    ///
    /// `signature` may include an optional `sha256=` prefix; comparison is
    /// constant-time on the hex digest. Empty strings are treated as
    /// [`WebhookVerifyFailure::MissingSignature`].
    pub fn verify(&self, raw_body: &[u8], signature: &str) -> MollieResult<()> {
        if signature.trim().is_empty() {
            return Err(MollieError::webhook_verification(
                WebhookVerifyFailure::MissingSignature,
            ));
        }
        if raw_body.is_empty() {
            return Err(MollieError::webhook_verification(
                WebhookVerifyFailure::EmptyBody,
            ));
        }
        if raw_body.len() > self.max_body_bytes {
            return Err(MollieError::webhook_verification(
                WebhookVerifyFailure::BodyTooLarge {
                    size: raw_body.len(),
                    max: self.max_body_bytes,
                },
            ));
        }
        let provided = normalize_signature(signature).ok_or_else(|| {
            MollieError::webhook_verification(WebhookVerifyFailure::MalformedSignature)
        })?;
        if signature_matches(self.primary.as_bytes(), raw_body, &provided) {
            return Ok(());
        }
        if let Some(ref previous) = self.previous {
            if signature_matches(previous.as_bytes(), raw_body, &provided) {
                return Ok(());
            }
        }
        Err(MollieError::webhook_verification(
            WebhookVerifyFailure::SignatureMismatch,
        ))
    }

    /// Verifies the signature then deserializes the body as JSON.
    pub fn verify_and_decode<T: serde::de::DeserializeOwned>(
        &self,
        raw_body: &[u8],
        signature: &str,
    ) -> MollieResult<T> {
        self.verify(raw_body, signature)?;
        serde_json::from_slice(raw_body).map_err(|error| {
            MollieError::webhook_verification(WebhookVerifyFailure::InvalidJson {
                message: error.to_string(),
            })
        })
    }

    /// Optional skew check for an event timestamp (seconds since Unix epoch).
    pub fn check_timestamp_skew(
        &self,
        event_unix_secs: i64,
        now_unix_secs: i64,
    ) -> MollieResult<()> {
        let Some(max_skew) = self.max_skew else {
            return Ok(());
        };
        let skew = (now_unix_secs - event_unix_secs).unsigned_abs();
        if skew > max_skew.as_secs() {
            return Err(MollieError::webhook_verification(
                WebhookVerifyFailure::TimestampOutOfWindow {
                    skew_secs: skew,
                    max_secs: max_skew.as_secs(),
                },
            ));
        }
        Ok(())
    }
}

/// Structured webhook verification failures.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WebhookVerifyFailure {
    /// Missing or empty signature header.
    MissingSignature,
    /// Signature string was not valid hex (or empty after normalization).
    MalformedSignature,
    /// HMAC did not match primary or previous secret.
    SignatureMismatch,
    /// Raw body was empty.
    EmptyBody,
    /// Body exceeded configured max size.
    BodyTooLarge {
        /// Actual size.
        size: usize,
        /// Configured max.
        max: usize,
    },
    /// Signature verified but JSON decode failed.
    InvalidJson {
        /// Serde error message.
        message: String,
    },
    /// Application timestamp skew check failed.
    TimestampOutOfWindow {
        /// Observed absolute skew in seconds.
        skew_secs: u64,
        /// Allowed maximum skew.
        max_secs: u64,
    },
}

impl fmt::Display for WebhookVerifyFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingSignature => f.write_str("missing X-Mollie-Signature"),
            Self::MalformedSignature => f.write_str("malformed webhook signature"),
            Self::SignatureMismatch => f.write_str("webhook signature mismatch"),
            Self::EmptyBody => f.write_str("empty webhook body"),
            Self::BodyTooLarge { size, max } => {
                write!(f, "webhook body too large ({size} > {max})")
            }
            Self::InvalidJson { message } => write!(f, "invalid webhook JSON: {message}"),
            Self::TimestampOutOfWindow {
                skew_secs,
                max_secs,
            } => {
                write!(f, "webhook timestamp skew {skew_secs}s exceeds {max_secs}s")
            }
        }
    }
}

fn normalize_signature(signature: &str) -> Option<String> {
    let trimmed = signature.trim();
    if trimmed.is_empty() {
        return None;
    }
    let hex = trimmed
        .strip_prefix("sha256=")
        .or_else(|| trimmed.strip_prefix("SHA256="))
        .unwrap_or(trimmed)
        .trim();
    if hex.is_empty() || !hex.chars().all(|c| c.is_ascii_hexdigit()) || hex.len() % 2 != 0 {
        return None;
    }
    Some(hex.to_ascii_lowercase())
}

fn signature_matches(secret: &[u8], body: &[u8], provided_hex: &str) -> bool {
    let Ok(mut mac) = HmacSha256::new_from_slice(secret) else {
        return false;
    };
    mac.update(body);
    let calculated = mac.finalize().into_bytes();
    let calculated_hex = hex_encode(calculated.as_slice());
    constant_time_eq(calculated_hex.as_bytes(), provided_hex.as_bytes())
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        out.push(HEX[(b >> 4) as usize] as char);
        out.push(HEX[(b & 0xf) as usize] as char);
    }
    out
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

/// Computes the hex HMAC-SHA256 Mollie would send (tests / fixtures).
pub fn compute_mollie_signature_hex(secret: &[u8], raw_body: &[u8]) -> MollieResult<String> {
    let mut mac = HmacSha256::new_from_slice(secret)
        .map_err(|_| MollieError::invalid_configuration("invalid webhook HMAC key length"))?;
    mac.update(raw_body);
    Ok(hex_encode(mac.finalize().into_bytes().as_slice()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verifies_known_vector() {
        let secret = "whsec_test_secret";
        let body = br#"{"id":"event_test","type":"payment.paid"}"#;
        let sig = compute_mollie_signature_hex(secret.as_bytes(), body).unwrap();
        let verifier = WebhookVerifier::new(secret).unwrap();
        verifier.verify(body, &sig).unwrap();
        verifier.verify(body, &format!("sha256={sig}")).unwrap();
    }

    #[test]
    fn rejects_tampered_body() {
        let secret = "whsec_test_secret";
        let body = br#"{"id":"event_test"}"#;
        let sig = compute_mollie_signature_hex(secret.as_bytes(), body).unwrap();
        let verifier = WebhookVerifier::new(secret).unwrap();
        assert!(verifier.verify(br#"{"id":"event_other"}"#, &sig).is_err());
    }

    #[test]
    fn supports_previous_secret_rotation() {
        let body = br#"{"ok":true}"#;
        let old = "old_secret";
        let new = "new_secret";
        let sig = compute_mollie_signature_hex(old.as_bytes(), body).unwrap();
        let verifier = WebhookVerifier::new(new)
            .unwrap()
            .with_previous_secret(old)
            .unwrap();
        verifier.verify(body, &sig).unwrap();
    }

    #[test]
    fn verify_and_decode_json() {
        #[derive(serde::Deserialize, Debug, PartialEq)]
        struct Ev {
            id: String,
        }
        let secret = "s";
        let body = br#"{"id":"event_1"}"#;
        let sig = compute_mollie_signature_hex(secret.as_bytes(), body).unwrap();
        let ev: Ev = WebhookVerifier::new(secret)
            .unwrap()
            .verify_and_decode(body, &sig)
            .unwrap();
        assert_eq!(ev.id, "event_1");
    }

    #[test]
    fn rejects_oversized_body() {
        let secret = "s";
        let body = vec![b'a'; 100];
        let sig = compute_mollie_signature_hex(secret.as_bytes(), &body).unwrap();
        let verifier = WebhookVerifier::new(secret)
            .unwrap()
            .with_max_body_bytes(10);
        assert!(verifier.verify(&body, &sig).is_err());
    }

    #[test]
    fn secret_debug_redacts() {
        let s = WebhookSigningSecret::new("super-secret").unwrap();
        assert!(!format!("{s:?}").contains("super-secret"));
    }

    #[test]
    fn missing_signature_is_distinct_from_malformed() {
        let v = WebhookVerifier::new("s").unwrap();
        let body = br#"{"a":1}"#;
        let missing = v.verify_header(body, None).unwrap_err();
        assert!(matches!(
            missing,
            MollieError::WebhookVerification {
                failure: WebhookVerifyFailure::MissingSignature
            }
        ));
        let empty = v.verify(body, "   ").unwrap_err();
        assert!(matches!(
            empty,
            MollieError::WebhookVerification {
                failure: WebhookVerifyFailure::MissingSignature
            }
        ));
        let malformed = v.verify(body, "not-hex!!").unwrap_err();
        assert!(matches!(
            malformed,
            MollieError::WebhookVerification {
                failure: WebhookVerifyFailure::MalformedSignature
            }
        ));
    }

    #[test]
    fn whitespace_only_body_change_fails_signature() {
        let secret = "s";
        let body = br#"{"id":"e"}"#;
        let sig = compute_mollie_signature_hex(secret.as_bytes(), body).unwrap();
        let v = WebhookVerifier::new(secret).unwrap();
        assert!(v.verify(br#"{"id":"e"} "#, &sig).is_err());
    }
}
