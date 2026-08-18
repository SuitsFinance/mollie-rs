//! Generated onboarding route methods.

use crate::{routes, types, Client, Error, ResponseValue};

/// Generated `onboarding` route methods on [`crate::Client`].
///
/// Client-owned request policy on [`crate::Client`]:
/// - idempotency via [`crate::Client::with_idempotency_key`] (default: UUID v4 per request)
/// - `testmode` query via [`crate::Client::with_testmode`] on operations that support it
///
/// The resolved idempotency key is returned on the response envelope
/// ([`crate::ResponseValue`] / [`crate::ResponseEnvelope`]).
#[allow(clippy::all)]
impl Client {
    /// Get onboarding status
    ///
    /// Retrieve the onboarding status of the currently authenticated organization.
    ///
    /// Sends a `GET` request to `/onboarding/me`
    ///
    /// Arguments:
    pub async fn get_onboarding_status<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::EntityOnboardingStatus>, Error<types::ErrorResponse>> {
        let url = self.endpoint("/onboarding/me");
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request.build()?;
        let response = self
            .send(request, routes::Operation::GetOnboardingStatus)
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

    /// Submit onboarding data
    ///
    /// *⚠️ We no longer recommend implementing this endpoint. Please refer to the Client Links API instead to kick off the
    /// onboarding process for your merchants.**
    ///
    /// Submit data that will be prefilled in the merchant's onboarding. The data you submit will only be processed when the
    /// onboarding status is `needs-data`.
    /// Information that the merchant has entered in their dashboard will not be overwritten.
    ///
    /// Sends a `POST` request to `/onboarding/me`
    ///
    /// Arguments:
    pub async fn submit_onboarding_data<'a>(
        &'a self,
        body: &'a types::SubmitOnboardingDataBody,
    ) -> Result<ResponseValue<()>, Error<types::ErrorResponse>> {
        let url = self.endpoint("/onboarding/me");
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::POST, url)?;
        #[allow(unused_mut)]
        let mut request = request.json(&body).build()?;
        let response = self
            .send(request, routes::Operation::SubmitOnboardingData)
            .await?;
        routes::response::json(
            response,
            &[204u16],
            &[429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }
}
