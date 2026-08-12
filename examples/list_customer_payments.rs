// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::list_customer_payments`.
//!
//! Route: `GET /customers/{customerId}/payments`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{
    CustomerToken, ListCustomerPaymentsResponse, PaymentToken, ProfileToken, Sorting,
};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::list_customer_payments`.
struct ListCustomerPaymentsExample;

impl RunnableExample for ListCustomerPaymentsExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "list_customer_payments";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /customers/{customerId}/payments";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let customer_id: CustomerToken =
                context.options().token("customer_id", "cst_1234567890");
            let from: Option<PaymentToken> = context.options().optional_token("from");
            let profile_id: Option<ProfileToken> = context.options().optional_token("profile_id");

            let response: ResponseValue<ListCustomerPaymentsResponse> = context
                .client()
                .list_customer_payments(
                    &customer_id,
                    from.as_ref(),
                    context.options().limit(50),
                    profile_id.as_ref(),
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
    support::run_example(ListCustomerPaymentsExample).await
}
