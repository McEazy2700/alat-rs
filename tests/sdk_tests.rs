//! SDK test suite.
//!
//! These tests are *grounded in the real ALAT response schemas* captured in
//! `MY_API_MAP.md`. The deserialization tests feed the portal's own example
//! bodies into the SDK's models, so a future schema drift fails CI rather than
//! silently breaking at runtime (which is exactly what the audit found in the
//! previous revision).

use alat::envelope::{ApiResponse, Envelope, ServiceResponse, StatusResponse};
use alat::modules::account::{Transaction, WalletAccountDetails};
use alat::modules::bills::{BillCategory, PaymentResult};
use alat::modules::transfer::{AccountNameEnquiry, BankList, TransferRequest, TransferResult};
use alat::modules::wallet::{CreateWalletWithBvnRequest, PartnershipAccountDetails};
use alat::{Client, Config};
use std::env;

// ---------------------------------------------------------------------------
// Request serialization — fields must match the server's typeName schema.
// ---------------------------------------------------------------------------

#[test]
fn bvn_request_serializes_to_real_schema() {
    let req = CreateWalletWithBvnRequest {
        phone_number: "08012345678".into(),
        email: "jane@example.com".into(),
        bvn: "12345678901".into(),
    };
    let json: serde_json::Value = serde_json::to_value(&req).unwrap();
    // PatnershipRequestWithBvn := { phoneNumber, email, bvn } — and nothing else.
    assert_eq!(json["phoneNumber"], "08012345678");
    assert_eq!(json["email"], "jane@example.com");
    assert_eq!(json["bvn"], "12345678901");
    let obj = json.as_object().unwrap();
    assert_eq!(obj.len(), 3, "must not send invented fields (e.g. dateOfBirth/reference)");
}

#[test]
fn transfer_request_includes_destination_bank_name() {
    let req = TransferRequest {
        amount: 5000.0,
        narration: "Invoice".into(),
        transaction_reference: "ref-1".into(),
        destination_bank_code: "058".into(),
        destination_bank_name: "GTBank".into(),
        destination_account_number: "0011223344".into(),
        destination_account_name: "Jane Doe".into(),
        source_account_number: "0223344556".into(),
    };
    let json = serde_json::to_value(&req).unwrap();
    // destinationBankName is part of OpenApiTransferRequest and was missing before.
    assert_eq!(json["destinationBankName"], "GTBank");
    assert_eq!(json["sourceAccountNumber"], "0223344556");
}

// ---------------------------------------------------------------------------
// Response deserialization — the real envelopes must decode (regression guard).
// ---------------------------------------------------------------------------

#[test]
fn wallet_details_decodes_real_envelope() {
    // B2BGetAccountV2ReponseServiceResponse: result nested, balance is a STRING.
    let body = r#"{"result":{"walletNumber":"0123456789","availableBalance":"1500.00",
        "walletStatus":"Active","accountType":"Wallet"},"successful":true,"message":"ok"}"#;
    let env: ServiceResponse<WalletAccountDetails> = serde_json::from_str(body).unwrap();
    let details = env.into_result().unwrap();
    assert_eq!(details.wallet_number, "0123456789");
    assert_eq!(details.available_balance, "1500.00");
}

#[test]
fn transaction_history_decodes_envelope_array() {
    let body = r#"{"result":[{"title":"t","amount":250.0,"type":"InterBank","date":"d",
        "transactionDate":"d","narration":"n","status":"Successful","creditType":"Default",
        "sender":"s","senderAccountNumber":"1","destinationBank":"b","destinationAccountNumber":"2",
        "recieverName":"Jane","referenceId":"id","isViewReceiptEnabled":true,"tranId":"t"}],
        "successful":true,"message":"ok"}"#;
    let env: ServiceResponse<Vec<Transaction>> = serde_json::from_str(body).unwrap();
    let txns = env.into_result().unwrap();
    assert_eq!(txns.len(), 1);
    assert_eq!(txns[0].receiver_name.as_deref(), Some("Jane")); // mapped from misspelled `recieverName`
    assert_eq!(txns[0].amount, 250.0);
}

