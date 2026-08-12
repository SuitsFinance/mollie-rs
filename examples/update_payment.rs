// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::update_payment`.
//!
//! Route: `PATCH /payments/{paymentId}`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{PaymentResponse, PaymentToken, UpdatePaymentBody};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::update_payment`.
struct UpdatePaymentExample;

impl RunnableExample for UpdatePaymentExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "update_payment";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "PATCH /payments/{paymentId}";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let payment_id: PaymentToken = context.options().token("payment_id", "tr_1234567890");
            let body: UpdatePaymentBody = context.options().body(UpdatePaymentBody::default())?;

            let response: ResponseValue<PaymentResponse> =
                context.client().update_payment(&payment_id, &body).await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(UpdatePaymentExample).await
}
