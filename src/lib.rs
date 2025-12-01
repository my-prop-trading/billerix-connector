pub mod api;
pub mod model;
pub mod webhook;

use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use sha2::Sha256;

/// Format the date exactly as API expects: "YYYY-MM-DDTHH:MM:SS"
pub fn format_date(date: DateTime<Utc>) -> String {
    date.format("%Y-%m-%dT%H:%M:%S").to_string()
}

pub fn generate_token(secret_key: &str, public_key: &str, ip: &str, date: DateTime<Utc>) -> String {
    let date_str = format_date(date);
    let token_string = format!("{secret_key}{public_key}{ip}{date_str}");
    let mut mac =
        Hmac::<Sha256>::new_from_slice(secret_key.as_bytes()).expect("HMAC can take any key size");
    mac.update(token_string.as_bytes());

    hex::encode(mac.finalize().into_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_token() {
        let secret_key = "secret-key-test123123123abc";
        let public_key = "aa46a835-36fa-4f75-ba3d-dc8785912345";
        let buyer_ip = "10.10.10.10";

        // PHP example: "2024-01-27T23:59:59"
        let naive =
            chrono::NaiveDateTime::parse_from_str("2024-01-27T23:59:59", "%Y-%m-%dT%H:%M:%S")
                .unwrap();
        let date = naive.and_utc();
        let expected = "5cdc01c2d66c52a513f58e077d85660468852fc141d305888416a151a05dc159";

        let token = generate_token(secret_key, public_key, buyer_ip, date);

        assert_eq!(token, expected);
    }
}
