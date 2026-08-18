//! Ergonomic Mollie client construction.
//!
//! The generated [`crate::Client`] remains available for callers that need
//! exact low-level control. [`MollieClient`] is the recommended entry point for
//! application code because it configures the base URL, authentication header,
//! timeout, and user agent without requiring callers to build `reqwest`.
//!
//! # Examples
//!
//! ```rust,no_run
//! use mollie_rs::MollieClient;
//!
//! # fn main() -> Result<(), mollie_rs::MollieError> {
//! let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;
//! let _raw_generated_client = client.raw();
//! # Ok(())
//! # }
//! ```
#![warn(missing_docs)]

use std::{ops::Deref, sync::Arc, time::Duration};

use reqwest::header::{HeaderMap, HeaderName, HeaderValue, AUTHORIZATION, USER_AGENT};
use reqwest::{Client as ReqwestClient, ClientBuilder as ReqwestClientBuilder, Url};

use crate::contract_drift::{ContractDriftObserver, SharedContractDriftObserver};
use crate::hooks::{RequestHook, SharedRequestHook};
use crate::ids::ProfileId;
use crate::{auth::Credential, error::MollieResult, Client, MollieError};

/// Default Mollie API v2 base URL.
pub const DEFAULT_BASE_URL: &str = "https://api.mollie.com/v2";

/// A Mollie API client with built-in HTTP and authentication setup.
#[derive(Clone, Debug)]
pub struct MollieClient {
    inner: Client,
}

impl MollieClient {
    /// Creates a Mollie client from an API key.
    ///
    /// # Errors
    ///
    /// Returns [`MollieError::InvalidConfiguration`] when the API key is
    /// blank or cannot be encoded as a bearer credential. Returns
    /// [`MollieError::Communication`] when the HTTP client cannot be built.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mollie_rs::MollieClient;
    ///
    /// # fn main() -> Result<(), mollie_rs::MollieError> {
    /// let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;
    /// let _ = client.raw();
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_api_key(api_key: impl Into<String>) -> MollieResult<Self> {
        Self::builder()
            .credential(Credential::api_key(api_key)?)
            .build()
    }

    /// Creates a Mollie client from an OAuth access token.
    ///
    /// # Errors
    ///
    /// Returns [`MollieError::InvalidConfiguration`] when the token is blank
    /// or cannot be encoded as a bearer credential. Returns
    /// [`MollieError::Communication`] when the HTTP client cannot be built.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mollie_rs::MollieClient;
    ///
    /// # fn main() -> Result<(), mollie_rs::MollieError> {
    /// let client = MollieClient::from_oauth_access_token("access-token")?;
    /// let _ = client.raw();
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_oauth_access_token(token: impl Into<String>) -> MollieResult<Self> {
        Self::builder()
            .credential(Credential::oauth_access_token(token)?)
            .build()
    }

    /// Creates a Mollie client using OAuth client credentials as Basic Auth.
    ///
    /// This is intended for OAuth token generation and revocation routes, not
    /// for ordinary resource API calls.
    pub fn from_basic_auth(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
    ) -> MollieResult<Self> {
        Self::builder()
            .credential(Credential::basic_auth(client_id, client_secret)?)
            .build()
    }

    /// Creates a Mollie client from environment variables.
    ///
    /// Loads a `.env` file from the current directory when present (missing
    /// file is ignored), then reads credentials in this order:
    /// 1. [`crate::env::MOLLIE_API_KEY_ENV`] (`MOLLIE_API_KEY`)
    /// 2. [`crate::env::MOLLIE_OAUTH_ACCESS_TOKEN_ENV`] (`MOLLIE_OAUTH_ACCESS_TOKEN`)
    ///
    /// When [`crate::env::MOLLIE_BASE_URL_ENV`] is set, it overrides the default
    /// base URL.
    ///
    /// # Errors
    ///
    /// Returns [`MollieError::InvalidConfiguration`] when dotenv fails to load,
    /// neither credential variable is set, a variable is invalid, or client
    /// construction fails.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mollie_rs::MollieClient;
    ///
    /// # fn main() -> Result<(), mollie_rs::MollieError> {
    /// let client = MollieClient::from_env()?;
    /// let _ = client.raw();
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_env() -> MollieResult<Self> {
        use crate::env::{var_optional, MOLLIE_BASE_URL_ENV};

        let mut builder = Self::builder().credential(Credential::from_env()?);

        if let Some(base_url) = var_optional(MOLLIE_BASE_URL_ENV)? {
            builder = builder.base_url(base_url);
        }

        builder.build()
    }

