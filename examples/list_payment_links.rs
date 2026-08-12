// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::list_payment_links`.
//!
//! Route: `GET /payment-links`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{ListPaymentLinksResponse, PaymentLinkToken};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::list_payment_links`.
struct ListPaymentLinksExample;

impl RunnableExample for ListPaymentLinksExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "list_payment_links";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /payment-links";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let from: Option<PaymentLinkToken> = context.options().optional_token("from");

            let response: ResponseValue<ListPaymentLinksResponse> = context
                .client()
                .list_payment_links(from.as_ref(), context.options().limit(50))
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(ListPaymentLinksExample).await
}
