// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::get_refund`.
//!
//! Route: `GET /payments/{paymentId}/refunds/{refundId}`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{EntityRefundResponse, PaymentToken, RefundToken};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::get_refund`.
struct GetRefundExample;

impl RunnableExample for GetRefundExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "get_refund";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /payments/{paymentId}/refunds/{refundId}";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let payment_id: PaymentToken = context.options().token("payment_id", "tr_1234567890");
            let refund_id: RefundToken = context.options().token("refund_id", "re_1234567890");

            let response: ResponseValue<EntityRefundResponse> = context
                .client()
                .get_refund(
                    &payment_id,
                    &refund_id,
                    Some(context.options().value("embed", "payments")),
                )
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(GetRefundExample).await
}
