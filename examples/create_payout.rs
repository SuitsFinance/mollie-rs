// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::create_payout`.
//!
//! Route: `POST /payouts`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{EntityPayoutResponse, PayoutRequest};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::create_payout`.
struct CreatePayoutExample;

impl RunnableExample for CreatePayoutExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "create_payout";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "POST /payouts";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let body: PayoutRequest = context
                .options()
                .body(from_value::<PayoutRequest>(json!({}))?)?;

            let response: ResponseValue<EntityPayoutResponse> =
                context.client().create_payout(&body).await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(CreatePayoutExample).await
}
