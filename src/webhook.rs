use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookModel {
    /// v2.1: charge; v2.0 subscription
    #[serde(rename = "type")]
    pub event_type: String,
    /// charge_authorize, charge_capture та charge_settle
    pub action: String,
    pub merchant: String,
    #[serde(rename = "chargeData")]
    pub charge_data: ChargeData,
    #[serde(rename = "paymentMethod")]
    pub payment_method: PaymentMethod,
    pub provider: Provider,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChargeData {
    pub id: String,
    /// authorized, captured, settled
    pub status: String,
    #[serde(rename = "buyerId")]
    pub buyer_id: String,
    #[serde(rename = "buyerEmail")]
    pub buyer_email: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "isServiceFee")]
    pub is_service_fee: bool,
    pub order: OrderModel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderModel {
    pub id: String,
    /// initial, oneclick, autocharge
    #[serde(rename = "type")]
    pub order_type: String,
    pub currency: String,
    pub amount: String,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethod {
    #[serde(rename = "type")]
    pub method_type: String,
    pub data: PaymentMethodData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodData {
    pub bin: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Provider {
    pub name: String,
    pub mid: Mid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mid {
    pub name: String,
}
