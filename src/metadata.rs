//! Operational response metadata extracted from HTTP headers and status.
//!
//! Payment integrations need request correlation, rate-limit state, and
//! retry hints without parsing raw [`HeaderMap`]s at every call site.
//! [`ResponseMetadata`] is the stable place for those fields.
//!
//! Only fields Mollie (or a proxy) actually sent are populated. Optional
//! client-side fields (`elapsed`, `attempt`, `operation_id`, `endpoint`) are
//! filled by the transport layer when known.
#![warn(missing_docs)]

use std::time::Duration;

use reqwest::header::HeaderMap;
use reqwest::StatusCode;

/// Maximum raw error body bytes retained in metadata-adjacent error contexts.
pub const MAX_RETAINED_BODY_BYTES: usize = 64 * 1024;

/// Correlation and transport metadata for one Mollie HTTP response (or error).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResponseMetadata {
    /// HTTP status code when known.
    pub status: Option<StatusCode>,
    /// Provider or edge request identifier when present.
    ///
    /// Checked headers: `request-id`, `x-request-id`, `x-mollie-request-id`.
    pub request_id: Option<String>,
    /// Resolved `Idempotency-Key` when known from request or response headers.
    pub idempotency_key: Option<String>,
    /// Parsed `Retry-After` delay when the header is present (delta-seconds or HTTP-date).
    pub retry_after: Option<Duration>,
    /// Parsed rate-limit **limit** when available from structured headers.
    pub rate_limit_limit: Option<u64>,
    /// Parsed rate-limit **remaining** when available.
    pub rate_limit_remaining: Option<u64>,
    /// Parsed rate-limit **reset** delay when available.
    pub rate_limit_reset: Option<Duration>,
    /// Raw `RateLimit` / `ratelimit` header value when present.
    pub rate_limit: Option<String>,
    /// Raw `RateLimit-Policy` / `ratelimit-policy` header value when present.
    pub rate_limit_policy: Option<String>,
    /// Response `Content-Type` header value when present.
    pub content_type: Option<String>,
    /// Response body size in bytes when known (may be truncated for errors).
    pub response_size: Option<usize>,
    /// Client-measured elapsed time for the attempt / operation when known.
    pub elapsed: Option<Duration>,
    /// OpenAPI / SDK operation id when known (transport policy).
    pub operation_id: Option<&'static str>,
    /// Request path or endpoint template when known.
    pub endpoint: Option<String>,
    /// 1-based attempt number when a retry policy executed.
    pub attempt: Option<u32>,
    /// Provider error title/code fragment when known from a HAL error body.
    pub provider_error_code: Option<String>,
    /// Stable SDK catalog key when classified.
    pub provider_error_key: Option<String>,
}

impl ResponseMetadata {
    /// Builds metadata from an HTTP status and response headers.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::time::Duration;
    ///
    /// use mollie_rs::ResponseMetadata;
    /// use reqwest::header::{HeaderMap, HeaderValue};
    /// use reqwest::StatusCode;
    ///
    /// let mut headers = HeaderMap::new();
    /// headers.insert("request-id", HeaderValue::from_static("req_123"));
    /// headers.insert("retry-after", HeaderValue::from_static("2"));
    /// headers.insert("idempotency-key", HeaderValue::from_static("key-1"));
    ///
    /// let meta = ResponseMetadata::from_status_and_headers(StatusCode::TOO_MANY_REQUESTS, &headers);
    /// assert_eq!(meta.request_id.as_deref(), Some("req_123"));
    /// assert_eq!(meta.retry_after, Some(Duration::from_secs(2)));
    /// assert_eq!(meta.idempotency_key.as_deref(), Some("key-1"));
    /// ```
    pub fn from_status_and_headers(status: StatusCode, headers: &HeaderMap) -> Self {
        let mut meta = Self::from_headers(headers);
        meta.status = Some(status);
        meta
    }

