use serde::{Deserialize, Serialize};

use super::detail::SwapPaymentDetailKey;

// data objects

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ViviPaymentMethod {
    pub id: String,
    pub key: SwapPaymentDetailKey,
    pub min_amount: f32,
    pub max_amount: f32,
    pub supported_deposit_currencies: Vec<String>,
    pub supported_withdrawal_method_keys: Vec<SwapPaymentDetailKey>,
    pub contract_type: String,
    pub is_incoming_payment_detail_required: bool,
    pub is_incoming_amount_required: bool,
    pub network_identifier: String,
}

// requests/responses

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ViviPaymentMethodsResponse {
    pub methods: Vec<ViviPaymentMethod>,
}
