//! Generated oauth route methods.

use crate::{routes, types, Client, Error, ResponseValue};

/// Generated `oauth` route methods on [`crate::Client`].
#[allow(clippy::all)]
impl Client {
    /// Generate OAuth access / refresh tokens using client credentials (Basic auth).
    ///
    /// Sends a `POST` request to `/oauth2/tokens`
    ///
    /// Arguments:
    /// - `authorization`: HTTP Basic auth header value (`Basic …`) with OAuth client credentials.
    /// - `content_type`: Optional `Content-Type` override (defaults to JSON body serialization).
    pub async fn oauth_generate_tokens<'a>(
        &'a self,
        authorization: &'a str,
        content_type: Option<&'a str>,
        body: &'a types::OauthGenerateTokensBody,
    ) -> Result<ResponseValue<types::OauthGenerateTokensResponse>, Error<types::ErrorResponse>>
    {
        let url = self.endpoint("/oauth2/tokens");
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::POST, url)?;
        let mut request = request
            .header(::reqwest::header::AUTHORIZATION, authorization)
            .json(&body);
        if let Some(value) = content_type {
            request = request.header(::reqwest::header::CONTENT_TYPE, value);
        }
        let request = request.build()?;
        let response = self
            .send(request, routes::Operation::OauthGenerateTokens)
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

    /// Revoke an OAuth access or refresh token using client credentials (Basic auth).
    ///
    /// Sends a `DELETE` request to `/oauth2/tokens`
    ///
    /// Arguments:
    /// - `authorization`: HTTP Basic auth header value (`Basic …`) with OAuth client credentials.
    /// - `content_type`: Optional `Content-Type` override (defaults to JSON body serialization).
    pub async fn oauth_revoke_tokens<'a>(
        &'a self,
        authorization: &'a str,
        content_type: Option<&'a str>,
        body: &'a types::OauthRevokeTokensBody,
    ) -> Result<ResponseValue<()>, Error<types::ErrorResponse>> {
        let url = self.endpoint("/oauth2/tokens");
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::DELETE, url)?;
        let mut request = request
            .header(::reqwest::header::AUTHORIZATION, authorization)
            .json(&body);
        if let Some(value) = content_type {
            request = request.header(::reqwest::header::CONTENT_TYPE, value);
        }
        let request = request.build()?;
        let response = self
            .send(request, routes::Operation::OauthRevokeTokens)
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
