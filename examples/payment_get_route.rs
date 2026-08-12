// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::payment_get_route`.
//!
//! Route: `GET /payments/{paymentId}/routes/{routeId}`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{ConnectRouteToken, PaymentToken, RouteGetResponse};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::payment_get_route`.
struct PaymentGetRouteExample;

impl RunnableExample for PaymentGetRouteExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "payment_get_route";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /payments/{paymentId}/routes/{routeId}";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let payment_id: PaymentToken = context.options().token("payment_id", "tr_1234567890");
            let route_id: ConnectRouteToken = context
                .options()
                .configured("route_id", from_value::<ConnectRouteToken>(json!({}))?)?;

            let response: ResponseValue<RouteGetResponse> = context
                .client()
                .payment_get_route(&payment_id, &route_id)
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(PaymentGetRouteExample).await
}
