# Wema ALAT Rust SDK — Audit Findings

**Date:** 2026-06-15
**Auditor:** automated source-of-truth audit
**Scope:**
1. Verify the correctness of `api_map.md` against the live ALAT developer portals.
2. Produce an independent, schema-aware API map (`MY_API_MAP.md`).
3. Audit the SDK (`src/`, `tests/`, `README.md`) against that ground truth for correctness,
   documentation sufficiency, and code quality.

---

## 0. Methodology — how the ground truth was obtained

The task brief assumed the docs are JavaScript-rendered and would need a Playwright scrape.
They are rendered by the **Azure API Management _managed_ developer portal** (knockout/paperbits SPA),
but that SPA is backed by a **public JSON management API on the same host**, so no browser
rendering was required. The data behind the rendered docs is reachable directly:

```
GET https://<portal-host>/developer/apis?api-version=2022-04-01-preview
GET https://<portal-host>/developer/apis/{apiId}/operations?api-version=2022-04-01-preview
GET https://<portal-host>/developer/apis/{apiId}/operations/{opId}?api-version=2022-04-01-preview
```

The operation-detail endpoint returns the **template/query params, required headers, and
request/response example bodies** (the `typeName` + example JSON) for every operation — i.e. the
schemas the rendered "Try it" console shows. I pulled all of this from both portals:

| Portal | Host | APIs | Operations |
| :-- | :-- | :--: | :--: |
| Playground | `playground.alat.ng` | 20 | 82 |
| APIM Dev | `wema-alatdev-apimgt.developer.azure-api.net` | 35 | 260 |
| **Total** | | **55** | **342** |

Artifacts produced by this audit:
- **[`MY_API_MAP.md`](./MY_API_MAP.md)** — independent ground-truth map with full request/response schemas for all 342 ops (committed to this repo).
- Raw extraction JSON (`playground.json`, `apim.json`) was generated under `/tmp/wema_extract/`.

> `api_map.md` (referenced throughout this report) is the **original Wema-published map** that was
> supplied as the audit *input*. It is intentionally **not tracked** in this repo; its corrected,
> schema-complete successor is [`MY_API_MAP.md`](./MY_API_MAP.md).

> Note: `api-version=2022-04-01-preview` is the only version the portal accepts; the gateway that
> actually serves traffic is `wema-alatdev-apimgt.azure-api.net` (the portal host is the
> `…developer.azure-api.net` sibling).

---

## 1. Audit of `api_map.md`

**Verdict: `api_map.md` is an accurate and complete _endpoint inventory_.** It was almost certainly
generated from the same management API.

I diffed every row of `api_map.md` against the 342 extracted operations, normalising whitespace and
the documented unicode artefacts (zero-width spaces / `&#8203;` that appear in the portal data):

| Check | Result |
| :-- | :-- |
| Rows parsed from `api_map.md` | 342 |
| Ground-truth operations | 342 (341 unique `(method, path)` — one duplicate, see below) |
| `(method, path)` rows in `api_map.md` **not** found in ground truth | **0** |
| Ground-truth operations **missing** from `api_map.md` | **0** |
| Exact mismatches incl. path-parameter names **and** casing | **0** |
| API base-path prefixes / API-ids in headers that mismatch | **0** (see false-positive note) |

So at the level it documents — base path, HTTP method, full path template, path-parameter names,
and casing — `api_map.md` contains **no hallucinations and no omissions**.

### 1.1 Limitations of `api_map.md` (not errors, but they matter for the SDK)

