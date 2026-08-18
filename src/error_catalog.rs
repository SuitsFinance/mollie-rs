//! Stable codes, keys, and message keys for Mollie success and error envelopes.
//!
//! # Errors
//!
//! Known Mollie HAL API failures are classified from `(status, detail)` into a
//! [`MollieErrorCatalogEntry`]. Every entry exposes:
//!
//! - a unique numeric [`MollieErrorCode`]
//! - an uppercase snake-case [`MollieErrorKey`]
//! - a dotted [`message_key`](MollieErrorCatalogEntry::message_key) for i18n
//!
//! # Success
//!
//! HTTP success statuses map to [`MollieSuccessCatalogEntry`] / [`MollieSuccessKey`]
//! for a parallel JSON envelope via [`MollieSuccessEnvelope`].
//!
//! # Examples
//!
//! ```rust
//! use mollie_rs::{types, MollieErrorCatalogEntry};
//!
//! let body = types::ErrorResponse {
//!     detail: "You have exceeded the rate limit. Please slow down your requests.".to_string(),
//!     field: None,
//!     links: types::ErrorResponseLinks {
//!         documentation: types::ErrorResponseLinksDocumentation {
//!             href: "https://docs.mollie.com/overview/handling-errors".to_string(),
//!             type_: "text/html".to_string(),
//!         },
//!     },
//!     status: 429,
//!     title: "Too Many Requests".to_string(),
//! };
//!
//! let entry = MollieErrorCatalogEntry::classify_api(&body);
//! assert_eq!(entry.code(), 42901);
//! assert_eq!(entry.key().as_str(), "RATE_LIMIT_EXCEEDED");
//! ```
#![warn(missing_docs)]

use crate::types;

/// Unique numeric code for a classified Mollie SDK error.
///
/// API errors use `HTTP_STATUS * 100 + n` (for example `40301`, `42901`).
/// Client-side errors use the `1xxxx` range.
pub type MollieErrorCode = u32;

/// Unique numeric code for a classified success response.
///
/// Success codes use `HTTP_STATUS * 100` (for example `20000`, `20100`).
pub type MollieSuccessCode = u32;

// ---------------------------------------------------------------------------
// Error keys
// ---------------------------------------------------------------------------

/// Uppercase snake-case key for a classified Mollie SDK error.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MollieErrorKey {
    /// Invalid list cursor / pagination token (HTTP 400).
    InvalidCursor,
    /// Access token is restricted to a single profile; org-level endpoint denied (OAuth).
    AccessTokenProfileRestricted,
    /// Demo account profile limit (HTTP 403).
    DemoProfileLimitReached,
    /// Demo account profile cannot be edited (HTTP 403).
    DemoProfileNotEditable,
    /// Terminal pairing not allowed for this credential/account (HTTP 403).
    TerminalPairingForbidden,
    /// Generic forbidden (HTTP 403 fallback).
    Forbidden,
    /// No entity exists for the given token/id (HTTP 404).
    EntityNotFound,
    /// Generic not found (HTTP 404 fallback).
    NotFound,
    /// Resource conflict (HTTP 409 fallback).
    Conflict,
    /// Payout cannot be canceled in its current state (HTTP 409).
    PayoutNotCancelable,
    /// Resource permanently gone (HTTP 410 fallback).
    Gone,
    /// Profile (or similar) has been deleted (HTTP 410).
    ProfileDeleted,
    /// Request validation failed (HTTP 422 field/detail family).
    ValidationError,
    /// Business-state conflict under 422 (already deleted, cannot cancel, not allowed).
    ResourceStateConflict,
    /// Generic unprocessable entity (HTTP 422 fallback).
    UnprocessableEntity,
    /// Mollie rate limit exceeded (global HTTP 429).
    RateLimitExceeded,
    /// Service temporarily unavailable (HTTP 503).
    ServiceTemporarilyUnavailable,
    /// Documented Mollie API error that has no more specific catalog entry.
    ApiError,
    /// Client configuration is invalid (missing credentials, blank token, …).
    InvalidConfiguration,
    /// Generated request failed local validation before send.
    InvalidRequest,
    /// HTTP transport failure.
    Communication,
    /// Invalid HTTP header value.
    InvalidHeaderValue,
    /// HTTP status without a parseable Mollie error body.
    UnexpectedStatus,
    /// Failed to read response body bytes.
    ResponseBody,
    /// Response body JSON did not match the expected type.
    InvalidResponsePayload,
    /// Request or response hook failure.
    Hook,
}

