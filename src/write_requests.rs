//! Validated builders for Mollie write request bodies.

use serde_json::json;

use crate::{
    types, ApplicationFee, Date, MandateId, MollieError, MollieResult, Money, PaymentDescription,
    WebhookUrl,
};

/// The validated required fields for creating a refund.
#[derive(Clone, Debug)]
pub struct CreateRefundRequired {
    /// The amount to refund.
    pub amount: Money,
    /// The customer-visible refund description.
    pub description: PaymentDescription,
    /// Optional request metadata.
    pub metadata: Option<serde_json::Value>,
    /// Whether routed funds should be fully reversed.
    pub reverse_routing: Option<bool>,
    /// Fine-grained reversals for routed funds.
    pub routing_reversals: Option<Vec<types::EntityRefundRoutingReversalsItem>>,
    /// Whether to create the refund in test mode for organization-level credentials.
    pub testmode: Option<bool>,
}

impl CreateRefundRequired {
    /// Validates the required create-refund fields.
    pub fn new(amount: Money, description: impl Into<String>) -> MollieResult<Self> {
        Ok(Self {
            amount,
            description: PaymentDescription::parse(description)?,
            metadata: None,
            reverse_routing: None,
            routing_reversals: None,
            testmode: None,
        })
    }

    /// Sets metadata that Mollie stores with the refund.
    pub fn with_metadata(mut self, value: serde_json::Value) -> Self {
        self.metadata = Some(value);
        self
    }

    /// Sets whether Mollie should fully reverse funds routed to connected
    /// merchants. This cannot be combined with `routingReversals`.
    pub fn with_reverse_routing(mut self, value: bool) -> MollieResult<Self> {
        if self.routing_reversals.is_some() {
            return Err(MollieError::invalid_request(
                "reverseRouting cannot be combined with routingReversals",
            ));
        }
        self.reverse_routing = Some(value);
        Ok(self)
    }

    /// Sets validated fine-grained reversals for routed funds.
    pub fn with_routing_reversals(
        mut self,
        value: Vec<types::EntityRefundRoutingReversalsItem>,
    ) -> MollieResult<Self> {
        if self.reverse_routing.is_some() {
            return Err(MollieError::invalid_request(
                "routingReversals cannot be combined with reverseRouting",
            ));
        }
        validate_routing_reversals(&value)?;
        self.routing_reversals = Some(value);
        Ok(self)
    }

    /// Sets the request-body test-mode value for organization-level credentials.
    pub fn with_testmode(mut self, value: bool) -> Self {
        self.testmode = Some(value);
        self
    }

    /// Builds a typed create-refund request without response-owned fields.
    pub fn into_request(self) -> MollieResult<types::EntityRefund> {
        let mut request: types::EntityRefund = serde_json::from_value(json!({
            "amount": self.amount.into_amount(),
            "description": self.description.into_string(),
            "metadata": self.metadata,
        }))
        .map_err(|error| MollieError::invalid_request(error.to_string()))?;
        validate_routing_reversals(self.routing_reversals.as_deref().unwrap_or_default())?;
        if self.reverse_routing.is_some() && self.routing_reversals.is_some() {
            return Err(MollieError::invalid_request(
                "reverseRouting cannot be combined with routingReversals",
            ));
        }
        request.reverse_routing = self.reverse_routing;
        request.routing_reversals = self.routing_reversals;
        request.testmode = self
            .testmode
            .map(|value| types::TestmodeCreate::from(Some(value)));
        Ok(request)
    }
}

/// The validated required fields for creating a subscription.
#[derive(Clone, Debug)]
pub struct CreateSubscriptionRequired {
    /// The recurring charge amount.
    pub amount: Money,
    /// The subscription description.
    pub description: PaymentDescription,
    /// The interval between charges.
    pub interval: String,
    /// Optional first charge date.
    pub start_date: Option<String>,
    /// Optional subscription webhook URL.
    pub webhook_url: Option<String>,
    /// Optional request metadata.
    pub metadata: Option<serde_json::Value>,
    /// Optional mandate to use for the recurring payments.
    pub mandate_id: Option<MandateId>,
    /// Optional payment method to use for the subscription.
    pub method: Option<types::SubscriptionMethod>,
    /// Optional total number of payments.
    pub times: Option<i64>,
    /// Optional Mollie Connect application fee charged on each payment.
    pub application_fee: Option<ApplicationFee>,
    /// Whether to create the subscription in test mode for organization-level credentials.
    pub testmode: Option<bool>,
}

