// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::list_profiles`.
//!
//! Route: `GET /profiles`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::ListProfilesResponse;
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::list_profiles`.
struct ListProfilesExample;

impl RunnableExample for ListProfilesExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "list_profiles";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /profiles";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let response: ResponseValue<ListProfilesResponse> = context
                .client()
                .list_profiles(
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
    support::run_example(ListProfilesExample).await
}
