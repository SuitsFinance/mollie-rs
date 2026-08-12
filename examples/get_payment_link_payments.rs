// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::get_payment_link_payments`.
//!
//! Route: `GET /payment-links/{paymentLinkId}/payments`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{GetPaymentLinkPaymentsResponse, PaymentLinkToken, PaymentToken, Sorting};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::get_payment_link_payments`.
struct GetPaymentLinkPaymentsExample;

impl RunnableExample for GetPaymentLinkPaymentsExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "get_payment_link_payments";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /payment-links/{paymentLinkId}/payments";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let payment_link_id: PaymentLinkToken =
                context.options().token("payment_link_id", "pl_1234567890");
            let from: Option<PaymentToken> = context.options().optional_token("from");

            let response: ResponseValue<GetPaymentLinkPaymentsResponse> = context
                .client()
                .get_payment_link_payments(
                    &payment_link_id,
                    from.as_ref(),
                    context.options().limit(50),
                    Some(context.options().configured("sort", Sorting::Desc)?),
                )
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(GetPaymentLinkPaymentsExample).await
}
