//! Typed Mollie API client for Rust.
//!
//! Owned and maintained by Suits Finance B.V. This is an unofficial SDK: it is not
//! affiliated with, endorsed by, or supported by Mollie B.V.
//!
//! The generated [`Client`] exposes one typed async method for every operation
//! in the checked-in Mollie OpenAPI spec. [`MollieClient`] is the ergonomic
//! facade for applications: it builds the HTTP client, sets authentication, and
//! dereferences to [`Client`] so the full generated route surface remains
//! available.
//!
//! # Examples
//!
//! ```rust,no_run
//! use mollie_rs::{types, IntoMollieFuture, MollieClient, Money};
//!
//! # async fn create() -> Result<(), mollie_rs::MollieError> {
//! let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;
//! let payment = mollie_rs::CreatePaymentRequired::new(
//!     "Order #12345",
//!     Money::new("EUR", "10.00")?,
//!     "https://example.com/return",
//! )?
//! .into_payment_request();
//!
//! let response = client
//!     .create_payment(None, &payment)
//!     .into_mollie_result()
//!     .await?;
//! // When no sticky key is configured, a UUID v4 is generated and returned.
//! let _key = response.idempotency_key();
//! let _payment = response.into_inner();
//! # Ok(())
//! # }
//! ```
//!
//! To reuse a key for retries of the same logical operation, bind it on the
//! client (owned, no lifetime coupling to request bodies):
//!
//! ```rust,no_run
//! # use mollie_rs::{CreatePaymentRequired, IntoMollieFuture, MollieClient, Money};
//! # async fn retry() -> Result<(), mollie_rs::MollieError> {
//! # let client = MollieClient::from_api_key("test_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")?;
//! # let payment = CreatePaymentRequired::new(
//! #     "Order #12345",
//! #     Money::new("EUR", "10.00")?,
//! #     "https://example.com/return",
//! # )?
//! # .into_payment_request();
//! let client = client.with_idempotency_key("6f7ef3e6-8c2f-4d1c-9f08-5ab7adf56c91");
//! let _response = client.create_payment(None, &payment).into_mollie_result().await?;
//! # Ok(())
//! # }
//! ```
//!
//! See `docs/api-overview.md` for the SDK capability map, `docs/route-coverage.md`
//! for the generated route matrix, and `docs/contracts/` for public facade
//! contracts.

pub mod address;
pub mod auth;
#[cfg(test)]
mod capabilities_fixture;
pub mod client;
pub mod country_code;
pub mod create_payment;
pub mod datetime;
pub mod domain;
pub mod empty;
pub mod env;
pub mod envelope;
pub mod error;
pub mod error_catalog;
pub mod factory;
pub mod hooks;
pub mod idempotency;
pub mod ids;
pub mod integration;
pub mod locale;
pub mod metadata;
pub mod money;
pub mod nullable_field;
pub mod open_enum;
pub mod operation_safety;
pub mod pagination;
pub mod payment_method;
pub mod phone_number;
#[cfg(test)]
mod postman_error_fixtures;
pub mod route_capabilities;
/// Application tracing-subscriber helpers (`app-helpers` feature, default on).
#[cfg(feature = "app-helpers")]
pub mod tracing_config;
pub mod transport;
pub mod webhook;
pub mod webhook_verify;
pub mod write_requests;

#[cfg(test)]
mod property_tests;
#[cfg(test)]
mod secret_leak_tests;

use reqwest::{Client as ReqwestClient, ClientBuilder as ReqwestClientBuilder};

