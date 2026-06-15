//! Funds transfer — move money to any Nigerian bank account.
//!
//! This is the **Funds Transfer OpenAPI** product (`/funds-transfer-open`),
//! published on the APIM Dev sandbox (see [`Config::apim_dev`](crate::Config::apim_dev)).
//! The safe transfer choreography is:
//!
//! 1. [`get_bank_list`](Client::get_bank_list) — resolve the destination bank's
//!    CBN/NIP code.
//! 2. [`verify_account`](Client::verify_account) — **name enquiry**: confirm the
//!    beneficiary name on the destination NUBAN *before* sending money. Nigerian
//!    transfers are instant and irreversible, so this step is not optional.
//! 3. [`transfer_funds`](Client::transfer_funds) — submit the (HMAC-signed)
//!    transfer. A successful call returns a *pending* result; the final outcome
//!    is delivered to your callback URL.
//!
//! # Security: the `hash` header
//!
//! Each transfer must carry an HMAC-SHA512 signature in the `hash` header,
//! computed with a secret salt key over the fields in this exact order:
//!
//! ```text
//! transactionReference + destinationBankCode + destinationAccountNumber + sourceAccountNumber + amount
//! ```
//!
//! [`transfer_funds`](Client::transfer_funds) computes and attaches it for you;
//! [`compute_transfer_hash`](Client::compute_transfer_hash) is exposed so you can
//! verify/reproduce it.

use crate::client::Client;
use crate::envelope::{Envelope, ServiceResponse};
use crate::error::{Error, Result};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Deserializer, Serialize};
use sha2::Sha512;

/// A bank in the NIBSS directory: its NIP/CBN routing code and name.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bank {
    /// The registered name of the financial institution.
    pub bank_name: String,
    /// The NIP/CBN routing code (e.g. `"058"` GTBank, `"057"` Zenith, `"035"` Wema).
    pub bank_code: String,
}

/// Wrapper that tolerates the API returning either a single bank object or an
/// array under `result`.
///
/// The portal's example for `GetAllBanks` shows a *single* object, but the
/// endpoint is logically a list; this accepts both so neither shape breaks.
#[derive(Debug, Clone)]
pub struct BankList(pub Vec<Bank>);

impl<'de> Deserialize<'de> for BankList {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum OneOrMany {
            Many(Vec<Bank>),
            One(Bank),
        }
        Ok(match OneOrMany::deserialize(deserializer)? {
            OneOrMany::Many(v) => BankList(v),
            OneOrMany::One(b) => BankList(vec![b]),
        })
    }
}

/// A single fee band returned alongside a name enquiry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargeFee {
    /// Fee band id.
    pub id: i64,
    /// Human-readable fee name.
    pub charge_fee_name: Option<String>,
    /// Transaction type the fee applies to (enum encoded as an integer).
    pub transaction_type: i64,
    /// The fee charged within this band.
    pub charge: f64,
    /// Lower bound of the amount band this fee applies to.
    pub lower: f64,
    /// Upper bound of the amount band this fee applies to.
    pub upper: f64,
}

/// The verified beneficiary returned by a name enquiry.
///
/// This is the `result` of `AccountNameEnquiryEnvelope`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountNameEnquiry {
    /// The destination bank's code (echoed back).
    pub bank_code: Option<String>,
    /// **The verified account holder name** — compare this against user intent.
    pub account_name: String,
    /// The queried account number (echoed back).
    pub account_number: Option<String>,
    /// Account currency (e.g. `"NGN"`).
    pub currency: Option<String>,
    /// Terms-and-conditions text, where applicable.
    pub terms_and_conditions: Option<String>,
    /// URL to the terms and conditions, where applicable.
    pub terms_and_conditions_url: Option<String>,
    /// Applicable interbank fee bands.
    #[serde(default)]
    pub charge_fee: Vec<ChargeFee>,
}

/// Request body for a transfer.
///
/// Server type: `OpenApiTransferRequest`. Every field except `narration` is
/// required by the bank. `destinationBankName` is part of the schema and **must**
/// be sent (the previous SDK omitted it).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferRequest {
    /// Amount to transfer, in Naira.
    ///
    /// Modeled as `f64` to match the API's JSON number. **Caveat:** this same
    /// value is stringified into the HMAC (`format!("{amount}")`), so its textual
    /// form must match what Wema signs server-side. Prefer whole-Naira amounts,
    /// and confirm the canonical amount format with Wema for production use.
    pub amount: f64,
    /// Narration / memo for the transfer.
    pub narration: String,
    /// Your unique client transaction reference (idempotency key — never reuse
    /// it on a retry; check status instead).
    pub transaction_reference: String,
    /// Destination bank's NIP/CBN code (from [`get_bank_list`](Client::get_bank_list)).
    pub destination_bank_code: String,
    /// Destination bank's name (from [`get_bank_list`](Client::get_bank_list)).
    pub destination_bank_name: String,
    /// Beneficiary's 10-digit NUBAN.
    pub destination_account_number: String,
    /// Beneficiary name, as confirmed by [`verify_account`](Client::verify_account).
    pub destination_account_name: String,
    /// The 10-digit NUBAN to debit.
    pub source_account_number: String,
}