impl MollieErrorKey {
    /// Returns the uppercase snake-case string form of this key.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidCursor => "INVALID_CURSOR",
            Self::AccessTokenProfileRestricted => "ACCESS_TOKEN_PROFILE_RESTRICTED",
            Self::DemoProfileLimitReached => "DEMO_PROFILE_LIMIT_REACHED",
            Self::DemoProfileNotEditable => "DEMO_PROFILE_NOT_EDITABLE",
            Self::TerminalPairingForbidden => "TERMINAL_PAIRING_FORBIDDEN",
            Self::Forbidden => "FORBIDDEN",
            Self::EntityNotFound => "ENTITY_NOT_FOUND",
            Self::NotFound => "NOT_FOUND",
            Self::Conflict => "CONFLICT",
            Self::PayoutNotCancelable => "PAYOUT_NOT_CANCELABLE",
            Self::Gone => "GONE",
            Self::ProfileDeleted => "PROFILE_DELETED",
            Self::ValidationError => "VALIDATION_ERROR",
            Self::ResourceStateConflict => "RESOURCE_STATE_CONFLICT",
            Self::UnprocessableEntity => "UNPROCESSABLE_ENTITY",
            Self::RateLimitExceeded => "RATE_LIMIT_EXCEEDED",
            Self::ServiceTemporarilyUnavailable => "SERVICE_TEMPORARILY_UNAVAILABLE",
            Self::ApiError => "API_ERROR",
            Self::InvalidConfiguration => "INVALID_CONFIGURATION",
            Self::InvalidRequest => "INVALID_REQUEST",
            Self::Communication => "COMMUNICATION",
            Self::InvalidHeaderValue => "INVALID_HEADER_VALUE",
            Self::UnexpectedStatus => "UNEXPECTED_STATUS",
            Self::ResponseBody => "RESPONSE_BODY",
            Self::InvalidResponsePayload => "INVALID_RESPONSE_PAYLOAD",
            Self::Hook => "HOOK",
        }
    }
}

impl std::fmt::Display for MollieErrorKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// One catalogued error identity: code, key, and i18n message key.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MollieErrorCatalogEntry {
    code: MollieErrorCode,
    key: MollieErrorKey,
    message_key: &'static str,
}

impl MollieErrorCatalogEntry {
    /// ## `INVALID_CURSOR`
    /// Invalid cursor value (HTTP 400).
    /// - `code`: 40001
    /// - `http_status_code`: 400
    ///
    pub const INVALID_CURSOR: Self = Self {
        code: 40001,
        key: MollieErrorKey::InvalidCursor,
        message_key: "errors.bad_request.invalid_cursor",
    };

    /// ## `ACCESS_TOKEN_PROFILE_RESTRICTED`
    /// Access token restricted to a profile (HTTP 403, OAuth/org endpoints).
    /// - `code`: 40301
    /// - `http_status_code`: 403
    ///
    pub const ACCESS_TOKEN_PROFILE_RESTRICTED: Self = Self {
        code: 40301,
        key: MollieErrorKey::AccessTokenProfileRestricted,
        message_key: "errors.forbidden.access_token_profile_restricted",
    };

    /// ## `DEMO_PROFILE_LIMIT_REACHED`
    /// Demo profile limit reached (HTTP 403).
    /// - `code`: 40302
    /// - `http_status_code`: 403
    ///
    pub const DEMO_PROFILE_LIMIT_REACHED: Self = Self {
        code: 40302,
        key: MollieErrorKey::DemoProfileLimitReached,
        message_key: "errors.forbidden.demo_profile_limit_reached",
    };

    /// ## `DEMO_PROFILE_NOT_EDITABLE`
    /// Demo profile not editable (HTTP 403).
    /// - `code`: 40303
    /// - `http_status_code`: 403
    ///
    pub const DEMO_PROFILE_NOT_EDITABLE: Self = Self {
        code: 40303,
        key: MollieErrorKey::DemoProfileNotEditable,
        message_key: "errors.forbidden.demo_profile_not_editable",
    };

    /// ## `TERMINAL_PAIRING_FORBIDDEN`
    /// Terminal pairing not allowed (HTTP 403).
    /// - `code`: 40304
    /// - `http_status_code`: 403
    ///
    pub const TERMINAL_PAIRING_FORBIDDEN: Self = Self {
        code: 40304,
        key: MollieErrorKey::TerminalPairingForbidden,
        message_key: "errors.forbidden.terminal_pairing",
    };