#[test]
fn name_enquiry_decodes_envelope() {
    let body = r#"{"result":{"bankCode":"058","accountName":"JANE DOE","accountNumber":"0011",
        "currency":"NGN","termsAndConditions":null,"termsAndConditionsUrl":null,"chargeFee":[]},
        "errorMessage":null,"errorMessages":[],"hasError":false,"timeGenerated":"x"}"#;
    let env: Envelope<AccountNameEnquiry> = serde_json::from_str(body).unwrap();
    assert_eq!(env.into_result().unwrap().account_name, "JANE DOE");
}

#[test]
fn name_enquiry_business_error_surfaces_message() {
    let body = r#"{"result":null,"errorMessage":"Account not found","errorMessages":["Account not found"],
        "hasError":true,"timeGenerated":"x"}"#;
    let env: Envelope<AccountNameEnquiry> = serde_json::from_str(body).unwrap();
    match env.into_result() {
        Err(alat::Error::Api { message, .. }) => assert_eq!(message, "Account not found"),
        other => panic!("expected Error::Api, got {other:?}"),
    }
}

#[test]
fn bank_list_accepts_single_or_array() {
    // The portal example shows a single object; the endpoint is logically a list.
    let single = r#"{"successful":true,"message":"ok","result":{"bankName":"GTBank","bankCode":"058"}}"#;
    let many = r#"{"successful":true,"message":"ok","result":[{"bankName":"GTBank","bankCode":"058"},
        {"bankName":"Zenith","bankCode":"057"}]}"#;
    let one: ServiceResponse<BankList> = serde_json::from_str(single).unwrap();
    assert_eq!(one.into_result().unwrap().0.len(), 1);
    let multi: ServiceResponse<BankList> = serde_json::from_str(many).unwrap();
    assert_eq!(multi.into_result().unwrap().0.len(), 2);
}

#[test]
fn transfer_result_decodes_envelope() {
    let body = r#"{"successful":true,"message":"Pending","result":{"status":{},
        "message":"Pending","platformTransactionReference":"PTR-1"}}"#;
    let env: ServiceResponse<TransferResult> = serde_json::from_str(body).unwrap();
    let result = env.into_result().unwrap();
    assert_eq!(result.platform_transaction_reference.as_deref(), Some("PTR-1"));
}

#[test]
fn partnership_details_decodes_response_model() {
    // status is a bool at the top level; the account data is nested under `data`.
    let body = r#"{"message":"ok","status":true,"code":"Success","statusCode":"OK","errors":[],
        "data":{"accountNumber":"0123456789","firstName":"Jane","lastName":"Doe",
        "email":"jane@example.com","phoneNumber":"08012345678"}}"#;
    let env: ApiResponse<PartnershipAccountDetails> = serde_json::from_str(body).unwrap();
    let details = env.into_result().unwrap();
    assert_eq!(details.first_name, "Jane");
    assert_eq!(details.account_number, "0123456789");
}

#[test]
fn status_response_maps_failure_to_api_error() {
    let body = r#"{"message":"Invalid BVN","status":false,"code":"InvalidBvn",
        "statusCode":"Continue","errors":["bvn mismatch"]}"#;
    let env: StatusResponse = serde_json::from_str(body).unwrap();
    match env.into_result() {
        Err(alat::Error::Api { message, code, errors }) => {
            assert_eq!(message, "Invalid BVN");
            assert_eq!(code.as_deref(), Some("InvalidBvn"));
            assert_eq!(errors, vec!["bvn mismatch"]);
        }
        other => panic!("expected Error::Api, got {other:?}"),
    }
}

#[test]
fn bills_envelope_decodes_nested_categories() {
    let body = r#"{"result":[{"id":1,"name":"Cable TV","billers":[{"id":10,"name":"DSTV",
        "identifier":"dstv","shortCode":"DS","isAquired":true,"requiredValidation":true,"charge":100,
        "flow":0,"packages":[{"id":100,"billerId":10,"name":"Compact","isAmountEditable":false,
        "amount":10500,"minAmount":0,"maxAmount":0}]}]}],"errorMessage":null,"errorMessages":[],
        "hasError":false,"timeGenerated":"x"}"#;
    let env: Envelope<Vec<BillCategory>> = serde_json::from_str(body).unwrap();
    let cats = env.into_result().unwrap();
    assert!(cats[0].billers[0].is_acquired); // mapped from misspelled `isAquired`
    assert_eq!(cats[0].billers[0].packages[0].name, "Compact");
}

