// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::delete_profile`.
//!
//! Route: `DELETE /profiles/{profileId}`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::ProfileToken;
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::delete_profile`.
struct DeleteProfileExample;

impl RunnableExample for DeleteProfileExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "delete_profile";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "DELETE /profiles/{profileId}";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let profile_id: ProfileToken = context.options().token("profile_id", "pfl_1234567890");

            let response: ResponseValue<()> = context.client().delete_profile(&profile_id).await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(DeleteProfileExample).await
}
