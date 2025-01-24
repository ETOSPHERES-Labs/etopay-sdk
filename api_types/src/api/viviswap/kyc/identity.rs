use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::File;

// data objects

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum OfficialDocumentType {
    Passport,
    DriversLicense,
    Id,
}

impl FromStr for OfficialDocumentType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "passport" => Ok(Self::Passport),
            "driverslicense" => Ok(Self::DriversLicense),
            "id" => Ok(Self::Id),
            _ => Err(format!(
                "'{s}' is not a valid value for OfficialDocumentType, expected 'passport', 'driverslicense' or 'id'"
            )),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct IdentityOfficialDocumentData {
    pub r#type: OfficialDocumentType,
    pub expiration_date: String,
    pub document_number: String,
    pub front_image: File,
    pub back_image: Option<File>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct IdentityPersonalDocumentData {
    pub video: File,
}

// request / response objects

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SetIdentityDataRequest {
    pub official_document: IdentityOfficialDocumentData,
    pub personal_document: IdentityPersonalDocumentData,
}
