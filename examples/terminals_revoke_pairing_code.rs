// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::terminals_revoke_pairing_code`.
//!
//! Route: `DELETE /terminals/pairing-codes/{pairingCodeId}`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{EntityPairingCode, TerminalPairingCodeToken};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::terminals_revoke_pairing_code`.
struct TerminalsRevokePairingCodeExample;

impl RunnableExample for TerminalsRevokePairingCodeExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "terminals_revoke_pairing_code";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "DELETE /terminals/pairing-codes/{pairingCodeId}";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let pairing_code_id: TerminalPairingCodeToken = context.options().configured(
                "pairing_code_id",
                from_value::<TerminalPairingCodeToken>(json!({}))?,
            )?;

            let response: ResponseValue<EntityPairingCode> = context
                .client()
                .terminals_revoke_pairing_code(&pairing_code_id)
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(TerminalsRevokePairingCodeExample).await
}
