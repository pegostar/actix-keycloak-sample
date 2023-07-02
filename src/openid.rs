use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderValue};
use serde_json::Value;
use crate::structs::base::Token;

pub async fn get_token(path: &str, payload: serde_json::Value) -> Result<Token, reqwest::Error> {
    let client = reqwest::Client::new();
    let k_res = client
        .post(path)
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .form(&payload)
        .send()
        .await?.error_for_status()?;
    k_res.json().await
}

pub fn get_introspect(path: &str, payload: serde_json::Value) -> Result<Value, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let data = payload.get("token").unwrap().as_str().unwrap();
    let mut headers :reqwest::header::HeaderMap = reqwest::header::HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(AUTHORIZATION, HeaderValue::from_str(data).unwrap());


    let k_res = client
        .post(path)
        .headers(headers)
        .form(&payload)
        .send();

    let data = k_res?.json();

    data
}
pub async fn set_renew(path: &str, access: &str, payload: serde_json::Value) -> Result<Token, reqwest::Error> {
    let client = reqwest::Client::new();

    let mut headers :reqwest::header::HeaderMap = reqwest::header::HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(AUTHORIZATION, HeaderValue::from_str(access).unwrap());

    let k_res = client
        .post(path)
        .headers(headers)
        .form(&payload)
        .send()
        .await?.error_for_status()?;
    k_res.json().await
}

pub async fn set_logout(path: &str, access: &str, payload: serde_json::Value) {
    let client = reqwest::Client::new();

    let mut headers :reqwest::header::HeaderMap = reqwest::header::HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(AUTHORIZATION, HeaderValue::from_str(access).unwrap());


    client
        .post(path)
        .headers(headers)
        .form(&payload)
        .send()
        .await.expect("TODO: panic message");
}