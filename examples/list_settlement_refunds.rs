// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::list_settlement_refunds`.
//!
//! Route: `GET /settlements/{settlementId}/refunds`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{ListSettlementRefundsResponse, RefundToken, SettlementToken};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::list_settlement_refunds`.
struct ListSettlementRefundsExample;

impl RunnableExample for ListSettlementRefundsExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "list_settlement_refunds";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /settlements/{settlementId}/refunds";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let settlement_id: SettlementToken = context
                .options()
                .configured("settlement_id", from_value::<SettlementToken>(json!({}))?)?;
            let from: Option<RefundToken> = context.options().optional_token("from");

            let response: ResponseValue<ListSettlementRefundsResponse> = context
                .client()
                .list_settlement_refunds(
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
    support::run_example(ListSettlementRefundsExample).await
}
