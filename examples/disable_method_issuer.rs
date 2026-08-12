// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::disable_method_issuer`.
//!
//! Route: `DELETE /profiles/{profileId}/methods/{methodId}/issuers/{issuerId}`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{
    DisableMethodIssuerIssuerId, DisableMethodIssuerProfileId, MethodIdWithIssuer, ProfileToken,
};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::disable_method_issuer`.
struct DisableMethodIssuerExample;

impl RunnableExample for DisableMethodIssuerExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "disable_method_issuer";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str =
        "DELETE /profiles/{profileId}/methods/{methodId}/issuers/{issuerId}";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let profile_id: DisableMethodIssuerProfileId = DisableMethodIssuerProfileId::from(
                ProfileToken::try_from(
                    context
                        .options()
                        .value("profile_id", "pfl_1234567890")
                        .to_owned(),
                )
                .expect("valid profile token fixture"),
            );
            let issuer_id: DisableMethodIssuerIssuerId = context.options().configured(
                "issuer_id",
                from_value::<DisableMethodIssuerIssuerId>(json!({}))?,
            )?;

            let response: ResponseValue<()> = context
                .client()
                .disable_method_issuer(
                    &profile_id,
                    context
                        .options()
                        .configured("method_id", MethodIdWithIssuer::Voucher)?,
                    &issuer_id,
                )
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(DisableMethodIssuerExample).await
}
