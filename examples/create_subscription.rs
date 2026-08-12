// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::create_subscription`.
//!
//! Route: `POST /customers/{customerId}/subscriptions`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{CustomerToken, SubscriptionRequest, SubscriptionResponse};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::create_subscription`.
struct CreateSubscriptionExample;

impl RunnableExample for CreateSubscriptionExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "create_subscription";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "POST /customers/{customerId}/subscriptions";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let customer_id: CustomerToken =
                context.options().token("customer_id", "cst_1234567890");
            let body: SubscriptionRequest =
                context.options().body(SubscriptionRequest::default())?;

            let response: ResponseValue<SubscriptionResponse> = context
                .client()
                .create_subscription(&customer_id, &body)
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(CreateSubscriptionExample).await
}
