# ALAT/Wema API onboarding & production access

A companion to the [README](./README.md) (how the API is structured + how to use the SDK) and
[`MY_API_MAP.md`](./MY_API_MAP.md) (every endpoint's schema). This guide answers the one thing those
don't: **how you actually get credentialed by Wema and go to production.**

Throughout, **✅ documented** marks facts quoted from the official ALAT/Wema portals, and **🔶 ask
Wema** marks things that are *not* published anywhere public and must be confirmed during onboarding.

---

## The one thing to know

Your `Ocp-Apim-Subscription-Key` is self-serve from the developer portal. **Every other credential —
the `x-api-key`, your channel ID, the `access` header, the funds-transfer salt, payout bearer/AES keys
— is issued by Wema Bank during merchant onboarding, not generated in the portal.** ✅ documented
(*"This value would be provided by the bank."*)

So a valid subscription key alone is **not enough** for most endpoints. Until Wema onboards you, calls
needing `x-api-key` fail (e.g. `Apikey is null!` or *Invalid Channel ID*) — that's expected, not a bug.

---

## How to get your `x-api-key` (and the rest)

Do both of these:

1. **Apply through the "Go Live" portal.** Every product page on `playground.alat.ng` has a **"Go
   Live"** button linking to Wema's **ALAT Open API Portal** —
   **`https://openapidelegation.azurewebsites.net`**. Register/log in there to request production
   credentials. ✅ documented
2. **Contact your Wema bank liaison.** Endpoint pages state: *"Merchants are expected to be onboarded
   before they are able to make valid calls to this API. Kindly contact your liaison to the bank for
   this."* ✅ documented

If you don't have a liaison yet, start at `help@alat.ng`, or the business contacts in
[Contacts](#contacts).

---

## Self-serve vs. bank-issued (credential map)

| Credential | Self-serve? | How you get it |
|---|---|---|
| `Ocp-Apim-Subscription-Key` | ✅ Yes | Portal → subscribe to a product → copy from Profile. |
| `x-api-key` (channel/API key) | ❌ No | Bank-issued at onboarding — *"provided by the bank."* |
| Channel ID (sent on every call) | ❌ No | Issued with your credentials — *"a unique ID you pass with every call."* |
| `access` header (bills/airtime, credit-wallet) | ❌ No | Bank-issued — *"provided by the bank."* |
| Funds-transfer `salt` (HMAC `hash`) | ❌ No | Bank-issued. 🔶 not separately published |
| Merchant-Payout **bearer token** | ⚙️ Runtime | `POST {base}/api/Authentication/authenticate` (24h, refreshable) — but the **username/password** are bank-issued. |
| Merchant-Payout **Vendor ID** | ❌ No | *"shared by the Bank."* |
| Merchant-Payout **AES `EncryptionKey`/`IV`** | ❌ No | Bank-provided (inline doc examples are TEST values). |
| Closed-Wallet **token** (`Authorization`) | ❌ No | *"generated for respective clients/merchants by Wema Bank."* |

---

## The path to production (two stages)

The portal documents the model directly: *"To begin using our APIs, you have to subscribe to a product
and use the test Authentication Codes to test your product … When you are ready to go live, you will
apply for production credentials."* ✅ documented

1. **Sandbox / test** *(self-serve)* — create a portal account, subscribe, get your subscription key,
   and integrate against the **test** channel credentials the docs provide (e.g. the TEST AES keys /
   Vendor IDs shown inline on Merchant Payout; sandbox OTP `123456`; pending-then-callback flows).
   Sandbox gateways: `playground.azure-api.net` and `wema-alatdev-apimgt.azure-api.net`.
2. **Go live / production** *(Wema-issued)* — apply via the Go-Live portal and/or your bank liaison;
   Wema provisions your production channel ID, `x-api-key`, token/credentials, and (for payouts) AES
   keys, plus your production gateway host.

### Credentials by product (so you request the right set)

| Product (SDK module) | Credentials beyond the subscription key |
|---|---|
| Wallet creation / account / bills / airtime (`wallet`, `account`, `bills`) | `x-api-key` (+ `access` for bills/airtime) |
| Funds transfer (`transfer`) | HMAC `hash` from a bank-issued **salt** |
| Merchant Payout *(not yet in the SDK)* | **bearer token** + **Vendor ID** + **AES** payload encryption |
| Closed Wallet *(not in the SDK)* | Wema-issued **token** in `Authorization` |

For a **collections + payout** build, the lightest production path is **Wallet Services** (collections /
virtual accounts) + **Funds Transfer** (payouts): one subscription key per product, an `x-api-key`, and
a transfer salt.

---

## 🔶 What Wema does *not* publish (ask during onboarding)

None of the following appear on any public portal — put them to your liaison / the Go-Live portal:

1. Which team/role issues the `x-api-key` + channel ID, and whether the Go-Live portal alone suffices.
2. Required **business documents / KYC** (CAC registration, directors' IDs, etc.).
3. Whether a **Wema corporate/settlement account is mandatory**, and for which APIs. *(Strongly implied
   for payouts — Merchant Payout pays out from "the merchant's designated Wema Bank Account" — but not
   stated as a precondition for every API.)*
4. **Fees, any partnership/integration agreement, and approval timelines** (sandbox → live).
5. The **production gateway base URLs**, and how production keys differ structurally from sandbox.
6. For your specific products: which credential set and which **salt / encryption keys** you'll receive.

---

## Contacts

- **Go-Live / production-credential portal:** `https://openapidelegation.azurewebsites.net` (ALAT Open
  API Portal — register/log in to apply).
- **General developer support:** `help@alat.ng`.
- **Business team contacts** (surfaced on the Gaming product docs — gaming-specific, but a real human
  route into Wema if the above stall): `gamingunit@wemabank.com`; Babafemi Oluyemi +234 814 266 4332;
  Dumtochi Onyeadi +234 815 636 0789; Olabode Ibrahim +234 802 119 2759.

---

## ⚠️ ALAT Open API is not ALATPay

**ALATPay** (`developer.alatpay.ng`) is a **separate** collections product with a **self-serve**
credential model: you create an ALATPay merchant account, add your business under **Settings** to get a
**Business ID**, and read your **API Key + Business ID** from the **Developer** section of the merchant
dashboard (plus a callback URL for transaction webhooks). New businesses are *"approved internally by
the ALATPay team."* ✅ documented

ALATPay's **API Key + Business ID is *not* the Open API `x-api-key`** — don't mix them. If your need is
card/transfer **checkout collections**, ALATPay may be a faster self-serve route than the
bank-onboarded Open API this SDK targets.

---

## Sources & freshness

Primary (first-party portals), verified live **2026-06-16**:

- `wema-alatdev-apimgt.developer.azure-api.net/` — two-stage flow, channel ID
- `playground.alat.ng/api-wallet-creation`, `/api-credit-wallet`, `/product-wallet-services`
- `wema-alatdev-apimgt.developer.azure-api.net/merchant-payout`, `/closed-wallet`, `/docs/gaming`
- `openapidelegation.azurewebsites.net/Identity/Account/Login` + `/privacy` (ALAT Open API Portal)
- `developer.alatpay.ng/get-started`, `/onboarding-businesses` (ALATPay, separate product)

Phone numbers, emails, and portal URLs are point-in-time and may change. The portal's gated production
workflow could not be inspected directly, so the **🔶 ask Wema** items remain undocumented publicly.