    /// ## `FORBIDDEN`
    /// Generic forbidden (HTTP 403 fallback).
    /// - `code`: 40300
    /// - `http_status_code`: 403
    ///
    pub const FORBIDDEN: Self = Self {
        code: 40300,
        key: MollieErrorKey::Forbidden,
        message_key: "errors.forbidden.generic",
    };

    /// ## `ENTITY_NOT_FOUND`
    /// No entity exists with token (HTTP 404).
    /// - `code`: 40401
    /// - `http_status_code`: 404
    ///
    pub const ENTITY_NOT_FOUND: Self = Self {
        code: 40401,
        key: MollieErrorKey::EntityNotFound,
        message_key: "errors.not_found.entity_not_found",
    };

    /// ## `NOT_FOUND`
    /// Generic not found (HTTP 404 fallback).
    /// - `code`: 40400
    /// - `http_status_code`: 404
    ///
    pub const NOT_FOUND: Self = Self {
        code: 40400,
        key: MollieErrorKey::NotFound,
        message_key: "errors.not_found.generic",
    };

    /// ## `CONFLICT`
    /// Generic conflict (HTTP 409).
    /// - `code`: 40900
    /// - `http_status_code`: 409
    ///
    pub const CONFLICT: Self = Self {
        code: 40900,
        key: MollieErrorKey::Conflict,
        message_key: "errors.conflict.generic",
    };

    /// ## `PAYOUT_NOT_CANCELABLE`
    /// Payout not cancelable (HTTP 409).
    /// - `code`: 40901
    /// - `http_status_code`: 409
    ///
    pub const PAYOUT_NOT_CANCELABLE: Self = Self {
        code: 40901,
        key: MollieErrorKey::PayoutNotCancelable,
        message_key: "errors.conflict.payout_not_cancelable",
    };

    /// ## `GONE`
    /// Resource gone (HTTP 410).
    /// - `code`: 41000
    /// - `http_status_code`: 410
    ///
    pub const GONE: Self = Self {
        code: 41000,
        key: MollieErrorKey::Gone,
        message_key: "errors.gone.generic",
    };

    /// ## `PROFILE_DELETED`
    /// Profile deleted (HTTP 410).
    /// - `code`: 41001
    /// - `http_status_code`: 410
    ///
    pub const PROFILE_DELETED: Self = Self {
        code: 41001,
        key: MollieErrorKey::ProfileDeleted,
        message_key: "errors.gone.profile_deleted",
    };

    /// ## `UNPROCESSABLE_ENTITY`
    /// Generic unprocessable entity (HTTP 422 fallback).
    /// - `code`: 42200
    /// - `http_status_code`: 422
    ///
    pub const UNPROCESSABLE_ENTITY: Self = Self {
        code: 42200,
        key: MollieErrorKey::UnprocessableEntity,
        message_key: "errors.unprocessable_entity.generic",
    };

    /// ## `VALIDATION_ERROR`
    /// Request validation error (HTTP 422).
    /// - `code`: 42201
    /// - `http_status_code`: 422
    ///
    pub const VALIDATION_ERROR: Self = Self {
        code: 42201,
        key: MollieErrorKey::ValidationError,
        message_key: "errors.unprocessable_entity.validation_error",
    };

    /// ## `RESOURCE_STATE_CONFLICT`
    /// Resource state conflict under 422.
    /// - `code`: 42202
    /// - `http_status_code`: 422
    ///
    pub const RESOURCE_STATE_CONFLICT: Self = Self {
        code: 42202,
        key: MollieErrorKey::ResourceStateConflict,
        message_key: "errors.unprocessable_entity.resource_state_conflict",
    };

    /// ## `RATE_LIMIT_EXCEEDED`
    /// Global Mollie rate limit exceeded (HTTP 429).
    /// - `code`: 42901
    /// - `http_status_code`: 429
    ///
    pub const RATE_LIMIT_EXCEEDED: Self = Self {
        code: 42901,
        key: MollieErrorKey::RateLimitExceeded,
        message_key: "errors.too_many_requests.rate_limit_exceeded",
    };

