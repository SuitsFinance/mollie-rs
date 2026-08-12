//! Generated transfers route methods.

use crate::{routes, types, Client, Error, ResponseValue};
use progenitor_client::encode_path;

/// Generated `transfers` route methods on [`crate::Client`].
///
/// Client-owned request policy on [`crate::Client`]:
/// - idempotency via [`crate::Client::with_idempotency_key`] (default: UUID v4 per request)
/// - `testmode` query via [`crate::Client::with_testmode`] on operations that support it
///
/// The resolved idempotency key is returned on the response envelope
/// ([`crate::ResponseValue`] / [`crate::ResponseEnvelope`]).
#[allow(clippy::all)]
impl Client {
    /// Create transfer
    ///
    /// > 🚧 Beta feature
    /// >
    /// > This feature is currently in beta testing, and the final specification may still change.
    ///
    /// Create a SEPA Credit Transfer from your Mollie Business Account.
    ///
    /// To initiate a transfer, you must provide the transfer scheme, the amount, the debtor IBAN (your Mollie Business
    /// Account IBAN), and the creditor (recipient) details.
    ///
    /// Each request must include an `Idempotency-Key` header to prevent duplicate transfers, and must be signed using the
    /// `X-Client-Signature` and `X-Client-Signed-At` headers.
    ///
    /// ### Simulating transfer scenarios in test mode
    ///
    /// In test mode, you can simulate various transfer scenarios by adjusting the transfer amount. This allows you to
    /// mimic the typical status progression of a real-world transfer. Note that a transfer's progression will stop once
    /// it reaches a final status: `blocked`, `failed`, `processed`, or `returned`.
    ///
    /// | Amount  | Scenario                                            | Webhook sequence                                                                                                                                                   |
    /// |---------|-----------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------|
    /// | `11.00` | Transfer initiated, pending review by Mollie        | `business-account-transfer.requested` → `business-account-transfer.initiated` → `business-account-transfer.pending-review`                                         |
    /// | `12.00` | Transfer initiated, blocked by Mollie               | `business-account-transfer.requested` → `business-account-transfer.initiated` → `business-account-transfer.pending-review` → `business-account-transfer.blocked`   |
    /// | `13.00` | Transfer initiated, failed on scheme submission     | `business-account-transfer.requested` → `business-account-transfer.initiated` → `business-account-transfer.failed`                                                 |
    /// | `14.00` | Transfer processed, then returned by receiving bank | `business-account-transfer.requested` → `business-account-transfer.initiated` → `business-account-transfer.processed` → `business-account-transfer.returned`       |
    /// | Other   | Default: transfer is processed                      | `business-account-transfer.requested` → `business-account-transfer.initiated` → `business-account-transfer.processed`                                              |
    ///
    /// Sends a `POST` request to `/business-accounts/transfers`
    ///
    /// Arguments:
    ///
    /// View the [public documentation](https://docs.mollie.com/reference/api-idempotency#using-an-idempotency-key)
    /// to learn more.
    /// - `x_client_signature`: A cryptographic signature of the request payload, used to verify the authenticity of the transfer request.
    /// - `x_client_signed_at`: The timestamp (in ISO 8601 format) indicating when the client signed the request. Used in conjunction with
    /// `X-Client-Signature` for request verification.
    pub async fn create_transfer<'a>(
        &'a self,
        idempotency_key: &'a str,
        x_client_signature: &'a str,
        x_client_signed_at: &'a str,
        body: &'a types::TransferRequest,
    ) -> Result<ResponseValue<types::TransferResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint("/business-accounts/transfers");
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::POST, url)?;
        // Prefer explicit caller key when provided; fall back to client-resolved key.
        let resolved_idempotency_key = if idempotency_key.is_empty() {
            resolved_idempotency_key
        } else {
            idempotency_key.to_string()
        };
        #[allow(unused_mut)]
        let mut request = request
            .header("Idempotency-Key", &resolved_idempotency_key)
            .header("X-Client-Signature", x_client_signature)
            .header("X-Client-Signed-At", x_client_signed_at)
            .json(&body)
            .build()?;
        let response = self
            .send(request, routes::Operation::CreateTransfer)
            .await?;
        routes::response::json(
            response,
            &[201u16],
            &[422u16, 429u16, 503u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Get transfer
    ///
    /// > 🚧 Beta feature
    /// >
    /// > This feature is currently in beta testing, and the final specification may still change.
    ///
    /// Retrieve a single transfer object by its transfer ID. This allows you to check the current status
    /// and details of a previously created transfer.
    ///
    /// Sends a `GET` request to `/business-accounts/transfers/{businessAccountsTransferId}`
    ///
    /// Arguments:
    /// - `business_accounts_transfer_id`: Provide the ID of the related transfer.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn get_transfer<'a>(
        &'a self,
        business_accounts_transfer_id: &'a types::BusinessAccountTransferToken,
    ) -> Result<ResponseValue<types::TransferResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/business-accounts/transfers/{}",
            encode_path(&business_accounts_transfer_id.to_string())
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
        let response = self.send(request, routes::Operation::GetTransfer).await?;
        routes::response::json(
            response,
            &[200u16],
            &[404u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }
}
