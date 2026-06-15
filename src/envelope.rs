//! Response envelopes used across the ALAT API surface.
//!
//! Every ALAT endpoint wraps its payload in one of **three** envelope shapes.
//! Modeling them generically here — rather than re-inventing a flat struct per
//! endpoint — is what lets the SDK actually deserialize live responses, and it
//! gives every call a uniform way to distinguish "the bank said no" (a business
//! failure carried *inside* a `200 OK`) from transport/HTTP errors.
//!
//! | Envelope            | Shape (success flag)                                            | Used by |
//! |---------------------|-----------------------------------------------------------------|---------|
//! | [`ServiceResponse`] | `{ result, successful, message }`                               | account, transaction history, funds transfer |
//! | [`Envelope`]        | `{ result, errorMessage, errorMessages, hasError, timeGenerated }` | name enquiry, bills, airtime |
//! | [`ApiResponse`]     | `{ data, status, message, code, statusCode, errors }`           | wallet creation, statement (with payload) |
//! | [`StatusResponse`]  | `{ status, message, code, statusCode, errors }` (no payload)    | wallet creation steps, debit restriction |
//!
//! Each wrapper exposes `into_result()` which returns the inner payload on
//! success or an [`Error::Api`] carrying the bank's own message/code on failure.

use crate::error::{Error, Result};
use serde::Deserialize;

/// The `ServiceResponse<T>` / `*Envelope` shape that uses a boolean
/// [`successful`](Self::successful) flag.
///
/// Returned by account maintenance (`GetAccountV2`, `transhistoryV2`) and the
/// funds-transfer product (`GetAllBanks`, `transfer-fund-request`). On the wire:
/// `{ "result": <T>, "successful": true, "message": "..." }`.
#[derive(Debug, Clone, Deserialize)]
pub struct ServiceResponse<T> {
    /// The business payload. `None` when the call failed or returned no body.
    #[serde(default = "none")]
    pub result: Option<T>,
    /// Whether the operation succeeded. This is the authoritative success flag.
    #[serde(default)]
    pub successful: bool,
    /// Optional human-readable status/diagnostic message.
    #[serde(default)]
    pub message: Option<String>,
}

impl<T> ServiceResponse<T> {
    /// Unwraps to the inner payload, or an [`Error::Api`] if `successful` is
    /// `false` (or the payload is missing on an otherwise "successful" reply).
    pub fn into_result(self) -> Result<T> {
        if self.successful {
            self.result.ok_or_else(|| Error::Api {
                message: self
                    .message
                    .unwrap_or_else(|| "successful response carried no `result` payload".into()),
                code: None,
                errors: Vec::new(),
            })
        } else {
            Err(Error::Api {
                message: self.message.unwrap_or_else(|| "request was not successful".into()),
                code: None,
                errors: Vec::new(),
            })
        }
    }
}

/// The `*Envelope` shape that reports failure via [`has_error`](Self::has_error)
/// plus error message lists.
///
/// Returned by name enquiry, bills, and airtime. On the wire:
/// `{ "result": <T>, "errorMessage": "...", "errorMessages": ["..."],
/// "hasError": false, "timeGenerated": "..." }`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Envelope<T> {
    /// The business payload. `None` when the call failed.
    #[serde(default = "none")]
    pub result: Option<T>,
    /// A single summary error message (present when `has_error` is true).
    #[serde(default)]
    pub error_message: Option<String>,
    /// Granular, field-level error messages.
    #[serde(default)]
    pub error_messages: Vec<String>,
    /// Authoritative failure flag: `true` means the bank rejected the request.
    #[serde(default)]
    pub has_error: bool,
    /// Server timestamp the response was generated (ISO-8601), if provided.
    #[serde(default)]
    pub time_generated: Option<String>,
}