- **L1 — No request/response schemas (the root cause of the SDK's bugs).** `api_map.md` lists only
  paths + methods. It carries **zero** field-level information, so anyone implementing against it
  alone (as the SDK author did) must _invent_ request and response field names. Every schema-level
  SDK bug in §3 traces back to this gap. `MY_API_MAP.md` closes it.
- **L2 — Re-used API ids across portals.** Two API ids appear in **both** portals with **different
  base paths**, and `api_map.md` correctly documents both instances:
  - `wallet-transfer-api` → Playground "Debit Wallet API" at `/debit-wallet`, **and** APIM
    "Wallet Transfer API" at `/wallet-transfer`.
  - `get-statement-service` → Playground at `/get-statement-service`, **and** APIM at `/get-statement`.
  A consumer must disambiguate by portal/base-path, not by id. (This is the only thing that looked
  like a base-path mismatch in tooling; on inspection both are documented correctly — false positive.)
- **L3 — Raw HTML / portal junk reproduced verbatim.** Several descriptions contain raw HTML tables
  and `<br/>` markup copied from the portal. One entry — `virtualaccountapi-api` `GET
  /VirtualAccount/api/v1/Prefix` — has a `<canvas>`/JavaScript "Draw a line" demo as its
  "description". This is the portal's own garbage faithfully copied; cosmetic only.

---

## 2. Independent API map

See **[`MY_API_MAP.md`](./MY_API_MAP.md)** (generated, ~11k lines). For every operation it records: full path, HTTP
method, operation name, path params, query params, required headers, and request + response example
bodies with their server-side `typeName`. This is the artifact the SDK should be implemented against.

---

## 3. SDK audit vs ground truth

The SDK implements **19 endpoints** (`get_partnership_account_details` covers two doc steps) out of
the 342 available — a deliberate, reasonable starter subset (wallet creation, account/statement,
transfer, bills/airtime). The problem is **not coverage; it is correctness**: the SDK was written
against `api_map.md`'s paths (which are right) plus **guessed schemas** (which are largely wrong).

### 3.1 🔴 CRITICAL — Response models do not match the real envelopes (most calls cannot deserialize)

The real APIs wrap every payload in one of **two** envelope conventions:

- **`*ServiceResponse` / `*Envelope`** (transfer, bills, airtime, wallet-details):
  `{ "result": …, "successful": bool, "message": str }` **or**
  `{ "result": …, "errorMessage": …, "errorMessages": [...], "hasError": bool, "timeGenerated": str }`
- **`*ResponseModel`** (wallet-creation, statement):
  `{ "data": …, "message": str, "status": bool, "code": str, "statusCode": str, "errors": [...] }`

The SDK invents a **third, non-existent** shape — a flat `{ status: bool, message: String, <one field> }` —
and models response data as **flat top-level fields** instead of nested under `result`/`data`.
Because the SDK's envelope fields (`status`, `banks`, `transactions`, `account_name`, …) are
**required** (`String`/`bool`, not `Option`) and are **absent** from the real responses, `serde`
fails the deserialization outright.

**Proof.** I fed the real portal example bodies into the SDK's own structs (throwaway test, since removed):

```
FAILS  WalletDetails              <- B2BGetAccountV2ReponseServiceResponse   : missing field `accountNumber`
FAILS  Vec<WalletTransaction>     <- TransactionHistoryModelListServiceResponse: invalid type: map, expected a sequence
FAILS  NameEnquiryResponse        <- AccountNameEnquiryEnvelope               : missing field `status`
FAILS  BankListResponse           <- BanksOpenApiServiceResponse              : missing field `status`
FAILS  TransferResponse           <- OpenAPITransactionResponseOpenApiServiceResponse: missing field `status`
FAILS  PartnershipAccountDetails  <- GetPartnershipAccountDetailsResponseResponseModel: invalid type: boolean, expected a string
PARSES InitiateStatementResponse  <- InitiateGetCustomerStatementResponse     : (but silently drops the token — see below)
```

So **6 of 7** representative response structs throw an error on a *successful* API response. The 7th
parses but **silently loses data**: `InitiateStatementResponse.statement_token` reads a top-level
`statementToken` that does not exist — the real token is `data.referenceId` — so it is always `None`,
which then breaks Step 2.

Net effect: a developer who wires up this SDK against the live sandbox gets
`Error::Serialization(...)` on essentially every read endpoint, even when the bank returned `200 OK`.

### 3.2 🔴 CRITICAL — Default Sandbox base URL is a dead host

`Config::base_url()` (`src/config.rs:91`) returns:

| Environment | SDK value | Reality |
| :-- | :-- | :-- |
| `Sandbox` (the **default**) | `https://sandbox.alat.ng` | **does not resolve (NXDOMAIN)** |
| `Production` | `https://api.alat.ng` | resolves, but **unverified** as the APIM gateway |

`sandbox.alat.ng` has no DNS record, so the out-of-the-box configuration (`Config::new_sandbox`,
which the README and lib docs both showcase) fails at DNS resolution on **every** request. The real
hosts are `playground.alat.ng` and the gateway `wema-alatdev-apimgt.azure-api.net`. The production
host is a guess and must be confirmed with Wema before being shipped as a default.

### 3.3 🔴 CRITICAL — Request bodies have wrong / missing / invented fields

Field-by-field vs the real request `typeName`s. ❌ = wrong or invented, ⚠️ = missing required field.

| SDK fn / struct | SDK fields | Real fields (`typeName`) | Problem |
| :-- | :-- | :-- | :-- |
| `create_wallet_bvn_step1` / `PostBvnWalletRequest` | `bvn, phoneNumber, dateOfBirth, reference` | `phoneNumber, email, bvn` (`PatnershipRequestWithBvn`) | ❌ `dateOfBirth`, `reference` don't exist; ⚠️ missing `email` |
| `create_wallet_bvn_step2` / `ValidateBvnOtpRequest` | `token, otp, reference` | `phoneNumber, otp, trackingId` (`PatnershipRequestV2`) | ❌ `token`, `reference`; ⚠️ missing `phoneNumber`, `trackingId` |
| `create_wallet_nin_step1` / `PostNinWalletRequest` | `nin, phoneNumber, reference` | `phoneNumber, email, nin` (`PatnershipRequestV3`) | ❌ `reference`; ⚠️ missing `email` |
| `create_wallet_nin_step2` / `ValidateNinOtpRequest` | `token, otp, reference` | `phoneNumber, otp, trackingId` (`PatnershipRequestV2`) | ❌ `token`, `reference`; ⚠️ missing `phoneNumber`, `trackingId` |
| `manage_debit_restriction` / `DebitRestrictionRequest` | `accountNumber, action, reason` | `pndType, accountNumber` (`PndRequest`) | ❌ `action`, `reason`; ⚠️ missing `pndType` (enum: `LiftPnd`/…) |
| `get_wallet_transaction_history` / `TransactionHistoryRequest` | `accountNumber, pageSize, pageNumber` | `accountNumber, from, to, keyWord` (`TransactionhistoryV2Request`) | ❌ `pageSize`, `pageNumber` (no pagination); ⚠️ missing date range `from`/`to` |
| `initiate_statement` / `InitiateStatementRequest` | `accountNumber, startDate, endDate` | `accountNumber, dateFrom, dateTo` (`InitiateGetCustomerStatementRequest`) | ❌ field names `startDate`/`endDate` → must be `dateFrom`/`dateTo`; also full ISO-8601 datetime, not `YYYY-MM-DD` |
| `get_statement_transactions` / `GetStatementTransactionsRequest` | `statementToken, page` | `referenceId` (`GetCustomerTransactionsRequest`) | ❌ both fields wrong; ⚠️ should be single `referenceId` |
| `validate_customer_bill` / `ValidateCustomerRequest` | `billerCode, customerIdentifier` | `channelId, identifier, packageId` (`ValidationRequest`) | ❌ all three differ |
| `pay_bill` / `PayBillRequest` | `reference, sourceAccountNumber, billerCode, customerIdentifier, amount` | `clientId, customerAccount, amount, charge, transactionReference, packageId, customerIdentifier, customerEmail, customerPhoneNumber, customerName, securityInfo` (`PayBillClientRequest`) | ❌ `billerCode`, `sourceAccountNumber`; ⚠️ missing `clientId, charge, packageId, customerEmail, customerPhoneNumber, customerName, securityInfo` |
| `purchase_airtime` / `AirtimePurchaseRequest` | `reference, sourceAccountNumber, phoneNumber, amount, operator` | `transactionReference, accountNumber, network, phoneNumber, amount, securityInfo, clientId` (`AirtimeForClientReqModel`) | ❌ `reference`→`transactionReference`, `sourceAccountNumber`→`accountNumber`, `operator`→`network`; ⚠️ missing `securityInfo`, `clientId` |
| `purchase_data` / `DataPurchaseRequest` | `reference, sourceAccountNumber, phoneNumber, operator, planCode` | `transactionReference, accountNumber, phoneNumber, packageCode(int), amount, network, securityInfo, clientId` (`DataForClientReqModel`) | ❌ names; `planCode`(str)→`packageCode`(int); ⚠️ missing `amount`, `securityInfo`, `clientId` |
| `transfer_funds` / `TransferRequest` | `transactionReference, destinationBankCode, destinationAccountNumber, sourceAccountNumber, amount, destinationAccountName, narration` | + `destinationBankName` (`OpenApiTransferRequest`) | ⚠️ missing `destinationBankName`; otherwise field names align ✅ |

Because of serde's default `camelCase` rename, these **serialize to the wrong JSON keys**, so the
bank receives payloads missing required fields → validation errors / rejected transactions.

### 3.4 🔴 CRITICAL — Missing the `access` header for all bills/airtime endpoints

Every `bills-payment-api` and `airtime-and-data-api` operation requires an **`access`** header
(`get_all_billers`, `validate_customer_bill`, `pay_bill`, `purchase_airtime` mark it required;
`get_data_plans`/`purchase_data` mark it optional). The `Client` only ever injects
`Ocp-Apim-Subscription-Key` and `x-api-key` (`src/client.rs:32-54`) and exposes **no way** to add a
per-call header through `get()`/`post()`. So none of the 5 bills/airtime calls can send `access`, and
they will be rejected. (Contrast: `transfer_funds` correctly adds its `hash` header — but only because
it bypasses the helpers with a hand-rolled request, see §3.7.)

### 3.5 🟠 HIGH — Wrong query parameters

- `get_partnership_account_details` sends `?reference={…}` (`src/modules/wallet.rs:187`). The real
  operation takes **`?phoneNumber=`** and returns the account under `data`. The current call passes
  an unrecognised parameter and the result struct can't parse the envelope anyway (§3.1).
- `get_data_plans` appends `?operator={…}` (`src/modules/bills.rs:175`). The real `GetDataPlans` takes
  **no query parameter** — it returns all networks grouped by `networkProvider`. The extra param is
  spurious.
- `verify_account` — the real `AccountNameEnquiry` accepts an optional **`channelId`** query param the
  SDK never sends; some channels require it. Worth surfacing.

### 3.6 🟠 HIGH — `transfer_funds` HMAC: amount is hashed as an unformatted `f64`

`compute_transfer_hash` (`src/modules/transfer.rs:103-124`) builds the signed string with
`format!("{}…", …, payload.amount)` where `amount: f64`. Rust's `Display` for `f64` produces
`"5000"` for `5000.0` and `"25000.5"` for `25000.50`. The server computes the same HMAC over its own
canonical amount string; if it expects `"5000.00"` / `"25000.50"` (or an integer-minor representation),
**every signature mismatches** and transfers are rejected. Money amounts should not be `f64` at all
(see §3.8). The **concatenation order** itself — `transactionReference + destinationBankCode +
destinationAccountNumber + sourceAccountNumber + amount` — does match the documented order ✅.

### 3.7 🟡 MEDIUM — Code quality

- **Silent auth loss on header build.** Header injection uses `if let Ok(val) = HeaderValue::from_str(...)`
  (`client.rs:41-48`) — an invalid key character means the auth header is **silently dropped** and
  requests go out unauthenticated with no error. Worse, `reqwest::Client::builder().build()` is
  `.unwrap_or_default()` (`client.rs:59`): if the build fails, the fallback `Client::default()` has
  **none of the default headers**, so the SDK would silently send unauthenticated requests. Both
  should surface an error (e.g. a fallible `Client::try_new` / `Config::validate`).
- **Two error idioms.** `compute_transfer_hash` returns `Result<String, String>` and is then mapped
  into `Error::ValidationError` at the call site (`transfer.rs:103, 155`). It should return the crate's
  `Result<_, Error>` directly. The `Error::ValidationError` variant is otherwise unused — none of the
  client-side validations promised in its doc comment (negative amount, bank-code length, NUBAN length,
  11-digit BVN/NIN) are actually implemented anywhere.
- **No request timeout.** The `reqwest::Client` is built without `.timeout(...)`. For banking calls a
  hung connection blocks indefinitely; a default timeout (and ideally per-call override) is expected.
- **`post()`/`get()` can't carry per-call headers**, which is why `transfer_funds` hand-rolls its own
  request and `access`/`channelId` are impossible. The core helpers need an extensible request path.
- **No builder / no shared base for envelopes.** Each module redeclares an ad-hoc envelope; a single
  generic `ServiceResponse<T>` / `ResponseModel<T>` (matching the two real conventions) would be
  correct *and* less code.
- **Module/path mismatch risk:** `transfer.rs` doc-comments cite API id `funds-transfer-api`
  (`/funds-transfer-open`, APIM portal) — correct — but the integration test (below) points the whole
  client at the **Playground** host, where that path does not exist.

### 3.8 🟡 MEDIUM — Monetary values use `f64`

`amount`, `balance`, `available_balance`, `price`, etc. are `f64`. Floating point is unsafe for money
(rounding, non-exact decimals) and, here, also breaks the transfer HMAC (§3.6). Use integer minor units
or a decimal type, and serialize deterministically.

### 3.9 🟡 MEDIUM — Tests give false confidence

- `tests/sdk_tests.rs:78` — `assert!(res.status || !res.status)` is a tautology (always true);
  it asserts nothing about the response.
- The integration test (`:66-71`) targets `https://playground.alat.ng` but calls `get_bank_list()`,
  whose path (`/funds-transfer-open/...`) lives on the **APIM** gateway, not Playground — so the test
  exercises the wrong host. It also can't fail meaningfully because both branches `println!` and pass.
- The only genuinely useful checks are the HMAC length test and the camelCase serialization test — and
  the latter validates the **wrong** field set (it confirms `PostBvnWalletRequest` serializes
  `dateOfBirth`/`reference`, which the API does not accept — see §3.3).

### 3.10 🟢 LOW — Documentation accuracy nits (this SDK is meant to be self-documenting)

The brief asks for documentation that is "syntactic as well as semantic." The prose quality is genuinely
good — NUBAN/BVN/NIN/NIBSS concepts are well explained. But several doc statements are **factually wrong**,
which is worse than missing docs in a doc-first SDK:

- **`README.md:103`** says bank code **`058` = Zenith Bank**. `058` is **GTBank**; Zenith is **`057`**.
  (`src/modules/transfer.rs:30` correctly says `058`→GTBank — so the two disagree.)
- **`config.rs:11`** documents the Sandbox base as `https://sandbox.alat.ng` — a non-resolving host (§3.2).
- Every doc-comment that says "Ref: … Operation: …" pins a struct to a real operation while the struct's
  fields contradict that operation's real schema (§3.3) — the citations make the wrong fields look
  authoritative.
- `account.rs` `WalletTransaction` / `TransactionHistoryRequest` docs describe pagination + a
  `balance`/`transactionType` model the API doesn't have.
- `lib.rs`/README "Name Inquiry" example reads `response.status` / `response.account_name`, neither of
  which exists on the real `AccountNameEnquiryEnvelope` (it exposes `result.accountName` + `hasError`).

---

## 4. Severity summary

| # | Severity | Finding | Where |
| :--: | :-- | :-- | :-- |
| 3.1 | 🔴 Critical | Response envelopes wrong → 6/7 structs fail to deserialize real 200s; 1 silently loses token | all modules |
| 3.2 | 🔴 Critical | Default Sandbox base URL `sandbox.alat.ng` does not resolve; prod host unverified | `config.rs` |
| 3.3 | 🔴 Critical | Request bodies: wrong/invented/missing fields on 11 of 12 request structs | all modules |
| 3.4 | 🔴 Critical | `access` header never sent → all bills/airtime calls rejected | `client.rs`, `bills.rs` |
| 3.5 | 🟠 High | Wrong query params (`reference`→`phoneNumber`; spurious `operator`; missing `channelId`) | `wallet.rs`, `bills.rs`, `transfer.rs` |
| 3.6 | 🟠 High | HMAC hashes `amount` as unformatted `f64` → signature mismatch risk | `transfer.rs` |
| 3.7 | 🟡 Medium | Silent auth drop (`if let Ok` + `unwrap_or_default`), two error idioms, no timeout, no per-call headers, unused validation | `client.rs`, `transfer.rs`, `error.rs` |
| 3.8 | 🟡 Medium | `f64` for money | all modules |
| 3.9 | 🟡 Medium | Tautological assert; integration test hits wrong host; serialization test locks in wrong fields | `tests/` |
| 3.10 | 🟢 Low | Doc inaccuracies (058=GTBank not Zenith; dead sandbox host; field/example docs contradict real schema) | `README.md`, `config.rs`, modules |

`api_map.md` itself: **no correctness findings** — accurate, complete inventory; its only gap is the
absence of schemas (now supplied by `MY_API_MAP.md`).

---

## 5. Recommendations (in priority order)

1. **Re-model responses around the two real envelopes.** Add generic `ServiceResponse<T>` (`result` +
   `successful`/`hasError` + `message`/`errorMessages`) and `ResponseModel<T>` (`data` + `status` +
   `code`/`statusCode`/`errors`), and make every endpoint return `…<ConcreteData>`. Regenerate the
   concrete `data` structs from `MY_API_MAP.md`'s example bodies.
2. **Fix every request struct** to the real field sets in §3.3 (drop `reference`/`token`/`action`/
   pagination; add `email`, `trackingId`, `pndType`, `from`/`to`, `securityInfo`, `clientId`, etc.).
3. **Fix base URLs.** Default Sandbox → the real gateway host; confirm the production host with Wema
   before release. Consider taking the base URL explicitly rather than guessing.
4. **Generalise headers.** Let `get()`/`post()` accept extra headers; wire `access` into bills/airtime
   and `channelId` into name-enquiry. Fold `transfer_funds` back onto the shared path.
5. **Make money safe & hashing deterministic.** Replace `f64` with integer minor units or a decimal
   type; format the amount canonically (confirm the exact string Wema signs) before HMAC.
6. **Harden the client.** Replace silent header/build failures with a fallible constructor; add a
   default + overridable timeout; implement (or delete the doc promise of) the client-side validations.
7. **Replace the tests.** Drop the tautology; add round-trip (de)serialization tests that load the real
   example bodies from `MY_API_MAP.md`; point the integration test at the correct host per endpoint.
8. **Make the docs true.** Fix `058`→GTBank, the sandbox host, and align every field/example doc with
   the real schema. In a doc-first SDK, a confidently-wrong comment is a defect.

> The fastest correct path is to **regenerate the request/response structs from `MY_API_MAP.md`**
> rather than hand-editing — the current structs were guessed from a schema-less inventory, which is
> exactly how the field-level bugs were introduced.
