# Wema ALAT API — Independent Ground-Truth API Map

> Generated directly from the Azure APIM developer-portal management API (`/developer/apis?api-version=2022-04-01-preview`) of both portals on 2026-06-15.
> This is the data behind the JS-rendered docs — paths, methods, **and** request/response
> example schemas. It is the source of truth the SDK's types are modeled against.

Portals:
- **Playground** — portal `https://playground.alat.ng`; gateway `https://playground.azure-api.net` (calls go to the gateway)
- **APIM Dev** — portal `https://wema-alatdev-apimgt.developer.azure-api.net`; gateway `https://wema-alatdev-apimgt.azure-api.net`


---

## Playground — 20 APIs, 82 operations


### `6878d54550991ae138623e2e`  ·  Partnership Account - Face Biometric Authentication
*Web API to open accounts using face biometric authentication.*  
- Base path: `/create-account-face`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### GET `/create-account-face/api/CustomerInfo/GetDropDownList`
- Operation: *Step 0 - GetAddressDropDownList*
- Required headers: `x-api-key`(req)
- Response 200 `ContactDetailDropDownModel`:
```json
{
  "countryModel": {
    "countryList": [
      {
        "id": 0,
        "countryCode": "string",
        "countryName": "string"
      }
    ],
    "stateList": [
      {
        "id": 0,
        "name": "string",
        "finacleCode": "string",
        "country": "string"
      }
    ],
    "lgaList": [
      {
        "lgaId": 0,
        "stateId": 0,
        "name": "string"
      }
    ],
    "lcdaList": [
      {
        "lcdaId": 0,
        "lgaId": 0,
        "name": "string"
      }
    ],
    "cityList": [
      {
        "id": 0,
        "stateId": 0,
        "name": "string"
      }
    ],
    "housingTypes": [
      {
        "id": 0,
        "value": "string",
        "code": "string"
      }
    ]
  }
}
```

#### POST `/create-account-face/api/partnership/tier1-bvn-withoutOtp-v2`
- Operation: *Step 1a - Generate Tier 1 Account with BVN*
- Required headers: `x-api-key`(req)
- Request body `TierOneBVNRequest`:
```json
{
  "phoneNumber": "string",
  "email": "string",
  "bvn": "string",
  "correlationId": "string"
}
```
- Response 200 `None`:
```json
{
  "message": "string",
  "status": true,
  "code": 1,
  "statusCode": 100,
  "errors": [
    "string"
  ],
  "data": {
    "accountGenerationStatus": "string",
    "trackingId": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    "addressVerificationStatus": "string"
  }
}
```

#### POST `/create-account-face/api/partnership/tier1-nin-withoutOtp-v2`
- Operation: *Step 1b - Generate Tier 1 Account with NIN*
- Required headers: `x-api-key`(req)
- Request body `TierOneNinRequest`:
```json
{
  "phoneNumber": "string",
  "email": "string",
  "nin": "string",
  "correlationId": "3fa85f64-5717-4562-b3fc-2c963f66afa6"
}
```
- Response 200 `None`:
```json
{
  "message": "string",
  "status": true,
  "code": 1,
  "statusCode": 100,
  "errors": [
    "string"
  ],
  "data": {
    "accountGenerationStatus": "string",
    "trackingId": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    "addressVerificationStatus": "string"
  }
}
```

#### POST `/create-account-face/api/partnership/tier2-partnershipaccount-withoutOtp-v2`
- Operation: *Step 1c - Generate Tier 2*
- Required headers: `x-api-key`(req)
- Request body `PartnerOnboardingRequestModel`:
```json
{
  "bvn": "string",
  "nin": "string",
  "phoneNumber": "string",
  "emailAddress": "string",
  "residentialAddress": {
    "buildingNumber": "string",
    "apartment": "string",
    "street": "string",
    "city": "string",
    "town": "string",
    "state": "string",
    "lga": "string",
    "lcda": "string",
    "landmark": "string",
    "additionalInformation": "string",
    "country": "string",
    "fullAddress": "string",
    "postalCode": "string"
  },
  "liveImageOfFace": "string",
  "correlationId": "3fa85f64-5717-4562-b3fc-2c963f66afa6"
}
```
- Response 200 `ResponseModel`:
```json
{
  "message": "string",
  "status": true,
  "code": 1,
  "statusCode": 100,
  "errors": [
    "string"
  ],
  "data": {
    "accountGenerationStatus": "string",
    "trackingId": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
    "addressVerificationStatus": "string"
  }
}
```

#### GET `/create-account-face/api/CustomerAccount/GetPartnershipAccountDetails`
- Operation: *Step 2 - GetPartnershipAccountDetails - GET*
- Query params: `phoneNumber:string`
- Required headers: `x-api-key`(req)
- Response 200 `GetPartnershipAccountDetailsResponseResponseModel`:
```json
{
  "message": "string",
  "status": true,
  "code": "InvalidBvn",
  "statusCode": "Continue",
  "errors": [
    "string"
  ],
  "data": {
    "accountNumber": "string",
    "firstName": "string",
    "lastName": "string",
    "email": "string",
    "phoneNumber": "string"
  }
}
```


### `account-creation-bvn`  ·  Wallet Creation API - BVN
*API creating tier 1 accounts using BVN*  
- Base path: `/account-creation`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### POST `/account-creation/api/CustomerAccount/ResendOtpRequest/ResendOtp`
- Operation: */api/CustomerAccount/ResendOtpRequest/ResendOtp - POST*
- Request body `ResendOtpModel`:
```json
{
  "trackingId": "string",
  "phoneNumber": "string"
}
```

#### POST `/account-creation/api/CustomerAccount/PostPartnershipAccountCreationWithBvn`
- Operation: *Step 1 - PostPartnershipAccountCreationWithBvn - POST*
- Required headers: `x-api-key`(req)
- Request body `PatnershipRequestWithBvn`:
```json
{
  "phoneNumber": "string",
  "email": "string",
  "bvn": "string"
}
```
- Response 200 `ResponseModel`:
```json
{
  "message": "string",
  "status": true,
  "code": "InvalidBvn",
  "statusCode": "Continue",
  "errors": [
    "string"
  ]
}
```

#### POST `/account-creation/api/CustomerAccount/ValidateBVNandEnqueueAccountCreation`
- Operation: *Step 2 - ValidateBVNandEnqueueAccountCreation - POST*
- Request body `PatnershipRequestV2`:
```json
{
  "phoneNumber": "string",
  "otp": "string",
  "trackingId": "string"
}
```
- Response 200 `ResponseModel`:
```json
{
  "message": "string",
  "status": true,
  "code": "InvalidBvn",
  "statusCode": "Continue",
  "errors": [
    "string"
  ]
}
```

#### GET `/account-creation/api/CustomerAccount/GetPartnershipAccountDetails`
- Operation: *Step 3 - GetPartnershipAccountDetails - GET*
- Query params: `phoneNumber:string`
- Response 200 `GetPartnershipAccountDetailsResponseResponseModel`:
```json
{
  "message": "string",
  "status": true,
  "code": "InvalidBvn",
  "statusCode": "Continue",
  "errors": [
    "string"
  ],
  "data": {
    "accountNumber": "string",
    "firstName": "string",
    "lastName": "string",
    "email": "string",
    "phoneNumber": "string"
  }
}
```

#### POST `/account-creation/api/CustomerAccount/PartnerDebitRestrictionManagement`
- Operation: *Step 4 - PartnerDebitRestrictionManagement - POST*
- Required headers: `x-api-key`(req)
- Request body `PndRequest`:
```json
{
  "pndType": "LiftPnd",
  "accountNumber": "string"
}
```
- Response 200 `ResponseModel`:
```json
{
  "message": "string",
  "status": true,
  "code": "InvalidBvn",
  "statusCode": "Continue",
  "errors": [
    "string"
  ]
}
```


### `account-upgrade-api`  ·  Account Upgrade API
*Web API For Partnership Account Upgrade*  
- Base path: `/account-upgrade`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### GET `/account-upgrade/api/partnership/partner-account-kyc-status`
- Operation: *partner-account-kyc-status - GET*
- Query params: `accountNumber:string`
- Required headers: `x-api-key`(req)
- Response 200 `PartnerKycStatusResponseResponseModel`:
```json
{
  "message": "string",
  "status": true,
  "code": 1,
  "statusCode": 100,
  "errors": [
    "string"
  ],
  "data": {
    "accountNumber": "string",
    "accountName": "string",
    "accountTier": "string",
    "accountStatus": "string",
    "restrictionStatus": "string",
    "addressVerificationStatus": "string"
  }
}
```

#### POST `/account-upgrade/api/partnership/partner-account-upgrade-tier2`
- Operation: *partner-account-upgrade-tier2 - POST*
- Required headers: `x-api-key`(req)
- Request body `PartnerAddressModelV2`:
```json
{
  "accountNumber": "string",
  "nin": "string",
  "bvn": "string",
  "liveImageOfFace": "string"
}
```
- Response 200 `ResponseModel`:
```json
{
  "message": "string",
  "status": true,
  "code": 1,
  "statusCode": 100,
  "errors": [
    "string"
  ]
}
```

#### POST `/account-upgrade/api/partnership/partner-account-upgrade-tier3`
- Operation: *partner-account-upgrade-tier3 - POST*
- Required headers: `x-api-key`(req)
- Request body `PartnerAddressModel`:
```json
{
  "residentialAddress": {
    "buildingNumber": "string",
    "apartment": "string",
    "street": "string",
    "city": "string",
    "town": "string",
    "state": "string",
    "lga": "string",
    "lcda": "string",
    "landmark": "string",
    "additionalInformation": "string",
    "country": "string",
    "fullAddress": "string",
    "postalCode": "string"
  },
  "accountNumber": "string"
}
```
- Response 200 `ResponseModel`:
```json
{
  "message": "string",
  "status": true,
  "code": 1,
  "statusCode": 100,
  "errors": [
    "string"
  ]
}
```


### `airtime-and-data-api`  ·  Airtime and Data API
- Base path: `/airtime-data`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### POST `/airtime-data/api/PartnerPayment/CheckTransactionStatus`
- Operation: *CheckTransactionStatusForPoolAccount - POST*
- Required headers: `access`(req)
- Request body `CheckTransactionStatusRequest`:
```json
{
  "transactionReference": "string",
  "transactionType": 1
}
```
- Response 200 `CheckTransactionStatusResponseEnvelope`:
```json
{
  "result": {
    "transactionReference": "string",
    "transactionStatus": 1
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```

#### GET `/airtime-data/api/Data/GetDataPlans`
- Operation: *GetDataPlans - GET*
- Required headers: `access`
- Response 200 `PackagesResponseListEnvelope`:
```json
{
  "result": [
    {
      "id": 0,
      "networkProvider": "string",
      "dataPackages": [
        {
          "id": 0,
          "name": "string",
          "amount": 0,
          "dataPlan": "string",
          "validity_Period": "string",
          "enabledForBNPL": true,
          "description": "string"
        }
      ]
    }
  ],
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```

#### POST `/airtime-data/api/Airtime/Client/PurchaseAirtime`
- Operation: *PurchaseAirtimeWithSingleAccount - POST*
- Required headers: `access`(req)
- Request body `AirtimeForClientReqModel`:
```json
{
  "transactionReference": "string",
  "accountNumber": "string",
  "network": "string",
  "phoneNumber": "string",
  "amount": 0,
  "securityInfo": "string",
  "clientId": "string"
}
```
- Response 200 `GlobalResponseEnvelope`:
```json
{
  "result": {
    "status": "string",
    "message": "string",
    "narration": "string",
    "transactionReference": "string",
    "platformTransactionReference": "string",
    "transactionStan": "string",
    "value": "string",
    "orinalTxnTransactionDate": "string"
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```

#### POST `/airtime-data/api/Data/Client/PurchaseData`
- Operation: *PurchaseDatawithSingleAccount - POST*
- Required headers: `access`
- Request body `DataForClientReqModel`:
```json
{
  "transactionReference": "string",
  "accountNumber": "string",
  "phoneNumber": "string",
  "packageCode": 0,
  "amount": 0,
  "network": "string",
  "securityInfo": "string",
  "clientId": "string"
}
```
- Response 200 `GlobalResponseEnvelope`:
```json
{
  "result": {
    "status": "string",
    "message": "string",
    "narration": "string",
    "transactionReference": "string",
    "platformTransactionReference": "string",
    "transactionStan": "string",
    "value": "string",
    "orinalTxnTransactionDate": "string"
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```

#### POST `/airtime-data/api/PartnerPayment/PurchasePartnerAirtimeWithPoolAccount`
- Operation: *PurchasePartnerAirtimeWithPoolAccount - POST*
- Required headers: `access`(req)
- Request body `PurchaseAirtimeWithPINRequest`:
```json
{
  "clientTransactionReference": "string",
  "network": "string",
  "phoneNumber": "string",
  "amount": 0,
  "securityInfo": "string"
}
```
- Response 200 `PurchaseAirtimeWithPINResponseEnvelope`:
```json
{
  "result": {
    "transactionReference": "string",
    "responseMessage": "string"
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```

#### POST `/airtime-data/api/PartnerPayment/PurchasePartnerDataWithPoolAccount`
- Operation: *PurchasePartnerDataWithPoolAccount - POST*
- Required headers: `access`
- Request body `PurchaseDataWithPINRequest`:
```json
{
  "transactionReference": "string",
  "phoneNumber": "string",
  "packageCode": 0,
  "amount": 0,
  "networkProvider": "string",
  "securityInfo": "string"
}
```
- Response 200 `PurchaseAirtimeWithPINResponseEnvelope`:
```json
{
  "result": {
    "transactionReference": "string",
    "responseMessage": "string"
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```


### `bills-payment-api`  ·  Bills Payment API
- Base path: `/bills-payment`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### GET `/bills-payment/api/BillsPayment/GetAllBills`
- Operation: *Step 1: GetAllBills*
- Required headers: `access`(req)
- Response 200 `CategoryViewModelListEnvelope`:
```json
{
  "result": [
    {
      "id": 0,
      "name": "string",
      "billers": [
        {
          "id": 0,
          "name": "string",
          "identifier": "string",
          "shortCode": "string",
          "isAquired": true,
          "requiredValidation": true,
          "charge": 0,
          "flow": 0,
          "packages": [
            {
              "id": 0,
              "billerId": 0,
              "name": "string",
              "isAmountEditable": true,
              "amount": 0,
              "minAmount": 0,
              "maxAmount": 0
            }
          ]
        }
      ]
    }
  ],
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```

#### POST `/bills-payment/api/BillsPayment/ValidateCustomer`
- Operation: *Step 2: ValidateCustomerIdentifier*
- Required headers: `access`(req)
- Request body `ValidationRequest`:
```json
{
  "channelId": "string",
  "identifier": "string",
  "packageId": 0
}
```
- Response 200 `ValidationResponseEnvelope`:
```json
{
  "result": {
    "isValidated": true,
    "customerName": "string",
    "message": "string",
    "validationInfo": "string",
    "creditLimit": 0
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```

#### POST `/bills-payment/api/Shared/PayBill`
- Operation: *Step 3: PayBill - Client Account*
- Required headers: `access`(req)
- Request body `PayBillClientRequest`:
```json
{
  "clientId": "string",
  "customerAccount": "string",
  "amount": 0,
  "charge": 0,
  "transactionReference": "string",
  "packageId": 0,
  "customerIdentifier": "string",
  "customerEmail": "string",
  "customerPhoneNumber": "string",
  "customerName": "string",
  "securityInfo": "string"
}
```
- Response 200 `GlobalResponseEnvelope`:
```json
{
  "result": {
    "status": "string",
    "message": "string",
    "narration": "string",
    "transactionReference": "string",
    "platformTransactionReference": "string",
    "transactionStan": "string",
    "orinalTxnTransactionDate": "string"
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```

#### POST `/bills-payment/api/PartnerPayment/PayBillWithPoolAccount`
- Operation: *Step 4 - PayBill - Single Pool Account*
- Required headers: `access`(req)
- Request body `PayBillWithPoolAccountRequest`:
```json
{
  "transactionReference": "string",
  "amount": 0,
  "charge": 0,
  "packageId": 0,
  "customerIdentifier": "string",
  "customerEmail": "string",
  "customerPhoneNumber": "string",
  "securityInfo": "string"
}
```
- Response 200 `GlobalResponseEnvelope`:
```json
{
  "result": {
    "status": "string",
    "message": "string",
    "narration": "string",
    "transactionReference": "string",
    "platformTransactionReference": "string",
    "transactionStan": "string",
    "orinalTxnTransactionDate": "string"
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```

#### POST `/bills-payment/api/PartnerPayment/checktransactionstatus`
- Operation: *Step 5 - CheckTransactionStatus*
- Required headers: `access`(req)
- Request body `CheckBillsTransactionStatusRequest`:
```json
{
  "transactionReference": "string"
}
```
- Response 200 `CheckTransactionStatusResponseEnvelope`:
```json
{
  "result": {
    "transactionReference": "string",
    "transactionStatus": 1
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "2025-03-28T10:35:42.971Z"
}
```

#### GET `/bills-payment/api/PartnerPayment/CheckPoolAccountBalance`
- Operation: *Step 6 - CheckPoolAccountBalance*
- Required headers: `access`(req)


### `buy-now-pay-later-service`  ·  Buy-Now-Pay-Later Service
- Base path: `/alat-bnpl`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### GET `/alat-bnpl/api/Eligibility/productoffers`
- Operation: *Step 1: Get Product Offers*
- Required headers: `x-merchant-id`(req), `x-merchant-authorization-key`(req)
- Response 200 `GetEligibilityDataOutputModel`:
```json
{
  "productId": 0,
  "productName": "string",
  "minimumTenor": 0,
  "maximumTenor": 0,
  "managementFee": 0,
  "advisoryFee": 0,
  "vat": 0,
  "productVariant": {
    "id": 0,
    "name": "string",
    "interestRate": 0,
    "maximumAmount": 0,
    "minimumAmount": 0
  }
}
```

#### POST `/alat-bnpl/api/Eligibility/ConsentRequest`
- Operation: *Step 2: ConsentRequest - POST*
- Required headers: `x-merchant-id`(req), `x-merchant-authorization-key`(req)
- Request body `EligibilityConsentRequestInputModel`:
```json
{
  "accountNumber": "string",
  "productAmount": 0,
  "equityAmount": 0,
  "tenor": 0,
  "customerReference": "string"
}
```
- Response 200 `EligibilityConsentRequestOutputModelServiceResponse`:
```json
{
  "message": "string",
  "successful": true,
  "response": {
    "accountName": "string"
  }
}
```

#### POST `/alat-bnpl/api/LoanApplication/AcceptTerms`
- Operation: *Step 3: AcceptTerms - POST*
- Required headers: `x-merchant-id`(req), `x-merchant-authorization-key`(req)
- Request body `LoanInputModel`:
```json
{
  "eligibilityId": "string",
  "isTermsAccepted": true
}
```

#### GET `/alat-bnpl/api/LoanApplication/loan-application-status`
- Operation: *Step 4: Get Loan Application Status*
- Query params: `customeReference:string`(req)
- Required headers: `x-merchant-id`(req), `x-merchant-authorization-key`(req)

#### POST `/alat-bnpl/api/LoanApplication/loan-liquidation`
- Operation: *Step 5: Liquidate Loan*
- Required headers: `x-merchant-id`(req), `x-merchant-authorization-key`(req)
- Request body `None`:
```json
{
  "eligibilityId": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
  "customerReference": "string"
}
```
- Response 200 `None`:
```json
{
  "message": "Liquidation Consent Request Successful",
  "successful": true
}
```


### `card-management-api`  ·  Card Management API
*Web API for management of physical card requests.*  
- Base path: `/card-management`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### POST `/card-management/api/Partner/partnerCard/link-card`
- Operation: *BulkCard - Link/ActivateCard - POST*
- Required headers: `x-api-key`(req)
- Request body `LinkCardModel`:
```json
{
  "accountNo": "string",
  "cardPan": "string",
  "cardReferenceNumber": "string",
  "pin": "string",
  "expiryDate": "string"
}
```
- Response 200 `StringResponseModel`:
```json
{
  "data": "string",
  "message": "string",
  "status": true,
  "code": 1
}
```

#### POST `/card-management/api/Partner/partnerCard/validate-otp`
- Operation: *BulkCard - Validate Link/Activate - POST*
- Query params: `otp:string`, `trackingId:string`
- Required headers: `x-api-key`(req)
- Response 200 `StringResponseModel`:
```json
{
  "data": "string",
  "message": "string",
  "status": true,
  "code": 1
}
```

#### POST `/card-management/api/Partner/partnerCard/bulk-card-request`
- Operation: *BulkCardRequest - POST*
- Required headers: `x-api-key`(req)
- Request body `BulkCardModel`:
```json
{
  "numberOfCards": 0,
  "deliveryAddress": "string"
}
```
- Response 200 `StringResponseModel`:
```json
{
  "data": "string",
  "message": "string",
  "status": true,
  "code": 1
}
```

#### POST `/card-management/api/Partner/partnerCard/hotlistCard`
- Operation: *HotlistCard - POST*
- Query params: `maskedPan:string`, `accountNumber:string`
- Required headers: `x-api-key`(req)
- Response 200 `PinResponseModel`:
```json
{
  "successful": true,
  "message": "string"
}
```

#### GET `/card-management/api/Partner/partnerCard/retrieveCard/{accountNo}`
- Operation: *RetrieveCardDetails - GET*
- Path params: `accountNo:`(req)
- Required headers: `x-api-key`(req)
- Response 200 `StringResponseModel`:
```json
{
  "data": "string",
  "message": "string",
  "status": true,
  "code": 1
}
```

#### POST `/card-management/api/Partner/partnerCard/activateCard`
- Operation: *SingleCardActivate - POST*
- Required headers: `x-api-key`(req)
- Request body `PartnerCardActivationModel`:
```json
{
  "accountNumber": "string",
  "newPin": "string",
  "expiryDate": "string",
  "emailAddress": "string",
  "fullPan": "string"
}
```
- Response 200 `PinResponseModel`:
```json
{
  "successful": true,
  "message": "string"
}
```

#### POST `/card-management/api/Partner/partnerCard/changeCardPin`
- Operation: *SingleCard-ChangeCardPin - POST*
- Required headers: `x-api-key`(req)
- Request body `ChangePartnerCardPinModel`:
```json
{
  "accountNumber": "string",
  "newPin": "string",
  "oldPin": "string",
  "expiryDate": "string",
  "emailAddress": "string",
  "fullPan": "string"
}
```
- Response 200 `PinResponseModel`:
```json
{
  "successful": true,
  "message": "string"
}
```

#### POST `/card-management/api/Partner/partnerCard/request`
- Operation: *SingleCardRequest - POST*
- Required headers: `x-api-key`(req)
- Request body `PartnerCardRequestModel`:
```json
{
  "accountNumber": "string",
  "emailaddress": "string",
  "phoneNumber": "string",
  "streetAddress": "string",
  "city": "string",
  "nearestBustop": "string",
  "state": "string",
  "compoundName": "string",
  "lga": "string",
  "lcda": "string",
  "apartment": "string",
  "cardKey": "string",
  "amount": 0,
  "creditLimit": 0
}
```
- Response 200 `StringResponseModel`:
```json
{
  "data": "string",
  "message": "string",
  "status": true,
  "code": 1
}
```

#### GET `/card-management/api/Partner/partnerCard/virtual-card-details/{accountNo}`
- Operation: *VirtualCard-GetCardDetails*
- Path params: `accountNo:`(req)
- Required headers: `x-api-key`(req)
- Response 200 `None`:
```json
{
  "data": "string",
  "message": "string",
  "status": true,
  "code": 1,
  "statusCode": 100
}
```

#### POST `/card-management/api/Partner/partnerCard/virtualCard`
- Operation: *VirtualCardRequest*
- Required headers: `x-api-key`(req)
- Request body `VirtualCardRequestObject`:
```json
{
  "emailaddress": "string",
  "phoneNumber": "string",
  "accountNo": "string",
  "customerAddress": "string",
  "customerState": "string",
  "cardKey": "string",
  "currency": "string"
}
```
- Response 200 `None`:
```json
{
  "data": "string",
  "message": "string",
  "status": true,
  "code": 1,
  "statusCode": 100
}
```


### `credit-check-api`  ·  Credit Check API
- Base path: `/credit-check`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### POST `/credit-check/api/CreditCheck/ThirdPartyIndivualCreditCheck`
- Operation: *ThirdPartyIndivualCreditCheck - POST*
- Request body `ThirdPartyViewRequestModel`:
```json
{
  "bvn": "string",
  "creditCheckType": 1
}
```
- Response 200 `CreditCheckResponseViewModelServiceResponse`:
```json
{
  "result": {
    "creditCheckRequestId": "string",
    "passedCreditCheck": true,
    "totalLoanInstalmentAmount": 0,
    "loanDefault": true
  },
  "successful": true,
  "message": "string"
}
```


### `direct-debit-service-merchants`  ·  Direct Debit Service - Merchants
*Create one-time and recurring payment schedules on WEMA/ALAT accounts.*  
- Base path: `/merchant-direct-debit`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### POST `/merchant-direct-debit/api/DirectDebit`
- Operation: *Step 1 - SetupDirectDebit - POST*
- Required headers: `x-merchant-authorization-key`(req), `x-merchant-id`(req)
- Request body `SetupDirectDebitInputModel`:
```json
{
  "customerAccountNumber": "string",
  "initialPaymentAmount": 0,
  "repaymentAmount": 0,
  "repaymentStartDate": "2024-09-09T10:22:00.758Z",
  "repaymentFrequency": 0,
  "repaymentDuration": 0,
  "sendTransactionNotification": true
}
```
- Response 200 `SetupDirectDebitOutputModelServiceResponse`:
```json
{
  "successful": true,
  "message": "string",
  "response": {
    "directDebitId": "string",
    "customerAccountNumber": "string",
    "customerAccountName": "string",
    "mandateStatus": "string"
  }
}
```

#### GET `/merchant-direct-debit/api/DirectDebit/{Id}`
- Operation: *Step 2 - GetStatusForCustomerConsentDirectDebit/{Id} - GET*
- Path params: `Id:string`(req)
- Required headers: `x-merchant-authorization-key`(req), `x-merchant-id`(req)
- Response 200 `GetDirectDebitByIdOutputModelServiceResponse`:
```json
{
  "successful": true,
  "message": "string",
  "response": {
    "collectionAccountNumber": "string",
    "collectionAccountBankCode": "string",
    "customerAccountNumber": "string",
    "amount": 0,
    "startDate": "string",
    "duration": 0,
    "frequency": "string",
    "consentGranted": true,
    "status": "string"
  }
}
```

#### POST `/merchant-direct-debit/api/DirectDebit/QueueDirectDebitScheduleById`
- Operation: *Step 3 - RunDirectDebitScheduleByMandate - POST*
- Required headers: `x-merchant-authorization-key`(req), `x-merchant-id`(req)
- Request body `QueueDirectDebitScheduleByMandateInputModel`:
```json
{
  "directDebitId": "string"
}
```
- Response 200 `ServiceResponse`:
```json
{
  "successful": true,
  "message": "string"
}
```

#### GET `/merchant-direct-debit/api/Collection/GetByMandateId/{mandateId}`
- Operation: *Step 4 - GetScheduleByMandateId - GET*
- Path params: `mandateId:string`(req)
- Required headers: `x-merchant-authorization-key`(req), `x-merchant-id`(req)
- Response 200 `GetCollectionsByMandateIdOutputModelServiceResponse`:
```json
{
  "successful": true,
  "message": "string",
  "response": {
    "date": "string",
    "proposedAmount": 0,
    "collectedAmount": 0,
    "paid": true
  }
}
```


### `get-statement-service`  ·  Get Statement API
*Use transaction statement for informed analytics.*  
- Base path: `/get-statement-service`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### POST `/get-statement-service/api/AccountMaintenance/InitiateGetCustomerStatement`
- Operation: *Step 1: InitiateGetCustomerStatement*
- Required headers: `x-api-key`(req)
- Request body `InitiateGetCustomerStatementRequest`:
```json
{
  "accountNumber": "string",
  "dateFrom": "2022-09-07T09:27:13.133Z",
  "dateTo": "2022-09-07T09:27:13.133Z"
}
```
- Response 200 `InitiateGetCustomerStatementResponse`:
```json
{
  "data": {
    "referenceId": "string"
  },
  "message": "string",
  "status": true
}
```

#### POST `/get-statement-service/api/AccountMaintenance/GetCustomerTransactions`
- Operation: *Step 2: GetCustomerTransactions*
- Required headers: `x-api-key`(req)
- Request body `GetCustomerTransactionsRequest`:
```json
{
  "referenceId": "string"
}
```
- Response 200 `GetCustomerTransactionsResponse`:
```json
{
  "data": [
    {
      "title": "string",
      "amount": 0,
      "type": "InterBank",
      "date": "2022-09-07T09:45:31.199Z",
      "narration": "string",
      "status": "Default",
      "creditType": "Default",
      "sender": "string",
      "senderAccountNumber": "string",
      "destinationBank": "string",
      "destinationAccountNumber": "string",
      "recieverName": "string",
      "referenceId": "string",
      "isViewReceiptEnabled": true,
      "tranId": "string"
    }
  ],
  "message": "string",
  "status": true
}
```


### `partnership-account-full-kyc`  ·  Partnership Account - KYC
*Web API to open KYC-ed account*  
- Base path: `/create-account`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### POST `/create-account/api/CustomerAccount/GeneratePartnershipAccountV3/Tier3`
- Operation: *Step 1 - GenerateFullAccount - POST*
- Required headers: `x-api-key`(req)
- Request body `PartnerOnboardingRequestModel`:
```json
{
  "bvn": "string",
  "nin": "string",
  "phoneNumber": "string",
  "emailAddress": "string",
  "residentialAddress": {
    "buildingNumber": "string",
    "apartment": "string",
    "street": "string",
    "city": "string",
    "town": "string",
    "state": "string",
    "lga": "string",
    "lcda": "string",
    "landmark": "string",
    "additionalInformation": "string",
    "country": "string",
    "fullAddress": "string"
  },
  "liveImageOfFace": "string"
}
```
- Response 200 `ResponseModel`:
```json
{
  "message": "string",
  "status": true,
  "code": "InvalidBvn",
  "statusCode": "Continue",
  "errors": [
    "string"
  ]
}
```

#### POST `/create-account/api/CustomerAccount/PartnershipTier1AccountCreation`
- Operation: *Step 1b - Generate Tier 1 Account with BVN*
- Required headers: `x-api-key`(req)
- Request body `None`:
```json
{
  "phoneNumber": "string",
  "email": "string",
  "bvn": "string"
}
```

#### POST `/create-account/api/CustomerAccount/PartnershipWalletAccountCreation`
- Operation: *Step 1c - Generate Tier 1 Account with NIN*
- Required headers: `x-api-key`(req)
- Request body `None`:
```json
{
  "phoneNumber": "string",
  "email": "string",
  "nin": "string"
}
```

#### GET `/create-account/api/CustomerAccount/GetPartnershipAccountDetails`
- Operation: *Step 2 - GetPartnershipAccountDetails - GET*
- Query params: `phoneNumber:string`
- Required headers: `x-api-key`(req)
- Response 200 `GetPartnershipAccountDetailsResponseResponseModel`:
```json
{
  "message": "string",
  "status": true,
  "code": "InvalidBvn",
  "statusCode": "Continue",
  "errors": [
    "string"
  ],
  "data": {
    "accountNumber": "string",
    "firstName": "string",
    "lastName": "string",
    "email": "string",
    "phoneNumber": "string"
  }
}
```


### `partnership-account-with-address-verification`  ·  Partnership Account - with Address Verification
*Web API to open account with the Bank carrying out address verification for account upgrade*  
- Base path: `/partnership-address-verification`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### POST `/partnership-address-verification/api/CustomerAccount/ResendOtpRequest/ResendOtp`
- Operation: */api/CustomerAccount/ResendOtpRequest/ResendOtp - POST*
- Request body `ResendOtpModel`:
```json
{
  "trackingId": "string",
  "phoneNumber": "string"
}
```

#### POST `/partnership-address-verification/api/CustomerAccount/GeneratePartnershipAccount`
- Operation: *Step 1 - Generate Account - POST*
- Required headers: `x-api-key`(req)
- Request body `PartnerOnboardingRequestModel`:
```json
{
  "bvn": "string",
  "nin": "string",
  "phoneNumber": "string",
  "emailAddress": "string",
  "residentialAddress": {
    "buildingNumber": "string",
    "apartment": "string",
    "street": "string",
    "city": "string",
    "town": "string",
    "state": "string",
    "lga": "string",
    "lcda": "string",
    "landmark": "string",
    "additionalInformation": "string",
    "country": "string",
    "fullAddress": "string",
    "postalCode": "string"
  },
  "liveImageOfFace": "string"
}
```
- Response 200 `ResponseModel`:
```json
{
  "message": "string",
  "status": true,
  "code": "InvalidBvn",
  "statusCode": "Continue",
  "errors": [
    "string"
  ]
}
```

