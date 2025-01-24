use serde::{Deserialize, Serialize};

/// Request for getting a new posident case id
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct NewCaseIdResponse {
    /// New Postident case id
    pub case_id: String,
    /// Username
    pub case_url: String,
}

/// Update the case details of a post identified by case_id.
#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UpdateCaseStatusRequest {
    /// Case ID
    pub case_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CaseDetailsResponse {
    pub case_id: String,
    pub archived: bool,
    pub status: String,
}