    /// Starts a configurable Mollie client builder.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mollie_rs::{auth::Credential, MollieClient};
    ///
    /// # fn main() -> Result<(), mollie_rs::MollieError> {
    /// let client = MollieClient::builder()
    ///     .credential(Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?)
    ///     .build()?;
    /// let _ = client.raw();
    /// # Ok(())
    /// # }
    /// ```
    pub fn builder() -> MollieClientBuilder {
        MollieClientBuilder::default()
    }

    /// Wraps an already generated client.
    ///
    /// Use this only when you need full control over the underlying transport.
    /// Most applications should prefer [`Self::from_api_key`] or
    /// [`Self::builder`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::{Client, MollieClient, DEFAULT_BASE_URL};
    ///
    /// let generated = Client::new(DEFAULT_BASE_URL).expect("default client");
    /// let client = MollieClient::from_generated(generated);
    /// assert_eq!(client.raw().baseurl(), DEFAULT_BASE_URL);
    /// ```
    pub const fn from_generated(inner: Client) -> Self {
        Self { inner }
    }

    /// Returns the generated client that owns every typed route method.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::{Client, MollieClient, DEFAULT_BASE_URL};
    ///
    /// let client = MollieClient::from_generated(Client::new(DEFAULT_BASE_URL).expect("default client"));
    /// assert_eq!(client.raw().baseurl(), DEFAULT_BASE_URL);
    /// ```
    pub const fn raw(&self) -> &Client {
        &self.inner
    }

    /// Consumes the facade and returns the generated client.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::{Client, MollieClient, DEFAULT_BASE_URL};
    ///
    /// let generated = MollieClient::from_generated(Client::new(DEFAULT_BASE_URL).expect("default client")).into_raw();
    /// assert_eq!(generated.baseurl(), DEFAULT_BASE_URL);
    /// ```
    pub fn into_raw(self) -> Client {
        self.inner
    }

    /// Returns a client that sends the given sticky idempotency key on every
    /// request until cleared.
    ///
    /// Prefer this for retries of the **same** logical operation. Unrelated
    /// operations should use distinct keys (or the default auto-UUID path).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::{Client, MollieClient, DEFAULT_BASE_URL};
    ///
    /// let client = MollieClient::from_generated(Client::new(DEFAULT_BASE_URL).expect("default client"))
    ///     .with_idempotency_key("6f7ef3e6-8c2f-4d1c-9f08-5ab7adf56c91");
    /// assert_eq!(
    ///     client.idempotency_key(),
    ///     Some("6f7ef3e6-8c2f-4d1c-9f08-5ab7adf56c91")
    /// );
    /// ```
    pub fn with_idempotency_key(self, key: impl Into<String>) -> Self {
        Self {
            inner: self.inner.with_idempotency_key(key),
        }
    }

    /// Clears any sticky idempotency key so subsequent requests generate a UUID
    /// v4.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::{Client, MollieClient, DEFAULT_BASE_URL};
    ///
    /// let client = MollieClient::from_generated(Client::new(DEFAULT_BASE_URL).expect("default client"))
    ///     .with_idempotency_key("retry-key")
    ///     .clear_idempotency_key();
    /// assert!(client.idempotency_key().is_none());
    /// ```
    pub fn clear_idempotency_key(self) -> Self {
        Self {
            inner: self.inner.clear_idempotency_key(),
        }
    }

    /// Returns the configured sticky idempotency key, if any.
    ///
    /// This is the key stored on the client, not the last auto-generated key.
    /// After a call, read the key that was sent from the response envelope.
    pub fn idempotency_key(&self) -> Option<&str> {
        self.inner.idempotency_key()
    }

    /// Returns a client that sends the sticky `testmode` query on routes that
    /// support it.
    ///
    /// Prefer this for organization-level OAuth credentials that need test
    /// entities.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::{Client, MollieClient, DEFAULT_BASE_URL};
    ///
    /// let client = MollieClient::from_generated(Client::new(DEFAULT_BASE_URL).expect("default client"))
    ///     .with_testmode(true);
    /// assert_eq!(client.testmode(), Some(true));
    /// ```
    pub fn with_testmode(self, testmode: bool) -> Self {
        Self {
            inner: self.inner.with_testmode(testmode),
        }
    }

