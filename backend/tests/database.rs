//! 真实数据库契约；仅对显式 TEST_DATABASE_URL 指向的专用测试库执行。

use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use rust_axum_starter::{
    api::router::router,
    bootstrap::{AppState, build_state},
    domain::health::HealthDomain,
    platform::{
        config::Settings,
        db::{DatabasePool, is_unique_violation},
    },
};
use serial_test::serial;
use tower::ServiceExt;

const EXPECTED_MIGRATIONS: i64 = 3;

#[tokio::test]
#[ignore = "需要 PostgreSQL 专用测试库 TEST_DATABASE_URL"]
#[serial]
async fn postgres_migrations_and_readiness_follow_contract() {
    database_contract("postgres").await;
}

#[tokio::test]
#[ignore = "需要 MySQL 专用测试库 TEST_DATABASE_URL"]
#[serial]
async fn mysql_migrations_and_readiness_follow_contract() {
    database_contract("mysql").await;
}

async fn database_contract(expected_kind: &str) {
    let url: String = std::env::var("TEST_DATABASE_URL").expect("必须提供 TEST_DATABASE_URL");
    let settings: Settings = Settings::from_sources(
        HashMap::new(),
        HashMap::from([
            ("APP_DATABASE__URL".to_owned(), url),
            ("APP_DATABASE__MAX_CONNECTIONS".to_owned(), "1".to_owned()),
        ]),
    )
    .unwrap();
    let state: AppState = build_state(&settings).unwrap();
    let database: DatabasePool = state.health.database.unwrap();
    assert_eq!(database.kind(), expected_kind);

    database.migrate().await.unwrap();
    database.migrate().await.unwrap();
    match &database {
        DatabasePool::Postgres(pool) => {
            let count: i64 =
                sqlx::query_scalar("SELECT COUNT(*) FROM _sqlx_migrations WHERE success = true")
                    .fetch_one(pool)
                    .await
                    .unwrap();
            assert_eq!(count, EXPECTED_MIGRATIONS);
            let mut connection = pool.acquire().await.unwrap();
            sqlx::query("CREATE TEMPORARY TABLE contract_unique (id INTEGER PRIMARY KEY)")
                .execute(&mut *connection)
                .await
                .unwrap();
            sqlx::query("INSERT INTO contract_unique (id) VALUES ($1)")
                .bind(1)
                .execute(&mut *connection)
                .await
                .unwrap();
            let error = sqlx::query("INSERT INTO contract_unique (id) VALUES ($1)")
                .bind(1)
                .execute(&mut *connection)
                .await
                .unwrap_err();
            assert!(is_unique_violation(&error));
            assert_probe_times_out(&database).await;
        }
        DatabasePool::MySql(pool) => {
            let count: i64 =
                sqlx::query_scalar("SELECT COUNT(*) FROM _sqlx_migrations WHERE success = true")
                    .fetch_one(pool)
                    .await
                    .unwrap();
            assert_eq!(count, EXPECTED_MIGRATIONS);
            let mut connection = pool.acquire().await.unwrap();
            sqlx::query(
                "CREATE TEMPORARY TABLE contract_unique (id INTEGER PRIMARY KEY) ENGINE=InnoDB",
            )
            .execute(&mut *connection)
            .await
            .unwrap();
            sqlx::query("INSERT INTO contract_unique (id) VALUES (?)")
                .bind(1)
                .execute(&mut *connection)
                .await
                .unwrap();
            let error = sqlx::query("INSERT INTO contract_unique (id) VALUES (?)")
                .bind(1)
                .execute(&mut *connection)
                .await
                .unwrap_err();
            assert!(is_unique_violation(&error));
            assert_probe_times_out(&database).await;
        }
    }

    let response = ready_response(&database, Duration::from_secs(3)).await;
    assert_eq!(response.status(), StatusCode::OK);
    let body = to_bytes(response.into_body(), 4096).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(
        json["data"],
        serde_json::json!({"ready": true, "database": "ready"})
    );
    database.close().await;
    let response = ready_response(&database, Duration::from_secs(1)).await;
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
}

async fn ready_response(database: &DatabasePool, timeout: Duration) -> axum::response::Response {
    router(AppState {
        health: HealthDomain::new(Some(database.clone()), timeout),
        tokens: None,
    })
    .oneshot(
        Request::builder()
            .uri("/ready")
            .body(Body::empty())
            .unwrap(),
    )
    .await
    .unwrap()
}

/// 此时唯一连接仍被占用，验证总时限覆盖连接池等待。
async fn assert_probe_times_out(database: &DatabasePool) {
    let started: Instant = Instant::now();
    let response = ready_response(database, Duration::from_millis(50)).await;
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    assert!(started.elapsed() < Duration::from_secs(1));
}
