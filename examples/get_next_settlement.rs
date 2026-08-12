// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::get_next_settlement`.
//!
//! Route: `GET /settlements/next`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::EntitySettlement;
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::get_next_settlement`.
struct GetNextSettlementExample;

impl RunnableExample for GetNextSettlementExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "get_next_settlement";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /settlements/next";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let response: ResponseValue<EntitySettlement> =
                context.client().get_next_settlement().await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(GetNextSettlementExample).await
}
