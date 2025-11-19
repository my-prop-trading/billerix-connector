use serde::{Deserialize, Serialize};

pub struct ApiRequest<T: Serialize> {
    pub buyer_ip: String,
    pub data: T,
}

#[derive(Debug, Serialize, Clone)]
pub struct CreateOrderRequest {
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateOrderResponse {}
