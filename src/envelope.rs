//! Response-envelope helpers for generated Mollie route calls.
//!
//! Generated methods return [`progenitor_client::ResponseValue`]. This module
//! provides a smaller crate-owned envelope for application code and tests.
//! Successful route responses always carry the resolved `idempotency_key`
//! and can expose operational [`crate::ResponseMetadata`].
//! (caller-supplied or client-generated UUID v4).
//!
//! # Examples
//!
//! ```rust
//! use mollie_rs::ResponseEnvelope;
//! use reqwest::StatusCode;
//!
//! let envelope = ResponseEnvelope::from_parts("ok", StatusCode::OK, Default::default());
//! assert_eq!(envelope.data(), &"ok");
//! ```
#![warn(missing_docs)]

use std::future::Future;

use reqwest::{header::HeaderMap, StatusCode};

use crate::{
    error::MollieResult,
    error_catalog::{MollieSuccessCatalogEntry, MollieSuccessEnvelope},
    routes::response::IDEMPOTENCY_KEY_HEADER,
    types, MollieError, ResponseValue,
};

/// Header-backed helpers for the generated [`ResponseValue`] type.
///
/// Generated route methods attach the resolved idempotency key to the response
/// headers so callers can read it without converting to [`ResponseEnvelope`].
///
/// # Examples
///
/// ```rust
/// use mollie_rs::{ResponseValue, ResponseValueExt};
/// use reqwest::StatusCode;
///
/// let mut headers = reqwest::header::HeaderMap::new();
/// headers.insert(
///     "idempotency-key",
///     "6f7ef3e6-8c2f-4d1c-9f08-5ab7adf56c91"
///         .parse()
///         .expect("static header value"),
/// );
/// let response = ResponseValue::new("ok", StatusCode::OK, headers);
/// assert_eq!(
///     response.idempotency_key(),
///     Some("6f7ef3e6-8c2f-4d1c-9f08-5ab7adf56c91")
/// );
/// ```
pub trait ResponseValueExt {
    /// Returns the idempotency key sent with the request, when present.
    ///
    /// Generated successful and error responses from this crate include the
    /// resolved key under the `idempotency-key` header.
    fn idempotency_key(&self) -> Option<&str>;
}

/// Reads the resolved idempotency key from [`ResponseValue`] response headers.
impl<T> ResponseValueExt for ResponseValue<T> {
    /// Returns the `idempotency-key` header value when present and UTF-8.
    fn idempotency_key(&self) -> Option<&str> {
        self.headers()
            .get(IDEMPOTENCY_KEY_HEADER)
            .and_then(|value| value.to_str().ok())
    }
}

/// Reads an idempotency key from response headers, if present and valid UTF-8.
///
/// Used when constructing [`ResponseEnvelope`] from parts or from a generated
/// [`ResponseValue`].
fn idempotency_key_from_headers(headers: &HeaderMap) -> Option<String> {
    headers
        .get(IDEMPOTENCY_KEY_HEADER)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned)
}

/// Shared crate-owned response envelope type.
///
/// # Examples
///
/// ```rust
/// use mollie_rs::MollieEnvelope;
///
/// let envelope = MollieEnvelope::ok("ok");
/// assert_eq!(envelope.into_inner(), "ok");
/// ```
pub type MollieEnvelope<T> = ResponseEnvelope<T>;

/// Shared crate-owned response result type.
///
/// # Examples
///
/// ```rust
/// use mollie_rs::{MollieResponse, ResponseEnvelope};
///
/// let response: MollieResponse<&str> = Ok(ResponseEnvelope::ok("ok"));
/// assert_eq!(response?.into_inner(), "ok");
/// # Ok::<(), mollie_rs::MollieError>(())
/// ```
pub type MollieResponse<T> = MollieResult<MollieEnvelope<T>>;

/// Shared generated route result type for operations with Mollie's documented
/// error response body.
///
/// # Examples
///
/// ```rust
/// use mollie_rs::{GeneratedMollieResult, ResponseValue};
/// use reqwest::StatusCode;
///
/// let generated: GeneratedMollieResult<&str> =
///     Ok(ResponseValue::new("ok", StatusCode::OK, Default::default()));
/// assert!(generated.is_ok());
/// ```
pub type GeneratedMollieResult<T> =
    Result<ResponseValue<T>, progenitor_client::Error<types::ErrorResponse>>;