#### POST `/partnership-address-verification/api/CustomerAccount/ValidateOtpAndGeneratePartnershipAccount`
- Operation: *Step 2 - ValidateOtpAndGeneratePartnershipAccount - POST*
- Required headers: `x-api-key`(req)
- Request body `PatnershipRequestV2`:
```json
{
  "phoneNumber": "string",
  "otp": "string",
  "trackingId": "string"
}
```
- Response 200 `ResponseModel`:
```json
{
  "message": "string",
  "status": true,
  "code": "InvalidBvn",
  "statusCode": "Continue",
  "errors": [
    "string"
  ]
}
```

#### GET `/partnership-address-verification/api/CustomerAccount/GetPartnershipAccountDetails`
- Operation: *Step 3 - GetPartnershipAccountDetails - GET*
- Query params: `phoneNumber:string`
- Required headers: `x-api-key`(req)
- Response 200 `GetPartnershipAccountDetailsResponseResponseModel`:
```json
{
  "message": "string",
  "status": true,
  "code": "InvalidBvn",
  "statusCode": "Continue",
  "errors": [
    "string"
  ],
  "data": {
    "accountNumber": "string",
    "firstName": "string",
    "lastName": "string",
    "email": "string",
    "phoneNumber": "string"
  }
}
```

#### POST `/partnership-address-verification/api/CustomerAccount/SubmitPartnerAddress`
- Operation: *Step 4 - ReSubmitPartnerAddress - POST*
- Required headers: `x-api-key`(req)
- Request body `PartnerAddressModel`:
```json
{
  "residentialAddress": {
    "buildingNumber": "string",
    "apartment": "string",
    "street": "string",
    "city": "string",
    "town": "string",
    "state": "string",
    "lga": "string",
    "lcda": "string",
    "landmark": "string",
    "additionalInformation": "string",
    "country": "string",
    "fullAddress": "string",
    "postalCode": "string"
  },
  "accountNumber": "string"
}
```
- Response 200 `ResponseModel`:
```json
{
  "message": "string",
  "status": true,
  "code": "InvalidBvn",
  "statusCode": "Continue",
  "errors": [
    "string"
  ]
}
```

#### POST `/partnership-address-verification/api/CustomerAccount/PartnerDebitRestrictionManagement`
- Operation: *Step 5 - PartnerDebitRestrictionManagement - POST*
- Required headers: `x-api-key`(req)
- Request body `PndRequest`:
```json
{
  "pndType": "LiftPnd",
  "accountNumber": "string"
}
```
- Response 200 `ResponseModel`:
```json
{
  "message": "string",
  "status": true,
  "code": "InvalidBvn",
  "statusCode": "Continue",
  "errors": [
    "string"
  ]
}
```


### `pay-with-bank-account-alat-authenticator`  ·  Pay with Bank Account - ALAT Authenticator
*Single direct debit to a WEMA/ALAT account, by consent from customer using the ALAT mobile app.*  
- Base path: `/pwba-authenticator`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### POST `/pwba-authenticator/api/EcommerceTransfer/v2/transfer-fund-request`
- Operation: *Step 1: Initiate Fund Transfer Request*
- Request body `EcommerceTransferRequest`:
```json
{
  "amount": 0,
  "sourceAccountNumber": "string",
  "channelId": "string",
  "narration": "string",
  "transactionReference": "string"
}
```
- Response 200 `ClientTransactionResponseEnvelope`:
```json
{
  "result": {
    "status": "string",
    "message": "string",
    "platformTransactionReference": "string"
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```

#### GET `/pwba-authenticator/api/EcommerceTransfer/CheckTransactionStatus/{channelId}/{TransactionRefernce}`
- Operation: *Step 2: Check Transaction Status*
- Path params: `channelId:string`(req), `TransactionRefernce:string`(req)
- Response 200 `ClientTransactionResponseEnvelope`:
```json
{
  "result": {
    "status": "string",
    "message": "string",
    "platformTransactionReference": "string"
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```


### `remita-payment-api`  ·  Remita-Payment API
*API for Remita payments by client channels*  
- Base path: `/remita-payment`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### POST `/remita-payment/api/RemitaPayment/ProcessRemitaPayment`
- Operation: *Pay Remita Bill for Client*
- Required headers: `access`(req)
- Request body `RemitaPaymentClientRequest`:
```json
{
  "channelId": "string",
  "customerAccount": "string",
  "amount": 0,
  "charge": 0,
  "transactionReference": "string",
  "customerEmail": "string",
  "customerPhoneNumber": "string",
  "customerName": "string",
  "channelType": "string",
  "accountName": "string",
  "rrr": "string",
  "payerEmail": "string",
  "payerName": "string",
  "payerNumber": "string",
  "description": "string",
  "securityInfo": "string"
}
```
- Response 200 `GlobalResponseEnvelope`:
```json
{
  "result": {
    "status": "string",
    "message": "string",
    "narration": "string",
    "transactionReference": "string",
    "platformTransactionReference": "string",
    "transactionStan": "string",
    "orinalTxnTransactionDate": "string"
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```

#### GET `/remita-payment/api/RemitaPayment/PrintRemitaReceipt/{rrr}`
- Operation: *Print RRR Receipt*
- Path params: `rrr:`(req)
- Required headers: `access`(req)
- Response 200 `None`:
```json
{
  "result": "string",
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "2021-02-08T07:16:31Z"
}
```

#### GET `/remita-payment/api/RemitaPayment/ValidateRrr/{rrr}/{channelId}`
- Operation: *Validate Remita Retrieval Reference*
- Path params: `rrr:`(req), `channelId:`(req)
- Required headers: `access`(req)
- Response 200 `None`:
```json
{
  "result": {
    "responseStatus": 111,
    "isValidated": true,
    "description": "string",
    "amount": 0,
    "charges": 0,
    "total": 0,
    "name": "string",
    "email": "string",
    "phone": "string",
    "rrrCode": "string",
    "bankCharge": 0,
    "message": "string"
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "2021-02-08T07:03:07Z"
}
```


### `term-deposit-backed-credit-card-issuance-service`  ·  Term-Deposit-Backed Credit Card Issuance Service
- Base path: `/fdcreditcard`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### POST `/fdcreditcard/api/fdbackedcreditcard/createfixeddeposit`
- Operation: *CreateFixedDeposit - POST*
- Required headers: `access`
- Request body `CreateFixedDepositPayload`:
```json
{
  "accountNumber": "string",
  "initialDepositAmount": 0
}
```
- Response 200 `StringResponseWrapper`:
```json
{
  "message": "string",
  "isSuccessful": true,
  "errors": [
    "string"
  ],
  "title": "string",
  "responseObject": "string",
  "responseObjectExists": true
}
```

#### POST `/fdcreditcard/api/fdbackedcreditcard/liquidatefixeddeposit`
- Operation: *LiquidateFixedDeposit - POST*
- Required headers: `access`
- Request body `LiquidateFixedDepositPayload`:
```json
{
  "accountNumber": "string"
}
```
- Response 200 `StringResponseWrapper`:
```json
{
  "message": "string",
  "isSuccessful": true,
  "errors": [
    "string"
  ],
  "title": "string",
  "responseObject": "string",
  "responseObjectExists": true
}
```

#### POST `/fdcreditcard/api/fdbackedcreditcard/requestcreditcard`
- Operation: *RequestCreditCard - POST*
- Required headers: `access`
- Request body `RequestCreditCardPayload`:
```json
{
  "accountNumber": "string",
  "phoneNumber": "string",
  "preferredName": "string",
  "amount": 0,
  "apartment": "string",
  "compoundName": "string",
  "streetAddress": "string",
  "nearestBustop": "string",
  "lcda": "string",
  "lga": "string",
  "city": "string",
  "state": "string"
}
```
- Response 200 `StringResponseWrapper`:
```json
{
  "message": "string",
  "isSuccessful": true,
  "errors": [
    "string"
  ],
  "title": "string",
  "responseObject": "string",
  "responseObjectExists": true
}
```


### `verifydiscountcode-merchant`  ·  VerifyDiscountCode - Merchant
*Programmatically verify discount codes presented by customers for redemption.*  
- Base path: `/merchant-verify-discount-code`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### POST `/merchant-verify-discount-code/api/Deals/VerifyDiscountCode`
- Operation: *VerifyDiscountCode*
- Request body `VerifyDiscountCodeRequest`:
```json
{
  "discountCode": "string",
  "partnerId": "3fa85f64-5717-4562-b3fc-2c963f66afa6"
}
```
- Response 200 `VerifyDiscountCodeResponse`:
```json
{
  "data": {
    "transactionDate": "string",
    "transactionAmount": 0,
    "referenceNumber": "string",
    "transactionRemark": "string",
    "accountNumber": "string",
    "accountName": "string",
    "bank": "string",
    "code": "string",
    "discount": "string",
    "dealType": "string",
    "verifiedBy": "string",
    "verifiedAt": "string"
  },
  "hasError": true,
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "timeGenerated": "2022-11-15T17:30:39.119Z"
}
```


### `wallet-creation-api`  ·  Wallet Creation API - NIN
*Web API for creating tier 1 accounts using NIN*  
- Base path: `/wallet-creation`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### POST `/wallet-creation/api/CustomerAccount/ResendOtpRequest/ResendOtp`
- Operation: */api/CustomerAccount/ResendOtpRequest/ResendOtp - POST*
- Request body `ResendOtpModel`:
```json
{
  "trackingId": "string",
  "phoneNumber": "string"
}
```

#### POST `/wallet-creation/api/CustomerAccount/GenerateWalletAccountForPartnerships/Request`
- Operation: *Step 1 - GenerateWalletAccountForPartnerships/Request - POST*
- Required headers: `x-api-key`(req)
- Request body `PatnershipRequestV3`:
```json
{
  "phoneNumber": "string",
  "email": "string",
  "nin": "string"
}
```
- Response 200 `ResponseModel`:
```json
{
  "message": "string",
  "status": true,
  "code": "InvalidBvn",
  "statusCode": "Continue",
  "errors": [
    "string"
  ]
}
```

#### POST `/wallet-creation/api/CustomerAccount/GenerateWalletAccountForPartnershipsV2/Otp`
- Operation: *Step 2 - GenerateWalletValidateNinUsingOtp - POST*
- Required headers: `x-api-key`(req)
- Request body `PatnershipRequestV2`:
```json
{
  "phoneNumber": "string",
  "otp": "string",
  "trackingId": "string"
}
```
- Response 200 `ResponseModel`:
```json
{
  "message": "string",
  "status": true,
  "code": "InvalidBvn",
  "statusCode": "Continue",
  "errors": [
    "string"
  ]
}
```

#### GET `/wallet-creation/api/CustomerAccount/GetPartnershipAccountDetails`
- Operation: *Step 3- GetPartnershipAccountDetails*
- Query params: `phoneNumber:string`
- Required headers: `x-api-key`(req)
- Response 200 `GetPartnershipAccountDetailsResponseResponseModel`:
```json
{
  "message": "string",
  "status": true,
  "code": "InvalidBvn",
  "statusCode": "Continue",
  "errors": [
    "string"
  ],
  "data": {
    "accountNumber": "string",
    "firstName": "string",
    "lastName": "string",
    "email": "string",
    "phoneNumber": "string"
  }
}
```

#### POST `/wallet-creation/api/CustomerAccount/PartnerDebitRestrictionManagement`
- Operation: *Step 4 - PartnerDebitRestrictionManagement - POST*
- Required headers: `x-api-key`(req)
- Request body `PndRequest`:
```json
{
  "pndType": "LiftPnd",
  "accountNumber": "string"
}
```
- Response 200 `ResponseModel`:
```json
{
  "message": "string",
  "status": true,
  "code": "InvalidBvn",
  "statusCode": "Continue",
  "errors": [
    "string"
  ]
}
```


### `wallet-payout-api`  ·  Credit Wallet API
*This API enables channels payout credits to wallets on their books.*  
- Base path: `/credit-wallet`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### GET `/credit-wallet/api/Shared/AccountNameEnquiry/Wallet/{accountNumber}`
- Operation: *Step 1: Name Enquiry - Destination Account/Wallet*
- Path params: `accountNumber:string`(req)
- Required headers: `access`(req)
- Response 200 `NameEnquiryResponseEnvelope`:
```json
{
  "result": {
    "bankCode": "string",
    "accountName": "string",
    "accountNumber": "string",
    "currency": "string"
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```

#### POST `/credit-wallet/api/IntraBankTransfer/FundWallet`
- Operation: *Step 2: Client to Fund Wallet/Account*
- Required headers: `access`(req)
- Request body `FundWalletRequest`:
```json
{
  "securityInfo": "string",
  "destinationAccountNumber": "string",
  "amount": 0,
  "narration": "string",
  "transactionReference": "string",
  "useCustomNarration": true
}
```
- Response 200 `GlobalResponseEnvelope`:
```json
{
  "result": {
    "status": "string",
    "message": "string",
    "narration": "string",
    "transactionReference": "string",
    "platformTransactionReference": "string",
    "transactionStan": "string",
    "orinalTxnTransactionDate": "string"
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```

#### GET `/credit-wallet/api/IntraBankTransfer/ConfirmClientTransferStatus/{clientTransactionReference}`
- Operation: *Step 3: Confirm Client Transaction Status*
- Path params: `clientTransactionReference:`(req)
- Required headers: `access`(req)
- Response 200 `ClientTransferCallBackResponseEnvelope`:
```json
{
  "result": {
    "title": "string",
    "message": "string",
    "data": {
      "status": "string",
      "message": "string",
      "narration": "string",
      "transactionReference": "string",
      "platformTransactionReference": "string",
      "transactionStan": "string",
      "orinalTxnTransactionDate": "2024-08-20T12:05:13.624Z"
    },
    "request": 1
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "2024-08-20T12:05:13.624Z"
}
```


### `wallet-services-account-maintenance-api`  ·  Wallet Services - Account Management API
*Web API for maintenance on wallets.*  
- Base path: `/ws-acct-mgt`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### GET `/ws-acct-mgt/api/AccountMaintenance/CustomerAccount/GetAccountV2/accountNumber/{accountNumber}`
- Operation: *Get Wallet Details - GET*
- Path params: `accountNumber:string`(req)
- Required headers: `x-api-key`(req)
- Response 200 `B2BGetAccountV2ReponseServiceResponse`:
```json
{
  "result": {
    "walletNumber": "string",
    "availableBalance": "string",
    "walletStatus": "string",
    "accountType": "string"
  },
  "successful": true,
  "message": "string"
}
```

#### POST `/ws-acct-mgt/api/AccountMaintenance/CustomerAccount/transhistoryV2`
- Operation: *Transaction History - POST*
- Required headers: `x-api-key`(req)
- Request body `TransactionhistoryV2Request`:
```json
{
  "accountNumber": "string",
  "from": "string",
  "to": "string",
  "keyWord": "string"
}
```
- Response 200 `TransactionHistoryModelListServiceResponse`:
```json
{
  "result": [
    {
      "title": "string",
      "amount": 0,
      "type": "InterBank",
      "date": "string",
      "transactionDate": "string",
      "narration": "string",
      "status": "Default",
      "creditType": "Default",
      "sender": "string",
      "senderAccountNumber": "string",
      "destinationBank": "string",
      "destinationAccountNumber": "string",
      "recieverName": "string",
      "referenceId": "string",
      "isViewReceiptEnabled": true,
      "tranId": "string"
    }
  ],
  "successful": true,
  "message": "string"
}
```


### `wallet-transfer-api`  ·  Debit Wallet API
*This API allows clients/channels to request and authorize debit transactions for wallets in their ecosystem.*  
- Base path: `/debit-wallet`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### GET `/debit-wallet/api/Shared/AccountNameEnquiry/Wallet/{accountNumber}`
- Operation: *Step 1: Account Name Enquiry - Source*
- Path params: `accountNumber:string`(req)
- Required headers: `access`(req)
- Response 200 `NameEnquiryResponseEnvelope`:
```json
{
  "result": {
    "bankCode": "string",
    "accountName": "string",
    "accountNumber": "string",
    "currency": "string"
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```

#### GET `/debit-wallet/api/Shared/GetAllBanks`
- Operation: *Step 2: Get All Banks*
- Response 200 `BanksEnvelope`:
```json
{
  "result": {
    "bankName": "string",
    "bankCode": "string"
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```

#### GET `/debit-wallet/api/Shared/AccountNameEnquiry/{bankCode}/{accountNumber}`
- Operation: *Step 3: Account Name Enquiry*
- Path params: `bankCode:string`(req), `accountNumber:string`(req)
- Response 200 `AccountNameEnquiryEnvelope`:
```json
{
  "result": {
    "bankCode": "string",
    "accountName": "string",
    "accountNumber": "string",
    "currency": "string",
    "termsAndConditions": "string",
    "termsAndConditionsUrl": "string",
    "chargeFee": [
      {
        "id": 0,
        "chargeFeeName": "string",
        "transactionType": 0,
        "charge": 0,
        "lower": 0,
        "upper": 0
      }
    ]
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```

#### GET `/debit-wallet/api/Shared/GetNIPCharges`
- Operation: *Step 4: Get NIP Charges*
- Response 200 `NIPChargesEnvelope`:
```json
{
  "result": {
    "chargeFees": [
      {
        "id": 0,
        "chargeFeeName": "string",
        "transactionType": 0,
        "charge": 0,
        "lower": 0,
        "upper": 0
      }
    ],
    "termsAndConditions": "string",
    "termsAndConditionsUrl": "string"
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```

#### POST `/debit-wallet/api/Shared/ProcessClientTransfer`
- Operation: *Step 5: Process Client Transfer*
- Required headers: `access`(req)
- Request body `ClientTransferRequestDto`:
```json
{
  "securityInfo": "string",
  "amount": 0,
  "destinationBankCode": "string",
  "destinationBankName": "string",
  "destinationAccountNumber": "string",
  "destinationAccountName": "string",
  "sourceAccountNumber": "string",
  "narration": "string",
  "transactionReference": "string",
  "useCustomNarration": true
}
```
- Response 200 `GlobalResponseEnvelope`:
```json
{
  "result": {
    "status": "string",
    "message": "string",
    "narration": "string",
    "transactionReference": "string",
    "platformTransactionReference": "string",
    "transactionStan": "string",
    "orinalTxnTransactionDate": "string"
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```

#### GET `/debit-wallet/api/IntraBankTransfer/ConfirmClientTransferStatus/{clientTransactionReference}`
- Operation: *Step 6: Confirm Client Transaction Status*
- Path params: `clientTransactionReference:`(req)
- Required headers: `access`(req)
- Response 200 `ClientTransferCallBackResponseEnvelope`:
```json
{
  "result": {
    "title": "string",
    "message": "string",
    "data": {
      "status": "string",
      "message": "string",
      "narration": "string",
      "transactionReference": "string",
      "platformTransactionReference": "string",
      "transactionStan": "string",
      "orinalTxnTransactionDate": "2024-08-20T12:05:13.624Z"
    },
    "request": 1
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "2024-08-20T12:05:13.624Z"
}
```


---

## APIM Dev — 35 APIs, 260 operations


### `60bee845378bc5612983eb4e`  ·  Onboarding API - Wallets
*Web API For Onboarding and Generating Wallet Accounts.*  
- Base path: `/onboarding-wallets`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### POST `/onboarding-wallets/api/CustomerAccount/GenerateWalletV2`
- Operation: *GenerateWallet*
- Request body `B2bCreateAccountDto`:
```json
{
  "gender": "Male",
  "email": "string",
  "firstName": "string",
  "lastName": "string",
  "dob": "string",
  "phoneNumber": "string"
}
```
- Response 200 `ServiceResponse`:
```json
{
  "successful": true,
  "message": "string"
}
```


### `60befbe4da5913917c2660d4`  ·  Account Maintenance Open API
*Web API For Account Maintenance*  
- Base path: `/account-maintenance`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### GET `/account-maintenance/api/AccountMaintenance/CustomerAccount/GetAccountV2/accountNumber/{accountNumber}`
- Operation: *GetAccount - GET*
- Path params: `accountNumber:string`(req)
- Response 200 `B2BGetAccountV2ReponseServiceResponse`:
```json
{
  "result": {
    "walletNumber": "string",
    "availableBalance": "string",
    "walletStatus": "string",
    "accountType": "string"
  },
  "successful": true,
  "message": "string"
}
```

#### GET `/account-maintenance/api/AccountMaintenance/CustomerAccount/GetAccountByPhoneNumber/phoneNumber/{phoneNumber}`
- Operation: *GetAccountByPhoneNumber - GET*
- Path params: `phoneNumber:string`(req)
- Required headers: `x-api-key`(req)
- Response 200 `GetAcctByPhoneNumberResponseServiceResponse`:
```json
{
  "result": {
    "firstName": "string",
    "middleName": "string",
    "lastName": "string",
    "dob": "string",
    "gender": "string",
    "accountNumber": "string",
    "channel_Id": "string",
    "phone": "string"
  },
  "successful": true,
  "message": "string"
}
```

#### POST `/account-maintenance/api/AccountMaintenance/CustomerAccount/transhistoryV2`
- Operation: *TransactionHistoryV2 - POST*
- Request body `TransactionhistoryV2Request`:
```json
{
  "accountNumber": "string",
  "from": "string",
  "to": "string"
}
```
- Response 200 `TransactionHistoryModelListServiceResponse`:
```json
{
  "result": [
    {
      "title": "string",
      "amount": 0.0,
      "type": "InterBank",
      "date": "string",
      "narration": "string",
      "status": "Default",
      "creditType": "Default",
      "sender": "string",
      "senderAccountNumber": "string",
      "destinationBank": "string",
      "destinationAccountNumber": "string",
      "recieverName": "string",
      "referenceId": "string",
      "isViewReceiptEnabled": true
    }
  ],
  "successful": true,
  "message": "string"
}
```


### `60dc94b32da0b12d5029aeb0`  ·  Onboarding Open API 
*Web API for account opening by partners.*  
- Base path: `/onboarding-open`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### POST `/onboarding-open/api/CustomerProfile/B2BOnboardingValidation`
- Operation: *1 - B2BOnboardingValidation*
- Request body `B2BOnboardingValidationRequest`:
```json
{
  "bvn": "string",
  "selfieImage": "string",
  "channelID": "3fa85f64-5717-4562-b3fc-2c963f66afa6"
}
```
- Response 200 `B2BValidationResponse`:
```json
{
  "message": "string",
  "status": true,
  "code": "BVNFacialValidationSuccesfulProceedToOnboarding",
  "trackingId": "string"
}
```

#### POST `/onboarding-open/api/CustomerProfile/B2BOnboarding`
- Operation: *2 - B2BOnboarding - POST*
- Request body `B2BOnboardingRequest`:
```json
{
  "bvn": "string",
  "signatureImage": "string",
  "selfieImage": "string",
  "channelID": "string"
}
```
- Response 200 `B2BOnboardingResponseResponseModel`:
```json
{
  "data": {
    "step": "string",
    "customerID": "string",
    "account": "string",
    "otpDestination": "string",
    "OtpTrackingID": "string"
  },
  "message": "string",
  "status": false,
  "code": "string"
}
```

#### POST `/onboarding-open/api​/verification​/v2​/GenerateOTPForB2B`
- Operation: *3: Re-GenerateOTP For BVN Validation*
- Request body `B2BOTPRequestV2`:
```json
{
  "bvn": "string",
  "channelID": "string"
}
```
- Response 200 `B2BOTPResponseModel`:
```json
{
  "otpTrackingID": "string",
  "message": "string",
  "isOtpGenerated": false
}
```

#### POST `/onboarding-open/api/verification/ValidateOtpV3`
- Operation: *4: Validate OTP For BVN Validation*
- Request body `ValidateModelV3`:
```json
{
  "trackingId": "string",
  "channelId": "string",
  "otp": "string"
}
```
- Response 200 `OtpValidationRes`:
```json
{
  "isValidated": true,
  "message": "string"
}
```

#### POST `/onboarding-open/api/CustomerProfile/CheckCustomerStatus`
- Operation: *5 - CheckCustomerStatus - POST*
- Request body `CustomerStatusRequest`:
```json
{
  "bvn": "string",
  "channelID": "string",
  "email": "string",
  "phoneNumber": "string"
}
```
- Response 200 `B2BOnboardingResponseResponseModel`:
```json
{
  "data": {
    "step": "InvalidBvn",
    "cif": "string",
    "account": "string",
    "otpDestination": "string"
  },
  "message": "string",
  "status": true,
  "code": "InvalidBvn"
}
```

#### POST `/onboarding-open/api/CustomerInfo/AddResidentialAddressV2`
- Operation: *6 - AddResidentialAddressV2 - POST*
- Request body `ResidentialAddressDto`:
```json
{
  "cif": "string",
  "buildingNumber": "string",
  "apartment": "string",
  "street": "string",
  "city": "string",
  "town": "string",
  "state": "string",
  "lga": "string",
  "lcda": "string",
  "landmark": "string",
  "additionalInformation": "string",
  "country": "string",
  "fullAddress": "string",
  "channelID": "string",
  "channelName": "string"
}
```
- Response 200 `ResponseModel`:
```json
{
  "message": "string",
  "status": true,
  "code": "InvalidBvn"
}
```

#### GET `/onboarding-open/api/CustomerInfo/GetDropDownList`
- Operation: *7 - GetDropDownListForLocation - GET*
- Response 200 `ContactDetailDropDownModel`:
```json
{
  "countryModel": {
    "countryList": [
      {
        "id": 0,
        "countryCode": "string",
        "countryName": "string"
      }
    ],
    "stateList": [
      {
        "id": 0,
        "name": "string",
        "finacleCode": "string",
        "country": "string"
      }
    ],
    "lgaList": [
      {
        "lgaId": 0,
        "stateId": 0,
        "name": "string"
      }
    ],
    "lcdaList": [
      {
        "lcdaId": 0,
        "lgaId": 0,
        "name": "string"
      }
    ],
    "cityList": [
      {
        "id": 0,
        "stateId": 0,
        "name": "string"
      }
    ]
  }
}
```

#### POST `/onboarding-open/api/CustomerInfo/documentV2`
- Operation: *8 - DocumentV2 - POST*
- Request body `CustomerDocUploadRequest`:
```json
{
  "clientName": "string",
  "clientId": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
  "cif": "string",
  "fullName": "string",
  "document": {
    "idNumber": "string",
    "typeFront": "Identity",
    "typeBack": "Identity",
    "identityType": "International_Passport",
    "uploadReason": "AccountReactivation",
    "extension": "JPG",
    "issueDate": "2021-07-09T11:49:38.812Z",
    "placeOfIssue": "string",
    "expiryDate": "2021-07-09T11:49:38.812Z",
    "base64EncodedImageFront": "string",
    "base64EncodedImageBack": "string"
  }
}
```
- Response 200 `StringResponseModel`:
```json
{
  "data": "string",
  "message": "string",
  "status": true,
  "code": "InvalidBvn"
}
```

#### POST `/onboarding-open/api/Mandate/AddMandateV2`
- Operation: *9 - Re-upload Mandate*
- Request body `MandateRequestUsingBvn`:
```json
{
  "bvn": "string",
  "type": "string",
  "clientId": "string",
  "clientName": "string",
  "base64Image": "string"
}
```
- Response 200 `MandateUploadResponse`:
```json
{
  "status": true,
  "message": "string",
  "mandateId": "string"
}
```

#### POST `/onboarding-open/api/verification/GenerateOTPForB2B`
- Operation: *Generate OTP For B2B*
- Request body `B2BOTPRequest`:
```json
{
  "phoneNumber": "string",
  "channelID": "3fa85f64-5717-4562-b3fc-2c963f66afa6"
}
```
- Response 200 `B2BOTPResponseModel`:
```json
{
  "otpTrackingID": "string",
  "message": "string",
  "isOtpGenerated": true
}
```

#### POST `/onboarding-open/api/verification/ValidateOtpV2`
- Operation: *Validate OTP*
- Request body `ValidateOTP`:
```json
{
  "trackingId": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
  "otp": "string",
  "destination": "string"
}
```
- Response 200 `OtpValidationRes`:
```json
{
  "isValidated": true,
  "message": "string"
}
```


### `63777129c37da3dbbd4fcbe8`  ·  Funds Transfer API
*APIs for funds transfer within banks.*  
- Base path: `/funds-transfer`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### GET `/funds-transfer/api/OpenApiTransfer/GetAllBanks`
- Operation: *1 - GetBankList*
- Response 200 `BanksOpenApiServiceResponse`:
```json
{
  "successful": true,
  "message": "string",
  "result": {
    "bankName": "string",
    "bankCode": "string"
  }
}
```

#### GET `/funds-transfer/api/Shared/AccountNameEnquiry/{bankCode}/{accountNumber}`
- Operation: *2 - AccountNameEnquiry*
- Path params: `bankCode:string`(req), `accountNumber:string`(req)
- Query params: `channelId:string`
- Response 200 `AccountNameEnquiryEnvelope`:
```json
{
  "result": {
    "bankCode": "string",
    "accountName": "string",
    "accountNumber": "string",
    "currency": "string",
    "termsAndConditions": "string",
    "termsAndConditionsUrl": "string",
    "chargeFee": [
      {
        "id": 0,
        "chargeFeeName": "string",
        "transactionType": 0,
        "charge": 0.0,
        "lower": 0.0,
        "upper": 0.0
      }
    ]
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```

#### POST `/funds-transfer/api/InterbankTransfer/Transfer`
- Operation: *3 - TransferFunds*
- Request body `OpenApiTransferRequest`:
```json
{
  "amount": 0,
  "destinationBankCode": "string",
  "destinationBankName": "string",
  "destinationAccountNumber": "string",
  "destinationAccountName": "string",
  "sourceAccountNumber": "string",
  "channelId": "string",
  "narration": "string",
  "transactionReference": "string",
  "recipientEmail": "string",
  "transactionType": 1,
  "authOptions": {
    "pin": "string",
    "otp": "string",
    "biometricPolicy": "string",
    "biometricToken": "string",
    "platformTransactionReference": "string",
    "authenticationType": 0
  }
}
```
- Response 200 `OpenAPITransactionResponseOpenApiServiceResponse`:
```json
{
  "successful": true,
  "message": "string",
  "result": {
    "status": {},
    "message": "string",
    "platformTransactionReference": "string"
  }
}
```


### `63a08ac93acd2c5c3ceaabcf`  ·  ALAT Faith - Tems APIs
*API to aid collections for religious organizations*  
- Base path: `/alat-faith-tems`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### GET `/alat-faith-tems/api/v{version}/Customer/TamsCustomerDetails/{accountNumber}`
- Operation: *Get customer details by account number*
- Path params: `version:string`(req), `accountNumber:string`(req)
- Response 200 `GetCustomerDetailsResponseDTOServiceResponse`:
```json
{
  "message": "string",
  "responseCode": 10,
  "response": {
    "name": "string",
    "emailAddress": "string",
    "phoneNumber": "string"
  }
}
```


### `account-creation-api`  ·  Consortium Account Creation API
*Web API for creating Consortium Accounts*  
- Base path: `/consortium-acct-creation`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### POST `/consortium-acct-creation/api/CustomerProfile/PartnerOnboarding`
- Operation: */api/CustomerProfile/PartnerOnboarding - POST*
- Required headers: `x-api-key`(req)
- Request body `FlutterWaveAPIModel`:
```json
{
  "preferredDeliveryDate": "2023-09-08T23:13:44.996Z",
  "cardType": "string",
  "bvn": "string",
  "email": "string",
  "customerReference": "string",
  "phoneNumber": "string",
  "address": {
    "buildingNumber": "string",
    "apartment": "string",
    "street": "string",
    "city": "string",
    "town": "string",
    "state": "string",
    "lga": "string",
    "lcda": "string",
    "landmark": "string",
    "additionalInformation": "string",
    "country": "string",
    "fullAddress": "string"
  },
  "idCardString": "string",
  "idCardBackString": "string"
}
```
- Response 200 `ResponseModel`:
```json
{
  "message": "string",
  "status": true,
  "code": "InvalidBvn"
}
```


### `account-management-api-consortium-accounts`  ·  Consortium Account Management API
*Web API for management of consortium accounts*  
- Base path: `/consortium-acct-mgt`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### GET `/consortium-acct-mgt/api/PartnerAccountManagement/Account`
- Operation: */api/PartnerAccountManagement/Account - GET*
- Query params: `accountNumber:string`
- Response 200 `FWAccountReponseServiceResponse`:
```json
{
  "result": {
    "accountNumber": "string",
    "availableBalance": "string",
    "accountStatus": "string",
    "accountType": "string"
  },
  "successful": true,
  "message": "string"
}
```

#### POST `/consortium-acct-mgt/api/PartnerAccountManagement/TransactionHistory`
- Operation: */api/PartnerAccountManagement/TransactionHistory - POST*
- Request body `StatementForLastTransDto`:
```json
{
  "email": "string",
  "count": 0,
  "accountNumber": "string"
}
```


