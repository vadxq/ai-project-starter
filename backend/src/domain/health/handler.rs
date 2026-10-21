//! 健康检查路由对应的 handler。

use crate::api::{error::ErrorResponse, response::SuccessResponse};
use axum::extract::State;
use serde::Serialize;
use utoipa::ToSchema;

use crate::{
    api::error::{AppError, AppResult},
    api::response::{self, ApiJson},
    bootstrap::AppState,
};

#[derive(Debug, Serialize, ToSchema)]
/// `/health` 的响应体。
pub struct HealthResponse {
    pub status: &'static str,
}

#[derive(Debug, Serialize, ToSchema)]
/// `/ready` 的响应体。
pub struct ReadyResponse {
    pub ready: bool,
    pub database: &'static str,
}

/// 存活探针，只表明进程已经正常接收请求。
#[utoipa::path(get, path = "/health", operation_id = "health", responses((status = 200, body = SuccessResponse<HealthResponse>)))]
pub async fn health() -> ApiJson<HealthResponse> {
    response::ok(HealthResponse { status: "ok" })
}

/// 就绪探针，会实际探测数据库是否可用。
#[utoipa::path(get, path = "/ready", operation_id = "ready", responses((status = 200, body = SuccessResponse<ReadyResponse>), (status = 503, body = ErrorResponse)))]
pub async fn ready(State(state): State<AppState>) -> AppResult<ApiJson<ReadyResponse>> {
    let Some(database) = &state.health.database else {
        return Err(AppError::unavailable("database not configured"));
    };

    match tokio::time::timeout(state.health.timeout, database.ping()).await {
        Ok(Ok(())) => Ok(response::ok(ReadyResponse {
            ready: true,
            database: "ready",
        })),
        Ok(Err(error)) => {
            tracing::warn!(database = database.kind(), error = %error, "database not ready");
            Err(AppError::unavailable("database not ready"))
        }
        Err(_) => {
            tracing::warn!(
                database = database.kind(),
                timeout_ms = state.health.timeout.as_millis(),
                "database probe timed out"
            );
            Err(AppError::unavailable("database not ready"))
        }
    }
}
