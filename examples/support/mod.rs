//! Shared runtime helpers for Mollie route examples.
//!
//! The generated examples intentionally keep route-specific code in each
//! `examples/<method>.rs` file and centralize environment handling here.
//!
//! Successful and failed responses are also appended to
//! `logs/<example_name>.log` under the crate root so you can inspect what
//! each token/profile/credential actually returned across runs.
//!
//! After every log write, this module rebuilds
//! [`docs/example-support-matrix.md`](../../docs/example-support-matrix.md)
//! from the **latest** entry in each `logs/*.log` file (plus every discovered
//! example binary as `untested` when no log exists yet).

use clap::Parser;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use std::{
    cell::RefCell,
    collections::BTreeMap,
    error::Error,
    ffi::OsString,
    fs::{self, OpenOptions},
    future::Future,
    io::{self, Write},
    num::NonZeroU64,
    path::{Path, PathBuf},
    pin::Pin,
};
use tracing::{error, info, warn};

use mollie_rs::prelude::MollieErrorEnvelope;
use mollie_rs::{
    try_init_tracing, types, Error as ProgenitorError, MollieClient, MollieError,
    MollieSuccessCatalogEntry, ResponseEnvelope, ResponseValue, MOLLIE_API_KEY_ENV,
    MOLLIE_OAUTH_ACCESS_TOKEN_ENV,
};

/// Result type used by runnable examples.
pub type ExampleResult<T> = Result<T, Box<dyn Error + Send + Sync>>;
/// Boxed future returned by [`RunnableExample`] implementations.
pub type ExampleFuture<'a> = Pin<Box<dyn Future<Output = ExampleResult<()>> + 'a>>;

const ACCESS_TOKEN_PROFILE_RESTRICTED_KEY: &str = "ACCESS_TOKEN_PROFILE_RESTRICTED";
const ACCESS_TOKEN_PROFILE_RESTRICTED_LABEL: &str = "access-token-not-profile-restricted";

/// Shared environment and CLI overrides for every route example.
///
/// CLI values take precedence over environment variables. The fields are
/// deliberately shared across examples so the same `PAYMENT_ID` or
/// `--payment-id` works for every route that needs it.
#[derive(Debug, Clone, Parser)]
#[command(
    version,
    about = "Run a Mollie route example with env or CLI fixture overrides"
)]
pub struct ExampleOptions {
    #[arg(long, env = "AMOUNT", value_name = "JSON")]
    pub amount: Option<String>,
    #[arg(long, env = "BALANCE_ID")]
    pub balance_id: Option<String>,
    #[arg(long, env = "BILLING_COUNTRY")]
    pub billing_country: Option<String>,
    #[arg(long, env = "CAPTURE_ID")]
    pub capture_id: Option<String>,
    #[arg(long, env = "CHARGEBACK_ID")]
    pub chargeback_id: Option<String>,
    #[arg(long, env = "CURRENCY")]
    pub currency: Option<String>,
    #[arg(long, env = "CUSTOMER_ID")]
    pub customer_id: Option<String>,
    #[arg(long, env = "DESCRIPTION")]
    pub description: Option<String>,
    #[arg(long, env = "EMAIL")]
    pub email: Option<String>,
    #[arg(long, env = "EMBED")]
    pub embed: Option<String>,
    #[arg(long, env = "EVENT_TYPES", value_name = "VALUE_OR_JSON")]
    pub event_types: Option<String>,
    #[arg(long, env = "FROM")]
    pub from: Option<String>,
    #[arg(long, env = "GROUPING")]
    pub grouping: Option<String>,
    #[arg(long, env = "ID")]
    pub id: Option<String>,
    #[arg(long, env = "INCLUDE")]
    pub include: Option<String>,
    #[arg(long, env = "INCLUDE_WALLETS")]
    pub include_wallets: Option<String>,
    #[arg(long, env = "INVOICE_MONTH")]
    pub invoice_month: Option<String>,
    #[arg(long, env = "LIMIT")]
    pub limit: Option<NonZeroU64>,
    #[arg(long, env = "LOCALE")]
    pub locale: Option<String>,
    #[arg(long, env = "MANDATE_ID")]
    pub mandate_id: Option<String>,
    #[arg(long, env = "METHOD_ID")]
    pub method_id: Option<String>,
    #[arg(long, env = "MONTH")]
    pub month: Option<String>,
    #[arg(long, env = "NAME")]
    pub name: Option<String>,
    #[arg(long, env = "ORDER_LINE_CATEGORIES", value_name = "VALUE_OR_JSON")]
    pub order_line_categories: Option<String>,
    #[arg(long, env = "PAYMENT_ID")]
    pub payment_id: Option<String>,
    #[arg(long, env = "PAYMENT_LINK_ID")]
    pub payment_link_id: Option<String>,
    #[arg(long, env = "PERMISSION_ID")]
    pub permission_id: Option<String>,
    #[arg(long, env = "PHONE")]
    pub phone: Option<String>,
    #[arg(long, env = "PROFILE_ID")]
    pub profile_id: Option<String>,
    #[arg(long, env = "REFUND_ID")]
    pub refund_id: Option<String>,
    #[arg(long, env = "REFERENCE")]
    pub reference: Option<String>,
    #[arg(long, env = "RESOURCE")]
    pub resource: Option<String>,
    #[arg(long, env = "SEQUENCE_TYPE")]
    pub sequence_type: Option<String>,
    #[arg(long, env = "SETTLEMENT_ID")]
    pub settlement_id: Option<String>,
    #[arg(long, env = "SORT")]
    pub sort: Option<String>,
    #[arg(long, env = "SUBSCRIPTION_ID", alias = "subcription-id")]
    pub subscription_id: Option<String>,
    #[arg(long, env = "TERMINAL_ID")]
    pub terminal_id: Option<String>,
    #[arg(long, env = "UNTIL")]
    pub until: Option<String>,
    #[arg(long, env = "WEBSITE")]
    pub website: Option<String>,
    #[arg(long, env = "YEAR")]
    pub year: Option<String>,
    #[arg(long, env = "MOLLIE_TESTMODE")]
    pub testmode: Option<bool>,
    #[arg(long, env = "EXAMPLE_BODY_JSON", value_name = "JSON")]
    pub body_json: Option<String>,
    #[arg(long, env = "EXAMPLE_BODY_FILE", value_name = "FILE")]
    pub body_file: Option<PathBuf>,
    #[arg(skip)]
    dynamic_values: BTreeMap<String, String>,
    #[arg(skip)]
    example_name: Option<String>,
}

