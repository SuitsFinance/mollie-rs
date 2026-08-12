// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::list_subscription_payments`.
//!
//! Route: `GET /customers/{customerId}/subscriptions/{subscriptionId}/payments`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{
    CustomerToken, ListSubscriptionPaymentsResponse, PaymentToken, ProfileToken, Sorting,
    SubscriptionToken,
};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::list_subscription_payments`.
struct ListSubscriptionPaymentsExample;

impl RunnableExample for ListSubscriptionPaymentsExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "list_subscription_payments";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str =
        "GET /customers/{customerId}/subscriptions/{subscriptionId}/payments";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let customer_id: CustomerToken =
                context.options().token("customer_id", "cst_1234567890");
            let subscription_id: SubscriptionToken =
                context.options().token("subscription_id", "sub_1234567890");
            let from: Option<PaymentToken> = context.options().optional_token("from");
            let profile_id: Option<ProfileToken> = context.options().optional_token("profile_id");

            let response: ResponseValue<ListSubscriptionPaymentsResponse> = context
                .client()
                .list_subscription_payments(
                    &customer_id,
                    &subscription_id,
                    from.as_ref(),
                    context.options().limit(50),
                    profile_id.as_ref(),
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
    support::run_example(ListSubscriptionPaymentsExample).await
}
