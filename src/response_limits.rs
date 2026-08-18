//! Configurable response body size limits for transport buffering.
//!
//! Production clients must not buffer unbounded provider payloads into memory.
//! Limits apply to successful JSON bodies, provider error bodies, and (as a
//! documented default) webhook raw bodies owned by the application verifier.

#![warn(missing_docs)]

use crate::webhook_verify::DEFAULT_MAX_WEBHOOK_BODY_BYTES;
use crate::MAX_RETAINED_BODY_BYTES;

/// Default maximum successful JSON response body (8 MiB).
pub const DEFAULT_MAX_JSON_BYTES: usize = 8 * 1024 * 1024;

/// Default maximum provider error body retained/decoded (64 KiB).
///
/// Matches [`crate::MAX_RETAINED_BODY_BYTES`] so error diagnostics stay bounded.
pub const DEFAULT_MAX_ERROR_BODY_BYTES: usize = MAX_RETAINED_BODY_BYTES;

/// Bounds on response bodies the SDK will buffer in memory.
///
/// Defaults are conservative for payment API payloads while remaining
/// operationally realistic for large list pages.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResponseLimits {
    /// Maximum bytes for successful JSON response bodies.
    pub max_json_bytes: usize,
    /// Maximum bytes for provider error response bodies.
    pub max_error_body_bytes: usize,
    /// Recommended maximum for application-owned webhook raw bodies.
    ///
    /// Applied by [`crate::WebhookVerifier`] when constructed with defaults;
    /// see [`crate::DEFAULT_MAX_WEBHOOK_BODY_BYTES`].
    pub max_webhook_bytes: usize,
}

impl Default for ResponseLimits {
    fn default() -> Self {
        Self {
            max_json_bytes: DEFAULT_MAX_JSON_BYTES,
            max_error_body_bytes: DEFAULT_MAX_ERROR_BODY_BYTES,
            max_webhook_bytes: DEFAULT_MAX_WEBHOOK_BODY_BYTES,
        }
    }
}

impl ResponseLimits {
    /// Returns production defaults.
    pub const fn new() -> Self {
        Self {
            max_json_bytes: DEFAULT_MAX_JSON_BYTES,
            max_error_body_bytes: DEFAULT_MAX_ERROR_BODY_BYTES,
            max_webhook_bytes: DEFAULT_MAX_WEBHOOK_BODY_BYTES,
        }
    }

    /// Sets the successful JSON body limit.
    pub const fn with_max_json_bytes(mut self, max: usize) -> Self {
        self.max_json_bytes = max;
        self
    }

    /// Sets the provider error body limit.
    pub const fn with_max_error_body_bytes(mut self, max: usize) -> Self {
        self.max_error_body_bytes = max;
        self
    }

    /// Sets the recommended webhook body limit.
    pub const fn with_max_webhook_bytes(mut self, max: usize) -> Self {
        self.max_webhook_bytes = max;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_positive_and_ordered() {
        let limits = ResponseLimits::default();
        assert!(limits.max_error_body_bytes > 0);
        assert!(limits.max_json_bytes >= limits.max_error_body_bytes);
        assert_eq!(limits.max_webhook_bytes, DEFAULT_MAX_WEBHOOK_BODY_BYTES);
        assert_eq!(limits.max_error_body_bytes, MAX_RETAINED_BODY_BYTES);
    }
}
