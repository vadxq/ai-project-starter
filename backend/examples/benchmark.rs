#[path = "../tests/support/mod.rs"]
mod support;

use std::time::{Duration, Instant};

use reqwest::{Client, Method};
use serde_json::{Value, json};
use tokio::task::JoinSet;

const SEED_ITEMS: usize = 1_000;
const CONCURRENCY: usize = 5;
const DURATION: Duration = Duration::from_secs(60);

async fn request(
    client: &Client,
    target: (&str, &str),
    input: (Method, &str, Option<Value>),
) -> Value {
    let mut request = client
        .request(input.0, format!("{}{}", target.0, input.1))
        .bearer_auth(target.1);
    if let Some(body) = input.2 {
        request = request.json(&body);
    }
    let response = request.send().await.expect("benchmark HTTP request");
    assert!(
        response.status().is_success(),
        "unexpected benchmark status: {}",
        response.status()
    );
    if response.status() == 204 {
        Value::Null
    } else {
        response.json().await.expect("benchmark DTO")
    }
}

async fn worker(base: String, token: String) -> Vec<u128> {
    let client = Client::new();
    let deadline = Instant::now() + DURATION;
    let mut latencies: Vec<u128> = Vec::new();
    while Instant::now() < deadline {
        let started = Instant::now();
        let item = request(
            &client,
            (&base, &token),
            (
                Method::POST,
                "/api/v1/items",
                Some(json!({"title":"Performance transaction", "completed":false})),
            ),
        )
        .await;
        latencies.push(started.elapsed().as_micros());
        let path = format!("/api/v1/items/{}", item["id"].as_str().unwrap());
        for (method, route, body) in [
            (Method::GET, path.clone(), None),
            (
                Method::PATCH,
                path.clone(),
                Some(json!({"version":1,"completed":true})),
            ),
            (
                Method::GET,
                "/api/v1/items?limit=20&offset=0".to_owned(),
                None,
            ),
            (Method::DELETE, format!("{path}?version=2"), None),
        ] {
            let started = Instant::now();
            request(&client, (&base, &token), (method, &route, body)).await;
            latencies.push(started.elapsed().as_micros());
        }
    }
    latencies
}

#[tokio::main]
async fn main() {
    let base = std::env::var("TEST_API_URL").expect("TEST_API_URL must point to a release API");
    let tokens = support::login(&base, "bob", "starter-password").await;
    let client = Client::new();
    let mut ids: Vec<String> = Vec::new();
    for _ in 0..SEED_ITEMS {
        let item = request(
            &client,
            (&base, &tokens.access_token),
            (
                Method::POST,
                "/api/v1/items",
                Some(json!({"title":"Benchmark seed", "completed":false})),
            ),
        )
        .await;
        ids.push(item["id"].as_str().unwrap().to_owned());
    }
    let mut workers = JoinSet::new();
    for _ in 0..CONCURRENCY {
        workers.spawn(worker(base.clone(), tokens.access_token.clone()));
    }
    let mut latencies: Vec<u128> = Vec::new();
    while let Some(result) = workers.join_next().await {
        latencies.extend(result.expect("benchmark worker"));
    }
    for id in ids {
        request(
            &client,
            (&base, &tokens.access_token),
            (
                Method::DELETE,
                &format!("/api/v1/items/{id}?version=1"),
                None,
            ),
        )
        .await;
    }
    latencies.sort_unstable();
    assert_eq!(
        client
            .post(format!("{base}/api/v1/auth/logout"))
            .json(&json!({"refreshToken": tokens.refresh_token}))
            .send()
            .await
            .unwrap()
            .status(),
        204
    );
    let p95: f64 = latencies[(latencies.len() * 95 / 100).saturating_sub(1)] as f64 / 1_000.0;
    println!(
        "{}",
        json!({"seedItems":SEED_ITEMS,"concurrency":CONCURRENCY,"durationSeconds":DURATION.as_secs(),"requests":latencies.len(),"p95Ms":p95,"unexpectedErrors":0})
    );
    assert!(p95 <= 300.0, "CRUD p95 exceeds 300 ms");
}
