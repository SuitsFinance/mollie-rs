//! Generated accounts route methods.

use crate::{routes, types, Client, Error, ResponseValue};
use progenitor_client::encode_path;

/// Generated `accounts` route methods on [`crate::Client`].
///
/// Client-owned request policy on [`crate::Client`]:
/// - idempotency via [`crate::Client::with_idempotency_key`] (default: UUID v4 per request)
/// - `testmode` query via [`crate::Client::with_testmode`] on operations that support it
///
/// The resolved idempotency key is returned on the response envelope
/// ([`crate::ResponseValue`] / [`crate::ResponseEnvelope`]).
#[allow(clippy::all)]
impl Client {
    /// List business accounts
    ///
    /// Retrieve all business accounts for the authenticated organization.
    ///
    /// The results are paginated.
    ///
    /// Sends a `GET` request to `/business-accounts/accounts`
    ///
    /// Arguments:
    /// - `from`: Provide an ID to start the result set from the item with the given ID and onwards. This allows you to paginate
    /// the result set.
    /// - `limit`: The maximum number of items to return. Defaults to 50 items.
    /// - `sort`: Used for setting the direction of the result set. Defaults to descending order, meaning the results are ordered from
    /// newest to oldest.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn list_business_accounts<'a>(
        &'a self,
        from: Option<&'a types::BusinessAccountToken>,
        limit: Option<::std::num::NonZeroU64>,
        sort: Option<types::Sorting>,
    ) -> Result<ResponseValue<types::ListBusinessAccountsResponse>, Error<types::ErrorResponse>>
    {
        let url = self.endpoint("/business-accounts/accounts");
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request
            .query(&progenitor_client::QueryParam::new("from", &from))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("sort", &sort))
            .query(&progenitor_client::QueryParam::new(
                "testmode",
                &self.testmode(),
            ))
            .build()?;
        self.reject_testmode_for("list_business_accounts")?;
        let response = self
            .send(request, routes::Operation::ListBusinessAccounts)
            .await?;
        routes::response::json(
            response,
            &[200u16],
            &[400u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Get business account
    ///
    /// Retrieve a single business account object by its account ID. This allows you to check the current status,
    /// balance, and account details.
    ///
    /// Sends a `GET` request to `/business-accounts/accounts/{businessAccountId}`
    ///
    /// Arguments:
    /// - `business_account_id`: Provide the ID of the related business account.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn get_business_account<'a>(
        &'a self,
        business_account_id: &'a types::BusinessAccountToken,
    ) -> Result<ResponseValue<types::BusinessAccountResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/business-accounts/accounts/{}",
            encode_path(&business_account_id.to_string())
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
        self.reject_testmode_for("get_business_account")?;
        let response = self
            .send(request, routes::Operation::GetBusinessAccount)
            .await?;
        routes::response::json(
            response,
            &[200u16],
            &[404u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// List transactions
    ///
    /// Retrieve all transactions for a specific business account.
    ///
    /// The results are paginated.
    ///
    /// Sends a `GET` request to `/business-accounts/accounts/{businessAccountId}/transactions`
    ///
    /// Arguments:
    /// - `business_account_id`: Provide the ID of the related business account.
    /// - `from`: Provide an ID to start the result set from the item with the given ID and onwards. This allows you to paginate
    /// the result set.
    /// - `limit`: The maximum number of items to return. Defaults to 50 items.
    /// - `sort`: Used for setting the direction of the result set. Defaults to descending order, meaning the results are ordered from
    /// newest to oldest.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn list_business_account_transactions<'a>(
        &'a self,
        business_account_id: &'a types::BusinessAccountToken,
        from: Option<&'a types::BusinessAccountTransactionToken>,
        limit: Option<::std::num::NonZeroU64>,
        sort: Option<types::Sorting>,
    ) -> Result<
        ResponseValue<types::ListBusinessAccountTransactionsResponse>,
        Error<types::ErrorResponse>,
    > {
        let url = self.endpoint(format_args!(
            "/business-accounts/accounts/{}/transactions",
            encode_path(&business_account_id.to_string())
        ));
        #[allow(unused_mut)]
        let (request, resolved_idempotency_key) = self.request(::reqwest::Method::GET, url)?;
        #[allow(unused_mut)]
        let mut request = request
            .query(&progenitor_client::QueryParam::new("from", &from))
            .query(&progenitor_client::QueryParam::new("limit", &limit))
            .query(&progenitor_client::QueryParam::new("sort", &sort))
            .query(&progenitor_client::QueryParam::new(
                "testmode",
                &self.testmode(),
            ))
            .build()?;
        self.reject_testmode_for("list_business_account_transactions")?;
        let response = self
            .send(request, routes::Operation::ListBusinessAccountTransactions)
            .await?;
        routes::response::json(
            response,
            &[200u16],
            &[400u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }

    /// Get transaction
    ///
    /// Retrieve a single transaction object by its transaction ID. This allows you to check the details,
    /// amount, counterparty, and balance impact of a specific transaction.
    ///
    /// Sends a `GET` request to `/business-accounts/accounts/{businessAccountId}/transactions/{transactionId}`
    ///
    /// Arguments:
    /// - `business_account_id`: Provide the ID of the related business account.
    /// - `transaction_id`: Provide the ID of the related transaction.
    ///
    /// Test entities cannot be retrieved when the endpoint is set to live mode, and vice versa.
    pub async fn get_business_account_transaction<'a>(
        &'a self,
        business_account_id: &'a types::BusinessAccountToken,
        transaction_id: &'a types::BusinessAccountTransactionToken,
    ) -> Result<ResponseValue<types::TransactionResponse>, Error<types::ErrorResponse>> {
        let url = self.endpoint(format_args!(
            "/business-accounts/accounts/{}/transactions/{}",
            encode_path(&business_account_id.to_string()),
            encode_path(&transaction_id.to_string())
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
        self.reject_testmode_for("get_business_account_transaction")?;
        let response = self
            .send(request, routes::Operation::GetBusinessAccountTransaction)
            .await?;
        routes::response::json(
            response,
            &[200u16],
            &[404u16, 429u16],
            &resolved_idempotency_key,
        )
        .await
    }
}
