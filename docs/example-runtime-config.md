# Example Runtime Configuration

Every generated route example reads the shared `.env` file and accepts the same
configuration through environment variables or Clap flags. A flag takes
precedence over its environment variable.

For example:

```powershell
$env:BILLING_COUNTRY = "NL"
$env:CUSTOMER_ID = "cst_real_customer"
cargo run --example list_methods -- --billing-country DE
cargo run --example get_subscription -- --customer-id cst_real_customer --subscription-id sub_real_subscription
```

The common route values include `BILLING_COUNTRY`, `CUSTOMER_ID`,
`SUBSCRIPTION_ID`, `PAYMENT_ID`, `PAYMENT_LINK_ID`, `PROFILE_ID`,
`BALANCE_ID`, `CAPTURE_ID`, `CHARGEBACK_ID`, `MANDATE_ID`, `REFUND_ID`,
`SETTLEMENT_ID`, `TERMINAL_ID`, `PERMISSION_ID`, `ID`, `LIMIT`, `CURRENCY`,
`EMBED`, `INCLUDE`, `LOCALE`, `RESOURCE`, `SEQUENCE_TYPE`, `SORT`, `FROM`,
`MONTH`, `INVOICE_MONTH`, `YEAR`, `REFERENCE`, and `UNTIL`. `INVOICE_MONTH`
uses the Mollie invoice-month format (`01` through `12`), while `MONTH` is
used by routes whose API expects a year-month value. Each has a matching kebab-case
flag, such as `--payment-id` or `--billing-country`.

Leave `FROM` and `PROFILE_ID` empty unless you intentionally override them.
Pagination `from` cursors need a real object id from a previous page.
`PROFILE_ID` is for organization-level OAuth credentials: with an API key
Mollie rejects `profileId` on list endpoints (`422` validation error, field
`profileId`). Values must be real `pfl_*` ids, not payment or other tokens.

Request-body string fields that many create/update examples need are also
first-class: `EMAIL`, `NAME`, `DESCRIPTION`, `PHONE`, and `WEBSITE` (flags
`--email`, `--name`, `--description`, `--phone`, `--website`). These apply
on top of the generated body (or `--body-json` / `--body-file`), so you can
run customer/profile-style examples without hand-writing full JSON:

```powershell
cargo run --example create_customer -- --name "Ada Lovelace" --email ada@example.com
cargo run --example create_profile -- --email owner@example.com --name "Example Shop" --phone "+31201234567" --website "https://example.com"
cargo run --example create_payment -- --description "Order 123" --amount-currency EUR --amount-value 12.50
```

`LOCALE`, `CURRENCY`, `AMOUNT`, `REFERENCE`, and `SEQUENCE_TYPE` are also
merged into request bodies when set, in addition to their route-query uses.

Enum values can be supplied as their Mollie API value, for example
`--sort desc`, `--resource payments`, or `--sequence-type recurring`.
`AMOUNT`, `EVENT_TYPES`, and `ORDER_LINE_CATEGORIES` accept either a plain
generated value or JSON. `EXAMPLE_BODY_JSON` and `EXAMPLE_BODY_FILE` replace
the generated request body when a create/update route needs a real payload.

`MOLLIE_TESTMODE=true` (or `--testmode true`) applies the client’s sticky
`testmode` query only to routes that declare it in the OpenAPI contract. It
does not force every route into test mode, and live-only reporting routes fail
before HTTP dispatch when the sticky value is configured. Request-body
`testmode` fields are controlled by the generated request type. See
[`contracts/test-mode.md`](contracts/test-mode.md). Credentials remain
configured by `MOLLIE_API_KEY` or `MOLLIE_OAUTH_ACCESS_TOKEN`.

Unknown long options are also accepted as fixture overrides, so you can pass
route-specific values without waiting for a new shared field. For example:

```powershell
cargo run --example get_invoice -- --invoice-id inv_real_invoice
cargo run --example create_payment -- --amount-currency EUR --amount-value 12.50 --description "Order 123"
```

Hyphenated body fields are converted to their JSON field names. For example,
`--amount-currency` and `--amount-value` update the nested `amount` object.
The same arbitrary names can be provided as uppercase environment variables,
such as `AMOUNT_CURRENCY` and `AMOUNT_VALUE`.