### `airtime-api`  ·  Airtime Open API
*API to purchase airtime and data.*  
- Base path: `/airtime`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### GET `/airtime/api/OpenApi/GetDataPlans`
- Operation: *GetDataPlans - GET*
- Response 200 `PackagesResponseListEnvelope`:
```json
{
  "result": [
    {
      "id": 0,
      "networkProvider": "string",
      "dataPackages": [
        {
          "id": 0,
          "name": "string",
          "amount": 0.0,
          "dataPlan": "string",
          "validity_Period": "string",
          "description": "string"
        }
      ]
    }
  ],
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```

#### POST `/airtime/api/OpenApi/PurchaseAirtime`
- Operation: *PurchaseAirtime - POST*
- Required headers: `hash`(req)
- Request body `OpenApiAirtimeRequest`:
```json
{
  "clientTransactionReference": "string",
  "network": "string",
  "accountNumber": "string",
  "phoneNumber": "string",
  "amount": 0.0
}
```
- Response 200 `GlobalResponseEnvelope`:
```json
{
  "result": {
    "status": "string",
    "message": "string",
    "narration": "string",
    "transactionReference": "string",
    "platformTransactionReference": "string",
    "transactionStan": "string"
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```

#### POST `/airtime/api/OpenApi/PurchaseData`
- Operation: *PurchaseData - POST*
- Required headers: `hash`(req)
- Request body `OpenApiDataRequest`:
```json
{
  "clientTransactionReference": "string",
  "accountNumber": "string",
  "phoneNumber": "string",
  "packageCode": 0
}
```
- Response 200 `GlobalResponseEnvelope`:
```json
{
  "result": {
    "status": "string",
    "message": "string",
    "narration": "string",
    "transactionReference": "string",
    "platformTransactionReference": "string",
    "transactionStan": "string"
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```


### `airtime-api-clickatell`  ·  Airtime API Clickatell
*API to purchase airtime and data*  
- Base path: `/airtime-int-clickatell`  |  subscriptionRequired: `False`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### POST `/airtime-int-clickatell/api/Airtime/ClickatellCallback`
- Operation: *Callback endpoint for Clickatell*
- Request body `ClickatellCallback`:
```json
{
  "clientTxnRef": "string",
  "raasTxnRef": "string",
  "reserveFundsTxnRef": "string",
  "responseCode": "string",
  "reserveAmount": 0,
  "feeAmount": 0,
  "clientShareAmount": 0,
  "settlementAmount": 0,
  "timestamp": "string",
  "token": "string"
}
```

#### GET `/airtime-int-clickatell/api/Data/GetDataPlans`
- Operation: *Get data plans*
- Response 200 `PackagesResponseListEnvelope`:
```json
{
  "result": [
    {
      "id": 0,
      "networkProvider": "string",
      "dataPackages": [
        {
          "id": 0,
          "name": "string",
          "amount": 0,
          "dataPlan": "string",
          "validity_Period": "string",
          "description": "string"
        }
      ]
    }
  ],
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```

#### POST `/airtime-int-clickatell/api/Airtime/PurchaseAirtimeWithPin`
- Operation: *Purchase airtime with PIN*
- Request body `AirtimeRequestWithPIN`:
```json
{
  "clientTransactionReference": "string",
  "accountNumber": "string",
  "cif": "string",
  "network": "string",
  "phoneNumber": "string",
  "amount": 0,
  "pin": "string",
  "channelId": "string",
  "securityInfo": "string",
  "isForPoint": true
}
```
- Response 200 `GlobalResponseEnvelope`:
```json
{
  "result": {
    "status": "string",
    "message": "string",
    "narration": "string",
    "transactionReference": "string",
    "platformTransactionReference": "string",
    "transactionStan": "string",
    "orinalTxnTransactionDate": "string"
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```

#### POST `/airtime-int-clickatell/api/Data/PurchaseDataWithPin`
- Operation: *Purchase data with PIN*
- Request body `DataRequestWithPin`:
```json
{
  "clientTransactionReference": "string",
  "accountNumber": "string",
  "phoneNumber": "string",
  "packageCode": 0,
  "pin": "string",
  "amount": 0,
  "network": "string",
  "channelId": "string",
  "cif": "string",
  "securityInfo": "string",
  "isForPoint": true
}
```
- Response 200 `GlobalResponseEnvelope`:
```json
{
  "result": {
    "status": "string",
    "message": "string",
    "narration": "string",
    "transactionReference": "string",
    "platformTransactionReference": "string",
    "transactionStan": "string",
    "orinalTxnTransactionDate": "string"
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```


### `alat-cards-api-v2`  ·  Alat Cards API V2
*Web API For Alat Cards*  
- Base path: `/alat-card-v2`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### POST `/alat-card-v2/api/Cards/ActivateCard`
- Operation: */api/Cards/ActivateCard - POST*
- Request body `ActivateCardDto`:
```json
{
  "profileId": "string",
  "accountNumber": "string",
  "newCardPin": "string",
  "panLast4": "string",
  "pin": "string",
  "expiryDate": "string",
  "cvV2": "string",
  "phoneNumber": "string",
  "username": "string",
  "emailAddress": "string",
  "cardRouteId": 0
}
```
- Response 200 `BooleanResponseModel`:
```json
{
  "data": true,
  "message": "string",
  "status": true,
  "code": 1
}
```

#### POST `/alat-card-v2/api/Cards/ActivateCardV2`
- Operation: */api/Cards/ActivateCardV2 - POST*
- Request body `PostillionSelectCardPinDto`:
```json
{
  "clientID": "string",
  "apiKey": "string",
  "accountNumber": "string",
  "newPin": "string",
  "expiryDate": "string",
  "emailAddress": "string",
  "fullPan": "string",
  "cardRouteId": 0
}
```
- Response 200 `StringPinResponseModel`:
```json
{
  "result": "string",
  "successful": true,
  "message": "string"
}
```

#### GET `/alat-card-v2/api/Cards/BackOfficeCardDeliveryFromPerso`
- Operation: */api/Cards/BackOfficeCardDeliveryFromPerso - GET*
- Query params: `startDate:string`, `endDate:string`
- Response 200 `InAppCardDeliveryFromPersoWrapperResponseModel`:
```json
{
  "data": {
    "totalEMPProcessed": 0,
    "cardSentForProcessing": [
      {
        "nameOnCard": "string",
        "cardRequestReference": "string",
        "accountNumber": "string",
        "dateEMPProcessed": "string",
        "status": "string"
      }
    ]
  },
  "message": "string",
  "status": true,
  "code": 1
}
```

#### GET `/alat-card-v2/api/Cards/BackOfficeCardDeliveryStatus`
- Operation: */api/Cards/BackOfficeCardDeliveryStatus - GET*
- Query params: `startDate:string`, `endDate:string`
- Response 200 `InAppCardDeliveryStatusWrapperResponseModel`:
```json
{
  "data": {
    "totalRequestCreated": 0,
    "totalEMPProcessing": 0,
    "totalErrorUploading": 0,
    "totalEMPProcessed": 0,
    "totalErrorProcessing": 0,
    "totalReceivedByGSD": 0,
    "totalEnrouteDelivery": 0,
    "totalDelivered": 0,
    "totalReturnedForDelivery": 0,
    "requestCreated": [
      {
        "cardRequestId": "string",
        "cardType": "string",
        "deliveryStatus": "string",
        "slaStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "jumiaDeliveryStatus": "string",
        "cif": "string",
        "accountName": "string",
        "accountNumber": "string",
        "deliveryAddress": "string",
        "courierForDelivery": "string",
        "estimatedDeliveryDate": "string",
        "dateAssignedToCourier": "string",
        "dateReadyFordelivery": "string",
        "dateOfEMPProcessed": "string",
        "dateReceivedByGSD": "string",
        "dateReturnedForDelivery": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "customerComplaint": 0,
        "lastEventDate": "string",
        "urlToTrackingPortalOfCourier": "string"
      }
    ],
    "empProcessing": [
      {
        "cardRequestId": "string",
        "cardType": "string",
        "deliveryStatus": "string",
        "slaStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "jumiaDeliveryStatus": "string",
        "cif": "string",
        "accountName": "string",
        "accountNumber": "string",
        "deliveryAddress": "string",
        "courierForDelivery": "string",
        "estimatedDeliveryDate": "string",
        "dateAssignedToCourier": "string",
        "dateReadyFordelivery": "string",
        "dateOfEMPProcessed": "string",
        "dateReceivedByGSD": "string",
        "dateReturnedForDelivery": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "customerComplaint": 0,
        "lastEventDate": "string",
        "urlToTrackingPortalOfCourier": "string"
      }
    ],
    "errorUploading": [
      {
        "cardRequestId": "string",
        "cardType": "string",
        "deliveryStatus": "string",
        "slaStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "jumiaDeliveryStatus": "string",
        "cif": "string",
        "accountName": "string",
        "accountNumber": "string",
        "deliveryAddress": "string",
        "courierForDelivery": "string",
        "estimatedDeliveryDate": "string",
        "dateAssignedToCourier": "string",
        "dateReadyFordelivery": "string",
        "dateOfEMPProcessed": "string",
        "dateReceivedByGSD": "string",
        "dateReturnedForDelivery": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "customerComplaint": 0,
        "lastEventDate": "string",
        "urlToTrackingPortalOfCourier": "string"
      }
    ],
    "empProcessed": [
      {
        "cardRequestId": "string",
        "cardType": "string",
        "deliveryStatus": "string",
        "slaStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "jumiaDeliveryStatus": "string",
        "cif": "string",
        "accountName": "string",
        "accountNumber": "string",
        "deliveryAddress": "string",
        "courierForDelivery": "string",
        "estimatedDeliveryDate": "string",
        "dateAssignedToCourier": "string",
        "dateReadyFordelivery": "string",
        "dateOfEMPProcessed": "string",
        "dateReceivedByGSD": "string",
        "dateReturnedForDelivery": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "customerComplaint": 0,
        "lastEventDate": "string",
        "urlToTrackingPortalOfCourier": "string"
      }
    ],
    "errorProcessing": [
      {
        "cardRequestId": "string",
        "cardType": "string",
        "deliveryStatus": "string",
        "slaStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "jumiaDeliveryStatus": "string",
        "cif": "string",
        "accountName": "string",
        "accountNumber": "string",
        "deliveryAddress": "string",
        "courierForDelivery": "string",
        "estimatedDeliveryDate": "string",
        "dateAssignedToCourier": "string",
        "dateReadyFordelivery": "string",
        "dateOfEMPProcessed": "string",
        "dateReceivedByGSD": "string",
        "dateReturnedForDelivery": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "customerComplaint": 0,
        "lastEventDate": "string",
        "urlToTrackingPortalOfCourier": "string"
      }
    ],
    "receivedByGSD": [
      {
        "cardRequestId": "string",
        "cardType": "string",
        "deliveryStatus": "string",
        "slaStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "jumiaDeliveryStatus": "string",
        "cif": "string",
        "accountName": "string",
        "accountNumber": "string",
        "deliveryAddress": "string",
        "courierForDelivery": "string",
        "estimatedDeliveryDate": "string",
        "dateAssignedToCourier": "string",
        "dateReadyFordelivery": "string",
        "dateOfEMPProcessed": "string",
        "dateReceivedByGSD": "string",
        "dateReturnedForDelivery": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "customerComplaint": 0,
        "lastEventDate": "string",
        "urlToTrackingPortalOfCourier": "string"
      }
    ],
    "enrouteDelivery": [
      {
        "cardRequestId": "string",
        "cardType": "string",
        "deliveryStatus": "string",
        "slaStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "jumiaDeliveryStatus": "string",
        "cif": "string",
        "accountName": "string",
        "accountNumber": "string",
        "deliveryAddress": "string",
        "courierForDelivery": "string",
        "estimatedDeliveryDate": "string",
        "dateAssignedToCourier": "string",
        "dateReadyFordelivery": "string",
        "dateOfEMPProcessed": "string",
        "dateReceivedByGSD": "string",
        "dateReturnedForDelivery": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "customerComplaint": 0,
        "lastEventDate": "string",
        "urlToTrackingPortalOfCourier": "string"
      }
    ],
    "delivered": [
      {
        "cardRequestId": "string",
        "cardType": "string",
        "deliveryStatus": "string",
        "slaStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "jumiaDeliveryStatus": "string",
        "cif": "string",
        "accountName": "string",
        "accountNumber": "string",
        "deliveryAddress": "string",
        "courierForDelivery": "string",
        "estimatedDeliveryDate": "string",
        "dateAssignedToCourier": "string",
        "dateReadyFordelivery": "string",
        "dateOfEMPProcessed": "string",
        "dateReceivedByGSD": "string",
        "dateReturnedForDelivery": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "customerComplaint": 0,
        "lastEventDate": "string",
        "urlToTrackingPortalOfCourier": "string"
      }
    ],
    "returnedForDelivery": [
      {
        "cardRequestId": "string",
        "cardType": "string",
        "deliveryStatus": "string",
        "slaStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "jumiaDeliveryStatus": "string",
        "cif": "string",
        "accountName": "string",
        "accountNumber": "string",
        "deliveryAddress": "string",
        "courierForDelivery": "string",
        "estimatedDeliveryDate": "string",
        "dateAssignedToCourier": "string",
        "dateReadyFordelivery": "string",
        "dateOfEMPProcessed": "string",
        "dateReceivedByGSD": "string",
        "dateReturnedForDelivery": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "customerComplaint": 0,
        "lastEventDate": "string",
        "urlToTrackingPortalOfCourier": "string"
      }
    ]
  },
  "message": "string",
  "status": true,
  "code": 1
}
```

#### GET `/alat-card-v2/api/Cards/BackOfficeCardDetails`
- Operation: */api/Cards/BackOfficeCardDetails - GET*
- Query params: `cardRequestReference:string`
- Response 200 `BackOfficeInAppCardDetails`:
```json
{
  "cif": "string",
  "cardRequestReference": "string",
  "phoneNumber": "string",
  "emailAddress": "string",
  "deliveryAddress": "string",
  "trackingId": "string",
  "courierForDelivery": "string",
  "estimatedDeliveryDate": "string",
  "dateOfEMPProcessed": "string",
  "dateReceivedByGSD": "string",
  "dateReturnedForDelivery": "string",
  "requestDate": "string",
  "customerComplaint": 0,
  "deliveryStatus": "string"
}
```

#### GET `/alat-card-v2/api/Cards/BackOfficeCardReports`
- Operation: */api/Cards/BackOfficeCardReports - GET*
- Query params: `startDate:string`, `endDate:string`
- Response 200 `InAppCardReportWrapperResponseModel`:
```json
{
  "data": {
    "totalRequest": 0,
    "totalSuccessfulRequests": 0,
    "totalActivated": 0,
    "totalSentForProcessing": 0,
    "totalDebitedSuccessful": 0,
    "totalDebitedSuccessfulButAwaitingProcessing": 0,
    "totalWithTrackingId": 0,
    "totalNotDeliveredAndNotActivated": 0,
    "totalDeliveredButNotActivated": 0,
    "totalNotSentForProcessing": 0,
    "totalWithoutTrackingID": 0,
    "totalFailedRequests": 0,
    "totalJumiaList": 0,
    "totalJumiaDeliverySuccessful": 0,
    "totalJumiaDeliveryPending": 0,
    "totalJumiaDeliveryStatusPassedSLA": 0,
    "totalUPSList": 0,
    "totalUPSDeliverySuccessful": 0,
    "totalUPSDeliveryPending": 0,
    "totalUPSDeliveryPassedSLA": 0,
    "totalSuccessfulDelivery": 0,
    "totalSLABreached": 0,
    "totalRequestCreated": 0,
    "totalEMPProcessing": 0,
    "totalErrorUploading": 0,
    "totalEMPProcessed": 0,
    "totalErrorProcessing": 0,
    "totalReceivedByGSD": 0,
    "totalEnrouteDelivery": 0,
    "empProcessing": [
      {
        "cardRequestId": "string",
        "cardType": "string",
        "deliveryStatus": "string",
        "slaStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "jumiaDeliveryStatus": "string",
        "cif": "string",
        "accountName": "string",
        "accountNumber": "string",
        "deliveryAddress": "string",
        "courierForDelivery": "string",
        "estimatedDeliveryDate": "string",
        "dateAssignedToCourier": "string",
        "dateReadyFordelivery": "string",
        "dateOfEMPProcessed": "string",
        "dateReceivedByGSD": "string",
        "dateReturnedForDelivery": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "customerComplaint": 0,
        "lastEventDate": "string",
        "urlToTrackingPortalOfCourier": "string"
      }
    ],
    "errorUploading": [
      {
        "cardRequestId": "string",
        "cardType": "string",
        "deliveryStatus": "string",
        "slaStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "jumiaDeliveryStatus": "string",
        "cif": "string",
        "accountName": "string",
        "accountNumber": "string",
        "deliveryAddress": "string",
        "courierForDelivery": "string",
        "estimatedDeliveryDate": "string",
        "dateAssignedToCourier": "string",
        "dateReadyFordelivery": "string",
        "dateOfEMPProcessed": "string",
        "dateReceivedByGSD": "string",
        "dateReturnedForDelivery": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "customerComplaint": 0,
        "lastEventDate": "string",
        "urlToTrackingPortalOfCourier": "string"
      }
    ],
    "empProcessed": [
      {
        "cardRequestId": "string",
        "cardType": "string",
        "deliveryStatus": "string",
        "slaStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "jumiaDeliveryStatus": "string",
        "cif": "string",
        "accountName": "string",
        "accountNumber": "string",
        "deliveryAddress": "string",
        "courierForDelivery": "string",
        "estimatedDeliveryDate": "string",
        "dateAssignedToCourier": "string",
        "dateReadyFordelivery": "string",
        "dateOfEMPProcessed": "string",
        "dateReceivedByGSD": "string",
        "dateReturnedForDelivery": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "customerComplaint": 0,
        "lastEventDate": "string",
        "urlToTrackingPortalOfCourier": "string"
      }
    ],
    "errorProcessing": [
      {
        "cardRequestId": "string",
        "cardType": "string",
        "deliveryStatus": "string",
        "slaStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "jumiaDeliveryStatus": "string",
        "cif": "string",
        "accountName": "string",
        "accountNumber": "string",
        "deliveryAddress": "string",
        "courierForDelivery": "string",
        "estimatedDeliveryDate": "string",
        "dateAssignedToCourier": "string",
        "dateReadyFordelivery": "string",
        "dateOfEMPProcessed": "string",
        "dateReceivedByGSD": "string",
        "dateReturnedForDelivery": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "customerComplaint": 0,
        "lastEventDate": "string",
        "urlToTrackingPortalOfCourier": "string"
      }
    ],
    "receivedByGSD": [
      {
        "cardRequestId": "string",
        "cardType": "string",
        "deliveryStatus": "string",
        "slaStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "jumiaDeliveryStatus": "string",
        "cif": "string",
        "accountName": "string",
        "accountNumber": "string",
        "deliveryAddress": "string",
        "courierForDelivery": "string",
        "estimatedDeliveryDate": "string",
        "dateAssignedToCourier": "string",
        "dateReadyFordelivery": "string",
        "dateOfEMPProcessed": "string",
        "dateReceivedByGSD": "string",
        "dateReturnedForDelivery": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "customerComplaint": 0,
        "lastEventDate": "string",
        "urlToTrackingPortalOfCourier": "string"
      }
    ],
    "enrouteDelivery": [
      {
        "cardRequestId": "string",
        "cardType": "string",
        "deliveryStatus": "string",
        "slaStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "jumiaDeliveryStatus": "string",
        "cif": "string",
        "accountName": "string",
        "accountNumber": "string",
        "deliveryAddress": "string",
        "courierForDelivery": "string",
        "estimatedDeliveryDate": "string",
        "dateAssignedToCourier": "string",
        "dateReadyFordelivery": "string",
        "dateOfEMPProcessed": "string",
        "dateReceivedByGSD": "string",
        "dateReturnedForDelivery": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "customerComplaint": 0,
        "lastEventDate": "string",
        "urlToTrackingPortalOfCourier": "string"
      }
    ],
    "succeededRequests": [
      {
        "cardRequestId": "string",
        "cardType": "string",
        "deliveryStatus": "string",
        "slaStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "jumiaDeliveryStatus": "string",
        "cif": "string",
        "accountName": "string",
        "accountNumber": "string",
        "deliveryAddress": "string",
        "courierForDelivery": "string",
        "estimatedDeliveryDate": "string",
        "dateAssignedToCourier": "string",
        "dateReadyFordelivery": "string",
        "dateOfEMPProcessed": "string",
        "dateReceivedByGSD": "string",
        "dateReturnedForDelivery": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "customerComplaint": 0,
        "lastEventDate": "string",
        "urlToTrackingPortalOfCourier": "string"
      }
    ],
    "successfulDebitedAndAwaitingXMLGeneration": [
      {
        "cardRequestId": "string",
        "cardType": "string",
        "deliveryStatus": "string",
        "slaStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "jumiaDeliveryStatus": "string",
        "cif": "string",
        "accountName": "string",
        "accountNumber": "string",
        "deliveryAddress": "string",
        "courierForDelivery": "string",
        "estimatedDeliveryDate": "string",
        "dateAssignedToCourier": "string",
        "dateReadyFordelivery": "string",
        "dateOfEMPProcessed": "string",
        "dateReceivedByGSD": "string",
        "dateReturnedForDelivery": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "customerComplaint": 0,
        "lastEventDate": "string",
        "urlToTrackingPortalOfCourier": "string"
      }
    ],
    "cardSentForProcessing": [
      {
        "cardRequestId": "string",
        "cardType": "string",
        "deliveryStatus": "string",
        "slaStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "jumiaDeliveryStatus": "string",
        "cif": "string",
        "accountName": "string",
        "accountNumber": "string",
        "deliveryAddress": "string",
        "courierForDelivery": "string",
        "estimatedDeliveryDate": "string",
        "dateAssignedToCourier": "string",
        "dateReadyFordelivery": "string",
        "dateOfEMPProcessed": "string",
        "dateReceivedByGSD": "string",
        "dateReturnedForDelivery": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "customerComplaint": 0,
        "lastEventDate": "string",
        "urlToTrackingPortalOfCourier": "string"
      }
    ],
    "trackingIdGenerated": [
      {
        "cardRequestId": "string",
        "cardType": "string",
        "deliveryStatus": "string",
        "slaStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "jumiaDeliveryStatus": "string",
        "cif": "string",
        "accountName": "string",
        "accountNumber": "string",
        "deliveryAddress": "string",
        "courierForDelivery": "string",
        "estimatedDeliveryDate": "string",
        "dateAssignedToCourier": "string",
        "dateReadyFordelivery": "string",
        "dateOfEMPProcessed": "string",
        "dateReceivedByGSD": "string",
        "dateReturnedForDelivery": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "customerComplaint": 0,
        "lastEventDate": "string",
        "urlToTrackingPortalOfCourier": "string"
      }
    ],
    "jumiaDeliverySuccessful": [
      {
        "cardRequestId": "string",
        "cardType": "string",
        "deliveryStatus": "string",
        "slaStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "jumiaDeliveryStatus": "string",
        "cif": "string",
        "accountName": "string",
        "accountNumber": "string",
        "deliveryAddress": "string",
        "courierForDelivery": "string",
        "estimatedDeliveryDate": "string",
        "dateAssignedToCourier": "string",
        "dateReadyFordelivery": "string",
        "dateOfEMPProcessed": "string",
        "dateReceivedByGSD": "string",
        "dateReturnedForDelivery": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "customerComplaint": 0,
        "lastEventDate": "string",
        "urlToTrackingPortalOfCourier": "string"
      }
    ],
    "jumiaDeliveryPending": [
      {
        "cardRequestId": "string",
        "cardType": "string",
        "deliveryStatus": "string",
        "slaStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "jumiaDeliveryStatus": "string",
        "cif": "string",
        "accountName": "string",
        "accountNumber": "string",
        "deliveryAddress": "string",
        "courierForDelivery": "string",
        "estimatedDeliveryDate": "string",
        "dateAssignedToCourier": "string",
        "dateReadyFordelivery": "string",
        "dateOfEMPProcessed": "string",
        "dateReceivedByGSD": "string",
        "dateReturnedForDelivery": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "customerComplaint": 0,
        "lastEventDate": "string",
        "urlToTrackingPortalOfCourier": "string"
      }
    ],
    "jumiaDeliveryPassedSLA": [
      {
        "cardRequestId": "string",
        "cardType": "string",
        "deliveryStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "jumiaDeliveryStatus": "string",
        "cif": "string",
        "accountName": "string",
        "accountNumber": "string",
        "deliveryAddress": "string",
        "courierForDelivery": "string",
        "estimatedDeliveryDate": "string",
        "dateAssignedToCourier": "string",
        "dateReadyFordelivery": "string",
        "dateOfEMPProcessed": "string",
        "dateReceivedByGSD": "string",
        "dateReturnedForDelivery": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "customerComplaint": 0,
        "lastEventDate": "string",
        "slaStatus": "string",
        "urlToTrackingPortalOfCourier": "string"
      }
    ],
    "upsDeliverySuccessful": [
      {
        "cardRequestId": "string",
        "cardType": "string",
        "deliveryStatus": "string",
        "slaStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "jumiaDeliveryStatus": "string",
        "cif": "string",
        "accountName": "string",
        "accountNumber": "string",
        "deliveryAddress": "string",
        "courierForDelivery": "string",
        "estimatedDeliveryDate": "string",
        "dateAssignedToCourier": "string",
        "dateReadyFordelivery": "string",
        "dateOfEMPProcessed": "string",
        "dateReceivedByGSD": "string",
        "dateReturnedForDelivery": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "customerComplaint": 0,
        "lastEventDate": "string",
        "urlToTrackingPortalOfCourier": "string"
      }
    ],
    "upsDeliveryPending": [
      {
        "cardRequestId": "string",
        "cardType": "string",
        "deliveryStatus": "string",
        "slaStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "jumiaDeliveryStatus": "string",
        "cif": "string",
        "accountName": "string",
        "accountNumber": "string",
        "deliveryAddress": "string",
        "courierForDelivery": "string",
        "estimatedDeliveryDate": "string",
        "dateAssignedToCourier": "string",
        "dateReadyFordelivery": "string",
        "dateOfEMPProcessed": "string",
        "dateReceivedByGSD": "string",
        "dateReturnedForDelivery": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "customerComplaint": 0,
        "lastEventDate": "string",
        "urlToTrackingPortalOfCourier": "string"
      }
    ],
    "upsDeliveryPassedSLA": [
      {
        "cardRequestId": "string",
        "cardType": "string",
        "deliveryStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "jumiaDeliveryStatus": "string",
        "cif": "string",
        "accountName": "string",
        "accountNumber": "string",
        "deliveryAddress": "string",
        "courierForDelivery": "string",
        "estimatedDeliveryDate": "string",
        "dateAssignedToCourier": "string",
        "dateReadyFordelivery": "string",
        "dateOfEMPProcessed": "string",
        "dateReceivedByGSD": "string",
        "dateReturnedForDelivery": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "customerComplaint": 0,
        "lastEventDate": "string",
        "slaStatus": "string",
        "urlToTrackingPortalOfCourier": "string"
      }
    ],
    "cardDeliveredButNotActivated": [
      {
        "cardRequestId": "string",
        "cardType": "string",
        "deliveryStatus": "string",
        "slaStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "jumiaDeliveryStatus": "string",
        "cif": "string",
        "accountName": "string",
        "accountNumber": "string",
        "deliveryAddress": "string",
        "courierForDelivery": "string",
        "estimatedDeliveryDate": "string",
        "dateAssignedToCourier": "string",
        "dateReadyFordelivery": "string",
        "dateOfEMPProcessed": "string",
        "dateReceivedByGSD": "string",
        "dateReturnedForDelivery": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "customerComplaint": 0,
        "lastEventDate": "string",
        "urlToTrackingPortalOfCourier": "string"
      }
    ],
    "cardDeliveredAndActivated": [
      {
        "cardRequestId": "string",
        "cardType": "string",
        "deliveryStatus": "string",
        "slaStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "jumiaDeliveryStatus": "string",
        "cif": "string",
        "accountName": "string",
        "accountNumber": "string",
        "deliveryAddress": "string",
        "courierForDelivery": "string",
        "estimatedDeliveryDate": "string",
        "dateAssignedToCourier": "string",
        "dateReadyFordelivery": "string",
        "dateOfEMPProcessed": "string",
        "dateReceivedByGSD": "string",
        "dateReturnedForDelivery": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "customerComplaint": 0,
        "lastEventDate": "string",
        "urlToTrackingPortalOfCourier": "string"
      }
    ],
    "successCardDelivered": [
      {
        "cardRequestId": "string",
        "cardType": "string",
        "deliveryStatus": "string",
        "slaStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "jumiaDeliveryStatus": "string",
        "cif": "string",
        "accountName": "string",
        "accountNumber": "string",
        "deliveryAddress": "string",
        "courierForDelivery": "string",
        "estimatedDeliveryDate": "string",
        "dateAssignedToCourier": "string",
        "dateReadyFordelivery": "string",
        "dateOfEMPProcessed": "string",
        "dateReceivedByGSD": "string",
        "dateReturnedForDelivery": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "customerComplaint": 0,
        "lastEventDate": "string",
        "urlToTrackingPortalOfCourier": "string"
      }
    ],
    "failedRequests": [
      {
        "cardRequestId": "string",
        "cardType": "string",
        "deliveryStatus": "string",
        "emailAddress": "string",
        "phoneNumber": "string",
        "requestAge": "string",
        "isPassedSLA": true,
        "cif": "string",
        "nameOnCard": "string",
        "accountNumber": "string",
        "address": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "lastEventDate": "string"
      }
    ],
    "cardRequestDetail": [
      {
        "cardRequestId": "string",
        "nameOnCard": "string",
        "accountNumber": "string",
        "phoneNumber": "string",
        "requestDate": "string",
        "transactionDate": "string",
        "processingDate": "string",
        "trackingId": "string",
        "deliveryStatus": "string",
        "activatedDate": "string"
      }
    ],
    "slaBreached": [
      {
        "cardRequestId": "string",
        "cardType": "string",
        "deliveryStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "jumiaDeliveryStatus": "string",
        "cif": "string",
        "accountName": "string",
        "accountNumber": "string",
        "deliveryAddress": "string",
        "courierForDelivery": "string",
        "estimatedDeliveryDate": "string",
        "dateAssignedToCourier": "string",
        "dateReadyFordelivery": "string",
        "dateOfEMPProcessed": "string",
        "dateReceivedByGSD": "string",
        "dateReturnedForDelivery": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "customerComplaint": 0,
        "lastEventDate": "string",
        "slaStatus": "string",
        "urlToTrackingPortalOfCourier": "string"
      }
    ]
  },
  "message": "string",
  "status": true,
  "code": 1
}
```

#### GET `/alat-card-v2/api/Cards/BackOfficeCustomerComplaint`
- Operation: */api/Cards/BackOfficeCustomerComplaint - GET*
- Query params: `startDate:string`, `endDate:string`
- Response 200 `CustomerComplaintListListResponseModel`:
```json
{
  "data": [
    {
      "complaintId": 0,
      "nameOfCustomer": "string",
      "cif": "string",
      "phoneNumber": "string",
      "emailAddres": "string",
      "deliveryAddress": "string",
      "dateOfCardRequest": "string",
      "estimatedDeliveryDate": "string",
      "currentDeliveryStatus": "string",
      "dateCustomerNotifiedForDelivery": "string",
      "statusOfComplaint": 0,
      "cardRequestReference": "string",
      "dateOfLastEngagementwithCustomer": "string",
      "dateCommentWasSaved": "string",
      "comment": "string",
      "staffName": "string"
    }
  ],
  "message": "string",
  "status": true,
  "code": 1
}
```

#### POST `/alat-card-v2/api/Cards/CardDeliveryCallBack`
- Operation: */api/Cards/CardDeliveryCallBack - POST*
- Request body `CardTrackingDto`:
```json
{
  "parcel": {
    "sid": "string",
    "referenceNumber": "string",
    "trackingNumber": "string"
  },
  "trackingEvents": [
    {
      "event": "string",
      "eventDate": "string",
      "id": 0,
      "isFinal": true,
      "size": "string",
      "dimensions": {
        "weight": {
          "value": 0,
          "units": "string"
        },
        "width": {
          "value": 0,
          "units": "string"
        },
        "height": {
          "value": 0,
          "units": "string"
        },
        "length": {
          "value": 0,
          "units": "string"
        }
      },
      "reason": {
        "name": "string",
        "code": "string"
      }
    }
  ]
}
```
- Response 200 `CardTrackingResponse`:
```json
{
  "response": "string",
  "isSuccessful": true
}
```

