// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::get_sales_invoice`.
//!
//! Route: `GET /sales-invoices/{salesInvoiceId}`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{SalesInvoiceResponse, SalesInvoiceToken};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::get_sales_invoice`.
struct GetSalesInvoiceExample;

impl RunnableExample for GetSalesInvoiceExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "get_sales_invoice";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /sales-invoices/{salesInvoiceId}";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let sales_invoice_id: SalesInvoiceToken = context.options().configured(
                "sales_invoice_id",
                from_value::<SalesInvoiceToken>(json!({}))?,
            )?;

            let response: ResponseValue<SalesInvoiceResponse> = context
                .client()
                .get_sales_invoice(&sales_invoice_id)
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(GetSalesInvoiceExample).await
}
