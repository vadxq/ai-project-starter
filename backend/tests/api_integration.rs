mod support;

use reqwest::{Client, Method, Response};
use serde_json::{Value, json};
use uuid::Uuid;

struct Session {
    http: Client,
    base: String,
    token: String,
}

impl Session {
    async fn request(&self, method: reqwest::Method, path: &str, body: Option<Value>) -> Response {
        let mut request = self
            .http
            .request(method, format!("{}{path}", self.base))
            .bearer_auth(&self.token)
            .header("Accept-Language", "zh-CN");
        if let Some(body) = body {
            request = request.json(&body);
        }
        request.send().await.unwrap()
    }
}

async fn problem(response: Response, status: u16, code: &str) {
    assert_eq!(response.status().as_u16(), status);
    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/problem+json"
    );
    let request_id = response
        .headers()
        .get("x-request-id")
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    let value: Value = response.json().await.unwrap();
    assert_eq!(value["code"], code);
    assert_eq!(value["status"], status);
    assert_eq!(value["requestId"], request_id);
    assert!(Uuid::parse_str(&request_id).is_ok());
}

async fn validation_cases(session: &Session) {
    for body in [
        json!({"title":" ","completed":false}),
        json!({"title":"中".repeat(81),"completed":false}),
        json!({"title":"ok"}),
        json!({"title":"ok","completed":false,"extra":1}),
        json!({"title":null,"completed":false}),
    ] {
        problem(
            session
                .request(Method::POST, "/api/v1/items", Some(body))
                .await,
            400,
            "validation_error",
        )
        .await;
    }
    for path in [
        "/api/v1/items",
        "/api/v1/items?limit=0&offset=0",
        "/api/v1/items?limit=101&offset=0",
        "/api/v1/items?limit=20&offset=-1",
        "/api/v1/items?limit=20&offset=0&extra=1",
        "/api/v1/items/not-a-uuid",
    ] {
        problem(
            session.request(Method::GET, path, None).await,
            400,
            "validation_error",
        )
        .await;
    }
    let path = format!("/api/v1/items/{}", Uuid::new_v4());
    for body in [
        json!({"version":1}),
        json!({"version":0,"completed":true}),
        json!({"version":1,"title":null}),
        json!({"version":1,"completed":null}),
    ] {
        problem(
            session.request(Method::PATCH, &path, Some(body)).await,
            400,
            "validation_error",
        )
        .await;
    }
    problem(
        session
            .request(
                Method::POST,
                "/api/v1/items",
                Some(json!({"title":"a".repeat(20_000),"completed":false})),
            )
            .await,
        413,
        "payload_too_large",
    )
    .await;
}

async fn crud_cases(alice: &Session, bob: &Session) {
    let response = alice
        .request(
            Method::POST,
            "/api/v1/items",
            Some(json!({"title":" \t真实事项\n","completed":false})),
        )
        .await;
    assert_eq!(response.status(), 201);
    let location = response
        .headers()
        .get("location")
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    let item: Value = response.json().await.unwrap();
    assert_eq!(item["title"], "真实事项");
    assert_eq!(item["version"], 1);
    let date = item["createdAt"].as_str().unwrap();
    assert!(regex::Regex::new(r"\.\d{3}Z$").unwrap().is_match(date));
    let get: Value = alice
        .request(Method::GET, &location, None)
        .await
        .json()
        .await
        .unwrap();
    assert_eq!(get, item);
    for method in [Method::GET, Method::PATCH, Method::DELETE] {
        let (path, body) = if method == Method::DELETE {
            (format!("{location}?version=1"), None)
        } else {
            (
                location.clone(),
                (method == Method::PATCH).then(|| json!({"version":1,"completed":true})),
            )
        };
        problem(bob.request(method, &path, body).await, 404, "not_found").await;
    }
    let page: Value = alice
        .request(Method::GET, "/api/v1/items?limit=100&offset=0", None)
        .await
        .json()
        .await
        .unwrap();
    assert!(
        page["items"]
            .as_array()
            .unwrap()
            .iter()
            .any(|value| value["id"] == item["id"])
    );
    let patch = Some(json!({"version":1,"completed":true}));
    let (first, second) = tokio::join!(
        alice.request(Method::PATCH, &location, patch.clone()),
        alice.request(Method::PATCH, &location, patch)
    );
    let mut statuses = [first.status().as_u16(), second.status().as_u16()];
    statuses.sort();
    assert_eq!(statuses, [200, 409]);
    problem(
        alice
            .request(Method::DELETE, &format!("{location}?version=1"), None)
            .await,
        409,
        "version_conflict",
    )
    .await;
    assert_eq!(
        alice
            .request(Method::DELETE, &format!("{location}?version=2"), None)
            .await
            .status(),
        204
    );
    problem(
        alice
            .request(Method::DELETE, &format!("{location}?version=2"), None)
            .await,
        404,
        "not_found",
    )
    .await;
}

