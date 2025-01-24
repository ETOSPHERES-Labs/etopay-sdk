use serde::{Deserialize, Serialize};

use super::File;

// data objects

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct KycOpenDocument {
    pub id: String,
    pub is_back_image_required: bool,
    pub r#type: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SetDocumentDataRequest {
    pub document_id: String,
    pub expiration_date: String,
    pub document_number: String,
    pub front_image: Option<File>,
    pub back_image: Option<File>,
}

// request / response objects

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GetKycDocumentsResponse {
    pub documents: Vec<KycOpenDocument>,
}
