use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SetPersonalDataRequest {
    pub full_name: String,
    pub date_of_birth: String,
}
