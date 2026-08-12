// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::create_capture`.
//!
//! Route: `POST /payments/{paymentId}/captures`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{CaptureResponse, EntityCapture, PaymentToken};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::create_capture`.
struct CreateCaptureExample;

impl RunnableExample for CreateCaptureExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "create_capture";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "POST /payments/{paymentId}/captures";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let payment_id: PaymentToken = context.options().token("payment_id", "tr_1234567890");
            let body: EntityCapture = context.options().body(EntityCapture::default())?;

            let response: ResponseValue<CaptureResponse> =
                context.client().create_capture(&payment_id, &body).await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(CreateCaptureExample).await
}
