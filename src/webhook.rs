use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookModel {
    /// v2.1: charge; v2.0 subscription
    #[serde(rename = "type")]
    pub event_type: String,
    /// charge_authorize, charge_capture, charge_settle, charge_unsuccess, ...
    pub action: String,
    pub merchant: String,
    /// Top-level buyer in newer payload shape (unified with subscription webhook).
    /// Older charge payloads carried `buyerId`/`buyerEmail` inside `chargeData` instead.
    #[serde(default)]
    pub buyer: Option<Buyer>,
    #[serde(rename = "chargeData")]
    pub charge_data: ChargeData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Buyer {
    pub id: String,
    #[serde(default)]
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChargeData {
    pub id: String,
    /// authorized, captured, settled, declined
    pub status: String,
    /// Legacy field, kept for backward compatibility with older payloads.
    /// Newer payloads put it on the top-level `buyer.id`.
    #[serde(rename = "buyerId", default)]
    pub buyer_id: Option<String>,
    /// Legacy field, kept for backward compatibility with older payloads.
    /// Newer payloads put it on the top-level `buyer.email`.
    #[serde(rename = "buyerEmail", default)]
    pub buyer_email: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "isServiceFee")]
    pub is_service_fee: bool,
    pub order: OrderModel,
    #[serde(rename = "paymentMethod")]
    pub payment_method: PaymentMethod,
    pub provider: Provider,
    #[serde(rename = "declineData")]
    pub decline_data: Option<DeclineData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeclineData {
    pub r#type: String,
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderModel {
    pub id: String,
    /// initial, oneclick, autocharge
    #[serde(rename = "type")]
    pub order_type: String,
    pub currency: String,
    pub amount: String,
    #[serde(rename = "metaData")]
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
    pub fn parse_webhook_legacy_buyer_in_charge_data() {
        let str = r#"
            {"type":"charge","action":"charge_settle","merchant":"merch","chargeData":{"id":"86ee39a5-8943-49a4-a8a1-346d90fdf03e","status":"settled","buyerId":"a0814ae3-81e9-48ae-943f-97c735706431","buyerEmail":"billerix-alpha-test2@mailinator.com","isServiceFee":false,"createdAt":"2025-12-03 14:31:34","order":{"id":"ZjDhLbAZ","type":"checkout","currency":"USD","amount":"245.00","locale":"en","metaData":{"payload":"CMjJ9ObOoZEDEiRlYjY1Mzg/cIRW4md594YYf/BgUtbjDlD6zzKmq0Gh9P4Fpj+eNlvxw36B+oEz52AkN/pQ2Bm/zHJAYS+QXwHhokDmea+cWxT0c9SO30YD4JaiGgrC9b/QUqdu60HZAKSQRC5krB+1CYcPXnsqKEXV0zf5mAlSKBvvHT4alVPRwunsqu/1t5+5V16zTM0FEWFaoDQZ703HZLwY1Ll9iNCwZUXs0faxLE2EwfrfOv88kzwN5GPb","order_id":"e29df24c-55ca-494f-af0d-4f6c27ec64ca","client_id":"eb653c41-d13b-4447-974a-7f22ff80e4f4","product_id":"4a1ce74c-372a-4296-a40b-ebd318a5222f"}},"paymentMethod":{"type":"card","data":{"bin":"424242"}},"provider":{"name":"checkout_com","mid":{"name":"checkout_chkt_ks"}}}}
            "#;
        let webhook: WebhookModel = serde_json::from_str(&str).unwrap();

        assert!(webhook.buyer.is_none());
        assert_eq!(
            webhook.charge_data.buyer_id.as_deref(),
            Some("a0814ae3-81e9-48ae-943f-97c735706431")
        );
        assert_eq!(
            webhook.charge_data.buyer_email.as_deref(),
            Some("billerix-alpha-test2@mailinator.com")
        );
    }

    #[test]
    pub fn parse_webhook_top_level_buyer_with_decline() {
        let str = r#"{"type":"charge_unsuccess","action":"charge_unsuccess","merchant":"onefunded","buyer":{"id":"a1b5bef9-949d-4cae-8e12-a355582b6010","email":"harilla16@outlook.com","metaData":[]},"chargeData":{"id":"39ed206f-be07-4d5c-bc39-96e6f607de70","status":"declined","firstName":"Redouane","lastName":"Harilla","isServiceFee":false,"createdAt":"2026-05-14 00:36:02.000000","order":{"id":"ZjipYPEN","type":"checkout","currency":"USD","amount":"12.80","locale":"en","metaData":{"payload":"x","order_id":"4f7deb6d-41ef-464f-9d89-632f93b94f63","client_id":"47f7ff8f-c51e-4167-b31e-270961f59693","product_id":"ad330837-a93c-4fca-88ad-aea268dd243b"}},"paymentMethod":{"type":"card","data":{"bin":"517800","lastFour":"2153"},"binData":{"brand":"mastercard","bin":"517800","issuer":"FIRST PREMIER BANK","cardType":"CREDIT","cardLevel":"STANDARD","countryIsoTwo":"US"}},"provider":{"name":"stripe","mid":{"name":"stripe_strp_oneatls_shp"}},"declineData":{"type":"general_decline","errorMessage":"Your card has been declined.","code":"stripe.try_again_later"}}}"#;
        let webhook: WebhookModel = serde_json::from_str(&str).unwrap();

        let buyer = webhook.buyer.expect("buyer must be present at top level");
        assert_eq!(buyer.id, "a1b5bef9-949d-4cae-8e12-a355582b6010");
        assert_eq!(buyer.email.as_deref(), Some("harilla16@outlook.com"));
        assert!(webhook.charge_data.buyer_id.is_none());
        assert!(webhook.charge_data.buyer_email.is_none());
        let decline = webhook.charge_data.decline_data.expect("decline data");
        assert_eq!(decline.code, "stripe.try_again_later");
    }
}
