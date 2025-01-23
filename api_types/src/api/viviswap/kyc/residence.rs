use serde::{Deserialize, Serialize};

use super::File;

// data objects

// request / response objects

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SetResidenceDataRequest {
    pub country_code: String,
    pub region: String,
    pub zip_code: String,
    pub city: String,
    pub address_line_1: String,
    pub address_line_2: String,
    pub is_public_entry: bool,
    pub public_entry_reference: Option<String>,
    pub has_no_official_document: bool,
    pub document_residence_proof: Option<File>,
}