#[tokio::test]
#[ignore = "requires running PostgreSQL and API with alice/bob accounts; see README"]
async fn real_jwt_and_database_contract() {
    let base = std::env::var("TEST_API_URL").expect("TEST_API_URL");
    let alice_tokens = support::login(&base, "alice", "starter-password").await;
    let bob_tokens = support::login(&base, "bob", "starter-password").await;
    let alice = Session {
        http: Client::new(),
        base: base.clone(),
        token: alice_tokens.access_token,
    };
    let bob = Session {
        http: Client::new(),
        base: base.clone(),
        token: bob_tokens.access_token,
    };
    assert_eq!(
        alice.request(Method::GET, "/health", None).await.status(),
        200
    );
    assert_eq!(
        alice.request(Method::GET, "/ready", None).await.status(),
        200
    );
    let me: Value = alice
        .request(Method::GET, "/api/v1/me", None)
        .await
        .json()
        .await
        .unwrap();
    let again: Value = alice
        .request(Method::GET, "/api/v1/me", None)
        .await
        .json()
        .await
        .unwrap();
    assert_eq!(me, again);
    let other: Value = bob
        .request(Method::GET, "/api/v1/me", None)
        .await
        .json()
        .await
        .unwrap();
    assert_ne!(me["id"], other["id"]);
    for token in [String::new(), "not-a-token".to_owned()] {
        let invalid = Session {
            http: Client::new(),
            base: base.clone(),
            token,
        };
        problem(
            invalid.request(Method::GET, "/api/v1/me", None).await,
            401,
            "unauthorized",
        )
        .await;
    }
    problem(
        alice.request(Method::GET, "/missing", None).await,
        404,
        "not_found",
    )
    .await;
    problem(
        alice.request(Method::PUT, "/api/v1/items", None).await,
        405,
        "method_not_allowed",
    )
    .await;
    validation_cases(&alice).await;
    crud_cases(&alice, &bob).await;
}

#[tokio::test]
#[ignore = "requires running PostgreSQL/API and JWT_SECRET; see README"]
async fn passwords_jwt_rotation_and_logout_are_enforced() {
    let base = std::env::var("TEST_API_URL").expect("TEST_API_URL");
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET");
    let http = Client::new();
    for username in ["alice", "nonexistent-account"] {
        let response = http
            .post(format!("{base}/api/v1/auth/login"))
            .json(&json!({"username":username,"password":"wrong-password"}))
            .send()
            .await
            .unwrap();
        problem(response, 401, "invalid_credentials").await;
    }
    let tokens = support::login(&base, "alice", "starter-password").await;
    let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.validate_aud = false;
    let claims = jsonwebtoken::decode::<Value>(
        &tokens.access_token,
        &jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .unwrap()
    .claims;
    for field in ["iss", "aud", "exp", "sub"] {
        let mut invalid = claims.clone();
        invalid[field] = match field {
            "exp" => json!(1),
            "sub" => json!("invalid-uuid"),
            _ => json!("wrong"),
        };
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256),
            &invalid,
            &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
        )
        .unwrap();
        let session = Session {
            http: http.clone(),
            base: base.clone(),
            token,
        };
        problem(
            session.request(Method::GET, "/api/v1/me", None).await,
            401,
            "unauthorized",
        )
        .await;
    }
    let tampered = jsonwebtoken::encode(
        &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(b"a-different-signing-secret-at-least-32-bytes"),
    )
    .unwrap();
    let invalid = Session {
        http: http.clone(),
        base: base.clone(),
        token: tampered,
    };
    problem(
        invalid.request(Method::GET, "/api/v1/me", None).await,
        401,
        "unauthorized",
    )
    .await;
    let refresh = || {
        http.post(format!("{base}/api/v1/auth/refresh"))
            .json(&json!({"refreshToken":tokens.refresh_token}))
            .send()
    };
    let (first, second) = tokio::join!(refresh(), refresh());
    let first = first.unwrap();
    let second = second.unwrap();
    let (winner, loser) = if first.status() == 200 {
        (first, second)
    } else {
        (second, first)
    };
    problem(loser, 401, "unauthorized").await;
    let rotated: support::Tokens = winner.json().await.unwrap();
    assert_ne!(rotated.refresh_token, tokens.refresh_token);
    let session = Session {
        http: http.clone(),
        base: base.clone(),
        token: rotated.access_token,
    };
    assert_eq!(
        session
            .request(Method::GET, "/api/v1/me", None)
            .await
            .status(),
        200
    );
    assert_eq!(
        http.post(format!("{base}/api/v1/auth/logout"))
            .json(&json!({"refreshToken":rotated.refresh_token}))
            .send()
            .await
            .unwrap()
            .status(),
        204
    );
    problem(
        session.request(Method::GET, "/api/v1/me", None).await,
        401,
        "unauthorized",
    )
    .await;
    problem(
        http.post(format!("{base}/api/v1/auth/refresh"))
            .json(&json!({"refreshToken":rotated.refresh_token}))
            .send()
            .await
            .unwrap(),
        401,
        "unauthorized",
    )
    .await;
}
