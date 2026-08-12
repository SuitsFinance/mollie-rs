// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::get_payment`.
//!
//! Route: `GET /payments/{paymentId}`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{PaymentResponse, PaymentToken};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::get_payment`.
struct GetPaymentExample;

impl RunnableExample for GetPaymentExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "get_payment";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /payments/{paymentId}";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let payment_id: PaymentToken = context.options().token("payment_id", "tr_1234567890");

            let response: ResponseValue<PaymentResponse> = context
                .client()
                .get_payment(
                    &payment_id,
                    Some(context.options().value("embed", "payments")),
                    Some(context.options().value("include", "issuers")),
                )
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(GetPaymentExample).await
}