    /// Builds metadata from headers only (status unknown).
    pub fn from_headers(headers: &HeaderMap) -> Self {
        let rate_limit =
            header_str(headers, "ratelimit").or_else(|| header_str(headers, "rate-limit"));
        let (limit, remaining, reset) = parse_rate_limit_triplet(headers, rate_limit.as_deref());

        Self {
            status: None,
            request_id: first_header_str(
                headers,
                &["request-id", "x-request-id", "x-mollie-request-id"],
            ),
            idempotency_key: header_str(headers, "idempotency-key"),
            retry_after: parse_retry_after(headers),
            rate_limit_limit: limit,
            rate_limit_remaining: remaining,
            rate_limit_reset: reset,
            rate_limit,
            rate_limit_policy: header_str(headers, "ratelimit-policy")
                .or_else(|| header_str(headers, "rate-limit-policy")),
            content_type: header_str(headers, "content-type"),
            response_size: None,
            elapsed: None,
            operation_id: None,
            endpoint: None,
            attempt: None,
            provider_error_code: None,
            provider_error_key: None,
        }
    }

    /// Returns true when the response status is HTTP 429.
    pub fn is_rate_limited(&self) -> bool {
        self.status == Some(StatusCode::TOO_MANY_REQUESTS)
    }

    /// Suggested wait before a retry when `Retry-After` or reset is present.
    pub fn suggested_retry_delay(&self) -> Option<Duration> {
        self.retry_after.or(self.rate_limit_reset)
    }

    /// Sets the retained response body size (does not store the body).
    pub fn with_response_size(mut self, size: usize) -> Self {
        self.response_size = Some(size);
        self
    }

    /// Sets client-measured elapsed duration.
    pub fn with_elapsed(mut self, elapsed: Duration) -> Self {
        self.elapsed = Some(elapsed);
        self
    }

    /// Sets transport operation identity.
    pub fn with_operation(
        mut self,
        operation_id: &'static str,
        endpoint: impl Into<String>,
    ) -> Self {
        self.operation_id = Some(operation_id);
        self.endpoint = Some(endpoint.into());
        self
    }

    /// Sets the 1-based attempt number.
    pub fn with_attempt(mut self, attempt: u32) -> Self {
        self.attempt = Some(attempt);
        self
    }

    /// Attaches classified provider error identifiers when known.
    pub fn with_provider_error(mut self, code: impl Into<String>, key: impl Into<String>) -> Self {
        self.provider_error_code = Some(code.into());
        self.provider_error_key = Some(key.into());
        self
    }

    /// Prefers an explicit idempotency key when headers did not echo one.
    pub fn with_idempotency_key_fallback(mut self, key: Option<String>) -> Self {
        if self.idempotency_key.is_none() {
            self.idempotency_key = key;
        }
        self
    }
}

/// Bounded raw provider response retained for diagnostics (errors only).
#[derive(Clone, Debug)]
pub struct ErrorResponseContext {
    /// HTTP status when known.
    pub status: Option<StatusCode>,
    /// Response headers (never includes client Authorization).
    pub headers: HeaderMap,
    /// Structured metadata extracted from status/headers.
    pub metadata: ResponseMetadata,
    /// Content-Type when present.
    pub content_type: Option<String>,
    /// Bounded raw body (truncated when larger than [`MAX_RETAINED_BODY_BYTES`]).
    pub body: bytes::Bytes,
    /// Whether `body` was truncated.
    pub body_truncated: bool,
    /// Parsed Mollie HAL error when the body was valid JSON of that shape.
    pub parsed: Option<crate::types::ErrorResponse>,
}