#[allow(dead_code)]
impl ExampleOptions {
    /// Parses the documented Clap options and also accepts arbitrary
    /// `--name value` fixture overrides for route arguments and request-body
    /// fields. Explicit CLI overrides take precedence over environment values.
    pub fn parse() -> Self {
        let (args, mut dynamic_values) = split_dynamic_args(std::env::args_os());
        let mut options = <Self as Parser>::parse_from(args);

        for (key, value) in std::env::vars() {
            let normalized = normalize_override_key(&key);
            let is_known_option = known_long_options()
                .iter()
                .any(|option| normalize_override_key(option) == normalized);
            let is_known_special =
                matches!(key.as_str(), "EXAMPLE_BODY_JSON" | "EXAMPLE_BODY_FILE");
            if is_dynamic_environment_key(&key) && !is_known_option && !is_known_special {
                dynamic_values.entry(normalized).or_insert(value);
            }
        }

        options.dynamic_values = dynamic_values;
        options
    }

    pub fn for_example(mut self, example_name: &'static str) -> Self {
        self.example_name = Some(example_name.to_owned());
        self
    }

    fn raw(&self, name: &str) -> Option<&str> {
        let value = match name {
            "amount" => self.amount.as_deref(),
            "balance_id" => self.balance_id.as_deref(),
            "billing_country" => self.billing_country.as_deref(),
            "capture_id" => self.capture_id.as_deref(),
            "chargeback_id" => self.chargeback_id.as_deref(),
            "currency" => self.currency.as_deref(),
            "customer_id" => self.customer_id.as_deref(),
            "description" => self.description.as_deref(),
            "email" => self.email.as_deref(),
            "embed" => self.embed.as_deref(),
            "event_types" => self.event_types.as_deref(),
            "from" => self.from.as_deref(),
            "grouping" => self.grouping.as_deref(),
            "id" => self.id.as_deref(),
            "include" => self.include.as_deref(),
            "include_wallets" => self.include_wallets.as_deref(),
            "invoice_month" => self.invoice_month.as_deref().or(self.month.as_deref()),
            "locale" => self.locale.as_deref(),
            "mandate_id" => self.mandate_id.as_deref(),
            "method_id" => self.method_id.as_deref(),
            "month" => self.month.as_deref(),
            "name" => self.name.as_deref(),
            "order_line_categories" => self.order_line_categories.as_deref(),
            "payment_id" => self.payment_id.as_deref(),
            "payment_link_id" => self.payment_link_id.as_deref(),
            "permission_id" => self.permission_id.as_deref(),
            "phone" => self.phone.as_deref(),
            "profile_id" => self.profile_id.as_deref(),
            "refund_id" => self.refund_id.as_deref(),
            "reference" => self.reference.as_deref(),
            "resource" => self.resource.as_deref(),
            "sequence_type" => self.sequence_type.as_deref(),
            "settlement_id" => self.settlement_id.as_deref(),
            "sort" => self.sort.as_deref(),
            "subscription_id" => self.subscription_id.as_deref(),
            "terminal_id" => self.terminal_id.as_deref(),
            "until" => self.until.as_deref(),
            "website" => self.website.as_deref(),
            "year" => self.year.as_deref(),
            _ => None,
        };
        if let Some(value) = value.filter(|value| !value.is_empty()) {
            return Some(value);
        }

        for alias in self.override_aliases(name) {
            let known_value = match alias {
                "balance_id" => self.balance_id.as_deref(),
                "profile_id" => self.profile_id.as_deref(),
                "settlement_id" => self.settlement_id.as_deref(),
                _ => None,
            };
            if let Some(value) =
                known_value.or_else(|| self.dynamic_values.get(alias).map(String::as_str))
            {
                if !value.is_empty() {
                    return Some(value);
                }
            }
        }

        self.dynamic_values
            .get(name)
            .map(String::as_str)
            .filter(|value| !value.is_empty())
    }

    fn override_aliases(&self, name: &str) -> Vec<&str> {
        if name != "id" {
            return Vec::new();
        }

        let example = self.example_name.as_deref().unwrap_or_default();
        if example.contains("sales_invoice") {
            vec!["sales_invoice_id", "invoice_id"]
        } else if example.contains("invoice") {
            vec!["invoice_id"]
        } else if example.contains("profile") {
            vec!["profile_id"]
        } else if example.contains("webhook") {
            vec!["webhook_id"]
        } else if example.contains("organization") {
            vec!["organization_id"]
        } else if example.contains("client") {
            vec!["client_id"]
        } else if example.contains("settlement") {
            vec!["settlement_id"]
        } else if example.contains("balance") {
            vec!["balance_id"]
        } else {
            Vec::new()
        }
    }

