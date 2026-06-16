//! Virtual accounts — collect inbound payments with per-payment/per-user attribution.
//!
//! This module models Wema's **Virtual Account API** (`/VirtualAccount`,
//! published in the 🔒 approval-gated *Virtual Account* product on the **APIM
//! Dev** gateway — use [`Config::apim_dev`](crate::Config::apim_dev)).
//!
//! # The mental model (read this first)
//!
//! Virtual accounts here are **not** individually created at the bank. Instead:
//!
//! 1. You register **one prefix** (e.g. `"9988"`) with [`create_prefix`](crate::Client::create_prefix),
//!    pointing it at:
//!    - a real, debitable Wema account you own — the **`settle_account`** (every
//!      naira paid into any virtual account under the prefix lands here), and
//!    - two **webhook URLs you host** (`name_enquiry_uri`, `trans_notify_uri`).
//! 2. You **mint virtual account numbers yourself** under that prefix — a NUBAN is
//!    just `prefix + a suffix you choose` (see [`compose_virtual_account_number`]).
//!    There is no per-account API call.
//! 3. A payer sends money to one of those numbers from **any** bank (over NIP).
//!    Wema recognises the prefix, calls your **name-enquiry** webhook to resolve
//!    the holder name, credits your `settle_account`, then calls your
//!    **trans-notify** webhook with the credit details.
//! 4. You can also pull settled inflows on demand with
//!    [`query_transactions`](crate::Client::query_transactions).
//!
//! ```text
//!  any bank ──NIP──▶ 9988-00042 (you minted) ─┐
//!  any bank ──NIP──▶ 9988-00043 (you minted) ─┼─▶ settle_account (real Wema acct)
//!                                              ─┘    + webhook: TransNotify → you
//! ```
//!
//! # Attribution patterns
//!
//! - **Per-payment (dynamic):** mint a fresh number per invoice, show it, expect
//!   that payment, then recycle it. The number *is* the payment reference —
//!   exact attribution, no amount/time guessing. Best for one-off, fixed-amount
//!   checkouts.
//! - **Per-user (static):** one number per user forever. Each credit is still a
//!   distinct [`TransNotifyRequest`] with its own `session_id` and exact amount,
//!   so you do **not** need fuzzy amount/time matching — you simply credit that
//!   user's balance per notification. Best for recurring top-ups/wallets.
//!
//! Either way, every credit notification carries [`TransNotifyRequest::cr_account`]
//! (which virtual account was paid) plus the payer's account/bank — which is all
//! you need to attribute the payment and, later, to refund it.
//!
//! # Two webhooks YOU must implement
//!
//! Wema calls these on the URLs you register in the prefix; the SDK gives you the
//! request/response **types**, not a client method (you are the server here):
//!
//! - **Name enquiry** — receives [`AccountLookupRequest`], returns the holder name
//!   ([`AccountLookupResponse`]). Must resolve any number you have minted, so
//!   store the number→name mapping *before* you display the account to a payer.
//! - **Transaction notify** — receives [`TransNotifyRequest`] (the credit), returns
//!   [`TransNotifyResponse`] to acknowledge. **Dedupe on `session_id`** — webhooks
//!   can be redelivered.
//!
//! # Caveats
//! - A NUBAN is **10 digits**; the prefix consumes the leading digits, so plan the
//!   remaining suffix space (recycle dynamic numbers; bound per-user counts).
//! - The exact response contract for the name-enquiry webhook is **vendor-defined**
//!   and should be confirmed with Wema — see [`AccountLookupResponse`].

use crate::client::Client;
use crate::error::Result;
use serde::{Deserialize, Serialize};

// ===========================================================================
// Prefix configuration (client-side calls)
// ===========================================================================

/// Configuration for a virtual-account **prefix** — the unit Wema actually
/// registers (server type: `PrefixSetup`).
///
/// One prefix governs an entire range of virtual account numbers and routes all
/// of their inflows to a single settlement account, while delegating name
/// resolution and credit notification to your webhooks.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrefixSetup {
    /// A label for the prefix owner (your channel/merchant name).
    pub user_name: String,
    /// The leading digits shared by every virtual account in this range
    /// (e.g. `"9988"`). All numbers you mint must start with this.
    pub prefix: String,
    /// Account currency (e.g. `"NGN"`).
    pub currency: String,
    /// Base URL of your webhook host; the enquiry/notify URIs are resolved
    /// against it.
    pub base_url: String,
    /// Path/URL of your **name-enquiry** webhook (Wema → you). Receives
    /// [`AccountLookupRequest`].
    pub name_enquiry_uri: String,
    /// Path/URL of your **transaction-notify** webhook (Wema → you). Receives
    /// [`TransNotifyRequest`].
    pub trans_notify_uri: String,
    /// How Wema authenticates to your webhooks (scheme agreed with Wema).
    pub auth_type: String,
    /// The credential Wema presents to your webhooks under `auth_type`.
    pub auth_key: String,
    /// The **real, debitable Wema account** that all inflows to this prefix are
    /// credited into. This is the "central account" you later pay out from.
    pub settle_account: String,
    /// Whether the prefix is active (inflows are only routed while `true`).
    pub is_active: bool,
}

