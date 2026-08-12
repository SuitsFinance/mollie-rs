// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::get_onboarding_status`.
//!
//! Route: `GET /onboarding/me`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::EntityOnboardingStatus;
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::get_onboarding_status`.
struct GetOnboardingStatusExample;

impl RunnableExample for GetOnboardingStatusExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "get_onboarding_status";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /onboarding/me";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let response: ResponseValue<EntityOnboardingStatus> =
                context.client().get_onboarding_status().await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(GetOnboardingStatusExample).await
}
