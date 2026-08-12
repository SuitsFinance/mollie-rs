//! Generated capabilities route methods.

use crate::{routes, types, Client, Error, ResponseValue};

/// Generated `capabilities` route methods on [`crate::Client`].
///
/// Client-owned request policy on [`crate::Client`]:
/// - idempotency via [`crate::Client::with_idempotency_key`] (default: UUID v4 per request)
/// - `testmode` query via [`crate::Client::with_testmode`] on operations that support it
///
/// The resolved idempotency key is returned on the response envelope
/// ([`crate::ResponseValue`] / [`crate::ResponseEnvelope`]).
#[allow(clippy::all)]
impl Client {
    /// List capabilities
    ///
    /// > 🚧 Beta feature
    /// >
    /// > This feature is currently in beta testing, and the final specification may still change.
    ///
    /// Retrieve a list of capabilities for an organization.
    ///
    /// This API provides detailed insights into the specific requirements and status of each client's onboarding journey.
    ///
    /// Capabilities are at the organization level, indicating if the organization can perform a given capability.
    /// Capabilities may have requirements, which provide more information on what is needed to use this capability.
    /// Requirements may have a due date, which indicates the date by which the requirement should be fulfilled.
    /// If a requirement is past due, the capability is disabled until the requirement is fulfilled.
    ///
    /// For payments, regardless them being at the profile level, the capability is listed at the organization level.
    /// This means that if at least one of the clients's profiles can receive payments,
    /// the payments capability is enabled, communicating that the organization can indeed receive payments.
    ///
    /// Sends a `GET` request to `/capabilities`
    ///
    /// Arguments:
    pub async fn list_capabilities<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::ListCapabilitiesResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint("/capabilities");
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request.build()?;
        let response = self
            .send(request, routes::Operation::ListCapabilities)
            .await?;
        routes::response::json(response, &[200u16], &[429u16], &resolved_idempotency_key).await
    }
}
