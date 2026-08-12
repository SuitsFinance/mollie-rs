// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::get_unmatched_credit_transfer`.
//!
//! Route: `GET /unmatched-credit-transfers/{unmatchedCreditTransferId}`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{EntityUnmatchedCreditTransfer, UnmatchedCreditTransferToken};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::get_unmatched_credit_transfer`.
struct GetUnmatchedCreditTransferExample;

impl RunnableExample for GetUnmatchedCreditTransferExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "get_unmatched_credit_transfer";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /unmatched-credit-transfers/{unmatchedCreditTransferId}";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let unmatched_credit_transfer_id: UnmatchedCreditTransferToken =
                context.options().configured(
                    "unmatched_credit_transfer_id",
                    from_value::<UnmatchedCreditTransferToken>(json!({}))?,
                )?;

            let response: ResponseValue<EntityUnmatchedCreditTransfer> = context
                .client()
                .get_unmatched_credit_transfer(&unmatched_credit_transfer_id)
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(GetUnmatchedCreditTransferExample).await
}
