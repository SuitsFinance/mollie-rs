// GENERATED ROUTE EXAMPLE: DO NOT EDIT BY HAND.
//! Runnable example for `Client::list_methods`.
//!
//! Route: `GET /methods`.

#[path = "support/mod.rs"]
mod support;

use mollie_rs::types::{
    Amount, LineCategories, ListMethodsResponse, Locale, MethodIncludeWalletsParameter,
    MethodResourceParameter, ProfileToken, SequenceType,
};
use mollie_rs::ResponseValue;
use serde_json::{from_value, json};
use support::{ExampleContext, ExampleFuture, ExampleResult, RunnableExample};

/// Runnable example for `Client::list_methods`.
struct ListMethodsExample;

impl RunnableExample for ListMethodsExample {
    /// Generated SDK method name demonstrated by this example.
    const NAME: &'static str = "list_methods";

    /// HTTP method and path demonstrated by this example.
    const ROUTE: &'static str = "GET /methods";

    /// Runs this route example with the shared example context.
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a> {
        Box::pin(async move {
            let amount: Amount = context.options().configured(
                "amount",
                from_value::<Amount>(json!({
                    "currency": "EUR",
                    "value": "10.00"
                }))?,
            )?;
            let locale: Locale = context
                .options()
                .configured("locale", from_value::<Locale>(json!({}))?)?;
            let profile_id: Option<ProfileToken> = context.options().optional_token("profile_id");

            let response: ResponseValue<ListMethodsResponse> =
                context
                    .client()
                    .list_methods(
                        Some(&amount),
                        Some(context.options().value("billing_country", "NL")),
                        Some(context.options().value("include", "issuers")),
                        Some(context.options().configured(
                            "include_wallets",
                            MethodIncludeWalletsParameter::Applepay,
                        )?),
                        Some(&locale),
                        Some(
                            context
                                .options()
                                .configured("order_line_categories", LineCategories::Eco)?,
                        ),
                        profile_id.as_ref(),
                        Some(
                            context
                                .options()
                                .configured("resource", MethodResourceParameter::Payments)?,
                        ),
                        Some(
                            context
                                .options()
                                .configured("sequence_type", SequenceType::Oneoff)?,
                        ),
                    )
                    .await?;

            support::print_response(Self::ROUTE, &response);
            Ok(())
        })
    }
}

#[tokio::main]
async fn main() -> ExampleResult<()> {
    support::run_example(ListMethodsExample).await
}