    /// ## `SERVICE_TEMPORARILY_UNAVAILABLE`
    /// Service temporarily unavailable (HTTP 503).
    /// - `code`: 50301
    /// - `http_status_code`: 503
    ///
    pub const SERVICE_TEMPORARILY_UNAVAILABLE: Self = Self {
        code: 50301,
        key: MollieErrorKey::ServiceTemporarilyUnavailable,
        message_key: "errors.service_unavailable.temporarily_unavailable",
    };

    /// Fallback for uncatalogued Mollie API errors (base code = `status * 100`).
    pub const fn api_fallback(status: i64) -> Self {
        let status_u: u32 = if status < 0 { 0 } else { status as u32 };
        // Prefer status-specific generic keys when possible.
        match status_u {
            400 => Self {
                code: 40000,
                key: MollieErrorKey::ApiError,
                message_key: "errors.bad_request.generic",
            },
            403 => Self::FORBIDDEN,
            404 => Self::NOT_FOUND,
            409 => Self::CONFLICT,
            410 => Self::GONE,
            422 => Self::UNPROCESSABLE_ENTITY,
            429 => Self {
                code: 42900,
                key: MollieErrorKey::RateLimitExceeded,
                message_key: "errors.too_many_requests.generic",
            },
            503 => Self {
                code: 50300,
                key: MollieErrorKey::ServiceTemporarilyUnavailable,
                message_key: "errors.service_unavailable.generic",
            },
            _ => Self {
                code: status_u.saturating_mul(100),
                key: MollieErrorKey::ApiError,
                message_key: "errors.api.generic",
            },
        }
    }

    /// ## `INVALID_CONFIGURATION`
    /// Client configuration failure.
    /// - `code`: 10001
    ///
    pub const INVALID_CONFIGURATION: Self = Self {
        code: 10001,
        key: MollieErrorKey::InvalidConfiguration,
        message_key: "errors.client.invalid_configuration",
    };

    /// ## `INVALID_REQUEST`
    /// Invalid request before send.
    /// - `code`: 10002
    ///
    pub const INVALID_REQUEST: Self = Self {
        code: 10002,
        key: MollieErrorKey::InvalidRequest,
        message_key: "errors.client.invalid_request",
    };

    /// ## `COMMUNICATION`
    /// HTTP communication failure.
    /// - `code`: 10003
    ///
    pub const COMMUNICATION: Self = Self {
        code: 10003,
        key: MollieErrorKey::Communication,
        message_key: "errors.client.communication",
    };

    /// ## `INVALID_HEADER_VALUE`
    /// Invalid header value.
    /// - `code`: 10004
    ///
    pub const INVALID_HEADER_VALUE: Self = Self {
        code: 10004,
        key: MollieErrorKey::InvalidHeaderValue,
        message_key: "errors.client.invalid_header_value",
    };

    /// ## `UNEXPECTED_STATUS`
    /// Unexpected HTTP status without a Mollie error body.
    /// - `code`: 10005
    ///
    pub const UNEXPECTED_STATUS: Self = Self {
        code: 10005,
        key: MollieErrorKey::UnexpectedStatus,
        message_key: "errors.client.unexpected_status",
    };

    /// ## `RESPONSE_BODY`
    /// Failed to read response body.
    /// - `code`: 10006
    ///
    pub const RESPONSE_BODY: Self = Self {
        code: 10006,
        key: MollieErrorKey::ResponseBody,
        message_key: "errors.client.response_body",
    };

    /// ## `INVALID_RESPONSE_PAYLOAD`
    /// Failed to decode response payload.
    /// - `code`: 10007
    ///
    pub const INVALID_RESPONSE_PAYLOAD: Self = Self {
        code: 10007,
        key: MollieErrorKey::InvalidResponsePayload,
        message_key: "errors.client.invalid_response_payload",
    };

    /// ## `HOOK`
    /// Hook failure.
    /// - `code`: 10008
    ///
    pub const HOOK: Self = Self {
        code: 10008,
        key: MollieErrorKey::Hook,
        message_key: "errors.client.hook",
    };

    /// Returns the unique numeric code.
    pub const fn code(self) -> MollieErrorCode {
        self.code
    }

    /// Returns the uppercase snake-case key.
    pub const fn key(self) -> MollieErrorKey {
        self.key
    }

