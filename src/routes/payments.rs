//! Generated payments route methods.

use crate::{routes, types, Client, Error, ResponseValue};
use progenitor_client::encode_path;

/// Generated `payments` route methods on [`crate::Client`].
///
/// Client-owned request policy on [`crate::Client`]:
/// - idempotency via [`crate::Client::with_idempotency_key`] (default: UUID v4 per request)
/// - `testmode` query via [`crate::Client::with_testmode`] on operations that support it
///
/// The resolved idempotency key is returned on the response envelope
/// ([`crate::ResponseValue`] / [`crate::ResponseEnvelope`]).
#[allow(clippy::all)]
impl Client {
    /// List payments
    ///
    /// Retrieve all payments created with the current website profile.
    ///
    /// The results are paginated.
    ///
    /// Sends a `GET` request to `/payments`
    ///
    /// Arguments:
    /// - `from`: Provide an ID to start the result set from the item with the given ID and onwards. This allows you to paginate
    /// the result set.
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
    pub async fn list_payments<'a>(
        &'a self,
        from: Option<&'a types::PaymentToken>,
        limit: Option<::std::num::NonZeroU64>,
        profile_id: Option<&'a types::ProfileToken>,
        sort: Option<types::Sorting>,
    ) -> Result<ResponseValue<types::ListPaymentsResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint("/payments");
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request
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
        let response = self.send(request, routes::Operation::ListPayments).await?;
        routes::response::json(
            response,
            &[200u16],
            &[400u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Create payment
    ///
    /// Payment creation is elemental to the Mollie API: this is where most payment
    /// implementations start off.
    ///
    /// Once you have created a payment, you should redirect your customer to the
    /// URL in the `_links.checkout` property from the response.
    ///
    /// To wrap your head around the payment process, an explanation and flow charts
    /// can be found in the 'Accepting payments' guide.
    ///
    /// If you specify the `method` parameter when creating a payment, optional
    /// additional parameters may be available for the payment method that are not listed below. Please refer to the
    /// guide on [method-specific parameters](extra-payment-parameters).
    ///
    /// Sends a `POST` request to `/payments`
    ///
    /// Arguments:
    /// - `include`: This endpoint allows you to include additional information via the `include` query string parameter.
    pub async fn create_payment<'a>(
        &'a self,
        include: Option<&'a str>,
        body: &'a types::PaymentRequest,
    ) -> Result<ResponseValue<types::PaymentResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint("/payments");
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::POST, url)?;
        #[allow(unused_mut)]
        let mut request = request
            .json(&body)
            .query(&progenitor_client::QueryParam::new("include", &include))
            .build()?;
        let response = self.send(request, routes::Operation::CreatePayment).await?;
        routes::response::json(
            response,
            &[201u16],
            &[422u16, 429u16, 503u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Get payment
    ///
    /// Retrieve a single payment object by its payment ID.
    ///
    /// Sends a `GET` request to `/payments/{paymentId}`
    ///
    /// Arguments:
    /// - `payment_id`: Provide the ID of the related payment.
    /// - `embed`: This endpoint allows embedding related API items by appending the following values via the `embed` query string
    /// parameter.
    /// - `include`: This endpoint allows you to include additional information via the `include` query string parameter.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn get_payment<'a>(
        &'a self,
        payment_id: &'a types::PaymentToken,
        embed: Option<&'a str>,
        include: Option<&'a str>,
    ) -> Result<ResponseValue<types::PaymentResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/payments/{}",
            encode_path(&payment_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request
            .query(&progenitor_client::QueryParam::new("embed", &embed))
            .query(&progenitor_client::QueryParam::new("include", &include))
            .query(&progenitor_client::QueryParam::new(
                "testmode",
                &self.testmode(),
            ))
            .build()?;
        let response = self.send(request, routes::Operation::GetPayment).await?;
        routes::response::json(
            response,
            &[200u16],
            &[404u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Cancel payment
    ///
    /// Depending on the payment method, you may be able to cancel a payment for a certain amount of time — usually until
    /// the next business day or as long as the payment status is open.
    ///
    /// Payments may also be canceled manually from the Mollie Dashboard.
    ///
    /// The `isCancelable` property on the [Payment object](get-payment) will indicate if the payment can be canceled.
    ///
    /// Sends a `DELETE` request to `/payments/{paymentId}`
    ///
    /// Arguments:
    /// - `payment_id`: Provide the ID of the related payment.
    pub async fn cancel_payment<'a>(
        &'a self,
        payment_id: &'a types::PaymentToken,
        body: &'a types::CancelPaymentBody,
    ) -> Result<ResponseValue<types::PaymentResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/payments/{}",
            encode_path(&payment_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::DELETE, url)?;
        #[allow(unused_mut)]
        let mut request = request.json(&body).build()?;
        let response = self.send(request, routes::Operation::CancelPayment).await?;
        routes::response::json(
            response,
            &[200u16],
            &[404u16, 422u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Update payment
    ///
    /// Certain details of an existing payment can be updated.
    ///
    /// Updating the payment details will not result in a webhook call.
    ///
    /// Sends a `PATCH` request to `/payments/{paymentId}`
    ///
    /// Arguments:
    /// - `payment_id`: Provide the ID of the related payment.
    pub async fn update_payment<'a>(
        &'a self,
        payment_id: &'a types::PaymentToken,
        body: &'a types::UpdatePaymentBody,
    ) -> Result<ResponseValue<types::PaymentResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/payments/{}",
            encode_path(&payment_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::PATCH, url)?;
        #[allow(unused_mut)]
        let mut request = request.json(&body).build()?;
        let response = self.send(request, routes::Operation::UpdatePayment).await?;
        routes::response::json(
            response,
            &[200u16],
            &[404u16, 422u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Release payment authorization
    ///
    /// Releases the full remaining authorized amount. Call this endpoint when you will not be making any additional
    /// captures. Payment authorizations may also be released manually from the Mollie Dashboard.
    ///
    /// Mollie will do its best to process release requests, but it is not guaranteed that it will succeed. It is up to
    /// the issuing bank if and when the hold will be released.
    ///
    /// If the request does succeed, the payment status will change to `canceled` for payments without captures.
    /// If there is a successful capture, the payment will transition to `paid`.
    ///
    /// Sends a `POST` request to `/payments/{paymentId}/release-authorization`
    ///
    /// Arguments:
    /// - `payment_id`: Provide the ID of the related payment.
    pub async fn release_authorization<'a>(
        &'a self,
        payment_id: &'a types::PaymentToken,
        body: &'a types::ReleaseAuthorizationBody,
    ) -> Result<ResponseValue<()>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/payments/{}/release-authorization",
            encode_path(&payment_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::POST, url)?;
        #[allow(unused_mut)]
        let mut request = request.json(&body).build()?;
        let response = self
            .send(request, routes::Operation::ReleaseAuthorization)
            .await?;
        routes::response::json(
            response,
            &[202u16],
            &[404u16, 422u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// List payment routes
    ///
    /// Retrieve a list of all routes created for a specific payment.
    ///
    /// Sends a `GET` request to `/payments/{paymentId}/routes`
    ///
    /// Arguments:
    /// - `payment_id`: Provide the ID of the related payment.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn payment_list_routes<'a>(
        &'a self,
        payment_id: &'a types::PaymentToken,
    ) -> Result<ResponseValue<types::PaymentListRoutesResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/payments/{}/routes",
            encode_path(&payment_id.to_string())
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
            .send(request, routes::Operation::PaymentListRoutes)
            .await?;
        routes::response::json(
            response,
            &[200u16],
            &[404u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Create a delayed route
    ///
    /// Create a route for a specific payment.
    /// The routed amount is credited to the account of your customer.
    ///
    /// Sends a `POST` request to `/payments/{paymentId}/routes`
    ///
    /// Arguments:
    /// - `payment_id`: Provide the ID of the related payment.
    pub async fn payment_create_route<'a>(
        &'a self,
        payment_id: &'a types::PaymentToken,
        body: &'a types::RouteCreateRequest,
    ) -> Result<ResponseValue<types::RouteCreateResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/payments/{}/routes",
            encode_path(&payment_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::POST, url)?;
        #[allow(unused_mut)]
        let mut request = request.json(&body).build()?;
        let response = self
            .send(request, routes::Operation::PaymentCreateRoute)
            .await?;
        routes::response::json(
            response,
            &[201u16],
            &[404u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Get a delayed route
    ///
    /// Retrieve a single route created for a specific payment.
    ///
    /// Sends a `GET` request to `/payments/{paymentId}/routes/{routeId}`
    ///
    /// Arguments:
    /// - `payment_id`: Provide the ID of the related payment.
    /// - `route_id`: Provide the ID of the route.
    pub async fn payment_get_route<'a>(
        &'a self,
        payment_id: &'a types::PaymentToken,
        route_id: &'a types::ConnectRouteToken,
    ) -> Result<ResponseValue<types::RouteGetResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/payments/{}/routes/{}",
            encode_path(&payment_id.to_string()),
            encode_path(&route_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request.build()?;
        let response = self
            .send(request, routes::Operation::PaymentGetRoute)
            .await?;
        routes::response::json(
            response,
            &[200u16],
            &[404u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }
}