impl CreateSubscriptionRequired {
    /// Validates the required create-subscription fields.
    pub fn new(
        amount: Money,
        description: impl Into<String>,
        interval: impl Into<String>,
    ) -> MollieResult<Self> {
        let interval = interval.into();
        validate_interval(&interval)?;
        Ok(Self {
            amount,
            description: PaymentDescription::parse(description)?,
            interval,
            start_date: None,
            webhook_url: None,
            metadata: None,
            mandate_id: None,
            method: None,
            times: None,
            application_fee: None,
            testmode: None,
        })
    }

    /// Sets and validates the subscription start date.
    pub fn with_start_date(mut self, value: impl AsRef<str>) -> MollieResult<Self> {
        self.start_date = Some(Date::parse(value.as_ref())?.to_string());
        Ok(self)
    }

    /// Sets and validates the subscription webhook URL.
    pub fn with_webhook_url(mut self, value: impl Into<String>) -> MollieResult<Self> {
        self.webhook_url = Some(WebhookUrl::parse(value)?.into_string());
        Ok(self)
    }

    /// Sets metadata that Mollie stores with the subscription.
    pub fn with_metadata(mut self, value: serde_json::Value) -> Self {
        self.metadata = Some(value);
        self
    }

    /// Sets and validates the mandate used for the subscription.
    pub fn with_mandate(mut self, value: impl AsRef<str>) -> MollieResult<Self> {
        self.mandate_id = Some(MandateId::parse(value)?);
        Ok(self)
    }

    /// Sets the payment method used for the subscription.
    pub fn with_method(mut self, value: impl AsRef<str>) -> MollieResult<Self> {
        let method = match value.as_ref() {
            "creditcard" => types::SubscriptionMethodInner::Creditcard,
            "directdebit" => types::SubscriptionMethodInner::Directdebit,
            "paypal" => types::SubscriptionMethodInner::Paypal,
            value => {
                return Err(MollieError::invalid_request(format!(
                    "invalid subscription payment method `{value}`"
                )))
            }
        };
        self.method = Some(types::SubscriptionMethod::from(Some(method)));
        Ok(self)
    }

    /// Sets and validates the total number of subscription payments.
    pub fn with_times(mut self, value: i64) -> MollieResult<Self> {
        if value <= 0 {
            return Err(MollieError::invalid_request(
                "subscription times must be greater than zero",
            ));
        }
        self.times = Some(value);
        Ok(self)
    }

    /// Sets the validated Mollie Connect application fee charged per payment.
    pub fn with_application_fee(mut self, value: ApplicationFee) -> Self {
        self.application_fee = Some(value);
        self
    }

    /// Sets the request-body test-mode value for organization-level credentials.
    pub fn with_testmode(mut self, value: bool) -> Self {
        self.testmode = Some(value);
        self
    }

    /// Builds a typed create-subscription request without response-owned fields.
    pub fn into_request(self) -> MollieResult<types::CreateSubscriptionRequest> {
        let mut request: types::CreateSubscriptionRequest = serde_json::from_value(json!({
            "amount": self.amount.into_amount(),
            "description": self.description.into_string(),
            "interval": self.interval,
            "startDate": self.start_date,
            "webhookUrl": self.webhook_url,
            "metadata": self.metadata,
        }))
        .map_err(|error| MollieError::invalid_request(error.to_string()))?;
        request.application_fee = self.application_fee.map(Into::into);
        request.mandate_id = self.mandate_id.map(MandateId::into_token);
        request.method = self.method;
        request.testmode = self
            .testmode
            .map(|value| types::TestmodeCreate::from(Some(value)));
        request.times = self.times;
        Ok(request)
    }
}

fn validate_routing_reversals(
    values: &[types::EntityRefundRoutingReversalsItem],
) -> MollieResult<()> {
    for reversal in values {
        if let Some(amount) = &reversal.amount {
            Money::try_from(amount)?;
        }
    }
    Ok(())
}

/// Validated fields for creating a capture on an authorized payment.
#[derive(Clone, Debug)]
pub struct CreateCaptureRequired {
    /// Optional partial capture amount (omit for remaining authorized amount).
    pub amount: Option<Money>,
    /// Optional capture description.
    pub description: Option<PaymentDescription>,
    /// Optional metadata.
    pub metadata: Option<serde_json::Value>,
}

impl CreateCaptureRequired {
    /// Creates a full remaining-amount capture with no description.
    pub fn full() -> Self {
        Self {
            amount: None,
            description: None,
            metadata: None,
        }
    }

