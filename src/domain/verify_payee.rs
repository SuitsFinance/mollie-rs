//! Verification of Payee (VoP) facade.
//!
//! Classified **NonRetryableWrite** — the transport will not auto-retry.
#![warn(missing_docs)]

use crate::domain::common::client_with_key;
use crate::types::{self, VerificationOfPayeeResponse};
use crate::{IdempotencyKey, IntoMollieFuture, MollieClient, MollieResponse, VerifyPayeeRequired};

/// Verification-of-payee operations scoped to a [`MollieClient`].
#[derive(Debug)]
pub struct VerifyPayeeApi<'a> {
    client: &'a MollieClient,
}

impl MollieClient {
    /// Returns the Verification of Payee facade.
    ///
    /// Named `payee_verifications` so it does not shadow the generated Tier-G
    /// method [`crate::Client::verify_payee`].
    pub fn payee_verifications(&self) -> VerifyPayeeApi<'_> {
        VerifyPayeeApi { client: self }
    }
}

impl VerifyPayeeApi<'_> {
    /// Performs a VoP check from a validated builder.
    pub async fn verify(
        &self,
        required: VerifyPayeeRequired,
        key: Option<IdempotencyKey>,
    ) -> MollieResponse<VerificationOfPayeeResponse> {
        let body = required.into_request();
        self.verify_raw(&body, key).await
    }

    /// Performs a VoP check from a generated body (advanced).
    pub async fn verify_raw(
        &self,
        body: &types::VerificationOfPayeeRequest,
        key: Option<IdempotencyKey>,
    ) -> MollieResponse<VerificationOfPayeeResponse> {
        client_with_key(self.client, key)
            .verify_payee(body)
            .into_mollie_result()
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{operation_safety_profile, RetryClass};

    #[test]
    fn verify_payee_is_non_retryable() {
        let p = operation_safety_profile("verify_payee").unwrap();
        assert!(p.retry_class.is_non_retryable());
        assert_eq!(p.retry_class, RetryClass::NonRetryableWrite);
    }

    #[test]
    fn builder_rejects_empty_name() {
        assert!(VerifyPayeeRequired::new("", "NL02ABNA0123456789").is_err());
        assert!(VerifyPayeeRequired::new("Jan Jansen", "bad").is_err());
    }

    #[test]
    fn builder_serializes_iban_format() {
        let body = VerifyPayeeRequired::new("Jan Jansen", "NL02ABNA0123456789")
            .unwrap()
            .into_request();
        let value = serde_json::to_value(&body).unwrap();
        assert_eq!(value["creditorBankAccount"]["format"], "iban");
        assert_eq!(
            value["creditorBankAccount"]["accountHolderName"],
            "Jan Jansen"
        );
    }
}