    /// Returns a configured string or the generated route fixture.
    pub fn value<'a>(&'a self, name: &str, default: &'a str) -> &'a str {
        self.raw(name).unwrap_or(default)
    }

    /// Returns an optional configured string, preserving the generated `None`
    /// default for cursors and profile filters.
    pub fn optional_value(&self, name: &str) -> Option<&str> {
        self.raw(name)
    }

    /// Returns a configured token-like generated type.
    pub fn token<T>(&self, name: &str, default: &str) -> T
    where
        T: TryFrom<String>,
        T::Error: std::fmt::Display,
    {
        let value = self.value(name, default).to_owned();
        T::try_from(value)
            .unwrap_or_else(|error| panic!("invalid {name} value for generated token: {error}"))
    }

    /// Returns an optional configured token-like generated type.
    pub fn optional_token<T>(&self, name: &str) -> Option<T>
    where
        T: TryFrom<String>,
        T::Error: std::fmt::Display,
    {
        self.optional_value(name).map(str::to_owned).map(|value| {
            T::try_from(value)
                .unwrap_or_else(|error| panic!("invalid {name} value for generated token: {error}"))
        })
    }

    /// Returns the configured limit or the generated page-size fixture.
    pub fn limit(&self, default: u64) -> Option<NonZeroU64> {
        self.limit.or_else(|| NonZeroU64::new(default))
    }

    /// Returns a configured boolean route argument.
    pub fn bool_value(&self, name: &str, default: bool) -> bool {
        self.raw(name)
            .map(|value| value.parse::<bool>())
            .transpose()
            .ok()
            .flatten()
            .unwrap_or(default)
    }

    /// Parses a configured scalar, enum, wrapper, or JSON fixture.
    ///
    /// Plain CLI values such as `--sort desc` are treated as JSON strings;
    /// object/array values such as `--amount '{"currency":"EUR",...}'`
    /// are parsed as JSON directly.
    pub fn configured<T>(&self, name: &str, default: T) -> ExampleResult<T>
    where
        T: DeserializeOwned,
    {
        let Some(raw) = self.raw(name) else {
            return Ok(default);
        };
        parse_configured(raw)
    }

    /// Parses an optional configured JSON value.
    pub fn optional_configured<T>(&self, name: &str) -> ExampleResult<Option<T>>
    where
        T: DeserializeOwned,
    {
        self.raw(name).map(parse_configured).transpose()
    }

    /// Replaces a generated request body with `EXAMPLE_BODY_JSON` /
    /// `--body-json`, or loads it from `EXAMPLE_BODY_FILE` / `--body-file`.
    ///
    /// First-class body fields such as `--email`, `--name`, `--description`,
    /// `--phone`, and `--website` (plus matching `EMAIL` / `NAME` / … env vars)
    /// are applied on top of the base body, as are arbitrary dynamic
    /// `--field value` overrides.
    pub fn body<T>(&self, default: T) -> ExampleResult<T>
    where
        T: DeserializeOwned + serde::Serialize,
    {
        let mut value = if let Some(path) = &self.body_file {
            let raw = fs::read_to_string(path)?;
            parse_configured_value(&raw)?
        } else if let Some(raw) = self.body_json.as_deref() {
            parse_configured_value(raw)?
        } else {
            serde_json::to_value(default)?
        };

        apply_body_overrides(&mut value, &self.body_override_values())?;
        serde_json::from_value(value).map_err(Into::into)
    }

    /// Shared body-field overrides: first-class clap/env values plus dynamic
    /// `--field` flags. Explicit first-class values win over dynamic ones.
    fn body_override_values(&self) -> BTreeMap<String, String> {
        let mut overrides = self.dynamic_values.clone();
        for key in [
            "amount",
            "currency",
            "description",
            "email",
            "locale",
            "name",
            "phone",
            "reference",
            "sequence_type",
            "website",
        ] {
            if let Some(value) = self.raw(key) {
                overrides.insert(key.to_owned(), value.to_owned());
            }
        }
        overrides
    }
}

fn normalize_override_key(key: &str) -> String {
    key.trim_start_matches("--")
        .replace('-', "_")
        .to_ascii_lowercase()
}

fn is_dynamic_environment_key(key: &str) -> bool {
    key.chars().all(|character| {
        character.is_ascii_uppercase() || character.is_ascii_digit() || character == '_'
    }) && !key.starts_with("MOLLIE_")
        && !key.starts_with("CARGO_")
        && !key.starts_with("RUST_")
        && !matches!(key, "PATH" | "HOME" | "PWD" | "USER" | "SHELL")
}

fn known_long_options() -> &'static [&'static str] {
    &[
        "amount",
        "balance-id",
        "billing-country",
        "capture-id",
        "chargeback-id",
        "currency",
        "customer-id",
        "description",
        "email",
        "embed",
        "event-types",
        "from",
        "grouping",
        "id",
        "include",
        "include-wallets",
        "invoice-month",
        "limit",
        "locale",
        "mandate-id",
        "method-id",
        "month",
        "name",
        "order-line-categories",
        "payment-id",
        "payment-link-id",
        "permission-id",
        "phone",
        "profile-id",
        "refund-id",
        "reference",
        "resource",
        "sequence-type",
        "settlement-id",
        "sort",
        "subscription-id",
        "subcription-id",
        "terminal-id",
        "until",
        "website",
        "year",
        "testmode",
        "body-json",
        "body-file",
        "help",
        "version",
    ]
}

