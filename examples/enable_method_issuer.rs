// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::enable_method_issuer`.
//!
//! Route: `POST /profiles/{profileId}/methods/{methodId}/issuers/{issuerId}`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{
    EnableMethodIssuerBody, EnableMethodIssuerIssuerId, EnableMethodIssuerProfileId,
    EnableMethodIssuerResponse, MethodIdWithIssuer, ProfileToken,
};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::enable_method_issuer`.
struct EnableMethodIssuerExample;

impl RunnableExample for EnableMethodIssuerExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "enable_method_issuer";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "POST /profiles/{profileId}/methods/{methodId}/issuers/{issuerId}";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let profile_id: EnableMethodIssuerProfileId = EnableMethodIssuerProfileId::from(
                ProfileToken::try_from(
                    context
                        .options()
                        .value("profile_id", "pfl_1234567890")
                        .to_owned(),
                )
                .expect("valid profile token fixture"),
            );
            let issuer_id: EnableMethodIssuerIssuerId = context.options().configured(
                "issuer_id",
                from_value::<EnableMethodIssuerIssuerId>(json!({}))?,
            )?;
            let body: EnableMethodIssuerBody =
                context.options().body(EnableMethodIssuerBody::default())?;

            let response: ResponseValue<EnableMethodIssuerResponse> = context
                .client()
                .enable_method_issuer(
                    &profile_id,
                    context
                        .options()
                        .configured("method_id", MethodIdWithIssuer::Voucher)?,
                    &issuer_id,
                    &body,
                )
                .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(EnableMethodIssuerExample).await
}