/// The `result` of a successful transfer submission.
///
/// Server type: `OpenAPITransactionResponseOpenApiServiceResponse.result`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferResult {
    /// Status indicator. The API encodes this as a free-form JSON value (an
    /// object in the documented example), so it is preserved as raw JSON.
    #[serde(default)]
    pub status: serde_json::Value,
    /// Status message (typically reflects a *pending* state on acceptance).
    pub message: Option<String>,
    /// The platform's transaction reference, used to query final status.
    pub platform_transaction_reference: Option<String>,
}

impl Client {
    /// Computes the HMAC-SHA512 `hash` for a transfer, hex-encoded.
    ///
    /// Concatenation order (per ALAT docs):
    /// `transactionReference + destinationBankCode + destinationAccountNumber +
    /// sourceAccountNumber + amount`.
    ///
    /// # Errors
    /// [`Error::Validation`] if `salt_key` is empty; [`Error::Configuration`] if
    /// HMAC initialization fails.
    pub fn compute_transfer_hash(&self, salt_key: &str, request: &TransferRequest) -> Result<String> {
        if salt_key.is_empty() {
            return Err(Error::Validation("transfer salt key must not be empty".into()));
        }
        type HmacSha512 = Hmac<Sha512>;
        let data = format!(
            "{}{}{}{}{}",
            request.transaction_reference,
            request.destination_bank_code,
            request.destination_account_number,
            request.source_account_number,
            request.amount,
        );
        let mut mac = HmacSha512::new_from_slice(salt_key.as_bytes())
            .map_err(|e| Error::Configuration(format!("HMAC init failed: {e}")))?;
        mac.update(data.as_bytes());
        Ok(hex::encode(mac.finalize().into_bytes()))
    }

    /// Fetches every bank in the NIBSS directory (name + routing code).
    ///
    /// `GET /funds-transfer-open/api/OpenApiTransfer/GetAllBanks`
    pub async fn get_bank_list(&self) -> Result<Vec<Bank>> {
        let banks = self
            .get_json::<ServiceResponse<BankList>>(
                "funds-transfer-open/api/OpenApiTransfer/GetAllBanks",
                &[],
                &[],
            )
            .await?
            .into_result()?;
        Ok(banks.0)
    }

    /// **Name enquiry** — resolve the registered holder name for a NUBAN at a
    /// given bank. Always do this before [`transfer_funds`](Client::transfer_funds).
    ///
    /// `channel_id` is an optional query parameter some channels require.
    ///
    /// `GET /funds-transfer-open/api/Shared/AccountNameEnquiry/{bankCode}/{accountNumber}`
    pub async fn verify_account(
        &self,
        bank_code: &str,
        account_number: &str,
        channel_id: Option<&str>,
    ) -> Result<AccountNameEnquiry> {
        let path = format!(
            "funds-transfer-open/api/Shared/AccountNameEnquiry/{}/{}",
            bank_code, account_number
        );
        let query: Vec<(&str, &str)> = match channel_id {
            Some(c) => vec![("channelId", c)],
            None => vec![],
        };
        self.get_json::<Envelope<AccountNameEnquiry>>(&path, &query, &[])
            .await?
            .into_result()
    }

    /// Submits an interbank transfer, signing it with the `hash` header.
    ///
    /// Returns the pending [`TransferResult`]; the final status arrives via your
    /// callback URL (or poll the platform reference).
    ///
    /// `POST /funds-transfer-open/api/OpenApiTransfer/transfer-fund-request`
    pub async fn transfer_funds(
        &self,
        salt_key: &str,
        request: &TransferRequest,
    ) -> Result<TransferResult> {
        let hash = self.compute_transfer_hash(salt_key, request)?;
        self.post_json::<_, ServiceResponse<TransferResult>>(
            "funds-transfer-open/api/OpenApiTransfer/transfer-fund-request",
            request,
            &[("hash", hash)],
        )
        .await?
        .into_result()
    }
}
