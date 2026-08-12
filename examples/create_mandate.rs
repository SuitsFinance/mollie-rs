// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::create_mandate`.
//!
//! Route: `POST /customers/{customerId}/mandates`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{CustomerToken, MandateRequest, MandateResponse};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::create_mandate`.
struct CreateMandateExample;

impl RunnableExample for CreateMandateExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "create_mandate";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "POST /customers/{customerId}/mandates";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let customer_id: CustomerToken =
                context.options().token("customer_id", "cst_1234567890");
            let body: MandateRequest =
                context.options().body(from_value::<MandateRequest>(json!({
                    "consumerAccount": "NL55INGB0000000000",
                    "consumerName": "Jane Doe",
                    "method": "directdebit",
                    "signatureDate": "2026-01-01"
                }))?)?;

            let response: ResponseValue<MandateResponse> =
                context.client().create_mandate(&customer_id, &body).await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(CreateMandateExample).await
}
