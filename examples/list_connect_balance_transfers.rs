// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::list_connect_balance_transfers`.
//!
//! Route: `GET /connect/balance-transfers`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{ListConnectBalanceTransfersResponse, Sorting};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::list_connect_balance_transfers`.
struct ListConnectBalanceTransfersExample;

impl RunnableExample for ListConnectBalanceTransfersExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "list_connect_balance_transfers";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /connect/balance-transfers";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let response: ResponseValue<ListConnectBalanceTransfersResponse> = context
                .client()
                .list_connect_balance_transfers(
                    context.options().optional_value("from"),
                    context.options().limit(50),
                    Some(context.options().configured("sort", Sorting::Desc)?),
                )
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(ListConnectBalanceTransfersExample).await
}
