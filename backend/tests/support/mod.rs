use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String,
}

// 使用真实 Rust 密码校验与 PostgreSQL 会话；不伪造认证响应。
pub async fn login(base: &str, username: &str, password: &str) -> Tokens {
    let response = Client::new()
        .post(format!("{base}/api/v1/auth/login"))
        .json(&json!({"username":username,"password":password}))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
    response.json().await.unwrap()
}
