//! Retry policy configuration.

use std::time::Duration;

use super::RetryClass;

/// Conservative retry policy for Mollie HTTP calls.
///
/// Default construction via [`RetryPolicy::disabled`] performs **no** automatic
/// retries. Use [`RetryPolicy::default_safe`] for reads and idempotent writes only.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RetryPolicy {
    /// When false, the transport never retries.
    pub enabled: bool,
    /// Maximum attempts including the first try (must be >= 1).
    pub max_attempts: u32,
    /// Soft wall-clock **retry budget** for scheduling attempts and backoff.
    ///
    /// When the budget is exhausted the transport **does not** issue an extra
    /// leftover request; it returns the last attempt’s result (or a timeout
    /// classification when no attempt ran). Prefer setting this to the maximum
    /// time you are willing to wait for retries **including** backoff sleeps.
    ///
    /// Named `total_deadline` for compatibility; prefer [`Self::retry_budget`].
    pub total_deadline: Duration,
    /// Initial backoff base for exponential growth.
    pub initial_backoff: Duration,
    /// Cap on computed backoff (and honored `Retry-After` is also capped).
    pub max_backoff: Duration,
    /// Retry GET/HEAD-class operations on transient failures.
    pub retry_safe_reads: bool,
    /// Retry write-class operations only when an idempotency key is sticky/present.
    pub retry_idempotent_writes: bool,
}

impl Default for RetryPolicy {
    /// Disabled retries (safe default).
    fn default() -> Self {
        Self::disabled()
    }
}

impl RetryPolicy {
    /// No automatic retries.
    pub const fn disabled() -> Self {
        Self {
            enabled: false,
            max_attempts: 1,
            total_deadline: Duration::from_secs(60),
            initial_backoff: Duration::from_millis(200),
            max_backoff: Duration::from_secs(8),
            retry_safe_reads: false,
            retry_idempotent_writes: false,
        }
    }

    /// Conservative production defaults: reads retry; writes only with idempotency.
    pub const fn default_safe() -> Self {
        Self {
            enabled: true,
            max_attempts: 3,
            total_deadline: Duration::from_secs(30),
            initial_backoff: Duration::from_millis(200),
            max_backoff: Duration::from_secs(8),
            retry_safe_reads: true,
            retry_idempotent_writes: true,
        }
    }

    /// Retry budget (alias for [`Self::total_deadline`]).
    ///
    /// Invariant: after this wall-clock duration from the first attempt start,
    /// the transport must not begin a new HTTP attempt.
    pub const fn retry_budget(&self) -> Duration {
        self.total_deadline
    }

    /// Sets the retry budget (writes both the public field and documents intent).
    pub fn with_retry_budget(mut self, budget: Duration) -> Self {
        self.total_deadline = budget;
        self
    }

    /// Returns whether this policy may retry the given class.
    ///
    /// For [`RetryClass::IdempotentWrite`], `has_sticky_idempotency_key` must be
    /// true: a **caller-bound** sticky key on the client (not merely an
    /// auto-generated per-request UUID). Auto keys alone must not enable
    /// multi-attempt write retries, because they are easy to misuse across
    /// logical operations if higher layers re-issue requests.
    ///
    /// [`RetryClass::NonRetryableWrite`], [`RetryClass::ProviderDefined`], and
    /// [`RetryClass::Unknown`] never auto-retry.
    pub fn allows(&self, class: RetryClass, has_sticky_idempotency_key: bool) -> bool {
        if !self.enabled || self.max_attempts <= 1 {
            return false;
        }
        match class {
            RetryClass::SafeRead => self.retry_safe_reads,
            RetryClass::IdempotentWrite => {
                self.retry_idempotent_writes && has_sticky_idempotency_key
            }
            RetryClass::NonRetryableWrite
            | RetryClass::NeverAutoRetry
            | RetryClass::ProviderDefined
            | RetryClass::Unknown => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disabled_never_allows() {
        let p = RetryPolicy::disabled();
        assert!(!p.allows(RetryClass::SafeRead, true));
        assert!(!p.allows(RetryClass::IdempotentWrite, true));
        assert!(!p.allows(RetryClass::NonRetryableWrite, true));
        assert!(!p.allows(RetryClass::NeverAutoRetry, true));
        assert!(!p.allows(RetryClass::ProviderDefined, true));
    }

    #[test]
    fn default_safe_requires_key_for_writes() {
        let p = RetryPolicy::default_safe();
        assert!(p.allows(RetryClass::SafeRead, false));
        assert!(!p.allows(RetryClass::IdempotentWrite, false));
        assert!(p.allows(RetryClass::IdempotentWrite, true));
        assert!(!p.allows(RetryClass::NonRetryableWrite, true));
        assert!(!p.allows(RetryClass::NeverAutoRetry, true));
        assert!(!p.allows(RetryClass::ProviderDefined, true));
        assert!(!p.allows(RetryClass::Unknown, true));
    }

    #[test]
    fn retry_budget_alias() {
        let p = RetryPolicy::default_safe().with_retry_budget(Duration::from_secs(5));
        assert_eq!(p.retry_budget(), Duration::from_secs(5));
        assert_eq!(p.total_deadline, Duration::from_secs(5));
    }
}