    /// Returns the dotted i18n message key.
    pub const fn message_key(self) -> &'static str {
        self.message_key
    }

    /// Classifies a Mollie HAL [`types::ErrorResponse`] into a catalog entry.
    ///
    /// Known detail/title patterns (from Postman harvest + live API) map to
    /// specific entries; everything else uses status-aware [`Self::api_fallback`].
    pub fn classify_api(body: &types::ErrorResponse) -> Self {
        let detail = body.detail.to_ascii_lowercase();
        let title = body.title.to_ascii_lowercase();

        if body.status == 400 && detail.contains("invalid cursor") {
            return Self::INVALID_CURSOR;
        }

        if body.status == 403 {
            if detail.contains("access token not restricted to a specific profile") {
                return Self::ACCESS_TOKEN_PROFILE_RESTRICTED;
            }
            if detail.contains("profile limit") && detail.contains("demo") {
                return Self::DEMO_PROFILE_LIMIT_REACHED;
            }
            if detail.contains("cannot be edited") && detail.contains("demo") {
                return Self::DEMO_PROFILE_NOT_EDITABLE;
            }
            // Provider-history / POS: pairing denied for account or credential.
            if detail.contains("pairing")
                || (title.contains("forbidden") && detail.contains("terminal"))
            {
                return Self::TERMINAL_PAIRING_FORBIDDEN;
            }
        }

        if body.status == 404
            && (detail.contains("no entity exists") || detail.contains("does not exist"))
        {
            return Self::ENTITY_NOT_FOUND;
        }

        if body.status == 409 {
            if detail.contains("payout")
                && (detail.contains("cannot be canceled")
                    || detail.contains("cannot be cancelled")
                    || detail.contains("current state"))
            {
                return Self::PAYOUT_NOT_CANCELABLE;
            }
            return Self::CONFLICT;
        }

        if body.status == 410 {
            if detail.contains("has been deleted") || detail.contains("profile with token") {
                return Self::PROFILE_DELETED;
            }
            return Self::GONE;
        }

        if body.status == 422 {
            // Business-state phrasing (Postman: subscription deleted, order cancel, etc.).
            if detail.contains("already deleted")
                || detail.contains("not allowed")
                || detail.contains("cannot be cancelled")
                || detail.contains("cannot be canceled")
                || detail.contains("cannot be updated")
                || detail.contains("cannot be deleted")
                || (detail.contains("none of the")
                    && (detail.contains("shipped") || detail.contains("canceled")))
            {
                return Self::RESOURCE_STATE_CONFLICT;
            }
            // Field-level and common validation phrasing from Mollie samples.
            if detail.contains("field")
                || detail.contains("missing")
                || detail.contains("invalid")
                || detail.contains("must be")
                || detail.contains("must be provided")
                || detail.contains("cannot")
                || detail.contains("amount contains")
            {
                return Self::VALIDATION_ERROR;
            }
            // Remaining 422 bodies still prefer validation family over bare API_ERROR.
            return Self::VALIDATION_ERROR;
        }

        if body.status == 429
            && (detail.contains("exceeded the rate limit") || title.contains("too many requests"))
        {
            return Self::RATE_LIMIT_EXCEEDED;
        }

        if body.status == 503
            && (detail.contains("temporarily not available")
                || detail.contains("temporarily")
                || detail.contains("unexpected error occurred")
                || detail.contains("try again later")
                || title.contains("service unavailable"))
        {
            return Self::SERVICE_TEMPORARILY_UNAVAILABLE;
        }

        Self::api_fallback(body.status)
    }
}

/// Serializable JSON error envelope shared by all [`crate::MollieError`] variants.
///
/// Status-family counterparts (`403`, `429`, …) use the **same object shape**.
/// Always includes [`Self::ok`] = `false` so success/error envelopes are parallel.
///
/// Full field documentation (not truncated):
///
/// | Field | Meaning |
/// | --- | --- |
/// | `ok` | Always `false` |
/// | `status` | HTTP status when the error came from a Mollie response |
/// | `code` | Stable numeric catalog code (e.g. `42901`) |
/// | `key` | Uppercase snake-case catalog key (e.g. `RATE_LIMIT_EXCEEDED`) |
/// | `message_key` | Dotted i18n key derived from the catalog (not a cut-down `detail`) |
/// | `title` | Mollie or SDK title (e.g. `"Too Many Requests"`) |
/// | `detail` | Full human-readable detail (complete Mollie string) |
/// | `field` | Request field name when Mollie set `field` |
/// | `documentation` | Docs URL from `_links.documentation.href` |
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct MollieErrorEnvelope {
    /// Always `false` for error envelopes.
    pub ok: bool,
    /// HTTP status when the error came from a Mollie response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<u16>,
    /// Unique numeric catalog code (for example `42901` for rate limiting).
    pub code: MollieErrorCode,
    /// Uppercase snake-case catalog key (for example `RATE_LIMIT_EXCEEDED`).
    pub key: MollieErrorKey,
    /// Dotted i18n message key (catalog mapping of Mollie `detail` when known).
    ///
    /// This is stable for translators; it is **not** a truncated copy of [`Self::detail`].
    pub message_key: &'static str,
    /// Mollie or SDK title when available (for example `"Too Many Requests"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Full human-readable detail (complete Mollie `detail` or SDK message — never truncated).
    pub detail: String,
    /// Mollie field name when the error is tied to a request field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// Documentation URL when Mollie provided `_links.documentation.href`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
}