    /// Clears sticky `testmode` so supporting routes omit the query param.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::{Client, MollieClient, DEFAULT_BASE_URL};
    ///
    /// let client = MollieClient::from_generated(Client::new(DEFAULT_BASE_URL).expect("default client"))
    ///     .with_testmode(true)
    ///     .clear_testmode();
    /// assert!(client.testmode().is_none());
    /// ```
    pub fn clear_testmode(self) -> Self {
        Self {
            inner: self.inner.clear_testmode(),
        }
    }

    /// Returns the configured sticky `testmode` value, if any.
    pub fn testmode(&self) -> Option<bool> {
        self.inner.testmode()
    }

    /// Returns a client with the given retry policy.
    ///
    /// Default policy is disabled. Prefer [`crate::RetryPolicy::default_safe`]
    /// for conservative automatic retries.
    pub fn with_retry_policy(self, policy: crate::RetryPolicy) -> Self {
        Self {
            inner: self.inner.with_retry_policy(policy),
        }
    }

    /// Returns the configured retry policy.
    pub fn retry_policy(&self) -> &crate::RetryPolicy {
        self.inner.retry_policy()
    }

    /// Scopes an [`crate::IdempotencyKey`] to this client clone for one logical write.
    pub fn with_idempotency(self, key: crate::IdempotencyKey) -> Self {
        self.with_idempotency_key(key.into_string())
    }

    /// Returns a client with a sticky default profile id for facades.
    pub fn with_profile_id(self, profile_id: &ProfileId) -> Self {
        Self {
            inner: self.inner.with_profile_id(profile_id.as_str().to_string()),
        }
    }

    /// Clears any sticky default profile id.
    pub fn clear_profile_id(self) -> Self {
        Self {
            inner: self.inner.clear_profile_id(),
        }
    }

    /// Returns the sticky default profile id, if any.
    pub fn profile_id(&self) -> Option<&str> {
        self.inner.profile_id()
    }

    /// Attaches a request lifecycle hook (metrics, correlation, test doubles).
    pub fn with_request_hook(self, hook: impl RequestHook + 'static) -> Self {
        Self {
            inner: self.inner.with_request_hook(Arc::new(hook)),
        }
    }

    /// Attaches a shared request lifecycle hook.
    pub fn with_shared_request_hook(self, hook: SharedRequestHook) -> Self {
        Self {
            inner: self.inner.with_request_hook(hook),
        }
    }

    /// Attaches a contract-drift observer (unknown enums, off-origin next links).
    pub fn with_contract_drift_observer(self, observer: impl ContractDriftObserver + 'static) -> Self {
        Self {
            inner: self
                .inner
                .with_contract_drift_observer(Arc::new(observer)),
        }
    }

    /// Attaches a shared contract-drift observer.
    pub fn with_shared_contract_drift_observer(
        self,
        observer: SharedContractDriftObserver,
    ) -> Self {
        Self {
            inner: self.inner.with_contract_drift_observer(observer),
        }
    }

    /// Returns a client that uses a different credential for subsequent calls.
    ///
    /// Rebuilds the underlying HTTP client with a new `Authorization` header
    /// while preserving:
    /// - base URL
    /// - request / connect timeouts
    /// - user agent
    /// - testmode
    /// - profile id
    /// - sticky idempotency key
    /// - retry policy
    /// - request hook
    /// - contract-drift observer
    /// - response body limits
    ///
    /// Custom default headers from the original builder are **not** replayed
    /// (reqwest does not expose them). Re-apply headers via
    /// [`MollieClientBuilder::default_header`] when needed. Fully custom
    /// transports should use [`MollieClient::from_generated`] with a pre-built
    /// `reqwest` client.
    ///
    /// The client remains cheaply cloneable after this call.
    pub fn with_credential(self, credential: Credential) -> MollieResult<Self> {
        if credential.is_blank() {
            return Err(MollieError::invalid_configuration(
                "Mollie credential cannot be blank",
            ));
        }

        let mut builder = Self::builder()
            .base_url(self.inner.baseurl())
            .credential(credential)
            .timeout(self.inner.timeout())
            .connect_timeout(self.inner.connect_timeout())
            .retry_policy(self.inner.retry_policy().clone())
            .response_limits(self.inner.response_limits());

        if let Some(ua) = self.inner.user_agent() {
            builder = builder.user_agent(ua);
        }
        if let Some(testmode) = self.inner.testmode() {
            builder = builder.testmode(testmode);
        }
        if let Some(profile_id) = self.inner.profile_id() {
            builder = builder.profile_id(profile_id);
        }
        if let Some(hook) = self.inner.request_hook() {
            builder = builder.shared_request_hook(hook.clone());
        }
        if let Some(observer) = self.inner.contract_drift_observer() {
            builder = builder.shared_contract_drift_observer(observer.clone());
        }

        let mut rebuilt = builder.build()?;
        if let Some(key) = self.inner.idempotency_key() {
            rebuilt = rebuilt.with_idempotency_key(key);
        }
        Ok(rebuilt)
    }
}

