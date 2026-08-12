// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::get_payout`.
//!
//! Route: `GET /payouts/{payoutId}`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::EntityPayoutResponse;
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::get_payout`.
struct GetPayoutExample;

impl RunnableExample for GetPayoutExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "get_payout";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /payouts/{payoutId}";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let response: ResponseValue<EntityPayoutResponse> = context
                .client()
                .get_payout(context.options().value("payout_id", "example-id"))
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(GetPayoutExample).await
}
