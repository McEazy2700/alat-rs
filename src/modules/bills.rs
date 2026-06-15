//! Bills payment & airtime/data — pay billers and top up phones.
//!
//! Two related products, both published on the Playground sandbox and both
//! gated by the **`access` header** (supply it via
//! [`Config::with_access_key`](crate::Config::with_access_key)):
//!
//! - **Bills Payment** (`/bills-payment`): list billers → validate the customer
//!   identifier → pay → (optionally) check transaction status.
//! - **Airtime & Data** (`/airtime-data`): list data plans → purchase airtime or
//!   data with a single client account.
//!
//! All of these are *asynchronous*: a successful call returns a **pending**
//! result and the final status is delivered to your callback URL. Use the
//! check-status calls to poll if needed.
//!
//! # Ecosystem concepts
//! - **Biller**: a merchant that accepts payments (power, TV, internet, etc.).
//! - **Package**: a specific payable product of a biller (e.g. a TV bouquet).
//! - **Customer validation**: confirming an identifier (meter no., smartcard,
//!   phone) exists at the biller before debiting the payer.

use crate::client::Client;
use crate::envelope::Envelope;
use crate::error::Result;
use serde::{Deserialize, Serialize};

// ===========================================================================
// Bills payment (/bills-payment) — gated by the `access` header
// ===========================================================================

/// A payable product offered by a biller.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BillPackage {
    /// Package id — pass this as `packageId` when validating/paying.
    pub id: i64,
    /// Id of the biller this package belongs to.
    pub biller_id: i64,
    /// Package display name.
    pub name: String,
    /// Whether the payer may choose the amount (vs. a fixed price).
    pub is_amount_editable: bool,
    /// Default/expected amount.
    pub amount: f64,
    /// Minimum payable amount (when editable).
    pub min_amount: f64,
    /// Maximum payable amount (when editable).
    pub max_amount: f64,
}

/// A biller within a category.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Biller {
    /// Biller id.
    pub id: i64,
    /// Biller display name.
    pub name: String,
    /// Stable identifier string for the biller.
    pub identifier: Option<String>,
    /// Short code for the biller.
    pub short_code: Option<String>,
    /// Whether the biller is currently acquired/active (API key `isAquired`).
    #[serde(rename = "isAquired")]
    pub is_acquired: bool,
    /// Whether the biller requires customer validation before payment.
    pub required_validation: bool,
    /// Flat charge associated with the biller.
    pub charge: f64,
    /// Payment-flow discriminator used by the platform.
    pub flow: i64,
    /// The biller's payable packages.
    #[serde(default)]
    pub packages: Vec<BillPackage>,
}

/// A bill category grouping billers (e.g. "Cable TV", "Electricity").
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BillCategory {
    /// Category id.
    pub id: i64,
    /// Category display name.
    pub name: String,
    /// Billers in this category.
    #[serde(default)]
    pub billers: Vec<Biller>,
}

/// Request body to validate a customer's identifier at a biller.
///
/// Server type: `ValidationRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateCustomerRequest {
    /// Your channel id, as issued by Wema.
    pub channel_id: String,
    /// The customer identifier to validate (meter no., smartcard, phone, …).
    pub identifier: String,
    /// The package being paid for (see [`BillPackage::id`]).
    pub package_id: i64,
}

/// Result of validating a customer identifier.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerValidation {
    /// Whether the identifier is valid at the biller.
    pub is_validated: bool,
    /// The resolved customer name, when available.
    pub customer_name: Option<String>,
    /// Additional message from the biller.
    pub message: Option<String>,
    /// Extra validation context (biller-specific).
    pub validation_info: Option<String>,
    /// Outstanding/credit limit, where the biller reports one.
    pub credit_limit: Option<f64>,
}