impl Deref for MollieClient {
    type Target = Client;

    /// Returns the generated client so route methods can be called directly.
    fn deref(&self) -> &Self::Target {
        self.raw()
    }
}

/// Builder for [`MollieClient`].
#[derive(Clone)]
pub struct MollieClientBuilder {
    base_url: String,
    credential: Option<Credential>,
    timeout: Duration,
    connect_timeout: Duration,
    user_agent: Option<String>,
    user_agent_suffix: Option<String>,
    default_headers: HeaderMap,
    testmode: Option<bool>,
    profile_id: Option<String>,
    retry_policy: crate::RetryPolicy,
    /// Optional caller customization applied before mandatory SDK security settings.
    http_configure:
        Option<std::sync::Arc<dyn Fn(ReqwestClientBuilder) -> ReqwestClientBuilder + Send + Sync>>,
    request_hook: Option<SharedRequestHook>,
    contract_drift_observer: Option<SharedContractDriftObserver>,
    response_limits: crate::ResponseLimits,
}

impl std::fmt::Debug for MollieClientBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MollieClientBuilder")
            .field("base_url", &self.base_url)
            .field(
                "credential",
                &self.credential.as_ref().map(|_| "<redacted>"),
            )
            .field("timeout", &self.timeout)
            .field("connect_timeout", &self.connect_timeout)
            .field("user_agent", &self.user_agent)
            .field("user_agent_suffix", &self.user_agent_suffix)
            .field("testmode", &self.testmode)
            .field("profile_id", &self.profile_id)
            .field("retry_policy", &self.retry_policy)
            .field(
                "http_configure",
                &self.http_configure.as_ref().map(|_| "<configure_http>"),
            )
            .field(
                "request_hook",
                &self.request_hook.as_ref().map(|_| "<hook>"),
            )
            .field(
                "contract_drift_observer",
                &self
                    .contract_drift_observer
                    .as_ref()
                    .map(|_| "<contract_drift_observer>"),
            )
            .field("response_limits", &self.response_limits)
            .finish_non_exhaustive()
    }
}

impl Default for MollieClientBuilder {
    /// Creates a builder configured for Mollie's production API endpoint.
    fn default() -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_string(),
            credential: None,
            timeout: Duration::from_secs(30),
            connect_timeout: Duration::from_secs(15),
            user_agent: Some(format!("mollie-rust/{}", env!("CARGO_PKG_VERSION"))),
            user_agent_suffix: None,
            default_headers: HeaderMap::new(),
            testmode: None,
            profile_id: None,
            retry_policy: crate::RetryPolicy::disabled(),
            http_configure: None,
            request_hook: None,
            contract_drift_observer: None,
            response_limits: crate::ResponseLimits::default(),
        }
    }
}

impl MollieClientBuilder {
    /// Sets the Mollie API base URL.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mollie_rs::{auth::Credential, MollieClient};
    ///
    /// # fn main() -> Result<(), mollie_rs::MollieError> {
    /// let client = MollieClient::builder()
    ///     .base_url("https://api.mollie.com/v2")
    ///     .credential(Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?)
    ///     .build()?;
    /// let _ = client.raw();
    /// # Ok(())
    /// # }
    /// ```
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Sets the API key or OAuth access token used for the `Authorization`
    /// header.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mollie_rs::{auth::Credential, MollieClient};
    ///
    /// # fn main() -> Result<(), mollie_rs::MollieError> {
    /// let client = MollieClient::builder()
    ///     .credential(Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?)
    ///     .build()?;
    /// let _ = client.raw();
    /// # Ok(())
    /// # }
    /// ```
    pub fn credential(mut self, credential: Credential) -> Self {
        self.credential = Some(credential);
        self
    }

