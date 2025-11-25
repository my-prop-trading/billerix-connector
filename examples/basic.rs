use std::{collections::HashMap, time::Duration};

use billerix_connector::{
    api::BillerixApi,
    model::{ApiRequest, OneTimePaymentRequest, PriceModel, Source},
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
    let ip = "66.94.29.13";
    let source = Source::Shop;
    let source_id = "local";
    let result = api
        .geo_info(ApiRequest {
            ip: ip.to_string(),
            data: (),
            source,
            source_id: source_id.to_string(),
        })
        .await;
    println!("{result:?}");

    let result = api
        .one_time_payment(&ApiRequest {
            ip: ip.to_string(),
            source: source.clone(),
            data: OneTimePaymentRequest {
                metadata: Some(HashMap::from([(
                    "test-key".to_string(),
                    "test-value".to_string(),
                )])),
                price: PriceModel {
                    amount: 5.0,
                    currency: "USD".to_string(),
                },
                buyer: None,
            },
            source_id: source_id.to_string(),
        })
        .await;
    println!("{result:?}");

    println!("RUN basic example: OK");
}