/// A settled inflow row returned by [`query_transactions`](Client::query_transactions)
/// (server type: the items of `TransQueryResponse.transactions`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualAccountTransaction {
    /// Payer's account number.
    #[serde(rename = "originatoraccountnumber")]
    pub originator_account_number: Option<String>,
    /// Amount credited (the API returns this as a string).
    pub amount: Option<String>,
    /// Payer's name.
    #[serde(rename = "originatorname")]
    pub originator_name: Option<String>,
    /// Transfer narration.
    pub narration: Option<String>,
    /// Name on the credited virtual account.
    #[serde(rename = "craccountname")]
    pub cr_account_name: Option<String>,
    /// Payment reference.
    #[serde(rename = "paymentreference")]
    pub payment_reference: Option<String>,
    /// Payer's bank name.
    #[serde(rename = "bankname")]
    pub bank_name: Option<String>,
    /// NIP session id (globally unique per credit — use as an idempotency key).
    #[serde(rename = "sessionid")]
    pub session_id: Option<String>,
    /// The **virtual account number** that was credited.
    #[serde(rename = "craccount")]
    pub cr_account: Option<String>,
    /// Payer's bank code.
    #[serde(rename = "bankcode")]
    pub bank_code: Option<String>,
    /// When the inflow was requested.
    #[serde(rename = "requestdate")]
    pub request_date: Option<String>,
    /// Raw NIBSS response.
    #[serde(rename = "nibssresponse")]
    pub nibss_response: Option<String>,
    /// Status of the onward send to your webhook.
    #[serde(rename = "sendstatus")]
    pub send_status: Option<String>,
    /// Response your webhook returned.
    #[serde(rename = "sendresponse")]
    pub send_response: Option<String>,
}

/// Request body to query settled inflows (server type: `TransQueryRequest`).
///
/// Acts as a filter — narrow by session id, credited virtual account, amount,
/// and/or transaction date depending on what you have.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransQueryRequest {
    /// NIP session id to look up (if known).
    #[serde(rename = "sessionid")]
    pub session_id: String,
    /// The virtual account number to filter by.
    #[serde(rename = "craccount")]
    pub cr_account: String,
    /// Amount to filter by.
    pub amount: f64,
    /// Transaction date to filter by.
    #[serde(rename = "txndate")]
    pub txn_date: String,
}

/// Response from [`query_transactions`](Client::query_transactions)
/// (server type: `TransQueryResponse`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransQueryResponse {
    /// Status code string for the query itself.
    pub status: Option<String>,
    /// Human-readable status description.
    pub status_desc: Option<String>,
    /// The matched inflows.
    #[serde(default)]
    pub transactions: Vec<VirtualAccountTransaction>,
}

// ===========================================================================
// Webhook payloads (YOU implement these endpoints — types only)
// ===========================================================================

/// Payload your **name-enquiry** webhook receives (server type:
/// `AccountLookupRequest`).
///
/// Wema sends this when a payer performs a name enquiry on one of your virtual
/// accounts. Resolve `account_number` to the holder name and reply with an
/// [`AccountLookupResponse`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountLookupRequest {
    /// The virtual account number being looked up.
    #[serde(rename = "accountnumber")]
    pub account_number: String,
    /// The bank code (Wema's) the lookup is scoped to.
    #[serde(rename = "bankcode")]
    pub bank_code: String,
}

/// Suggested reply from your **name-enquiry** webhook.
///
/// > ⚠️ The portal does not publish the exact response schema for this
/// > vendor-hosted endpoint — the only hard requirement is that you return the
/// > **account holder name**. This struct is a sensible, documented default;
/// > confirm the precise field names/shape with Wema during onboarding and adjust
/// > if needed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountLookupResponse {
    /// The resolved account holder name to display to the payer.
    #[serde(rename = "accountname")]
    pub account_name: String,
    /// Echo of the looked-up account number (optional).
    #[serde(rename = "accountnumber", skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    /// Optional status string (e.g. `"00"`/`"success"`), per your agreement with Wema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Payload your **transaction-notify** webhook receives when a virtual account is
