// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::get_current_organization`.
//!
//! Route: `GET /organizations/me`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::EntityOrganization;
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::get_current_organization`.
struct GetCurrentOrganizationExample;

impl RunnableExample for GetCurrentOrganizationExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "get_current_organization";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /organizations/me";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let response: ResponseValue<EntityOrganization> =
                context.client().get_current_organization().await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(GetCurrentOrganizationExample).await
}