    /// Sets the total request timeout.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use std::time::Duration;
    ///
    /// use mollie_rs::{auth::Credential, MollieClient};
    ///
    /// # fn main() -> Result<(), mollie_rs::MollieError> {
    /// let client = MollieClient::builder()
    ///     .timeout(Duration::from_secs(10))
    ///     .credential(Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?)
    ///     .build()?;
    /// let _ = client.raw();
    /// # Ok(())
    /// # }
    /// ```
    pub const fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Sets the TCP/TLS connection timeout.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use std::time::Duration;
    ///
    /// use mollie_rs::{auth::Credential, MollieClient};
    ///
    /// # fn main() -> Result<(), mollie_rs::MollieError> {
    /// let client = MollieClient::builder()
    ///     .connect_timeout(Duration::from_secs(5))
    ///     .credential(Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?)
    ///     .build()?;
    /// let _ = client.raw();
    /// # Ok(())
    /// # }
    /// ```
    pub const fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }

    /// Sets the HTTP user agent.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mollie_rs::{auth::Credential, MollieClient};
    ///
    /// # fn main() -> Result<(), mollie_rs::MollieError> {
    /// let client = MollieClient::builder()
    ///     .user_agent("my-app/1.0")
    ///     .credential(Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?)
    ///     .build()?;
    /// let _ = client.raw();
    /// # Ok(())
    /// # }
    /// ```
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// Appends a suffix to the default SDK user agent (e.g. `"my-app/1.0"`).
    ///
    /// When [`Self::user_agent`] is also set, the suffix is appended to that
    /// value. Official Speakeasy SDKs expose an equivalent `customUserAgent`.
    pub fn user_agent_suffix(mut self, suffix: impl Into<String>) -> Self {
        self.user_agent_suffix = Some(suffix.into());
        self
    }

    /// Sets a sticky default profile id for facades that support profile context.
    ///
    /// Operation-level profile parameters always take precedence when the
    /// application passes them explicitly.
    pub fn profile_id(mut self, profile_id: impl AsRef<str>) -> Self {
        self.profile_id = Some(profile_id.as_ref().to_string());
        self
    }

    /// Customizes the `reqwest` client builder before mandatory SDK security
    /// settings are applied (INV-HTTP-01).
    ///
    /// The closure runs first; the SDK then forces redirect-none, TLS 1.2+,
    /// default Authorization/User-Agent headers, and builder timeouts so
    /// callers cannot silently re-enable redirects or drop auth headers via
    /// the safe builder path.
    ///
    /// For fully unrestricted transport control (tests / advanced adapters),
    /// use [`MollieClient::from_generated`] with a hand-built [`Client`].
    pub fn configure_http<F>(mut self, configure: F) -> Self
    where
        F: Fn(ReqwestClientBuilder) -> ReqwestClientBuilder + Send + Sync + 'static,
    {
        self.http_configure = Some(std::sync::Arc::new(configure));
        self
    }

    /// Sets response body buffering limits for JSON and error decoding.
    pub fn response_limits(mut self, limits: crate::ResponseLimits) -> Self {
        self.response_limits = limits;
        self
    }

    /// Attaches a request lifecycle hook.
    pub fn request_hook(mut self, hook: impl RequestHook + 'static) -> Self {
        self.request_hook = Some(Arc::new(hook));
        self
    }

    /// Attaches a shared request lifecycle hook.
    pub fn shared_request_hook(mut self, hook: SharedRequestHook) -> Self {
        self.request_hook = Some(hook);
        self
    }

    /// Attaches a contract-drift observer (TEL-001).
    pub fn contract_drift_observer(mut self, observer: impl ContractDriftObserver + 'static) -> Self {
        self.contract_drift_observer = Some(Arc::new(observer));
        self
    }

    /// Attaches a shared contract-drift observer.
    pub fn shared_contract_drift_observer(
        mut self,
        observer: SharedContractDriftObserver,
    ) -> Self {
        self.contract_drift_observer = Some(observer);
        self
    }

    /// Adds a default HTTP header to every request.
    ///
    /// # Errors
    ///
    /// Returns [`MollieError::InvalidHeaderValue`] when the value cannot be
    /// represented as an HTTP header value.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mollie_rs::{auth::Credential, MollieClient};
    /// use reqwest::header::HeaderName;
    ///
    /// # fn main() -> Result<(), mollie_rs::MollieError> {
    /// let client = MollieClient::builder()
    ///     .credential(Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?)
    ///     .default_header(HeaderName::from_static("x-request-source"), "docs")?
    ///     .build()?;
    /// let _ = client.raw();
    /// # Ok(())
    /// # }
    /// ```
    pub fn default_header(
        mut self,
        name: HeaderName,
        value: impl AsRef<str>,
    ) -> MollieResult<Self> {
        self.default_headers
            .insert(name, HeaderValue::from_str(value.as_ref())?);
        Ok(self)
    }

    /// Sets sticky `testmode` for routes that support the query parameter.
    ///
    /// Useful when building an OAuth client that always talks to test entities.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mollie_rs::{auth::Credential, MollieClient};
    ///
    /// # fn main() -> Result<(), mollie_rs::MollieError> {
    /// let client = MollieClient::builder()
    ///     .credential(Credential::oauth_access_token("access-token")?)
    ///     .testmode(true)
    ///     .build()?;
    /// assert_eq!(client.testmode(), Some(true));
    /// # Ok(())
    /// # }
    /// ```
    pub fn testmode(mut self, testmode: bool) -> Self {
        self.testmode = Some(testmode);
        self
    }

    /// Builds the configured [`MollieClient`].
    ///
    /// # Errors
    ///
    /// Returns [`MollieError::InvalidConfiguration`] when the base URL is
    /// invalid or no credential was configured. Returns
    /// [`MollieError::InvalidHeaderValue`] when a generated header value is
    /// invalid. Returns [`MollieError::Communication`] when `reqwest` cannot
    /// build the HTTP client.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mollie_rs::{auth::Credential, MollieClient};
    ///
    /// # fn main() -> Result<(), mollie_rs::MollieError> {
    /// let client = MollieClient::builder()
    ///     .credential(Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?)
    ///     .build()?;
    /// let _ = client.raw();
    /// # Ok(())
    /// # }
    /// ```
    pub fn build(self) -> MollieResult<MollieClient> {
        validate_base_url(&self.base_url)?;

        let credential: Credential = self
            .credential
            .ok_or_else(|| MollieError::invalid_configuration("missing Mollie credential"))?;
        if credential.is_blank() {
            return Err(MollieError::invalid_configuration(
                "Mollie credential cannot be blank",
            ));
        }

        let mut headers: HeaderMap = self.default_headers;
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&credential.authorization_value())?,
        );
        let mut user_agent = self
            .user_agent
            .unwrap_or_else(|| format!("mollie-rust/{}", env!("CARGO_PKG_VERSION")));
        if let Some(suffix) = self.user_agent_suffix {
            let suffix = suffix.trim();
            if !suffix.is_empty() {
                user_agent = format!("{user_agent} {suffix}");
            }
        }
        headers.insert(USER_AGENT, HeaderValue::from_str(&user_agent)?);

        // INV-HOST-01 / INV-HTTP-01: caller configure runs first; SDK
        // security settings always apply last so they cannot be silently
        // disabled via safe builder customization.
        let mut builder = ReqwestClient::builder();
        if let Some(configure) = &self.http_configure {
            builder = configure(builder);
        }
        let http_client: ReqwestClient = builder
            .default_headers(headers)
            .redirect(reqwest::redirect::Policy::none())
            .min_tls_version(reqwest::tls::Version::TLS_1_2)
            .connect_timeout(self.connect_timeout)
            .timeout(self.timeout)
            .build()?;

        tracing::debug!(base_url = %self.base_url, "built Mollie client");

        let mut inner = Client::new_with_client(&self.base_url, http_client)
            .with_transport_timeouts(self.timeout, self.connect_timeout)
            .with_user_agent_string(user_agent)
            .with_response_limits(self.response_limits);
        if let Some(testmode) = self.testmode {
            inner = inner.with_testmode(testmode);
        }
        if let Some(profile_id) = self.profile_id {
            inner = inner.with_profile_id(profile_id);
        }
        if let Some(hook) = self.request_hook {
            inner = inner.with_request_hook(hook);
        }
        if let Some(observer) = self.contract_drift_observer {
            inner = inner.with_contract_drift_observer(observer);
        }
        inner = inner.with_retry_policy(self.retry_policy);

        Ok(MollieClient::from_generated(inner))
    }

    /// Sets the automatic retry policy (disabled by default).
    pub fn retry_policy(mut self, policy: crate::RetryPolicy) -> Self {
        self.retry_policy = policy;
        self
    }
}