#[test]
fn payment_result_handles_missing_optional_fields() {
    // bills PayBill omits `value`; airtime includes it — both must decode.
    let bill = r#"{"result":{"status":"Pending","message":"m","narration":"n",
        "transactionReference":"r","platformTransactionReference":"p","transactionStan":"s",
        "orinalTxnTransactionDate":"d"},"errorMessage":null,"errorMessages":[],"hasError":false,
        "timeGenerated":"x"}"#;
    let env: Envelope<PaymentResult> = serde_json::from_str(bill).unwrap();
    let r = env.into_result().unwrap();
    assert!(r.value.is_none());
    assert_eq!(r.original_transaction_date.as_deref(), Some("d"));
}

// ---------------------------------------------------------------------------
// Virtual accounts (collections) + webhooks.
// ---------------------------------------------------------------------------

#[test]
fn compose_virtual_account_number_concatenates() {
    use alat::modules::virtual_account::compose_virtual_account_number;
    assert_eq!(compose_virtual_account_number("9988", "000042"), "9988000042");
}

#[test]
fn trans_notify_webhook_decodes_with_attribution_fields() {
    use alat::modules::virtual_account::TransNotifyRequest;
    // The credit notification Wema POSTs to your webhook (lowercase wire keys).
    let body = r#"{"originatoraccountnumber":"0011223344","amount":"5000.00","currency":"NGN",
        "originatorname":"JOHN PAYER","narration":"order#123","craccountname":"ACME LTD",
        "paymentreference":"pref","reference":"ref","bankname":"GTBank","sessionid":"SESS-1",
        "craccount":"9988000042","bankcode":"058","created_at":"2026-06-16T10:00:00Z"}"#;
    let n: TransNotifyRequest = serde_json::from_str(body).unwrap();
    assert_eq!(n.cr_account, "9988000042"); // which virtual account (attribution)
    assert_eq!(n.originator_account_number, "0011223344"); // payer (for refunds)
    assert_eq!(n.bank_code, "058");
    assert_eq!(n.session_id, "SESS-1"); // idempotency key
}

#[test]
fn prefix_setup_round_trips() {
    use alat::modules::virtual_account::PrefixSetup;
    let setup = PrefixSetup {
        user_name: "ACME".into(), prefix: "9988".into(), currency: "NGN".into(),
        base_url: "https://acme.example".into(), name_enquiry_uri: "/nip".into(),
        trans_notify_uri: "/notify".into(), auth_type: "Bearer".into(), auth_key: "k".into(),
        settle_account: "0123456789".into(), is_active: true,
    };
    let json = serde_json::to_value(&setup).unwrap();
    assert_eq!(json["settleAccount"], "0123456789");
    assert_eq!(json["nameEnquiryUri"], "/nip");
    let back: PrefixSetup = serde_json::from_value(json).unwrap();
    assert_eq!(back.prefix, "9988");
}

#[test]
fn generic_callback_decodes_pascalcase_and_extracts_nuban() {
    use alat::modules::webhook::{Callback, NubanData, NubanType, RequestType};
    // PascalCase variant with the NUBAN data model (RequestType 1 = WalletCreation).
    let body = r#"{"Title":"Wallet","Message":"created","Request":1,
        "Data":{"CustomerID":"C1","NUBANName":"ACME","NUBAN":"0123456789",
        "NUBANStatus":"Active","NUBANType":2,"Email":"a@b.com"}}"#;
    let cb: Callback = serde_json::from_str(body).unwrap();
    assert_eq!(cb.request_type, RequestType::WalletCreation);
    let nuban: NubanData = cb.data_as().unwrap();
    assert_eq!(nuban.nuban.as_deref(), Some("0123456789"));
    assert_eq!(nuban.nuban_type, Some(NubanType::Wallet));
}

#[test]
fn generic_callback_accepts_camelcase_and_unknown_request_type() {
    use alat::modules::webhook::{Callback, RequestType};
    let body = r#"{"title":"x","message":"y","requestType":99,"data":{}}"#;
    let cb: Callback = serde_json::from_str(body).unwrap();
    assert_eq!(cb.request_type, RequestType::Other(99)); // forward-compatible
}

// ---------------------------------------------------------------------------
// NIP charges (fees) + refund math.
// ---------------------------------------------------------------------------

