//! Authentication helpers for constructing Mollie API clients.
//!
//! This module keeps credential formatting in one place so callers do not have
//! to hand-build `Authorization` headers.
//!
//! # Examples
//!
//! ```rust
//! use mollie_rs::auth::Credential;
//!
//! let credential = Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;
//! assert_eq!(credential.scheme(), "Bearer");
//! # Ok::<(), mollie_rs::MollieError>(())
//! ```
#![warn(missing_docs)]

use std::fmt;

use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine};

use crate::env::{
    load_dotenv, var, MOLLIE_API_KEY_ENV, MOLLIE_OAUTH_ACCESS_TOKEN_ENV,
    MOLLIE_OAUTH_CLIENT_ID_ENV, MOLLIE_OAUTH_CLIENT_SECRET_ENV,
};
use crate::{MollieError, MollieResult};

/// A validated Mollie API key.
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct ApiKey(String);

impl ApiKey {
    /// Creates an API key after validating it can safely be used in a bearer
    /// authorization header.
    ///
    /// # Errors
    ///
    /// Returns [`MollieError::InvalidConfiguration`] when the API key is
    /// blank, has leading or trailing whitespace, or contains control
    /// characters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::ApiKey;
    ///
    /// let key = ApiKey::new("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;
    /// assert_eq!(key.as_str(), "test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");
    /// # Ok::<(), mollie_rs::MollieError>(())
    /// ```
    pub fn new(api_key: impl Into<String>) -> MollieResult<Self> {
        let api_key: String = api_key.into();
        validate_secret("Mollie API key", &api_key)?;
        Ok(Self(api_key))
    }

    /// Returns the raw API key.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Creates an API key from the `MOLLIE_API_KEY` environment variable.
    ///
    /// Loads a `.env` file from the current directory when present (missing
    /// file is ignored), then reads the process environment.
    ///
    /// # Errors
    ///
    /// Returns [`MollieError::InvalidConfiguration`] when dotenv fails to load,
    /// the variable is missing or not UTF-8, or the value fails API-key
    /// validation.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mollie_rs::ApiKey;
    ///
    /// # fn main() -> Result<(), mollie_rs::MollieError> {
    /// let _key = ApiKey::from_env()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_env() -> MollieResult<Self> {
        load_dotenv()?;
        Self::new(var(MOLLIE_API_KEY_ENV)?)
    }
}

impl TryFrom<&str> for ApiKey {
    type Error = MollieError;

    /// Converts a string slice into a validated API key.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for ApiKey {
    type Error = MollieError;

    /// Converts an owned string into a validated API key.
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl AsRef<str> for ApiKey {
    /// Returns the API key as a string slice.
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Debug for ApiKey {
    /// Formats the API key without exposing the secret value.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ApiKey").field(&"<redacted>").finish()
    }
}

#[cfg(feature = "zeroize")]
impl Drop for ApiKey {
    fn drop(&mut self) {
        use zeroize::Zeroize;
        self.0.zeroize();
    }
}

/// A validated Mollie OAuth access token.
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct OAuthAccessToken(String);

impl OAuthAccessToken {
    /// Creates an OAuth access token after validating it can safely be used in
    /// a bearer authorization header.
    ///
    /// # Errors
    ///
    /// Returns [`MollieError::InvalidConfiguration`] when the token is blank,
    /// has leading or trailing whitespace, or contains control characters.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::OAuthAccessToken;
    ///
    /// let token = OAuthAccessToken::new("access-token")?;
    /// assert_eq!(token.as_str(), "access-token");
    /// # Ok::<(), mollie_rs::MollieError>(())
    /// ```
    pub fn new(token: impl Into<String>) -> MollieResult<Self> {
        let token = token.into();
        validate_secret("Mollie OAuth access token", &token)?;
        Ok(Self(token))
    }