#### GET `/alat-card-v2/api/Cards/CardsEnrouteDelivery`
- Operation: */api/Cards/CardsEnrouteDelivery - GET*
- Query params: `startDate:string`, `endDate:string`
- Response 200 `GSDDeliveryStatusListResponseModel`:
```json
{
  "data": [
    {
      "cardRequestReference": "string",
      "cardRequestId": "string",
      "accountName": "string",
      "phoneNumber": "string",
      "emailAddres": "string",
      "deliveryAddress": "string",
      "estimatedDeliveryDate": "string",
      "courierCompanyAssigned": "string",
      "dateAssignedToCourier": "string",
      "deliveryStatus": "string"
    }
  ],
  "message": "string",
  "status": true,
  "code": 1
}
```

#### GET `/alat-card-v2/api/Cards/CardsReadyForDelivery`
- Operation: */api/Cards/CardsReadyForDelivery - GET*
- Query params: `startDate:string`, `endDate:string`
- Response 200 `GSDDeliveryStatusListResponseModel`:
```json
{
  "data": [
    {
      "cardRequestReference": "string",
      "cardRequestId": "string",
      "accountName": "string",
      "phoneNumber": "string",
      "emailAddres": "string",
      "deliveryAddress": "string",
      "estimatedDeliveryDate": "string",
      "courierCompanyAssigned": "string",
      "dateAssignedToCourier": "string",
      "deliveryStatus": "string"
    }
  ],
  "message": "string",
  "status": true,
  "code": 1
}
```

#### GET `/alat-card-v2/api/Cards/CardsReturnedForDelivery`
- Operation: */api/Cards/CardsReturnedForDelivery - GET*
- Query params: `startDate:string`, `endDate:string`
- Response 200 `GSDDeliveryStatusListResponseModel`:
```json
{
  "data": [
    {
      "cardRequestReference": "string",
      "cardRequestId": "string",
      "accountName": "string",
      "phoneNumber": "string",
      "emailAddres": "string",
      "deliveryAddress": "string",
      "estimatedDeliveryDate": "string",
      "courierCompanyAssigned": "string",
      "dateAssignedToCourier": "string",
      "deliveryStatus": "string"
    }
  ],
  "message": "string",
  "status": true,
  "code": 1
}
```

#### POST `/alat-card-v2/api/Cards/ChangeCardPin`
- Operation: */api/Cards/ChangeCardPin - POST*
- Request body `PinChangeDto`:
```json
{
  "customerId": "string",
  "accountNumber": "string",
  "newCardPin": "string",
  "oldCardPin": "string",
  "pan": "string",
  "pin": "string",
  "expiryDate": "string",
  "cvV2": "string",
  "phoneNumber": "string",
  "username": "string",
  "emailAddress": "string",
  "cardRouteId": 0
}
```
- Response 200 `BooleanResponseModel`:
```json
{
  "data": true,
  "message": "string",
  "status": true,
  "code": 1
}
```

#### POST `/alat-card-v2/api/Cards/ChangeCardPinV2`
- Operation: */api/Cards/ChangeCardPinV2 - POST*
- Request body `PostillionChangeCardPinDto`:
```json
{
  "clientID": "string",
  "apiKey": "string",
  "accountNumber": "string",
  "expiryDate": "string",
  "oldCardPin": "string",
  "newCardPin": "string",
  "fullPan": "string",
  "emailAddress": "string",
  "cardRouteId": 0
}
```
- Response 200 `StringPinResponseModel`:
```json
{
  "result": "string",
  "successful": true,
  "message": "string"
}
```

#### GET `/alat-card-v2/api/Cards/CheckCardRequest/{cardRef}`
- Operation: */api/Cards/CheckCardRequest/{cardRef} - GET*
- Path params: `cardRef:string`(req)
- Response 200 `CheckCardResponseResponseModel`:
```json
{
  "data": {
    "accountDetails": {
      "accountNumber": "string",
      "accountName": "string",
      "accountType": "string",
      "accountSol": "string",
      "accountBranchName": "string",
      "accountCif": "string",
      "currency": "string",
      "schemeCode": "string"
    },
    "customerDetails": {
      "cif": "string",
      "customerType": "string",
      "name": "string",
      "mobileNumber": "string",
      "email": "string",
      "address": "string"
    },
    "linkedAccount": {
      "accountNumber": "string",
      "accountName": "string",
      "accountType": "string",
      "accountSol": "string",
      "accountBranchName": "string",
      "accountCif": "string",
      "currency": "string",
      "schemeCode": "string"
    },
    "card": {
      "id": "string",
      "cardProduct": "string",
      "maskedPan": "string",
      "expiryDate": "string",
      "linkedAccounts": [
        "string"
      ],
      "cardStatus": "string",
      "name": "string",
      "currency": "string",
      "fee": "string"
    },
    "processStatus": "string",
    "requestor": "string",
    "requestDate": "string",
    "authorizer": "string",
    "authorizedDate": "string",
    "concurrer": "string",
    "concurrenceDate": "string",
    "extractor": "string",
    "extractionDate": "string",
    "hoCardReceiver": "string",
    "hoCardReceiveDate": "string"
  },
  "message": "string",
  "status": true,
  "code": 1
}
```

#### GET `/alat-card-v2/api/Cards/CheckVCardRequestValidity/{id}`
- Operation: */api/Cards/CheckVCardRequestValidity/{id} - GET*
- Path params: `id:string`(req)
- Response 200 `BooleanResponseWrapper`:
```json
{
  "data": true,
  "message": "string",
  "messages": [
    "string"
  ],
  "hasMessage": true,
  "timeGenerated": "string"
}
```

#### GET `/alat-card-v2/api/Cards/CheckVirtualDollarCardV2RequestValidty/{id}`
- Operation: */api/Cards/CheckVirtualDollarCardV2RequestValidty/{id} - GET*
- Path params: `id:string`(req)
- Response 200 `BooleanResponseWrapper`:
```json
{
  "data": true,
  "message": "string",
  "messages": [
    "string"
  ],
  "hasMessage": true,
  "timeGenerated": "string"
}
```

#### GET `/alat-card-v2/api/Cards/CheckVirtualDollarCardV2Status`
- Operation: */api/Cards/CheckVirtualDollarCardV2Status - GET*
- Query params: `cif:string`
- Response 200 `FimiVCardInfoResponseVirtualDollarCardV2ResponseModel`:
```json
{
  "message": "string",
  "hasDollarCard": true,
  "costOfCard": 0,
  "exchangeRate": 0,
  "vat": "string"
}
```

#### POST `/alat-card-v2/api/Cards/CreateVCard`
- Operation: */api/Cards/CreateVCard - POST*
- Request body `CreateVCardReq`:
```json
{
  "customerId": "string",
  "channelId": "string",
  "firstName": "string",
  "lastName": "string",
  "name": "string",
  "pin": "string",
  "cif": "string",
  "amount": 0,
  "sourceAccount": "string",
  "schemeCode": "string",
  "emailAddress": "string",
  "birthday": "string",
  "residentStateExternalId": "string",
  "sex": "string",
  "residentCountry": 0,
  "birthplace": "string",
  "residentCityInLatin": "string",
  "addressInLatin": "string"
}
```
- Response 200 `StringResponseModel`:
```json
{
  "data": "string",
  "message": "string",
  "status": true,
  "code": 1
}
```

#### POST `/alat-card-v2/api/Cards/CreateVirtualDollarCardV2`
- Operation: */api/Cards/CreateVirtualDollarCardV2 - POST*
- Request body `CreateVCardReq`:
```json
{
  "customerId": "string",
  "channelId": "string",
  "firstName": "string",
  "lastName": "string",
  "name": "string",
  "pin": "string",
  "cif": "string",
  "amount": 0,
  "sourceAccount": "string",
  "schemeCode": "string",
  "emailAddress": "string",
  "birthday": "string",
  "residentStateExternalId": "string",
  "sex": "string",
  "residentCountry": 0,
  "birthplace": "string",
  "residentCityInLatin": "string",
  "addressInLatin": "string"
}
```
- Response 200 `StringResponseModel`:
```json
{
  "data": "string",
  "message": "string",
  "status": true,
  "code": 1
}
```

#### POST `/alat-card-v2/api/Cards/CustomerComplaintService`
- Operation: */api/Cards/CustomerComplaintService - POST*
- Request body `DelayComplainReq`:
```json
{
  "cif": "string",
  "requestedCardId": "string",
  "nameOnCard": "string",
  "accountNumber": "string"
}
```
- Response 200 `ComplaintResponseResponseModel`:
```json
{
  "data": {
    "isSuccessful": true,
    "response": "string"
  },
  "message": "string",
  "status": true,
  "code": 1
}
```

#### POST `/alat-card-v2/api/Cards/EngagementWithCustomer`
- Operation: */api/Cards/EngagementWithCustomer - POST*
- Request body `UpdateCustomerEngagement`:
```json
{
  "complaintId": 0,
  "deliveryAddress": "string",
  "estimatedDeliveryDate": "string",
  "comment": "string",
  "staffName": "string"
}
```
- Response 200 `BooleanResponseModel`:
```json
{
  "data": true,
  "message": "string",
  "status": true,
  "code": 1
}
```

#### POST `/alat-card-v2/api/Cards/ExcelUploadCardInformation`
- Operation: */api/Cards/ExcelUploadCardInformation - POST*
- Query params: `routeId:integer`
- Response 200 `BooleanResponseModel`:
```json
{
  "data": true,
  "message": "string",
  "status": true,
  "code": 1
}
```

#### GET `/alat-card-v2/api/Cards/GenerateExcel`
- Operation: */api/Cards/GenerateExcel - GET*
- Query params: `dateProcessed:string`
- Response 200 `CardTrackingModelListResponseModel`:
```json
{
  "data": [
    {
      "customer": "string",
      "trackingId": "string",
      "cardReference": "string",
      "address": "string",
      "status": "string",
      "phone": "string",
      "emailAddress": "string",
      "vendorName": "string",
      "deliveryAgent": 1,
      "processedDate": "string"
    }
  ],
  "message": "string",
  "status": true,
  "code": 1
}
```

#### GET `/alat-card-v2/api/Cards/GetAddressInfo`
- Operation: */api/Cards/GetAddressInfo - GET*
- Response 200 `CountryModel`:
```json
{
  "countryList": [
    {
      "id": 0,
      "countryCode": "string",
      "countryName": "string"
    }
  ],
  "stateList": [
    {
      "id": 0,
      "name": "string",
      "finacleCode": "string",
      "stateId": "string",
      "country": "string"
    }
  ],
  "lgaList": [
    {
      "lgaId": 0,
      "stateId": 0,
      "name": "string"
    }
  ],
  "lcdaList": [
    {
      "lcdaId": 0,
      "lgaId": 0,
      "name": "string"
    }
  ],
  "cityList": [
    {
      "id": 0,
      "stateId": 0,
      "name": "string",
      "isJumiaCity": true
    }
  ],
  "branches": [
    {
      "branchId": "string",
      "branchName": "string",
      "address1": "string",
      "address2": "string",
      "stateId": "string"
    }
  ]
}
```

#### GET `/alat-card-v2/api/Cards/GetAllCardRequestStatusByDate`
- Operation: */api/Cards/GetAllCardRequestStatusByDate - GET*
- Query params: `datePrinted:string`
- Response 200 `CardTrackingModelListResponseModel`:
```json
{
  "data": [
    {
      "customer": "string",
      "trackingId": "string",
      "cardReference": "string",
      "address": "string",
      "status": "string",
      "phone": "string",
      "emailAddress": "string",
      "vendorName": "string",
      "deliveryAgent": 1,
      "processedDate": "string"
    }
  ],
  "message": "string",
  "status": true,
  "code": 1
}
```

#### GET `/alat-card-v2/api/Cards/GetAllCourierCompanies`
- Operation: */api/Cards/GetAllCourierCompanies - GET*

#### GET `/alat-card-v2/api/Cards/GetAllSLAStatusBreached`
- Operation: */api/Cards/GetAllSLAStatusBreached - GET*
- Query params: `startDate:string`, `endDate:string`
- Response 200 `SLABreachedRespListResponseModel`:
```json
{
  "data": [
    {
      "cardRequestId": "string",
      "cardType": "string",
      "deliveryStatus": "string",
      "isPassedSLA": true,
      "emailAddress": "string",
      "phoneNumber": "string",
      "cif": "string",
      "nameOnCard": "string",
      "accountNumber": "string",
      "deliveryAddress": "string",
      "courierForDelivery": "string",
      "estimatedDeliveryDate": "string",
      "requestDate": "string",
      "trackingId": "string",
      "cardRequestReference": "string",
      "requestAge": "string",
      "slaStatus": "string",
      "customerComplaint": 0
    }
  ],
  "message": "string",
  "status": true,
  "code": 1
}
```

#### GET `/alat-card-v2/api/Cards/GetAndUpdateJumiaCities`
- Operation: */api/Cards/GetAndUpdateJumiaCities - GET*
- Response 200 `BooleanResponseModel`:
```json
{
  "data": true,
  "message": "string",
  "status": true,
  "code": 1
}
```

#### GET `/alat-card-v2/api/Cards/GetBranch`
- Operation: */api/Cards/GetBranch - GET*
- Query params: `branch:string`
- Response 200 `BranchResponse`:
```json
{
  "branchId": "string",
  "branchName": "string",
  "address1": "string",
  "address2": "string",
  "stateId": "string"
}
```

#### GET `/alat-card-v2/api/Cards/GetCardAccounts/{cif}`
- Operation: */api/Cards/GetCardAccounts/{cif} - GET*
- Path params: `cif:string`(req)
- Response 200 `CardAccountResponse`:
```json
{
  "cards": [
    {
      "schemeCode": "string",
      "name": "string",
      "price": 0,
      "cardKey": "string",
      "deliveryInsideLagos": 0,
      "deliveryOutsideLagos": 0
    }
  ],
  "accounts": [
    {
      "schemeCode": "string",
      "accountName": "string",
      "accountNumber": "string",
      "currency": 0,
      "isFirstCard": true,
      "pndStatus": "string",
      "availableBalance": 0,
      "lienedAmount": 0,
      "unclearedCheque": 0,
      "bookBalance": 0,
      "isDebitable": true,
      "schemeType": "string",
      "type": "string",
      "accountStatus": "string",
      "pndReason": "string",
      "maxLimitInterBank": 0,
      "maxLimitIntraBank": 0
    }
  ]
}
```

#### GET `/alat-card-v2/api/Cards/GetCardControlSettings`
- Operation: */api/Cards/GetCardControlSettings - GET*
- Query params: `cif:string`, `pan:string`
- Response 200 `CardControlStateResponseResponseModel`:
```json
{
  "data": {
    "cashStatusField": true,
    "chipFallBackStatusField": true,
    "foreignTravelStatusField": true,
    "masterStatusField": true,
    "posStatusField": true,
    "webStatusField": true,
    "endDateField": "string",
    "startDateField": "string",
    "allowAllForeignCountries": true
  },
  "message": "string",
  "status": true,
  "code": 1
}
```

#### GET `/alat-card-v2/api/Cards/GetCardDeliveryHistoryByTrackingId/{trackingId}`
- Operation: */api/Cards/GetCardDeliveryHistoryByTrackingId/{trackingId} - GET*
- Path params: `trackingId:string`(req)

#### GET `/alat-card-v2/api/Cards/GetCardDetailsByTrackingId/{trackingId}`
- Operation: */api/Cards/GetCardDetailsByTrackingId/{trackingId} - GET*
- Path params: `trackingId:string`(req)

#### GET `/alat-card-v2/api/Cards/GetCardProductAccount`
- Operation: */api/Cards/GetCardProductAccount - GET*
- Query params: `cif:string`
- Response 200 `GetCardDesignListResponseModel`:
```json
{
  "data": [
    {
      "cardProduct": "string",
      "accountNumber": "string",
      "nameOnCard": "string"
    }
  ],
  "message": "string",
  "status": true,
  "code": 1
}
```

#### GET `/alat-card-v2/api/Cards/GetCardReportByRequestDate`
- Operation: */api/Cards/GetCardReportByRequestDate - GET*
- Query params: `startDate:string`, `endDate:string`
- Response 200 `CardReportWrapperResponseModel`:
```json
{
  "data": {
    "totalRequest": 0,
    "totalSuccessfulRequests": 0,
    "totalActivated": 0,
    "totalSentForProcessing": 0,
    "totalDebitedSuccessful": 0,
    "totalDeliveryPassesSLA": 0,
    "totalDebitedSuccessfulButAwaitingProcessing": 0,
    "totalWithTrackingId": 0,
    "totalNotDeliveredAndNotActivated": 0,
    "totalDeliveredButNotActivated": 0,
    "totalNotSentForProcessing": 0,
    "totalWithoutTrackingID": 0,
    "totalFailedRequests": 0,
    "totalJumiaList": 0,
    "totalJumiaDeliverySuccessful": 0,
    "totalJumiaDeliveryPending": 0,
    "totalJumiaDeliveryStatusPassedSLA": 0,
    "totalUPSList": 0,
    "totalUPSDeliverySuccessful": 0,
    "totalUPSDeliveryPending": 0,
    "totalUPSDeliveryPassedSLA": 0,
    "totalSuccessfulDelivery": 0,
    "succeededRequests": [
      {
        "cardType": "string",
        "deliveryStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "cif": "string",
        "nameOnCard": "string",
        "accountNumber": "string",
        "address": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "lastEventDate": "string"
      }
    ],
    "cardDeliveryPassedSLAReq": [
      {
        "cardType": "string",
        "deliveryStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "cif": "string",
        "nameOnCard": "string",
        "accountNumber": "string",
        "address": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "lastEventDate": "string"
      }
    ],
    "successfulDebitedAndAwaitingXMLGeneration": [
      {
        "cardType": "string",
        "deliveryStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "cif": "string",
        "nameOnCard": "string",
        "accountNumber": "string",
        "address": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "lastEventDate": "string"
      }
    ],
    "cardSentForProcessing": [
      {
        "cardType": "string",
        "deliveryStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "cif": "string",
        "nameOnCard": "string",
        "accountNumber": "string",
        "address": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "lastEventDate": "string"
      }
    ],
    "trackingIdGenerated": [
      {
        "cardType": "string",
        "deliveryStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "cif": "string",
        "nameOnCard": "string",
        "accountNumber": "string",
        "address": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "lastEventDate": "string"
      }
    ],
    "jumiaDeliverySuccessful": [
      {
        "cardType": "string",
        "status": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "cif": "string",
        "nameOnCard": "string",
        "accountNumber": "string",
        "address": "string",
        "requestDate": "string",
        "lastEventDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "requestAge": "string",
        "isActivated": true
      }
    ],
    "jumiaDeliveryPending": [
      {
        "cardType": "string",
        "status": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "cif": "string",
        "nameOnCard": "string",
        "accountNumber": "string",
        "address": "string",
        "requestDate": "string",
        "lastEventDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "requestAge": "string",
        "isActivated": true
      }
    ],
    "jumiaDeliveryPassedSLA": [
      {
        "cardType": "string",
        "deliveryStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "cif": "string",
        "nameOnCard": "string",
        "accountNumber": "string",
        "address": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "lastEventDate": "string"
      }
    ],
    "upsDeliverySuccessful": [
      {
        "cardType": "string",
        "deliveryStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "cif": "string",
        "nameOnCard": "string",
        "accountNumber": "string",
        "address": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "lastEventDate": "string"
      }
    ],
    "upsDeliveryPending": [
      {
        "cardType": "string",
        "deliveryStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "cif": "string",
        "nameOnCard": "string",
        "accountNumber": "string",
        "address": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "lastEventDate": "string"
      }
    ],
    "upsDeliveryPassedSLA": [
      {
        "cardType": "string",
        "deliveryStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "cif": "string",
        "nameOnCard": "string",
        "accountNumber": "string",
        "address": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "lastEventDate": "string"
      }
    ],
    "cardDeliveredButNotActivated": [
      {
        "cardType": "string",
        "deliveryStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "cif": "string",
        "nameOnCard": "string",
        "accountNumber": "string",
        "address": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "lastEventDate": "string"
      }
    ],
    "cardDeliveredAndActivated": [
      {
        "cardType": "string",
        "deliveryStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "cif": "string",
        "nameOnCard": "string",
        "accountNumber": "string",
        "address": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "lastEventDate": "string"
      }
    ],
    "successCardDelivered": [
      {
        "cardType": "string",
        "deliveryStatus": "string",
        "isPassedSLA": true,
        "emailAddress": "string",
        "phoneNumber": "string",
        "cif": "string",
        "nameOnCard": "string",
        "accountNumber": "string",
        "address": "string",
        "requestDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "batchId": "string",
        "isActivated": true,
        "requestAge": "string",
        "lastEventDate": "string"
      }
    ],
    "failedRequests": [
      {
        "cardType": "string",
        "deliveryStatus": "string",
        "emailAddress": "string",
        "phoneNumber": "string",
        "requestAge": "string",
        "isPassedSLA": true,
        "cif": "string",
        "nameOnCard": "string",
        "accountNumber": "string",
        "address": "string",
        "requestDate": "string",
        "lastEventDate": "string",
        "trackingId": "string",
        "cardRequestReference": "string"
      }
    ],
    "cardRequestDetail": [
      {
        "nameOnCard": "string",
        "accountNumber": "string",
        "phoneNumber": "string",
        "requestDate": "string",
        "transactionDate": "string",
        "processingDate": "string",
        "trackingId": "string",
        "deliveryStatus": "string",
        "activatedDate": "string"
      }
    ]
  },
  "message": "string",
  "status": true,
  "code": 1
}
```

#### GET `/alat-card-v2/api/Cards/GetCardRequestDetail`
- Operation: */api/Cards/GetCardRequestDetail - GET*
- Query params: `accountNumber:string`
- Response 200 `CardRequestDetailReqListResponseModel`:
```json
{
  "data": [
    {
      "requestDate": "string",
      "transactionDate": "string",
      "processingDate": "string",
      "trackingId": "string",
      "statuss": "string",
      "status": [
        {
          "event": "string",
          "eventDate": "string",
          "id": 0,
          "isFinal": true,
          "size": "string",
          "dimensions": {
            "weight": {
              "value": 0,
              "units": "string"
            },
            "width": {
              "value": 0,
              "units": "string"
            },
            "height": {
              "value": 0,
              "units": "string"
            },
            "length": {
              "value": 0,
              "units": "string"
            }
          },
          "reason": {
            "name": "string",
            "code": "string"
          }
        }
      ],
      "activatedDate": "string"
    }
  ],
  "message": "string",
  "status": true,
  "code": 1
}
```

#### GET `/alat-card-v2/api/Cards/GetCardRequestPassedSLA`
- Operation: */api/Cards/GetCardRequestPassedSLA - GET*
- Query params: `startDate:string`, `endDate:string`, `skip:integer`, `take:integer`
- Response 200 `DeliveryPassedSLARespListResponseModel`:
```json
{
  "data": [
    {
      "cif": "string",
      "fullname": "string",
      "paymentAccountNumber": "string",
      "trackingId": "string",
      "cardRequestReference": "string"
    }
  ],
  "message": "string",
  "status": true,
  "code": 1
}
```

#### GET `/alat-card-v2/api/Cards/GetCardRequestStatus`
- Operation: */api/Cards/GetCardRequestStatus - GET*
- Query params: `account:string`
- Response 200 `CardTrackingModelResponseModel`:
```json
{
  "data": {
    "customer": "string",
    "trackingId": "string",
    "cardReference": "string",
    "address": "string",
    "status": "string",
    "phone": "string",
    "emailAddress": "string",
    "vendorName": "string",
    "deliveryAgent": 1,
    "processedDate": "string"
  },
  "message": "string",
  "status": true,
  "code": 1
}
```

#### GET `/alat-card-v2/api/Cards/GetClientDetails/{cif}`
- Operation: */api/Cards/GetClientDetails/{cif} - GET*
- Path params: `cif:string`(req)
- Query params: `addParam:string`
- Response 200 `ClientDetailsResp`:
```json
{
  "clientId": "string",
  "name": "string",
  "title": "string",
  "sex": "string",
  "birthDay": "string",
  "document": "string",
  "vip": "string",
  "addressCont": "string",
  "phone": "string",
  "mobilePhone": "string",
  "externalId": "string"
}
```

#### GET `/alat-card-v2/api/Cards/GetCreditCardBalance/{accountNumber}`
- Operation: */api/Cards/GetCreditCardBalance/{accountNumber} - GET*
- Path params: `accountNumber:string`(req)

#### GET `/alat-card-v2/api/Cards/GetCreditCardBalanceV2/{accountNumber}`
- Operation: */api/Cards/GetCreditCardBalanceV2/{accountNumber} - GET*
- Path params: `accountNumber:string`(req)

#### GET `/alat-card-v2/api/Cards/GetCreditCardDetails`
- Operation: */api/Cards/GetCreditCardDetails - GET*
- Query params: `cif:string`
- Response 200 `CardDetailResponseListCreditCardsResponse`:
```json
{
  "result": [
    {
      "id": "string",
      "cardProduct": "string",
      "pan": "string",
      "expiryDate": "string",
      "linkedAccounts": [
        "string"
      ],
      "cardStatus": "string",
      "nameOnCard": "string",
      "currency": "string",
      "fee": "string",
      "cardRouteId": 0,
      "cardType": 0,
      "limit": 0,
      "mbr": 0,
      "maskedPan": "string",
      "name": "string",
      "cardControlStateResponse": {
        "cashStatusField": true,
        "chipFallBackStatusField": true,
        "foreignTravelStatusField": true,
        "masterStatusField": true,
        "posStatusField": true,
        "webStatusField": true,
        "endDateField": "string",
        "foreignTravelCountriesField": "string",
        "startDateField": "string",
        "allowAllForeignCountries": true
      }
    }
  ],
  "cardStatus": "string",
  "creditCardKey": "string",
  "price": "string",
  "deliveryWithinLAG": "string",
  "deliveryOutsideLAG": "string",
  "insurance": 0,
  "cardRouteId": 0,
  "creditCardSchemeCode": "string",
  "termsAndConditions": "string",
  "maxLimit": 0,
  "minLimit": 0,
  "creditCardStatus": 0,
  "isSuccessful": true,
  "message": "string"
}
```

#### GET `/alat-card-v2/api/Cards/GetCurrentDeliveryStatusByTrackingId/{trackingId}`
- Operation: */api/Cards/GetCurrentDeliveryStatusByTrackingId/{trackingId} - GET*
- Path params: `trackingId:string`(req)

#### GET `/alat-card-v2/api/Cards/GetCurrentRequestStatusByAccount`
- Operation: */api/Cards/GetCurrentRequestStatusByAccount - GET*
- Query params: `accountNumber:string`
- Response 200 `CardStatusModelListResponseModel`:
```json
{
  "data": [
    {
      "cardType": "string",
      "emailAddress": "string",
      "phoneNumber": "string",
      "cif": "string",
      "nameOnCard": "string",
      "accountNumber": "string",
      "deliveryOption": "string",
      "address": "string",
      "requestDate": "string",
      "trackingId": "string",
      "cardRequestReference": "string",
      "dateSentForProcessing": "string",
      "batchNo": "string",
      "cardStatusSummary": "string",
      "status": [
        {
          "event": "string",
          "eventDate": "string",
          "id": 0,
          "isFinal": true,
          "size": "string",
          "dimensions": {
            "weight": {
              "value": 0,
              "units": "string"
            },
            "width": {
              "value": 0,
              "units": "string"
            },
            "height": {
              "value": 0,
              "units": "string"
            },
            "length": {
              "value": 0,
              "units": "string"
            }
          },
          "reason": {
            "name": "string",
            "code": "string"
          }
        }
      ],
      "isPassedSLA": true
    }
  ],
  "message": "string",
  "status": true,
  "code": 1
}
```

#### GET `/alat-card-v2/api/Cards/GetCustomerAddressByCardReference`
- Operation: */api/Cards/GetCustomerAddressByCardReference - GET*
- Query params: `cardRef:string`
- Response 200 `CustomerAddressInfoResponseModel`:
```json
{
  "data": {
    "accountNo": "string",
    "cardReference": "string",
    "city": "string",
    "state": "string"
  },
  "message": "string",
  "status": true,
  "code": 1
}
```

#### POST `/alat-card-v2/api/Cards/GetCustomerCards`
- Operation: */api/Cards/GetCustomerCards - POST*
- Request body `ApiCardsGetCustomerCardsPostRequest`:
```json
[
  {
    "accountNumber": "string",
    "schemeCode": "string"
  }
]
```
- Response 200 `CardDetailResponseListResponseModel`:
```json
{
  "data": [
    {
      "id": "string",
      "cardProduct": "string",
      "pan": "string",
      "expiryDate": "string",
      "linkedAccounts": [
        "string"
      ],
      "cardStatus": "string",
      "nameOnCard": "string",
      "currency": "string",
      "fee": "string",
      "cardRouteId": 0,
      "cardType": 0,
      "limit": 0,
      "mbr": 0,
      "maskedPan": "string",
      "name": "string",
      "cardControlStateResponse": {
        "cashStatusField": true,
        "chipFallBackStatusField": true,
        "foreignTravelStatusField": true,
        "masterStatusField": true,
        "posStatusField": true,
        "webStatusField": true,
        "endDateField": "string",
        "foreignTravelCountriesField": "string",
        "startDateField": "string",
        "allowAllForeignCountries": true
      }
    }
  ],
  "message": "string",
  "status": true,
  "code": 1
}
```

#### POST `/alat-card-v2/api/Cards/GetCustomerCardsV2`
- Operation: */api/Cards/GetCustomerCardsV2 - POST*
- Request body `ApiCardsGetCustomerCardsV2PostRequest`:
```json
[
  {
    "accountNumber": "string",
    "schemeCode": "string"
  }
]
```
- Response 200 `PostillionGetCardDetailsResponse`:
```json
{
  "data": [
    {
      "name": "string",
      "accountNo": "string",
      "bin": "string",
      "last4": "string",
      "expiryDate": "string",
      "message": "string"
    }
  ],
  "successful": true,
  "message": "string"
}
```

#### POST `/alat-card-v2/api/Cards/GetCustomerCardsV3`
- Operation: */api/Cards/GetCustomerCardsV3 - POST*
- Request body `CardDetailRequest`:
```json
{
  "cif": "string"
}
```
- Response 200 `CardDetailResponseListResponseModel`:
```json
{
  "data": [
    {
      "id": "string",
      "cardProduct": "string",
      "pan": "string",
      "expiryDate": "string",
      "linkedAccounts": [
        "string"
      ],
      "cardStatus": "string",
      "nameOnCard": "string",
      "currency": "string",
      "fee": "string",
      "cardRouteId": 0,
      "cardType": 0,
      "limit": 0,
      "mbr": 0,
      "maskedPan": "string",
      "name": "string",
      "cardControlStateResponse": {
        "cashStatusField": true,
        "chipFallBackStatusField": true,
        "foreignTravelStatusField": true,
        "masterStatusField": true,
        "posStatusField": true,
        "webStatusField": true,
        "endDateField": "string",
        "foreignTravelCountriesField": "string",
        "startDateField": "string",
        "allowAllForeignCountries": true
      }
    }
  ],
  "message": "string",
  "status": true,
  "code": 1
}
```

#### GET `/alat-card-v2/api/Cards/GetEMPAccount/{accountNumber}`
- Operation: */api/Cards/GetEMPAccount/{accountNumber} - GET*
- Path params: `accountNumber:string`(req)

#### GET `/alat-card-v2/api/Cards/GetFinaclePhone`
- Operation: */api/Cards/GetFinaclePhone - GET*
- Query params: `cif:string`
- Response 200 `BooleanResponseModel`:
```json
{
  "data": true,
  "message": "string",
  "status": true,
  "code": 1
}
```

#### GET `/alat-card-v2/api/Cards/GetGeneratedTrackingIdReport`
- Operation: */api/Cards/GetGeneratedTrackingIdReport - GET*
- Query params: `startDate:string`, `endDate:string`
- Response 200 `GeneratedtrackingIdReportsResponseModel`:
```json
{
  "data": {
    "totalTrackingIdGenerated": 0,
    "totalJumiaTrackingIdGenerated": 0,
    "totalUpsTrackingIdGenerated": 0,
    "jumiaTrackingIdGenerated": [
      {
        "cif": "string",
        "fullname": "string",
        "accountNumber": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "agentDeliveryStatus": "string",
        "batchId": "string",
        "requestDate": "string"
      }
    ],
    "upsTrackingIdGenerated": [
      {
        "cif": "string",
        "fullname": "string",
        "accountNumber": "string",
        "trackingId": "string",
        "cardRequestReference": "string",
        "agentDeliveryStatus": "string",
        "batchId": "string",
        "requestDate": "string"
      }
    ]
  },
  "message": "string",
  "status": true,
  "code": 1
}
```

#### GET `/alat-card-v2/api/Cards/GetJumiaCardDeliveryHistoryByDate`
- Operation: */api/Cards/GetJumiaCardDeliveryHistoryByDate - GET*
- Query params: `startDate:string`, `endDate:string`

#### GET `/alat-card-v2/api/Cards/GetJumiaCities`
- Operation: */api/Cards/GetJumiaCities - GET*
- Response 200 `JumiaCityListResponseModel`:
```json
{
  "data": [
    {
      "cityId": "string",
      "name": "string",
      "regionId": "string",
      "regionName": "string",
      "id": "string"
    }
  ],
  "message": "string",
  "status": true,
  "code": 1
}
```

