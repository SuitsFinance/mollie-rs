// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::create_payment_link`.
//!
//! Route: `POST /payment-links`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{CreatePaymentLinkBody, PaymentLinkResponse};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::create_payment_link`.
struct CreatePaymentLinkExample;

impl RunnableExample for CreatePaymentLinkExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "create_payment_link";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "POST /payment-links";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let body: CreatePaymentLinkBody = context.options().body(from_value::<
                CreatePaymentLinkBody,
            >(json!({
                "amount": {
                    "currency": "EUR",
                    "value": "10.00"
                },
                "description": "Order #12345"
            }))?)?;

            let response: ResponseValue<PaymentLinkResponse> =
                context.client().create_payment_link(&body).await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(CreatePaymentLinkExample).await
}
