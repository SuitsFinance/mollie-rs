// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::get_balance`.
//!
//! Route: `GET /balances/{balanceId}`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{BalanceToken, EntityBalance};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::get_balance`.
struct GetBalanceExample;

impl RunnableExample for GetBalanceExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "get_balance";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /balances/{balanceId}";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let balance_id: BalanceToken = context.options().token("balance_id", "bal_1234567890");

            let response: ResponseValue<EntityBalance> =
                context.client().get_balance(&balance_id).await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(GetBalanceExample).await
}