#### GET `/alat-card-v2/api/Cards/GetJumiaCurrentDeliveryStatusByDate`
- Operation: */api/Cards/GetJumiaCurrentDeliveryStatusByDate - GET*
- Query params: `startDate:string`, `endDate:string`

#### GET `/alat-card-v2/api/Cards/GetJumiaDeliveryPassedSLA`
- Operation: */api/Cards/GetJumiaDeliveryPassedSLA - GET*
- Query params: `startDate:string`, `endDate:string`, `skip:integer`, `take:integer`
- Response 200 `DeliveryPassedSLARespListResponseModel`:
```json
{
  "data": [
    {
      "cif": "string",
      "fullname": "string",
      "paymentAccountNumber": "string",
      "trackingId": "string",
      "cardRequestReference": "string"
    }
  ],
  "message": "string",
  "status": true,
  "code": 1
}
```

#### GET `/alat-card-v2/api/Cards/GetLinkedCards`
- Operation: */api/Cards/GetLinkedCards - GET*
- Query params: `cif:string`
- Response 200 `CardItemDtoListResponseModel`:
```json
{
  "data": [
    {
      "cardType": "string",
      "pan": "string",
      "code": "string",
      "maskedPan": "string"
    }
  ],
  "message": "string",
  "status": true,
  "code": 1
}
```

#### GET `/alat-card-v2/api/Cards/GetPendingUPSDeliveryByDate`
- Operation: */api/Cards/GetPendingUPSDeliveryByDate - GET*
- Query params: `startDate:string`, `endDate:string`, `skip:integer`, `take:integer`

#### GET `/alat-card-v2/api/Cards/GetReasons`
- Operation: */api/Cards/GetReasons - GET*
- Response 200 `ReasonListResponseModel`:
```json
{
  "data": [
    {
      "id": 0,
      "code": "string",
      "description": "string"
    }
  ],
  "message": "string",
  "status": true,
  "code": 1
}
```

#### GET `/alat-card-v2/api/Cards/GetSuccessfulUPSDeliveryByDate`
- Operation: */api/Cards/GetSuccessfulUPSDeliveryByDate - GET*
- Query params: `startDate:string`, `endDate:string`, `skip:integer`, `take:integer`

#### GET `/alat-card-v2/api/Cards/GetTrackingCardDetailsV2`
- Operation: */api/Cards/GetTrackingCardDetailsV2 - GET*
- Query params: `cif:string`
- Response 200 `CardTrackingDetailListResponseModel`:
```json
{
  "data": [
    {
      "accountNumber": "string",
      "nameOnCard": "string",
      "cardKey": "string",
      "status": 1,
      "isActivated": true,
      "dueDate": "string",
      "isOverDue": true,
      "cardRequestId": "string",
      "complaintStatus": 0,
      "timestampHistory": {
        "orderProcessingDate": "string",
        "cardProducedDate": "string",
        "readyforDeliveryDate": "string",
        "enrouteDeliveryDate": "string",
        "collectedDate": "string"
      }
    }
  ],
  "message": "string",
  "status": true,
  "code": 1
}
```

#### GET `/alat-card-v2/api/Cards/GetUnprocessedCards`
- Operation: */api/Cards/GetUnprocessedCards - GET*
- Query params: `startDate:string`, `endDate:string`
- Response 200 `CardFilterModelListResponseModel`:
```json
{
  "data": [
    {
      "cardKey": "string",
      "emailAddress": "string",
      "phoneNumber": "string",
      "cif": "string",
      "fullname": "string",
      "nameOnCard": "string",
      "paymentAccountNumber": "string",
      "deliveryOption": "string",
      "state": "string",
      "city": "string",
      "lga": "string",
      "lcda": "string",
      "apartment": "string",
      "receivingDate": "string",
      "streetAddress": "string",
      "nearestBustop": "string",
      "branchId": 0,
      "requestDate": "string",
      "trackingId": "string",
      "studentRequestId": 0,
      "datePrinted": "string",
      "preferredDate": "string",
      "cardRequestReference": "string"
    }
  ],
  "message": "string",
  "status": true,
  "code": 1
}
```

#### GET `/alat-card-v2/api/Cards/GetUnprocessedVCar3DSecure`
- Operation: */api/Cards/GetUnprocessedVCar3DSecure - GET*

#### GET `/alat-card-v2/api/Cards/GetUPSCardDeliveryStatusByDate`
- Operation: */api/Cards/GetUPSCardDeliveryStatusByDate - GET*
- Query params: `startDate:string`, `endDate:string`, `skip:integer`, `take:integer`

#### GET `/alat-card-v2/api/Cards/GetUPSDeliveryPassedSLA`
- Operation: */api/Cards/GetUPSDeliveryPassedSLA - GET*
- Query params: `startDate:string`, `endDate:string`, `skip:integer`, `take:integer`
- Response 200 `DeliveryPassedSLARespListResponseModel`:
```json
{
  "data": [
    {
      "cif": "string",
      "fullname": "string",
      "paymentAccountNumber": "string",
      "trackingId": "string",
      "cardRequestReference": "string"
    }
  ],
  "message": "string",
  "status": true,
  "code": 1
}
```

#### GET `/alat-card-v2/api/Cards/GetUtilityCards1`
- Operation: */api/Cards/GetUtilityCards1 - GET*
- Query params: `cif:string`
- Response 200 `UtilityCardListUtilityCardsResponse`:
```json
{
  "result": [
    {
      "id": "string",
      "cardProduct": "string",
      "maskedPan": "string",
      "expiryDate": "string",
      "linkedAccounts": [
        "string"
      ],
      "cardStatus": "string",
      "name": "string",
      "currency": "string",
      "fee": "string",
      "balance": 0,
      "cardRouteId": 0
    }
  ],
  "cardStatus": "string",
  "cardKey": "string",
  "price": 0,
  "deliveryWithinLAG": 0,
  "deliveryOutsideLAG": 0,
  "cardRouteId": 0,
  "schemeCode": "string",
  "isSuccessful": true,
  "message": "string"
}
```

#### GET `/alat-card-v2/api/Cards/GetVirtualDollarCardV2Balance`
- Operation: */api/Cards/GetVirtualDollarCardV2Balance - GET*
- Query params: `cif:string`
- Response 200 `VirtualDollarCardBalanceDTO`:
```json
{
  "balance": "string"
}
```

#### GET `/alat-card-v2/api/Cards/GetVirtualDollarCardV2Details`
- Operation: */api/Cards/GetVirtualDollarCardV2Details - GET*
- Query params: `cif:string`, `pin:string`
- Response 200 `FimiAccountInfoResponseResponseModel`:
```json
{
  "data": {
    "balance": 0,
    "name": "string",
    "pan": "string",
    "address": "string",
    "city": "string",
    "state": "string",
    "zip": "string",
    "cvv": "string",
    "expiry": "string",
    "linkedAccount": "string",
    "status": 0
  },
  "message": "string",
  "status": true,
  "code": 1
}
```

#### POST `/alat-card-v2/api/Cards/GSDCardsReadyForDelivery`
- Operation: */api/Cards/GSDCardsReadyForDelivery - POST*
- Request body `ApiCardsGSDCardsReadyForDeliveryPostRequest`:
```json
[
  {
    "cardRequestReference": "string"
  }
]
```
- Response 200 `GSDExcelUploadRespListResponseModel`:
```json
{
  "data": [
    {
      "isSuccessful": true,
      "successfulCount": 0
    }
  ],
  "message": "string",
  "status": true,
  "code": 1
}
```

#### POST `/alat-card-v2/api/Cards/GSDExcelUploadCardInformation`
- Operation: */api/Cards/GSDExcelUploadCardInformation - POST*
- Query params: `routeId:integer`
- Response 200 `GSDExcelUploadResp`:
```json
{
  "isSuccessful": true,
  "successfulCount": 0
}
```

#### POST `/alat-card-v2/api/Cards/GSDUpDateDeliverStatus`
- Operation: */api/Cards/GSDUpDateDeliverStatus - POST*
- Request body `GSDUpdateCustomerDetail`:
```json
{
  "cardRequestReference": "string",
  "deliveryAddress": "string",
  "estimatedDeliveryDate": "string",
  "deliveryStatus": "string",
  "staffName": "string"
}
```
- Response 200 `StringResponseModel`:
```json
{
  "data": "string",
  "message": "string",
  "status": true,
  "code": 1
}
```

#### POST `/alat-card-v2/api/Cards/HotlistVirtualCard`
- Operation: */api/Cards/HotlistVirtualCard - POST*
- Request body `HotListVCardRequestDto`:
```json
{
  "cif": "string",
  "reasonCode": "string"
}
```
- Response 200 `ResponseModel`:
```json
{
  "message": "string",
  "status": true,
  "code": 1
}
```

#### POST `/alat-card-v2/api/Cards/HotlistVirtualDollarCardV2`
- Operation: */api/Cards/HotlistVirtualDollarCardV2 - POST*
- Request body `HotListVCardRequestDto`:
```json
{
  "cif": "string",
  "reasonCode": "string"
}
```
- Response 200 `ResponseModel`:
```json
{
  "message": "string",
  "status": true,
  "code": 1
}
```

#### POST `/alat-card-v2/api/Cards/MakeCardRequest`
- Operation: */api/Cards/MakeCardRequest - POST*
- Request body `CardRequestModel`:
```json
{
  "customerId": "string",
  "channelId": "string",
  "pin": "string",
  "schemeCode": "string",
  "cif": "string",
  "accountNumber": "string",
  "preferredName": "string",
  "emailaddress": "string",
  "phoneNumber": "string",
  "deliveryOption": "string",
  "branchId": "string",
  "state": "string",
  "compoundName": "string",
  "lga": "string",
  "lcda": "string",
  "apartment": "string",
  "city": "string",
  "customerCode": "string",
  "streetAddress": "string",
  "nearestBustop": "string",
  "cardKey": "string",
  "cardProduct": "string",
  "deliverToAlternative": true,
  "alternativeName": "string",
  "alternativePhone": "string",
  "isSameAsContact": true,
  "amount": 0,
  "preferredDate": "string",
  "creditLimit": 0
}
```
- Response 200 `StringResponseModel`:
```json
{
  "data": "string",
  "message": "string",
  "status": true,
  "code": 1
}
```

#### POST `/alat-card-v2/api/Cards/PhygitalHotlistCard`
- Operation: */api/Cards/PhygitalHotlistCard - POST*
- Request body `CardHotListRequestDtoV2`:
```json
{
  "pin": "string",
  "accountNumber": "string",
  "maskedPan": "string",
  "reasonCode": "string",
  "comment": "string",
  "emailAddress": "string",
  "cardRouteId": 0
}
```
- Response 200 `StringResponseModel`:
```json
{
  "data": "string",
  "message": "string",
  "status": true,
  "code": 1
}
```

#### POST `/alat-card-v2/api/Cards/PostillionRetrievePin`
- Operation: */api/Cards/PostillionRetrievePin - POST*
- Request body `RetrievePinDto`:
```json
{
  "clientID": "string",
  "apiKey": "string",
  "accountNumber": "string",
  "expiryDate": "string",
  "emailAddress": "string",
  "fullPan": "string",
  "cardRouteId": 0
}
```
- Response 200 `RetrievePinResponseModel`:
```json
{
  "successful": true,
  "data": "string",
  "message": "string"
}
```

#### GET `/alat-card-v2/api/Cards/ProvisionCustomerForCardControl`
- Operation: */api/Cards/ProvisionCustomerForCardControl - GET*
- Query params: `cif:string`
- Response 200 `StringResponseModel`:
```json
{
  "data": "string",
  "message": "string",
  "status": true,
  "code": 1
}
```

#### POST `/alat-card-v2/api/Cards/SaveCardControlSetting`
- Operation: */api/Cards/SaveCardControlSetting - POST*
- Request body `CardControlStateModel`:
```json
{
  "cif": "string",
  "pan": "string",
  "finnaclePhone": "string",
  "cardSettingRequest": {
    "cashStatusField": true,
    "chipFallBackStatusField": true,
    "foreignTravelStatusField": true,
    "masterStatusField": true,
    "posStatusField": true,
    "webStatusField": true,
    "endDateField": "string",
    "startDateField": "string"
  }
}
```
- Response 200 `CardControlStateResponseResponseModel`:
```json
{
  "data": {
    "cashStatusField": true,
    "chipFallBackStatusField": true,
    "foreignTravelStatusField": true,
    "masterStatusField": true,
    "posStatusField": true,
    "webStatusField": true,
    "endDateField": "string",
    "startDateField": "string",
    "allowAllForeignCountries": true
  },
  "message": "string",
  "status": true,
  "code": 1
}
```

#### POST `/alat-card-v2/api/Cards/SaveReasons`
- Operation: */api/Cards/SaveReasons - POST*
- Request body `ApiCardsSaveReasonsPostRequest`:
```json
[
  {
    "code": "string",
    "description": "string"
  }
]
```
- Response 200 `BooleanResponseModel`:
```json
{
  "data": true,
  "message": "string",
  "status": true,
  "code": 1
}
```

#### POST `/alat-card-v2/api/Cards/SendCardHotList`
- Operation: */api/Cards/SendCardHotList - POST*
- Request body `CardHotListRequestDto`:
```json
{
  "customerId": "string",
  "pin": "string",
  "accountNumber": "string",
  "maskedPan": "string",
  "reasonCode": "string",
  "comment": "string",
  "emailAddress": "string",
  "cardRouteId": 0
}
```
- Response 200 `StringResponseModel`:
```json
{
  "data": "string",
  "message": "string",
  "status": true,
  "code": 1
}
```

#### GET `/alat-card-v2/api/Cards/VCardGetAccountInformation`
- Operation: */api/Cards/VCardGetAccountInformation - GET*
- Query params: `cif:string`
- Response 200 `FimiAccountInfoResponseListResponseModelVCard`:
```json
{
  "accountInfo": [
    {
      "balance": 0,
      "name": "string",
      "pan": "string",
      "address": "string",
      "city": "string",
      "state": "string",
      "zip": "string",
      "cvv": "string",
      "expiry": "string",
      "linkedAccount": "string",
      "status": 0
    }
  ],
  "vCardCharge": 0,
  "maxAmount": 0,
  "message": "string",
  "status": true,
  "code": 1
}
```


### `alat-faith-apis`  ·  ALAT Faith APIs
- Base path: `/alatfaith`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### GET `/alatfaith/api/v{version}/Customer/AccountValidationConsent`
- Operation: *Get customer account number validation consent*
- Path params: `version:string`(req)
- Query params: `AccountNumber:string`
- Required headers: `ChannelId`
- Response 200 `AccountValidationConsentResponseDTOServiceResponse`:
```json
{
  "message": "string",
  "responseCode": 10,
  "response": {
    "reference": "string",
    "duration": 0,
    "expiryTime": "string"
  }
}
```

#### GET `/alatfaith/api/v{version}/Customer/AccountValidationConsentStatus`
- Operation: *Get customer account number validation consent status*
- Path params: `version:string`(req)
- Query params: `AccountNumber:string`, `Reference:string`
- Required headers: `ChannelId`
- Response 200 `AccountValidationConsentFinalResponseDTOServiceResponse`:
```json
{
  "message": "string",
  "responseCode": 10,
  "response": {
    "reference": "string",
    "accountNumber": "string",
    "accountName": "string",
    "message": "string",
    "isApproved": true,
    "couldGetAccountDetails": true,
    "expiryTime": "string",
    "approvalStatus": 1
  }
}
```

#### GET `/alatfaith/api/v{version}/Customer/CustomerDetails/{accountNumber}`
- Operation: *Get customer details by account number*
- Path params: `accountNumber:string`(req), `version:string`(req)
- Response 200 `GetCustomerDetailsResponseDTOServiceResponse`:
```json
{
  "message": "string",
  "responseCode": 10,
  "response": {
    "name": "string",
    "emailAddress": "string",
    "phoneNumber": "string"
  }
}
```

#### GET `/alatfaith/api/v{version}/Customer/TamsCustomerDetails/{accountNumber}`
- Operation: *Get customer details for tams customer by account number*
- Path params: `accountNumber:string`(req), `version:string`(req)
- Response 200 `GetCustomerDetailsResponseDTOServiceResponse`:
```json
{
  "message": "string",
  "responseCode": 10,
  "response": {
    "name": "string",
    "emailAddress": "string",
    "phoneNumber": "string"
  }
}
```

#### POST `/alatfaith/api/v{version}/Customer/AccountValidationConsentCallBack`
- Operation: *Validate customer account number consent request call back*
- Path params: `version:string`(req)
- Request body `AccountValidationConsentCallBackRequestDTO`:
```json
{
  "reference": "string",
  "customerCif": "string",
  "isApproved": true
}
```
- Response 200 `AccountValidationConsentCallBackResponseDTO`:
```json
{
  "responseMessage": "string",
  "responseCode": "string"
}
```


### `alat-pay-e-commerce-transfers`  ·  ALAT Pay E-Commerce Transfers
*API to make payments using a Wema Bank account number.*  
- Base path: `/alat-pay`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### GET `/alat-pay/api/EcommerceTransfer/CheckTransactionStatus/{channelId}/{TransactionRefernce}`
- Operation: *Check Transaction Status*
- Path params: `channelId:`(req), `TransactionRefernce:`(req)
- Response 200 `ClientTransactionResponseEnvelope`:
```json
{
  "result": {
    "status": "string",
    "message": "string",
    "platformTransactionReference": "string"
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```

#### POST `/alat-pay/api/EcommerceTransfer/transfer-fund-request`
- Operation: *Initiate Fund Transfer Request*
- Request body `EcommerceTransferRequest`:
```json
{
  "amount": 0,
  "sourceAccountNumber": "string",
  "channelId": "string",
  "narration": "string",
  "transactionReference": "string"
}
```
- Response 200 `ClientTransactionResponseEnvelope`:
```json
{
  "result": {
    "status": "string",
    "message": "string",
    "platformTransactionReference": "string"
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```

#### POST `/alat-pay/api/EcommerceTransfer/transfer-fund-validation`
- Operation: *Validate Fund Transfer Request*
- Request body `EcommerceTransferValidation`:
```json
{
  "channelId": "string",
  "transactionReference": "string",
  "platformTransactionReference": "string",
  "otp": "string"
}
```
- Response 200 `ClientTransactionResponseEnvelope`:
```json
{
  "result": {
    "status": "string",
    "message": "string",
    "platformTransactionReference": "string"
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```


### `alat-tech-test-api`  ·  ALAT Tech Test API
- Base path: `/alat-test`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### GET `/alat-test/api/Shared/GetAllBanks`
- Operation: *Get Banks*
- Response 200 `Bank's Envelope`:
```json
{
  "result": {
    "bankName": "string",
    "bankCode": "string"
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "2022-02-10T09:53:04.464Z"
}
```


### `alat-tech-test-api-v2`  ·  ALAT Tech Test API
- Base path: `/alat-test`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### GET `/alat-test/api/Shared/GetAllBanks`
- Operation: *Get Banks*
- Response 200 `Bank's Envelope`:
```json
{
  "result": {
    "bankName": "string",
    "bankCode": "string"
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "2022-02-10T09:53:04.464Z"
}
```


### `alatwallet-apigateway`  ·  ALAT Wallet API Gateway
*API for the Closed Wallet API system.*  
- Base path: `/alat-wallet`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### POST `/alat-wallet/api/v2/Transactions/FundAccountFromWalletBySubagent`
- Operation: *FundAccountFromWalletBySubagent*
- Request body `FundAccountFromWalletViewModelV2`:
```json
{
  "walletNumber": "string",
  "pin": "string",
  "amount": 0,
  "description": "string",
  "paymentRef": "string",
  "narration": "string",
  "branchCode": "string"
}
```
- Response 200 `ApiResponse[WalletTransactionViewModel]`:
```json
{
  "errors": [
    "string"
  ],
  "payload": {
    "phoneNumber": "string",
    "walletNumber": "string",
    "balance": 0,
    "email": "string",
    "name": "string",
    "accountNumber": "string",
    "status": 0,
    "transaction": {
      "amount": 0,
      "reference": "string",
      "paymentRef": "string",
      "transactionMode": 0,
      "transactionStatus": 0,
      "description": "string",
      "phoneNumber": "string",
      "narration": "string",
      "transactionDate": "string"
    }
  },
  "totalCount": 0,
  "status": "string",
  "totalAmountSuccessful": 0,
  "totalAmountFailed": 0,
  "code": 1,
  "result": 0,
  "description": "string"
}
```

#### POST `/alat-wallet/api/v2/Transactions/FundAccountFromWalletBySuperAgent`
- Operation: *FundAccountFromWalletBySuperAgent*
- Request body `FundAccountFromWalletViewModelV2`:
```json
{
  "walletNumber": "string",
  "pin": "string",
  "amount": 0,
  "description": "string",
  "paymentRef": "string",
  "narration": "string",
  "branchCode": "string"
}
```
- Response 200 `ApiResponse[WalletTransactionViewModel]`:
```json
{
  "errors": [
    "string"
  ],
  "payload": {
    "phoneNumber": "string",
    "walletNumber": "string",
    "balance": 0,
    "email": "string",
    "name": "string",
    "accountNumber": "string",
    "status": 0,
    "transaction": {
      "amount": 0,
      "reference": "string",
      "paymentRef": "string",
      "transactionMode": 0,
      "transactionStatus": 0,
      "description": "string",
      "phoneNumber": "string",
      "narration": "string",
      "transactionDate": "string"
    }
  },
  "totalCount": 0,
  "status": "string",
  "totalAmountSuccessful": 0,
  "totalAmountFailed": 0,
  "code": 1,
  "result": 0,
  "description": "string"
}
```

#### POST `/alat-wallet/api/v2/Transactions/FundWalletFromAccountBySubagent`
- Operation: *FundWalletFromAccountBySubagent*
- Request body `FundAccountFromWalletViewModelV2`:
```json
{
  "walletNumber": "string",
  "pin": "string",
  "amount": 0,
  "description": "string",
  "paymentRef": "string",
  "narration": "string",
  "branchCode": "string"
}
```
- Response 200 `ApiResponse[WalletTransactionViewModel]`:
```json
{
  "errors": [
    "string"
  ],
  "payload": {
    "phoneNumber": "string",
    "walletNumber": "string",
    "balance": 0,
    "email": "string",
    "name": "string",
    "accountNumber": "string",
    "status": 0,
    "transaction": {
      "amount": 0,
      "reference": "string",
      "paymentRef": "string",
      "transactionMode": 0,
      "transactionStatus": 0,
      "description": "string",
      "phoneNumber": "string",
      "narration": "string",
      "transactionDate": "string"
    }
  },
  "totalCount": 0,
  "status": "string",
  "totalAmountSuccessful": 0,
  "totalAmountFailed": 0,
  "code": 1,
  "result": 0,
  "description": "string"
}
```

#### POST `/alat-wallet/api/v2/Transactions/FundWalletFromAccountBySuperAgent`
- Operation: *FundWalletFromAccountBySuperAgent*
- Request body `FundAccountFromWalletViewModelV2`:
```json
{
  "walletNumber": "string",
  "pin": "string",
  "amount": 0,
  "description": "string",
  "paymentRef": "string",
  "narration": "string",
  "branchCode": "string"
}
```
- Response 200 `ApiResponse[WalletTransactionViewModel]`:
```json
{
  "errors": [
    "string"
  ],
  "payload": {
    "phoneNumber": "string",
    "walletNumber": "string",
    "balance": 0,
    "email": "string",
    "name": "string",
    "accountNumber": "string",
    "status": 0,
    "transaction": {
      "amount": 0,
      "reference": "string",
      "paymentRef": "string",
      "transactionMode": 0,
      "transactionStatus": 0,
      "description": "string",
      "phoneNumber": "string",
      "narration": "string",
      "transactionDate": "string"
    }
  },
  "totalCount": 0,
  "status": "string",
  "totalAmountSuccessful": 0,
  "totalAmountFailed": 0,
  "code": 1,
  "result": 0,
  "description": "string"
}
```

#### POST `/alat-wallet/api/v2/Transactions/IntraWalletTransfer`
- Operation: *IntraWalletTransfer*
- Request body `TransferToWalletViewModelV2`:
```json
{
  "debitWalletNumber": "string",
  "creditWalletNumber": "string",
  "pin": "string",
  "amount": 0,
  "paymentRef": "string",
  "description": "string",
  "narration": "string",
  "branchCode": "string",
  "agentType": 1
}
```
- Response 200 `ApiResponse[WalletTransactionViewModel]`:
```json
{
  "errors": [
    "string"
  ],
  "payload": {
    "phoneNumber": "string",
    "walletNumber": "string",
    "balance": 0,
    "email": "string",
    "name": "string",
    "accountNumber": "string",
    "status": 0,
    "transaction": {
      "amount": 0,
      "reference": "string",
      "paymentRef": "string",
      "transactionMode": 0,
      "transactionStatus": 0,
      "description": "string",
      "phoneNumber": "string",
      "narration": "string",
      "transactionDate": "string"
    }
  },
  "totalCount": 0,
  "status": "string",
  "totalAmountSuccessful": 0,
  "totalAmountFailed": 0,
  "code": 1,
  "result": 0,
  "description": "string"
}
```

#### POST `/alat-wallet/api/v2/Transactions/IntraWalletTransferFromPrincipal`
- Operation: *IntraWalletTransferFromPrincipal*
- Request body `TransferToSubAgentWalletDtoV2`:
```json
{
  "debitWalletNumber": "string",
  "subAgentWalletNumber": "string",
  "pin": "string",
  "amount": 0,
  "paymentRef": "string",
  "description": "string",
  "narration": "string",
  "branchCode": "string"
}
```
- Response 200 `ApiResponse[WalletTransactionViewModel]`:
```json
{
  "errors": [
    "string"
  ],
  "payload": {
    "phoneNumber": "string",
    "walletNumber": "string",
    "balance": 0,
    "email": "string",
    "name": "string",
    "accountNumber": "string",
    "status": 0,
    "transaction": {
      "amount": 0,
      "reference": "string",
      "paymentRef": "string",
      "transactionMode": 0,
      "transactionStatus": 0,
      "description": "string",
      "phoneNumber": "string",
      "narration": "string",
      "transactionDate": "string"
    }
  },
  "totalCount": 0,
  "status": "string",
  "totalAmountSuccessful": 0,
  "totalAmountFailed": 0,
  "code": 1,
  "result": 0,
  "description": "string"
}
```

#### POST `/alat-wallet/api/v2/Transactions/IntraWalletTransferFromSubAgents`
- Operation: *IntraWalletTransferFromSubAgents*
- Request body `TransferToPrincipalWalletViewModelV2`:
```json
{
  "debitWalletNumber": "string",
  "pin": "string",
  "amount": 0,
  "paymentRef": "string",
  "description": "string",
  "narration": "string",
  "branchCode": "string"
}
```
- Response 200 `ApiResponse[WalletTransactionViewModel]`:
```json
{
  "errors": [
    "string"
  ],
  "payload": {
    "phoneNumber": "string",
    "walletNumber": "string",
    "balance": 0,
    "email": "string",
    "name": "string",
    "accountNumber": "string",
    "status": 0,
    "transaction": {
      "amount": 0,
      "reference": "string",
      "paymentRef": "string",
      "transactionMode": 0,
      "transactionStatus": 0,
      "description": "string",
      "phoneNumber": "string",
      "narration": "string",
      "transactionDate": "string"
    }
  },
  "totalCount": 0,
  "status": "string",
  "totalAmountSuccessful": 0,
  "totalAmountFailed": 0,
  "code": 1,
  "result": 0,
  "description": "string"
}
```

#### POST `/alat-wallet/api/Security/ChangePin`
- Operation: *Security - ChangePin*
- Request body `ChangePinViewModel`:
```json
{
  "oldPin": "string",
  "newPin": "string",
  "phoneNumber": "string"
}
```

#### POST `/alat-wallet/api/Security/ForgotPin`
- Operation: *Security - ForgotPin*
- Request body `ForgotPinViewModel`:
```json
{
  "phoneNumber": "string"
}
```

#### POST `/alat-wallet/api/Security/ResetPin`
- Operation: *Security - ResetPin*
- Request body `ResetPinViewModel`:
```json
{
  "pin": "string",
  "phoneNumber": "string",
  "code": "string"
}
```

#### POST `/alat-wallet/api/Security/ValidatePin`
- Operation: *Security - ValidatePin*
- Request body `ValidatePinViewModel`:
```json
{
  "pin": "string",
  "phoneNumber": "string"
}
```

#### POST `/alat-wallet/api/v2/Transactions/TransferToCustomerAccountByAgent`
- Operation: *TransferToCustomerAccountByAgent*
- Request body `FundCustomerAccountByAgentViewModelV2`:
```json
{
  "agentWalletNumber": "string",
  "amount": 0,
  "paymentRef": "string",
  "description": "string",
  "pin": "string",
  "controllerId": "string",
  "narration": "string",
  "branchCode": "string"
}
```
- Response 200 `ApiResponse[AgentWalletDebitSummaryViewModel]`:
```json
{
  "errors": [
    "string"
  ],
  "payload": {
    "agentWallet": {
      "phoneNumber": "string",
      "walletNumber": "string",
      "balance": 0,
      "email": "string",
      "name": "string",
      "accountNumber": "string",
      "status": 0,
      "transaction": {
        "amount": 0,
        "reference": "string",
        "paymentRef": "string",
        "transactionMode": 0,
        "transactionStatus": 0,
        "description": "string",
        "phoneNumber": "string",
        "narration": "string",
        "transactionDate": "string"
      }
    }
  },
  "totalCount": 0,
  "status": "string",
  "totalAmountSuccessful": 0,
  "totalAmountFailed": 0,
  "code": 1,
  "result": 0,
  "description": "string"
}
```

#### POST `/alat-wallet/api/WalletsV2/CreateSubAgentWallet`
- Operation: *Wallet - CreateSubAgentWallet*
- Request body `CreateWalletViewModel`:
```json
{
  "phoneNumber": "string",
  "name": "string",
  "email": "string",
  "walletType": 0,
  "accountNumber": "string",
  "principalAgentWalletNo": "string",
  "pin": "string"
}
```
- Response 200 `ApiResponse[WalletDetailsViewModel]`:
```json
{
  "errors": [
    "string"
  ],
  "payload": {
    "phoneNumber": "string",
    "walletNumber": "string",
    "balance": 0,
    "email": "string",
    "name": "string",
    "accountNumber": "string",
    "status": 0
  },
  "totalCount": 0,
  "status": "string",
  "totalAmountSuccessful": 0,
  "totalAmountFailed": 0,
  "code": 1,
  "result": 0,
  "description": "string"
}
```

#### POST `/alat-wallet/api/WalletsV2/CreateSuperAgentwallet`
- Operation: *Wallet - CreateSuperAgentwallet*
- Request body `CreateWalletViewModel`:
```json
{
  "phoneNumber": "string",
  "name": "string",
  "email": "string",
  "walletType": 0,
  "accountNumber": "string",
  "principalAgentWalletNo": "string",
  "pin": "string"
}
```
- Response 200 `ApiResponse[WalletDetailsViewModel]`:
```json
{
  "errors": [
    "string"
  ],
  "payload": {
    "phoneNumber": "string",
    "walletNumber": "string",
    "balance": 0,
    "email": "string",
    "name": "string",
    "accountNumber": "string",
    "status": 0
  },
  "totalCount": 0,
  "status": "string",
  "totalAmountSuccessful": 0,
  "totalAmountFailed": 0,
  "code": 1,
  "result": 0,
  "description": "string"
}
```

#### POST `/alat-wallet/api/Wallets/CreateWallet`
- Operation: *Wallet - CreateWallet*
- Request body `CreateWalletViewModel`:
```json
{
  "phoneNumber": "string",
  "name": "string",
  "email": "string",
  "walletType": 0,
  "accountNumber": "string",
  "principalAgentWalletNo": "string",
  "pin": "string"
}
```
- Response 200 `ApiResponse[WalletDetailsViewModel]`:
```json
{
  "errors": [
    "string"
  ],
  "payload": {
    "phoneNumber": "string",
    "walletNumber": "string",
    "balance": 0,
    "email": "string",
    "name": "string",
    "accountNumber": "string",
    "status": 0
  },
  "totalCount": 0,
  "status": "string",
  "totalAmountSuccessful": 0,
  "totalAmountFailed": 0,
  "code": 1,
  "result": 0,
  "description": "string"
}
```

#### GET `/alat-wallet/api/WalletsV2/GetBalance`
- Operation: *Wallet - GetBalance*
- Query params: `walletNumber:string`
- Response 200 `ApiResponse[Decimal]`:
```json
{
  "errors": [
    "string"
  ],
  "payload": 0,
  "totalCount": 0,
  "status": "string",
  "totalAmountSuccessful": 0,
  "totalAmountFailed": 0,
  "code": 1,
  "result": 0,
  "description": "string"
}
```

