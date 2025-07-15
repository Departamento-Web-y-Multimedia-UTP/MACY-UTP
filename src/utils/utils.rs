use axum::{Json, http::StatusCode};
use serde_json::json;

use reqwest::header::{HeaderMap,HeaderValue};

/// Converts any error with a label into an Axum-compatible error response.
pub fn json_error<E: std::fmt::Display>(
    status: StatusCode,
    label: &str,
    err: E,
) -> (StatusCode, Json<serde_json::Value>) {
    let error_msg = format!("{}: {}", label, err);
    (
        status,
        Json(json!({
            "success": false,
            "error": error_msg
        })),
    )
}

pub fn insert_auth_headers(api_key: String, secret_key: String, auth_token: Option<String>) -> HeaderMap {
    let mut auth_headers = HeaderMap::new();

    auth_headers.insert("api-key", HeaderValue::from_str(&api_key).unwrap());
    auth_headers.insert("secret-key", HeaderValue::from_str(&secret_key).unwrap());

    if let Some(t) = auth_token{
        auth_headers.insert("authorization", HeaderValue::from_str(&t).unwrap());
    }

    //println!("{:#?}", auth_headers);
    return auth_headers;
}