    /// Returns the raw OAuth access token.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Creates an OAuth access token from the `MOLLIE_OAUTH_ACCESS_TOKEN`
    /// environment variable.
    ///
    /// Loads a `.env` file from the current directory when present (missing
    /// file is ignored), then reads the process environment.
    ///
    /// # Errors
    ///
    /// Returns [`MollieError::InvalidConfiguration`] when dotenv fails to load,
    /// the variable is missing or not UTF-8, or the value fails token
    /// validation.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mollie_rs::OAuthAccessToken;
    ///
    /// # fn main() -> Result<(), mollie_rs::MollieError> {
    /// let _token = OAuthAccessToken::from_env()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_env() -> MollieResult<Self> {
        load_dotenv()?;
        Self::new(var(MOLLIE_OAUTH_ACCESS_TOKEN_ENV)?)
    }
}

impl TryFrom<&str> for OAuthAccessToken {
    type Error = MollieError;

    /// Converts a string slice into a validated OAuth access token.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<String> for OAuthAccessToken {
    type Error = MollieError;

    /// Converts an owned string into a validated OAuth access token.
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl AsRef<str> for OAuthAccessToken {
    /// Returns the OAuth access token as a string slice.
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Debug for OAuthAccessToken {
    /// Formats the OAuth access token without exposing the secret value.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("OAuthAccessToken")
            .field(&"<redacted>")
            .finish()
    }
}

#[cfg(feature = "zeroize")]
impl Drop for OAuthAccessToken {
    fn drop(&mut self) {
        use zeroize::Zeroize;
        self.0.zeroize();
    }
}

/// Validated OAuth client credentials for Mollie Basic Auth.
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct BasicAuth {
    client_id: String,
    client_secret: String,
    encoded: String,
}

impl BasicAuth {
    /// Creates Basic Auth credentials from an OAuth client ID and secret.
    pub fn new(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
    ) -> MollieResult<Self> {
        let client_id = client_id.into();
        let client_secret = client_secret.into();
        validate_secret("Mollie OAuth client ID", &client_id)?;
        validate_secret("Mollie OAuth client secret", &client_secret)?;
        let raw = format!("{client_id}:{client_secret}");

        Ok(Self {
            client_id,
            client_secret,
            encoded: BASE64_STANDARD.encode(raw),
        })
    }

    /// Returns the OAuth client ID without exposing the client secret.
    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    /// Returns the value for the HTTP `Authorization` header.
    pub fn authorization_value(&self) -> String {
        format!("Basic {}", self.encoded)
    }

    /// Creates Basic Auth credentials from environment variables.
    pub fn from_env() -> MollieResult<Self> {
        load_dotenv()?;
        Self::new(
            var(MOLLIE_OAUTH_CLIENT_ID_ENV)?,
            var(MOLLIE_OAUTH_CLIENT_SECRET_ENV)?,
        )
    }
}

impl fmt::Debug for BasicAuth {
    /// Formats Basic Auth without exposing the client secret or encoded value.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BasicAuth")
            .field("client_id", &self.client_id)
            .field("client_secret", &"<redacted>")
            .finish()
    }
}

#[cfg(feature = "zeroize")]
impl Drop for BasicAuth {
    fn drop(&mut self) {
        use zeroize::Zeroize;
        self.client_secret.zeroize();
        self.encoded.zeroize();
    }
}

/// Authentication material used to create the `Authorization` header.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Credential {
    /// A Mollie API key such as `test_...` or `live_...`.
    ApiKey(ApiKey),
    /// An OAuth access token issued for a Mollie organization or app.
    OAuthAccessToken(OAuthAccessToken),
    /// OAuth client ID and secret encoded for token-management Basic Auth.
    BasicAuth(BasicAuth),
}

impl Credential {
    /// Creates a credential from a Mollie API key.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::auth::Credential;
    ///
    /// let credential = Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;
    /// assert!(matches!(credential, Credential::ApiKey(_)));
    /// # Ok::<(), mollie_rs::MollieError>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`MollieError::InvalidConfiguration`] when the API key fails
    /// validation.
    pub fn api_key(api_key: impl Into<String>) -> MollieResult<Self> {
        Ok(Self::ApiKey(ApiKey::new(api_key)?))
    }