fn split_dynamic_args<I>(args: I) -> (Vec<OsString>, BTreeMap<String, String>)
where
    I: IntoIterator<Item = OsString>,
{
    let mut args = args.into_iter();
    let mut filtered = Vec::new();
    let mut dynamic = BTreeMap::new();
    let mut after_separator = false;

    if let Some(program) = args.next() {
        filtered.push(program);
    }

    let known = known_long_options();
    let remaining: Vec<OsString> = args.collect();
    let mut index = 0usize;
    while index < remaining.len() {
        let argument = &remaining[index];
        let Some(argument_text) = argument.to_str() else {
            filtered.push(argument.clone());
            index += 1;
            continue;
        };

        if after_separator || argument_text == "--" || !argument_text.starts_with("--") {
            after_separator |= argument_text == "--";
            filtered.push(argument.clone());
            index += 1;
            continue;
        }

        let without_prefix = &argument_text[2..];
        let (raw_key, inline_value) = without_prefix
            .split_once('=')
            .map_or((without_prefix, None), |(key, value)| (key, Some(value)));
        if known.contains(&raw_key) {
            filtered.push(argument.clone());
            index += 1;
            continue;
        }

        let value = if let Some(value) = inline_value {
            value.to_owned()
        } else if remaining
            .get(index + 1)
            .and_then(|value| value.to_str())
            .is_some_and(|next| !next.starts_with("--"))
        {
            index += 1;
            remaining[index].to_string_lossy().into_owned()
        } else {
            "true".to_owned()
        };
        dynamic.insert(normalize_override_key(raw_key), value);
        index += 1;
    }

    (filtered, dynamic)
}

#[allow(dead_code)]
fn parse_configured_value(raw: &str) -> ExampleResult<serde_json::Value> {
    match serde_json::from_str(raw) {
        Ok(value) => Ok(value),
        Err(_) => Ok(serde_json::Value::String(raw.to_owned())),
    }
}

fn parse_configured<T>(raw: &str) -> ExampleResult<T>
where
    T: DeserializeOwned,
{
    let value = parse_configured_value(raw)?;
    serde_json::from_value(value).map_err(Into::into)
}

fn apply_body_overrides(
    value: &mut serde_json::Value,
    overrides: &BTreeMap<String, String>,
) -> ExampleResult<()> {
    for (key, raw) in overrides {
        let path = body_override_path(key);
        if path.is_empty() {
            continue;
        }
        let override_value = if raw.starts_with('{') || raw.starts_with('[') {
            parse_configured_value(raw)?
        } else {
            serde_json::Value::String(raw.clone())
        };
        set_json_path(value, &path, override_value);
    }
    Ok(())
}

fn body_override_path(key: &str) -> Vec<String> {
    let nested_roots = [
        "amount",
        "billing_address",
        "details",
        "payment_details",
        "recipient",
        "organization",
        "profile",
    ];
    for root in nested_roots {
        let prefix = format!("{root}_");
        if let Some(child) = key.strip_prefix(&prefix) {
            let child = if root == "amount" && child == "current" {
                "currency"
            } else {
                child
            };
            return vec![to_camel_case(root), to_camel_case(child)];
        }
    }
    vec![to_camel_case(key)]
}

fn to_camel_case(value: &str) -> String {
    let mut parts = value.split('_');
    let Some(first) = parts.next() else {
        return String::new();
    };
    let mut output = first.to_owned();
    for part in parts {
        let mut chars = part.chars();
        if let Some(first) = chars.next() {
            output.extend(first.to_uppercase());
            output.extend(chars);
        }
    }
    output
}

fn set_json_path(value: &mut serde_json::Value, path: &[String], replacement: serde_json::Value) {
    if path.len() == 1 {
        if !value.is_object() {
            *value = serde_json::Value::Object(serde_json::Map::new());
        }
        if let Some(object) = value.as_object_mut() {
            object.insert(path[0].clone(), replacement);
        }
        return;
    }

    if !value.is_object() {
        *value = serde_json::Value::Object(serde_json::Map::new());
    }
    if let Some(object) = value.as_object_mut() {
        let child = object
            .entry(path[0].clone())
            .or_insert_with(|| serde_json::Value::Object(serde_json::Map::new()));
        set_json_path(child, &path[1..], replacement);
    }
}

// Name of the example currently running (set by `run_example`).
// Used so `print_response` / error handling can append to
// `logs/<example_name>.log` without changing every generated call site.
thread_local! {
    static CURRENT_EXAMPLE: RefCell<Option<&'static str>> = const { RefCell::new(None) };
}

/// Shared state available to every route example.
#[allow(dead_code)]
pub struct ExampleContext {
    client: MollieClient,
    options: ExampleOptions,
}

impl ExampleContext {
    /// Creates a context from an already configured client.
    pub const fn new(client: MollieClient, options: ExampleOptions) -> Self {
        Self { client, options }
    }

    /// Returns the SDK client used by the example.
    pub const fn client(&self) -> &MollieClient {
        &self.client
    }

    /// Returns the env/CLI values used by this example.
    #[allow(dead_code)]
    pub const fn options(&self) -> &ExampleOptions {
        &self.options
    }
}

/// A runnable route example.
pub trait RunnableExample {
    /// Generated SDK method name demonstrated by the example.
    const NAME: &'static str;

