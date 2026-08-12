// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::return_unmatched_credit_transfer`.
//!
//! Route: `POST /unmatched-credit-transfers/{unmatchedCreditTransferId}/return`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{UnmatchedCreditTransferActionResponse, UnmatchedCreditTransferToken};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::return_unmatched_credit_transfer`.
struct ReturnUnmatchedCreditTransferExample;

impl RunnableExample for ReturnUnmatchedCreditTransferExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "return_unmatched_credit_transfer";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str =
        "POST /unmatched-credit-transfers/{unmatchedCreditTransferId}/return";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let unmatched_credit_transfer_id: UnmatchedCreditTransferToken =
                context.options().configured(
                    "unmatched_credit_transfer_id",
                    from_value::<UnmatchedCreditTransferToken>(json!({}))?,
                )?;

            let response: ResponseValue<UnmatchedCreditTransferActionResponse> = context
                .client()
                .return_unmatched_credit_transfer(&unmatched_credit_transfer_id)
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(ReturnUnmatchedCreditTransferExample).await
}