/// credited (server type: `TransNotifyRequest`).
///
/// This single payload carries everything needed to (a) attribute the payment
/// via [`cr_account`](Self::cr_account) and (b) later refund the payer via
/// [`originator_account_number`](Self::originator_account_number) +
/// [`bank_code`](Self::bank_code). **Dedupe on [`session_id`](Self::session_id).**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransNotifyRequest {
    /// Payer's account number (use for refunds).
    #[serde(rename = "originatoraccountnumber")]
    pub originator_account_number: String,
    /// Amount credited (the API sends this as a string).
    pub amount: String,
    /// Currency (e.g. `"NGN"`).
    pub currency: String,
    /// Payer's name.
    #[serde(rename = "originatorname")]
    pub originator_name: String,
    /// Transfer narration.
    pub narration: String,
    /// Name on the credited virtual account.
    #[serde(rename = "craccountname")]
    pub cr_account_name: String,
    /// Payment reference.
    #[serde(rename = "paymentreference")]
    pub payment_reference: String,
    /// Secondary reference.
    pub reference: String,
    /// Payer's bank name.
    #[serde(rename = "bankname")]
    pub bank_name: String,
    /// NIP session id — globally unique per credit. **Use as the idempotency key.**
    #[serde(rename = "sessionid")]
    pub session_id: String,
    /// The **virtual account number that was credited** — your attribution key.
    #[serde(rename = "craccount")]
    pub cr_account: String,
    /// Payer's bank code (use for refunds).
    #[serde(rename = "bankcode")]
    pub bank_code: String,
    /// Timestamp the credit was created.
    pub created_at: String,
}

/// Acknowledgement your **transaction-notify** webhook returns (server type:
/// `TransNotifyResponse`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransNotifyResponse {
    /// Your reference for the acknowledgement (wire key `ref`).
    #[serde(rename = "ref")]
    pub ack_ref: String,
    /// The transaction reference you assign to the recorded credit.
    #[serde(rename = "transactionreference")]
    pub transaction_reference: String,
    /// Status code you return (e.g. `"00"` for accepted).
    pub status: String,
    /// Human-readable status description.
    pub status_desc: String,
}

/// Composes a virtual account number from a prefix and a suffix.
///
/// A NUBAN is 10 digits; this simply concatenates `prefix + suffix` (it does not
/// enforce length, since prefix length is assigned by Wema). Mint and **store**
/// the number before showing it to a payer, so your name-enquiry webhook can
/// resolve it.
///
/// ```
/// use alat::modules::virtual_account::compose_virtual_account_number;
/// assert_eq!(compose_virtual_account_number("9988", "000042"), "9988000042");
/// ```
pub fn compose_virtual_account_number(prefix: &str, suffix: &str) -> String {
    format!("{prefix}{suffix}")
}

impl Client {
    /// Registers a new virtual-account **prefix** (points it at your settlement
    /// account + webhooks). Returns the gateway's raw acknowledgement string.
    ///
    /// `POST /VirtualAccount/api/v1/Prefix/CreateNew`
    pub async fn create_prefix(&self, setup: &PrefixSetup) -> Result<String> {
        self.post_json::<_, String>("VirtualAccount/api/v1/Prefix/CreateNew", setup, &[])
            .await
    }

    /// Updates an existing prefix's configuration (e.g. rotate webhook URLs,
    /// toggle `is_active`). Returns the gateway's raw acknowledgement string.
    ///
    /// `POST /VirtualAccount/api/v1/Prefix/Modify`
    pub async fn modify_prefix(&self, setup: &PrefixSetup) -> Result<String> {
        self.post_json::<_, String>("VirtualAccount/api/v1/Prefix/Modify", setup, &[])
            .await
    }

    /// Lists all prefixes registered for your channel.
    ///
    /// `GET /VirtualAccount/api/v1/Prefix`
    pub async fn list_prefixes(&self) -> Result<Vec<PrefixSetup>> {
        self.get_json::<Vec<PrefixSetup>>("VirtualAccount/api/v1/Prefix", &[], &[])
            .await
    }

    /// Fetches the configuration of a single prefix.
    ///
    /// `GET /VirtualAccount/api/v1/Prefix/{prefix}`
    pub async fn get_prefix(&self, prefix: &str) -> Result<PrefixSetup> {
        let path = format!("VirtualAccount/api/v1/Prefix/{prefix}");
        self.get_json::<PrefixSetup>(&path, &[], &[]).await
    }

    /// Queries settled inflows (use to reconcile against your webhook records).
    ///
    /// `POST /VirtualAccount/api/v1/Trans/TransQuery`
    pub async fn query_transactions(
        &self,
        request: &TransQueryRequest,
    ) -> Result<TransQueryResponse> {
        self.post_json::<_, TransQueryResponse>("VirtualAccount/api/v1/Trans/TransQuery", request, &[])
            .await
    }
}
