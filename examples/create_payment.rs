// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::create_payment`.
//!
//! Route: `POST /payments`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{PaymentRequest, PaymentResponse};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::create_payment`.
struct CreatePaymentExample;

impl RunnableExample for CreatePaymentExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "create_payment";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "POST /payments";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let body: PaymentRequest = context
                .options()
                .body(from_value::<PaymentRequest>(json!({}))?)?;

            let response: ResponseValue<PaymentResponse> = context
                .client()
                .create_payment(Some(context.options().value("include", "issuers")), &body)
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(CreatePaymentExample).await
}