/// Request body to pay a bill from a client account.
///
/// Server type: `PayBillClientRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayBillRequest {
    /// Your channel/client id.
    pub client_id: String,
    /// The 10-digit NUBAN to debit.
    pub customer_account: String,
    /// Amount to pay.
    pub amount: f64,
    /// Service charge to apply.
    pub charge: f64,
    /// Your unique transaction reference (idempotency key).
    pub transaction_reference: String,
    /// The package being paid for (see [`BillPackage::id`]).
    pub package_id: i64,
    /// The customer identifier (meter no., smartcard, …).
    pub customer_identifier: String,
    /// Customer email (for receipts/notifications).
    pub customer_email: String,
    /// Customer phone number.
    pub customer_phone_number: String,
    /// Customer name.
    pub customer_name: String,
    /// Channel-encrypted security info, where required by your integration.
    pub security_info: String,
}

/// The `result` of a payment/purchase (`GlobalResponseEnvelope`).
///
/// Shared by bill payment and airtime/data purchase. Some fields appear only for
/// certain operations (e.g. [`value`](Self::value) for airtime), hence `Option`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentResult {
    /// Transaction status string (often a *pending* state on acceptance).
    pub status: Option<String>,
    /// Status message.
    pub message: Option<String>,
    /// Transaction narration.
    pub narration: Option<String>,
    /// Your transaction reference (echoed back).
    pub transaction_reference: Option<String>,
    /// The platform's transaction reference, for status queries.
    pub platform_transaction_reference: Option<String>,
    /// Settlement/switch transaction STAN.
    pub transaction_stan: Option<String>,
    /// Operation-specific value (e.g. token/units for airtime).
    pub value: Option<String>,
    /// Original transaction date (API key `orinalTxnTransactionDate`).
    #[serde(rename = "orinalTxnTransactionDate")]
    pub original_transaction_date: Option<String>,
}

/// Request body to check a bill transaction's status.
///
/// Server type: `CheckBillsTransactionStatusRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckTransactionStatusRequest {
    /// The transaction reference to query.
    pub transaction_reference: String,
}

/// The status of a previously submitted transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionStatus {
    /// The queried transaction reference.
    pub transaction_reference: Option<String>,
    /// Status code, encoded as an integer enum by the platform (e.g. pending /
    /// successful / failed). Map it according to your channel's documentation.
    pub transaction_status: i64,
}

// ===========================================================================
// Airtime & data (/airtime-data) — gated by the `access` header
// ===========================================================================

/// A single data package within a network's plan list.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataPackage {
    /// Package id — pass this as `packageCode` when purchasing.
    pub id: i64,
    /// Package display name.
    pub name: String,
    /// Package price.
    pub amount: f64,
    /// Data allowance description (e.g. `"5GB"`).
    pub data_plan: Option<String>,
    /// Validity period (API key `validity_Period`, e.g. `"30 days"`).
    #[serde(rename = "validity_Period")]
    pub validity_period: Option<String>,
    /// Whether this plan is enabled for Buy-Now-Pay-Later.
    pub enabled_for_bnpl: Option<bool>,
    /// Free-text description.
    pub description: Option<String>,
}

/// Data plans for a single network provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataPlanCategory {
    /// Category id.
    pub id: i64,
    /// Network provider name (e.g. `"MTN"`, `"Airtel"`, `"Glo"`, `"9mobile"`).
    pub network_provider: String,
    /// The available data packages for this provider.
    #[serde(default)]
    pub data_packages: Vec<DataPackage>,
}

/// Request body to purchase airtime from a single client account.
///
/// Server type: `AirtimeForClientReqModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseAirtimeRequest {
    /// Your unique transaction reference.
    pub transaction_reference: String,
    /// The 10-digit NUBAN to debit.
    pub account_number: String,
    /// The network operator (e.g. `"MTN"`).
    pub network: String,
    /// Recipient phone number to credit.
    pub phone_number: String,
    /// Airtime amount.
    pub amount: f64,
    /// Channel-encrypted security info, where required.
    pub security_info: String,
    /// Your channel/client id.
    pub client_id: String,
}

