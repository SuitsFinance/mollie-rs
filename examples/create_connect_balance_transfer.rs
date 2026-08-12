// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::create_connect_balance_transfer`.
//!
//! Route: `POST /connect/balance-transfers`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{EntityBalanceTransfer, EntityBalanceTransferResponse};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::create_connect_balance_transfer`.
struct CreateConnectBalanceTransferExample;

impl RunnableExample for CreateConnectBalanceTransferExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "create_connect_balance_transfer";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "POST /connect/balance-transfers";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let body: EntityBalanceTransfer = context
                .options()
                .body(from_value::<EntityBalanceTransfer>(json!({}))?)?;

            let response: ResponseValue<EntityBalanceTransferResponse> = context
                .client()
                .create_connect_balance_transfer(&body)
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(CreateConnectBalanceTransferExample).await
}
