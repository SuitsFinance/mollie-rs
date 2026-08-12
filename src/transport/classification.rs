//! Operation retry classification for Mollie routes.
//!
//! **Source of truth:** [`crate::route_capability`] metadata generated from the
//! OpenAPI operation registry. HTTP method is only a last-resort fallback and
//! never upgrades an unknown write into an auto-retryable class.

/// How the transport layer may retry an operation.
///
/// Prefer route capability metadata ([`crate::route_capability`]) over raw HTTP
/// method classification when the operation id is known.
///
/// ## Classes
///
/// | Class | Auto-retry under [`crate::RetryPolicy::default_safe`] |
/// | --- | --- |
/// | [`SafeRead`](Self::SafeRead) | Yes (no idempotency key required) |
/// | [`IdempotentWrite`](Self::IdempotentWrite) | Only with a **sticky** caller-owned key |
/// | [`NonRetryableWrite`](Self::NonRetryableWrite) | Never |
/// | [`ProviderDefined`](Self::ProviderDefined) | Never by default (explicit policy later) |
/// | [`NeverAutoRetry`](Self::NeverAutoRetry) | Never (historical alias of non-retryable) |
/// | [`Unknown`](Self::Unknown) | Never |
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RetryClass {
    /// Safe read (GET/HEAD); may retry transient failures without idempotency.
    SafeRead,
    /// Provider-idempotent write: retry only with a **caller-owned** sticky
    /// idempotency key (never with an auto-generated per-request UUID alone).
    ///
    /// This is the payment-safe equivalent of “provider idempotent write” used
    /// in the parity registry (`retry_class: provider_idempotent_write`).
    IdempotentWrite,
    /// Financial or side-effecting write that the SDK must **never** auto-retry.
    ///
    /// Used for token churn, one-shot side effects, and operations where a
    /// second attempt is not safe even with an idempotency key.
    NonRetryableWrite,
    /// Behavior is defined by Mollie/provider docs for this operation and is
    /// not expressed as a generic safe-read / idempotent-write rule.
    ///
    /// Transport treats this as non-retryable until an explicit per-operation
    /// policy is configured (safe default for a payment SDK).
    ProviderDefined,
    /// Historical name for non-retryable writes.
    ///
    /// Prefer [`Self::NonRetryableWrite`]. Kept so existing matches and docs
    /// that used this variant remain valid on the 0.6 line.
    NeverAutoRetry,
    /// Classification unknown (missing registry entry); treat as never auto-retry.
    Unknown,
}

impl RetryClass {
    /// Registry / docs alias: provider-idempotent write.
    pub const fn provider_idempotent_write() -> Self {
        Self::IdempotentWrite
    }

    /// Historical constructor for never-auto-retry policy.
    pub const fn never_auto_retry() -> Self {
        Self::NonRetryableWrite
    }

    /// Returns `true` when the class never participates in automatic retries.
    pub const fn is_non_retryable(self) -> bool {
        matches!(
            self,
            Self::NonRetryableWrite | Self::NeverAutoRetry | Self::ProviderDefined | Self::Unknown
        )
    }
}

/// Last-resort classification when no route capability exists for an operation.
///
/// **Security note:** unknown methods and all non-safe methods map to
/// [`RetryClass::Unknown`] / never-auto-retry. The registry must list every
/// generated operation; this fallback must not re-introduce method-based
/// “POST is retryable” assumptions.
pub fn classify_http_method(method: &str) -> RetryClass {
    match method.to_ascii_uppercase().as_str() {
        "GET" | "HEAD" | "OPTIONS" => RetryClass::SafeRead,
        // Writes without registry metadata: never auto-retry.
        "POST" | "PUT" | "PATCH" | "DELETE" => RetryClass::Unknown,
        _ => RetryClass::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_is_safe_read() {
        assert_eq!(classify_http_method("GET"), RetryClass::SafeRead);
    }

    #[test]
    fn post_without_registry_is_not_auto_retryable() {
        assert_eq!(classify_http_method("POST"), RetryClass::Unknown);
        assert!(classify_http_method("POST").is_non_retryable());
    }

    #[test]
    fn aliases() {
        assert_eq!(
            RetryClass::provider_idempotent_write(),
            RetryClass::IdempotentWrite
        );
        assert_eq!(
            RetryClass::never_auto_retry(),
            RetryClass::NonRetryableWrite
        );
        assert!(RetryClass::NeverAutoRetry.is_non_retryable());
    }
}
