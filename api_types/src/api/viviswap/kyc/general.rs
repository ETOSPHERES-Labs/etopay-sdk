use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SetGeneralDataRequest {
    pub is_individual: bool,
    pub is_pep: bool,
    pub is_us_citizen: bool,
    pub is_regulatory_disclosure: bool,
    pub country_of_residence: String,
    pub nationality: String,
}
