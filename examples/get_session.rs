// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::get_session`.
//!
//! Route: `GET /sessions/{sessionId}`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{SessionResponse, SessionToken};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::get_session`.
struct GetSessionExample;

impl RunnableExample for GetSessionExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "get_session";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /sessions/{sessionId}";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let session_id: SessionToken = context
                .options()
                .configured("session_id", from_value::<SessionToken>(json!({}))?)?;

            let response: ResponseValue<SessionResponse> =
                context.client().get_session(&session_id).await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(GetSessionExample).await
}
