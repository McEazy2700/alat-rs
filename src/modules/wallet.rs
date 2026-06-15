//! Wallet creation & onboarding — open Tier-1 ALAT wallets programmatically.
//!
//! ALAT lets partners onboard customers into **Tier-1 wallets** using either a
//! **BVN** or a **NIN** as the identity anchor. The two flows are mirror images
//! of each other and live on two different products/base paths:
//!
//! | Flow | Product (base path)      | Identity field |
//! |------|--------------------------|----------------|
//! | BVN  | `/account-creation`      | `bvn`          |
//! | NIN  | `/wallet-creation`       | `nin`          |
//!
//! Both follow the same 3-step choreography:
//!
//! 1. **Initiate** — submit identity + phone + email. The bank sends a one-time
//!    password (OTP) to the customer's registered phone and returns an
//!    acknowledgement (the `trackingId` you pass to step 2 is issued out-of-band
//!    / via your callback in the live flow).
//! 2. **Validate OTP** — submit `phone + otp + trackingId`. On success the wallet
//!    is enqueued for creation; the resulting NUBAN arrives via your callback URL
//!    (`Request = 1 WalletCreation`).
//! 3. **Fetch details** — once created, look the account up by phone number.
//!
//! Helpers are provided for **resending the OTP** and for **debit-restriction
//! (PND) management** on the created wallet.
//!
//! # Ecosystem concepts
//! - **Tier-1 wallet**: a low-KYC account with CBN-mandated balance/transaction
//!   limits, openable purely digitally.
//! - **BVN / NIN**: 11-digit identifiers (Bank Verification Number / National
//!   Identification Number) used as the KYC anchor.
//! - **OTP**: a one-time password sent to the customer's phone to prove consent.
//! - **PND ("Post No Debit")**: a restriction that blocks debits from an account.

use crate::client::Client;
use crate::envelope::{Acknowledgement, ApiResponse, StatusResponse};
use crate::error::Result;
use serde::{Deserialize, Serialize};

// ===========================================================================
// Request payloads
// ===========================================================================

/// Step 1 (BVN) — request body for `PostPartnershipAccountCreationWithBvn`.
///
/// Server type: `PatnershipRequestWithBvn`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWalletWithBvnRequest {
    /// Customer's phone number, as registered with the BVN.
    pub phone_number: String,
    /// Customer's email address (used for profiling/notifications).
    pub email: String,
    /// The customer's 11-digit Bank Verification Number.
    pub bvn: String,
}

/// Step 1 (NIN) — request body for `GenerateWalletAccountForPartnerships/Request`.
///
/// Server type: `PatnershipRequestV3`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateWalletWithNinRequest {
    /// Customer's phone number, as registered with the NIN.
    pub phone_number: String,
    /// Customer's email address.
    pub email: String,
    /// The customer's 11-digit National Identification Number.
    pub nin: String,
}

/// Step 2 (BVN & NIN) — request body for OTP validation.
///
/// Server type: `PatnershipRequestV2`. Shared verbatim by both flows.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateOtpRequest {
    /// Customer's phone number (the one the OTP was sent to).
    pub phone_number: String,
    /// The one-time password the customer received.
    pub otp: String,
    /// The tracking id correlating this validation with the step-1 request.
    pub tracking_id: String,
}

/// Request body to re-send the onboarding OTP.
///
/// Server type: `ResendOtpModel`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResendOtpRequest {
    /// The tracking id from the step-1 response.
    pub tracking_id: String,
    /// The customer's phone number to re-send the OTP to.
    pub phone_number: String,
}

/// Request body for debit-restriction (PND) management.
///
/// Server type: `PndRequest`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebitRestrictionRequest {
    /// The restriction action. The portal documents `"LiftPnd"` to remove a
    /// restriction; the place-restriction value is the symmetric counterpart
    /// (commonly `"PlacePnd"`). Confirm the exact tokens with Wema for your
    /// channel before relying on placement.
    pub pnd_type: String,
    /// The 10-digit NUBAN to apply the restriction change to.
    pub account_number: String,
}

// ===========================================================================
// Response payloads
// ===========================================================================

/// The created wallet's details, returned by `GetPartnershipAccountDetails`.
///
/// This is the `data` object inside the `ResponseModel` envelope. Note the API
/// returns the customer name as separate `firstName`/`lastName` fields rather
/// than a single display name.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartnershipAccountDetails {
    /// The generated 10-digit NUBAN account number.
    pub account_number: String,
    /// Customer's first name.
    pub first_name: String,
    /// Customer's last name.
    pub last_name: String,
    /// Customer's email address.
    pub email: String,
    /// Customer's phone number.
    pub phone_number: String,
}

impl Client {
    // ---- BVN flow (base path: /account-creation) --------------------------

    /// **BVN · Step 1** — initiate wallet creation with a BVN.
    ///
    /// Triggers an OTP to the customer's phone. The returned [`Acknowledgement`]
    /// confirms acceptance; proceed to [`validate_bvn_otp`](Client::validate_bvn_otp).
    ///
    /// `POST /account-creation/api/CustomerAccount/PostPartnershipAccountCreationWithBvn`
    pub async fn create_wallet_with_bvn(
        &self,
        request: &CreateWalletWithBvnRequest,
    ) -> Result<Acknowledgement> {
        self.post_json::<_, StatusResponse>(
            "account-creation/api/CustomerAccount/PostPartnershipAccountCreationWithBvn",
            request,
            &[],
        )
        .await?
        .into_result()
    }

