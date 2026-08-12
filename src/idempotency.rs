//! Request-scoped idempotency keys for Mollie write operations.
//!
//! Prefer binding a key to **one logical operation** (and its retries) rather
//! than leaving a sticky key on the client across unrelated calls.
//!
//! # Recommended usage
//!
//! ```rust,no_run
//! use mollie_rs::{IdempotencyKey, MollieClient};
//!
//! # async fn example(client: MollieClient) -> Result<(), mollie_rs::MollieError> {
//! let key = IdempotencyKey::generate();
//! // Scope the key to this retryable write only:
//! let client = client.with_idempotency_key(key.as_str());
//! // … perform create_payment / create_refund, then drop the scoped client …
//! let _ = client;
//! # Ok(())
//! # }
//! ```
//!
//! Client-global sticky keys (`Client::with_idempotency_key`) remain available
//! for compatibility but are easy to misuse across unrelated operations.
#![warn(missing_docs)]

use std::fmt;

use crate::{MollieError, MollieResult};

/// Maximum length accepted for a Mollie idempotency key (UUID-friendly upper bound).
pub const IDEMPOTENCY_KEY_MAX_LEN: usize = 64;

/// A validated idempotency key for one logical Mollie operation.
#[derive(Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct IdempotencyKey(String);

impl IdempotencyKey {
    /// Validates a caller-supplied idempotency key.
    ///
    /// # Errors
    ///
    /// Returns [`MollieError::InvalidRequest`] when the key is empty, longer
    /// than [`IDEMPOTENCY_KEY_MAX_LEN`], or contains whitespace/control chars.
    pub fn new(key: impl Into<String>) -> MollieResult<Self> {
        let key = key.into();
        if key.is_empty() {
            return Err(MollieError::invalid_request(
                "idempotency key cannot be empty",
            ));
        }
        if key.len() > IDEMPOTENCY_KEY_MAX_LEN {
            return Err(MollieError::invalid_request(format!(
                "idempotency key exceeds {IDEMPOTENCY_KEY_MAX_LEN} characters"
            )));
        }
        if key.chars().any(|c| c.is_whitespace() || c.is_control()) {
            return Err(MollieError::invalid_request(
                "idempotency key cannot contain whitespace or control characters",
            ));
        }
        Ok(Self(key))
    }

    /// Generates a new UUID v4 key suitable for a single logical operation.
    pub fn generate() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }

    /// Returns the key as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the key and returns the owned string.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Debug for IdempotencyKey {
    /// Redacts the raw key in debug output (correlation should use logs carefully).
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("IdempotencyKey(<redacted>)")
    }
}

impl fmt::Display for IdempotencyKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for IdempotencyKey {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl TryFrom<&str> for IdempotencyKey {
    type Error = MollieError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<IdempotencyKey> for String {
    fn from(value: IdempotencyKey) -> Self {
        value.into_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_produces_unique_keys() {
        let a = IdempotencyKey::generate();
        let b = IdempotencyKey::generate();
        assert_ne!(a.as_str(), b.as_str());
        assert!(uuid::Uuid::parse_str(a.as_str()).is_ok());
    }

    #[test]
    fn rejects_empty_and_whitespace() {
        assert!(IdempotencyKey::new("").is_err());
        assert!(IdempotencyKey::new("bad key").is_err());
    }

    #[test]
    fn accepts_uuid() {
        let key = IdempotencyKey::new("6f7ef3e6-8c2f-4d1c-9f08-5ab7adf56c91").unwrap();
        assert_eq!(key.as_str(), "6f7ef3e6-8c2f-4d1c-9f08-5ab7adf56c91");
        assert!(!format!("{key:?}").contains("6f7ef3e6"));
    }
}