#[test]
fn nip_charges_decode_and_band_lookup() {
    use alat::modules::transfer::NipCharges;
    let charges = NipCharges {
        charge_fees: serde_json::from_str(
            r#"[{"id":1,"chargeFeeName":"low","transactionType":0,"charge":10.0,"lower":0.0,"upper":5000.0},
                {"id":2,"chargeFeeName":"mid","transactionType":0,"charge":25.0,"lower":5000.01,"upper":50000.0}]"#,
        ).unwrap(),
        terms_and_conditions: None,
        terms_and_conditions_url: None,
    };
    assert_eq!(charges.charge_for(3000.0).unwrap().charge, 10.0);
    assert_eq!(charges.charge_for(20000.0).unwrap().charge, 25.0);
    assert!(charges.charge_for(999999.0).is_none());

    // Refund math: refund = paid - service_charge - transfer_fee.
    let paid = 20000.0;
    let fee = charges.charge_for(paid).unwrap().charge; // 25.0
    let service_charge = 100.0;
    assert_eq!(paid - service_charge - fee, 19875.0);
}

// ---------------------------------------------------------------------------
// HMAC signing.
// ---------------------------------------------------------------------------

#[test]
fn transfer_hash_is_128_hex_chars_and_deterministic() {
    let client = Client::new(Config::apim_dev("sub", "api")).unwrap();
    let req = TransferRequest {
        amount: 25000.0,
        narration: "Test".into(),
        transaction_reference: "ref_123456".into(),
        destination_bank_code: "058".into(),
        destination_bank_name: "GTBank".into(),
        destination_account_number: "0112233445".into(),
        destination_account_name: "John Doe".into(),
        source_account_number: "0223344556".into(),
    };
    let h1 = client.compute_transfer_hash("salt", &req).unwrap();
    let h2 = client.compute_transfer_hash("salt", &req).unwrap();
    assert_eq!(h1.len(), 128); // SHA-512 -> 64 bytes -> 128 hex chars
    assert_eq!(h1, h2, "hashing must be deterministic for idempotency");
    assert!(client.compute_transfer_hash("", &req).is_err(), "empty salt must be rejected");
}

#[test]
fn sibling_client_swaps_subscription_key_only() {
    // A client per APIM product: same gateway + channel key, different sub key.
    let wallet = Client::new(Config::playground("WALLET_SERVICES_KEY", "channel_api_key")).unwrap();
    let statements = wallet.with_subscription_key("GET_STATEMENT_KEY").unwrap();

    assert_eq!(wallet.subscription_key(), "WALLET_SERVICES_KEY");
    assert_eq!(statements.subscription_key(), "GET_STATEMENT_KEY");
    // Channel key and gateway are unchanged on the sibling.
    assert_eq!(statements.config().api_key, "channel_api_key");
    assert_eq!(statements.config().base_url, "https://playground.alat.ng");
    // An invalid swapped key is rejected, not silently accepted.
    assert!(wallet.with_subscription_key("bad\nkey").is_err());
}

#[test]
fn invalid_credentials_fail_client_construction() {
    // A header-invalid key must surface, not be silently dropped.
    let bad = Config::playground("ok", "bad\nkey");
    assert!(matches!(Client::new(bad), Err(alat::Error::Configuration(_))));
}

// ---------------------------------------------------------------------------
// Optional live integration smoke test (skipped unless keys are present).
// ---------------------------------------------------------------------------

/// Hits the live APIM Dev sandbox `GetAllBanks` (funds-transfer product).
///
/// Set `WEMA_APIM_SUB_KEY` (+ optionally `WEMA_APIM_API_KEY`) to run it. Note the
/// funds-transfer product lives on the APIM Dev gateway, **not** Playground.
#[tokio::test]
async fn live_get_bank_list_smoke() {
    let Ok(sub) = env::var("WEMA_APIM_SUB_KEY") else {
        eprintln!("skipping live test: WEMA_APIM_SUB_KEY not set");
        return;
    };
    let api = env::var("WEMA_APIM_API_KEY").unwrap_or_default();
    let client = Client::new(Config::apim_dev(sub, api)).unwrap();
    match client.get_bank_list().await {
        Ok(banks) => assert!(!banks.is_empty(), "expected at least one bank"),
        Err(e) => panic!("live GetAllBanks failed: {e}"),
    }
}
