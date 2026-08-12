// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::get_terminal`.
//!
//! Route: `GET /terminals/{terminalId}`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{EntityTerminal, TerminalToken};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::get_terminal`.
struct GetTerminalExample;

impl RunnableExample for GetTerminalExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "get_terminal";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /terminals/{terminalId}";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let terminal_id: TerminalToken =
                context.options().token("terminal_id", "term_1234567890");

            let response: ResponseValue<EntityTerminal> =
                context.client().get_terminal(&terminal_id).await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(GetTerminalExample).await
}
