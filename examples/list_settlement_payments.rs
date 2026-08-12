// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::list_settlement_payments`.
//!
//! Route: `GET /settlements/{settlementId}/payments`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{
    ListSettlementPaymentsResponse, PaymentToken, ProfileToken, SettlementToken, Sorting,
};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::list_settlement_payments`.
struct ListSettlementPaymentsExample;

impl RunnableExample for ListSettlementPaymentsExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "list_settlement_payments";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /settlements/{settlementId}/payments";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let settlement_id: SettlementToken = context
                .options()
                .configured("settlement_id", from_value::<SettlementToken>(json!({}))?)?;
            let from: Option<PaymentToken> = context.options().optional_token("from");
            let profile_id: Option<ProfileToken> = context.options().optional_token("profile_id");

            let response: ResponseValue<ListSettlementPaymentsResponse> = context
                .client()
                .list_settlement_payments(
                    &settlement_id,
                    from.as_ref(),
                    context.options().limit(50),
                    profile_id.as_ref(),
                    Some(context.options().configured("sort", Sorting::Desc)?),
                )
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(ListSettlementPaymentsExample).await
}
