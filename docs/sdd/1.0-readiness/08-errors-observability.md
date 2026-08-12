# SDD 08 — Errors & observability

## Existing
Rich `MollieError` + is_timeout/connection/cancelled/authz/retryable/rate_limited; Annotated transport context; hooks with redacted URL.

## Desired
Expose delivery outcome; distinguish is_transient vs can_retry_safely(profile, sticky); hook metadata allowlist (operation, status, request_id, attempt, latency, retry_reason)—never secrets/bodies.

## Acceptance
- [ ] Helpers documented
- [ ] Secret leak tests cover hooks
