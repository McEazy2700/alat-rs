//! Account statements — retrieve a customer's transaction statement.
//!
//! The **Get Statement** product (`/get-statement-service`) generates a
//! statement asynchronously in two steps:
//!
//! 1. **Initiate** — register the request for an account over a date range. The
//!    bank returns a `referenceId` once the statement is prepared.
//! 2. **Retrieve** — exchange that `referenceId` for the actual transaction rows.
//!
//! The transaction rows reuse the shared [`Transaction`] type from the account
//! module.

use crate::client::Client;
use crate::envelope::ApiResponse;
use crate::error::Result;
use crate::modules::account::Transaction;
use serde::{Deserialize, Serialize};

/// Request body to initiate a statement.
///
/// Server type: `InitiateGetCustomerStatementRequest`. The date fields are named
/// `dateFrom`/`dateTo` and accept ISO-8601 date-times (e.g.
/// `2022-09-07T09:27:13.133Z`).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitiateStatementRequest {
    /// The 10-digit NUBAN to produce a statement for.
    pub account_number: String,
    /// Inclusive start of the statement window (ISO-8601 date-time).
    pub date_from: String,
    /// Inclusive end of the statement window (ISO-8601 date-time).
    pub date_to: String,
}

/// The `data` object returned by `InitiateGetCustomerStatement`: just the
/// reference id used to retrieve the prepared statement.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StatementReference {
    reference_id: String,
}

/// Request body to retrieve a prepared statement.
///
/// Server type: `GetCustomerTransactionsRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetStatementRequest {
    /// The `referenceId` returned by [`initiate_statement`](Client::initiate_statement).
    pub reference_id: String,
}

impl Client {
    /// **Step 1** — initiate statement generation; returns the `referenceId` to
    /// pass to [`get_statement_transactions`](Client::get_statement_transactions).
    ///
    /// `POST /get-statement-service/api/AccountMaintenance/InitiateGetCustomerStatement`
    pub async fn initiate_statement(&self, request: &InitiateStatementRequest) -> Result<String> {
        let reference = self
            .post_json::<_, ApiResponse<StatementReference>>(
                "get-statement-service/api/AccountMaintenance/InitiateGetCustomerStatement",
                request,
                &[],
            )
            .await?
            .into_result()?;
        Ok(reference.reference_id)
    }

    /// **Step 2** — retrieve the prepared statement's transaction rows.
    ///
    /// `POST /get-statement-service/api/AccountMaintenance/GetCustomerTransactions`
    pub async fn get_statement_transactions(
        &self,
        request: &GetStatementRequest,
    ) -> Result<Vec<Transaction>> {
        self.post_json::<_, ApiResponse<Vec<Transaction>>>(
            "get-statement-service/api/AccountMaintenance/GetCustomerTransactions",
            request,
            &[],
        )
        .await?
        .into_result()
    }
}
