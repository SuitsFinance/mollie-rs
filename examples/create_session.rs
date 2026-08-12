// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::create_session`.
//!
//! Route: `POST /sessions`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{SessionRequest, SessionResponse};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::create_session`.
struct CreateSessionExample;

impl RunnableExample for CreateSessionExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "create_session";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "POST /sessions";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let body: SessionRequest = context
                .options()
                .body(from_value::<SessionRequest>(json!({}))?)?;

            let response: ResponseValue<SessionResponse> =
                context.client().create_session(&body).await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(CreateSessionExample).await
}
