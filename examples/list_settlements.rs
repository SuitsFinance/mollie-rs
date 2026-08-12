// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::list_settlements`.
//!
//! Route: `GET /settlements`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{BalanceToken, Currencies, ListSettlementsResponse};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::list_settlements`.
struct ListSettlementsExample;

impl RunnableExample for ListSettlementsExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "list_settlements";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /settlements";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let balance_id: BalanceToken = context.options().token("balance_id", "bal_1234567890");

            let response: ResponseValue<ListSettlementsResponse> = context
                .client()
                .list_settlements(
                    Some(&balance_id),
                    Some(
                        context
                            .options()
                            .configured("currencies", Currencies::Eur)?,
                    ),
                    context.options().optional_value("from"),
                    context.options().limit(50),
                    Some(context.options().value("month", "2026-01")),
                    Some(context.options().value("year", "2026")),
                )
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(ListSettlementsExample).await
}
