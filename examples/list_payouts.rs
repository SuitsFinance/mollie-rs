// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::list_payouts`.
//!
//! Route: `GET /payouts`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{ListPayoutsBalanceId, ListPayoutsResponse, Sorting};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::list_payouts`.
struct ListPayoutsExample;

impl RunnableExample for ListPayoutsExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "list_payouts";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /payouts";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let balance_id: ListPayoutsBalanceId = context
                .options()
                .configured("balance_id", from_value::<ListPayoutsBalanceId>(json!({}))?)?;

            let response: ResponseValue<ListPayoutsResponse> = context
                .client()
                .list_payouts(
                    Some(&balance_id),
                    context.options().optional_value("from"),
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
    support::run_example(ListPayoutsExample).await
}
