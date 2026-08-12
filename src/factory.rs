//! Shared success and error factories for application-facing envelopes.
//!
//! Prefer these constructors when building tests, fixtures, or mapping boundary
//! JSON without calling Mollie. All API error factories produce the same
//! [`MollieErrorEnvelope`] shape via [`MollieError::to_envelope`]:
//!
//! - `ok: false`
//! - `status`, `code`, `key`, `message_key`
//! - `title`, `detail`, optional `field`, optional `documentation`
//!
//! # Global HTTP errors (Postman-harvested)
//!
//! | Status | Factory | Catalog key |
//! | --- | --- | --- |
//! | 400 | [`invalid_cursor`] | `INVALID_CURSOR` |
//! | 403 | [`demo_profile_limit_reached`], [`demo_profile_not_editable`], [`access_token_profile_restricted`] | demo / OAuth keys |
//! | 404 | [`entity_not_found`] | `ENTITY_NOT_FOUND` |
//! | 409 | [`payout_not_cancelable`], [`conflict`] | `PAYOUT_NOT_CANCELABLE` / `CONFLICT` |
//! | 410 | [`profile_deleted`], [`gone`] | `PROFILE_DELETED` / `GONE` |
//! | 422 | [`validation_error`], [`resource_state_conflict`] | `VALIDATION_ERROR` / `RESOURCE_STATE_CONFLICT` |
//! | 429 | [`rate_limit_exceeded`] | `RATE_LIMIT_EXCEEDED` |
//! | 503 | [`service_temporarily_unavailable`] | `SERVICE_TEMPORARILY_UNAVAILABLE` |
//!
//! ```rust
//! use mollie_rs::factory;
//!
//! let err = factory::rate_limit_exceeded();
//! assert!(err.is_rate_limited());
//! let envelope = err.to_envelope();
//! assert!(!envelope.ok);
//! assert_eq!(envelope.status, Some(429));
//! assert_eq!(envelope.title.as_deref(), Some("Too Many Requests"));
//! ```
//!
//! # Success
//!
//! ```rust
//! use mollie_rs::factory;
//!
//! let ok = factory::success_ok("payload");
//! assert!(ok.ok);
//! assert_eq!(ok.code, 20000);
//! ```
#![warn(missing_docs)]

pub use crate::error_catalog::{
    MollieErrorCatalogEntry, MollieErrorEnvelope, MollieErrorKey, MollieSuccessCatalogEntry,
    MollieSuccessEnvelope, MollieSuccessKey,
};
pub use crate::MollieError;

/// Creates the **global** Mollie HTTP 429 rate-limit error.
///
/// Reuse this for every route that can be rate-limited, including
/// `list_clients` (`GET /clients` / `/v2/clients`), `list_capabilities`, and
/// all other operations. There is no per-route 429 constructor.
///
/// Envelope: `title` = `"Too Many Requests"`, key `RATE_LIMIT_EXCEEDED`, code `42901`.
pub fn rate_limit_exceeded() -> MollieError {
    MollieError::rate_limit_exceeded()
}

/// Creates the known Mollie HTTP 400 invalid list-cursor error.
///
/// Catalog: `INVALID_CURSOR` / `40001`. Used when pagination `from` cursors are invalid.
pub fn invalid_cursor() -> MollieError {
    MollieError::invalid_cursor()
}

/// Creates a known Mollie HTTP 404 “no entity exists with token” error.
///
/// Catalog: `ENTITY_NOT_FOUND` / `40401`.
///
/// # Parameters
///
/// - `token` — resource id embedded in the detail string (e.g. `tr_…`, `org_…`).
pub fn entity_not_found(token: impl AsRef<str>) -> MollieError {
    MollieError::entity_not_found(token)
}

/// Creates a Mollie HTTP 422 field-validation error.
///
/// Catalog: `VALIDATION_ERROR` / `42201`. Prefer this for missing/invalid fields;
/// use [`resource_state_conflict`] for business-state failures.
///
/// # Parameters
///
/// - `detail` — human-readable Mollie-style detail (kept on the envelope)
/// - `field` — optional request field name (`field` on the envelope when set)
pub fn validation_error(detail: impl Into<String>, field: Option<&str>) -> MollieError {
    MollieError::validation_error(detail, field)
}

