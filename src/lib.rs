//! # Wema ALAT Rust SDK
//!
//! An unofficial, asynchronous, type-safe Rust client for the **ALAT by Wema**
//! developer APIs. ALAT is the fully-digital bank by Wema Bank (Nigeria); its
//! APIM gateways let fintech partners open wallets, move money, and pay bills
//! programmatically.
//!
//! This crate is designed to double as **documentation**: every type and method
//! is annotated with both *what* it is (syntax) and *why/when* you use it
//! (semantics), and each endpoint method cites its exact HTTP path. All models
//! are derived from the live API schemas (see `MY_API_MAP.md` in the repo).
//!
//! ## Core ecosystem terminology
//!
//! - **NUBAN** — the standard 10-digit Nigerian bank account number. Every wallet
//!   and account in this SDK is identified by one.
//! - **BVN / NIN** — 11-digit identity anchors (Bank Verification Number /
//!   National Identification Number) used for KYC during wallet creation.
//! - **Name enquiry** — looking up the registered holder name for a NUBAN before
//!   sending money. Nigerian transfers are instant and irreversible, so this is a
//!   mandatory safety step.
//! - **NIBSS / NIP** — the national switch (NIBSS) and its instant-payments rail
//!   (NIP) that interbank transfers route through.
//!
//! ## Authentication
//!
//! APIM gates each product with a **subscription key**
//! (`Ocp-Apim-Subscription-Key`); ALAT identifies your software **channel** with
//! an **API key** (`x-api-key`). Bills/airtime additionally require an **access
//! key** (`access` header). Configure these on [`Config`].
//!
//! ## Gateways: one client per gateway
//!
//! The reference endpoints span two sandboxes with different hosts and keys:
//! funds transfer lives on the **APIM Dev** gateway, while wallet creation,
//! account, statement, bills, and airtime live on **Playground**. A [`Client`]
//! is bound to one gateway, so against the sandboxes you may need two clients.
//! In production, Wema typically fronts all your products behind a single
//! gateway host — pass it to [`Config::new`]. See [`config`] for details.
//!
//! ## Quickstart: verify a recipient before paying them
//!
//! ```no_run
//! use alat::{Client, Config};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), alat::Error> {
//!     // Funds transfer / name enquiry is on the APIM Dev gateway.
//!     let client = Client::new(Config::apim_dev("subscription_key", "api_key"))?;
//!
//!     // Name enquiry: confirm who owns 0123456789 at Wema (035) before sending.
//!     let beneficiary = client.verify_account("035", "0123456789", None).await?;
//!     println!("Verified recipient: {}", beneficiary.account_name);
//!     Ok(())
//! }
//! ```
//!
//! ## Error model
//!
//! Calls return [`Result<T>`]. Failures are one of: [`Error::Network`]
//! (transport), [`Error::Http`] (non-2xx), [`Error::Decode`] (schema mismatch),
//! [`Error::Api`] (the bank rejected an otherwise-valid request, with its own
//! message/code), or [`Error::Configuration`] / [`Error::Validation`]
//! (client-side). See [`error`].

pub mod client;
pub mod config;
pub mod envelope;
pub mod error;
pub mod modules;

// Ergonomic crate-root re-exports.
pub use client::Client;
pub use config::Config;
pub use envelope::{Acknowledgement, ApiResponse, Envelope, ServiceResponse, StatusResponse};
pub use error::{Error, Result};
