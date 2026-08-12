// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::get_permission`.
//!
//! Route: `GET /permissions/{permissionId}`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{EntityPermission, PermissionToken};
use mollie_rs::ResponseValue;
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::get_permission`.
struct GetPermissionExample;

impl RunnableExample for GetPermissionExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "get_permission";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /permissions/{permissionId}";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let permission_id: PermissionToken =
                context.options().token("permission_id", "payments.read");

            let response: ResponseValue<EntityPermission> =
                context.client().get_permission(&permission_id).await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(GetPermissionExample).await
}