#### GET `/alat-wallet/api/WalletsV2/GetSubAgentsBySuperAgent`
- Operation: *Wallet - GetSubAgentsBySuperAgent*
- Query params: `phoneNumber:string`

#### GET `/alat-wallet/api/WalletsV2/WalletDetails`
- Operation: *Wallet - WalletDetails*
- Query params: `walletNumber:string`


### `bills-platform`  ·  Bills Payment Open API
*API for the payment of bills using accounts opened via the Onboarding API - Wallets .*  
- Base path: `/open-bills`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### GET `/open-bills/api/OpenApiBills/GetAllBills`
- Operation: *STEP 1: GetAllBillsAndPackages*
- Response 200 `CategoryViewModelListOpenApiServiceResponse`:
```json
{
  "successful": true,
  "message": "string",
  "result": [
    {
      "id": 0,
      "name": "string",
      "billers": [
        {
          "id": 0,
          "name": "string",
          "identifier": "string",
          "isAquired": true,
          "requiredValidation": true,
          "charge": 0.0,
          "flow": 0,
          "packages": [
            {
              "id": 0,
              "billerId": 0,
              "name": "string",
              "isAmountEditable": true,
              "amount": 0.0,
              "minAmount": 0.0,
              "maxAmount": 0.0
            }
          ]
        }
      ]
    }
  ]
}
```

#### POST `/open-bills/api/OpenApiBills/ValidateCustomer`
- Operation: *STEP 2: ValidateCustomer*
- Request body `ValidationRequest`:
```json
{
  "channelId": "string",
  "identifier": "string",
  "packageId": 0
}
```
- Response 200 `ValidationResponseOpenApiServiceResponse`:
```json
{
  "successful": true,
  "message": "string",
  "result": {
    "isValidated": true,
    "customerName": "string",
    "message": "string",
    "validationInfo": "string"
  }
}
```

#### POST `/open-bills/api/OpenApiBills/PayBill`
- Operation: *STEP 3: Pay Bill*
- Required headers: `hash`(req)
- Request body `OpenApiPayBillRequest`:
```json
{
  "customerAccount": "string",
  "amount": 0.0,
  "charge": 0.0,
  "transactionReference": "string",
  "packageId": 0,
  "customerIdentifier": "string",
  "customerEmail": "string",
  "customerPhoneNumber": "string",
  "customerName": "string"
}
```
- Response 200 `OpenAPITransactionResponseOpenApiServiceResponse`:
```json
{
  "successful": true,
  "message": "string",
  "result": {
    "status": {},
    "message": "string",
    "platformTransactionReference": "string"
  }
}
```


### `cardless-transaction-api`  ·  Cardless Transaction Platform API (Coralpay)
*An Api that manage requests from CardLess Transaction API.*  
- Base path: `/cardlesstrans`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### POST `/cardlesstrans/api/coralpayservice/CreateSettlement`
- Operation: */api/coralpayservice/CreateSettlement - POST*
- Request body `CoralPaySettlement`:
```json
{
  "id": 0,
  "sessionId": "string",
  "destinationInstitutionId": "string",
  "sourceAccountId": "string",
  "sourceAccountName": "string",
  "creditAccount": "string",
  "creditAccountName": "string",
  "generatedReference": "string",
  "narration": "string",
  "paymentRef": "string",
  "channel": 0,
  "deviceId": "string",
  "amount": 0,
  "feeAmount": 0,
  "merchantId": "string",
  "originateInstitutionCode": "string",
  "originateInstitutionName": "string",
  "financleResponse": "string",
  "reason": "string",
  "responseCode": "string",
  "responseDescription": "string",
  "transactionStatus": 0,
  "dateCreated": "string"
}
```

#### POST `/cardlesstrans/api/coralpayservice/NameInquriy`
- Operation: */api/coralpayservice/NameInquriy - POST*
- Query params: `request:string`

#### POST `/cardlesstrans/api/coralpayservice/Reversal`
- Operation: */api/coralpayservice/Reversal - POST*
- Query params: `request:string`


### `consortium-accounts-payment-api`  ·  Consortium Accounts Payment API
*Web API to consumate credit and debit transactions on Consortium Wallets*  
- Base path: `/consortium-payment`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### POST `/consortium-payment/api/IntraBankFxTransfer/DebitWallet`
- Operation: *Debits Fx wallet account*
- Required headers: `access`(req)
- Request body `FxClientWalletTransferRequestModel`:
```json
{
  "amount": 0,
  "sourceAccountNumber": "string",
  "narration": "string",
  "transactionReference": "string",
  "useCustomNarration": true
}
```
- Response 200 `GlobalResponseEnvelope`:
```json
{
  "result": {
    "status": "string",
    "message": "string",
    "narration": "string",
    "transactionReference": "string",
    "platformTransactionReference": "string",
    "transactionStan": "string",
    "orinalTxnTransactionDate": "string"
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```

#### POST `/consortium-payment/api/IntraBankFxTransfer/FundWallet`
- Operation: *Funds Fx wallet account*
- Required headers: `access`(req)
- Request body `FundWalletRequest`:
```json
{
  "securityInfo": "string",
  "destinationAccountNumber": "string",
  "amount": 0,
  "narration": "string",
  "transactionReference": "string",
  "useCustomNarration": true
}
```
- Response 200 `GlobalResponseEnvelope`:
```json
{
  "result": {
    "status": "string",
    "message": "string",
    "narration": "string",
    "transactionReference": "string",
    "platformTransactionReference": "string",
    "transactionStan": "string",
    "orinalTxnTransactionDate": "string"
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```

#### GET `/consortium-payment/api/Shared/AccountNameEnquiry/Wallet/{accountNumber}`
- Operation: *Name enquiry endpoint for client's wallet account*
- Path params: `accountNumber:string`(req)
- Required headers: `access`
- Response 200 `NameEnquiryResponseEnvelope`:
```json
{
  "result": {
    "bankCode": "string",
    "accountName": "string",
    "accountNumber": "string",
    "currency": "string"
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```


### `consortium-card-management`  ·  Consortium Card Management
*Web API for management of consortium cards.*  
- Base path: `/consortium-card`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### POST `/consortium-card/api/Partner/card`
- Operation: */api/Partner/card - POST*
- Request body `CardCreationRequest`:
```json
{
  "accountSchemeType": "string",
  "preferredDate": "string",
  "cardType": "string",
  "channelId": "string",
  "emailaddress": "string",
  "customerInformationId": "string",
  "phoneNumber": "string",
  "firstName": "string",
  "lastName": "string",
  "branchId": "string",
  "cif": "string",
  "accountNumber": "string",
  "accountSchemeCode": "string",
  "accountCurrency": "string",
  "flutterModel": {
    "cardType": "string",
    "bvn": "string",
    "email": "string",
    "customerReference": "string",
    "phoneNumber": "string",
    "idCardUrl": "string",
    "idCardBackUrl": "string",
    "address": {
      "buildingNumber": "string",
      "apartment": "string",
      "street": "string",
      "city": "string",
      "town": "string",
      "state": "string",
      "lga": "string",
      "lcda": "string",
      "landmark": "string",
      "additionalInformation": "string",
      "country": "string",
      "fullAddress": "string"
    }
  }
}
```
- Response 200 `StringResponseModel`:
```json
{
  "data": "string",
  "message": "string",
  "status": true
}
```

#### GET `/consortium-card/api/Partner/card/{cardReference}`
- Operation: */api/Partner/card/{cardReference} - GET*
- Path params: `cardReference:string`(req)
- Query params: `cardType:string`, `digest:string`
- Response 200 `CardRetrievalResponseResponseModel`:
```json
{
  "data": {
    "cardType": "string",
    "cardPan": "string",
    "expiryDate": "string",
    "cardHolder": "string"
  },
  "message": "string",
  "status": true
}
```

#### POST `/consortium-card/api/Partner/card/activate`
- Operation: */api/Partner/card/activate - POST*
- Request body `FWCardActivateRequest`:
```json
{
  "cardReference": "string",
  "pin": "string",
  "cardType": "string",
  "digest": "string"
}
```
- Response 200 `PinResponseModel`:
```json
{
  "successful": true,
  "message": "string"
}
```

#### POST `/consortium-card/api/Partner/card/hotlist`
- Operation: */api/Partner/card/hotlist - POST*
- Request body `FWCardHotlistRequest`:
```json
{
  "cardReference": "string",
  "cardType": "string",
  "hotlistReasonCode": "string",
  "comment": "string",
  "digest": "string"
}
```
- Response 200 `PinResponseModel`:
```json
{
  "successful": true,
  "message": "string"
}
```

#### POST `/consortium-card/api/Partner/card/pin`
- Operation: */api/Partner/card/pin - POST*
- Request body `FWPinSetRequest`:
```json
{
  "cardReference": "string",
  "newPIN": "string",
  "oldPIN": "string",
  "cardType": "string",
  "digest": "string"
}
```
- Response 200 `PinResponseModel`:
```json
{
  "successful": true,
  "message": "string"
}
```


### `customer-identification-service`  ·  Customer Identification Service
*This API is used to validate the identity of a WEMA/ALAT customer.*  
- Base path: `/customer-identification`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### GET `/customer-identification/api/v{version}/Customer/AccountValidationConsent`
- Operation: *Get customer account number validation consent*
- Path params: `version:string`(req)
- Query params: `AccountNumber:string`
- Required headers: `ChannelId`
- Response 200 `GetCustomerDetailsResponseDTOServiceResponse`:
```json
{
  "message": "string",
  "responseCode": 10,
  "response": {
    "name": "string",
    "emailAddress": "string",
    "phoneNumber": "string"
  }
}
```

#### GET `/customer-identification/api/v{version}/Customer/AccountValidationConsentStatus`
- Operation: *Get customer account number validation consent status*
- Path params: `version:string`(req)
- Query params: `AccountNumber:string`, `Reference:string`
- Required headers: `ChannelId`
- Response 200 `AccountValidationConsentFinalResponseDTOServiceResponse`:
```json
{
  "message": "string",
  "responseCode": 10,
  "response": {
    "accountNumber": "string",
    "accountName": "string",
    "message": "string",
    "isApproved": true,
    "couldGetAccountDetails": true,
    "expiryTime": "string",
    "approvalStatus": 1
  }
}
```


### `freddy-chat-apis`  ·  Freddy Chat APIs
*API for the Freddy Chat Bot on ALAT.*  
- Base path: `/freddy`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### POST `/freddy/api/Airtime/PurchaseAirtimeWithPin`
- Operation: *BuyAirtimeWithPIN*
- Required headers: `authorization`(req), `alat-token`(req)
- Request body `None`:
```json
{
  "accountNumber": "string",
  "network": "string",
  "phoneNumber": "string",
  "amount": 0,
  "pin": "string"
}
```
- Response 200 `None`:
```json
{
  "data": {
    "transactionStatus": "string",
    "message": "string",
    "narration": "string",
    "transactionReference": "string"
  },
  "message": "string",
  "messages": [
    "string"
  ],
  "hasMessage": true,
  "timeGenerated": "2021-04-30T18:39:08.678Z",
  "requestStatus": 1,
  "eventId": "3fa85f64-5717-4562-b3fc-2c963f66afa6"
}
```

#### GET `/freddy/api/AirtimeAndDataBeneficiaryandRecurringPayment/GetBeneficiaries`
- Operation: *GetBeneficiaries*
- Query params: `transactiontype:integer`(req), `pageNumber:integer`(req), `pagesize:integer`(req)
- Required headers: `authorization`(req), `alat-token`(req)
- Response 200 `None`:
```json
{
  "data": {
    "items": [
      {
        "id": "string",
        "phoneNumber": "string",
        "vendCode": "string",
        "cif": "string",
        "transactiontype": 0,
        "nickName": "string",
        "amount": 0,
        "datapackageId": 0
      }
    ],
    "currentPage": 0,
    "totalPages": 0,
    "pageSize": 0,
    "totalCount": 0,
    "hasPrevious": true,
    "hasNext": true
  },
  "message": "string",
  "messages": [
    "string"
  ],
  "hasMessage": true,
  "timeGenerated": "2021-04-30T18:30:35.366Z",
  "requestStatus": 1,
  "eventId": "3fa85f64-5717-4562-b3fc-2c963f66afa6"
}
```

#### GET `/freddy/api​/ProfileMaintenance​/GetProfileInformation`
- Operation: *GetCustomerInfo*
- Query params: `cif:string`(req)
- Required headers: `authorization`(req), `alat-token`(req)
- Response 200 `None`:
```json
{
  "data": {
    "firstName": "string",
    "lastName": "string",
    "middleName": "string",
    "phoneNumber": "string",
    "message": "string",
    "email": "string",
    "cif": "string",
    "profileId": "string",
    "isPinBlocked": true,
    "hasSetPin": true,
    "isProfileActive": true,
    "bvn": "string",
    "kycStatus": "string",
    "isExisting": true,
    "bvnAddress": "string",
    "bvnBase64Image": "string",
    "dob": "string",
    "bvnNumber": "string",
    "customerIDFormerAlat": 0,
    "isSignatureApproved": true,
    "isSelfieApproved": true,
    "isResidentialAddressApproved": true,
    "isIDCardApproved": true,
    "isAccountGenerated": true,
    "residentialAddress": "string",
    "nationality": "string",
    "accounts": [
      {
        "accountNumber": "string",
        "accountName": "string",
        "branchCode": "string",
        "currency": "string",
        "schemeCode": "string"
      }
    ]
  },
  "message": "string",
  "messages": [
    "string"
  ],
  "hasMessage": true,
  "timeGenerated": "2021-04-30T19:01:49.342Z",
  "requestStatus": 1,
  "eventId": "3fa85f64-5717-4562-b3fc-2c963f66afa6"
}
```


### `funds-transfer-api`  ·  Funds Transfer OpenAPI
*APIs for funds transfer within banks.*  
- Base path: `/funds-transfer-open`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### GET `/funds-transfer-open/api/OpenApiTransfer/GetAllBanks`
- Operation: *STEP 1: GetBankList*
- Response 200 `BanksOpenApiServiceResponse`:
```json
{
  "successful": true,
  "message": "string",
  "result": {
    "bankName": "string",
    "bankCode": "string"
  }
}
```

#### GET `/funds-transfer-open/api/Shared/AccountNameEnquiry/{bankCode}/{accountNumber}`
- Operation: *STEP 2: AccountNameEnquiry*
- Path params: `bankCode:string`(req), `accountNumber:string`(req)
- Query params: `channelId:string`
- Response 200 `AccountNameEnquiryEnvelope`:
```json
{
  "result": {
    "bankCode": "string",
    "accountName": "string",
    "accountNumber": "string",
    "currency": "string",
    "termsAndConditions": "string",
    "termsAndConditionsUrl": "string",
    "chargeFee": [
      {
        "id": 0,
        "chargeFeeName": "string",
        "transactionType": 0,
        "charge": 0.0,
        "lower": 0.0,
        "upper": 0.0
      }
    ]
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```

#### POST `/funds-transfer-open/api/OpenApiTransfer/transfer-fund-request`
- Operation: *STEP 3: TransferFunds*
- Required headers: `hash`(req)
- Request body `OpenApiTransferRequest`:
```json
{
  "amount": 0.0,
  "narration": "string",
  "transactionReference": "string",
  "destinationBankCode": "string",
  "destinationBankName": "string",
  "destinationAccountNumber": "string",
  "destinationAccountName": "string",
  "sourceAccountNumber": "string"
}
```
- Response 200 `OpenAPITransactionResponseOpenApiServiceResponse`:
```json
{
  "successful": true,
  "message": "string",
  "result": {
    "status": {},
    "message": "string",
    "platformTransactionReference": "string"
  }
}
```


### `get-statement-service`  ·  Get statement service
*This REST API is used to retrieve an ALAT customer's transaction statement over a specified period of time.*  
- Base path: `/get-statement`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### POST `/get-statement/api/AccountMaintenance/InitiateGetCustomerStatement`
- Operation: *STEP 1: InitiateGetCustomerStatement*
- Required headers: `x-api-key`(req)
- Request body `InitiateGetCustomerStatementRequest`:
```json
{
  "accountNumber": "string",
  "dateFrom": "2022-09-07T09:27:13.133Z",
  "dateTo": "2022-09-07T09:27:13.133Z"
}
```
- Response 200 `InitiateGetCustomerStatementResponse`:
```json
{
  "data": {
    "referenceId": "string"
  },
  "message": "string",
  "status": true
}
```

#### POST `/get-statement/api/AccountMaintenance/GetCustomerTransactions`
- Operation: *STEP 2: GetCustomerTransactions*
- Required headers: `x-api-key`(req)
- Request body `GetCustomerTransactionsRequest`:
```json
{
  "referenceId": "string"
}
```
- Response 200 `GetCustomerTransactionsResponse`:
```json
{
  "data": [
    {
      "title": "string",
      "amount": 0,
      "type": "InterBank",
      "date": "2022-09-07T09:45:31.199Z",
      "narration": "string",
      "status": "Default",
      "creditType": "Default",
      "sender": "string",
      "senderAccountNumber": "string",
      "destinationBank": "string",
      "destinationAccountNumber": "string",
      "recieverName": "string",
      "referenceId": "string",
      "isViewReceiptEnabled": true,
      "tranId": "string"
    }
  ],
  "message": "string",
  "status": true
}
```


### `merchant-payout-api`  ·  Merchant-Payout API
- Base path: `/merchant-payout`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### GET `/merchant-payout/api/Decrypt`
- Operation: */api/Decrypt - GET*
- Query params: `data:string`

#### GET `/merchant-payout/api/Encrypt`
- Operation: */api/Encrypt - GET*
- Query params: `data:string`

#### POST `/merchant-payout/api/Authentication/authenticate`
- Operation: *Authenticate*
- Request body `User`:
```json
{
  "id": 0,
  "firstName": "string",
  "lastName": "string",
  "username": "string",
  "password": "string",
  "token": "string",
  "refreshToken": "string",
  "refreshTokenExpires": "string"
}
```

#### POST `/merchant-payout/api/Authentication/RefreshToken`
- Operation: *Refresh Token*
- Request body `User`:
```json
{
  "id": 0,
  "firstName": "string",
  "lastName": "string",
  "username": "string",
  "password": "string",
  "token": "string",
  "refreshToken": "string",
  "refreshTokenExpires": "string"
}
```

#### GET `/merchant-payout/api/WMServices/GetBalance`
- Operation: *WMServices/GetBalance*
- Query params: `AccountNumber:string`

#### POST `/merchant-payout/api/WMServices/GetStatement`
- Operation: *WMServices/GetStatement*
- Request body `StatementRequest`:
```json
{
  "accountStatementRequest": "string"
}
```

#### GET `/merchant-payout/api/WMServices/GetTransactionStatus`
- Operation: *WMServices/GetTransactionStatus*
- Query params: `TransactionReference:string`

#### POST `/merchant-payout/api/WMServices/NIPFundTransfer`
- Operation: *WMServices/NIPFundTransfer *
- Request body `FundTransfer`:
```json
{
  "fundTransferRequest": "string"
}
```

#### POST `/merchant-payout/api/WMServices/NIPNameEnquiry`
- Operation: *WMServices/NIPNameEnquiry*
- Request body `NERequest`:
```json
{
  "nameEnquiryRequest": "string"
}
```

#### GET `/merchant-payout/api/WMServices/PayOutTSQ`
- Operation: *WMServices/PayOutTSQ*
- Query params: `SessionID:string`


### `microsite-service-api`  ·  Microsite_Service.API
- Base path: `/microsite-acct`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### POST `/microsite-acct/api/AccountGeneration/CheckCustomerExistOnAlat`
- Operation: */api/AccountGeneration/CheckCustomerExistOnAlat - POST*
- Query params: `key:string`
- Request body `ProfileRequestModel`:
```json
{
  "phoneNumber": "string",
  "email": "string",
  "requestChannel": "string"
}
```
- Response 200 `ExistingCustomerFlagResponseModel`:
```json
{
  "status": true,
  "message": "string",
  "statusCode": 100,
  "errors": [
    "string"
  ],
  "count": 0,
  "data": {
    "isExistingCustomer": true,
    "showOtpPage": true,
    "accountNumber": "string",
    "accountName": "string",
    "maskedPhoneNumber": "string"
  },
  "result": {
    "items": [
      {
        "isExistingCustomer": true,
        "showOtpPage": true,
        "accountNumber": "string",
        "accountName": "string",
        "maskedPhoneNumber": "string"
      }
    ],
    "totalItemCount": 0,
    "totalPageCount": 0,
    "currentPageSize": 0,
    "currentPageNumber": 0,
    "hasPrevious": true,
    "hasNext": true
  },
  "pager": {
    "pageNumber": 0,
    "pageCount": 0,
    "pageSize": 0,
    "hasNextPage": true,
    "hasPreviousPage": true,
    "firstItemOnPage": 0,
    "lastItemOnPage": 0,
    "totalItemCount": 0
  }
}
```

#### GET `/microsite-acct/api/AccountGeneration/DownloadRecordToExcel`
- Operation: */api/AccountGeneration/DownloadRecordToExcel - GET*
- Query params: `channel:string`, `key:string`

#### GET `/microsite-acct/api/AccountGeneration/GetAllOnboardingRecord`
- Operation: */api/AccountGeneration/GetAllOnboardingRecord - GET*
- Query params: `channel:string`
- Response 200 `StringResponseModel`:
```json
{
  "status": true,
  "message": "string",
  "statusCode": 100,
  "errors": [
    "string"
  ],
  "count": 0,
  "data": "string",
  "result": {
    "items": [
      "string"
    ],
    "totalItemCount": 0,
    "totalPageCount": 0,
    "currentPageSize": 0,
    "currentPageNumber": 0,
    "hasPrevious": true,
    "hasNext": true
  },
  "pager": {
    "pageNumber": 0,
    "pageCount": 0,
    "pageSize": 0,
    "hasNextPage": true,
    "hasPreviousPage": true,
    "firstItemOnPage": 0,
    "lastItemOnPage": 0,
    "totalItemCount": 0
  }
}
```

#### GET `/microsite-acct/api/AccountGeneration/GetCustomerConsent`
- Operation: */api/AccountGeneration/GetCustomerConsent - GET*
- Query params: `channelId:string`, `key:string`
- Response 200 `StringResponseModel`:
```json
{
  "status": true,
  "message": "string",
  "statusCode": 100,
  "errors": [
    "string"
  ],
  "count": 0,
  "data": "string",
  "result": {
    "items": [
      "string"
    ],
    "totalItemCount": 0,
    "totalPageCount": 0,
    "currentPageSize": 0,
    "currentPageNumber": 0,
    "hasPrevious": true,
    "hasNext": true
  },
  "pager": {
    "pageNumber": 0,
    "pageCount": 0,
    "pageSize": 0,
    "hasNextPage": true,
    "hasPreviousPage": true,
    "firstItemOnPage": 0,
    "lastItemOnPage": 0,
    "totalItemCount": 0
  }
}
```

#### POST `/microsite-acct/api/AccountGeneration/GetCustomerOnboardingRecord`
- Operation: */api/AccountGeneration/GetCustomerOnboardingRecord - POST*
- Query params: `key:string`
- Request body `CustomerDetailsRequest`:
```json
{
  "searchBy": 1,
  "value": "string",
  "maxItem": 0,
  "pageNumber": 0
}
```
- Response 200 `StringResponseModel`:
```json
{
  "status": true,
  "message": "string",
  "statusCode": 100,
  "errors": [
    "string"
  ],
  "count": 0,
  "data": "string",
  "result": {
    "items": [
      "string"
    ],
    "totalItemCount": 0,
    "totalPageCount": 0,
    "currentPageSize": 0,
    "currentPageNumber": 0,
    "hasPrevious": true,
    "hasNext": true
  },
  "pager": {
    "pageNumber": 0,
    "pageCount": 0,
    "pageSize": 0,
    "hasNextPage": true,
    "hasPreviousPage": true,
    "firstItemOnPage": 0,
    "lastItemOnPage": 0,
    "totalItemCount": 0
  }
}
```

#### GET `/microsite-acct/api/AccountGeneration/GetNINData`
- Operation: */api/AccountGeneration/GetNINData - GET*
- Query params: `channel:string`, `phoneNumber:string`
- Response 200 `StringResponseModel`:
```json
{
  "status": true,
  "message": "string",
  "statusCode": 100,
  "errors": [
    "string"
  ],
  "count": 0,
  "data": "string",
  "result": {
    "items": [
      "string"
    ],
    "totalItemCount": 0,
    "totalPageCount": 0,
    "currentPageSize": 0,
    "currentPageNumber": 0,
    "hasPrevious": true,
    "hasNext": true
  },
  "pager": {
    "pageNumber": 0,
    "pageCount": 0,
    "pageSize": 0,
    "hasNextPage": true,
    "hasPreviousPage": true,
    "firstItemOnPage": 0,
    "lastItemOnPage": 0,
    "totalItemCount": 0
  }
}
```

#### POST `/microsite-acct/api/AccountGeneration/GetNiNInformation`
- Operation: */api/AccountGeneration/GetNiNInformation - POST*
- Query params: `key:string`
- Request body `NinRequest`:
```json
{
  "phoneNumber": "string",
  "channelId": "string",
  "accountGenerationConsent": true,
  "nin": "string"
}
```
- Response 200 `NinResponseResponseModel`:
```json
{
  "status": true,
  "message": "string",
  "statusCode": 100,
  "errors": [
    "string"
  ],
  "count": 0,
  "data": {
    "isNinRetrieved": true,
    "isNinConnectedToExistingRecord": true,
    "showOtpPage": true,
    "maskedPhoneNumber": "string"
  },
  "result": {
    "items": [
      {
        "isNinRetrieved": true,
        "isNinConnectedToExistingRecord": true,
        "showOtpPage": true,
        "maskedPhoneNumber": "string"
      }
    ],
    "totalItemCount": 0,
    "totalPageCount": 0,
    "currentPageSize": 0,
    "currentPageNumber": 0,
    "hasPrevious": true,
    "hasNext": true
  },
  "pager": {
    "pageNumber": 0,
    "pageCount": 0,
    "pageSize": 0,
    "hasNextPage": true,
    "hasPreviousPage": true,
    "firstItemOnPage": 0,
    "lastItemOnPage": 0,
    "totalItemCount": 0
  }
}
```

#### POST `/microsite-acct/api/AccountGeneration/GetNiNInformationV2`
- Operation: */api/AccountGeneration/GetNiNInformationV2 - POST*
- Query params: `key:string`
- Request body `NinRequest`:
```json
{
  "phoneNumber": "string",
  "channelId": "string",
  "accountGenerationConsent": true,
  "nin": "string"
}
```
- Response 200 `NinResponseResponseModel`:
```json
{
  "status": true,
  "message": "string",
  "statusCode": 100,
  "errors": [
    "string"
  ],
  "count": 0,
  "data": {
    "isNinRetrieved": true,
    "isNinConnectedToExistingRecord": true,
    "showOtpPage": true,
    "maskedPhoneNumber": "string"
  },
  "result": {
    "items": [
      {
        "isNinRetrieved": true,
        "isNinConnectedToExistingRecord": true,
        "showOtpPage": true,
        "maskedPhoneNumber": "string"
      }
    ],
    "totalItemCount": 0,
    "totalPageCount": 0,
    "currentPageSize": 0,
    "currentPageNumber": 0,
    "hasPrevious": true,
    "hasNext": true
  },
  "pager": {
    "pageNumber": 0,
    "pageCount": 0,
    "pageSize": 0,
    "hasNextPage": true,
    "hasPreviousPage": true,
    "firstItemOnPage": 0,
    "lastItemOnPage": 0,
    "totalItemCount": 0
  }
}
```

#### POST `/microsite-acct/api/AccountGeneration/MicrositeCIFAndAccountGenerated`
- Operation: */api/AccountGeneration/MicrositeCIFAndAccountGenerated - POST*
- Request body `PlatformWebHookResponse`:
```json
{
  "title": "string",
  "message": "string",
  "data": {
    "customerID": "string",
    "nubanName": "string",
    "nuban": "string",
    "nubanStatus": "string",
    "type": 1,
    "email": "string",
    "phoneNumber": "string",
    "requestId": "string"
  },
  "request": 1
}
```
- Response 200 `StringResponseModel`:
```json
{
  "status": true,
  "message": "string",
  "statusCode": 100,
  "errors": [
    "string"
  ],
  "count": 0,
  "data": "string",
  "result": {
    "items": [
      "string"
    ],
    "totalItemCount": 0,
    "totalPageCount": 0,
    "currentPageSize": 0,
    "currentPageNumber": 0,
    "hasPrevious": true,
    "hasNext": true
  },
  "pager": {
    "pageNumber": 0,
    "pageCount": 0,
    "pageSize": 0,
    "hasNextPage": true,
    "hasPreviousPage": true,
    "firstItemOnPage": 0,
    "lastItemOnPage": 0,
    "totalItemCount": 0
  }
}
```

#### POST `/microsite-acct/api/AccountGeneration/MicrositeOnboardingToAlatRequest`
- Operation: */api/AccountGeneration/MicrositeOnboardingToAlatRequest - POST*
- Query params: `key:string`
- Request body `MicrositeToAlatOnboardingRequest`:
```json
{
  "emailAddress": "string",
  "phoneNumber": "string",
  "maritalStatus": 1,
  "stateOfOrigin": "string",
  "lgaOfOrigin": "string",
  "stateOfResidence": "string",
  "lgaOfResidence": "string",
  "highestLevelOfEducation": "string",
  "requestChannel": "string",
  "bvn": "string",
  "nin": "string",
  "businessName": "string",
  "businessSector": "string",
  "moreBusinessInformation": "string",
  "accountNumber": "string",
  "accountName": "string",
  "accountGenerationConsent": true,
  "dateOfBirth": "string",
  "hiredTalentId": "string",
  "hiredPartnerId": "string",
  "hiredhrId": "string",
  "registrationType": 1,
  "businessAssociationName": "string"
}
```
- Response 200 `ExistingCustomerFlagResponseModel`:
```json
{
  "status": true,
  "message": "string",
  "statusCode": 100,
  "errors": [
    "string"
  ],
  "count": 0,
  "data": {
    "isExistingCustomer": true,
    "showOtpPage": true,
    "accountNumber": "string",
    "accountName": "string",
    "maskedPhoneNumber": "string"
  },
  "result": {
    "items": [
      {
        "isExistingCustomer": true,
        "showOtpPage": true,
        "accountNumber": "string",
        "accountName": "string",
        "maskedPhoneNumber": "string"
      }
    ],
    "totalItemCount": 0,
    "totalPageCount": 0,
    "currentPageSize": 0,
    "currentPageNumber": 0,
    "hasPrevious": true,
    "hasNext": true
  },
  "pager": {
    "pageNumber": 0,
    "pageCount": 0,
    "pageSize": 0,
    "hasNextPage": true,
    "hasPreviousPage": true,
    "firstItemOnPage": 0,
    "lastItemOnPage": 0,
    "totalItemCount": 0
  }
}
```

#### POST `/microsite-acct/api/AccountGeneration/ResendOtp`
- Operation: */api/AccountGeneration/ResendOtp - POST*
- Query params: `key:string`
- Request body `ResendOtpModel`:
```json
{
  "nin": "string",
  "requestChannel": "string"
}
```
- Response 200 `StringResponseModel`:
```json
{
  "status": true,
  "message": "string",
  "statusCode": 100,
  "errors": [
    "string"
  ],
  "count": 0,
  "data": "string",
  "result": {
    "items": [
      "string"
    ],
    "totalItemCount": 0,
    "totalPageCount": 0,
    "currentPageSize": 0,
    "currentPageNumber": 0,
    "hasPrevious": true,
    "hasNext": true
  },
  "pager": {
    "pageNumber": 0,
    "pageCount": 0,
    "pageSize": 0,
    "hasNextPage": true,
    "hasPreviousPage": true,
    "firstItemOnPage": 0,
    "lastItemOnPage": 0,
    "totalItemCount": 0
  }
}
```

#### POST `/microsite-acct/api/AccountGeneration/SendOtpVerificationEmailToCustomer`
- Operation: */api/AccountGeneration/SendOtpVerificationEmailToCustomer - POST*
- Query params: `key:string`
- Request body `SendEmailRequest`:
```json
{
  "emailAddress": "string",
  "phoneNumber": "string",
  "channelId": "string"
}
```
- Response 200 `StringResponseModel`:
```json
{
  "status": true,
  "message": "string",
  "statusCode": 100,
  "errors": [
    "string"
  ],
  "count": 0,
  "data": "string",
  "result": {
    "items": [
      "string"
    ],
    "totalItemCount": 0,
    "totalPageCount": 0,
    "currentPageSize": 0,
    "currentPageNumber": 0,
    "hasPrevious": true,
    "hasNext": true
  },
  "pager": {
    "pageNumber": 0,
    "pageCount": 0,
    "pageSize": 0,
    "hasNextPage": true,
    "hasPreviousPage": true,
    "firstItemOnPage": 0,
    "lastItemOnPage": 0,
    "totalItemCount": 0
  }
}
```

