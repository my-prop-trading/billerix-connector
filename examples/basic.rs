use std::time::Duration;

use billerix_connector::{
    api::BillerixApi,
    model::{ApiRequest, CreateOrderRequest},
};

#[tokio::main]
async fn main() {
    let base_url = "todo";
    let merchant_code = std::env::var("BILLERIX_MERCHANT_CODE").unwrap();
    let public_key = std::env::var("BILLERIX_PUBLIC_KEY").unwrap();
    let secrete_key = std::env::var("BILLERIX_SECRET_KEY").unwrap();
    let api = BillerixApi::new(
        base_url,
        merchant_code,
        public_key,
        secrete_key,
        Duration::from_secs(15),
    );
    let result = api.create_order(&ApiRequest {
        buyer_ip: "127.0.0.1".to_string(),
        data: CreateOrderRequest {},
    })
    .await;
    println!("{result:?}");

    println!("RUN basic example: OK");
}