    /// HTTP verb and path demonstrated by the example.
    const ROUTE: &'static str;

    /// Runs the example against the configured [`ExampleContext`].
    fn run<'a>(&'a self, context: &'a ExampleContext) -> ExampleFuture<'a>;
}

/// Builds a client from env / `.env` and runs an example.
///
/// Installs tracing via [`mollie_rs::try_init_tracing`], then builds
/// [`MollieClient::from_env`](mollie_rs::MollieClient::from_env) (which loads
/// dotenv internally).
///
/// Missing credentials are treated as a clean skip so `cargo run --example`
/// stays safe in local checkouts.
///
/// Each success/error response body is also appended to
/// `logs/<example_name>.log` (see [`append_example_log`]).
pub async fn run_example<E: RunnableExample>(example: E) -> ExampleResult<()> {
    let _ = try_init_tracing();
    let _ = dotenvy::dotenv();
    let options = ExampleOptions::parse().for_example(E::NAME);
    CURRENT_EXAMPLE.with(|slot| {
        *slot.borrow_mut() = Some(E::NAME);
    });

    let client: MollieClient = match MollieClient::from_env() {
        Ok(client) => client,
        Err(error) if error.is_missing_mollie_credentials() => {
            error!(
                example = E::NAME,
                route = E::ROUTE,
                api_key_env = MOLLIE_API_KEY_ENV,
                oauth_env = MOLLIE_OAUTH_ACCESS_TOKEN_ENV,
                "missing Mollie credentials; set env vars or a .env file to run this example against Mollie"
            );
            append_example_log(
                E::NAME,
                &format!(
                    "SKIP missing credentials (set {MOLLIE_API_KEY_ENV} or {MOLLIE_OAUTH_ACCESS_TOKEN_ENV})\nroute: {}\n",
                    E::ROUTE
                ),
            );
            return Ok(());
        }
        Err(error) => {
            error!(
                example = E::NAME,
                route = E::ROUTE,
                error = %error,
                "failed to build Mollie client from environment"
            );
            append_example_log(
                E::NAME,
                &format!(
                    "ERROR failed to build client\nroute: {}\nerror: {error}\n",
                    E::ROUTE
                ),
            );
            return Err(Box::new(error));
        }
    };

    let client = match options.testmode {
        Some(testmode) => client.with_testmode(testmode),
        None => client,
    };

    info!(
        example = E::NAME,
        route = E::ROUTE,
        "running Mollie route example"
    );

    let context: ExampleContext = ExampleContext::new(client, options);
    match example.run(&context).await {
        Ok(()) => {
            info!(
                example = E::NAME,
                route = E::ROUTE,
                "Mollie route example completed successfully"
            );
            Ok(())
        }
        Err(error) => {
            // Prefer crate-owned MollieError so catalog + JSON envelope logging works
            // for generated route errors (`progenitor_client::Error<ErrorResponse>`).
            let error: Box<dyn Error + Send + Sync> =
                match error.downcast::<ProgenitorError<types::ErrorResponse>>() {
                    Ok(route_error) => {
                        Box::new(MollieError::from(*route_error)) as Box<dyn Error + Send + Sync>
                    }
                    Err(other) => other,
                };

            if let Some(mollie) = error.downcast_ref::<MollieError>() {
                let envelope: MollieErrorEnvelope = mollie.to_envelope();
                error!(
                    example = E::NAME,
                    route = E::ROUTE,
                    error = %mollie,
                    status = envelope.status,
                    code = envelope.code,
                    key = %envelope.key,
                    message_key = envelope.message_key,
                    detail = %envelope.detail,
                    "Mollie route example failed"
                );
                // Pretty-print envelope + one-line summary on the *same* stream
                // so Windows consoles do not interleave stdout/stderr.
                // Exit so `main` does not Debug-dump headers/body.
                let summary = format!(
                    "{}: {} (code {}, key {})",
                    envelope.title.as_deref().unwrap_or("Mollie API error"),
                    envelope.detail,
                    envelope.code,
                    envelope.key,
                );
                match serde_json::to_string_pretty(&envelope) {
                    Ok(pretty) => {
                        println!("{pretty}\n{summary}");
                        let _ = io::stdout().flush();
                        append_example_log(
                            E::NAME,
                            &format!(
                                "ERROR response\nroute: {}\nstatus: {:?}\ncode: {}\nkey: {}\nmessage_key: {:?}\nsummary: {summary}\nbody:\n{pretty}\n",
                                E::ROUTE,
                                envelope.status,
                                envelope.code,
                                envelope.key,
                                envelope.message_key,
                            ),
                        );
                    }
                    Err(serialize_error) => {
                        error!(%serialize_error, "failed to serialize error envelope as pretty JSON");
                        println!("{summary}");
                        let _ = io::stdout().flush();
                        append_example_log(
                            E::NAME,
                            &format!(
                                "ERROR response (serialize failed: {serialize_error})\nroute: {}\nsummary: {summary}\n",
                                E::ROUTE
                            ),
                        );
                    }
                }
                std::process::exit(1);
            }

            error!(
                example = E::NAME,
                route = E::ROUTE,
                error = %error,
                "Mollie route example failed"
            );
            append_example_log(
                E::NAME,
                &format!(
                    "ERROR non-Mollie failure\nroute: {}\nerror: {error}\n",
                    E::ROUTE
                ),
            );
            Err(error)
        }
    }
}

