//! Generated captures route methods.

use crate::{routes, types, Client, Error, ResponseValue};
use progenitor_client::encode_path;

/// Generated `captures` route methods on [`crate::Client`].
///
/// Client-owned request policy on [`crate::Client`]:
/// - idempotency via [`crate::Client::with_idempotency_key`] (default: UUID v4 per request)
/// - `testmode` query via [`crate::Client::with_testmode`] on operations that support it
///
/// The resolved idempotency key is returned on the response envelope
/// ([`crate::ResponseValue`] / [`crate::ResponseEnvelope`]).
#[allow(clippy::all)]
impl Client {
    /// List captures
    ///
    /// Retrieve a list of all captures created for a specific payment.
    ///
    /// The results are paginated.
    ///
    /// Sends a `GET` request to `/payments/{paymentId}/captures`
    ///
    /// Arguments:
    /// - `payment_id`: Provide the ID of the related payment.
    /// - `embed`: This endpoint allows embedding related API items by appending the following values via the `embed` query string
    /// parameter.
    /// - `from`: Provide an ID to start the result set from the item with the given ID and onwards. This allows you to paginate the
    /// result set.
    /// - `limit`: The maximum number of items to return. Defaults to 50 items.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn list_captures<'a>(
        &'a self,
        payment_id: &'a types::PaymentToken,
        embed: Option<&'a str>,
        from: Option<&'a types::CaptureToken>,
        limit: Option<::std::num::NonZeroU64>,
    ) -> Result<ResponseValue<types::ListCapturesResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/payments/{}/captures",
            encode_path(&payment_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request
            .query(&progenitor_client::QueryParam::new("embed", &embed))
            .query(&progenitor_client::QueryParam::new("from", &from))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new(
                "testmode",
                &self.testmode(),
            ))
            .build()?;
        let response = self.send(request, routes::Operation::ListCaptures).await?;
        routes::response::json(
            response,
            &[200u16],
            &[400u16, 404u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Create capture
    ///
    /// Capture an *authorized* payment.
    ///
    /// Some payment methods allow you to first collect a customer's authorization,
    /// and capture the amount at a later point.
    ///
    /// By default, Mollie captures payments automatically. If however you
    /// configured your payment with `captureMode: manual`, you can capture the payment using this endpoint after
    /// having collected the customer's authorization.
    ///
    /// Sends a `POST` request to `/payments/{paymentId}/captures`
    ///
    /// Arguments:
    /// - `payment_id`: Provide the ID of the related payment.
    pub async fn create_capture<'a>(
        &'a self,
        payment_id: &'a types::PaymentToken,
        body: &'a types::EntityCapture,
    ) -> Result<ResponseValue<types::CaptureResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/payments/{}/captures",
            encode_path(&payment_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::POST, url)?;
        #[allow(unused_mut)]
        let mut request = request.json(&body).build()?;
        let response = self.send(request, routes::Operation::CreateCapture).await?;
        routes::response::json(
            response,
            &[201u16],
            &[404u16, 422u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Get capture
    ///
    /// Retrieve a single payment capture by its ID and the ID of its parent
    /// payment.
    ///
    /// Sends a `GET` request to `/payments/{paymentId}/captures/{captureId}`
    ///
    /// Arguments:
    /// - `payment_id`: Provide the ID of the related payment.
    /// - `capture_id`: Provide the ID of the related capture.
    /// - `embed`: This endpoint allows embedding related API items by appending the following values via the `embed` query string
    /// parameter.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn get_capture<'a>(
        &'a self,
        payment_id: &'a types::PaymentToken,
        capture_id: &'a types::CaptureToken,
        embed: Option<&'a str>,
    ) -> Result<ResponseValue<types::CaptureResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/payments/{}/captures/{}",
            encode_path(&payment_id.to_string()),
            encode_path(&capture_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request
            .query(&progenitor_client::QueryParam::new("embed", &embed))
            .query(&progenitor_client::QueryParam::new(
                "testmode",
                &self.testmode(),
            ))
            .build()?;
        let response = self.send(request, routes::Operation::GetCapture).await?;
        routes::response::json(
            response,
            &[200u16],
            &[404u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }
}
