// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::list_clients`.
//!
//! Route: `GET /clients`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::ListClientsResponse;
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::list_clients`.
struct ListClientsExample;

impl RunnableExample for ListClientsExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "list_clients";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /clients";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let response: ResponseValue<ListClientsResponse> = context
                .client()
                .list_clients(
                    Some(context.options().value("embed", "payments")),
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
    support::run_example(ListClientsExample).await
}