/// Logs the response status and pretty-prints the JSON body for a generated
/// route response, and appends the same payload to `logs/<example>.log`.
pub fn print_response<T>(route: &str, response: &ResponseValue<T>)
where
    T: serde::Serialize,
{
    let status: StatusCode = response.status();
    let catalog: MollieSuccessCatalogEntry =
        MollieSuccessCatalogEntry::from_status(status.as_u16());
    info!(
        %route,
        status = %status,
        code = catalog.code(),
        key = catalog.key().as_str(),
        message_key = catalog.message_key(),
        "Mollie response"
    );
    let body = print_pretty_json(response.as_ref());
    if let Some(pretty) = body {
        let example = current_example_name().unwrap_or("unknown");
        append_example_log(
            example,
            &format!(
                "OK response\nroute: {route}\nstatus: {status}\ncode: {}\nkey: {}\nmessage_key: {}\nbody:\n{pretty}\n",
                catalog.code(),
                catalog.key().as_str(),
                catalog.message_key(),
            ),
        );
    }
}

/// Logs a crate-owned response envelope using the success catalog and
/// pretty-prints the JSON body, appending to `logs/<example>.log`.
#[allow(dead_code)] // Available to hand-written / non-generated examples.
pub fn print_envelope<T>(route: &str, envelope: &ResponseEnvelope<T>)
where
    T: serde::Serialize,
{
    let success: MollieSuccessCatalogEntry = envelope.success_catalog();
    info!(
        %route,
        status = %envelope.status(),
        code = success.code(),
        key = success.key().as_str(),
        message_key = success.message_key(),
        "Mollie response envelope"
    );
    let body = print_pretty_json(envelope.data());
    if let Some(pretty) = body {
        let example = current_example_name().unwrap_or("unknown");
        append_example_log(
            example,
            &format!(
                "OK envelope\nroute: {route}\nstatus: {}\ncode: {}\nkey: {}\nmessage_key: {}\nbody:\n{pretty}\n",
                envelope.status(),
                success.code(),
                success.key().as_str(),
                success.message_key(),
            ),
        );
    }
}

/// Serializes `value` as indented JSON and prints it to stdout.
///
/// Returns the pretty JSON string on success so callers can also write it to
/// the per-example log file.
fn print_pretty_json<T>(value: &T) -> Option<String>
where
    T: serde::Serialize,
{
    match serde_json::to_string_pretty(value) {
        Ok(pretty) => {
            // Print the JSON text itself — `{:#?}` would escape newlines into one line.
            println!("{pretty}");
            let _ = io::stdout().flush();
            Some(pretty)
        }
        Err(error) => {
            error!(%error, "failed to serialize response body as pretty JSON");
            None
        }
    }
}

fn current_example_name() -> Option<&'static str> {
    CURRENT_EXAMPLE.with(|slot| *slot.borrow())
}

/// Directory for per-example response logs (`<crate root>/logs`).
fn example_logs_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("logs")
}

/// Path of the append-only log file for `example_name` (`logs/<name>.log`).
fn example_log_path(example_name: &str) -> PathBuf {
    example_logs_dir().join(format!("{example_name}.log"))
}

/// Appends a timestamped entry to `logs/<example_name>.log`, creating
/// `logs/` if needed. Failures are warned and never fail the example.
///
/// After a successful append, rebuilds
/// [`docs/example-support-matrix.md`](../../docs/example-support-matrix.md)
/// from every `logs/*.log` file so support status stays current.
fn append_example_log(example_name: &str, body: &str) {
    let dir = example_logs_dir();
    if let Err(error) = fs::create_dir_all(&dir) {
        warn!(%error, path = %dir.display(), "failed to create example logs directory");
        return;
    }

    let path = example_log_path(example_name);
    let timestamp = log_timestamp();
    let entry = format!("========== {timestamp} ==========\nexample: {example_name}\n{body}\n");

    match OpenOptions::new().create(true).append(true).open(&path) {
        Ok(mut file) => {
            if let Err(error) = file.write_all(entry.as_bytes()) {
                warn!(%error, path = %path.display(), "failed to append example response log");
            } else {
                info!(path = %path.display(), "appended Mollie example response log");
                rebuild_example_support_matrix();
            }
        }
        Err(error) => {
            warn!(%error, path = %path.display(), "failed to open example response log");
        }
    }
}

/// Timestamp for log entry headers (unix seconds; chrono clock feature is off).
fn log_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => format!("{}s", duration.as_secs()),
        Err(_) => "unknown".to_owned(),
    }
}

/// Crate root (parent of `examples/`).
fn crate_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

/// Path of the auto-generated support matrix markdown file.
fn example_support_matrix_path() -> PathBuf {
    crate_root().join("docs").join("example-support-matrix.md")
}

/// Outcome of the latest log entry for one example.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SupportOutcome {
    /// Last run returned a success body (`OK response` / `OK envelope`).
    Supported,
    /// Last run logged an error response or non-Mollie failure.
    Failed,
    /// Missing credentials; example did not call Mollie.
    Skipped,
    /// No `logs/<example>.log` yet (or unparseable).
    Untested,
}

impl SupportOutcome {
    fn as_str(self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::Failed => "failed",
            Self::Skipped => "skipped",
            Self::Untested => "untested",
        }
    }

    fn sort_key(self) -> u8 {
        match self {
            Self::Failed => 0,
            Self::Supported => 1,
            Self::Skipped => 2,
            Self::Untested => 3,
        }
    }
}

