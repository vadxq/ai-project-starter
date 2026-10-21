use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Method, Request, StatusCode},
    routing::post,
};
use serde::Deserialize;
use serde_json::{Value, json};
use tower::util::ServiceExt;

use rust_axum_starter::{
    api::{
        response::{self, ApiJson, AppJson},
        router::router,
    },
    bootstrap::AppState,
    domain::health::HealthDomain,
};

#[derive(Debug, Deserialize)]
struct EchoRequest {
    name: String,
}

async fn echo(AppJson(req): AppJson<EchoRequest>) -> ApiJson<Value> {
    response::ok(json!({ "name": req.name }))
}

fn test_state() -> AppState {
    AppState {
        health: HealthDomain::new(None, std::time::Duration::from_secs(1)),
        tokens: None,
    }
}

async fn response_json(response: axum::response::Response) -> Value {
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    serde_json::from_slice(&body).unwrap()
}

#[tokio::test]
async fn health_route_returns_ok_json() {
    let app = router(test_state());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response_json(response).await,
        json!({
            "code": "200",
            "message": "success",
            "data": { "status": "ok" }
        })
    );
}

#[tokio::test]
async fn ready_route_without_database_returns_unavailable() {
    let app = router(test_state());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/ready")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(
        response_json(response).await["message"],
        "database not configured"
    );
}

#[tokio::test]
async fn unknown_route_returns_not_found_json() {
    let app = router(test_state());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/missing")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    assert_eq!(response_json(response).await["message"], "route not found");
}

#[tokio::test]
async fn unsupported_method_returns_method_not_allowed_json() {
    let app = router(test_state());

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(
        response_json(response).await["message"],
        "method not allowed"
    );
}

#[tokio::test]
async fn app_json_maps_invalid_json_to_unified_error() {
    let app = Router::new().route("/echo", post(echo));

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/echo")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"name":"starter""#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        response_json(response).await["message"],
        "invalid json body"
    );
}

#[tokio::test]
async fn request_ids_are_generated_and_propagated_on_all_responses() {
    for (method, path) in [
        (Method::GET, "/health"),
        (Method::GET, "/ready"),
        (Method::GET, "/missing"),
        (Method::POST, "/health"),
    ] {
        let response = router(test_state())
            .oneshot(
                Request::builder()
                    .method(method)
                    .uri(path)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let id = response
            .headers()
            .get("x-request-id")
            .expect("响应必须包含请求 ID");
        assert_eq!(id.to_str().unwrap().len(), 36);
    }
}

#[tokio::test]
async fn supplied_request_id_is_preserved_including_cors_preflight() {
    let response = router(test_state())
        .oneshot(
            Request::builder()
                .method(Method::OPTIONS)
                .uri("/health")
                .header("origin", "https://example.com")
                .header("access-control-request-method", "GET")
                .header("x-request-id", "integration-request")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()["x-request-id"], "integration-request");
    assert_eq!(response.headers()["access-control-allow-origin"], "*");
}

#[tokio::test]
async fn app_json_preserves_axum_rejection_status_and_unified_body() {
    let cases = [
        (
            None,
            r#"{"name":"starter"}"#,
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
        ),
        (
            Some("application/json"),
            r#"{"name":42}"#,
            StatusCode::UNPROCESSABLE_ENTITY,
        ),
        (
            Some("application/json"),
            r#"{"name":"starter""#,
            StatusCode::BAD_REQUEST,
        ),
    ];
    for (content_type, body, expected) in cases {
        let mut request = Request::builder().method(Method::POST).uri("/echo");
        if let Some(value) = content_type {
            request = request.header("content-type", value);
        }
        let response = Router::new()
            .route("/echo", post(echo))
            .oneshot(request.body(Body::from(body)).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), expected);
        let body = response_json(response).await;
        assert_eq!(body["code"], expected.as_u16().to_string());
        assert_eq!(body["details"]["source_status"], expected.as_u16());
    }
}

#[tokio::test]
async fn app_json_returns_payload_too_large_for_body_limit() {
    let response = Router::new()
        .route("/echo", post(echo))
        .layer(axum::extract::DefaultBodyLimit::max(16))
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/echo")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"name":"larger than the body limit"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let body = response_json(response).await;
    assert_eq!(body["code"], "413");
    assert_eq!(body["details"]["source_status"], 413);
}
