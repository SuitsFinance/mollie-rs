//! First-class delivery outcomes for payment-safe transport decisions.
//!
//! A timeout after bytes may have left the client is **not** the same as a
//! connect failure before transmit. Financial writes must treat the former as
//! [`DeliveryOutcome::Unknown`] and only auto-retry when policy + sticky key allow.

use super::{RetryClass, RetryPolicy};

/// Whether the client can treat a request as definitively delivered.
///
/// | Variant | Meaning |
/// | --- | --- |
/// | [`NotSent`](Self::NotSent) | Request not known to leave the client |
/// | [`Rejected`](Self::Rejected) | Provider definitive rejection (e.g. 4xx) |
/// | [`Succeeded`](Self::Succeeded) | Definitive success |
/// | [`Unknown`](Self::Unknown) | May have been processed (post-transmit timeout/reset/cancel) |
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum DeliveryOutcome {
    /// Request not known to leave the client (connect/DNS/builder before write).
    NotSent,
    /// Provider returned a definitive client/server rejection the SDK will not
    /// auto-retry as success.
    Rejected,
    /// Definitive success (2xx as applicable).
    Succeeded,
    /// Outcome unknown: timeout/reset after transmit, drop of in-flight future,
    /// or ambiguous gateway failure mid-flight.
    Unknown,
}

impl DeliveryOutcome {
    /// Returns `true` when another HTTP attempt may be considered (still subject
    /// to [`RetryPolicy::allows`] and attempt/budget caps).
    pub const fn may_schedule_retry(self) -> bool {
        matches!(self, Self::NotSent | Self::Unknown)
    }

    /// Returns `true` for ambiguous post-transmit failures.
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// Classifies a `reqwest` error into a delivery outcome.
///
/// Connect/builder failures map to [`DeliveryOutcome::NotSent`]. Timeouts and
/// other mid-flight failures map to [`DeliveryOutcome::Unknown`].
pub fn classify_reqwest_error(error: &reqwest::Error) -> DeliveryOutcome {
    if error.is_connect() || error.is_builder() {
        return DeliveryOutcome::NotSent;
    }
    if error.is_timeout() {
        return DeliveryOutcome::Unknown;
    }
    // Request/body errors after the call started are treated as unknown for
    // payment safety (may have reached the provider).
    if error.is_request() || error.is_body() {
        return DeliveryOutcome::Unknown;
    }
    // Decode happens after a response body was received; mutation may have applied.
    if error.is_decode() {
        return DeliveryOutcome::Unknown;
    }
    DeliveryOutcome::Unknown
}

/// Classifies an HTTP status from a completed response.
pub fn classify_http_status(status: reqwest::StatusCode) -> DeliveryOutcome {
    if status.is_success() {
        DeliveryOutcome::Succeeded
    } else if super::is_transient_http_status(status) {
        // Transient gateway statuses are ambiguous for writes until a later
        // definitive response; treat as Unknown for delivery semantics.
        DeliveryOutcome::Unknown
    } else if status.is_client_error() || status.is_server_error() {
        DeliveryOutcome::Rejected
    } else {
        DeliveryOutcome::Unknown
    }
}

/// Whether the transport may auto-retry given delivery outcome + policy class.
///
/// Invariants:
/// - Policy/`RetryClass` sticky-key rules still apply via [`RetryPolicy::allows`].
/// - [`DeliveryOutcome::Rejected`] / [`Succeeded`] never auto-retry.
/// - [`Unknown`] only retries when the class is policy-eligible (reads, or
///   idempotent writes with sticky key) — never upgrades a non-retryable write.
pub fn should_auto_retry(
    outcome: DeliveryOutcome,
    class: RetryClass,
    has_sticky_idempotency_key: bool,
    policy: &RetryPolicy,
) -> bool {
    if !outcome.may_schedule_retry() {
        return false;
    }
    policy.allows(class, has_sticky_idempotency_key)
}

/// Synthetic attempt events for model/property tests of the retry engine.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AttemptEvent {
    /// TCP/connect failed before request body left the client.
    ConnectFailure,
    /// Timeout after the request may have been in flight.
    Timeout,
    /// HTTP 429.
    Status429,
    /// HTTP 503.
    Status503,
    /// HTTP 400 validation.
    Status400,
    /// HTTP 200 success.
    Success,
    /// Wall-clock budget already exhausted before this attempt would start.
    DeadlineExhausted,
}

/// Pure simulation of attempt scheduling (no I/O) for property tests.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RetrySimulation {
    /// How many HTTP attempts would begin.
    pub attempts_started: u32,
    /// Last delivery outcome observed (if any attempt ran).
    pub last_outcome: Option<DeliveryOutcome>,
    /// Whether a success response ended the loop.
    pub succeeded: bool,
}