pub use address::{Address, POSTAL_CODE_OPTIONAL_COUNTRIES};
pub use auth::{ApiKey, BasicAuth, Credential, OAuthAccessToken};
pub use client::{MollieClient, MollieClientBuilder, DEFAULT_BASE_URL};
pub use country_code::CountryCode;
pub use create_payment::{
    CreatePaymentRequired, PaymentDescription, RedirectUrl, PAYMENT_DESCRIPTION_MAX_LEN,
};
pub use datetime::{Date, DateTime};
pub use domain::{
    CapturesApi, ConnectBalanceTransfersApi, MandatesApi, OAuthApi, PaymentLinksApi, PaymentsApi,
    PayoutsApi, RefundsApi, SessionsApi, SubscriptionsApi, TerminalsApi, TransferClientSignature,
    TransfersApi, UnmatchedCreditTransfersApi, VerifyPayeeApi, WebhooksApi,
};
pub use empty::EmptyResponse;
pub use env::{
    load_dotenv, load_dotenv_from, var, var_optional, var_os, MOLLIE_API_KEY_ENV,
    MOLLIE_BASE_URL_ENV, MOLLIE_OAUTH_ACCESS_TOKEN_ENV, MOLLIE_OAUTH_CLIENT_ID_ENV,
    MOLLIE_OAUTH_CLIENT_SECRET_ENV,
};
pub use envelope::{
    GeneratedMollieResult, IntoMollieFuture, IntoMollieResult, MollieEnvelope, MollieResponse,
    ResponseEnvelope, ResponseValueExt,
};
pub use error::{MollieError, MollieResult};
pub use error_catalog::{
    MollieErrorCatalogEntry, MollieErrorCode, MollieErrorEnvelope, MollieErrorKey,
    MollieSuccessCatalogEntry, MollieSuccessCode, MollieSuccessEnvelope, MollieSuccessKey,
};
pub use hooks::{NoopHook, RequestContext, RequestHook, SharedRequestHook};
pub use idempotency::{IdempotencyKey, IDEMPOTENCY_KEY_MAX_LEN};
pub use ids::{
    BalanceId, CaptureId, ChargebackId, CustomerId, MandateId, PaymentId, PaymentLinkId, ProfileId,
    RefundId, SalesInvoiceId, SettlementId, SubscriptionId, TerminalId,
};
pub use integration::{
    ClaimResult, EventStoreReplayAdapter, PaymentStateRefetcher, WebhookDispatcher,
    WebhookEventStore, WebhookReplayStore,
};
pub use locale::Locale;
pub use metadata::{ErrorResponseContext, ResponseMetadata, MAX_RETAINED_BODY_BYTES};
pub use money::{
    AmountValue, ApplicationFee, ApplicationFeeDescription, Currency, Money,
    APPLICATION_FEE_DESCRIPTION_MAX_LEN,
};
pub use nullable_field::{is_omitted as nullable_field_is_omitted, NullableField};
pub use open_enum::{OpenEnum, OpenEnumError, OPEN_ENUM_MAX_RAW_LEN};
pub use operation_safety::{
    all_operation_safety_profiles, high_risk_coverage, operation_safety_profile, AuthClass,
    IdempotencyClass, MutationClass, OperationExposure, OperationRisk, OperationSafetyProfile,
    PaginationPolicy, ProfileScope, TestmodePolicy, HIGH_RISK_WRITE_OPERATION_IDS,
    PAYMENT_CAPABILITY_MUTATION_OPERATION_IDS,
};
pub use pagination::{
    AsyncPaginator, ItemStream, Page, PageCursor, PaginationGuard, DEFAULT_PAGE_LIMIT,
    MAX_PAGE_LIMIT,
};
pub use payment_method::PaymentMethod;
pub use phone_number::PhoneNumber;
pub use route_capabilities::{
    retry_class_for_operation, route_capability, RouteAccess, RouteCapability, ROUTE_CAPABILITIES,
};
#[cfg(feature = "app-helpers")]
pub use tracing_config::{
    init_tracing, init_tracing_with_filter, try_init_tracing, try_init_tracing_with_filter,
};
pub use transport::{compute_backoff, DeliveryOutcome, RetryClass, RetryPolicy};
pub use webhook::{WebhookNotification, WebhookUrl};
pub use webhook_verify::{
    compute_mollie_signature_hex, VerifiedWebhook, WebhookSigningSecret, WebhookVerifier,
    WebhookVerifyFailure, DEFAULT_MAX_WEBHOOK_BODY_BYTES, MOLLIE_SIGNATURE_HEADER,
};
pub use write_requests::{
    ConnectBalanceTransferParty, CreateCaptureRequired, CreateConnectBalanceTransferRequired,
    CreatePaymentLinkRequired, CreatePayoutRequired, CreateRefundRequired,
    CreateSepaMandateRequired, CreateSubscriptionRequired, CreateTransferRequired,
    VerifyPayeeRequired,
};

/// Re-export of the `tracing` crate for application instrumentation.
pub use tracing;
/// Re-export of the `tracing-subscriber` crate used by [`init_tracing`].
///
/// Only available with the default `app-helpers` feature.
#[cfg(feature = "app-helpers")]
pub use tracing_subscriber;

use progenitor_client::ClientHooks;
#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, ClientInfo, Error, ResponseValue};

/// Generated Mollie API route groups (inherent methods on [`Client`]).
pub mod routes;
/// Generated OpenAPI types for request and response bodies.
pub mod types;

/// Low-level typed Mollie API client (OpenAPI-generated route surface).
///
/// Prefer [`MollieClient`] for application construction (base URL, auth, and
/// HTTP defaults). Generated route methods live as inherent methods on this
/// type (and therefore also on [`MollieClient`] via `Deref`).
///
/// Client state shared by generated routes:
///
/// - **Idempotency**: no sticky key (default) → UUID v4 per request;
///   [`Self::with_idempotency_key`] reuses an owned key for retries of the same
///   logical operation. The resolved key is always sent and returned on the
///   response envelope ([`ResponseEnvelope::idempotency_key`] /
///   [`ResponseValueExt::idempotency_key`]).
/// - **Test mode**: [`Self::with_testmode`] sets a sticky `testmode` query for
///   operations that document it (typical for OAuth org tokens). Default `None`
///   leaves the credential mode unchanged.
///
/// Version: 1.0.0
#[derive(Clone)]
pub struct Client {
    /// Configured API base URL (scheme, host, optional path stem).
    pub(crate) baseurl: String,
    /// Shared `reqwest` HTTP client used by generated route methods.
    pub(crate) client: ReqwestClient,
    /// Sticky idempotency key for outbound requests.
    ///
    /// When `None` or empty, each request generates a fresh UUID v4. Set via
    /// [`Self::with_idempotency_key`] to reuse a key across retries of the same
    /// logical operation. Do not reuse one key for unrelated operations.
    ///
    /// Prefer [`IdempotencyKey`] and a short-lived scoped client for one logical
    /// write rather than leaving a sticky key for unrelated operations.
    pub(crate) idempotency_key: Option<String>,
    /// Sticky `testmode` query for routes that support it.
    ///
    /// When `None`, the query param is omitted and the credential mode applies.
    /// Set via [`Self::with_testmode`] (common for OAuth access tokens that need
    /// test entities). Body-level `testmode` fields on request types are separate.
    pub(crate) testmode: Option<bool>,
    /// Sticky default profile id for facades that support profile context.
    ///
    /// Generated route methods still accept explicit `profile_id` parameters;
    /// operation arguments take precedence over this default when both are set
    /// by application code.
    pub(crate) profile_id: Option<String>,
    /// Opt-in retry policy (disabled by default).
    pub(crate) retry_policy: transport::RetryPolicy,
    /// Optional request lifecycle hook (metrics / correlation / test doubles).
    pub(crate) request_hook: Option<hooks::SharedRequestHook>,
    /// Request timeout retained so credential rebuilds can preserve it.
    pub(crate) timeout: std::time::Duration,
    /// Connect timeout retained so credential rebuilds can preserve it.
    pub(crate) connect_timeout: std::time::Duration,
    /// User-Agent string retained for credential rebuilds (no secrets).
    pub(crate) user_agent: Option<String>,
}