/// A typed response body plus HTTP status, headers, and resolved idempotency key.
///
/// This is the crate-owned success envelope. Convert generated
/// [`ResponseValue`] results with [`ResponseEnvelope::from_response_value`] or
/// [`IntoMollieResult`].
#[derive(Clone, Debug)]
pub struct ResponseEnvelope<T> {
    /// Deserialized success body for the route.
    data: T,
    /// HTTP status code of the response.
    status: StatusCode,
    /// Response headers, including the echoed `idempotency-key` when known.
    headers: HeaderMap,
    /// Idempotency key sent with the request (caller-supplied or generated).
    idempotency_key: Option<String>,
}

/// Accessors and constructors for the crate-owned response envelope.
impl<T> ResponseEnvelope<T> {
    /// Creates an envelope from a data value, status, and headers.
    ///
    /// When headers contain `idempotency-key`, it is stored on the envelope.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::ResponseEnvelope;
    /// use reqwest::StatusCode;
    ///
    /// let envelope = ResponseEnvelope::from_parts(42, StatusCode::CREATED, Default::default());
    /// assert_eq!(envelope.status(), StatusCode::CREATED);
    /// ```
    pub fn from_parts(data: T, status: StatusCode, headers: HeaderMap) -> Self {
        let idempotency_key = idempotency_key_from_headers(&headers);
        Self {
            data,
            status,
            headers,
            idempotency_key,
        }
    }

    /// Creates an envelope with an explicit idempotency key.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::ResponseEnvelope;
    /// use reqwest::StatusCode;
    ///
    /// let envelope = ResponseEnvelope::from_parts_with_idempotency(
    ///     "ok",
    ///     StatusCode::OK,
    ///     Default::default(),
    ///     Some("6f7ef3e6-8c2f-4d1c-9f08-5ab7adf56c91".to_string()),
    /// );
    /// assert_eq!(
    ///     envelope.idempotency_key(),
    ///     Some("6f7ef3e6-8c2f-4d1c-9f08-5ab7adf56c91")
    /// );
    /// ```
    pub fn from_parts_with_idempotency(
        data: T,
        status: StatusCode,
        headers: HeaderMap,
        idempotency_key: Option<String>,
    ) -> Self {
        Self {
            data,
            status,
            headers,
            idempotency_key,
        }
    }

    /// Creates a successful in-memory envelope for tests and local factories.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::ResponseEnvelope;
    ///
    /// let envelope = ResponseEnvelope::ok("created");
    /// assert_eq!(envelope.into_inner(), "created");
    /// ```
    pub fn ok(data: T) -> Self {
        Self::from_parts(data, StatusCode::OK, HeaderMap::new())
    }

    /// Creates an envelope from the generated response type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::{ResponseEnvelope, ResponseValue};
    /// use reqwest::StatusCode;
    ///
    /// let response = ResponseValue::new("ok", StatusCode::OK, Default::default());
    /// let envelope = ResponseEnvelope::from_response_value(response);
    /// assert_eq!(envelope.status(), StatusCode::OK);
    /// ```
    pub fn from_response_value(response: ResponseValue<T>) -> Self {
        let status = response.status();
        let headers = response.headers().clone();
        let idempotency_key = idempotency_key_from_headers(&headers);
        let data = response.into_inner();
        Self::from_parts_with_idempotency(data, status, headers, idempotency_key)
    }

    /// Returns the typed response body.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::ResponseEnvelope;
    ///
    /// let envelope = ResponseEnvelope::ok("ok");
    /// assert_eq!(envelope.data(), &"ok");
    /// ```
    pub const fn data(&self) -> &T {
        &self.data
    }

    /// Returns the HTTP status for the response.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::ResponseEnvelope;
    /// use reqwest::StatusCode;
    ///
    /// assert_eq!(ResponseEnvelope::ok("ok").status(), StatusCode::OK);
    /// ```
    pub const fn status(&self) -> StatusCode {
        self.status
    }

