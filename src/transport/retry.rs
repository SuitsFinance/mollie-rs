//! Backoff computation for conservative retries.

use std::time::Duration;

use super::RetryPolicy;

/// Outcome of evaluating whether another attempt should run.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RetryDecision {
    /// Do not retry.
    Stop,
    /// Sleep then retry.
    RetryAfter(Duration),
}

/// Computes exponential backoff with full jitter in `[1ms, min(base*2^(n-1), max)]`.
///
/// `attempt` is 1-based for the **next** attempt after a failure.
///
/// When `Retry-After` is present it wins (capped by `max_backoff`).
/// Jitter uses OS entropy via `getrandom` when available; if entropy fails,
/// a deterministic mix is used so callers still make progress (documented as
/// degraded, not preferred for multi-host thundering-herd avoidance).
pub fn compute_backoff(
    policy: &RetryPolicy,
    attempt: u32,
    retry_after: Option<Duration>,
) -> Duration {
    if let Some(server) = retry_after {
        return server.min(policy.max_backoff).max(Duration::from_millis(1));
    }
    let exp = attempt.saturating_sub(1).min(16);
    let base_ms = policy.initial_backoff.as_millis() as u64;
    let max_ms = policy.max_backoff.as_millis() as u64;
    let ceiling = base_ms.saturating_mul(1u64 << exp).min(max_ms).max(1);
    let mixed = random_u64().unwrap_or_else(|| {
        ceiling
            .wrapping_mul(1103515245)
            .wrapping_add(attempt as u64 * 12345)
    }) % (ceiling + 1);
    Duration::from_millis(mixed.max(1))
}

fn random_u64() -> Option<u64> {
    getrandom::u64().ok()
}

/// Status codes the transport may retry under a permissive policy.
pub fn is_transient_http_status(status: reqwest::StatusCode) -> bool {
    matches!(status.as_u16(), 408 | 429 | 500 | 502 | 503 | 504)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn respects_retry_after_over_backoff() {
        let policy = RetryPolicy::default_safe();
        let delay = compute_backoff(&policy, 2, Some(Duration::from_secs(7)));
        assert_eq!(delay, Duration::from_secs(7));
    }

    #[test]
    fn backoff_is_positive_and_capped() {
        let policy = RetryPolicy::default_safe();
        let delay = compute_backoff(&policy, 20, None);
        assert!(delay >= Duration::from_millis(1));
        assert!(delay <= policy.max_backoff);
    }

    #[test]
    fn does_not_treat_client_errors_as_transient() {
        assert!(!is_transient_http_status(reqwest::StatusCode::BAD_REQUEST));
        assert!(!is_transient_http_status(reqwest::StatusCode::UNAUTHORIZED));
        assert!(!is_transient_http_status(
            reqwest::StatusCode::UNPROCESSABLE_ENTITY
        ));
        assert!(is_transient_http_status(
            reqwest::StatusCode::TOO_MANY_REQUESTS
        ));
        assert!(is_transient_http_status(reqwest::StatusCode::BAD_GATEWAY));
    }
}