impl<T> Envelope<T> {
    /// Unwraps to the inner payload, or an [`Error::Api`] when `has_error` is
    /// `true` (or the payload is missing).
    pub fn into_result(self) -> Result<T> {
        if self.has_error {
            let message = self
                .error_message
                .filter(|m| !m.is_empty())
                .or_else(|| self.error_messages.first().cloned())
                .unwrap_or_else(|| "request returned an error".into());
            return Err(Error::Api {
                message,
                code: None,
                errors: self.error_messages,
            });
        }
        self.result.ok_or_else(|| Error::Api {
            message: "response reported no error but carried no `result` payload".into(),
            code: None,
            errors: Vec::new(),
        })
    }
}

/// The `ResponseModel` shape (with a payload) that reports success via a boolean
/// [`status`](Self::status) and nests data under `data`.
///
/// Returned by statement and the wallet-creation account-details lookup. On the
/// wire: `{ "data": <T>, "status": true, "message": "...", "code": "...",
/// "statusCode": "...", "errors": [...] }`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T> {
    /// The business payload, nested under `data`. `None` on failure.
    #[serde(default = "none")]
    pub data: Option<T>,
    /// Authoritative success flag.
    #[serde(default)]
    pub status: bool,
    /// Human-readable message.
    #[serde(default)]
    pub message: Option<String>,
    /// Machine-readable result code (e.g. `"InvalidBvn"`, `"Success"`).
    #[serde(default)]
    pub code: Option<String>,
    /// A secondary status code string (e.g. `"Continue"`).
    #[serde(default)]
    pub status_code: Option<String>,
    /// Field-level error messages, when present.
    #[serde(default)]
    pub errors: Vec<String>,
}

impl<T> ApiResponse<T> {
    /// Unwraps to the inner `data`, or an [`Error::Api`] when `status` is
    /// `false` (or `data` is missing).
    pub fn into_result(self) -> Result<T> {
        if self.status {
            self.data.ok_or_else(|| Error::Api {
                message: self
                    .message
                    .unwrap_or_else(|| "successful response carried no `data` payload".into()),
                code: self.code,
                errors: self.errors,
            })
        } else {
            Err(Error::Api {
                message: self.message.unwrap_or_else(|| "request was not successful".into()),
                code: self.code,
                errors: self.errors,
            })
        }
    }
}

/// The `ResponseModel` shape **without** a payload — a pure acknowledgement.
///
/// Returned by the wallet-creation steps (BVN/NIN onboarding, OTP validation)
/// and debit-restriction management, which report only success + a message. On
/// the wire: `{ "status": true, "message": "...", "code": "...",
/// "statusCode": "...", "errors": [...] }`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusResponse {
    /// Authoritative success flag.
    #[serde(default)]
    pub status: bool,
    /// Human-readable message describing the outcome.
    #[serde(default)]
    pub message: Option<String>,
    /// Machine-readable result code (e.g. `"InvalidBvn"`).
    #[serde(default)]
    pub code: Option<String>,
    /// A secondary status code string (e.g. `"Continue"`).
    #[serde(default)]
    pub status_code: Option<String>,
    /// Field-level error messages, when present.
    #[serde(default)]
    pub errors: Vec<String>,
}

impl StatusResponse {
    /// Converts into an [`Acknowledgement`] on success, or an [`Error::Api`] on
    /// failure (`status == false`).
    pub fn into_result(self) -> Result<Acknowledgement> {
        if self.status {
            Ok(Acknowledgement {
                message: self.message,
                code: self.code,
            })
        } else {
            Err(Error::Api {
                message: self.message.unwrap_or_else(|| "request was not successful".into()),
                code: self.code,
                errors: self.errors,
            })
        }
    }
}

/// A successful, payload-less outcome from an ALAT operation.
///
/// Returned by "fire and acknowledge" calls — onboarding steps, OTP validation,
/// debit-restriction changes — where the bank confirms acceptance but returns
/// no resource. The asynchronous result (e.g. the generated NUBAN) arrives later
/// via your registered callback URL.
#[derive(Debug, Clone)]
pub struct Acknowledgement {
    /// The bank's confirmation message, if any.
    pub message: Option<String>,
    /// The bank's result code, if any (e.g. `"Success"`).
    pub code: Option<String>,
}

/// serde default helper: `Option<T>` defaults to `None` without requiring
/// `T: Default`.
fn none<T>() -> Option<T> {
    None
}