    /// Returns the HTTP response headers.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::ResponseEnvelope;
    ///
    /// assert!(ResponseEnvelope::ok("ok").headers().is_empty());
    /// ```
    pub const fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    /// Returns the idempotency key sent with the request, when known.
    ///
    /// Generated route methods always resolve a key (caller-supplied or a
    /// client-generated UUID v4) and attach it to the response.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::ResponseEnvelope;
    /// use reqwest::StatusCode;
    ///
    /// let envelope = ResponseEnvelope::from_parts_with_idempotency(
    ///     "ok",
    ///     StatusCode::OK,
    ///     Default::default(),
    ///     Some("6f7ef3e6-8c2f-4d1c-9f08-5ab7adf56c91".to_string()),
    /// );
    /// assert_eq!(
    ///     envelope.idempotency_key(),
    ///     Some("6f7ef3e6-8c2f-4d1c-9f08-5ab7adf56c91")
    /// );
    /// ```
    pub fn idempotency_key(&self) -> Option<&str> {
        self.idempotency_key.as_deref()
    }

    /// Returns operational metadata (request id, rate limits, `Retry-After`, …).
    ///
    /// Prefer this over parsing [`Self::headers`] for correlation and retry
    /// decisions. See [`crate::ResponseMetadata`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::ResponseEnvelope;
    /// use reqwest::header::{HeaderMap, HeaderValue};
    /// use reqwest::StatusCode;
    ///
    /// let mut headers = HeaderMap::new();
    /// headers.insert("request-id", HeaderValue::from_static("req_1"));
    /// let envelope = ResponseEnvelope::from_parts("ok", StatusCode::OK, headers);
    /// assert_eq!(envelope.metadata().request_id.as_deref(), Some("req_1"));
    /// ```
    pub fn metadata(&self) -> crate::ResponseMetadata {
        let mut meta = crate::ResponseMetadata::from_status_and_headers(self.status, &self.headers);
        if meta.idempotency_key.is_none() {
            meta.idempotency_key = self.idempotency_key.clone();
        }
        meta
    }

    /// Consumes the envelope and returns the typed response body.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::ResponseEnvelope;
    ///
    /// assert_eq!(ResponseEnvelope::ok("ok").into_inner(), "ok");
    /// ```
    pub fn into_inner(self) -> T {
        self.data
    }

    /// Maps the response body while preserving status, headers, and idempotency key.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::ResponseEnvelope;
    ///
    /// let envelope = ResponseEnvelope::ok(2).map(|value| value * 2);
    /// assert_eq!(envelope.into_inner(), 4);
    /// ```
    pub fn map<U>(self, map: impl FnOnce(T) -> U) -> ResponseEnvelope<U> {
        ResponseEnvelope {
            data: map(self.data),
            status: self.status,
            headers: self.headers,
            idempotency_key: self.idempotency_key,
        }
    }

    /// Returns the success catalog entry for this response's HTTP status.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::ResponseEnvelope;
    /// use reqwest::StatusCode;
    ///
    /// let entry = ResponseEnvelope::from_parts("x", StatusCode::CREATED, Default::default())
    ///     .success_catalog();
    /// assert_eq!(entry.code(), 20100);
    /// assert_eq!(entry.key().as_str(), "CREATED");
    /// ```
    pub fn success_catalog(&self) -> MollieSuccessCatalogEntry {
        MollieSuccessCatalogEntry::from_status(self.status.as_u16())
    }

    /// Converts into a serializable success envelope (typed `data` preserved).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::ResponseEnvelope;
    ///
    /// let success = ResponseEnvelope::ok("customer").to_success_envelope();
    /// assert!(success.ok);
    /// assert_eq!(success.status, 200);
    /// assert_eq!(success.code, 20000);
    /// assert_eq!(success.data, "customer");
    /// ```
    pub fn to_success_envelope(self) -> MollieSuccessEnvelope<T> {
        MollieSuccessEnvelope::from_status_data(self.status.as_u16(), self.data)
    }
}

