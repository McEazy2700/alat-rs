//! Inbound webhooks — the generic ALAT callback your server must handle.
//!
//! Most money-movement and onboarding operations on ALAT are **asynchronous**:
//! the API call returns a *"Pending"* status, and the **final** outcome is later
//! delivered by Wema as an HTTP `POST` to a single **callback URL** you register
//! with them. You implement that endpoint; this module gives you the types to
//! deserialize it.
//!
//! > This is distinct from the **virtual-account** webhooks (name-enquiry &
//! > transaction-notify), whose types live in
//! > [`modules::virtual_account`](crate::modules::virtual_account). The callback
//! > here is the *generic* one used by wallet creation, account creation, PIN
//! > validation, and payment results.
//!
//! # The generic callback model
//!
//! Every callback has the same envelope — a title, a message, a **dynamic
//! `data`** object, and a **[`RequestType`]** discriminator that tells you what
//! `data` contains:
//!
//! ```jsonc
//! { "Title": "...", "Message": "...", "Data": { /* depends on RequestType */ }, "Request": 1 }
//! ```
//!
//! Build **one** endpoint, deserialize into [`Callback`], `match` on
//! [`Callback::request_type`], then decode [`Callback::data`] into the matching
//! payload with [`Callback::data_as`].
//!
//! ```no_run
//! use alat::modules::webhook::{Callback, RequestType, NubanData};
//!
//! fn handle(body: &str) -> Result<(), Box<dyn std::error::Error>> {
//!     let cb: Callback = serde_json::from_str(body)?;
//!     match cb.request_type {
//!         RequestType::WalletCreation | RequestType::AccountCreation => {
//!             let nuban: NubanData = cb.data_as()?;
//!             println!("created NUBAN {}", nuban.nuban.unwrap_or_default());
//!         }
//!         RequestType::PaymentResponse => { /* settle the payment */ }
//!         other => eprintln!("unhandled callback: {other:?}"),
//!     }
//!     Ok(())
//! }
//! ```
//!
//! > **Casing caveat:** different ALAT products send this envelope with slightly
//! > different key casing (`Title`/`Request` vs `title`/`requestType`). [`Callback`]
//! > accepts both via serde aliases, but always verify against a real callback in
//! > the sandbox during integration. **Never trust a callback blindly** — confirm
//! > the transaction independently (status query / statement) before releasing
//! > goods or value.

use crate::error::{Error, Result};
use serde::{Deserialize, Deserializer};

/// What a [`Callback`]'s `data` payload represents.
///
/// Encoded on the wire as an integer. Unknown values are preserved as
/// [`RequestType::Other`] so a new ALAT callback type never breaks deserialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestType {
    /// `1` — a wallet was created; `data` is a [`NubanData`].
    WalletCreation,
    /// `2` — an account was created (B2B onboarding); `data` is a [`NubanData`].
    AccountCreation,
    /// `3` — PIN validation result during a payment.
    PinValidation,
    /// `4` — final status (successful/failed) of a payment transaction.
    PaymentResponse,
    /// `5` — KYC ID card rejected; re-upload required; `data` is an [`InformationRejectedData`].
    IdCardUploadRequired,
    /// `6` — KYC selfie rejected; re-upload required; `data` is an [`InformationRejectedData`].
    SelfieUploadRequired,
    /// `7` — KYC signature rejected; re-upload required; `data` is an [`InformationRejectedData`].
    SignatureUploadRequired,
    /// `8` — address verification rejected; resubmit required; `data` is an [`InformationRejectedData`].
    AddressUploadRequired,
    /// Any other/forward-compatible value Wema may introduce.
    Other(u8),
}

impl From<u8> for RequestType {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::WalletCreation,
            2 => Self::AccountCreation,
            3 => Self::PinValidation,
            4 => Self::PaymentResponse,
            5 => Self::IdCardUploadRequired,
            6 => Self::SelfieUploadRequired,
            7 => Self::SignatureUploadRequired,
            8 => Self::AddressUploadRequired,
            n => Self::Other(n),
        }
    }
}

