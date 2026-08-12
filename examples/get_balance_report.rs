// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::get_balance_report`.
//!
//! Route: `GET /balances/{balanceId}/report`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{BalanceReportGrouping, BalanceToken, EntityBalanceReport};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::get_balance_report`.
struct GetBalanceReportExample;

impl RunnableExample for GetBalanceReportExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "get_balance_report";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /balances/{balanceId}/report";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let balance_id: BalanceToken = context.options().token("balance_id", "bal_1234567890");

            let response: ResponseValue<EntityBalanceReport> = context
                .client()
                .get_balance_report(
                    &balance_id,
                    context.options().value("from", "2026-01-01"),
                    Some(
                        context
                            .options()
                            .configured("grouping", BalanceReportGrouping::StatusBalances)?,
                    ),
                    context.options().value("until", "2026-01-01"),
                )
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(GetBalanceReportExample).await
}
