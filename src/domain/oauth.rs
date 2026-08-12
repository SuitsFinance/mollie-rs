//! OAuth token lifecycle facade (generate / revoke).
//!
//! Uses HTTP Basic client credentials on `/oauth2/tokens` (outside `/v2`).
//! Retry class is **NonRetryableWrite** — the transport never auto-retries
//! token churn. Secrets must never appear in Debug output (see `BasicAuth`).
#![warn(missing_docs)]

use crate::auth::{BasicAuth, Credential};
use crate::types::{OauthGenerateTokensBody, OauthGenerateTokensResponse, OauthRevokeTokensBody};
use crate::{IntoMollieFuture, MollieClient, MollieError, MollieResponse, MollieResult};

/// OAuth token operations scoped to a [`MollieClient`].
///
/// Prefer constructing the client with [`MollieClient::from_basic_auth`] or
/// pass [`BasicAuth`] / [`Credential::BasicAuth`] into each call. Do not pass
/// API keys where client credentials are required.
#[derive(Debug)]
pub struct OAuthApi<'a> {
    client: &'a MollieClient,
}

impl MollieClient {
    /// Returns the OAuth token-lifecycle facade.
    pub fn oauth(&self) -> OAuthApi<'_> {
        OAuthApi { client: self }
    }
}

impl OAuthApi<'_> {
    /// Generates access / refresh tokens (`POST /oauth2/tokens`).
    ///
    /// `client_credentials` must be OAuth app Basic auth (client id + secret).
    /// Classification: never auto-retry (INV-WRITE / NonRetryableWrite).
    pub async fn generate_tokens(
        &self,
        client_credentials: &BasicAuth,
        body: &OauthGenerateTokensBody,
    ) -> MollieResponse<OauthGenerateTokensResponse> {
        let authorization = client_credentials.authorization_value();
        self.client
            .oauth_generate_tokens(&authorization, None, body)
            .into_mollie_result()
            .await
    }

    /// Generates tokens using a [`Credential`], rejecting non-Basic variants.
    pub async fn generate_tokens_with_credential(
        &self,
        credential: &Credential,
        body: &OauthGenerateTokensBody,
    ) -> MollieResponse<OauthGenerateTokensResponse> {
        let basic = basic_auth_only(credential)?;
        self.generate_tokens(basic, body).await
    }

    /// Revokes an access or refresh token (`DELETE /oauth2/tokens`).
    pub async fn revoke_tokens(
        &self,
        client_credentials: &BasicAuth,
        body: &OauthRevokeTokensBody,
    ) -> MollieResponse<()> {
        let authorization = client_credentials.authorization_value();
        self.client
            .oauth_revoke_tokens(&authorization, None, body)
            .into_mollie_result()
            .await
    }

    /// Revokes tokens using a [`Credential`], rejecting non-Basic variants.
    pub async fn revoke_tokens_with_credential(
        &self,
        credential: &Credential,
        body: &OauthRevokeTokensBody,
    ) -> MollieResponse<()> {
        let basic = basic_auth_only(credential)?;
        self.revoke_tokens(basic, body).await
    }
}

fn basic_auth_only(credential: &Credential) -> MollieResult<&BasicAuth> {
    match credential {
        Credential::BasicAuth(basic) => Ok(basic),
        Credential::ApiKey(_) | Credential::OAuthAccessToken(_) => {
            Err(MollieError::invalid_request(
                "OAuth token endpoints require Basic client credentials, not an API key or access token",
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{OauthGrantType, OauthTokenTypeHint};

    #[test]
    fn rejects_api_key_credential_class() {
        let cred = Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx").unwrap();
        let err = basic_auth_only(&cred).unwrap_err();
        assert!(err.to_string().contains("Basic client credentials"));
    }

    #[test]
    fn accepts_basic_auth_credential_class() {
        let cred = Credential::basic_auth("client-id", "client-secret").unwrap();
        assert!(basic_auth_only(&cred).is_ok());
    }

    #[test]
    fn basic_auth_debug_redacts_secret() {
        let basic = BasicAuth::new("id", "super-secret-value").unwrap();
        let dbg = format!("{basic:?}");
        assert!(!dbg.contains("super-secret-value"));
    }

    #[test]
    fn oauth_ops_are_non_retryable_in_profile() {
        let gen = crate::operation_safety_profile("oauth_generate_tokens").unwrap();
        let rev = crate::operation_safety_profile("oauth_revoke_tokens").unwrap();
        assert!(gen.retry_class.is_non_retryable());
        assert!(rev.retry_class.is_non_retryable());
        assert_eq!(gen.auth_class(), crate::AuthClass::OAuthClient);
    }

    #[test]
    fn body_types_are_constructible() {
        let _ = OauthGenerateTokensBody {
            code: Some("auth_code".into()),
            grant_type: OauthGrantType::AuthorizationCode,
            redirect_uri: Some("https://example.com/callback".into()),
            refresh_token: None,
        };
        let _ = OauthRevokeTokensBody {
            token: "access-token".into(),
            token_type_hint: OauthTokenTypeHint::AccessToken,
        };
    }
}
