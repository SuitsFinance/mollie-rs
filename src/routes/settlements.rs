//! Generated settlements route methods.

use crate::{routes, types, Client, Error, ResponseValue};
use progenitor_client::encode_path;

/// Generated `settlements` route methods on [`crate::Client`].
///
/// Client-owned request policy on [`crate::Client`]:
/// - idempotency via [`crate::Client::with_idempotency_key`] (default: UUID v4 per request)
/// - `testmode` query via [`crate::Client::with_testmode`] on operations that support it
///
/// The resolved idempotency key is returned on the response envelope
/// ([`crate::ResponseValue`] / [`crate::ResponseEnvelope`]).
#[allow(clippy::all)]
impl Client {
    /// List settlements
    ///
    /// Retrieve a list of all your settlements.
    ///
    /// The results are paginated.
    ///
    /// Sends a `GET` request to `/settlements`
    ///
    /// Arguments:
    /// - `balance_id`: Provide the token of the balance to filter the settlements by. This is
    /// the balance token that the settlement was settled to.
    /// - `currencies`: Provides the currencies to retrieve the settlements. It accepts multiple currencies in a comma-separated format.
    /// - `from`: Provide an ID to start the result set from the item with the given ID and onwards. This allows you to paginate the
    /// result set.
    /// - `limit`: The maximum number of items to return. Defaults to 50 items.
    /// - `month`: Provide the month to query the settlements. Must be used combined with `year` parameter
    /// - `year`: Provide the year to query the settlements. Must be used combined with `month` parameter
    pub async fn list_settlements<'a>(
        &'a self,
        balance_id: Option<&'a types::BalanceToken>,
        currencies: Option<types::Currencies>,
        from: Option<&'a str>,
        limit: Option<::std::num::NonZeroU64>,
        month: Option<&'a str>,
        year: Option<&'a str>,
    ) -> Result<ResponseValue<types::ListSettlementsResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint("/settlements");
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request
            .query(&progenitor_client::QueryParam::new(
                "balanceId",
                &balance_id,
            ))
            .query(&progenitor_client::QueryParam::new(
                "currencies",
                &currencies,
            ))
            .query(&progenitor_client::QueryParam::new("from", &from))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("month", &month))
            .query(&progenitor_client::QueryParam::new("year", &year))
            .build()?;
        self.reject_testmode_for("list_settlements")?;
        let response = self
            .send(request, routes::Operation::ListSettlements)
            .await?;
        routes::response::json(
            response,
            &[200u16],
            &[400u16, 404u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }

    /// Get settlement
    ///
    /// Retrieve a single settlement by its ID.
    ///
    /// To lookup settlements by their bank reference, replace the ID in the URL by
    /// a reference. For example: `1234567.2404.03`.
    ///
    /// A settlement represents a transfer of your balance funds to your external bank account.
    ///
    /// Settlements will typically include a report that details what balance transactions have taken place between this
    /// settlement and the previous one.
    ///
    /// For more accurate bookkeeping, refer to the [balance report](get-balance-report) endpoint or the
    /// [balance transactions](list-balance-transactions) endpoint.
    ///
    /// Sends a `GET` request to `/settlements/{settlementId}`
    ///
    /// Arguments:
    /// - `settlement_id`: Provide the ID of the related settlement.
    pub async fn get_settlement<'a>(
        &'a self,
        settlement_id: &'a types::SettlementToken,
    ) -> Result<ResponseValue<types::EntitySettlement>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/settlements/{}",
            encode_path(&settlement_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request.build()?;
        self.reject_testmode_for("get_settlement")?;
        let response = self.send(request, routes::Operation::GetSettlement).await?;
        routes::response::json(
            response,
            &[200u16],
            &[404u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }

    /// Get open settlement
    ///
    /// Retrieve the details of the open balance of the organization. This will return a settlement object representing your
    /// organization's balance.
    ///
    /// For a complete reference of the settlement object, refer to the [Get settlement endpoint](get-settlement)
    /// documentation.
    ///
    /// For more accurate bookkeeping, refer to the [balance report](get-balance-report) endpoint or the
    /// [balance transactions](list-balance-transactions) endpoint.
    ///
    /// Sends a `GET` request to `/settlements/open`
    ///
    /// Arguments:
    pub async fn get_open_settlement<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::EntitySettlement>, Error<types::ErrorResponse>> {
        let url = self.endpoint("/settlements/open");
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request.build()?;
        self.reject_testmode_for("get_open_settlement")?;
        let response = self
            .send(request, routes::Operation::GetOpenSettlement)
            .await?;
        routes::response::json(
            response,
            &[200u16],
            &[429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }

    /// Get next settlement
    ///
    /// Retrieve the details of the current settlement, that has not yet been paid out.
    ///
    /// For a complete reference of the settlement object, refer to the [Get settlement endpoint](get-settlement)
    /// documentation.
    ///
    /// For more accurate bookkeeping, refer to the [balance report](get-balance-report) endpoint or the
    /// [balance transactions](list-balance-transactions) endpoint.
    ///
    /// Sends a `GET` request to `/settlements/next`
    ///
    /// Arguments:
    pub async fn get_next_settlement<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::EntitySettlement>, Error<types::ErrorResponse>> {
        let url = self.endpoint("/settlements/next");
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request.build()?;
        self.reject_testmode_for("get_next_settlement")?;
        let response = self
            .send(request, routes::Operation::GetNextSettlement)
            .await?;
        routes::response::json(
            response,
            &[200u16],
            &[429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }

    /// List settlement payments
    ///
    /// Retrieve all payments included in the given settlement.
    ///
    /// The response is in the same format as the response of the [List payments endpoint](list-payments).
    ///
    /// For capture-based payment methods such as Klarna, the payments are not listed here. Refer to the
    /// [List captures endpoint](list-captures) endpoint instead.
    ///
    /// Sends a `GET` request to `/settlements/{settlementId}/payments`
    ///
    /// Arguments:
    /// - `settlement_id`: Provide the ID of the related settlement.
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
    pub async fn list_settlement_payments<'a>(
        &'a self,
        settlement_id: &'a types::SettlementToken,
        from: Option<&'a types::PaymentToken>,
        limit: Option<::std::num::NonZeroU64>,
        profile_id: Option<&'a types::ProfileToken>,
        sort: Option<types::Sorting>,
    ) -> Result<ResponseValue<types::ListSettlementPaymentsResponse>, Error<types::ErrorResponse>>
    {
        let url = self.endpoint(format_args!(
            "/settlements/{}/payments",
            encode_path(&settlement_id.to_string())
        ));
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
            .build()?;
        self.reject_testmode_for("list_settlement_payments")?;
        let response = self
            .send(request, routes::Operation::ListSettlementPayments)
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

    /// List settlement captures
    ///
    /// Retrieve all captures included in the given settlement.
    ///
    /// The response is in the same format as the response of the [List captures endpoint](list-captures).
    ///
    /// Sends a `GET` request to `/settlements/{settlementId}/captures`
    ///
    /// Arguments:
    /// - `settlement_id`: Provide the ID of the related settlement.
    /// - `embed`: This endpoint allows embedding related API items by appending the following values via the `embed` query string
    /// parameter.
    /// - `from`: Provide an ID to start the result set from the item with the given ID and onwards. This allows you to paginate
    /// the result set.
    /// - `limit`: The maximum number of items to return. Defaults to 50 items.
    pub async fn list_settlement_captures<'a>(
        &'a self,
        settlement_id: &'a types::SettlementToken,
        embed: Option<&'a str>,
        from: Option<&'a types::CaptureToken>,
        limit: Option<::std::num::NonZeroU64>,
    ) -> Result<ResponseValue<types::ListSettlementCapturesResponse>, Error<types::ErrorResponse>>
    {
        let url = self.endpoint(format_args!(
            "/settlements/{}/captures",
            encode_path(&settlement_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request
            .query(&progenitor_client::QueryParam::new("embed", &embed))
            .query(&progenitor_client::QueryParam::new("from", &from))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .build()?;
        self.reject_testmode_for("list_settlement_captures")?;
        let response = self
            .send(request, routes::Operation::ListSettlementCaptures)
            .await?;
        routes::response::json(
            response,
            &[200u16],
            &[400u16, 404u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }

    /// List settlement refunds
    ///
    /// Retrieve all refunds 'deducted' from the given settlement.
    ///
    /// The response is in the same format as the response of the [List refunds endpoint](list-refunds).
    ///
    /// Sends a `GET` request to `/settlements/{settlementId}/refunds`
    ///
    /// Arguments:
    /// - `settlement_id`: Provide the ID of the related settlement.
    /// - `embed`: This endpoint allows embedding related API items by appending the following values via the `embed` query string
    /// parameter.
    /// - `from`: Provide an ID to start the result set from the item with the given ID and onwards. This allows you to paginate
    /// the result set.
    /// - `limit`: The maximum number of items to return. Defaults to 50 items.
    pub async fn list_settlement_refunds<'a>(
        &'a self,
        settlement_id: &'a types::SettlementToken,
        embed: Option<&'a str>,
        from: Option<&'a types::RefundToken>,
        limit: Option<::std::num::NonZeroU64>,
    ) -> Result<ResponseValue<types::ListSettlementRefundsResponse>, Error<types::ErrorResponse>>
    {
        let url = self.endpoint(format_args!(
            "/settlements/{}/refunds",
            encode_path(&settlement_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request
            .query(&progenitor_client::QueryParam::new("embed", &embed))
            .query(&progenitor_client::QueryParam::new("from", &from))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .build()?;
        self.reject_testmode_for("list_settlement_refunds")?;
        let response = self
            .send(request, routes::Operation::ListSettlementRefunds)
            .await?;
        routes::response::json(
            response,
            &[200u16],
            &[400u16, 404u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }

    /// List settlement chargebacks
    ///
    /// Retrieve all chargebacks 'deducted' from the given settlement.
    ///
    /// The response is in the same format as the response of the [List chargebacks endpoint](list-chargebacks).
    ///
    /// Sends a `GET` request to `/settlements/{settlementId}/chargebacks`
    ///
    /// Arguments:
    /// - `settlement_id`: Provide the ID of the related settlement.
    /// - `embed`: This endpoint allows embedding related API items by appending the following values via the `embed` query string
    /// parameter.
    /// - `from`: Provide an ID to start the result set from the item with the given ID and onwards. This allows you to paginate the
    /// result set.
    /// - `limit`: The maximum number of items to return. Defaults to 50 items.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn list_settlement_chargebacks<'a>(
        &'a self,
        settlement_id: &'a types::SettlementToken,
        embed: Option<&'a str>,
        from: Option<&'a types::ChargebackToken>,
        limit: Option<::std::num::NonZeroU64>,
    ) -> Result<ResponseValue<types::ListSettlementChargebacksResponse>, Error<types::ErrorResponse>>
    {
        let url = self.endpoint(format_args!(
            "/settlements/{}/chargebacks",
            encode_path(&settlement_id.to_string())
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
        self.reject_testmode_for("list_settlement_chargebacks")?;
        let response = self
            .send(request, routes::Operation::ListSettlementChargebacks)
            .await?;
        routes::response::json(
            response,
            &[200u16],
            &[400u16, 404u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }
}