impl std::fmt::Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client")
            .field("baseurl", &self.baseurl)
            .field(
                "idempotency_key",
                &self.idempotency_key.as_ref().map(|_| "<redacted>"),
            )
            .field("testmode", &self.testmode)
            .field("profile_id", &self.profile_id)
            .field("retry_policy", &self.retry_policy)
            .field(
                "request_hook",
                &self.request_hook.as_ref().map(|_| "<hook>"),
            )
            .field("timeout", &self.timeout)
            .field("connect_timeout", &self.connect_timeout)
            .field("user_agent", &self.user_agent)
            .finish_non_exhaustive()
    }
}

/// Construction, HTTP helpers, and request lifecycle for generated routes.
impl Client {
    /// Create a new client with default HTTP timeouts (15s connect and total).
    ///
    /// `baseurl` is the base URL provided to the internal `reqwest::Client`,
    /// and should include a scheme and hostname, as well as port and a path
    /// stem if applicable.
    ///
    /// # Errors
    ///
    /// Returns [`reqwest::Error`] when the default HTTP client cannot be built
    /// (for example when the TLS backend fails to initialize).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::{Client, DEFAULT_BASE_URL};
    ///
    /// let client = Client::new(DEFAULT_BASE_URL)?;
    /// assert_eq!(client.baseurl(), DEFAULT_BASE_URL);
    /// # Ok::<(), reqwest::Error>(())
    /// ```
    pub fn new(baseurl: &str) -> Result<Self, reqwest::Error> {
        #[cfg(not(target_arch = "wasm32"))]
        let client: ReqwestClientBuilder = {
            let dur: std::time::Duration = ::std::time::Duration::from_secs(15u64);
            ReqwestClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = ReqwestClientBuilder::new();
        match client.build() {
            Ok(http_client) => Ok(Self::new_with_client(baseurl, http_client)),
            Err(error) => Err(error),
        }
    }

    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal `reqwest::Client`,
    /// and should include a scheme and hostname, as well as port and a path
    /// stem if applicable.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::{Client, DEFAULT_BASE_URL};
    ///
    /// let http = reqwest::Client::new();
    /// let client = Client::new_with_client(DEFAULT_BASE_URL, http);
    /// assert_eq!(client.baseurl(), DEFAULT_BASE_URL);
    /// ```
    pub fn new_with_client(baseurl: &str, client: ReqwestClient) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
            idempotency_key: None,
            testmode: None,
            profile_id: None,
            retry_policy: transport::RetryPolicy::disabled(),
            request_hook: None,
            // Defaults match historical Client::new (15s). Builders override.
            timeout: std::time::Duration::from_secs(15),
            connect_timeout: std::time::Duration::from_secs(15),
            user_agent: None,
        }
    }

    /// Records HTTP timeout settings used when rebuilding credentials.
    pub(crate) fn with_transport_timeouts(
        mut self,
        timeout: std::time::Duration,
        connect_timeout: std::time::Duration,
    ) -> Self {
        self.timeout = timeout;
        self.connect_timeout = connect_timeout;
        self
    }

    /// Records the User-Agent for credential rebuilds.
    pub(crate) fn with_user_agent_string(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// Returns the configured request timeout.
    pub fn timeout(&self) -> std::time::Duration {
        self.timeout
    }

    /// Returns the configured connect timeout.
    pub fn connect_timeout(&self) -> std::time::Duration {
        self.connect_timeout
    }

    /// Returns the configured User-Agent, if known.
    pub fn user_agent(&self) -> Option<&str> {
        self.user_agent.as_deref()
    }

    /// Returns a client with the given retry policy (clones transport settings).
    pub fn with_retry_policy(mut self, policy: transport::RetryPolicy) -> Self {
        self.retry_policy = policy;
        self
    }

    /// Returns the configured retry policy.
    pub fn retry_policy(&self) -> &transport::RetryPolicy {
        &self.retry_policy
    }

    /// Returns a client that sends the given sticky idempotency key on every
    /// request until cleared.
    ///
    /// **Discouraged for long-lived clients:** a sticky key reused across
    /// unrelated operations violates Mollie idempotency semantics. Prefer
    /// [`IdempotencyKey`] with a short-lived clone via
    /// [`MollieClient::with_idempotency`] for one logical write (and its
    /// transport retries).
    ///
    /// Prefer this only for retries of the **same** logical operation.
    /// Empty keys are ignored and treated as missing (UUID v4 is generated).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::{Client, DEFAULT_BASE_URL};
    ///
    /// let client = Client::new(DEFAULT_BASE_URL).expect("default client")
    ///     .with_idempotency_key("6f7ef3e6-8c2f-4d1c-9f08-5ab7adf56c91");
    /// assert_eq!(
    ///     client.idempotency_key(),
    ///     Some("6f7ef3e6-8c2f-4d1c-9f08-5ab7adf56c91")
    /// );
    /// ```
    pub fn with_idempotency_key(mut self, key: impl Into<String>) -> Self {
        self.idempotency_key = Some(key.into());
        self
    }

    /// Returns a cloned client with a sticky idempotency key, leaving `self`
    /// unchanged.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::{Client, DEFAULT_BASE_URL};
    ///
    /// let client = Client::new(DEFAULT_BASE_URL).expect("default client");
    /// let scoped = client.with_idempotency_key_ref("retry-key");
    /// assert!(client.idempotency_key().is_none());
    /// assert_eq!(scoped.idempotency_key(), Some("retry-key"));
    /// ```
    pub fn with_idempotency_key_ref(&self, key: impl Into<String>) -> Self {
        self.clone().with_idempotency_key(key)
    }

    /// Clears any sticky idempotency key so subsequent requests generate a UUID
    /// v4.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::{Client, DEFAULT_BASE_URL};
    ///
    /// let client = Client::new(DEFAULT_BASE_URL).expect("default client")
    ///     .with_idempotency_key("retry-key")
    ///     .clear_idempotency_key();
    /// assert!(client.idempotency_key().is_none());
    /// ```
    pub fn clear_idempotency_key(mut self) -> Self {
        self.idempotency_key = None;
        self
    }

    /// Returns the configured sticky idempotency key, if any.
    ///
    /// This is the key stored on the client, not necessarily the key that was
    /// last sent (auto-generated keys are not stored). Read the key that was
    /// sent from the response envelope after a call.
    pub fn idempotency_key(&self) -> Option<&str> {
        self.idempotency_key.as_deref()
    }

    /// Returns a client that sends the sticky `testmode` query on routes that
    /// support it.
    ///
    /// Prefer this for organization-level OAuth credentials that need test
    /// entities. API keys created for live or test mode usually leave the
    /// default (`None`) so the credential decides.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::{Client, DEFAULT_BASE_URL};
    ///
    /// let client = Client::new(DEFAULT_BASE_URL).expect("default client").with_testmode(true);
    /// assert_eq!(client.testmode(), Some(true));
    /// ```
    pub fn with_testmode(mut self, testmode: bool) -> Self {
        self.testmode = Some(testmode);
        self
    }

    /// Returns a cloned client with sticky `testmode`, leaving `self` unchanged.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::{Client, DEFAULT_BASE_URL};
    ///
    /// let client = Client::new(DEFAULT_BASE_URL).expect("default client");
    /// let scoped = client.with_testmode_ref(true);
    /// assert!(client.testmode().is_none());
    /// assert_eq!(scoped.testmode(), Some(true));
    /// ```
    pub fn with_testmode_ref(&self, testmode: bool) -> Self {
        self.clone().with_testmode(testmode)
    }

    /// Clears sticky `testmode` so supporting routes omit the query param.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::{Client, DEFAULT_BASE_URL};
    ///
    /// let client = Client::new(DEFAULT_BASE_URL).expect("default client")
    ///     .with_testmode(true)
    ///     .clear_testmode();
    /// assert!(client.testmode().is_none());
    /// ```
    pub fn clear_testmode(mut self) -> Self {
        self.testmode = None;
        self
    }

    /// Returns the configured sticky `testmode` value, if any.
    ///
    /// Generated routes that document the query param pass this value through
    /// `QueryParam` (so `None` omits the parameter).
    pub fn testmode(&self) -> Option<bool> {
        self.testmode
    }

    /// Returns a client with a sticky default profile id for facades.
    ///
    /// Does not rewrite generated OpenAPI method signatures; domain facades and
    /// application code should prefer this default when an operation-level
    /// profile override is omitted.
    pub fn with_profile_id(mut self, profile_id: impl Into<String>) -> Self {
        self.profile_id = Some(profile_id.into());
        self
    }

    /// Clears any sticky default profile id.
    pub fn clear_profile_id(mut self) -> Self {
        self.profile_id = None;
        self
    }

    /// Returns the sticky default profile id, if any.
    pub fn profile_id(&self) -> Option<&str> {
        self.profile_id.as_deref()
    }

    /// Attaches a shared request lifecycle hook.
    pub fn with_request_hook(mut self, hook: hooks::SharedRequestHook) -> Self {
        self.request_hook = Some(hook);
        self
    }

    /// Returns the configured request hook, if any.
    pub fn request_hook(&self) -> Option<&hooks::SharedRequestHook> {
        self.request_hook.as_ref()
    }

    /// Rejects sticky test mode for an operation that Mollie exposes only in
    /// live mode.
    ///
    /// Business-operation routes such as balances, settlements, and invoices
    /// do not support the `testmode` query parameter. Keeping this check in
    /// the generated route lifecycle prevents a caller from accidentally
    /// making a live request after asking for test mode.
    #[allow(clippy::result_large_err)]
    pub(crate) fn reject_testmode_for(
        &self,
        operation: &str,
    ) -> Result<(), Error<types::ErrorResponse>> {
        if self.testmode.is_some() {
            return Err(Error::InvalidRequest(format!(
                "testmode is not supported for the `{operation}` operation"
            )));
        }

        Ok(())
    }

    /// Returns the configured base URL used for generated route requests.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::{Client, DEFAULT_BASE_URL};
    ///
    /// let client = Client::new(DEFAULT_BASE_URL).expect("default client");
    /// assert_eq!(client.baseurl(), DEFAULT_BASE_URL);
    /// ```
    pub fn baseurl(&self) -> &str {
        &self.baseurl
    }

    /// Returns the underlying HTTP client.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mollie_rs::{Client, DEFAULT_BASE_URL};
    ///
    /// let client = Client::new(DEFAULT_BASE_URL).expect("default client");
    /// let _http_client = client.http_client();
    /// ```
    pub fn http_client(&self) -> &ReqwestClient {
        &self.client
    }

    /// Join a generated API path onto the configured base URL.
    ///
    /// # Arguments
    ///
    /// * `path` — absolute API path such as `/payments` (no host).
    ///
    /// # Returns
    ///
    /// Full request URL string used by generated route methods.
    pub(crate) fn endpoint(&self, path: impl ::std::fmt::Display) -> String {
        let path = path.to_string();
        // OAuth token endpoints live on the API host root (`/oauth2/...`), not
        // under the `/v2` resource stem used by DEFAULT_BASE_URL.
        if path.starts_with("/oauth2/") {
            if let Ok(mut url) = ::reqwest::Url::parse(&self.baseurl) {
                url.set_path(&path);
                url.set_query(None);
                url.set_fragment(None);
                return url.to_string();
            }
        }
        format!("{}{}", self.baseurl, path)
    }

    /// Build a request with the common generated-route headers applied.
    ///
    /// Always sends an `Idempotency-Key` header. Resolution uses client state
    /// ([`Self::idempotency_key`]): a non-empty sticky key is reused; otherwise
    /// a UUID v4 is generated. The resolved key is returned so generated routes
    /// can attach it to the response envelope.
    ///
    /// # Arguments
    ///
    /// * `method` — HTTP method for the route.
    /// * `url` — absolute request URL (typically from [`Self::endpoint`]).
    ///
    /// # Returns
    ///
    /// A tuple of `(RequestBuilder, resolved_idempotency_key)`. The string is
    /// always non-empty and is the exact value sent as `Idempotency-Key`.
    ///
    /// # Errors
    ///
    /// Returns [`reqwest::header::InvalidHeaderValue`] when the resolved key
    /// cannot be encoded as an HTTP header value.
    pub(crate) fn request(
        &self,
        method: ::reqwest::Method,
        url: String,
    ) -> Result<(::reqwest::RequestBuilder, String), ::reqwest::header::InvalidHeaderValue> {
        let resolved_key: String = match self.idempotency_key.as_deref() {
            Some(value) if !value.is_empty() => value.to_string(),
            _ => uuid::Uuid::new_v4().to_string(),
        };

        let mut headers: reqwest::header::HeaderMap =
            ::reqwest::header::HeaderMap::with_capacity(2usize);
        headers.append(
            ::reqwest::header::HeaderName::from_static("api-version"),
            ::reqwest::header::HeaderValue::from_static(Self::api_version()),
        );
        headers.append("idempotency-key", resolved_key.as_str().try_into()?);

        Ok((
            self.client
                .request(method, url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(headers),
            resolved_key,
        ))
    }

    /// Execute a request through the generated client hook lifecycle.
    ///
    /// Runs `pre` hooks, performs the HTTP call, then runs `post` hooks from
    /// [`ClientHooks`].
    ///
    /// # Arguments
    ///
    /// * `request` — fully built `reqwest` request (method, URL, headers, body).
    /// * `operation` — generated operation id metadata for hooks and tracing.
    ///
    /// # Errors
    ///
    /// Propagates hook failures and transport errors as [`Error`].
    ///
    /// When [`Self::retry_policy`] is enabled, retries transient failures for
    /// safe reads, and for writes **only** when a sticky/caller-bound
    /// idempotency key is set on the client. Auto-generated per-request keys
    /// alone do **not** enable write retries. Retries require a cloneable body.
    ///
    /// Retry classification prefers [`route_capabilities::route_capability`] for
    /// the operation id and falls back to HTTP method classification.
    ///
    /// The retry budget ([`RetryPolicy::total_deadline`]) limits scheduling of
    /// further attempts and backoff. When the budget is exhausted the SDK
    /// returns the **last attempt’s result** and does **not** issue an extra
    /// leftover request.
    pub(crate) async fn send<E>(
        &self,
        mut request: ::reqwest::Request,
        operation: routes::Operation,
    ) -> Result<::reqwest::Response, Error<E>> {
        let info = operation.info();
        self.pre(&mut request, &info).await?;

        let policy = &self.retry_policy;
        // Registry is source of truth; method fallback never upgrades writes.
        let class = route_capabilities::retry_class_for_operation(
            operation.id(),
            request.method().as_str(),
        );
        let has_sticky = self
            .idempotency_key
            .as_ref()
            .is_some_and(|key| !key.is_empty());
        let may_retry = policy.allows(class, has_sticky);
        let max_attempts = if may_retry {
            policy.max_attempts.max(1)
        } else {
            1
        };
        let started = std::time::Instant::now();
        let retry_budget = policy.retry_budget();

        for attempt in 1..=max_attempts {
            // Invariant: remaining budget checked → attempt begins within budget
            // or no request is sent. Never a leftover send after budget exit.
            if attempt > 1 && started.elapsed() >= retry_budget {
                return Err(Error::InvalidRequest(format!(
                    "retry budget exhausted for operation `{}` (no further attempt sent)",
                    operation.id()
                )));
            }

            let method = request.method().as_str().to_string();
            let url_redacted = redact_url_for_hooks(request.url());
            let hook_ctx = hooks::RequestContext {
                operation: operation.id(),
                method,
                url_redacted,
                attempt,
                has_sticky_idempotency: has_sticky,
                profile_id: self.profile_id.clone(),
                testmode: self.testmode,
            };
            if let Some(hook) = self.request_hook.as_ref() {
                hook.before_request(&hook_ctx, &mut request);
            }

            let is_last = attempt == max_attempts;
            let request_for_attempt = if is_last {
                std::mem::replace(
                    &mut request,
                    ::reqwest::Request::new(
                        ::reqwest::Method::GET,
                        ::reqwest::Url::parse("http://127.0.0.1/").expect("static url"),
                    ),
                )
            } else if let Some(cloned) = request.try_clone() {
                cloned
            } else {
                // Non-cloneable body: single attempt only.
                let result = self.exec(request, &info).await;
                self.post(&result, &info).await?;
                return Ok(result?);
            };

            let result = self.exec(request_for_attempt, &info).await;

            if !is_last {
                // Delivery-aware retry (INV-DELIV-01 / INV-WRITE-01):
                // NotSent and Unknown may retry only when policy+class+sticky allow.
                // Rejected/Succeeded never auto-retry. Timeout is Unknown, not
                // "safe connection failure".
                let should_retry = match &result {
                    Ok(response) => {
                        let outcome = transport::classify_http_status(response.status());
                        transport::should_auto_retry(outcome, class, has_sticky, policy)
                            && transport::is_transient_http_status(response.status())
                    }
                    Err(err) => {
                        let outcome = transport::classify_reqwest_error(err);
                        transport::should_auto_retry(outcome, class, has_sticky, policy)
                    }
                };

                if should_retry {
                    let retry_after = result.as_ref().ok().and_then(|response| {
                        response
                            .headers()
                            .get("retry-after")
                            .and_then(|v| v.to_str().ok())
                            .and_then(|s| s.parse::<u64>().ok())
                            .map(std::time::Duration::from_secs)
                    });
                    let delay = transport::compute_backoff(policy, attempt + 1, retry_after);
                    // If backoff would push past the budget, return this attempt
                    // instead of sleeping and sending another leftover request.
                    if started.elapsed().saturating_add(delay) >= retry_budget {
                        tracing::debug!(
                            attempt,
                            operation = operation.id(),
                            budget_ms = retry_budget.as_millis() as u64,
                            "retry budget exhausted before backoff; returning last attempt"
                        );
                        self.post(&result, &info).await?;
                        return Ok(result?);
                    }
                    tracing::debug!(
                        attempt,
                        max_attempts,
                        delay_ms = delay.as_millis() as u64,
                        has_sticky_idempotency = has_sticky,
                        operation = operation.id(),
                        retry_class = ?class,
                        "retrying Mollie HTTP request"
                    );
                    let _ = result;
                    tokio::time::sleep(delay).await;
                    continue;
                }
            }

            if let (Some(hook), Ok(response)) = (self.request_hook.as_ref(), result.as_ref()) {
                let metadata = crate::metadata::ResponseMetadata::from_status_and_headers(
                    response.status(),
                    response.headers(),
                )
                .with_attempt(attempt);
                hook.after_response(&hook_ctx, &metadata);
            }

            self.post(&result, &info).await?;
            return Ok(result?);
        }

        Err(Error::InvalidRequest(format!(
            "retry budget exhausted for operation `{}`",
            operation.id()
        )))
    }
}

/// Redacts query strings for hook/logging surfaces.
///
/// Mollie resource paths rarely put secrets in the query; when a query is
/// present it is replaced with a marker so credentials never appear in hooks.
fn redact_url_for_hooks(url: &reqwest::Url) -> String {
    let mut redacted = url.clone();
    if redacted.query().is_some() {
        redacted.set_query(Some("<redacted>"));
    }
    redacted.to_string()
}

/// [`ClientInfo`] implementation used by progenitor-generated request helpers.
impl ClientInfo<()> for Client {
    /// Returns the OpenAPI / client API version string advertised to hooks.
    fn api_version() -> &'static str {
        "1.0.0"
    }

    /// Returns the configured base URL for this client instance.
    fn baseurl(&self) -> &str {
        self.baseurl.as_str()
    }

    /// Returns the shared `reqwest` client.
    fn client(&self) -> &reqwest::Client {
        &self.client
    }

    /// Returns the empty inner context unit value for this client.
    fn inner(&self) -> &() {
        &()
    }
}