    /// Creates a credential from environment variables.
    ///
    /// Loads a `.env` file when present, then prefers `MOLLIE_API_KEY` and
    /// falls back to `MOLLIE_OAUTH_ACCESS_TOKEN`.
    ///
    /// # Errors
    ///
    /// Returns [`MollieError::InvalidConfiguration`] when dotenv fails, neither
    /// credential variable is set, a value is not UTF-8, or validation fails.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mollie_rs::Credential;
    ///
    /// # fn main() -> Result<(), mollie_rs::MollieError> {
    /// let _credential = Credential::from_env()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_env() -> MollieResult<Self> {
        use crate::env::{
            load_dotenv, var_optional, MOLLIE_API_KEY_ENV, MOLLIE_OAUTH_ACCESS_TOKEN_ENV,
            MOLLIE_OAUTH_CLIENT_ID_ENV, MOLLIE_OAUTH_CLIENT_SECRET_ENV,
        };

        load_dotenv()?;

        if let Some(api_key) = var_optional(MOLLIE_API_KEY_ENV)? {
            return Self::api_key(api_key);
        }

        if let Some(token) = var_optional(MOLLIE_OAUTH_ACCESS_TOKEN_ENV)? {
            return Self::oauth_access_token(token);
        }

        let client_id = var_optional(MOLLIE_OAUTH_CLIENT_ID_ENV)?;
        let client_secret = var_optional(MOLLIE_OAUTH_CLIENT_SECRET_ENV)?;
        match (client_id, client_secret) {
            (Some(client_id), Some(client_secret)) => Self::basic_auth(client_id, client_secret),
            (Some(_), None) | (None, Some(_)) => Err(MollieError::invalid_configuration(
                "Mollie OAuth client ID and client secret must be configured together",
            )),
            (None, None) => Err(MollieError::missing_mollie_credentials()),
        }
    }

    /// Creates a credential from an already validated API key.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::{ApiKey, Credential};
    ///
    /// let key = ApiKey::new("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;
    /// let credential = Credential::from_api_key(key);
    /// assert_eq!(credential.scheme(), "Bearer");
    /// # Ok::<(), mollie_rs::MollieError>(())
    /// ```
    pub fn from_api_key(api_key: ApiKey) -> Self {
        Self::ApiKey(api_key)
    }

    /// Creates a credential from an OAuth access token.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::auth::Credential;
    ///
    /// let credential = Credential::oauth_access_token("access-token")?;
    /// assert!(matches!(credential, Credential::OAuthAccessToken(_)));
    /// # Ok::<(), mollie_rs::MollieError>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`MollieError::InvalidConfiguration`] when the OAuth access
    /// token fails validation.
    pub fn oauth_access_token(token: impl Into<String>) -> MollieResult<Self> {
        Ok(Self::OAuthAccessToken(OAuthAccessToken::new(token)?))
    }

    /// Creates a credential from an already validated OAuth access token.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::{Credential, OAuthAccessToken};
    ///
    /// let token = OAuthAccessToken::new("access-token")?;
    /// let credential = Credential::from_oauth_access_token(token);
    /// assert_eq!(credential.scheme(), "Bearer");
    /// # Ok::<(), mollie_rs::MollieError>(())
    /// ```
    pub fn from_oauth_access_token(token: OAuthAccessToken) -> Self {
        Self::OAuthAccessToken(token)
    }

    /// Creates Basic Auth credentials from an OAuth client ID and secret.
    pub fn basic_auth(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
    ) -> MollieResult<Self> {
        Ok(Self::BasicAuth(BasicAuth::new(client_id, client_secret)?))
    }

    /// Wraps already validated OAuth Basic Auth credentials.
    pub fn from_basic_auth(credentials: BasicAuth) -> Self {
        Self::BasicAuth(credentials)
    }

    /// Returns the HTTP authorization scheme for the credential.
    ///
    /// Mollie accepts both API keys and OAuth access tokens as bearer
    /// credentials.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::auth::Credential;
    ///
    /// assert_eq!(Credential::api_key("test_xxx")?.scheme(), "Bearer");
    /// # Ok::<(), mollie_rs::MollieError>(())
    /// ```
    pub const fn scheme(&self) -> &'static str {
        match self {
            Self::ApiKey(_) | Self::OAuthAccessToken(_) => "Bearer",
            Self::BasicAuth(_) => "Basic",
        }
    }

    /// Returns the raw credential secret.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::auth::Credential;
    ///
    /// assert_eq!(Credential::api_key("test_xxx")?.secret(), "test_xxx");
    /// # Ok::<(), mollie_rs::MollieError>(())
    /// ```
    pub fn secret(&self) -> &str {
        match self {
            Self::ApiKey(value) => value.as_str(),
            Self::OAuthAccessToken(value) => value.as_str(),
            Self::BasicAuth(value) => &value.encoded,
        }
    }

    /// Returns the value to use for the HTTP `Authorization` header.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::auth::Credential;
    ///
    /// assert_eq!(
    ///     Credential::api_key("test_xxx")?.authorization_value(),
    ///     "Bearer test_xxx",
    /// );
    /// # Ok::<(), mollie_rs::MollieError>(())
    /// ```
    pub fn authorization_value(&self) -> String {
        format!("{} {}", self.scheme(), self.secret())
    }

    /// Returns `true` when the credential contains only whitespace.
    ///
    /// This remains available for compatibility, but typed credential
    /// constructors reject blank values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::auth::Credential;
    ///
    /// assert!(Credential::api_key("   ").is_err());
    /// ```
    pub fn is_blank(&self) -> bool {
        self.secret().trim().is_empty()
    }
}

