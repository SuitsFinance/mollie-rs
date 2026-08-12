// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::list_unmatched_credit_transfers`.
//!
//! Route: `GET /unmatched-credit-transfers`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::ListUnmatchedCreditTransfersResponse;
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::list_unmatched_credit_transfers`.
struct ListUnmatchedCreditTransfersExample;

impl RunnableExample for ListUnmatchedCreditTransfersExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "list_unmatched_credit_transfers";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /unmatched-credit-transfers";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let response: ResponseValue<ListUnmatchedCreditTransfersResponse> = context
                .client()
                .list_unmatched_credit_transfers(
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
    support::run_example(ListUnmatchedCreditTransfersExample).await
}