#### POST `/microsite-acct/api/AccountGeneration/SubmitAccountCreationRequest`
- Operation: */api/AccountGeneration/SubmitAccountCreationRequest - POST*
- Request body `ProfileCreationQueueModel`:
```json
{
  "watchListCheckEnabled": true,
  "isALATOfficeRequest": true,
  "staffBranch": "string",
  "cif": "string",
  "currency": "string",
  "bvn": "string",
  "referralBy": "string",
  "profileUsername": "string",
  "firstName": "string",
  "lastName": "string",
  "middleName": "string",
  "gender": "string",
  "dateOfBirth": "string",
  "phonenumber": "string",
  "email": "string",
  "channel": "string",
  "accountManager": "string",
  "type": 0,
  "clientName": "string",
  "clientId": "string",
  "apFID": "string",
  "mobileOS": "string",
  "version": "string",
  "signature": "string",
  "country": "string",
  "placePND": true,
  "isExistingCustomer": true,
  "guardianFullname": "string",
  "guardianRelationshipWithKid": "string",
  "requestId": "string",
  "useV2": true,
  "isPartnershipAccount": true,
  "partnershipSchemeCode": "string",
  "partnershipGlCode": "string",
  "placeWalletPND": true,
  "partnerRequestData": "string",
  "isAlatPay": true
}
```
- Response 200 `StringResponseModel`:
```json
{
  "status": true,
  "message": "string",
  "statusCode": 100,
  "errors": [
    "string"
  ],
  "count": 0,
  "data": "string",
  "result": {
    "items": [
      "string"
    ],
    "totalItemCount": 0,
    "totalPageCount": 0,
    "currentPageSize": 0,
    "currentPageNumber": 0,
    "hasPrevious": true,
    "hasNext": true
  },
  "pager": {
    "pageNumber": 0,
    "pageCount": 0,
    "pageSize": 0,
    "hasNextPage": true,
    "hasPreviousPage": true,
    "firstItemOnPage": 0,
    "lastItemOnPage": 0,
    "totalItemCount": 0
  }
}
```

#### POST `/microsite-acct/api/AccountGeneration/SubmitAuatonOnboardingRequest`
- Operation: */api/AccountGeneration/SubmitAuatonOnboardingRequest - POST*
- Query params: `key:string`
- Request body `AuatonOnboardingRequest`:
```json
{
  "emailAddress": "string",
  "phoneNumber": "string",
  "requestChannel": "string",
  "bvn": "string",
  "nin": "string",
  "membershipNumber": "string"
}
```
- Response 200 `ExistingCustomerFlagResponseModel`:
```json
{
  "status": true,
  "message": "string",
  "statusCode": 100,
  "errors": [
    "string"
  ],
  "count": 0,
  "data": {
    "isExistingCustomer": true,
    "showOtpPage": true,
    "accountNumber": "string",
    "accountName": "string",
    "maskedPhoneNumber": "string"
  },
  "result": {
    "items": [
      {
        "isExistingCustomer": true,
        "showOtpPage": true,
        "accountNumber": "string",
        "accountName": "string",
        "maskedPhoneNumber": "string"
      }
    ],
    "totalItemCount": 0,
    "totalPageCount": 0,
    "currentPageSize": 0,
    "currentPageNumber": 0,
    "hasPrevious": true,
    "hasNext": true
  },
  "pager": {
    "pageNumber": 0,
    "pageCount": 0,
    "pageSize": 0,
    "hasNextPage": true,
    "hasPreviousPage": true,
    "firstItemOnPage": 0,
    "lastItemOnPage": 0,
    "totalItemCount": 0
  }
}
```

#### POST `/microsite-acct/api/AccountGeneration/SubmitBusinessApplicationRequest`
- Operation: */api/AccountGeneration/SubmitBusinessApplicationRequest - POST*
- Query params: `key:string`
- Request body `BusinessOnboardingRequest`:
```json
{
  "firstName": "string",
  "lastName": "string",
  "middleName": "string",
  "gender": 1,
  "dateOfBirth": "string",
  "emailAddress": "string",
  "phoneNumber": "string",
  "maritalStatus": 1,
  "stateOfOrigin": "string",
  "lgaOfOrigin": "string",
  "stateOfResidence": "string",
  "lgaOfResidence": "string",
  "technicalSkill": "string",
  "requestChannel": "string",
  "bvn": "string",
  "nin": "string",
  "needHelp": true,
  "businessName": "string",
  "isCACRegistered": true,
  "businessSector": "string",
  "moreBusinessInfo": "string",
  "smedanId": "string",
  "association": "string"
}
```
- Response 200 `StringResponseModel`:
```json
{
  "status": true,
  "message": "string",
  "statusCode": 100,
  "errors": [
    "string"
  ],
  "count": 0,
  "data": "string",
  "result": {
    "items": [
      "string"
    ],
    "totalItemCount": 0,
    "totalPageCount": 0,
    "currentPageSize": 0,
    "currentPageNumber": 0,
    "hasPrevious": true,
    "hasNext": true
  },
  "pager": {
    "pageNumber": 0,
    "pageCount": 0,
    "pageSize": 0,
    "hasNextPage": true,
    "hasPreviousPage": true,
    "firstItemOnPage": 0,
    "lastItemOnPage": 0,
    "totalItemCount": 0
  }
}
```

#### POST `/microsite-acct/api/AccountGeneration/SubmitFashionSoukApplicationRequest`
- Operation: */api/AccountGeneration/SubmitFashionSoukApplicationRequest - POST*
- Query params: `key:string`
- Request body `LeatherFairOnboardingRequest`:
```json
{
  "firstName": "string",
  "lastName": "string",
  "middleName": "string",
  "gender": 1,
  "dateOfBirth": "string",
  "emailAddress": "string",
  "phoneNumber": "string",
  "businessName": "string",
  "businessSector": "string",
  "age": 0,
  "stateOfResidence": "string",
  "cityOfResidence": "string",
  "website": "string",
  "instagramHandle": "string",
  "linkedInProfile": "string",
  "durationOfOperation": "string",
  "isGenerateRevenue": true,
  "outsourceProduction": "string",
  "productionLocation": "string",
  "businessNeed": "string",
  "moreBusinessInfo": "string",
  "needHelp": true,
  "bvn": "string",
  "nin": "string",
  "requestChannel": "string",
  "managerName": "string",
  "isFirstTimeVendor": true,
  "businessCategory": "string",
  "products": [
    "string"
  ],
  "preferredStall": "string"
}
```
- Response 200 `StringResponseModel`:
```json
{
  "status": true,
  "message": "string",
  "statusCode": 100,
  "errors": [
    "string"
  ],
  "count": 0,
  "data": "string",
  "result": {
    "items": [
      "string"
    ],
    "totalItemCount": 0,
    "totalPageCount": 0,
    "currentPageSize": 0,
    "currentPageNumber": 0,
    "hasPrevious": true,
    "hasNext": true
  },
  "pager": {
    "pageNumber": 0,
    "pageCount": 0,
    "pageSize": 0,
    "hasNextPage": true,
    "hasPreviousPage": true,
    "firstItemOnPage": 0,
    "lastItemOnPage": 0,
    "totalItemCount": 0
  }
}
```

#### POST `/microsite-acct/api/AccountGeneration/SubmitHiredHrRequest`
- Operation: */api/AccountGeneration/SubmitHiredHrRequest - POST*
- Query params: `key:string`
- Request body `HiredHrOnboardingRequestDto`:
```json
{
  "emailAddress": "string",
  "phoneNumber": "string",
  "requestChannel": "string",
  "bvn": "string",
  "nin": "string",
  "fullName": "string",
  "organizationName": "string",
  "officialEmailAddress": "string",
  "designation": "string",
  "willLoveToHire": true,
  "havePermissionToHire": true,
  "rolesToHire": "string",
  "totalNoOfTalents": 0,
  "representativeName": "string",
  "representativeDesignation": "string",
  "representativePhoneNumber": "string",
  "representativeEmail": "string",
  "accountGenerationConsent": true,
  "accountNumber": "string",
  "accountName": "string"
}
```
- Response 200 `ExistingCustomerFlagResponseModel`:
```json
{
  "status": true,
  "message": "string",
  "statusCode": 100,
  "errors": [
    "string"
  ],
  "count": 0,
  "data": {
    "isExistingCustomer": true,
    "showOtpPage": true,
    "accountNumber": "string",
    "accountName": "string",
    "maskedPhoneNumber": "string"
  },
  "result": {
    "items": [
      {
        "isExistingCustomer": true,
        "showOtpPage": true,
        "accountNumber": "string",
        "accountName": "string",
        "maskedPhoneNumber": "string"
      }
    ],
    "totalItemCount": 0,
    "totalPageCount": 0,
    "currentPageSize": 0,
    "currentPageNumber": 0,
    "hasPrevious": true,
    "hasNext": true
  },
  "pager": {
    "pageNumber": 0,
    "pageCount": 0,
    "pageSize": 0,
    "hasNextPage": true,
    "hasPreviousPage": true,
    "firstItemOnPage": 0,
    "lastItemOnPage": 0,
    "totalItemCount": 0
  }
}
```

#### POST `/microsite-acct/api/AccountGeneration/SubmitHiredPartnerRequest`
- Operation: */api/AccountGeneration/SubmitHiredPartnerRequest - POST*
- Query params: `key:string`
- Request body `HiredPartnerOnboardingRequestDto`:
```json
{
  "emailAddress": "string",
  "phoneNumber": "string",
  "requestChannel": "string",
  "bvn": "string",
  "nin": "string",
  "organizationName": "string",
  "contactPersonName": "string",
  "contactPersonRole": "string",
  "representativePhoneNumber": "string",
  "contactPersonEmail": "string",
  "contactPersonPhoneNo": "string",
  "partnershipArea": "string",
  "organizationalBackground": "string",
  "objectivesAndExpectations": "string",
  "audienceAndReach": "string",
  "resourcesForTalent": "string",
  "collaborationEffortDescription": "string",
  "logisticsAndOperationsInfo": "string",
  "communicationChannelsInformation": "string",
  "kpiMeasurementInformation": "string",
  "accountGenerationConsent": true,
  "accountNumber": "string",
  "accountName": "string"
}
```
- Response 200 `ExistingCustomerFlagResponseModel`:
```json
{
  "status": true,
  "message": "string",
  "statusCode": 100,
  "errors": [
    "string"
  ],
  "count": 0,
  "data": {
    "isExistingCustomer": true,
    "showOtpPage": true,
    "accountNumber": "string",
    "accountName": "string",
    "maskedPhoneNumber": "string"
  },
  "result": {
    "items": [
      {
        "isExistingCustomer": true,
        "showOtpPage": true,
        "accountNumber": "string",
        "accountName": "string",
        "maskedPhoneNumber": "string"
      }
    ],
    "totalItemCount": 0,
    "totalPageCount": 0,
    "currentPageSize": 0,
    "currentPageNumber": 0,
    "hasPrevious": true,
    "hasNext": true
  },
  "pager": {
    "pageNumber": 0,
    "pageCount": 0,
    "pageSize": 0,
    "hasNextPage": true,
    "hasPreviousPage": true,
    "firstItemOnPage": 0,
    "lastItemOnPage": 0,
    "totalItemCount": 0
  }
}
```

#### POST `/microsite-acct/api/AccountGeneration/SubmitHiredTalentRequest`
- Operation: */api/AccountGeneration/SubmitHiredTalentRequest - POST*
- Query params: `key:string`
- Request body `HiredTalentOnboardingRequestDto`:
```json
{
  "emailAddress": "string",
  "phoneNumber": "string",
  "requestChannel": "string",
  "bvn": "string",
  "nin": "string",
  "location": "string",
  "areaOfSpecialization": "string",
  "yearsOfExperience": "string",
  "willUndergoSpeedInterviewSession": true,
  "cv": "string",
  "referralSource": "string",
  "accountGenerationConsent": true,
  "accountNumber": "string",
  "accountName": "string"
}
```
- Response 200 `ExistingCustomerFlagResponseModel`:
```json
{
  "status": true,
  "message": "string",
  "statusCode": 100,
  "errors": [
    "string"
  ],
  "count": 0,
  "data": {
    "isExistingCustomer": true,
    "showOtpPage": true,
    "accountNumber": "string",
    "accountName": "string",
    "maskedPhoneNumber": "string"
  },
  "result": {
    "items": [
      {
        "isExistingCustomer": true,
        "showOtpPage": true,
        "accountNumber": "string",
        "accountName": "string",
        "maskedPhoneNumber": "string"
      }
    ],
    "totalItemCount": 0,
    "totalPageCount": 0,
    "currentPageSize": 0,
    "currentPageNumber": 0,
    "hasPrevious": true,
    "hasNext": true
  },
  "pager": {
    "pageNumber": 0,
    "pageCount": 0,
    "pageSize": 0,
    "hasNextPage": true,
    "hasPreviousPage": true,
    "firstItemOnPage": 0,
    "lastItemOnPage": 0,
    "totalItemCount": 0
  }
}
```

#### POST `/microsite-acct/api/AccountGeneration/SubmitHiredVendorRequest`
- Operation: */api/AccountGeneration/SubmitHiredVendorRequest - POST*
- Query params: `key:string`
- Request body `HiredVendorOnboardingRequestDto`:
```json
{
  "emailAddress": "string",
  "phoneNumber": "string",
  "requestChannel": "string",
  "bvn": "string",
  "nin": "string",
  "fullName": "string",
  "serviceArea": "string",
  "accountGenerationConsent": true,
  "accountNumber": "string",
  "accountName": "string",
  "proofOfPayment": true
}
```
- Response 200 `ExistingCustomerFlagResponseModel`:
```json
{
  "status": true,
  "message": "string",
  "statusCode": 100,
  "errors": [
    "string"
  ],
  "count": 0,
  "data": {
    "isExistingCustomer": true,
    "showOtpPage": true,
    "accountNumber": "string",
    "accountName": "string",
    "maskedPhoneNumber": "string"
  },
  "result": {
    "items": [
      {
        "isExistingCustomer": true,
        "showOtpPage": true,
        "accountNumber": "string",
        "accountName": "string",
        "maskedPhoneNumber": "string"
      }
    ],
    "totalItemCount": 0,
    "totalPageCount": 0,
    "currentPageSize": 0,
    "currentPageNumber": 0,
    "hasPrevious": true,
    "hasNext": true
  },
  "pager": {
    "pageNumber": 0,
    "pageCount": 0,
    "pageSize": 0,
    "hasNextPage": true,
    "hasPreviousPage": true,
    "firstItemOnPage": 0,
    "lastItemOnPage": 0,
    "totalItemCount": 0
  }
}
```

#### POST `/microsite-acct/api/AccountGeneration/SubmitLeatherFairApplicationRequest`
- Operation: */api/AccountGeneration/SubmitLeatherFairApplicationRequest - POST*
- Query params: `key:string`
- Request body `LeatherFairOnboardingRequest`:
```json
{
  "firstName": "string",
  "lastName": "string",
  "middleName": "string",
  "gender": 1,
  "dateOfBirth": "string",
  "emailAddress": "string",
  "phoneNumber": "string",
  "businessName": "string",
  "businessSector": "string",
  "age": 0,
  "stateOfResidence": "string",
  "cityOfResidence": "string",
  "website": "string",
  "instagramHandle": "string",
  "linkedInProfile": "string",
  "durationOfOperation": "string",
  "isGenerateRevenue": true,
  "outsourceProduction": "string",
  "productionLocation": "string",
  "businessNeed": "string",
  "moreBusinessInfo": "string",
  "needHelp": true,
  "bvn": "string",
  "nin": "string",
  "requestChannel": "string",
  "managerName": "string",
  "isFirstTimeVendor": true,
  "businessCategory": "string",
  "products": [
    "string"
  ],
  "preferredStall": "string"
}
```
- Response 200 `StringResponseModel`:
```json
{
  "status": true,
  "message": "string",
  "statusCode": 100,
  "errors": [
    "string"
  ],
  "count": 0,
  "data": "string",
  "result": {
    "items": [
      "string"
    ],
    "totalItemCount": 0,
    "totalPageCount": 0,
    "currentPageSize": 0,
    "currentPageNumber": 0,
    "hasPrevious": true,
    "hasNext": true
  },
  "pager": {
    "pageNumber": 0,
    "pageCount": 0,
    "pageSize": 0,
    "hasNextPage": true,
    "hasPreviousPage": true,
    "firstItemOnPage": 0,
    "lastItemOnPage": 0,
    "totalItemCount": 0
  }
}
```

#### POST `/microsite-acct/api/AccountGeneration/SubmitSaraRequest`
- Operation: */api/AccountGeneration/SubmitSaraRequest - POST*
- Query params: `key:string`
- Request body `CustomerOnboardingRequest`:
```json
{
  "firstName": "string",
  "lastName": "string",
  "middleName": "string",
  "gender": 1,
  "dateOfBirth": "string",
  "emailAddress": "string",
  "phoneNumber": "string",
  "maritalStatus": 1,
  "stateOfOrigin": "string",
  "lgaOfOrigin": "string",
  "stateOfResidence": "string",
  "lgaOfResidence": "string",
  "highestLevelOfEducation": "string",
  "technicalSkill": "string",
  "participationReason": "string",
  "requestChannel": "string",
  "bvn": "string",
  "nin": "string",
  "needHelp": true
}
```
- Response 200 `StringResponseModel`:
```json
{
  "status": true,
  "message": "string",
  "statusCode": 100,
  "errors": [
    "string"
  ],
  "count": 0,
  "data": "string",
  "result": {
    "items": [
      "string"
    ],
    "totalItemCount": 0,
    "totalPageCount": 0,
    "currentPageSize": 0,
    "currentPageNumber": 0,
    "hasPrevious": true,
    "hasNext": true
  },
  "pager": {
    "pageNumber": 0,
    "pageCount": 0,
    "pageSize": 0,
    "hasNextPage": true,
    "hasPreviousPage": true,
    "firstItemOnPage": 0,
    "lastItemOnPage": 0,
    "totalItemCount": 0
  }
}
```

#### POST `/microsite-acct/api/AccountGeneration/SubmitTNYOnboardingRequest`
- Operation: */api/AccountGeneration/SubmitTNYOnboardingRequest - POST*
- Query params: `key:string`
- Request body `TNYOnboardingRequest`:
```json
{
  "firstName": "string",
  "lastName": "string",
  "middleName": "string",
  "gender": 1,
  "dateOfBirth": "string",
  "emailAddress": "string",
  "phoneNumber": "string",
  "maritalStatus": 1,
  "stateOfOrigin": "string",
  "lgaOfOrigin": "string",
  "stateOfResidence": "string",
  "lgaOfResidence": "string",
  "highestLevelOfEducation": "string",
  "requestChannel": "string",
  "bvn": "string",
  "nin": "string",
  "needHelp": true
}
```
- Response 200 `StringResponseModel`:
```json
{
  "status": true,
  "message": "string",
  "statusCode": 100,
  "errors": [
    "string"
  ],
  "count": 0,
  "data": "string",
  "result": {
    "items": [
      "string"
    ],
    "totalItemCount": 0,
    "totalPageCount": 0,
    "currentPageSize": 0,
    "currentPageNumber": 0,
    "hasPrevious": true,
    "hasNext": true
  },
  "pager": {
    "pageNumber": 0,
    "pageCount": 0,
    "pageSize": 0,
    "hasNextPage": true,
    "hasPreviousPage": true,
    "firstItemOnPage": 0,
    "lastItemOnPage": 0,
    "totalItemCount": 0
  }
}
```

#### POST `/microsite-acct/api/AccountGeneration/ValidateBvnAndNinRequest`
- Operation: */api/AccountGeneration/ValidateBvnAndNinRequest - POST*
- Query params: `key:string`
- Request body `ValidateBvnAndNinRequest`:
```json
{
  "bvn": "string",
  "nin": "string",
  "requestChannel": "string",
  "phoneNumber": "string",
  "accountGenerationConsent": true
}
```
- Response 200 `ExistingCustomerFlagResponseModel`:
```json
{
  "status": true,
  "message": "string",
  "statusCode": 100,
  "errors": [
    "string"
  ],
  "count": 0,
  "data": {
    "isExistingCustomer": true,
    "showOtpPage": true,
    "accountNumber": "string",
    "accountName": "string",
    "maskedPhoneNumber": "string"
  },
  "result": {
    "items": [
      {
        "isExistingCustomer": true,
        "showOtpPage": true,
        "accountNumber": "string",
        "accountName": "string",
        "maskedPhoneNumber": "string"
      }
    ],
    "totalItemCount": 0,
    "totalPageCount": 0,
    "currentPageSize": 0,
    "currentPageNumber": 0,
    "hasPrevious": true,
    "hasNext": true
  },
  "pager": {
    "pageNumber": 0,
    "pageCount": 0,
    "pageSize": 0,
    "hasNextPage": true,
    "hasPreviousPage": true,
    "firstItemOnPage": 0,
    "lastItemOnPage": 0,
    "totalItemCount": 0
  }
}
```

#### POST `/microsite-acct/api/AccountGeneration/ValidateMicrositeOnboardingToAlatRequest`
- Operation: */api/AccountGeneration/ValidateMicrositeOnboardingToAlatRequest - POST*
- Query params: `key:string`
- Request body `ValidateBvnAndNinRequest`:
```json
{
  "bvn": "string",
  "nin": "string",
  "requestChannel": "string",
  "phoneNumber": "string",
  "accountGenerationConsent": true
}
```
- Response 200 `ExistingCustomerFlagResponseModel`:
```json
{
  "status": true,
  "message": "string",
  "statusCode": 100,
  "errors": [
    "string"
  ],
  "count": 0,
  "data": {
    "isExistingCustomer": true,
    "showOtpPage": true,
    "accountNumber": "string",
    "accountName": "string",
    "maskedPhoneNumber": "string"
  },
  "result": {
    "items": [
      {
        "isExistingCustomer": true,
        "showOtpPage": true,
        "accountNumber": "string",
        "accountName": "string",
        "maskedPhoneNumber": "string"
      }
    ],
    "totalItemCount": 0,
    "totalPageCount": 0,
    "currentPageSize": 0,
    "currentPageNumber": 0,
    "hasPrevious": true,
    "hasNext": true
  },
  "pager": {
    "pageNumber": 0,
    "pageCount": 0,
    "pageSize": 0,
    "hasNextPage": true,
    "hasPreviousPage": true,
    "firstItemOnPage": 0,
    "lastItemOnPage": 0,
    "totalItemCount": 0
  }
}
```

#### POST `/microsite-acct/api/AccountGeneration/ValidateMicrositeOnboardingToAlatRequestV2`
- Operation: */api/AccountGeneration/ValidateMicrositeOnboardingToAlatRequestV2 - POST*
- Query params: `key:string`
- Request body `ValidateBvnAndNinRequestV2`:
```json
{
  "bvn": "string",
  "nin": "string",
  "requestChannel": "string",
  "phoneNumber": "string",
  "accountGenerationConsent": true
}
```
- Response 200 `ExistingCustomerFlagResponseModel`:
```json
{
  "status": true,
  "message": "string",
  "statusCode": 100,
  "errors": [
    "string"
  ],
  "count": 0,
  "data": {
    "isExistingCustomer": true,
    "showOtpPage": true,
    "accountNumber": "string",
    "accountName": "string",
    "maskedPhoneNumber": "string"
  },
  "result": {
    "items": [
      {
        "isExistingCustomer": true,
        "showOtpPage": true,
        "accountNumber": "string",
        "accountName": "string",
        "maskedPhoneNumber": "string"
      }
    ],
    "totalItemCount": 0,
    "totalPageCount": 0,
    "currentPageSize": 0,
    "currentPageNumber": 0,
    "hasPrevious": true,
    "hasNext": true
  },
  "pager": {
    "pageNumber": 0,
    "pageCount": 0,
    "pageSize": 0,
    "hasNextPage": true,
    "hasPreviousPage": true,
    "firstItemOnPage": 0,
    "lastItemOnPage": 0,
    "totalItemCount": 0
  }
}
```

#### POST `/microsite-acct/api/AccountGeneration/ValidateOtp`
- Operation: */api/AccountGeneration/ValidateOtp - POST*
- Query params: `key:string`
- Request body `ValidateOtp`:
```json
{
  "otp": "string",
  "phoneNumber": "string",
  "channel": "string"
}
```
- Response 200 `NinDataResponseModel`:
```json
{
  "status": true,
  "message": "string",
  "statusCode": 100,
  "errors": [
    "string"
  ],
  "count": 0,
  "data": {
    "firstName": "string",
    "lastName": "string",
    "gender": "string",
    "dateOfBirth": "string"
  },
  "result": {
    "items": [
      {
        "firstName": "string",
        "lastName": "string",
        "gender": "string",
        "dateOfBirth": "string"
      }
    ],
    "totalItemCount": 0,
    "totalPageCount": 0,
    "currentPageSize": 0,
    "currentPageNumber": 0,
    "hasPrevious": true,
    "hasNext": true
  },
  "pager": {
    "pageNumber": 0,
    "pageCount": 0,
    "pageSize": 0,
    "hasNextPage": true,
    "hasPreviousPage": true,
    "firstItemOnPage": 0,
    "lastItemOnPage": 0,
    "totalItemCount": 0
  }
}
```

#### POST `/microsite-acct/api/AccountGeneration/ValidateOtpV2`
- Operation: */api/AccountGeneration/ValidateOtpV2 - POST*
- Query params: `key:string`
- Request body `ValidateOtp`:
```json
{
  "otp": "string",
  "phoneNumber": "string",
  "channel": "string"
}
```
- Response 200 `NinDataResponseModel`:
```json
{
  "status": true,
  "message": "string",
  "statusCode": 100,
  "errors": [
    "string"
  ],
  "count": 0,
  "data": {
    "firstName": "string",
    "lastName": "string",
    "gender": "string",
    "dateOfBirth": "string"
  },
  "result": {
    "items": [
      {
        "firstName": "string",
        "lastName": "string",
        "gender": "string",
        "dateOfBirth": "string"
      }
    ],
    "totalItemCount": 0,
    "totalPageCount": 0,
    "currentPageSize": 0,
    "currentPageNumber": 0,
    "hasPrevious": true,
    "hasNext": true
  },
  "pager": {
    "pageNumber": 0,
    "pageCount": 0,
    "pageSize": 0,
    "hasNextPage": true,
    "hasPreviousPage": true,
    "firstItemOnPage": 0,
    "lastItemOnPage": 0,
    "totalItemCount": 0
  }
}
```

#### POST `/microsite-acct/api/AccountGeneration/ValidateOtpV3`
- Operation: */api/AccountGeneration/ValidateOtpV3 - POST*
- Query params: `key:string`
- Request body `ValidateOtpV2`:
```json
{
  "otp": "string",
  "nin": "string",
  "channel": "string"
}
```
- Response 200 `StringResponseModel`:
```json
{
  "status": true,
  "message": "string",
  "statusCode": 100,
  "errors": [
    "string"
  ],
  "count": 0,
  "data": "string",
  "result": {
    "items": [
      "string"
    ],
    "totalItemCount": 0,
    "totalPageCount": 0,
    "currentPageSize": 0,
    "currentPageNumber": 0,
    "hasPrevious": true,
    "hasNext": true
  },
  "pager": {
    "pageNumber": 0,
    "pageCount": 0,
    "pageSize": 0,
    "hasNextPage": true,
    "hasPreviousPage": true,
    "firstItemOnPage": 0,
    "lastItemOnPage": 0,
    "totalItemCount": 0
  }
}
```

#### POST `/microsite-acct/api/Auth/LMS/Authenticate`
- Operation: */api/Auth/LMS/Authenticate - POST*
- Query params: `key:string`
- Request body `AuthenticationRequest`:
```json
{
  "emailAddress": "string",
  "channel": "string"
}
```
- Response 200 `AuthResponseResponseModel`:
```json
{
  "status": true,
  "message": "string",
  "statusCode": 100,
  "errors": [
    "string"
  ],
  "count": 0,
  "data": {
    "otpTrackingId": "string"
  },
  "result": {
    "items": [
      {
        "otpTrackingId": "string"
      }
    ],
    "totalItemCount": 0,
    "totalPageCount": 0,
    "currentPageSize": 0,
    "currentPageNumber": 0,
    "hasPrevious": true,
    "hasNext": true
  },
  "pager": {
    "pageNumber": 0,
    "pageCount": 0,
    "pageSize": 0,
    "hasNextPage": true,
    "hasPreviousPage": true,
    "firstItemOnPage": 0,
    "lastItemOnPage": 0,
    "totalItemCount": 0
  }
}
```

#### POST `/microsite-acct/api/Auth/LMS/Validate`
- Operation: */api/Auth/LMS/Validate - POST*
- Query params: `key:string`
- Request body `ValidateOtpModel`:
```json
{
  "otp": "string",
  "otpTrackingId": "string",
  "email": "string",
  "channel": "string"
}
```
- Response 200 `StringResponseModel`:
```json
{
  "status": true,
  "message": "string",
  "statusCode": 100,
  "errors": [
    "string"
  ],
  "count": 0,
  "data": "string",
  "result": {
    "items": [
      "string"
    ],
    "totalItemCount": 0,
    "totalPageCount": 0,
    "currentPageSize": 0,
    "currentPageNumber": 0,
    "hasPrevious": true,
    "hasNext": true
  },
  "pager": {
    "pageNumber": 0,
    "pageCount": 0,
    "pageSize": 0,
    "hasNextPage": true,
    "hasPreviousPage": true,
    "firstItemOnPage": 0,
    "lastItemOnPage": 0,
    "totalItemCount": 0
  }
}
```

#### POST `/microsite-acct/api/Auth/login`
- Operation: */api/Auth/login - POST*
- Request body `LoginDetails`:
```json
{
  "userName": "string",
  "password": "string"
}
```

#### POST `/microsite-acct/api/Auth/reissuetoken`
- Operation: */api/Auth/reissuetoken - POST*
- Request body `RefreshTokenRequest`:
```json
{
  "accessToken": "string",
  "refreshToken": "string"
}
```


### `moneyguardconnect-api`  ·  MoneyGuardConnect.Api
*The MoneyGuardConnect project integrates with MoneyGuard to monitor Alat customer's accounts against fraudulent activities*  
- Base path: `/moneyguardconnect`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### GET `/moneyguardconnect/api/MoneyGuardConnect/GetCustomerAccountByAccountNumber/{accountNumber}`
- Operation: */api/MoneyGuardConnect/GetCustomerAccountByAccountNumber/{accountNumber} - GET*
- Path params: `accountNumber:string`(req)
- Required headers: `x-alumnihub-session-id`
- Response 200 `BankAccountDetail`:
```json
{
  "id": "string",
  "number": "string",
  "balance": 0,
  "type": "string",
  "dateCreated": "string"
}
```

#### GET `/moneyguardconnect/api/MoneyGuardConnect/GetCustomerAccounts`
- Operation: */api/MoneyGuardConnect/GetCustomerAccounts - GET*
- Required headers: `x-alumnihub-session-id`
- Response 200 `ApiMoneyGuardConnectGetCustomerAccountsGet200ApplicationJsonResponse`:
```json
[
  {
    "id": "string",
    "number": "string",
    "balance": 0,
    "type": "string",
    "dateCreated": "string"
  }
]
```

#### GET `/moneyguardconnect/api/MoneyGuardConnect/GetCustomerDetails/{customerId}`
- Operation: */api/MoneyGuardConnect/GetCustomerDetails/{customerId} - GET*
- Path params: `customerId:string`(req)
- Response 200 `GetCustomerDetailsResponse`:
```json
{
  "customerId": "string",
  "surname": "string",
  "firstName": "string",
  "otherNames": "string",
  "gender": "string",
  "bvn": "string",
  "email": "string"
}
```


### `outlet-link-api`  ·  Outlet-Link API
- Base path: `/outlet-link`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### GET `/outlet-link/Merchant`
- Operation: */Merchant - GET*
- Query params: `page:integer`, `pageSize:integer`, `name:string`
- Response 200 `MerchantGet200ApplicationJsonResponse`:
```json
[
  {
    "id": "string",
    "email": "string",
    "phone": "string",
    "name": "string",
    "paylink": "string",
    "dateCreated": "string",
    "accountNumber": "string",
    "verificationToken": "string",
    "dateVerified": "string",
    "isVerified": true
  }
]
```

#### GET `/outlet-link/Merchant/{id}`
- Operation: */Merchant/{id} - GET*
- Path params: `id:string`(req)
- Response 200 `Merchant`:
```json
{
  "id": "string",
  "email": "string",
  "phone": "string",
  "name": "string",
  "paylink": "string",
  "dateCreated": "string",
  "accountNumber": "string",
  "verificationToken": "string",
  "dateVerified": "string",
  "isVerified": true
}
```

