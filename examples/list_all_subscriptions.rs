// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::list_all_subscriptions`.
//!
//! Route: `GET /subscriptions`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::ListAllSubscriptionsResponse;
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::list_all_subscriptions`.
struct ListAllSubscriptionsExample;

impl RunnableExample for ListAllSubscriptionsExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "list_all_subscriptions";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /subscriptions";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let response: ResponseValue<ListAllSubscriptionsResponse> = context
                .client()
                .list_all_subscriptions(
                    context.options().optional_value("from"),
                    context.options().limit(50),
                    context.options().optional_value("profile_id"),
                )
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(ListAllSubscriptionsExample).await
}
