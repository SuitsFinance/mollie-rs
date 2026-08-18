//! Generated organizations route methods.

use crate::{routes, types, Client, Error, ResponseValue};
use progenitor_client::encode_path;

/// Generated `organizations` route methods on [`crate::Client`].
///
/// Client-owned request policy on [`crate::Client`]:
/// - idempotency via [`crate::Client::with_idempotency_key`] (default: UUID v4 per request)
/// - `testmode` query via [`crate::Client::with_testmode`] on operations that support it
///
/// The resolved idempotency key is returned on the response envelope
/// ([`crate::ResponseValue`] / [`crate::ResponseEnvelope`]).
#[allow(clippy::all)]
impl Client {
    /// Get organization
    ///
    /// Retrieve a single organization by its ID.
    ///
    /// You can normally only retrieve the currently authenticated organization with this endpoint. This is primarily useful
    /// for OAuth apps. See also [Get current organization](get-current-organization).
    ///
    /// If you have a *partner account*', you can retrieve organization details of connected organizations.
    ///
    /// Sends a `GET` request to `/organizations/{organizationId}`
    ///
    /// Arguments:
    /// - `organization_id`: Provide the ID of the related organization.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn get_organization<'a>(
        &'a self,
        organization_id: &'a types::OrganizationToken,
    ) -> Result<ResponseValue<types::EntityOrganization>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/organizations/{}",
            encode_path(&organization_id.to_string())
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
        let response = self
            .send(request, routes::Operation::GetOrganization)
            .await?;
        routes::response::json(
            response,
            &[200u16],
            &[404u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }

    /// Get current organization
    ///
    /// Retrieve the currently authenticated organization. A convenient alias of the [Get organization](get-organization)
    /// endpoint.
    ///
    /// For a complete reference of the organization object, refer to the [Get organization](get-organization) endpoint
    /// documentation.
    ///
    /// Sends a `GET` request to `/organizations/me`
    ///
    /// Arguments:
    pub async fn get_current_organization<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::EntityOrganization>, Error<types::ErrorResponse>> {
        let url = self.endpoint("/organizations/me");
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request.build()?;
        let response = self
            .send(request, routes::Operation::GetCurrentOrganization)
            .await?;
        routes::response::json(
            response,
            &[200u16],
            &[429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }

    /// Get partner status
    ///
    /// Retrieve partnership details about the currently authenticated organization. Only relevant for so-called *partner
    /// accounts*.
    ///
    /// Sends a `GET` request to `/organizations/me/partner`
    ///
    /// Arguments:
    pub async fn get_partner_status<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::GetPartnerStatusResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint("/organizations/me/partner");
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request.build()?;
        let response = self
            .send(request, routes::Operation::GetPartnerStatus)
            .await?;
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
