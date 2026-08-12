// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::get_transfer`.
//!
//! Route: `GET /business-accounts/transfers/{businessAccountsTransferId}`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{BusinessAccountTransferToken, TransferResponse};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::get_transfer`.
struct GetTransferExample;

impl RunnableExample for GetTransferExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "get_transfer";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /business-accounts/transfers/{businessAccountsTransferId}";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let business_accounts_transfer_id: BusinessAccountTransferToken =
                context.options().configured(
                    "business_accounts_transfer_id",
                    from_value::<BusinessAccountTransferToken>(json!({}))?,
                )?;

            let response: ResponseValue<TransferResponse> = context
                .client()
                .get_transfer(&business_accounts_transfer_id)
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(GetTransferExample).await
}
