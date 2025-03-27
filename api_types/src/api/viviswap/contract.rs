use serde::{Deserialize, Serialize};

use crate::api::decimal::Decimal;

// data objects

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ViviswapContract {
    pub id: String,
    pub reference: String,
    pub incoming_payment_method_id: String,
    pub incoming_payment_detail_id: Option<String>,
    pub outgoing_payment_method_id: String,
    pub outgoing_payment_detail_id: String,
    pub details: Option<ViviswapApiContractDetails>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ViviswapApiContractBankDetails {
    pub beneficiary: String,
    pub name_of_bank: String,
    pub address_of_bank: String,
    pub address: String,
    pub bic: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ViviswapApiContractCryptoDetails {
    pub deposit_address: String,
    pub wallet_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ViviswapApiContractSofortDetails {
    pub transaction_id: String,
    pub payment_url: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum ViviswapApiContractDetails {
    BankAccount(ViviswapApiContractBankDetails),
    Crypto(ViviswapApiContractCryptoDetails),
    Sofort(ViviswapApiContractSofortDetails),
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SimplifiedContract {
    pub id: String,
    pub reference: String,
    pub incoming_payment_method_id: String,
    pub incoming_payment_detail_id: Option<String>,
    pub outgoing_payment_method_id: String,
    pub outgoing_payment_detail_id: String,
}

// serialization objects

// create contract

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ContractRequestBody {
    pub amount: Option<Decimal>,
    pub incoming_payment_method_id: String,
    pub incoming_payment_detail_id: Option<String>,
    pub outgoing_payment_method_id: String,
    pub outgoing_payment_detail_id: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ViviswapContractCreationResponse {
    pub contract: Option<ViviswapContract>,
}

// delete contract

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema, utoipa::IntoParams))]
pub struct DeleteContractRequestPaths {
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct DeleteContractResponse {
    pub contract: Option<SimplifiedContract>,
}

// get contract

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema, utoipa::IntoParams))]
pub struct ContractRequestPaths {
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GetContractResponse {
    pub contract: ViviswapContract,
}

// get contracts

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GetContractsResponse {
    pub contracts: Vec<SimplifiedContract>,
}
