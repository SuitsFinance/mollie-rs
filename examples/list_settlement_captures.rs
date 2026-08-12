// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::list_settlement_captures`.
//!
//! Route: `GET /settlements/{settlementId}/captures`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{CaptureToken, ListSettlementCapturesResponse, SettlementToken};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::list_settlement_captures`.
struct ListSettlementCapturesExample;

impl RunnableExample for ListSettlementCapturesExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "list_settlement_captures";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /settlements/{settlementId}/captures";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let settlement_id: SettlementToken = context
                .options()
                .configured("settlement_id", from_value::<SettlementToken>(json!({}))?)?;
            let from: Option<CaptureToken> = context.options().optional_token("from");

            let response: ResponseValue<ListSettlementCapturesResponse> = context
                .client()
                .list_settlement_captures(
                    &settlement_id,
                    Some(context.options().value("embed", "payments")),
                    from.as_ref(),
                    context.options().limit(50),
                )
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(ListSettlementCapturesExample).await
}