/// Latest status derived from one example log (or discovery-only untested row).
#[derive(Debug, Clone)]
struct ExampleSupportRow {
    example: String,
    route: String,
    outcome: SupportOutcome,
    status: String,
    code: String,
    key: String,
    label: String,
    summary: String,
    updated: String,
    log_rel: String,
}

/// Rebuild `docs/example-support-matrix.md` from `logs/*.log` + discovered examples.
///
/// Never fails the example: matrix write problems are warned only.
fn rebuild_example_support_matrix() {
    let rows = collect_example_support_rows();
    let markdown = render_example_support_matrix(&rows);
    let path = example_support_matrix_path();
    if let Some(parent) = path.parent() {
        if let Err(error) = fs::create_dir_all(parent) {
            warn!(
                %error,
                path = %parent.display(),
                "failed to create docs dir for example support matrix"
            );
            return;
        }
    }
    match fs::write(&path, markdown) {
        Ok(()) => {
            info!(
                path = %path.display(),
                rows = rows.len(),
                "rebuilt example support matrix from logs"
            );
        }
        Err(error) => {
            warn!(
                %error,
                path = %path.display(),
                "failed to write example support matrix"
            );
        }
    }
}

/// Discover every example and merge with the latest log outcome for each.
fn collect_example_support_rows() -> Vec<ExampleSupportRow> {
    let mut by_name: BTreeMap<String, ExampleSupportRow> = BTreeMap::new();

    for (name, route) in discover_example_targets() {
        by_name.insert(
            name.clone(),
            ExampleSupportRow {
                example: name,
                route,
                outcome: SupportOutcome::Untested,
                status: "-".to_owned(),
                code: "-".to_owned(),
                key: "-".to_owned(),
                label: "-".to_owned(),
                summary: "no log yet".to_owned(),
                updated: "-".to_owned(),
                log_rel: "-".to_owned(),
            },
        );
    }

    let logs_dir = example_logs_dir();
    let Ok(entries) = fs::read_dir(&logs_dir) else {
        return by_name.into_values().collect();
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("log") {
            continue;
        }
        let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        let Ok(content) = fs::read_to_string(&path) else {
            continue;
        };
        let Some(parsed) = parse_latest_log_entry(stem, &content) else {
            continue;
        };
        let log_rel = format!("logs/{stem}.log");
        by_name
            .entry(stem.to_owned())
            .and_modify(|row| {
                row.route = if parsed.route.is_empty() {
                    row.route.clone()
                } else {
                    parsed.route.clone()
                };
                row.outcome = parsed.outcome;
                row.status = parsed.status.clone();
                row.code = parsed.code.clone();
                row.key = parsed.key.clone();
                row.label = parsed.label.clone();
                row.summary = parsed.summary.clone();
                row.updated = parsed.updated.clone();
                row.log_rel = log_rel.clone();
            })
            .or_insert_with(|| ExampleSupportRow {
                example: stem.to_owned(),
                route: parsed.route,
                outcome: parsed.outcome,
                status: parsed.status,
                code: parsed.code,
                key: parsed.key,
                label: parsed.label,
                summary: parsed.summary,
                updated: parsed.updated,
                log_rel,
            });
    }

    let mut rows: Vec<ExampleSupportRow> = by_name.into_values().collect();
    rows.sort_by(|a, b| {
        a.outcome
            .sort_key()
            .cmp(&b.outcome.sort_key())
            .then_with(|| a.example.cmp(&b.example))
    });
    rows
}

