use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub struct ApiRequest<T: Serialize> {
    pub ip: String,
    pub data: T,
}

#[derive(Debug, Serialize, Clone)]
pub struct GeoInfoRequest {
    pub ip: String,
    pub merchant: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GeoInfoResponse {
    #[serde(rename = "recommendedCurrency")]
    pub recommended_currency: String,
    #[serde(rename = "iso2Code")]
    pub iso2_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OneTimePaymentRequest {
    #[serde(rename = "metaData")]
    pub metadata: Option<HashMap<String, String>>,
    pub price: PriceModel,
    pub buyer: Option<BuyerModel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceModel {
    pub amount: f64,
    pub currency: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuyerModel {
    pub email: String,
    pub locale: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OneTimePaymentResponse {}
