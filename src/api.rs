use crate::format_date;
use crate::model::{
    ApiRequest, GeoInfoRequest, GeoInfoResponse, OneTimePaymentRequest, OneTimePaymentResponse,
};
use chrono::Utc;
use flurl::{hyper::Method, FlUrl};
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;
use std::time::Duration;

pub struct BillerixApi {
    base_url: String,
    merchant_code: String,
    public_key: String,
    secret_key: String,
    timeout: Duration,
}

impl BillerixApi {
    pub fn new(
        base_url: impl Into<String>,
        merchant_code: impl Into<String>,
        public_key: impl Into<String>,
        secret_key: impl Into<String>,
        timeout: Duration,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            merchant_code: merchant_code.into(),
            public_key: public_key.into(),
            secret_key: secret_key.into(),
            timeout,
        }
    }

    pub async fn one_time_payment(
        &self,
        req: &ApiRequest<OneTimePaymentRequest>,
    ) -> Result<OneTimePaymentResponse, String> {
        let endpoint = "/api/v3/initials/one-time/url";
        let method = Method::POST;
        self.send_flurl_deserialized(endpoint, &method, req).await
    }

    pub async fn geo_info(&self, req: ApiRequest<()>) -> Result<GeoInfoResponse, String> {
        let endpoint = "api/v3/geoip/info";
        let method = Method::GET;
        self.send_flurl_deserialized(
            endpoint,
            &method,
            &ApiRequest {
                ip: req.ip.clone(),
                data: GeoInfoRequest {
                    ip: req.ip,
                    merchant: self.merchant_code.clone(),
                },
                source: req.source,
                source_id: req.source_id,
            },
        )
        .await
    }

    async fn send_flurl_deserialized<R: Serialize + Debug, T: DeserializeOwned + Debug>(
        &self,
        endpoint: &str,
        method: &Method,
        request: &ApiRequest<R>,
    ) -> Result<T, String> {
        let response = self.send_flurl(endpoint, method, request).await?;
        let result: Result<T, _> = serde_json::from_str(&response);

        let Ok(body) = result else {
            let msg = format!(
                "Failed to deserialize: {:?}. Url: {:?} {:?}. Request: {:?}. Body: {}",
                result, method, endpoint, request.data, response
            );
            return Err(msg);
        };

        Ok(body)
    }

    async fn send_flurl<R: Serialize + Debug>(
        &self,
        endpoint: &str,
        method: &Method,
        request: &ApiRequest<R>,
    ) -> Result<String, String> {
        let request_json = serde_json::to_string(&request.data).map_err(|e| format!("{:?}", e))?;
        let request_bytes: Option<Vec<u8>> = Some(request_json.clone().into_bytes());
        let flurl = self.build_flurl(endpoint, request);

        let result = if method == Method::GET {
            flurl.get().await
        } else if method == Method::POST {
            flurl.post(request_bytes).await
        } else if method == Method::PUT {
            flurl.put(request_bytes).await
        } else if method == Method::PATCH {
            flurl.patch(request_bytes).await
        } else if method == Method::DELETE {
            flurl.delete().await
        } else {
            panic!("not implemented");
        };

        let Ok(resp) = result else {
            return Err(format!(
                "FlUrl failed to receive_body: Url: {}. Request: {:?}. {:?}",
                endpoint,
                request_json,
                result.unwrap_err()
            ));
        };

        handle_flurl_resp(resp, Some(&request_json), endpoint, method).await
    }

    fn build_flurl<T: Serialize>(&self, endpoint: &str, req: &ApiRequest<T>) -> FlUrl {
        let url = format!("{}/{}", self.base_url, endpoint);
        let flurl = FlUrl::new(&url).set_timeout(self.timeout);
        let flurl = self.add_headers(flurl, req);

        flurl
    }

    fn add_headers<T: Serialize>(&self, flurl: FlUrl, req: &ApiRequest<T>) -> FlUrl {
        let content_str = "application/json";
        let date = Utc::now();
        let token = crate::generate_token(&self.secret_key, &self.public_key, &req.ip, date);

        flurl
            .with_header("Content-Type", content_str)
            .with_header("Accept", content_str)
            .with_header("x-public-key", &self.public_key)
            .with_header("x-buyer-ip", &req.ip)
            .with_header("ip", &req.ip)
            .with_header("x-date", format_date(date)) // 2024-01-27T23:59:59
            .with_header("x-token", token)
            .with_header("merchant", &self.merchant_code)
            .with_header("x-source", req.source.as_str())
            .with_header("x-id", &req.source_id)
    }
}

async fn handle_flurl_resp(
    response: flurl::FlUrlResponse,
    request_json: Option<&str>,
    request_url: &str,
    request_method: &Method,
) -> Result<String, String> {
    let status_code = response.get_status_code();
    let result = response.receive_body().await;

    let Ok(body_bytes) = result else {
        return Err(format!("FlUrl failed to receive_body: {:?}", result.unwrap_err()).into());
    };

    let body_str = String::from_utf8(body_bytes).unwrap();

    if status_code > 299 {
        return Err(format!("Response code: {status_code:?}. Url: {request_method:?} {request_url}. Request: {request_json:?} Response: {body_str}"));
    }

    Ok(body_str)
}