#### GET `/outlet-link/Merchant/Account/{accountNumber}`
- Operation: */Merchant/Account/{accountNumber} - GET*
- Path params: `accountNumber:string`(req)
- Response 200 `BankAccount`:
```json
{
  "customerId": "string",
  "accountNumber": "string",
  "availableBalance": 0.0,
  "schemeCode": "string",
  "schemeType": "string",
  "branchId": "string",
  "accountName": "string",
  "accountManagerId": "string",
  "currency": "string"
}
```

#### POST `/outlet-link/Merchant/Create`
- Operation: */Merchant/Create - POST*
- Request body `MerchantModel`:
```json
{
  "name": "string",
  "email": "string",
  "phone": "string",
  "accountNumber": "string",
  "paylink": "string"
}
```
- Response 200 `Merchant`:
```json
{
  "id": "string",
  "email": "string",
  "phone": "string",
  "name": "string",
  "paylink": "string",
  "dateCreated": "string",
  "accountNumber": "string",
  "verificationToken": "string",
  "dateVerified": "string",
  "isVerified": true
}
```

#### GET `/outlet-link/Merchant/GetByEmail`
- Operation: */Merchant/GetByEmail - GET*
- Query params: `email:string`
- Response 200 `Merchant`:
```json
{
  "id": "string",
  "email": "string",
  "phone": "string",
  "name": "string",
  "paylink": "string",
  "dateCreated": "string",
  "accountNumber": "string",
  "verificationToken": "string",
  "dateVerified": "string",
  "isVerified": true
}
```

#### GET `/outlet-link/Merchant/GetByLink`
- Operation: */Merchant/GetByLink - GET*
- Query params: `link:string`
- Response 200 `Merchant`:
```json
{
  "id": "string",
  "email": "string",
  "phone": "string",
  "name": "string",
  "paylink": "string",
  "dateCreated": "string",
  "accountNumber": "string",
  "verificationToken": "string",
  "dateVerified": "string",
  "isVerified": true
}
```

#### POST `/outlet-link/Merchant/RequestVerification`
- Operation: */Merchant/RequestVerification - POST*
- Request body `EmailModel`:
```json
{
  "email": "string"
}
```

#### POST `/outlet-link/Merchant/UpdateLink`
- Operation: */Merchant/UpdateLink - POST*
- Request body `LinkUpdateModel`:
```json
{
  "email": "string",
  "paylink": "string"
}
```

#### POST `/outlet-link/Merchant/Verify`
- Operation: */Merchant/Verify - POST*
- Request body `OtpModel`:
```json
{
  "email": "string",
  "token": "string",
  "accountNumber": "string"
}
```

#### POST `/outlet-link/Payment`
- Operation: */Payment - POST*
- Request body `PaymentModel`:
```json
{
  "merchantId": "string",
  "amount": 0.0,
  "payerName": "string",
  "payerEmail": "string",
  "narration": "string",
  "method": 1
}
```
- Response 200 `PaymentInfoDO`:
```json
{
  "merchantId": "string",
  "paymentMethod": 1,
  "merchantName": "string",
  "amount": 0.0,
  "status": 0,
  "bankInfo": {
    "transactionId": "string",
    "bankName": "string",
    "accountNumber": "string",
    "expiryMinutes": 0
  }
}
```

#### GET `/outlet-link/Payment/{transactionId}`
- Operation: */Payment/{transactionId} - GET*
- Path params: `transactionId:string`(req)
- Response 200 `PaymentInfoDO`:
```json
{
  "merchantId": "string",
  "paymentMethod": 1,
  "merchantName": "string",
  "amount": 0.0,
  "status": 0,
  "bankInfo": {
    "transactionId": "string",
    "bankName": "string",
    "accountNumber": "string",
    "expiryMinutes": 0
  }
}
```

#### POST `/outlet-link/Payment/Complete`
- Operation: */Payment/Complete - POST*
- Request body `TransactionModel`:
```json
{
  "amount": "string",
  "originatoraccountnumber": "string",
  "originatorname": "string",
  "narration": "string",
  "craccountname": "string",
  "craccount": "string",
  "bankname": "string",
  "bankcode": "string",
  "sessionid": "string",
  "paymentreference": "string",
  "requestdate": "string",
  "nibssresponse": "string",
  "sendstatus": "string",
  "sendresponse": "string"
}
```
- Response 200 `BankNotificationResponse`:
```json
{
  "transactionreference": "string",
  "status": "string",
  "status_desc": "string"
}
```

#### POST `/outlet-link/Payment/Enquire`
- Operation: */Payment/Enquire - POST*
- Request body `NameEnquiryModel`:
```json
{
  "accountnumber": "string"
}
```
- Response 200 `NameEnquiryResponse`:
```json
{
  "accountname": "string",
  "status": "string",
  "status_desc": "string"
}
```

#### GET `/outlet-link/Payment/Merchant`
- Operation: */Payment/Merchant - GET*
- Query params: `merchantLink:string`
- Response 200 `Merchant`:
```json
{
  "id": "string",
  "email": "string",
  "phone": "string",
  "name": "string",
  "paylink": "string",
  "dateCreated": "string",
  "accountNumber": "string",
  "verificationToken": "string",
  "dateVerified": "string",
  "isVerified": true
}
```

#### GET `/outlet-link/Receipt`
- Operation: */Receipt - GET*
- Query params: `merchantId:string`, `page:integer`, `pageSize:integer`
- Response 200 `ReceiptGet200ApplicationJsonResponse`:
```json
[
  {
    "id": "string",
    "merchantId": "string",
    "payerName": "string",
    "payerEmail": "string",
    "amount": 0.0,
    "narration": "string",
    "transactionId": "string",
    "remarks": "string",
    "date": "string",
    "method": 1,
    "status": 0,
    "settlementId": "string"
  }
]
```


### `partnership-account`  ·  Partnership Account Creation
- Base path: `/partnership-account`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### POST `/partnership-account/api/CustomerAccount/GenerateAccountForPatnerships`
- Operation: *Generate Account For Partnership*
- Required headers: `x-api-key`(req)
- Request body `PatnershipAccountRequest`:
```json
{
  "gender": "string",
  "email": "string",
  "firstName": "string",
  "lastName": "string",
  "dob": "string",
  "phoneNumber": "string",
  "schemeCode": "string"
}
```
- Response 200 `ResponseModel`:
```json
{
  "message": "string",
  "status": false,
  "code": "string"
}
```


### `payattitude-cardless-transaction-platform-api`  ·  Cardless Transaction Platform API (PayAttitude)
*An Api that manage requests from CardLess Transaction API. it also accepts and returns application/json data.*  
- Base path: `/cardlesspayattitude`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### POST `/cardlesspayattitude/api/PayAttitude/authenticateTransaction`
- Operation: */api/PayAttitude/authenticateTransaction - POST*
- Required headers: `apiKey`(req), `AuthToken`(req)
- Request body `PINAuthReqModel`:
```json
{
  "session": "string",
  "type": "string",
  "action": "string",
  "current": "string",
  "amount": 0,
  "account": "string",
  "phone": "string",
  "fee": 0,
  "name": "string",
  "summary": "string",
  "channel": "string",
  "channelId": "string",
  "code": "string",
  "authInfo": {
    "pin": "string"
  }
}
```
- Response 200 `PINAuthRespModel`:
```json
{
  "session": "string",
  "operator": "string",
  "account": "string",
  "phone": "string",
  "action": "string",
  "current": "string",
  "imei": "string",
  "imsi": "string",
  "transactionId": "string",
  "approvalCode": "string",
  "amount": "string",
  "fee": "string",
  "name": "string",
  "summary": "string",
  "statusCode": "string",
  "status": "string"
}
```

#### POST `/cardlesspayattitude/api/PayAttitude/NoticeOrReversal`
- Operation: */api/PayAttitude/NoticeOrReversal - POST*
- Required headers: `apiKey`(req), `AuthToken`(req)
- Request body `Notice_ReversalRequestModel`:
```json
{
  "transactionId": "string",
  "approvalCode": "string",
  "statusCode": "string",
  "status": "string",
  "session": "string",
  "type": "string",
  "action": "string",
  "current": "string",
  "uniqueCode": "string",
  "account": "string",
  "bank": "string",
  "phone": "string",
  "amount": "string",
  "code": "string",
  "actualStatusCode": "string",
  "actualStatus": "string",
  "actualId": "string",
  "reason": "string",
  "actualRRN": "string"
}
```
- Response 200 `NoticeRespModel`:
```json
{
  "session": "string",
  "success": true,
  "status": "string",
  "type": "string",
  "statusCode": "string"
}
```

#### POST `/cardlesspayattitude/api/PayAttitude/ValidatePhone`
- Operation: */api/PayAttitude/ValidatePhone - POST*
- Required headers: `apiKey`(req), `AuthToken`(req)
- Request body `PhoneValidationReqModel`:
```json
{
  "phone": "string"
}
```
- Response 200 `PhoneValidationRespModel`:
```json
{
  "status": "string",
  "success": true,
  "name": "string",
  "account": "string"
}
```

#### POST `/cardlesspayattitude/api/PayAttitudeInboundTransfer/Name-Enquiry.{format}`
- Operation: */api/PayAttitudeInboundTransfer/Name-Enquiry - POST*
- Path params: `format:`(req)
- Required headers: `apiKey`(req), `AuthToken`(req)
- Request body `NameEnquiry`:
```json
{
  "accountNumber": "string"
}
```
- Response 200 `NameEnquiryResponse`:
```json
{
  "responseCode": "string",
  "accountNumber": "string",
  "accountName": "string",
  "phoneNumber": "string"
}
```

#### POST `/cardlesspayattitude/api/PayAttitudeInboundTransfer/Reversal.{format}`
- Operation: */api/PayAttitudeInboundTransfer/Reversal - POST*
- Path params: `format:`(req)
- Required headers: `apiKey`(req), `AuthToken`(req)
- Request body `Reversal`:
```json
{
  "transactionId": "string"
}
```
- Response 200 `ReversalResponse`:
```json
{
  "transactionId": "string",
  "responseCode": "string",
  "description": "string",
  "refId": "string"
}
```

#### POST `/cardlesspayattitude/api/PayAttitudeInboundTransfer/Transfer.{format}`
- Operation: */api/PayAttitudeInboundTransfer/Transfer - POST*
- Path params: `format:`(req)
- Required headers: `apiKey`(req), `AuthToken`(req)
- Request body `Transfer`:
```json
{
  "transactionId": "string",
  "debitAccount": "string",
  "creditAccount": "string",
  "amount": "string",
  "narration": "string"
}
```
- Response 200 `TransferResponse`:
```json
{
  "transactionId": "string",
  "responseCode": "string",
  "description": "string",
  "refId": "string"
}
```

#### POST `/cardlesspayattitude/ ​/api​/PayAttitudeInboundTransfer​/TransferStatusQuery.{format}`
- Operation: */api/PayAttitudeInboundTransfer/TransferStatusQuery - POST*
- Path params: `format:`(req)
- Required headers: `apiKey`(req), `AuthToken`(req)
- Request body `TransferQuery`:
```json
{
  "transactionId": "string"
}
```
- Response 200 `TransferResponse`:
```json
{
  "transactionId": "string",
  "responseCode": "string",
  "description": "string",
  "refId": "string"
}
```


### `payout-api`  ·  Payout API
*This API enables merchants payout credits to customers in their books.*  
- Base path: `/payout`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### POST `/payout/api/IntraBankTransfer/FundWallet`
- Operation: *Endpoint for client to fund wallet*
- Required headers: `access`(req)
- Request body `FundWalletRequest`:
```json
{
  "securityInfo": "string",
  "destinationAccountNumber": "string",
  "amount": 0,
  "narration": "string",
  "transactionReference": "string",
  "useCustomNarration": true
}
```
- Response 200 `GlobalResponseEnvelope`:
```json
{
  "result": {
    "status": "string",
    "message": "string",
    "narration": "string",
    "transactionReference": "string",
    "platformTransactionReference": "string",
    "transactionStan": "string",
    "orinalTxnTransactionDate": "string"
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```

#### GET `/payout/api/Shared/AccountNameEnquiry/Wallet/{accountNumber}`
- Operation: *Name enquiry endpoint for client's wallet account*
- Path params: `accountNumber:string`(req)
- Required headers: `access`(req)
- Response 200 `NameEnquiryResponseEnvelope`:
```json
{
  "result": {
    "bankCode": "string",
    "accountName": "string",
    "accountNumber": "string",
    "currency": "string"
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```


### `smeloan-api`  ·  SMELoan.Api
*API to help customers access SME loans digitally.*  
- Base path: `/smeloan`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### POST `/smeloan/api/LoanApplication/Createloan`
- Operation: */api/LoanApplication/Createloan - POST*
- Request body `CreateNewLoanDto`:
```json
{
  "cif": "string",
  "loanProductId": "string",
  "accountNumber": "string",
  "loanAmount": 0,
  "paybackOptionId": "string",
  "loanPurpose": "string",
  "requestChannel": "string",
  "secondaryEmail": "string",
  "requestId": "string",
  "customData": [
    {
      "fieldName": "string",
      "value": "string"
    }
  ]
}
```
- Response 200 `StringResponse`:
```json
{
  "statusCode": 0,
  "data": "string",
  "status": true,
  "message": "string",
  "errors": {}
}
```

#### GET `/smeloan/api/LoanApplication/CustomerLoans/{cif}`
- Operation: */api/LoanApplication/CustomerLoans/{cif} - GET*
- Path params: `cif:string`(req)
- Query params: `PageNumber:integer`, `PageSize:integer`
- Response 200 `LoanToReturnPagedList`:
```json
{
  "currentPage": 0,
  "totalPages": 0,
  "pageSize": 0,
  "totalCount": 0,
  "hasPrevious": true,
  "hasNext": true,
  "data": [
    {
      "id": "string",
      "cif": "string",
      "loanProductId": "string",
      "accountNumber": "string",
      "loanAmount": 0,
      "paybackOptionId": "string",
      "loanPurpose": "string",
      "requestChannel": "string",
      "referenceNumber": "string",
      "loanIdSME": "string",
      "status": "string",
      "createdAt": "string"
    }
  ]
}
```

#### GET `/smeloan/api/LoanApplication/CustomerPendingLoan/{cif}`
- Operation: */api/LoanApplication/CustomerPendingLoan/{cif} - GET*
- Path params: `cif:string`(req)
- Response 200 `PendingLoanResponse`:
```json
{
  "statusCode": 0,
  "data": {
    "id": "string",
    "loanProductId": "string",
    "accountNumber": "string",
    "status": "string",
    "cif": "string",
    "createdAt": "string",
    "loanAmount": 0,
    "loanBalance": 0,
    "monthlyRepayment": 0,
    "principalAmount": 0,
    "repaymentAmount": 0,
    "interestRate": 0,
    "dueDate": "string",
    "repaymentStartDate": "string",
    "nextRepaymentDate": "string",
    "isOtherBankRequest": true,
    "loanDocuments": [
      {
        "id": 0,
        "productName": "string",
        "loanProductId": "string",
        "documents": [
          {
            "id": 0,
            "documenType": "string"
          }
        ]
      }
    ]
  },
  "status": true,
  "message": "string",
  "errors": {}
}
```

#### GET `/smeloan/api/LoanApplication/GetCustomerLoans/{accountNumberOrCustomerId}`
- Operation: */api/LoanApplication/GetCustomerLoans/{accountNumberOrCustomerId} - GET*
- Path params: `accountNumberOrCustomerId:string`(req)
- Response 200 `CustomerLoanListPaginatedResponse`:
```json
{
  "currentPage": 0,
  "pageSize": 0,
  "totalPages": 0,
  "totalCount": 0,
  "hasPrevious": true,
  "hasNext": true,
  "data": [
    {
      "customerId": "string",
      "accountNumber": "string",
      "accountName": "string",
      "taxID": "string",
      "bvn": "string",
      "accountBranchId": "string",
      "accountBranchName": "string",
      "accountSchemeCode": "string",
      "companyType": "string",
      "currency": "string",
      "loanProductId": "string",
      "loanProductName": "string",
      "loanAmount": 0,
      "paybackOption": "string",
      "loanPurpose": "string",
      "loanTenor": 0,
      "interestRate": 0,
      "moratoriumPeriod": 0,
      "requestType": "string",
      "eligibleAmount": 0,
      "averageCreditTurnover": 0,
      "turnoverPercentage": 0,
      "existingLoanWithOtherBank": true,
      "referenceNumber": "string",
      "accountOfficerEmail": "string",
      "branchManagerEmail": "string",
      "accountOfficerCode": "string",
      "isPrequalified": true,
      "isQualified": true,
      "isEligible": true,
      "eligibilityMethod": "string",
      "retryCount": 0,
      "primaryEmail": "string",
      "secondaryEmail": "string",
      "phoneNumber": "string",
      "businessName": "string",
      "businessDetails": "string",
      "businessRegNumber": "string",
      "businessRegistrationDate": "string",
      "businessSectorCode": "string",
      "businessSectorName": "string",
      "businessExperience": 0,
      "businessAddress": "string",
      "requestDate": "string",
      "requestChannel": "string",
      "requestBranchID": "string",
      "submittedBy": "string",
      "brokerCode": "string",
      "hasExceptionalApproval": true,
      "offerLetterIsSent": true,
      "offerLetterSentDate": "string",
      "confirmPostDatedCheck": true,
      "postDatedCheckConfirmedBy": "string",
      "crmS_CODE": "string",
      "creditCheckStatus": "string",
      "loanStatus": "string",
      "loanProcessor": "string",
      "loanChecker": "string",
      "readyForBooking": true,
      "bookingId": "string",
      "amountDisbursed": 0,
      "disbursementId": "string",
      "disbursementDate": "string",
      "customData": [
        {
          "fieldName": "string",
          "value": "string"
        }
      ],
      "id": "string",
      "createdDate": "string",
      "updatedDate": "string"
    }
  ]
}
```

#### PUT `/smeloan/api/LoanApplication/UpdateloanStatus`
- Operation: */api/LoanApplication/UpdateloanStatus - PUT*
- Request body `UpdateLoanStatusDto`:
```json
{
  "referenceNumber": "string",
  "loanStatus": "string"
}
```
- Response 200 `StringResponse`:
```json
{
  "statusCode": 0,
  "data": "string",
  "status": true,
  "message": "string",
  "errors": {}
}
```

#### POST `/smeloan/api/LoanEligibility/CheckEligibility`
- Operation: */api/LoanEligibility/CheckEligibility - POST*
- Request body `EligibilityCheckDto`:
```json
{
  "loanProductId": "string",
  "accountNumber": "string",
  "loanAmount": 0,
  "isReturnCustomer": true,
  "hasLoanWithOtherBank": true
}
```
- Response 200 `EligibilityResponseResponse`:
```json
{
  "statusCode": 0,
  "data": {
    "loanProductId": "string",
    "loanProductName": "string",
    "customerId": "string",
    "accountNumber": "string",
    "hasValidSchemeCode": true,
    "hasActiveAccount": true,
    "hasMinimumRelationship": true,
    "hasNoExistingLoan": true,
    "hasNoReturnChequeHistory": true,
    "hasNoExistingLoanWithOtherBank": true,
    "hasMinimumTurnover": true,
    "hasMinimumInflowCount": true,
    "hasEligibleLoanAmount": true,
    "canApplyWithExistingLoan": true,
    "hasValidBusinessSector": true,
    "hasValidBusinessExistence": true,
    "loanAmount": 0,
    "applicableTurnoverPercentage": 0,
    "isReturningCustomer": true,
    "defaultEligibleAmount": 0,
    "eligibleAmountWithAdditionalAccount": 0,
    "eligibleAmountWithOtherBankStatement": 0,
    "totalEligibleAmount": 0,
    "loanLimit": 0,
    "isEligible": true,
    "isQualified": true,
    "date": "string",
    "isArchived": true
  },
  "status": true,
  "message": "string",
  "errors": {}
}
```

#### GET `/smeloan/api/LoanEligibility/GetDocuments`
- Operation: */api/LoanEligibility/GetDocuments - GET*
- Response 200 `LoanProductDocumentListResponse`:
```json
{
  "statusCode": 0,
  "data": [
    {
      "id": 0,
      "productName": "string",
      "loanProductId": "string",
      "documents": [
        {
          "id": 0,
          "documenType": "string"
        }
      ]
    }
  ],
  "status": true,
  "message": "string",
  "errors": {}
}
```

#### GET `/smeloan/api/LoanEligibility/LoanProducts/GetList`
- Operation: */api/LoanEligibility/LoanProducts/GetList - GET*
- Response 200 `ApiLoanEligibilityLoanProductsGetListGet200ApplicationJsonResponse`:
```json
[
  {
    "productName": "string",
    "productCode": "string",
    "requiredSchemeCodes": "string",
    "minimumLoanAmount": 0,
    "maximumLoanAmount": 0,
    "moratoriumPeriod": 0,
    "minimumCreditInflowCount": 0,
    "averageMonthlyTurnover": 0,
    "preferredBusinessType": "string",
    "excludedBranches": "string",
    "minimumRelationshipInMonth": 0,
    "minimumBusinessExperience": 0,
    "allowExistingLoanWithSameBank": true,
    "allowExistingLoanWithOtherBank": true,
    "newCustomerTurnoverPercentage": 0,
    "returningCustomerTurnoverPercentage": 0,
    "otherBankStatementTurnoverPercentage": 0,
    "allowTopup": true,
    "enableOtherBankStatement": true,
    "internalStatementDuration": 0,
    "otherBankStatementDuration": 0,
    "enableAdditionalAccountStatement": true,
    "isActive": true,
    "createdBy": "string",
    "updatedBy": "string",
    "deleted": {},
    "deletedTime": "string",
    "customFields": [
      {
        "id": "string",
        "loanProductId": "string",
        "fieldName": "string",
        "isRequired": true,
        "isActive": true
      }
    ],
    "paybackOptions": [
      {
        "loanProductId": "string",
        "title": "string",
        "tenor": 0,
        "interestRate": 0,
        "id": "string",
        "createdDate": "string",
        "updatedDate": "string"
      }
    ],
    "id": "string",
    "createdDate": "string",
    "updatedDate": "string"
  }
]
```

#### GET `/smeloan/api/LoanEligibility/OtherBankStatementRequests/{cif}`
- Operation: */api/LoanEligibility/OtherBankStatementRequests/{cif} - GET*
- Path params: `cif:string`(req)
- Query params: `PageNumber:integer`, `PageSize:integer`

#### POST `/smeloan/api/LoanEligibility/OtherBankStatements/Confirm`
- Operation: */api/LoanEligibility/OtherBankStatements/Confirm - POST*
- Request body `ConfirmBankStatementDto`:
```json
{
  "ticketNo": "string",
  "password": "string",
  "requestId": "string"
}
```
- Response 200 `StringResponse`:
```json
{
  "statusCode": 0,
  "data": "string",
  "status": true,
  "message": "string",
  "errors": {}
}
```

#### GET `/smeloan/api/LoanEligibility/OtherBankStatements/GetActiveBanks`
- Operation: */api/LoanEligibility/OtherBankStatements/GetActiveBanks - GET*
- Response 200 `ApiLoanEligibilityOtherBankStatementsGetActiveBanksGet200ApplicationJsonResponse`:
```json
[
  {
    "id": 0,
    "name": "string",
    "sortCode": "string"
  }
]
```

#### POST `/smeloan/api/LoanEligibility/OtherBankStatements/RequestStatement`
- Operation: */api/LoanEligibility/OtherBankStatements/RequestStatement - POST*
- Request body `BankStatementRequestDto`:
```json
{
  "loanProductId": "string",
  "cif": "string",
  "otherBankId": 0,
  "otherBankName": "string",
  "otherBankAccountNumber": "string",
  "otherBankAccountName": "string",
  "phoneNumber": "string",
  "emailAddress": "string",
  "bvn": "string",
  "internalAccountNumber": "string"
}
```
- Response 200 `StringResponse`:
```json
{
  "statusCode": 0,
  "data": "string",
  "status": true,
  "message": "string",
  "errors": {}
}
```

#### POST `/smeloan/api/LoanManagement/LoanDocument/UploadByUrl`
- Operation: */api/LoanManagement/LoanDocument/UploadByUrl - POST*
- Request body `LoanDocumentDto`:
```json
{
  "loanId": "string",
  "uploadedBy": "string",
  "comment": "string",
  "files": [
    {
      "documentType": "string",
      "filename": "string",
      "fileExtension": "string",
      "fileMimeType": "string",
      "fileLength": 0,
      "filepath": "string"
    }
  ]
}
```
- Response 200 `StringResponse`:
```json
{
  "statusCode": 0,
  "data": "string",
  "status": true,
  "message": "string",
  "errors": {}
}
```

#### POST `/smeloan/api/LoanRecovery/CardTokenizations/Initiate`
- Operation: */api/LoanRecovery/CardTokenizations/Initiate - POST*
- Request body `CardTokenizationInitionDto`:
```json
{
  "loanId": "string",
  "paymentChannel": "string",
  "email": "string"
}
```
- Response 200 `CardTokenizationInitiationResponseResponse`:
```json
{
  "statusCode": 0,
  "data": {
    "referenceNumber": "string",
    "paymentKey": "string",
    "amount": 0
  },
  "status": true,
  "message": "string",
  "errors": {}
}
```


### `the-callback-url`  ·  The Callback URL
- Base path: `/callback-url`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### GET `/callback-url/callbackURL`
- Operation: *The Generic Callback URL*


### `verifyrewardcode`  ·  VerifyDiscountCode
- Base path: `/verify-discount-code`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### POST `/verify-discount-code/api/Deals/VerifyDiscountCode`
- Operation: *VerifyDiscountCode*
- Request body `VerifyDiscountCodeRequest`:
```json
{
  "discountCode": "string",
  "partnerId": "3fa85f64-5717-4562-b3fc-2c963f66afa6"
}
```
- Response 200 `VerifyDiscountCodeResponse`:
```json
{
  "data": {
    "transactionDate": "string",
    "transactionAmount": 0,
    "referenceNumber": "string",
    "transactionRemark": "string",
    "accountNumber": "string",
    "accountName": "string",
    "bank": "string",
    "code": "string",
    "discount": "string",
    "dealType": "string",
    "verifiedBy": "string",
    "verifiedAt": "string"
  },
  "hasError": true,
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "timeGenerated": "2022-11-15T17:30:39.119Z"
}
```


### `virtualaccountapi-api`  ·  Virtual Account API
- Base path: `/VirtualAccount`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### POST `/VirtualAccount/api/v1/Account/AccountLookup`
- Operation: */api/v1/Account/AccountLookup - POST*
- Request body `AccountLookupRequest`:
```json
{
  "accountnumber": "string",
  "bankcode": "string"
}
```

#### GET `/VirtualAccount/api/v1/Prefix`
- Operation: */api/v1/Prefix - GET*
- Response 200 `ApiV1PrefixGet200ApplicationJsonResponse`:
```json
[
  {
    "userName": "string",
    "prefix": "string",
    "currency": "string",
    "baseURL": "string",
    "nameEnquiryUri": "string",
    "transNotifyUri": "string",
    "authType": "string",
    "authKey": "string",
    "settleAccount": "string",
    "isActive": true
  }
]
```

#### GET `/VirtualAccount/api/v1/Prefix/{prefix}`
- Operation: */api/v1/Prefix/{prefix} - GET*
- Path params: `prefix:string`(req)
- Response 200 `PrefixSetup`:
```json
{
  "userName": "string",
  "prefix": "string",
  "currency": "string",
  "baseURL": "string",
  "nameEnquiryUri": "string",
  "transNotifyUri": "string",
  "authType": "string",
  "authKey": "string",
  "settleAccount": "string",
  "isActive": true
}
```

#### POST `/VirtualAccount/api/v1/Prefix/CreateNew`
- Operation: */api/v1/Prefix/CreateNew - POST*
- Request body `PrefixSetup`:
```json
{
  "userName": "string",
  "prefix": "string",
  "currency": "string",
  "baseURL": "string",
  "nameEnquiryUri": "string",
  "transNotifyUri": "string",
  "authType": "string",
  "authKey": "string",
  "settleAccount": "string",
  "isActive": true
}
```

#### POST `/VirtualAccount/api/v1/Prefix/Modify`
- Operation: */api/v1/Prefix/Modify - POST*
- Request body `PrefixSetup`:
```json
{
  "userName": "string",
  "prefix": "string",
  "currency": "string",
  "baseURL": "string",
  "nameEnquiryUri": "string",
  "transNotifyUri": "string",
  "authType": "string",
  "authKey": "string",
  "settleAccount": "string",
  "isActive": true
}
```

#### POST `/VirtualAccount/api/v1/Trans/TransNotify`
- Operation: */api/v1/Trans/TransNotify - POST*
- Request body `TransNotifyRequest`:
```json
{
  "originatoraccountnumber": "string",
  "amount": "string",
  "currency": "string",
  "originatorname": "string",
  "narration": "string",
  "craccountname": "string",
  "paymentreference": "string",
  "reference": "string",
  "bankname": "string",
  "sessionid": "string",
  "craccount": "string",
  "bankcode": "string",
  "created_at": "string"
}
```
- Response 200 `TransNotifyResponse`:
```json
{
  "ref": "string",
  "transactionreference": "string",
  "status": "string",
  "status_desc": "string"
}
```

#### POST `/VirtualAccount/api/v1/Trans/TransQuery`
- Operation: */api/v1/Trans/TransQuery - POST*
- Request body `TransQueryRequest`:
```json
{
  "sessionid": "string",
  "craccount": "string",
  "amount": 0.0,
  "txndate": "string"
}
```
- Response 200 `TransQueryResponse`:
```json
{
  "status": "string",
  "status_desc": "string",
  "transactions": [
    {
      "originatoraccountnumber": "string",
      "amount": "string",
      "originatorname": "string",
      "narration": "string",
      "craccountname": "string",
      "paymentreference": "string",
      "bankname": "string",
      "sessionid": "string",
      "craccount": "string",
      "bankcode": "string",
      "requestdate": "string",
      "nibssresponse": "string",
      "sendstatus": "string",
      "sendresponse": "string"
    }
  ]
}
```


### `wallet-transfer-api`  ·  Wallet Transfer API
*This API allows clients/channels to request and authorize payment for wallets in their ecosystem.*  
- Base path: `/wallet-transfer`  |  subscriptionRequired: `True`  |  sub-key header: `Ocp-Apim-Subscription-Key`

#### GET `/wallet-transfer/api/Shared/AccountNameEnquiry/Wallet/{accountNumber}`
- Operation: *Account Name Enquiry - Client specific wallets*
- Path params: `accountNumber:string`(req)
- Response 200 `NameEnquiryResponseEnvelope`:
```json
{
  "result": {
    "bankCode": "string",
    "accountName": "string",
    "accountNumber": "string",
    "currency": "string"
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```

#### GET `/wallet-transfer/api/Shared/GetAllBanks`
- Operation: *Step 1: Get All Banks*
- Response 200 `BanksEnvelope`:
```json
{
  "result": {
    "bankName": "string",
    "bankCode": "string"
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```

#### GET `/wallet-transfer/api/Shared/AccountNameEnquiry/{bankCode}/{accountNumber}`
- Operation: *Step 2: Account Name Enquiry*
- Path params: `bankCode:string`(req), `accountNumber:string`(req)
- Query params: `channelId:string`
- Response 200 `AccountNameEnquiryEnvelope`:
```json
{
  "result": {
    "bankCode": "string",
    "accountName": "string",
    "accountNumber": "string",
    "currency": "string",
    "termsAndConditions": "string",
    "termsAndConditionsUrl": "string",
    "chargeFee": [
      {
        "id": 0,
        "chargeFeeName": "string",
        "transactionType": 0,
        "charge": 0,
        "lower": 0,
        "upper": 0
      }
    ]
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```

#### GET `/wallet-transfer/api/Shared/GetNIPCharges`
- Operation: *Step 3: Get NIP Charges*
- Response 200 `NIPChargesEnvelope`:
```json
{
  "result": {
    "chargeFees": [
      {
        "id": 0,
        "chargeFeeName": "string",
        "transactionType": 0,
        "charge": 0,
        "lower": 0,
        "upper": 0
      }
    ],
    "termsAndConditions": "string",
    "termsAndConditionsUrl": "string"
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```

#### POST `/wallet-transfer/api/Shared/ProcessClientTransfer`
- Operation: *Step 4: Process Client Transfer*
- Required headers: `access`(req)
- Request body `ClientTransferRequestDto`:
```json
{
  "securityInfo": "string",
  "amount": 0,
  "destinationBankCode": "string",
  "destinationBankName": "string",
  "destinationAccountNumber": "string",
  "destinationAccountName": "string",
  "sourceAccountNumber": "string",
  "narration": "string",
  "transactionReference": "string",
  "useCustomNarration": true
}
```
- Response 200 `GlobalResponseEnvelope`:
```json
{
  "result": {
    "status": "string",
    "message": "string",
    "narration": "string",
    "transactionReference": "string",
    "platformTransactionReference": "string",
    "transactionStan": "string",
    "orinalTxnTransactionDate": "string"
  },
  "errorMessage": "string",
  "errorMessages": [
    "string"
  ],
  "hasError": true,
  "timeGenerated": "string"
}
```
