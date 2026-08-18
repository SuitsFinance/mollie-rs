//! Generated balances route methods.

use crate::{routes, types, Client, Error, ResponseValue};
use progenitor_client::encode_path;

/// Generated `balances` route methods on [`crate::Client`].
///
/// Client-owned request policy on [`crate::Client`]:
/// - idempotency via [`crate::Client::with_idempotency_key`] (default: UUID v4 per request)
/// - `testmode` query via [`crate::Client::with_testmode`] on operations that support it
///
/// The resolved idempotency key is returned on the response envelope
/// ([`crate::ResponseValue`] / [`crate::ResponseEnvelope`]).
#[allow(clippy::all)]
impl Client {
    /// List balances
    ///
    /// Retrieve a list of the organization's balances, including the primary balance.
    ///
    /// The results are paginated.
    ///
    /// Sends a `GET` request to `/balances`
    ///
    /// Arguments:
    /// - `currency`: Optionally only return balances with the given currency. For example: `EUR`.
    /// - `from`: Provide an ID to start the result set from the item with the given ID and onwards. This allows you to paginate the
    /// result set.
    /// - `limit`: The maximum number of items to return. Defaults to 50 items.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn list_balances<'a>(
        &'a self,
        currency: Option<&'a str>,
        from: Option<&'a str>,
        limit: Option<::std::num::NonZeroU64>,
    ) -> Result<ResponseValue<types::ListBalancesResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint("/balances");
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request
            .query(&progenitor_client::QueryParam::new("currency", &currency))
            .query(&progenitor_client::QueryParam::new("from", &from))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new(
                "testmode",
                &self.testmode(),
            ))
            .build()?;
        self.reject_testmode_for("list_balances")?;
        let response = self.send(request, routes::Operation::ListBalances).await?;
        routes::response::json(
            response,
            &[200u16],
            &[400u16, 404u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }

    /// Get balance
    ///
    /// When processing payments with Mollie, we put all pending funds — usually
    /// minus Mollie fees — on a balance. Once you have linked a bank account to your Mollie account, we can pay out your
    /// balance towards this bank account.
    ///
    /// With the Balances API you can retrieve your current balance. The response
    /// includes two amounts:
    ///
    ///  The *pending amount*. These are payments that have been marked as `paid`,
    /// but are not yet available on your balance.
    ///  The *available amount*. This is the amount that you can get paid out to
    /// your bank account, or use for refunds.
    ///
    /// With instant payment methods like iDEAL, payments are moved to the available
    /// balance instantly. With slower payment methods, like credit card for example, it can take a few days before the
    /// funds are available on your balance. These funds will be shown under the *pending amount* in the meanwhile.
    ///
    /// Sends a `GET` request to `/balances/{balanceId}`
    ///
    /// Arguments:
    /// - `balance_id`: Provide the ID of the related balance.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn get_balance<'a>(
        &'a self,
        balance_id: &'a types::BalanceToken,
    ) -> Result<ResponseValue<types::EntityBalance>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/balances/{}",
            encode_path(&balance_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request
            .query(&progenitor_client::QueryParam::new(
                "testmode",
                &self.testmode(),
            ))
            .build()?;
        self.reject_testmode_for("get_balance")?;
        let response = self.send(request, routes::Operation::GetBalance).await?;
        routes::response::json(
            response,
            &[200u16],
            &[404u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }

    /// Get primary balance
    ///
    /// Retrieve the primary balance. This is the balance of your account's primary
    /// currency, where all payments are settled to by default.
    ///
    /// This endpoint is a convenient alias of the [Get balance](get-balance)
    /// endpoint.
    ///
    /// Sends a `GET` request to `/balances/primary`
    ///
    /// Arguments:
    pub async fn get_primary_balance<'a>(
        &'a self,
    ) -> Result<ResponseValue<types::EntityBalance>, Error<types::ErrorResponse>> {
        let url = self.endpoint("/balances/primary");
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request.build()?;
        self.reject_testmode_for("get_primary_balance")?;
        let response = self
            .send(request, routes::Operation::GetPrimaryBalance)
            .await?;
        routes::response::json(
            response,
            &[200u16],
            &[429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }

    /// Get balance report
    ///
    /// Retrieve a summarized report for all transactions on a given balance within a given timeframe.
    ///
    /// The API also provides a detailed report on all 'prepayments' for Mollie fees that were deducted from your balance
    /// during the reported period, ahead of your Mollie invoice.
    ///
    /// The alias `primary` can be used instead of the balance ID to refer to the
    /// organization's primary balance.
    ///
    /// Sends a `GET` request to `/balances/{balanceId}/report`
    ///
    /// Arguments:
    /// - `balance_id`: Provide the ID of the related balance.
    /// - `from`: The start date of the report, in `YYYY-MM-DD` format. The from date is
    /// 'inclusive', and in Central European Time. This means a report with for example `from=2024-01-01` will
    /// include transactions from 2024-01-01 0:00:00 CET and onwards.
    /// - `grouping`: You can retrieve reports in two different formats. With the `status-balances` format, transactions are grouped
    /// by status (e.g. `pending`, `available`), then by transaction type, and then by other sub-groupings where
    /// available (e.g. payment method).
    ///
    /// With the `transaction-categories` format, transactions are grouped by
    /// transaction type, then by status, and then again by other sub-groupings where available.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    /// - `until`: The end date of the report, in `YYYY-MM-DD` format. The until date is 'exclusive', and in Central European Time.
    /// This means a report with for example `until=2024-02-01` will include transactions up until
    /// 2024-01-31 23:59:59 CET.
    pub async fn get_balance_report<'a>(
        &'a self,
        balance_id: &'a types::BalanceToken,
        from: &'a str,
        grouping: Option<types::BalanceReportGrouping>,
        until: &'a str,
    ) -> Result<ResponseValue<types::EntityBalanceReport>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/balances/{}/report",
            encode_path(&balance_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request
            .query(&progenitor_client::QueryParam::new("from", &from))
            .query(&progenitor_client::QueryParam::new("grouping", &grouping))
            .query(&progenitor_client::QueryParam::new(
                "testmode",
                &self.testmode(),
            ))
            .query(&progenitor_client::QueryParam::new("until", &until))
            .build()?;
        self.reject_testmode_for("get_balance_report")?;
        let response = self
            .send(request, routes::Operation::GetBalanceReport)
            .await?;
        routes::response::json(
            response,
            &[200u16],
            &[404u16, 422u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }

    /// List balance transactions
    ///
    /// Retrieve a list of all balance transactions. Transactions include for
    /// example payments, refunds, chargebacks, and settlements.
    ///
    /// For an aggregated report of these balance transactions, refer to the [Get
    /// balance report](get-balance-report) endpoint.
    ///
    /// The alias `primary` can be used instead of the balance ID to refer to the
    /// organization's primary balance.
    ///
    /// The results are paginated.
    ///
    /// Sends a `GET` request to `/balances/{balanceId}/transactions`
    ///
    /// Arguments:
    /// - `balance_id`: Provide the ID of the related balance.
    /// - `from`: Provide an ID to start the result set from the item with the given ID and onwards. This allows you to paginate the
    /// result set.
    /// - `limit`: The maximum number of items to return. Defaults to 50 items.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn list_balance_transactions<'a>(
        &'a self,
        balance_id: &'a types::BalanceToken,
        from: Option<&'a str>,
        limit: Option<::std::num::NonZeroU64>,
    ) -> Result<ResponseValue<types::ListBalanceTransactionsResponse>, Error<types::ErrorResponse>>
    {
        let url = self.endpoint(format_args!(
            "/balances/{}/transactions",
            encode_path(&balance_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request
            .query(&progenitor_client::QueryParam::new("from", &from))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new(
                "testmode",
                &self.testmode(),
            ))
            .build()?;
        self.reject_testmode_for("list_balance_transactions")?;
        let response = self
            .send(request, routes::Operation::ListBalanceTransactions)
            .await?;
        routes::response::json(
            response,
            &[200u16],
            &[400u16, 404u16, 429u16],
            &resolved_idempotency_key,
            self.response_limits(),
        )
        .await
    }
}
