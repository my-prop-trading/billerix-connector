use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Source {
    /// Саппорт-панель (staff panel)
    Staff,

    /// Сторінки магазину (shop)
    Shop,

    /// Контрольна панель клієнта/баєра (client panel)
    Cp,

    /// Пряма лінка на ордер (directlink)
    Directlink,

    /// Відділ продажів (sales)
    Sales,
}

impl Source {
    pub fn as_str(&self) -> &'static str {
        match self {
            Source::Staff => "staff",
            Source::Shop => "shop",
            Source::Cp => "cp",
            Source::Directlink => "directlink",
            Source::Sales => "sales",
        }
    }
}

pub struct ApiRequest<T: Serialize> {
    pub ip: String,
    pub data: T,
    pub source: Source,
    pub source_id: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
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
pub struct OneTimePaymentResponse {
    pub url: String,
}
