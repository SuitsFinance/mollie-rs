// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::get_open_settlement`.
//!
//! Route: `GET /settlements/open`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::EntitySettlement;
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::get_open_settlement`.
struct GetOpenSettlementExample;

impl RunnableExample for GetOpenSettlementExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "get_open_settlement";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /settlements/open";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let response: ResponseValue<EntitySettlement> =
                context.client().get_open_settlement().await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(GetOpenSettlementExample).await
}
