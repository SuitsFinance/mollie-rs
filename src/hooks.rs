//! Narrow request lifecycle hooks for observability and test doubles.
//!
//! These hooks intentionally stay small. Prefer application middleware for
//! complex pipelines; use hooks for redacted metrics, correlation IDs, and
//! request mutation that must stay next to the Mollie client.

use std::sync::Arc;

use crate::error::MollieError;
use crate::metadata::ResponseMetadata;

/// Context available to request hooks (never includes raw secrets).
#[derive(Clone, Debug)]
pub struct RequestContext {
    /// OpenAPI / generated operation id (snake_case).
    pub operation: &'static str,
    /// HTTP method string (e.g. `"GET"`).
    pub method: String,
    /// Request URL with sensitive query values redacted where known.
    pub url_redacted: String,
    /// 1-based attempt number when retries are enabled.
    pub attempt: u32,
    /// Whether a sticky caller-owned idempotency key is bound.
    pub has_sticky_idempotency: bool,
    /// Sticky profile id when configured (token form, not a secret).
    pub profile_id: Option<String>,
    /// Sticky testmode when configured.
    pub testmode: Option<bool>,
}

/// Lifecycle hooks invoked around each HTTP attempt.
///
/// All methods have default no-op implementations so callers can implement only
/// what they need.
pub trait RequestHook: Send + Sync {
    /// Called before each HTTP attempt. May mutate headers (not Authorization
    /// secrets in logs). Prefer additive headers over replacing auth.
    fn before_request(&self, _context: &RequestContext, _request: &mut reqwest::Request) {}

    /// Called after a response is received (any status).
    fn after_response(&self, _context: &RequestContext, _metadata: &ResponseMetadata) {}

    /// Called when the SDK maps a failure into [`MollieError`] (best-effort;
    /// transport-level failures may only surface as progenitor errors).
    fn on_error(&self, _context: &RequestContext, _error: &MollieError) {}
}

/// Shared hook handle stored on clients.
pub type SharedRequestHook = Arc<dyn RequestHook>;

/// No-op hook used when none is configured.
#[derive(Clone, Copy, Debug, Default)]
pub struct NoopHook;

impl RequestHook for NoopHook {}
