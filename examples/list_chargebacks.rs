// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::list_chargebacks`.
//!
//! Route: `GET /payments/{paymentId}/chargebacks`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{ChargebackToken, ListChargebacksResponse, PaymentToken};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::list_chargebacks`.
struct ListChargebacksExample;

impl RunnableExample for ListChargebacksExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "list_chargebacks";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /payments/{paymentId}/chargebacks";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let payment_id: PaymentToken = context.options().token("payment_id", "tr_1234567890");
            let from: Option<ChargebackToken> = context.options().optional_token("from");

            let response: ResponseValue<ListChargebacksResponse> = context
                .client()
                .list_chargebacks(
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
    support::run_example(ListChargebacksExample).await
}
