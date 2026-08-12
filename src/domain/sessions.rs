//! Components checkout session facade (beta).
//!
//! `create_session` is an **IdempotentWrite** — sticky keys enable safe retries.
#![warn(missing_docs)]

use crate::domain::common::client_with_key;
use crate::types::{self, SessionResponse};
use crate::{IdempotencyKey, IntoMollieFuture, MollieClient, MollieResponse};

/// Session operations scoped to a [`MollieClient`].
#[derive(Debug)]
pub struct SessionsApi<'a> {
    client: &'a MollieClient,
}

impl MollieClient {
    /// Returns the checkout sessions facade.
    pub fn sessions(&self) -> SessionsApi<'_> {
        SessionsApi { client: self }
    }
}

impl SessionsApi<'_> {
    /// Creates a Mollie Components checkout session.
    pub async fn create(
        &self,
        body: &types::SessionRequest,
        key: Option<IdempotencyKey>,
    ) -> MollieResponse<SessionResponse> {
        client_with_key(self.client, key)
            .create_session(body)
            .into_mollie_result()
            .await
    }

    /// Fetches a session by id (`sess_…` / provider session token).
    pub async fn get(&self, session_id: &str) -> MollieResponse<SessionResponse> {
        let token = types::SessionToken(session_id.to_string());
        self.client.get_session(&token).into_mollie_result().await
    }
}

#[cfg(test)]
mod tests {
    use crate::{operation_safety_profile, RetryClass};

    #[test]
    fn create_session_is_idempotent_write() {
        let p = operation_safety_profile("create_session").unwrap();
        assert_eq!(p.retry_class, RetryClass::IdempotentWrite);
        assert!(p.supports_idempotency);
    }
}
