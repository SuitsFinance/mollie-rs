// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::create_customer_payment`.
//!
//! Route: `POST /customers/{customerId}/payments`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{CustomerToken, PaymentRequest, PaymentResponse};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::create_customer_payment`.
struct CreateCustomerPaymentExample;

impl RunnableExample for CreateCustomerPaymentExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "create_customer_payment";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "POST /customers/{customerId}/payments";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let customer_id: CustomerToken =
                context.options().token("customer_id", "cst_1234567890");
            let body: PaymentRequest = context
                .options()
                .body(from_value::<PaymentRequest>(json!({}))?)?;

            let response: ResponseValue<PaymentResponse> = context
                .client()
                .create_customer_payment(&customer_id, &body)
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(CreateCustomerPaymentExample).await
}
