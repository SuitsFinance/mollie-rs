//! Error types returned by the hand-written Mollie SDK facade.
//!
//! Generated route methods return [`progenitor_client::Error`]. The facade
//! converts those values into [`MollieError`] so applications can match one
//! error family across configuration, transport, payload, and API failures.
//!
//! # Examples
//!
//! ```rust
//! use mollie_rs::{MollieClient, MollieError};
//!
//! let error = MollieClient::from_api_key("");
//! assert!(matches!(error, Err(MollieError::InvalidConfiguration { .. })));
//! ```
#![warn(missing_docs)]

use std::time::Duration;

use bytes::Bytes;
use reqwest::{header::HeaderMap, StatusCode};

use crate::env::{
    MOLLIE_API_KEY_ENV, MOLLIE_OAUTH_ACCESS_TOKEN_ENV, MOLLIE_OAUTH_CLIENT_ID_ENV,
    MOLLIE_OAUTH_CLIENT_SECRET_ENV,
};
use crate::error_catalog::{MollieErrorCatalogEntry, MollieErrorEnvelope, MollieErrorKey};
use crate::metadata::{truncate_body_bytes, ErrorResponseContext, ResponseMetadata};
use crate::types::{ErrorResponse, ErrorResponseLinks, ErrorResponseLinksDocumentation};
use crate::webhook_verify::WebhookVerifyFailure;

/// Result alias used by the ergonomic Mollie facade.
pub type MollieResult<T> = Result<T, MollieError>;

/// A crate-owned error family for configuration, HTTP, payload, and API errors.
#[derive(Debug, thiserror::Error)]
pub enum MollieError {
    /// The client could not be configured before any request was sent.
    #[error("invalid Mollie client configuration: {message}")]
    InvalidConfiguration {
        /// Human-readable configuration failure.
        message: String,
    },

    /// A generated request failed local validation before it was sent.
    #[error("invalid Mollie request: {0}")]
    InvalidRequest(String),