impl MollieErrorEnvelope {
    /// Builds an envelope from a catalog entry and API error body.
    pub fn from_api(entry: MollieErrorCatalogEntry, body: &types::ErrorResponse) -> Self {
        let status = if body.status >= 0 && body.status <= u16::MAX as i64 {
            Some(body.status as u16)
        } else {
            None
        };

        Self {
            ok: false,
            status,
            code: entry.code(),
            key: entry.key(),
            message_key: entry.message_key(),
            title: Some(body.title.clone()),
            detail: body.detail.clone(),
            field: body.field.clone(),
            documentation: Some(body.links.documentation.href.clone()),
        }
    }

    /// Builds an envelope for a non-API SDK error.
    pub fn from_client(
        entry: MollieErrorCatalogEntry,
        status: Option<u16>,
        detail: impl Into<String>,
        title: Option<String>,
    ) -> Self {
        Self {
            ok: false,
            status,
            code: entry.code(),
            key: entry.key(),
            message_key: entry.message_key(),
            title,
            detail: detail.into(),
            field: None,
            documentation: None,
        }
    }
}

// ---------------------------------------------------------------------------
// Success keys / envelope
// ---------------------------------------------------------------------------

/// Uppercase snake-case key for a classified success response.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MollieSuccessKey {
    /// HTTP 200 OK.
    Ok,
    /// HTTP 201 Created.
    Created,
    /// HTTP 202 Accepted.
    Accepted,
    /// HTTP 204 No Content.
    NoContent,
    /// Other 2xx success.
    Success,
}

impl MollieSuccessKey {
    /// Returns the uppercase snake-case string form of this key.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ok => "OK",
            Self::Created => "CREATED",
            Self::Accepted => "ACCEPTED",
            Self::NoContent => "NO_CONTENT",
            Self::Success => "SUCCESS",
        }
    }
}

impl std::fmt::Display for MollieSuccessKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// One catalogued success identity: code, key, and i18n message key.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MollieSuccessCatalogEntry {
    code: MollieSuccessCode,
    key: MollieSuccessKey,
    message_key: &'static str,
}

impl MollieSuccessCatalogEntry {
    /// ## `OK`
    /// HTTP 200.
    /// - `code`: 20000
    ///
    pub const OK: Self = Self {
        code: 20000,
        key: MollieSuccessKey::Ok,
        message_key: "success.ok",
    };

    /// ## `CREATED`
    /// HTTP 201.
    /// - `code`: 20100
    ///
    pub const CREATED: Self = Self {
        code: 20100,
        key: MollieSuccessKey::Created,
        message_key: "success.created",
    };

    /// ## `ACCEPTED`
    /// HTTP 202.
    /// - `code`: 20200
    ///
    pub const ACCEPTED: Self = Self {
        code: 20200,
        key: MollieSuccessKey::Accepted,
        message_key: "success.accepted",
    };

    /// ## `NO_CONTENT`
    /// HTTP 204.
    /// - `code`: 20400
    ///
    pub const NO_CONTENT: Self = Self {
        code: 20400,
        key: MollieSuccessKey::NoContent,
        message_key: "success.no_content",
    };

    /// Other 2xx.
    pub const fn from_status(status: u16) -> Self {
        match status {
            200 => Self::OK,
            201 => Self::CREATED,
            202 => Self::ACCEPTED,
            204 => Self::NO_CONTENT,
            other => Self {
                code: (other as u32).saturating_mul(100),
                key: MollieSuccessKey::Success,
                message_key: "success.generic",
            },
        }
    }

