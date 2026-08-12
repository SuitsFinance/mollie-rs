// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::create_refund`.
//!
//! Route: `POST /payments/{paymentId}/refunds`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{EntityRefundResponse, PaymentToken, RefundRequest};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::create_refund`.
struct CreateRefundExample;

impl RunnableExample for CreateRefundExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "create_refund";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "POST /payments/{paymentId}/refunds";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let payment_id: PaymentToken = context.options().token("payment_id", "tr_1234567890");
            let body: RefundRequest = context
                .options()
                .body(from_value::<RefundRequest>(json!({}))?)?;

            let response: ResponseValue<EntityRefundResponse> =
                context.client().create_refund(&payment_id, &body).await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(CreateRefundExample).await
}
