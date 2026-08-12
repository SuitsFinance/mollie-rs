//! Generated unmatched credit transfers route methods.

use crate::{routes, types, Client, Error, ResponseValue};
use progenitor_client::encode_path;

/// Generated `unmatched credit transfers` route methods on [`crate::Client`].
///
/// Client-owned request policy on [`crate::Client`]:
/// - idempotency via [`crate::Client::with_idempotency_key`] (default: UUID v4 per request)
/// - `testmode` query via [`crate::Client::with_testmode`] on operations that support it
///
/// The resolved idempotency key is returned on the response envelope
/// ([`crate::ResponseValue`] / [`crate::ResponseEnvelope`]).
#[allow(clippy::all)]
impl Client {
    /// List unmatched credit transfers
    ///
    /// > 🚧 Beta feature
    /// >
    /// > This feature is currently in private beta, and the final specification may still change.
    ///
    /// Retrieves a list of unmatched credit transfers for the profile.
    ///
    /// The results are paginated.
    ///
    /// Sends a `GET` request to `/unmatched-credit-transfers`
    ///
    /// Arguments:
    /// - `from`: Provide an ID to start the result set from the item with the given ID and onwards. This allows you to paginate the
    /// result set.
    /// - `limit`: The maximum number of items to return. Defaults to 50 items.
    pub async fn list_unmatched_credit_transfers<'a>(
        &'a self,
        from: Option<&'a str>,
        limit: Option<::std::num::NonZeroU64>,
    ) -> Result<
        ResponseValue<types::ListUnmatchedCreditTransfersResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = self.endpoint("/unmatched-credit-transfers");
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request
            .query(&progenitor_client::QueryParam::new("from", &from))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .build()?;
        self.reject_testmode_for("list_unmatched_credit_transfers")?;
        let response = self
            .send(request, routes::Operation::ListUnmatchedCreditTransfers)
            .await?;
        routes::response::json(
            response,
            &[200u16],
            &[400u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Get unmatched credit transfer
    ///
    /// > 🚧 Beta feature
    /// >
    /// > This feature is currently in private beta, and the final specification may still change.
    ///
    /// Retrieves a single unmatched credit transfer by its identifier.
    ///
    /// Sends a `GET` request to `/unmatched-credit-transfers/{unmatchedCreditTransferId}`
    ///
    /// Arguments:
    /// - `unmatched_credit_transfer_id`: Provide the ID of the related unmatched credit transfer.
    pub async fn get_unmatched_credit_transfer<'a>(
        &'a self,
        unmatched_credit_transfer_id: &'a types::UnmatchedCreditTransferToken,
    ) -> Result<ResponseValue<types::EntityUnmatchedCreditTransfer>, Error<types::ErrorResponse>>
    {
        let url = self.endpoint(format_args!(
            "/unmatched-credit-transfers/{}",
            encode_path(&unmatched_credit_transfer_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request.build()?;
        self.reject_testmode_for("get_unmatched_credit_transfer")?;
        let response = self
            .send(request, routes::Operation::GetUnmatchedCreditTransfer)
            .await?;
        routes::response::json(
            response,
            &[200u16],
            &[404u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Match unmatched credit transfer
    ///
    /// > 🚧 Beta feature
    /// >
    /// > This feature is currently in private beta, and the final specification may still change.
    ///
    /// Matches an unmatched credit transfer to one or more payments, settling the funds accordingly.
    ///
    /// Sends a `POST` request to `/unmatched-credit-transfers/{unmatchedCreditTransferId}/match`
    ///
    /// Arguments:
    /// - `unmatched_credit_transfer_id`: Provide the ID of the related unmatched credit transfer.
    pub async fn match_unmatched_credit_transfer<'a>(
        &'a self,
        unmatched_credit_transfer_id: &'a types::UnmatchedCreditTransferToken,
        body: &'a types::UnmatchedCreditTransferMatchRequest,
    ) -> Result<
        ResponseValue<types::UnmatchedCreditTransferActionResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = self.endpoint(format_args!(
            "/unmatched-credit-transfers/{}/match",
            encode_path(&unmatched_credit_transfer_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::POST, url)?;
        #[allow(unused_mut)]
        let mut request = request.json(&body).build()?;
        let response = self
            .send(request, routes::Operation::MatchUnmatchedCreditTransfer)
            .await?;
        routes::response::json(
            response,
            &[201u16],
            &[404u16, 422u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Return unmatched credit transfer
    ///
    /// > 🚧 Beta feature
    /// >
    /// > This feature is currently in private beta, and the final specification may still change.
    ///
    /// Returns an unmatched credit transfer, sending the funds back to the original sender.
    ///
    /// Sends a `POST` request to `/unmatched-credit-transfers/{unmatchedCreditTransferId}/return`
    ///
    /// Arguments:
    /// - `unmatched_credit_transfer_id`: Provide the ID of the related unmatched credit transfer.
    pub async fn return_unmatched_credit_transfer<'a>(
        &'a self,
        unmatched_credit_transfer_id: &'a types::UnmatchedCreditTransferToken,
    ) -> Result<
        ResponseValue<types::UnmatchedCreditTransferActionResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = self.endpoint(format_args!(
            "/unmatched-credit-transfers/{}/return",
            encode_path(&unmatched_credit_transfer_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::POST, url)?;
        #[allow(unused_mut)]
        let mut request = request.build()?;
        let response = self
            .send(request, routes::Operation::ReturnUnmatchedCreditTransfer)
            .await?;
        routes::response::json(
            response,
            &[201u16],
            &[404u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }
}