    /// Returns the unique numeric code.
    pub const fn code(self) -> MollieSuccessCode {
        self.code
    }

    /// Returns the uppercase snake-case key.
    pub const fn key(self) -> MollieSuccessKey {
        self.key
    }

    /// Returns the dotted i18n message key.
    pub const fn message_key(self) -> &'static str {
        self.message_key
    }
}

/// Serializable JSON success envelope for a typed route body.
///
/// Parallel to [`MollieErrorEnvelope`]: always includes [`Self::ok`] = `true`.
///
/// | Field | Meaning |
/// | --- | --- |
/// | `ok` | Always `true` |
/// | `status` | HTTP status (`200` / `201` / `202` / `204` / …) |
/// | `code` | Catalog code (`20000`, `20100`, …) |
/// | `key` | `OK` / `CREATED` / `ACCEPTED` / `NO_CONTENT` / `SUCCESS` |
/// | `message_key` | e.g. `success.ok` |
/// | `data` | Full typed response body (not truncated) |
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct MollieSuccessEnvelope<T> {
    /// Always `true` for success envelopes.
    pub ok: bool,
    /// HTTP status of the Mollie response.
    pub status: u16,
    /// Unique numeric catalog code (for example `20000` for OK).
    pub code: MollieSuccessCode,
    /// Uppercase snake-case catalog key (for example `OK`).
    pub key: MollieSuccessKey,
    /// Dotted i18n message key (for example `success.ok`).
    pub message_key: &'static str,
    /// Typed response body (same `T` as [`crate::ResponseEnvelope`]; full payload).
    pub data: T,
}

impl<T> MollieSuccessEnvelope<T> {
    /// Builds a success envelope from HTTP status and data.
    /// - `status`: u16
    /// - `data`: T
    ///
    /// ## Returns
    /// - `Self`
    pub fn from_status_data(status: u16, data: T) -> Self {
        let entry: MollieSuccessCatalogEntry = MollieSuccessCatalogEntry::from_status(status);
        Self {
            ok: true,
            status,
            code: entry.code(),
            key: entry.key(),
            message_key: entry.message_key(),
            data,
        }
    }

    /// HTTP 200 factory.
    pub fn ok(data: T) -> Self {
        Self::from_status_data(200, data)
    }

    /// HTTP 201 factory.
    pub fn created(data: T) -> Self {
        Self::from_status_data(201, data)
    }

    /// HTTP 202 factory.
    pub fn accepted(data: T) -> Self {
        Self::from_status_data(202, data)
    }

    /// Consumes the envelope and returns the typed body.
    pub fn into_data(self) -> T {
        self.data
    }

    /// Returns a reference to the typed body.
    pub const fn data(&self) -> &T {
        &self.data
    }

    /// Maps the body while preserving success metadata.
    pub fn map<U>(self, map: impl FnOnce(T) -> U) -> MollieSuccessEnvelope<U> {
        MollieSuccessEnvelope {
            ok: self.ok,
            status: self.status,
            code: self.code,
            key: self.key,
            message_key: self.message_key,
            data: map(self.data),
        }
    }
}