/// Default [`ClientHooks`] implementation (no custom pre/post hooks).
impl ClientHooks<()> for &Client {}

/// Convenience re-exports of the types most applications import together.
///
/// Includes client construction, credentials, money helpers, response
/// envelopes, and [`ResponseValueExt`] for reading resolved idempotency keys.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::{
        load_dotenv, load_dotenv_from, var, var_optional, Address, AmountValue, ApiKey,
        ApplicationFee, ApplicationFeeDescription, BasicAuth, Client, CountryCode,
        CreatePaymentRequired, Credential, Currency, Date, DateTime, GeneratedMollieResult,
        IntoMollieFuture, IntoMollieResult, Locale, MollieClient, MollieClientBuilder,
        MollieEnvelope, MollieError, MollieErrorCatalogEntry, MollieErrorEnvelope, MollieErrorKey,
        MollieResponse, MollieResult, MollieSuccessEnvelope, MollieSuccessKey, Money,
        OAuthAccessToken, PaymentDescription, PaymentId, PaymentMethod, PhoneNumber, ProfileId,
        RedirectUrl, ResponseEnvelope, ResponseMetadata, ResponseValueExt, WebhookNotification,
        WebhookUrl, APPLICATION_FEE_DESCRIPTION_MAX_LEN, DEFAULT_BASE_URL, MOLLIE_API_KEY_ENV,
        MOLLIE_BASE_URL_ENV, MOLLIE_OAUTH_ACCESS_TOKEN_ENV, MOLLIE_OAUTH_CLIENT_ID_ENV,
        MOLLIE_OAUTH_CLIENT_SECRET_ENV,
    };

    #[cfg(feature = "app-helpers")]
    #[allow(unused_imports)]
    pub use super::{
        init_tracing, init_tracing_with_filter, try_init_tracing, try_init_tracing_with_filter,
    };
}

