// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::update_profile`.
//!
//! Route: `PATCH /profiles/{profileId}`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{ProfileResponse, ProfileToken, UpdateProfileBody};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::update_profile`.
struct UpdateProfileExample;

impl RunnableExample for UpdateProfileExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "update_profile";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "PATCH /profiles/{profileId}";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let profile_id: ProfileToken = context.options().token("profile_id", "pfl_1234567890");
            let body: UpdateProfileBody = context.options().body(UpdateProfileBody::default())?;

            let response: ResponseValue<ProfileResponse> =
                context.client().update_profile(&profile_id, &body).await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(UpdateProfileExample).await
}