    /// Creates a partial capture for a validated amount.
    pub fn partial(amount: Money) -> Self {
        Self {
            amount: Some(amount),
            description: None,
            metadata: None,
        }
    }

    /// Sets a validated description.
    pub fn with_description(mut self, description: impl Into<String>) -> MollieResult<Self> {
        self.description = Some(PaymentDescription::parse(description)?);
        Ok(self)
    }

    /// Sets metadata.
    pub fn with_metadata(mut self, value: serde_json::Value) -> Self {
        self.metadata = Some(value);
        self
    }

    /// Builds a write-only capture body (no response-owned fields).
    ///
    /// Constructed via JSON so generated response-owned fields stay absent
    /// rather than requiring an exhaustive `EntityCapture { ... }` literal.
    pub fn into_request(self) -> MollieResult<types::EntityCapture> {
        let mut value = json!({});
        if let Some(amount) = self.amount {
            let amount_value = amount.into_amount();
            value["amount"] = json!({
                "currency": amount_value.currency,
                "value": amount_value.value,
            });
        }
        if let Some(description) = self.description {
            value["description"] = json!(description.into_string());
        }
        if let Some(metadata) = self.metadata {
            value["metadata"] = metadata;
        }
        serde_json::from_value(value)
            .map_err(|error| MollieError::invalid_request(error.to_string()))
    }
}

/// Validated SEPA Direct Debit mandate create fields.
#[derive(Clone, Debug)]
pub struct CreateSepaMandateRequired {
    /// Account holder name.
    pub consumer_name: String,
    /// IBAN (consumerAccount).
    pub consumer_account: String,
    /// Optional BIC.
    pub consumer_bic: Option<String>,
    /// Optional unique mandate reference.
    pub mandate_reference: Option<String>,
}

impl CreateSepaMandateRequired {
    /// Validates required SEPA mandate fields.
    pub fn new(
        consumer_name: impl Into<String>,
        consumer_account: impl Into<String>,
    ) -> MollieResult<Self> {
        let consumer_name = consumer_name.into();
        let consumer_account = consumer_account.into();
        if consumer_name.trim().is_empty() {
            return Err(MollieError::invalid_request(
                "mandate consumerName cannot be empty",
            ));
        }
        validate_iban_like(&consumer_account)?;
        Ok(Self {
            consumer_name,
            consumer_account,
            consumer_bic: None,
            mandate_reference: None,
        })
    }

    /// Sets an optional BIC.
    pub fn with_bic(mut self, bic: impl Into<String>) -> MollieResult<Self> {
        let bic = bic.into();
        if bic.trim().is_empty() {
            return Err(MollieError::invalid_request(
                "mandate consumerBic cannot be empty",
            ));
        }
        self.consumer_bic = Some(bic);
        Ok(self)
    }

    /// Sets an optional mandate reference.
    pub fn with_reference(mut self, reference: impl Into<String>) -> Self {
        self.mandate_reference = Some(reference.into());
        self
    }

    /// Builds a generated mandate request body.
    pub fn into_request(self) -> MollieResult<types::MandateRequest> {
        serde_json::from_value(json!({
            "method": "directdebit",
            "consumerName": self.consumer_name,
            "consumerAccount": self.consumer_account,
            "consumerBic": self.consumer_bic,
            "mandateReference": self.mandate_reference,
        }))
        .map_err(|error| MollieError::invalid_request(error.to_string()))
    }
}

/// Validated payment-link create fields.
#[derive(Clone, Debug)]
pub struct CreatePaymentLinkRequired {
    /// Link description shown to the customer.
    pub description: PaymentDescription,
    /// Optional fixed amount (open amount when `None`).
    pub amount: Option<Money>,
    /// Optional redirect URL after payment.
    pub redirect_url: Option<String>,
    /// Optional webhook URL.
    pub webhook_url: Option<WebhookUrl>,
}

impl CreatePaymentLinkRequired {
    /// Creates a payment link with a validated description.
    pub fn new(description: impl Into<String>) -> MollieResult<Self> {
        Ok(Self {
            description: PaymentDescription::parse(description)?,
            amount: None,
            redirect_url: None,
            webhook_url: None,
        })
    }

    /// Sets a fixed amount.
    pub fn with_amount(mut self, amount: Money) -> Self {
        self.amount = Some(amount);
        self
    }