    /// The HTTP client failed to build or send a request.
    #[error("Mollie HTTP communication failed: {0}")]
    Communication(#[from] reqwest::Error),

    /// A header value could not be encoded for an HTTP request.
    #[error("invalid Mollie HTTP header value: {0}")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),

    /// Mollie returned a documented API error response.
    #[error("Mollie API returned status {status}")]
    Api {
        /// HTTP status returned by Mollie.
        status: StatusCode,
        /// Response headers returned by Mollie.
        headers: HeaderMap,
        /// Typed Mollie error response body.
        body: Box<ErrorResponse>,
    },

    /// Mollie returned a status code that the OpenAPI contract did not list
    /// for the generated operation.
    #[error("Mollie API returned undocumented status {status}")]
    UnexpectedStatus {
        /// HTTP status returned by Mollie.
        status: StatusCode,
    },

    /// The response body could not be read.
    #[error("failed to read Mollie response body: {0}")]
    ResponseBody(#[source] reqwest::Error),

    /// A documented response body did not match the generated response type.
    #[error("failed to decode Mollie response payload: {source}")]
    InvalidResponsePayload {
        /// Raw bytes returned by the response (bounded).
        bytes: Bytes,
        /// JSON decoding error.
        source: serde_json::Error,
    },

    /// The provider returned a non-success response that could not be decoded as
    /// Mollie's HAL error document (HTML gateway pages, empty bodies, etc.).
    #[error("malformed Mollie provider response")]
    MalformedProviderResponse {
        /// Bounded status/headers/body context for diagnostics.
        context: Box<ErrorResponseContext>,
    },

    /// Next-gen webhook signature or payload verification failed.
    #[error("Mollie webhook verification failed: {failure}")]
    WebhookVerification {
        /// Structured verification failure.
        failure: WebhookVerifyFailure,
    },

    /// A request or response hook returned an error.
    #[error("Mollie client hook failed: {0}")]
    Hook(String),

    /// An error annotated with transport operation / attempt metadata.
    ///
    /// Prefer constructing via [`MollieError::with_transport_context`]. Nested
    /// annotations peel to the innermost source for status/body matching.
    #[error("{source}")]
    Annotated {
        /// Underlying error.
        #[source]
        source: Box<MollieError>,
        /// OpenAPI operation id when known.
        operation_id: Option<&'static str>,
        /// 1-based attempt number when known.
        attempt: Option<u32>,
        /// HTTP method when known.
        method: Option<&'static str>,
    },
}

impl MollieError {
    /// Creates an invalid-configuration error with a message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::MollieError;
    ///
    /// let error = MollieError::invalid_configuration("missing token");
    /// assert!(matches!(error, MollieError::InvalidConfiguration { .. }));
    /// ```
    pub fn invalid_configuration(message: impl Into<String>) -> Self {
        Self::InvalidConfiguration {
            message: message.into(),
        }
    }

    /// Creates a configuration error for a missing process environment variable.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::MollieError;
    ///
    /// let error = MollieError::missing_env_var("MOLLIE_API_KEY");
    /// assert!(error.is_missing_env_var("MOLLIE_API_KEY"));
    /// ```
    pub fn missing_env_var(key: impl AsRef<str>) -> Self {
        Self::invalid_configuration(format!("missing environment variable `{}`", key.as_ref()))
    }

    /// Creates a configuration error when an environment variable is not valid UTF-8.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::MollieError;
    ///
    /// let error = MollieError::invalid_env_var_encoding("MOLLIE_API_KEY");
    /// assert!(matches!(error, MollieError::InvalidConfiguration { .. }));
    /// ```
    pub fn invalid_env_var_encoding(key: impl AsRef<str>) -> Self {
        Self::invalid_configuration(format!(
            "environment variable `{}` is not valid UTF-8",
            key.as_ref()
        ))
    }

    /// Creates a configuration error when neither Mollie credential env var is set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::MollieError;
    ///
    /// let error = MollieError::missing_mollie_credentials();
    /// assert!(error.is_missing_mollie_credentials());
    /// ```
    pub fn missing_mollie_credentials() -> Self {
        Self::invalid_configuration(format!(
            "missing environment variable `{MOLLIE_API_KEY_ENV}`, `{MOLLIE_OAUTH_ACCESS_TOKEN_ENV}`, or both `{MOLLIE_OAUTH_CLIENT_ID_ENV}` and `{MOLLIE_OAUTH_CLIENT_SECRET_ENV}`"
        ))
    }

    /// Returns `true` when this error is a missing process environment variable.
    pub fn is_missing_env_var(&self, key: &str) -> bool {
        matches!(
            self,
            Self::InvalidConfiguration { message }
                if message == &format!("missing environment variable `{key}`")
        )
    }

    /// Returns `true` when neither Mollie credential environment variable was set.
    pub fn is_missing_mollie_credentials(&self) -> bool {
        matches!(
            self,
            Self::InvalidConfiguration { message }
                if message
                    == &format!(
                        "missing environment variable `{MOLLIE_API_KEY_ENV}`, `{MOLLIE_OAUTH_ACCESS_TOKEN_ENV}`, or both `{MOLLIE_OAUTH_CLIENT_ID_ENV}` and `{MOLLIE_OAUTH_CLIENT_SECRET_ENV}`"
                    )
        )
    }

    /// Creates an invalid-request error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::MollieError;
    ///
    /// let error = MollieError::invalid_request("missing amount");
    /// assert!(matches!(error, MollieError::InvalidRequest(_)));
    /// ```
    pub fn invalid_request(message: impl Into<String>) -> Self {
        Self::InvalidRequest(message.into())
    }

    /// Creates a documented Mollie API error from response parts.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::{
    ///     types::{ErrorResponse, ErrorResponseLinks, ErrorResponseLinksDocumentation},
    ///     MollieError,
    /// };
    /// use reqwest::StatusCode;
    ///
    /// let body = ErrorResponse {
    ///     detail: "The payment id is invalid.".to_string(),
    ///     field: Some("paymentId".to_string()),
    ///     links: ErrorResponseLinks {
    ///         documentation: ErrorResponseLinksDocumentation {
    ///             href: "https://docs.mollie.com/errors".to_string(),
    ///             type_: "text/html".to_string(),
    ///         },
    ///     },
    ///     status: 422,
    ///     title: "Unprocessable Entity".to_string(),
    /// };
    ///
    /// let error = MollieError::api(StatusCode::UNPROCESSABLE_ENTITY, Default::default(), body);
    /// assert_eq!(error.status(), Some(StatusCode::UNPROCESSABLE_ENTITY));
    /// ```
    pub fn api(status: StatusCode, headers: HeaderMap, body: ErrorResponse) -> Self {
        Self::Api {
            status,
            headers,
            body: Box::new(body),
        }
    }

    /// Creates the known Mollie 403 for profile-restricted access tokens.
    ///
    /// Org-level endpoints such as `list_capabilities` reject tokens that are
    /// restricted to a single profile.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::MollieError;
    /// use reqwest::StatusCode;
    ///
    /// let error = MollieError::access_token_profile_restricted();
    /// assert_eq!(error.status(), Some(StatusCode::FORBIDDEN));
    /// assert_eq!(
    ///     error.catalog_entry().key().as_str(),
    ///     "ACCESS_TOKEN_PROFILE_RESTRICTED"
    /// );
    /// ```
    pub fn access_token_profile_restricted() -> Self {
        Self::api(
            StatusCode::FORBIDDEN,
            HeaderMap::new(),
            ErrorResponse {
                detail: "This API endpoint is only available with an access token not restricted to a specific profile.".to_string(),
                field: None,
                links: ErrorResponseLinks {
                    documentation: ErrorResponseLinksDocumentation {
                        href: "https://docs.mollie.com/reference/handling-errors".to_string(),
                        type_: "text/html".to_string(),
                    },
                },
                status: 403,
                title: "Forbidden".to_string(),
            },
        )
    }

    /// Creates the known Mollie 403 when terminal pairing is not allowed.
    ///
    /// Distinct from rate-limit (429), auth failure (401), and profile-restricted
    /// OAuth tokens so POS callers can branch without string-matching transport noise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::{MollieError, MollieErrorKey};
    /// use reqwest::StatusCode;
    ///
    /// let error = MollieError::terminal_pairing_forbidden();
    /// assert_eq!(error.status(), Some(StatusCode::FORBIDDEN));
    /// assert_eq!(
    ///     error.catalog_entry().key(),
    ///     MollieErrorKey::TerminalPairingForbidden
    /// );
    /// ```
    pub fn terminal_pairing_forbidden() -> Self {
        Self::api(
            StatusCode::FORBIDDEN,
            HeaderMap::new(),
            ErrorResponse {
                detail: "Pairing not allowed".to_string(),
                field: None,
                links: ErrorResponseLinks {
                    documentation: ErrorResponseLinksDocumentation {
                        href: "https://docs.mollie.com/reference/handling-errors".to_string(),
                        type_: "text/html".to_string(),
                    },
                },
                status: 403,
                title: "Forbidden".to_string(),
            },
        )
    }

    /// Creates the known **global** Mollie 429 rate-limit error.
    ///
    /// Applies to every route (including `list_capabilities`) when Mollie
    /// rate-limits the credential. Envelope title is always
    /// `"Too Many Requests"`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::MollieError;
    /// use reqwest::StatusCode;
    ///
    /// let error = MollieError::rate_limit_exceeded();
    /// assert_eq!(error.status(), Some(StatusCode::TOO_MANY_REQUESTS));
    /// assert_eq!(error.catalog_entry().key().as_str(), "RATE_LIMIT_EXCEEDED");
    /// assert_eq!(
    ///     error.to_envelope().title.as_deref(),
    ///     Some("Too Many Requests")
    /// );
    /// ```
    pub fn rate_limit_exceeded() -> Self {
        Self::api(
            StatusCode::TOO_MANY_REQUESTS,
            HeaderMap::new(),
            ErrorResponse {
                detail: "You have exceeded the rate limit. Please slow down your requests."
                    .to_string(),
                field: None,
                links: ErrorResponseLinks {
                    documentation: ErrorResponseLinksDocumentation {
                        href: "https://docs.mollie.com/overview/handling-errors".to_string(),
                        type_: "text/html".to_string(),
                    },
                },
                status: 429,
                title: "Too Many Requests".to_string(),
            },
        )
    }

    /// Creates a known Mollie 400 invalid-cursor error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::{MollieError, MollieErrorKey};
    ///
    /// let error = MollieError::invalid_cursor();
    /// assert_eq!(error.catalog_entry().key(), MollieErrorKey::InvalidCursor);
    /// ```
    pub fn invalid_cursor() -> Self {
        Self::api(
            StatusCode::BAD_REQUEST,
            HeaderMap::new(),
            ErrorResponse {
                detail: "Invalid cursor value".to_string(),
                field: None,
                links: ErrorResponseLinks {
                    documentation: ErrorResponseLinksDocumentation {
                        href: "https://docs.mollie.com/overview/handling-errors".to_string(),
                        type_: "text/html".to_string(),
                    },
                },
                status: 400,
                title: "Bad Request".to_string(),
            },
        )
    }

    /// Creates a known Mollie 404 entity-not-found error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::{MollieError, MollieErrorKey};
    ///
    /// let error = MollieError::entity_not_found("tr_xxx");
    /// assert_eq!(error.catalog_entry().key(), MollieErrorKey::EntityNotFound);
    /// ```
    pub fn entity_not_found(token: impl AsRef<str>) -> Self {
        let token = token.as_ref();
        Self::api(
            StatusCode::NOT_FOUND,
            HeaderMap::new(),
            ErrorResponse {
                detail: format!("No entity exists with token '{token}'"),
                field: None,
                links: ErrorResponseLinks {
                    documentation: ErrorResponseLinksDocumentation {
                        href: "https://docs.mollie.com/overview/handling-errors".to_string(),
                        type_: "text/html".to_string(),
                    },
                },
                status: 404,
                title: "Not Found".to_string(),
            },
        )
    }

    /// Creates a Mollie 422 validation error with optional field name.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::{MollieError, MollieErrorKey};
    ///
    /// let error = MollieError::validation_error("The 'description' field is missing", Some("description"));
    /// assert_eq!(error.catalog_entry().key(), MollieErrorKey::ValidationError);
    /// ```
    pub fn validation_error(detail: impl Into<String>, field: Option<&str>) -> Self {
        Self::api(
            StatusCode::UNPROCESSABLE_ENTITY,
            HeaderMap::new(),
            ErrorResponse {
                detail: detail.into(),
                field: field.map(str::to_string),
                links: Self::docs_links("https://docs.mollie.com/overview/handling-errors"),
                status: 422,
                title: "Unprocessable Entity".to_string(),
            },
        )
    }

    /// Creates a 422 resource-state conflict (already deleted, cannot cancel, …).
    ///
    /// Catalog: [`MollieErrorCatalogEntry::RESOURCE_STATE_CONFLICT`] (`42202`).
    /// Envelope title is `"Unprocessable Entity"`; full `detail` is preserved.
    pub fn resource_state_conflict(detail: impl Into<String>) -> Self {
        Self::api(
            StatusCode::UNPROCESSABLE_ENTITY,
            HeaderMap::new(),
            ErrorResponse {
                detail: detail.into(),
                field: None,
                links: Self::docs_links("https://docs.mollie.com/overview/handling-errors"),
                status: 422,
                title: "Unprocessable Entity".to_string(),
            },
        )
    }

    /// Creates a 403 demo profile limit error.
    ///
    /// Catalog: [`MollieErrorCatalogEntry::DEMO_PROFILE_LIMIT_REACHED`] (`40302`).
    /// Detail: `"Profile limit has been reached for demo accounts."`.
    pub fn demo_profile_limit_reached() -> Self {
        Self::api(
            StatusCode::FORBIDDEN,
            HeaderMap::new(),
            ErrorResponse {
                detail: "Profile limit has been reached for demo accounts.".to_string(),
                field: None,
                links: Self::docs_links("https://docs.mollie.com/overview/handling-errors"),
                status: 403,
                title: "Forbidden".to_string(),
            },
        )
    }

    /// Creates a 403 demo profile not-editable error.
    ///
    /// Catalog: [`MollieErrorCatalogEntry::DEMO_PROFILE_NOT_EDITABLE`] (`40303`).
    pub fn demo_profile_not_editable() -> Self {
        Self::api(
            StatusCode::FORBIDDEN,
            HeaderMap::new(),
            ErrorResponse {
                detail: "This profile cannot be edited because it belongs to a demo account."
                    .to_string(),
                field: None,
                links: Self::docs_links("https://docs.mollie.com/overview/handling-errors"),
                status: 403,
                title: "Forbidden".to_string(),
            },
        )
    }

    /// Creates a 409 payout-not-cancelable conflict.
    ///
    /// Catalog: [`MollieErrorCatalogEntry::PAYOUT_NOT_CANCELABLE`] (`40901`).
    pub fn payout_not_cancelable() -> Self {
        Self::api(
            StatusCode::CONFLICT,
            HeaderMap::new(),
            ErrorResponse {
                detail: "The payout cannot be canceled in its current state.".to_string(),
                field: None,
                links: Self::docs_links("https://docs.mollie.com/errors"),
                status: 409,
                title: "Conflict".to_string(),
            },
        )
    }

    /// Creates a generic 409 conflict with a custom detail.
    ///
    /// Catalog falls back to [`MollieErrorCatalogEntry::CONFLICT`] (`40900`)
    /// unless the detail matches a more specific pattern.
    pub fn conflict(detail: impl Into<String>) -> Self {
        Self::api(
            StatusCode::CONFLICT,
            HeaderMap::new(),
            ErrorResponse {
                detail: detail.into(),
                field: None,
                links: Self::docs_links("https://docs.mollie.com/errors"),
                status: 409,
                title: "Conflict".to_string(),
            },
        )
    }

    /// Creates a 410 profile-deleted error.
    ///
    /// Catalog: [`MollieErrorCatalogEntry::PROFILE_DELETED`] (`41001`).
    pub fn profile_deleted(token: impl AsRef<str>) -> Self {
        let token = token.as_ref();
        Self::api(
            StatusCode::GONE,
            HeaderMap::new(),
            ErrorResponse {
                detail: format!("Profile with token {token} has been deleted."),
                field: None,
                links: Self::docs_links("https://docs.mollie.com/overview/handling-errors"),
                status: 410,
                title: "Gone".to_string(),
            },
        )
    }

    /// Creates a generic 410 gone error.
    ///
    /// Catalog: [`MollieErrorCatalogEntry::GONE`] (`41000`) unless detail matches
    /// a more specific key such as profile deleted.
    pub fn gone(detail: impl Into<String>) -> Self {
        Self::api(
            StatusCode::GONE,
            HeaderMap::new(),
            ErrorResponse {
                detail: detail.into(),
                field: None,
                links: Self::docs_links("https://docs.mollie.com/overview/handling-errors"),
                status: 410,
                title: "Gone".to_string(),
            },
        )
    }

    /// Creates a global 503 service temporarily unavailable error.
    ///
    /// Catalog: [`MollieErrorCatalogEntry::SERVICE_TEMPORARILY_UNAVAILABLE`] (`50301`).
    /// Envelope title is `"Service Unavailable"`; pass the full Mollie `detail`.
    pub fn service_temporarily_unavailable(detail: impl Into<String>) -> Self {
        Self::api(
            StatusCode::SERVICE_UNAVAILABLE,
            HeaderMap::new(),
            ErrorResponse {
                detail: detail.into(),
                field: None,
                links: Self::docs_links("https://docs.mollie.com/overview/handling-errors"),
                status: 503,
                title: "Service Unavailable".to_string(),
            },
        )
    }

    /// Builds Mollie HAL `_links.documentation` for factory API errors.
    fn docs_links(href: &str) -> ErrorResponseLinks {
        ErrorResponseLinks {
            documentation: ErrorResponseLinksDocumentation {
                href: href.to_string(),
                type_: "text/html".to_string(),
            },
        }
    }

    /// Creates an undocumented-status error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::MollieError;
    /// use reqwest::StatusCode;
    ///
    /// let error = MollieError::unexpected_status(StatusCode::IM_A_TEAPOT);
    /// assert_eq!(error.status(), Some(StatusCode::IM_A_TEAPOT));
    /// ```
    pub const fn unexpected_status(status: StatusCode) -> Self {
        Self::UnexpectedStatus { status }
    }

    /// Creates a response-payload decode error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bytes::Bytes;
    /// use mollie_rs::MollieError;
    ///
    /// let source = serde_json::from_slice::<serde_json::Value>(b"{").unwrap_err();
    /// let error = MollieError::invalid_response_payload(Bytes::from_static(b"{"), source);
    /// assert!(matches!(error, MollieError::InvalidResponsePayload { .. }));
    /// ```
    pub fn invalid_response_payload(bytes: Bytes, source: serde_json::Error) -> Self {
        Self::InvalidResponsePayload { bytes, source }
    }

    /// Creates a hook error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::MollieError;
    ///
    /// let error = MollieError::hook("request hook rejected the request");
    /// assert!(matches!(error, MollieError::Hook(_)));
    /// ```
    pub fn hook(message: impl Into<String>) -> Self {
        Self::Hook(message.into())
    }

    /// Creates a webhook verification error.
    pub fn webhook_verification(failure: WebhookVerifyFailure) -> Self {
        Self::WebhookVerification { failure }
    }

    /// Creates a malformed-provider-response error with bounded context.
    pub fn malformed_provider_response(context: ErrorResponseContext) -> Self {
        Self::MalformedProviderResponse {
            context: Box::new(context),
        }
    }

    /// Returns the HTTP status code when the error came from a response.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::MollieError;
    ///
    /// assert_eq!(MollieError::invalid_configuration("missing token").status(), None);
    /// ```
    pub fn status(&self) -> Option<StatusCode> {
        match self {
            Self::Annotated { source, .. } => source.status(),
            Self::Api { status, .. } | Self::UnexpectedStatus { status } => Some(*status),
            Self::MalformedProviderResponse { context } => context.status,
            Self::Communication(error) => error.status(),
            Self::InvalidConfiguration { .. }
            | Self::InvalidRequest(_)
            | Self::InvalidHeaderValue(_)
            | Self::ResponseBody(_)
            | Self::InvalidResponsePayload { .. }
            | Self::WebhookVerification { .. }
            | Self::Hook(_) => None,
        }
    }

    /// Returns operational metadata when this error carries headers/status.
    ///
    /// When this error was annotated with [`Self::with_transport_context`],
    /// `operation_id` and `attempt` are filled even if the provider response
    /// alone did not carry them.
    pub fn metadata(&self) -> ResponseMetadata {
        let (inner, operation_id, attempt) = self.transport_annotation();
        let mut meta = match inner {
            Self::Api {
                status,
                headers,
                body,
            } => ResponseMetadata::from_status_and_headers(*status, headers)
                .with_provider_error(body.title.clone(), body.status.to_string()),
            Self::MalformedProviderResponse { context } => context.metadata.clone(),
            Self::UnexpectedStatus { status } => ResponseMetadata {
                status: Some(*status),
                ..ResponseMetadata::default()
            },
            _ => ResponseMetadata::default(),
        };
        if meta.operation_id.is_none() {
            meta.operation_id = operation_id;
        }
        if meta.attempt.is_none() {
            meta.attempt = attempt;
        }
        meta
    }

    /// Attaches transport operation / attempt / method context for incident
    /// analysis and reconciliation.
    ///
    /// Does not alter the underlying failure class. Nested annotations keep the
    /// outermost non-`None` fields.
    pub fn with_transport_context(
        self,
        operation_id: &'static str,
        attempt: u32,
        method: Option<&'static str>,
    ) -> Self {
        Self::Annotated {
            source: Box::new(self),
            operation_id: Some(operation_id),
            attempt: Some(attempt),
            method,
        }
    }

    /// Peels annotated wrappers and returns the source error plus transport fields.
    fn transport_annotation(&self) -> (&Self, Option<&'static str>, Option<u32>) {
        match self {
            Self::Annotated {
                source,
                operation_id,
                attempt,
                ..
            } => {
                let (inner, inner_op, inner_attempt) = source.transport_annotation();
                (inner, operation_id.or(inner_op), attempt.or(inner_attempt))
            }
            other => (other, None, None),
        }
    }

    /// HTTP method from transport annotation when present.
    pub fn method(&self) -> Option<&'static str> {
        match self {
            Self::Annotated { method, source, .. } => method.or_else(|| source.method()),
            _ => None,
        }
    }

    /// Request id from response headers when present.
    pub fn request_id(&self) -> Option<String> {
        self.metadata().request_id
    }

    /// Parsed `Retry-After` when present on an API / malformed error.
    pub fn retry_after(&self) -> Option<Duration> {
        self.metadata().retry_after
    }

    /// Mollie error title when this is a documented API error.
    pub fn provider_code(&self) -> Option<&str> {
        self.as_api_body().map(|body| body.title.as_str())
    }

    /// Stable catalog key string for this error.
    pub fn provider_key(&self) -> &str {
        self.catalog_entry().key().as_str()
    }

    /// Returns `true` when the underlying transport timed out.
    pub fn is_timeout(&self) -> bool {
        match self {
            Self::Annotated { source, .. } => source.is_timeout(),
            Self::Communication(error) | Self::ResponseBody(error) => error.is_timeout(),
            _ => false,
        }
    }

    /// Returns `true` when the underlying transport failed to connect.
    pub fn is_connection_failure(&self) -> bool {
        match self {
            Self::Annotated { source, .. } => source.is_connection_failure(),
            Self::Communication(error) | Self::ResponseBody(error) => error.is_connect(),
            _ => false,
        }
    }

    /// Best-effort [`crate::DeliveryOutcome`] for payment-safe caller decisions.
    ///
    /// - Connect failures → [`DeliveryOutcome::NotSent`](crate::DeliveryOutcome::NotSent) (may retry under policy)
    /// - Timeouts / cancel-after-transmit → [`DeliveryOutcome::Unknown`](crate::DeliveryOutcome::Unknown)
    /// - Documented API 4xx → [`DeliveryOutcome::Rejected`](crate::DeliveryOutcome::Rejected)
    /// - Local config/validation before send → [`DeliveryOutcome::NotSent`](crate::DeliveryOutcome::NotSent)
    ///
    /// **Cancellation (INV-CANCEL-01):** dropping an in-flight request future can
    /// leave a write in [`DeliveryOutcome::Unknown`](crate::DeliveryOutcome::Unknown). Applications must use a
    /// caller-owned sticky idempotency key for any write they may cancel and retry.
    pub fn delivery_outcome(&self) -> Option<crate::DeliveryOutcome> {
        use crate::DeliveryOutcome;
        match self {
            Self::Annotated { source, .. } => source.delivery_outcome(),
            Self::InvalidConfiguration { .. }
            | Self::InvalidRequest(_)
            | Self::InvalidHeaderValue(_) => Some(DeliveryOutcome::NotSent),
            Self::Communication(error) | Self::ResponseBody(error) => {
                Some(crate::transport::classify_reqwest_error(error))
            }
            Self::Api { status, .. } | Self::UnexpectedStatus { status } => {
                Some(crate::transport::classify_http_status(*status))
            }
            Self::MalformedProviderResponse { context } => context
                .status
                .map(crate::transport::classify_http_status)
                .or(Some(crate::DeliveryOutcome::Unknown)),
            Self::InvalidResponsePayload { .. } => Some(DeliveryOutcome::Unknown),
            Self::WebhookVerification { .. } | Self::Hook(_) => None,
        }
    }

    /// Returns `true` when delivery is ambiguous (may have been processed).
    pub fn is_outcome_unknown(&self) -> bool {
        matches!(
            self.delivery_outcome(),
            Some(crate::DeliveryOutcome::Unknown)
        )
    }

    /// Returns `true` when the request was cancelled (best-effort).
    ///
    /// `reqwest` does not always expose cancellation distinctly; this currently
    /// matches request-builder / body errors that mention cancel when present.
    ///
    /// Treat cancellation after transmit like [`DeliveryOutcome::Unknown`](crate::DeliveryOutcome::Unknown) for
    /// financial writes — see [`Self::delivery_outcome`].
    pub fn is_cancelled(&self) -> bool {
        match self {
            Self::Annotated { source, .. } => source.is_cancelled(),
            Self::InvalidRequest(message) => message.to_ascii_lowercase().contains("cancel"),
            Self::Communication(error) | Self::ResponseBody(error) => {
                let msg = error.to_string().to_ascii_lowercase();
                msg.contains("cancel") || msg.contains("canceled") || msg.contains("cancelled")
            }
            _ => false,
        }
    }

    /// 1-based attempt count when response metadata recorded it.
    pub fn attempt_count(&self) -> Option<u32> {
        self.metadata().attempt
    }

    /// Operation id when response metadata recorded it.
    pub fn operation(&self) -> Option<&'static str> {
        self.metadata().operation_id
    }

