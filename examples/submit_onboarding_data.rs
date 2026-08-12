// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::submit_onboarding_data`.
//!
//! Route: `POST /onboarding/me`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::SubmitOnboardingDataBody;
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::submit_onboarding_data`.
struct SubmitOnboardingDataExample;

impl RunnableExample for SubmitOnboardingDataExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "submit_onboarding_data";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "POST /onboarding/me";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let body: SubmitOnboardingDataBody = context
                .options()
                .body(SubmitOnboardingDataBody::default())?;

            let response: ResponseValue<()> =
                context.client().submit_onboarding_data(&body).await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(SubmitOnboardingDataExample).await
}
