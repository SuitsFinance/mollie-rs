// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::request_apple_pay_payment_session`.
//!
//! Route: `POST /wallets/applepay/sessions`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{EntitySession2, RequestApplePayPaymentSessionBody};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::request_apple_pay_payment_session`.
struct RequestApplePayPaymentSessionExample;

impl RunnableExample for RequestApplePayPaymentSessionExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "request_apple_pay_payment_session";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "POST /wallets/applepay/sessions";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let body: RequestApplePayPaymentSessionBody = context.options().body(from_value::<RequestApplePayPaymentSessionBody>(json!({
                "domain": "pay.example.com",
                "validationUrl": "https://apple-pay-gateway-cert.apple.com/paymentservices/paymentSession"
            }))?)?;

            let response: ResponseValue<EntitySession2> = context
                .client()
                .request_apple_pay_payment_session(&body)
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(RequestApplePayPaymentSessionExample).await
}
