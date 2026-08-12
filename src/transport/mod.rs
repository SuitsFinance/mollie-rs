//! Transport policy: retries, rate limits, deadlines, and delivery outcomes.
//!
//! Automatic retries are **opt-in** and conservative by default. Payment writes
//! are never retried automatically unless the operation is classified as safe
//! **and** a sticky/request-scoped idempotency key is present.
//!
//! Delivery outcomes ([`DeliveryOutcome`]) distinguish connect failures
//! ([`DeliveryOutcome::NotSent`]) from post-transmit ambiguity
//! ([`DeliveryOutcome::Unknown`]). See `docs/sdd/1.0-readiness/06-retries-idempotency.md`.

mod classification;
mod delivery;
mod policy;
mod rate_limit;
mod retry;

pub use classification::{classify_http_method, RetryClass};
pub use delivery::{
    classify_http_status, classify_reqwest_error, should_auto_retry, simulate_retry_loop,
    AttemptEvent, DeliveryOutcome, RetrySimulation,
};
pub use policy::RetryPolicy;
pub use rate_limit::RateLimitState;
pub use retry::{compute_backoff, is_transient_http_status, RetryDecision};
