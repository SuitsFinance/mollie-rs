// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::payment_list_routes`.
//!
//! Route: `GET /payments/{paymentId}/routes`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{PaymentListRoutesResponse, PaymentToken};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::payment_list_routes`.
struct PaymentListRoutesExample;

impl RunnableExample for PaymentListRoutesExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "payment_list_routes";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /payments/{paymentId}/routes";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let payment_id: PaymentToken = context.options().token("payment_id", "tr_1234567890");

            let response: ResponseValue<PaymentListRoutesResponse> =
                context.client().payment_list_routes(&payment_id).await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(PaymentListRoutesExample).await
}
