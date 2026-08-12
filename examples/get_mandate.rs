// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::get_mandate`.
//!
//! Route: `GET /customers/{customerId}/mandates/{mandateId}`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{CustomerToken, MandateResponse, MandateToken};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::get_mandate`.
struct GetMandateExample;

impl RunnableExample for GetMandateExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "get_mandate";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /customers/{customerId}/mandates/{mandateId}";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let customer_id: CustomerToken =
                context.options().token("customer_id", "cst_1234567890");
            let mandate_id: MandateToken = context.options().token("mandate_id", "mdt_1234567890");

            let response: ResponseValue<MandateResponse> = context
                .client()
                .get_mandate(&customer_id, &mandate_id)
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(GetMandateExample).await
}
