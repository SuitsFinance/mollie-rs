//! Generated wallets route methods.

use crate::{routes, types, Client, Error, ResponseValue};

/// Generated `wallets` route methods on [`crate::Client`].
///
/// Client-owned request policy on [`crate::Client`]:
/// - idempotency via [`crate::Client::with_idempotency_key`] (default: UUID v4 per request)
/// - `testmode` query via [`crate::Client::with_testmode`] on operations that support it
///
/// The resolved idempotency key is returned on the response envelope
/// ([`crate::ResponseValue`] / [`crate::ResponseEnvelope`]).
#[allow(clippy::all)]
impl Client {
    /// Request Apple Pay payment session
    ///
    /// When integrating Apple Pay in your own checkout on the web, you need to
    /// [provide merchant validation](https://developer.apple.com/documentation/apple_pay_on_the_web/apple_pay_js_api/providing_merchant_validation).
    /// This is normally done using Apple's
    /// [Requesting an Apple Pay Session](https://developer.apple.com/documentation/apple_pay_on_the_web/apple_pay_js_api/requesting_an_apple_pay_payment_session).
    /// The merchant validation proves to Apple that a validated merchant is calling the Apple Pay Javascript APIs.
    ///
    /// To integrate Apple Pay via Mollie, you will have to call the Mollie API instead of Apple's API. The response of this
    /// API call can then be passed as-is to the completion method, `completeMerchantValidation`.
    ///
    /// Before requesting an Apple Pay Payment Session, you must place the domain validation file on your server at:
    /// `https://[domain]/.well-known/apple-developer-merchantid-domain-association`. Without this file, it will not be
    /// possible to use Apple Pay on your domain.
    ///
    /// Each new transaction requires a new payment session object. Merchant session objects are not reusable, and they
    /// expire after five minutes.
    ///
    /// Payment sessions cannot be requested directly from the browser. The request must be sent from your server. For the
    /// full documentation, see the official
    /// [Apple Pay JS API](https://developer.apple.com/documentation/apple_pay_on_the_web/apple_pay_js_api) documentation.
    ///
    /// Sends a `POST` request to `/wallets/applepay/sessions`
    ///
    /// Arguments:
    pub async fn request_apple_pay_payment_session<'a>(
        &'a self,
        body: &'a types::RequestApplePayPaymentSessionBody,
    ) -> Result<ResponseValue<types::EntitySession2>, Error<types::ErrorResponse>> {
        let url = self.endpoint("/wallets/applepay/sessions");
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::POST, url)?;
        #[allow(unused_mut)]
        let mut request = request.json(&body).build()?;
        let response = self
            .send(request, routes::Operation::RequestApplePayPaymentSession)
            .await?;
        routes::response::json(
            response,
            &[201u16],
            &[422u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }
}
