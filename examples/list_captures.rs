// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::list_captures`.
//!
//! Route: `GET /payments/{paymentId}/captures`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{CaptureToken, ListCapturesResponse, PaymentToken};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::list_captures`.
struct ListCapturesExample;

impl RunnableExample for ListCapturesExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "list_captures";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /payments/{paymentId}/captures";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let payment_id: PaymentToken = context.options().token("payment_id", "tr_1234567890");
            let from: Option<CaptureToken> = context.options().optional_token("from");

            let response: ResponseValue<ListCapturesResponse> = context
                .client()
                .list_captures(
                    &payment_id,
                    Some(context.options().value("embed", "payments")),
                    from.as_ref(),
                    context.options().limit(50),
                )
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(ListCapturesExample).await
}
