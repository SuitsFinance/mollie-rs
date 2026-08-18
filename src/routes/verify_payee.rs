//! Generated verify payee route methods.

use crate::{routes, types, Client, Error, ResponseValue};

/// Generated `verify payee` route methods on [`crate::Client`].
///
/// Client-owned request policy on [`crate::Client`]:
/// - idempotency via [`crate::Client::with_idempotency_key`] (default: UUID v4 per request)
/// - `testmode` query via [`crate::Client::with_testmode`] on operations that support it
///
/// The resolved idempotency key is returned on the response envelope
/// ([`crate::ResponseValue`] / [`crate::ResponseEnvelope`]).
#[allow(clippy::all)]
impl Client {
    /// Verify Payee
    ///
    /// > 🚧 Beta feature
    /// >
    /// > This feature is currently in beta testing, and the final specification may still change.
    ///
    /// Perform a Verification of Payee (VoP) check. This allows you to verify the account holder name against the
    /// records held by the receiving bank before initiating a transfer.
    ///
    /// The verification result indicates whether the provided name matches, closely matches, or does not match the
    /// name on file at the receiving bank. This helps prevent misdirected payments.
    ///
    /// ### Simulating verification scenarios in test mode
    ///
    /// In test mode, you can simulate various verification outcomes by adjusting the creditor name in the
    /// `creditorBankAccount.accountHolderName` property. This allows you to test all possible Verification of Payee
    /// results without needing special properties. The names are case insensitive.
    ///
    /// | Account holder name                    | Scenario                                      | Verification result | Suggested name |
    /// |----------------------------------------|-----------------------------------------------|---------------------|----------------|
    /// | `John Close Match`                     | Name closely matches the bank records          | `close-match`       | `John Match`   |
    /// | `John No Match`                        | Name does not match the bank records           | `no-match`          | —              |
    /// | `John Unavailable`                     | Verification is not available                  | `not-available`     | —              |
    /// | Any other name                         | Default: name matches the bank records         | `match`             | —              |
    ///
    /// Sends a `POST` request to `/business-accounts/payee-verifications`
    ///
    /// Arguments:
    pub async fn verify_payee<'a>(
        &'a self,
        body: &'a types::VerificationOfPayeeRequest,
    ) -> Result<ResponseValue<types::VerificationOfPayeeResponse>, Error<types::ErrorResponse>>
    {
        let url = self.endpoint("/business-accounts/payee-verifications");
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::POST, url)?;
        #[allow(unused_mut)]
        let mut request = request.json(&body).build()?;
        let response = self.send(request, routes::Operation::VerifyPayee).await?;
        routes::response::json(
            response,
            &[200u16],
            &[422u16, 429u16, 503u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }
}
