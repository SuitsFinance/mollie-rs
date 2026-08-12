// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::list_terminals`.
//!
//! Route: `GET /terminals`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{ListTerminalsResponse, Sorting, TerminalToken};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::list_terminals`.
struct ListTerminalsExample;

impl RunnableExample for ListTerminalsExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "list_terminals";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /terminals";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let from: Option<TerminalToken> = context.options().optional_token("from");

            let response: ResponseValue<ListTerminalsResponse> = context
                .client()
                .list_terminals(
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
    support::run_example(ListTerminalsExample).await
}
