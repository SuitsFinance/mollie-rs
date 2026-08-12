// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::create_profile`.
//!
//! Route: `POST /profiles`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{ProfileRequest, ProfileResponse};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::create_profile`.
struct CreateProfileExample;

impl RunnableExample for CreateProfileExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "create_profile";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "POST /profiles";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let body: ProfileRequest = context
                .options()
                .body(from_value::<ProfileRequest>(json!({}))?)?;

            let response: ResponseValue<ProfileResponse> =
                context.client().create_profile(&body).await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(CreateProfileExample).await
}