    /// Returns `true` for HTTP 401 authentication failures.
    pub fn is_authentication_failure(&self) -> bool {
        matches!(self.status(), Some(StatusCode::UNAUTHORIZED))
    }

    /// Returns `true` for HTTP 403 authorization failures.
    pub fn is_authorization_failure(&self) -> bool {
        matches!(self.status(), Some(StatusCode::FORBIDDEN))
    }

    /// Returns `true` when this is a webhook verification failure.
    pub fn is_webhook_verification_failure(&self) -> bool {
        match self {
            Self::Annotated { source, .. } => source.is_webhook_verification_failure(),
            Self::WebhookVerification { .. } => true,
            _ => false,
        }
    }

    /// Returns `true` when Mollie returned a client-caused `4xx` status.
    pub fn is_client_error(&self) -> bool {
        self.status().is_some_and(|status| status.is_client_error())
    }

    /// Returns `true` when Mollie or an upstream gateway returned a `5xx` status.
    pub fn is_server_error(&self) -> bool {
        self.status().is_some_and(|status| status.is_server_error())
    }

    /// Returns `true` when retrying may succeed without changing the request.
    ///
    /// Mollie documents `429` and transient `5xx` responses as retry candidates.
    /// The caller remains responsible for backoff and reusing the same
    /// idempotency key for a retried write.
    pub fn is_retryable(&self) -> bool {
        if self.is_timeout() {
            return true;
        }
        matches!(
            self.status(),
            Some(
                StatusCode::REQUEST_TIMEOUT
                    | StatusCode::TOO_MANY_REQUESTS
                    | StatusCode::INTERNAL_SERVER_ERROR
                    | StatusCode::BAD_GATEWAY
                    | StatusCode::SERVICE_UNAVAILABLE
                    | StatusCode::GATEWAY_TIMEOUT
            )
        )
    }

