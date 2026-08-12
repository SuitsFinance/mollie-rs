// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::get_profile`.
//!
//! Route: `GET /profiles/{profileId}`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{ProfileResponse, ProfileToken};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::get_profile`.
struct GetProfileExample;

impl RunnableExample for GetProfileExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "get_profile";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /profiles/{profileId}";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let profile_id: ProfileToken = context.options().token("profile_id", "pfl_1234567890");

            let response: ResponseValue<ProfileResponse> =
                context.client().get_profile(&profile_id).await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(GetProfileExample).await
}
