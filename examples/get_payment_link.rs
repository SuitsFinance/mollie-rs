// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::get_payment_link`.
//!
//! Route: `GET /payment-links/{paymentLinkId}`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{PaymentLinkResponse, PaymentLinkToken};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::get_payment_link`.
struct GetPaymentLinkExample;

impl RunnableExample for GetPaymentLinkExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "get_payment_link";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /payment-links/{paymentLinkId}";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let payment_link_id: PaymentLinkToken =
                context.options().token("payment_link_id", "pl_1234567890");

            let response: ResponseValue<PaymentLinkResponse> =
                context.client().get_payment_link(&payment_link_id).await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(GetPaymentLinkExample).await
}