    /// Returns the Mollie HAL error body when this is an API error.
    pub const fn as_api_body(&self) -> Option<&ErrorResponse> {
        match self {
            Self::Annotated { source, .. } => source.as_api_body(),
            Self::Api { body, .. } => Some(body),
            _ => None,
        }
    }

    /// Returns `true` when the error came from a documented Mollie error body.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::MollieError;
    ///
    /// assert!(!MollieError::invalid_configuration("missing token").is_api_error());
    /// ```
    pub const fn is_api_error(&self) -> bool {
        match self {
            Self::Annotated { source, .. } => source.is_api_error(),
            Self::Api { .. } => true,
            _ => false,
        }
    }

    /// Returns `true` when this error is catalogued as a rate-limit failure.
    pub fn is_rate_limited(&self) -> bool {
        self.catalog_entry().key() == MollieErrorKey::RateLimitExceeded
    }

    /// Returns `true` when this error is catalogued as a validation failure.
    pub fn is_validation_error(&self) -> bool {
        matches!(
            self.catalog_entry().key(),
            MollieErrorKey::ValidationError
                | MollieErrorKey::ResourceStateConflict
                | MollieErrorKey::UnprocessableEntity
        )
    }

    /// Returns `true` when this error is catalogued as not-found.
    pub fn is_not_found(&self) -> bool {
        matches!(
            self.catalog_entry().key(),
            MollieErrorKey::EntityNotFound | MollieErrorKey::NotFound
        )
    }

