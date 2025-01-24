use serde::{Deserialize, Serialize};

mod create;
mod details;
mod status;

pub use create::*;
pub use details::*;
pub use status::*;

// PurchaseModel
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PurchaseModel {
    CLIK,
    CPIC,
}
impl std::fmt::Display for PurchaseModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PurchaseModel::CLIK => write!(f, "CLIK"),
            PurchaseModel::CPIC => write!(f, "CPIC"),
        }
    }
}

impl TryFrom<String> for PurchaseModel {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "CLIK" => Ok(PurchaseModel::CLIK),
            "CPIC" => Ok(PurchaseModel::CPIC),
            _ => Err(format!("Invalid value for PurchaseModel: `{value}")),
        }
    }
}

// Reason
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Reason {
    PURCHASE,
    LIKE,
}

impl std::fmt::Display for Reason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Reason::PURCHASE => write!(f, "PURCHASE"),
            Reason::LIKE => write!(f, "LIKE"),
        }
    }
}

impl TryFrom<String> for Reason {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "PURCHASE" => Ok(Reason::PURCHASE),
            "LIKE" => Ok(Reason::LIKE),
            _ => Err(format!("Invalid value for Reason: `{value}")),
        }
    }
}
