//! Rate-limit state helpers.

use std::time::Duration;

use crate::ResponseMetadata;

/// Snapshot of rate-limit headers useful for backoff decisions.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RateLimitState {
    /// Parsed remaining quota when known.
    pub remaining: Option<u64>,
    /// Parsed limit when known.
    pub limit: Option<u64>,
    /// Suggested wait from `Retry-After` or reset.
    pub retry_after: Option<Duration>,
}

impl RateLimitState {
    /// Builds state from response metadata.
    pub fn from_metadata(meta: &ResponseMetadata) -> Self {
        Self {
            remaining: meta.rate_limit_remaining,
            limit: meta.rate_limit_limit,
            retry_after: meta.suggested_retry_delay(),
        }
    }
}