    /// Returns the catalog entry (code, key, message_key) for this error.
    ///
    /// For API errors, known Mollie `detail` strings map to specific entries
    /// such as [`MollieErrorCatalogEntry::ACCESS_TOKEN_PROFILE_RESTRICTED`] and
    /// [`MollieErrorCatalogEntry::RATE_LIMIT_EXCEEDED`].
    pub fn catalog_entry(&self) -> MollieErrorCatalogEntry {
        match self {
            Self::Annotated { source, .. } => source.catalog_entry(),
            Self::Api { body, .. } => MollieErrorCatalogEntry::classify_api(body),
            Self::InvalidConfiguration { .. } => MollieErrorCatalogEntry::INVALID_CONFIGURATION,
            Self::InvalidRequest(_) => MollieErrorCatalogEntry::INVALID_REQUEST,
            Self::Communication(_) => MollieErrorCatalogEntry::COMMUNICATION,
            Self::InvalidHeaderValue(_) => MollieErrorCatalogEntry::INVALID_HEADER_VALUE,
            Self::UnexpectedStatus { .. } => MollieErrorCatalogEntry::UNEXPECTED_STATUS,
            Self::ResponseBody(_) => MollieErrorCatalogEntry::RESPONSE_BODY,
            Self::InvalidResponsePayload { .. } | Self::MalformedProviderResponse { .. } => {
                MollieErrorCatalogEntry::INVALID_RESPONSE_PAYLOAD
            }
            Self::WebhookVerification { .. } => MollieErrorCatalogEntry::INVALID_REQUEST,
            Self::Hook(_) => MollieErrorCatalogEntry::HOOK,
        }
    }

