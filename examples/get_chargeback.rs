// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::get_chargeback`.
//!
//! Route: `GET /payments/{paymentId}/chargebacks/{chargebackId}`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{ChargebackToken, EntityChargeback, PaymentToken};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::get_chargeback`.
struct GetChargebackExample;

impl RunnableExample for GetChargebackExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "get_chargeback";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /payments/{paymentId}/chargebacks/{chargebackId}";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let payment_id: PaymentToken = context.options().token("payment_id", "tr_1234567890");
            let chargeback_id: ChargebackToken =
                context.options().token("chargeback_id", "chb_1234567890");

            let response: ResponseValue<EntityChargeback> = context
                .client()
                .get_chargeback(
                    &payment_id,
                    &chargeback_id,
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
    support::run_example(GetChargebackExample).await
}
