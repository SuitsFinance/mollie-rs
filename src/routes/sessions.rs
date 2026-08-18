//! Generated sessions route methods.

use crate::{routes, types, Client, Error, ResponseValue};
use progenitor_client::encode_path;

/// Generated `sessions` route methods on [`crate::Client`].
///
/// Client-owned request policy on [`crate::Client`]:
/// - idempotency via [`crate::Client::with_idempotency_key`] (default: UUID v4 per request)
/// - `testmode` query via [`crate::Client::with_testmode`] on operations that support it
///
/// The resolved idempotency key is returned on the response envelope
/// ([`crate::ResponseValue`] / [`crate::ResponseEnvelope`]).
#[allow(clippy::all)]
impl Client {
    /// Create session
    ///
    /// > 🚧 Beta feature
    /// >
    /// > This feature is currently in private beta, and the final specification may still change.
    ///
    /// Create a session to start a checkout process with Mollie Components.
    ///
    /// Sends a `POST` request to `/sessions`
    ///
    /// Arguments:
    pub async fn create_session<'a>(
        &'a self,
        body: &'a types::SessionRequest,
    ) -> Result<ResponseValue<types::SessionResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint("/sessions");
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::POST, url)?;
        #[allow(unused_mut)]
        let mut request = request.json(&body).build()?;
        let response = self.send(request, routes::Operation::CreateSession).await?;
        routes::response::json(
            response,
            &[201u16],
            &[422u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }

    /// Get session
    ///
    /// > 🚧 Beta feature
    /// >
    /// > This feature is currently in private beta, and the final specification may still change.
    ///
    /// Retrieve a session to view its details and status to inform your customers about the checkout process.
    ///
    /// Sends a `GET` request to `/sessions/{sessionId}`
    ///
    /// Arguments:
    /// - `session_id`: Provide the ID of the related session.
    pub async fn get_session<'a>(
        &'a self,
        session_id: &'a types::SessionToken,
    ) -> Result<ResponseValue<types::SessionResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/sessions/{}",
            encode_path(&session_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request.build()?;
        let response = self.send(request, routes::Operation::GetSession).await?;
        routes::response::json(
            response,
            &[200u16],
            &[429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }
}
