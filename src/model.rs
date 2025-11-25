use serde::{Deserialize, Serialize};

pub struct ApiRequest<T: Serialize> {
    pub ip: String,
    pub data: T,
}

#[derive(Debug, Serialize, Clone)]
pub struct CreateOrderRequest {}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateOrderResponse {}

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