    /// Sets redirect URL (must be absolute http/https when present).
    pub fn with_redirect_url(mut self, url: impl Into<String>) -> MollieResult<Self> {
        let url = url.into();
        let parsed = reqwest::Url::parse(&url).map_err(|error| {
            MollieError::invalid_request(format!("invalid payment-link redirectUrl: {error}"))
        })?;
        if parsed.scheme() != "http" && parsed.scheme() != "https" {
            return Err(MollieError::invalid_request(
                "payment-link redirectUrl must use http or https",
            ));
        }
        self.redirect_url = Some(url);
        Ok(self)
    }

    /// Sets a validated webhook URL.
    pub fn with_webhook_url(mut self, url: WebhookUrl) -> Self {
        self.webhook_url = Some(url);
        self
    }

    /// Builds the generated payment-link body.
    pub fn into_request(self) -> MollieResult<types::CreatePaymentLinkBody> {
        let description_str = self.description.into_string();
        let description: types::CreatePaymentLinkBodyDescription =
            description_str.parse().map_err(|error| {
                MollieError::invalid_request(format!("payment-link description: {error}"))
            })?;
        let mut body: types::CreatePaymentLinkBody = serde_json::from_value(json!({
            "description": description_str,
        }))
        .map_err(|error| MollieError::invalid_request(error.to_string()))?;
        body.description = description;
        if let Some(amount) = self.amount {
            body.amount = Some(Some(amount).into());
        }
        body.redirect_url = self.redirect_url;
        body.webhook_url = self.webhook_url.map(WebhookUrl::into_string);
        Ok(body)
    }
}

/// Basic IBAN shape check (not a full mod-97 validation).
fn validate_iban_like(value: &str) -> MollieResult<()> {
    validate_iban_like_labeled(value, "IBAN")
}

fn validate_iban_like_labeled(value: &str, label: &str) -> MollieResult<()> {
    let compact: String = value.chars().filter(|c| !c.is_whitespace()).collect();
    if compact.len() < 15 || compact.len() > 34 {
        return Err(MollieError::invalid_request(format!(
            "{label} must be 15..=34 characters"
        )));
    }
    if !compact.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(MollieError::invalid_request(format!(
            "{label} must be alphanumeric"
        )));
    }
    if !compact
        .get(..2)
        .is_some_and(|cc| cc.chars().all(|c| c.is_ascii_alphabetic()))
    {
        return Err(MollieError::invalid_request(format!(
            "{label} must start with a country code"
        )));
    }
    Ok(())
}

/// Validated required fields for creating a balance payout.
#[derive(Clone, Debug)]
pub struct CreatePayoutRequired {
    /// Balance to pay out (`bal_…`).
    pub balance_id: crate::BalanceId,
    /// Optional amount; omit for full available balance (minus reserve).
    pub amount: Option<Money>,
    /// Optional bank-statement description (max 255).
    pub description: Option<String>,
    /// Optional body `testmode` for organization-level credentials.
    pub testmode: Option<bool>,
}

impl CreatePayoutRequired {
    /// Creates a full-balance payout request for the given balance.
    pub fn full_balance(balance_id: crate::BalanceId) -> Self {
        Self {
            balance_id,
            amount: None,
            description: None,
            testmode: None,
        }
    }

    /// Parses `balance_id` then builds a full-balance payout request.
    pub fn full_balance_str(balance_id: impl AsRef<str>) -> MollieResult<Self> {
        Ok(Self::full_balance(crate::BalanceId::parse(balance_id)?))
    }

    /// Creates a partial payout with a validated [`Money`] amount.
    pub fn with_amount_for_balance(balance_id: crate::BalanceId, amount: Money) -> Self {
        Self {
            balance_id,
            amount: Some(amount),
            description: None,
            testmode: None,
        }
    }

    /// Parses `balance_id` then builds a partial payout request.
    pub fn with_amount_for_balance_str(
        balance_id: impl AsRef<str>,
        amount: Money,
    ) -> MollieResult<Self> {
        Ok(Self::with_amount_for_balance(
            crate::BalanceId::parse(balance_id)?,
            amount,
        ))
    }

    /// Sets the bank-statement description (1..=255 chars when present).
    pub fn with_description(mut self, description: impl Into<String>) -> MollieResult<Self> {
        let description = description.into();
        let trimmed = description.trim();
        if trimmed.is_empty() || trimmed.chars().count() > 255 {
            return Err(MollieError::invalid_request(
                "payout description must be 1..=255 characters",
            ));
        }
        self.description = Some(trimmed.to_string());
        Ok(self)
    }

