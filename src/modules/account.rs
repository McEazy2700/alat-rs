//! Account maintenance — wallet balance/details and transaction history.
//!
//! These endpoints belong to the **Wallet Services – Account Management** product
//! (`/ws-acct-mgt`) and let you inspect wallets your channel manages: their
//! current balance/status, and a list of past transactions over a date range.
//!
//! # Ecosystem concepts
//! - **NUBAN**: the standard 10-digit Nigerian account number identifying a wallet.
//! - **Available vs. ledger balance**: the API exposes a single
//!   `availableBalance` here (the spendable amount). Note it is returned as a
//!   **string**, not a number — see [`WalletAccountDetails::available_balance`].

use crate::client::Client;
use crate::envelope::ServiceResponse;
use crate::error::Result;
use serde::{Deserialize, Serialize};

/// A wallet's identity, balance, and status.
///
/// This is the `result` object of `B2BGetAccountV2ReponseServiceResponse`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletAccountDetails {
    /// The 10-digit NUBAN of the wallet (the API names this `walletNumber`).
    pub wallet_number: String,
    /// The spendable balance. **Returned as a string by the API** (e.g.
    /// `"1500.00"`); parse it yourself if you need arithmetic, to avoid the
    /// rounding hazards of binary floating point on money.
    pub available_balance: String,
    /// The wallet's status (e.g. `"Active"`, `"Dormant"`, `"PND"`).
    pub wallet_status: String,
    /// The account/product type (e.g. `"Wallet"`).
    pub account_type: String,
}

/// Request body for the transaction-history lookup.
///
/// Server type: `TransactionhistoryV2Request`. The window is expressed with
/// `from`/`to` (the API does **not** offer page-number/page-size paging here).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionHistoryRequest {
    /// The 10-digit NUBAN whose history to fetch.
    pub account_number: String,
    /// Start of the date range. The API accepts a date/date-time string; use the
    /// format your channel was onboarded with (commonly `YYYY-MM-DD`).
    pub from: String,
    /// End of the date range (same format as [`from`](Self::from)).
    pub to: String,
    /// Optional free-text filter applied to the transactions (narration, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_word: Option<String>,
}

/// A single transaction row.
///
/// Shared by both the account-history (`transhistoryV2`) and statement
/// (`GetCustomerTransactions`) endpoints; fields absent from one of them are
/// modeled as `Option`. Several JSON keys carry the API's own spellings
/// (`recieverName`, etc.) and are mapped to clean Rust names via `serde(rename)`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    /// Short title/label for the transaction.
    pub title: Option<String>,
    /// Transaction amount.
    pub amount: f64,
    /// Channel/route of the transaction (e.g. `"InterBank"`, `"IntraBank"`).
    #[serde(rename = "type")]
    pub transaction_type: Option<String>,
    /// Display date string.
    pub date: Option<String>,
    /// Posting date/time (present on account history; absent on statements).
    pub transaction_date: Option<String>,
    /// Human-readable narration.
    pub narration: Option<String>,
    /// Transaction status (e.g. `"Default"`, `"Successful"`).
    pub status: Option<String>,
    /// Credit type classification (e.g. `"Default"`).
    pub credit_type: Option<String>,
    /// Sender's display name.
    pub sender: Option<String>,
    /// Sender's account number.
    pub sender_account_number: Option<String>,
    /// Destination bank name.
    pub destination_bank: Option<String>,
    /// Destination account number.
    pub destination_account_number: Option<String>,
    /// Receiver's display name (API key is the misspelled `recieverName`).
    #[serde(rename = "recieverName")]
    pub receiver_name: Option<String>,
    /// Reference id for the transaction.
    pub reference_id: Option<String>,
    /// Whether a receipt can be viewed for this transaction.
    pub is_view_receipt_enabled: Option<bool>,
    /// Core-banking transaction id.
    pub tran_id: Option<String>,
}

impl Client {
    /// Fetches a wallet's balance, status, and type by NUBAN.
    ///
    /// `GET /ws-acct-mgt/api/AccountMaintenance/CustomerAccount/GetAccountV2/accountNumber/{accountNumber}`
    pub async fn get_wallet_details(&self, account_number: &str) -> Result<WalletAccountDetails> {
        let path = format!(
            "ws-acct-mgt/api/AccountMaintenance/CustomerAccount/GetAccountV2/accountNumber/{}",
            account_number
        );
        self.get_json::<ServiceResponse<WalletAccountDetails>>(&path, &[], &[])
            .await?
            .into_result()
    }

    /// Fetches a wallet's transaction history over a date range.
    ///
    /// `POST /ws-acct-mgt/api/AccountMaintenance/CustomerAccount/transhistoryV2`
    pub async fn get_transaction_history(
        &self,
        request: &TransactionHistoryRequest,
    ) -> Result<Vec<Transaction>> {
        self.post_json::<_, ServiceResponse<Vec<Transaction>>>(
            "ws-acct-mgt/api/AccountMaintenance/CustomerAccount/transhistoryV2",
            request,
            &[],
        )
        .await?
        .into_result()
    }
}