    /// Builds a consistent JSON-serializable error envelope for this error.
    ///
    /// HTTP status-family counterparts (`403`, `429`, …) share the same object
    /// shape; known details map to stable `code`, `key`, and `message_key`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::MollieError;
    ///
    /// let envelope = MollieError::rate_limit_exceeded().to_envelope();
    /// assert_eq!(envelope.status, Some(429));
    /// assert_eq!(envelope.code, 42901);
    /// assert_eq!(envelope.key.as_str(), "RATE_LIMIT_EXCEEDED");
    /// assert_eq!(
    ///     envelope.message_key,
    ///     "errors.too_many_requests.rate_limit_exceeded"
    /// );
    /// ```
    pub fn to_envelope(&self) -> MollieErrorEnvelope {
        match self {
            Self::Annotated { source, .. } => source.to_envelope(),
            Self::Api { body, .. } => MollieErrorEnvelope::from_api(self.catalog_entry(), body),
            Self::InvalidConfiguration { message } => MollieErrorEnvelope::from_client(
                MollieErrorCatalogEntry::INVALID_CONFIGURATION,
                None,
                message.clone(),
                Some("Invalid Configuration".to_string()),
            ),
            Self::InvalidRequest(message) => MollieErrorEnvelope::from_client(
                MollieErrorCatalogEntry::INVALID_REQUEST,
                None,
                message.clone(),
                Some("Invalid Request".to_string()),
            ),
            Self::Communication(error) => MollieErrorEnvelope::from_client(
                MollieErrorCatalogEntry::COMMUNICATION,
                error.status().map(|s| s.as_u16()),
                error.to_string(),
                Some("Communication Error".to_string()),
            ),
            Self::InvalidHeaderValue(error) => MollieErrorEnvelope::from_client(
                MollieErrorCatalogEntry::INVALID_HEADER_VALUE,
                None,
                error.to_string(),
                Some("Invalid Header Value".to_string()),
            ),
            Self::UnexpectedStatus { status } => MollieErrorEnvelope::from_client(
                MollieErrorCatalogEntry::UNEXPECTED_STATUS,
                Some(status.as_u16()),
                format!("Mollie API returned undocumented status {status}"),
                Some("Unexpected Status".to_string()),
            ),
            Self::ResponseBody(error) => MollieErrorEnvelope::from_client(
                MollieErrorCatalogEntry::RESPONSE_BODY,
                error.status().map(|s| s.as_u16()),
                error.to_string(),
                Some("Response Body Error".to_string()),
            ),
            Self::InvalidResponsePayload { source, .. } => MollieErrorEnvelope::from_client(
                MollieErrorCatalogEntry::INVALID_RESPONSE_PAYLOAD,
                None,
                source.to_string(),
                Some("Invalid Response Payload".to_string()),
            ),
            Self::MalformedProviderResponse { context } => MollieErrorEnvelope::from_client(
                MollieErrorCatalogEntry::INVALID_RESPONSE_PAYLOAD,
                context.status.map(|s| s.as_u16()),
                format!("malformed provider response: {}", context.body_preview(200)),
                Some("Malformed Provider Response".to_string()),
            ),
            Self::WebhookVerification { failure } => MollieErrorEnvelope::from_client(
                MollieErrorCatalogEntry::INVALID_REQUEST,
                None,
                failure.to_string(),
                Some("Webhook Verification Failed".to_string()),
            ),
            Self::Hook(message) => MollieErrorEnvelope::from_client(
                MollieErrorCatalogEntry::HOOK,
                None,
                message.clone(),
                Some("Hook Error".to_string()),
            ),
        }
    }

