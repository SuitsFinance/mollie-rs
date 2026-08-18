//! Generated payment links route methods.

use crate::{routes, types, Client, Error, ResponseValue};
use progenitor_client::encode_path;

/// Generated `payment links` route methods on [`crate::Client`].
///
/// Client-owned request policy on [`crate::Client`]:
/// - idempotency via [`crate::Client::with_idempotency_key`] (default: UUID v4 per request)
/// - `testmode` query via [`crate::Client::with_testmode`] on operations that support it
///
/// The resolved idempotency key is returned on the response envelope
/// ([`crate::ResponseValue`] / [`crate::ResponseEnvelope`]).
#[allow(clippy::all)]
impl Client {
    /// List payment links
    ///
    /// Retrieve a list of all payment links.
    ///
    /// The results are paginated.
    ///
    /// Sends a `GET` request to `/payment-links`
    ///
    /// Arguments:
    /// - `from`: Provide an ID to start the result set from the item with the given ID and onwards. This allows you to paginate the
    /// result set.
    /// - `limit`: The maximum number of items to return. Defaults to 50 items.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn list_payment_links<'a>(
        &'a self,
        from: Option<&'a types::PaymentLinkToken>,
        limit: Option<::std::num::NonZeroU64>,
    ) -> Result<ResponseValue<types::ListPaymentLinksResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint("/payment-links");
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request
            .query(&progenitor_client::QueryParam::new("from", &from))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new(
                "testmode",
                &self.testmode(),
            ))
            .build()?;
        let response = self
            .send(request, routes::Operation::ListPaymentLinks)
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

    /// Create payment link
    ///
    /// With the Payment links API you can generate payment links that by default, unlike regular payments, do not expire.
    /// The payment link can be shared with your customers and will redirect them to them the payment page where they can
    /// complete the payment. A [payment](get-payment) will only be created once the customer initiates the payment.
    ///
    /// Sends a `POST` request to `/payment-links`
    ///
    /// Arguments:
    pub async fn create_payment_link<'a>(
        &'a self,
        body: &'a types::CreatePaymentLinkBody,
    ) -> Result<ResponseValue<types::PaymentLinkResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint("/payment-links");
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::POST, url)?;
        #[allow(unused_mut)]
        let mut request = request.json(&body).build()?;
        let response = self
            .send(request, routes::Operation::CreatePaymentLink)
            .await?;
        routes::response::json(
            response,
            &[201u16],
            &[404u16, 422u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }

    /// Get payment link
    ///
    /// Retrieve a single payment link by its ID.
    ///
    /// Sends a `GET` request to `/payment-links/{paymentLinkId}`
    ///
    /// Arguments:
    /// - `payment_link_id`: Provide the ID of the related payment link.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn get_payment_link<'a>(
        &'a self,
        payment_link_id: &'a types::PaymentLinkToken,
    ) -> Result<ResponseValue<types::PaymentLinkResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/payment-links/{}",
            encode_path(&payment_link_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request
            .query(&progenitor_client::QueryParam::new(
                "testmode",
                &self.testmode(),
            ))
            .build()?;
        let response = self
            .send(request, routes::Operation::GetPaymentLink)
            .await?;
        routes::response::json(
            response,
            &[200u16],
            &[404u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }

    /// Delete payment link
    ///
    /// Payment links which have not been opened and no payments have been made yet can be deleted entirely.
    /// This can be useful for removing payment links that have been incorrectly configured or that are no longer relevant.
    ///
    /// Once deleted, the payment link will no longer show up in the API or Mollie dashboard.
    ///
    /// To simply disable a payment link without fully deleting it, you can use the `archived` parameter on the
    /// [Update payment link](update-payment-link) endpoint instead.
    ///
    /// Sends a `DELETE` request to `/payment-links/{paymentLinkId}`
    ///
    /// Arguments:
    /// - `payment_link_id`: Provide the ID of the related payment link.
    pub async fn delete_payment_link<'a>(
        &'a self,
        payment_link_id: &'a types::PaymentLinkToken,
        body: &'a types::DeletePaymentLinkBody,
    ) -> Result<ResponseValue<()>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/payment-links/{}",
            encode_path(&payment_link_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::DELETE, url)?;
        #[allow(unused_mut)]
        let mut request = request.json(&body).build()?;
        let response = self
            .send(request, routes::Operation::DeletePaymentLink)
            .await?;
        routes::response::json(
            response,
            &[204u16],
            &[404u16, 422u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }

    /// Update payment link
    ///
    /// Certain details of an existing payment link can be updated.
    ///
    /// Sends a `PATCH` request to `/payment-links/{paymentLinkId}`
    ///
    /// Arguments:
    /// - `payment_link_id`: Provide the ID of the related payment link.
    pub async fn update_payment_link<'a>(
        &'a self,
        payment_link_id: &'a types::PaymentLinkToken,
        body: &'a types::UpdatePaymentLinkBody,
    ) -> Result<ResponseValue<types::PaymentLinkResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/payment-links/{}",
            encode_path(&payment_link_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::PATCH, url)?;
        #[allow(unused_mut)]
        let mut request = request.json(&body).build()?;
        let response = self
            .send(request, routes::Operation::UpdatePaymentLink)
            .await?;
        routes::response::json(
            response,
            &[200u16],
            &[404u16, 422u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }

    /// Get payment link payments
    ///
    /// Retrieve the list of payments for a specific payment link.
    ///
    /// The results are paginated.
    ///
    /// Sends a `GET` request to `/payment-links/{paymentLinkId}/payments`
    ///
    /// Arguments:
    /// - `payment_link_id`: Provide the ID of the related payment link.
    /// - `from`: Provide an ID to start the result set from the item with the given ID and onwards. This allows you to paginate the
    /// result set.
    /// - `limit`: The maximum number of items to return. Defaults to 50 items.
    /// - `sort`: Used for setting the direction of the result set. Defaults to descending order, meaning the results are ordered from
    /// newest to oldest.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn get_payment_link_payments<'a>(
        &'a self,
        payment_link_id: &'a types::PaymentLinkToken,
        from: Option<&'a types::PaymentToken>,
        limit: Option<::std::num::NonZeroU64>,
        sort: Option<types::Sorting>,
    ) -> Result<ResponseValue<types::GetPaymentLinkPaymentsResponse>, Error<types::ErrorResponse>>
    {
        let url = self.endpoint(format_args!(
            "/payment-links/{}/payments",
            encode_path(&payment_link_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request
            .query(&progenitor_client::QueryParam::new("from", &from))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("sort", &sort))
            .query(&progenitor_client::QueryParam::new(
                "testmode",
                &self.testmode(),
            ))
            .build()?;
        let response = self
            .send(request, routes::Operation::GetPaymentLinkPayments)
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