    /// Sets body-level testmode for organization credentials.
    pub fn with_testmode(mut self, value: bool) -> Self {
        self.testmode = Some(value);
        self
    }

    /// Builds a create-payout body that serializes write fields only.
    pub fn into_request(self) -> MollieResult<types::PayoutRequest> {
        let mut body: types::EntityPayout = serde_json::from_value(json!({
            "balanceId": self.balance_id.as_str(),
        }))
        .map_err(|error| MollieError::invalid_request(error.to_string()))?;
        if let Some(amount) = self.amount {
            let amount = amount.into_amount();
            body.amount = Some(types::AmountNullable(Some(types::AmountNullableInner {
                currency: amount.currency,
                value: amount.value,
            })));
        }
        if let Some(description) = self.description {
            body.description = Some(
                description
                    .parse::<types::EntityPayoutDescription>()
                    .map_err(|error| {
                        MollieError::invalid_request(format!("payout description: {error}"))
                    })?,
            );
        }
        body.testmode = self
            .testmode
            .map(|value| types::TestmodeCreate::from(Some(value)));
        Ok(types::PayoutRequest(body))
    }
}

/// Validated required fields for creating a business-account SEPA transfer.
#[derive(Clone, Debug)]
pub struct CreateTransferRequired {
    /// Transfer amount (EUR for SEPA).
    pub amount: Money,
    /// Debtor Mollie business-account IBAN.
    pub debtor_iban: String,
    /// Creditor account holder name.
    pub creditor_name: String,
    /// Creditor IBAN.
    pub creditor_iban: String,
    /// SEPA scheme.
    pub transfer_scheme: types::TransferSchemeType,
    /// Optional bank-statement description.
    pub description: Option<String>,
    /// Optional body testmode.
    pub testmode: Option<bool>,
}

impl CreateTransferRequired {
    /// Validates transfer create fields (money + IBANs + names).
    pub fn new(
        amount: Money,
        debtor_iban: impl Into<String>,
        creditor_name: impl Into<String>,
        creditor_iban: impl Into<String>,
        transfer_scheme: types::TransferSchemeType,
    ) -> MollieResult<Self> {
        let debtor_iban = debtor_iban.into();
        let creditor_iban = creditor_iban.into();
        let creditor_name = creditor_name.into();
        validate_iban_like_labeled(&debtor_iban, "transfer debtorIban")?;
        validate_iban_like_labeled(&creditor_iban, "transfer creditor IBAN")?;
        if creditor_name.trim().is_empty() {
            return Err(MollieError::invalid_request(
                "transfer creditor fullName must not be empty",
            ));
        }
        Ok(Self {
            amount,
            debtor_iban,
            creditor_name: creditor_name.trim().to_string(),
            creditor_iban,
            transfer_scheme,
            description: None,
            testmode: None,
        })
    }

    /// Sets a transfer description validated against Mollie's pattern.
    pub fn with_description(mut self, description: impl Into<String>) -> MollieResult<Self> {
        let description = description.into();
        description
            .parse::<types::TransferRequestDescription>()
            .map_err(|error| {
                MollieError::invalid_request(format!("transfer description: {error}"))
            })?;
        self.description = Some(description);
        Ok(self)
    }

    /// Sets body-level testmode.
    pub fn with_testmode(mut self, value: bool) -> Self {
        self.testmode = Some(value);
        self
    }

    /// Builds the generated transfer request body.
    pub fn into_request(self) -> MollieResult<types::TransferRequest> {
        let description = match self.description {
            Some(value) => Some(value.parse::<types::TransferRequestDescription>().map_err(
                |error| MollieError::invalid_request(format!("transfer description: {error}")),
            )?),
            None => None,
        };
        Ok(types::TransferRequest {
            amount: self.amount.into_amount(),
            business_account_transaction_id: None,
            created_at: None,
            credit_debit_indicator: None,
            creditor: types::TransferParty {
                full_name: self.creditor_name,
                account: types::TransferPartyAccount {
                    iban: self.creditor_iban,
                },
            },
            debtor: None,
            debtor_iban: self.debtor_iban,
            description,
            id: None,
            metadata: None,
            mode: None,
            resource: None,
            status: None,
            status_history: Vec::new(),
            status_reason: None,
            testmode: self
                .testmode
                .map(|value| types::TestmodeCreate::from(Some(value))),
            transfer_scheme: types::TransferScheme {
                type_: self.transfer_scheme,
            },
        })
    }
}

