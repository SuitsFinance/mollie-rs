// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::verify_payee`.
//!
//! Route: `POST /business-accounts/payee-verifications`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{VerificationOfPayeeRequest, VerificationOfPayeeResponse};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::verify_payee`.
struct VerifyPayeeExample;

impl RunnableExample for VerifyPayeeExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "verify_payee";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "POST /business-accounts/payee-verifications";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let body: VerificationOfPayeeRequest =
                context
                    .options()
                    .body(from_value::<VerificationOfPayeeRequest>(json!({}))?)?;

            let response: ResponseValue<VerificationOfPayeeResponse> =
                context.client().verify_payee(&body).await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(VerifyPayeeExample).await
}
