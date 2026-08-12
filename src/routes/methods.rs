//! Generated methods route methods.

use crate::{routes, types, Client, Error, ResponseValue};
use progenitor_client::encode_path;

/// Generated `methods` route methods on [`crate::Client`].
///
/// Client-owned request policy on [`crate::Client`]:
/// - idempotency via [`crate::Client::with_idempotency_key`] (default: UUID v4 per request)
/// - `testmode` query via [`crate::Client::with_testmode`] on operations that support it
///
/// The resolved idempotency key is returned on the response envelope
/// ([`crate::ResponseValue`] / [`crate::ResponseEnvelope`]).
#[allow(clippy::all)]
impl Client {
    /// List payment methods
    ///
    /// Retrieve all enabled payment methods. The results of this endpoint are
    /// *not** paginated — unlike most other list endpoints in our API.
    ///
    /// For test mode, all pending and enabled payment methods are returned. If no
    /// payment methods are requested yet, the most popular payment methods are returned in the test mode. For live
    /// mode, only fully enabled payment methods are returned.
    ///
    /// Payment methods can be requested and enabled via the Mollie Dashboard, or
    /// via the [Enable payment method endpoint](enable-method) of the Profiles API.
    ///
    /// The list can optionally be filtered using a number of parameters described
    /// below.
    ///
    /// By default, only payment methods for the Euro currency are returned. If you
    /// wish to retrieve payment methods which exclusively support other currencies (e.g. Twint), you need to use the
    /// `amount` parameters.
    ///
    /// ℹ️ **Note:** This endpoint only returns **online** payment methods. If you wish to retrieve the information about
    /// a non-online payment method, you can use the [Get payment method endpoint](get-method).
    ///
    /// Sends a `GET` request to `/methods`
    ///
    /// Arguments:
    /// - `amount`: If supplied, only payment methods that support the amount and currency
    /// are returned.
    ///
    /// Example: `/v2/methods?amount[value]=100.00&amount[currency]=USD`
    /// - `billing_country`: The country taken from your customer's billing address in ISO 3166-1 alpha-2 format. This parameter can be used
    /// to check whether your customer is eligible for certain payment methods, for example for Klarna.
    ///
    /// Example: `/v2/methods?resource=orders&billingCountry=DE`
    /// - `include`: This endpoint allows you to include additional information via the `include` query string parameter.
    /// - `include_wallets`: A comma-separated list of the wallets you support in your checkout. Wallets often require wallet specific code
    /// to check if they are available on the shoppers device, hence the need to indicate your support.
    /// - `locale`: Response language
    /// - `order_line_categories`: A comma-separated list of the line categories you support in your checkout.
    ///
    /// Example: `/v2/methods?orderLineCategories=eco,meal`
    /// - `profile_id`: The identifier referring to the [profile](get-profile) you wish to
    /// retrieve the resources for.
    ///
    /// Most API credentials are linked to a single profile. In these cases the `profileId` must not be sent. For
    /// organization-level credentials such as OAuth access tokens however, the `profileId` parameter is required.
    /// - `resource`: **⚠️ We no longer recommend using the Orders API. Please refer to the [Payments API](payments-api) instead.**
    ///
    /// Indicate if you will use the result for the [Create order](create-order)
    /// or the [Create payment](create-payment) endpoint.
    ///
    /// When passing the value `orders`, the result will include payment methods
    /// that are only available for payments created via the Orders API.
    /// - `sequence_type`: Set this parameter to `first` to only return the enabled methods that
    /// can be used for the first payment of a recurring sequence.
    ///
    /// Set it to `recurring` to only return enabled methods that can be used for recurring payments or subscriptions.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn list_methods<'a>(
        &'a self,
        amount: Option<&'a types::Amount>,
        billing_country: Option<&'a str>,
        include: Option<&'a str>,
        include_wallets: Option<types::MethodIncludeWalletsParameter>,
        locale: Option<&'a types::Locale>,
        order_line_categories: Option<types::LineCategories>,
        profile_id: Option<&'a types::ProfileToken>,
        resource: Option<types::MethodResourceParameter>,
        sequence_type: Option<types::SequenceType>,
    ) -> Result<ResponseValue<types::ListMethodsResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint("/methods");
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        // OpenAPI `style: deepObject` for `amount` — progenitor QueryParam drops the
        // parameter name for structs and would emit bare `currency`/`value`.
        let amount_currency = amount.map(|amount| amount.currency.as_str());
        let amount_value = amount.map(|amount| amount.value.as_str());
        #[allow(unused_mut)]
        let mut request = request
            .query(&progenitor_client::QueryParam::new(
                "amount[currency]",
                &amount_currency,
            ))
            .query(&progenitor_client::QueryParam::new(
                "amount[value]",
                &amount_value,
            ))
            .query(&progenitor_client::QueryParam::new(
                "billingCountry",
                &billing_country,
            ))
            .query(&progenitor_client::QueryParam::new("include", &include))
            .query(&progenitor_client::QueryParam::new(
                "includeWallets",
                &include_wallets,
            ))
            .query(&progenitor_client::QueryParam::new("locale", &locale))
            .query(&progenitor_client::QueryParam::new(
                "orderLineCategories",
                &order_line_categories,
            ))
            .query(&progenitor_client::QueryParam::new(
                "profileId",
                &profile_id,
            ))
            .query(&progenitor_client::QueryParam::new("resource", &resource))
            .query(&progenitor_client::QueryParam::new(
                "sequenceType",
                &sequence_type,
            ))
            .query(&progenitor_client::QueryParam::new(
                "testmode",
                &self.testmode(),
            ))
            .build()?;
        let response = self.send(request, routes::Operation::ListMethods).await?;
        routes::response::json(
            response,
            &[200u16],
            &[400u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// List all payment methods
    ///
    /// Retrieve all payment methods that Mollie offers, regardless of the eligibility of the organization for the specific
    /// method. The results of this endpoint are **not** paginated — unlike most other list endpoints in our API.
    ///
    /// The list can optionally be filtered using a number of parameters described below.
    ///
    /// ℹ️ **Note:** This endpoint only returns **online** payment methods. If you wish to retrieve the information about
    /// a non-online payment method, you can use the [Get payment method endpoint](get-method).
    ///
    /// Sends a `GET` request to `/methods/all`
    ///
    /// Arguments:
    /// - `amount`: If supplied, only payment methods that support the amount and currency
    /// are returned.
    ///
    /// Example: `/v2/methods/all?amount[value]=100.00&amount[currency]=USD`
    /// - `include`: This endpoint allows you to include additional information via the `include` query string parameter.
    /// - `locale`: Response language
    /// - `profile_id`: The identifier referring to the [profile](get-profile) you wish to
    /// retrieve the resources for.
    ///
    /// Most API credentials are linked to a single profile. In these cases the `profileId` must not be sent. For
    /// organization-level credentials such as OAuth access tokens however, the `profileId` parameter is required.
    /// - `sequence_type`: Set this parameter to `first` to only return the methods that
    /// can be used for the first payment of a recurring sequence.
    ///
    /// Set it to `recurring` to only return methods that can be used for recurring payments or subscriptions.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn list_all_methods<'a>(
        &'a self,
        amount: Option<&'a types::Amount>,
        include: Option<&'a str>,
        locale: Option<&'a types::Locale>,
        profile_id: Option<&'a types::ProfileToken>,
        sequence_type: Option<types::SequenceType>,
    ) -> Result<ResponseValue<types::ListAllMethodsResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint("/methods/all");
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        // OpenAPI `style: deepObject` for `amount` — progenitor QueryParam drops the
        // parameter name for structs and would emit bare `currency`/`value`.
        let amount_currency = amount.map(|amount| amount.currency.as_str());
        let amount_value = amount.map(|amount| amount.value.as_str());
        #[allow(unused_mut)]
        let mut request = request
            .query(&progenitor_client::QueryParam::new(
                "amount[currency]",
                &amount_currency,
            ))
            .query(&progenitor_client::QueryParam::new(
                "amount[value]",
                &amount_value,
            ))
            .query(&progenitor_client::QueryParam::new("include", &include))
            .query(&progenitor_client::QueryParam::new("locale", &locale))
            .query(&progenitor_client::QueryParam::new(
                "profileId",
                &profile_id,
            ))
            .query(&progenitor_client::QueryParam::new(
                "sequenceType",
                &sequence_type,
            ))
            .query(&progenitor_client::QueryParam::new(
                "testmode",
                &self.testmode(),
            ))
            .build()?;
        let response = self
            .send(request, routes::Operation::ListAllMethods)
            .await?;
        routes::response::json(
            response,
            &[200u16],
            &[400u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Get payment method
    ///
    /// Retrieve a single payment method by its ID.
    ///
    /// If a method is not available on this profile, a `404 Not Found` response is
    /// returned. If the method is available but not enabled yet, a status `403 Forbidden` is returned. You can enable
    /// payments methods via the [Enable payment method endpoint](enable-method) of the Profiles API, or via
    /// the Mollie Dashboard.
    ///
    /// If you do not know the method's ID, you can use the [methods list
    /// endpoint](list-methods) to retrieve all payment methods that are available.
    ///
    /// Additionally, it is possible to check if wallet methods such as Apple Pay
    /// are enabled by passing the wallet ID (`applepay`) as the method ID.
    ///
    /// Sends a `GET` request to `/methods/{methodId}`
    ///
    /// Arguments:
    /// - `method_id`: Provide the ID of the related payment method.
    /// - `currency`: If provided, the `minimumAmount` and `maximumAmount` will be converted
    /// to the given currency. An error is returned if the currency is not supported by the payment method.
    /// - `include`: This endpoint allows you to include additional information via the `include` query string parameter.
    /// - `locale`: Response language
    /// - `profile_id`: The identifier referring to the [profile](get-profile) you wish to
    /// retrieve the resources for.
    ///
    /// Most API credentials are linked to a single profile. In these cases the `profileId` must not be sent. For
    /// organization-level credentials such as OAuth access tokens however, the `profileId` parameter is required.
    /// - `sequence_type`: Set this parameter to `first` to only return the methods that
    /// can be used for the first payment of a recurring sequence.
    ///
    /// Set it to `recurring` to only return methods that can be used for recurring payments or subscriptions.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn get_method<'a>(
        &'a self,
        method_id: &'a types::Method,
        currency: Option<&'a str>,
        include: Option<&'a str>,
        locale: Option<&'a types::Locale>,
        profile_id: Option<&'a types::ProfileToken>,
        sequence_type: Option<types::SequenceType>,
    ) -> Result<ResponseValue<types::EntityMethodGet>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/methods/{}",
            encode_path(&method_path_code(method_id))
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request
            .query(&progenitor_client::QueryParam::new("currency", &currency))
            .query(&progenitor_client::QueryParam::new("include", &include))
            .query(&progenitor_client::QueryParam::new("locale", &locale))
            .query(&progenitor_client::QueryParam::new(
                "profileId",
                &profile_id,
            ))
            .query(&progenitor_client::QueryParam::new(
                "sequenceType",
                &sequence_type,
            ))
            .query(&progenitor_client::QueryParam::new(
                "testmode",
                &self.testmode(),
            ))
            .build()?;
        let response = self.send(request, routes::Operation::GetMethod).await?;
        routes::response::json(
            response,
            &[200u16],
            &[400u16, 404u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Enable payment method
    ///
    /// Enable a payment method on a specific profile.
    ///
    /// When using a profile-specific API credential, the alias `me` can be used
    /// instead of the profile ID to refer to the current profile.
    ///
    /// Some payment methods require extra steps in order to be activated. In cases
    /// where a step at the payment method provider needs to be completed first, the status will be set to
    /// `pending-external` and the response will contain a link to complete the activation at the provider.
    ///
    /// To enable voucher or gift card issuers, refer to the [Enable payment method issuer](enable-method-issuer) endpoint.
    ///
    /// Sends a `POST` request to `/profiles/{profileId}/methods/{methodId}`
    ///
    /// Arguments:
    /// - `profile_id`: Provide the ID of the related profile.
    /// - `method_id`: Provide the ID of the related payment method.
    pub async fn enable_method<'a>(
        &'a self,
        profile_id: &'a types::EnableMethodProfileId,
        method_id: &'a types::Method,
    ) -> Result<ResponseValue<types::EntityMethodGet>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/profiles/{}/methods/{}",
            encode_path(&profile_id.to_string()),
            encode_path(&method_path_code(method_id))
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::POST, url)?;
        #[allow(unused_mut)]
        let mut request = request.build()?;
        let response = self.send(request, routes::Operation::EnableMethod).await?;
        routes::response::json(
            response,
            &[200u16],
            &[404u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Disable payment method
    ///
    /// Disable a payment method on a specific profile.
    ///
    /// When using a profile-specific API credential, the alias `me` can be used
    /// instead of the profile ID to refer to the current profile.
    ///
    /// Sends a `DELETE` request to `/profiles/{profileId}/methods/{methodId}`
    ///
    /// Arguments:
    /// - `profile_id`: Provide the ID of the related profile.
    /// - `method_id`: Provide the ID of the related payment method.
    pub async fn disable_method<'a>(
        &'a self,
        profile_id: &'a types::DisableMethodProfileId,
        method_id: &'a types::Method,
    ) -> Result<ResponseValue<()>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/profiles/{}/methods/{}",
            encode_path(&profile_id.to_string()),
            encode_path(&method_path_code(method_id))
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::DELETE, url)?;
        #[allow(unused_mut)]
        let mut request = request.build()?;
        let response = self.send(request, routes::Operation::DisableMethod).await?;
        routes::response::json(
            response,
            &[204u16],
            &[404u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Enable payment method issuer
    ///
    /// Enable an issuer for a payment method on a specific profile.
    ///
    /// Currently only the payment methods `voucher` and `giftcard` are supported.
    ///
    /// When using a profile-specific API credential, the alias `me` can be used instead of the profile ID to refer to the
    /// current profile.
    ///
    /// Sends a `POST` request to `/profiles/{profileId}/methods/{methodId}/issuers/{issuerId}`
    ///
    /// Arguments:
    /// - `profile_id`: Provide the ID of the related profile.
    /// - `method_id`: Provide the ID of the related payment method.
    /// - `issuer_id`: Provide the ID of the related issuer.
    pub async fn enable_method_issuer<'a>(
        &'a self,
        profile_id: &'a types::EnableMethodIssuerProfileId,
        method_id: types::MethodIdWithIssuer,
        issuer_id: &'a types::EnableMethodIssuerIssuerId,
        body: &'a types::EnableMethodIssuerBody,
    ) -> Result<ResponseValue<types::EnableMethodIssuerResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/profiles/{}/methods/{}/issuers/{}",
            encode_path(&profile_id.to_string()),
            encode_path(&method_id.to_string()),
            encode_path(&issuer_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::POST, url)?;
        #[allow(unused_mut)]
        let mut request = request.json(&body).build()?;
        let response = self
            .send(request, routes::Operation::EnableMethodIssuer)
            .await?;
        routes::response::json(
            response,
            &[200u16],
            &[404u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Disable payment method issuer
    ///
    /// Disable an issuer for a payment method on a specific profile.
    ///
    /// Currently only the payment methods `voucher` and `giftcard` are supported.
    ///
    /// When using a profile-specific API credential, the alias `me` can be used instead of the profile ID to refer to the
    /// current profile.
    ///
    /// Sends a `DELETE` request to `/profiles/{profileId}/methods/{methodId}/issuers/{issuerId}`
    ///
    /// Arguments:
    /// - `profile_id`: Provide the ID of the related profile.
    /// - `method_id`: Provide the ID of the related payment method.
    /// - `issuer_id`: Provide the ID of the related issuer.
    pub async fn disable_method_issuer<'a>(
        &'a self,
        profile_id: &'a types::DisableMethodIssuerProfileId,
        method_id: types::MethodIdWithIssuer,
        issuer_id: &'a types::DisableMethodIssuerIssuerId,
    ) -> Result<ResponseValue<()>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/profiles/{}/methods/{}/issuers/{}",
            encode_path(&profile_id.to_string()),
            encode_path(&method_id.to_string()),
            encode_path(&issuer_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::DELETE, url)?;
        #[allow(unused_mut)]
        let mut request = request.build()?;
        let response = self
            .send(request, routes::Operation::DisableMethodIssuer)
            .await?;
        routes::response::json(
            response,
            &[204u16],
            &[404u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }
}

fn method_path_code(method: &crate::types::Method) -> String {
    match &method.0 {
        Some(inner) => match serde_json::to_value(inner) {
            Ok(serde_json::Value::String(s)) => s,
            _ => String::new(),
        },
        None => String::new(),
    }
}
