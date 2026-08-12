// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::list_balance_transactions`.
//!
//! Route: `GET /balances/{balanceId}/transactions`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{BalanceToken, ListBalanceTransactionsResponse};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::list_balance_transactions`.
struct ListBalanceTransactionsExample;

impl RunnableExample for ListBalanceTransactionsExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "list_balance_transactions";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /balances/{balanceId}/transactions";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let balance_id: BalanceToken = context.options().token("balance_id", "bal_1234567890");

            let response: ResponseValue<ListBalanceTransactionsResponse> = context
                .client()
                .list_balance_transactions(
                    &balance_id,
                    context.options().optional_value("from"),
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
    support::run_example(ListBalanceTransactionsExample).await
}
