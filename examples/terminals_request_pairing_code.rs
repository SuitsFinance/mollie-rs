// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::terminals_request_pairing_code`.
//!
//! Route: `POST /terminals/pairing-codes`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{EntityPairingCode, TerminalsRequestPairingCodeBody};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::terminals_request_pairing_code`.
struct TerminalsRequestPairingCodeExample;

impl RunnableExample for TerminalsRequestPairingCodeExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "terminals_request_pairing_code";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "POST /terminals/pairing-codes";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let body: TerminalsRequestPairingCodeBody =
                context
                    .options()
                    .body(from_value::<TerminalsRequestPairingCodeBody>(json!({}))?)?;

            let response: ResponseValue<EntityPairingCode> = context
                .client()
                .terminals_request_pairing_code(
                    Some(context.options().value("include", "issuers")),
                    &body,
                )
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(TerminalsRequestPairingCodeExample).await
}
