// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::list_balances`.
//!
//! Route: `GET /balances`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::ListBalancesResponse;
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::list_balances`.
struct ListBalancesExample;

impl RunnableExample for ListBalancesExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "list_balances";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /balances";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let response: ResponseValue<ListBalancesResponse> = context
                .client()
                .list_balances(
                    Some(context.options().value("currency", "EUR")),
                    context.options().optional_value("from"),
                    context.options().limit(50),
                )
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(ListBalancesExample).await
}
