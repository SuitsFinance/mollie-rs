//! Webhook workflow helpers (classic + Next-gen + event fetch).
#![warn(missing_docs)]

use crate::types;
use crate::{
    IntoMollieFuture, MollieClient, MollieResponse, WebhookNotification, WebhookVerifier,
    WebhookVerifyFailure,
};

/// Webhook helpers scoped to a [`MollieClient`].
#[derive(Debug)]
pub struct WebhooksApi<'a> {
    client: &'a MollieClient,
}

impl MollieClient {
    /// Returns the webhooks domain facade.
    pub fn webhooks(&self) -> WebhooksApi<'_> {
        WebhooksApi { client: self }
    }
}

impl WebhooksApi<'_> {
    /// Parses a classic form-encoded Mollie callback (`id=…`).
    ///
    /// Does **not** prove authenticity. Refetch the resource via the API.
    pub fn parse_classic(body: impl AsRef<[u8]>) -> crate::MollieResult<WebhookNotification> {
        WebhookNotification::parse_form_urlencoded(body)
    }

    /// Verifies a Next-gen raw body using [`WebhookVerifier`].
    pub fn verify_next_gen(
        verifier: &WebhookVerifier,
        raw_body: &[u8],
        signature_header: Option<&str>,
    ) -> crate::MollieResult<()> {
        verifier.verify_header(raw_body, signature_header)
    }

    /// Verifies then decodes a Next-gen JSON payload.
    pub fn verify_and_decode_next_gen<T: serde::de::DeserializeOwned>(
        verifier: &WebhookVerifier,
        raw_body: &[u8],
        signature_header: Option<&str>,
    ) -> crate::MollieResult<T> {
        match signature_header.map(str::trim).filter(|s| !s.is_empty()) {
            None => Err(crate::MollieError::webhook_verification(
                WebhookVerifyFailure::MissingSignature,
            )),
            Some(sig) => verifier.verify_and_decode(raw_body, sig),
        }
    }

    /// Fetches a webhook event by id (provider authenticity check alternative).
    ///
    /// Use after receiving an event id, or when signature secrets may be
    /// compromised — HMAC alone is not replay protection.
    pub async fn get_event(&self, event_id: &str) -> MollieResponse<types::EntityWebhookEvent> {
        self.client
            .get_webhook_event(&types::WebhookEventToken(event_id.to_string()))
            .into_mollie_result()
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{compute_mollie_signature_hex, WebhookVerifier};

    #[test]
    fn classic_parse_round_trip() {
        let n = WebhooksApi::parse_classic("id=tr_abc").unwrap();
        assert_eq!(n.id(), "tr_abc");
    }

    #[test]
    fn next_gen_verify_and_decode() {
        #[derive(serde::Deserialize, PartialEq, Debug)]
        struct Ev {
            id: String,
        }
        let secret = "s";
        let body = br#"{"id":"event_1"}"#;
        let sig = compute_mollie_signature_hex(secret.as_bytes(), body).unwrap();
        let v = WebhookVerifier::new(secret).unwrap();
        let ev: Ev = WebhooksApi::verify_and_decode_next_gen(&v, body, Some(&sig)).unwrap();
        assert_eq!(ev.id, "event_1");
    }
}
