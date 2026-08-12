//! Generated payouts route methods.

use crate::{routes, types, Client, Error, ResponseValue};
use progenitor_client::encode_path;

/// Generated `payouts` route methods on [`crate::Client`].
///
/// Client-owned request policy on [`crate::Client`]:
/// - idempotency via [`crate::Client::with_idempotency_key`] (default: UUID v4 per request)
/// - `testmode` query via [`crate::Client::with_testmode`] on operations that support it
///
/// The resolved idempotency key is returned on the response envelope
/// ([`crate::ResponseValue`] / [`crate::ResponseEnvelope`]).
#[allow(clippy::all)]
impl Client {
    /// List payouts
    ///
    /// Retrieve a list of all payouts for your organization, including payouts initiated automatically by the
    /// balance's payout schedule and payouts requested via the API or dashboard.
    ///
    /// Only payouts created on or after April 1st, 2026 are returned.
    ///
    /// The results are paginated. Use the `from` query parameter together with `_links.next` to iterate through
    /// the full result set.
    ///
    /// Sends a `GET` request to `/payouts`
    ///
    /// Arguments:
    /// - `balance_id`: Return only payouts for the balance with the given ID. The value must be a valid balance
    /// token in the format `bal_*`.
    /// - `from`: Provide an ID to start the result set from the item with the given ID and onwards. This allows you to paginate the
    /// result set.
    /// - `limit`: The maximum number of items to return. Defaults to 50 items.
    /// - `sort`: Used for setting the direction of the result set. Defaults to descending order, meaning the results are ordered from
    /// newest to oldest.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn list_payouts<'a>(
        &'a self,
        balance_id: Option<&'a types::ListPayoutsBalanceId>,
        from: Option<&'a str>,
        limit: Option<::std::num::NonZeroU64>,
        sort: Option<types::Sorting>,
    ) -> Result<ResponseValue<types::ListPayoutsResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint("/payouts");
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request
            .query(&progenitor_client::QueryParam::new(
                "balanceId",
                &balance_id,
            ))
            .query(&progenitor_client::QueryParam::new("from", &from))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("sort", &sort))
            .query(&progenitor_client::QueryParam::new(
                "testmode",
                &self.testmode(),
            ))
            .build()?;
        self.reject_testmode_for("list_payouts")?;
        let response = self.send(request, routes::Operation::ListPayouts).await?;
        routes::response::json(
            response,
            &[200u16],
            &[400u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Create payout
    ///
    /// Request a payout from one of your balances to the balance's configured bank account.
    ///
    /// The payout will be executed on the next scheduled business day. If no `amount` is specified, the full available
    /// balance minus any configured balance reserve is paid out.
    ///
    /// Once the payout is created with status `requested`, you can cancel it via the
    /// [Cancel payout](cancel-payout) endpoint, up until the payout moves to `initiated`.
    ///
    /// Creating a payout via the API automatically sets the balance's `transferFrequency` to `never`,
    /// pausing any previously configured automatic settlement schedule. To resume automatic settlements,
    /// update the transfer frequency via the dashboard.
    ///
    /// ### Webhooks
    ///
    /// Subscribe to the following webhook events to track payout status changes. See the
    /// [Webhook Subscriptions API](list-webhooks) for details on subscribing.
    ///
    /// | Event | Description |
    /// |---|---|
    /// | `payout.initiated` | The payout is being executed and funds are reserved. |
    /// | `payout.processing-at-bank` | The payout has been submitted to the bank. |
    /// | `payout.completed` | The payout has been sent to the destination bank account. |
    /// | `payout.canceled` | The payout was canceled via the API before being submitted to the bank. |
    /// | `payout.failed` | The payout failed after creation, including bank rejections and post-submission cancellations. |
    ///
    /// ### Payout failure reasons
    ///
    /// A payout request may fail immediately if one of the following conditions applies:
    ///
    /// - A payout is already scheduled for the next business day for this balance.
    /// - The balance has insufficient funds.
    /// - The balance is not active.
    /// - Payouts are blocked for this organization.
    /// - The balance has queued refunds.
    /// - One of the organization's balances is below the negative balance threshold.
    /// - The payout destination (bank account) is invalid or not configured.
    ///
    /// Sends a `POST` request to `/payouts`
    ///
    /// Arguments:
    pub async fn create_payout<'a>(
        &'a self,
        body: &'a types::PayoutRequest,
    ) -> Result<ResponseValue<types::EntityPayoutResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint("/payouts");
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::POST, url)?;
        #[allow(unused_mut)]
        let mut request = request.json(&body).build()?;
        let response = self.send(request, routes::Operation::CreatePayout).await?;
        routes::response::json(
            response,
            &[201u16],
            &[422u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Get payout
    ///
    /// Retrieve a single payout by its ID.
    ///
    /// Sends a `GET` request to `/payouts/{payoutId}`
    ///
    /// Arguments:
    /// - `payout_id`: Provide the ID of the payout.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn get_payout<'a>(
        &'a self,
        payout_id: &'a str,
    ) -> Result<ResponseValue<types::EntityPayoutResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/payouts/{}",
            encode_path(&payout_id.to_string())
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
        self.reject_testmode_for("get_payout")?;
        let response = self.send(request, routes::Operation::GetPayout).await?;
        routes::response::json(
            response,
            &[200u16],
            &[404u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Cancel payout
    ///
    /// Cancel a payout. A payout can only be canceled while it has the status `requested`. Once the payout moves
    /// to `initiated`, it is too late to cancel.
    ///
    /// The canceled payout object is returned with the status set to `canceled`.
    ///
    /// Sends a `DELETE` request to `/payouts/{payoutId}`
    ///
    /// Arguments:
    /// - `payout_id`: Provide the ID of the payout.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn cancel_payout<'a>(
        &'a self,
        payout_id: &'a str,
    ) -> Result<ResponseValue<types::EntityPayoutResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/payouts/{}",
            encode_path(&payout_id.to_string())
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
        let response = self.send(request, routes::Operation::CancelPayout).await?;
        routes::response::json(
            response,
            &[200u16],
            &[404u16, 409u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }
}
