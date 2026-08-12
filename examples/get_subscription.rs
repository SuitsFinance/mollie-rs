// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::get_subscription`.
//!
//! Route: `GET /customers/{customerId}/subscriptions/{subscriptionId}`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{CustomerToken, SubscriptionResponse, SubscriptionToken};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::get_subscription`.
struct GetSubscriptionExample;

impl RunnableExample for GetSubscriptionExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "get_subscription";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /customers/{customerId}/subscriptions/{subscriptionId}";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let customer_id: CustomerToken =
                context.options().token("customer_id", "cst_1234567890");
            let subscription_id: SubscriptionToken =
                context.options().token("subscription_id", "sub_1234567890");

            let response: ResponseValue<SubscriptionResponse> = context
                .client()
                .get_subscription(&customer_id, &subscription_id)
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(GetSubscriptionExample).await
}
