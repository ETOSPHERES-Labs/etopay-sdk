use serde::{Deserialize, Serialize};

// data objects

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Currency {
    pub short: String,
    pub name: String,
    pub character: String,
    pub course: f32,
    pub date: String,
    pub decimals: i32,
    pub iso_code: String,
    pub is_digital_asset: bool,
    pub networks: Vec<Network>,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Network {
    pub name: String,
    pub identifier: String,
    pub explorer_url: String,
    pub level: i32,
    pub base_network_identifier: Option<String>,
    pub is_disabled: bool,
}

// requests/responses

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GetCurrenciesResponse {
    pub currencies: Vec<Currency>,
}
