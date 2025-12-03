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
    #[serde(rename = "paymentMethod")]
    pub payment_method: PaymentMethod,
    pub provider: Provider,
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
    pub data: Option<PaymentMethodData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentMethodData {
    pub bin: Option<String>,
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

#[cfg(test)]
pub mod test {
    use crate::webhook::WebhookModel;

    #[test]
    pub fn parse_webhook() {
        let str = r#"
            {"type":"charge","action":"charge_settle","merchant":"merch","chargeData":{"id":"86ee39a5-8943-49a4-a8a1-346d90fdf03e","status":"settled","buyerId":"a0814ae3-81e9-48ae-943f-97c735706431","buyerEmail":"billerix-alpha-test2@mailinator.com","isServiceFee":false,"createdAt":"2025-12-03 14:31:34","order":{"id":"ZjDhLbAZ","type":"checkout","currency":"USD","amount":"245.00","locale":"en","metaData":{"payload":"CMjJ9ObOoZEDEiRlYjY1Mzg/cIRW4md594YYf/BgUtbjDlD6zzKmq0Gh9P4Fpj+eNlvxw36B+oEz52AkN/pQ2Bm/zHJAYS+QXwHhokDmea+cWxT0c9SO30YD4JaiGgrC9b/QUqdu60HZAKSQRC5krB+1CYcPXnsqKEXV0zf5mAlSKBvvHT4alVPRwunsqu/1t5+5V16zTM0FEWFaoDQZ703HZLwY1Ll9iNCwZUXs0faxLE2EwfrfOv88kzwN5GPb","order_id":"e29df24c-55ca-494f-af0d-4f6c27ec64ca","client_id":"eb653c41-d13b-4447-974a-7f22ff80e4f4","product_id":"4a1ce74c-372a-4296-a40b-ebd318a5222f"}},"paymentMethod":{"type":"card","data":{"bin":"424242"}},"provider":{"name":"checkout_com","mid":{"name":"checkout_chkt_ks"}}}}
            "#;
        let webhook: Result<WebhookModel, _> = serde_json::from_str(&str);

        webhook.unwrap();
    }
}
