# Wema ALAT Rust SDK — and a field guide to the ALAT APIs

An **unofficial**, asynchronous, type-safe Rust SDK for the [ALAT by Wema](https://www.alat.ng/)
developer APIs — and, just as importantly, **the map of the ALAT API that Wema's own docs don't give
you in one place**.

ALAT's APIs are spread across two separate developer portals, grouped into ~30 products, and rendered
through a JavaScript console that hides the request/response schemas. This repository flattens all of
that into:

- a **type-safe Rust client** with correct paths, payloads, and response models (this crate);
- a complete, schema-level **API map** ([`MY_API_MAP.md`](./MY_API_MAP.md)) covering **all 55 API
  groups / 342 endpoints** across both portals;
- this README, a **plain-English guide** to how the API is structured, **which keys you need, where to
  get them, and which product to subscribe to for which endpoint**.

> Everything here was derived directly from the live ALAT APIM developer portals. This project is
> **not affiliated with or endorsed by Wema Bank**.

---

## Contents

1. [How the ALAT API is structured](#1-how-the-alat-api-is-structured)
2. [The two portals: Playground vs. APIM Dev](#2-the-two-portals-playground-vs-apim-dev)
3. [Authentication: the four credentials](#3-authentication-the-four-credentials)
4. [Getting your keys, step by step](#4-getting-your-keys-step-by-step)
5. [Product & endpoint catalog](#5-product--endpoint-catalog)
6. [How responses are shaped (envelopes & the callback model)](#6-how-responses-are-shaped-envelopes--the-callback-model)
7. [Financial & ecosystem concepts](#7-financial--ecosystem-concepts)
8. [Using this SDK](#8-using-this-sdk)
9. [What this SDK covers](#9-what-this-sdk-covers)
10. [Error handling](#10-error-handling)
11. [Full schema reference](#11-full-schema-reference)

---

## 1. How the ALAT API is structured

ALAT runs on **Microsoft Azure API Management (APIM)**. Three concepts explain almost everything:

- **Gateway** — a single host that receives every API call, e.g. `https://playground.azure-api.net`.
  ⚠️ This is **not** the sign-up portal host (`playground.alat.ng`) — calls to the portal host 404.
- **API group** — a related set of endpoints mounted under a **path prefix**, e.g. the funds-transfer
  group lives under `/funds-transfer-open`. Every request is therefore
  `https://<gateway>/<path-prefix>/<operation>`.
- **Product** — a *bundle of API groups* that you **subscribe** to. Subscribing yields a
  **subscription key**, and that one key authorizes **every API group inside that product** (and
  nothing outside it). Products are the unit of access — not individual endpoints.

```
https://playground.azure-api.net / bills-payment / api/BillsPayment/GetAllBills
└───────── gateway ────────────┘  └─ path prefix ┘ └──────── operation ────────┘
                              (an API group inside the "Wallet Services" product)
```

So the practical workflow is: **pick the endpoint → find which product contains its API group →
subscribe to that product → use the resulting subscription key.** Section 5 is the lookup table for
that.

---

## 2. The two portals: Playground vs. APIM Dev

ALAT publishes **two independent APIM instances**. They have different gateways, different keys, and
**different product catalogs** — a key issued on one is useless on the other.

| | **Playground** | **APIM Dev** |
| :-- | :-- | :-- |
| Sign-up / keys portal | `https://playground.alat.ng` | `https://wema-alatdev-apimgt.developer.azure-api.net` |
| API gateway (calls go here) | `https://playground.azure-api.net` | `https://wema-alatdev-apimgt.azure-api.net` |
| Focus | partner wallet ecosystem | open-banking / merchant ecosystem |
| Products | 10 | 22 |
| `Config` constructor | `Config::playground(...)` | `Config::apim_dev(...)` |

> ⚠️ On **Playground** the portal and the gateway are the **same host**. On **APIM Dev** they
> **differ** — you sign up at `…developer.azure-api.net` but your code calls
> `…azure-api.net` (no `developer.`). The SDK's `Config::apim_dev(...)` already targets the gateway.

Because a [`Client`] is bound to one gateway + key, spanning both portals means more than one client.
In **production**, Wema typically fronts all of *your* subscribed products behind a single gateway host
they issue you — pass it to `Config::new(base_url, sub_key, api_key)`.

---

## 3. Authentication: the four credentials

Depending on the endpoint, a call carries up to four secrets. Crucially, **only the first is
self-serve** — the rest are issued by Wema when they onboard your integration "channel":

| Credential | Sent as | Scope | How to obtain |
| :-- | :-- | :-- | :-- |
| **Subscription key** | `Ocp-Apim-Subscription-Key` header | per **product** | **Self-serve**: subscribe to a product on the portal (Section 4). |
| **Channel API key** | `x-api-key` header | per **channel** (constant) | Issued by **Wema** when your channel is registered. |
| **Access key** | `access` header | bills & airtime only | Issued by **Wema** for the bills/airtime product. |
| **Transfer salt** | used to compute the `hash` header (HMAC-SHA512) | per **channel** | Issued by **Wema**; used to sign transfers. |

What you can do with each:

- **Subscription key only** → call endpoints that need nothing else, e.g. **`GetAllBanks`** and
  **name enquiry** on the funds-transfer product. Great for a first end-to-end smoke test.
- **+ channel `x-api-key`** → wallet creation, account, statement, and most write operations.
- **+ `access`** → bills payment and airtime/data.
- **+ salt** → submitting transfers (the SDK computes the `hash` for you).

The SDK validates these at construction and injects them per request; see Section 8.

---

## 4. Getting your keys, step by step

### A. Self-serve: a subscription key (do this now)

1. Open the **developer portal** for the gateway you need (Section 2) and click **Sign up**; confirm
   your email and **Sign in**.
2. Go to **Products** and open the product that contains your target API group (use the catalog in
   Section 5). Most products show **"Subscribe"** and are granted instantly (a few are marked
   🔒 *approval required* and are reviewed by Wema).
3. After subscribing, open your **Profile** — your subscription lists a **Primary** and **Secondary**
   key. Either works as `Ocp-Apim-Subscription-Key`.

Repeat per product/portal you need (Section 9 lists what this SDK uses).

### B. Wema-issued: channel API key, access key, transfer salt

These identify and secure *your* integration and are **not** available from the portal UI. Request them
from Wema's developer/partner team — the portal's published contact is **help@alat.ng**, and there's a
**Contact/Support** link once you're signed in. Tell them which products you've subscribed to and that
you need sandbox **channel credentials** (`x-api-key`, the bills/airtime `access` key, and the
funds-transfer **salt**).

> **Sandbox conveniences** (per Wema's docs): the development OTP is **`123456`**, and onboarding/
> payment flows return a **"Pending"** status immediately, with the final result delivered to your
> **callback URL** (Section 6).

---

## 5. Product & endpoint catalog

The lookup table promised in Section 1: every product on each portal, the API groups (path prefixes) it
unlocks, and the endpoint count. Endpoints used by **this SDK** are tagged. For per-operation request/
response **schemas**, see [`MY_API_MAP.md`](./MY_API_MAP.md).

### Playground

- **Developer portal (sign up / get keys):** `https://playground.alat.ng`
- **API gateway (what the SDK calls):** `https://playground.azure-api.net`
- **10 products · 20 distinct API groups · 82 endpoints** (some groups appear in more than one product)

| Product (subscribe to this) | API groups it unlocks | Endpoints |
| :-- | :-- | :--: |
| **Account Creation - Address Verification** | `/bills-payment` · `/card-management` · `/credit-wallet` · `/debit-wallet` · `/partnership-address-verification` · `/ws-acct-mgt` | 33 |
| **Buy Now Pay Later Service** | `/alat-bnpl` | 5 |
| **Direct Debit Service - Scheduled Payments** | `/merchant-direct-debit` | 4 |
| **Get Statement Service** | `/get-statement-service` | 2 |
| **Pay with Bank Account** | `/pwba-authenticator` | 2 |
| **Remita Payment** | *(no APIs currently published)* | 0 |
| **Term-Deposit-Credit-Card** | `/credit-check` · `/credit-wallet` · `/debit-wallet` · `/partnership-address-verification` · `/fdcreditcard` · `/ws-acct-mgt` | 21 |
| **Verify Discount Code - Merchants** | `/merchant-verify-discount-code` | 1 |
| **Virtual Naira Card** | *(no APIs currently published)* | 0 |
| **Wallet Services** | `/account-upgrade` · `/airtime-data` · `/bills-payment` · `/card-management` · `/credit-wallet` · `/debit-wallet` · `/create-account-face` · `/create-account` · `/partnership-address-verification` · `/remita-payment` · `/account-creation` · `/wallet-creation` · `/ws-acct-mgt` | 64 |

<details><summary>Full API-group breakdown for Playground (paths · purpose · #endpoints)</summary>

**Account Creation - Address Verification**  
_This API requires the bank to perform physical address verification for account upgrade._  
- `Bills Payment API` → `/bills-payment` · 6 endpoints  — **✅ bills in this SDK**
- `Card Management API` → `/card-management` · 10 endpoints
- `Credit Wallet API` → `/credit-wallet` · 3 endpoints
- `Debit Wallet API` → `/debit-wallet` · 6 endpoints
- `Partnership Account - with Address Verification` → `/partnership-address-verification` · 6 endpoints
- `Wallet Services - Account Management API` → `/ws-acct-mgt` · 2 endpoints  — **✅ account in this SDK**

**Buy Now Pay Later Service**  
_Customers can now access short-term financing that allows them to make purchases and pay for them over time._  
- `Buy-Now-Pay-Later Service` → `/alat-bnpl` · 5 endpoints

**Direct Debit Service - Scheduled Payments**  
_Create one-time and recurring payment schedules on WEMA/ALAT accounts._  
- `Direct Debit Service - Merchants` → `/merchant-direct-debit` · 4 endpoints

**Get Statement Service**  
_Use transaction statement for informed analytics._  
- `Get Statement API` → `/get-statement-service` · 2 endpoints  — **✅ statement in this SDK**

**Pay with Bank Account**  
_API to make payments using a Wema/ALAT bank account number._  
- `Pay with Bank Account - ALAT Authenticator` → `/pwba-authenticator` · 2 endpoints

**Remita Payment**  
_Pay your Remita bills on the ALAT Playground._  
- *(no APIs currently published under this product)*

**Term-Deposit-Credit-Card**  
_APIs to issue credit cards backed by term deposits._  
- `Credit Check API` → `/credit-check` · 1 endpoints
- `Credit Wallet API` → `/credit-wallet` · 3 endpoints
- `Debit Wallet API` → `/debit-wallet` · 6 endpoints
- `Partnership Account - with Address Verification` → `/partnership-address-verification` · 6 endpoints
- `Term-Deposit-Backed Credit Card Issuance Service` → `/fdcreditcard` · 3 endpoints
- `Wallet Services - Account Management API` → `/ws-acct-mgt` · 2 endpoints  — **✅ account in this SDK**

**Verify Discount Code - Merchants**  
_Merchants who have offered discounts on ALAT Rewards can programmatically verify discount codes presented by customers for redemption._  
- `VerifyDiscountCode - Merchant` → `/merchant-verify-discount-code` · 1 endpoints

**Virtual Naira Card**  
_API to request and manage Virtual Naira cards_  
- *(no APIs currently published under this product)*

**Wallet Services**  
_APIs to help businesses build a robust ecosystem for creating and maintaining wallets._  
- `Account Upgrade API` → `/account-upgrade` · 3 endpoints
- `Airtime and Data API` → `/airtime-data` · 6 endpoints  — **✅ airtime in this SDK**
- `Bills Payment API` → `/bills-payment` · 6 endpoints  — **✅ bills in this SDK**
- `Card Management API` → `/card-management` · 10 endpoints
- `Credit Wallet API` → `/credit-wallet` · 3 endpoints
- `Debit Wallet API` → `/debit-wallet` · 6 endpoints
- `Partnership Account - Face Biometric Authentication` → `/create-account-face` · 5 endpoints
- `Partnership Account - KYC` → `/create-account` · 4 endpoints
- `Partnership Account - with Address Verification` → `/partnership-address-verification` · 6 endpoints
- `Remita-Payment API` → `/remita-payment` · 3 endpoints
- `Wallet Creation API - BVN` → `/account-creation` · 5 endpoints  — **✅ wallet (BVN) in this SDK**
- `Wallet Creation API - NIN` → `/wallet-creation` · 5 endpoints  — **✅ wallet (NIN) in this SDK**
- `Wallet Services - Account Management API` → `/ws-acct-mgt` · 2 endpoints  — **✅ account in this SDK**

</details>

### APIM Dev

- **Developer portal (sign up / get keys):** `https://wema-alatdev-apimgt.developer.azure-api.net`
- **API gateway (what the SDK calls):** `https://wema-alatdev-apimgt.azure-api.net`
- **22 products · 33 distinct API groups · 258 endpoints** (some groups appear in more than one product)

| Product (subscribe to this) | API groups it unlocks | Endpoints |
| :-- | :-- | :--: |
| **ALAT Faith** | `/alat-faith-tems` · `/alatfaith` | 6 |
| **ALAT Pay** | `/alat-pay` | 3 |
| **Account Generation by Microsite** | `/microsite-acct` | 33 |
| **Cardless Transactions (Coralpay)** | `/cardlesstrans` | 3 |
| **Cardless Transactions (PayAttitude)** | `/cardlesspayattitude` | 7 |
| **Clickatell** | `/airtime-int-clickatell` | 4 |
| **Consortium - PTA Product** | `/consortium-acct-creation` · `/consortium-acct-mgt` · `/consortium-payment` · `/consortium-card` | 11 |
| **Customer Identification Service** | `/customer-identification` | 2 |
| **Digital SME Loan** | `/smeloan` | 14 |
| **Fresh Desk** | `/freddy` | 3 |
| **Gaming Product** | `/alat-wallet` · `/merchant-payout` | 28 |
| **Get Transaction Statement** | `/get-statement` | 2 |
| **HQ Demo Cards** 🔒*(approval required)* | `/alat-card-v2` | 80 |
| **Merchant - Payout API** | `/merchant-payout` | 10 |
| **Money Guard Connect** | `/moneyguardconnect` | 3 |
| **Onboarding - Open API** | `/account-maintenance` · `/onboarding-wallets` · `/onboarding-open` · `/callback-url` | 16 |
| **Open Banking APIs** | `/account-maintenance` · `/airtime` · `/open-bills` · `/funds-transfer-open` | 12 |
| **Outlet Link Pay** | `/outlet-link` | 15 |
| **Partnership Accounts** | `/account-maintenance` · `/funds-transfer` · `/partnership-account` | 7 |
| **Payout API** | `/payout` · `/wallet-transfer` | 7 |
| **VerifyDiscountCode** | `/verify-discount-code` | 1 |
| **Virtual Account** 🔒*(approval required)* | `/VirtualAccount` | 7 |

<details><summary>Full API-group breakdown for APIM Dev (paths · purpose · #endpoints)</summary>

**ALAT Faith**  
_API to aid collections for religious organizations_  
- `ALAT Faith - Tems APIs` → `/alat-faith-tems` · 1 endpoints
- `ALAT Faith APIs` → `/alatfaith` · 5 endpoints

**ALAT Pay**  
_API for e-commerce payments using a Wema Bank account number._  
- `ALAT Pay E-Commerce Transfers` → `/alat-pay` · 3 endpoints

**Account Generation by Microsite**  
_APIs to support acquisition drive by special initiatives and programs._  
- `Microsite_Service.API` → `/microsite-acct` · 33 endpoints

**Cardless Transactions (Coralpay)**  
_API for cardless transactions._  
- `Cardless Transaction Platform API (Coralpay)` → `/cardlesstrans` · 3 endpoints

**Cardless Transactions (PayAttitude)**  
_APIs for cardless transactions_  
- `Cardless Transaction Platform API (PayAttitude)` → `/cardlesspayattitude` · 7 endpoints

**Clickatell**  
_For Clickatell integration_  
- `Airtime API Clickatell` → `/airtime-int-clickatell` · 4 endpoints

**Consortium - PTA Product**  
_This product offers PTAs to customers for transactions on physical or virtual cards._  
- `Consortium Account Creation API` → `/consortium-acct-creation` · 1 endpoints
- `Consortium Account Management API` → `/consortium-acct-mgt` · 2 endpoints
- `Consortium Accounts Payment API` → `/consortium-payment` · 3 endpoints
- `Consortium Card Management` → `/consortium-card` · 5 endpoints

**Customer Identification Service**  
_This API allows channels to validate WEMA/ALAT customer's identity._  
- `Customer Identification Service` → `/customer-identification` · 2 endpoints

**Digital SME Loan**  
_API to help SME customers access digitlal loans._  
- `SMELoan.Api` → `/smeloan` · 14 endpoints

**Fresh Desk**  
_APIs for Fresh Desk Integration._  
- `Freddy Chat APIs` → `/freddy` · 3 endpoints

**Gaming Product**  
_A bouquet of API services for gaming companies. Refer to the Product Summary page for more information._  
- `ALAT Wallet API Gateway` → `/alat-wallet` · 18 endpoints
- `Merchant-Payout API` → `/merchant-payout` · 10 endpoints

**Get Transaction Statement**  
_This REST API is used to retrieve an ALAT customer's transaction statement over a specified period of time._  
- `Get statement service` → `/get-statement` · 2 endpoints  — **✅ statement in this SDK**

**HQ Demo Cards** — 🔒 approval required  
_API for card operations_  
- `Alat Cards API V2` → `/alat-card-v2` · 80 endpoints

**Merchant - Payout API**  
_This API is to afford the merchant the ability to carry out fund transfer services._  
- `Merchant-Payout API` → `/merchant-payout` · 10 endpoints

**Money Guard Connect**  
_API for Money Guard Connect_  
- `MoneyGuardConnect.Api` → `/moneyguardconnect` · 3 endpoints

**Onboarding - Open API**  
_Open API for onboarding and generating wallet accounts for customers._  
- `Account Maintenance Open API` → `/account-maintenance` · 3 endpoints
- `Onboarding API - Wallets` → `/onboarding-wallets` · 1 endpoints
- `Onboarding Open API ` → `/onboarding-open` · 11 endpoints
- `The Callback URL` → `/callback-url` · 1 endpoints

**Open Banking APIs**  
_APIs for open banking._  
- `Account Maintenance Open API` → `/account-maintenance` · 3 endpoints
- `Airtime Open API` → `/airtime` · 3 endpoints
- `Bills Payment Open API` → `/open-bills` · 3 endpoints
- `Funds Transfer OpenAPI` → `/funds-transfer-open` · 3 endpoints  — **✅ transfer in this SDK**

**Outlet Link Pay**  
_Outlet Link Pay_  
- `Outlet-Link API` → `/outlet-link` · 15 endpoints

**Partnership Accounts**  
_APIs to create and manage accounts for partners._  
- `Account Maintenance Open API` → `/account-maintenance` · 3 endpoints
- `Funds Transfer API` → `/funds-transfer` · 3 endpoints
- `Partnership Account Creation` → `/partnership-account` · 1 endpoints

**Payout API**  
_Payout service used by merchants to channel specific accounts._  
- `Payout API` → `/payout` · 2 endpoints
- `Wallet Transfer API` → `/wallet-transfer` · 5 endpoints

**VerifyDiscountCode**  
_API to verify discount code for ALAT Rewards payment._  
- `VerifyDiscountCode` → `/verify-discount-code` · 1 endpoints

**Virtual Account** — 🔒 approval required  
_The WEMA Virtual account service is to provide the ability for the bank to receive NIP inflows into Vendor collection accounts on behalf of their customers._  
- `Virtual Account API` → `/VirtualAccount` · 7 endpoints

</details>

---

## 6. How responses are shaped (envelopes & the callback model)

### Response envelopes

Every ALAT response wraps its payload in one of **three** envelopes. The SDK models all three
generically (see [`envelope`]) and unwraps them for you, turning a bank-side failure into an
[`Error::Api`] that carries the bank's own message/code.

| Envelope | Shape | Success flag | Seen on |
| :-- | :-- | :-- | :-- |
| `ServiceResponse<T>` | `{ result, successful, message }` | `successful` | account, history, transfer |
| `Envelope<T>` | `{ result, errorMessage, errorMessages, hasError, timeGenerated }` | `!hasError` | name enquiry, bills, airtime |
| `ResponseModel<T>` | `{ data, status, message, code, statusCode, errors }` | `status` | wallet creation, statement |

### The asynchronous callback model

Money-movement and onboarding endpoints are **asynchronous**: a successful call returns **"Pending"**,
and ALAT later POSTs the final result to a **callback URL** you register with them. The callback body is
a generic model with a `RequestType` enum, e.g. `1 = WalletCreation`, `2 = AccountCreation`,
`3 = PinValidation`, `4 = PaymentResponse`, `5–8 = KYC re-upload prompts`. Build one callback endpoint
and switch on `RequestType`. (Full callback schema: the `The Callback URL` group in
[`MY_API_MAP.md`](./MY_API_MAP.md).)

---

## 7. Financial & ecosystem concepts

- **NUBAN** — the standard **10-digit** Nigerian bank account number. Every wallet/account uses it.
- **BVN** — 11-digit **Bank Verification Number**; the cross-bank biometric identity anchor for KYC.
- **NIN** — 11-digit **National Identification Number**; an alternative KYC anchor for Tier-1 wallets.
- **Tier-1 wallet** — a low-KYC account openable purely digitally, with CBN balance/transaction limits.
- **Name enquiry** — looking up the registered holder name for a NUBAN **before** sending money;
  Nigerian transfers are instant and irreversible, so this is mandatory.
- **Intrabank vs. interbank** — within Wema/ALAT vs. to another bank (the latter routes over NIBSS).
- **NIBSS / NIP** — the national switch and its instant-payments rail used for interbank transfers.
- **PND ("Post No Debit")** — a restriction that blocks debits from an account.
- **Common bank codes** — Wema `035`, GTBank `058`, Zenith `057`, Access `044`, UBA `033`,
  First Bank `011` (fetch the authoritative list with [`get_bank_list`](Client::get_bank_list)).

---

## 8. Using this SDK

### Install

```toml
[dependencies]
alat-rs = { path = "." } # or a published version
tokio = { version = "1", features = ["full"] }
```

### One client per gateway; sibling clients per product key

APIM issues **one subscription key per product**. Only the subscription key varies between products on
the same gateway (your `x-api-key` is constant), so derive sibling clients that **share one connection
pool**:

```rust,no_run
use alat::{Client, Config};
# fn main() -> Result<(), alat::Error> {
// Playground · "Wallet Services" product (wallet, account, bills, airtime):
let wallet = Client::new(Config::playground("WALLET_SERVICES_KEY", "channel_api_key"))?;
// Playground · "Get Statement Service" product — shares the HTTP pool:
let statements = wallet.with_subscription_key("GET_STATEMENT_KEY")?;
// APIM Dev · "Open Banking APIs" product (funds transfer) — different gateway:
let transfers = Client::new(Config::apim_dev("OPEN_BANKING_KEY", "channel_api_key"))?;
# let _ = (wallet, statements, transfers);
# Ok(())
# }
```

Add the bills/airtime credential with `Config::with_access_key("...")`, and tune the timeout with
`Config::with_timeout(...)`.

### Quickstart: verify a recipient, then transfer

```rust,no_run
use alat::{Client, Config};
use alat::modules::transfer::TransferRequest;

#[tokio::main]
async fn main() -> Result<(), alat::Error> {
    let client = Client::new(Config::apim_dev("OPEN_BANKING_KEY", "API_KEY"))?;

    // Always confirm the beneficiary first — transfers are irreversible.
    let who = client.verify_account("058", "0011223344", None).await?; // 058 = GTBank
    println!("Sending to: {}", who.account_name);

    let req = TransferRequest {
        amount: 5000.0,
        narration: "Invoice".into(),
        transaction_reference: "unique-ref-0001".into(), // idempotency key
        destination_bank_code: "058".into(),
        destination_bank_name: "GTBank".into(),
        destination_account_number: "0011223344".into(),
        destination_account_name: who.account_name,
        source_account_number: "0223344556".into(),
    };
    let res = client.transfer_funds("YOUR_SALT_KEY", &req).await?; // hash computed for you
    println!("Pending: {:?}", res.platform_transaction_reference);
    Ok(())
}
```

### Quickstart: open a Tier-1 wallet with a BVN

```rust,no_run
use alat::{Client, Config};
use alat::modules::wallet::{CreateWalletWithBvnRequest, ValidateOtpRequest};

#[tokio::main]
async fn main() -> Result<(), alat::Error> {
    let client = Client::new(Config::playground("WALLET_SERVICES_KEY", "API_KEY"))?;

    client.create_wallet_with_bvn(&CreateWalletWithBvnRequest {
        phone_number: "08012345678".into(),
        email: "jane@example.com".into(),
        bvn: "12345678901".into(),
    }).await?;                                  // step 1 → sends OTP

    client.validate_bvn_otp(&ValidateOtpRequest {
        phone_number: "08012345678".into(),
        otp: "123456".into(),                    // sandbox OTP
        tracking_id: "tracking-id".into(),
    }).await?;                                  // step 2 → enqueues creation

    let acct = client.get_bvn_partnership_account_details("08012345678").await?;
    println!("New NUBAN: {}", acct.account_number);     // step 3
    Ok(())
}
```

---

## 9. What this SDK covers

The SDK wraps a focused, correct subset of the 342 endpoints. Each method maps to a product (and thus a
subscription key) and a gateway:

| SDK module / methods | API group | Product to subscribe to | Gateway |
| :-- | :-- | :-- | :-- |
| `wallet` — `create_wallet_with_bvn`, `validate_bvn_otp`, `resend_bvn_otp`, `get_bvn_partnership_account_details`, `set_bvn_debit_restriction` (+ NIN equivalents) | `/account-creation`, `/wallet-creation` | **Wallet Services** | Playground |
| `account` — `get_wallet_details`, `get_transaction_history` | `/ws-acct-mgt` | **Wallet Services** | Playground |
| `statement` — `initiate_statement`, `get_statement_transactions` | `/get-statement-service` | **Get Statement Service** | Playground |
| `bills` — `get_all_bills`, `validate_customer`, `pay_bill`, `check_bill_transaction_status` | `/bills-payment` | **Wallet Services** | Playground |
| `bills` (airtime) — `get_data_plans`, `purchase_airtime`, `purchase_data` | `/airtime-data` | **Wallet Services** | Playground |
| `transfer` — `get_bank_list`, `verify_account`, `transfer_funds`, `get_nip_charges` | `/funds-transfer-open`, `/debit-wallet` (charges) | **Open Banking APIs** / **Wallet Services** | APIM Dev / Playground |
| `virtual_account` — `create_prefix`, `modify_prefix`, `list_prefixes`, `get_prefix`, `query_transactions` (+ webhook models) | `/VirtualAccount` | **Virtual Account** 🔒 | APIM Dev |
| `webhook` — generic callback types (`Callback`, `RequestType`, `NubanData`, …) | — (you host the endpoint) | — | — |

So a full local test uses **three subscription keys**: *Wallet Services* + *Get Statement Service*
(Playground) and *Open Banking APIs* (APIM Dev); collections additionally need the 🔒 approval-gated
*Virtual Account* product (APIM Dev). Everything else in Section 5 is documented in
[`MY_API_MAP.md`](./MY_API_MAP.md) and is straightforward to add following the same patterns.

### Building a collections → payout → refund system

`virtual_account` + `transfer` + `webhook` compose into the common aggregator flow: register a prefix
pointing at your central settlement account, mint a virtual account per payment, receive credits via
your `TransNotify` webhook, then disburse/refund from the central account with `transfer_funds`
(size refunds with `AccountNameEnquiry::charge_for` / `get_nip_charges`). Virtual accounts are
**collection-only** — payouts always debit the real central account. See the crate docs for the full
walkthrough.

### Running the tests

```bash
cargo test                       # offline: schema round-trips, HMAC, multi-key
# Live smoke test (needs only a subscription key for Open Banking APIs):
export WEMA_APIM_SUB_KEY="<your Open Banking key>"
cargo test --test sdk_tests live_get_bank_list_smoke -- --nocapture
```

---

## 10. Error handling

Every call returns `Result<T, alat::Error>`:

| Variant | Meaning | Typical cause |
| :-- | :-- | :-- |
| [`Error::Network`] | no usable HTTP response | DNS/timeout/TLS; safe to retry idempotent reads |
| [`Error::Http`] | non-2xx from the gateway | wrong/expired key, not subscribed (`401`/`403`), rate limit (`429`) |
| [`Error::Decode`] | 2xx but body didn't match the schema | API drift (body is included for diagnosis) |
| [`Error::Api`] | the bank rejected an otherwise-valid request | "Invalid BVN", "Insufficient funds", … (carries message + code) |
| [`Error::Configuration`] | bad/missing credential, caught before any I/O | invalid header bytes, missing `access` key |
| [`Error::Validation`] | client-side guard | e.g. empty transfer salt |

---

## 11. Full schema reference

- **[`MY_API_MAP.md`](./MY_API_MAP.md)** — every one of the 342 endpoints across both portals, with
  path, method, params, required headers, and **request/response example schemas**, extracted directly
  from the live ALAT developer portals.

## License

MIT. Unofficial; not affiliated with or endorsed by Wema Bank. API details are derived from Wema's
public developer portals and may change without notice — treat [`MY_API_MAP.md`](./MY_API_MAP.md) as a
snapshot and verify against the portals for production use.