impl MollieSuccessEnvelope<()> {
    /// HTTP 204 factory (empty body).
    pub fn no_content() -> Self {
        Self::from_status_data(204, ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_links(href: &str) -> types::ErrorResponseLinks {
        types::ErrorResponseLinks {
            documentation: types::ErrorResponseLinksDocumentation {
                href: href.to_string(),
                type_: "text/html".to_string(),
            },
        }
    }

    #[test]
    fn classifies_access_token_profile_restricted() {
        let body = types::ErrorResponse {
            detail: "This API endpoint is only available with an access token not restricted to a specific profile.".to_string(),
            field: None,
            links: sample_links("https://docs.mollie.com/reference/handling-errors"),
            status: 403,
            title: "Forbidden".to_string(),
        };

        let entry = MollieErrorCatalogEntry::classify_api(&body);
        assert_eq!(
            entry,
            MollieErrorCatalogEntry::ACCESS_TOKEN_PROFILE_RESTRICTED
        );
        assert_eq!(entry.code(), 40301);
    }

    #[test]
    fn classifies_terminal_pairing_forbidden_from_fixture_detail() {
        let raw = include_str!("../tests/fixtures/provider_history/terminal_pairing_403.json");
        let v: serde_json::Value = serde_json::from_str(raw).expect("fixture json");
        let body = types::ErrorResponse {
            detail: v["detail"].as_str().unwrap().to_string(),
            field: None,
            links: sample_links("https://docs.mollie.com/errors"),
            status: v["status"].as_u64().unwrap() as i64,
            title: v["title"].as_str().unwrap().to_string(),
        };
        let entry = MollieErrorCatalogEntry::classify_api(&body);
        assert_eq!(entry, MollieErrorCatalogEntry::TERMINAL_PAIRING_FORBIDDEN);
        assert_eq!(entry.code(), 40304);
        assert_eq!(entry.key(), MollieErrorKey::TerminalPairingForbidden);
        assert_ne!(entry, MollieErrorCatalogEntry::RATE_LIMIT_EXCEEDED);
        assert_ne!(
            entry,
            MollieErrorCatalogEntry::ACCESS_TOKEN_PROFILE_RESTRICTED
        );
        assert_ne!(entry.key(), MollieErrorKey::Forbidden);
    }

    #[test]
    fn classifies_rate_limit_exceeded() {
        let body = types::ErrorResponse {
            detail: "You have exceeded the rate limit. Please slow down your requests.".to_string(),
            field: None,
            links: sample_links("https://docs.mollie.com/overview/handling-errors"),
            status: 429,
            title: "Too Many Requests".to_string(),
        };

        let entry = MollieErrorCatalogEntry::classify_api(&body);
        assert_eq!(entry, MollieErrorCatalogEntry::RATE_LIMIT_EXCEEDED);
        assert_eq!(entry.code(), 42901);
    }

    #[test]
    fn classifies_invalid_cursor() {
        let body = types::ErrorResponse {
            detail: "Invalid cursor value".to_string(),
            field: None,
            links: sample_links("https://docs.mollie.com/errors"),
            status: 400,
            title: "Bad Request".to_string(),
        };
        assert_eq!(
            MollieErrorCatalogEntry::classify_api(&body).key(),
            MollieErrorKey::InvalidCursor
        );
    }

    #[test]
    fn classifies_entity_not_found() {
        let body = types::ErrorResponse {
            detail: "No entity exists with token 'uct_abcDEFghij123456789'".to_string(),
            field: None,
            links: sample_links("https://docs.mollie.com/errors"),
            status: 404,
            title: "Not Found".to_string(),
        };
        assert_eq!(
            MollieErrorCatalogEntry::classify_api(&body).key(),
            MollieErrorKey::EntityNotFound
        );
    }

    #[test]
    fn classifies_validation_error() {
        let body = types::ErrorResponse {
            detail: "The 'description' field is missing".to_string(),
            field: Some("description".to_string()),
            links: sample_links("https://docs.mollie.com/errors"),
            status: 422,
            title: "Unprocessable Entity".to_string(),
        };
        let entry = MollieErrorCatalogEntry::classify_api(&body);
        assert_eq!(entry.key(), MollieErrorKey::ValidationError);
        assert_eq!(entry.code(), 42201);
    }

    #[test]
    fn error_envelope_has_ok_false() {
        let body = types::ErrorResponse {
            detail: "You have exceeded the rate limit. Please slow down your requests.".to_string(),
            field: None,
            links: sample_links("https://docs.mollie.com/overview/handling-errors"),
            status: 429,
            title: "Too Many Requests".to_string(),
        };
        let envelope =
            MollieErrorEnvelope::from_api(MollieErrorCatalogEntry::RATE_LIMIT_EXCEEDED, &body);
        assert!(!envelope.ok);
        assert_eq!(envelope.status, Some(429));
    }

    #[test]
    fn success_envelope_from_status() {
        let envelope = MollieSuccessEnvelope::created("payment");
        assert!(envelope.ok);
        assert_eq!(envelope.status, 201);
        assert_eq!(envelope.code, 20100);
        assert_eq!(envelope.key.as_str(), "CREATED");
        assert_eq!(envelope.data, "payment");
    }

    #[test]
    fn unknown_api_error_uses_status_fallback() {
        let body = types::ErrorResponse {
            detail: "Something else went wrong.".to_string(),
            field: None,
            links: sample_links("https://docs.mollie.com/errors"),
            status: 418,
            title: "I'm a teapot".to_string(),
        };

        let entry = MollieErrorCatalogEntry::classify_api(&body);
        assert_eq!(entry.key(), MollieErrorKey::ApiError);
        assert_eq!(entry.code(), 41800);
    }
}
