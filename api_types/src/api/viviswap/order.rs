use serde::{Deserialize, Serialize};

use super::detail::PaymentDetail;
// data objects

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum OrderStatus {
    Pending,
    Canceled,
    Refunded,
    Failed,
    Success,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Order {
    pub id: String,
    pub is_payed_out: bool,
    pub is_approved: bool,
    pub is_canceled: bool,
    pub fees_amount_eur: f32,
    pub crypto_fees: f32,
    pub contract_id: String,
    pub incoming_payment_method_id: String,
    pub incoming_payment_method_currency: String,
    pub incoming_amount: f32,
    pub incoming_course: f32,
    pub outgoing_payment_method_id: String,
    pub outgoing_payment_method_currency: String,
    pub outgoing_amount: f32,
    pub outgoing_course: f32,
    pub refund_amount: Option<f32>,
    pub refund_course: Option<f32>,
    pub refund_payment_method_id: Option<String>,
    pub status: i32,
    pub creation_date: String,
    pub incoming_payment_detail: Option<PaymentDetail>,
    pub outgoing_payment_detail: Option<PaymentDetail>,
    pub refund_payment_detail: Option<PaymentDetail>,
}

/// Orders list
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct OrderList {
    pub orders: Vec<Order>,
}

// request/response objects

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema, utoipa::IntoParams))]
pub struct GetOrderQuery {
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GetOrderResponse {
    pub order: Order,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema, utoipa::IntoParams))]
pub struct GetOrdersQuery {
    pub start: u32,
    pub limit: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GetOrdersResponse {
    pub count: i32,
    pub start: i32,
    pub limit: i32,
    pub orders: Vec<Order>,
}
