// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::cancel_payment`.
//!
//! Route: `DELETE /payments/{paymentId}`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{CancelPaymentBody, PaymentResponse, PaymentToken};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::cancel_payment`.
struct CancelPaymentExample;

impl RunnableExample for CancelPaymentExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "cancel_payment";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "DELETE /payments/{paymentId}";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let payment_id: PaymentToken = context.options().token("payment_id", "tr_1234567890");
            let body: CancelPaymentBody = context.options().body(CancelPaymentBody::default())?;

            let response: ResponseValue<PaymentResponse> =
                context.client().cancel_payment(&payment_id, &body).await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(CancelPaymentExample).await
}
