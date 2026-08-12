//! Generated chargebacks route methods.

use crate::{routes, types, Client, Error, ResponseValue};
use progenitor_client::encode_path;

/// Generated `chargebacks` route methods on [`crate::Client`].
///
/// Client-owned request policy on [`crate::Client`]:
/// - idempotency via [`crate::Client::with_idempotency_key`] (default: UUID v4 per request)
/// - `testmode` query via [`crate::Client::with_testmode`] on operations that support it
///
/// The resolved idempotency key is returned on the response envelope
/// ([`crate::ResponseValue`] / [`crate::ResponseEnvelope`]).
#[allow(clippy::all)]
impl Client {
    /// List payment chargebacks
    ///
    /// Retrieve the chargebacks initiated for a specific payment.
    ///
    /// The results are paginated.
    ///
    /// Sends a `GET` request to `/payments/{paymentId}/chargebacks`
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
    pub async fn list_chargebacks<'a>(
        &'a self,
        payment_id: &'a types::PaymentToken,
        embed: Option<&'a str>,
        from: Option<&'a types::ChargebackToken>,
        limit: Option<::std::num::NonZeroU64>,
    ) -> Result<ResponseValue<types::ListChargebacksResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/payments/{}/chargebacks",
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
        let response = self
            .send(request, routes::Operation::ListChargebacks)
            .await?;
        routes::response::json(
            response,
            &[200u16],
            &[400u16, 404u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Get payment chargeback
    ///
    /// Retrieve a single payment chargeback by its ID and the ID of its parent payment.
    ///
    /// Sends a `GET` request to `/payments/{paymentId}/chargebacks/{chargebackId}`
    ///
    /// Arguments:
    /// - `payment_id`: Provide the ID of the related payment.
    /// - `chargeback_id`: Provide the ID of the related chargeback.
    /// - `embed`: This endpoint allows embedding related API items by appending the following values via the `embed` query string
    /// parameter.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn get_chargeback<'a>(
        &'a self,
        payment_id: &'a types::PaymentToken,
        chargeback_id: &'a types::ChargebackToken,
        embed: Option<&'a str>,
    ) -> Result<ResponseValue<types::EntityChargeback>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/payments/{}/chargebacks/{}",
            encode_path(&payment_id.to_string()),
            encode_path(&chargeback_id.to_string())
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
        let response = self.send(request, routes::Operation::GetChargeback).await?;
        routes::response::json(
            response,
            &[200u16],
            &[404u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// List all chargebacks
    ///
    /// Retrieve all chargebacks initiated for all your payments.
    ///
    /// The results are paginated.
    ///
    /// Sends a `GET` request to `/chargebacks`
    ///
    /// Arguments:
    /// - `embed`: This endpoint allows embedding related API items by appending the following values via the `embed` query string
    /// parameter.
    /// - `from`: Provide an ID to start the result set from the item with the given ID and onwards. This allows you to paginate the
    /// result set.
    /// - `limit`: The maximum number of items to return. Defaults to 50 items.
    /// - `profile_id`: The identifier referring to the [profile](get-profile) you wish to
    /// retrieve chargebacks for.
    ///
    /// Most API credentials are linked to a single profile. In these cases the
    /// `profileId` is already implied.
    ///
    /// To retrieve all chargebacks across the organization, use an
    /// organization-level API credential and omit the `profileId` parameter.
    /// - `sort`: Used for setting the direction of the result set. Defaults to descending order, meaning the results are ordered from
    /// newest to oldest.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn list_all_chargebacks<'a>(
        &'a self,
        embed: Option<&'a str>,
        from: Option<&'a types::ChargebackToken>,
        limit: Option<::std::num::NonZeroU64>,
        profile_id: Option<&'a types::ProfileToken>,
        sort: Option<types::Sorting>,
    ) -> Result<ResponseValue<types::ListAllChargebacksResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint("/chargebacks");
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
            .send(request, routes::Operation::ListAllChargebacks)
            .await?;
        routes::response::json(
            response,
            &[200u16],
            &[400u16, 404u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }
}
