// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::get_connect_balance_transfer`.
//!
//! Route: `GET /connect/balance-transfers/{balanceTransferId}`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{ConnectBalanceTransferToken, EntityBalanceTransferResponse};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::get_connect_balance_transfer`.
struct GetConnectBalanceTransferExample;

impl RunnableExample for GetConnectBalanceTransferExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "get_connect_balance_transfer";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /connect/balance-transfers/{balanceTransferId}";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let balance_transfer_id: ConnectBalanceTransferToken = context.options().configured(
                "balance_transfer_id",
                from_value::<ConnectBalanceTransferToken>(json!({}))?,
            )?;

            let response: ResponseValue<EntityBalanceTransferResponse> = context
                .client()
                .get_connect_balance_transfer(&balance_transfer_id)
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(GetConnectBalanceTransferExample).await
}
