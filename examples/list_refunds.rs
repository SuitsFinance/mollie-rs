// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::list_refunds`.
//!
//! Route: `GET /payments/{paymentId}/refunds`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{ListRefundsResponse, PaymentToken, RefundToken};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::list_refunds`.
struct ListRefundsExample;

impl RunnableExample for ListRefundsExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "list_refunds";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /payments/{paymentId}/refunds";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let payment_id: PaymentToken = context.options().token("payment_id", "tr_1234567890");
            let from: Option<RefundToken> = context.options().optional_token("from");

            let response: ResponseValue<ListRefundsResponse> = context
                .client()
                .list_refunds(
                    &payment_id,
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
    support::run_example(ListRefundsExample).await
}