/// One side of a Connect balance transfer (organization party).
#[derive(Clone, Debug)]
pub struct ConnectBalanceTransferParty {
    /// Organization id (`org_…`).
    pub organization_id: String,
    /// Party-facing description (ledger line).
    pub description: String,
}

/// Validated required fields for creating a Connect balance transfer.
#[derive(Clone, Debug)]
pub struct CreateConnectBalanceTransferRequired {
    /// Transfer amount.
    pub amount: Money,
    /// Initiating-party description.
    pub description: String,
    /// Source organization.
    pub source: ConnectBalanceTransferParty,
    /// Destination organization.
    pub destination: ConnectBalanceTransferParty,
    /// Optional Mollie category.
    pub category: Option<types::BalanceTransferCategory>,
    /// Optional structured metadata (≤ ~1KB).
    pub metadata: Option<serde_json::Map<String, serde_json::Value>>,
}

impl CreateConnectBalanceTransferRequired {
    /// Validates amount, descriptions, and organization ids.
    pub fn new(
        amount: Money,
        description: impl Into<String>,
        source_organization_id: impl Into<String>,
        source_description: impl Into<String>,
        destination_organization_id: impl Into<String>,
        destination_description: impl Into<String>,
    ) -> MollieResult<Self> {
        let description = require_description(description, "connect balance transfer description")?;
        let source = ConnectBalanceTransferParty {
            organization_id: require_org_id(source_organization_id, "source organization id")?,
            description: require_description(
                source_description,
                "connect balance transfer source description",
            )?,
        };
        let destination = ConnectBalanceTransferParty {
            organization_id: require_org_id(
                destination_organization_id,
                "destination organization id",
            )?,
            description: require_description(
                destination_description,
                "connect balance transfer destination description",
            )?,
        };
        if source.organization_id == destination.organization_id {
            return Err(MollieError::invalid_request(
                "connect balance transfer source and destination organizations must differ",
            ));
        }
        Ok(Self {
            amount,
            description,
            source,
            destination,
            category: None,
            metadata: None,
        })
    }

    /// Sets optional transfer category.
    pub fn with_category(mut self, category: types::BalanceTransferCategory) -> Self {
        self.category = Some(category);
        self
    }

    /// Sets optional metadata map.
    pub fn with_metadata(
        mut self,
        metadata: serde_json::Map<String, serde_json::Value>,
    ) -> MollieResult<Self> {
        let encoded = serde_json::to_vec(&metadata)
            .map_err(|error| MollieError::invalid_request(error.to_string()))?;
        if encoded.len() > 1024 {
            return Err(MollieError::invalid_request(
                "connect balance transfer metadata must be at most ~1KB",
            ));
        }
        self.metadata = Some(metadata);
        Ok(self)
    }

    /// Builds a create body that serializes write fields only.
    pub fn into_request(self) -> MollieResult<types::EntityBalanceTransfer> {
        let amount = self.amount.into_amount();
        let mut value = json!({
            "amount": {
                "currency": amount.currency,
                "value": amount.value,
            },
            "description": self.description,
            "source": {
                "type": "organization",
                "id": self.source.organization_id,
                "description": self.source.description,
            },
            "destination": {
                "type": "organization",
                "id": self.destination.organization_id,
                "description": self.destination.description,
            },
        });
        if let Some(category) = self.category {
            value["category"] = json!(category);
        }
        if let Some(metadata) = self.metadata {
            value["metadata"] = serde_json::Value::Object(metadata);
        }
        serde_json::from_value(value)
            .map_err(|error| MollieError::invalid_request(error.to_string()))
    }
}

fn require_description(value: impl Into<String>, label: &str) -> MollieResult<String> {
    let value = value.into();
    let trimmed = value.trim();
    let chars = trimmed.chars().count();
    if chars == 0 || chars > 255 {
        return Err(MollieError::invalid_request(format!(
            "{label} must be 1..=255 characters"
        )));
    }
    Ok(trimmed.to_string())
}

fn require_org_id(value: impl Into<String>, label: &str) -> MollieResult<String> {
    let value = value.into();
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(MollieError::invalid_request(format!(
            "{label} must not be empty"
        )));
    }
    if !trimmed.starts_with("org_") {
        return Err(MollieError::invalid_request(format!(
            "{label} must start with org_"
        )));
    }
    Ok(trimmed.to_string())
}