    /// **BVN · Step 2** — validate the OTP and enqueue wallet creation.
    ///
    /// On success the wallet is queued; the NUBAN is delivered to your callback
    /// URL. Then fetch it with
    /// [`get_bvn_partnership_account_details`](Client::get_bvn_partnership_account_details).
    ///
    /// `POST /account-creation/api/CustomerAccount/ValidateBVNandEnqueueAccountCreation`
    pub async fn validate_bvn_otp(&self, request: &ValidateOtpRequest) -> Result<Acknowledgement> {
        self.post_json::<_, StatusResponse>(
            "account-creation/api/CustomerAccount/ValidateBVNandEnqueueAccountCreation",
            request,
            &[],
        )
        .await?
        .into_result()
    }

    /// **BVN** — re-send the onboarding OTP.
    ///
    /// `POST /account-creation/api/CustomerAccount/ResendOtpRequest/ResendOtp`
    pub async fn resend_bvn_otp(&self, request: &ResendOtpRequest) -> Result<Acknowledgement> {
        self.post_json::<_, StatusResponse>(
            "account-creation/api/CustomerAccount/ResendOtpRequest/ResendOtp",
            request,
            &[],
        )
        .await?
        .into_result()
    }

    /// **BVN · Step 3** — fetch the created wallet's details by phone number.
    ///
    /// `GET /account-creation/api/CustomerAccount/GetPartnershipAccountDetails?phoneNumber=...`
    pub async fn get_bvn_partnership_account_details(
        &self,
        phone_number: &str,
    ) -> Result<PartnershipAccountDetails> {
        self.get_json::<ApiResponse<PartnershipAccountDetails>>(
            "account-creation/api/CustomerAccount/GetPartnershipAccountDetails",
            &[("phoneNumber", phone_number)],
            &[],
        )
        .await?
        .into_result()
    }

    /// **BVN** — place or lift a debit restriction (PND) on the wallet.
    ///
    /// `POST /account-creation/api/CustomerAccount/PartnerDebitRestrictionManagement`
    pub async fn set_bvn_debit_restriction(
        &self,
        request: &DebitRestrictionRequest,
    ) -> Result<Acknowledgement> {
        self.post_json::<_, StatusResponse>(
            "account-creation/api/CustomerAccount/PartnerDebitRestrictionManagement",
            request,
            &[],
        )
        .await?
        .into_result()
    }

    // ---- NIN flow (base path: /wallet-creation) ---------------------------

    /// **NIN · Step 1** — initiate wallet creation with a NIN.
    ///
    /// `POST /wallet-creation/api/CustomerAccount/GenerateWalletAccountForPartnerships/Request`
    pub async fn create_wallet_with_nin(
        &self,
        request: &CreateWalletWithNinRequest,
    ) -> Result<Acknowledgement> {
        self.post_json::<_, StatusResponse>(
            "wallet-creation/api/CustomerAccount/GenerateWalletAccountForPartnerships/Request",
            request,
            &[],
        )
        .await?
        .into_result()
    }

    /// **NIN · Step 2** — validate the OTP and enqueue wallet creation.
    ///
    /// `POST /wallet-creation/api/CustomerAccount/GenerateWalletAccountForPartnershipsV2/Otp`
    pub async fn validate_nin_otp(&self, request: &ValidateOtpRequest) -> Result<Acknowledgement> {
        self.post_json::<_, StatusResponse>(
            "wallet-creation/api/CustomerAccount/GenerateWalletAccountForPartnershipsV2/Otp",
            request,
            &[],
        )
        .await?
        .into_result()
    }

    /// **NIN** — re-send the onboarding OTP.
    ///
    /// `POST /wallet-creation/api/CustomerAccount/ResendOtpRequest/ResendOtp`
    pub async fn resend_nin_otp(&self, request: &ResendOtpRequest) -> Result<Acknowledgement> {
        self.post_json::<_, StatusResponse>(
            "wallet-creation/api/CustomerAccount/ResendOtpRequest/ResendOtp",
            request,
            &[],
        )
        .await?
        .into_result()
    }

    /// **NIN · Step 3** — fetch the created wallet's details by phone number.
    ///
    /// `GET /wallet-creation/api/CustomerAccount/GetPartnershipAccountDetails?phoneNumber=...`
    pub async fn get_nin_partnership_account_details(
        &self,
        phone_number: &str,
    ) -> Result<PartnershipAccountDetails> {
        self.get_json::<ApiResponse<PartnershipAccountDetails>>(
            "wallet-creation/api/CustomerAccount/GetPartnershipAccountDetails",
            &[("phoneNumber", phone_number)],
            &[],
        )
        .await?
        .into_result()
    }

    /// **NIN** — place or lift a debit restriction (PND) on the wallet.
    ///
    /// `POST /wallet-creation/api/CustomerAccount/PartnerDebitRestrictionManagement`
    pub async fn set_nin_debit_restriction(
        &self,
        request: &DebitRestrictionRequest,
    ) -> Result<Acknowledgement> {
        self.post_json::<_, StatusResponse>(
            "wallet-creation/api/CustomerAccount/PartnerDebitRestrictionManagement",
            request,
            &[],
        )
        .await?
        .into_result()
    }
}