    /// Serializes [`Self::to_envelope`] to a [`serde_json::Value`].
    pub fn to_json_value(&self) -> serde_json::Value {
        serde_json::to_value(self.to_envelope()).unwrap_or_else(|_| {
            serde_json::json!({
                "code": self.catalog_entry().code(),
                "key": self.catalog_entry().key().as_str(),
                "message_key": self.catalog_entry().message_key(),
                "detail": self.to_string(),
            })
        })
    }
}

impl From<progenitor_client::Error<ErrorResponse>> for MollieError {
    /// Converts a generated client error into the crate-owned error family.
    fn from(error: progenitor_client::Error<ErrorResponse>) -> Self {
        match error {
            progenitor_client::Error::InvalidRequest(message) => Self::InvalidRequest(message),
            progenitor_client::Error::CommunicationError(error)
            | progenitor_client::Error::InvalidUpgrade(error) => Self::Communication(error),
            progenitor_client::Error::ErrorResponse(response) => {
                let status: StatusCode = response.status();
                let headers: HeaderMap = response.headers().clone();
                let body: Box<ErrorResponse> = Box::new(response.into_inner());
                Self::Api {
                    status,
                    headers,
                    body,
                }
            }
            progenitor_client::Error::ResponseBodyError(error) => Self::ResponseBody(error),
            progenitor_client::Error::InvalidResponsePayload(bytes, source) => {
                Self::InvalidResponsePayload {
                    bytes: truncate_body_bytes(bytes),
                    source,
                }
            }
            progenitor_client::Error::UnexpectedResponse(response) => Self::UnexpectedStatus {
                status: response.status(),
            },
            progenitor_client::Error::Custom(message) => Self::Hook(message),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod factory {
        use super::*;

        #[test]
        fn invalid_request_creates_invalid_request_error() {
            let error: MollieError = MollieError::invalid_request("missing amount");

            assert!(matches!(error, MollieError::InvalidRequest(_)));
            assert_eq!(error.status(), None);
        }

        #[test]
        fn unexpected_status_creates_response_error() {
            let error: MollieError = MollieError::unexpected_status(StatusCode::IM_A_TEAPOT);

            assert_eq!(error.status(), Some(StatusCode::IM_A_TEAPOT));
        }

        #[test]
        fn api_creates_documented_api_error() {
            let body: ErrorResponse = ErrorResponse {
                detail: "The payment id is invalid.".to_string(),
                field: Some("paymentId".to_string()),
                links: ErrorResponseLinks {
                    documentation: ErrorResponseLinksDocumentation {
                        href: "https://docs.mollie.com/errors".to_string(),
                        type_: "text/html".to_string(),
                    },
                },
                status: 422,
                title: "Unprocessable Entity".to_string(),
            };

            let error: MollieError =
                MollieError::api(StatusCode::UNPROCESSABLE_ENTITY, HeaderMap::new(), body);

            assert!(error.is_api_error());
            assert_eq!(error.status(), Some(StatusCode::UNPROCESSABLE_ENTITY));
        }

        #[test]
        fn transport_context_fills_operation_and_attempt() {
            let error = MollieError::invalid_request("boom").with_transport_context(
                "create_payment",
                2,
                Some("POST"),
            );
            assert_eq!(error.operation(), Some("create_payment"));
            assert_eq!(error.attempt_count(), Some(2));
            assert_eq!(error.method(), Some("POST"));
            assert_eq!(error.to_string(), "invalid Mollie request: boom");
        }

        #[test]
        fn access_token_profile_restricted_factory() {
            let error: MollieError = MollieError::access_token_profile_restricted();
            let envelope: MollieErrorEnvelope = error.to_envelope();

            assert_eq!(error.status(), Some(StatusCode::FORBIDDEN));
            assert_eq!(envelope.status, Some(403));
            assert_eq!(envelope.code, 40301);
            assert_eq!(envelope.key.as_str(), "ACCESS_TOKEN_PROFILE_RESTRICTED");
            assert_eq!(
                envelope.message_key,
                "errors.forbidden.access_token_profile_restricted"
            );
            assert!(envelope
                .detail
                .contains("not restricted to a specific profile"));
        }

        #[test]
        fn rate_limit_exceeded_factory() {
            let error: MollieError = MollieError::rate_limit_exceeded();
            let envelope: MollieErrorEnvelope = error.to_envelope();

            assert_eq!(error.status(), Some(StatusCode::TOO_MANY_REQUESTS));
            assert!(!envelope.ok);
            assert_eq!(envelope.status, Some(429));
            assert_eq!(envelope.code, 42901);
            assert_eq!(envelope.key.as_str(), "RATE_LIMIT_EXCEEDED");
            assert_eq!(envelope.title.as_deref(), Some("Too Many Requests"));
            assert_eq!(
                envelope.message_key,
                "errors.too_many_requests.rate_limit_exceeded"
            );
            assert!(error.is_rate_limited());
            assert_eq!(
                crate::factory::rate_limit_exceeded()
                    .to_envelope()
                    .title
                    .as_deref(),
                Some("Too Many Requests")
            );
        }

        /// Classifies client errors as corrective and transient server errors as retryable.
        #[test]
        fn classifies_status_families_and_retryability() {
            let validation = MollieError::validation_error("invalid amount", Some("amount"));
            assert!(validation.is_client_error());
            assert!(!validation.is_server_error());
            assert!(!validation.is_retryable());

            let rate_limited = MollieError::rate_limit_exceeded();
            assert!(rate_limited.is_client_error());
            assert!(rate_limited.is_retryable());

            let unavailable = MollieError::service_temporarily_unavailable("try again later");
            assert!(!unavailable.is_client_error());
            assert!(unavailable.is_server_error());
            assert!(unavailable.is_retryable());
        }

        #[test]
        fn validation_and_not_found_factories() {
            let validation: MollieError =
                MollieError::validation_error("The 'amount' field is missing", Some("amount"));
            assert!(validation.is_validation_error());
            assert_eq!(
                validation
                    .as_api_body()
                    .and_then(|b| b.field.clone())
                    .as_deref(),
                Some("amount")
            );

            let missing: MollieError = MollieError::entity_not_found("tr_x");
            assert!(missing.is_not_found());
            assert!(MollieError::invalid_cursor().is_api_error());
        }
    }
}