/// Validated Verification-of-Payee request.
#[derive(Clone, Debug)]
pub struct VerifyPayeeRequired {
    /// Account holder name to verify.
    pub account_holder_name: String,
    /// Creditor IBAN.
    pub account_number: String,
    /// Optional body testmode.
    pub testmode: Option<bool>,
}

impl VerifyPayeeRequired {
    /// Validates name + IBAN for VoP.
    pub fn new(
        account_holder_name: impl Into<String>,
        account_number: impl Into<String>,
    ) -> MollieResult<Self> {
        let account_holder_name = account_holder_name.into();
        let account_number = account_number.into();
        if account_holder_name.trim().is_empty() {
            return Err(MollieError::invalid_request(
                "verify-payee accountHolderName must not be empty",
            ));
        }
        validate_iban_like_labeled(&account_number, "verify-payee accountNumber")?;
        Ok(Self {
            account_holder_name: account_holder_name.trim().to_string(),
            account_number,
            testmode: None,
        })
    }

    /// Sets body-level testmode.
    pub fn with_testmode(mut self, value: bool) -> Self {
        self.testmode = Some(value);
        self
    }

    /// Builds the generated VoP request body.
    pub fn into_request(self) -> types::VerificationOfPayeeRequest {
        types::VerificationOfPayeeRequest {
            creditor_bank_account: types::CreditorBankAccount {
                account_holder_name: self.account_holder_name,
                account_number: self.account_number,
                format: types::AccountNumberFormat::Iban,
            },
            testmode: self
                .testmode
                .map(|value| types::TestmodeCreate::from(Some(value))),
        }
    }
}

