use serde::{Deserialize, Serialize};

// data objects

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum KycStep {
    Undefined,
    General,
    Personal,
    Identity,
    Residence,
    Amla,
    Document,
    Completed,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum KycVerificationStatus {
    Unverified,
    PartiallyVerified,
    Verified,
}

// request / response objects

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct KycDetailsResponse {
    pub is_verified: bool,
    pub is_individual: bool,
    pub full_name: String,
    pub submission_step: KycStep,
    pub verified_step: KycStep,
    pub verification_status: KycVerificationStatus,
    pub monthly_limit_eur: f32,
}
