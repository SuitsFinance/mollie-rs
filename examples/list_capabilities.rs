// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::list_capabilities`.
//!
//! Route: `GET /capabilities`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::ListCapabilitiesResponse;
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::list_capabilities`.
struct ListCapabilitiesExample;

impl RunnableExample for ListCapabilitiesExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "list_capabilities";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /capabilities";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let response: ResponseValue<ListCapabilitiesResponse> =
                context.client().list_capabilities().await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(ListCapabilitiesExample).await
}
