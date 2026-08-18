//! Generated permissions route methods.

use crate::{routes, types, Client, Error, ResponseValue};
use progenitor_client::encode_path;

/// Generated `permissions` route methods on [`crate::Client`].
///
/// Client-owned request policy on [`crate::Client`]:
/// - idempotency via [`crate::Client::with_idempotency_key`] (default: UUID v4 per request)
/// - `testmode` query via [`crate::Client::with_testmode`] on operations that support it
///
/// The resolved idempotency key is returned on the response envelope
/// ([`crate::ResponseValue`] / [`crate::ResponseEnvelope`]).
#[allow(clippy::all)]
impl Client {
    /// List permissions
    ///
    /// Retrieve a list of all permissions available to the current access token.
    ///
    /// The results are **not** paginated.
    ///
    /// Sends a `GET` request to `/permissions`
    ///
    /// Arguments:
    pub async fn list_permissions<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::ListPermissionsResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint("/permissions");
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request.build()?;
        let response = self
            .send(request, routes::Operation::ListPermissions)
            .await?;
        routes::response::json(
            response,
            &[200u16],
            &[400u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }

    /// Get permission
    ///
    /// Retrieve a single permission by its ID, and see if the permission is granted to the current access token.
    ///
    /// Sends a `GET` request to `/permissions/{permissionId}`
    ///
    /// Arguments:
    /// - `permission_id`: Provide the ID of the related permission.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn get_permission<'a>(
        &'a self,
        permission_id: &'a types::PermissionToken,
    ) -> Result<ResponseValue<types::EntityPermission>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/permissions/{}",
            encode_path(&permission_id.to_string())
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
        let response = self.send(request, routes::Operation::GetPermission).await?;
        routes::response::json(
            response,
            &[200u16],
            &[404u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }
}
