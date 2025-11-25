use std::time::Duration;

use billerix_connector::{
    api::BillerixApi,
    model::{ApiRequest, CreateOrderRequest},
};

#[tokio::main]
async fn main() {
    let base_url = "https://pay-sandbox.billerbase.com";
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
    let ip = "127.0.0.1";
    let result = api.geo_info(ip).await;
    println!("{result:?}");

    let result = api
        .create_order(&ApiRequest {
            ip: ip.to_string(),
            data: CreateOrderRequest {},
        })
        .await;
    println!("{result:?}");

    println!("RUN basic example: OK");
}