/// Creates a Mollie HTTP 422 resource-state conflict.
///
/// Catalog: `RESOURCE_STATE_CONFLICT` / `42202` (already deleted, cannot cancel, not allowed, …).
///
/// # Parameters
///
/// - `detail` — full Mollie detail string preserved on the envelope
pub fn resource_state_conflict(detail: impl Into<String>) -> MollieError {
    MollieError::resource_state_conflict(detail)
}

/// Creates the OAuth/org HTTP 403 for profile-restricted access tokens.
///
/// Catalog: `ACCESS_TOKEN_PROFILE_RESTRICTED` / `40301`.
/// Org-level endpoints reject tokens bound to a single profile.
pub fn access_token_profile_restricted() -> MollieError {
    MollieError::access_token_profile_restricted()
}

/// Creates the HTTP 403 demo-account profile limit error.
///
/// Catalog: `DEMO_PROFILE_LIMIT_REACHED` / `40302`.
pub fn demo_profile_limit_reached() -> MollieError {
    MollieError::demo_profile_limit_reached()
}

/// Creates the HTTP 403 demo-account profile not-editable error.
///
/// Catalog: `DEMO_PROFILE_NOT_EDITABLE` / `40303`.
pub fn demo_profile_not_editable() -> MollieError {
    MollieError::demo_profile_not_editable()
}

/// Creates the HTTP 409 payout-not-cancelable conflict.
///
/// Catalog: `PAYOUT_NOT_CANCELABLE` / `40901`.
pub fn payout_not_cancelable() -> MollieError {
    MollieError::payout_not_cancelable()
}

/// Creates a generic HTTP 409 conflict with a custom detail.
///
/// Catalog: usually `CONFLICT` / `40900` (or a more specific key if the detail matches).
///
/// # Parameters
///
/// - `detail` — human-readable conflict description
pub fn conflict(detail: impl Into<String>) -> MollieError {
    MollieError::conflict(detail)
}

/// Creates the HTTP 410 profile-deleted error.
///
/// Catalog: `PROFILE_DELETED` / `41001`.
///
/// # Parameters
///
/// - `token` — profile id embedded in the detail (e.g. `pfl_…`)
pub fn profile_deleted(token: impl AsRef<str>) -> MollieError {
    MollieError::profile_deleted(token)
}

/// Creates a generic HTTP 410 Gone error.
///
/// Catalog: `GONE` / `41000` when the detail does not match a more specific key.
///
/// # Parameters
///
/// - `detail` — human-readable gone description
pub fn gone(detail: impl Into<String>) -> MollieError {
    MollieError::gone(detail)
}

/// Creates a global HTTP 503 service-temporarily-unavailable error.
///
/// Catalog: `SERVICE_TEMPORARILY_UNAVAILABLE` / `50301`.
/// Use for transfer/verification/payment-platform temporary failures.
///
/// # Parameters
///
/// - `detail` — full Mollie detail string preserved on the envelope
pub fn service_temporarily_unavailable(detail: impl Into<String>) -> MollieError {
    MollieError::service_temporarily_unavailable(detail)
}

/// Creates a client configuration error (missing credentials, blank key, …).
///
/// Catalog: `INVALID_CONFIGURATION` / `10001` (no HTTP status).
///
/// # Parameters
///
/// - `message` — configuration failure description
pub fn invalid_configuration(message: impl Into<String>) -> MollieError {
    MollieError::invalid_configuration(message)
}

/// Builds an HTTP 200 success envelope with catalog key `OK` / code `20000`.
///
/// # Parameters
///
/// - `data` — typed success payload (same `T` as a route response body)
pub fn success_ok<T>(data: T) -> MollieSuccessEnvelope<T> {
    MollieSuccessEnvelope::ok(data)
}

/// Builds an HTTP 201 success envelope with catalog key `CREATED` / code `20100`.
///
/// # Parameters
///
/// - `data` — typed created resource body
pub fn success_created<T>(data: T) -> MollieSuccessEnvelope<T> {
    MollieSuccessEnvelope::created(data)
}

/// Builds an HTTP 202 success envelope with catalog key `ACCEPTED` / code `20200`.
///
/// # Parameters
///
/// - `data` — typed accepted payload
pub fn success_accepted<T>(data: T) -> MollieSuccessEnvelope<T> {
    MollieSuccessEnvelope::accepted(data)
}

/// Builds an HTTP 204 success envelope with catalog key `NO_CONTENT` / code `20400`.
///
/// Empty body (`data: ()`).
pub fn success_no_content() -> MollieSuccessEnvelope<()> {
    MollieSuccessEnvelope::no_content()
}
