use reqwest::Client;
use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String,
}

// 同一真实 HTTP 验证适用于 PostgreSQL 和 MySQL，不伪造认证响应。
pub async fn login(base: &str, username: &str, password: &str) -> Tokens {
    let response = Client::new()
        .post(format!("{base}/api/v1/auth/login"))
        .json(&json!({"username":username,"password":password}))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
    let body: Value = response.json().await.unwrap();
    assert_eq!(body["code"], "200");
    serde_json::from_value(body["data"].clone()).unwrap()
}