/// Validates Mollie's documented subscription interval syntax.
fn validate_interval(value: &str) -> MollieResult<()> {
    let mut parts = value.split_whitespace();
    let number = parts.next().unwrap_or_default();
    let unit = parts.next().unwrap_or_default();
    if parts.next().is_some()
        || number.is_empty()
        || !number.chars().all(|character| character.is_ascii_digit())
        || !matches!(unit, "day" | "days" | "week" | "weeks" | "month" | "months")
    {
        return Err(MollieError::invalid_request(format!(
            "invalid subscription interval `{value}`"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        CreateCaptureRequired, CreatePaymentLinkRequired, CreateRefundRequired,
        CreateSepaMandateRequired, CreateSubscriptionRequired,
    };
    use crate::{types, ApplicationFee, Money, WebhookUrl};

    #[test]
    /// Ensures refund builders do not accept response-owned fields.
    fn refund_builder_serializes_create_fields_only() {
        let request = CreateRefundRequired::new(Money::new("EUR", "10.00").unwrap(), "Refund")
            .unwrap()
            .into_request()
            .unwrap();
        let value = serde_json::to_value(request).unwrap();
        assert_eq!(value["description"], "Refund");
        assert!(value.get("id").is_none());
        assert!(value.get("createdAt").is_none());
        assert!(value.get("_links").is_none());
    }

    #[test]
    fn refund_builder_serializes_validated_routing_fields() {
        let reversal = types::EntityRefundRoutingReversalsItem {
            amount: Some(Money::new("EUR", "2.00").unwrap().into_amount()),
            source: None,
        };
        let request = CreateRefundRequired::new(Money::new("EUR", "10.00").unwrap(), "Refund")
            .unwrap()
            .with_routing_reversals(vec![reversal])
            .unwrap()
            .with_testmode(true)
            .into_request()
            .unwrap();
        let value = serde_json::to_value(request).unwrap();
        assert_eq!(value["routingReversals"][0]["amount"]["value"], "2.00");
        assert_eq!(value["testmode"], true);
    }

    #[test]
    fn refund_builder_rejects_conflicting_routing_options() {
        let reversal = types::EntityRefundRoutingReversalsItem::default();
        let error = CreateRefundRequired::new(Money::new("EUR", "10.00").unwrap(), "Refund")
            .unwrap()
            .with_routing_reversals(vec![reversal])
            .unwrap()
            .with_reverse_routing(true)
            .unwrap_err();
        assert!(error.to_string().contains("cannot be combined"));
    }

    #[test]
    /// Ensures subscription builders validate interval syntax and omit response data.
    fn subscription_builder_validates_interval() {
        assert!(CreateSubscriptionRequired::new(
            Money::new("EUR", "10.00").unwrap(),
            "Monthly",
            "1 month",
        )
        .is_ok());
        assert!(CreateSubscriptionRequired::new(
            Money::new("EUR", "10.00").unwrap(),
            "Monthly",
            "monthly",
        )
        .is_err());
    }

    #[test]
    fn subscription_builder_serializes_optional_create_fields() {
        let request = CreateSubscriptionRequired::new(
            Money::new("EUR", "10.00").unwrap(),
            "Monthly",
            "1 month",
        )
        .unwrap()
        .with_mandate("mdt_1234567890")
        .unwrap()
        .with_method("paypal")
        .unwrap()
        .with_times(12)
        .unwrap()
        .with_application_fee(
            ApplicationFee::new(Money::new("EUR", "1.00").unwrap(), "Platform fee").unwrap(),
        )
        .with_testmode(true)
        .into_request()
        .unwrap();
        let value = serde_json::to_value(request).unwrap();
        assert_eq!(value["mandateId"], "mdt_1234567890");
        assert_eq!(value["method"], "paypal");
        assert_eq!(value["times"], 12);
        assert_eq!(value["applicationFee"]["amount"]["value"], "1.00");
        assert_eq!(value["testmode"], true);
    }

    #[test]
    fn subscription_builder_rejects_invalid_optional_fields() {
        let builder = CreateSubscriptionRequired::new(
            Money::new("EUR", "10.00").unwrap(),
            "Monthly",
            "1 month",
        )
        .unwrap();
        assert!(builder.clone().with_mandate("cst_wrong_prefix").is_err());
        assert!(builder.clone().with_method("ideal").is_err());
        assert!(builder.with_times(0).is_err());
    }

    #[test]
    fn capture_builder_serializes_partial_amount_only() {
        let request = CreateCaptureRequired::partial(Money::new("EUR", "5.00").unwrap())
            .with_description("Partial capture")
            .unwrap()
            .into_request()
            .unwrap();
        let value = serde_json::to_value(request).unwrap();
        assert_eq!(value["amount"]["value"], "5.00");
        assert_eq!(value["description"], "Partial capture");
        assert!(value.get("id").is_none());
        assert!(value.get("status").is_none());
        assert!(value.get("paymentId").is_none());
    }

    #[test]
    fn capture_builder_full_omits_amount() {
        let request = CreateCaptureRequired::full().into_request().unwrap();
        let value = serde_json::to_value(request).unwrap();
        assert!(value.get("amount").is_none() || value["amount"].is_null());
    }

    #[test]
    fn sepa_mandate_builder_rejects_empty_name_and_bad_iban() {
        assert!(CreateSepaMandateRequired::new("", "NL91ABNA0417164300").is_err());
        assert!(CreateSepaMandateRequired::new("A. Holder", "short").is_err());
        assert!(CreateSepaMandateRequired::new("A. Holder", "1191ABNA0417164300").is_err());
    }

    #[test]
    fn sepa_mandate_builder_serializes_directdebit_fields() {
        let request = CreateSepaMandateRequired::new("A. Holder", "NL91 ABNA 0417 1643 00")
            .unwrap()
            .with_bic("ABNANL2A")
            .unwrap()
            .with_reference("REF-1")
            .into_request()
            .unwrap();
        let value = serde_json::to_value(request).unwrap();
        assert_eq!(value["method"], "directdebit");
        assert_eq!(value["consumerName"], "A. Holder");
        assert_eq!(value["consumerAccount"], "NL91 ABNA 0417 1643 00");
        assert_eq!(value["consumerBic"], "ABNANL2A");
        assert_eq!(value["mandateReference"], "REF-1");
        assert!(value.get("id").is_none());
        assert!(value.get("status").is_none());
    }

    #[test]
    fn payment_link_builder_serializes_description_and_amount() {
        let request = CreatePaymentLinkRequired::new("Invoice #42")
            .unwrap()
            .with_amount(Money::new("EUR", "19.99").unwrap())
            .with_redirect_url("https://example.com/done")
            .unwrap()
            .with_webhook_url(WebhookUrl::parse("https://example.com/hook").unwrap())
            .into_request()
            .unwrap();
        let value = serde_json::to_value(request).unwrap();
        assert_eq!(value["description"], "Invoice #42");
        assert_eq!(value["amount"]["value"], "19.99");
        assert_eq!(value["redirectUrl"], "https://example.com/done");
        assert_eq!(value["webhookUrl"], "https://example.com/hook");
    }

    #[test]
    fn payment_link_builder_rejects_non_http_redirect() {
        let error = CreatePaymentLinkRequired::new("Link")
            .unwrap()
            .with_redirect_url("ftp://example.com/x")
            .unwrap_err();
        assert!(error.to_string().contains("http"));
    }
}
