use serde::{Deserialize, Serialize};

use crate::api::{decimal::Decimal, generic::ApiCryptoCurrency};

// data objects

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Course {
    pub course: Decimal,
    pub date: String,
}

// requests

// get course

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema, utoipa::IntoParams))]
pub struct GetCourseRequestQueries {
    pub currency: ApiCryptoCurrency,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema, utoipa::IntoParams))]
pub struct GetCourseResponse {
    pub course: Course,
}

// get course history

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema, utoipa::IntoParams))]
pub struct GetCourseHistoryRequestQueries {
    pub currency: ApiCryptoCurrency,
    pub from_date: Option<String>,
    pub to_date: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GetCourseHistoryResponse {
    pub courses: Vec<Course>,
}