/// Request body to purchase a data bundle from a single client account.
///
/// Server type: `DataForClientReqModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseDataRequest {
    /// Your unique transaction reference.
    pub transaction_reference: String,
    /// The 10-digit NUBAN to debit.
    pub account_number: String,
    /// Recipient phone number to credit.
    pub phone_number: String,
    /// The data package code (see [`DataPackage::id`]).
    pub package_code: i64,
    /// The package amount.
    pub amount: f64,
    /// The network operator (e.g. `"MTN"`).
    pub network: String,
    /// Channel-encrypted security info, where required.
    pub security_info: String,
    /// Your channel/client id.
    pub client_id: String,
}

impl Client {
    /// Lists every bill category, biller, and package available to your channel.
    ///
    /// Requires the `access` header. `GET /bills-payment/api/BillsPayment/GetAllBills`
    pub async fn get_all_bills(&self) -> Result<Vec<BillCategory>> {
        let headers = self.access_headers(true)?;
        self.get_json::<Envelope<Vec<BillCategory>>>(
            "bills-payment/api/BillsPayment/GetAllBills",
            &[],
            &headers,
        )
        .await?
        .into_result()
    }

    /// Validates a customer identifier at a biller before payment.
    ///
    /// Requires the `access` header. `POST /bills-payment/api/BillsPayment/ValidateCustomer`
    pub async fn validate_customer(
        &self,
        request: &ValidateCustomerRequest,
    ) -> Result<CustomerValidation> {
        let headers = self.access_headers(true)?;
        self.post_json::<_, Envelope<CustomerValidation>>(
            "bills-payment/api/BillsPayment/ValidateCustomer",
            request,
            &headers,
        )
        .await?
        .into_result()
    }

    /// Pays a bill from a client account (returns a pending result).
    ///
    /// Requires the `access` header. `POST /bills-payment/api/Shared/PayBill`
    pub async fn pay_bill(&self, request: &PayBillRequest) -> Result<PaymentResult> {
        let headers = self.access_headers(true)?;
        self.post_json::<_, Envelope<PaymentResult>>(
            "bills-payment/api/Shared/PayBill",
            request,
            &headers,
        )
        .await?
        .into_result()
    }

    /// Checks the status of a previously submitted bill transaction.
    ///
    /// Requires the `access` header.
    /// `POST /bills-payment/api/PartnerPayment/checktransactionstatus`
    pub async fn check_bill_transaction_status(
        &self,
        request: &CheckTransactionStatusRequest,
    ) -> Result<TransactionStatus> {
        let headers = self.access_headers(true)?;
        self.post_json::<_, Envelope<TransactionStatus>>(
            "bills-payment/api/PartnerPayment/checktransactionstatus",
            request,
            &headers,
        )
        .await?
        .into_result()
    }

    /// Lists data plans across all networks.
    ///
    /// The `access` header is optional here but sent if configured.
    /// `GET /airtime-data/api/Data/GetDataPlans`
    pub async fn get_data_plans(&self) -> Result<Vec<DataPlanCategory>> {
        let headers = self.access_headers(false)?;
        self.get_json::<Envelope<Vec<DataPlanCategory>>>(
            "airtime-data/api/Data/GetDataPlans",
            &[],
            &headers,
        )
        .await?
        .into_result()
    }

    /// Purchases airtime from a single client account (returns a pending result).
    ///
    /// Requires the `access` header.
    /// `POST /airtime-data/api/Airtime/Client/PurchaseAirtime`
    pub async fn purchase_airtime(&self, request: &PurchaseAirtimeRequest) -> Result<PaymentResult> {
        let headers = self.access_headers(true)?;
        self.post_json::<_, Envelope<PaymentResult>>(
            "airtime-data/api/Airtime/Client/PurchaseAirtime",
            request,
            &headers,
        )
        .await?
        .into_result()
    }

    /// Purchases a data bundle from a single client account (returns a pending result).
    ///
    /// The `access` header is optional here but sent if configured.
    /// `POST /airtime-data/api/Data/Client/PurchaseData`
    pub async fn purchase_data(&self, request: &PurchaseDataRequest) -> Result<PaymentResult> {
        let headers = self.access_headers(false)?;
        self.post_json::<_, Envelope<PaymentResult>>(
            "airtime-data/api/Data/Client/PurchaseData",
            request,
            &headers,
        )
        .await?
        .into_result()
    }
}