impl ErrorResponseContext {
    /// Builds a context from status, headers, and raw body, capping body size.
    pub fn from_parts(status: StatusCode, headers: HeaderMap, body: bytes::Bytes) -> Self {
        let (body, body_truncated) = truncate_body(body);
        let mut metadata = ResponseMetadata::from_status_and_headers(status, &headers)
            .with_response_size(body.len());
        let content_type = metadata.content_type.clone();
        let parsed = serde_json::from_slice::<crate::types::ErrorResponse>(&body).ok();
        if let Some(ref error) = parsed {
            metadata = metadata.with_provider_error(error.title.clone(), error.status.to_string());
        }
        Self {
            status: Some(status),
            headers,
            metadata,
            content_type,
            body,
            body_truncated,
            parsed,
        }
    }

    /// Lossy UTF-8 preview of the retained body for logs (never assume JSON).
    pub fn body_preview(&self, max_chars: usize) -> String {
        let text = String::from_utf8_lossy(&self.body);
        if text.chars().count() <= max_chars {
            text.into_owned()
        } else {
            text.chars().take(max_chars).collect::<String>() + "…"
        }
    }
}

fn truncate_body(body: bytes::Bytes) -> (bytes::Bytes, bool) {
    if body.len() <= MAX_RETAINED_BODY_BYTES {
        (body, false)
    } else {
        (body.slice(..MAX_RETAINED_BODY_BYTES), true)
    }
}

/// Truncates response bodies retained on error paths.
pub fn truncate_body_bytes(body: bytes::Bytes) -> bytes::Bytes {
    truncate_body(body).0
}

pub(crate) fn header_str(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
}

fn first_header_str(headers: &HeaderMap, names: &[&str]) -> Option<String> {
    names.iter().find_map(|name| header_str(headers, name))
}

/// Parses `Retry-After` as delta-seconds or HTTP-date (RFC 7231).
///
/// HTTP-date values in the past yield [`Duration::ZERO`] (caller should not sleep).
/// Unparseable values are ignored.
fn parse_retry_after(headers: &HeaderMap) -> Option<Duration> {
    let raw = header_str(headers, "retry-after")?;
    if let Ok(secs) = raw.parse::<u64>() {
        return Some(Duration::from_secs(secs));
    }
    http_date_to_delay(&raw)
}

/// Parses IMF-fix / RFC 2822 HTTP-date `Retry-After` into a delay from now.
fn http_date_to_delay(raw: &str) -> Option<Duration> {
    let parsed = chrono::DateTime::parse_from_rfc2822(raw).ok()?;
    let target = parsed.with_timezone(&chrono::Utc);
    let now = chrono::Utc::now();
    let delta = target.signed_duration_since(now);
    let secs = delta.num_seconds();
    if secs <= 0 {
        Some(Duration::ZERO)
    } else {
        Some(Duration::from_secs(secs as u64))
    }
}

/// Best-effort parse of common rate-limit header shapes.
///
/// Supports discrete headers (`x-ratelimit-limit`, …) and a combined
/// `ratelimit: limit=…, remaining=…, reset=…` style when present.
fn parse_rate_limit_triplet(
    headers: &HeaderMap,
    combined: Option<&str>,
) -> (Option<u64>, Option<u64>, Option<Duration>) {
    let limit = header_u64(headers, "x-ratelimit-limit")
        .or_else(|| header_u64(headers, "ratelimit-limit"))
        .or_else(|| parse_named_u64(combined, "limit"));
    let remaining = header_u64(headers, "x-ratelimit-remaining")
        .or_else(|| header_u64(headers, "ratelimit-remaining"))
        .or_else(|| parse_named_u64(combined, "remaining"));
    let reset = header_u64(headers, "x-ratelimit-reset")
        .or_else(|| header_u64(headers, "ratelimit-reset"))
        .or_else(|| parse_named_u64(combined, "reset"))
        .map(Duration::from_secs);
    (limit, remaining, reset)
}

fn header_u64(headers: &HeaderMap, name: &str) -> Option<u64> {
    header_str(headers, name)?.parse().ok()
}

