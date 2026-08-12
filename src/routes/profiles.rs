//! Generated profiles route methods.

use crate::{routes, types, Client, Error, ResponseValue};
use progenitor_client::encode_path;

/// Generated `profiles` route methods on [`crate::Client`].
///
/// Client-owned request policy on [`crate::Client`]:
/// - idempotency via [`crate::Client::with_idempotency_key`] (default: UUID v4 per request)
/// - `testmode` query via [`crate::Client::with_testmode`] on operations that support it
///
/// The resolved idempotency key is returned on the response envelope
/// ([`crate::ResponseValue`] / [`crate::ResponseEnvelope`]).
#[allow(clippy::all)]
impl Client {
    /// List profiles
    ///
    /// Retrieve a list of all of your profiles.
    ///
    /// The results are paginated.
    ///
    /// Sends a `GET` request to `/profiles`
    ///
    /// Arguments:
    /// - `from`: Provide an ID to start the result set from the item with the given ID and onwards. This allows you to paginate the
    /// result set.
    /// - `limit`: The maximum number of items to return. Defaults to 50 items.
    pub async fn list_profiles<'a>(
        &'a self,
        from: Option<&'a str>,
        limit: Option<::std::num::NonZeroU64>,
    ) -> Result<ResponseValue<types::ListProfilesResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint("/profiles");
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request
            .query(&progenitor_client::QueryParam::new("from", &from))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .build()?;
        let response = self.send(request, routes::Operation::ListProfiles).await?;
        routes::response::json(
            response,
            &[200u16],
            &[400u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Create profile
    ///
    /// Create a profile to process payments on.
    ///
    /// Profiles are required for payment processing. Normally they are created via the Mollie dashboard. Alternatively, you
    /// can use this endpoint to automate profile creation.
    ///
    /// Sends a `POST` request to `/profiles`
    ///
    /// Arguments:
    pub async fn create_profile<'a>(
        &'a self,
        body: &'a types::ProfileRequest,
    ) -> Result<ResponseValue<types::ProfileResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint("/profiles");
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::POST, url)?;
        #[allow(unused_mut)]
        let mut request = request.json(&body).build()?;
        let response = self.send(request, routes::Operation::CreateProfile).await?;
        routes::response::json(
            response,
            &[201u16],
            &[403u16, 422u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Get profile
    ///
    /// Retrieve a single profile by its ID.
    ///
    /// Sends a `GET` request to `/profiles/{profileId}`
    ///
    /// Arguments:
    /// - `profile_id`: Provide the ID of the related profile.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn get_profile<'a>(
        &'a self,
        profile_id: &'a types::ProfileToken,
    ) -> Result<ResponseValue<types::ProfileResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/profiles/{}",
            encode_path(&profile_id.to_string())
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
        let response = self.send(request, routes::Operation::GetProfile).await?;
        routes::response::json(
            response,
            &[200u16],
            &[404u16, 410u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Delete profile
    ///
    /// Delete a profile. A deleted profile and its related credentials can no longer be used for accepting payments.
    ///
    /// Sends a `DELETE` request to `/profiles/{profileId}`
    ///
    /// Arguments:
    /// - `profile_id`: Provide the ID of the related profile.
    pub async fn delete_profile<'a>(
        &'a self,
        profile_id: &'a types::ProfileToken,
    ) -> Result<ResponseValue<()>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/profiles/{}",
            encode_path(&profile_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::DELETE, url)?;
        #[allow(unused_mut)]
        let mut request = request.build()?;
        let response = self.send(request, routes::Operation::DeleteProfile).await?;
        routes::response::json(
            response,
            &[204u16],
            &[404u16, 410u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Update profile
    ///
    /// Update an existing profile.
    ///
    /// Profiles are required for payment processing. Normally they are created and updated via the Mollie dashboard.
    /// Alternatively, you can use this endpoint to automate profile management.
    ///
    /// Sends a `PATCH` request to `/profiles/{profileId}`
    ///
    /// Arguments:
    /// - `profile_id`: Provide the ID of the related profile.
    pub async fn update_profile<'a>(
        &'a self,
        profile_id: &'a types::ProfileToken,
        body: &'a types::UpdateProfileBody,
    ) -> Result<ResponseValue<types::ProfileResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/profiles/{}",
            encode_path(&profile_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::PATCH, url)?;
        #[allow(unused_mut)]
        let mut request = request.json(&body).build()?;
        let response = self.send(request, routes::Operation::UpdateProfile).await?;
        routes::response::json(
            response,
            &[200u16],
            &[403u16, 404u16, 410u16, 422u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Get current profile
    ///
    /// Retrieve the currently authenticated profile. A convenient alias of the [Get profile](get-profile)
    /// endpoint.
    ///
    /// For a complete reference of the profile object, refer to the [Get profile](get-profile) endpoint
    /// documentation.
    ///
    /// Sends a `GET` request to `/profiles/me`
    ///
    /// Arguments:
    pub async fn get_current_profile<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::ProfileResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint("/profiles/me");
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request.build()?;
        let response = self
            .send(request, routes::Operation::GetCurrentProfile)
            .await?;
        routes::response::json(response, &[200u16], &[429u16], &resolved_idempotency_key).await
    }
}
