// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::get_partner_status`.
//!
//! Route: `GET /organizations/me/partner`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::GetPartnerStatusResponse;
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::get_partner_status`.
struct GetPartnerStatusExample;

impl RunnableExample for GetPartnerStatusExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "get_partner_status";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /organizations/me/partner";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let response: ResponseValue<GetPartnerStatusResponse> =
                context.client().get_partner_status().await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(GetPartnerStatusExample).await
}
