// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::list_permissions`.
//!
//! Route: `GET /permissions`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::ListPermissionsResponse;
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::list_permissions`.
struct ListPermissionsExample;

impl RunnableExample for ListPermissionsExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "list_permissions";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /permissions";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let response: ResponseValue<ListPermissionsResponse> =
                context.client().list_permissions().await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(ListPermissionsExample).await
}
