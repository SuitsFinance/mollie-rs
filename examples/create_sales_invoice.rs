// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::create_sales_invoice`.
//!
//! Route: `POST /sales-invoices`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{SalesInvoiceRequest, SalesInvoiceResponse};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::create_sales_invoice`.
struct CreateSalesInvoiceExample;

impl RunnableExample for CreateSalesInvoiceExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "create_sales_invoice";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "POST /sales-invoices";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let body: SalesInvoiceRequest = context
                .options()
                .body(from_value::<SalesInvoiceRequest>(json!({}))?)?;

            let response: ResponseValue<SalesInvoiceResponse> =
                context.client().create_sales_invoice(&body).await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(CreateSalesInvoiceExample).await
}