/// Unit tests for client request helpers and response-envelope idempotency.
#[cfg(test)]
mod tests {
    use super::{
        Client, Error, ResponseEnvelope, ResponseValue, ResponseValueExt, DEFAULT_BASE_URL,
    };
    use reqwest::StatusCode;

    /// Asserts `key` is a non-empty UUID v4 and not equal to any forbidden value.
    fn assert_generated_uuid_v4_idempotency_key(key: &str, forbidden: &[&str]) {
        assert!(
            !key.is_empty(),
            "resolved idempotency key must be non-empty"
        );
        for bad in forbidden {
            assert_ne!(
                key, *bad,
                "resolved key must not reuse invalid sticky value {bad:?}"
            );
        }
        let parsed = uuid::Uuid::parse_str(key)
            .unwrap_or_else(|err| panic!("expected UUID, got {key:?}: {err}"));
        assert_eq!(
            parsed.get_version(),
            Some(uuid::Version::Random),
            "expected UUID v4 (random), got version {:?} for {key}",
            parsed.get_version()
        );
    }

    /// Missing / blank sticky keys (and cleared keys) never become the wire value;
    /// `request` always resolves a fresh UUID v4 instead.
    #[test]
    fn request_generates_uuid_v4_for_missing_or_blank_idempotency_keys() {
        // Default client: no sticky key (`None`).
        let none_client = Client::new(DEFAULT_BASE_URL).expect("default client");
        assert!(none_client.idempotency_key().is_none());
        let (_builder, key_none) = none_client
            .request(reqwest::Method::GET, none_client.endpoint("/payments"))
            .expect("request should build");
        assert_generated_uuid_v4_idempotency_key(&key_none, &["", "None", "null"]);

        // Explicit empty string sticky key must not be sent.
        let empty_client = Client::new(DEFAULT_BASE_URL)
            .expect("default client")
            .with_idempotency_key("");
        assert_eq!(empty_client.idempotency_key(), Some(""));
        let (_builder, key_empty) = empty_client
            .request(reqwest::Method::POST, empty_client.endpoint("/payments"))
            .expect("request should build");
        assert_generated_uuid_v4_idempotency_key(&key_empty, &["", "None", "null"]);
        assert_ne!(
            key_empty, key_none,
            "auto-generated keys should not collide across independent requests"
        );

        // Empty string via clone helper.
        let empty_ref_client: Client = Client::new(DEFAULT_BASE_URL)
            .expect("default client")
            .with_idempotency_key_ref("");
        assert_eq!(empty_ref_client.idempotency_key(), Some(""));
        let (_builder, key_empty_ref) = empty_ref_client
            .request(reqwest::Method::GET, empty_ref_client.endpoint("/payments"))
            .expect("request should build");
        assert_generated_uuid_v4_idempotency_key(&key_empty_ref, &[""]);

        // Cleared sticky key restores auto generation (stored state is `None`).
        let cleared_client: Client = Client::new(DEFAULT_BASE_URL)
            .expect("default client")
            .with_idempotency_key("sticky-should-not-be-used")
            .clear_idempotency_key();
        assert!(cleared_client.idempotency_key().is_none());
        let (_builder, key_cleared) = cleared_client
            .request(reqwest::Method::GET, cleared_client.endpoint("/payments"))
            .expect("request should build");
        assert_generated_uuid_v4_idempotency_key(
            &key_cleared,
            &["", "sticky-should-not-be-used", "None", "null"],
        );

        // Two requests on the same blank client each get a distinct UUID v4.
        let (_builder, key_again) = empty_client
            .request(reqwest::Method::GET, empty_client.endpoint("/payments"))
            .expect("request should build");
        assert_generated_uuid_v4_idempotency_key(&key_again, &[""]);
        assert_ne!(
            key_empty, key_again,
            "blank sticky key must generate a new UUID per request, not reuse a fixed empty value"
        );
    }

