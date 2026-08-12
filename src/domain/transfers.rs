//! Business-account transfer facade (SEPA credit).
//!
//! `create_transfer` is an **IdempotentWrite** that also requires client
//! signing headers. The facade never invents signatures or idempotency keys —
//! callers must supply a sticky [`IdempotencyKey`] and signature material.
#![warn(missing_docs)]

use crate::domain::common::client_with_key;
use crate::types::{self, TransferResponse};
use crate::{
    CreateTransferRequired, IdempotencyKey, IntoMollieFuture, MollieClient, MollieError,
    MollieResponse, MollieResult,
};

/// Transfer operations scoped to a [`MollieClient`].
#[derive(Debug)]
pub struct TransfersApi<'a> {
    client: &'a MollieClient,
}

impl MollieClient {
    /// Returns the business-account transfers facade.
    pub fn transfers(&self) -> TransfersApi<'_> {
        TransfersApi { client: self }
    }
}

/// Client-side signature headers required by Mollie transfer create.
#[derive(Clone, Debug)]
pub struct TransferClientSignature<'a> {
    /// `X-Client-Signature` value.
    pub signature: &'a str,
    /// `X-Client-Signed-At` ISO-8601 timestamp.
    pub signed_at: &'a str,
}

impl TransfersApi<'_> {
    /// Creates a SEPA transfer from a validated builder.
    ///
    /// # Errors
    ///
    /// Returns [`MollieError::InvalidRequest`] when the sticky key or signature
    /// headers are empty (fail closed — never auto-generate financial keys).
    pub async fn create(
        &self,
        required: CreateTransferRequired,
        key: &IdempotencyKey,
        signature: TransferClientSignature<'_>,
    ) -> MollieResponse<TransferResponse> {
        let body = required.into_request()?;
        self.create_raw(&body, key, signature).await
    }

    /// Creates a transfer from a generated body (advanced).
    pub async fn create_raw(
        &self,
        body: &types::TransferRequest,
        key: &IdempotencyKey,
        signature: TransferClientSignature<'_>,
    ) -> MollieResponse<TransferResponse> {
        require_non_empty("Idempotency-Key", key.as_str())?;
        require_non_empty("X-Client-Signature", signature.signature)?;
        require_non_empty("X-Client-Signed-At", signature.signed_at)?;
        client_with_key(self.client, Some(key.clone()))
            .create_transfer(key.as_str(), signature.signature, signature.signed_at, body)
            .into_mollie_result()
            .await
    }

    /// Fetches a transfer by id (`batrf_…`).
    pub async fn get(&self, transfer_id: &str) -> MollieResponse<TransferResponse> {
        let token = types::BusinessAccountTransferToken(transfer_id.to_string());
        self.client.get_transfer(&token).into_mollie_result().await
    }
}

fn require_non_empty(label: &str, value: &str) -> MollieResult<()> {
    if value.trim().is_empty() {
        return Err(MollieError::invalid_request(format!(
            "{label} is required for business-account transfers"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::TransferSchemeType;
    use crate::{operation_safety_profile, Money, RetryClass};

    #[test]
    fn create_transfer_is_idempotent_write() {
        let p = operation_safety_profile("create_transfer").unwrap();
        assert_eq!(p.retry_class, RetryClass::IdempotentWrite);
    }

    #[test]
    fn builder_rejects_bad_iban_and_empty_name() {
        let amount = Money::new("EUR", "25.00").unwrap();
        assert!(CreateTransferRequired::new(
            amount.clone(),
            "short",
            "Jan Jansen",
            "NL02ABNA0123456789",
            TransferSchemeType::SepaCredit,
        )
        .is_err());
        assert!(CreateTransferRequired::new(
            amount,
            "NL55MLLE0123456789",
            "   ",
            "NL02ABNA0123456789",
            TransferSchemeType::SepaCredit,
        )
        .is_err());
    }

    #[test]
    fn builder_serializes_write_fields() {
        let body = CreateTransferRequired::new(
            Money::new("EUR", "25.00").unwrap(),
            "NL55MLLE0123456789",
            "Jan Jansen",
            "NL02ABNA0123456789",
            TransferSchemeType::SepaCreditInst,
        )
        .unwrap()
        .with_description("Invoice 12345")
        .unwrap()
        .into_request()
        .unwrap();
        let value = serde_json::to_value(&body).unwrap();
        assert_eq!(value["amount"]["value"], "25.00");
        assert_eq!(value["debtorIban"], "NL55MLLE0123456789");
        assert_eq!(value["creditor"]["fullName"], "Jan Jansen");
        assert_eq!(value["transferScheme"]["type"], "sepa-credit-inst");
        assert!(value.get("id").is_none() || value["id"].is_null());
        assert!(value.get("status").is_none() || value["status"].is_null());
    }

    #[test]
    fn rejects_empty_signature_material() {
        let err = require_non_empty("X-Client-Signature", "  ").unwrap_err();
        assert!(err.to_string().contains("X-Client-Signature"));
    }
}
