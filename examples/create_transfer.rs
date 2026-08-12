// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::create_transfer`.
//!
//! Route: `POST /business-accounts/transfers`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{TransferRequest, TransferResponse};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::create_transfer`.
struct CreateTransferExample;

impl RunnableExample for CreateTransferExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "create_transfer";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "POST /business-accounts/transfers";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let body: TransferRequest = context
                .options()
                .body(from_value::<TransferRequest>(json!({}))?)?;

            let response: ResponseValue<TransferResponse> = context
                .client()
                .create_transfer(
                    context.options().value("idempotency_key", "example-id"),
                    context.options().value("x_client_signature", "example-id"),
                    context.options().value("x_client_signed_at", "example-id"),
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
    support::run_example(CreateTransferExample).await
}