    /// Sticky keys configured on the client are returned unchanged.
    #[test]
    fn request_preserves_sticky_idempotency_key() {
        let expected: &str = "6f7ef3e6-8c2f-4d1c-9f08-5ab7adf56c91";
        let client: Client = Client::new(DEFAULT_BASE_URL)
            .expect("default client")
            .with_idempotency_key(expected);
        let (_builder, key) = client
            .request(reqwest::Method::POST, client.endpoint("/payments"))
            .expect("request should build");
        assert_eq!(key, expected);
    }

    /// Sticky testmode is stored on the client.
    #[test]
    fn with_testmode_sets_sticky_flag() {
        let client: Client = Client::new(DEFAULT_BASE_URL)
            .expect("default client")
            .with_testmode(true);
        assert_eq!(client.testmode(), Some(true));
        let client: Client = client.with_testmode(false);
        assert_eq!(client.testmode(), Some(false));
    }

    /// `clear_testmode` restores the default (`None`).
    #[test]
    fn clear_testmode_restores_none() {
        let client: Client = Client::new(DEFAULT_BASE_URL)
            .expect("default client")
            .with_testmode(true)
            .clear_testmode();
        assert!(client.testmode().is_none());
    }

    /// `with_testmode_ref` clones without mutating the original.
    #[test]
    fn with_testmode_ref_leaves_original_unchanged() {
        let client: Client = Client::new(DEFAULT_BASE_URL).expect("default client");
        let scoped: Client = client.with_testmode_ref(true);
        assert!(client.testmode().is_none());
        assert_eq!(scoped.testmode(), Some(true));
    }

