// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::list_subscriptions`.
//!
//! Route: `GET /customers/{customerId}/subscriptions`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{CustomerToken, ListSubscriptionsResponse, Sorting, SubscriptionToken};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::list_subscriptions`.
struct ListSubscriptionsExample;

impl RunnableExample for ListSubscriptionsExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "list_subscriptions";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /customers/{customerId}/subscriptions";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let customer_id: CustomerToken =
                context.options().token("customer_id", "cst_1234567890");
            let from: Option<SubscriptionToken> = context.options().optional_token("from");

            let response: ResponseValue<ListSubscriptionsResponse> = context
                .client()
                .list_subscriptions(
                    &customer_id,
                    from.as_ref(),
                    context.options().limit(50),
                    Some(context.options().configured("sort", Sorting::Desc)?),
                )
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(ListSubscriptionsExample).await
}
