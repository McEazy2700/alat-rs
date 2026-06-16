//! Feature modules.
//!
//! Each submodule defines the request/response types for one product area and
//! implements the corresponding endpoint methods as inherent methods on
//! [`Client`](crate::Client). Bringing that type into scope therefore makes all
//! endpoints available; the types live under their module ([`Client`](crate::Client))
//! (e.g. [`modules::transfer::TransferRequest`](transfer::TransferRequest)).

pub mod account;
pub mod bills;
pub mod statement;
pub mod transfer;
pub mod virtual_account;
pub mod wallet;
pub mod webhook;