/// Converts generated route results into crate-owned Mollie results.
pub trait IntoMollieResult<T> {
    /// Converts a generated route result into a [`ResponseEnvelope`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::{IntoMollieResult, ResponseValue};
    /// use reqwest::StatusCode;
    ///
    /// let generated: mollie_rs::GeneratedMollieResult<&str> =
    ///     Ok(ResponseValue::new("ok", StatusCode::OK, Default::default()));
    /// let envelope = generated.into_mollie_result().expect("response should convert");
    /// assert_eq!(envelope.into_inner(), "ok");
    /// ```
    fn into_mollie_result(self) -> MollieResponse<T>;

    /// Converts a generated route result into only the typed response body.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::{IntoMollieResult, ResponseValue};
    /// use reqwest::StatusCode;
    ///
    /// let generated: mollie_rs::GeneratedMollieResult<&str> =
    ///     Ok(ResponseValue::new("ok", StatusCode::OK, Default::default()));
    /// assert_eq!(generated.into_mollie_data().expect("response should convert"), "ok");
    /// ```
    fn into_mollie_data(self) -> MollieResult<T>;
}

/// Converts [`GeneratedMollieResult`] values into crate-owned [`MollieResponse`] /
/// [`MollieResult`] types, preserving status, headers, and idempotency key.
impl<T> IntoMollieResult<T> for GeneratedMollieResult<T> {
    /// Maps a successful [`ResponseValue`] into [`ResponseEnvelope`] and errors
    /// into [`MollieError`].
    fn into_mollie_result(self) -> MollieResponse<T> {
        self.map(ResponseEnvelope::from_response_value)
            .map_err(MollieError::from)
    }

    /// Maps a successful response into only its typed body (`into_inner`).
    fn into_mollie_data(self) -> MollieResult<T> {
        self.into_mollie_result().map(ResponseEnvelope::into_inner)
    }
}

/// Converts generated route futures into crate-owned Mollie futures.
///
/// This lets callers convert before `.await`, so route calls read as:
/// `client.get_payment(...).into_mollie_data().await?`.
pub trait IntoMollieFuture<T>: Future + Sized {
    /// Converts a generated route future into a future resolving to a
    /// [`ResponseEnvelope`].
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mollie_rs::{IntoMollieFuture, ResponseValue};
    /// use reqwest::StatusCode;
    ///
    /// # async fn example() -> Result<(), mollie_rs::MollieError> {
    /// let generated = async {
    ///     Ok::<_, mollie_rs::Error<mollie_rs::types::ErrorResponse>>(ResponseValue::new(
    ///         "ok",
    ///         StatusCode::OK,
    ///         Default::default(),
    ///     ))
    /// };
    /// let envelope = generated.into_mollie_result().await?;
    /// assert_eq!(envelope.into_inner(), "ok");
    /// # Ok(())
    /// # }
    /// ```
    fn into_mollie_result(self) -> impl Future<Output = MollieResponse<T>>;

    /// Converts a generated route future into a future resolving to only the
    /// typed response body.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mollie_rs::{IntoMollieFuture, ResponseValue};
    /// use reqwest::StatusCode;
    ///
    /// # async fn example() -> Result<(), mollie_rs::MollieError> {
    /// let generated = async {
    ///     Ok::<_, mollie_rs::Error<mollie_rs::types::ErrorResponse>>(ResponseValue::new(
    ///         "ok",
    ///         StatusCode::OK,
    ///         Default::default(),
    ///     ))
    /// };
    /// assert_eq!(generated.into_mollie_data().await?, "ok");
    /// # Ok(())
    /// # }
    /// ```
    fn into_mollie_data(self) -> impl Future<Output = MollieResult<T>>;
}

/// Blanket conversion for any future that resolves to a [`GeneratedMollieResult`].
///
/// This covers generated route method futures so callers can write
/// `client.create_payment(...).into_mollie_result().await`.
impl<F, T> IntoMollieFuture<T> for F
where
    F: Future<Output = GeneratedMollieResult<T>>,
{
    /// Awaits the generated route future and converts into a [`ResponseEnvelope`].
    async fn into_mollie_result(self) -> MollieResponse<T> {
        IntoMollieResult::into_mollie_result(self.await)
    }

    /// Awaits the generated route future and converts into only the typed body.
    async fn into_mollie_data(self) -> MollieResult<T> {
        IntoMollieResult::into_mollie_data(self.await)
    }
}