    /// Rejects sticky test mode for live-only business-operation routes.
    #[test]
    fn rejects_testmode_for_live_only_operations() {
        let client: Client = Client::new(DEFAULT_BASE_URL)
            .expect("default client")
            .with_testmode(false);
        let error = client
            .reject_testmode_for("list_settlements")
            .expect_err("live-only routes must reject configured testmode");

        assert!(matches!(error, Error::InvalidRequest(message) if message.contains("testmode")));
        assert!(client
            .clear_testmode()
            .reject_testmode_for("list_settlements")
            .is_ok());
    }

    /// Header-echoed keys are readable on [`ResponseValue`] and [`ResponseEnvelope`].
    #[test]
    fn response_envelope_exposes_idempotency_key_from_headers() {
        let mut headers: reqwest::header::HeaderMap = reqwest::header::HeaderMap::new();
        headers.insert(
            "idempotency-key",
            "6f7ef3e6-8c2f-4d1c-9f08-5ab7adf56c91"
                .parse()
                .expect("static header value"),
        );
        let response: ResponseValue<&str> = ResponseValue::new("ok", StatusCode::OK, headers);
        assert_eq!(
            response.idempotency_key(),
            Some("6f7ef3e6-8c2f-4d1c-9f08-5ab7adf56c91")
        );
        let envelope: ResponseEnvelope<&str> = ResponseEnvelope::from_response_value(response);
        assert_eq!(
            envelope.idempotency_key(),
            Some("6f7ef3e6-8c2f-4d1c-9f08-5ab7adf56c91")
        );
    }

