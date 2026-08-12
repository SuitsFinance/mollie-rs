// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::get_invoice`.
//!
//! Route: `GET /invoices/{invoiceId}`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{EntityInvoice, InvoiceToken};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::get_invoice`.
struct GetInvoiceExample;

impl RunnableExample for GetInvoiceExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "get_invoice";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /invoices/{invoiceId}";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let invoice_id: InvoiceToken = context
                .options()
                .configured("invoice_id", from_value::<InvoiceToken>(json!({}))?)?;

            let response: ResponseValue<EntityInvoice> =
                context.client().get_invoice(&invoice_id).await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(GetInvoiceExample).await
}
