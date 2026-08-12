// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::create_client_link`.
//!
//! Route: `POST /client-links`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{ClientLinkRequest, ClientLinkResponse};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::create_client_link`.
struct CreateClientLinkExample;

impl RunnableExample for CreateClientLinkExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "create_client_link";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "POST /client-links";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let body: ClientLinkRequest = context
                .options()
                .body(from_value::<ClientLinkRequest>(json!({}))?)?;

            let response: ResponseValue<ClientLinkResponse> =
                context.client().create_client_link(&body).await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(CreateClientLinkExample).await
}