impl<'de> Deserialize<'de> for RequestType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from(u8::deserialize(deserializer)?))
    }
}

/// Whether a [`NubanData`] refers to a full account or a wallet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NubanType {
    /// `1` — a full bank account.
    Account,
    /// `2` — a wallet.
    Wallet,
    /// Any other/forward-compatible value.
    Other(u8),
}

impl From<u8> for NubanType {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Account,
            2 => Self::Wallet,
            n => Self::Other(n),
        }
    }
}

impl<'de> Deserialize<'de> for NubanType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from(u8::deserialize(deserializer)?))
    }
}

/// The generic ALAT callback envelope.
///
/// Field casing varies by product, so each field carries serde aliases for the
/// known variants. Inspect [`request_type`](Self::request_type), then decode
/// [`data`](Self::data) with [`data_as`](Self::data_as).
#[derive(Debug, Clone, Deserialize)]
pub struct Callback {
    /// Short title of the notification.
    #[serde(alias = "Title", alias = "title", default)]
    pub title: Option<String>,
    /// Human-readable message.
    #[serde(alias = "Message", alias = "message", default)]
    pub message: Option<String>,
    /// The dynamic payload, kept raw until you decode it with [`data_as`](Self::data_as).
    #[serde(alias = "Data", alias = "data", default)]
    pub data: serde_json::Value,
    /// Discriminator describing what [`data`](Self::data) contains.
    #[serde(alias = "Request", alias = "request", alias = "RequestType", alias = "requestType")]
    pub request_type: RequestType,
}

impl Callback {
    /// Decodes [`data`](Self::data) into the concrete payload for this callback's
    /// [`request_type`](Self::request_type) (e.g. [`NubanData`] for wallet/account
    /// creation, [`InformationRejectedData`] for KYC re-upload prompts).
    ///
    /// # Errors
    /// [`Error::Decode`] if `data` does not match `T`.
    pub fn data_as<T: serde::de::DeserializeOwned>(&self) -> Result<T> {
        serde_json::from_value(self.data.clone()).map_err(|e| Error::Decode {
            message: e.to_string(),
            body: self.data.to_string(),
        })
    }
}

/// The `data` payload for [`RequestType::WalletCreation`] / [`RequestType::AccountCreation`].
///
/// Carries the newly created NUBAN and the customer it belongs to. All fields are
/// optional to tolerate casing/availability differences across products.
#[derive(Debug, Clone, Deserialize)]
pub struct NubanData {
    /// The customer's id in ALAT's system.
    #[serde(alias = "CustomerID", alias = "customerID", alias = "customerId", default)]
    pub customer_id: Option<String>,
    /// The name registered on the NUBAN.
    #[serde(alias = "NUBANName", alias = "nubanName", default)]
    pub nuban_name: Option<String>,
    /// The 10-digit NUBAN that was created.
    #[serde(alias = "NUBAN", alias = "nuban", default)]
    pub nuban: Option<String>,
    /// The NUBAN's status (e.g. `"Active"`).
    #[serde(alias = "NUBANStatus", alias = "nubanStatus", default)]
    pub nuban_status: Option<String>,
    /// Whether the NUBAN is an account or a wallet.
    #[serde(alias = "NUBANType", alias = "nubanType", default)]
    pub nuban_type: Option<NubanType>,
    /// The customer's email.
    #[serde(alias = "Email", alias = "email", default)]
    pub email: Option<String>,
}

/// The `data` payload for the KYC re-upload callbacks
/// ([`RequestType::IdCardUploadRequired`] … [`RequestType::AddressUploadRequired`]).
#[derive(Debug, Clone, Deserialize)]
pub struct InformationRejectedData {
    /// The customer whose KYC artifact was rejected.
    #[serde(alias = "CustomerID", alias = "customerID", alias = "customerId", default)]
    pub customer_id: Option<String>,
    /// Why it was rejected / what to resubmit.
    #[serde(alias = "Reason", alias = "reason", default)]
    pub reason: Option<String>,
}
