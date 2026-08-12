// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::list_invoices`.
//!
//! Route: `GET /invoices`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{ListInvoicesResponse, Sorting};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::list_invoices`.
struct ListInvoicesExample;

impl RunnableExample for ListInvoicesExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "list_invoices";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /invoices";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let response: ResponseValue<ListInvoicesResponse> = context
                .client()
                .list_invoices(
                    context.options().optional_value("from"),
                    context.options().limit(50),
                    Some(context.options().value("reference", "INV-12345")),
                    Some(context.options().configured("sort", Sorting::Desc)?),
                    Some(context.options().value("year", "2026")),
                )
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(ListInvoicesExample).await
}
