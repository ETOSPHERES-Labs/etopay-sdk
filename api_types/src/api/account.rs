//! Contains types needed to interface with the /user endpoints

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NewCustomer {
    pub country_code: String,
    pub business_partner: BusinessPartner,
    pub contract_currency: ContractCurrency,
    pub vat_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Customer {
    pub username: String,
    pub country_code: String,
    pub business_partner: BusinessPartner,
    pub contract_currency: ContractCurrency,
    pub vat_id: Option<String>,
    pub customer_id: Option<String>,
    pub contractaccount: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum BusinessPartner {
    Privat,
    Company,
    Kleinunternehmer,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum ContractCurrency {
    EUR,
    USD,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct TaxData {
    pub country_code: String,
    pub tax_code: String,
    pub tax_percent: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct TaxInfoQuery {
    pub tax_process: TaxProcess,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum TaxProcess {
    Sell,
    Buy,
    Fee,
    ComplimentDonate,
    ComplimentReceive,
    ComplimentFee,
}
