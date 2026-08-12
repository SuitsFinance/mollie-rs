// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::get_settlement`.
//!
//! Route: `GET /settlements/{settlementId}`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{EntitySettlement, SettlementToken};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::get_settlement`.
struct GetSettlementExample;

impl RunnableExample for GetSettlementExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "get_settlement";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /settlements/{settlementId}";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let settlement_id: SettlementToken = context
                .options()
                .configured("settlement_id", from_value::<SettlementToken>(json!({}))?)?;

            let response: ResponseValue<EntitySettlement> =
                context.client().get_settlement(&settlement_id).await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(GetSettlementExample).await
}
