// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::list_customers`.
//!
//! Route: `GET /customers`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{CustomerToken, ListCustomersResponse, Sorting};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::list_customers`.
struct ListCustomersExample;

impl RunnableExample for ListCustomersExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "list_customers";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /customers";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let from: Option<CustomerToken> = context.options().optional_token("from");

            let response: ResponseValue<ListCustomersResponse> = context
                .client()
                .list_customers(
                    from.as_ref(),
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
    support::run_example(ListCustomersExample).await
}
