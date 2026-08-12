//! Generated connect route methods.

use crate::{routes, types, Client, Error, ResponseValue};
use progenitor_client::encode_path;

/// Generated `connect` route methods on [`crate::Client`].
///
/// Client-owned request policy on [`crate::Client`]:
/// - idempotency via [`crate::Client::with_idempotency_key`] (default: UUID v4 per request)
/// - `testmode` query via [`crate::Client::with_testmode`] on operations that support it
///
/// The resolved idempotency key is returned on the response envelope
/// ([`crate::ResponseValue`] / [`crate::ResponseEnvelope`]).
#[allow(clippy::all)]
impl Client {
    /// List all Connect balance transfers
    ///
    /// Returns a paginated list of balance transfers associated with your organization. These may be a balance transfer that was received or sent from your balance, or a balance transfer that you initiated on behalf of your clients. If no balance transfers are available, the resulting array will be empty. This request should never throw an error.
    ///
    /// Sends a `GET` request to `/connect/balance-transfers`
    ///
    /// Arguments:
    /// - `from`: Provide an ID to start the result set from the item with the given ID and onwards. This allows you to paginate the
    /// result set.
    /// - `limit`: The maximum number of items to return. Defaults to 50 items.
    /// - `sort`: Used for setting the direction of the result set. Defaults to descending order, meaning the results are ordered from
    /// newest to oldest.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn list_connect_balance_transfers<'a>(
        &'a self,
        from: Option<&'a str>,
        limit: Option<::std::num::NonZeroU64>,
        sort: Option<types::Sorting>,
    ) -> Result<
        ResponseValue<types::ListConnectBalanceTransfersResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = self.endpoint("/connect/balance-transfers");
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
            .send(request, routes::Operation::ListConnectBalanceTransfers)
            .await?;
        routes::response::json(
            response,
            &[200u16],
            &[400u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Create a Connect balance transfer
    ///
    /// This API endpoint allows you to create a balance transfer from your organization's balance to a connected organization's balance, or vice versa.
    /// You can also create a balance transfer between two connected organizations.
    /// To create a balance transfer, you must be authenticated as the source organization, and the destination organization must be a connected organization
    /// that has authorized the `balance-transfers.write` scope for your organization.
    ///
    /// Sends a `POST` request to `/connect/balance-transfers`
    ///
    /// Arguments:
    pub async fn create_connect_balance_transfer<'a>(
        &'a self,
        body: &'a types::EntityBalanceTransfer,
    ) -> Result<ResponseValue<types::EntityBalanceTransferResponse>, Error<types::ErrorResponse>>
    {
        let url = self.endpoint("/connect/balance-transfers");
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::POST, url)?;
        #[allow(unused_mut)]
        let mut request = request.json(&body).build()?;
        let response = self
            .send(request, routes::Operation::CreateConnectBalanceTransfer)
            .await?;
        routes::response::json(
            response,
            &[201u16],
            &[422u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Get a Connect balance transfer
    ///
    /// Retrieve a single Connect balance transfer object by its ID.
    ///
    /// Sends a `GET` request to `/connect/balance-transfers/{balanceTransferId}`
    ///
    /// Arguments:
    /// - `balance_transfer_id`: Provide the ID of the related balance transfer.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn get_connect_balance_transfer<'a>(
        &'a self,
        balance_transfer_id: &'a types::ConnectBalanceTransferToken,
    ) -> Result<ResponseValue<types::EntityBalanceTransferResponse>, Error<types::ErrorResponse>>
    {
        let url = self.endpoint(format_args!(
            "/connect/balance-transfers/{}",
            encode_path(&balance_transfer_id.to_string())
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
            .send(request, routes::Operation::GetConnectBalanceTransfer)
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
