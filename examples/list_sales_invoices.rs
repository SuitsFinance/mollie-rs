// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::list_sales_invoices`.
//!
//! Route: `GET /sales-invoices`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::ListSalesInvoicesResponse;
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::list_sales_invoices`.
struct ListSalesInvoicesExample;

impl RunnableExample for ListSalesInvoicesExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "list_sales_invoices";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /sales-invoices";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let response: ResponseValue<ListSalesInvoicesResponse> = context
                .client()
                .list_sales_invoices(
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
    support::run_example(ListSalesInvoicesExample).await
}