fn parse_named_u64(combined: Option<&str>, key: &str) -> Option<u64> {
    let combined = combined?;
    for part in combined.split([',', ';']) {
        let part = part.trim();
        if let Some((name, value)) = part.split_once('=') {
            if name.trim().eq_ignore_ascii_case(key) {
                return value.trim().parse().ok();
            }
        }
    }
    // IETF draft style: "10;w=1" → first integer as remaining/limit hint only for `limit`.
    if key == "limit" || key == "remaining" {
        let head = combined.split([';', ',', ' ']).next()?;
        return head.parse().ok();
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::header::HeaderValue;

    #[test]
    fn parses_rate_limit_and_retry_after_headers() {
        let mut headers = HeaderMap::new();
        headers.insert("retry-after", HeaderValue::from_static("5"));
        headers.insert("ratelimit", HeaderValue::from_static("10;w=1"));
        headers.insert("ratelimit-policy", HeaderValue::from_static("100;w=60"));
        headers.insert("x-request-id", HeaderValue::from_static("abc"));
        headers.insert("x-ratelimit-remaining", HeaderValue::from_static("3"));

        let meta =
            ResponseMetadata::from_status_and_headers(StatusCode::TOO_MANY_REQUESTS, &headers);
        assert!(meta.is_rate_limited());
        assert_eq!(meta.retry_after, Some(Duration::from_secs(5)));
        assert_eq!(meta.rate_limit.as_deref(), Some("10;w=1"));
        assert_eq!(meta.rate_limit_policy.as_deref(), Some("100;w=60"));
        assert_eq!(meta.request_id.as_deref(), Some("abc"));
        assert_eq!(meta.rate_limit_remaining, Some(3));
    }

    #[test]
    fn parses_past_http_date_retry_after_as_zero() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "retry-after",
            HeaderValue::from_static("Wed, 21 Oct 2015 07:28:00 GMT"),
        );
        let meta = ResponseMetadata::from_headers(&headers);
        assert_eq!(meta.retry_after, Some(Duration::ZERO));
    }

    #[test]
    fn parses_future_http_date_retry_after() {
        let when = chrono::Utc::now() + chrono::Duration::seconds(120);
        let raw = when.to_rfc2822();
        let mut headers = HeaderMap::new();
        headers.insert("retry-after", HeaderValue::from_str(&raw).expect("header"));
        let meta = ResponseMetadata::from_headers(&headers);
        let delay = meta.retry_after.expect("parsed");
        assert!(delay >= Duration::from_secs(100));
        assert!(delay <= Duration::from_secs(130));
    }

    #[test]
    fn ignores_garbage_retry_after() {
        let mut headers = HeaderMap::new();
        headers.insert("retry-after", HeaderValue::from_static("not-a-date"));
        let meta = ResponseMetadata::from_headers(&headers);
        assert_eq!(meta.retry_after, None);
    }

    #[test]
    fn truncates_oversized_error_body() {
        let body = bytes::Bytes::from(vec![b'x'; MAX_RETAINED_BODY_BYTES + 10]);
        let ctx = ErrorResponseContext::from_parts(StatusCode::BAD_GATEWAY, HeaderMap::new(), body);
        assert!(ctx.body_truncated);
        assert_eq!(ctx.body.len(), MAX_RETAINED_BODY_BYTES);
    }

    #[test]
    fn missing_headers_yield_empty_optional_fields() {
        let meta = ResponseMetadata::from_status_and_headers(StatusCode::OK, &HeaderMap::new());
        assert!(meta.request_id.is_none());
        assert!(meta.idempotency_key.is_none());
        assert!(meta.retry_after.is_none());
        assert!(meta.rate_limit.is_none());
    }

    #[test]
    fn malformed_numeric_rate_limit_is_ignored() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-ratelimit-limit",
            HeaderValue::from_static("not-a-number"),
        );
        let meta = ResponseMetadata::from_headers(&headers);
        assert_eq!(meta.rate_limit_limit, None);
    }
}
