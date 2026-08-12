// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::get_capture`.
//!
//! Route: `GET /payments/{paymentId}/captures/{captureId}`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{CaptureResponse, CaptureToken, PaymentToken};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::get_capture`.
struct GetCaptureExample;

impl RunnableExample for GetCaptureExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "get_capture";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /payments/{paymentId}/captures/{captureId}";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let payment_id: PaymentToken = context.options().token("payment_id", "tr_1234567890");
            let capture_id: CaptureToken = context.options().token("capture_id", "cpt_1234567890");

            let response: ResponseValue<CaptureResponse> = context
                .client()
                .get_capture(
                    &payment_id,
                    &capture_id,
                    Some(context.options().value("embed", "payments")),
                )
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(GetCaptureExample).await
}
