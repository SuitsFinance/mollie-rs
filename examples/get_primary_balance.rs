// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::get_primary_balance`.
//!
//! Route: `GET /balances/primary`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::EntityBalance;
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::get_primary_balance`.
struct GetPrimaryBalanceExample;

impl RunnableExample for GetPrimaryBalanceExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "get_primary_balance";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /balances/primary";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let response: ResponseValue<EntityBalance> =
                context.client().get_primary_balance().await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(GetPrimaryBalanceExample).await
}