/// Simulates the send-loop attempt policy against a fixed event sequence.
///
/// Each event is consumed only when an attempt is allowed to start. Proves:
/// - financial / non-retryable writes without sticky key → `attempts_started <= 1`
/// - no attempt begins after a [`AttemptEvent::DeadlineExhausted`] marker when
///   it appears as the next scheduled event under budget rules
pub fn simulate_retry_loop(
    class: RetryClass,
    has_sticky: bool,
    policy: &RetryPolicy,
    events: &[AttemptEvent],
) -> RetrySimulation {
    let may_retry = policy.allows(class, has_sticky);
    let max_attempts = if may_retry {
        policy.max_attempts.max(1)
    } else {
        1
    };

    let mut attempts_started = 0u32;
    let mut last_outcome = None;
    let mut succeeded = false;
    let mut event_idx = 0usize;

    for attempt in 1..=max_attempts {
        let Some(event) = events.get(event_idx) else {
            break;
        };
        event_idx += 1;

        if matches!(event, AttemptEvent::DeadlineExhausted) {
            // No HTTP attempt begins once the budget is exhausted.
            break;
        }

        attempts_started = attempts_started.saturating_add(1);

        let outcome = match event {
            AttemptEvent::ConnectFailure => DeliveryOutcome::NotSent,
            AttemptEvent::Timeout => DeliveryOutcome::Unknown,
            AttemptEvent::Status429 | AttemptEvent::Status503 => DeliveryOutcome::Unknown,
            AttemptEvent::Status400 => DeliveryOutcome::Rejected,
            AttemptEvent::Success => DeliveryOutcome::Succeeded,
            AttemptEvent::DeadlineExhausted => unreachable!("handled above"),
        };
        last_outcome = Some(outcome);

        if matches!(outcome, DeliveryOutcome::Succeeded) {
            succeeded = true;
            break;
        }

        let is_last = attempt == max_attempts;
        if is_last {
            break;
        }

        let retry = should_auto_retry(outcome, class, has_sticky, policy);
        // Transient HTTP statuses also require policy eligibility (already in should_auto_retry).
        if !retry {
            break;
        }
        // Peek: if next event is deadline, do not start another attempt.
        if matches!(events.get(event_idx), Some(AttemptEvent::DeadlineExhausted)) {
            break;
        }
    }

    RetrySimulation {
        attempts_started,
        last_outcome,
        succeeded,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn financial_write_without_sticky_single_attempt_on_timeout() {
        let policy = RetryPolicy::default_safe();
        let sim = simulate_retry_loop(
            RetryClass::IdempotentWrite,
            false,
            &policy,
            &[
                AttemptEvent::Timeout,
                AttemptEvent::Timeout,
                AttemptEvent::Success,
            ],
        );
        assert_eq!(sim.attempts_started, 1);
        assert_eq!(sim.last_outcome, Some(DeliveryOutcome::Unknown));
        assert!(!sim.succeeded);
    }

    #[test]
    fn idempotent_write_with_sticky_retries_unknown_then_success() {
        let policy = RetryPolicy::default_safe();
        let sim = simulate_retry_loop(
            RetryClass::IdempotentWrite,
            true,
            &policy,
            &[
                AttemptEvent::Timeout,
                AttemptEvent::Status503,
                AttemptEvent::Success,
            ],
        );
        assert_eq!(sim.attempts_started, 3);
        assert!(sim.succeeded);
    }

    #[test]
    fn non_retryable_never_retries_connect_failure() {
        let policy = RetryPolicy::default_safe();
        let sim = simulate_retry_loop(
            RetryClass::NonRetryableWrite,
            true,
            &policy,
            &[AttemptEvent::ConnectFailure, AttemptEvent::Success],
        );
        assert_eq!(sim.attempts_started, 1);
        assert!(!sim.succeeded);
    }

    #[test]
    fn no_attempt_after_deadline_marker() {
        let policy = RetryPolicy::default_safe();
        let sim = simulate_retry_loop(
            RetryClass::SafeRead,
            false,
            &policy,
            &[
                AttemptEvent::Status503,
                AttemptEvent::DeadlineExhausted,
                AttemptEvent::Success,
            ],
        );
        assert_eq!(sim.attempts_started, 1);
        assert!(!sim.succeeded);
    }

    #[test]
    fn safe_read_retries_not_sent() {
        let policy = RetryPolicy::default_safe();
        let sim = simulate_retry_loop(
            RetryClass::SafeRead,
            false,
            &policy,
            &[AttemptEvent::ConnectFailure, AttemptEvent::Success],
        );
        assert_eq!(sim.attempts_started, 2);
        assert!(sim.succeeded);
    }

    #[test]
    fn rejected_does_not_retry() {
        let policy = RetryPolicy::default_safe();
        let sim = simulate_retry_loop(
            RetryClass::SafeRead,
            false,
            &policy,
            &[AttemptEvent::Status400, AttemptEvent::Success],
        );
        assert_eq!(sim.attempts_started, 1);
        assert_eq!(sim.last_outcome, Some(DeliveryOutcome::Rejected));
    }
}
