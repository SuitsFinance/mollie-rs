// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::match_unmatched_credit_transfer`.
//!
//! Route: `POST /unmatched-credit-transfers/{unmatchedCreditTransferId}/match`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{
    UnmatchedCreditTransferActionResponse, UnmatchedCreditTransferMatchRequest,
    UnmatchedCreditTransferToken,
};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::match_unmatched_credit_transfer`.
struct MatchUnmatchedCreditTransferExample;

impl RunnableExample for MatchUnmatchedCreditTransferExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "match_unmatched_credit_transfer";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str =
        "POST /unmatched-credit-transfers/{unmatchedCreditTransferId}/match";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let unmatched_credit_transfer_id: UnmatchedCreditTransferToken =
                context.options().configured(
                    "unmatched_credit_transfer_id",
                    from_value::<UnmatchedCreditTransferToken>(json!({}))?,
                )?;
            let body: UnmatchedCreditTransferMatchRequest =
                context
                    .options()
                    .body(from_value::<UnmatchedCreditTransferMatchRequest>(
                        json!({}),
                    )?)?;

            let response: ResponseValue<UnmatchedCreditTransferActionResponse> = context
                .client()
                .match_unmatched_credit_transfer(&unmatched_credit_transfer_id, &body)
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(MatchUnmatchedCreditTransferExample).await
}
