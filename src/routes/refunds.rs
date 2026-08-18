//! Generated refunds route methods.

use crate::{routes, types, Client, Error, ResponseValue};
use progenitor_client::encode_path;

/// Generated `refunds` route methods on [`crate::Client`].
///
/// Client-owned request policy on [`crate::Client`]:
/// - idempotency via [`crate::Client::with_idempotency_key`] (default: UUID v4 per request)
/// - `testmode` query via [`crate::Client::with_testmode`] on operations that support it
///
/// The resolved idempotency key is returned on the response envelope
/// ([`crate::ResponseValue`] / [`crate::ResponseEnvelope`]).
#[allow(clippy::all)]
impl Client {
    /// List payment refunds
    ///
    /// Retrieve a list of all refunds created for a specific payment.
    ///
    /// The results are paginated.
    ///
    /// Sends a `GET` request to `/payments/{paymentId}/refunds`
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
    pub async fn list_refunds<'a>(
        &'a self,
        payment_id: &'a types::PaymentToken,
        embed: Option<&'a str>,
        from: Option<&'a types::RefundToken>,
        limit: Option<::std::num::NonZeroU64>,
    ) -> Result<ResponseValue<types::ListRefundsResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/payments/{}/refunds",
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
        let response = self.send(request, routes::Operation::ListRefunds).await?;
        routes::response::json(
            response,
            &[200u16],
            &[400u16, 404u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }

    /// Create payment refund
    ///
    /// Creates a refund for a specific payment. The refunded amount is credited to your customer usually either via a bank
    /// transfer or by refunding the amount to your customer's credit card.
    ///
    /// Sends a `POST` request to `/payments/{paymentId}/refunds`
    ///
    /// Arguments:
    /// - `payment_id`: Provide the ID of the related payment.
    pub async fn create_refund<'a>(
        &'a self,
        payment_id: &'a types::PaymentToken,
        body: &'a types::RefundRequest,
    ) -> Result<ResponseValue<types::EntityRefundResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/payments/{}/refunds",
            encode_path(&payment_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::POST, url)?;
        #[allow(unused_mut)]
        let mut request = request.json(&body).build()?;
        let response = self.send(request, routes::Operation::CreateRefund).await?;
        routes::response::json(
            response,
            &[201u16],
            &[404u16, 409u16, 422u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }

    /// Get payment refund
    ///
    /// Retrieve a single payment refund by its ID and the ID of its parent payment.
    ///
    /// Sends a `GET` request to `/payments/{paymentId}/refunds/{refundId}`
    ///
    /// Arguments:
    /// - `payment_id`: Provide the ID of the related payment.
    /// - `refund_id`: Provide the ID of the related refund.
    /// - `embed`: This endpoint allows embedding related API items by appending the following values via the `embed` query string
    /// parameter.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn get_refund<'a>(
        &'a self,
        payment_id: &'a types::PaymentToken,
        refund_id: &'a types::RefundToken,
        embed: Option<&'a str>,
    ) -> Result<ResponseValue<types::EntityRefundResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/payments/{}/refunds/{}",
            encode_path(&payment_id.to_string()),
            encode_path(&refund_id.to_string())
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
        let response = self.send(request, routes::Operation::GetRefund).await?;
        routes::response::json(
            response,
            &[200u16],
            &[404u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }

    /// Cancel payment refund
    ///
    /// Refunds will be executed with a delay of two hours. Until that time, refunds may be canceled manually via the
    /// Mollie Dashboard, or by using this endpoint.
    ///
    /// A refund can only be canceled while its `status` field is either `queued` or `pending`. See the
    /// [Get refund endpoint](get-refund) for more information.
    ///
    /// Sends a `DELETE` request to `/payments/{paymentId}/refunds/{refundId}`
    ///
    /// Arguments:
    /// - `payment_id`: Provide the ID of the related payment.
    /// - `refund_id`: Provide the ID of the related refund.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn cancel_refund<'a>(
        &'a self,
        payment_id: &'a types::PaymentToken,
        refund_id: &'a types::RefundToken,
    ) -> Result<ResponseValue<()>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/payments/{}/refunds/{}",
            encode_path(&payment_id.to_string()),
            encode_path(&refund_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::DELETE, url)?;
        #[allow(unused_mut)]
        let mut request = request
            .query(&progenitor_client::QueryParam::new(
                "testmode",
                &self.testmode(),
            ))
            .build()?;
        let response = self.send(request, routes::Operation::CancelRefund).await?;
        routes::response::json(
            response,
            &[204u16],
            &[404u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }

    /// List all refunds
    ///
    /// Retrieve a list of all of your refunds.
    ///
    /// The results are paginated.
    ///
    /// Sends a `GET` request to `/refunds`
    ///
    /// Arguments:
    /// - `embed`: This endpoint allows embedding related API items by appending the following values via the `embed` query string
    /// parameter.
    /// - `from`: Provide an ID to start the result set from the item with the given ID and onwards. This allows you to paginate the
    /// result set.
    /// - `limit`: The maximum number of items to return. Defaults to 50 items.
    /// - `profile_id`: The identifier referring to the [profile](get-profile) you wish to
    /// retrieve the resources for.
    ///
    /// Most API credentials are linked to a single profile. In these cases the `profileId` must not be sent. For
    /// organization-level credentials such as OAuth access tokens however, the `profileId` parameter is required.
    /// - `sort`: Used for setting the direction of the result set. Defaults to descending order, meaning the results are ordered from
    /// newest to oldest.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn list_all_refunds<'a>(
        &'a self,
        embed: Option<&'a str>,
        from: Option<&'a types::RefundToken>,
        limit: Option<::std::num::NonZeroU64>,
        profile_id: Option<&'a types::ProfileToken>,
        sort: Option<types::Sorting>,
    ) -> Result<ResponseValue<types::ListAllRefundsResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint("/refunds");
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request
            .query(&progenitor_client::QueryParam::new("embed", &embed))
            .query(&progenitor_client::QueryParam::new("from", &from))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new(
                "profileId",
                &profile_id,
            ))
            .query(&progenitor_client::QueryParam::new("sort", &sort))
            .query(&progenitor_client::QueryParam::new(
                "testmode",
                &self.testmode(),
            ))
            .build()?;
        let response = self
            .send(request, routes::Operation::ListAllRefunds)
            .await?;
        routes::response::json(
            response,
            &[200u16],
            &[400u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }
}
