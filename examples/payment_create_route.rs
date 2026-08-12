// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::payment_create_route`.
//!
//! Route: `POST /payments/{paymentId}/routes`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{PaymentToken, RouteCreateRequest, RouteCreateResponse};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::payment_create_route`.
struct PaymentCreateRouteExample;

impl RunnableExample for PaymentCreateRouteExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "payment_create_route";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "POST /payments/{paymentId}/routes";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let payment_id: PaymentToken = context.options().token("payment_id", "tr_1234567890");
            let body: RouteCreateRequest = context
                .options()
                .body(from_value::<RouteCreateRequest>(json!({}))?)?;

            let response: ResponseValue<RouteCreateResponse> = context
                .client()
                .payment_create_route(&payment_id, &body)
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(PaymentCreateRouteExample).await
}
