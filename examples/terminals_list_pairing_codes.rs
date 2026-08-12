// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::terminals_list_pairing_codes`.
//!
//! Route: `GET /terminals/pairing-codes`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{Sorting, TerminalsListPairingCodesResponse};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::terminals_list_pairing_codes`.
struct TerminalsListPairingCodesExample;

impl RunnableExample for TerminalsListPairingCodesExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "terminals_list_pairing_codes";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /terminals/pairing-codes";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let response: ResponseValue<TerminalsListPairingCodesResponse> = context
                .client()
                .terminals_list_pairing_codes(
                    context.options().optional_value("from"),
                    context.options().limit(50),
                    context.options().optional_value("profile_id"),
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
    support::run_example(TerminalsListPairingCodesExample).await
}