/// Validates the configured Mollie base URL before constructing the client.
fn validate_base_url(base_url: &str) -> MollieResult<()> {
    let parsed: Url = Url::parse(base_url).map_err(|error| {
        MollieError::invalid_configuration(format!("invalid Mollie base URL `{base_url}`: {error}"))
    })?;

    if parsed.scheme() == "https" {
        return Ok(());
    }

    let is_loopback_http = parsed.scheme() == "http"
        && parsed
            .host_str()
            .map(|host| {
                host.eq_ignore_ascii_case("localhost")
                    || host
                        .parse::<std::net::IpAddr>()
                        .is_ok_and(|address| address.is_loopback())
            })
            .unwrap_or(false);
    if !is_loopback_http {
        return Err(MollieError::invalid_configuration(
            "Mollie base URL must use HTTPS (HTTP is allowed only for loopback test endpoints)",
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_credential_preserves_timeouts_and_user_agent() {
        let client = MollieClient::builder()
            .credential(Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx").unwrap())
            .timeout(Duration::from_secs(9))
            .connect_timeout(Duration::from_secs(4))
            .user_agent("parity-test/9.9")
            .build()
            .expect("build");
        assert_eq!(client.raw().timeout(), Duration::from_secs(9));
        assert_eq!(client.raw().connect_timeout(), Duration::from_secs(4));
        assert_eq!(client.raw().user_agent(), Some("parity-test/9.9"));

        let scoped = client
            .with_credential(Credential::oauth_access_token("access-token").unwrap())
            .expect("scope credential");
        assert_eq!(scoped.raw().timeout(), Duration::from_secs(9));
        assert_eq!(scoped.raw().connect_timeout(), Duration::from_secs(4));
        assert_eq!(scoped.raw().user_agent(), Some("parity-test/9.9"));
        assert_eq!(scoped.raw().baseurl(), DEFAULT_BASE_URL);
    }

    mod mollie_client_builder {
        use super::*;

        #[test]
        fn build_returns_error_when_credential_is_missing() {
            let error: MollieError = MollieClient::builder().build().unwrap_err();

            assert!(matches!(error, MollieError::InvalidConfiguration { .. }));
        }

        #[test]
        fn build_returns_error_when_credential_is_blank() {
            let error: MollieError = MollieClient::from_api_key(" ").unwrap_err();

            assert!(matches!(error, MollieError::InvalidConfiguration { .. }));
        }

        #[test]
        fn build_returns_client_when_api_key_is_present() {
            let client: MollieClient =
                MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")
                    .expect("client should build");

            assert_eq!(client.raw().baseurl(), DEFAULT_BASE_URL);
        }

        #[test]
        fn build_returns_error_when_base_url_is_invalid() {
            let error: MollieError = MollieClient::builder()
                .base_url("mailto:support@example.com")
                .credential(
                    Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")
                        .expect("api key should be valid"),
                )
                .build()
                .unwrap_err();

            assert!(matches!(error, MollieError::InvalidConfiguration { .. }));
        }

        #[test]
        fn build_rejects_remote_http_base_urls() {
            let error: MollieError = MollieClient::builder()
                .base_url("http://api.mollie.com/v2")
                .credential(
                    Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")
                        .expect("api key should be valid"),
                )
                .build()
                .unwrap_err();

            assert!(matches!(error, MollieError::InvalidConfiguration { .. }));
        }

        #[test]
        fn build_allows_loopback_http_for_mock_servers() {
            let client = MollieClient::builder()
                .base_url("http://127.0.0.1:12345/v2")
                .credential(
                    Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")
                        .expect("api key should be valid"),
                )
                .build()
                .expect("loopback mock endpoint should be allowed");

            assert_eq!(client.raw().baseurl(), "http://127.0.0.1:12345/v2");
        }

        #[test]
        fn configure_http_still_builds_client() {
            let client = MollieClient::builder()
                .credential(
                    Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")
                        .expect("api key should be valid"),
                )
                .configure_http(|b| b.user_agent("custom-prefix"))
                .build()
                .expect("configure_http should build");
            assert_eq!(client.raw().baseurl(), DEFAULT_BASE_URL);
        }

        #[test]
        fn response_limits_are_applied_to_inner_client() {
            let limits = crate::ResponseLimits::default().with_max_json_bytes(1024);
            let client = MollieClient::builder()
                .credential(
                    Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")
                        .expect("api key should be valid"),
                )
                .response_limits(limits)
                .build()
                .expect("client should build");
            assert_eq!(client.raw().response_limits().max_json_bytes, 1024);
        }

        #[test]
        fn with_credential_preserves_response_limits() {
            let limits = crate::ResponseLimits::default().with_max_error_body_bytes(512);
            let client = MollieClient::builder()
                .credential(
                    Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")
                        .expect("api key should be valid"),
                )
                .response_limits(limits)
                .build()
                .expect("client should build");
            let scoped = client
                .with_credential(Credential::oauth_access_token("access-token").expect("token"))
                .expect("scope credential");
            assert_eq!(scoped.raw().response_limits().max_error_body_bytes, 512);
        }

        #[tokio::test]
        async fn configure_http_cannot_reenable_redirects() {
            use wiremock::matchers::{method, path};
            use wiremock::{Mock, MockServer, ResponseTemplate};

            let server = MockServer::start().await;
            Mock::given(method("GET"))
                .and(path("/from"))
                .respond_with(ResponseTemplate::new(302).insert_header("Location", "/to"))
                .mount(&server)
                .await;
            Mock::given(method("GET"))
                .and(path("/to"))
                .respond_with(ResponseTemplate::new(200).set_body_string(r#"{"ok":true}"#))
                .mount(&server)
                .await;

            let client = MollieClient::builder()
                .base_url(server.uri())
                .credential(
                    Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")
                        .expect("api key should be valid"),
                )
                // Attempt to re-enable redirects; SDK must force Policy::none last.
                .configure_http(|b| b.redirect(reqwest::redirect::Policy::limited(10)))
                .build()
                .expect("client should build");

            let result = client
                .http_client()
                .get(format!("{}/from", server.uri()))
                .send()
                .await
                .expect("request should complete without following redirect");
            assert_eq!(
                result.status(),
                reqwest::StatusCode::FOUND,
                "safe builder must not follow redirects"
            );
        }

        #[test]
        fn configure_http_proxy_userinfo_is_not_in_builder_debug() {
            // Proxy credentials must not appear in Debug of the builder (closure is opaque).
            let secret_userinfo = "proxy-user:s3cret-proxy-pass";
            let proxy_url = format!("http://{secret_userinfo}@127.0.0.1:8888");
            let builder = MollieClient::builder()
                .credential(
                    Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")
                        .expect("api key should be valid"),
                )
                .configure_http(move |b| {
                    match reqwest::Proxy::all(proxy_url.as_str()) {
                        Ok(p) => b.proxy(p),
                        Err(_) => b,
                    }
                });
            let dbg = format!("{builder:?}");
            assert!(
                !dbg.contains("s3cret-proxy-pass"),
                "builder Debug must not leak proxy password: {dbg}"
            );
            assert!(
                !dbg.contains("proxy-user"),
                "builder Debug must not leak proxy username: {dbg}"
            );
            assert!(
                dbg.contains("configure_http"),
                "Debug should only note that configure_http was set"
            );
            // Build still succeeds with a loopback proxy target (no network required to construct).
            builder.build().expect("client with proxy configure should build");
        }

        #[test]
        fn configure_http_no_proxy_still_applies_tls_floor() {
            let client = MollieClient::builder()
                .credential(
                    Credential::api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")
                        .expect("api key should be valid"),
                )
                .configure_http(|b| b.no_proxy())
                .build()
                .expect("no_proxy configure should build");
            assert_eq!(client.raw().baseurl(), DEFAULT_BASE_URL);
        }
    }
}
