use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionWebhookModel {
    #[serde(rename = "type")]
    pub event_type: String,

    pub action: String,
    pub merchant: String,

    #[serde(rename = "chargeId")]
    pub charge_id: String,

    pub buyer: Buyer,

    #[serde(rename = "subscriptionData")]
    pub subscription_data: SubscriptionData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Buyer {
    pub id: String,
    pub email: String,

    #[serde(rename = "metaData")]
    pub meta_data: Vec<serde_json::Value>, // unknown structure
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionData {
    pub id: String,
    pub status: String,

    #[serde(rename = "buyerId")]
    pub buyer_id: String,

    #[serde(rename = "buyerEmail")]
    pub buyer_email: String,

    #[serde(rename = "buyerMetaData")]
    pub buyer_meta_data: Vec<serde_json::Value>,

    #[serde(rename = "startDate")]
    pub start_date: NaiveDateTime,

    #[serde(rename = "endDate")]
    pub end_date: NaiveDateTime,

    #[serde(rename = "createdAt")]
    pub created_at: NaiveDateTime,

    #[serde(rename = "updatedAt")]
    pub updated_at: NaiveDateTime,

    #[serde(rename = "isManual")]
    pub is_manual: bool,

    #[serde(rename = "autochargeStatus")]
    pub autocharge_status: bool,

    pub product: Product,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub code: String,
    pub name: String,

    #[serde(rename = "nameByLocale")]
    pub name_by_locale: String,

    #[serde(rename = "productPrice")]
    pub product_price: ProductPrice,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductPrice {
    pub amount: f64,
    pub currency: String,
}
