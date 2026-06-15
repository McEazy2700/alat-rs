//! Error types for the Wema ALAT SDK.
//!
//! Every fallible SDK call returns [`Result<T>`], which is an alias for
//! `std::result::Result<T, Error>`. The [`enum@Error`] enum models the four
//! distinct ways a call can fail, so that callers can branch on the *kind* of
//! problem (e.g. retry on a transport error, surface a validation message to a
//! user, or alert on a bank-side rejection) rather than parsing strings.

use thiserror::Error;

/// All errors that can occur while using the Wema ALAT SDK.
///
/// The variants are ordered from "lowest level" (transport) to "highest level"
/// (a perfectly well-formed HTTP exchange in which the *bank* reported a
/// business failure inside the response envelope).
#[derive(Debug, Error)]
pub enum Error {
    /// A transport-level failure from the underlying HTTP client (`reqwest`).
    ///
    /// This means the request never produced a usable HTTP response: DNS could
    /// not be resolved, the connection timed out, TLS negotiation failed, etc.
    /// These are typically transient and safe to retry **for idempotent reads**
    /// (never blindly retry a transfer — use a status-check endpoint instead).
    #[error("network/transport error: {0}")]
    Network(#[from] reqwest::Error),

    /// The server returned a non-success HTTP status code (4xx/5xx).
    ///
    /// The gateway itself rejected the request before (or instead of) producing
    /// a business response — e.g. `401 Unauthorized` (bad subscription/API key),
    /// `403 Forbidden` (not subscribed to the product), `429 Too Many Requests`,
    /// or `500`. The raw response body is preserved verbatim for diagnostics.
    #[error("HTTP {status} from ALAT gateway: {body}")]
    Http {
        /// The HTTP status code returned by the Azure APIM gateway.
        status: reqwest::StatusCode,
        /// The raw, un-parsed response body (often a JSON or plain-text reason).
        body: String,
    },

    /// The HTTP call succeeded (2xx) but the JSON body could not be decoded into
    /// the expected Rust type.
    ///
    /// This usually signals a drift between this SDK's models and the live API
    /// schema. The offending body (truncated) is included so the mismatch can be
    /// diagnosed without re-running the request.
    #[error("failed to decode ALAT response: {message}\n--- body ---\n{body}")]
    Decode {
        /// The `serde_json` error message describing the mismatch.
        message: String,
        /// The response body that failed to decode (truncated for readability).
        body: String,
    },

    /// The bank reported a *business* failure inside an otherwise valid response.
    ///
    /// The HTTP exchange was fine (2xx, well-formed JSON), but the response
    /// envelope's success flag was negative (`hasError: true`, `successful:
    /// false`, or `status: false`). Examples: "Invalid BVN", "Insufficient
    /// funds", "Customer not found". The human-readable [`message`](Self::Api::message),
    /// machine [`code`](Self::Api::code), and any field-level [`errors`](Self::Api::errors)
    /// are surfaced directly from the envelope.
    #[error("ALAT API rejected the request: {message}{}", .code.as_deref().map(|c| format!(" (code: {c})")).unwrap_or_default())]
    Api {
        /// Human-readable message from the response envelope.
        message: String,
        /// Optional machine-readable error code (e.g. `"InvalidBvn"`).
        code: Option<String>,
        /// Optional list of granular, field-level error messages.
        errors: Vec<String>,
    },

    /// The SDK could not be configured or a request could not be constructed.
    ///
    /// Raised before any network I/O — e.g. a subscription/API key that contains
    /// bytes that are not valid in an HTTP header, or a required credential
    /// (such as the bills/airtime `access` key) that was not supplied.
    #[error("configuration error: {0}")]
    Configuration(String),

    /// Client-side validation rejected the input before a request was sent.
    ///
    /// Used for fail-fast guards that save a doomed round-trip, such as an empty
    /// salt key when signing a transfer.
    #[error("validation error: {0}")]
    Validation(String),
}

/// Convenience alias used by every fallible operation in this crate.
pub type Result<T> = std::result::Result<T, Error>;
