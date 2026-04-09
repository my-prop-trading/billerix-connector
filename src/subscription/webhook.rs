use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscriptionWebhookModel {
    #[serde(rename = "type")]
    pub event_type: String,

    pub action: SubscriptionAction,
    pub merchant: String,

    #[serde(rename = "chargeId")]
    pub charge_id: String,

    pub buyer: Buyer,

    #[serde(rename = "subscriptionData")]
    pub subscription_data: SubscriptionData,
}

/// Exact webhook action values as stored in DB / received from API.
///
/// This enum is **string-backed** and matches values like:
/// `subscription_manual_renew`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubscriptionAction {
    // --- Freeze / Unfreeze ---
    #[serde(rename = "subscription_manual_freeze")]
    ManualFreeze,

    #[serde(rename = "subscription_manual_unfreeze")]
    ManualUnfreeze,

    #[serde(rename = "subscription_scheduled_unfreeze")]
    ScheduledUnfreeze,

    // --- Creation ---
    #[serde(rename = "subscription_manual_create")]
    ManualCreate,

    #[serde(rename = "subscription_user_create")]
    UserCreate,

    // --- Chargeback ---
    #[serde(rename = "subscription_event_chargeback")]
    EventChargeback,

    #[serde(rename = "subscription_manual_chargeback")]
    ManualChargeback,

    // --- Refund / Void ---
    #[serde(rename = "subscription_event_refund")]
    EventRefund,

    #[serde(rename = "subscription_event_void")]
    EventVoid,

    // --- Disable ---
    #[serde(rename = "subscription_manual_disable")]
    ManualDisable,

    #[serde(rename = "subscription_scheduled_disable")]
    ScheduledDisable,

    // --- Block ---
    #[serde(rename = "subscription_scheduled_failed_capture")]
    ScheduledFailedCapture,

    // --- Auto-charge ON ---
    #[serde(rename = "subscription_manual_auto_charge_on")]
    ManualAutoChargeOn,

    #[serde(rename = "subscription_user_auto_charge_on")]
    UserAutoChargeOn,

    #[serde(rename = "subscription_scheduled_auto_charge_on")]
    ScheduledAutoChargeOn,

    // --- Auto-charge OFF ---
    #[serde(rename = "subscription_manual_auto_charge_off")]
    ManualAutoChargeOff,

    #[serde(rename = "subscription_user_auto_charge_off")]
    UserAutoChargeOff,

    #[serde(rename = "subscription_scheduled_auto_charge_off")]
    ScheduledAutoChargeOff,

    #[serde(rename = "subscription_fraud_auto_charge_off")]
    FraudAutoChargeOff,

    #[serde(rename = "subscription_system_auto_charge_off")]
    SystemAutoChargeOff,

    // --- Upgrade / Downgrade / Renew ---
    #[serde(rename = "subscription_manual_upgrade")]
    ManualUpgrade,

    #[serde(rename = "subscription_user_upgrade")]
    UserUpgrade,

    #[serde(rename = "subscription_manual_downgrade")]
    ManualDowngrade,

    #[serde(rename = "subscription_user_downgrade")]
    UserDowngrade,

    #[serde(rename = "subscription_manual_renew")]
    ManualRenew,

    #[serde(rename = "subscription_user_renew")]
    UserRenew,

    #[serde(rename = "subscription_scheduled_renew")]
    ScheduledRenew,

    // --- End date / Expire ---
    #[serde(rename = "subscription_manual_end_date")]
    ManualEndDate,

    #[serde(rename = "subscription_scheduled_expire")]
    ScheduledExpire,

    // --- Tokenization ---
    #[serde(rename = "subscription_manual_tokenize")]
    ManualTokenize,

    #[serde(rename = "subscription_rotation_tokenize")]
    RotationTokenize,

    #[serde(rename = "subscription_scheduled_tokenize")]
    ScheduledTokenize,

    // --- Cleanup ---
    #[serde(rename = "subscription_scheduled_remove_expired_charge_method")]
    RemoveExpiredChargeMethod,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubscriptionActionWrapper {
    Known(SubscriptionAction),
    Unknown(String),
}