    /// Empty list endpoints return `"count": 0`; `ListCount` must accept zero
    /// (OpenAPI previously claimed `minimum: 1`, which produced NonZeroU64).
    #[test]
    fn list_count_deserializes_zero() {
        let count: crate::types::ListCount =
            serde_json::from_str("0").expect("count 0 must decode");
        assert_eq!(*count, 0);
        assert_eq!(count.0, 0);

        let nonempty: crate::types::ListCount =
            serde_json::from_str("5").expect("count 5 must decode");
        assert_eq!(*nonempty, 5);
    }

    /// List-embedded entities often omit `_links.documentation` even when the
    /// OpenAPI schema marks it required for single-resource GET.
    #[test]
    fn entity_chargeback_links_allow_missing_documentation() {
        let json = r#"{
            "self": { "href": "https://api.mollie.com/v2/payments/tr_x/chargebacks/chb_x", "type": "application/hal+json" },
            "payment": { "href": "https://api.mollie.com/v2/payments/tr_x", "type": "application/hal+json" }
        }"#;
        let links: crate::types::EntityChargebackLinks =
            serde_json::from_str(json).expect("chargeback links without documentation");
        assert!(links.documentation.is_none());
        assert_eq!(
            links.payment.href,
            "https://api.mollie.com/v2/payments/tr_x"
        );
    }

    /// Same omission pattern as customers/chargebacks for refund list embeds.
    #[test]
    fn entity_refund_links_allow_missing_documentation() {
        let json = r#"{
            "self": { "href": "https://api.mollie.com/v2/payments/tr_x/refunds/re_x", "type": "application/hal+json" },
            "payment": { "href": "https://api.mollie.com/v2/payments/tr_x", "type": "application/hal+json" }
        }"#;
        let links: crate::types::EntityRefundLinks =
            serde_json::from_str(json).expect("refund links without documentation");
        assert!(links.documentation.is_none());
    }

    /// Live refunds return `"metadata": null`; decode as `None` (not required Metadata).
    #[test]
    fn entity_refund_allows_null_metadata() {
        let json = r#"{
            "resource": "refund",
            "id": "re_test",
            "mode": "live",
            "amount": { "value": "1.00", "currency": "EUR" },
            "status": "refunded",
            "createdAt": "2026-01-22T10:39:23+00:00",
            "description": "Credit",
            "metadata": null,
            "paymentId": "tr_test",
            "_links": {
                "self": { "href": "https://api.mollie.com/v2/payments/tr_test/refunds/re_test", "type": "application/hal+json" },
                "payment": { "href": "https://api.mollie.com/v2/payments/tr_test", "type": "application/hal+json" }
            }
        }"#;
        let refund: crate::types::EntityRefund =
            serde_json::from_str(json).expect("refund with null metadata");
        assert!(refund.metadata.is_none());
    }
}
