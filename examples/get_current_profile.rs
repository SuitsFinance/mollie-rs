// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::get_current_profile`.
//!
//! Route: `GET /profiles/me`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::ProfileResponse;
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::get_current_profile`.
struct GetCurrentProfileExample;

impl RunnableExample for GetCurrentProfileExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "get_current_profile";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /profiles/me";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let response: ResponseValue<ProfileResponse> =
                context.client().get_current_profile().await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(GetCurrentProfileExample).await
}