/// Scan `examples/*.rs` for `const NAME` / `const ROUTE` (generated + hand-written).
fn discover_example_targets() -> Vec<(String, String)> {
    let examples_dir = crate_root().join("examples");
    let Ok(entries) = fs::read_dir(&examples_dir) else {
        return Vec::new();
    };

    let mut out: Vec<(String, String)> = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("rs") {
            continue;
        }
        // Skip support module and non-runnable helpers.
        if path.file_name().and_then(|n| n.to_str()) == Some("support") {
            continue;
        }
        let Ok(source) = fs::read_to_string(&path) else {
            continue;
        };
        if !source.contains("impl RunnableExample") {
            continue;
        }
        let name = extract_const_str(&source, "NAME").unwrap_or_else(|| {
            path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_owned()
        });
        let route = extract_const_str(&source, "ROUTE").unwrap_or_else(|| "-".to_owned());
        out.push((name, route));
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

/// Extract `const NAME: &'static str = "...";` (or `ROUTE`) from example source.
fn extract_const_str(source: &str, const_name: &str) -> Option<String> {
    let needle = format!("const {const_name}:");
    let line = source.lines().find(|l| l.contains(&needle))?;
    let after_eq = line.split_once('=')?.1.trim();
    let quoted = after_eq.trim_start_matches('"');
    let value = quoted.split('"').next()?.to_owned();
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

/// Parse the **last** timestamped block in a `logs/<example>.log` file.
fn parse_latest_log_entry(example_name: &str, content: &str) -> Option<ExampleSupportRow> {
    let mut blocks: Vec<&str> = content
        .split("========== ")
        .filter(|b| !b.trim().is_empty())
        .collect();
    let last = blocks.pop()?;
    // Header line: `1783900198s ==========\nexample: ...\n...`
    let (header, rest) = last.split_once(" ==========")?;
    let updated = header.trim().to_owned();
    let body = rest.trim_start_matches('\n');

    let mut route = String::new();
    let mut status = String::new();
    let mut code = String::new();
    let mut key = String::new();
    let mut summary = String::new();
    let mut kind_line = String::new();

    for line in body.lines() {
        if line.starts_with("example:") {
            continue;
        }
        if kind_line.is_empty()
            && (line.starts_with("OK ") || line.starts_with("ERROR ") || line.starts_with("SKIP "))
        {
            kind_line = line.to_owned();
            continue;
        }
        if let Some(v) = line.strip_prefix("route: ") {
            route = v.trim().to_owned();
        } else if let Some(v) = line.strip_prefix("status: ") {
            status = v.trim().to_owned();
        } else if let Some(v) = line.strip_prefix("code: ") {
            code = v.trim().to_owned();
        } else if let Some(v) = line.strip_prefix("key: ") {
            key = v.trim().to_owned();
        } else if let Some(v) = line.strip_prefix("summary: ") {
            summary = v.trim().to_owned();
        } else if line.starts_with("body:") {
            break;
        } else if summary.is_empty() && line.starts_with("error: ") {
            summary = line.trim_start_matches("error: ").trim().to_owned();
        }
    }

    let outcome = if kind_line.starts_with("OK ") {
        SupportOutcome::Supported
    } else if kind_line.starts_with("SKIP ") {
        SupportOutcome::Skipped
    } else if kind_line.starts_with("ERROR ") {
        SupportOutcome::Failed
    } else if body.contains("OK response") || body.contains("OK envelope") {
        SupportOutcome::Supported
    } else if body.contains("SKIP ") {
        SupportOutcome::Skipped
    } else if body.contains("ERROR ") {
        SupportOutcome::Failed
    } else {
        SupportOutcome::Untested
    };

    if summary.is_empty() {
        summary = if kind_line.is_empty() {
            "see log".to_owned()
        } else {
            kind_line.clone()
        };
    }
    if status.is_empty() {
        status = "-".to_owned();
    }
    if code.is_empty() {
        code = "-".to_owned();
    }
    if key.is_empty() {
        key = "-".to_owned();
    }
    if route.is_empty() {
        route = "-".to_owned();
    }

    Some(ExampleSupportRow {
        example: example_name.to_owned(),
        route,
        outcome,
        status,
        code,
        label: support_label(&key).to_owned(),
        key,
        summary,
        updated,
        log_rel: format!("logs/{example_name}.log"),
    })
}

fn support_label(key: &str) -> &'static str {
    match key {
        ACCESS_TOKEN_PROFILE_RESTRICTED_KEY => ACCESS_TOKEN_PROFILE_RESTRICTED_LABEL,
        _ => "-",
    }
}

fn md_cell(value: &str) -> String {
    value
        .replace('|', "\\|")
        .replace('\n', " ")
        .replace('\r', "")
}

fn render_example_support_matrix(rows: &[ExampleSupportRow]) -> String {
    let mut supported = 0usize;
    let mut failed = 0usize;
    let mut skipped = 0usize;
    let mut untested = 0usize;
    for row in rows {
        match row.outcome {
            SupportOutcome::Supported => supported += 1,
            SupportOutcome::Failed => failed += 1,
            SupportOutcome::Skipped => skipped += 1,
            SupportOutcome::Untested => untested += 1,
        }
    }

    let mut out = String::new();
    out.push_str("# Example support matrix\n\n");
    out.push_str(
        "Auto-generated from the **latest** entry in each `logs/<example>.log` file whenever a route example runs (`examples/support/mod.rs`).\n\n",
    );
    out.push_str(
        "Do not edit by hand - re-run examples (or delete a log and re-run) to refresh a row.\n\n",
    );
    out.push_str("Offline rebuild (no API calls):\n\n");
    out.push_str("```sh\n");
    out.push_str("python scripts/rebuild_example_support_matrix.py\n");
    out.push_str("```\n\n");
    out.push_str("## How to read this\n\n");
    out.push_str("| Support | Meaning |\n");
    out.push_str("| --- | --- |\n");
    out.push_str(
        "| `supported` | Last run logged `OK response` / `OK envelope` (HTTP success decoded). |\n",
    );
    out.push_str(
        "| `failed` | Last run logged `ERROR ...` (API error, decode error, or client failure). |\n",
    );
    out.push_str("| `skipped` | Missing credentials; example did not call Mollie. |\n");
    out.push_str("| `untested` | No `logs/<example>.log` yet (or unparseable). |\n\n");
    out.push_str(
        "| Label | Meaning |\n| --- | --- |\n| `access-token-not-profile-restricted` | The endpoint requires an access token that is not restricted to a specific profile. |\n\n",
    );
    out.push_str(&format!(
        "**Totals:** {total} examples - **{supported}** supported, **{failed}** failed, **{skipped}** skipped, **{untested}** untested.\n\n",
        total = rows.len(),
    ));
    out.push_str(
        "Detail and full bodies stay in the per-example log; this table is the roll-up.\n\n",
    );
    out.push_str("## Matrix\n\n");
    out.push_str(
        "| Example | Route | Support | HTTP | Code | Key | Label | Summary | Log | Updated |\n",
    );
    out.push_str("| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |\n");

    for row in rows {
        out.push_str(&format!(
            "| `{}` | `{}` | `{}` | {} | {} | {} | {} | {} | `{}` | {} |\n",
            md_cell(&row.example),
            md_cell(&row.route),
            row.outcome.as_str(),
            md_cell(&row.status),
            md_cell(&row.code),
            md_cell(&row.key),
            md_cell(&row.label),
            md_cell(&row.summary),
            md_cell(&row.log_rel),
            md_cell(&row.updated),
        ));
    }

    out.push('\n');
    out
}