impl From<ApiKey> for Credential {
    /// Wraps a validated API key as a credential.
    fn from(value: ApiKey) -> Self {
        Self::from_api_key(value)
    }
}

impl From<OAuthAccessToken> for Credential {
    /// Wraps a validated OAuth access token as a credential.
    fn from(value: OAuthAccessToken) -> Self {
        Self::from_oauth_access_token(value)
    }
}

impl From<BasicAuth> for Credential {
    /// Wraps validated Basic Auth credentials.
    fn from(value: BasicAuth) -> Self {
        Self::from_basic_auth(value)
    }
}

/// Validates a credential secret before it is used in an authorization header.
fn validate_secret(name: &str, value: &str) -> MollieResult<()> {
    if value.trim().is_empty() {
        return Err(MollieError::invalid_configuration(format!(
            "{name} cannot be blank"
        )));
    }

    if value.trim() != value {
        return Err(MollieError::invalid_configuration(format!(
            "{name} cannot include leading or trailing whitespace"
        )));
    }

    if value.chars().any(char::is_control) {
        return Err(MollieError::invalid_configuration(format!(
            "{name} cannot include control characters"
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    mod api_key {
        use super::*;

        #[test]
        fn new_rejects_blank_value() {
            let error: MollieError = ApiKey::new(" ").unwrap_err();

            assert!(matches!(error, MollieError::InvalidConfiguration { .. }));
        }

        #[test]
        fn debug_redacts_secret() {
            let key: ApiKey = ApiKey::new("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")
                .expect("api key should be valid");

            assert_eq!(format!("{key:?}"), "ApiKey(\"<redacted>\")");
        }
    }

    mod credential {
        use super::*;

        #[test]
        fn authorization_value_uses_bearer_scheme() {
            let credential: Credential = Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")
                .expect("api key should be valid");

            assert_eq!(
                credential.authorization_value(),
                "Bearer test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
            );
        }

        #[test]
        fn basic_auth_encodes_client_credentials_and_redacts_debug() {
            let credential: Credential = Credential::basic_auth("client-id", "client-secret")
                .expect("Basic Auth should be valid");

            assert_eq!(credential.scheme(), "Basic");
            assert_eq!(
                credential.authorization_value(),
                "Basic Y2xpZW50LWlkOmNsaWVudC1zZWNyZXQ="
            );
            assert!(!format!("{credential:?}").contains("client-secret"));
        }
    }
}
